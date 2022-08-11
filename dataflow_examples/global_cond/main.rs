use ::libc;
extern "C" {
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
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
pub static mut c: pthread_cond_t = __anonunion_pthread_cond_t_951761805 {
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
};
pub unsafe extern "C" fn f1() {
    pthread_mutex_lock(&mut m);
    pthread_cond_wait(
        &mut c as *mut pthread_cond_t,
        &mut m as *mut pthread_mutex_t,
    );
    pthread_mutex_unlock(&mut m);
}
pub unsafe extern "C" fn f2() {
    pthread_mutex_lock(&mut m);
    pthread_cond_wait(
        &mut c as *mut pthread_cond_t,
        &mut m as *mut pthread_mutex_t,
    );
}
pub unsafe extern "C" fn f3() {
    pthread_cond_wait(
        &mut c as *mut pthread_cond_t,
        &mut m as *mut pthread_mutex_t,
    );
    pthread_mutex_unlock(&mut m);
}
pub unsafe extern "C" fn f4() {
    pthread_cond_wait(
        &mut c as *mut pthread_cond_t,
        &mut m as *mut pthread_mutex_t,
    );
}
unsafe fn main_0() -> libc::c_int {
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
