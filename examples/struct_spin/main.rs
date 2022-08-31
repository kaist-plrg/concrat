use ::libc;
extern "C" {
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
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> libc::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: libc::c_int,
    ) -> libc::c_int;
    fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
    fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> libc::c_int;
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
pub struct __pthread_rwlock_arch_t {
    pub __readers: libc::c_uint,
    pub __writers: libc::c_uint,
    pub __wrphase_futex: libc::c_uint,
    pub __writers_futex: libc::c_uint,
    pub __pad3: libc::c_uint,
    pub __pad4: libc::c_uint,
    pub __cur_writer: libc::c_int,
    pub __shared: libc::c_int,
    pub __rwelision: libc::c_schar,
    pub __pad1: [libc::c_uchar; 7],
    pub __pad2: libc::c_ulong,
    pub __flags: libc::c_uint,
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
pub union __anonunion_pthread_rwlock_t_656928968 {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type pthread_rwlock_t = __anonunion_pthread_rwlock_t_656928968;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_rwlockattr_t_145707745 {
    pub __size: [libc::c_char; 8],
    pub __align: libc::c_long,
}
pub type pthread_rwlockattr_t = __anonunion_pthread_rwlockattr_t_145707745;
pub type pthread_spinlock_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ss_89128768 {
    pub n1: libc::c_int,
    pub m1: pthread_mutex_t,
    pub n2: libc::c_int,
    pub m2: pthread_rwlock_t,
    pub n3: libc::c_int,
    pub m3: pthread_spinlock_t,
}
pub type ss = __anonstruct_ss_89128768;
pub unsafe extern "C" fn f1(mut s: *mut ss) {
    pthread_mutex_lock(&mut (*s).m1);
    (*s).n1 += 1;
    pthread_mutex_unlock(&mut (*s).m1);
    pthread_rwlock_wrlock(&mut (*s).m2);
    (*s).n2 += 1;
    pthread_rwlock_unlock(&mut (*s).m2);
    pthread_spin_lock(&mut (*s).m3);
    (*s).n3 += 1;
    pthread_spin_unlock(&mut (*s).m3);
}
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1(arg as *mut ss);
    return 0 as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: ss = ss {
        n1: 0,
        m1: __anonunion_pthread_mutex_t_335460617 {
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
        n2: 0,
        m2: __anonunion_pthread_rwlock_t_656928968 {
            __data: __pthread_rwlock_arch_t {
                __readers: 0,
                __writers: 0,
                __wrphase_futex: 0,
                __writers_futex: 0,
                __pad3: 0,
                __pad4: 0,
                __cur_writer: 0,
                __shared: 0,
                __rwelision: 0,
                __pad1: [0; 7],
                __pad2: 0,
                __flags: 0,
            },
        },
        n3: 0,
        m3: 0,
    };
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    s.n1 = 1 as libc::c_int;
    pthread_mutex_init(&mut s.m1, 0 as *mut libc::c_void as *const pthread_mutexattr_t);
    s.n2 = 2 as libc::c_int;
    pthread_rwlock_init(
        &mut s.m2 as *mut pthread_rwlock_t,
        0 as *mut libc::c_void as *const pthread_rwlockattr_t,
    );
    s.n3 = 3 as libc::c_int;
    pthread_spin_init(&mut s.m3, 0 as *mut libc::c_void as libc::c_int);
    pthread_create(
        &mut id1 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        &mut s as *mut ss as *mut libc::c_void,
    );
    pthread_create(
        &mut id2 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        &mut s as *mut ss as *mut libc::c_void,
    );
    pthread_join(id1, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    pthread_join(id2, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
