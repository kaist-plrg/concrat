use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn exit(_: libc::c_int) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> libc::c_int;
    fn timer_settime(
        __timerid: timer_t,
        __flags: libc::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> libc::c_int;
    fn timer_gettime(__timerid: timer_t, __value: *mut itimerspec) -> libc::c_int;
    fn timer_getoverrun(__timerid: timer_t) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn ftime(__timebuf: *mut timeb) -> libc::c_int;
    fn lstat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
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
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
    fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> libc::c_int;
    fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: __clockid_t,
    ) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __timer_t = *mut libc::c_void;
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
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
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
pub union __anonunion____missing_field_name_648058633 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_366146944 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_366146943 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_366146944,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_648058633,
    pub __annonCompField2: __anonunion____missing_field_name_366146943,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
pub type pthread_t = libc::c_ulong;
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
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigevent {
    pub sigev_value: __sigval_t,
    pub sigev_signo: libc::c_int,
    pub sigev_notify: libc::c_int,
    pub _sigev_un: __anonunion__sigev_un_577170306,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion__sigev_un_577170306 {
    pub _pad: [libc::c_int; 12],
    pub _tid: __pid_t,
    pub _sigev_thread: __anonstruct__sigev_thread_746770901,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigev_thread_746770901 {
    pub _function: Option::<unsafe extern "C" fn(__sigval_t) -> ()>,
    pub _attribute: *mut pthread_attr_t,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeb {
    pub time: time_t,
    pub millitm: libc::c_ushort,
    pub timezone: libc::c_short,
    pub dstflag: libc::c_short,
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
pub union __anonunion__bounds_445958387 {
    pub _addr_bnd: __anonstruct__addr_bnd_5259977,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigfault_2227063 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: __anonunion__bounds_445958387,
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
pub union __anonunion__sifields_838399793 {
    pub _pad: [libc::c_int; 28],
    pub _kill: __anonstruct__kill_244518854,
    pub _timer: __anonstruct__timer_490064738,
    pub _rt: __anonstruct__rt_619254530,
    pub _sigchld: __anonstruct__sigchld_284671705,
    pub _sigfault: __anonstruct__sigfault_2227063,
    pub _sigpoll: __anonstruct__sigpoll_386613454,
    pub _sigsys: __anonstruct__sigsys_44812255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_siginfo_t_1066041944 {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: __anonunion__sifields_838399793,
}
pub type siginfo_t = __anonstruct_siginfo_t_1066041944;
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
unsafe extern "C" fn handler(
    mut sig: libc::c_int,
    mut si: *mut siginfo_t,
    mut uc: *mut libc::c_void,
) {
    if si as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        printf(b"Caught signal %d\n\0" as *const u8 as *const libc::c_char, sig);
    } else if si as libc::c_ulong != uc as libc::c_ulong {
        printf(b"Caught signal %d\n\0" as *const u8 as *const libc::c_char, sig);
    }
}
pub unsafe extern "C" fn pthread_test(mut args: *mut libc::c_void) -> *mut libc::c_void {
    let mut fakeMutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
    };
    let mut fakeCond: pthread_cond_t = __anonunion_pthread_cond_t_951761805 {
        __data: __pthread_cond_s {
            __annonCompField1: __anonunion____missing_field_name_648058633 {
                __wseq: 0,
            },
            __annonCompField2: __anonunion____missing_field_name_366146943 {
                __g1_start: 0,
            },
            __g_refs: [0; 2],
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
        },
    };
    let mut monotonic_cond: pthread_cond_t = __anonunion_pthread_cond_t_951761805 {
        __data: __pthread_cond_s {
            __annonCompField1: __anonunion____missing_field_name_648058633 {
                __wseq: 0,
            },
            __annonCompField2: __anonunion____missing_field_name_366146943 {
                __g1_start: 0,
            },
            __g_refs: [0; 2],
            __g_size: [0; 2],
            __g1_orig_size: 0,
            __wrefs: 0,
            __g_signals: [0; 2],
        },
    };
    let mut attr: pthread_condattr_t = __anonunion_pthread_condattr_t_488594145 {
        __size: [0; 4],
    };
    let mut timeToWait: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut rt: libc::c_int = 0;
    fakeMutex.__data.__lock = 0 as libc::c_int;
    fakeMutex.__data.__count = 0 as libc::c_uint;
    fakeMutex.__data.__owner = 0 as libc::c_int;
    fakeMutex.__data.__nusers = 0 as libc::c_uint;
    fakeMutex.__data.__kind = 0 as libc::c_int;
    fakeMutex.__data.__spins = 0 as libc::c_int as libc::c_short;
    fakeMutex.__data.__elision = 0 as libc::c_int as libc::c_short;
    fakeMutex.__data.__list.__prev = 0 as *mut __pthread_internal_list;
    fakeMutex.__data.__list.__next = 0 as *mut __pthread_internal_list;
    fakeCond.__data.__annonCompField1.__wseq = 0 as libc::c_ulonglong;
    fakeCond.__data.__annonCompField2.__g1_start = 0 as libc::c_ulonglong;
    fakeCond.__data.__g_refs[0 as libc::c_int as usize] = 0 as libc::c_uint;
    fakeCond.__data.__g_refs[1 as libc::c_int as usize] = 0 as libc::c_uint;
    fakeCond.__data.__g_size[0 as libc::c_int as usize] = 0 as libc::c_uint;
    fakeCond.__data.__g_size[1 as libc::c_int as usize] = 0 as libc::c_uint;
    fakeCond.__data.__g1_orig_size = 0 as libc::c_uint;
    fakeCond.__data.__wrefs = 0 as libc::c_uint;
    fakeCond.__data.__g_signals[0 as libc::c_int as usize] = 0 as libc::c_uint;
    fakeCond.__data.__g_signals[1 as libc::c_int as usize] = 0 as libc::c_uint;
    args = args;
    clock_gettime(0 as libc::c_int, &mut now);
    timeToWait.tv_sec = now.tv_sec + 1 as libc::c_long;
    timeToWait.tv_nsec = now.tv_nsec;
    printf(
        b"pthread_cond_timedwait: CLOCK_REALTIME test\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"(Intentionally sleeping 1 second...)\n\0" as *const u8 as *const libc::c_char,
    );
    fflush(stdout);
    pthread_mutex_lock(&mut fakeMutex);
    rt = pthread_cond_timedwait(
        &mut fakeCond as *mut pthread_cond_t,
        &mut fakeMutex as *mut pthread_mutex_t,
        &mut timeToWait as *mut timespec as *const timespec,
    );
    if rt != 110 as libc::c_int {
        printf(b"pthread_cond_timedwait failed\n\0" as *const u8 as *const libc::c_char);
        pthread_mutex_unlock(&mut fakeMutex);
        exit(1 as libc::c_int);
    }
    pthread_mutex_unlock(&mut fakeMutex);
    pthread_condattr_init(&mut attr);
    pthread_condattr_setclock(&mut attr, 1 as libc::c_int);
    pthread_cond_init(
        &mut monotonic_cond as *mut pthread_cond_t,
        &mut attr as *mut pthread_condattr_t as *const pthread_condattr_t,
    );
    clock_gettime(1 as libc::c_int, &mut now);
    timeToWait.tv_sec = now.tv_sec + 1 as libc::c_long;
    timeToWait.tv_nsec = now.tv_nsec;
    printf(
        b"pthread_cond_timedwait: CLOCK_MONOTONIC test\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"(Intentionally sleeping 1 second...)\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"(If this test hangs for more than a few seconds, please report\n your glibc version and system details as FORCE_MONOTONIC_FIX\n issue at https://github.com/wolfcw/libfaketime)\n\0"
            as *const u8 as *const libc::c_char,
    );
    fflush(stdout);
    pthread_mutex_lock(&mut fakeMutex);
    rt = pthread_cond_timedwait(
        &mut monotonic_cond as *mut pthread_cond_t,
        &mut fakeMutex as *mut pthread_mutex_t,
        &mut timeToWait as *mut timespec as *const timespec,
    );
    if rt != 110 as libc::c_int {
        printf(b"pthread_cond_timedwait failed\n\0" as *const u8 as *const libc::c_char);
        pthread_mutex_unlock(&mut fakeMutex);
        exit(1 as libc::c_int);
    }
    pthread_mutex_unlock(&mut fakeMutex);
    pthread_cond_destroy(&mut monotonic_cond);
    return 0 as *mut libc::c_void;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut now: time_t = 0;
    let mut tb: timeb = timeb {
        time: 0,
        millitm: 0,
        timezone: 0,
        dstflag: 0,
    };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut timerid1: timer_t = 0 as *mut libc::c_void;
    let mut timerid2: timer_t = 0 as *mut libc::c_void;
    let mut sev: sigevent = sigevent {
        sigev_value: sigval { sival_int: 0 },
        sigev_signo: 0,
        sigev_notify: 0,
        _sigev_un: __anonunion__sigev_un_577170306 {
            _pad: [0; 12],
        },
    };
    let mut its: itimerspec = itimerspec {
        it_interval: timespec { tv_sec: 0, tv_nsec: 0 },
        it_value: timespec { tv_sec: 0, tv_nsec: 0 },
    };
    let mut mask: sigset_t = sigset_t { __val: [0; 16] };
    let mut sa: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut buf: stat = stat {
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
    let mut thread: pthread_t = 0;
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: time_t = 0;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut timer_getoverrun_timerid1: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut timer_getoverrun_timerid2: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    timerid1 = 0 as timer_t;
    printf(
        b"%s\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    pthread_create(
        &mut thread as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            pthread_test as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    pthread_join(thread, &mut ret);
    sa.sa_flags = 4 as libc::c_int;
    sa
        .__sigaction_handler
        .sa_sigaction = Some(
        handler
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigemptyset(&mut sa.sa_mask);
    tmp = sigaction(
        10 as libc::c_int,
        &mut sa as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    if tmp == -(1 as libc::c_int) {
        perror(b"sigaction\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    printf(
        b"Blocking signal %d\n\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int,
    );
    sigemptyset(&mut mask);
    sigaddset(&mut mask, 10 as libc::c_int);
    tmp___0 = sigprocmask(
        2 as libc::c_int,
        &mut mask as *mut sigset_t as *const sigset_t,
        0 as *mut libc::c_void as *mut sigset_t,
    );
    if tmp___0 == -(1 as libc::c_int) {
        perror(b"sigaction\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    sev.sigev_notify = 0 as libc::c_int;
    sev.sigev_signo = 10 as libc::c_int;
    sev.sigev_value.sival_ptr = &mut timerid1 as *mut timer_t as *mut libc::c_void;
    tmp___1 = timer_create(
        0 as libc::c_int,
        &mut sev as *mut sigevent,
        &mut timerid1 as *mut timer_t,
    );
    if tmp___1 == -(1 as libc::c_int) {
        perror(b"timer_create\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    its.it_value.tv_sec = 1 as libc::c_int as __time_t;
    its.it_value.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    its.it_interval.tv_sec = 0 as libc::c_int as __time_t;
    its.it_interval.tv_nsec = 300000000 as libc::c_int as __syscall_slong_t;
    tmp___2 = timer_settime(
        timerid1,
        0 as libc::c_int,
        &mut its as *mut itimerspec as *const itimerspec,
        0 as *mut libc::c_void as *mut itimerspec,
    );
    if tmp___2 == -(1 as libc::c_int) {
        perror(b"timer_settime\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    sev.sigev_value.sival_ptr = &mut timerid2 as *mut timer_t as *mut libc::c_void;
    tmp___3 = timer_create(
        0 as libc::c_int,
        &mut sev as *mut sigevent,
        &mut timerid2 as *mut timer_t,
    );
    if tmp___3 == -(1 as libc::c_int) {
        perror(b"timer_create\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    clock_gettime(0 as libc::c_int, &mut its.it_value);
    its.it_value.tv_sec += 3 as libc::c_long;
    its.it_interval.tv_sec = 0 as libc::c_int as __time_t;
    its.it_interval.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    tmp___4 = timer_settime(
        timerid2,
        1 as libc::c_int,
        &mut its as *mut itimerspec as *const itimerspec,
        0 as *mut libc::c_void as *mut itimerspec,
    );
    if tmp___4 == -(1 as libc::c_int) {
        perror(b"timer_settime\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    time(&mut now);
    tmp___5 = ctime(&mut now as *mut time_t as *const time_t);
    printf(
        b"time()         : Current date and time: %s\0" as *const u8
            as *const libc::c_char,
        tmp___5,
    );
    tmp___6 = time(0 as *mut libc::c_void as *mut time_t);
    printf(
        b"time(NULL)     : Seconds since Epoch  : %u\n\0" as *const u8
            as *const libc::c_char,
        tmp___6 as libc::c_uint,
    );
    ftime(&mut tb);
    tmp___7 = ctime(&mut tb.time as *mut time_t as *const time_t);
    printf(
        b"ftime()        : Current date and time: %s\0" as *const u8
            as *const libc::c_char,
        tmp___7,
    );
    printf(
        b"(Intentionally sleeping 2 seconds...)\n\0" as *const u8 as *const libc::c_char,
    );
    fflush(stdout);
    if argc < 3 as libc::c_int {
        sleep(1 as libc::c_uint);
        usleep(1000000 as libc::c_int as __useconds_t);
    }
    gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
    tmp___8 = ctime(&mut tv.tv_sec as *mut __time_t as *const time_t);
    printf(
        b"gettimeofday() : Current date and time: %s\0" as *const u8
            as *const libc::c_char,
        tmp___8,
    );
    tmp___9 = sigprocmask(
        1 as libc::c_int,
        &mut mask as *mut sigset_t as *const sigset_t,
        0 as *mut libc::c_void as *mut sigset_t,
    );
    if tmp___9 == -(1 as libc::c_int) {
        perror(b"sigprocmask\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    clock_gettime(0 as libc::c_int, &mut ts);
    tmp___10 = ctime(&mut ts.tv_sec as *mut __time_t as *const time_t);
    printf(
        b"clock_gettime(): Current date and time: %s\0" as *const u8
            as *const libc::c_char,
        tmp___10,
    );
    tmp___11 = timer_getoverrun(timerid1);
    timer_getoverrun_timerid1 = tmp___11;
    if timer_getoverrun_timerid1 != 3 as libc::c_int {
        printf(
            b"timer_getoverrun(timerid1) FAILED, must be 3 but got: %d\n\0" as *const u8
                as *const libc::c_char,
            timer_getoverrun_timerid1,
        );
    }
    timer_gettime(timerid1, &mut its);
    tmp___12 = timer_getoverrun(timerid2);
    timer_getoverrun_timerid2 = tmp___12;
    if timer_getoverrun_timerid2 != 0 as libc::c_int {
        printf(
            b"timer_getoverrun(timerid2) FAILED, must be 0 but got: %d\n\0" as *const u8
                as *const libc::c_char,
            timer_getoverrun_timerid2,
        );
    }
    timer_gettime(timerid2, &mut its);
    lstat(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        &mut buf as *mut stat,
    );
    tmp___13 = ctime(&mut buf.st_mtim.tv_sec as *mut __time_t as *const time_t);
    printf(
        b"stat(): mod. time of file '%s': %s\0" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
        tmp___13,
    );
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
