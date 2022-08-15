use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    fn time(__timer: *mut time_t) -> time_t;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    static mut optarg: *mut libc::c_char;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigwait(__set: *const sigset_t, __sig: *mut libc::c_int) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn sysinfo(__info: *mut sysinfo) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn closelog();
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn scandir(
        __dir: *const libc::c_char,
        __namelist: *mut *mut *mut dirent,
        __selector: Option::<unsafe extern "C" fn(*const dirent) -> libc::c_int>,
        __cmp: Option::<
            unsafe extern "C" fn(*mut *const dirent, *mut *const dirent) -> libc::c_int,
        >,
    ) -> libc::c_int;
    fn alphasort(__e1: *mut *const dirent, __e2: *mut *const dirent) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn __xpg_basename(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn sem_init(
        __sem: *mut sem_t,
        __pshared: libc::c_int,
        __value: libc::c_uint,
    ) -> libc::c_int;
    fn sem_destroy(__sem: *mut sem_t) -> libc::c_int;
    fn ptrace(__request: __ptrace_request, _: ...) -> libc::c_long;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn pthread_detach(__th: pthread_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
    fn sem_wait(__sem: *mut sem_t) -> libc::c_int;
    fn sem_timedwait(__sem: *mut sem_t, __abstime: *const timespec) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
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
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn geteuid() -> __uid_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn pthread_setcanceltype(
        __type: libc::c_int,
        __oldtype: *mut libc::c_int,
    ) -> libc::c_int;
    fn sem_post(__sem: *mut sem_t) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    fn execl(
        __path: *const libc::c_char,
        __arg: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> libc::c_int;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
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
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
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
pub type __gnuc_va_list = __builtin_va_list;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type pid_t = __pid_t;
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
pub union __anonunion____missing_field_name_920131302 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_1003085126 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1003085125 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_1003085126,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_920131302,
    pub __annonCompField2: __anonunion____missing_field_name_1003085125,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
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
pub union __anonunion_pthread_cond_t_951761805 {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type pthread_cond_t = __anonunion_pthread_cond_t_951761805;
pub type va_list___0 = __gnuc_va_list;
pub type LogLevel = libc::c_uint;
pub const error: LogLevel = 4;
pub const crit: LogLevel = 3;
pub const warn: LogLevel = 2;
pub const info: LogLevel = 1;
pub const debug: LogLevel = 0;
pub type __u16 = libc::c_ushort;
pub type __u32 = libc::c_uint;
pub type __kernel_long_t = libc::c_long;
pub type __kernel_ulong_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sysinfo {
    pub uptime: __kernel_long_t,
    pub loads: [__kernel_ulong_t; 3],
    pub totalram: __kernel_ulong_t,
    pub freeram: __kernel_ulong_t,
    pub sharedram: __kernel_ulong_t,
    pub bufferram: __kernel_ulong_t,
    pub totalswap: __kernel_ulong_t,
    pub freeswap: __kernel_ulong_t,
    pub procs: __u16,
    pub pad: __u16,
    pub totalhigh: __kernel_ulong_t,
    pub freehigh: __kernel_ulong_t,
    pub mem_unit: __u32,
    pub _f: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_sem_t_1036286215 {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_long,
}
pub type sem_t = __anonunion_sem_t_1036286215;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub bTriggered: bool,
    pub bManualReset: bool,
    pub Name: [libc::c_char; 64],
    pub nWaiters: libc::c_int,
}
pub type EHandleType = libc::c_uint;
pub const SEMAPHORE: EHandleType = 1;
pub const EVENT: EHandleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_654135139 {
    pub event: Event,
    pub semaphore: sem_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Handle {
    pub __annonCompField4: __anonunion____missing_field_name_654135139,
    pub type_0: EHandleType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProcDumpConfiguration {
    pub ProcessId: pid_t,
    pub ProcessName: *mut libc::c_char,
    pub SystemInfo: sysinfo,
    pub NumberOfDumpsCollecting: libc::c_int,
    pub NumberOfDumpsCollected: libc::c_int,
    pub bTerminated: bool,
    pub nQuit: libc::c_int,
    pub evtQuit: Handle,
    pub bTriggerThenSnoozeCPU: bool,
    pub bTriggerThenSnoozeMemory: bool,
    pub bTriggerThenSnoozeTimer: bool,
    pub CpuThreshold: libc::c_int,
    pub bCpuTriggerBelowValue: bool,
    pub MemoryThreshold: libc::c_int,
    pub bMemoryTriggerBelowValue: bool,
    pub ThresholdSeconds: libc::c_int,
    pub bTimerThreshold: bool,
    pub NumberOfDumpsToCollect: libc::c_int,
    pub WaitingForProcessName: bool,
    pub DiagnosticsLoggingEnabled: bool,
    pub ThreadThreshold: libc::c_int,
    pub FileDescriptorThreshold: libc::c_int,
    pub SignalNumber: libc::c_int,
    pub PollingInterval: libc::c_int,
    pub CoreDumpPath: *mut libc::c_char,
    pub CoreDumpName: *mut libc::c_char,
    pub nThreads: libc::c_int,
    pub Threads: [pthread_t; 3],
    pub semAvailableDumpSlots: Handle,
    pub evtCtrlHandlerCleanupComplete: Handle,
    pub evtBannerPrinted: Handle,
    pub evtConfigurationPrinted: Handle,
    pub evtDebugThreadInitialized: Handle,
    pub evtStartMonitoring: Handle,
    pub gcorePid: pid_t,
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
pub union __anonunion_pthread_condattr_t_488594145 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_condattr_t = __anonunion_pthread_condattr_t_488594145;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
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
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub __domainname: [libc::c_char; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
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
pub type __ptrace_request = libc::c_uint;
pub const PTRACE_GET_SYSCALL_INFO: __ptrace_request = 16910;
pub const PTRACE_SECCOMP_GET_METADATA: __ptrace_request = 16909;
pub const PTRACE_SECCOMP_GET_FILTER: __ptrace_request = 16908;
pub const PTRACE_SETSIGMASK: __ptrace_request = 16907;
pub const PTRACE_GETSIGMASK: __ptrace_request = 16906;
pub const PTRACE_PEEKSIGINFO: __ptrace_request = 16905;
pub const PTRACE_LISTEN: __ptrace_request = 16904;
pub const PTRACE_INTERRUPT: __ptrace_request = 16903;
pub const PTRACE_SEIZE: __ptrace_request = 16902;
pub const PTRACE_SETREGSET: __ptrace_request = 16901;
pub const PTRACE_GETREGSET: __ptrace_request = 16900;
pub const PTRACE_SETSIGINFO: __ptrace_request = 16899;
pub const PTRACE_GETSIGINFO: __ptrace_request = 16898;
pub const PTRACE_GETEVENTMSG: __ptrace_request = 16897;
pub const PTRACE_SETOPTIONS: __ptrace_request = 16896;
pub const PTRACE_SINGLEBLOCK: __ptrace_request = 33;
pub const PTRACE_SYSEMU_SINGLESTEP: __ptrace_request = 32;
pub const PTRACE_SYSEMU: __ptrace_request = 31;
pub const PTRACE_ARCH_PRCTL: __ptrace_request = 30;
pub const PTRACE_SET_THREAD_AREA: __ptrace_request = 26;
pub const PTRACE_GET_THREAD_AREA: __ptrace_request = 25;
pub const PTRACE_SYSCALL: __ptrace_request = 24;
pub const PTRACE_SETFPXREGS: __ptrace_request = 19;
pub const PTRACE_GETFPXREGS: __ptrace_request = 18;
pub const PTRACE_DETACH: __ptrace_request = 17;
pub const PTRACE_ATTACH: __ptrace_request = 16;
pub const PTRACE_SETFPREGS: __ptrace_request = 15;
pub const PTRACE_GETFPREGS: __ptrace_request = 14;
pub const PTRACE_SETREGS: __ptrace_request = 13;
pub const PTRACE_GETREGS: __ptrace_request = 12;
pub const PTRACE_SINGLESTEP: __ptrace_request = 9;
pub const PTRACE_KILL: __ptrace_request = 8;
pub const PTRACE_CONT: __ptrace_request = 7;
pub const PTRACE_POKEUSER: __ptrace_request = 6;
pub const PTRACE_POKEDATA: __ptrace_request = 5;
pub const PTRACE_POKETEXT: __ptrace_request = 4;
pub const PTRACE_PEEKUSER: __ptrace_request = 3;
pub const PTRACE_PEEKDATA: __ptrace_request = 2;
pub const PTRACE_PEEKTEXT: __ptrace_request = 1;
pub const PTRACE_TRACEME: __ptrace_request = 0;
pub type __clockid_t = libc::c_int;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_result {
    pub retVal: libc::c_int,
    pub threadIndex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct coordinator {
    pub condEventTriggered: pthread_cond_t,
    pub mutexEventTriggered: pthread_mutex_t,
    pub results: *mut thread_result,
    pub numberTriggered: libc::c_int,
    pub nWaiters: libc::c_int,
    pub stopIssued: libc::c_int,
    pub evtCanCleanUp: Handle,
    pub evtStartWaiting: Handle,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread_args {
    pub handle: *mut Handle,
    pub coordinator: *mut coordinator,
    pub milliseconds: libc::c_int,
    pub retVal: libc::c_int,
    pub threadIndex: libc::c_int,
}
pub type DIR = __dirstream;
pub type gid_t = __gid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ProcessStat {
    pub pid: pid_t,
    pub comm: *mut libc::c_char,
    pub state: libc::c_char,
    pub ppid: pid_t,
    pub pgrp: gid_t,
    pub session: libc::c_int,
    pub tty_nr: libc::c_int,
    pub tpgid: gid_t,
    pub flags: libc::c_uint,
    pub minflt: libc::c_ulong,
    pub cminflt: libc::c_ulong,
    pub majflt: libc::c_ulong,
    pub cmajflt: libc::c_ulong,
    pub utime: libc::c_ulong,
    pub stime: libc::c_ulong,
    pub cutime: libc::c_ulong,
    pub cstime: libc::c_ulong,
    pub priority: libc::c_long,
    pub nice: libc::c_long,
    pub num_threads: libc::c_long,
    pub itrealvalue: libc::c_long,
    pub starttime: libc::c_ulonglong,
    pub vsize: libc::c_ulong,
    pub rss: libc::c_long,
    pub rsslim: libc::c_ulong,
    pub startcode: libc::c_ulong,
    pub endcode: libc::c_ulong,
    pub startstack: libc::c_ulong,
    pub kstkesp: libc::c_ulong,
    pub kstkeip: libc::c_ulong,
    pub signal: libc::c_ulong,
    pub blocked: libc::c_ulong,
    pub sigignore: libc::c_ulong,
    pub sigcatch: libc::c_ulong,
    pub wchan: libc::c_ulong,
    pub nswap: libc::c_ulong,
    pub cnswap: libc::c_ulong,
    pub exit_signal: libc::c_int,
    pub processor: libc::c_int,
    pub rt_priority: libc::c_uint,
    pub policy: libc::c_uint,
    pub delayacct_blkio_ticks: libc::c_ulonglong,
    pub guest_time: libc::c_ulong,
    pub cguest_time: libc::c_long,
    pub start_data: libc::c_ulong,
    pub end_data: libc::c_ulong,
    pub start_brk: libc::c_ulong,
    pub arg_start: libc::c_ulong,
    pub arg_end: libc::c_ulong,
    pub env_start: libc::c_ulong,
    pub env_end: libc::c_ulong,
    pub exit_code: libc::c_int,
    pub num_filedescriptors: libc::c_int,
}
pub type ECoreDumpType = libc::c_uint;
pub const MANUAL: ECoreDumpType = 6;
pub const TIME: ECoreDumpType = 5;
pub const SIGNAL: ECoreDumpType = 4;
pub const FILEDESC: ECoreDumpType = 3;
pub const THREAD: ECoreDumpType = 2;
pub const CPU: ECoreDumpType = 1;
pub const COMMIT: ECoreDumpType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoreDumpWriter {
    pub Config: *mut ProcDumpConfiguration,
    pub Type: ECoreDumpType,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
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
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MagicVersion {
    pub Magic: [uint8_t; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_584942040 {
    pub _magic: MagicVersion,
    pub Magic: [uint8_t; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IpcHeader {
    pub __annonCompField6: __anonunion____missing_field_name_584942040,
    pub Size: uint16_t,
    pub CommandSet: uint8_t,
    pub CommandId: uint8_t,
    pub Reserved: uint16_t,
}
static mut LogLevelStrings: [*const libc::c_char; 5] = [
    b"DEBUG\0" as *const u8 as *const libc::c_char,
    b"INFO\0" as *const u8 as *const libc::c_char,
    b"WARN\0" as *const u8 as *const libc::c_char,
    b"CRITICAL\0" as *const u8 as *const libc::c_char,
    b"ERROR\0" as *const u8 as *const libc::c_char,
];
pub static mut LoggerLock: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub unsafe extern "C" fn LogFormatter(
    mut logLevel: LogLevel,
    mut message: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    let mut timeBuff: [libc::c_char; 64] = [0; 64];
    let mut rawTime: time_t = 0;
    let mut timeInfo: *mut tm = 0 as *mut tm;
    let mut trace: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut copy: ::std::ffi::VaListImpl;
    let mut traceLen: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut argsLen: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    timeInfo = 0 as *mut libc::c_void as *mut tm;
    trace = 0 as *mut libc::c_void as *mut libc::c_char;
    copy = args.clone();
    pthread_mutex_lock(&mut LoggerLock);
    rawTime = time(0 as *mut libc::c_void as *mut time_t);
    timeInfo = localtime(&mut rawTime as *mut time_t as *const time_t);
    strftime(
        timeBuff.as_mut_ptr(),
        64 as libc::c_int as size_t,
        b"%T\0" as *const u8 as *const libc::c_char,
        timeInfo as *const tm,
    );
    tmp = snprintf(
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as libc::c_int as size_t,
        b"[%s - %s]: \0" as *const u8 as *const libc::c_char,
        timeBuff.as_mut_ptr(),
        LogLevelStrings[logLevel as usize],
    );
    traceLen = tmp;
    tmp___0 = vsnprintf(
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as libc::c_int as size_t,
        message,
        copy.as_va_list(),
    );
    argsLen = tmp___0;
    tmp___1 = malloc((traceLen + argsLen + 1 as libc::c_int) as size_t);
    trace = tmp___1 as *mut libc::c_char;
    if trace.is_null() {
        pthread_mutex_unlock(&mut LoggerLock);
        return;
    }
    sprintf(
        trace,
        b"[%s - %s]: \0" as *const u8 as *const libc::c_char,
        timeBuff.as_mut_ptr(),
        LogLevelStrings[logLevel as usize],
    );
    vsprintf(trace.offset(traceLen as isize), message, args.as_va_list());
    if logLevel as libc::c_uint != 0 as libc::c_uint {
        puts(trace as *const libc::c_char);
    }
    syslog(7 as libc::c_int, b"%s\0" as *const u8 as *const libc::c_char, trace);
    free(trace as *mut libc::c_void);
    pthread_mutex_unlock(&mut LoggerLock);
}
pub unsafe extern "C" fn Log(
    mut logLevel: LogLevel,
    mut message: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    LogFormatter(logLevel, message, args_0.as_va_list());
}
pub unsafe extern "C" fn DiagTrace(mut message: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    if g_config.DiagnosticsLoggingEnabled {
        LogFormatter(debug, message, args_0.as_va_list());
    }
}
pub unsafe extern "C" fn CreateEvent(
    mut IsManualReset: bool,
    mut InitialState: bool,
) -> *mut Event {
    let mut event: *mut Event = 0 as *mut Event;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<Event>() as libc::c_ulong);
    event = tmp as *mut Event;
    if event as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        Log(error, b"INTERNAL_ERROR\0" as *const u8 as *const libc::c_char);
        DiagTrace(
            b"CreateEvent: failed memory allocation. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 24\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    InitEvent(event, IsManualReset, InitialState);
    return event;
}
pub unsafe extern "C" fn CreateNamedEvent(
    mut IsManualReset: bool,
    mut InitialState: bool,
    mut Name: *mut libc::c_char,
) -> *mut Event {
    let mut event: *mut Event = 0 as *mut Event;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<Event>() as libc::c_ulong);
    event = tmp as *mut Event;
    if event as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"CreateNamedEvent: failed memory allocation. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 43\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    InitNamedEvent(event, IsManualReset, InitialState, Name);
    return event;
}
pub unsafe extern "C" fn InitEvent(
    mut Event: *mut Event,
    mut IsManualReset: bool,
    mut InitialState: bool,
) {
    InitNamedEvent(
        Event,
        IsManualReset,
        InitialState,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
}
static mut unamedEventId: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn InitNamedEvent(
    mut Event: *mut Event,
    mut IsManualReset: bool,
    mut InitialState: bool,
    mut Name: *mut libc::c_char,
) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    pthread_mutex_init(
        &mut (*Event).mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    tmp = pthread_cond_init(
        &mut (*Event).cond as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    if tmp != 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"InitNamedEvent: failed pthread_cond_init. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 76\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    (*Event).bManualReset = IsManualReset;
    (*Event).bTriggered = InitialState;
    (*Event).nWaiters = 0 as libc::c_int;
    if Name as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        unamedEventId += 1;
        sprintf(
            ((*Event).Name).as_mut_ptr(),
            b"Unnamed Event %d\0" as *const u8 as *const libc::c_char,
            unamedEventId,
        );
    } else {
        tmp___0 = strlen(Name as *const libc::c_char);
        if tmp___0 >= 64 as libc::c_ulong {
            strncpy(
                ((*Event).Name).as_mut_ptr(),
                Name as *const libc::c_char,
                64 as libc::c_int as size_t,
            );
            (*Event).Name[63 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        } else {
            strcpy(((*Event).Name).as_mut_ptr(), Name as *const libc::c_char);
        }
    };
}
pub unsafe extern "C" fn DestroyEvent(mut Event: *mut Event) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = pthread_cond_destroy(&mut (*Event).cond);
    if tmp != 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"DestroyEvent: failed pthread_cond_destroy. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 104\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    tmp___0 = pthread_mutex_destroy(&mut (*Event).mutex);
    if tmp___0 != 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"DestroyEvent: failed pthread_mutex_destroy. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 109\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
}
pub unsafe extern "C" fn SetEvent(mut Event: *mut Event) -> bool {
    let mut success: libc::c_int = 0;
    success = 0 as libc::c_int;
    success = pthread_mutex_lock(&mut (*Event).mutex);
    if success == 0 as libc::c_int {
        (*Event).bTriggered = 1 as libc::c_int != 0;
        if (*Event).bManualReset {
            pthread_cond_broadcast(&mut (*Event).cond);
        } else {
            pthread_cond_signal(&mut (*Event).cond);
        }
        pthread_mutex_unlock(&mut (*Event).mutex);
    } else {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"SetEvent: failed pthread_mutex_lock. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 135\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    return success == 0 as libc::c_int;
}
pub unsafe extern "C" fn ResetEvent(mut Event: *mut Event) -> bool {
    let mut success: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    success = 0 as libc::c_int;
    success = pthread_mutex_lock(&mut (*Event).mutex);
    if success == 0 as libc::c_int {
        (*Event).bTriggered = 0 as libc::c_int != 0;
        tmp = pthread_mutex_unlock(&mut (*Event).mutex);
        if tmp != 0 as libc::c_int {
            Log(
                error,
                b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                    as *const u8 as *const libc::c_char,
            );
            DiagTrace(
                b"ResetEvent: failed pthread_mutex_unlock. %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/Events.c, at line 157\0" as *const u8 as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
    } else {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"ResetEvent: failed pthread_mutex_lock. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Events.c, at line 163\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    return success == 0 as libc::c_int;
}
pub static mut g_evtConfigurationInitialized: Handle = {
    let mut init = Handle {
        __annonCompField4: __anonunion____missing_field_name_654135139 {
            event: {
                let mut init = Event {
                    mutex: __anonunion_pthread_mutex_t_335460617 {
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
                    },
                    cond: __anonunion_pthread_cond_t_951761805 {
                        __data: {
                            let mut init = __pthread_cond_s {
                                __annonCompField1: __anonunion____missing_field_name_920131302 {
                                    __wseq: 0 as libc::c_ulonglong,
                                },
                                __annonCompField2: __anonunion____missing_field_name_1003085125 {
                                    __g1_start: 0 as libc::c_ulonglong,
                                },
                                __g_refs: [0 as libc::c_uint, 0 as libc::c_uint],
                                __g_size: [0 as libc::c_uint, 0 as libc::c_uint],
                                __g1_orig_size: 0 as libc::c_uint,
                                __wrefs: 0 as libc::c_uint,
                                __g_signals: [0 as libc::c_uint, 0 as libc::c_uint],
                            };
                            init
                        },
                    },
                    bTriggered: 0 as libc::c_int != 0,
                    bManualReset: 1 as libc::c_int != 0,
                    Name: [
                        'C' as i32 as libc::c_char,
                        'o' as i32 as libc::c_char,
                        'n' as i32 as libc::c_char,
                        'f' as i32 as libc::c_char,
                        'i' as i32 as libc::c_char,
                        'g' as i32 as libc::c_char,
                        'u' as i32 as libc::c_char,
                        'r' as i32 as libc::c_char,
                        'a' as i32 as libc::c_char,
                        't' as i32 as libc::c_char,
                        'i' as i32 as libc::c_char,
                        'o' as i32 as libc::c_char,
                        'n' as i32 as libc::c_char,
                        'I' as i32 as libc::c_char,
                        'n' as i32 as libc::c_char,
                        'i' as i32 as libc::c_char,
                        't' as i32 as libc::c_char,
                        'i' as i32 as libc::c_char,
                        'a' as i32 as libc::c_char,
                        'l' as i32 as libc::c_char,
                        'i' as i32 as libc::c_char,
                        'z' as i32 as libc::c_char,
                        'e' as i32 as libc::c_char,
                        'd' as i32 as libc::c_char,
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
                    ],
                    nWaiters: 0 as libc::c_int,
                };
                init
            },
        },
        type_0: EVENT,
    };
    init
};
static mut sig_set: sigset_t = sigset_t { __val: [0; 16] };
static mut sig_thread_id: pthread_t = 0;
static mut sig_monitor_thread_id: pthread_t = 0;
pub static mut HZ: libc::c_long = 0;
pub static mut MAXIMUM_CPU: libc::c_int = 0;
pub static mut g_config: ProcDumpConfiguration = ProcDumpConfiguration {
    ProcessId: 0,
    ProcessName: 0 as *const libc::c_char as *mut libc::c_char,
    SystemInfo: sysinfo {
        uptime: 0,
        loads: [0; 3],
        totalram: 0,
        freeram: 0,
        sharedram: 0,
        bufferram: 0,
        totalswap: 0,
        freeswap: 0,
        procs: 0,
        pad: 0,
        totalhigh: 0,
        freehigh: 0,
        mem_unit: 0,
        _f: [0; 0],
    },
    NumberOfDumpsCollecting: 0,
    NumberOfDumpsCollected: 0,
    bTerminated: false,
    nQuit: 0,
    evtQuit: Handle {
        __annonCompField4: __anonunion____missing_field_name_654135139 {
            event: Event {
                mutex: __anonunion_pthread_mutex_t_335460617 {
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
                cond: __anonunion_pthread_cond_t_951761805 {
                    __data: __pthread_cond_s {
                        __annonCompField1: __anonunion____missing_field_name_920131302 {
                            __wseq: 0,
                        },
                        __annonCompField2: __anonunion____missing_field_name_1003085125 {
                            __g1_start: 0,
                        },
                        __g_refs: [0; 2],
                        __g_size: [0; 2],
                        __g1_orig_size: 0,
                        __wrefs: 0,
                        __g_signals: [0; 2],
                    },
                },
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    bTriggerThenSnoozeCPU: false,
    bTriggerThenSnoozeMemory: false,
    bTriggerThenSnoozeTimer: false,
    CpuThreshold: 0,
    bCpuTriggerBelowValue: false,
    MemoryThreshold: 0,
    bMemoryTriggerBelowValue: false,
    ThresholdSeconds: 0,
    bTimerThreshold: false,
    NumberOfDumpsToCollect: 0,
    WaitingForProcessName: false,
    DiagnosticsLoggingEnabled: false,
    ThreadThreshold: 0,
    FileDescriptorThreshold: 0,
    SignalNumber: 0,
    PollingInterval: 0,
    CoreDumpPath: 0 as *const libc::c_char as *mut libc::c_char,
    CoreDumpName: 0 as *const libc::c_char as *mut libc::c_char,
    nThreads: 0,
    Threads: [0; 3],
    semAvailableDumpSlots: Handle {
        __annonCompField4: __anonunion____missing_field_name_654135139 {
            event: Event {
                mutex: __anonunion_pthread_mutex_t_335460617 {
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
                cond: __anonunion_pthread_cond_t_951761805 {
                    __data: __pthread_cond_s {
                        __annonCompField1: __anonunion____missing_field_name_920131302 {
                            __wseq: 0,
                        },
                        __annonCompField2: __anonunion____missing_field_name_1003085125 {
                            __g1_start: 0,
                        },
                        __g_refs: [0; 2],
                        __g_size: [0; 2],
                        __g1_orig_size: 0,
                        __wrefs: 0,
                        __g_signals: [0; 2],
                    },
                },
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    evtCtrlHandlerCleanupComplete: Handle {
        __annonCompField4: __anonunion____missing_field_name_654135139 {
            event: Event {
                mutex: __anonunion_pthread_mutex_t_335460617 {
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
                cond: __anonunion_pthread_cond_t_951761805 {
                    __data: __pthread_cond_s {
                        __annonCompField1: __anonunion____missing_field_name_920131302 {
                            __wseq: 0,
                        },
                        __annonCompField2: __anonunion____missing_field_name_1003085125 {
                            __g1_start: 0,
                        },
                        __g_refs: [0; 2],
                        __g_size: [0; 2],
                        __g1_orig_size: 0,
                        __wrefs: 0,
                        __g_signals: [0; 2],
                    },
                },
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    evtBannerPrinted: Handle {
        __annonCompField4: __anonunion____missing_field_name_654135139 {
            event: Event {
                mutex: __anonunion_pthread_mutex_t_335460617 {
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
                cond: __anonunion_pthread_cond_t_951761805 {
                    __data: __pthread_cond_s {
                        __annonCompField1: __anonunion____missing_field_name_920131302 {
                            __wseq: 0,
                        },
                        __annonCompField2: __anonunion____missing_field_name_1003085125 {
                            __g1_start: 0,
                        },
                        __g_refs: [0; 2],
                        __g_size: [0; 2],
                        __g1_orig_size: 0,
                        __wrefs: 0,
                        __g_signals: [0; 2],
                    },
                },
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    evtConfigurationPrinted: Handle {
        __annonCompField4: __anonunion____missing_field_name_654135139 {
            event: Event {
                mutex: __anonunion_pthread_mutex_t_335460617 {
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
                cond: __anonunion_pthread_cond_t_951761805 {
                    __data: __pthread_cond_s {
                        __annonCompField1: __anonunion____missing_field_name_920131302 {
                            __wseq: 0,
                        },
                        __annonCompField2: __anonunion____missing_field_name_1003085125 {
                            __g1_start: 0,
                        },
                        __g_refs: [0; 2],
                        __g_size: [0; 2],
                        __g1_orig_size: 0,
                        __wrefs: 0,
                        __g_signals: [0; 2],
                    },
                },
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    evtDebugThreadInitialized: Handle {
        __annonCompField4: __anonunion____missing_field_name_654135139 {
            event: Event {
                mutex: __anonunion_pthread_mutex_t_335460617 {
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
                cond: __anonunion_pthread_cond_t_951761805 {
                    __data: __pthread_cond_s {
                        __annonCompField1: __anonunion____missing_field_name_920131302 {
                            __wseq: 0,
                        },
                        __annonCompField2: __anonunion____missing_field_name_1003085125 {
                            __g1_start: 0,
                        },
                        __g_refs: [0; 2],
                        __g_size: [0; 2],
                        __g1_orig_size: 0,
                        __wrefs: 0,
                        __g_signals: [0; 2],
                    },
                },
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    evtStartMonitoring: Handle {
        __annonCompField4: __anonunion____missing_field_name_654135139 {
            event: Event {
                mutex: __anonunion_pthread_mutex_t_335460617 {
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
                cond: __anonunion_pthread_cond_t_951761805 {
                    __data: __pthread_cond_s {
                        __annonCompField1: __anonunion____missing_field_name_920131302 {
                            __wseq: 0,
                        },
                        __annonCompField2: __anonunion____missing_field_name_1003085125 {
                            __g1_start: 0,
                        },
                        __g_refs: [0; 2],
                        __g_size: [0; 2],
                        __g1_orig_size: 0,
                        __wrefs: 0,
                        __g_signals: [0; 2],
                    },
                },
                bTriggered: false,
                bManualReset: false,
                Name: [0; 64],
                nWaiters: 0,
            },
        },
        type_0: EVENT,
    },
    gcorePid: 0,
};
pub static mut ptrace_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub unsafe extern "C" fn SignalThread(
    mut input: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut config: *mut ProcDumpConfiguration = 0 as *mut ProcDumpConfiguration;
    let mut sig_caught: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    config = input as *mut ProcDumpConfiguration;
    rc = sigwait(
        &mut sig_set as *mut sigset_t as *const sigset_t,
        &mut sig_caught as *mut libc::c_int,
    );
    if rc != 0 as libc::c_int {
        Log(error, b"Failed to wait on signal\0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    match sig_caught {
        2 => {
            SetQuit(config, 1 as libc::c_int);
            if (*config).gcorePid != 2147483647 as libc::c_int {
                Log(info, b"Shutting down gcore\0" as *const u8 as *const libc::c_char);
                rc = kill(-(*config).gcorePid, 9 as libc::c_int);
                if rc != 0 as libc::c_int {
                    Log(
                        error,
                        b"Failed to shutdown gcore.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            if (*config).SignalNumber != -(1 as libc::c_int) {
                pthread_mutex_lock(&mut ptrace_mutex);
                ptrace(
                    PTRACE_DETACH,
                    (*config).ProcessId,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
                pthread_mutex_unlock(&mut ptrace_mutex);
                rc = pthread_cancel(sig_monitor_thread_id);
                if rc != 0 as libc::c_int {
                    Log(
                        error,
                        b"An error occurred while canceling SignalMonitorThread.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    exit(-(1 as libc::c_int));
                }
            }
            Log(info, b"Quit\0" as *const u8 as *const libc::c_char);
        }
        _ => {
            fprintf(
                stderr,
                b"\nUnexpected signal %d\n\0" as *const u8 as *const libc::c_char,
                sig_caught,
            );
        }
    }
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn InitProcDump() {
    let mut tmp: bool = false;
    openlog(
        b"ProcDump\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        (1 as libc::c_int) << 3 as libc::c_int,
    );
    tmp = CheckKernelVersion();
    if tmp as libc::c_int == 0 as libc::c_int {
        Log(
            error,
            b"Kernel version lower than 3.5+.\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    InitProcDumpConfiguration(&mut g_config);
    pthread_mutex_init(
        &mut LoggerLock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    pthread_mutex_init(
        &mut ptrace_mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
}
pub unsafe extern "C" fn ExitProcDump() {
    pthread_mutex_destroy(&mut LoggerLock);
    closelog();
    FreeProcDumpConfiguration(&mut g_config);
}
pub unsafe extern "C" fn InitProcDumpConfiguration(
    mut self_0: *mut ProcDumpConfiguration,
) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_long = 0;
    tmp = WaitForSingleObject(&mut g_evtConfigurationInitialized, 0 as libc::c_int);
    if tmp == 0 as libc::c_int {
        return;
    }
    tmp___0 = sysconf(84 as libc::c_int);
    MAXIMUM_CPU = 100 as libc::c_int * tmp___0 as libc::c_int;
    HZ = sysconf(2 as libc::c_int);
    sysinfo(&mut (*self_0).SystemInfo);
    InitNamedEvent(
        &mut (*self_0).evtCtrlHandlerCleanupComplete.__annonCompField4.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"CtrlHandlerCleanupComplete\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).evtCtrlHandlerCleanupComplete.type_0 = EVENT;
    InitNamedEvent(
        &mut (*self_0).evtBannerPrinted.__annonCompField4.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"BannerPrinted\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).evtBannerPrinted.type_0 = EVENT;
    InitNamedEvent(
        &mut (*self_0).evtConfigurationPrinted.__annonCompField4.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"ConfigurationPrinted\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).evtConfigurationPrinted.type_0 = EVENT;
    InitNamedEvent(
        &mut (*self_0).evtDebugThreadInitialized.__annonCompField4.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"DebugThreadInitialized\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).evtDebugThreadInitialized.type_0 = EVENT;
    InitNamedEvent(
        &mut (*self_0).evtQuit.__annonCompField4.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"Quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).evtQuit.type_0 = EVENT;
    InitNamedEvent(
        &mut (*self_0).evtStartMonitoring.__annonCompField4.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"StartMonitoring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).evtStartMonitoring.type_0 = EVENT;
    sem_init(
        &mut (*self_0).semAvailableDumpSlots.__annonCompField4.semaphore,
        0 as libc::c_int,
        1 as libc::c_uint,
    );
    (*self_0).semAvailableDumpSlots.type_0 = SEMAPHORE;
    (*self_0).ProcessId = 2147483647 as libc::c_int;
    (*self_0).NumberOfDumpsCollected = 0 as libc::c_int;
    (*self_0).NumberOfDumpsToCollect = 1 as libc::c_int;
    (*self_0).CpuThreshold = -(1 as libc::c_int);
    (*self_0).MemoryThreshold = -(1 as libc::c_int);
    (*self_0).ThreadThreshold = -(1 as libc::c_int);
    (*self_0).FileDescriptorThreshold = -(1 as libc::c_int);
    (*self_0).SignalNumber = -(1 as libc::c_int);
    (*self_0).ThresholdSeconds = 10 as libc::c_int;
    (*self_0).bCpuTriggerBelowValue = 0 as libc::c_int != 0;
    (*self_0).bMemoryTriggerBelowValue = 0 as libc::c_int != 0;
    (*self_0).bTimerThreshold = 0 as libc::c_int != 0;
    (*self_0).WaitingForProcessName = 0 as libc::c_int != 0;
    (*self_0).DiagnosticsLoggingEnabled = 0 as libc::c_int != 0;
    (*self_0).gcorePid = 2147483647 as libc::c_int;
    (*self_0).PollingInterval = 1000 as libc::c_int;
    (*self_0).CoreDumpPath = 0 as *mut libc::c_void as *mut libc::c_char;
    (*self_0).CoreDumpName = 0 as *mut libc::c_void as *mut libc::c_char;
    SetEvent(&mut g_evtConfigurationInitialized.__annonCompField4.event);
}
pub unsafe extern "C" fn FreeProcDumpConfiguration(
    mut self_0: *mut ProcDumpConfiguration,
) {
    let mut tmp: libc::c_int = 0;
    DestroyEvent(&mut (*self_0).evtCtrlHandlerCleanupComplete.__annonCompField4.event);
    DestroyEvent(&mut (*self_0).evtBannerPrinted.__annonCompField4.event);
    DestroyEvent(&mut (*self_0).evtConfigurationPrinted.__annonCompField4.event);
    DestroyEvent(&mut (*self_0).evtDebugThreadInitialized.__annonCompField4.event);
    DestroyEvent(&mut (*self_0).evtQuit.__annonCompField4.event);
    DestroyEvent(&mut (*self_0).evtStartMonitoring.__annonCompField4.event);
    sem_destroy(&mut (*self_0).semAvailableDumpSlots.__annonCompField4.semaphore);
    tmp = strcmp(
        (*self_0).ProcessName as *const libc::c_char,
        b"null\0" as *const u8 as *const libc::c_char,
    );
    if tmp != 0 as libc::c_int {
        free((*self_0).ProcessName as *mut libc::c_void);
    }
    free((*self_0).CoreDumpPath as *mut libc::c_void);
    free((*self_0).CoreDumpName as *mut libc::c_void);
}
pub unsafe extern "C" fn GetOptions(
    mut self_0: *mut ProcDumpConfiguration,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut next_option: libc::c_int = 0;
    let mut option_index: libc::c_int = 0;
    let mut short_options: *const libc::c_char = 0 as *const libc::c_char;
    let mut long_options: [option; 16] = [option {
        name: 0 as *const libc::c_char,
        has_arg: 0,
        flag: 0 as *mut libc::c_int,
        val: 0,
    }; 16];
    let mut tempOutputPath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut statbuf: stat = stat {
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
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: bool = false;
    let mut tmp___4: libc::c_int = 0;
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
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: bool = false;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___20: bool = false;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: bool = false;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: libc::c_int = 0;
    let mut tmp___26: bool = false;
    let mut tmp___27: libc::c_int = 0;
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: bool = false;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: bool = false;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___34: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___35: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___36: libc::c_int = 0;
    let mut tmp___37: size_t = 0;
    let mut tmp___38: libc::c_int = 0;
    let mut tmp___39: libc::c_int = 0;
    let mut tmp___40: libc::c_int = 0;
    let mut tmp___41: libc::c_int = 0;
    let mut tmp___42: libc::c_int = 0;
    let mut tmp___43: libc::c_int = 0;
    let mut tmp___44: libc::c_int = 0;
    let mut tmp___45: libc::c_int = 0;
    tmp = WaitForSingleObject(&mut g_evtConfigurationInitialized, 0 as libc::c_int);
    if tmp != 0 as libc::c_int {
        DiagTrace(
            b"GetOptions: Configuration not initialized. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 202\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if argc < 2 as libc::c_int {
        DiagTrace(
            b"GetOptions: Invalid number of command line arguments. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 207\0" as *const u8
                as *const libc::c_char,
        );
        tmp___0 = PrintUsage(self_0);
        return tmp___0;
    }
    option_index = 0 as libc::c_int;
    short_options = b"+p:C:c:M:m:n:s:w:T:F:G:I:o:dh\0" as *const u8
        as *const libc::c_char;
    long_options[0 as libc::c_int as usize]
        .name = b"pid\0" as *const u8 as *const libc::c_char;
    long_options[0 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[0 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[0 as libc::c_int as usize].val = 'p' as i32;
    long_options[1 as libc::c_int as usize]
        .name = b"cpu\0" as *const u8 as *const libc::c_char;
    long_options[1 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[1 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[1 as libc::c_int as usize].val = 'C' as i32;
    long_options[2 as libc::c_int as usize]
        .name = b"lower-cpu\0" as *const u8 as *const libc::c_char;
    long_options[2 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[2 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[2 as libc::c_int as usize].val = 'c' as i32;
    long_options[3 as libc::c_int as usize]
        .name = b"memory\0" as *const u8 as *const libc::c_char;
    long_options[3 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[3 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[3 as libc::c_int as usize].val = 'M' as i32;
    long_options[4 as libc::c_int as usize]
        .name = b"lower-mem\0" as *const u8 as *const libc::c_char;
    long_options[4 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[4 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[4 as libc::c_int as usize].val = 'm' as i32;
    long_options[5 as libc::c_int as usize]
        .name = b"number-of-dumps\0" as *const u8 as *const libc::c_char;
    long_options[5 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[5 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[5 as libc::c_int as usize].val = 'n' as i32;
    long_options[6 as libc::c_int as usize]
        .name = b"time-between-dumps\0" as *const u8 as *const libc::c_char;
    long_options[6 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[6 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[6 as libc::c_int as usize].val = 's' as i32;
    long_options[7 as libc::c_int as usize]
        .name = b"wait\0" as *const u8 as *const libc::c_char;
    long_options[7 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[7 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[7 as libc::c_int as usize].val = 'w' as i32;
    long_options[8 as libc::c_int as usize]
        .name = b"threads\0" as *const u8 as *const libc::c_char;
    long_options[8 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[8 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[8 as libc::c_int as usize].val = 'T' as i32;
    long_options[9 as libc::c_int as usize]
        .name = b"filedescriptors\0" as *const u8 as *const libc::c_char;
    long_options[9 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[9 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[9 as libc::c_int as usize].val = 'F' as i32;
    long_options[10 as libc::c_int as usize]
        .name = b"signal\0" as *const u8 as *const libc::c_char;
    long_options[10 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[10 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[10 as libc::c_int as usize].val = 'G' as i32;
    long_options[11 as libc::c_int as usize]
        .name = b"pollinginterval\0" as *const u8 as *const libc::c_char;
    long_options[11 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[11 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[11 as libc::c_int as usize].val = 'I' as i32;
    long_options[12 as libc::c_int as usize]
        .name = b"output-path\0" as *const u8 as *const libc::c_char;
    long_options[12 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    long_options[12 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[12 as libc::c_int as usize].val = 'o' as i32;
    long_options[13 as libc::c_int as usize]
        .name = b"diag\0" as *const u8 as *const libc::c_char;
    long_options[13 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[13 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[13 as libc::c_int as usize].val = 'd' as i32;
    long_options[14 as libc::c_int as usize]
        .name = b"help\0" as *const u8 as *const libc::c_char;
    long_options[14 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[14 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[14 as libc::c_int as usize].val = 'h' as i32;
    long_options[15 as libc::c_int as usize]
        .name = 0 as *mut libc::c_void as *const libc::c_char;
    long_options[15 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    long_options[15 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    long_options[15 as libc::c_int as usize].val = 0 as libc::c_int;
    tempOutputPath = 0 as *mut libc::c_void as *mut libc::c_char;
    loop {
        next_option = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            short_options,
            long_options.as_mut_ptr() as *const option,
            &mut option_index,
        );
        if !(next_option != -(1 as libc::c_int)) {
            break;
        }
        match next_option {
            112 => {
                tmp___1 = atoi(optarg as *const libc::c_char);
                (*self_0).ProcessId = tmp___1;
                tmp___3 = LookupProcessByPid(self_0);
                if !tmp___3 {
                    Log(
                        error,
                        b"Invalid PID - failed looking up process name by PID.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    tmp___2 = PrintUsage(self_0);
                    return tmp___2;
                }
            }
            67 => {
                if (*self_0).CpuThreshold != -(1 as libc::c_int) {
                    Log(
                        error,
                        b"Invalid CPU threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    tmp___4 = PrintUsage(self_0);
                    return tmp___4;
                } else {
                    tmp___5 = IsValidNumberArg(optarg as *const libc::c_char);
                    if tmp___5 {
                        tmp___6 = atoi(optarg as *const libc::c_char);
                        (*self_0).CpuThreshold = tmp___6;
                        if tmp___6 < 0 as libc::c_int {
                            Log(
                                error,
                                b"Invalid CPU threshold specified.\0" as *const u8
                                    as *const libc::c_char,
                            );
                            tmp___4 = PrintUsage(self_0);
                            return tmp___4;
                        } else {
                            if (*self_0).CpuThreshold > MAXIMUM_CPU {
                                Log(
                                    error,
                                    b"Invalid CPU threshold specified.\0" as *const u8
                                        as *const libc::c_char,
                                );
                                tmp___4 = PrintUsage(self_0);
                                return tmp___4;
                            }
                        }
                    } else {
                        Log(
                            error,
                            b"Invalid CPU threshold specified.\0" as *const u8
                                as *const libc::c_char,
                        );
                        tmp___4 = PrintUsage(self_0);
                        return tmp___4;
                    }
                }
            }
            73 => {
                tmp___8 = IsValidNumberArg(optarg as *const libc::c_char);
                if tmp___8 {
                    tmp___9 = atoi(optarg as *const libc::c_char);
                    (*self_0).PollingInterval = tmp___9;
                    if tmp___9 < 0 as libc::c_int {
                        Log(
                            error,
                            b"Invalid polling interval specified (minimum %d).\0"
                                as *const u8 as *const libc::c_char,
                            1000 as libc::c_int,
                        );
                        tmp___7 = PrintUsage(self_0);
                        return tmp___7;
                    } else {
                        if (*self_0).PollingInterval < 1000 as libc::c_int {
                            Log(
                                error,
                                b"Invalid polling interval specified (minimum %d).\0"
                                    as *const u8 as *const libc::c_char,
                                1000 as libc::c_int,
                            );
                            tmp___7 = PrintUsage(self_0);
                            return tmp___7;
                        }
                    }
                } else {
                    Log(
                        error,
                        b"Invalid polling interval specified (minimum %d).\0"
                            as *const u8 as *const libc::c_char,
                        1000 as libc::c_int,
                    );
                    tmp___7 = PrintUsage(self_0);
                    return tmp___7;
                }
            }
            84 => {
                if (*self_0).ThreadThreshold != -(1 as libc::c_int) {
                    Log(
                        error,
                        b"Invalid threads threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    tmp___10 = PrintUsage(self_0);
                    return tmp___10;
                } else {
                    tmp___11 = IsValidNumberArg(optarg as *const libc::c_char);
                    if tmp___11 {
                        tmp___12 = atoi(optarg as *const libc::c_char);
                        (*self_0).ThreadThreshold = tmp___12;
                        if tmp___12 < 0 as libc::c_int {
                            Log(
                                error,
                                b"Invalid threads threshold specified.\0" as *const u8
                                    as *const libc::c_char,
                            );
                            tmp___10 = PrintUsage(self_0);
                            return tmp___10;
                        }
                    } else {
                        Log(
                            error,
                            b"Invalid threads threshold specified.\0" as *const u8
                                as *const libc::c_char,
                        );
                        tmp___10 = PrintUsage(self_0);
                        return tmp___10;
                    }
                }
            }
            70 => {
                if (*self_0).FileDescriptorThreshold != -(1 as libc::c_int) {
                    Log(
                        error,
                        b"Invalid file descriptor threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    tmp___13 = PrintUsage(self_0);
                    return tmp___13;
                } else {
                    tmp___14 = IsValidNumberArg(optarg as *const libc::c_char);
                    if tmp___14 {
                        tmp___15 = atoi(optarg as *const libc::c_char);
                        (*self_0).FileDescriptorThreshold = tmp___15;
                        if tmp___15 < 0 as libc::c_int {
                            Log(
                                error,
                                b"Invalid file descriptor threshold specified.\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            tmp___13 = PrintUsage(self_0);
                            return tmp___13;
                        }
                    } else {
                        Log(
                            error,
                            b"Invalid file descriptor threshold specified.\0"
                                as *const u8 as *const libc::c_char,
                        );
                        tmp___13 = PrintUsage(self_0);
                        return tmp___13;
                    }
                }
            }
            71 => {
                if (*self_0).SignalNumber != -(1 as libc::c_int) {
                    Log(
                        error,
                        b"Invalid signal specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    tmp___16 = PrintUsage(self_0);
                    return tmp___16;
                } else {
                    tmp___17 = IsValidNumberArg(optarg as *const libc::c_char);
                    if tmp___17 {
                        tmp___18 = atoi(optarg as *const libc::c_char);
                        (*self_0).SignalNumber = tmp___18;
                        if tmp___18 < 0 as libc::c_int {
                            Log(
                                error,
                                b"Invalid signal specified.\0" as *const u8
                                    as *const libc::c_char,
                            );
                            tmp___16 = PrintUsage(self_0);
                            return tmp___16;
                        }
                    } else {
                        Log(
                            error,
                            b"Invalid signal specified.\0" as *const u8
                                as *const libc::c_char,
                        );
                        tmp___16 = PrintUsage(self_0);
                        return tmp___16;
                    }
                }
            }
            99 => {
                if (*self_0).CpuThreshold != -(1 as libc::c_int) {
                    Log(
                        error,
                        b"Invalid CPU threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    tmp___19 = PrintUsage(self_0);
                    return tmp___19;
                } else {
                    tmp___20 = IsValidNumberArg(optarg as *const libc::c_char);
                    if tmp___20 {
                        tmp___21 = atoi(optarg as *const libc::c_char);
                        (*self_0).CpuThreshold = tmp___21;
                        if tmp___21 < 0 as libc::c_int {
                            Log(
                                error,
                                b"Invalid CPU threshold specified.\0" as *const u8
                                    as *const libc::c_char,
                            );
                            tmp___19 = PrintUsage(self_0);
                            return tmp___19;
                        } else {
                            if (*self_0).CpuThreshold > MAXIMUM_CPU {
                                Log(
                                    error,
                                    b"Invalid CPU threshold specified.\0" as *const u8
                                        as *const libc::c_char,
                                );
                                tmp___19 = PrintUsage(self_0);
                                return tmp___19;
                            }
                        }
                    } else {
                        Log(
                            error,
                            b"Invalid CPU threshold specified.\0" as *const u8
                                as *const libc::c_char,
                        );
                        tmp___19 = PrintUsage(self_0);
                        return tmp___19;
                    }
                }
                (*self_0).bCpuTriggerBelowValue = 1 as libc::c_int != 0;
            }
            77 => {
                if (*self_0).MemoryThreshold != -(1 as libc::c_int) {
                    Log(
                        error,
                        b"Invalid memory threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    tmp___22 = PrintUsage(self_0);
                    return tmp___22;
                } else {
                    tmp___23 = IsValidNumberArg(optarg as *const libc::c_char);
                    if tmp___23 {
                        tmp___24 = atoi(optarg as *const libc::c_char);
                        (*self_0).MemoryThreshold = tmp___24;
                        if tmp___24 < 0 as libc::c_int {
                            Log(
                                error,
                                b"Invalid memory threshold specified.\0" as *const u8
                                    as *const libc::c_char,
                            );
                            tmp___22 = PrintUsage(self_0);
                            return tmp___22;
                        }
                    } else {
                        Log(
                            error,
                            b"Invalid memory threshold specified.\0" as *const u8
                                as *const libc::c_char,
                        );
                        tmp___22 = PrintUsage(self_0);
                        return tmp___22;
                    }
                }
            }
            109 => {
                if (*self_0).MemoryThreshold != -(1 as libc::c_int) {
                    Log(
                        error,
                        b"Invalid memory threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    tmp___25 = PrintUsage(self_0);
                    return tmp___25;
                } else {
                    tmp___26 = IsValidNumberArg(optarg as *const libc::c_char);
                    if tmp___26 {
                        tmp___27 = atoi(optarg as *const libc::c_char);
                        (*self_0).MemoryThreshold = tmp___27;
                        if tmp___27 < 0 as libc::c_int {
                            Log(
                                error,
                                b"Invalid memory threshold specified.\0" as *const u8
                                    as *const libc::c_char,
                            );
                            tmp___25 = PrintUsage(self_0);
                            return tmp___25;
                        }
                    } else {
                        Log(
                            error,
                            b"Invalid memory threshold specified.\0" as *const u8
                                as *const libc::c_char,
                        );
                        tmp___25 = PrintUsage(self_0);
                        return tmp___25;
                    }
                }
                (*self_0).bMemoryTriggerBelowValue = 1 as libc::c_int != 0;
            }
            110 => {
                tmp___29 = IsValidNumberArg(optarg as *const libc::c_char);
                if tmp___29 {
                    tmp___30 = atoi(optarg as *const libc::c_char);
                    (*self_0).NumberOfDumpsToCollect = tmp___30;
                    if tmp___30 < 0 as libc::c_int {
                        Log(
                            error,
                            b"Invalid dumps threshold specified.\0" as *const u8
                                as *const libc::c_char,
                        );
                        tmp___28 = PrintUsage(self_0);
                        return tmp___28;
                    }
                } else {
                    Log(
                        error,
                        b"Invalid dumps threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    tmp___28 = PrintUsage(self_0);
                    return tmp___28;
                }
            }
            115 => {
                tmp___32 = IsValidNumberArg(optarg as *const libc::c_char);
                if tmp___32 {
                    tmp___33 = atoi(optarg as *const libc::c_char);
                    (*self_0).ThresholdSeconds = tmp___33;
                    if tmp___33 == 0 as libc::c_int {
                        Log(
                            error,
                            b"Invalid time threshold specified.\0" as *const u8
                                as *const libc::c_char,
                        );
                        tmp___31 = PrintUsage(self_0);
                        return tmp___31;
                    }
                } else {
                    Log(
                        error,
                        b"Invalid time threshold specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    tmp___31 = PrintUsage(self_0);
                    return tmp___31;
                }
            }
            119 => {
                (*self_0).WaitingForProcessName = 1 as libc::c_int != 0;
                (*self_0).ProcessName = strdup(optarg as *const libc::c_char);
            }
            111 => {
                tempOutputPath = strdup(optarg as *const libc::c_char);
                tmp___36 = stat(
                    tempOutputPath as *const libc::c_char,
                    &mut statbuf as *mut stat,
                );
                let mut current_block_304: u64;
                if tmp___36 == 0 as libc::c_int {
                    if statbuf.st_mode & 61440 as libc::c_uint == 16384 as libc::c_uint {
                        (*self_0).CoreDumpPath = tempOutputPath;
                        (*self_0)
                            .CoreDumpName = 0 as *mut libc::c_void as *mut libc::c_char;
                        current_block_304 = 7539340762994337730;
                    } else {
                        current_block_304 = 9460471516650888218;
                    }
                } else {
                    current_block_304 = 9460471516650888218;
                }
                match current_block_304 {
                    9460471516650888218 => {
                        tmp___37 = strlen(tempOutputPath as *const libc::c_char);
                        if *tempOutputPath
                            .offset(tmp___37.wrapping_sub(1 as libc::c_ulong) as isize)
                            as libc::c_int == 47 as libc::c_int
                        {
                            (*self_0).CoreDumpPath = tempOutputPath;
                            (*self_0)
                                .CoreDumpName = 0 as *mut libc::c_void as *mut libc::c_char;
                        } else {
                            tmp___34 = dirname(tempOutputPath);
                            (*self_0)
                                .CoreDumpPath = strdup(tmp___34 as *const libc::c_char);
                            free(tempOutputPath as *mut libc::c_void);
                            tempOutputPath = strdup(optarg as *const libc::c_char);
                            tmp___35 = __xpg_basename(tempOutputPath);
                            (*self_0)
                                .CoreDumpName = strdup(tmp___35 as *const libc::c_char);
                            free(tempOutputPath as *mut libc::c_void);
                        }
                    }
                    _ => {}
                }
                tmp___39 = stat(
                    (*self_0).CoreDumpPath as *const libc::c_char,
                    &mut statbuf as *mut stat,
                );
                if tmp___39 < 0 as libc::c_int {
                    Log(
                        error,
                        b"Invalid directory (\"%s\") provided for core dump output.\0"
                            as *const u8 as *const libc::c_char,
                        (*self_0).CoreDumpPath,
                    );
                    tmp___38 = PrintUsage(self_0);
                    return tmp___38;
                } else {
                    if !(statbuf.st_mode & 61440 as libc::c_uint
                        == 16384 as libc::c_uint)
                    {
                        Log(
                            error,
                            b"Invalid directory (\"%s\") provided for core dump output.\0"
                                as *const u8 as *const libc::c_char,
                            (*self_0).CoreDumpPath,
                        );
                        tmp___38 = PrintUsage(self_0);
                        return tmp___38;
                    }
                }
            }
            100 => {
                (*self_0).DiagnosticsLoggingEnabled = 1 as libc::c_int != 0;
            }
            104 => {
                tmp___40 = PrintUsage(self_0);
                return tmp___40;
            }
            _ => {
                Log(
                    error,
                    b"Invalid switch specified\0" as *const u8 as *const libc::c_char,
                );
                tmp___41 = PrintUsage(self_0);
                return tmp___41;
            }
        }
    }
    if (*self_0).CoreDumpPath as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
    {
        (*self_0).CoreDumpPath = strdup(b".\0" as *const u8 as *const libc::c_char);
    }
    if (*self_0).NumberOfDumpsToCollect != -(1 as libc::c_int) {
        if (*self_0).MemoryThreshold == -(1 as libc::c_int) {
            if (*self_0).CpuThreshold == -(1 as libc::c_int) {
                if (*self_0).ThreadThreshold == -(1 as libc::c_int) {
                    if (*self_0).FileDescriptorThreshold == -(1 as libc::c_int) {
                        (*self_0).bTimerThreshold = 1 as libc::c_int != 0;
                    }
                }
            }
        }
    }
    if (*self_0).SignalNumber != -(1 as libc::c_int) {
        if (*self_0).CpuThreshold != -(1 as libc::c_int) {
            Log(
                error,
                b"Signal trigger must be the only trigger specified.\0" as *const u8
                    as *const libc::c_char,
            );
            tmp___42 = PrintUsage(self_0);
            return tmp___42;
        } else {
            if (*self_0).ThreadThreshold != -(1 as libc::c_int) {
                Log(
                    error,
                    b"Signal trigger must be the only trigger specified.\0" as *const u8
                        as *const libc::c_char,
                );
                tmp___42 = PrintUsage(self_0);
                return tmp___42;
            } else {
                if (*self_0).FileDescriptorThreshold != -(1 as libc::c_int) {
                    Log(
                        error,
                        b"Signal trigger must be the only trigger specified.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    tmp___42 = PrintUsage(self_0);
                    return tmp___42;
                } else {
                    if (*self_0).MemoryThreshold != -(1 as libc::c_int) {
                        Log(
                            error,
                            b"Signal trigger must be the only trigger specified.\0"
                                as *const u8 as *const libc::c_char,
                        );
                        tmp___42 = PrintUsage(self_0);
                        return tmp___42;
                    }
                }
            }
        }
        if (*self_0).PollingInterval != 1000 as libc::c_int {
            Log(
                error,
                b"Polling interval has no meaning during signal monitoring.\0"
                    as *const u8 as *const libc::c_char,
            );
            tmp___43 = PrintUsage(self_0);
            return tmp___43;
        }
        (*self_0).bTimerThreshold = 0 as libc::c_int != 0;
    }
    if (*self_0).ProcessId == 2147483647 as libc::c_int {
        if !(*self_0).WaitingForProcessName {
            Log(
                error,
                b"A valid PID or process name must be specified\0" as *const u8
                    as *const libc::c_char,
            );
            tmp___44 = PrintUsage(self_0);
            return tmp___44;
        }
    }
    if (*self_0).ProcessId != 2147483647 as libc::c_int {
        if (*self_0).WaitingForProcessName {
            Log(
                error,
                b"Please only specify one of -p or -w\0" as *const u8
                    as *const libc::c_char,
            );
            tmp___45 = PrintUsage(self_0);
            return tmp___45;
        }
    }
    if !(*self_0).WaitingForProcessName {
        (*self_0).ProcessName = GetProcessName((*self_0).ProcessId);
    }
    DiagTrace(
        b"GetOpts and initial Configuration finished %s\0" as *const u8
            as *const libc::c_char,
        b"in src/ProcDumpConfiguration.c, at line 429\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn LookupProcessByPid(
    mut self_0: *mut ProcDumpConfiguration,
) -> bool {
    let mut statFilePath: [libc::c_char; 32] = [0; 32];
    let mut fd: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut FILE = 0 as *mut FILE;
    sprintf(
        statFilePath.as_mut_ptr(),
        b"/proc/%d/stat\0" as *const u8 as *const libc::c_char,
        (*self_0).ProcessId,
    );
    tmp = fopen(
        statFilePath.as_mut_ptr() as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    fd = tmp;
    if fd as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        Log(
            error,
            b"No process matching the specified PID can be found.\0" as *const u8
                as *const libc::c_char,
        );
        Log(
            error,
            b"Try elevating the command prompt (i.e., `sudo procdump ...`)\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    fclose(fd);
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn FilterForPid(mut entry: *const dirent) -> libc::c_int {
    let mut tmp: bool = false;
    tmp = IsValidNumberArg(((*entry).d_name).as_ptr());
    return tmp as libc::c_int;
}
pub unsafe extern "C" fn WaitForProcessName(
    mut self_0: *mut ProcDumpConfiguration,
) -> bool {
    let mut nameList: *mut *mut dirent = 0 as *mut *mut dirent;
    let mut moreThanOne: bool = false;
    let mut matchingPid: pid_t = 0;
    let mut numEntries: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut procPid: pid_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut nameForPid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    Log(
        info,
        b"Waiting for process '%s' to launch...\0" as *const u8 as *const libc::c_char,
        (*self_0).ProcessName,
    );
    loop {
        moreThanOne = 0 as libc::c_int != 0;
        matchingPid = 2147483647 as libc::c_int;
        tmp = scandir(
            b"/proc/\0" as *const u8 as *const libc::c_char,
            &mut nameList as *mut *mut *mut dirent,
            Some(FilterForPid as unsafe extern "C" fn(*const dirent) -> libc::c_int),
            Some(
                alphasort
                    as unsafe extern "C" fn(
                        *mut *const dirent,
                        *mut *const dirent,
                    ) -> libc::c_int,
            ),
        );
        numEntries = tmp;
        i = 0 as libc::c_int;
        while i < numEntries {
            tmp___0 = atoi(
                ((**nameList.offset(i as isize)).d_name).as_mut_ptr()
                    as *const libc::c_char,
            );
            procPid = tmp___0;
            tmp___1 = GetProcessName(procPid);
            nameForPid = tmp___1;
            tmp___2 = strcmp(
                nameForPid as *const libc::c_char,
                b"null\0" as *const u8 as *const libc::c_char,
            );
            if !(tmp___2 == 0 as libc::c_int) {
                tmp___3 = strcmp(
                    nameForPid as *const libc::c_char,
                    (*self_0).ProcessName as *const libc::c_char,
                );
                if tmp___3 == 0 as libc::c_int {
                    if matchingPid == 2147483647 as libc::c_int {
                        matchingPid = procPid;
                    } else {
                        Log(
                            error,
                            b"More than one matching process found, exiting...\0"
                                as *const u8 as *const libc::c_char,
                        );
                        moreThanOne = 1 as libc::c_int != 0;
                        free(nameForPid as *mut libc::c_void);
                        break;
                    }
                }
                free(nameForPid as *mut libc::c_void);
            }
            i += 1;
        }
        i___0 = 0 as libc::c_int;
        while i___0 < numEntries {
            free(*nameList.offset(i___0 as isize) as *mut libc::c_void);
            i___0 += 1;
        }
        free(nameList as *mut libc::c_void);
        if moreThanOne {
            (*self_0).bTerminated = 1 as libc::c_int != 0;
            return 0 as libc::c_int != 0;
        } else {
            if matchingPid != 2147483647 as libc::c_int {
                (*self_0).ProcessId = matchingPid;
                Log(
                    info,
                    b"Found process with PID %d\0" as *const u8 as *const libc::c_char,
                    matchingPid,
                );
                return 1 as libc::c_int != 0;
            }
        }
    };
}
pub unsafe extern "C" fn GetProcessName(mut pid: pid_t) -> *mut libc::c_char {
    let mut procFilePath: [libc::c_char; 32] = [0; 32];
    let mut fileBuffer: [libc::c_char; 4097] = [0; 4097];
    let mut charactersRead: libc::c_int = 0;
    let mut itr: libc::c_int = 0;
    let mut stringItr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut processName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut procFile: *mut FILE = 0 as *mut FILE;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    charactersRead = 0 as libc::c_int;
    itr = 0 as libc::c_int;
    tmp = sprintf(
        procFilePath.as_mut_ptr(),
        b"/proc/%d/cmdline\0" as *const u8 as *const libc::c_char,
        pid,
    );
    if tmp < 0 as libc::c_int {
        return b"null\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    procFile = fopen(
        procFilePath.as_mut_ptr() as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if procFile as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = fgets(fileBuffer.as_mut_ptr(), 4097 as libc::c_int, procFile);
        if tmp___1 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            fclose(procFile);
            tmp___0 = strlen(fileBuffer.as_mut_ptr() as *const libc::c_char);
            if tmp___0 == 0 as libc::c_ulong {
                Log(debug, b"Empty cmdline.\n\0" as *const u8 as *const libc::c_char);
            } else {
                Log(
                    debug,
                    b"Failed to read from %s.\n\0" as *const u8 as *const libc::c_char,
                    procFilePath.as_mut_ptr(),
                );
            }
            return b"null\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        fclose(procFile);
    } else {
        Log(
            debug,
            b"Failed to open %s.\n\0" as *const u8 as *const libc::c_char,
            procFilePath.as_mut_ptr(),
        );
        return b"null\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    stringItr = fileBuffer.as_mut_ptr();
    tmp___2 = strlen(fileBuffer.as_mut_ptr() as *const libc::c_char);
    charactersRead = tmp___2 as libc::c_int;
    i = 0 as libc::c_int;
    while i <= charactersRead {
        if fileBuffer[i as usize] as libc::c_int == 0 as libc::c_int {
            itr = i - itr;
            tmp___5 = strcmp(
                stringItr as *const libc::c_char,
                b"sudo\0" as *const u8 as *const libc::c_char,
            );
            if tmp___5 != 0 as libc::c_int {
                processName = strrchr(stringItr as *const libc::c_char, '/' as i32);
                if processName as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    tmp___3 = strdup(
                        processName.offset(1 as libc::c_int as isize)
                            as *const libc::c_char,
                    );
                    return tmp___3;
                } else {
                    tmp___4 = strdup(stringItr as *const libc::c_char);
                    return tmp___4;
                }
            } else {
                stringItr = stringItr.offset((itr + 1 as libc::c_int) as isize);
            }
        }
        i += 1;
    }
    Log(
        debug,
        b"Failed to extract process name from /proc/PID/cmdline\0" as *const u8
            as *const libc::c_char,
    );
    return b"null\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
pub unsafe extern "C" fn CreateTriggerThreads(
    mut self_0: *mut ProcDumpConfiguration,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    rc = 0 as libc::c_int;
    (*self_0).nThreads = 0 as libc::c_int;
    rc = sigemptyset(&mut sig_set);
    if rc < 0 as libc::c_int {
        DiagTrace(
            b"CreateTriggerThreads: sigemptyset failed. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 598\0" as *const u8
                as *const libc::c_char,
        );
        return rc;
    }
    rc = sigaddset(&mut sig_set, 2 as libc::c_int);
    if rc < 0 as libc::c_int {
        DiagTrace(
            b"CreateTriggerThreads: sigaddset failed. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 603\0" as *const u8
                as *const libc::c_char,
        );
        return rc;
    }
    rc = sigaddset(&mut sig_set, 15 as libc::c_int);
    if rc < 0 as libc::c_int {
        DiagTrace(
            b"CreateTriggerThreads: sigaddset failed. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 608\0" as *const u8
                as *const libc::c_char,
        );
        return rc;
    }
    rc = pthread_sigmask(
        0 as libc::c_int,
        &mut sig_set as *mut sigset_t as *const __sigset_t,
        0 as *mut libc::c_void as *mut __sigset_t,
    );
    if rc != 0 as libc::c_int {
        DiagTrace(
            b"CreateTriggerThreads: pthread_sigmask failed. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 614\0" as *const u8
                as *const libc::c_char,
        );
        return rc;
    }
    if (*self_0).CpuThreshold != -(1 as libc::c_int) {
        tmp = (*self_0).nThreads;
        (*self_0).nThreads += 1;
        rc = pthread_create(
            &mut *((*self_0).Threads).as_mut_ptr().offset(tmp as isize)
                as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                CpuMonitoringThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create CpuThread. %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 621\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    if (*self_0).MemoryThreshold != -(1 as libc::c_int) {
        tmp___0 = (*self_0).nThreads;
        (*self_0).nThreads += 1;
        rc = pthread_create(
            &mut *((*self_0).Threads).as_mut_ptr().offset(tmp___0 as isize)
                as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                CommitMonitoringThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create CommitThread. %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 628\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    if (*self_0).ThreadThreshold != -(1 as libc::c_int) {
        tmp___1 = (*self_0).nThreads;
        (*self_0).nThreads += 1;
        rc = pthread_create(
            &mut *((*self_0).Threads).as_mut_ptr().offset(tmp___1 as isize)
                as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                ThreadCountMonitoringThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create ThreadThread. %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 635\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    if (*self_0).FileDescriptorThreshold != -(1 as libc::c_int) {
        tmp___2 = (*self_0).nThreads;
        (*self_0).nThreads += 1;
        rc = pthread_create(
            &mut *((*self_0).Threads).as_mut_ptr().offset(tmp___2 as isize)
                as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                FileDescriptorCountMonitoringThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create FileDescriptorThread. %s\0"
                    as *const u8 as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 642\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    if (*self_0).SignalNumber != -(1 as libc::c_int) {
        rc = pthread_create(
            &mut sig_monitor_thread_id as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                SignalMonitoringThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create SignalMonitoringThread. %s\0"
                    as *const u8 as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 649\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    if (*self_0).bTimerThreshold {
        tmp___3 = (*self_0).nThreads;
        (*self_0).nThreads += 1;
        rc = pthread_create(
            &mut *((*self_0).Threads).as_mut_ptr().offset(tmp___3 as isize)
                as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                TimerThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            self_0 as *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            DiagTrace(
                b"CreateTriggerThreads: failed to create TimerThread. %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/ProcDumpConfiguration.c, at line 656\0" as *const u8
                    as *const libc::c_char,
            );
            return rc;
        }
    }
    rc = pthread_create(
        &mut sig_thread_id as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            SignalThread as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        self_0 as *mut libc::c_void,
    );
    if rc != 0 as libc::c_int {
        DiagTrace(
            b"CreateTriggerThreads: failed to create SignalThread. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/ProcDumpConfiguration.c, at line 663\0" as *const u8
                as *const libc::c_char,
        );
        return rc;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn WaitForQuit(
    mut self_0: *mut ProcDumpConfiguration,
    mut milliseconds: libc::c_int,
) -> libc::c_int {
    let mut tmp: bool = false;
    let mut wait___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    tmp = ContinueMonitoring(self_0);
    if !tmp {
        return 128 as libc::c_int;
    }
    tmp___0 = WaitForSingleObject(&mut (*self_0).evtQuit, milliseconds);
    wait___0 = tmp___0;
    if wait___0 == 110 as libc::c_int {
        tmp___1 = ContinueMonitoring(self_0);
        if !tmp___1 {
            return 128 as libc::c_int;
        }
    }
    return wait___0;
}
pub unsafe extern "C" fn WaitForQuitOrEvent(
    mut self_0: *mut ProcDumpConfiguration,
    mut handle: *mut Handle,
    mut milliseconds: libc::c_int,
) -> libc::c_int {
    let mut waits: [*mut Handle; 2] = [0 as *mut Handle; 2];
    let mut tmp: bool = false;
    let mut wait___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: bool = false;
    let mut tmp___2: bool = false;
    waits[0 as libc::c_int as usize] = &mut (*self_0).evtQuit;
    waits[1 as libc::c_int as usize] = handle;
    tmp = ContinueMonitoring(self_0);
    if !tmp {
        return 128 as libc::c_int;
    }
    tmp___0 = WaitForMultipleObjects(
        2 as libc::c_int,
        waits.as_mut_ptr(),
        0 as libc::c_int != 0,
        milliseconds,
    );
    wait___0 = tmp___0;
    if wait___0 == 110 as libc::c_int {
        tmp___1 = ContinueMonitoring(self_0);
        if !tmp___1 {
            return 128 as libc::c_int;
        }
    }
    if wait___0 == 0 as libc::c_int {
        tmp___2 = ContinueMonitoring(self_0);
        if !tmp___2 {
            return 128 as libc::c_int;
        }
    }
    return wait___0;
}
pub unsafe extern "C" fn WaitForAllThreadsToTerminate(
    mut self_0: *mut ProcDumpConfiguration,
) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    rc = 0 as libc::c_int;
    if (*self_0).SignalNumber != -(1 as libc::c_int) {
        rc = pthread_join(
            sig_monitor_thread_id,
            0 as *mut libc::c_void as *mut *mut libc::c_void,
        );
        if rc != 0 as libc::c_int {
            Log(
                error,
                b"An error occurred while joining SignalMonitorThread.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*self_0).nThreads {
            rc = pthread_join(
                (*self_0).Threads[i as usize],
                0 as *mut libc::c_void as *mut *mut libc::c_void,
            );
            if rc != 0 as libc::c_int {
                Log(
                    error,
                    b"An error occurred while joining threads\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            i += 1;
        }
    }
    pthread_cancel(sig_thread_id);
    rc = pthread_join(sig_thread_id, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    if rc != 0 as libc::c_int {
        Log(
            error,
            b"An error occurred while joining SignalThread.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    return rc;
}
pub unsafe extern "C" fn IsQuit(mut self_0: *mut ProcDumpConfiguration) -> bool {
    return (*self_0).nQuit != 0 as libc::c_int;
}
pub unsafe extern "C" fn SetQuit(
    mut self_0: *mut ProcDumpConfiguration,
    mut quit: libc::c_int,
) -> libc::c_int {
    (*self_0).nQuit = quit;
    SetEvent(&mut (*self_0).evtQuit.__annonCompField4.event);
    return (*self_0).nQuit;
}
pub unsafe extern "C" fn PrintConfiguration(
    mut self_0: *mut ProcDumpConfiguration,
) -> bool {
    let mut tmp: libc::c_int = 0;
    tmp = WaitForSingleObject(&mut (*self_0).evtConfigurationPrinted, 0 as libc::c_int);
    if tmp == 110 as libc::c_int {
        if (*self_0).SignalNumber != -(1 as libc::c_int) {
            printf(
                b"** NOTE ** Signal triggers use PTRACE which will impact the performance of the target process\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        printf(
            b"Process:\t\t%s\0" as *const u8 as *const libc::c_char,
            (*self_0).ProcessName,
        );
        if !(*self_0).WaitingForProcessName {
            printf(b" (%d)\0" as *const u8 as *const libc::c_char, (*self_0).ProcessId);
        } else {
            printf(b" (pending)\0" as *const u8 as *const libc::c_char);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        if (*self_0).CpuThreshold != -(1 as libc::c_int) {
            if (*self_0).bCpuTriggerBelowValue {
                printf(
                    b"CPU Threshold:\t\t<%d\n\0" as *const u8 as *const libc::c_char,
                    (*self_0).CpuThreshold,
                );
            } else {
                printf(
                    b"CPU Threshold:\t\t>=%d\n\0" as *const u8 as *const libc::c_char,
                    (*self_0).CpuThreshold,
                );
            }
        } else {
            printf(b"CPU Threshold:\t\tn/a\n\0" as *const u8 as *const libc::c_char);
        }
        if (*self_0).MemoryThreshold != -(1 as libc::c_int) {
            if (*self_0).bMemoryTriggerBelowValue {
                printf(
                    b"Commit Threshold:\t<%d\n\0" as *const u8 as *const libc::c_char,
                    (*self_0).MemoryThreshold,
                );
            } else {
                printf(
                    b"Commit Threshold:\t>=%d\n\0" as *const u8 as *const libc::c_char,
                    (*self_0).MemoryThreshold,
                );
            }
        } else {
            printf(b"Commit Threshold:\tn/a\n\0" as *const u8 as *const libc::c_char);
        }
        if (*self_0).ThreadThreshold != -(1 as libc::c_int) {
            printf(
                b"Thread Threshold:\t>=%d\n\0" as *const u8 as *const libc::c_char,
                (*self_0).ThreadThreshold,
            );
        } else {
            printf(b"Thread Threshold:\t\tn/a\n\0" as *const u8 as *const libc::c_char);
        }
        if (*self_0).FileDescriptorThreshold != -(1 as libc::c_int) {
            printf(
                b"File descriptor Threshold:\t>=%d\n\0" as *const u8
                    as *const libc::c_char,
                (*self_0).FileDescriptorThreshold,
            );
        } else {
            printf(
                b"File descriptor Threshold:\t\tn/a\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*self_0).SignalNumber != -(1 as libc::c_int) {
            printf(
                b"Signal number:\t%d\n\0" as *const u8 as *const libc::c_char,
                (*self_0).SignalNumber,
            );
        } else {
            printf(b"Signal:\t\tn/a\n\0" as *const u8 as *const libc::c_char);
        }
        printf(
            b"Polling interval (ms):\t%d\n\0" as *const u8 as *const libc::c_char,
            (*self_0).PollingInterval,
        );
        printf(
            b"Threshold (s):\t%d\n\0" as *const u8 as *const libc::c_char,
            (*self_0).ThresholdSeconds,
        );
        printf(
            b"Number of Dumps:\t%d\n\0" as *const u8 as *const libc::c_char,
            (*self_0).NumberOfDumpsToCollect,
        );
        printf(
            b"Output directory for core dumps:\t%s\n\0" as *const u8
                as *const libc::c_char,
            (*self_0).CoreDumpPath,
        );
        if (*self_0).CoreDumpName as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            printf(
                b"Custom name for core dumps:\t%s_<counter>.<pid>\n\0" as *const u8
                    as *const libc::c_char,
                (*self_0).CoreDumpName,
            );
        }
        SetEvent(&mut (*self_0).evtConfigurationPrinted.__annonCompField4.event);
        return 1 as libc::c_int != 0;
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn ContinueMonitoring(
    mut self_0: *mut ProcDumpConfiguration,
) -> bool {
    let mut tmp: libc::c_int = 0;
    if (*self_0).NumberOfDumpsCollected >= (*self_0).NumberOfDumpsToCollect {
        return 0 as libc::c_int != 0;
    }
    if (*self_0).bTerminated {
        return 0 as libc::c_int != 0;
    }
    tmp = kill((*self_0).ProcessId, 0 as libc::c_int);
    if tmp != 0 {
        (*self_0).bTerminated = 1 as libc::c_int != 0;
        Log(
            error,
            b"Target process is no longer alive\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn BeginMonitoring(
    mut self_0: *mut ProcDumpConfiguration,
) -> bool {
    let mut tmp: bool = false;
    tmp = SetEvent(&mut (*self_0).evtStartMonitoring.__annonCompField4.event);
    return tmp;
}
pub unsafe extern "C" fn IsValidNumberArg(mut arg: *const libc::c_char) -> bool {
    let mut strLen: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___1: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    tmp = strlen(arg);
    strLen = tmp as libc::c_int;
    i = 0 as libc::c_int;
    while i < strLen {
        tmp___0 = __ctype_b_loc();
        if *(*tmp___0).offset(*arg.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & 2048 as libc::c_int == 0
        {
            tmp___1 = __ctype_b_loc();
            if *(*tmp___1).offset(*arg.offset(i as isize) as libc::c_int as isize)
                as libc::c_int & 8192 as libc::c_int == 0
            {
                return 0 as libc::c_int != 0;
            }
        }
        i += 1;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn CheckKernelVersion() -> bool {
    let mut kernelInfo: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        __domainname: [0; 65],
    };
    let mut version: libc::c_int = 0;
    let mut patch: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    tmp___4 = uname(&mut kernelInfo);
    if tmp___4 == 0 as libc::c_int {
        patch = 0 as libc::c_int;
        tmp___1 = sscanf(
            (kernelInfo.release).as_mut_ptr() as *const libc::c_char,
            b"%d.%d\0" as *const u8 as *const libc::c_char,
            &mut version as *mut libc::c_int,
            &mut patch as *mut libc::c_int,
        );
        if tmp___1 != 2 as libc::c_int {
            Log(
                error,
                b"Cannot validate kernel version\0" as *const u8 as *const libc::c_char,
            );
            tmp = __errno_location();
            tmp___0 = strerror(*tmp);
            DiagTrace(
                b"%s %s\0" as *const u8 as *const libc::c_char,
                tmp___0,
                b"in src/ProcDumpConfiguration.c, at line 963\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        if version > 3 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        if version == 3 as libc::c_int {
            if patch >= 5 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
        }
    } else {
        tmp___2 = __errno_location();
        tmp___3 = strerror(*tmp___2);
        Log(error, tmp___3 as *const libc::c_char);
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn PrintBanner() {
    printf(
        b"\nProcDump v1.2 - Sysinternals process dump utility\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Copyright (C) 2020 Microsoft Corporation. All rights reserved. Licensed under the MIT license.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"Mark Russinovich, Mario Hewardt, John Salem, Javid Habibi\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"Monitors a process and writes a dump file when the process meets the\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"specified criteria.\n\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn PrintUsage(
    mut self_0: *mut ProcDumpConfiguration,
) -> libc::c_int {
    printf(
        b"\nUsage: procdump [OPTIONS...] TARGET\n\0" as *const u8 as *const libc::c_char,
    );
    printf(b"   OPTIONS\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"      -h          Prints this help screen\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"      -C          Trigger core dump generation when CPU exceeds or equals specified value (0 to 100 * nCPU)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -c          Trigger core dump generation when CPU is less than specified value (0 to 100 * nCPU)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -M          Trigger core dump generation when memory commit exceeds or equals specified value (MB)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -m          Trigger core dump generation when when memory commit is less than specified value (MB)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -T          Trigger when thread count exceeds or equals specified value.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -F          Trigger when file descriptor count exceeds or equals specified value.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -G          Trigger when signal with the specified value (num) is sent (uses PTRACE and will affect performance of target process).\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -I          Polling frequency in milliseconds (default is %d)\n\0"
            as *const u8 as *const libc::c_char,
        1000 as libc::c_int,
    );
    printf(
        b"      -n          Number of core dumps to write before exiting (default is %d)\n\0"
            as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    printf(
        b"      -s          Consecutive seconds before dump is written (default is %d)\n\0"
            as *const u8 as *const libc::c_char,
        10 as libc::c_int,
    );
    printf(
        b"      -o          Path and/or filename prefix where the core dump is written to\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -d          Writes diagnostic logs to syslog\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"   TARGET must be exactly one of these:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"      -p          pid of the process\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"      -w          Name of the process executable\n\n\0" as *const u8
            as *const libc::c_char,
    );
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn WaitForSingleObject(
    mut Handle: *mut Handle,
    mut Milliseconds: libc::c_int,
) -> libc::c_int {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    rc = 0 as libc::c_int;
    if Milliseconds != -(1 as libc::c_int) {
        clock_gettime(0 as libc::c_int, &mut ts);
        ts.tv_sec += (Milliseconds / 1000 as libc::c_int) as __time_t;
        ts.tv_nsec
            += (Milliseconds % 1000 as libc::c_int * 1000000 as libc::c_int)
                as __syscall_slong_t;
    }
    match (*Handle).type_0 as libc::c_uint {
        0 => {
            rc = pthread_mutex_lock(&mut (*Handle).__annonCompField4.event.mutex);
            if rc == 0 as libc::c_int {
                (*Handle).__annonCompField4.event.nWaiters += 1;
                while !(*Handle).__annonCompField4.event.bTriggered {
                    if !(rc == 0 as libc::c_int) {
                        break;
                    }
                    if Milliseconds == -(1 as libc::c_int) {
                        tmp = pthread_cond_wait(
                            &mut (*Handle).__annonCompField4.event.cond
                                as *mut pthread_cond_t,
                            &mut (*Handle).__annonCompField4.event.mutex
                                as *mut pthread_mutex_t,
                        );
                        rc = tmp;
                    } else {
                        tmp___0 = pthread_cond_timedwait(
                            &mut (*Handle).__annonCompField4.event.cond
                                as *mut pthread_cond_t,
                            &mut (*Handle).__annonCompField4.event.mutex
                                as *mut pthread_mutex_t,
                            &mut ts as *mut timespec as *const timespec,
                        );
                        rc = tmp___0;
                    }
                }
                (*Handle).__annonCompField4.event.nWaiters -= 1;
                if (*Handle).__annonCompField4.event.nWaiters == 0 as libc::c_int {
                    if !(*Handle).__annonCompField4.event.bManualReset {
                        (*Handle)
                            .__annonCompField4
                            .event
                            .bTriggered = 0 as libc::c_int != 0;
                    }
                }
                pthread_mutex_unlock(&mut (*Handle).__annonCompField4.event.mutex);
            }
        }
        1 => {
            if Milliseconds == -(1 as libc::c_int) {
                tmp___1 = sem_wait(&mut (*Handle).__annonCompField4.semaphore);
                rc = tmp___1;
            } else {
                tmp___2 = sem_timedwait(
                    &mut (*Handle).__annonCompField4.semaphore as *mut sem_t,
                    &mut ts as *mut timespec as *const timespec,
                );
                rc = tmp___2;
            }
        }
        _ => {
            rc = -(1 as libc::c_int);
        }
    }
    return rc;
}
pub unsafe extern "C" fn WaiterThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut rc: libc::c_int = 0;
    let mut input: *mut thread_args = 0 as *mut thread_args;
    let mut result: thread_result = thread_result {
        retVal: 0,
        threadIndex: 0,
    };
    let mut tmp: libc::c_int = 0;
    input = thread_args as *mut thread_args;
    rc = WaitForSingleObject(
        &mut (*(*input).coordinator).evtStartWaiting,
        2000 as libc::c_int,
    );
    if (*input).milliseconds == -(1 as libc::c_int) {
        loop {
            rc = WaitForSingleObject((*input).handle, 5000 as libc::c_int);
            if !((*(*input).coordinator).stopIssued == 0) {
                break;
            }
            if !(rc == 110 as libc::c_int) {
                break;
            }
        }
    } else {
        rc = WaitForSingleObject((*input).handle, (*input).milliseconds);
    }
    pthread_mutex_lock(&mut (*(*input).coordinator).mutexEventTriggered);
    result.retVal = rc;
    result.threadIndex = (*input).threadIndex;
    tmp = (*(*input).coordinator).numberTriggered;
    (*(*input).coordinator).numberTriggered += 1;
    *((*(*input).coordinator).results).offset(tmp as isize) = result;
    pthread_mutex_unlock(&mut (*(*input).coordinator).mutexEventTriggered);
    pthread_cond_signal(&mut (*(*input).coordinator).condEventTriggered);
    WaitForSingleObject(&mut (*(*input).coordinator).evtCanCleanUp, -(1 as libc::c_int));
    pthread_mutex_lock(&mut (*(*input).coordinator).mutexEventTriggered);
    (*(*input).coordinator).nWaiters -= 1;
    if (*(*input).coordinator).nWaiters == 0 as libc::c_int {
        pthread_mutex_unlock(&mut (*(*input).coordinator).mutexEventTriggered);
        pthread_mutex_destroy(&mut (*(*input).coordinator).mutexEventTriggered);
        pthread_cond_destroy(&mut (*(*input).coordinator).condEventTriggered);
        free((*(*input).coordinator).results as *mut libc::c_void);
        free((*input).coordinator as *mut libc::c_void);
        free(input as *mut libc::c_void);
    } else {
        pthread_mutex_unlock(&mut (*(*input).coordinator).mutexEventTriggered);
        free(input as *mut libc::c_void);
    }
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn WaitForMultipleObjects(
    mut Count: libc::c_int,
    mut Handles: *mut *mut Handle,
    mut WaitAll: bool,
    mut Milliseconds: libc::c_int,
) -> libc::c_int {
    let mut coordinator: *mut coordinator = 0 as *mut coordinator;
    let mut results: *mut thread_result = 0 as *mut thread_result;
    let mut threads: *mut pthread_t = 0 as *mut pthread_t;
    let mut thread_args: *mut *mut thread_args = 0 as *mut *mut thread_args;
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut t: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut retVal: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut thread_result = 0 as *mut thread_result;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(
        (::std::mem::size_of::<pthread_t>() as libc::c_ulong)
            .wrapping_mul(Count as libc::c_ulong),
    );
    threads = tmp as *mut pthread_t;
    tmp___0 = malloc(
        (::std::mem::size_of::<*mut thread_args>() as libc::c_ulong)
            .wrapping_mul(Count as libc::c_ulong),
    );
    thread_args = tmp___0 as *mut *mut thread_args;
    tmp___1 = malloc(::std::mem::size_of::<coordinator>() as libc::c_ulong);
    coordinator = tmp___1 as *mut coordinator;
    if coordinator as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        printf(
            b"ERROR: Failed to malloc in %s\n\0" as *const u8 as *const libc::c_char,
            b"src/Handle.c\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    (*coordinator).numberTriggered = 0 as libc::c_int;
    (*coordinator).stopIssued = 0 as libc::c_int;
    (*coordinator).evtCanCleanUp.type_0 = EVENT;
    (*coordinator).evtStartWaiting.type_0 = EVENT;
    InitNamedEvent(
        &mut (*coordinator).evtCanCleanUp.__annonCompField4.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"CanCleanUp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    InitNamedEvent(
        &mut (*coordinator).evtStartWaiting.__annonCompField4.event,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        b"StartWaiting\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pthread_cond_init(
        &mut (*coordinator).condEventTriggered as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    pthread_mutex_init(
        &mut (*coordinator).mutexEventTriggered,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    tmp___3 = malloc(
        (::std::mem::size_of::<thread_result>() as libc::c_ulong)
            .wrapping_mul(Count as libc::c_ulong),
    );
    tmp___2 = tmp___3 as *mut thread_result;
    (*coordinator).results = tmp___2;
    results = tmp___2;
    if Milliseconds != -(1 as libc::c_int) {
        clock_gettime(0 as libc::c_int, &mut ts);
        ts.tv_sec += (Milliseconds / 1000 as libc::c_int) as __time_t;
        ts.tv_nsec
            += (Milliseconds % 1000 as libc::c_int * 1000000 as libc::c_int)
                as __syscall_slong_t;
    }
    pthread_mutex_lock(&mut (*coordinator).mutexEventTriggered);
    t = 0 as libc::c_int;
    while t < Count {
        tmp___4 = malloc(::std::mem::size_of::<thread_args>() as libc::c_ulong);
        let ref mut fresh0 = *thread_args.offset(t as isize);
        *fresh0 = tmp___4 as *mut thread_args;
        if *thread_args.offset(t as isize) as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            printf(
                b"ERROR: Failed to alloc in %s\n\0" as *const u8 as *const libc::c_char,
                b"src/Handle.c\0" as *const u8 as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
        let ref mut fresh1 = (**thread_args.offset(t as isize)).handle;
        *fresh1 = *Handles.offset(t as isize);
        if Milliseconds == -(1 as libc::c_int) {
            (**thread_args.offset(t as isize)).milliseconds = Milliseconds;
        } else {
            (**thread_args.offset(t as isize))
                .milliseconds = Milliseconds + 100 as libc::c_int;
        }
        (**thread_args.offset(t as isize)).threadIndex = t;
        let ref mut fresh2 = (**thread_args.offset(t as isize)).coordinator;
        *fresh2 = coordinator;
        rc = pthread_create(
            threads.offset(t as isize),
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                WaiterThread
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            *thread_args.offset(t as isize) as *mut libc::c_void,
        );
        if rc != 0 {
            printf(
                b"ERROR: pthread_create failed in %s with error %d\n\0" as *const u8
                    as *const libc::c_char,
                b"src/Handle.c\0" as *const u8 as *const libc::c_char,
                rc,
            );
            exit(-(1 as libc::c_int));
        }
        t += 1;
    }
    (*coordinator).nWaiters = Count;
    SetEvent(&mut (*coordinator).evtStartWaiting.__annonCompField4.event);
    let mut current_block_54: u64;
    loop {
        if WaitAll {
            if (*coordinator).numberTriggered < Count {
                current_block_54 = 9789429621685773810;
            } else {
                current_block_54 = 2766144271120963283;
            }
        } else {
            current_block_54 = 2766144271120963283;
        }
        match current_block_54 {
            2766144271120963283 => {
                if WaitAll {
                    break;
                }
                if !((*coordinator).numberTriggered == 0 as libc::c_int) {
                    break;
                }
            }
            _ => {}
        }
        if !(rc == 0 as libc::c_int) {
            break;
        }
        if Milliseconds == -(1 as libc::c_int) {
            rc = pthread_cond_wait(
                &mut (*coordinator).condEventTriggered as *mut pthread_cond_t,
                &mut (*coordinator).mutexEventTriggered as *mut pthread_mutex_t,
            );
            if rc != 0 as libc::c_int {
                break;
            }
        } else {
            rc = pthread_cond_timedwait(
                &mut (*coordinator).condEventTriggered as *mut pthread_cond_t,
                &mut (*coordinator).mutexEventTriggered as *mut pthread_mutex_t,
                &mut ts as *mut timespec as *const timespec,
            );
            if rc != 0 as libc::c_int {
                break;
            }
        }
    }
    (*coordinator).stopIssued = 1 as libc::c_int;
    pthread_mutex_unlock(&mut (*coordinator).mutexEventTriggered);
    t = 0 as libc::c_int;
    while t < Count {
        pthread_detach(*threads.offset(t as isize));
        t += 1;
    }
    SetEvent(&mut (*coordinator).evtCanCleanUp.__annonCompField4.event);
    free(threads as *mut libc::c_void);
    free(thread_args as *mut libc::c_void);
    if rc != 0 {
        retVal = rc;
    } else if WaitAll {
        retVal = rc;
    } else {
        retVal = (*results.offset(0 as libc::c_int as isize)).retVal
            + (*results.offset(0 as libc::c_int as isize)).threadIndex;
    }
    return retVal;
}
pub unsafe extern "C" fn GetProcessStat(
    mut pid: pid_t,
    mut proc_0: *mut ProcessStat,
) -> bool {
    let mut procFilePath: [libc::c_char; 32] = [0; 32];
    let mut fileBuffer: [libc::c_char; 1024] = [0; 1024];
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut savePtr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut procFile: *mut FILE = 0 as *mut FILE;
    let mut fddir: *mut DIR = 0 as *mut DIR;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_long = 0;
    let mut tmp___5: libc::c_long = 0;
    let mut tmp___6: libc::c_long = 0;
    let mut tmp___7: libc::c_long = 0;
    let mut tmp___8: libc::c_long = 0;
    let mut tmp___9: libc::c_ulong = 0;
    let mut tmp___10: libc::c_long = 0;
    let mut tmp___11: libc::c_long = 0;
    let mut tmp___12: libc::c_ulong = 0;
    let mut tmp___13: libc::c_ulong = 0;
    let mut tmp___14: libc::c_long = 0;
    savePtr = 0 as *mut libc::c_void as *mut libc::c_char;
    procFile = 0 as *mut libc::c_void as *mut FILE;
    fddir = 0 as *mut libc::c_void as *mut DIR;
    entry = 0 as *mut libc::c_void as *mut dirent;
    tmp = sprintf(
        procFilePath.as_mut_ptr(),
        b"/proc/%d/fdinfo\0" as *const u8 as *const libc::c_char,
        pid,
    );
    if tmp < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    fddir = opendir(procFilePath.as_mut_ptr() as *const libc::c_char);
    if !fddir.is_null() {
        (*proc_0).num_filedescriptors = 0 as libc::c_int;
        loop {
            entry = readdir(fddir);
            if !(entry as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                break;
            }
            (*proc_0).num_filedescriptors += 1;
        }
        closedir(fddir);
    } else {
        Log(
            error,
            b"Failed to open %s. Exiting...\n\0" as *const u8 as *const libc::c_char,
            procFilePath.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0).num_filedescriptors -= 2 as libc::c_int;
    tmp___0 = sprintf(
        procFilePath.as_mut_ptr(),
        b"/proc/%d/stat\0" as *const u8 as *const libc::c_char,
        pid,
    );
    if tmp___0 < 0 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    procFile = fopen(
        procFilePath.as_mut_ptr() as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if procFile as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = fgets(
            fileBuffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            procFile,
        );
        if tmp___1 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            Log(
                error,
                b"Failed to read from %s. Exiting...\n\0" as *const u8
                    as *const libc::c_char,
                procFilePath.as_mut_ptr(),
            );
            fclose(procFile);
            return 0 as libc::c_int != 0;
        }
        fclose(procFile);
    } else {
        Log(
            error,
            b"Failed to open %s.\n\0" as *const u8 as *const libc::c_char,
            procFilePath.as_mut_ptr(),
        );
        return 0 as libc::c_int != 0;
    }
    tmp___2 = atoi(fileBuffer.as_mut_ptr() as *const libc::c_char);
    (*proc_0).pid = tmp___2;
    savePtr = strrchr(fileBuffer.as_mut_ptr() as *const libc::c_char, ')' as i32);
    if savePtr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        savePtr = savePtr.offset(2 as libc::c_int as isize);
        tmp___3 = strtok_r(
            savePtr,
            b" \0" as *const u8 as *const libc::c_char,
            &mut savePtr as *mut *mut libc::c_char,
        );
        (*proc_0).state = *tmp___3.offset(0 as libc::c_int as isize);
    }
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Parent PID. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 84\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___4 = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).ppid = tmp___4 as pid_t;
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Process group ID. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 93\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___5 = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).pgrp = tmp___5 as gid_t;
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Session ID. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 102\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___6 = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).session = tmp___6 as libc::c_int;
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Controlling terminal. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 111\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___7 = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).tty_nr = tmp___7 as libc::c_int;
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Foreground group ID. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 120\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___8 = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).tpgid = tmp___8 as gid_t;
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Kernel flags. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 129\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___9 = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).flags = tmp___9 as libc::c_uint;
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - Minflt. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 138\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .minflt = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cminflt. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 147\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .cminflt = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - majflt. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 156\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .majflt = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cmajflt. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 165\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .cmajflt = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - utime. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 174\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .utime = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - stime. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 183\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .stime = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cutime. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 192\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .cutime = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cstime. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 201\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .cstime = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - priority. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 210\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .priority = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - nice. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 219\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .nice = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - num_threads. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 228\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .num_threads = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - itrealvalue. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 237\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .itrealvalue = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - starttime. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 246\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .starttime = strtoull(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - vsize. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 255\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .vsize = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - rss. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 264\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .rss = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - rsslim. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 273\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .rsslim = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - startcode. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 282\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .startcode = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - endcode. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 291\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .endcode = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - startstack. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 300\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .startstack = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - kstkesp. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 309\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .kstkesp = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - kstkeip. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 318\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .kstkeip = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - signal. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 327\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .signal = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - blocked. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 336\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .blocked = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - sigignore. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 345\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .sigignore = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - sigcatch. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 354\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .sigcatch = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - wchan. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 363\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .wchan = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - nswap. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 372\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .nswap = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cnswap. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 381\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .cnswap = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - exit_signal. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 390\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___10 = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).exit_signal = tmp___10 as libc::c_int;
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - processor. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 399\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___11 = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).processor = tmp___11 as libc::c_int;
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - rt_priority. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 408\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___12 = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).rt_priority = tmp___12 as libc::c_uint;
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - policy. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 417\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___13 = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).policy = tmp___13 as libc::c_uint;
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - delayacct_blkio_ticks. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 426\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .delayacct_blkio_ticks = strtoull(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - guest_time. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 435\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .guest_time = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - cguest_time. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 444\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .cguest_time = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - start_data. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 453\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .start_data = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - end_data. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 462\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .end_data = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - start_brk. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 471\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .end_data = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - arg_start. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 480\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .arg_start = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - arg_end. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 489\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .arg_end = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - env_start. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 498\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .env_start = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - env_end. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 507\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*proc_0)
        .env_end = strtoul(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    token = strtok_r(
        0 as *mut libc::c_void as *mut libc::c_char,
        b" \0" as *const u8 as *const libc::c_char,
        &mut savePtr as *mut *mut libc::c_char,
    );
    if token as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        DiagTrace(
            b"GetProcessStat: failed to get token from proc/[pid]/stat - exit_code. %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/Process.c, at line 516\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___14 = strtol(
        token as *const libc::c_char,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    (*proc_0).exit_code = tmp___14 as libc::c_int;
    return 1 as libc::c_int != 0;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: __uid_t = 0;
    let mut tmp___1: bool = false;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: bool = false;
    PrintBanner();
    InitProcDump();
    tmp = GetOptions(&mut g_config, argc, argv);
    if tmp != 0 as libc::c_int {
        DiagTrace(
            b"main: failed to parse command line arguments %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Procdump.c, at line 21\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    PrintConfiguration(&mut g_config);
    printf(
        b"\nPress Ctrl-C to end monitoring without terminating the process.\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    tmp___0 = geteuid();
    if tmp___0 != 0 as libc::c_uint {
        Log(
            warn,
            b"Procdump not running with elevated credentials. If your uid does not match the uid of the target process procdump will not be able to capture memory dumps\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if g_config.WaitingForProcessName {
        tmp___1 = WaitForProcessName(&mut g_config);
        if tmp___1 as libc::c_int == 0 as libc::c_int {
            ExitProcDump();
        }
    }
    tmp___2 = CreateTriggerThreads(&mut g_config);
    if tmp___2 != 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"main: failed to create trigger threads. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Procdump.c, at line 45\0" as *const u8 as *const libc::c_char,
        );
        ExitProcDump();
    }
    tmp___3 = BeginMonitoring(&mut g_config);
    if tmp___3 as libc::c_int == 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"main: failed to start monitoring. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/Procdump.c, at line 51\0" as *const u8 as *const libc::c_char,
        );
        ExitProcDump();
    }
    WaitForAllThreadsToTerminate(&mut g_config);
    ExitProcDump();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn CommitMonitoringThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut config: *mut ProcDumpConfiguration = 0 as *mut ProcDumpConfiguration;
    let mut pageSize_kb: libc::c_long = 0;
    let mut memUsage: libc::c_ulong = 0;
    let mut proc_0: ProcessStat = ProcessStat {
        pid: 0,
        comm: 0 as *mut libc::c_char,
        state: 0,
        ppid: 0,
        pgrp: 0,
        session: 0,
        tty_nr: 0,
        tpgid: 0,
        flags: 0,
        minflt: 0,
        cminflt: 0,
        majflt: 0,
        cmajflt: 0,
        utime: 0,
        stime: 0,
        cutime: 0,
        cstime: 0,
        priority: 0,
        nice: 0,
        num_threads: 0,
        itrealvalue: 0,
        starttime: 0,
        vsize: 0,
        rss: 0,
        rsslim: 0,
        startcode: 0,
        endcode: 0,
        startstack: 0,
        kstkesp: 0,
        kstkeip: 0,
        signal: 0,
        blocked: 0,
        sigignore: 0,
        sigcatch: 0,
        wchan: 0,
        nswap: 0,
        cnswap: 0,
        exit_signal: 0,
        processor: 0,
        rt_priority: 0,
        policy: 0,
        delayacct_blkio_ticks: 0,
        guest_time: 0,
        cguest_time: 0,
        start_data: 0,
        end_data: 0,
        start_brk: 0,
        arg_start: 0,
        arg_end: 0,
        env_start: 0,
        env_end: 0,
        exit_code: 0,
        num_filedescriptors: 0,
    };
    let mut rc: libc::c_int = 0;
    let mut writer: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: bool = false;
    DiagTrace(
        b"CommitMonitoringThread: Starting Trigger Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 16\0" as *const u8 as *const libc::c_char,
    );
    config = thread_args as *mut ProcDumpConfiguration;
    memUsage = 0 as libc::c_ulong;
    proc_0.pid = 0 as libc::c_int;
    proc_0.comm = 0 as *mut libc::c_char;
    proc_0.state = 0 as libc::c_int as libc::c_char;
    proc_0.ppid = 0 as libc::c_int;
    proc_0.pgrp = 0 as libc::c_uint;
    proc_0.session = 0 as libc::c_int;
    proc_0.tty_nr = 0 as libc::c_int;
    proc_0.tpgid = 0 as libc::c_uint;
    proc_0.flags = 0 as libc::c_uint;
    proc_0.minflt = 0 as libc::c_ulong;
    proc_0.cminflt = 0 as libc::c_ulong;
    proc_0.majflt = 0 as libc::c_ulong;
    proc_0.cmajflt = 0 as libc::c_ulong;
    proc_0.utime = 0 as libc::c_ulong;
    proc_0.stime = 0 as libc::c_ulong;
    proc_0.cutime = 0 as libc::c_ulong;
    proc_0.cstime = 0 as libc::c_ulong;
    proc_0.priority = 0 as libc::c_long;
    proc_0.nice = 0 as libc::c_long;
    proc_0.num_threads = 0 as libc::c_long;
    proc_0.itrealvalue = 0 as libc::c_long;
    proc_0.starttime = 0 as libc::c_ulonglong;
    proc_0.vsize = 0 as libc::c_ulong;
    proc_0.rss = 0 as libc::c_long;
    proc_0.rsslim = 0 as libc::c_ulong;
    proc_0.startcode = 0 as libc::c_ulong;
    proc_0.endcode = 0 as libc::c_ulong;
    proc_0.startstack = 0 as libc::c_ulong;
    proc_0.kstkesp = 0 as libc::c_ulong;
    proc_0.kstkeip = 0 as libc::c_ulong;
    proc_0.signal = 0 as libc::c_ulong;
    proc_0.blocked = 0 as libc::c_ulong;
    proc_0.sigignore = 0 as libc::c_ulong;
    proc_0.sigcatch = 0 as libc::c_ulong;
    proc_0.wchan = 0 as libc::c_ulong;
    proc_0.nswap = 0 as libc::c_ulong;
    proc_0.cnswap = 0 as libc::c_ulong;
    proc_0.exit_signal = 0 as libc::c_int;
    proc_0.processor = 0 as libc::c_int;
    proc_0.rt_priority = 0 as libc::c_uint;
    proc_0.policy = 0 as libc::c_uint;
    proc_0.delayacct_blkio_ticks = 0 as libc::c_ulonglong;
    proc_0.guest_time = 0 as libc::c_ulong;
    proc_0.cguest_time = 0 as libc::c_long;
    proc_0.start_data = 0 as libc::c_ulong;
    proc_0.end_data = 0 as libc::c_ulong;
    proc_0.start_brk = 0 as libc::c_ulong;
    proc_0.arg_start = 0 as libc::c_ulong;
    proc_0.arg_end = 0 as libc::c_ulong;
    proc_0.env_start = 0 as libc::c_ulong;
    proc_0.env_end = 0 as libc::c_ulong;
    proc_0.exit_code = 0 as libc::c_int;
    proc_0.num_filedescriptors = 0 as libc::c_int;
    rc = 0 as libc::c_int;
    tmp = NewCoreDumpWriter(COMMIT, config);
    writer = tmp;
    tmp___0 = sysconf(30 as libc::c_int);
    pageSize_kb = tmp___0 >> 10 as libc::c_int;
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 1 as libc::c_int {
        let mut current_block_71: u64;
        loop {
            rc = WaitForQuit(config, (*config).PollingInterval);
            if !(rc == 110 as libc::c_int) {
                break;
            }
            tmp___1 = GetProcessStat((*config).ProcessId, &mut proc_0);
            if tmp___1 {
                memUsage = (proc_0.rss * pageSize_kb >> 10 as libc::c_int)
                    as libc::c_ulong;
                memUsage = memUsage
                    .wrapping_add(
                        (proc_0.nswap).wrapping_mul(pageSize_kb as libc::c_ulong)
                            >> 10 as libc::c_int,
                    );
                if (*config).bMemoryTriggerBelowValue {
                    if memUsage < (*config).MemoryThreshold as libc::c_ulong {
                        current_block_71 = 17467640331785294267;
                    } else {
                        current_block_71 = 4255821591825487887;
                    }
                } else {
                    current_block_71 = 4255821591825487887;
                }
                match current_block_71 {
                    4255821591825487887 => {
                        if (*config).bMemoryTriggerBelowValue {
                            continue;
                        }
                        if !(memUsage >= (*config).MemoryThreshold as libc::c_ulong) {
                            continue;
                        }
                    }
                    _ => {}
                }
                Log(
                    info,
                    b"Commit: %ld MB\0" as *const u8 as *const libc::c_char,
                    memUsage,
                );
                rc = WriteCoreDump(writer);
                rc = WaitForQuit(
                    config,
                    (*config).ThresholdSeconds * 1000 as libc::c_int,
                );
                if rc != 110 as libc::c_int {
                    break;
                }
            } else {
                Log(
                    error,
                    b"An error occurred while parsing procfs\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"CommitMonitoringThread: Exiting Trigger Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 59\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn ThreadCountMonitoringThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut config: *mut ProcDumpConfiguration = 0 as *mut ProcDumpConfiguration;
    let mut proc_0: ProcessStat = ProcessStat {
        pid: 0,
        comm: 0 as *mut libc::c_char,
        state: 0,
        ppid: 0,
        pgrp: 0,
        session: 0,
        tty_nr: 0,
        tpgid: 0,
        flags: 0,
        minflt: 0,
        cminflt: 0,
        majflt: 0,
        cmajflt: 0,
        utime: 0,
        stime: 0,
        cutime: 0,
        cstime: 0,
        priority: 0,
        nice: 0,
        num_threads: 0,
        itrealvalue: 0,
        starttime: 0,
        vsize: 0,
        rss: 0,
        rsslim: 0,
        startcode: 0,
        endcode: 0,
        startstack: 0,
        kstkesp: 0,
        kstkeip: 0,
        signal: 0,
        blocked: 0,
        sigignore: 0,
        sigcatch: 0,
        wchan: 0,
        nswap: 0,
        cnswap: 0,
        exit_signal: 0,
        processor: 0,
        rt_priority: 0,
        policy: 0,
        delayacct_blkio_ticks: 0,
        guest_time: 0,
        cguest_time: 0,
        start_data: 0,
        end_data: 0,
        start_brk: 0,
        arg_start: 0,
        arg_end: 0,
        env_start: 0,
        env_end: 0,
        exit_code: 0,
        num_filedescriptors: 0,
    };
    let mut rc: libc::c_int = 0;
    let mut writer: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp___0: bool = false;
    DiagTrace(
        b"ThreadCountMonitoringThread: Starting Thread Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 65\0" as *const u8 as *const libc::c_char,
    );
    config = thread_args as *mut ProcDumpConfiguration;
    proc_0.pid = 0 as libc::c_int;
    proc_0.comm = 0 as *mut libc::c_char;
    proc_0.state = 0 as libc::c_int as libc::c_char;
    proc_0.ppid = 0 as libc::c_int;
    proc_0.pgrp = 0 as libc::c_uint;
    proc_0.session = 0 as libc::c_int;
    proc_0.tty_nr = 0 as libc::c_int;
    proc_0.tpgid = 0 as libc::c_uint;
    proc_0.flags = 0 as libc::c_uint;
    proc_0.minflt = 0 as libc::c_ulong;
    proc_0.cminflt = 0 as libc::c_ulong;
    proc_0.majflt = 0 as libc::c_ulong;
    proc_0.cmajflt = 0 as libc::c_ulong;
    proc_0.utime = 0 as libc::c_ulong;
    proc_0.stime = 0 as libc::c_ulong;
    proc_0.cutime = 0 as libc::c_ulong;
    proc_0.cstime = 0 as libc::c_ulong;
    proc_0.priority = 0 as libc::c_long;
    proc_0.nice = 0 as libc::c_long;
    proc_0.num_threads = 0 as libc::c_long;
    proc_0.itrealvalue = 0 as libc::c_long;
    proc_0.starttime = 0 as libc::c_ulonglong;
    proc_0.vsize = 0 as libc::c_ulong;
    proc_0.rss = 0 as libc::c_long;
    proc_0.rsslim = 0 as libc::c_ulong;
    proc_0.startcode = 0 as libc::c_ulong;
    proc_0.endcode = 0 as libc::c_ulong;
    proc_0.startstack = 0 as libc::c_ulong;
    proc_0.kstkesp = 0 as libc::c_ulong;
    proc_0.kstkeip = 0 as libc::c_ulong;
    proc_0.signal = 0 as libc::c_ulong;
    proc_0.blocked = 0 as libc::c_ulong;
    proc_0.sigignore = 0 as libc::c_ulong;
    proc_0.sigcatch = 0 as libc::c_ulong;
    proc_0.wchan = 0 as libc::c_ulong;
    proc_0.nswap = 0 as libc::c_ulong;
    proc_0.cnswap = 0 as libc::c_ulong;
    proc_0.exit_signal = 0 as libc::c_int;
    proc_0.processor = 0 as libc::c_int;
    proc_0.rt_priority = 0 as libc::c_uint;
    proc_0.policy = 0 as libc::c_uint;
    proc_0.delayacct_blkio_ticks = 0 as libc::c_ulonglong;
    proc_0.guest_time = 0 as libc::c_ulong;
    proc_0.cguest_time = 0 as libc::c_long;
    proc_0.start_data = 0 as libc::c_ulong;
    proc_0.end_data = 0 as libc::c_ulong;
    proc_0.start_brk = 0 as libc::c_ulong;
    proc_0.arg_start = 0 as libc::c_ulong;
    proc_0.arg_end = 0 as libc::c_ulong;
    proc_0.env_start = 0 as libc::c_ulong;
    proc_0.env_end = 0 as libc::c_ulong;
    proc_0.exit_code = 0 as libc::c_int;
    proc_0.num_filedescriptors = 0 as libc::c_int;
    rc = 0 as libc::c_int;
    tmp = NewCoreDumpWriter(THREAD, config);
    writer = tmp;
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 1 as libc::c_int {
        loop {
            rc = WaitForQuit(config, (*config).PollingInterval);
            if !(rc == 110 as libc::c_int) {
                break;
            }
            tmp___0 = GetProcessStat((*config).ProcessId, &mut proc_0);
            if tmp___0 {
                if !(proc_0.num_threads >= (*config).ThreadThreshold as libc::c_long) {
                    continue;
                }
                Log(
                    info,
                    b"Threads: %ld\0" as *const u8 as *const libc::c_char,
                    proc_0.num_threads,
                );
                rc = WriteCoreDump(writer);
                rc = WaitForQuit(
                    config,
                    (*config).ThresholdSeconds * 1000 as libc::c_int,
                );
                if rc != 110 as libc::c_int {
                    break;
                }
            } else {
                Log(
                    error,
                    b"An error occurred while parsing procfs\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"ThreadCountMonitoringThread: Exiting Thread trigger Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 98\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn FileDescriptorCountMonitoringThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut config: *mut ProcDumpConfiguration = 0 as *mut ProcDumpConfiguration;
    let mut proc_0: ProcessStat = ProcessStat {
        pid: 0,
        comm: 0 as *mut libc::c_char,
        state: 0,
        ppid: 0,
        pgrp: 0,
        session: 0,
        tty_nr: 0,
        tpgid: 0,
        flags: 0,
        minflt: 0,
        cminflt: 0,
        majflt: 0,
        cmajflt: 0,
        utime: 0,
        stime: 0,
        cutime: 0,
        cstime: 0,
        priority: 0,
        nice: 0,
        num_threads: 0,
        itrealvalue: 0,
        starttime: 0,
        vsize: 0,
        rss: 0,
        rsslim: 0,
        startcode: 0,
        endcode: 0,
        startstack: 0,
        kstkesp: 0,
        kstkeip: 0,
        signal: 0,
        blocked: 0,
        sigignore: 0,
        sigcatch: 0,
        wchan: 0,
        nswap: 0,
        cnswap: 0,
        exit_signal: 0,
        processor: 0,
        rt_priority: 0,
        policy: 0,
        delayacct_blkio_ticks: 0,
        guest_time: 0,
        cguest_time: 0,
        start_data: 0,
        end_data: 0,
        start_brk: 0,
        arg_start: 0,
        arg_end: 0,
        env_start: 0,
        env_end: 0,
        exit_code: 0,
        num_filedescriptors: 0,
    };
    let mut rc: libc::c_int = 0;
    let mut writer: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp___0: bool = false;
    DiagTrace(
        b"FileDescriptorCountMonitoringThread: Starting Filedescriptor Thread %s\0"
            as *const u8 as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 105\0" as *const u8 as *const libc::c_char,
    );
    config = thread_args as *mut ProcDumpConfiguration;
    proc_0.pid = 0 as libc::c_int;
    proc_0.comm = 0 as *mut libc::c_char;
    proc_0.state = 0 as libc::c_int as libc::c_char;
    proc_0.ppid = 0 as libc::c_int;
    proc_0.pgrp = 0 as libc::c_uint;
    proc_0.session = 0 as libc::c_int;
    proc_0.tty_nr = 0 as libc::c_int;
    proc_0.tpgid = 0 as libc::c_uint;
    proc_0.flags = 0 as libc::c_uint;
    proc_0.minflt = 0 as libc::c_ulong;
    proc_0.cminflt = 0 as libc::c_ulong;
    proc_0.majflt = 0 as libc::c_ulong;
    proc_0.cmajflt = 0 as libc::c_ulong;
    proc_0.utime = 0 as libc::c_ulong;
    proc_0.stime = 0 as libc::c_ulong;
    proc_0.cutime = 0 as libc::c_ulong;
    proc_0.cstime = 0 as libc::c_ulong;
    proc_0.priority = 0 as libc::c_long;
    proc_0.nice = 0 as libc::c_long;
    proc_0.num_threads = 0 as libc::c_long;
    proc_0.itrealvalue = 0 as libc::c_long;
    proc_0.starttime = 0 as libc::c_ulonglong;
    proc_0.vsize = 0 as libc::c_ulong;
    proc_0.rss = 0 as libc::c_long;
    proc_0.rsslim = 0 as libc::c_ulong;
    proc_0.startcode = 0 as libc::c_ulong;
    proc_0.endcode = 0 as libc::c_ulong;
    proc_0.startstack = 0 as libc::c_ulong;
    proc_0.kstkesp = 0 as libc::c_ulong;
    proc_0.kstkeip = 0 as libc::c_ulong;
    proc_0.signal = 0 as libc::c_ulong;
    proc_0.blocked = 0 as libc::c_ulong;
    proc_0.sigignore = 0 as libc::c_ulong;
    proc_0.sigcatch = 0 as libc::c_ulong;
    proc_0.wchan = 0 as libc::c_ulong;
    proc_0.nswap = 0 as libc::c_ulong;
    proc_0.cnswap = 0 as libc::c_ulong;
    proc_0.exit_signal = 0 as libc::c_int;
    proc_0.processor = 0 as libc::c_int;
    proc_0.rt_priority = 0 as libc::c_uint;
    proc_0.policy = 0 as libc::c_uint;
    proc_0.delayacct_blkio_ticks = 0 as libc::c_ulonglong;
    proc_0.guest_time = 0 as libc::c_ulong;
    proc_0.cguest_time = 0 as libc::c_long;
    proc_0.start_data = 0 as libc::c_ulong;
    proc_0.end_data = 0 as libc::c_ulong;
    proc_0.start_brk = 0 as libc::c_ulong;
    proc_0.arg_start = 0 as libc::c_ulong;
    proc_0.arg_end = 0 as libc::c_ulong;
    proc_0.env_start = 0 as libc::c_ulong;
    proc_0.env_end = 0 as libc::c_ulong;
    proc_0.exit_code = 0 as libc::c_int;
    proc_0.num_filedescriptors = 0 as libc::c_int;
    rc = 0 as libc::c_int;
    tmp = NewCoreDumpWriter(FILEDESC, config);
    writer = tmp;
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 1 as libc::c_int {
        loop {
            rc = WaitForQuit(config, (*config).PollingInterval);
            if !(rc == 110 as libc::c_int) {
                break;
            }
            tmp___0 = GetProcessStat((*config).ProcessId, &mut proc_0);
            if tmp___0 {
                if !(proc_0.num_filedescriptors >= (*config).FileDescriptorThreshold) {
                    continue;
                }
                Log(
                    info,
                    b"File descriptors: %ld\0" as *const u8 as *const libc::c_char,
                    proc_0.num_filedescriptors,
                );
                rc = WriteCoreDump(writer);
                rc = WaitForQuit(
                    config,
                    (*config).ThresholdSeconds * 1000 as libc::c_int,
                );
                if rc != 110 as libc::c_int {
                    break;
                }
            } else {
                Log(
                    error,
                    b"An error occurred while parsing procfs\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"FileDescriptorCountMonitoringThread: Exiting Filedescriptor trigger Thread %s\0"
            as *const u8 as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 138\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn SignalMonitoringThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut config: *mut ProcDumpConfiguration = 0 as *mut ProcDumpConfiguration;
    let mut wstatus: libc::c_int = 0;
    let mut signum: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut writer: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: libc::c_long = 0;
    let mut tmp___2: libc::c_long = 0;
    DiagTrace(
        b"SignalMonitoringThread: Starting SignalMonitoring Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 154\0" as *const u8 as *const libc::c_char,
    );
    config = thread_args as *mut ProcDumpConfiguration;
    signum = -(1 as libc::c_int);
    rc = 0 as libc::c_int;
    tmp = NewCoreDumpWriter(SIGNAL, config);
    writer = tmp;
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 1 as libc::c_int {
        tmp___2 = ptrace(
            PTRACE_SEIZE,
            (*config).ProcessId,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
        if tmp___2 == -(1 as libc::c_long) {
            Log(
                error,
                b"Unable to PTRACE the target process\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            loop {
                waitpid((*config).ProcessId, &mut wstatus, 0 as libc::c_int);
                if wstatus & 127 as libc::c_int == 0 as libc::c_int {
                    ptrace(
                        PTRACE_DETACH,
                        (*config).ProcessId,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    break;
                } else if ((wstatus & 127 as libc::c_int) + 1 as libc::c_int)
                        as libc::c_schar as libc::c_int >> 1 as libc::c_int
                        > 0 as libc::c_int
                    {
                    ptrace(
                        PTRACE_DETACH,
                        (*config).ProcessId,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    break;
                } else {
                    pthread_mutex_lock(&mut ptrace_mutex);
                    signum = (wstatus & 65280 as libc::c_int) >> 8 as libc::c_int;
                    if signum == (*config).SignalNumber {
                        tmp___0 = ptrace(
                            PTRACE_DETACH,
                            (*config).ProcessId,
                            0 as libc::c_int,
                            19 as libc::c_int,
                        );
                        if tmp___0 == -(1 as libc::c_long) {
                            Log(
                                error,
                                b"Unable to PTRACE (DETACH) the target process\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            pthread_mutex_unlock(&mut ptrace_mutex);
                            break;
                        } else {
                            Log(
                                info,
                                b"Signal intercepted: %d\0" as *const u8
                                    as *const libc::c_char,
                                signum,
                            );
                            rc = WriteCoreDump(writer);
                            kill((*config).ProcessId, 18 as libc::c_int);
                            if (*config).NumberOfDumpsCollected
                                >= (*config).NumberOfDumpsToCollect
                            {
                                kill((*config).ProcessId, signum);
                                pthread_mutex_unlock(&mut ptrace_mutex);
                                break;
                            } else {
                                ptrace(
                                    PTRACE_CONT,
                                    (*config).ProcessId,
                                    0 as *mut libc::c_void,
                                    signum,
                                );
                                tmp___1 = ptrace(
                                    PTRACE_SEIZE,
                                    (*config).ProcessId,
                                    0 as *mut libc::c_void,
                                    0 as *mut libc::c_void,
                                );
                                if tmp___1 == -(1 as libc::c_long) {
                                    Log(
                                        error,
                                        b"Unable to PTRACE the target process\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    pthread_mutex_unlock(&mut ptrace_mutex);
                                    break;
                                } else {
                                    pthread_mutex_unlock(&mut ptrace_mutex);
                                }
                            }
                        }
                    } else {
                        ptrace(
                            PTRACE_CONT,
                            (*config).ProcessId,
                            0 as *mut libc::c_void,
                            signum,
                        );
                        pthread_mutex_unlock(&mut ptrace_mutex);
                    }
                }
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"SignalMonitoringThread: Exiting SignalMonitoring Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 232\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn CpuMonitoringThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut config: *mut ProcDumpConfiguration = 0 as *mut ProcDumpConfiguration;
    let mut totalTime: libc::c_ulong = 0;
    let mut elapsedTime: libc::c_ulong = 0;
    let mut sysInfo: sysinfo = sysinfo {
        uptime: 0,
        loads: [0; 3],
        totalram: 0,
        freeram: 0,
        sharedram: 0,
        bufferram: 0,
        totalswap: 0,
        freeswap: 0,
        procs: 0,
        pad: 0,
        totalhigh: 0,
        freehigh: 0,
        mem_unit: 0,
        _f: [0; 0],
    };
    let mut cpuUsage: libc::c_int = 0;
    let mut writer: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut rc: libc::c_int = 0;
    let mut proc_0: ProcessStat = ProcessStat {
        pid: 0,
        comm: 0 as *mut libc::c_char,
        state: 0,
        ppid: 0,
        pgrp: 0,
        session: 0,
        tty_nr: 0,
        tpgid: 0,
        flags: 0,
        minflt: 0,
        cminflt: 0,
        majflt: 0,
        cmajflt: 0,
        utime: 0,
        stime: 0,
        cutime: 0,
        cstime: 0,
        priority: 0,
        nice: 0,
        num_threads: 0,
        itrealvalue: 0,
        starttime: 0,
        vsize: 0,
        rss: 0,
        rsslim: 0,
        startcode: 0,
        endcode: 0,
        startstack: 0,
        kstkesp: 0,
        kstkeip: 0,
        signal: 0,
        blocked: 0,
        sigignore: 0,
        sigcatch: 0,
        wchan: 0,
        nswap: 0,
        cnswap: 0,
        exit_signal: 0,
        processor: 0,
        rt_priority: 0,
        policy: 0,
        delayacct_blkio_ticks: 0,
        guest_time: 0,
        cguest_time: 0,
        start_data: 0,
        end_data: 0,
        start_brk: 0,
        arg_start: 0,
        arg_end: 0,
        env_start: 0,
        env_end: 0,
        exit_code: 0,
        num_filedescriptors: 0,
    };
    let mut tmp___0: bool = false;
    DiagTrace(
        b"CpuMonitoringThread: Starting Trigger Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 238\0" as *const u8 as *const libc::c_char,
    );
    config = thread_args as *mut ProcDumpConfiguration;
    totalTime = 0 as libc::c_ulong;
    elapsedTime = 0 as libc::c_ulong;
    tmp = NewCoreDumpWriter(CPU, config);
    writer = tmp;
    rc = 0 as libc::c_int;
    proc_0.pid = 0 as libc::c_int;
    proc_0.comm = 0 as *mut libc::c_char;
    proc_0.state = 0 as libc::c_int as libc::c_char;
    proc_0.ppid = 0 as libc::c_int;
    proc_0.pgrp = 0 as libc::c_uint;
    proc_0.session = 0 as libc::c_int;
    proc_0.tty_nr = 0 as libc::c_int;
    proc_0.tpgid = 0 as libc::c_uint;
    proc_0.flags = 0 as libc::c_uint;
    proc_0.minflt = 0 as libc::c_ulong;
    proc_0.cminflt = 0 as libc::c_ulong;
    proc_0.majflt = 0 as libc::c_ulong;
    proc_0.cmajflt = 0 as libc::c_ulong;
    proc_0.utime = 0 as libc::c_ulong;
    proc_0.stime = 0 as libc::c_ulong;
    proc_0.cutime = 0 as libc::c_ulong;
    proc_0.cstime = 0 as libc::c_ulong;
    proc_0.priority = 0 as libc::c_long;
    proc_0.nice = 0 as libc::c_long;
    proc_0.num_threads = 0 as libc::c_long;
    proc_0.itrealvalue = 0 as libc::c_long;
    proc_0.starttime = 0 as libc::c_ulonglong;
    proc_0.vsize = 0 as libc::c_ulong;
    proc_0.rss = 0 as libc::c_long;
    proc_0.rsslim = 0 as libc::c_ulong;
    proc_0.startcode = 0 as libc::c_ulong;
    proc_0.endcode = 0 as libc::c_ulong;
    proc_0.startstack = 0 as libc::c_ulong;
    proc_0.kstkesp = 0 as libc::c_ulong;
    proc_0.kstkeip = 0 as libc::c_ulong;
    proc_0.signal = 0 as libc::c_ulong;
    proc_0.blocked = 0 as libc::c_ulong;
    proc_0.sigignore = 0 as libc::c_ulong;
    proc_0.sigcatch = 0 as libc::c_ulong;
    proc_0.wchan = 0 as libc::c_ulong;
    proc_0.nswap = 0 as libc::c_ulong;
    proc_0.cnswap = 0 as libc::c_ulong;
    proc_0.exit_signal = 0 as libc::c_int;
    proc_0.processor = 0 as libc::c_int;
    proc_0.rt_priority = 0 as libc::c_uint;
    proc_0.policy = 0 as libc::c_uint;
    proc_0.delayacct_blkio_ticks = 0 as libc::c_ulonglong;
    proc_0.guest_time = 0 as libc::c_ulong;
    proc_0.cguest_time = 0 as libc::c_long;
    proc_0.start_data = 0 as libc::c_ulong;
    proc_0.end_data = 0 as libc::c_ulong;
    proc_0.start_brk = 0 as libc::c_ulong;
    proc_0.arg_start = 0 as libc::c_ulong;
    proc_0.arg_end = 0 as libc::c_ulong;
    proc_0.env_start = 0 as libc::c_ulong;
    proc_0.env_end = 0 as libc::c_ulong;
    proc_0.exit_code = 0 as libc::c_int;
    proc_0.num_filedescriptors = 0 as libc::c_int;
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 1 as libc::c_int {
        let mut current_block_72: u64;
        loop {
            rc = WaitForQuit(config, (*config).PollingInterval);
            if !(rc == 110 as libc::c_int) {
                break;
            }
            sysinfo(&mut sysInfo);
            tmp___0 = GetProcessStat((*config).ProcessId, &mut proc_0);
            if tmp___0 {
                totalTime = (proc_0.utime)
                    .wrapping_add(proc_0.stime)
                    .wrapping_div(HZ as libc::c_ulong);
                elapsedTime = (sysInfo.uptime
                    - (proc_0.starttime).wrapping_div(HZ as libc::c_ulonglong)
                        as libc::c_long) as libc::c_ulong;
                cpuUsage = (100 as libc::c_int as libc::c_double
                    * (totalTime as libc::c_double / elapsedTime as libc::c_double))
                    as libc::c_int;
                if (*config).bCpuTriggerBelowValue {
                    if cpuUsage < (*config).CpuThreshold {
                        current_block_72 = 7338326421160555951;
                    } else {
                        current_block_72 = 5933544542068298357;
                    }
                } else {
                    current_block_72 = 5933544542068298357;
                }
                match current_block_72 {
                    5933544542068298357 => {
                        if (*config).bCpuTriggerBelowValue {
                            continue;
                        }
                        if !(cpuUsage >= (*config).CpuThreshold) {
                            continue;
                        }
                    }
                    _ => {}
                }
                Log(info, b"CPU:\t%d%%\0" as *const u8 as *const libc::c_char, cpuUsage);
                rc = WriteCoreDump(writer);
                rc = WaitForQuit(
                    config,
                    (*config).ThresholdSeconds * 1000 as libc::c_int,
                );
                if rc != 110 as libc::c_int {
                    break;
                }
            } else {
                Log(
                    error,
                    b"An error occurred while parsing procfs\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"CpuTCpuMonitoringThread: Exiting Trigger Thread %s\0" as *const u8
            as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 285\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn TimerThread(
    mut thread_args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut config: *mut ProcDumpConfiguration = 0 as *mut ProcDumpConfiguration;
    let mut writer: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut rc: libc::c_int = 0;
    DiagTrace(
        b"TimerThread: Starting Trigger Thread %s\0" as *const u8 as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 291\0" as *const u8 as *const libc::c_char,
    );
    config = thread_args as *mut ProcDumpConfiguration;
    tmp = NewCoreDumpWriter(TIME, config);
    writer = tmp;
    rc = 0 as libc::c_int;
    rc = WaitForQuitOrEvent(
        config,
        &mut (*config).evtStartMonitoring,
        -(1 as libc::c_int),
    );
    if rc == 1 as libc::c_int {
        loop {
            rc = WaitForQuit(config, 0 as libc::c_int);
            if !(rc == 110 as libc::c_int) {
                break;
            }
            Log(info, b"Timed:\0" as *const u8 as *const libc::c_char);
            rc = WriteCoreDump(writer);
            rc = WaitForQuit(config, (*config).ThresholdSeconds * 1000 as libc::c_int);
            if rc != 110 as libc::c_int {
                break;
            }
        }
    }
    free(writer as *mut libc::c_void);
    DiagTrace(
        b"TimerThread: Exiting Trigger Thread %s\0" as *const u8 as *const libc::c_char,
        b"in src/TriggerThreadProcs.c, at line 312\0" as *const u8 as *const libc::c_char,
    );
    pthread_exit(0 as *mut libc::c_void);
}
static mut CoreDumpTypeStrings: [*const libc::c_char; 7] = [
    b"commit\0" as *const u8 as *const libc::c_char,
    b"cpu\0" as *const u8 as *const libc::c_char,
    b"thread\0" as *const u8 as *const libc::c_char,
    b"filedesc\0" as *const u8 as *const libc::c_char,
    b"signal\0" as *const u8 as *const libc::c_char,
    b"time\0" as *const u8 as *const libc::c_char,
    b"manual\0" as *const u8 as *const libc::c_char,
];
pub unsafe extern "C" fn NewCoreDumpWriter(
    mut type_0: ECoreDumpType,
    mut config: *mut ProcDumpConfiguration,
) -> *mut CoreDumpWriter {
    let mut writer: *mut CoreDumpWriter = 0 as *mut CoreDumpWriter;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<CoreDumpWriter>() as libc::c_ulong);
    writer = tmp as *mut CoreDumpWriter;
    if writer as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"NewCoreDumpWriter: failed to allocate memory. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 39\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    (*writer).Config = config;
    (*writer).Type = type_0;
    return writer;
}
pub unsafe extern "C" fn GetPath(mut lineBuf: *mut libc::c_char) -> *mut libc::c_char {
    let mut delim: [libc::c_char; 2] = [0; 2];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    delim[0 as libc::c_int as usize] = ' ' as i32 as libc::c_char;
    delim[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tmp = strtok(lineBuf, delim.as_mut_ptr() as *const libc::c_char);
    ptr = tmp;
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        ptr = strtok(
            0 as *mut libc::c_void as *mut libc::c_char,
            delim.as_mut_ptr() as *const libc::c_char,
        );
        i += 1;
    }
    if ptr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = strlen(ptr as *const libc::c_char);
        *ptr
            .offset(
                tmp___0.wrapping_sub(1 as libc::c_ulong) as isize,
            ) = '\u{0}' as i32 as libc::c_char;
    }
    return ptr;
}
pub unsafe extern "C" fn IsCoreClrProcess(
    mut self_0: *mut CoreDumpWriter,
    mut socketName: *mut *mut libc::c_char,
) -> bool {
    let mut bRet: bool = false;
    let mut procFile: *mut FILE = 0 as *mut FILE;
    let mut lineBuf: [libc::c_char; 4096] = [0; 4096];
    let mut tmpFolder: [libc::c_char; 4096] = [0; 4096];
    let mut prefixTmpFolder: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *mut libc::c_int = 0 as *mut libc::c_int;
    bRet = 0 as libc::c_int != 0;
    *socketName = 0 as *mut libc::c_void as *mut libc::c_char;
    procFile = 0 as *mut libc::c_void as *mut FILE;
    prefixTmpFolder = 0 as *mut libc::c_void as *mut libc::c_char;
    prefixTmpFolder = getenv(b"TMPDIR\0" as *const u8 as *const libc::c_char);
    if prefixTmpFolder as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        snprintf(
            tmpFolder.as_mut_ptr(),
            4096 as libc::c_int as size_t,
            b"/tmp/dotnet-diagnostic-%d\0" as *const u8 as *const libc::c_char,
            (*(*self_0).Config).ProcessId,
        );
    } else {
        strncpy(
            tmpFolder.as_mut_ptr(),
            prefixTmpFolder as *const libc::c_char,
            4096 as libc::c_int as size_t,
        );
    }
    procFile = fopen(
        b"/proc/net/unix\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if procFile as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        fgets(
            lineBuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                as libc::c_int,
            procFile,
        );
        loop {
            tmp___7 = fgets(lineBuf.as_mut_ptr(), 4096 as libc::c_int, procFile);
            if !(tmp___7 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                break;
            }
            tmp = GetPath(lineBuf.as_mut_ptr());
            ptr = tmp;
            if !(ptr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                continue;
            }
            tmp___5 = strlen(tmpFolder.as_mut_ptr() as *const libc::c_char);
            tmp___6 = strncmp(
                ptr as *const libc::c_char,
                tmpFolder.as_mut_ptr() as *const libc::c_char,
                tmp___5,
            );
            if !(tmp___6 == 0 as libc::c_int) {
                continue;
            }
            tmp___0 = strlen(ptr as *const libc::c_char);
            tmp___1 = malloc(
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(tmp___0)
                    .wrapping_add(1 as libc::c_ulong),
            );
            *socketName = tmp___1 as *mut libc::c_char;
            if !(*socketName as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
            {
                continue;
            }
            tmp___2 = strlen(ptr as *const libc::c_char);
            memset(
                *socketName as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(tmp___2)
                    .wrapping_add(1 as libc::c_ulong),
            );
            tmp___3 = strlen(ptr as *const libc::c_char);
            tmp___4 = strncpy(
                *socketName,
                ptr as *const libc::c_char,
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(tmp___3)
                    .wrapping_add(1 as libc::c_ulong),
            );
            if tmp___4 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                DiagTrace(
                    b"CoreCLR diagnostics socket: %s %s\0" as *const u8
                        as *const libc::c_char,
                    socketName,
                    b"in src/CoreDumpWriter.c, at line 140\0" as *const u8
                        as *const libc::c_char,
                );
                bRet = 1 as libc::c_int != 0;
            }
            break;
        }
    } else {
        tmp___8 = __errno_location();
        DiagTrace(
            b"Failed to open /proc/net/unix [%d]. %s\0" as *const u8
                as *const libc::c_char,
            *tmp___8,
            b"in src/CoreDumpWriter.c, at line 151\0" as *const u8 as *const libc::c_char,
        );
    }
    if procFile as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        fclose(procFile);
        procFile = 0 as *mut libc::c_void as *mut FILE;
    }
    if *socketName as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if bRet as libc::c_int == 0 as libc::c_int {
            free(*socketName as *mut libc::c_void);
            *socketName = 0 as *mut libc::c_void as *mut libc::c_char;
        }
    }
    return bRet;
}
pub unsafe extern "C" fn WriteCoreDump(mut self_0: *mut CoreDumpWriter) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut socketName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    rc = 0 as libc::c_int;
    rc = WaitForQuitOrEvent(
        (*self_0).Config,
        &mut (*(*self_0).Config).semAvailableDumpSlots,
        -(1 as libc::c_int),
    );
    if rc == 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDump: failed WaitForQuitOrEvent. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 189\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    tmp = pthread_setcanceltype(
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    if tmp != 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDump: failed pthread_setcanceltype. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 194\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    match rc {
        1 => {
            socketName = 0 as *mut libc::c_void as *mut libc::c_char;
            IsCoreClrProcess(self_0, &mut socketName);
            rc = WriteCoreDumpInternal(self_0, socketName);
            if rc == 0 as libc::c_int {
                tmp___0 = sem_post(
                    &mut (*(*self_0).Config)
                        .semAvailableDumpSlots
                        .__annonCompField4
                        .semaphore,
                );
                if tmp___0 == -(1 as libc::c_int) {
                    Log(
                        error,
                        b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                            as *const u8 as *const libc::c_char,
                    );
                    DiagTrace(
                        b"WriteCoreDump: failed sem_post. %s\0" as *const u8
                            as *const libc::c_char,
                        b"in src/CoreDumpWriter.c, at line 208\0" as *const u8
                            as *const libc::c_char,
                    );
                    if !socketName.is_null() {
                        free(socketName as *mut libc::c_void);
                    }
                    exit(-(1 as libc::c_int));
                }
            }
            if !socketName.is_null() {
                free(socketName as *mut libc::c_void);
            }
        }
        0 | 128 => {}
        _ => {
            DiagTrace(
                b"WriteCoreDump: Error in default case %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/CoreDumpWriter.c, at line 219\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    tmp___1 = pthread_setcanceltype(
        1 as libc::c_int,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    if tmp___1 != 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDump: failed pthread_setcanceltype. %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 224\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    return rc;
}
pub unsafe extern "C" fn GetUint16(mut buffer: *mut libc::c_char) -> *mut uint16_t {
    let mut dumpFileNameW: *mut uint16_t = 0 as *mut uint16_t;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut tmp___1: size_t = 0;
    dumpFileNameW = 0 as *mut libc::c_void as *mut uint16_t;
    if buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = strlen(buffer as *const libc::c_char);
        tmp___0 = malloc(
            tmp
                .wrapping_add(1 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<uint16_t>() as libc::c_ulong),
        );
        dumpFileNameW = tmp___0 as *mut uint16_t;
        i = 0 as libc::c_int;
        loop {
            tmp___1 = strlen(buffer as *const libc::c_char);
            if !((i as size_t) < tmp___1.wrapping_add(1 as libc::c_ulong)) {
                break;
            }
            *dumpFileNameW.offset(i as isize) = *buffer.offset(i as isize) as uint16_t;
            i += 1;
        }
    }
    return dumpFileNameW;
}
pub unsafe extern "C" fn GenerateCoreClrDump(
    mut socketName: *mut libc::c_char,
    mut dumpFileName: *mut libc::c_char,
) -> bool {
    let mut bRet: bool = false;
    let mut addr: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut fd: libc::c_int = 0;
    let mut dumpFileNameW: *mut uint16_t = 0 as *mut uint16_t;
    let mut temp_buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dumpFileNameLen: libc::c_uint = 0;
    let mut tmp: size_t = 0;
    let mut payloadSize: libc::c_int = 0;
    let mut dumpType: libc::c_uint = 0;
    let mut diagnostics: libc::c_uint = 0;
    let mut totalPacketSize: uint16_t = 0;
    let mut dumpHeader: IpcHeader = IpcHeader {
        __annonCompField6: __anonunion____missing_field_name_584942040 {
            _magic: MagicVersion { Magic: [0; 14] },
        },
        Size: 0,
        CommandSet: 0,
        CommandId: 0,
        Reserved: 0,
    };
    let mut temp_buffer_cur: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut retHeader: IpcHeader = IpcHeader {
        __annonCompField6: __anonunion____missing_field_name_584942040 {
            _magic: MagicVersion { Magic: [0; 14] },
        },
        Size: 0,
        CommandSet: 0,
        CommandId: 0,
        Reserved: 0,
    };
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut res: int32_t = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: ssize_t = 0;
    let mut tmp___4: ssize_t = 0;
    let mut tmp___5: ssize_t = 0;
    let mut tmp___6: libc::c_int = 0;
    bRet = 0 as libc::c_int != 0;
    addr.sun_family = 0 as libc::c_int as sa_family_t;
    addr.sun_path[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[6 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[7 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[9 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[10 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[11 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[12 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[13 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[14 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[15 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[16 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[17 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[18 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[19 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[20 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[21 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[22 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[23 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[24 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[25 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[26 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[27 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[28 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[29 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[30 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[31 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[32 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[33 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[34 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[35 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[36 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[37 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[38 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[39 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[40 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[41 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[42 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[43 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[44 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[45 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[46 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[47 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[48 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[49 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[50 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[51 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[52 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[53 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[54 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[55 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[56 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[57 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[58 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[59 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[60 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[61 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[62 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[63 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[64 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[65 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[66 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[67 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[68 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[69 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[70 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[71 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[72 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[73 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[74 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[75 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[76 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[77 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[78 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[79 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[80 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[81 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[82 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[83 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[84 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[85 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[86 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[87 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[88 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[89 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[90 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[91 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[92 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[93 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[94 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[95 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[96 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[97 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[98 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[99 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[100 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[101 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[102 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[103 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[104 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[105 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[106 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    addr.sun_path[107 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    fd = 0 as libc::c_int;
    dumpFileNameW = 0 as *mut libc::c_void as *mut uint16_t;
    temp_buffer = 0 as *mut libc::c_void;
    dumpFileNameW = GetUint16(dumpFileName);
    if dumpFileNameW as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        fd = socket(1 as libc::c_int, 1 as libc::c_int, 0 as libc::c_int);
        if fd == -(1 as libc::c_int) {
            DiagTrace(
                b"Failed to create socket for .NET Core dump generation. %s\0"
                    as *const u8 as *const libc::c_char,
                b"in src/CoreDumpWriter.c, at line 276\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            memset(
                &mut addr as *mut sockaddr_un as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong,
            );
            addr.sun_family = 1 as libc::c_int as sa_family_t;
            strncpy(
                (addr.sun_path).as_mut_ptr(),
                socketName as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong),
            );
            tmp___6 = connect(
                fd,
                &mut addr as *mut sockaddr_un as *mut sockaddr as *const sockaddr,
                ::std::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
            );
            if tmp___6 == -(1 as libc::c_int) {
                DiagTrace(
                    b"Failed to connect to socket for .NET Core dump generation. %s\0"
                        as *const u8 as *const libc::c_char,
                    b"in src/CoreDumpWriter.c, at line 287\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                tmp = strlen(dumpFileName as *const libc::c_char);
                dumpFileNameLen = tmp.wrapping_add(1 as libc::c_ulong) as libc::c_uint;
                payloadSize = ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                    as libc::c_int;
                payloadSize = (payloadSize as libc::c_ulong)
                    .wrapping_add(
                        (dumpFileNameLen as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<wchar_t>() as libc::c_ulong,
                            ),
                    ) as libc::c_int;
                dumpType = 4 as libc::c_uint;
                payloadSize = (payloadSize as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    as libc::c_int;
                diagnostics = 0 as libc::c_uint;
                payloadSize = (payloadSize as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    as libc::c_int;
                totalPacketSize = (::std::mem::size_of::<IpcHeader>() as libc::c_ulong)
                    .wrapping_add(payloadSize as libc::c_ulong) as uint16_t;
                temp_buffer = malloc(totalPacketSize as size_t);
                if temp_buffer as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    memset(temp_buffer, 0 as libc::c_int, totalPacketSize as size_t);
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[0 as libc::c_int as usize] = 'D' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[1 as libc::c_int as usize] = 'O' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[2 as libc::c_int as usize] = 'T' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[3 as libc::c_int as usize] = 'N' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[4 as libc::c_int as usize] = 'E' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[5 as libc::c_int as usize] = 'T' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[6 as libc::c_int as usize] = '_' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[7 as libc::c_int as usize] = 'I' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[8 as libc::c_int as usize] = 'P' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[9 as libc::c_int as usize] = 'C' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[10 as libc::c_int as usize] = '_' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[11 as libc::c_int as usize] = 'V' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[12 as libc::c_int as usize] = '1' as i32 as uint8_t;
                    dumpHeader
                        .__annonCompField6
                        ._magic
                        .Magic[13 as libc::c_int as usize] = '\u{0}' as i32 as uint8_t;
                    dumpHeader.Size = totalPacketSize;
                    dumpHeader.CommandSet = 1 as libc::c_int as uint8_t;
                    dumpHeader.CommandId = 1 as libc::c_int as uint8_t;
                    dumpHeader.Reserved = 0 as libc::c_int as uint16_t;
                    temp_buffer_cur = temp_buffer;
                    memcpy(
                        temp_buffer_cur,
                        &mut dumpHeader as *mut IpcHeader as *const libc::c_void,
                        ::std::mem::size_of::<IpcHeader>() as libc::c_ulong,
                    );
                    temp_buffer_cur = temp_buffer_cur
                        .offset(
                            ::std::mem::size_of::<IpcHeader>() as libc::c_ulong as isize,
                        );
                    memcpy(
                        temp_buffer_cur,
                        &mut dumpFileNameLen as *mut libc::c_uint as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    );
                    temp_buffer_cur = temp_buffer_cur
                        .offset(
                            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                                as isize,
                        );
                    memcpy(
                        temp_buffer_cur,
                        dumpFileNameW as *const libc::c_void,
                        (dumpFileNameLen as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                            ),
                    );
                    temp_buffer_cur = temp_buffer_cur
                        .offset(
                            (dumpFileNameLen as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                                ) as isize,
                        );
                    memcpy(
                        temp_buffer_cur,
                        &mut dumpType as *mut libc::c_uint as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    );
                    temp_buffer_cur = temp_buffer_cur
                        .offset(
                            ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                                as isize,
                        );
                    memcpy(
                        temp_buffer_cur,
                        &mut diagnostics as *mut libc::c_uint as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong,
                    );
                    tmp___5 = send(
                        fd,
                        temp_buffer as *const libc::c_void,
                        totalPacketSize as size_t,
                        0 as libc::c_int,
                    );
                    if tmp___5 == -(1 as libc::c_long) {
                        tmp___0 = __errno_location();
                        DiagTrace(
                            b"Failed sending packet to diagnostics server [%d] %s\0"
                                as *const u8 as *const libc::c_char,
                            *tmp___0,
                            b"in src/CoreDumpWriter.c, at line 336\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        tmp___4 = recv(
                            fd,
                            &mut retHeader as *mut IpcHeader as *mut libc::c_void,
                            ::std::mem::size_of::<IpcHeader>() as libc::c_ulong,
                            0 as libc::c_int,
                        );
                        if tmp___4 == -(1 as libc::c_long) {
                            tmp___1 = __errno_location();
                            DiagTrace(
                                b"Failed receiving response header from diagnostics server [%d] %s\0"
                                    as *const u8 as *const libc::c_char,
                                *tmp___1,
                                b"in src/CoreDumpWriter.c, at line 344\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else if retHeader.Size as libc::c_int != 24 as libc::c_int {
                            DiagTrace(
                                b"Failed validating header size in response header from diagnostics server [%d != 24] %s\0"
                                    as *const u8 as *const libc::c_char,
                                retHeader.Size as libc::c_int,
                                b"in src/CoreDumpWriter.c, at line 351\0" as *const u8
                                    as *const libc::c_char,
                            );
                        } else {
                            res = -(1 as libc::c_int);
                            tmp___3 = recv(
                                fd,
                                &mut res as *mut int32_t as *mut libc::c_void,
                                ::std::mem::size_of::<int32_t>() as libc::c_ulong,
                                0 as libc::c_int,
                            );
                            if tmp___3 == -(1 as libc::c_long) {
                                tmp___2 = __errno_location();
                                DiagTrace(
                                    b"Failed receiving result code from response payload from diagnostics server [%d] %s\0"
                                        as *const u8 as *const libc::c_char,
                                    *tmp___2,
                                    b"in src/CoreDumpWriter.c, at line 359\0" as *const u8
                                        as *const libc::c_char,
                                );
                            } else if res == 0 as libc::c_int {
                                bRet = 1 as libc::c_int != 0;
                            }
                        }
                    }
                }
            }
        }
    }
    if temp_buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free(temp_buffer);
        temp_buffer = 0 as *mut libc::c_void;
    }
    if fd != 0 as libc::c_int {
        close(fd);
        fd = 0 as libc::c_int;
    }
    if dumpFileNameW as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free(dumpFileNameW as *mut libc::c_void);
        dumpFileNameW = 0 as *mut libc::c_void as *mut uint16_t;
    }
    return bRet;
}
pub unsafe extern "C" fn WriteCoreDumpInternal(
    mut self_0: *mut CoreDumpWriter,
    mut socketName: *mut libc::c_char,
) -> libc::c_int {
    let mut date: [libc::c_char; 26] = [0; 26];
    let mut command: [libc::c_char; 1024] = [0; 1024];
    let mut outputBuffer: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut lineBuffer: [libc::c_char; 1024] = [0; 1024];
    let mut gcorePrefixName: [libc::c_char; 1024] = [0; 1024];
    let mut coreDumpFileName: [libc::c_char; 1024] = [0; 1024];
    let mut lineLength: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut rawTime: time_t = 0;
    let mut gcorePid: pid_t = 0;
    let mut timerInfo: *mut tm = 0 as *mut tm;
    let mut commandPipe: *mut FILE = 0 as *mut FILE;
    let mut desc: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pid: pid_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: bool = false;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j___0: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___13: libc::c_int = 0;
    rc = 0 as libc::c_int;
    timerInfo = 0 as *mut libc::c_void as *mut tm;
    commandPipe = 0 as *mut libc::c_void as *mut FILE;
    desc = CoreDumpTypeStrings[(*self_0).Type as usize];
    tmp = sanitize((*(*self_0).Config).ProcessName);
    name = tmp;
    pid = (*(*self_0).Config).ProcessId;
    tmp___0 = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(15 as libc::c_ulong),
    );
    outputBuffer = tmp___0 as *mut *mut libc::c_char;
    if outputBuffer as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: failed gcore output buffer allocation %s\0"
                as *const u8 as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 428\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    rawTime = time(0 as *mut libc::c_void as *mut time_t);
    timerInfo = localtime(&mut rawTime as *mut time_t as *const time_t);
    if timerInfo as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: failed localtime %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 436\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    strftime(
        date.as_mut_ptr(),
        26 as libc::c_int as size_t,
        b"%Y-%m-%d_%H:%M:%S\0" as *const u8 as *const libc::c_char,
        timerInfo as *const tm,
    );
    if (*(*self_0).Config).CoreDumpName as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        tmp___1 = snprintf(
            gcorePrefixName.as_mut_ptr(),
            1024 as libc::c_int as size_t,
            b"%s/%s_%d\0" as *const u8 as *const libc::c_char,
            (*(*self_0).Config).CoreDumpPath,
            (*(*self_0).Config).CoreDumpName,
            (*(*self_0).Config).NumberOfDumpsCollected,
        );
        if tmp___1 < 0 as libc::c_int {
            Log(
                error,
                b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                    as *const u8 as *const libc::c_char,
            );
            DiagTrace(
                b"WriteCoreDumpInternal: failed sprintf custom output file name %s\0"
                    as *const u8 as *const libc::c_char,
                b"in src/CoreDumpWriter.c, at line 448\0" as *const u8
                    as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
    } else {
        tmp___2 = snprintf(
            gcorePrefixName.as_mut_ptr(),
            1024 as libc::c_int as size_t,
            b"%s/%s_%s_%s\0" as *const u8 as *const libc::c_char,
            (*(*self_0).Config).CoreDumpPath,
            name,
            desc,
            date.as_mut_ptr(),
        );
        if tmp___2 < 0 as libc::c_int {
            Log(
                error,
                b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                    as *const u8 as *const libc::c_char,
            );
            DiagTrace(
                b"WriteCoreDumpInternal: failed sprintf default output file name %s\0"
                    as *const u8 as *const libc::c_char,
                b"in src/CoreDumpWriter.c, at line 455\0" as *const u8
                    as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
    }
    tmp___3 = snprintf(
        command.as_mut_ptr(),
        1024 as libc::c_int as size_t,
        b"gcore -o %s %d 2>&1\0" as *const u8 as *const libc::c_char,
        gcorePrefixName.as_mut_ptr(),
        pid,
    );
    if tmp___3 < 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: failed sprintf gcore command %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 463\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    tmp___4 = snprintf(
        coreDumpFileName.as_mut_ptr(),
        1024 as libc::c_int as size_t,
        b"%s.%d\0" as *const u8 as *const libc::c_char,
        gcorePrefixName.as_mut_ptr(),
        pid,
    );
    if tmp___4 < 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: failed sprintf core file name %s\0" as *const u8
                as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 470\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    tmp___5 = access(
        (*(*self_0).Config).CoreDumpPath as *const libc::c_char,
        2 as libc::c_int,
    );
    if tmp___5 < 0 as libc::c_int {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"WriteCoreDumpInternal: no write permission to core dump target file %s %s\0"
                as *const u8 as *const libc::c_char,
            coreDumpFileName.as_mut_ptr(),
            b"in src/CoreDumpWriter.c, at line 478\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    if socketName as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___6 = GenerateCoreClrDump(socketName, coreDumpFileName.as_mut_ptr());
        if tmp___6 as libc::c_int == 0 as libc::c_int {
            Log(
                error,
                b"An error occurred while generating the core dump for .NET 3.x+ process\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            Log(
                info,
                b"Core dump %d generated: %s\0" as *const u8 as *const libc::c_char,
                (*(*self_0).Config).NumberOfDumpsCollected,
                coreDumpFileName.as_mut_ptr(),
            );
            (*(*self_0).Config).NumberOfDumpsCollected += 1;
            if (*(*self_0).Config).NumberOfDumpsCollected
                >= (*(*self_0).Config).NumberOfDumpsToCollect
            {
                SetEvent(&mut (*(*self_0).Config).evtQuit.__annonCompField4.event);
                rc = 1 as libc::c_int;
            }
        }
        free(outputBuffer as *mut libc::c_void);
    } else {
        commandPipe = popen2(
            command.as_mut_ptr() as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
            &mut gcorePid,
        );
        (*(*self_0).Config).gcorePid = gcorePid;
        if commandPipe as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            Log(
                error,
                b"An error occurred while generating the core dump\0" as *const u8
                    as *const libc::c_char,
            );
            DiagTrace(
                b"WriteCoreDumpInternal: Failed to open pipe to gcore %s\0" as *const u8
                    as *const libc::c_char,
                b"in src/CoreDumpWriter.c, at line 512\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        i = 0 as libc::c_int;
        while i < 15 as libc::c_int {
            tmp___9 = fgets(
                lineBuffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                commandPipe,
            );
            if !(tmp___9 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                break;
            }
            tmp___7 = strlen(lineBuffer.as_mut_ptr() as *const libc::c_char);
            lineLength = tmp___7 as libc::c_int;
            tmp___8 = malloc(
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(lineLength as libc::c_ulong),
            );
            let ref mut fresh3 = *outputBuffer.offset(i as isize);
            *fresh3 = tmp___8 as *mut libc::c_char;
            if *outputBuffer.offset(i as isize) as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong
            {
                strncpy(
                    *outputBuffer.offset(i as isize),
                    lineBuffer.as_mut_ptr() as *const libc::c_char,
                    (lineLength - 1 as libc::c_int) as size_t,
                );
                *(*outputBuffer.offset(i as isize))
                    .offset(
                        (lineLength - 1 as libc::c_int) as isize,
                    ) = '\u{0}' as i32 as libc::c_char;
            } else {
                Log(
                    error,
                    b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                        as *const u8 as *const libc::c_char,
                );
                DiagTrace(
                    b"WriteCoreDumpInternal: failed to allocate gcore error message buffer %s\0"
                        as *const u8 as *const libc::c_char,
                    b"in src/CoreDumpWriter.c, at line 527\0" as *const u8
                        as *const libc::c_char,
                );
                exit(-(1 as libc::c_int));
            }
            i += 1;
        }
        (*(*self_0).Config).gcorePid = 2147483647 as libc::c_int;
        pclose(commandPipe);
        tmp___10 = strstr(
            *outputBuffer.offset((i - 1 as libc::c_int) as isize) as *const libc::c_char,
            b"gcore: failed\0" as *const u8 as *const libc::c_char,
        );
        if tmp___10 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            Log(
                error,
                b"An error occurred while generating the core dump\0" as *const u8
                    as *const libc::c_char,
            );
            j = 0 as libc::c_int;
            while j < i {
                if *outputBuffer.offset(j as isize) as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    Log(
                        error,
                        b"GCORE - %s\0" as *const u8 as *const libc::c_char,
                        *outputBuffer.offset(j as isize),
                    );
                }
                j += 1;
            }
            exit(1 as libc::c_int);
        }
        j___0 = 0 as libc::c_int;
        while j___0 < i {
            free(*outputBuffer.offset(j___0 as isize) as *mut libc::c_void);
            j___0 += 1;
        }
        free(outputBuffer as *mut libc::c_void);
        sleep(1 as libc::c_uint);
        tmp___13 = access(
            coreDumpFileName.as_mut_ptr() as *const libc::c_char,
            0 as libc::c_int,
        );
        if tmp___13 != -(1 as libc::c_int) {
            if (*(*self_0).Config).nQuit != 0 {
                tmp___11 = unlink(coreDumpFileName.as_mut_ptr() as *const libc::c_char);
                ret = tmp___11;
                if ret < 0 as libc::c_int {
                    tmp___12 = __errno_location();
                    if *tmp___12 != 2 as libc::c_int {
                        DiagTrace(
                            b"WriteCoreDumpInternal: Failed to remove partial core dump %s\0"
                                as *const u8 as *const libc::c_char,
                            b"in src/CoreDumpWriter.c, at line 564\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(-(1 as libc::c_int));
                    }
                }
            } else {
                Log(
                    info,
                    b"Core dump %d generated: %s\0" as *const u8 as *const libc::c_char,
                    (*(*self_0).Config).NumberOfDumpsCollected,
                    coreDumpFileName.as_mut_ptr(),
                );
                (*(*self_0).Config).NumberOfDumpsCollected += 1;
                if (*(*self_0).Config).NumberOfDumpsCollected
                    >= (*(*self_0).Config).NumberOfDumpsToCollect
                {
                    SetEvent(&mut (*(*self_0).Config).evtQuit.__annonCompField4.event);
                    rc = 1 as libc::c_int;
                }
            }
        }
    }
    free(name as *mut libc::c_void);
    return rc;
}
pub unsafe extern "C" fn popen2(
    mut command: *const libc::c_char,
    mut type_0: *const libc::c_char,
    mut pid: *mut pid_t,
) -> *mut FILE {
    let mut pipefd: [libc::c_int; 2] = [0; 2];
    let mut childPid: pid_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut FILE = 0 as *mut FILE;
    let mut tmp___1: *mut FILE = 0 as *mut FILE;
    tmp = pipe(pipefd.as_mut_ptr());
    if tmp == -(1 as libc::c_int) {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"popen2: unable to open pipe %s\0" as *const u8 as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 605\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    childPid = fork();
    if childPid == -(1 as libc::c_int) {
        Log(
            error,
            b"Internal Error has occurred. If problem continues to occur run procdump with -d flag to trace issue\0"
                as *const u8 as *const libc::c_char,
        );
        DiagTrace(
            b"popen2: unable to fork process %s\0" as *const u8 as *const libc::c_char,
            b"in src/CoreDumpWriter.c, at line 612\0" as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    if childPid == 0 as libc::c_int {
        setpgid(0 as libc::c_int, 0 as libc::c_int);
        if *type_0.offset(0 as libc::c_int as isize) as libc::c_int == 114 as libc::c_int
        {
            close(pipefd[0 as libc::c_int as usize]);
            dup2(pipefd[1 as libc::c_int as usize], 1 as libc::c_int);
        } else {
            close(pipefd[1 as libc::c_int as usize]);
            dup2(pipefd[0 as libc::c_int as usize], 0 as libc::c_int);
        }
        execl(
            b"/bin/bash\0" as *const u8 as *const libc::c_char,
            b"bash\0" as *const u8 as *const libc::c_char,
            b"-c\0" as *const u8 as *const libc::c_char,
            command,
            0 as *mut libc::c_void as *mut libc::c_char,
        );
        return 0 as *mut libc::c_void as *mut FILE;
    } else {
        setpgid(childPid, childPid);
        *pid = childPid;
        if *type_0.offset(0 as libc::c_int as isize) as libc::c_int == 114 as libc::c_int
        {
            close(pipefd[1 as libc::c_int as usize]);
            tmp___0 = fdopen(
                pipefd[0 as libc::c_int as usize],
                b"r\0" as *const u8 as *const libc::c_char,
            );
            return tmp___0;
        } else {
            close(pipefd[0 as libc::c_int as usize]);
            tmp___1 = fdopen(
                pipefd[1 as libc::c_int as usize],
                b"w\0" as *const u8 as *const libc::c_char,
            );
            return tmp___1;
        }
    };
}
pub unsafe extern "C" fn sanitize(
    mut processName: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut sanitizedProcessName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___1: size_t = 0;
    if processName as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        Log(error, b"NULL process name.\n\0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    tmp = strdup(processName as *const libc::c_char);
    sanitizedProcessName = tmp;
    i = 0 as libc::c_int;
    loop {
        tmp___1 = strlen(sanitizedProcessName as *const libc::c_char);
        if !((i as size_t) < tmp___1) {
            break;
        }
        tmp___0 = __ctype_b_loc();
        if *(*tmp___0)
            .offset(*sanitizedProcessName.offset(i as isize) as libc::c_int as isize)
            as libc::c_int & 8 as libc::c_int == 0
        {
            *sanitizedProcessName.offset(i as isize) = '_' as i32 as libc::c_char;
        }
        i += 1;
    }
    return sanitizedProcessName;
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
