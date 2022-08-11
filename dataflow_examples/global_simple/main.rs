use ::libc;
extern "C" {
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
pub static mut n: libc::c_int = 0 as libc::c_int;
pub static mut x: libc::c_int = 0 as libc::c_int;
pub static mut a: libc::c_int = 1 as libc::c_int;
pub static mut m: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
pub unsafe extern "C" fn f1() {
    pthread_mutex_lock(&mut m);
    n += a;
    x += 1;
    pthread_mutex_unlock(&mut m);
    f2();
}
pub unsafe extern "C" fn f2() {
    pthread_mutex_lock(&mut m);
    n += a;
    pthread_mutex_unlock(&mut m);
    x += 1;
    f3();
}
pub unsafe extern "C" fn f3() {
    pthread_mutex_lock(&mut m);
    n += a;
    pthread_mutex_unlock(&mut m);
}
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
    return 0 as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    let mut id: pthread_t = 0;
    pthread_create(
        &mut id as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(t_fun as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    pthread_join(id, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
