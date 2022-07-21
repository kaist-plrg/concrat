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
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t)
        -> libc::c_int;
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
pub union __anonunion____missing_field_name_456658959 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_554396209 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_554396208 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_554396209,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_456658959,
    pub __annonCompField2: __anonunion____missing_field_name_554396208,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
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
pub struct __anonstruct_s_108742580 {
    pub n1: libc::c_int,
    pub m1: pthread_mutex_t,
    pub cond: pthread_cond_t,
}
pub static mut s: __anonstruct_s_108742580 = {
    let mut init = __anonstruct_s_108742580 {
        n1: 0 as libc::c_int,
        m1: __anonunion_pthread_mutex_t_335460617 {
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
                    __annonCompField1: __anonunion____missing_field_name_456658959 {
                        __wseq: 0 as libc::c_ulonglong,
                    },
                    __annonCompField2: __anonunion____missing_field_name_554396208 {
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
    };
    init
};
pub unsafe extern "C" fn f1() {
    pthread_mutex_lock(&mut s.m1);
    s.n1 += 1;
    if s.n1 == 1 as libc::c_int {
        pthread_cond_wait(
            &mut s.cond as *mut pthread_cond_t,
            &mut s.m1 as *mut pthread_mutex_t,
        );
    } else {
        pthread_cond_signal(&mut s.cond);
    }
    pthread_mutex_unlock(&mut s.m1);
}
pub unsafe extern "C" fn t_fun(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    f1();
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
