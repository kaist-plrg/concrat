use ::libc;
extern "C" {
    #[no_mangle]
    fn pthread_create(__newthread: *mut pthread_t,
                      __attr: *const pthread_attr_t,
                      __start_routine:
                          Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                     -> *mut libc::c_void>,
                      __arg: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void)
     -> libc::c_int;
    #[no_mangle]
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    #[no_mangle]
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
    pub __spins: libc::c_int,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 64],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_long,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
#[no_mangle]
pub static mut n1: [libc::c_int; 3] = [0; 3];
#[no_mangle]
pub static mut num_mutex: [pthread_mutex_t; 3] =
    [pthread_mutex_t{__data:
                         {
                             let mut init =
                                 __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                   __count:
                                                       0 as libc::c_int as
                                                           libc::c_uint,
                                                   __owner: 0 as libc::c_int,
                                                   __nusers:
                                                       0 as libc::c_int as
                                                           libc::c_uint,
                                                   __kind:
                                                       PTHREAD_MUTEX_TIMED_NP
                                                           as libc::c_int,
                                                   __spins: 0 as libc::c_int,
                                                   __list:
                                                       {
                                                           let mut init =
                                                               __pthread_internal_list{__prev:
                                                                                           0
                                                                                               as
                                                                                               *const __pthread_internal_list
                                                                                               as
                                                                                               *mut __pthread_internal_list,
                                                                                       __next:
                                                                                           0
                                                                                               as
                                                                                               *const __pthread_internal_list
                                                                                               as
                                                                                               *mut __pthread_internal_list,};
                                                           init
                                                       },};
                             init
                         },},
     pthread_mutex_t{__data:
                         {
                             let mut init =
                                 __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                   __count:
                                                       0 as libc::c_int as
                                                           libc::c_uint,
                                                   __owner: 0 as libc::c_int,
                                                   __nusers:
                                                       0 as libc::c_int as
                                                           libc::c_uint,
                                                   __kind:
                                                       PTHREAD_MUTEX_TIMED_NP
                                                           as libc::c_int,
                                                   __spins: 0 as libc::c_int,
                                                   __list:
                                                       {
                                                           let mut init =
                                                               __pthread_internal_list{__prev:
                                                                                           0
                                                                                               as
                                                                                               *const __pthread_internal_list
                                                                                               as
                                                                                               *mut __pthread_internal_list,
                                                                                       __next:
                                                                                           0
                                                                                               as
                                                                                               *const __pthread_internal_list
                                                                                               as
                                                                                               *mut __pthread_internal_list,};
                                                           init
                                                       },};
                             init
                         },},
     pthread_mutex_t{__data:
                         {
                             let mut init =
                                 __pthread_mutex_s{__lock: 0 as libc::c_int,
                                                   __count:
                                                       0 as libc::c_int as
                                                           libc::c_uint,
                                                   __owner: 0 as libc::c_int,
                                                   __nusers:
                                                       0 as libc::c_int as
                                                           libc::c_uint,
                                                   __kind:
                                                       PTHREAD_MUTEX_TIMED_NP
                                                           as libc::c_int,
                                                   __spins: 0 as libc::c_int,
                                                   __list:
                                                       {
                                                           let mut init =
                                                               __pthread_internal_list{__prev:
                                                                                           0
                                                                                               as
                                                                                               *const __pthread_internal_list
                                                                                               as
                                                                                               *mut __pthread_internal_list,
                                                                                       __next:
                                                                                           0
                                                                                               as
                                                                                               *const __pthread_internal_list
                                                                                               as
                                                                                               *mut __pthread_internal_list,};
                                                           init
                                                       },};
                             init
                         },}];
#[no_mangle]
pub unsafe extern "C" fn f1() {
    pthread_mutex_lock(&mut *num_mutex.as_mut_ptr().offset(0 as libc::c_int as
                                                               isize));
    n1[0 as libc::c_int as usize] =
        n1[0 as libc::c_int as usize] + 1 as libc::c_int;
    pthread_mutex_unlock(&mut *num_mutex.as_mut_ptr().offset(0 as libc::c_int
                                                                 as isize));
    pthread_mutex_lock(&mut *num_mutex.as_mut_ptr().offset(1 as libc::c_int as
                                                               isize));
    n1[1 as libc::c_int as usize] =
        n1[1 as libc::c_int as usize] + 1 as libc::c_int;
    pthread_mutex_unlock(&mut *num_mutex.as_mut_ptr().offset(1 as libc::c_int
                                                                 as isize));
    pthread_mutex_lock(&mut *num_mutex.as_mut_ptr().offset(2 as libc::c_int as
                                                               isize));
    n1[2 as libc::c_int as usize] =
        n1[2 as libc::c_int as usize] + 1 as libc::c_int;
    pthread_mutex_unlock(&mut *num_mutex.as_mut_ptr().offset(2 as libc::c_int
                                                                 as isize));
}
#[no_mangle]
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void)
 -> *mut libc::c_void {
    f1();
    return 0 as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    let mut id1: pthread_t = 0;
    let mut id2: pthread_t = 0;
    pthread_create(&mut id1, 0 as *const pthread_attr_t,
                   Some(t_fun as
                            unsafe extern "C" fn(_: *mut libc::c_void)
                                -> *mut libc::c_void),
                   0 as *mut libc::c_void);
    pthread_create(&mut id2, 0 as *const pthread_attr_t,
                   Some(t_fun as
                            unsafe extern "C" fn(_: *mut libc::c_void)
                                -> *mut libc::c_void),
                   0 as *mut libc::c_void);
    pthread_join(id1, 0 as *mut *mut libc::c_void);
    pthread_join(id2, 0 as *mut *mut libc::c_void);
    return 0;
}
#[main]
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
