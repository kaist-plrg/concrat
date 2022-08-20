use ::libc;
extern "C" {
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> libc::c_int;
    fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: libc::c_int,
    ) -> libc::c_int;
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
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sc_mutex {
    pub mtx: pthread_mutex_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutexattr_t_488594144 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_mutexattr_t = __anonunion_pthread_mutexattr_t_488594144;
unsafe fn main_0() -> libc::c_int {
    let mut mutex: sc_mutex = sc_mutex {
        mtx: __anonunion_pthread_mutex_t_335460617 {
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
    };
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    tmp___2 = sc_mutex_init(&mut mutex);
    if !(tmp___2 == 0 as libc::c_int) {
        __assert_fail(
            b"sc_mutex_init(&mutex) == 0\0" as *const u8 as *const libc::c_char,
            b"mutex_test.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_uint,
            b"main\0" as *const u8 as *const libc::c_char,
        );
    }
    sc_mutex_lock(&mut mutex);
    sc_mutex_unlock(&mut mutex);
    tmp___6 = sc_mutex_term(&mut mutex);
    if !(tmp___6 == 0 as libc::c_int) {
        __assert_fail(
            b"sc_mutex_term(&mutex) == 0\0" as *const u8 as *const libc::c_char,
            b"mutex_test.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_uint,
            b"main\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sc_mutex_init(mut mtx: *mut sc_mutex) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut attr: pthread_mutexattr_t = __anonunion_pthread_mutexattr_t_488594144 {
        __size: [0; 4],
    };
    let mut mut_0: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
    let mut tmp___1: libc::c_int = 0;
    mut_0.__data.__lock = 0 as libc::c_int;
    mut_0.__data.__count = 0 as libc::c_uint;
    mut_0.__data.__owner = 0 as libc::c_int;
    mut_0.__data.__nusers = 0 as libc::c_uint;
    mut_0.__data.__kind = 0 as libc::c_int;
    mut_0.__data.__spins = 0 as libc::c_int as libc::c_short;
    mut_0.__data.__elision = 0 as libc::c_int as libc::c_short;
    mut_0.__data.__list.__prev = 0 as *mut __pthread_internal_list;
    mut_0.__data.__list.__next = 0 as *mut __pthread_internal_list;
    (*mtx).mtx = mut_0;
    rc = pthread_mutexattr_init(&mut attr);
    if rc != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    rc = pthread_mutexattr_settype(&mut attr, 0 as libc::c_int);
    if !(rc == 0 as libc::c_int) {
        __assert_fail(
            b"rc == 0\0" as *const u8 as *const libc::c_char,
            b"sc_mutex.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_uint,
            b"sc_mutex_init\0" as *const u8 as *const libc::c_char,
        );
    }
    rc = pthread_mutex_init(
        &mut (*mtx).mtx,
        &mut attr as *mut pthread_mutexattr_t as *const pthread_mutexattr_t,
    );
    rv = pthread_mutexattr_destroy(&mut attr);
    if !(rv == 0 as libc::c_int) {
        __assert_fail(
            b"rv == 0\0" as *const u8 as *const libc::c_char,
            b"sc_mutex.c\0" as *const u8 as *const libc::c_char,
            88 as libc::c_uint,
            b"sc_mutex_init\0" as *const u8 as *const libc::c_char,
        );
    }
    if rc != 0 as libc::c_int {
        tmp___1 = -(1 as libc::c_int);
    } else {
        tmp___1 = 0 as libc::c_int;
    }
    return tmp___1;
}
pub unsafe extern "C" fn sc_mutex_term(mut mtx: *mut sc_mutex) -> libc::c_int {
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    rc = pthread_mutex_destroy(&mut (*mtx).mtx);
    if rc != 0 as libc::c_int {
        tmp = -(1 as libc::c_int);
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp;
}
pub unsafe extern "C" fn sc_mutex_lock(mut mtx: *mut sc_mutex) {
    let mut rc: libc::c_int = 0;
    rc = pthread_mutex_lock(&mut (*mtx).mtx);
    if !(rc == 0 as libc::c_int) {
        __assert_fail(
            b"rc == 0\0" as *const u8 as *const libc::c_char,
            b"sc_mutex.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_uint,
            b"sc_mutex_lock\0" as *const u8 as *const libc::c_char,
        );
    }
}
pub unsafe extern "C" fn sc_mutex_unlock(mut mtx: *mut sc_mutex) {
    let mut rc: libc::c_int = 0;
    rc = pthread_mutex_unlock(&mut (*mtx).mtx);
    if !(rc == 0 as libc::c_int) {
        __assert_fail(
            b"rc == 0\0" as *const u8 as *const libc::c_char,
            b"sc_mutex.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_uint,
            b"sc_mutex_unlock\0" as *const u8 as *const libc::c_char,
        );
    }
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
