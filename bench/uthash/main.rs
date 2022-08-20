use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> libc::c_int;
    fn pthread_rwlock_destroy(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
    fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_rwlockattr_t_145707745 {
    pub __size: [libc::c_char; 8],
    pub __align: libc::c_long,
}
pub type pthread_rwlockattr_t = __anonunion_pthread_rwlockattr_t_145707745;
pub type ptrdiff_t = libc::c_long;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut libc::c_void,
    pub next: *mut libc::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *const libc::c_void,
    pub keylen: libc::c_uint,
    pub hashv: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_table {
    pub buckets: *mut UT_hash_bucket,
    pub num_buckets: libc::c_uint,
    pub log2_num_buckets: libc::c_uint,
    pub num_items: libc::c_uint,
    pub tail: *mut UT_hash_handle,
    pub hho: ptrdiff_t,
    pub ideal_chain_maxlen: libc::c_uint,
    pub nonideal_items: libc::c_uint,
    pub ineff_expands: libc::c_uint,
    pub noexpand: libc::c_uint,
    pub signature: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_bucket {
    pub hh_head: *mut UT_hash_handle,
    pub count: libc::c_uint,
    pub expand_mult: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_elt_521126190 {
    pub i: libc::c_int,
    pub hh: UT_hash_handle,
}
pub type elt = __anonstruct_elt_521126190;
pub static mut elts: *mut elt = 0 as *const libc::c_void as *mut libc::c_void
    as *mut elt;
pub static mut lock: pthread_rwlock_t = __anonunion_pthread_rwlock_t_656928968 {
    __data: __pthread_rwlock_arch_t {
        __readers: 0,
        __writers: 0,
        __wrphase_futex: 0,
        __writers_futex: 0,
        __pad3: 0,
        __pad4: 0,
        __cur_writer: 0,
        __shared: 0,
        __rwelision: 0,
        __pad1: [0; 7],
        __pad2: 0,
        __flags: 0,
    },
};
pub unsafe extern "C" fn thread_routine_r(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut num_found: libc::c_long = 0;
    let mut e: *mut elt = 0 as *mut elt;
    let mut tmp: libc::c_int = 0;
    let mut _hf_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut _hf_bkt: libc::c_uint = 0;
    let mut tmp___0: libc::c_int = 0;
    num_found = 0 as libc::c_long;
    i = 0 as libc::c_int;
    while i < 100000 as libc::c_int {
        tmp = pthread_rwlock_rdlock(&mut lock);
        if tmp != 0 as libc::c_int {
            fprintf(
                stderr,
                b"can't acquire read lock\n\0" as *const u8 as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
        e = 0 as *mut libc::c_void as *mut elt;
        if !elts.is_null() {
            _hj_key = &mut i as *mut libc::c_int as *const libc::c_uchar;
            _hf_hashv = 4276993775 as libc::c_uint;
            _hj_j = 2654435769 as libc::c_uint;
            _hj_i = _hj_j;
            _hj_k = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                as libc::c_uint;
            while _hj_k >= 12 as libc::c_uint {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                    << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                    << 24 as libc::c_int,
                            ),
                    );
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                    << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                    << 24 as libc::c_int,
                            ),
                    );
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(11 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 13 as libc::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 8 as libc::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 13 as libc::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 12 as libc::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 16 as libc::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 5 as libc::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 3 as libc::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 10 as libc::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 15 as libc::c_int;
                _hj_key = _hj_key.offset(12 as libc::c_int as isize);
                _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
            }
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
                );
            let mut current_block_60: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        );
                    current_block_60 = 3026868095061610054;
                }
                10 => {
                    current_block_60 = 3026868095061610054;
                }
                9 => {
                    current_block_60 = 8895861603392283775;
                }
                8 => {
                    current_block_60 = 13531606622793134300;
                }
                7 => {
                    current_block_60 = 6745904150434730236;
                }
                6 => {
                    current_block_60 = 9029838380463007194;
                }
                5 => {
                    current_block_60 = 11367868399951679202;
                }
                4 => {
                    current_block_60 = 13452014249558490425;
                }
                3 => {
                    current_block_60 = 2814346528284509618;
                }
                2 => {
                    current_block_60 = 12682046021601810017;
                }
                1 => {
                    current_block_60 = 13509398688507827998;
                }
                _ => {
                    current_block_60 = 13826291924415791078;
                }
            }
            match current_block_60 {
                3026868095061610054 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        );
                    current_block_60 = 8895861603392283775;
                }
                _ => {}
            }
            match current_block_60 {
                8895861603392283775 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        );
                    current_block_60 = 13531606622793134300;
                }
                _ => {}
            }
            match current_block_60 {
                13531606622793134300 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        );
                    current_block_60 = 6745904150434730236;
                }
                _ => {}
            }
            match current_block_60 {
                6745904150434730236 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        );
                    current_block_60 = 9029838380463007194;
                }
                _ => {}
            }
            match current_block_60 {
                9029838380463007194 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        );
                    current_block_60 = 11367868399951679202;
                }
                _ => {}
            }
            match current_block_60 {
                11367868399951679202 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                        );
                    current_block_60 = 13452014249558490425;
                }
                _ => {}
            }
            match current_block_60 {
                13452014249558490425 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        );
                    current_block_60 = 2814346528284509618;
                }
                _ => {}
            }
            match current_block_60 {
                2814346528284509618 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        );
                    current_block_60 = 12682046021601810017;
                }
                _ => {}
            }
            match current_block_60 {
                12682046021601810017 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        );
                    current_block_60 = 13509398688507827998;
                }
                _ => {}
            }
            match current_block_60 {
                13509398688507827998 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                        );
                }
                _ => {}
            }
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as libc::c_int;
            e = 0 as *mut libc::c_void as *mut elt;
            if !elts.is_null() {
                _hf_bkt = _hf_hashv
                    & ((*(*elts).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                if (*((*(*elts).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head
                    as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
                {
                    e = ((*((*(*elts).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head
                        as *mut libc::c_char)
                        .offset(-((*(*elts).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut elt;
                } else {
                    e = 0 as *mut libc::c_void as *mut elt;
                }
                while e as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                    if (*e).hh.hashv == _hf_hashv {
                        if (*e).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        {
                            tmp___0 = memcmp(
                                (*e).hh.key,
                                &mut i as *mut libc::c_int as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            );
                            if tmp___0 == 0 as libc::c_int {
                                break;
                            }
                        }
                    }
                    if (*e).hh.hh_next as libc::c_ulong
                        != 0 as *mut libc::c_void as libc::c_ulong
                    {
                        e = ((*e).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*elts).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut elt;
                    } else {
                        e = 0 as *mut libc::c_void as *mut elt;
                    }
                }
            }
        }
        if !e.is_null() {
            num_found += 1;
        }
        pthread_rwlock_unlock(&mut lock);
        i += 1;
    }
    return num_found as *mut libc::c_void;
}
pub unsafe extern "C" fn thread_routine_w(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut num_deld: libc::c_long = 0;
    let mut e: *mut elt = 0 as *mut elt;
    let mut tmp: libc::c_int = 0;
    let mut _hf_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut _hf_bkt: libc::c_uint = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _ha_hashv: libc::c_uint = 0;
    let mut _hj_i___0: libc::c_uint = 0;
    let mut _hj_j___0: libc::c_uint = 0;
    let mut _hj_k___0: libc::c_uint = 0;
    let mut _hj_key___0: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _ha_bkt: libc::c_uint = 0;
    let mut _ha_head: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
    let mut _he_bkt: libc::c_uint = 0;
    let mut _he_bkt_i: libc::c_uint = 0;
    let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
    let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: libc::c_uint = 0;
    num_deld = 0 as libc::c_long;
    i = 0 as libc::c_int;
    while i < 100000 as libc::c_int {
        tmp = pthread_rwlock_wrlock(&mut lock);
        if tmp != 0 as libc::c_int {
            fprintf(
                stderr,
                b"can't acquire write lock\n\0" as *const u8 as *const libc::c_char,
            );
            exit(-(1 as libc::c_int));
        }
        e = 0 as *mut libc::c_void as *mut elt;
        if !elts.is_null() {
            _hj_key = &mut i as *mut libc::c_int as *const libc::c_uchar;
            _hf_hashv = 4276993775 as libc::c_uint;
            _hj_j = 2654435769 as libc::c_uint;
            _hj_i = _hj_j;
            _hj_k = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                as libc::c_uint;
            while _hj_k >= 12 as libc::c_uint {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                    << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                    << 24 as libc::c_int,
                            ),
                    );
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                    << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                    << 24 as libc::c_int,
                            ),
                    );
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                    << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key.offset(11 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 13 as libc::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 8 as libc::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 13 as libc::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 12 as libc::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 16 as libc::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 5 as libc::c_int;
                _hj_i = _hj_i.wrapping_sub(_hj_j);
                _hj_i = _hj_i.wrapping_sub(_hf_hashv);
                _hj_i ^= _hf_hashv >> 3 as libc::c_int;
                _hj_j = _hj_j.wrapping_sub(_hf_hashv);
                _hj_j = _hj_j.wrapping_sub(_hj_i);
                _hj_j ^= _hj_i << 10 as libc::c_int;
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
                _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
                _hf_hashv ^= _hj_j >> 15 as libc::c_int;
                _hj_key = _hj_key.offset(12 as libc::c_int as isize);
                _hj_k = _hj_k.wrapping_sub(12 as libc::c_uint);
            }
            _hf_hashv = _hf_hashv
                .wrapping_add(
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
                );
            let mut current_block_60: u64;
            match _hj_k {
                11 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        );
                    current_block_60 = 14936044374265219789;
                }
                10 => {
                    current_block_60 = 14936044374265219789;
                }
                9 => {
                    current_block_60 = 3904386449872907327;
                }
                8 => {
                    current_block_60 = 3673019761254966263;
                }
                7 => {
                    current_block_60 = 16620298045565028098;
                }
                6 => {
                    current_block_60 = 9476869031680062646;
                }
                5 => {
                    current_block_60 = 6226537248067767992;
                }
                4 => {
                    current_block_60 = 10631816851787393565;
                }
                3 => {
                    current_block_60 = 15155215915847730705;
                }
                2 => {
                    current_block_60 = 10456996632932790566;
                }
                1 => {
                    current_block_60 = 5722737199739957318;
                }
                _ => {
                    current_block_60 = 13303144130133872306;
                }
            }
            match current_block_60 {
                14936044374265219789 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        );
                    current_block_60 = 3904386449872907327;
                }
                _ => {}
            }
            match current_block_60 {
                3904386449872907327 => {
                    _hf_hashv = _hf_hashv
                        .wrapping_add(
                            (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        );
                    current_block_60 = 3673019761254966263;
                }
                _ => {}
            }
            match current_block_60 {
                3673019761254966263 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        );
                    current_block_60 = 16620298045565028098;
                }
                _ => {}
            }
            match current_block_60 {
                16620298045565028098 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        );
                    current_block_60 = 9476869031680062646;
                }
                _ => {}
            }
            match current_block_60 {
                9476869031680062646 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        );
                    current_block_60 = 6226537248067767992;
                }
                _ => {}
            }
            match current_block_60 {
                6226537248067767992 => {
                    _hj_j = _hj_j
                        .wrapping_add(
                            *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                        );
                    current_block_60 = 10631816851787393565;
                }
                _ => {}
            }
            match current_block_60 {
                10631816851787393565 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
                        );
                    current_block_60 = 15155215915847730705;
                }
                _ => {}
            }
            match current_block_60 {
                15155215915847730705 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        );
                    current_block_60 = 10456996632932790566;
                }
                _ => {}
            }
            match current_block_60 {
                10456996632932790566 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                                << 8 as libc::c_int,
                        );
                    current_block_60 = 5722737199739957318;
                }
                _ => {}
            }
            match current_block_60 {
                5722737199739957318 => {
                    _hj_i = _hj_i
                        .wrapping_add(
                            *_hj_key.offset(0 as libc::c_int as isize) as libc::c_uint,
                        );
                }
                _ => {}
            }
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 13 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 8 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 13 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 12 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 16 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 5 as libc::c_int;
            _hj_i = _hj_i.wrapping_sub(_hj_j);
            _hj_i = _hj_i.wrapping_sub(_hf_hashv);
            _hj_i ^= _hf_hashv >> 3 as libc::c_int;
            _hj_j = _hj_j.wrapping_sub(_hf_hashv);
            _hj_j = _hj_j.wrapping_sub(_hj_i);
            _hj_j ^= _hj_i << 10 as libc::c_int;
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_i);
            _hf_hashv = _hf_hashv.wrapping_sub(_hj_j);
            _hf_hashv ^= _hj_j >> 15 as libc::c_int;
            e = 0 as *mut libc::c_void as *mut elt;
            if !elts.is_null() {
                _hf_bkt = _hf_hashv
                    & ((*(*elts).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
                if (*((*(*elts).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head
                    as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
                {
                    e = ((*((*(*elts).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head
                        as *mut libc::c_char)
                        .offset(-((*(*elts).hh.tbl).hho as isize)) as *mut libc::c_void
                        as *mut elt;
                } else {
                    e = 0 as *mut libc::c_void as *mut elt;
                }
                while e as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                    if (*e).hh.hashv == _hf_hashv {
                        if (*e).hh.keylen as libc::c_ulong
                            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        {
                            tmp___0 = memcmp(
                                (*e).hh.key,
                                &mut i as *mut libc::c_int as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            );
                            if tmp___0 == 0 as libc::c_int {
                                break;
                            }
                        }
                    }
                    if (*e).hh.hh_next as libc::c_ulong
                        != 0 as *mut libc::c_void as libc::c_ulong
                    {
                        e = ((*e).hh.hh_next as *mut libc::c_char)
                            .offset(-((*(*elts).hh.tbl).hho as isize))
                            as *mut libc::c_void as *mut elt;
                    } else {
                        e = 0 as *mut libc::c_void as *mut elt;
                    }
                }
            }
        }
        if e.is_null() {
            tmp___1 = malloc(::std::mem::size_of::<elt>() as libc::c_ulong);
            e = tmp___1 as *mut elt;
            if e.is_null() {
                exit(-(1 as libc::c_int));
            }
            (*e).i = i;
            _hj_key___0 = &mut (*e).i as *mut libc::c_int as *const libc::c_uchar;
            _ha_hashv = 4276993775 as libc::c_uint;
            _hj_j___0 = 2654435769 as libc::c_uint;
            _hj_i___0 = _hj_j___0;
            _hj_k___0 = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                as libc::c_uint;
            while _hj_k___0 >= 12 as libc::c_uint {
                _hj_i___0 = _hj_i___0
                    .wrapping_add(
                        (*_hj_key___0.offset(0 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key___0.offset(1 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key___0.offset(2 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key___0.offset(3 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hj_j___0 = _hj_j___0
                    .wrapping_add(
                        (*_hj_key___0.offset(4 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key___0.offset(5 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key___0.offset(6 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key___0.offset(7 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _ha_hashv = _ha_hashv
                    .wrapping_add(
                        (*_hj_key___0.offset(8 as libc::c_int as isize) as libc::c_uint)
                            .wrapping_add(
                                (*_hj_key___0.offset(9 as libc::c_int as isize)
                                    as libc::c_uint) << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key___0.offset(10 as libc::c_int as isize)
                                    as libc::c_uint) << 16 as libc::c_int,
                            )
                            .wrapping_add(
                                (*_hj_key___0.offset(11 as libc::c_int as isize)
                                    as libc::c_uint) << 24 as libc::c_int,
                            ),
                    );
                _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
                _hj_i___0 = _hj_i___0.wrapping_sub(_ha_hashv);
                _hj_i___0 ^= _ha_hashv >> 13 as libc::c_int;
                _hj_j___0 = _hj_j___0.wrapping_sub(_ha_hashv);
                _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
                _hj_j___0 ^= _hj_i___0 << 8 as libc::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i___0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j___0);
                _ha_hashv ^= _hj_j___0 >> 13 as libc::c_int;
                _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
                _hj_i___0 = _hj_i___0.wrapping_sub(_ha_hashv);
                _hj_i___0 ^= _ha_hashv >> 12 as libc::c_int;
                _hj_j___0 = _hj_j___0.wrapping_sub(_ha_hashv);
                _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
                _hj_j___0 ^= _hj_i___0 << 16 as libc::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i___0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j___0);
                _ha_hashv ^= _hj_j___0 >> 5 as libc::c_int;
                _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
                _hj_i___0 = _hj_i___0.wrapping_sub(_ha_hashv);
                _hj_i___0 ^= _ha_hashv >> 3 as libc::c_int;
                _hj_j___0 = _hj_j___0.wrapping_sub(_ha_hashv);
                _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
                _hj_j___0 ^= _hj_i___0 << 10 as libc::c_int;
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_i___0);
                _ha_hashv = _ha_hashv.wrapping_sub(_hj_j___0);
                _ha_hashv ^= _hj_j___0 >> 15 as libc::c_int;
                _hj_key___0 = _hj_key___0.offset(12 as libc::c_int as isize);
                _hj_k___0 = _hj_k___0.wrapping_sub(12 as libc::c_uint);
            }
            _ha_hashv = _ha_hashv
                .wrapping_add(
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as libc::c_uint,
                );
            let mut current_block_174: u64;
            match _hj_k___0 {
                11 => {
                    _ha_hashv = _ha_hashv
                        .wrapping_add(
                            (*_hj_key___0.offset(10 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_174 = 13287524705005776359;
                }
                10 => {
                    current_block_174 = 13287524705005776359;
                }
                9 => {
                    current_block_174 = 7120397033281741945;
                }
                8 => {
                    current_block_174 = 5250614332951224594;
                }
                7 => {
                    current_block_174 = 5245318971642783885;
                }
                6 => {
                    current_block_174 = 13102332982549100014;
                }
                5 => {
                    current_block_174 = 14141739721812636285;
                }
                4 => {
                    current_block_174 = 9322039630446129790;
                }
                3 => {
                    current_block_174 = 7662403615634336687;
                }
                2 => {
                    current_block_174 = 1409218803810080683;
                }
                1 => {
                    current_block_174 = 18364361674211581114;
                }
                _ => {
                    current_block_174 = 18323319510560553714;
                }
            }
            match current_block_174 {
                13287524705005776359 => {
                    _ha_hashv = _ha_hashv
                        .wrapping_add(
                            (*_hj_key___0.offset(9 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_174 = 7120397033281741945;
                }
                _ => {}
            }
            match current_block_174 {
                7120397033281741945 => {
                    _ha_hashv = _ha_hashv
                        .wrapping_add(
                            (*_hj_key___0.offset(8 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_174 = 5250614332951224594;
                }
                _ => {}
            }
            match current_block_174 {
                5250614332951224594 => {
                    _hj_j___0 = _hj_j___0
                        .wrapping_add(
                            (*_hj_key___0.offset(7 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_174 = 5245318971642783885;
                }
                _ => {}
            }
            match current_block_174 {
                5245318971642783885 => {
                    _hj_j___0 = _hj_j___0
                        .wrapping_add(
                            (*_hj_key___0.offset(6 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_174 = 13102332982549100014;
                }
                _ => {}
            }
            match current_block_174 {
                13102332982549100014 => {
                    _hj_j___0 = _hj_j___0
                        .wrapping_add(
                            (*_hj_key___0.offset(5 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_174 = 14141739721812636285;
                }
                _ => {}
            }
            match current_block_174 {
                14141739721812636285 => {
                    _hj_j___0 = _hj_j___0
                        .wrapping_add(
                            *_hj_key___0.offset(4 as libc::c_int as isize)
                                as libc::c_uint,
                        );
                    current_block_174 = 9322039630446129790;
                }
                _ => {}
            }
            match current_block_174 {
                9322039630446129790 => {
                    _hj_i___0 = _hj_i___0
                        .wrapping_add(
                            (*_hj_key___0.offset(3 as libc::c_int as isize)
                                as libc::c_uint) << 24 as libc::c_int,
                        );
                    current_block_174 = 7662403615634336687;
                }
                _ => {}
            }
            match current_block_174 {
                7662403615634336687 => {
                    _hj_i___0 = _hj_i___0
                        .wrapping_add(
                            (*_hj_key___0.offset(2 as libc::c_int as isize)
                                as libc::c_uint) << 16 as libc::c_int,
                        );
                    current_block_174 = 1409218803810080683;
                }
                _ => {}
            }
            match current_block_174 {
                1409218803810080683 => {
                    _hj_i___0 = _hj_i___0
                        .wrapping_add(
                            (*_hj_key___0.offset(1 as libc::c_int as isize)
                                as libc::c_uint) << 8 as libc::c_int,
                        );
                    current_block_174 = 18364361674211581114;
                }
                _ => {}
            }
            match current_block_174 {
                18364361674211581114 => {
                    _hj_i___0 = _hj_i___0
                        .wrapping_add(
                            *_hj_key___0.offset(0 as libc::c_int as isize)
                                as libc::c_uint,
                        );
                }
                _ => {}
            }
            _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
            _hj_i___0 = _hj_i___0.wrapping_sub(_ha_hashv);
            _hj_i___0 ^= _ha_hashv >> 13 as libc::c_int;
            _hj_j___0 = _hj_j___0.wrapping_sub(_ha_hashv);
            _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
            _hj_j___0 ^= _hj_i___0 << 8 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i___0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j___0);
            _ha_hashv ^= _hj_j___0 >> 13 as libc::c_int;
            _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
            _hj_i___0 = _hj_i___0.wrapping_sub(_ha_hashv);
            _hj_i___0 ^= _ha_hashv >> 12 as libc::c_int;
            _hj_j___0 = _hj_j___0.wrapping_sub(_ha_hashv);
            _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
            _hj_j___0 ^= _hj_i___0 << 16 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i___0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j___0);
            _ha_hashv ^= _hj_j___0 >> 5 as libc::c_int;
            _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
            _hj_i___0 = _hj_i___0.wrapping_sub(_ha_hashv);
            _hj_i___0 ^= _ha_hashv >> 3 as libc::c_int;
            _hj_j___0 = _hj_j___0.wrapping_sub(_ha_hashv);
            _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
            _hj_j___0 ^= _hj_i___0 << 10 as libc::c_int;
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_i___0);
            _ha_hashv = _ha_hashv.wrapping_sub(_hj_j___0);
            _ha_hashv ^= _hj_j___0 >> 15 as libc::c_int;
            (*e).hh.hashv = _ha_hashv;
            (*e).hh.key = &mut (*e).i as *mut libc::c_int as *const libc::c_void;
            (*e)
                .hh
                .keylen = ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                as libc::c_uint;
            if elts.is_null() {
                (*e).hh.next = 0 as *mut libc::c_void;
                (*e).hh.prev = 0 as *mut libc::c_void;
                tmp___2 = malloc(
                    ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                );
                (*e).hh.tbl = tmp___2 as *mut UT_hash_table;
                if ((*e).hh.tbl).is_null() {
                    exit(-(1 as libc::c_int));
                } else {
                    memset(
                        (*e).hh.tbl as *mut libc::c_void,
                        '\u{0}' as i32,
                        ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
                    );
                    (*(*e).hh.tbl).tail = &mut (*e).hh;
                    (*(*e).hh.tbl).num_buckets = 32 as libc::c_uint;
                    (*(*e).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
                    (*(*e).hh.tbl)
                        .hho = (&mut (*e).hh as *mut UT_hash_handle as *mut libc::c_char)
                        .offset_from(e as *mut libc::c_char) as libc::c_long;
                    tmp___3 = malloc(
                        (32 as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                            ),
                    );
                    (*(*e).hh.tbl).buckets = tmp___3 as *mut UT_hash_bucket;
                    (*(*e).hh.tbl).signature = 2685476833 as libc::c_uint;
                    if ((*(*e).hh.tbl).buckets).is_null() {
                        exit(-(1 as libc::c_int));
                    } else {
                        memset(
                            (*(*e).hh.tbl).buckets as *mut libc::c_void,
                            '\u{0}' as i32,
                            (32 as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                                ),
                        );
                    }
                }
                elts = e;
            } else {
                (*e).hh.tbl = (*elts).hh.tbl;
                (*e).hh.next = 0 as *mut libc::c_void;
                (*e)
                    .hh
                    .prev = ((*(*elts).hh.tbl).tail as *mut libc::c_char)
                    .offset(-((*(*elts).hh.tbl).hho as isize)) as *mut libc::c_void;
                (*(*(*elts).hh.tbl).tail).next = e as *mut libc::c_void;
                (*(*elts).hh.tbl).tail = &mut (*e).hh;
            }
            (*(*elts).hh.tbl).num_items = ((*(*elts).hh.tbl).num_items).wrapping_add(1);
            _ha_bkt = _ha_hashv
                & ((*(*elts).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            _ha_head = ((*(*elts).hh.tbl).buckets).offset(_ha_bkt as isize);
            (*_ha_head).count = ((*_ha_head).count).wrapping_add(1);
            (*e).hh.hh_next = (*_ha_head).hh_head;
            (*e).hh.hh_prev = 0 as *mut libc::c_void as *mut UT_hash_handle;
            if (*_ha_head).hh_head as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong
            {
                (*(*_ha_head).hh_head).hh_prev = &mut (*e).hh;
            }
            (*_ha_head).hh_head = &mut (*e).hh;
            if (*_ha_head).count
                >= ((*_ha_head).expand_mult)
                    .wrapping_add(1 as libc::c_uint)
                    .wrapping_mul(10 as libc::c_uint)
            {
                if (*(*e).hh.tbl).noexpand == 0 {
                    tmp___4 = malloc(
                        (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                            .wrapping_mul((*(*e).hh.tbl).num_buckets as libc::c_ulong)
                            .wrapping_mul(2 as libc::c_ulong),
                    );
                    _he_new_buckets = tmp___4 as *mut UT_hash_bucket;
                    if _he_new_buckets.is_null() {
                        exit(-(1 as libc::c_int));
                    } else {
                        memset(
                            _he_new_buckets as *mut libc::c_void,
                            '\u{0}' as i32,
                            (::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong)
                                .wrapping_mul((*(*e).hh.tbl).num_buckets as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_ulong),
                        );
                        if (*(*e).hh.tbl).num_items
                            & ((*(*e).hh.tbl).num_buckets)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_sub(1 as libc::c_uint) != 0 as libc::c_uint
                        {
                            tmp___5 = 1 as libc::c_uint;
                        } else {
                            tmp___5 = 0 as libc::c_uint;
                        }
                        (*(*e).hh.tbl)
                            .ideal_chain_maxlen = ((*(*e).hh.tbl).num_items
                            >> ((*(*e).hh.tbl).log2_num_buckets)
                                .wrapping_add(1 as libc::c_uint))
                            .wrapping_add(tmp___5);
                        (*(*e).hh.tbl).nonideal_items = 0 as libc::c_uint;
                        _he_bkt_i = 0 as libc::c_uint;
                        while _he_bkt_i < (*(*e).hh.tbl).num_buckets {
                            _he_thh = (*((*(*e).hh.tbl).buckets)
                                .offset(_he_bkt_i as isize))
                                .hh_head;
                            while _he_thh as libc::c_ulong
                                != 0 as *mut libc::c_void as libc::c_ulong
                            {
                                _he_hh_nxt = (*_he_thh).hh_next;
                                _he_bkt = (*_he_thh).hashv
                                    & ((*(*e).hh.tbl).num_buckets)
                                        .wrapping_mul(2 as libc::c_uint)
                                        .wrapping_sub(1 as libc::c_uint);
                                _he_newbkt = _he_new_buckets.offset(_he_bkt as isize);
                                (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                                if (*_he_newbkt).count > (*(*e).hh.tbl).ideal_chain_maxlen {
                                    (*(*e).hh.tbl)
                                        .nonideal_items = ((*(*e).hh.tbl).nonideal_items)
                                        .wrapping_add(1);
                                    if (*_he_newbkt).count
                                        > ((*_he_newbkt).expand_mult)
                                            .wrapping_mul((*(*e).hh.tbl).ideal_chain_maxlen)
                                    {
                                        (*_he_newbkt)
                                            .expand_mult = ((*_he_newbkt).expand_mult).wrapping_add(1);
                                    }
                                }
                                (*_he_thh)
                                    .hh_prev = 0 as *mut libc::c_void as *mut UT_hash_handle;
                                (*_he_thh).hh_next = (*_he_newbkt).hh_head;
                                if (*_he_newbkt).hh_head as libc::c_ulong
                                    != 0 as *mut libc::c_void as libc::c_ulong
                                {
                                    (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                                }
                                (*_he_newbkt).hh_head = _he_thh;
                                _he_thh = _he_hh_nxt;
                            }
                            _he_bkt_i = _he_bkt_i.wrapping_add(1);
                        }
                        free((*(*e).hh.tbl).buckets as *mut libc::c_void);
                        (*(*e).hh.tbl)
                            .num_buckets = ((*(*e).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint);
                        (*(*e).hh.tbl)
                            .log2_num_buckets = ((*(*e).hh.tbl).log2_num_buckets)
                            .wrapping_add(1);
                        (*(*e).hh.tbl).buckets = _he_new_buckets;
                        if (*(*e).hh.tbl).nonideal_items
                            > (*(*e).hh.tbl).num_items >> 1 as libc::c_int
                        {
                            (*(*e).hh.tbl)
                                .ineff_expands = ((*(*e).hh.tbl).ineff_expands)
                                .wrapping_add(1);
                        } else {
                            (*(*e).hh.tbl).ineff_expands = 0 as libc::c_uint;
                        }
                        if (*(*e).hh.tbl).ineff_expands > 1 as libc::c_uint {
                            (*(*e).hh.tbl).noexpand = 1 as libc::c_uint;
                            fprintf(
                                stderr,
                                b"warning: bucket expansion inhibited\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    }
                }
            }
        }
        pthread_rwlock_unlock(&mut lock);
        i += 1;
    }
    return num_deld as *mut libc::c_void;
}
unsafe fn main_0() -> libc::c_int {
    let mut i: libc::c_uint = 0;
    let mut num_added: libc::c_long = 0;
    let mut status: libc::c_int = 0;
    let mut thread_r1: pthread_t = 0;
    let mut thread_r2: pthread_t = 0;
    let mut thread_w1: pthread_t = 0;
    let mut thread_w2: pthread_t = 0;
    let mut thread_result: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    num_added = 0 as libc::c_long;
    tmp___0 = pthread_rwlock_init(
        &mut lock as *mut pthread_rwlock_t,
        0 as *mut libc::c_void as *const pthread_rwlockattr_t,
    );
    if tmp___0 != 0 as libc::c_int {
        fprintf(stderr, b"lock init failed\n\0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    status = pthread_create(
        &mut thread_r1 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            thread_routine_r
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    if status != 0 {
        printf(b"failure: status %d\n\0" as *const u8 as *const libc::c_char, status);
        exit(-(1 as libc::c_int));
    }
    status = pthread_create(
        &mut thread_r2 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            thread_routine_r
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    if status != 0 {
        printf(b"failure: status %d\n\0" as *const u8 as *const libc::c_char, status);
        exit(-(1 as libc::c_int));
    }
    status = pthread_create(
        &mut thread_w1 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            thread_routine_w
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    if status != 0 {
        printf(b"failure: status %d\n\0" as *const u8 as *const libc::c_char, status);
        exit(-(1 as libc::c_int));
    }
    status = pthread_create(
        &mut thread_w2 as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            thread_routine_w
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    if status != 0 {
        printf(b"failure: status %d\n\0" as *const u8 as *const libc::c_char, status);
        exit(-(1 as libc::c_int));
    }
    status = pthread_join(thread_r1, &mut thread_result);
    printf(
        b"thread result: %d %ld\n\0" as *const u8 as *const libc::c_char,
        status,
        thread_result as libc::c_long,
    );
    status = pthread_join(thread_r2, &mut thread_result);
    printf(
        b"thread result: %d %ld\n\0" as *const u8 as *const libc::c_char,
        status,
        thread_result as libc::c_long,
    );
    status = pthread_join(thread_w1, &mut thread_result);
    printf(
        b"thread result: %d %ld\n\0" as *const u8 as *const libc::c_char,
        status,
        thread_result as libc::c_long,
    );
    status = pthread_join(thread_w2, &mut thread_result);
    printf(
        b"thread result: %d %ld\n\0" as *const u8 as *const libc::c_char,
        status,
        thread_result as libc::c_long,
    );
    if elts as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        i = (*(*elts).hh.tbl).num_items;
    } else {
        i = 0 as libc::c_uint;
    }
    printf(
        b"final count of items in hash: %u\n\0" as *const u8 as *const libc::c_char,
        i,
    );
    tmp___1 = pthread_rwlock_destroy(&mut lock);
    if tmp___1 != 0 as libc::c_int {
        fprintf(stderr, b"lock destroy failed\n\0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
