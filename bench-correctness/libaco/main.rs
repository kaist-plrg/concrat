#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool, rustc_private, thread_local, untagged_unions)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn mprotect(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
    ) -> libc::c_int;
    fn acosw(from_co: *mut aco_t, to_co: *mut aco_t) -> *mut libc::c_void;
    fn aco_save_fpucw_mxcsr(p: *mut libc::c_void);
    fn aco_funcp_protector_asm();
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
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
pub struct __anonstruct_aco_save_stack_t_539581013 {
    pub ptr: *mut libc::c_void,
    pub sz: size_t,
    pub valid_sz: size_t,
    pub max_cpsz: size_t,
    pub ct_save: size_t,
    pub ct_restore: size_t,
}
pub type aco_save_stack_t = __anonstruct_aco_save_stack_t_539581013;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aco_s {
    pub reg: [*mut libc::c_void; 9],
    pub main_co: *mut aco_t,
    pub arg: *mut libc::c_void,
    pub is_end: libc::c_char,
    pub fp: Option::<unsafe extern "C" fn() -> ()>,
    pub save_stack: aco_save_stack_t,
    pub share_stack: *mut aco_share_stack_t,
}
pub type aco_share_stack_t = __anonstruct_aco_share_stack_t_624142846;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_aco_share_stack_t_624142846 {
    pub ptr: *mut libc::c_void,
    pub sz: size_t,
    pub align_highptr: *mut libc::c_void,
    pub align_retptr: *mut libc::c_void,
    pub align_validsz: size_t,
    pub align_limit: size_t,
    pub owner: *mut aco_t,
    pub guard_page_enabled: libc::c_char,
    pub real_ptr: *mut libc::c_void,
    pub real_sz: size_t,
}
pub type aco_t = aco_s;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
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
#[thread_local]
pub static mut aco_gtls_co: *mut aco_t = 0 as *const aco_t as *mut aco_t;
pub unsafe extern "C" fn aco_runtime_test() {
    let mut tmp: libc::c_long = 0;
    let mut tmp___0: libc::c_long = 0;
    tmp = (::std::mem::size_of::<libc::c_int>() as libc::c_ulong >= 4 as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp == 0 {
        abort();
    }
    tmp___0 = (::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        <= ::std::mem::size_of::<size_t>() as libc::c_ulong) as libc::c_int
        as libc::c_long;
    if tmp___0 == 0 {
        abort();
    }
}
unsafe extern "C" fn aco_default_protector_last_word() {
    let mut co: *mut aco_t = 0 as *mut aco_t;
    let mut tmp: libc::c_long = 0;
    co = aco_gtls_co;
    fprintf(
        stderr,
        b"error: aco_default_protector_last_word triggered\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"error: co:%p should call `aco_exit()` instead of direct `return` in co_fp:%p to finish its execution\n\0"
            as *const u8 as *const libc::c_char,
        co,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            *mut libc::c_void,
        >((*co).fp),
    );
    tmp = 0 as libc::c_long;
    if tmp == 0 {
        abort();
    }
}
#[thread_local]
static mut aco_gtls_last_word_fp: Option::<unsafe extern "C" fn() -> ()> = Some(
    aco_default_protector_last_word as unsafe extern "C" fn() -> (),
);
#[thread_local]
static mut aco_gtls_fpucw_mxcsr: [*mut libc::c_void; 1] = [0 as *const libc::c_void
    as *mut libc::c_void; 1];
pub unsafe extern "C" fn aco_thread_init(
    mut last_word_co_fp: Option::<unsafe extern "C" fn() -> ()>,
) {
    aco_save_fpucw_mxcsr(aco_gtls_fpucw_mxcsr.as_mut_ptr() as *mut libc::c_void);
    if ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        *mut libc::c_void,
    >(last_word_co_fp) as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
    {
        aco_gtls_last_word_fp = last_word_co_fp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn aco_funcp_protector() {
    let mut tmp: libc::c_long = 0;
    if ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        *mut libc::c_void,
    >(aco_gtls_last_word_fp) as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
    {
        (Some(aco_gtls_last_word_fp.expect("non-null function pointer")))
            .expect("non-null function pointer")();
    } else {
        aco_default_protector_last_word();
    }
    tmp = 0 as libc::c_long;
    if tmp == 0 {
        abort();
    }
}
pub unsafe extern "C" fn aco_share_stack_new(mut sz: size_t) -> *mut aco_share_stack_t {
    let mut tmp: *mut aco_share_stack_t = 0 as *mut aco_share_stack_t;
    tmp = aco_share_stack_new2(sz, 1 as libc::c_int as libc::c_char);
    return tmp;
}
pub unsafe extern "C" fn aco_share_stack_new2(
    mut sz: size_t,
    mut guard_page_enabled: libc::c_char,
) -> *mut aco_share_stack_t {
    let mut tmp: libc::c_long = 0;
    let mut u_pgsz: size_t = 0;
    let mut pgsz: libc::c_long = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_long = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_long = 0;
    let mut new_sz: size_t = 0;
    let mut tmp___5: libc::c_long = 0;
    let mut tmp___6: libc::c_long = 0;
    let mut tmp___7: libc::c_long = 0;
    let mut tmp___8: libc::c_long = 0;
    let mut tmp___9: libc::c_long = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_long = 0;
    let mut p: *mut aco_share_stack_t = 0 as *mut aco_share_stack_t;
    let mut tmp___12: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___13: libc::c_long = 0;
    let mut tmp___14: libc::c_long = 0;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: libc::c_long = 0;
    let mut tmp___18: libc::c_long = 0;
    let mut tmp___19: libc::c_long = 0;
    let mut u_p: uintptr_t = 0;
    let mut tmp___20: libc::c_long = 0;
    if sz == 0 as libc::c_ulong {
        sz = 2097152 as libc::c_int as size_t;
    }
    if sz < 4096 as libc::c_ulong {
        sz = 4096 as libc::c_int as size_t;
    }
    tmp = (sz > 0 as libc::c_ulong) as libc::c_int as libc::c_long;
    if tmp == 0 {
        abort();
    }
    u_pgsz = 0 as libc::c_int as size_t;
    if guard_page_enabled as libc::c_int != 0 as libc::c_int {
        tmp___0 = sysconf(30 as libc::c_int);
        pgsz = tmp___0;
        if pgsz > 0 as libc::c_long {
            if pgsz - 1 as libc::c_long & pgsz == 0 as libc::c_long {
                tmp___1 = 1 as libc::c_int;
            } else {
                tmp___1 = 0 as libc::c_int;
            }
        } else {
            tmp___1 = 0 as libc::c_int;
        }
        tmp___2 = tmp___1 as libc::c_long;
        if tmp___2 == 0 {
            abort();
        }
        u_pgsz = pgsz as libc::c_ulong;
        if u_pgsz == pgsz as libc::c_ulong {
            if u_pgsz << 1 as libc::c_int >> 1 as libc::c_int == u_pgsz {
                tmp___3 = 1 as libc::c_int;
            } else {
                tmp___3 = 0 as libc::c_int;
            }
        } else {
            tmp___3 = 0 as libc::c_int;
        }
        tmp___4 = tmp___3 as libc::c_long;
        if tmp___4 == 0 {
            abort();
        }
        if sz <= u_pgsz {
            sz = u_pgsz << 1 as libc::c_int;
        } else {
            if sz & u_pgsz.wrapping_sub(1 as libc::c_ulong) != 0 as libc::c_ulong {
                new_sz = sz & !u_pgsz.wrapping_sub(1 as libc::c_ulong);
                tmp___5 = (new_sz >= u_pgsz) as libc::c_int as libc::c_long;
                if tmp___5 == 0 {
                    abort();
                }
                tmp___6 = (new_sz.wrapping_add(u_pgsz << 1 as libc::c_int) >= new_sz)
                    as libc::c_int as libc::c_long;
                if tmp___6 == 0 {
                    abort();
                }
                new_sz = (new_sz as libc::c_ulong)
                    .wrapping_add(u_pgsz << 1 as libc::c_int) as size_t as size_t;
                tmp___7 = (sz.wrapping_div(u_pgsz).wrapping_add(2 as libc::c_ulong)
                    == new_sz.wrapping_div(u_pgsz)) as libc::c_int as libc::c_long;
                if tmp___7 == 0 {
                    abort();
                }
            } else {
                tmp___8 = (sz.wrapping_add(u_pgsz) >= sz) as libc::c_int as libc::c_long;
                if tmp___8 == 0 {
                    abort();
                }
                new_sz = sz.wrapping_add(u_pgsz);
                tmp___9 = (sz.wrapping_div(u_pgsz).wrapping_add(1 as libc::c_ulong)
                    == new_sz.wrapping_div(u_pgsz)) as libc::c_int as libc::c_long;
                if tmp___9 == 0 {
                    abort();
                }
            }
            sz = new_sz;
            if sz.wrapping_div(u_pgsz) > 1 as libc::c_ulong {
                if sz & u_pgsz.wrapping_sub(1 as libc::c_ulong) == 0 as libc::c_ulong {
                    tmp___10 = 1 as libc::c_int;
                } else {
                    tmp___10 = 0 as libc::c_int;
                }
            } else {
                tmp___10 = 0 as libc::c_int;
            }
            tmp___11 = tmp___10 as libc::c_long;
            if tmp___11 == 0 {
                abort();
            }
        }
    }
    tmp___12 = malloc(::std::mem::size_of::<aco_share_stack_t>() as libc::c_ulong);
    p = tmp___12 as *mut aco_share_stack_t;
    tmp___13 = (p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___13 != 0 {
        fprintf(
            stderr,
            b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                as *const libc::c_char,
            b"aco.c\0" as *const u8 as *const libc::c_char,
            248 as libc::c_int,
            b"aco_share_stack_new2\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<aco_share_stack_t>() as libc::c_ulong,
    );
    if guard_page_enabled as libc::c_int != 0 as libc::c_int {
        (*p)
            .real_ptr = mmap(
            0 as *mut libc::c_void,
            sz,
            3 as libc::c_int,
            34 as libc::c_int,
            -(1 as libc::c_int),
            0 as libc::c_int as __off_t,
        );
        tmp___14 = !((*p).real_ptr as libc::c_ulong
            != -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong) as libc::c_int
            as libc::c_long;
        if tmp___14 != 0 {
            fprintf(
                stderr,
                b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                    as *const libc::c_char,
                b"aco.c\0" as *const u8 as *const libc::c_char,
                255 as libc::c_int,
                b"aco_share_stack_new2\0" as *const u8 as *const libc::c_char,
            );
            abort();
        }
        (*p).guard_page_enabled = 1 as libc::c_int as libc::c_char;
        tmp___15 = mprotect((*p).real_ptr, u_pgsz, 1 as libc::c_int);
        if 0 as libc::c_int == tmp___15 {
            tmp___16 = 1 as libc::c_int;
        } else {
            tmp___16 = 0 as libc::c_int;
        }
        tmp___17 = tmp___16 as libc::c_long;
        if tmp___17 == 0 {
            abort();
        }
        (*p)
            .ptr = ((*p).real_ptr as uintptr_t).wrapping_add(u_pgsz)
            as *mut libc::c_void;
        (*p).real_sz = sz;
        tmp___18 = (sz >= u_pgsz << 1 as libc::c_int) as libc::c_int as libc::c_long;
        if tmp___18 == 0 {
            abort();
        }
        (*p).sz = sz.wrapping_sub(u_pgsz);
    } else {
        (*p).sz = sz;
        (*p).ptr = malloc(sz);
        tmp___19 = ((*p).ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
            as libc::c_int as libc::c_long;
        if tmp___19 != 0 {
            fprintf(
                stderr,
                b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                    as *const libc::c_char,
                b"aco.c\0" as *const u8 as *const libc::c_char,
                267 as libc::c_int,
                b"aco_share_stack_new2\0" as *const u8 as *const libc::c_char,
            );
            abort();
        }
    }
    (*p).owner = 0 as *mut libc::c_void as *mut aco_t;
    u_p = ((*p).sz)
        .wrapping_sub(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                << 1 as libc::c_int,
        )
        .wrapping_add((*p).ptr as uintptr_t);
    u_p = (u_p >> 4 as libc::c_int) << 4 as libc::c_int;
    (*p).align_highptr = u_p as *mut libc::c_void;
    (*p)
        .align_retptr = u_p
        .wrapping_sub(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        as *mut libc::c_void;
    let ref mut fresh0 = *((*p).align_retptr as *mut *mut libc::c_void);
    *fresh0 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        *mut libc::c_void,
    >(Some(aco_funcp_protector_asm as unsafe extern "C" fn() -> ()));
    tmp___20 = ((*p).sz
        > (16 as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    << 1 as libc::c_int,
            )
            .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong))
        as libc::c_int as libc::c_long;
    if tmp___20 == 0 {
        abort();
    }
    (*p)
        .align_limit = ((*p).sz)
        .wrapping_sub(16 as libc::c_ulong)
        .wrapping_sub(
            (::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                << 1 as libc::c_int,
        );
    return p;
}
pub unsafe extern "C" fn aco_share_stack_destroy(mut sstk: *mut aco_share_stack_t) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_long = 0;
    if sstk as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if (*sstk).ptr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
    } else {
        tmp = 0 as libc::c_int;
    }
    tmp___0 = tmp as libc::c_long;
    if tmp___0 == 0 {
        abort();
    }
    if (*sstk).guard_page_enabled != 0 {
        tmp___1 = munmap((*sstk).real_ptr, (*sstk).real_sz);
        if 0 as libc::c_int == tmp___1 {
            tmp___2 = 1 as libc::c_int;
        } else {
            tmp___2 = 0 as libc::c_int;
        }
        tmp___3 = tmp___2 as libc::c_long;
        if tmp___3 == 0 {
            abort();
        }
        (*sstk).real_ptr = 0 as *mut libc::c_void;
        (*sstk).ptr = 0 as *mut libc::c_void;
    } else {
        free((*sstk).ptr);
        (*sstk).ptr = 0 as *mut libc::c_void;
    }
    free(sstk as *mut libc::c_void);
}
pub unsafe extern "C" fn aco_create(
    mut main_co: *mut aco_t,
    mut share_stack: *mut aco_share_stack_t,
    mut save_stack_sz: size_t,
    mut fp: Option::<unsafe extern "C" fn() -> ()>,
    mut arg: *mut libc::c_void,
) -> *mut aco_t {
    let mut p: *mut aco_t = 0 as *mut aco_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: libc::c_long = 0;
    let mut tmp___2: libc::c_long = 0;
    let mut tmp___3: libc::c_long = 0;
    tmp = malloc(::std::mem::size_of::<aco_t>() as libc::c_ulong);
    p = tmp as *mut aco_t;
    tmp___0 = (p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___0 != 0 {
        fprintf(
            stderr,
            b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                as *const libc::c_char,
            b"aco.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
            b"aco_create\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    memset(
        p as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<aco_t>() as libc::c_ulong,
    );
    if main_co as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = (share_stack as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int as libc::c_long;
        if tmp___1 == 0 {
            abort();
        }
        (*p).share_stack = share_stack;
        (*p)
            .reg[4 as libc::c_int
            as usize] = ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            *mut libc::c_void,
        >(fp);
        (*p).reg[5 as libc::c_int as usize] = (*(*p).share_stack).align_retptr;
        (*p)
            .reg[8 as libc::c_int
            as usize] = aco_gtls_fpucw_mxcsr[0 as libc::c_int as usize];
        (*p).main_co = main_co;
        (*p).arg = arg;
        (*p).fp = fp;
        if save_stack_sz == 0 as libc::c_ulong {
            save_stack_sz = 64 as libc::c_int as size_t;
        }
        (*p).save_stack.ptr = malloc(save_stack_sz);
        tmp___2 = ((*p).save_stack.ptr as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int as libc::c_long;
        if tmp___2 != 0 {
            fprintf(
                stderr,
                b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                    as *const libc::c_char,
                b"aco.c\0" as *const u8 as *const libc::c_char,
                344 as libc::c_int,
                b"aco_create\0" as *const u8 as *const libc::c_char,
            );
            abort();
        }
        (*p).save_stack.sz = save_stack_sz;
        (*p).save_stack.valid_sz = 0 as libc::c_int as size_t;
        return p;
    } else {
        (*p).main_co = 0 as *mut libc::c_void as *mut aco_t;
        (*p).arg = arg;
        (*p).fp = fp;
        (*p).share_stack = 0 as *mut libc::c_void as *mut aco_share_stack_t;
        (*p).save_stack.ptr = 0 as *mut libc::c_void;
        return p;
    };
}
pub unsafe extern "C" fn aco_resume(mut resume_co: *mut aco_t) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut owner_co: *mut aco_t = 0 as *mut aco_t;
    let mut tmp___1: libc::c_long = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_long = 0;
    let mut tmp___4: libc::c_long = 0;
    let mut tmp___5: libc::c_long = 0;
    let mut xmm0: u128 = 0;
    let mut xmm1: u128 = 0;
    let mut xmm2: u128 = 0;
    let mut xmm3: u128 = 0;
    let mut xmm4: u128 = 0;
    let mut xmm5: u128 = 0;
    let mut xmm6: u128 = 0;
    let mut xmm7: u128 = 0;
    let mut tmp___6: libc::c_long = 0;
    let mut tmp___7: libc::c_long = 0;
    let mut xmm0___0: u128 = 0;
    let mut xmm1___0: u128 = 0;
    let mut xmm2___0: u128 = 0;
    let mut xmm3___0: u128 = 0;
    let mut xmm4___0: u128 = 0;
    let mut xmm5___0: u128 = 0;
    let mut xmm6___0: u128 = 0;
    let mut xmm7___0: u128 = 0;
    if resume_co as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if (*resume_co).main_co as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            if (*resume_co).is_end as libc::c_int == 0 as libc::c_int {
                tmp = 1 as libc::c_int;
            } else {
                tmp = 0 as libc::c_int;
            }
        } else {
            tmp = 0 as libc::c_int;
        }
    } else {
        tmp = 0 as libc::c_int;
    }
    tmp___0 = tmp as libc::c_long;
    if tmp___0 == 0 {
        abort();
    }
    if (*(*resume_co).share_stack).owner as libc::c_ulong != resume_co as libc::c_ulong {
        if (*(*resume_co).share_stack).owner as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            owner_co = (*(*resume_co).share_stack).owner;
            tmp___1 = ((*owner_co).share_stack as libc::c_ulong
                == (*resume_co).share_stack as libc::c_ulong) as libc::c_int
                as libc::c_long;
            if tmp___1 == 0 {
                abort();
            }
            if (*(*owner_co).share_stack).align_retptr as uintptr_t
                >= (*owner_co).reg[5 as libc::c_int as usize] as uintptr_t
            {
                if ((*(*owner_co).share_stack).align_highptr as uintptr_t)
                    .wrapping_sub((*(*owner_co).share_stack).align_limit)
                    <= (*owner_co).reg[5 as libc::c_int as usize] as uintptr_t
                {
                    tmp___2 = 1 as libc::c_int;
                } else {
                    tmp___2 = 0 as libc::c_int;
                }
            } else {
                tmp___2 = 0 as libc::c_int;
            }
            tmp___3 = tmp___2 as libc::c_long;
            if tmp___3 == 0 {
                abort();
            }
            (*owner_co)
                .save_stack
                .valid_sz = ((*(*owner_co).share_stack).align_retptr as uintptr_t)
                .wrapping_sub((*owner_co).reg[5 as libc::c_int as usize] as uintptr_t);
            if (*owner_co).save_stack.sz < (*owner_co).save_stack.valid_sz {
                free((*owner_co).save_stack.ptr);
                (*owner_co).save_stack.ptr = 0 as *mut libc::c_void;
                loop {
                    (*owner_co).save_stack.sz <<= 1 as libc::c_int;
                    tmp___4 = ((*owner_co).save_stack.sz > 0 as libc::c_ulong)
                        as libc::c_int as libc::c_long;
                    if tmp___4 == 0 {
                        abort();
                    }
                    if (*owner_co).save_stack.sz >= (*owner_co).save_stack.valid_sz {
                        break;
                    }
                }
                (*owner_co).save_stack.ptr = malloc((*owner_co).save_stack.sz);
                tmp___5 = ((*owner_co).save_stack.ptr as libc::c_ulong
                    == 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int
                    as libc::c_long;
                if tmp___5 != 0 {
                    fprintf(
                        stderr,
                        b"Aborting: failed to allocate memory: %s:%d:%s\n\0" as *const u8
                            as *const libc::c_char,
                        b"aco.c\0" as *const u8 as *const libc::c_char,
                        403 as libc::c_int,
                        b"aco_resume\0" as *const u8 as *const libc::c_char,
                    );
                    abort();
                }
            }
            if (*owner_co).save_stack.valid_sz > 0 as libc::c_ulong {
                if (*owner_co).reg[5 as libc::c_int as usize] as uintptr_t
                    & 15 as libc::c_ulong == 0 as libc::c_ulong
                {
                    if (*owner_co).save_stack.ptr as uintptr_t & 15 as libc::c_ulong
                        == 0 as libc::c_ulong
                    {
                        if (*owner_co).save_stack.valid_sz & 15 as libc::c_ulong
                            == 8 as libc::c_ulong
                        {
                            if (*owner_co).save_stack.valid_sz >> 4 as libc::c_int
                                >= 0 as libc::c_ulong
                            {
                                if (*owner_co).save_stack.valid_sz >> 4 as libc::c_int
                                    <= 8 as libc::c_ulong
                                {
                                    match (*owner_co).save_stack.valid_sz >> 4 as libc::c_int {
                                        1 => {
                                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(0 as libc::c_int as isize);
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(0 as libc::c_int as isize) = xmm0;
                                        }
                                        2 => {
                                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(0 as libc::c_int as isize);
                                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(1 as libc::c_int as isize);
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(0 as libc::c_int as isize) = xmm0;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(1 as libc::c_int as isize) = xmm1;
                                        }
                                        3 => {
                                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(0 as libc::c_int as isize);
                                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(1 as libc::c_int as isize);
                                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(2 as libc::c_int as isize);
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(0 as libc::c_int as isize) = xmm0;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(1 as libc::c_int as isize) = xmm1;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(2 as libc::c_int as isize) = xmm2;
                                        }
                                        4 => {
                                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(0 as libc::c_int as isize);
                                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(1 as libc::c_int as isize);
                                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(2 as libc::c_int as isize);
                                            xmm3 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(3 as libc::c_int as isize);
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(0 as libc::c_int as isize) = xmm0;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(1 as libc::c_int as isize) = xmm1;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(2 as libc::c_int as isize) = xmm2;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(3 as libc::c_int as isize) = xmm3;
                                        }
                                        5 => {
                                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(0 as libc::c_int as isize);
                                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(1 as libc::c_int as isize);
                                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(2 as libc::c_int as isize);
                                            xmm3 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(3 as libc::c_int as isize);
                                            xmm4 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(4 as libc::c_int as isize);
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(0 as libc::c_int as isize) = xmm0;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(1 as libc::c_int as isize) = xmm1;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(2 as libc::c_int as isize) = xmm2;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(3 as libc::c_int as isize) = xmm3;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(4 as libc::c_int as isize) = xmm4;
                                        }
                                        6 => {
                                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(0 as libc::c_int as isize);
                                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(1 as libc::c_int as isize);
                                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(2 as libc::c_int as isize);
                                            xmm3 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(3 as libc::c_int as isize);
                                            xmm4 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(4 as libc::c_int as isize);
                                            xmm5 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(5 as libc::c_int as isize);
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(0 as libc::c_int as isize) = xmm0;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(1 as libc::c_int as isize) = xmm1;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(2 as libc::c_int as isize) = xmm2;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(3 as libc::c_int as isize) = xmm3;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(4 as libc::c_int as isize) = xmm4;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(5 as libc::c_int as isize) = xmm5;
                                        }
                                        7 => {
                                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(0 as libc::c_int as isize);
                                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(1 as libc::c_int as isize);
                                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(2 as libc::c_int as isize);
                                            xmm3 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(3 as libc::c_int as isize);
                                            xmm4 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(4 as libc::c_int as isize);
                                            xmm5 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(5 as libc::c_int as isize);
                                            xmm6 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(6 as libc::c_int as isize);
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(0 as libc::c_int as isize) = xmm0;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(1 as libc::c_int as isize) = xmm1;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(2 as libc::c_int as isize) = xmm2;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(3 as libc::c_int as isize) = xmm3;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(4 as libc::c_int as isize) = xmm4;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(5 as libc::c_int as isize) = xmm5;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(6 as libc::c_int as isize) = xmm6;
                                        }
                                        8 => {
                                            xmm0 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(0 as libc::c_int as isize);
                                            xmm1 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(1 as libc::c_int as isize);
                                            xmm2 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(2 as libc::c_int as isize);
                                            xmm3 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(3 as libc::c_int as isize);
                                            xmm4 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(4 as libc::c_int as isize);
                                            xmm5 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(5 as libc::c_int as isize);
                                            xmm6 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(6 as libc::c_int as isize);
                                            xmm7 = *((*owner_co).reg[5 as libc::c_int as usize]
                                                as *mut u128)
                                                .offset(7 as libc::c_int as isize);
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(0 as libc::c_int as isize) = xmm0;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(1 as libc::c_int as isize) = xmm1;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(2 as libc::c_int as isize) = xmm2;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(3 as libc::c_int as isize) = xmm3;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(4 as libc::c_int as isize) = xmm4;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(5 as libc::c_int as isize) = xmm5;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(6 as libc::c_int as isize) = xmm6;
                                            *((*owner_co).save_stack.ptr as *mut u128)
                                                .offset(7 as libc::c_int as isize) = xmm7;
                                        }
                                        0 | _ => {}
                                    }
                                    *(((*owner_co).save_stack.ptr as uintptr_t)
                                        .wrapping_add((*owner_co).save_stack.valid_sz)
                                        .wrapping_sub(8 as libc::c_ulong)
                                        as *mut uint64_t) = *(((*owner_co)
                                        .reg[5 as libc::c_int as usize] as uintptr_t)
                                        .wrapping_add((*owner_co).save_stack.valid_sz)
                                        .wrapping_sub(8 as libc::c_ulong) as *mut uint64_t);
                                } else {
                                    memcpy(
                                        (*owner_co).save_stack.ptr,
                                        (*owner_co).reg[5 as libc::c_int as usize]
                                            as *const libc::c_void,
                                        (*owner_co).save_stack.valid_sz,
                                    );
                                }
                            } else {
                                memcpy(
                                    (*owner_co).save_stack.ptr,
                                    (*owner_co).reg[5 as libc::c_int as usize]
                                        as *const libc::c_void,
                                    (*owner_co).save_stack.valid_sz,
                                );
                            }
                        } else {
                            memcpy(
                                (*owner_co).save_stack.ptr,
                                (*owner_co).reg[5 as libc::c_int as usize]
                                    as *const libc::c_void,
                                (*owner_co).save_stack.valid_sz,
                            );
                        }
                    } else {
                        memcpy(
                            (*owner_co).save_stack.ptr,
                            (*owner_co).reg[5 as libc::c_int as usize]
                                as *const libc::c_void,
                            (*owner_co).save_stack.valid_sz,
                        );
                    }
                } else {
                    memcpy(
                        (*owner_co).save_stack.ptr,
                        (*owner_co).reg[5 as libc::c_int as usize]
                            as *const libc::c_void,
                        (*owner_co).save_stack.valid_sz,
                    );
                }
                (*owner_co)
                    .save_stack
                    .ct_save = ((*owner_co).save_stack.ct_save).wrapping_add(1);
            }
            if (*owner_co).save_stack.valid_sz > (*owner_co).save_stack.max_cpsz {
                (*owner_co).save_stack.max_cpsz = (*owner_co).save_stack.valid_sz;
            }
            (*(*owner_co).share_stack).owner = 0 as *mut libc::c_void as *mut aco_t;
            (*(*owner_co).share_stack).align_validsz = 0 as libc::c_int as size_t;
        }
        tmp___6 = ((*(*resume_co).share_stack).owner as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int as libc::c_long;
        if tmp___6 == 0 {
            abort();
        }
        tmp___7 = ((*resume_co).save_stack.valid_sz
            <= ((*(*resume_co).share_stack).align_limit)
                .wrapping_sub(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                )) as libc::c_int as libc::c_long;
        if tmp___7 == 0 {
            abort();
        }
        if (*resume_co).save_stack.valid_sz > 0 as libc::c_ulong {
            if (*resume_co).save_stack.ptr as uintptr_t & 15 as libc::c_ulong
                == 0 as libc::c_ulong
            {
                if ((*(*resume_co).share_stack).align_retptr as uintptr_t)
                    .wrapping_sub((*resume_co).save_stack.valid_sz) as *mut libc::c_void
                    as uintptr_t & 15 as libc::c_ulong == 0 as libc::c_ulong
                {
                    if (*resume_co).save_stack.valid_sz & 15 as libc::c_ulong
                        == 8 as libc::c_ulong
                    {
                        if (*resume_co).save_stack.valid_sz >> 4 as libc::c_int
                            >= 0 as libc::c_ulong
                        {
                            if (*resume_co).save_stack.valid_sz >> 4 as libc::c_int
                                <= 8 as libc::c_ulong
                            {
                                match (*resume_co).save_stack.valid_sz >> 4 as libc::c_int {
                                    1 => {
                                        xmm0___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(0 as libc::c_int as isize);
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(0 as libc::c_int as isize) = xmm0___0;
                                    }
                                    2 => {
                                        xmm0___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(0 as libc::c_int as isize);
                                        xmm1___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(1 as libc::c_int as isize);
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(0 as libc::c_int as isize) = xmm0___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(1 as libc::c_int as isize) = xmm1___0;
                                    }
                                    3 => {
                                        xmm0___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(0 as libc::c_int as isize);
                                        xmm1___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(1 as libc::c_int as isize);
                                        xmm2___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(2 as libc::c_int as isize);
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(0 as libc::c_int as isize) = xmm0___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(1 as libc::c_int as isize) = xmm1___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(2 as libc::c_int as isize) = xmm2___0;
                                    }
                                    4 => {
                                        xmm0___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(0 as libc::c_int as isize);
                                        xmm1___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(1 as libc::c_int as isize);
                                        xmm2___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(2 as libc::c_int as isize);
                                        xmm3___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(3 as libc::c_int as isize);
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(0 as libc::c_int as isize) = xmm0___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(1 as libc::c_int as isize) = xmm1___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(2 as libc::c_int as isize) = xmm2___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(3 as libc::c_int as isize) = xmm3___0;
                                    }
                                    5 => {
                                        xmm0___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(0 as libc::c_int as isize);
                                        xmm1___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(1 as libc::c_int as isize);
                                        xmm2___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(2 as libc::c_int as isize);
                                        xmm3___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(3 as libc::c_int as isize);
                                        xmm4___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(4 as libc::c_int as isize);
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(0 as libc::c_int as isize) = xmm0___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(1 as libc::c_int as isize) = xmm1___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(2 as libc::c_int as isize) = xmm2___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(3 as libc::c_int as isize) = xmm3___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(4 as libc::c_int as isize) = xmm4___0;
                                    }
                                    6 => {
                                        xmm0___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(0 as libc::c_int as isize);
                                        xmm1___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(1 as libc::c_int as isize);
                                        xmm2___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(2 as libc::c_int as isize);
                                        xmm3___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(3 as libc::c_int as isize);
                                        xmm4___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(4 as libc::c_int as isize);
                                        xmm5___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(5 as libc::c_int as isize);
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(0 as libc::c_int as isize) = xmm0___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(1 as libc::c_int as isize) = xmm1___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(2 as libc::c_int as isize) = xmm2___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(3 as libc::c_int as isize) = xmm3___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(4 as libc::c_int as isize) = xmm4___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(5 as libc::c_int as isize) = xmm5___0;
                                    }
                                    7 => {
                                        xmm0___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(0 as libc::c_int as isize);
                                        xmm1___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(1 as libc::c_int as isize);
                                        xmm2___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(2 as libc::c_int as isize);
                                        xmm3___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(3 as libc::c_int as isize);
                                        xmm4___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(4 as libc::c_int as isize);
                                        xmm5___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(5 as libc::c_int as isize);
                                        xmm6___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(6 as libc::c_int as isize);
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(0 as libc::c_int as isize) = xmm0___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(1 as libc::c_int as isize) = xmm1___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(2 as libc::c_int as isize) = xmm2___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(3 as libc::c_int as isize) = xmm3___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(4 as libc::c_int as isize) = xmm4___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(5 as libc::c_int as isize) = xmm5___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(6 as libc::c_int as isize) = xmm6___0;
                                    }
                                    8 => {
                                        xmm0___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(0 as libc::c_int as isize);
                                        xmm1___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(1 as libc::c_int as isize);
                                        xmm2___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(2 as libc::c_int as isize);
                                        xmm3___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(3 as libc::c_int as isize);
                                        xmm4___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(4 as libc::c_int as isize);
                                        xmm5___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(5 as libc::c_int as isize);
                                        xmm6___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(6 as libc::c_int as isize);
                                        xmm7___0 = *((*resume_co).save_stack.ptr as *mut u128)
                                            .offset(7 as libc::c_int as isize);
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(0 as libc::c_int as isize) = xmm0___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(1 as libc::c_int as isize) = xmm1___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(2 as libc::c_int as isize) = xmm2___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(3 as libc::c_int as isize) = xmm3___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(4 as libc::c_int as isize) = xmm4___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(5 as libc::c_int as isize) = xmm5___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(6 as libc::c_int as isize) = xmm6___0;
                                        *(((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                                            as *mut libc::c_void as *mut u128)
                                            .offset(7 as libc::c_int as isize) = xmm7___0;
                                    }
                                    0 | _ => {}
                                }
                                *((((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                    .wrapping_sub((*resume_co).save_stack.valid_sz)
                                    as *mut libc::c_void as uintptr_t)
                                    .wrapping_add((*resume_co).save_stack.valid_sz)
                                    .wrapping_sub(8 as libc::c_ulong)
                                    as *mut uint64_t) = *(((*resume_co).save_stack.ptr
                                    as uintptr_t)
                                    .wrapping_add((*resume_co).save_stack.valid_sz)
                                    .wrapping_sub(8 as libc::c_ulong) as *mut uint64_t);
                            } else {
                                memcpy(
                                    ((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                        .wrapping_sub((*resume_co).save_stack.valid_sz)
                                        as *mut libc::c_void,
                                    (*resume_co).save_stack.ptr as *const libc::c_void,
                                    (*resume_co).save_stack.valid_sz,
                                );
                            }
                        } else {
                            memcpy(
                                ((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                    .wrapping_sub((*resume_co).save_stack.valid_sz)
                                    as *mut libc::c_void,
                                (*resume_co).save_stack.ptr as *const libc::c_void,
                                (*resume_co).save_stack.valid_sz,
                            );
                        }
                    } else {
                        memcpy(
                            ((*(*resume_co).share_stack).align_retptr as uintptr_t)
                                .wrapping_sub((*resume_co).save_stack.valid_sz)
                                as *mut libc::c_void,
                            (*resume_co).save_stack.ptr as *const libc::c_void,
                            (*resume_co).save_stack.valid_sz,
                        );
                    }
                } else {
                    memcpy(
                        ((*(*resume_co).share_stack).align_retptr as uintptr_t)
                            .wrapping_sub((*resume_co).save_stack.valid_sz)
                            as *mut libc::c_void,
                        (*resume_co).save_stack.ptr as *const libc::c_void,
                        (*resume_co).save_stack.valid_sz,
                    );
                }
            } else {
                memcpy(
                    ((*(*resume_co).share_stack).align_retptr as uintptr_t)
                        .wrapping_sub((*resume_co).save_stack.valid_sz)
                        as *mut libc::c_void,
                    (*resume_co).save_stack.ptr as *const libc::c_void,
                    (*resume_co).save_stack.valid_sz,
                );
            }
            (*resume_co)
                .save_stack
                .ct_restore = ((*resume_co).save_stack.ct_restore).wrapping_add(1);
        }
        if (*resume_co).save_stack.valid_sz > (*resume_co).save_stack.max_cpsz {
            (*resume_co).save_stack.max_cpsz = (*resume_co).save_stack.valid_sz;
        }
        (*(*resume_co).share_stack)
            .align_validsz = ((*resume_co).save_stack.valid_sz)
            .wrapping_add(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong);
        (*(*resume_co).share_stack).owner = resume_co;
    }
    aco_gtls_co = resume_co;
    acosw((*resume_co).main_co, resume_co);
    aco_gtls_co = (*resume_co).main_co;
}
pub unsafe extern "C" fn aco_destroy(mut co: *mut aco_t) {
    let mut tmp: libc::c_long = 0;
    tmp = (co as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int
        as libc::c_long;
    if tmp == 0 {
        abort();
    }
    if (*co).main_co as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(co as *mut libc::c_void);
    } else {
        if (*(*co).share_stack).owner as libc::c_ulong == co as libc::c_ulong {
            (*(*co).share_stack).owner = 0 as *mut libc::c_void as *mut aco_t;
            (*(*co).share_stack).align_validsz = 0 as libc::c_int as size_t;
        }
        free((*co).save_stack.ptr);
        (*co).save_stack.ptr = 0 as *mut libc::c_void;
        free(co as *mut libc::c_void);
    };
}
pub static mut gl_race_aco_yield_ct: uint64_t = 0 as libc::c_int as uint64_t;
pub static mut gl_race_aco_yield_ct_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
};
pub unsafe extern "C" fn foo(mut ct: libc::c_int) {
    let mut tmp: libc::c_long = 0;
    let mut tmp___0: libc::c_long = 0;
    printf(
        b"co:%p save_stack:%p share_stack:%p yield_ct:%d\n\0" as *const u8
            as *const libc::c_char,
        aco_gtls_co,
        (*aco_gtls_co).save_stack.ptr,
        (*(*aco_gtls_co).share_stack).ptr,
        ct,
    );
    pthread_mutex_lock(&mut gl_race_aco_yield_ct_mutex);
    gl_race_aco_yield_ct = gl_race_aco_yield_ct.wrapping_add(1);
    pthread_mutex_unlock(&mut gl_race_aco_yield_ct_mutex);
    tmp = (aco_gtls_co as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp == 0 {
        abort();
    }
    tmp___0 = ((*aco_gtls_co).main_co as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int as libc::c_long;
    if tmp___0 == 0 {
        abort();
    }
    acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    let ref mut fresh1 = *((*aco_gtls_co).arg as *mut libc::c_int);
    *fresh1 += 1;
}
pub unsafe extern "C" fn co_fp0() {
    let mut this_co: *mut aco_t = 0 as *mut aco_t;
    let mut tmp: libc::c_long = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: libc::c_long = 0;
    let mut ct: libc::c_int = 0;
    let mut tmp___2: libc::c_long = 0;
    let mut tmp___3: libc::c_long = 0;
    let mut tmp___4: libc::c_long = 0;
    let mut tmp___5: libc::c_long = 0;
    let mut tmp___6: libc::c_long = 0;
    this_co = aco_gtls_co;
    tmp = !((*this_co).main_co as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int as libc::c_long;
    if tmp == 0 {
        abort();
    }
    tmp___0 = (::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        libc::c_ulong,
    >((*this_co).fp)
        == ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            *mut libc::c_void,
        >(Some(co_fp0 as unsafe extern "C" fn() -> ())) as libc::c_ulong) as libc::c_int
        as libc::c_long;
    if tmp___0 == 0 {
        abort();
    }
    tmp___1 = ((*this_co).is_end as libc::c_int == 0 as libc::c_int) as libc::c_int
        as libc::c_long;
    if tmp___1 == 0 {
        abort();
    }
    ct = 0 as libc::c_int;
    while ct < 6 as libc::c_int {
        foo(ct);
        ct += 1;
    }
    printf(
        b"co:%p save_stack:%p share_stack:%p co_exit()\n\0" as *const u8
            as *const libc::c_char,
        this_co,
        (*this_co).save_stack.ptr,
        (*(*this_co).share_stack).ptr,
    );
    pthread_mutex_lock(&mut gl_race_aco_yield_ct_mutex);
    gl_race_aco_yield_ct = gl_race_aco_yield_ct.wrapping_add(1);
    pthread_mutex_unlock(&mut gl_race_aco_yield_ct_mutex);
    (*aco_gtls_co).is_end = 1 as libc::c_int as libc::c_char;
    tmp___2 = ((*(*aco_gtls_co).share_stack).owner as libc::c_ulong
        == aco_gtls_co as libc::c_ulong) as libc::c_int as libc::c_long;
    if tmp___2 == 0 {
        abort();
    }
    (*(*aco_gtls_co).share_stack).owner = 0 as *mut libc::c_void as *mut aco_t;
    (*(*aco_gtls_co).share_stack).align_validsz = 0 as libc::c_int as size_t;
    tmp___3 = (aco_gtls_co as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___3 == 0 {
        abort();
    }
    tmp___4 = ((*aco_gtls_co).main_co as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int as libc::c_long;
    if tmp___4 == 0 {
        abort();
    }
    acosw(aco_gtls_co, (*aco_gtls_co).main_co);
    tmp___5 = 0 as libc::c_long;
    if tmp___5 == 0 {
        abort();
    }
    tmp___6 = 0 as libc::c_long;
    if tmp___6 == 0 {
        abort();
    }
}
pub unsafe extern "C" fn pmain(
    mut pthread_in_arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut t: pthread_t = 0;
    let mut tmp: pthread_t = 0;
    let mut idx: size_t = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut main_co: *mut aco_t = 0 as *mut aco_t;
    let mut tmp___1: *mut aco_t = 0 as *mut aco_t;
    let mut tmp___2: libc::c_long = 0;
    let mut sstk: *mut aco_share_stack_t = 0 as *mut aco_share_stack_t;
    let mut tmp___3: *mut aco_share_stack_t = 0 as *mut aco_share_stack_t;
    let mut tmp___4: libc::c_long = 0;
    let mut sstk2: *mut aco_share_stack_t = 0 as *mut aco_share_stack_t;
    let mut tmp___5: *mut aco_share_stack_t = 0 as *mut aco_share_stack_t;
    let mut tmp___6: libc::c_long = 0;
    let mut co_ct_arg_point_to_me: libc::c_int = 0;
    let mut co2_ct_arg_point_to_me: libc::c_int = 0;
    let mut co3_ct_arg_point_to_me: libc::c_int = 0;
    let mut co: *mut aco_t = 0 as *mut aco_t;
    let mut tmp___7: *mut aco_t = 0 as *mut aco_t;
    let mut tmp___8: libc::c_long = 0;
    let mut co2: *mut aco_t = 0 as *mut aco_t;
    let mut tmp___9: *mut aco_t = 0 as *mut aco_t;
    let mut co3: *mut aco_t = 0 as *mut aco_t;
    let mut tmp___10: *mut aco_t = 0 as *mut aco_t;
    let mut tmp___11: libc::c_long = 0;
    let mut tmp___12: libc::c_long = 0;
    let mut ct: libc::c_int = 0;
    let mut tmp___13: libc::c_long = 0;
    let mut tmp___14: libc::c_long = 0;
    let mut tmp___15: libc::c_long = 0;
    let mut tmp___16: libc::c_long = 0;
    let mut tmp___17: libc::c_long = 0;
    let mut tmp___18: libc::c_long = 0;
    let mut tmp___19: libc::c_long = 0;
    let mut tmp___20: libc::c_long = 0;
    let mut tmp___21: libc::c_long = 0;
    let mut tmp___22: libc::c_long = 0;
    let mut tmp___23: libc::c_long = 0;
    let mut tmp___24: libc::c_long = 0;
    let mut tmp_gl_ct: uint64_t = 0;
    tmp = pthread_self();
    t = tmp;
    idx = 0 as libc::c_int as size_t;
    tmp___0 = (::std::mem::size_of::<pthread_t>() as libc::c_ulong > 0 as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___0 == 0 {
        abort();
    }
    printf(b"\ntid:0x\0" as *const u8 as *const libc::c_char);
    while idx < ::std::mem::size_of::<pthread_t>() as libc::c_ulong {
        printf(
            b"%02x\0" as *const u8 as *const libc::c_char,
            *(&mut t as *mut pthread_t as *mut uint8_t).offset(idx as isize)
                as libc::c_int,
        );
        idx = idx.wrapping_add(1);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    aco_thread_init(
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn() -> ()>,
        >(0 as *mut libc::c_void),
    );
    tmp___1 = aco_create(
        0 as *mut libc::c_void as *mut aco_t,
        0 as *mut libc::c_void as *mut aco_share_stack_t,
        0 as libc::c_int as size_t,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn() -> ()>,
        >(0 as *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    main_co = tmp___1;
    tmp___2 = (main_co as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___2 == 0 {
        abort();
    }
    tmp___3 = aco_share_stack_new(0 as libc::c_int as size_t);
    sstk = tmp___3;
    tmp___4 = (sstk as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___4 == 0 {
        abort();
    }
    tmp___5 = aco_share_stack_new(0 as libc::c_int as size_t);
    sstk2 = tmp___5;
    tmp___6 = (sstk2 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___6 == 0 {
        abort();
    }
    co_ct_arg_point_to_me = 0 as libc::c_int;
    co2_ct_arg_point_to_me = 0 as libc::c_int;
    co3_ct_arg_point_to_me = 0 as libc::c_int;
    tmp___7 = aco_create(
        main_co,
        sstk,
        0 as libc::c_int as size_t,
        Some(co_fp0 as unsafe extern "C" fn() -> ()),
        &mut co_ct_arg_point_to_me as *mut libc::c_int as *mut libc::c_void,
    );
    co = tmp___7;
    tmp___8 = (co as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___8 == 0 {
        abort();
    }
    tmp___9 = aco_create(
        main_co,
        sstk2,
        0 as libc::c_int as size_t,
        Some(co_fp0 as unsafe extern "C" fn() -> ()),
        &mut co2_ct_arg_point_to_me as *mut libc::c_int as *mut libc::c_void,
    );
    co2 = tmp___9;
    tmp___10 = aco_create(
        main_co,
        sstk2,
        0 as libc::c_int as size_t,
        Some(co_fp0 as unsafe extern "C" fn() -> ()),
        &mut co3_ct_arg_point_to_me as *mut libc::c_int as *mut libc::c_void,
    );
    co3 = tmp___10;
    tmp___11 = (co2 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___11 == 0 {
        abort();
    }
    tmp___12 = (co3 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as libc::c_long;
    if tmp___12 == 0 {
        abort();
    }
    ct = 0 as libc::c_int;
    while ct < 6 as libc::c_int {
        tmp___13 = ((*co).is_end as libc::c_int == 0 as libc::c_int) as libc::c_int
            as libc::c_long;
        if tmp___13 == 0 {
            abort();
        }
        aco_resume(co);
        tmp___14 = (co_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long;
        if tmp___14 == 0 {
            abort();
        }
        tmp___15 = ((*co2).is_end as libc::c_int == 0 as libc::c_int) as libc::c_int
            as libc::c_long;
        if tmp___15 == 0 {
            abort();
        }
        aco_resume(co2);
        tmp___16 = (co2_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long;
        if tmp___16 == 0 {
            abort();
        }
        tmp___17 = ((*co3).is_end as libc::c_int == 0 as libc::c_int) as libc::c_int
            as libc::c_long;
        if tmp___17 == 0 {
            abort();
        }
        aco_resume(co3);
        tmp___18 = (co3_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long;
        if tmp___18 == 0 {
            abort();
        }
        printf(b"main_co:%p\n\0" as *const u8 as *const libc::c_char, main_co);
        ct += 1;
    }
    aco_resume(co);
    tmp___19 = (co_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long;
    if tmp___19 == 0 {
        abort();
    }
    tmp___20 = ((*co).is_end != 0) as libc::c_int as libc::c_long;
    if tmp___20 == 0 {
        abort();
    }
    aco_resume(co2);
    tmp___21 = (co2_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long;
    if tmp___21 == 0 {
        abort();
    }
    tmp___22 = ((*co2).is_end != 0) as libc::c_int as libc::c_long;
    if tmp___22 == 0 {
        abort();
    }
    aco_resume(co3);
    tmp___23 = (co3_ct_arg_point_to_me == ct) as libc::c_int as libc::c_long;
    if tmp___23 == 0 {
        abort();
    }
    tmp___24 = ((*co3).is_end != 0) as libc::c_int as libc::c_long;
    if tmp___24 == 0 {
        abort();
    }
    printf(b"main_co:%p\n\0" as *const u8 as *const libc::c_char, main_co);
    printf(
        b"\ncopy-stack co:%p:\n    max stack copy size:%zu\n    save (from share stack to save stack) counter of the private save stack:%zu\n    restore (from save stack to share stack) counter of the private save stack:%zu\n\0"
            as *const u8 as *const libc::c_char,
        co,
        (*co).save_stack.max_cpsz,
        (*co).save_stack.ct_save,
        (*co).save_stack.ct_restore,
    );
    printf(
        b"\n(Since the share stack used by the co has only one user `co`, so there is no need to save/restore the stack every time during resume & yield execution, thus you can call it a co has 'standalone stack' which just is a very special case of copy-stack.)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"\ncopy-stack co2:%p:\n    max stack copy size:%zu\n    save (from share stack to save stack) counter of the private save stack:%zu\n    restore (from save stack to share stack) counter of the private save stack:%zu\n\0"
            as *const u8 as *const libc::c_char,
        co2,
        (*co2).save_stack.max_cpsz,
        (*co2).save_stack.ct_save,
        (*co2).save_stack.ct_restore,
    );
    printf(
        b"\ncopy-stack co3:%p:\n    max stack copy size:%zu\n    save (from share stack to save stack) counter of the private save stack:%zu\n    restore (from save stack to share stack) counter of the private save stack:%zu\n\0"
            as *const u8 as *const libc::c_char,
        co3,
        (*co3).save_stack.max_cpsz,
        (*co3).save_stack.ct_save,
        (*co3).save_stack.ct_restore,
    );
    printf(
        b"\n(The co2 & co3 share the share stack sstk2, thus it is necessary to save/restore the stack every time during resume & yield execution, thus it is a ordinary case of copy-stack.)\n\0"
            as *const u8 as *const libc::c_char,
    );
    pthread_mutex_lock(&mut gl_race_aco_yield_ct_mutex);
    tmp_gl_ct = gl_race_aco_yield_ct;
    pthread_mutex_unlock(&mut gl_race_aco_yield_ct_mutex);
    printf(
        b"\ngl_race_aco_yield_ct:%lu\n\0" as *const u8 as *const libc::c_char,
        tmp_gl_ct,
    );
    aco_destroy(co);
    co = 0 as *mut libc::c_void as *mut aco_t;
    aco_destroy(co2);
    co2 = 0 as *mut libc::c_void as *mut aco_t;
    aco_destroy(co3);
    co3 = 0 as *mut libc::c_void as *mut aco_t;
    aco_share_stack_destroy(sstk);
    sstk = 0 as *mut libc::c_void as *mut aco_share_stack_t;
    aco_share_stack_destroy(sstk2);
    sstk2 = 0 as *mut libc::c_void as *mut aco_share_stack_t;
    aco_destroy(main_co);
    main_co = 0 as *mut libc::c_void as *mut aco_t;
    return 0 as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    let mut t1: pthread_t = 0;
    let mut t2: pthread_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_long = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_long = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_long = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_long = 0;
    tmp = pthread_create(
        &mut t1 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(pmain as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    if 0 as libc::c_int == tmp {
        tmp___0 = 1 as libc::c_int;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    tmp___1 = tmp___0 as libc::c_long;
    if tmp___1 == 0 {
        abort();
    }
    tmp___2 = pthread_create(
        &mut t2 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(pmain as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        0 as *mut libc::c_void,
    );
    if 0 as libc::c_int == tmp___2 {
        tmp___3 = 1 as libc::c_int;
    } else {
        tmp___3 = 0 as libc::c_int;
    }
    tmp___4 = tmp___3 as libc::c_long;
    if tmp___4 == 0 {
        abort();
    }
    tmp___5 = pthread_join(t1, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    if 0 as libc::c_int == tmp___5 {
        tmp___6 = 1 as libc::c_int;
    } else {
        tmp___6 = 0 as libc::c_int;
    }
    tmp___7 = tmp___6 as libc::c_long;
    if tmp___7 == 0 {
        abort();
    }
    tmp___8 = pthread_join(t2, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    if 0 as libc::c_int == tmp___8 {
        tmp___9 = 1 as libc::c_int;
    } else {
        tmp___9 = 0 as libc::c_int;
    }
    tmp___10 = tmp___9 as libc::c_long;
    if tmp___10 == 0 {
        abort();
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
