use ::libc;
extern "C" {
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
pub unsafe extern "C" fn rdlock() {
    pthread_rwlock_rdlock(&mut lock);
}
pub unsafe extern "C" fn wrlock() {
    pthread_rwlock_wrlock(&mut lock);
}
pub unsafe extern "C" fn unlock() {
    pthread_rwlock_unlock(&mut lock);
}
pub unsafe extern "C" fn f1() {
    let mut x: libc::c_int = 0;
    pthread_rwlock_rdlock(&mut lock);
    x = n;
    pthread_rwlock_unlock(&mut lock);
    x += 1;
    pthread_rwlock_wrlock(&mut lock);
    n += x;
    pthread_rwlock_unlock(&mut lock);
    rdlock();
    x = n;
    unlock();
    x += 1;
    wrlock();
    n += x;
    unlock();
}
unsafe fn main_0() -> libc::c_int {
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
