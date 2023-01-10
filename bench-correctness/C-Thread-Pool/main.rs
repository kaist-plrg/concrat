#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool, rustc_private, untagged_unions)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn pthread_kill(__threadid: pthread_t, __signo: libc::c_int) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_detach(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
}
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub union __anonunion__bounds_505112296 {
    pub _addr_bnd: __anonstruct__addr_bnd_5259977,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigfault_657862099 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: __anonunion__bounds_505112296,
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
pub union __anonunion__sifields_369293474 {
    pub _pad: [libc::c_int; 28],
    pub _kill: __anonstruct__kill_244518854,
    pub _timer: __anonstruct__timer_490064738,
    pub _rt: __anonstruct__rt_619254530,
    pub _sigchld: __anonstruct__sigchld_284671705,
    pub _sigfault: __anonstruct__sigfault_657862099,
    pub _sigpoll: __anonstruct__sigpoll_386613454,
    pub _sigsys: __anonstruct__sigsys_44812255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_siginfo_t_727121837 {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: __anonunion__sifields_369293474,
}
pub type siginfo_t = __anonstruct_siginfo_t_727121837;
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
pub union __anonunion____missing_field_name_878880208 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_1034561119 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1034561118 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_1034561119,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_878880208,
    pub __annonCompField2: __anonunion____missing_field_name_1034561118,
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thpool_ {
    pub threads: *mut *mut thread,
    pub num_threads_alive: libc::c_int,
    pub num_threads_working: libc::c_int,
    pub thcount_lock: pthread_mutex_t,
    pub threads_all_idle: pthread_cond_t,
    pub jobqueue: jobqueue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jobqueue {
    pub rwmutex: pthread_mutex_t,
    pub front: *mut job,
    pub rear: *mut job,
    pub has_jobs: *mut bsem,
    pub len: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bsem {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
    pub v: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct job {
    pub prev: *mut job,
    pub function: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thread {
    pub id: libc::c_int,
    pub pthread: pthread_t,
    pub thpool_p: *mut thpool_,
}
pub type threadpool = *mut thpool_;
static mut threads_keepalive: libc::c_int = 0;
static mut threads_on_hold: libc::c_int = 0;
pub unsafe extern "C" fn thpool_init(mut num_threads: libc::c_int) -> *mut thpool_ {
    let mut thpool_p: *mut thpool_ = 0 as *mut thpool_;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n: libc::c_int = 0;
    threads_on_hold = 0 as libc::c_int;
    threads_keepalive = 1 as libc::c_int;
    if num_threads < 0 as libc::c_int {
        num_threads = 0 as libc::c_int;
    }
    tmp = malloc(::std::mem::size_of::<thpool_>() as libc::c_ulong);
    thpool_p = tmp as *mut thpool_;
    if thpool_p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"thpool_init(): Could not allocate memory for thread pool\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as *mut libc::c_void as *mut thpool_;
    }
    (*thpool_p).num_threads_alive = 0 as libc::c_int;
    (*thpool_p).num_threads_working = 0 as libc::c_int;
    tmp___0 = jobqueue_init(&mut (*thpool_p).jobqueue);
    if tmp___0 == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"thpool_init(): Could not allocate memory for job queue\n\0" as *const u8
                as *const libc::c_char,
        );
        free(thpool_p as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut thpool_;
    }
    tmp___1 = malloc(
        (num_threads as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut thread>() as libc::c_ulong),
    );
    (*thpool_p).threads = tmp___1 as *mut *mut thread;
    if (*thpool_p).threads as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"thpool_init(): Could not allocate memory for threads\n\0" as *const u8
                as *const libc::c_char,
        );
        jobqueue_destroy(&mut (*thpool_p).jobqueue);
        free(thpool_p as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut thpool_;
    }
    pthread_mutex_init(
        &mut (*thpool_p).thcount_lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    pthread_cond_init(
        &mut (*thpool_p).threads_all_idle as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    n = 0 as libc::c_int;
    while n < num_threads {
        thread_init(thpool_p, ((*thpool_p).threads).offset(n as isize), n);
        n += 1;
    }
    while (*thpool_p).num_threads_alive != num_threads {}
    return thpool_p;
}
pub unsafe extern "C" fn thpool_add_work(
    mut thpool_p: *mut thpool_,
    mut function_p: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut arg_p: *mut libc::c_void,
) -> libc::c_int {
    let mut newjob: *mut job = 0 as *mut job;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<job>() as libc::c_ulong);
    newjob = tmp as *mut job;
    if newjob as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"thpool_add_work(): Could not allocate memory for new job\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*newjob).function = function_p;
    (*newjob).arg = arg_p;
    jobqueue_push(&mut (*thpool_p).jobqueue, newjob);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn thpool_wait(mut thpool_p: *mut thpool_) {
    pthread_mutex_lock(&mut (*thpool_p).thcount_lock);
    loop {
        if (*thpool_p).jobqueue.len == 0 {
            if (*thpool_p).num_threads_working == 0 {
                break;
            }
        }
        pthread_cond_wait(
            &mut (*thpool_p).threads_all_idle as *mut pthread_cond_t,
            &mut (*thpool_p).thcount_lock as *mut pthread_mutex_t,
        );
    }
    pthread_mutex_unlock(&mut (*thpool_p).thcount_lock);
}
pub unsafe extern "C" fn thpool_destroy(mut thpool_p: *mut thpool_) {
    let mut threads_total: libc::c_int = 0;
    let mut TIMEOUT: libc::c_double = 0.;
    let mut start: time_t = 0;
    let mut end: time_t = 0;
    let mut tpassed: libc::c_double = 0.;
    let mut n: libc::c_int = 0;
    if thpool_p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    threads_total = (*thpool_p).num_threads_alive;
    threads_keepalive = 0 as libc::c_int;
    TIMEOUT = 1.0f64;
    tpassed = 0.0f64;
    time(&mut start);
    while tpassed < TIMEOUT {
        if (*thpool_p).num_threads_alive == 0 {
            break;
        }
        bsem_post_all((*thpool_p).jobqueue.has_jobs);
        time(&mut end);
        tpassed = difftime(end, start);
    }
    while (*thpool_p).num_threads_alive != 0 {
        bsem_post_all((*thpool_p).jobqueue.has_jobs);
        sleep(1 as libc::c_uint);
    }
    jobqueue_destroy(&mut (*thpool_p).jobqueue);
    n = 0 as libc::c_int;
    while n < threads_total {
        thread_destroy(*((*thpool_p).threads).offset(n as isize));
        n += 1;
    }
    free((*thpool_p).threads as *mut libc::c_void);
    free(thpool_p as *mut libc::c_void);
}
pub unsafe extern "C" fn thpool_pause(mut thpool_p: *mut thpool_) {
    let mut n: libc::c_int = 0;
    n = 0 as libc::c_int;
    while n < (*thpool_p).num_threads_alive {
        pthread_kill(
            (**((*thpool_p).threads).offset(n as isize)).pthread,
            10 as libc::c_int,
        );
        n += 1;
    }
}
pub unsafe extern "C" fn thpool_resume(mut thpool_p: *mut thpool_) {
    threads_on_hold = 0 as libc::c_int;
}
pub unsafe extern "C" fn thpool_num_threads_working(
    mut thpool_p: *mut thpool_,
) -> libc::c_int {
    return (*thpool_p).num_threads_working;
}
unsafe extern "C" fn thread_init(
    mut thpool_p: *mut thpool_,
    mut thread_p: *mut *mut thread,
    mut id: libc::c_int,
) -> libc::c_int {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<thread>() as libc::c_ulong);
    *thread_p = tmp as *mut thread;
    if *thread_p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        fprintf(
            stderr,
            b"thread_init(): Could not allocate memory for thread\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (**thread_p).thpool_p = thpool_p;
    (**thread_p).id = id;
    pthread_create(
        &mut (**thread_p).pthread as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut thread) -> *mut libc::c_void>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(Some(thread_do as unsafe extern "C" fn(*mut thread) -> *mut libc::c_void)),
        *thread_p as *mut libc::c_void,
    );
    pthread_detach((**thread_p).pthread);
    return 0 as libc::c_int;
}
unsafe extern "C" fn thread_hold(mut sig_id: libc::c_int) {
    threads_on_hold = 1 as libc::c_int;
    while threads_on_hold != 0 {
        sleep(1 as libc::c_uint);
    }
}
unsafe extern "C" fn thread_do(mut thread_p: *mut thread) -> *mut libc::c_void {
    let mut thread_name: [libc::c_char; 32] = [0; 32];
    let mut tmp: libc::c_uint = 0;
    let mut thpool_p: *mut thpool_ = 0 as *mut thpool_;
    let mut act: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut tmp___0: libc::c_int = 0;
    let mut func_buff: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()> = None;
    let mut arg_buff: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut job_p: *mut job = 0 as *mut job;
    let mut tmp___1: *mut job = 0 as *mut job;
    thread_name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 32 as libc::c_uint) {
        thread_name[tmp as usize] = 0 as libc::c_int as libc::c_char;
        tmp = tmp.wrapping_add(1);
    }
    snprintf(
        thread_name.as_mut_ptr(),
        32 as libc::c_int as size_t,
        b"thread-pool-%d\0" as *const u8 as *const libc::c_char,
        (*thread_p).id,
    );
    prctl(15 as libc::c_int, thread_name.as_mut_ptr());
    thpool_p = (*thread_p).thpool_p;
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 0 as libc::c_int;
    act
        .__sigaction_handler
        .sa_handler = Some(thread_hold as unsafe extern "C" fn(libc::c_int) -> ());
    tmp___0 = sigaction(
        10 as libc::c_int,
        &mut act as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    if tmp___0 == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"thread_do(): cannot handle SIGUSR1\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_mutex_lock(&mut (*thpool_p).thcount_lock);
    (*thpool_p)
        .num_threads_alive = ::std::ptr::read_volatile::<
        libc::c_int,
    >(&(*thpool_p).num_threads_alive as *const libc::c_int) + 1 as libc::c_int;
    pthread_mutex_unlock(&mut (*thpool_p).thcount_lock);
    while threads_keepalive != 0 {
        bsem_wait((*thpool_p).jobqueue.has_jobs);
        if threads_keepalive != 0 {
            pthread_mutex_lock(&mut (*thpool_p).thcount_lock);
            (*thpool_p)
                .num_threads_working = ::std::ptr::read_volatile::<
                libc::c_int,
            >(&(*thpool_p).num_threads_working as *const libc::c_int) + 1 as libc::c_int;
            pthread_mutex_unlock(&mut (*thpool_p).thcount_lock);
            tmp___1 = jobqueue_pull(&mut (*thpool_p).jobqueue);
            job_p = tmp___1;
            if !job_p.is_null() {
                func_buff = (*job_p).function;
                arg_buff = (*job_p).arg;
                (Some(func_buff.expect("non-null function pointer")))
                    .expect("non-null function pointer")(arg_buff);
                free(job_p as *mut libc::c_void);
            }
            pthread_mutex_lock(&mut (*thpool_p).thcount_lock);
            (*thpool_p)
                .num_threads_working = ::std::ptr::read_volatile::<
                libc::c_int,
            >(&(*thpool_p).num_threads_working as *const libc::c_int) - 1 as libc::c_int;
            if (*thpool_p).num_threads_working == 0 {
                pthread_cond_signal(&mut (*thpool_p).threads_all_idle);
            }
            pthread_mutex_unlock(&mut (*thpool_p).thcount_lock);
        }
    }
    pthread_mutex_lock(&mut (*thpool_p).thcount_lock);
    (*thpool_p)
        .num_threads_alive = ::std::ptr::read_volatile::<
        libc::c_int,
    >(&(*thpool_p).num_threads_alive as *const libc::c_int) - 1 as libc::c_int;
    pthread_mutex_unlock(&mut (*thpool_p).thcount_lock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn thread_destroy(mut thread_p: *mut thread) {
    free(thread_p as *mut libc::c_void);
}
unsafe extern "C" fn jobqueue_init(mut jobqueue_p: *mut jobqueue) -> libc::c_int {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    (*jobqueue_p).len = 0 as libc::c_int;
    (*jobqueue_p).front = 0 as *mut libc::c_void as *mut job;
    (*jobqueue_p).rear = 0 as *mut libc::c_void as *mut job;
    tmp = malloc(::std::mem::size_of::<bsem>() as libc::c_ulong);
    (*jobqueue_p).has_jobs = tmp as *mut bsem;
    if (*jobqueue_p).has_jobs as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    pthread_mutex_init(
        &mut (*jobqueue_p).rwmutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    bsem_init((*jobqueue_p).has_jobs, 0 as libc::c_int);
    return 0 as libc::c_int;
}
unsafe extern "C" fn jobqueue_clear(mut jobqueue_p: *mut jobqueue) {
    let mut tmp: *mut job = 0 as *mut job;
    while (*jobqueue_p).len != 0 {
        tmp = jobqueue_pull(jobqueue_p);
        free(tmp as *mut libc::c_void);
    }
    (*jobqueue_p).front = 0 as *mut libc::c_void as *mut job;
    (*jobqueue_p).rear = 0 as *mut libc::c_void as *mut job;
    bsem_reset((*jobqueue_p).has_jobs);
    (*jobqueue_p).len = 0 as libc::c_int;
}
unsafe extern "C" fn jobqueue_push(mut jobqueue_p: *mut jobqueue, mut newjob: *mut job) {
    pthread_mutex_lock(&mut (*jobqueue_p).rwmutex);
    (*newjob).prev = 0 as *mut libc::c_void as *mut job;
    match (*jobqueue_p).len {
        0 => {
            (*jobqueue_p).front = newjob;
            (*jobqueue_p).rear = newjob;
        }
        _ => {
            (*(*jobqueue_p).rear).prev = newjob;
            (*jobqueue_p).rear = newjob;
        }
    }
    (*jobqueue_p).len += 1;
    bsem_post((*jobqueue_p).has_jobs);
    pthread_mutex_unlock(&mut (*jobqueue_p).rwmutex);
}
unsafe extern "C" fn jobqueue_pull(mut jobqueue_p: *mut jobqueue) -> *mut job {
    let mut job_p: *mut job = 0 as *mut job;
    pthread_mutex_lock(&mut (*jobqueue_p).rwmutex);
    job_p = (*jobqueue_p).front;
    match (*jobqueue_p).len {
        0 => {}
        1 => {
            (*jobqueue_p).front = 0 as *mut libc::c_void as *mut job;
            (*jobqueue_p).rear = 0 as *mut libc::c_void as *mut job;
            (*jobqueue_p).len = 0 as libc::c_int;
        }
        _ => {
            (*jobqueue_p).front = (*job_p).prev;
            (*jobqueue_p).len -= 1;
            bsem_post((*jobqueue_p).has_jobs);
        }
    }
    pthread_mutex_unlock(&mut (*jobqueue_p).rwmutex);
    return job_p;
}
unsafe extern "C" fn jobqueue_destroy(mut jobqueue_p: *mut jobqueue) {
    jobqueue_clear(jobqueue_p);
    free((*jobqueue_p).has_jobs as *mut libc::c_void);
}
unsafe extern "C" fn bsem_init(mut bsem_p: *mut bsem, mut value: libc::c_int) {
    if value < 0 as libc::c_int {
        fprintf(
            stderr,
            b"bsem_init(): Binary semaphore can take only values 1 or 0\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        if value > 1 as libc::c_int {
            fprintf(
                stderr,
                b"bsem_init(): Binary semaphore can take only values 1 or 0\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    pthread_mutex_init(
        &mut (*bsem_p).mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    pthread_cond_init(
        &mut (*bsem_p).cond as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    (*bsem_p).v = value;
}
unsafe extern "C" fn bsem_reset(mut bsem_p: *mut bsem) {
    bsem_init(bsem_p, 0 as libc::c_int);
}
unsafe extern "C" fn bsem_post(mut bsem_p: *mut bsem) {
    pthread_mutex_lock(&mut (*bsem_p).mutex);
    (*bsem_p).v = 1 as libc::c_int;
    pthread_cond_signal(&mut (*bsem_p).cond);
    pthread_mutex_unlock(&mut (*bsem_p).mutex);
}
unsafe extern "C" fn bsem_post_all(mut bsem_p: *mut bsem) {
    pthread_mutex_lock(&mut (*bsem_p).mutex);
    (*bsem_p).v = 1 as libc::c_int;
    pthread_cond_broadcast(&mut (*bsem_p).cond);
    pthread_mutex_unlock(&mut (*bsem_p).mutex);
}
unsafe extern "C" fn bsem_wait(mut bsem_p: *mut bsem) {
    pthread_mutex_lock(&mut (*bsem_p).mutex);
    while (*bsem_p).v != 1 as libc::c_int {
        pthread_cond_wait(
            &mut (*bsem_p).cond as *mut pthread_cond_t,
            &mut (*bsem_p).mutex as *mut pthread_mutex_t,
        );
    }
    (*bsem_p).v = 0 as libc::c_int;
    pthread_mutex_unlock(&mut (*bsem_p).mutex);
}
pub static mut mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub static mut sum: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn increment() {
    pthread_mutex_lock(&mut mutex);
    sum += 1;
    pthread_mutex_unlock(&mut mutex);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_jobs: libc::c_int = 0;
    let mut tmp: libc::c_long = 0;
    let mut num_threads: libc::c_int = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut thpool: threadpool = 0 as *mut thpool_;
    let mut tmp___1: threadpool = 0 as *mut thpool_;
    let mut n: libc::c_int = 0;
    if argc != 3 as libc::c_int {
        puts(
            b"This testfile needs excactly two arguments\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    tmp = strtol(
        *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
        &mut p as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    num_jobs = tmp as libc::c_int;
    tmp___0 = strtol(
        *argv.offset(2 as libc::c_int as isize) as *const libc::c_char,
        &mut p as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    num_threads = tmp___0 as libc::c_int;
    tmp___1 = thpool_init(num_threads);
    thpool = tmp___1;
    n = 0 as libc::c_int;
    while n < num_jobs {
        thpool_add_work(
            thpool,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(
                ::std::mem::transmute::<
                    Option::<unsafe extern "C" fn() -> ()>,
                    *mut libc::c_void,
                >(Some(increment as unsafe extern "C" fn() -> ())),
            ),
            0 as *mut libc::c_void,
        );
        n += 1;
    }
    thpool_wait(thpool);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, sum);
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
