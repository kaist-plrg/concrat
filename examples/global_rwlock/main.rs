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
    fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
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
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_rwlock_t_656928968 {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type pthread_rwlock_t = __anonunion_pthread_rwlock_t_656928968;
pub static mut n: libc::c_int = 0 as libc::c_int;
pub static mut lock: pthread_rwlock_t = __anonunion_pthread_rwlock_t_656928968 {
    __data: {
        let mut init = __pthread_rwlock_arch_t {
            __readers: 0 as libc::c_uint,
            __writers: 0 as libc::c_uint,
            __wrphase_futex: 0 as libc::c_uint,
            __writers_futex: 0 as libc::c_uint,
            __pad3: 0 as libc::c_uint,
            __pad4: 0 as libc::c_uint,
            __cur_writer: 0 as libc::c_int,
            __shared: 0 as libc::c_int,
            __rwelision: 0 as libc::c_int as libc::c_schar,
            __pad1: [
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
            ],
            __pad2: 0 as libc::c_ulong,
            __flags: 0 as libc::c_uint,
        };
        init
    },
};
pub unsafe extern "C" fn f1() -> libc::c_int {
    let mut x: libc::c_int = 0;
    pthread_rwlock_rdlock(&mut lock);
    x = n;
    pthread_rwlock_unlock(&mut lock);
    return x;
}
pub unsafe extern "C" fn f2() {
    pthread_rwlock_wrlock(&mut lock);
    n += 1;
    pthread_rwlock_unlock(&mut lock);
}
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    f2();
    return 0 as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(
        &mut id1 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pthread_create(
        &mut id2 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pthread_join(id1, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    pthread_join(id2, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
