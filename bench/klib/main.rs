use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __int64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub union __anonunion____missing_field_name_127042150 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_571951528 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_571951527 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_571951528,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_127042150,
    pub __annonCompField2: __anonunion____missing_field_name_571951527,
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
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kt_for_t {
    pub n_threads: libc::c_int,
    pub n: libc::c_long,
    pub w: *mut ktf_worker_t,
    pub func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_long, libc::c_int) -> (),
    >,
    pub data: *mut libc::c_void,
}
pub type ktf_worker_t = __anonstruct_ktf_worker_t_495033186;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ktf_worker_t_495033186 {
    pub t: *mut kt_for_t,
    pub i: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kt_forpool_t {
    pub n_threads: libc::c_int,
    pub n_pending: libc::c_int,
    pub n: libc::c_long,
    pub tid: *mut pthread_t,
    pub w: *mut kto_worker_t,
    pub func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_long, libc::c_int) -> (),
    >,
    pub data: *mut libc::c_void,
    pub mutex: pthread_mutex_t,
    pub cv_m: pthread_cond_t,
    pub cv_s: pthread_cond_t,
}
pub type kto_worker_t = __anonstruct_kto_worker_t_407073300;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_kto_worker_t_407073300 {
    pub t: *mut kt_forpool_t,
    pub i: libc::c_long,
    pub action: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ktp_t {
    pub shared: *mut libc::c_void,
    pub func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub index: int64_t,
    pub n_workers: libc::c_int,
    pub n_steps: libc::c_int,
    pub workers: *mut ktp_worker_t,
    pub mutex: pthread_mutex_t,
    pub cv: pthread_cond_t,
}
pub type ktp_worker_t = __anonstruct_ktp_worker_t_323515798;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ktp_worker_t_323515798 {
    pub pl: *mut ktp_t,
    pub index: int64_t,
    pub step: libc::c_int,
    pub data: *mut libc::c_void,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_pipeline_t_503116822 {
    pub fp: *mut FILE,
    pub max_lines: libc::c_int,
    pub buf_size: libc::c_int,
    pub n_threads: libc::c_int,
    pub buf: *mut libc::c_char,
}
pub type pipeline_t = __anonstruct_pipeline_t_503116822;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_step_t_216488822 {
    pub n_lines: libc::c_int,
    pub lines: *mut *mut libc::c_char,
}
pub type step_t = __anonstruct_step_t_216488822;
#[inline]
unsafe extern "C" fn steal_work(mut t: *mut kt_for_t) -> libc::c_long {
    let mut i: libc::c_int = 0;
    let mut min_i: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut min: libc::c_long = 0;
    let mut tmp: libc::c_long = 0;
    min_i = -(1 as libc::c_int);
    min = 9223372036854775807 as libc::c_long;
    i = 0 as libc::c_int;
    while i < (*t).n_threads {
        if min > (*((*t).w).offset(i as isize)).i {
            min = (*((*t).w).offset(i as isize)).i;
            min_i = i;
        }
        i += 1;
    }
    k = ::std::intrinsics::atomic_xadd_seqcst(
        &mut (*((*t).w).offset(min_i as isize)).i as *mut libc::c_long,
        (*t).n_threads as libc::c_long,
    );
    if k >= (*t).n {
        tmp = -(1 as libc::c_long);
    } else {
        tmp = k;
    }
    return tmp;
}
unsafe extern "C" fn ktf_worker(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut w: *mut ktf_worker_t = 0 as *mut ktf_worker_t;
    let mut i: libc::c_long = 0;
    w = data as *mut ktf_worker_t;
    loop {
        i = ::std::intrinsics::atomic_xadd_seqcst(
            &mut (*w).i as *mut libc::c_long,
            (*(*w).t).n_threads as libc::c_long,
        );
        if i >= (*(*w).t).n {
            break;
        }
        (Some(((*(*w).t).func).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*(*w).t).data,
            i,
            w.offset_from((*(*w).t).w) as libc::c_long as libc::c_int,
        );
    }
    loop {
        i = steal_work((*w).t);
        if !(i >= 0 as libc::c_long) {
            break;
        }
        (Some(((*(*w).t).func).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*(*w).t).data,
            i,
            w.offset_from((*(*w).t).w) as libc::c_long as libc::c_int,
        );
    }
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn kt_for(
    mut n_threads: libc::c_int,
    mut func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_long, libc::c_int) -> (),
    >,
    mut data: *mut libc::c_void,
    mut n: libc::c_long,
) {
    let mut i: libc::c_int = 0;
    let mut t: kt_for_t = kt_for_t {
        n_threads: 0,
        n: 0,
        w: 0 as *mut ktf_worker_t,
        func: None,
        data: 0 as *mut libc::c_void,
    };
    let mut tid: *mut pthread_t = 0 as *mut pthread_t;
    let mut tmp: *mut _ = 0 as *mut _;
    let mut tmp___0: *mut _ = 0 as *mut _;
    let mut j: libc::c_long = 0;
    if n_threads > 1 as libc::c_int {
        t.func = func;
        t.data = data;
        t.n_threads = n_threads;
        t.n = n;
        let mut fresh0 = ::std::vec::from_elem(
            0,
            (n_threads as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<ktf_worker_t>() as libc::c_ulong)
                as usize,
        );
        tmp = fresh0.as_mut_ptr();
        t.w = tmp as *mut ktf_worker_t;
        let mut fresh1 = ::std::vec::from_elem(
            0,
            (n_threads as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<pthread_t>() as libc::c_ulong)
                as usize,
        );
        tmp___0 = fresh1.as_mut_ptr();
        tid = tmp___0 as *mut pthread_t;
        i = 0 as libc::c_int;
        while i < n_threads {
            let ref mut fresh2 = (*(t.w).offset(i as isize)).t;
            *fresh2 = &mut t;
            (*(t.w).offset(i as isize)).i = i as libc::c_long;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < n_threads {
            pthread_create(
                tid.offset(i as isize),
                0 as *const pthread_attr_t,
                Some(
                    ktf_worker
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                (t.w).offset(i as isize) as *mut libc::c_void,
            );
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < n_threads {
            pthread_join(*tid.offset(i as isize), 0 as *mut *mut libc::c_void);
            i += 1;
        }
    } else {
        j = 0 as libc::c_long;
        while j < n {
            (Some(func.expect("non-null function pointer")))
                .expect("non-null function pointer")(data, j, 0 as libc::c_int);
            j += 1;
        }
    };
}
#[inline]
unsafe extern "C" fn kt_fp_steal_work(mut t: *mut kt_forpool_t) -> libc::c_long {
    let mut i: libc::c_int = 0;
    let mut min_i: libc::c_int = 0;
    let mut k: libc::c_long = 0;
    let mut min: libc::c_long = 0;
    let mut tmp: libc::c_long = 0;
    min_i = -(1 as libc::c_int);
    min = 9223372036854775807 as libc::c_long;
    i = 0 as libc::c_int;
    while i < (*t).n_threads {
        if min > (*((*t).w).offset(i as isize)).i {
            min = (*((*t).w).offset(i as isize)).i;
            min_i = i;
        }
        i += 1;
    }
    k = ::std::intrinsics::atomic_xadd_seqcst(
        &mut (*((*t).w).offset(min_i as isize)).i as *mut libc::c_long,
        (*t).n_threads as libc::c_long,
    );
    if k >= (*t).n {
        tmp = -(1 as libc::c_long);
    } else {
        tmp = k;
    }
    return tmp;
}
unsafe extern "C" fn kt_fp_worker(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut w: *mut kto_worker_t = 0 as *mut kto_worker_t;
    let mut fp: *mut kt_forpool_t = 0 as *mut kt_forpool_t;
    let mut i: libc::c_long = 0;
    let mut action: libc::c_int = 0;
    w = data as *mut kto_worker_t;
    fp = (*w).t;
    loop {
        pthread_mutex_lock(&mut (*fp).mutex);
        (*fp).n_pending -= 1;
        if (*fp).n_pending == 0 as libc::c_int {
            pthread_cond_signal(&mut (*fp).cv_m);
        }
        (*w).action = 0 as libc::c_int;
        while (*w).action == 0 as libc::c_int {
            pthread_cond_wait(
                &mut (*fp).cv_s as *mut pthread_cond_t,
                &mut (*fp).mutex as *mut pthread_mutex_t,
            );
        }
        action = (*w).action;
        pthread_mutex_unlock(&mut (*fp).mutex);
        if action < 0 as libc::c_int {
            break;
        }
        loop {
            i = ::std::intrinsics::atomic_xadd_seqcst(
                &mut (*w).i as *mut libc::c_long,
                (*fp).n_threads as libc::c_long,
            );
            if i >= (*fp).n {
                break;
            }
            (Some(((*fp).func).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*fp).data, i, w.offset_from((*fp).w) as libc::c_long as libc::c_int);
        }
        loop {
            i = kt_fp_steal_work(fp);
            if !(i >= 0 as libc::c_long) {
                break;
            }
            (Some(((*fp).func).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*fp).data, i, w.offset_from((*fp).w) as libc::c_long as libc::c_int);
        }
    }
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn kt_forpool_init(
    mut n_threads: libc::c_int,
) -> *mut libc::c_void {
    let mut fp: *mut kt_forpool_t = 0 as *mut kt_forpool_t;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kt_forpool_t>() as libc::c_ulong,
    );
    fp = tmp as *mut kt_forpool_t;
    tmp___0 = n_threads;
    (*fp).n_pending = tmp___0;
    (*fp).n_threads = tmp___0;
    tmp___1 = calloc(
        (*fp).n_threads as size_t,
        ::std::mem::size_of::<pthread_t>() as libc::c_ulong,
    );
    (*fp).tid = tmp___1 as *mut pthread_t;
    tmp___2 = calloc(
        (*fp).n_threads as size_t,
        ::std::mem::size_of::<kto_worker_t>() as libc::c_ulong,
    );
    (*fp).w = tmp___2 as *mut kto_worker_t;
    i = 0 as libc::c_int;
    while i < (*fp).n_threads {
        let ref mut fresh3 = (*((*fp).w).offset(i as isize)).t;
        *fresh3 = fp;
        i += 1;
    }
    pthread_mutex_init(&mut (*fp).mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(
        &mut (*fp).cv_m as *mut pthread_cond_t,
        0 as *const pthread_condattr_t,
    );
    pthread_cond_init(
        &mut (*fp).cv_s as *mut pthread_cond_t,
        0 as *const pthread_condattr_t,
    );
    i = 0 as libc::c_int;
    while i < (*fp).n_threads {
        pthread_create(
            ((*fp).tid).offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                kt_fp_worker
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            ((*fp).w).offset(i as isize) as *mut libc::c_void,
        );
        i += 1;
    }
    pthread_mutex_lock(&mut (*fp).mutex);
    while (*fp).n_pending != 0 {
        pthread_cond_wait(
            &mut (*fp).cv_m as *mut pthread_cond_t,
            &mut (*fp).mutex as *mut pthread_mutex_t,
        );
    }
    pthread_mutex_unlock(&mut (*fp).mutex);
    return fp as *mut libc::c_void;
}
pub unsafe extern "C" fn kt_forpool_destroy(mut _fp: *mut libc::c_void) {
    let mut fp: *mut kt_forpool_t = 0 as *mut kt_forpool_t;
    let mut i: libc::c_int = 0;
    fp = _fp as *mut kt_forpool_t;
    i = 0 as libc::c_int;
    while i < (*fp).n_threads {
        (*((*fp).w).offset(i as isize)).action = -(1 as libc::c_int);
        i += 1;
    }
    pthread_cond_broadcast(&mut (*fp).cv_s);
    i = 0 as libc::c_int;
    while i < (*fp).n_threads {
        pthread_join(*((*fp).tid).offset(i as isize), 0 as *mut *mut libc::c_void);
        i += 1;
    }
    pthread_cond_destroy(&mut (*fp).cv_s);
    pthread_cond_destroy(&mut (*fp).cv_m);
    pthread_mutex_destroy(&mut (*fp).mutex);
    free((*fp).w as *mut libc::c_void);
    free((*fp).tid as *mut libc::c_void);
    free(fp as *mut libc::c_void);
}
pub unsafe extern "C" fn kt_forpool(
    mut _fp: *mut libc::c_void,
    mut func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, libc::c_long, libc::c_int) -> (),
    >,
    mut data: *mut libc::c_void,
    mut n: libc::c_long,
) {
    let mut fp: *mut kt_forpool_t = 0 as *mut kt_forpool_t;
    let mut i: libc::c_long = 0;
    fp = _fp as *mut kt_forpool_t;
    let mut current_block_22: u64;
    if !fp.is_null() {
        if (*fp).n_threads > 1 as libc::c_int {
            (*fp).n = n;
            (*fp).func = func;
            (*fp).data = data;
            (*fp).n_pending = (*fp).n_threads;
            i = 0 as libc::c_long;
            while i < (*fp).n_threads as libc::c_long {
                (*((*fp).w).offset(i as isize)).i = i;
                (*((*fp).w).offset(i as isize)).action = 1 as libc::c_int;
                i += 1;
            }
            pthread_mutex_lock(&mut (*fp).mutex);
            pthread_cond_broadcast(&mut (*fp).cv_s);
            while (*fp).n_pending != 0 {
                pthread_cond_wait(
                    &mut (*fp).cv_m as *mut pthread_cond_t,
                    &mut (*fp).mutex as *mut pthread_mutex_t,
                );
            }
            pthread_mutex_unlock(&mut (*fp).mutex);
            current_block_22 = 5601891728916014340;
        } else {
            current_block_22 = 17266348182960711543;
        }
    } else {
        current_block_22 = 17266348182960711543;
    }
    match current_block_22 {
        17266348182960711543 => {
            i = 0 as libc::c_long;
            while i < n {
                (Some(func.expect("non-null function pointer")))
                    .expect("non-null function pointer")(data, i, 0 as libc::c_int);
                i += 1;
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn ktp_worker(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut w: *mut ktp_worker_t = 0 as *mut ktp_worker_t;
    let mut p: *mut ktp_t = 0 as *mut ktp_t;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: int64_t = 0;
    w = data as *mut ktp_worker_t;
    p = (*w).pl;
    while (*w).step < (*p).n_steps {
        pthread_mutex_lock(&mut (*p).mutex);
        loop {
            i = 0 as libc::c_int;
            while i < (*p).n_workers {
                if !(w as libc::c_ulong
                    == ((*p).workers).offset(i as isize) as libc::c_ulong)
                {
                    if (*((*p).workers).offset(i as isize)).step <= (*w).step {
                        if (*((*p).workers).offset(i as isize)).index < (*w).index {
                            break;
                        }
                    }
                }
                i += 1;
            }
            if i == (*p).n_workers {
                break;
            }
            pthread_cond_wait(
                &mut (*p).cv as *mut pthread_cond_t,
                &mut (*p).mutex as *mut pthread_mutex_t,
            );
        }
        pthread_mutex_unlock(&mut (*p).mutex);
        if (*w).step != 0 {
            tmp = (*w).data;
        } else {
            tmp = 0 as *mut libc::c_void;
        }
        (*w)
            .data = (Some(((*p).func).expect("non-null function pointer")))
            .expect("non-null function pointer")((*p).shared, (*w).step, tmp);
        pthread_mutex_lock(&mut (*p).mutex);
        if (*w).step == (*p).n_steps - 1 as libc::c_int {
            (*w).step = ((*w).step + 1 as libc::c_int) % (*p).n_steps;
        } else if !((*w).data).is_null() {
            (*w).step = ((*w).step + 1 as libc::c_int) % (*p).n_steps;
        } else {
            (*w).step = (*p).n_steps;
        }
        if (*w).step == 0 as libc::c_int {
            tmp___0 = (*p).index;
            (*p).index += 1;
            (*w).index = tmp___0;
        }
        pthread_cond_broadcast(&mut (*p).cv);
        pthread_mutex_unlock(&mut (*p).mutex);
    }
    pthread_exit(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn kt_pipeline(
    mut n_threads: libc::c_int,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    mut shared_data: *mut libc::c_void,
    mut n_steps: libc::c_int,
) {
    let mut aux: ktp_t = ktp_t {
        shared: 0 as *mut libc::c_void,
        func: None,
        index: 0,
        n_workers: 0,
        n_steps: 0,
        workers: 0 as *mut ktp_worker_t,
        mutex: __anonunion_pthread_mutex_t_335460617 {
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
        cv: __anonunion_pthread_cond_t_951761805 {
            __data: __pthread_cond_s {
                __annonCompField1: __anonunion____missing_field_name_127042150 {
                    __wseq: 0,
                },
                __annonCompField2: __anonunion____missing_field_name_571951527 {
                    __g1_start: 0,
                },
                __g_refs: [0; 2],
                __g_size: [0; 2],
                __g1_orig_size: 0,
                __wrefs: 0,
                __g_signals: [0; 2],
            },
        },
    };
    let mut tid: *mut pthread_t = 0 as *mut pthread_t;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut _ = 0 as *mut _;
    let mut w: *mut ktp_worker_t = 0 as *mut ktp_worker_t;
    let mut tmp___0: int64_t = 0;
    let mut tmp___1: *mut _ = 0 as *mut _;
    if n_threads < 1 as libc::c_int {
        n_threads = 1 as libc::c_int;
    }
    aux.n_workers = n_threads;
    aux.n_steps = n_steps;
    aux.func = func;
    aux.shared = shared_data;
    aux.index = 0 as libc::c_int as int64_t;
    pthread_mutex_init(&mut aux.mutex, 0 as *const pthread_mutexattr_t);
    pthread_cond_init(
        &mut aux.cv as *mut pthread_cond_t,
        0 as *const pthread_condattr_t,
    );
    let mut fresh4 = ::std::vec::from_elem(
        0,
        (n_threads as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<ktp_worker_t>() as libc::c_ulong)
            as usize,
    );
    tmp = fresh4.as_mut_ptr();
    aux.workers = tmp as *mut ktp_worker_t;
    i = 0 as libc::c_int;
    while i < n_threads {
        w = (aux.workers).offset(i as isize);
        (*w).step = 0 as libc::c_int;
        (*w).pl = &mut aux;
        (*w).data = 0 as *mut libc::c_void;
        tmp___0 = aux.index;
        aux.index += 1;
        (*w).index = tmp___0;
        i += 1;
    }
    let mut fresh5 = ::std::vec::from_elem(
        0,
        (n_threads as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pthread_t>() as libc::c_ulong) as usize,
    );
    tmp___1 = fresh5.as_mut_ptr();
    tid = tmp___1 as *mut pthread_t;
    i = 0 as libc::c_int;
    while i < n_threads {
        pthread_create(
            tid.offset(i as isize),
            0 as *const pthread_attr_t,
            Some(
                ktp_worker
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            (aux.workers).offset(i as isize) as *mut libc::c_void,
        );
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n_threads {
        pthread_join(*tid.offset(i as isize), 0 as *mut *mut libc::c_void);
        i += 1;
    }
    pthread_mutex_destroy(&mut aux.mutex);
    pthread_cond_destroy(&mut aux.cv);
}
unsafe extern "C" fn worker_for(
    mut _data: *mut libc::c_void,
    mut i: libc::c_long,
    mut tid: libc::c_int,
) {
    let mut step: *mut step_t = 0 as *mut step_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: size_t = 0;
    step = _data as *mut step_t;
    s = *((*step).lines).offset(i as isize);
    tmp = strlen(s as *const libc::c_char);
    l = tmp.wrapping_sub(1 as libc::c_ulong) as libc::c_int;
    if !(*s.offset(l as isize) as libc::c_int == 10 as libc::c_int) {
        __assert_fail(
            b"s[l] == '\\n'\0" as *const u8 as *const libc::c_char,
            b"test/kthread_test2.c\0" as *const u8 as *const libc::c_char,
            26 as libc::c_uint,
            b"worker_for\0" as *const u8 as *const libc::c_char,
        );
    }
    j = 0 as libc::c_int;
    while j < l >> 1 as libc::c_int {
        t = *s.offset(j as isize) as libc::c_int;
        *s.offset(j as isize) = *s.offset((l - 1 as libc::c_int - j) as isize);
        *s.offset((l - 1 as libc::c_int - j) as isize) = t as libc::c_char;
        j += 1;
    }
}
unsafe extern "C" fn worker_pipeline(
    mut shared: *mut libc::c_void,
    mut step: libc::c_int,
    mut in_0: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut p: *mut pipeline_t = 0 as *mut pipeline_t;
    let mut s: *mut step_t = 0 as *mut step_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s___0: *mut step_t = 0 as *mut step_t;
    p = shared as *mut pipeline_t;
    if step == 0 as libc::c_int {
        tmp = calloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<step_t>() as libc::c_ulong,
        );
        s = tmp as *mut step_t;
        tmp___0 = calloc(
            (*p).max_lines as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        );
        (*s).lines = tmp___0 as *mut *mut libc::c_char;
        loop {
            tmp___1 = fgets((*p).buf, (*p).buf_size, (*p).fp);
            if !(tmp___1 as libc::c_ulong != 0 as *mut libc::c_char as libc::c_ulong) {
                break;
            }
            let ref mut fresh6 = *((*s).lines).offset((*s).n_lines as isize);
            *fresh6 = strdup((*p).buf as *const libc::c_char);
            (*s).n_lines += 1;
            if (*s).n_lines >= (*p).max_lines {
                break;
            }
        }
        if (*s).n_lines != 0 {
            return s as *mut libc::c_void;
        }
    } else if step == 1 as libc::c_int {
        kt_for(
            (*p).n_threads,
            Some(
                worker_for
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_long,
                        libc::c_int,
                    ) -> (),
            ),
            in_0,
            (*(in_0 as *mut step_t)).n_lines as libc::c_long,
        );
        return in_0;
    } else {
        if step == 2 as libc::c_int {
            s___0 = in_0 as *mut step_t;
            while (*s___0).n_lines > 0 as libc::c_int {
                (*s___0).n_lines -= 1;
                fputs(
                    *((*s___0).lines).offset((*s___0).n_lines as isize)
                        as *const libc::c_char,
                    stdout,
                );
                free(
                    *((*s___0).lines).offset((*s___0).n_lines as isize)
                        as *mut libc::c_void,
                );
            }
            free((*s___0).lines as *mut libc::c_void);
            free(s___0 as *mut libc::c_void);
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pl: pipeline_t = pipeline_t {
        fp: 0 as *mut FILE,
        max_lines: 0,
        buf_size: 0,
        n_threads: 0,
        buf: 0 as *mut libc::c_char,
    };
    let mut pl_threads: libc::c_int = 0;
    let mut tmp___0: *mut FILE = 0 as *mut FILE;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    if argc == 1 as libc::c_int {
        fprintf(
            stderr,
            b"Usage: reverse <in.txt> [pipeline_threads [for_threads]]\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    tmp___1 = strcmp(
        *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
        b"-\0" as *const u8 as *const libc::c_char,
    );
    if tmp___1 != 0 {
        tmp___0 = fopen(
            *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        pl.fp = tmp___0;
    } else {
        pl.fp = stdin;
    }
    if pl.fp as libc::c_ulong == 0 as *mut FILE as libc::c_ulong {
        fprintf(
            stderr,
            b"ERROR: failed to open the input file.\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if argc > 2 as libc::c_int {
        tmp___2 = atoi(*argv.offset(2 as libc::c_int as isize) as *const libc::c_char);
        pl_threads = tmp___2;
    } else {
        pl_threads = 3 as libc::c_int;
    }
    pl.max_lines = 4096 as libc::c_int;
    pl.buf_size = 65536 as libc::c_int;
    if argc > 3 as libc::c_int {
        tmp___3 = atoi(*argv.offset(3 as libc::c_int as isize) as *const libc::c_char);
        pl.n_threads = tmp___3;
    } else {
        pl.n_threads = 1 as libc::c_int;
    }
    tmp___4 = calloc(pl.buf_size as size_t, 1 as libc::c_int as size_t);
    pl.buf = tmp___4 as *mut libc::c_char;
    kt_pipeline(
        pl_threads,
        Some(
            worker_pipeline
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        &mut pl as *mut pipeline_t as *mut libc::c_void,
        3 as libc::c_int,
    );
    free(pl.buf as *mut libc::c_void);
    if pl.fp as libc::c_ulong != stdin as libc::c_ulong {
        fclose(pl.fp);
    }
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
