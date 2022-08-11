use ::libc;
extern "C" {
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ss_672045599 {
    pub n: libc::c_int,
    pub m: pthread_mutex_t,
}
pub type ss = __anonstruct_ss_672045599;
pub unsafe extern "C" fn f1() {
    let mut s: *mut ss = 0 as *mut ss;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<ss>() as libc::c_ulong);
    s = tmp as *mut ss;
    (*s).n = 0 as libc::c_int;
    pthread_mutex_init(
        &mut (*s).m,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
}
pub unsafe extern "C" fn f2(mut s: *mut ss) {
    (*s).n = 0 as libc::c_int;
    pthread_mutex_destroy(&mut (*s).m);
    free(s as *mut libc::c_void);
}
pub unsafe extern "C" fn f3(mut s: *mut ss) {
    pthread_mutex_lock(&mut (*s).m);
    (*s).n += 1;
    pthread_mutex_unlock(&mut (*s).m);
}
unsafe fn main_0() -> libc::c_int {
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
