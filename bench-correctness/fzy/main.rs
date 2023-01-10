#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, label_break_value, register_tool, rustc_private, untagged_unions)]
use ::c2rust_out::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn clock() -> clock_t;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn asprintf(
        __ptr: *mut *mut libc::c_char,
        __fmt: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    );
    static mut stderr: *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __clock_t = libc::c_long;
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
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___sigset_t_991265788 {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = __anonstruct___sigset_t_991265788;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_suite_info {
    pub tests_run: libc::c_uint,
    pub passed: libc::c_uint,
    pub failed: libc::c_uint,
    pub skipped: libc::c_uint,
    pub pre_suite: clock_t,
    pub post_suite: clock_t,
    pub pre_test: clock_t,
    pub post_test: clock_t,
}
pub type greatest_suite_cb = unsafe extern "C" fn() -> ();
pub type greatest_setup_cb = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type greatest_teardown_cb = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type greatest_equal_cb = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
pub type greatest_printf_cb = unsafe extern "C" fn(
    *const libc::c_void,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_type_info {
    pub equal: Option::<greatest_equal_cb>,
    pub print: Option::<greatest_printf_cb>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_memory_cmp_env {
    pub exp: *const libc::c_uchar,
    pub got: *const libc::c_uchar,
    pub size: size_t,
}
pub type __anonenum_greatest_flag_t_126737729 = libc::c_uint;
pub const GREATEST_FLAG_LIST_ONLY: __anonenum_greatest_flag_t_126737729 = 2;
pub const GREATEST_FLAG_FIRST_FAIL: __anonenum_greatest_flag_t_126737729 = 1;
pub type greatest_flag_t = __anonenum_greatest_flag_t_126737729;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_prng {
    pub random_order: libc::c_uchar,
    pub initialized: libc::c_uchar,
    pub pad_0: [libc::c_uchar; 2],
    pub state: libc::c_ulong,
    pub count: libc::c_ulong,
    pub count_ceil: libc::c_ulong,
    pub count_run: libc::c_ulong,
    pub mod_0: libc::c_ulong,
    pub a: libc::c_ulong,
    pub c: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_run_info {
    pub flags: libc::c_uchar,
    pub verbosity: libc::c_uchar,
    pub pad_0: [libc::c_uchar; 2],
    pub tests_run: libc::c_uint,
    pub suite: greatest_suite_info,
    pub passed: libc::c_uint,
    pub failed: libc::c_uint,
    pub skipped: libc::c_uint,
    pub assertions: libc::c_uint,
    pub fail_line: libc::c_uint,
    pub pad_1: libc::c_uint,
    pub fail_file: *const libc::c_char,
    pub msg: *const libc::c_char,
    pub setup: Option::<greatest_setup_cb>,
    pub setup_udata: *mut libc::c_void,
    pub teardown: Option::<greatest_teardown_cb>,
    pub teardown_udata: *mut libc::c_void,
    pub col: libc::c_uint,
    pub width: libc::c_uint,
    pub suite_filter: *const libc::c_char,
    pub test_filter: *const libc::c_char,
    pub test_exclude: *const libc::c_char,
    pub prng: [greatest_prng; 2],
    pub begin: clock_t,
    pub end: clock_t,
    pub pad_jmp_buf: libc::c_int,
    pub jump_dest: jmp_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct greatest_report_t {
    pub passed: libc::c_uint,
    pub failed: libc::c_uint,
    pub skipped: libc::c_uint,
    pub assertions: libc::c_uint,
}
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type greatest_test_res = libc::c_int;
pub const GREATEST_TEST_RES_SKIP: greatest_test_res = 1;
pub const GREATEST_TEST_RES_FAIL: greatest_test_res = -1;
pub const GREATEST_TEST_RES_PASS: greatest_test_res = 0;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type theft_seed = uint64_t;
pub type theft_hash = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_bloom {
    pub bit_count: uint8_t,
    pub size: size_t,
    pub bits: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_mt {
    pub mt: [uint64_t; 312],
    pub mti: int16_t,
}
pub type int16_t = __int16_t;
pub type __int16_t = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft {
    pub out: *mut FILE,
    pub seed: theft_seed,
    pub requested_bloom_bits: uint8_t,
    pub bloom: *mut theft_bloom,
    pub mt: *mut theft_mt,
}
pub type theft_alloc_cb = unsafe extern "C" fn(
    *mut theft,
    theft_seed,
    *mut libc::c_void,
) -> *mut libc::c_void;
pub type theft_free_cb = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
pub type theft_hash_cb = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_void,
) -> theft_hash;
pub type theft_shrink_cb = unsafe extern "C" fn(
    *mut libc::c_void,
    uint32_t,
    *mut libc::c_void,
) -> *mut libc::c_void;
pub type theft_print_cb = unsafe extern "C" fn(
    *mut FILE,
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
pub type __anonenum_theft_trial_res_808224241 = libc::c_uint;
pub const THEFT_TRIAL_ERROR: __anonenum_theft_trial_res_808224241 = 4;
pub const THEFT_TRIAL_DUP: __anonenum_theft_trial_res_808224241 = 3;
pub const THEFT_TRIAL_SKIP: __anonenum_theft_trial_res_808224241 = 2;
pub const THEFT_TRIAL_FAIL: __anonenum_theft_trial_res_808224241 = 1;
pub const THEFT_TRIAL_PASS: __anonenum_theft_trial_res_808224241 = 0;
pub type theft_trial_res = __anonenum_theft_trial_res_808224241;
pub type theft_propfun = unsafe extern "C" fn() -> theft_trial_res;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_type_info {
    pub alloc: Option::<theft_alloc_cb>,
    pub free: Option::<theft_free_cb>,
    pub hash: Option::<theft_hash_cb>,
    pub shrink: Option::<theft_shrink_cb>,
    pub print: Option::<theft_print_cb>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_trial_info {
    pub name: *const libc::c_char,
    pub trial: libc::c_int,
    pub seed: theft_seed,
    pub status: theft_trial_res,
    pub arity: uint8_t,
    pub args: *mut *mut libc::c_void,
}
pub type __anonenum_theft_progress_callback_res_308568895 = libc::c_uint;
pub const THEFT_PROGRESS_HALT: __anonenum_theft_progress_callback_res_308568895 = 1;
pub const THEFT_PROGRESS_CONTINUE: __anonenum_theft_progress_callback_res_308568895 = 0;
pub type theft_progress_callback_res = __anonenum_theft_progress_callback_res_308568895;
pub type theft_progress_cb = unsafe extern "C" fn(
    *mut theft_trial_info,
    *mut libc::c_void,
) -> theft_progress_callback_res;
pub type __anonenum_theft_run_res_746447195 = libc::c_int;
pub const THEFT_RUN_ERROR_MISSING_CALLBACK: __anonenum_theft_run_res_746447195 = -2;
pub const THEFT_RUN_ERROR_BAD_ARGS: __anonenum_theft_run_res_746447195 = -1;
pub const THEFT_RUN_ERROR: __anonenum_theft_run_res_746447195 = 2;
pub const THEFT_RUN_FAIL: __anonenum_theft_run_res_746447195 = 1;
pub const THEFT_RUN_PASS: __anonenum_theft_run_res_746447195 = 0;
pub type theft_run_res = __anonenum_theft_run_res_746447195;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_trial_report {
    pub pass: size_t,
    pub fail: size_t,
    pub skip: size_t,
    pub dup: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_cfg {
    pub fun: Option::<theft_propfun>,
    pub type_info: [*mut theft_type_info; 10],
    pub name: *const libc::c_char,
    pub always_seed_count: libc::c_int,
    pub always_seeds: *mut theft_seed,
    pub trials: libc::c_int,
    pub progress_cb: Option::<theft_progress_cb>,
    pub env: *mut libc::c_void,
    pub report: *mut theft_trial_report,
    pub seed: theft_seed,
}
pub type score_t = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_options_t_734899051 {
    pub benchmark: libc::c_int,
    pub filter: *const libc::c_char,
    pub init_search: *const libc::c_char,
    pub tty_filename: *const libc::c_char,
    pub show_scores: libc::c_int,
    pub num_lines: libc::c_uint,
    pub scrolloff: libc::c_uint,
    pub prompt: *const libc::c_char,
    pub workers: libc::c_uint,
    pub input_delimiter: libc::c_char,
    pub show_info: libc::c_int,
}
pub type options_t = __anonstruct_options_t_734899051;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scored_result {
    pub score: score_t,
    pub str_0: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_choices_t_109572495 {
    pub buffer: *mut libc::c_char,
    pub buffer_size: size_t,
    pub capacity: size_t,
    pub size: size_t,
    pub strings: *mut *const libc::c_char,
    pub results: *mut scored_result,
    pub available: size_t,
    pub selection: size_t,
    pub worker_count: libc::c_uint,
}
pub type choices_t = __anonstruct_choices_t_109572495;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_struct {
    pub needle_len: libc::c_int,
    pub haystack_len: libc::c_int,
    pub lower_needle: [libc::c_char; 1024],
    pub lower_haystack: [libc::c_char; 1024],
    pub match_bonus: [score_t; 1024],
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
pub union __anonunion_pthread_mutexattr_t_488594144 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_mutexattr_t = __anonunion_pthread_mutexattr_t_488594144;
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
pub struct result_list {
    pub list: *mut scored_result,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct worker {
    pub thread_id: pthread_t,
    pub job: *mut search_job,
    pub worker_num: libc::c_uint,
    pub result: result_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct search_job {
    pub lock: pthread_mutex_t,
    pub choices: *mut choices_t,
    pub search: *const libc::c_char,
    pub processed: size_t,
    pub workers: *mut worker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __anonenum_all_gen_res_t_575372223 = libc::c_uint;
pub const ALL_GEN_ERROR: __anonenum_all_gen_res_t_575372223 = 3;
pub const ALL_GEN_DUP: __anonenum_all_gen_res_t_575372223 = 2;
pub const ALL_GEN_SKIP: __anonenum_all_gen_res_t_575372223 = 1;
pub const ALL_GEN_OK: __anonenum_all_gen_res_t_575372223 = 0;
pub type all_gen_res_t = __anonenum_all_gen_res_t_575372223;
pub type __anonenum_shrink_res_238061164 = libc::c_uint;
pub const SHRINK_ERROR: __anonenum_shrink_res_238061164 = 2;
pub const SHRINK_DEAD_END: __anonenum_shrink_res_238061164 = 1;
pub const SHRINK_OK: __anonenum_shrink_res_238061164 = 0;
pub type shrink_res = __anonenum_shrink_res_238061164;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_propfun_info {
    pub name: *const libc::c_char,
    pub fun: Option::<theft_propfun>,
    pub arity: uint8_t,
    pub type_info: [*mut theft_type_info; 10],
    pub always_seed_count: libc::c_int,
    pub always_seeds: *mut theft_seed,
}
pub type __uint16_t = libc::c_ushort;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct theft_hasher {
    pub accum: theft_hash,
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    let mut tmp: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___0: __int32_t = 0;
    if __c >= -(128 as libc::c_int) {
        if __c < 256 as libc::c_int {
            tmp = __ctype_tolower_loc();
            tmp___0 = *(*tmp).offset(__c as isize);
        } else {
            tmp___0 = __c;
        }
    } else {
        tmp___0 = __c;
    }
    return tmp___0;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    let mut tmp: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___0: __int32_t = 0;
    if __c >= -(128 as libc::c_int) {
        if __c < 256 as libc::c_int {
            tmp = __ctype_toupper_loc();
            tmp___0 = *(*tmp).offset(__c as isize);
        } else {
            tmp___0 = __c;
        }
    } else {
        tmp___0 = __c;
    }
    return tmp___0;
}
pub static mut greatest_info: greatest_run_info = greatest_run_info {
    flags: 0,
    verbosity: 0,
    pad_0: [0; 2],
    tests_run: 0,
    suite: greatest_suite_info {
        tests_run: 0,
        passed: 0,
        failed: 0,
        skipped: 0,
        pre_suite: 0,
        post_suite: 0,
        pre_test: 0,
        post_test: 0,
    },
    passed: 0,
    failed: 0,
    skipped: 0,
    assertions: 0,
    fail_line: 0,
    pad_1: 0,
    fail_file: 0 as *const libc::c_char,
    msg: 0 as *const libc::c_char,
    setup: None,
    setup_udata: 0 as *const libc::c_void as *mut libc::c_void,
    teardown: None,
    teardown_udata: 0 as *const libc::c_void as *mut libc::c_void,
    col: 0,
    width: 0,
    suite_filter: 0 as *const libc::c_char,
    test_filter: 0 as *const libc::c_char,
    test_exclude: 0 as *const libc::c_char,
    prng: [greatest_prng {
        random_order: 0,
        initialized: 0,
        pad_0: [0; 2],
        state: 0,
        count: 0,
        count_ceil: 0,
        count_run: 0,
        mod_0: 0,
        a: 0,
        c: 0,
    }; 2],
    begin: 0,
    end: 0,
    pad_jmp_buf: 0,
    jump_dest: [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1],
};
unsafe extern "C" fn greatest_name_match(
    mut name: *const libc::c_char,
    mut filter: *const libc::c_char,
    mut res_if_none: libc::c_int,
) -> libc::c_int {
    let mut offset: size_t = 0;
    let mut filter_len: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    offset = 0 as libc::c_int as size_t;
    if !filter.is_null() {
        tmp = strlen(filter);
        tmp___0 = tmp;
    } else {
        tmp___0 = 0 as libc::c_int as size_t;
    }
    filter_len = tmp___0;
    if filter_len == 0 as libc::c_ulong {
        return res_if_none;
    }
    while *name.offset(offset as isize) as libc::c_int != 0 as libc::c_int {
        if *name.offset(offset as isize) as libc::c_int
            == *filter.offset(0 as libc::c_int as isize) as libc::c_int
        {
            tmp___1 = strncmp(name.offset(offset as isize), filter, filter_len);
            if 0 as libc::c_int == tmp___1 {
                return 1 as libc::c_int;
            }
        }
        offset = offset.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn greatest_test_pre(
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut g: *mut greatest_run_info = 0 as *mut greatest_run_info;
    let mut match___0: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut p: *mut greatest_prng = 0 as *mut greatest_prng;
    g = &mut greatest_info;
    tmp = greatest_name_match(name, (*g).test_filter, 1 as libc::c_int);
    if tmp != 0 {
        tmp___0 = greatest_name_match(name, (*g).test_exclude, 0 as libc::c_int);
        if tmp___0 != 0 {
            tmp___1 = 0 as libc::c_int;
        } else {
            tmp___1 = 1 as libc::c_int;
        }
    } else {
        tmp___1 = 0 as libc::c_int;
    }
    match___0 = tmp___1;
    if greatest_info.flags as libc::c_int & 2 as libc::c_int != 0 {
        if match___0 != 0 {
            fprintf(stdout, b"  %s\n\0" as *const u8 as *const libc::c_char, name);
        }
        return 0 as libc::c_int;
    }
    if match___0 != 0 {
        if !(greatest_info.flags as libc::c_int & 1 as libc::c_int == 0) {
            if !((*g).suite.failed == 0 as libc::c_uint) {
                return 0 as libc::c_int;
            }
        }
        p = &mut *((*g).prng).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut greatest_prng;
        if (*p).random_order != 0 {
            (*p).count = ((*p).count).wrapping_add(1);
            if (*p).initialized == 0 {
                return 0 as libc::c_int
            } else {
                if ((*p).count).wrapping_sub(1 as libc::c_ulong) != (*p).state {
                    return 0 as libc::c_int;
                }
            }
        }
        (*g).suite.pre_test = clock();
        if (*g).suite.pre_test == -(1 as libc::c_long) {
            fprintf(
                stdout,
                b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
                b"g->suite.pre_test\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if ((*g).setup).is_some() {
            (Some(((*g).setup).expect("non-null function pointer")))
                .expect("non-null function pointer")((*g).setup_udata);
        }
        (*p).count_run = ((*p).count_run).wrapping_add(1);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
pub unsafe extern "C" fn greatest_test_post(
    mut name: *const libc::c_char,
    mut res: libc::c_int,
) {
    let mut udata: *mut libc::c_void = 0 as *mut libc::c_void;
    greatest_info.suite.post_test = clock();
    if greatest_info.suite.post_test == -(1 as libc::c_long) {
        fprintf(
            stdout,
            b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
            b"greatest_info.suite.post_test\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if (greatest_info.teardown).is_some() {
        udata = greatest_info.teardown_udata;
        (Some((greatest_info.teardown).expect("non-null function pointer")))
            .expect("non-null function pointer")(udata);
    }
    if res <= -(1 as libc::c_int) {
        greatest_do_fail(name);
    } else if res >= 1 as libc::c_int {
        greatest_do_skip(name);
    } else if res == 0 as libc::c_int {
        greatest_do_pass(name);
    }
    greatest_info.suite.tests_run = (greatest_info.suite.tests_run).wrapping_add(1);
    greatest_info.col = (greatest_info.col).wrapping_add(1);
    if greatest_info.verbosity as libc::c_int > 0 as libc::c_int {
        fprintf(
            stdout,
            b" (%lu ticks, %.3f sec)\0" as *const u8 as *const libc::c_char,
            (greatest_info.suite.post_test as libc::c_ulong)
                .wrapping_sub(greatest_info.suite.pre_test as libc::c_ulong),
            (greatest_info.suite.post_test - greatest_info.suite.pre_test)
                as libc::c_double / (1.0f64 * 1000000 as libc::c_long as libc::c_double),
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    } else if (greatest_info.col).wrapping_rem(greatest_info.width) == 0 as libc::c_uint
        {
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info.col = 0 as libc::c_uint;
    }
    fflush(stdout);
}
unsafe extern "C" fn report_suite() {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    if greatest_info.suite.tests_run > 0 as libc::c_uint {
        if greatest_info.suite.tests_run == 1 as libc::c_uint {
            tmp = b"\0" as *const u8 as *const libc::c_char;
        } else {
            tmp = b"s\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stdout,
            b"\n%u test%s - %u passed, %u failed, %u skipped\0" as *const u8
                as *const libc::c_char,
            greatest_info.suite.tests_run,
            tmp,
            greatest_info.suite.passed,
            greatest_info.suite.failed,
            greatest_info.suite.skipped,
        );
        fprintf(
            stdout,
            b" (%lu ticks, %.3f sec)\0" as *const u8 as *const libc::c_char,
            (greatest_info.suite.post_suite as libc::c_ulong)
                .wrapping_sub(greatest_info.suite.pre_suite as libc::c_ulong),
            (greatest_info.suite.post_suite - greatest_info.suite.pre_suite)
                as libc::c_double / (1.0f64 * 1000000 as libc::c_long as libc::c_double),
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn update_counts_and_reset_suite() {
    greatest_info
        .setup = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<greatest_setup_cb>,
    >(0 as *mut libc::c_void);
    greatest_info.setup_udata = 0 as *mut libc::c_void;
    greatest_info
        .teardown = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<greatest_teardown_cb>,
    >(0 as *mut libc::c_void);
    greatest_info.teardown_udata = 0 as *mut libc::c_void;
    greatest_info
        .passed = (greatest_info.passed).wrapping_add(greatest_info.suite.passed);
    greatest_info
        .failed = (greatest_info.failed).wrapping_add(greatest_info.suite.failed);
    greatest_info
        .skipped = (greatest_info.skipped).wrapping_add(greatest_info.suite.skipped);
    greatest_info
        .tests_run = (greatest_info.tests_run)
        .wrapping_add(greatest_info.suite.tests_run);
    memset(
        &mut greatest_info.suite as *mut greatest_suite_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<greatest_suite_info>() as libc::c_ulong,
    );
    greatest_info.col = 0 as libc::c_uint;
}
pub unsafe extern "C" fn greatest_suite_pre(
    mut suite_name: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut greatest_prng = 0 as *mut greatest_prng;
    let mut tmp: libc::c_int = 0;
    p = &mut *(greatest_info.prng).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut greatest_prng;
    tmp = greatest_name_match(suite_name, greatest_info.suite_filter, 1 as libc::c_int);
    if tmp != 0 {
        if greatest_info.flags as libc::c_int & 1 as libc::c_int != 0 {
            if greatest_info.failed > 0 as libc::c_uint {
                return 0 as libc::c_int;
            }
        }
    } else {
        return 0 as libc::c_int
    }
    if (*p).random_order != 0 {
        (*p).count = ((*p).count).wrapping_add(1);
        if (*p).initialized == 0 {
            return 0 as libc::c_int
        } else {
            if ((*p).count).wrapping_sub(1 as libc::c_ulong) != (*p).state {
                return 0 as libc::c_int;
            }
        }
    }
    (*p).count_run = ((*p).count_run).wrapping_add(1);
    update_counts_and_reset_suite();
    fprintf(
        stdout,
        b"\n* Suite %s:\n\0" as *const u8 as *const libc::c_char,
        suite_name,
    );
    greatest_info.suite.pre_suite = clock();
    if greatest_info.suite.pre_suite == -(1 as libc::c_long) {
        fprintf(
            stdout,
            b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
            b"greatest_info.suite.pre_suite\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn greatest_suite_post() {
    greatest_info.suite.post_suite = clock();
    if greatest_info.suite.post_suite == -(1 as libc::c_long) {
        fprintf(
            stdout,
            b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
            b"greatest_info.suite.post_suite\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    report_suite();
}
unsafe extern "C" fn greatest_run_suite(
    mut suite_cb: Option::<greatest_suite_cb>,
    mut suite_name: *const libc::c_char,
) {
    let mut tmp: libc::c_int = 0;
    tmp = greatest_suite_pre(suite_name);
    if tmp != 0 {
        (Some(suite_cb.expect("non-null function pointer")))
            .expect("non-null function pointer")();
        greatest_suite_post();
    }
}
pub unsafe extern "C" fn greatest_do_pass(mut name: *const libc::c_char) {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    if greatest_info.verbosity as libc::c_int > 0 as libc::c_int {
        if !(greatest_info.msg).is_null() {
            tmp = greatest_info.msg;
        } else {
            tmp = b"\0" as *const u8 as *const libc::c_char;
        }
        fprintf(stdout, b"PASS %s: %s\0" as *const u8 as *const libc::c_char, name, tmp);
    } else {
        fprintf(stdout, b".\0" as *const u8 as *const libc::c_char);
    }
    greatest_info.suite.passed = (greatest_info.suite.passed).wrapping_add(1);
}
pub unsafe extern "C" fn greatest_do_fail(mut name: *const libc::c_char) {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    if greatest_info.verbosity as libc::c_int > 0 as libc::c_int {
        if !(greatest_info.msg).is_null() {
            tmp = greatest_info.msg;
        } else {
            tmp = b"\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stdout,
            b"FAIL %s: %s (%s:%u)\0" as *const u8 as *const libc::c_char,
            name,
            tmp,
            greatest_info.fail_file,
            greatest_info.fail_line,
        );
    } else {
        fprintf(stdout, b"F\0" as *const u8 as *const libc::c_char);
        greatest_info.col = (greatest_info.col).wrapping_add(1);
        if greatest_info.col != 0 as libc::c_uint {
            fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
            greatest_info.col = 0 as libc::c_uint;
        }
        if !(greatest_info.msg).is_null() {
            tmp___0 = greatest_info.msg;
        } else {
            tmp___0 = b"\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stdout,
            b"FAIL %s: %s (%s:%u)\n\0" as *const u8 as *const libc::c_char,
            name,
            tmp___0,
            greatest_info.fail_file,
            greatest_info.fail_line,
        );
    }
    greatest_info.suite.failed = (greatest_info.suite.failed).wrapping_add(1);
}
pub unsafe extern "C" fn greatest_do_skip(mut name: *const libc::c_char) {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    if greatest_info.verbosity as libc::c_int > 0 as libc::c_int {
        if !(greatest_info.msg).is_null() {
            tmp = greatest_info.msg;
        } else {
            tmp = b"\0" as *const u8 as *const libc::c_char;
        }
        fprintf(stdout, b"SKIP %s: %s\0" as *const u8 as *const libc::c_char, name, tmp);
    } else {
        fprintf(stdout, b"s\0" as *const u8 as *const libc::c_char);
    }
    greatest_info.suite.skipped = (greatest_info.suite.skipped).wrapping_add(1);
}
pub unsafe extern "C" fn greatest_do_assert_equal_t(
    mut exp: *const libc::c_void,
    mut got: *const libc::c_void,
    mut type_info: *mut greatest_type_info,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut eq: libc::c_int = 0;
    eq = 0 as libc::c_int;
    if type_info as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int
    } else {
        if ::std::mem::transmute::<
            Option::<greatest_equal_cb>,
            libc::c_ulong,
        >((*type_info).equal) == 0 as *mut libc::c_void as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
    }
    eq = (Some(((*type_info).equal).expect("non-null function pointer")))
        .expect("non-null function pointer")(exp, got, udata);
    if eq == 0 {
        if ::std::mem::transmute::<
            Option::<greatest_printf_cb>,
            libc::c_ulong,
        >((*type_info).print) != 0 as *mut libc::c_void as libc::c_ulong
        {
            fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
            (Some(((*type_info).print).expect("non-null function pointer")))
                .expect("non-null function pointer")(exp, udata);
            fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
            (Some(((*type_info).print).expect("non-null function pointer")))
                .expect("non-null function pointer")(got, udata);
            fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        } else {
            fprintf(
                stdout,
                b"GREATEST_ASSERT_EQUAL_T failure at %s:%u\n\0" as *const u8
                    as *const libc::c_char,
                greatest_info.fail_file,
                greatest_info.fail_line,
            );
        }
    }
    return eq;
}
pub unsafe extern "C" fn greatest_usage(mut name: *const libc::c_char) {
    fprintf(
        stdout,
        b"Usage: %s [--help] [-hlfv] [-s SUITE] [-t TEST]\n  -h, --help  print this Help\n  -l          List suites and tests, then exit (dry run)\n  -f          Stop runner after first failure\n  -v          Verbose output\n  -s SUITE    only run suites containing string SUITE\n  -t TEST     only run tests containing string TEST\n  -x EXCLUDE  exclude tests containing string EXCLUDE\n\0"
            as *const u8 as *const libc::c_char,
        name,
    );
}
unsafe extern "C" fn greatest_parse_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut f: libc::c_char = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    i = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
            == 45 as libc::c_int
        {
            f = *(*argv.offset(i as isize)).offset(1 as libc::c_int as isize);
            if f as libc::c_int == 115 as libc::c_int {
                current_block = 18043409425672922519;
            } else if f as libc::c_int == 116 as libc::c_int {
                current_block = 18043409425672922519;
            } else if f as libc::c_int == 120 as libc::c_int {
                current_block = 18043409425672922519;
            } else {
                current_block = 1841672684692190573;
            }
            match current_block {
                18043409425672922519 => {
                    if argc <= i + 1 as libc::c_int {
                        greatest_usage(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                }
                _ => {}
            }
            let mut current_block_30: u64;
            match f as libc::c_int {
                115 => {
                    greatest_set_suite_filter(
                        *argv.offset((i + 1 as libc::c_int) as isize)
                            as *const libc::c_char,
                    );
                    i += 1;
                    current_block_30 = 9853141518545631134;
                }
                116 => {
                    greatest_set_test_filter(
                        *argv.offset((i + 1 as libc::c_int) as isize)
                            as *const libc::c_char,
                    );
                    i += 1;
                    current_block_30 = 9853141518545631134;
                }
                120 => {
                    greatest_set_test_exclude(
                        *argv.offset((i + 1 as libc::c_int) as isize)
                            as *const libc::c_char,
                    );
                    i += 1;
                    current_block_30 = 9853141518545631134;
                }
                102 => {
                    greatest_stop_at_first_fail();
                    current_block_30 = 9853141518545631134;
                }
                108 => {
                    greatest_info
                        .flags = (greatest_info.flags as libc::c_int | 2 as libc::c_int)
                        as libc::c_uchar;
                    current_block_30 = 9853141518545631134;
                }
                118 => {
                    greatest_info
                        .verbosity = (greatest_info.verbosity as libc::c_int
                        + 1 as libc::c_int) as libc::c_uchar;
                    current_block_30 = 9853141518545631134;
                }
                104 => {
                    greatest_usage(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    );
                    exit(0 as libc::c_int);
                }
                45 => {
                    tmp___0 = strncmp(
                        b"--help\0" as *const u8 as *const libc::c_char,
                        *argv.offset(i as isize) as *const libc::c_char,
                        6 as libc::c_int as size_t,
                    );
                    if 0 as libc::c_int == tmp___0 {
                        greatest_usage(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                        );
                        exit(0 as libc::c_int);
                    } else {
                        tmp = strncmp(
                            b"--\0" as *const u8 as *const libc::c_char,
                            *argv.offset(i as isize) as *const libc::c_char,
                            2 as libc::c_int as size_t,
                        );
                        if 0 as libc::c_int == tmp {
                            return;
                        }
                    }
                    current_block_30 = 11466758465337983566;
                }
                _ => {
                    current_block_30 = 11466758465337983566;
                }
            }
            match current_block_30 {
                9853141518545631134 => {}
                _ => {
                    fprintf(
                        stdout,
                        b"Unknown argument '%s'\n\0" as *const u8 as *const libc::c_char,
                        *argv.offset(i as isize),
                    );
                    greatest_usage(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
            }
        }
        i += 1;
    }
}
pub unsafe extern "C" fn greatest_all_passed() -> libc::c_int {
    return (greatest_info.failed == 0 as libc::c_uint) as libc::c_int;
}
pub unsafe extern "C" fn greatest_set_test_filter(mut filter: *const libc::c_char) {
    greatest_info.test_filter = filter;
}
pub unsafe extern "C" fn greatest_set_test_exclude(mut filter: *const libc::c_char) {
    greatest_info.test_exclude = filter;
}
pub unsafe extern "C" fn greatest_set_suite_filter(mut filter: *const libc::c_char) {
    greatest_info.suite_filter = filter;
}
pub unsafe extern "C" fn greatest_stop_at_first_fail() {
    greatest_info
        .flags = (greatest_info.flags as libc::c_int | 1 as libc::c_int)
        as libc::c_uchar;
}
pub unsafe extern "C" fn greatest_get_report(mut report: *mut greatest_report_t) {
    if !report.is_null() {
        (*report).passed = greatest_info.passed;
        (*report).failed = greatest_info.failed;
        (*report).skipped = greatest_info.skipped;
        (*report).assertions = greatest_info.assertions;
    }
}
pub unsafe extern "C" fn greatest_get_verbosity() -> libc::c_uint {
    return greatest_info.verbosity as libc::c_uint;
}
pub unsafe extern "C" fn greatest_set_verbosity(mut verbosity: libc::c_uint) {
    greatest_info.verbosity = verbosity as libc::c_uchar;
}
pub unsafe extern "C" fn greatest_set_flag(mut flag: greatest_flag_t) {
    greatest_info
        .flags = (greatest_info.flags as libc::c_uint | flag as libc::c_uint)
        as libc::c_uchar;
}
pub unsafe extern "C" fn GREATEST_SET_SETUP_CB(
    mut cb: Option::<greatest_setup_cb>,
    mut udata: *mut libc::c_void,
) {
    greatest_info.setup = cb;
    greatest_info.setup_udata = udata;
}
pub unsafe extern "C" fn GREATEST_SET_TEARDOWN_CB(
    mut cb: Option::<greatest_teardown_cb>,
    mut udata: *mut libc::c_void,
) {
    greatest_info.teardown = cb;
    greatest_info.teardown_udata = udata;
}
unsafe extern "C" fn greatest_string_equal_cb(
    mut exp: *const libc::c_void,
    mut got: *const libc::c_void,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut size: *mut size_t = 0 as *mut size_t;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    size = udata as *mut size_t;
    if size as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = strncmp(exp as *const libc::c_char, got as *const libc::c_char, *size);
        tmp___1 = (0 as libc::c_int == tmp) as libc::c_int;
    } else {
        tmp___0 = strcmp(exp as *const libc::c_char, got as *const libc::c_char);
        tmp___1 = (0 as libc::c_int == tmp___0) as libc::c_int;
    }
    return tmp___1;
}
unsafe extern "C" fn greatest_string_printf_cb(
    mut t: *const libc::c_void,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = fprintf(
        stdout,
        b"%s\0" as *const u8 as *const libc::c_char,
        t as *const libc::c_char,
    );
    return tmp;
}
pub static mut greatest_type_info_string: greatest_type_info = {
    let mut init = greatest_type_info {
        equal: Some(
            greatest_string_equal_cb
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        print: Some(
            greatest_string_printf_cb
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    };
    init
};
unsafe extern "C" fn greatest_memory_equal_cb(
    mut exp: *const libc::c_void,
    mut got: *const libc::c_void,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut env: *mut greatest_memory_cmp_env = 0 as *mut greatest_memory_cmp_env;
    let mut tmp: libc::c_int = 0;
    env = udata as *mut greatest_memory_cmp_env;
    tmp = memcmp(exp, got, (*env).size);
    return (0 as libc::c_int == tmp) as libc::c_int;
}
unsafe extern "C" fn greatest_memory_printf_cb(
    mut t: *const libc::c_void,
    mut udata: *mut libc::c_void,
) -> libc::c_int {
    let mut env: *mut greatest_memory_cmp_env = 0 as *mut greatest_memory_cmp_env;
    let mut buf: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut diff_mark: libc::c_uchar = 0;
    let mut out: *mut FILE = 0 as *mut FILE;
    let mut i: size_t = 0;
    let mut line_i: size_t = 0;
    let mut line_len: size_t = 0;
    let mut len: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut c: libc::c_uchar = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    env = udata as *mut greatest_memory_cmp_env;
    buf = t as *const libc::c_uchar;
    diff_mark = ' ' as i32 as libc::c_uchar;
    out = stdout;
    line_len = 0 as libc::c_int as size_t;
    len = 0 as libc::c_int;
    i = 0 as libc::c_int as size_t;
    while i < (*env).size {
        diff_mark = ' ' as i32 as libc::c_uchar;
        line_len = ((*env).size).wrapping_sub(i);
        if line_len > 16 as libc::c_ulong {
            line_len = 16 as libc::c_int as size_t;
        }
        line_i = i;
        while line_i < i.wrapping_add(line_len) {
            if *((*env).exp).offset(line_i as isize) as libc::c_int
                != *((*env).got).offset(line_i as isize) as libc::c_int
            {
                diff_mark = 'X' as i32 as libc::c_uchar;
            }
            line_i = line_i.wrapping_add(1);
        }
        tmp = fprintf(
            out,
            b"\n%04x %c \0" as *const u8 as *const libc::c_char,
            i as libc::c_uint,
            diff_mark as libc::c_int,
        );
        len += tmp;
        line_i = i;
        while line_i < i.wrapping_add(line_len) {
            m = (*((*env).exp).offset(line_i as isize) as libc::c_int
                == *((*env).got).offset(line_i as isize) as libc::c_int) as libc::c_int;
            if m != 0 {
                tmp___0 = ' ' as i32;
            } else {
                tmp___0 = '<' as i32;
            }
            tmp___1 = fprintf(
                out,
                b"%02x%c\0" as *const u8 as *const libc::c_char,
                *buf.offset(line_i as isize) as libc::c_int,
                tmp___0,
            );
            len += tmp___1;
            line_i = line_i.wrapping_add(1);
        }
        line_i = 0 as libc::c_int as size_t;
        while line_i < (16 as libc::c_ulong).wrapping_sub(line_len) {
            tmp___2 = fprintf(out, b"   \0" as *const u8 as *const libc::c_char);
            len += tmp___2;
            line_i = line_i.wrapping_add(1);
        }
        fprintf(out, b" \0" as *const u8 as *const libc::c_char);
        line_i = i;
        while line_i < i.wrapping_add(line_len) {
            c = *buf.offset(line_i as isize);
            tmp___5 = __ctype_b_loc();
            if *(*tmp___5).offset(c as libc::c_int as isize) as libc::c_int
                & 16384 as libc::c_int != 0
            {
                tmp___4 = c as libc::c_int;
            } else {
                tmp___4 = '.' as i32;
            }
            tmp___6 = fprintf(out, b"%c\0" as *const u8 as *const libc::c_char, tmp___4);
            len += tmp___6;
            line_i = line_i.wrapping_add(1);
        }
        i = (i as libc::c_ulong).wrapping_add(line_len) as size_t as size_t;
    }
    tmp___7 = fprintf(out, b"\n\0" as *const u8 as *const libc::c_char);
    len += tmp___7;
    return len;
}
pub unsafe extern "C" fn greatest_prng_init_first_pass(mut id: libc::c_int) {
    greatest_info.prng[id as usize].random_order = 1 as libc::c_int as libc::c_uchar;
    greatest_info.prng[id as usize].count_run = 0 as libc::c_ulong;
}
static mut primes: [libc::c_ulong; 13] = [
    11 as libc::c_ulong,
    101 as libc::c_ulong,
    1009 as libc::c_ulong,
    10007 as libc::c_ulong,
    100003 as libc::c_ulong,
    1000003 as libc::c_ulong,
    10000019 as libc::c_ulong,
    100000007 as libc::c_ulong,
    1000000007 as libc::c_ulong,
    1538461 as libc::c_ulong,
    1865471 as libc::c_ulong,
    17471 as libc::c_ulong,
    2147483647 as libc::c_ulong,
];
pub unsafe extern "C" fn greatest_prng_init_second_pass(
    mut id: libc::c_int,
    mut seed: libc::c_ulong,
) -> libc::c_int {
    let mut prng: *mut greatest_prng = 0 as *mut greatest_prng;
    prng = &mut *(greatest_info.prng).as_mut_ptr().offset(id as isize)
        as *mut greatest_prng;
    if (*prng).count == 0 as libc::c_ulong {
        return 0 as libc::c_int;
    }
    (*prng).mod_0 = 1 as libc::c_ulong;
    (*prng).count_ceil = (*prng).count;
    while (*prng).mod_0 < (*prng).count {
        (*prng).mod_0 <<= 1 as libc::c_int;
    }
    (*prng).state = seed & 536870911 as libc::c_ulong;
    (*prng)
        .a = (4 as libc::c_ulong)
        .wrapping_mul((*prng).state)
        .wrapping_add(1 as libc::c_ulong);
    (*prng)
        .c = primes[seed
        .wrapping_mul(16451 as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<[libc::c_ulong; 13]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong) as usize];
    (*prng).initialized = 1 as libc::c_int as libc::c_uchar;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn greatest_prng_step(mut id: libc::c_int) {
    let mut p: *mut greatest_prng = 0 as *mut greatest_prng;
    p = &mut *(greatest_info.prng).as_mut_ptr().offset(id as isize)
        as *mut greatest_prng;
    loop {
        (*p)
            .state = ((*p).a).wrapping_mul((*p).state).wrapping_add((*p).c)
            & ((*p).mod_0).wrapping_sub(1 as libc::c_ulong);
        if !((*p).state >= (*p).count_ceil) {
            break;
        }
    };
}
pub static mut greatest_type_info_memory: greatest_type_info = {
    let mut init = greatest_type_info {
        equal: Some(
            greatest_memory_equal_cb
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        print: Some(
            greatest_memory_printf_cb
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    };
    init
};
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    memset(
        &mut greatest_info as *mut greatest_run_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<greatest_run_info>() as libc::c_ulong,
    );
    greatest_info.width = 72 as libc::c_uint;
    greatest_info.begin = clock();
    if greatest_info.begin == -(1 as libc::c_long) {
        fprintf(
            stdout,
            b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
            b"greatest_info.begin\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    greatest_parse_options(argc, argv);
    greatest_run_suite(
        Some(match_suite as unsafe extern "C" fn() -> ()),
        b"match_suite\0" as *const u8 as *const libc::c_char,
    );
    greatest_run_suite(
        Some(choices_suite as unsafe extern "C" fn() -> ()),
        b"choices_suite\0" as *const u8 as *const libc::c_char,
    );
    greatest_run_suite(
        Some(properties_suite as unsafe extern "C" fn() -> ()),
        b"properties_suite\0" as *const u8 as *const libc::c_char,
    );
    if greatest_info.flags as libc::c_int & 2 as libc::c_int == 0 {
        update_counts_and_reset_suite();
        greatest_info.end = clock();
        if greatest_info.end == -(1 as libc::c_long) {
            fprintf(
                stdout,
                b"clock error: %s\n\0" as *const u8 as *const libc::c_char,
                b"greatest_info.end\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if greatest_info.tests_run == 1 as libc::c_uint {
            tmp = b"\0" as *const u8 as *const libc::c_char;
        } else {
            tmp = b"s\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stdout,
            b"\nTotal: %u test%s\0" as *const u8 as *const libc::c_char,
            greatest_info.tests_run,
            tmp,
        );
        fprintf(
            stdout,
            b" (%lu ticks, %.3f sec)\0" as *const u8 as *const libc::c_char,
            (greatest_info.end as libc::c_ulong)
                .wrapping_sub(greatest_info.begin as libc::c_ulong),
            (greatest_info.end - greatest_info.begin) as libc::c_double
                / (1.0f64 * 1000000 as libc::c_long as libc::c_double),
        );
        if greatest_info.assertions == 1 as libc::c_uint {
            tmp___0 = b"\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___0 = b"s\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stdout,
            b", %u assertion%s\n\0" as *const u8 as *const libc::c_char,
            greatest_info.assertions,
            tmp___0,
        );
        fprintf(
            stdout,
            b"Pass: %u, fail: %u, skip: %u.\n\0" as *const u8 as *const libc::c_char,
            greatest_info.passed,
            greatest_info.failed,
            greatest_info.skipped,
        );
    }
    tmp___3 = greatest_all_passed();
    if tmp___3 != 0 {
        tmp___2 = 0 as libc::c_int;
    } else {
        tmp___2 = 1 as libc::c_int;
    }
    return tmp___2;
}
unsafe extern "C" fn string_alloc_cb(
    mut t: *mut theft,
    mut seed: theft_hash,
    mut env: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut limit: libc::c_int = 0;
    let mut sz: size_t = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: size_t = 0;
    let mut s: theft_hash = 0;
    let mut tmp___0: theft_hash = 0;
    let mut b: uint8_t = 0;
    limit = 128 as libc::c_int;
    sz = seed.wrapping_rem(limit as libc::c_ulong).wrapping_add(1 as libc::c_ulong);
    tmp = malloc(sz.wrapping_add(1 as libc::c_ulong));
    str = tmp as *mut libc::c_char;
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(2 as libc::c_int) as *mut libc::c_void;
    }
    i = 0 as libc::c_int as size_t;
    while i < sz {
        tmp___0 = theft_random(t);
        s = tmp___0;
        b = 0 as libc::c_int as uint8_t;
        while (b as libc::c_ulong) < ::std::mem::size_of::<theft_hash>() as libc::c_ulong
        {
            if i.wrapping_add(b as size_t) >= sz {
                break;
            }
            *str
                .offset(
                    i.wrapping_add(b as size_t) as isize,
                ) = ((s >> 8 as libc::c_int * b as libc::c_int) as uint8_t as libc::c_int
                & 255 as libc::c_int) as libc::c_char;
            b = (b as libc::c_int + 1 as libc::c_int) as uint8_t;
        }
        i = (i as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<theft_hash>() as libc::c_ulong) as size_t
            as size_t;
    }
    *str.offset(sz as isize) = 0 as libc::c_int as libc::c_char;
    return str as *mut libc::c_void;
}
unsafe extern "C" fn string_free_cb(
    mut instance: *mut libc::c_void,
    mut env: *mut libc::c_void,
) {
    free(instance);
}
unsafe extern "C" fn string_print_cb(
    mut f: *mut FILE,
    mut instance: *mut libc::c_void,
    mut env: *mut libc::c_void,
) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: size_t = 0;
    let mut tmp: size_t = 0;
    let mut bytes: uint8_t = 0;
    let mut i: size_t = 0;
    str = instance as *mut libc::c_char;
    tmp = strlen(str as *const libc::c_char);
    size = tmp;
    fprintf(f, b"str[%zd]:\n    \0" as *const u8 as *const libc::c_char, size);
    bytes = 0 as libc::c_int as uint8_t;
    i = 0 as libc::c_int as size_t;
    while i < size {
        fprintf(
            f,
            b"%02x\0" as *const u8 as *const libc::c_char,
            *str.offset(i as isize) as libc::c_int,
        );
        bytes = (bytes as libc::c_int + 1 as libc::c_int) as uint8_t;
        if bytes as libc::c_int == 16 as libc::c_int {
            fprintf(f, b"\n    \0" as *const u8 as *const libc::c_char);
            bytes = 0 as libc::c_int as uint8_t;
        }
        i = i.wrapping_add(1);
    }
    fprintf(f, b"\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn string_hash_cb(
    mut instance: *mut libc::c_void,
    mut env: *mut libc::c_void,
) -> uint64_t {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut size: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: theft_hash = 0;
    str = instance as *mut libc::c_char;
    tmp = strlen(str as *const libc::c_char);
    size = tmp as libc::c_int;
    tmp___0 = theft_hash_onepass(str as *mut uint8_t, size as size_t);
    return tmp___0;
}
unsafe extern "C" fn string_shrink_cb(
    mut instance: *mut libc::c_void,
    mut tactic: uint32_t,
    mut env: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    str = instance as *mut libc::c_char;
    tmp = strlen(str as *const libc::c_char);
    n = tmp as libc::c_int;
    if tactic == 0 as libc::c_uint {
        tmp___0 = strndup(str as *const libc::c_char, (n / 2 as libc::c_int) as size_t);
        return tmp___0 as *mut libc::c_void;
    } else if tactic == 1 as libc::c_uint {
        tmp___1 = strndup(
            str.offset((n / 2 as libc::c_int) as isize) as *const libc::c_char,
            (n / 2 as libc::c_int) as size_t,
        );
        return tmp___1 as *mut libc::c_void;
    } else {
        return -(3 as libc::c_int) as *mut libc::c_void
    };
}
static mut string_info: theft_type_info = {
    let mut init = theft_type_info {
        alloc: Some(
            string_alloc_cb
                as unsafe extern "C" fn(
                    *mut theft,
                    theft_hash,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        free: Some(
            string_free_cb
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        ),
        hash: Some(
            string_hash_cb
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> uint64_t,
        ),
        shrink: Some(
            string_shrink_cb
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    uint32_t,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        print: Some(
            string_print_cb
                as unsafe extern "C" fn(
                    *mut FILE,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
    };
    init
};
unsafe extern "C" fn prop_should_return_results_if_there_is_a_match(
    mut needle: *mut libc::c_char,
    mut haystack: *mut libc::c_char,
) -> theft_trial_res {
    let mut match_exists: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut score: score_t = 0.;
    let mut tmp___0: score_t = 0.;
    let mut tmp___1: libc::c_float = 0.;
    tmp = has_match(needle as *const libc::c_char, haystack as *const libc::c_char);
    match_exists = tmp;
    if match_exists == 0 {
        return THEFT_TRIAL_SKIP;
    }
    tmp___0 = match_0(needle as *const libc::c_char, haystack as *const libc::c_char);
    score = tmp___0;
    if *needle.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        return THEFT_TRIAL_SKIP;
    }
    tmp___1 = ::std::f32::INFINITY;
    if score == -tmp___1 as score_t {
        return THEFT_TRIAL_FAIL;
    }
    return THEFT_TRIAL_PASS;
}
unsafe extern "C" fn should_return_results_if_there_is_a_match() -> greatest_test_res {
    let mut t: *mut theft = 0 as *mut theft;
    let mut tmp: *mut theft = 0 as *mut theft;
    let mut cfg: theft_cfg = theft_cfg {
        fun: None,
        type_info: [0 as *mut theft_type_info; 10],
        name: 0 as *const libc::c_char,
        always_seed_count: 0,
        always_seeds: 0 as *mut theft_seed,
        trials: 0,
        progress_cb: None,
        env: 0 as *mut libc::c_void,
        report: 0 as *mut theft_trial_report,
        seed: 0,
    };
    let mut tmp___0: libc::c_uint = 0;
    let mut res: theft_run_res = THEFT_RUN_PASS;
    let mut tmp___1: theft_run_res = THEFT_RUN_PASS;
    tmp = theft_init(0 as libc::c_int as uint8_t);
    t = tmp;
    cfg
        .fun = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> theft_trial_res,
        >,
        Option::<theft_propfun>,
    >(
        Some(
            prop_should_return_results_if_there_is_a_match
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut libc::c_char,
                ) -> theft_trial_res,
        ),
    );
    cfg.type_info[0 as libc::c_int as usize] = &mut string_info;
    cfg.type_info[1 as libc::c_int as usize] = &mut string_info;
    tmp___0 = 2 as libc::c_uint;
    while !(tmp___0 >= 10 as libc::c_uint) {
        cfg.type_info[tmp___0 as usize] = 0 as *mut theft_type_info;
        tmp___0 = tmp___0.wrapping_add(1);
    }
    cfg
        .name = b"should_return_results_if_there_is_a_match\0" as *const u8
        as *const libc::c_char;
    cfg.always_seed_count = 0 as libc::c_int;
    cfg.always_seeds = 0 as *mut theft_seed;
    cfg.trials = 100000 as libc::c_int;
    cfg.progress_cb = None;
    cfg.env = 0 as *mut libc::c_void;
    cfg.report = 0 as *mut theft_trial_report;
    cfg.seed = 0 as libc::c_ulong;
    tmp___1 = theft_run(t, &mut cfg);
    res = tmp___1;
    theft_free(t);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_int != res as libc::c_int {
        greatest_info
            .fail_file = b"test/test_properties.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 112 as libc::c_uint;
        greatest_info
            .msg = b"should_return_results_if_there_is_a_match\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn prop_positions_should_match_characters_in_string(
    mut needle: *mut libc::c_char,
    mut haystack: *mut libc::c_char,
) -> theft_trial_res {
    let mut match_exists: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut positions: *mut size_t = 0 as *mut size_t;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut __res: libc::c_int = 0;
    let mut tmp___3: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut __res___0: libc::c_int = 0;
    let mut tmp___5: *mut *const __int32_t = 0 as *mut *const __int32_t;
    tmp = has_match(needle as *const libc::c_char, haystack as *const libc::c_char);
    match_exists = tmp;
    if match_exists == 0 {
        return THEFT_TRIAL_SKIP;
    }
    tmp___0 = strlen(needle as *const libc::c_char);
    n = tmp___0 as libc::c_int;
    tmp___1 = calloc(n as size_t, ::std::mem::size_of::<size_t>() as libc::c_ulong);
    positions = tmp___1 as *mut size_t;
    if positions.is_null() {
        return THEFT_TRIAL_ERROR;
    }
    match_positions(
        needle as *const libc::c_char,
        haystack as *const libc::c_char,
        positions,
    );
    i = 1 as libc::c_int;
    while i < n {
        if *positions.offset(i as isize)
            <= *positions.offset((i - 1 as libc::c_int) as isize)
        {
            return THEFT_TRIAL_FAIL;
        }
        i += 1;
    }
    i___0 = 0 as libc::c_int;
    while i___0 < n {
        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong > 1 as libc::c_ulong {
            __res = toupper(*needle.offset(i___0 as isize) as libc::c_int);
        } else {
            tmp___3 = __ctype_toupper_loc();
            __res = *(*tmp___3)
                .offset(*needle.offset(i___0 as isize) as libc::c_int as isize);
        }
        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong > 1 as libc::c_ulong {
            __res___0 = toupper(
                *haystack.offset(*positions.offset(i___0 as isize) as isize)
                    as libc::c_int,
            );
        } else {
            tmp___5 = __ctype_toupper_loc();
            __res___0 = *(*tmp___5)
                .offset(
                    *haystack.offset(*positions.offset(i___0 as isize) as isize)
                        as libc::c_int as isize,
                );
        }
        if __res != __res___0 {
            return THEFT_TRIAL_FAIL;
        }
        i___0 += 1;
    }
    free(positions as *mut libc::c_void);
    return THEFT_TRIAL_PASS;
}
unsafe extern "C" fn positions_should_match_characters_in_string() -> greatest_test_res {
    let mut t: *mut theft = 0 as *mut theft;
    let mut tmp: *mut theft = 0 as *mut theft;
    let mut cfg: theft_cfg = theft_cfg {
        fun: None,
        type_info: [0 as *mut theft_type_info; 10],
        name: 0 as *const libc::c_char,
        always_seed_count: 0,
        always_seeds: 0 as *mut theft_seed,
        trials: 0,
        progress_cb: None,
        env: 0 as *mut libc::c_void,
        report: 0 as *mut theft_trial_report,
        seed: 0,
    };
    let mut tmp___0: libc::c_uint = 0;
    let mut res: theft_run_res = THEFT_RUN_PASS;
    let mut tmp___1: theft_run_res = THEFT_RUN_PASS;
    tmp = theft_init(0 as libc::c_int as uint8_t);
    t = tmp;
    cfg
        .fun = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> theft_trial_res,
        >,
        Option::<theft_propfun>,
    >(
        Some(
            prop_positions_should_match_characters_in_string
                as unsafe extern "C" fn(
                    *mut libc::c_char,
                    *mut libc::c_char,
                ) -> theft_trial_res,
        ),
    );
    cfg.type_info[0 as libc::c_int as usize] = &mut string_info;
    cfg.type_info[1 as libc::c_int as usize] = &mut string_info;
    tmp___0 = 2 as libc::c_uint;
    while !(tmp___0 >= 10 as libc::c_uint) {
        cfg.type_info[tmp___0 as usize] = 0 as *mut theft_type_info;
        tmp___0 = tmp___0.wrapping_add(1);
    }
    cfg
        .name = b"positions_should_match_characters_in_string\0" as *const u8
        as *const libc::c_char;
    cfg.always_seed_count = 0 as libc::c_int;
    cfg.always_seeds = 0 as *mut theft_seed;
    cfg.trials = 100000 as libc::c_int;
    cfg.progress_cb = None;
    cfg.env = 0 as *mut libc::c_void;
    cfg.report = 0 as *mut theft_trial_report;
    cfg.seed = 0 as libc::c_ulong;
    tmp___1 = theft_run(t, &mut cfg);
    res = tmp___1;
    theft_free(t);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_int != res as libc::c_int {
        greatest_info
            .fail_file = b"test/test_properties.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 158 as libc::c_uint;
        greatest_info
            .msg = b"should_return_results_if_there_is_a_match\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
pub unsafe extern "C" fn properties_suite() {
    let mut res: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut res___0: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    tmp___0 = greatest_test_pre(
        b"should_return_results_if_there_is_a_match\0" as *const u8
            as *const libc::c_char,
    );
    if tmp___0 == 1 as libc::c_int {
        tmp = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res = tmp as greatest_test_res;
        if res as libc::c_int == 0 as libc::c_int {
            res = should_return_results_if_there_is_a_match();
        }
        greatest_test_post(
            b"should_return_results_if_there_is_a_match\0" as *const u8
                as *const libc::c_char,
            res as libc::c_int,
        );
    }
    tmp___2 = greatest_test_pre(
        b"positions_should_match_characters_in_string\0" as *const u8
            as *const libc::c_char,
    );
    if tmp___2 == 1 as libc::c_int {
        tmp___1 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___0 = tmp___1 as greatest_test_res;
        if res___0 as libc::c_int == 0 as libc::c_int {
            res___0 = positions_should_match_characters_in_string();
        }
        greatest_test_post(
            b"positions_should_match_characters_in_string\0" as *const u8
                as *const libc::c_char,
            res___0 as libc::c_int,
        );
    }
}
static mut default_options: options_t = options_t {
    benchmark: 0,
    filter: 0 as *const libc::c_char,
    init_search: 0 as *const libc::c_char,
    tty_filename: 0 as *const libc::c_char,
    show_scores: 0,
    num_lines: 0,
    scrolloff: 0,
    prompt: 0 as *const libc::c_char,
    workers: 0,
    input_delimiter: 0,
    show_info: 0,
};
static mut choices: choices_t = choices_t {
    buffer: 0 as *const libc::c_char as *mut libc::c_char,
    buffer_size: 0,
    capacity: 0,
    size: 0,
    strings: 0 as *const *const libc::c_char as *mut *const libc::c_char,
    results: 0 as *const scored_result as *mut scored_result,
    available: 0,
    selection: 0,
    worker_count: 0,
};
unsafe extern "C" fn setup(mut udata: *mut libc::c_void) {
    options_init(&mut default_options);
    choices_init(&mut choices, &mut default_options);
}
unsafe extern "C" fn teardown(mut udata: *mut libc::c_void) {
    choices_destroy(&mut choices);
}
unsafe extern "C" fn test_choices_empty() -> greatest_test_res {
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.size {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.size);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 29 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.size)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 30 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 31 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_prev(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 34 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_next(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 37 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn test_choices_1() -> greatest_test_res {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    choices_add(&mut choices, b"tags\0" as *const u8 as *const libc::c_char);
    choices_search(&mut choices, b"\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 1 as libc::c_ulong != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 46 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 47 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_search(&mut choices, b"t\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 1 as libc::c_ulong != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 50 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 51 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_prev(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 54 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_next(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 57 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = choices_get(&mut choices, 0 as libc::c_int as size_t);
    tmp___0 = strcmp(tmp, b"tags\0" as *const u8 as *const libc::c_char);
    if tmp___0 != 0 {
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 59 as libc::c_uint;
        greatest_info
            .msg = b"!strcmp(choices_get(&choices, 0), \"tags\")\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp___1 = choices_get(&mut choices, 1 as libc::c_int as size_t);
    if 0 as *mut libc::c_void as libc::c_ulong != tmp___1 as libc::c_ulong {
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 60 as libc::c_uint;
        greatest_info
            .msg = b"NULL != choices_get(&choices, 1)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn test_choices_2() -> greatest_test_res {
    let mut type_info: *mut greatest_type_info = 0 as *mut greatest_type_info;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut type_info___0: *mut greatest_type_info = 0 as *mut greatest_type_info;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut type_info___1: *mut greatest_type_info = 0 as *mut greatest_type_info;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    choices_add(&mut choices, b"tags\0" as *const u8 as *const libc::c_char);
    choices_add(&mut choices, b"test\0" as *const u8 as *const libc::c_char);
    choices_search(&mut choices, b"\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 71 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 2 as libc::c_ulong != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 72 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(2) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_next(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 1 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 75 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_next(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 77 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_prev(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 1 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 80 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_prev(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 82 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_search(&mut choices, b"te\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 1 as libc::c_ulong != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 86 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 87 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    type_info = &mut greatest_type_info_string;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = choices_get(&mut choices, 0 as libc::c_int as size_t);
    tmp___0 = greatest_do_assert_equal_t(
        b"test\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp as *const libc::c_void,
        type_info,
        0 as *mut libc::c_void,
    );
    if tmp___0 == 0 {
        if !(type_info as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            if !(::std::mem::transmute::<
                Option::<greatest_equal_cb>,
                libc::c_ulong,
            >((*type_info).equal) == 0 as *mut libc::c_void as libc::c_ulong)
            {
                greatest_info
                    .fail_file = b"test/test_choices.c\0" as *const u8
                    as *const libc::c_char;
                greatest_info.fail_line = 88 as libc::c_uint;
                greatest_info
                    .msg = b"\"test\" != choices_get(&choices, 0)\0" as *const u8
                    as *const libc::c_char;
                return GREATEST_TEST_RES_FAIL;
            }
        }
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 88 as libc::c_uint;
        greatest_info
            .msg = b"type_info->equal callback missing!\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_next(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 91 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_prev(&mut choices);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 94 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_search(&mut choices, b"foobar\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 98 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 99 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_search(&mut choices, b"ts\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 2 as libc::c_ulong != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 103 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(2) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 104 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    type_info___0 = &mut greatest_type_info_string;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp___1 = choices_get(&mut choices, 0 as libc::c_int as size_t);
    tmp___2 = greatest_do_assert_equal_t(
        b"test\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___1 as *const libc::c_void,
        type_info___0,
        0 as *mut libc::c_void,
    );
    if tmp___2 == 0 {
        if !(type_info___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            if !(::std::mem::transmute::<
                Option::<greatest_equal_cb>,
                libc::c_ulong,
            >((*type_info___0).equal) == 0 as *mut libc::c_void as libc::c_ulong)
            {
                greatest_info
                    .fail_file = b"test/test_choices.c\0" as *const u8
                    as *const libc::c_char;
                greatest_info.fail_line = 105 as libc::c_uint;
                greatest_info
                    .msg = b"\"test\" != choices_get(&choices, 0)\0" as *const u8
                    as *const libc::c_char;
                return GREATEST_TEST_RES_FAIL;
            }
        }
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 105 as libc::c_uint;
        greatest_info
            .msg = b"type_info->equal callback missing!\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    type_info___1 = &mut greatest_type_info_string;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp___3 = choices_get(&mut choices, 1 as libc::c_int as size_t);
    tmp___4 = greatest_do_assert_equal_t(
        b"tags\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp___3 as *const libc::c_void,
        type_info___1,
        0 as *mut libc::c_void,
    );
    if tmp___4 == 0 {
        if !(type_info___1 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            if !(::std::mem::transmute::<
                Option::<greatest_equal_cb>,
                libc::c_ulong,
            >((*type_info___1).equal) == 0 as *mut libc::c_void as libc::c_ulong)
            {
                greatest_info
                    .fail_file = b"test/test_choices.c\0" as *const u8
                    as *const libc::c_char;
                greatest_info.fail_line = 106 as libc::c_uint;
                greatest_info
                    .msg = b"\"tags\" != choices_get(&choices, 1)\0" as *const u8
                    as *const libc::c_char;
                return GREATEST_TEST_RES_FAIL;
            }
        }
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 106 as libc::c_uint;
        greatest_info
            .msg = b"type_info->equal callback missing!\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn test_choices_without_search() -> greatest_test_res {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 114 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 115 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.size {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.size);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 116 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.size)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = choices_get(&mut choices, 0 as libc::c_int as size_t);
    if 0 as *mut libc::c_void as libc::c_ulong != tmp as libc::c_ulong {
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 117 as libc::c_uint;
        greatest_info
            .msg = b"NULL != choices_get(&choices, 0)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    choices_add(&mut choices, b"test\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 121 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != choices.selection {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.selection);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 122 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (choices.selection)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 1 as libc::c_ulong != choices.size {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.size);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 123 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (choices.size)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp___0 = choices_get(&mut choices, 0 as libc::c_int as size_t);
    if 0 as *mut libc::c_void as libc::c_ulong != tmp___0 as libc::c_ulong {
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 124 as libc::c_uint;
        greatest_info
            .msg = b"NULL != choices_get(&choices, 0)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn test_choices_unicode() -> greatest_test_res {
    choices_add(
        &mut choices,
        b"Edmund Husserl - M\xC3\xA9ditations cart\xC3\xA9siennes - Introduction a la ph\xC3\xA9nom\xC3\xA9nologie.pdf\0"
            as *const u8 as *const libc::c_char,
    );
    choices_search(&mut choices, b"e\0" as *const u8 as *const libc::c_char);
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn test_choices_large_input() -> greatest_test_res {
    let mut N: libc::c_int = 0;
    let mut strings: [*mut libc::c_char; 100000] = [0 as *mut libc::c_char; 100000];
    let mut i: libc::c_int = 0;
    let mut type_info: *mut greatest_type_info = 0 as *mut greatest_type_info;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    N = 100000 as libc::c_int;
    i = 0 as libc::c_int;
    while i < N {
        asprintf(
            &mut *strings.as_mut_ptr().offset(i as isize) as *mut *mut libc::c_char,
            b"%i\0" as *const u8 as *const libc::c_char,
            i,
        );
        choices_add(&mut choices, strings[i as usize] as *const libc::c_char);
        i += 1;
    }
    choices_search(&mut choices, b"12\0" as *const u8 as *const libc::c_char);
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 8146 as libc::c_ulong != choices.available {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            8146 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(stdout, b"%zu\0" as *const u8 as *const libc::c_char, choices.available);
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 149 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(8146) != (choices.available)\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    type_info = &mut greatest_type_info_string;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = choices_get(&mut choices, 0 as libc::c_int as size_t);
    tmp___0 = greatest_do_assert_equal_t(
        b"12\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        tmp as *const libc::c_void,
        type_info,
        0 as *mut libc::c_void,
    );
    if tmp___0 == 0 {
        if !(type_info as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            if !(::std::mem::transmute::<
                Option::<greatest_equal_cb>,
                libc::c_ulong,
            >((*type_info).equal) == 0 as *mut libc::c_void as libc::c_ulong)
            {
                greatest_info
                    .fail_file = b"test/test_choices.c\0" as *const u8
                    as *const libc::c_char;
                greatest_info.fail_line = 151 as libc::c_uint;
                greatest_info
                    .msg = b"\"12\" != choices_get(&choices, 0)\0" as *const u8
                    as *const libc::c_char;
                return GREATEST_TEST_RES_FAIL;
            }
        }
        greatest_info
            .fail_file = b"test/test_choices.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 151 as libc::c_uint;
        greatest_info
            .msg = b"type_info->equal callback missing!\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    i___0 = 0 as libc::c_int;
    while i___0 < N {
        free(strings[i___0 as usize] as *mut libc::c_void);
        i___0 += 1;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
pub unsafe extern "C" fn choices_suite() {
    let mut res: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut res___0: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut res___1: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut res___2: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut res___3: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut res___4: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    GREATEST_SET_SETUP_CB(
        Some(setup as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
    GREATEST_SET_TEARDOWN_CB(
        Some(teardown as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        0 as *mut libc::c_void,
    );
    tmp___0 = greatest_test_pre(
        b"test_choices_empty\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 == 1 as libc::c_int {
        tmp = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res = tmp as greatest_test_res;
        if res as libc::c_int == 0 as libc::c_int {
            res = test_choices_empty();
        }
        greatest_test_post(
            b"test_choices_empty\0" as *const u8 as *const libc::c_char,
            res as libc::c_int,
        );
    }
    tmp___2 = greatest_test_pre(b"test_choices_1\0" as *const u8 as *const libc::c_char);
    if tmp___2 == 1 as libc::c_int {
        tmp___1 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___0 = tmp___1 as greatest_test_res;
        if res___0 as libc::c_int == 0 as libc::c_int {
            res___0 = test_choices_1();
        }
        greatest_test_post(
            b"test_choices_1\0" as *const u8 as *const libc::c_char,
            res___0 as libc::c_int,
        );
    }
    tmp___4 = greatest_test_pre(b"test_choices_2\0" as *const u8 as *const libc::c_char);
    if tmp___4 == 1 as libc::c_int {
        tmp___3 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___1 = tmp___3 as greatest_test_res;
        if res___1 as libc::c_int == 0 as libc::c_int {
            res___1 = test_choices_2();
        }
        greatest_test_post(
            b"test_choices_2\0" as *const u8 as *const libc::c_char,
            res___1 as libc::c_int,
        );
    }
    tmp___6 = greatest_test_pre(
        b"test_choices_without_search\0" as *const u8 as *const libc::c_char,
    );
    if tmp___6 == 1 as libc::c_int {
        tmp___5 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___2 = tmp___5 as greatest_test_res;
        if res___2 as libc::c_int == 0 as libc::c_int {
            res___2 = test_choices_without_search();
        }
        greatest_test_post(
            b"test_choices_without_search\0" as *const u8 as *const libc::c_char,
            res___2 as libc::c_int,
        );
    }
    tmp___8 = greatest_test_pre(
        b"test_choices_unicode\0" as *const u8 as *const libc::c_char,
    );
    if tmp___8 == 1 as libc::c_int {
        tmp___7 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___3 = tmp___7 as greatest_test_res;
        if res___3 as libc::c_int == 0 as libc::c_int {
            res___3 = test_choices_unicode();
        }
        greatest_test_post(
            b"test_choices_unicode\0" as *const u8 as *const libc::c_char,
            res___3 as libc::c_int,
        );
    }
    tmp___10 = greatest_test_pre(
        b"test_choices_large_input\0" as *const u8 as *const libc::c_char,
    );
    if tmp___10 == 1 as libc::c_int {
        tmp___9 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___4 = tmp___9 as greatest_test_res;
        if res___4 as libc::c_int == 0 as libc::c_int {
            res___4 = test_choices_large_input();
        }
        greatest_test_post(
            b"test_choices_large_input\0" as *const u8 as *const libc::c_char,
            res___4 as libc::c_int,
        );
    }
}
unsafe extern "C" fn exact_match_should_return_true() -> greatest_test_res {
    let mut tmp: libc::c_int = 0;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = has_match(
        b"a\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    if tmp == 0 {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 14 as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"a\", \"a\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn partial_match_should_return_true() -> greatest_test_res {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = has_match(
        b"a\0" as *const u8 as *const libc::c_char,
        b"ab\0" as *const u8 as *const libc::c_char,
    );
    if tmp == 0 {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 19 as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"a\", \"ab\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp___0 = has_match(
        b"a\0" as *const u8 as *const libc::c_char,
        b"ba\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 == 0 {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 20 as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"a\", \"ba\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn match_with_delimiters_in_between() -> greatest_test_res {
    let mut tmp: libc::c_int = 0;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = has_match(
        b"abc\0" as *const u8 as *const libc::c_char,
        b"a|b|c\0" as *const u8 as *const libc::c_char,
    );
    if tmp == 0 {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 25 as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"abc\", \"a|b|c\")\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn non_match_should_return_false() -> greatest_test_res {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = has_match(
        b"a\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if tmp != 0 {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 30 as libc::c_uint;
        greatest_info
            .msg = b"!has_match(\"a\", \"\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp___0 = has_match(
        b"a\0" as *const u8 as *const libc::c_char,
        b"b\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 != 0 {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 31 as libc::c_uint;
        greatest_info
            .msg = b"!has_match(\"a\", \"b\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp___1 = has_match(
        b"ass\0" as *const u8 as *const libc::c_char,
        b"tags\0" as *const u8 as *const libc::c_char,
    );
    if tmp___1 != 0 {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 32 as libc::c_uint;
        greatest_info
            .msg = b"!has_match(\"ass\", \"tags\")\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn empty_query_should_always_match() -> greatest_test_res {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = has_match(
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if tmp == 0 {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 38 as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"\", \"\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp___0 = has_match(
        b"\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 == 0 {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 39 as libc::c_uint;
        greatest_info
            .msg = b"has_match(\"\", \"a\")\0" as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_starts_of_words() -> greatest_test_res {
    let mut tmp: score_t = 0.;
    let mut tmp___0: score_t = 0.;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = match_0(
        b"amor\0" as *const u8 as *const libc::c_char,
        b"app/models/order\0" as *const u8 as *const libc::c_char,
    );
    tmp___0 = match_0(
        b"amor\0" as *const u8 as *const libc::c_char,
        b"app/models/zrder\0" as *const u8 as *const libc::c_char,
    );
    if !(tmp > tmp___0) {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 47 as libc::c_uint;
        greatest_info
            .msg = b"match(\"amor\", \"app/models/order\") > match(\"amor\", \"app/models/zrder\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_consecutive_letters() -> greatest_test_res {
    let mut tmp: score_t = 0.;
    let mut tmp___0: score_t = 0.;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = match_0(
        b"amo\0" as *const u8 as *const libc::c_char,
        b"app/m/foo\0" as *const u8 as *const libc::c_char,
    );
    tmp___0 = match_0(
        b"amo\0" as *const u8 as *const libc::c_char,
        b"app/models/foo\0" as *const u8 as *const libc::c_char,
    );
    if !(tmp < tmp___0) {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 53 as libc::c_uint;
        greatest_info
            .msg = b"match(\"amo\", \"app/m/foo\") < match(\"amo\", \"app/models/foo\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_contiguous_over_letter_following_period() -> greatest_test_res {
    let mut tmp: score_t = 0.;
    let mut tmp___0: score_t = 0.;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = match_0(
        b"gemfil\0" as *const u8 as *const libc::c_char,
        b"Gemfile.lock\0" as *const u8 as *const libc::c_char,
    );
    tmp___0 = match_0(
        b"gemfil\0" as *const u8 as *const libc::c_char,
        b"Gemfile\0" as *const u8 as *const libc::c_char,
    );
    if !(tmp < tmp___0) {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 59 as libc::c_uint;
        greatest_info
            .msg = b"match(\"gemfil\", \"Gemfile.lock\") < match(\"gemfil\", \"Gemfile\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_shorter_matches() -> greatest_test_res {
    let mut tmp: score_t = 0.;
    let mut tmp___0: score_t = 0.;
    let mut tmp___1: score_t = 0.;
    let mut tmp___2: score_t = 0.;
    let mut tmp___3: score_t = 0.;
    let mut tmp___4: score_t = 0.;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = match_0(
        b"abce\0" as *const u8 as *const libc::c_char,
        b"abcdef\0" as *const u8 as *const libc::c_char,
    );
    tmp___0 = match_0(
        b"abce\0" as *const u8 as *const libc::c_char,
        b"abc de\0" as *const u8 as *const libc::c_char,
    );
    if !(tmp > tmp___0) {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 64 as libc::c_uint;
        greatest_info
            .msg = b"match(\"abce\", \"abcdef\") > match(\"abce\", \"abc de\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp___1 = match_0(
        b"abc\0" as *const u8 as *const libc::c_char,
        b"    a b c \0" as *const u8 as *const libc::c_char,
    );
    tmp___2 = match_0(
        b"abc\0" as *const u8 as *const libc::c_char,
        b" a  b  c \0" as *const u8 as *const libc::c_char,
    );
    if !(tmp___1 > tmp___2) {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 65 as libc::c_uint;
        greatest_info
            .msg = b"match(\"abc\", \"    a b c \") > match(\"abc\", \" a  b  c \")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp___3 = match_0(
        b"abc\0" as *const u8 as *const libc::c_char,
        b" a b c    \0" as *const u8 as *const libc::c_char,
    );
    tmp___4 = match_0(
        b"abc\0" as *const u8 as *const libc::c_char,
        b" a  b  c \0" as *const u8 as *const libc::c_char,
    );
    if !(tmp___3 > tmp___4) {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 66 as libc::c_uint;
        greatest_info
            .msg = b"match(\"abc\", \" a b c    \") > match(\"abc\", \" a  b  c \")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_shorter_candidates() -> greatest_test_res {
    let mut tmp: score_t = 0.;
    let mut tmp___0: score_t = 0.;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = match_0(
        b"test\0" as *const u8 as *const libc::c_char,
        b"tests\0" as *const u8 as *const libc::c_char,
    );
    tmp___0 = match_0(
        b"test\0" as *const u8 as *const libc::c_char,
        b"testing\0" as *const u8 as *const libc::c_char,
    );
    if !(tmp > tmp___0) {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 71 as libc::c_uint;
        greatest_info
            .msg = b"match(\"test\", \"tests\") > match(\"test\", \"testing\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn should_prefer_start_of_candidate() -> greatest_test_res {
    let mut tmp: score_t = 0.;
    let mut tmp___0: score_t = 0.;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    tmp = match_0(
        b"test\0" as *const u8 as *const libc::c_char,
        b"testing\0" as *const u8 as *const libc::c_char,
    );
    tmp___0 = match_0(
        b"test\0" as *const u8 as *const libc::c_char,
        b"/testing\0" as *const u8 as *const libc::c_char,
    );
    if !(tmp > tmp___0) {
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 77 as libc::c_uint;
        greatest_info
            .msg = b"match(\"test\", \"testing\") > match(\"test\", \"/testing\")\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_exact_match() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut greatest_GOT: libc::c_double = 0.;
    let mut tmp___0: score_t = 0.;
    let mut greatest_TOL: libc::c_double = 0.;
    let mut greatest_EXP___0: libc::c_double = 0.;
    let mut tmp___1: libc::c_float = 0.;
    let mut greatest_GOT___0: libc::c_double = 0.;
    let mut tmp___2: score_t = 0.;
    let mut greatest_TOL___0: libc::c_double = 0.;
    tmp = ::std::f32::INFINITY;
    greatest_EXP = tmp as libc::c_double;
    tmp___0 = match_0(
        b"abc\0" as *const u8 as *const libc::c_char,
        b"abc\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT = tmp___0;
    greatest_TOL = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_102: {
        let mut current_block_11: u64;
        if greatest_EXP > greatest_GOT {
            if greatest_EXP - greatest_GOT > greatest_TOL {
                current_block_11 = 2912852826589703590;
            } else {
                current_block_11 = 1287987036039841897;
            }
        } else {
            current_block_11 = 1287987036039841897;
        }
        match current_block_11 {
            1287987036039841897 => {
                if greatest_EXP < greatest_GOT {
                    if greatest_GOT - greatest_EXP > greatest_TOL {
                        current_block_11 = 2912852826589703590;
                    } else {
                        current_block_11 = 13797916685926291137;
                    }
                } else {
                    current_block_11 = 13797916685926291137;
                }
                match current_block_11 {
                    2912852826589703590 => {}
                    _ => {
                        break 's_102;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 83 as libc::c_uint;
        greatest_info
            .msg = b"((__builtin_inff ())) != (match(\"abc\", \"abc\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    tmp___1 = ::std::f32::INFINITY;
    greatest_EXP___0 = tmp___1 as libc::c_double;
    tmp___2 = match_0(
        b"aBc\0" as *const u8 as *const libc::c_char,
        b"abC\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___0 = tmp___2;
    greatest_TOL___0 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_189: {
        let mut current_block_25: u64;
        if greatest_EXP___0 > greatest_GOT___0 {
            if greatest_EXP___0 - greatest_GOT___0 > greatest_TOL___0 {
                current_block_25 = 11558753698618723007;
            } else {
                current_block_25 = 11909167839317650520;
            }
        } else {
            current_block_25 = 11909167839317650520;
        }
        match current_block_25 {
            11909167839317650520 => {
                if greatest_EXP___0 < greatest_GOT___0 {
                    if greatest_GOT___0 - greatest_EXP___0 > greatest_TOL___0 {
                        current_block_25 = 11558753698618723007;
                    } else {
                        current_block_25 = 8845338526596852646;
                    }
                } else {
                    current_block_25 = 8845338526596852646;
                }
                match current_block_25 {
                    11558753698618723007 => {}
                    _ => {
                        break 's_189;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___0,
            greatest_TOL___0,
            greatest_GOT___0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 84 as libc::c_uint;
        greatest_info
            .msg = b"((__builtin_inff ())) != (match(\"aBc\", \"abC\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_empty_query() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut greatest_GOT: libc::c_double = 0.;
    let mut tmp___0: score_t = 0.;
    let mut greatest_TOL: libc::c_double = 0.;
    let mut greatest_EXP___0: libc::c_double = 0.;
    let mut tmp___1: libc::c_float = 0.;
    let mut greatest_GOT___0: libc::c_double = 0.;
    let mut tmp___2: score_t = 0.;
    let mut greatest_TOL___0: libc::c_double = 0.;
    let mut greatest_EXP___1: libc::c_double = 0.;
    let mut tmp___3: libc::c_float = 0.;
    let mut greatest_GOT___1: libc::c_double = 0.;
    let mut tmp___4: score_t = 0.;
    let mut greatest_TOL___1: libc::c_double = 0.;
    tmp = ::std::f32::INFINITY;
    greatest_EXP = -tmp as libc::c_double;
    tmp___0 = match_0(
        b"\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT = tmp___0;
    greatest_TOL = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_112: {
        let mut current_block_11: u64;
        if greatest_EXP > greatest_GOT {
            if greatest_EXP - greatest_GOT > greatest_TOL {
                current_block_11 = 5890370896692070023;
            } else {
                current_block_11 = 1580227563013984454;
            }
        } else {
            current_block_11 = 1580227563013984454;
        }
        match current_block_11 {
            1580227563013984454 => {
                if greatest_EXP < greatest_GOT {
                    if greatest_GOT - greatest_EXP > greatest_TOL {
                        current_block_11 = 5890370896692070023;
                    } else {
                        current_block_11 = 2668756484064249700;
                    }
                } else {
                    current_block_11 = 2668756484064249700;
                }
                match current_block_11 {
                    5890370896692070023 => {}
                    _ => {
                        break 's_112;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 90 as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(\"\", \"\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    tmp___1 = ::std::f32::INFINITY;
    greatest_EXP___0 = -tmp___1 as libc::c_double;
    tmp___2 = match_0(
        b"\0" as *const u8 as *const libc::c_char,
        b"a\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___0 = tmp___2;
    greatest_TOL___0 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_199: {
        let mut current_block_25: u64;
        if greatest_EXP___0 > greatest_GOT___0 {
            if greatest_EXP___0 - greatest_GOT___0 > greatest_TOL___0 {
                current_block_25 = 15966807312559603041;
            } else {
                current_block_25 = 7252661072853379181;
            }
        } else {
            current_block_25 = 7252661072853379181;
        }
        match current_block_25 {
            7252661072853379181 => {
                if greatest_EXP___0 < greatest_GOT___0 {
                    if greatest_GOT___0 - greatest_EXP___0 > greatest_TOL___0 {
                        current_block_25 = 15966807312559603041;
                    } else {
                        current_block_25 = 980989089337379490;
                    }
                } else {
                    current_block_25 = 980989089337379490;
                }
                match current_block_25 {
                    15966807312559603041 => {}
                    _ => {
                        break 's_199;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___0,
            greatest_TOL___0,
            greatest_GOT___0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 91 as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(\"\", \"a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    tmp___3 = ::std::f32::INFINITY;
    greatest_EXP___1 = -tmp___3 as libc::c_double;
    tmp___4 = match_0(
        b"\0" as *const u8 as *const libc::c_char,
        b"bb\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___1 = tmp___4;
    greatest_TOL___1 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_286: {
        let mut current_block_39: u64;
        if greatest_EXP___1 > greatest_GOT___1 {
            if greatest_EXP___1 - greatest_GOT___1 > greatest_TOL___1 {
                current_block_39 = 12879406656980934819;
            } else {
                current_block_39 = 12141613568675028563;
            }
        } else {
            current_block_39 = 12141613568675028563;
        }
        match current_block_39 {
            12141613568675028563 => {
                if greatest_EXP___1 < greatest_GOT___1 {
                    if greatest_GOT___1 - greatest_EXP___1 > greatest_TOL___1 {
                        current_block_39 = 12879406656980934819;
                    } else {
                        current_block_39 = 16415152177862271243;
                    }
                } else {
                    current_block_39 = 16415152177862271243;
                }
                match current_block_39 {
                    12879406656980934819 => {}
                    _ => {
                        break 's_286;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___1,
            greatest_TOL___1,
            greatest_GOT___1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 92 as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(\"\", \"bb\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_gaps() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = 0.;
    let mut greatest_GOT: libc::c_double = 0.;
    let mut tmp: score_t = 0.;
    let mut greatest_TOL: libc::c_double = 0.;
    let mut greatest_EXP___0: libc::c_double = 0.;
    let mut greatest_GOT___0: libc::c_double = 0.;
    let mut tmp___0: score_t = 0.;
    let mut greatest_TOL___0: libc::c_double = 0.;
    let mut greatest_EXP___1: libc::c_double = 0.;
    let mut greatest_GOT___1: libc::c_double = 0.;
    let mut tmp___1: score_t = 0.;
    let mut greatest_TOL___1: libc::c_double = 0.;
    let mut greatest_EXP___2: libc::c_double = 0.;
    let mut greatest_GOT___2: libc::c_double = 0.;
    let mut tmp___2: score_t = 0.;
    let mut greatest_TOL___2: libc::c_double = 0.;
    let mut greatest_EXP___3: libc::c_double = 0.;
    let mut greatest_GOT___3: libc::c_double = 0.;
    let mut tmp___3: score_t = 0.;
    let mut greatest_TOL___3: libc::c_double = 0.;
    let mut greatest_EXP___4: libc::c_double = 0.;
    let mut greatest_GOT___4: libc::c_double = 0.;
    let mut tmp___4: score_t = 0.;
    let mut greatest_TOL___4: libc::c_double = 0.;
    greatest_EXP = -0.005f64;
    tmp = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"*a\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT = tmp;
    greatest_TOL = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_127: {
        let mut current_block_10: u64;
        if greatest_EXP > greatest_GOT {
            if greatest_EXP - greatest_GOT > greatest_TOL {
                current_block_10 = 4886383922345668843;
            } else {
                current_block_10 = 11439030827524731688;
            }
        } else {
            current_block_10 = 11439030827524731688;
        }
        match current_block_10 {
            11439030827524731688 => {
                if greatest_EXP < greatest_GOT {
                    if greatest_GOT - greatest_EXP > greatest_TOL {
                        current_block_10 = 4886383922345668843;
                    } else {
                        current_block_10 = 15768484401365413375;
                    }
                } else {
                    current_block_10 = 15768484401365413375;
                }
                match current_block_10 {
                    4886383922345668843 => {}
                    _ => {
                        break 's_127;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 97 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005) != (match(\"a\", \"*a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___0 = -0.005f64 * 2 as libc::c_int as libc::c_double;
    tmp___0 = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"*ba\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___0 = tmp___0;
    greatest_TOL___0 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_211: {
        let mut current_block_23: u64;
        if greatest_EXP___0 > greatest_GOT___0 {
            if greatest_EXP___0 - greatest_GOT___0 > greatest_TOL___0 {
                current_block_23 = 542087605492696235;
            } else {
                current_block_23 = 12936493699536942178;
            }
        } else {
            current_block_23 = 12936493699536942178;
        }
        match current_block_23 {
            12936493699536942178 => {
                if greatest_EXP___0 < greatest_GOT___0 {
                    if greatest_GOT___0 - greatest_EXP___0 > greatest_TOL___0 {
                        current_block_23 = 542087605492696235;
                    } else {
                        current_block_23 = 12199444798915819164;
                    }
                } else {
                    current_block_23 = 12199444798915819164;
                }
                match current_block_23 {
                    542087605492696235 => {}
                    _ => {
                        break 's_211;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___0,
            greatest_TOL___0,
            greatest_GOT___0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 98 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2) != (match(\"a\", \"*ba\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___1 = -0.005f64 * 2 as libc::c_int as libc::c_double + -0.005f64;
    tmp___1 = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"**a*\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___1 = tmp___1;
    greatest_TOL___1 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_295: {
        let mut current_block_36: u64;
        if greatest_EXP___1 > greatest_GOT___1 {
            if greatest_EXP___1 - greatest_GOT___1 > greatest_TOL___1 {
                current_block_36 = 348414481607958388;
            } else {
                current_block_36 = 14238804492798074475;
            }
        } else {
            current_block_36 = 14238804492798074475;
        }
        match current_block_36 {
            14238804492798074475 => {
                if greatest_EXP___1 < greatest_GOT___1 {
                    if greatest_GOT___1 - greatest_EXP___1 > greatest_TOL___1 {
                        current_block_36 = 348414481607958388;
                    } else {
                        current_block_36 = 9859671972921157070;
                    }
                } else {
                    current_block_36 = 9859671972921157070;
                }
                match current_block_36 {
                    348414481607958388 => {}
                    _ => {
                        break 's_295;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___1,
            greatest_TOL___1,
            greatest_GOT___1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 99 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + -0.005) != (match(\"a\", \"**a*\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___2 = -0.005f64 * 2 as libc::c_int as libc::c_double
        + -0.005f64 * 2 as libc::c_int as libc::c_double;
    tmp___2 = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"**a**\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___2 = tmp___2;
    greatest_TOL___2 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_379: {
        let mut current_block_49: u64;
        if greatest_EXP___2 > greatest_GOT___2 {
            if greatest_EXP___2 - greatest_GOT___2 > greatest_TOL___2 {
                current_block_49 = 17700620964354312741;
            } else {
                current_block_49 = 11031476441834017546;
            }
        } else {
            current_block_49 = 11031476441834017546;
        }
        match current_block_49 {
            11031476441834017546 => {
                if greatest_EXP___2 < greatest_GOT___2 {
                    if greatest_GOT___2 - greatest_EXP___2 > greatest_TOL___2 {
                        current_block_49 = 17700620964354312741;
                    } else {
                        current_block_49 = 3736434875406665187;
                    }
                } else {
                    current_block_49 = 3736434875406665187;
                }
                match current_block_49 {
                    17700620964354312741 => {}
                    _ => {
                        break 's_379;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___2,
            greatest_TOL___2,
            greatest_GOT___2,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 100 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + -0.005*2) != (match(\"a\", \"**a**\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___3 = -0.005f64 * 2 as libc::c_int as libc::c_double + 1.0f64
        + -0.005f64 * 2 as libc::c_int as libc::c_double;
    tmp___3 = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        b"**aa**\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___3 = tmp___3;
    greatest_TOL___3 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_463: {
        let mut current_block_62: u64;
        if greatest_EXP___3 > greatest_GOT___3 {
            if greatest_EXP___3 - greatest_GOT___3 > greatest_TOL___3 {
                current_block_62 = 858003953985153771;
            } else {
                current_block_62 = 18176699114596844322;
            }
        } else {
            current_block_62 = 18176699114596844322;
        }
        match current_block_62 {
            18176699114596844322 => {
                if greatest_EXP___3 < greatest_GOT___3 {
                    if greatest_GOT___3 - greatest_EXP___3 > greatest_TOL___3 {
                        current_block_62 = 858003953985153771;
                    } else {
                        current_block_62 = 5687667889785024198;
                    }
                } else {
                    current_block_62 = 5687667889785024198;
                }
                match current_block_62 {
                    858003953985153771 => {}
                    _ => {
                        break 's_463;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___3,
            greatest_TOL___3,
            greatest_GOT___3,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 101 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + 1.0 + -0.005*2) != (match(\"aa\", \"**aa**\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___4 = -0.005f64 + -0.005f64 + -0.01f64 + -0.005f64 + -0.005f64;
    tmp___4 = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        b"**a*a**\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___4 = tmp___4;
    greatest_TOL___4 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_547: {
        let mut current_block_75: u64;
        if greatest_EXP___4 > greatest_GOT___4 {
            if greatest_EXP___4 - greatest_GOT___4 > greatest_TOL___4 {
                current_block_75 = 9042206790921940462;
            } else {
                current_block_75 = 2876643211467120586;
            }
        } else {
            current_block_75 = 2876643211467120586;
        }
        match current_block_75 {
            2876643211467120586 => {
                if greatest_EXP___4 < greatest_GOT___4 {
                    if greatest_GOT___4 - greatest_EXP___4 > greatest_TOL___4 {
                        current_block_75 = 9042206790921940462;
                    } else {
                        current_block_75 = 4751196792806374320;
                    }
                } else {
                    current_block_75 = 4751196792806374320;
                }
                match current_block_75 {
                    9042206790921940462 => {}
                    _ => {
                        break 's_547;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___4,
            greatest_TOL___4,
            greatest_GOT___4,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 102 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + -0.005 + -0.01 + -0.005 + -0.005) != (match(\"aa\", \"**a*a**\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_consecutive() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = 0.;
    let mut greatest_GOT: libc::c_double = 0.;
    let mut tmp: score_t = 0.;
    let mut greatest_TOL: libc::c_double = 0.;
    let mut greatest_EXP___0: libc::c_double = 0.;
    let mut greatest_GOT___0: libc::c_double = 0.;
    let mut tmp___0: score_t = 0.;
    let mut greatest_TOL___0: libc::c_double = 0.;
    let mut greatest_EXP___1: libc::c_double = 0.;
    let mut greatest_GOT___1: libc::c_double = 0.;
    let mut tmp___1: score_t = 0.;
    let mut greatest_TOL___1: libc::c_double = 0.;
    greatest_EXP = -0.005f64 + 1.0f64;
    tmp = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        b"*aa\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT = tmp;
    greatest_TOL = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_103: {
        let mut current_block_10: u64;
        if greatest_EXP > greatest_GOT {
            if greatest_EXP - greatest_GOT > greatest_TOL {
                current_block_10 = 14476871134583122115;
            } else {
                current_block_10 = 12383665075275062824;
            }
        } else {
            current_block_10 = 12383665075275062824;
        }
        match current_block_10 {
            12383665075275062824 => {
                if greatest_EXP < greatest_GOT {
                    if greatest_GOT - greatest_EXP > greatest_TOL {
                        current_block_10 = 14476871134583122115;
                    } else {
                        current_block_10 = 4808432441040389987;
                    }
                } else {
                    current_block_10 = 4808432441040389987;
                }
                match current_block_10 {
                    14476871134583122115 => {}
                    _ => {
                        break 's_103;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 107 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + 1.0) != (match(\"aa\", \"*aa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___0 = -0.005f64 + 1.0f64 * 2 as libc::c_int as libc::c_double;
    tmp___0 = match_0(
        b"aaa\0" as *const u8 as *const libc::c_char,
        b"*aaa\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___0 = tmp___0;
    greatest_TOL___0 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_187: {
        let mut current_block_23: u64;
        if greatest_EXP___0 > greatest_GOT___0 {
            if greatest_EXP___0 - greatest_GOT___0 > greatest_TOL___0 {
                current_block_23 = 15500952090147633589;
            } else {
                current_block_23 = 17972742153957905975;
            }
        } else {
            current_block_23 = 17972742153957905975;
        }
        match current_block_23 {
            17972742153957905975 => {
                if greatest_EXP___0 < greatest_GOT___0 {
                    if greatest_GOT___0 - greatest_EXP___0 > greatest_TOL___0 {
                        current_block_23 = 15500952090147633589;
                    } else {
                        current_block_23 = 8180496224585318153;
                    }
                } else {
                    current_block_23 = 8180496224585318153;
                }
                match current_block_23 {
                    15500952090147633589 => {}
                    _ => {
                        break 's_187;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___0,
            greatest_TOL___0,
            greatest_GOT___0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 108 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + 1.0*2) != (match(\"aaa\", \"*aaa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___1 = -0.005f64 + -0.01f64 + 1.0f64;
    tmp___1 = match_0(
        b"aaa\0" as *const u8 as *const libc::c_char,
        b"*a*aa\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___1 = tmp___1;
    greatest_TOL___1 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_271: {
        let mut current_block_36: u64;
        if greatest_EXP___1 > greatest_GOT___1 {
            if greatest_EXP___1 - greatest_GOT___1 > greatest_TOL___1 {
                current_block_36 = 90223242765313234;
            } else {
                current_block_36 = 12305032772382436835;
            }
        } else {
            current_block_36 = 12305032772382436835;
        }
        match current_block_36 {
            12305032772382436835 => {
                if greatest_EXP___1 < greatest_GOT___1 {
                    if greatest_GOT___1 - greatest_EXP___1 > greatest_TOL___1 {
                        current_block_36 = 90223242765313234;
                    } else {
                        current_block_36 = 1724319918354933278;
                    }
                } else {
                    current_block_36 = 1724319918354933278;
                }
                match current_block_36 {
                    90223242765313234 => {}
                    _ => {
                        break 's_271;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___1,
            greatest_TOL___1,
            greatest_GOT___1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 109 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + -0.01 + 1.0) != (match(\"aaa\", \"*a*aa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_slash() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = 0.;
    let mut greatest_GOT: libc::c_double = 0.;
    let mut tmp: score_t = 0.;
    let mut greatest_TOL: libc::c_double = 0.;
    let mut greatest_EXP___0: libc::c_double = 0.;
    let mut greatest_GOT___0: libc::c_double = 0.;
    let mut tmp___0: score_t = 0.;
    let mut greatest_TOL___0: libc::c_double = 0.;
    let mut greatest_EXP___1: libc::c_double = 0.;
    let mut greatest_GOT___1: libc::c_double = 0.;
    let mut tmp___1: score_t = 0.;
    let mut greatest_TOL___1: libc::c_double = 0.;
    greatest_EXP = -0.005f64 + 0.9f64;
    tmp = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"/a\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT = tmp;
    greatest_TOL = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_103: {
        let mut current_block_10: u64;
        if greatest_EXP > greatest_GOT {
            if greatest_EXP - greatest_GOT > greatest_TOL {
                current_block_10 = 11482696289284552327;
            } else {
                current_block_10 = 8992702653393914899;
            }
        } else {
            current_block_10 = 8992702653393914899;
        }
        match current_block_10 {
            8992702653393914899 => {
                if greatest_EXP < greatest_GOT {
                    if greatest_GOT - greatest_EXP > greatest_TOL {
                        current_block_10 = 11482696289284552327;
                    } else {
                        current_block_10 = 4808432441040389987;
                    }
                } else {
                    current_block_10 = 4808432441040389987;
                }
                match current_block_10 {
                    11482696289284552327 => {}
                    _ => {
                        break 's_103;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 114 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + 0.9) != (match(\"a\", \"/a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___0 = -0.005f64 * 2 as libc::c_int as libc::c_double + 0.9f64;
    tmp___0 = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"*/a\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___0 = tmp___0;
    greatest_TOL___0 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_187: {
        let mut current_block_23: u64;
        if greatest_EXP___0 > greatest_GOT___0 {
            if greatest_EXP___0 - greatest_GOT___0 > greatest_TOL___0 {
                current_block_23 = 8596694028928804531;
            } else {
                current_block_23 = 8112792337107201250;
            }
        } else {
            current_block_23 = 8112792337107201250;
        }
        match current_block_23 {
            8112792337107201250 => {
                if greatest_EXP___0 < greatest_GOT___0 {
                    if greatest_GOT___0 - greatest_EXP___0 > greatest_TOL___0 {
                        current_block_23 = 8596694028928804531;
                    } else {
                        current_block_23 = 8180496224585318153;
                    }
                } else {
                    current_block_23 = 8180496224585318153;
                }
                match current_block_23 {
                    8596694028928804531 => {}
                    _ => {
                        break 's_187;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___0,
            greatest_TOL___0,
            greatest_GOT___0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 115 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + 0.9) != (match(\"a\", \"*/a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___1 = -0.005f64 * 2 as libc::c_int as libc::c_double + 0.9f64 + 1.0f64;
    tmp___1 = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        b"a/aa\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___1 = tmp___1;
    greatest_TOL___1 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_271: {
        let mut current_block_36: u64;
        if greatest_EXP___1 > greatest_GOT___1 {
            if greatest_EXP___1 - greatest_GOT___1 > greatest_TOL___1 {
                current_block_36 = 15786185339583955218;
            } else {
                current_block_36 = 6399959805796044092;
            }
        } else {
            current_block_36 = 6399959805796044092;
        }
        match current_block_36 {
            6399959805796044092 => {
                if greatest_EXP___1 < greatest_GOT___1 {
                    if greatest_GOT___1 - greatest_EXP___1 > greatest_TOL___1 {
                        current_block_36 = 15786185339583955218;
                    } else {
                        current_block_36 = 1724319918354933278;
                    }
                } else {
                    current_block_36 = 1724319918354933278;
                }
                match current_block_36 {
                    15786185339583955218 => {}
                    _ => {
                        break 's_271;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___1,
            greatest_TOL___1,
            greatest_GOT___1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 116 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + 0.9 + 1.0) != (match(\"aa\", \"a/aa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_capital() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = 0.;
    let mut greatest_GOT: libc::c_double = 0.;
    let mut tmp: score_t = 0.;
    let mut greatest_TOL: libc::c_double = 0.;
    let mut greatest_EXP___0: libc::c_double = 0.;
    let mut greatest_GOT___0: libc::c_double = 0.;
    let mut tmp___0: score_t = 0.;
    let mut greatest_TOL___0: libc::c_double = 0.;
    let mut greatest_EXP___1: libc::c_double = 0.;
    let mut greatest_GOT___1: libc::c_double = 0.;
    let mut tmp___1: score_t = 0.;
    let mut greatest_TOL___1: libc::c_double = 0.;
    greatest_EXP = -0.005f64 + 0.7f64;
    tmp = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"bA\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT = tmp;
    greatest_TOL = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_103: {
        let mut current_block_10: u64;
        if greatest_EXP > greatest_GOT {
            if greatest_EXP - greatest_GOT > greatest_TOL {
                current_block_10 = 3843682600566272439;
            } else {
                current_block_10 = 3328521707790564226;
            }
        } else {
            current_block_10 = 3328521707790564226;
        }
        match current_block_10 {
            3328521707790564226 => {
                if greatest_EXP < greatest_GOT {
                    if greatest_GOT - greatest_EXP > greatest_TOL {
                        current_block_10 = 3843682600566272439;
                    } else {
                        current_block_10 = 4808432441040389987;
                    }
                } else {
                    current_block_10 = 4808432441040389987;
                }
                match current_block_10 {
                    3843682600566272439 => {}
                    _ => {
                        break 's_103;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 121 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + 0.7) != (match(\"a\", \"bA\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___0 = -0.005f64 * 2 as libc::c_int as libc::c_double + 0.7f64;
    tmp___0 = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"baA\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___0 = tmp___0;
    greatest_TOL___0 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_187: {
        let mut current_block_23: u64;
        if greatest_EXP___0 > greatest_GOT___0 {
            if greatest_EXP___0 - greatest_GOT___0 > greatest_TOL___0 {
                current_block_23 = 15478861190448192315;
            } else {
                current_block_23 = 8921855888538883675;
            }
        } else {
            current_block_23 = 8921855888538883675;
        }
        match current_block_23 {
            8921855888538883675 => {
                if greatest_EXP___0 < greatest_GOT___0 {
                    if greatest_GOT___0 - greatest_EXP___0 > greatest_TOL___0 {
                        current_block_23 = 15478861190448192315;
                    } else {
                        current_block_23 = 8180496224585318153;
                    }
                } else {
                    current_block_23 = 8180496224585318153;
                }
                match current_block_23 {
                    15478861190448192315 => {}
                    _ => {
                        break 's_187;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___0,
            greatest_TOL___0,
            greatest_GOT___0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 122 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + 0.7) != (match(\"a\", \"baA\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___1 = -0.005f64 * 2 as libc::c_int as libc::c_double + 0.7f64 + 1.0f64;
    tmp___1 = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        b"baAa\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___1 = tmp___1;
    greatest_TOL___1 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_271: {
        let mut current_block_36: u64;
        if greatest_EXP___1 > greatest_GOT___1 {
            if greatest_EXP___1 - greatest_GOT___1 > greatest_TOL___1 {
                current_block_36 = 10783262605118383410;
            } else {
                current_block_36 = 15499895622621926141;
            }
        } else {
            current_block_36 = 15499895622621926141;
        }
        match current_block_36 {
            15499895622621926141 => {
                if greatest_EXP___1 < greatest_GOT___1 {
                    if greatest_GOT___1 - greatest_EXP___1 > greatest_TOL___1 {
                        current_block_36 = 10783262605118383410;
                    } else {
                        current_block_36 = 1724319918354933278;
                    }
                } else {
                    current_block_36 = 1724319918354933278;
                }
                match current_block_36 {
                    10783262605118383410 => {}
                    _ => {
                        break 's_271;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___1,
            greatest_TOL___1,
            greatest_GOT___1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 123 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*2 + 0.7 + 1.0) != (match(\"aa\", \"baAa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_dot() -> greatest_test_res {
    let mut greatest_EXP: libc::c_double = 0.;
    let mut greatest_GOT: libc::c_double = 0.;
    let mut tmp: score_t = 0.;
    let mut greatest_TOL: libc::c_double = 0.;
    let mut greatest_EXP___0: libc::c_double = 0.;
    let mut greatest_GOT___0: libc::c_double = 0.;
    let mut tmp___0: score_t = 0.;
    let mut greatest_TOL___0: libc::c_double = 0.;
    let mut greatest_EXP___1: libc::c_double = 0.;
    let mut greatest_GOT___1: libc::c_double = 0.;
    let mut tmp___1: score_t = 0.;
    let mut greatest_TOL___1: libc::c_double = 0.;
    greatest_EXP = -0.005f64 + 0.6f64;
    tmp = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b".a\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT = tmp;
    greatest_TOL = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_103: {
        let mut current_block_10: u64;
        if greatest_EXP > greatest_GOT {
            if greatest_EXP - greatest_GOT > greatest_TOL {
                current_block_10 = 11932186269143546039;
            } else {
                current_block_10 = 749547298679088599;
            }
        } else {
            current_block_10 = 749547298679088599;
        }
        match current_block_10 {
            749547298679088599 => {
                if greatest_EXP < greatest_GOT {
                    if greatest_GOT - greatest_EXP > greatest_TOL {
                        current_block_10 = 11932186269143546039;
                    } else {
                        current_block_10 = 4808432441040389987;
                    }
                } else {
                    current_block_10 = 4808432441040389987;
                }
                match current_block_10 {
                    11932186269143546039 => {}
                    _ => {
                        break 's_103;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 128 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + 0.6) != (match(\"a\", \".a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___0 = -0.005f64 * 3 as libc::c_int as libc::c_double + 0.6f64;
    tmp___0 = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"*a.a\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___0 = tmp___0;
    greatest_TOL___0 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_187: {
        let mut current_block_23: u64;
        if greatest_EXP___0 > greatest_GOT___0 {
            if greatest_EXP___0 - greatest_GOT___0 > greatest_TOL___0 {
                current_block_23 = 12358785892614131969;
            } else {
                current_block_23 = 16506781290394243242;
            }
        } else {
            current_block_23 = 16506781290394243242;
        }
        match current_block_23 {
            16506781290394243242 => {
                if greatest_EXP___0 < greatest_GOT___0 {
                    if greatest_GOT___0 - greatest_EXP___0 > greatest_TOL___0 {
                        current_block_23 = 12358785892614131969;
                    } else {
                        current_block_23 = 8180496224585318153;
                    }
                } else {
                    current_block_23 = 8180496224585318153;
                }
                match current_block_23 {
                    12358785892614131969 => {}
                    _ => {
                        break 's_187;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___0,
            greatest_TOL___0,
            greatest_GOT___0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 129 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005*3 + 0.6) != (match(\"a\", \"*a.a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_EXP___1 = -0.005f64 + -0.01f64 + 0.6f64;
    tmp___1 = match_0(
        b"a\0" as *const u8 as *const libc::c_char,
        b"*a.a\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___1 = tmp___1;
    greatest_TOL___1 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_271: {
        let mut current_block_36: u64;
        if greatest_EXP___1 > greatest_GOT___1 {
            if greatest_EXP___1 - greatest_GOT___1 > greatest_TOL___1 {
                current_block_36 = 13501966108672989504;
            } else {
                current_block_36 = 1843372554378388235;
            }
        } else {
            current_block_36 = 1843372554378388235;
        }
        match current_block_36 {
            1843372554378388235 => {
                if greatest_EXP___1 < greatest_GOT___1 {
                    if greatest_GOT___1 - greatest_EXP___1 > greatest_TOL___1 {
                        current_block_36 = 13501966108672989504;
                    } else {
                        current_block_36 = 1724319918354933278;
                    }
                } else {
                    current_block_36 = 1724319918354933278;
                }
                match current_block_36 {
                    13501966108672989504 => {}
                    _ => {
                        break 's_271;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___1,
            greatest_TOL___1,
            greatest_GOT___1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 130 as libc::c_uint;
        greatest_info
            .msg = b"(-0.005 + -0.01 + 0.6) != (match(\"a\", \"*a.a\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn score_long_string() -> greatest_test_res {
    let mut string: [libc::c_char; 4096] = [0; 4096];
    let mut greatest_EXP: libc::c_double = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut greatest_GOT: libc::c_double = 0.;
    let mut tmp___0: score_t = 0.;
    let mut greatest_TOL: libc::c_double = 0.;
    let mut greatest_EXP___0: libc::c_double = 0.;
    let mut tmp___1: libc::c_float = 0.;
    let mut greatest_GOT___0: libc::c_double = 0.;
    let mut tmp___2: score_t = 0.;
    let mut greatest_TOL___0: libc::c_double = 0.;
    let mut greatest_EXP___1: libc::c_double = 0.;
    let mut tmp___3: libc::c_float = 0.;
    let mut greatest_GOT___1: libc::c_double = 0.;
    let mut tmp___4: score_t = 0.;
    let mut greatest_TOL___1: libc::c_double = 0.;
    memset(
        string.as_mut_ptr() as *mut libc::c_void,
        'a' as i32,
        (::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong),
    );
    string[(::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
    tmp = ::std::f32::INFINITY;
    greatest_EXP = -tmp as libc::c_double;
    tmp___0 = match_0(
        b"aa\0" as *const u8 as *const libc::c_char,
        string.as_mut_ptr() as *const libc::c_char,
    );
    greatest_GOT = tmp___0;
    greatest_TOL = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_120: {
        let mut current_block_13: u64;
        if greatest_EXP > greatest_GOT {
            if greatest_EXP - greatest_GOT > greatest_TOL {
                current_block_13 = 11109754133707481537;
            } else {
                current_block_13 = 9368068004608096162;
            }
        } else {
            current_block_13 = 9368068004608096162;
        }
        match current_block_13 {
            9368068004608096162 => {
                if greatest_EXP < greatest_GOT {
                    if greatest_GOT - greatest_EXP > greatest_TOL {
                        current_block_13 = 11109754133707481537;
                    } else {
                        current_block_13 = 11298138898191919651;
                    }
                } else {
                    current_block_13 = 11298138898191919651;
                }
                match current_block_13 {
                    11109754133707481537 => {}
                    _ => {
                        break 's_120;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP,
            greatest_TOL,
            greatest_GOT,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 139 as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(\"aa\", string)) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    tmp___1 = ::std::f32::INFINITY;
    greatest_EXP___0 = -tmp___1 as libc::c_double;
    tmp___2 = match_0(
        string.as_mut_ptr() as *const libc::c_char,
        b"aa\0" as *const u8 as *const libc::c_char,
    );
    greatest_GOT___0 = tmp___2;
    greatest_TOL___0 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_207: {
        let mut current_block_27: u64;
        if greatest_EXP___0 > greatest_GOT___0 {
            if greatest_EXP___0 - greatest_GOT___0 > greatest_TOL___0 {
                current_block_27 = 2164299676237793539;
            } else {
                current_block_27 = 8361761665238631918;
            }
        } else {
            current_block_27 = 8361761665238631918;
        }
        match current_block_27 {
            8361761665238631918 => {
                if greatest_EXP___0 < greatest_GOT___0 {
                    if greatest_GOT___0 - greatest_EXP___0 > greatest_TOL___0 {
                        current_block_27 = 2164299676237793539;
                    } else {
                        current_block_27 = 10891380440665537214;
                    }
                } else {
                    current_block_27 = 10891380440665537214;
                }
                match current_block_27 {
                    2164299676237793539 => {}
                    _ => {
                        break 's_207;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___0,
            greatest_TOL___0,
            greatest_GOT___0,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 140 as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(string, \"aa\")) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    tmp___3 = ::std::f32::INFINITY;
    greatest_EXP___1 = -tmp___3 as libc::c_double;
    tmp___4 = match_0(
        string.as_mut_ptr() as *const libc::c_char,
        string.as_mut_ptr() as *const libc::c_char,
    );
    greatest_GOT___1 = tmp___4;
    greatest_TOL___1 = 0.000001f64;
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    's_294: {
        let mut current_block_41: u64;
        if greatest_EXP___1 > greatest_GOT___1 {
            if greatest_EXP___1 - greatest_GOT___1 > greatest_TOL___1 {
                current_block_41 = 14291470892977124624;
            } else {
                current_block_41 = 18426425189023435072;
            }
        } else {
            current_block_41 = 18426425189023435072;
        }
        match current_block_41 {
            18426425189023435072 => {
                if greatest_EXP___1 < greatest_GOT___1 {
                    if greatest_GOT___1 - greatest_EXP___1 > greatest_TOL___1 {
                        current_block_41 = 14291470892977124624;
                    } else {
                        current_block_41 = 6560072651652764009;
                    }
                } else {
                    current_block_41 = 6560072651652764009;
                }
                match current_block_41 {
                    14291470892977124624 => {}
                    _ => {
                        break 's_294;
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stdout,
            b"\nExpected: %g +/- %g\n     Got: %g\n\0" as *const u8
                as *const libc::c_char,
            greatest_EXP___1,
            greatest_TOL___1,
            greatest_GOT___1,
        );
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 141 as libc::c_uint;
        greatest_info
            .msg = b"(-(__builtin_inff ())) != (match(string, string)) +/- SCORE_TOLERANCE\0"
            as *const u8 as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn positions_consecutive() -> greatest_test_res {
    let mut positions: [size_t; 3] = [0; 3];
    match_positions(
        b"amo\0" as *const u8 as *const libc::c_char,
        b"app/models/foo\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 149 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 4 as libc::c_ulong != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 150 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(4) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 5 as libc::c_ulong != positions[2 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[2 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 151 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(5) != (positions[2])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn positions_start_of_word() -> greatest_test_res {
    let mut positions: [size_t; 4] = [0; 4];
    match_positions(
        b"amor\0" as *const u8 as *const libc::c_char,
        b"app/models/order\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 163 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 4 as libc::c_ulong != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 164 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(4) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 11 as libc::c_ulong != positions[2 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            11 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[2 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 165 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(11) != (positions[2])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 12 as libc::c_ulong != positions[3 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[3 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 166 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(12) != (positions[3])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn positions_no_bonuses() -> greatest_test_res {
    let mut positions: [size_t; 2] = [0; 2];
    match_positions(
        b"as\0" as *const u8 as *const libc::c_char,
        b"tags\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 1 as libc::c_ulong != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 174 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 3 as libc::c_ulong != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 175 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(3) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    match_positions(
        b"as\0" as *const u8 as *const libc::c_char,
        b"examples.txt\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 2 as libc::c_ulong != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 178 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(2) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 7 as libc::c_ulong != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 179 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(7) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn positions_multiple_candidates_start_of_words() -> greatest_test_res {
    let mut positions: [size_t; 3] = [0; 3];
    match_positions(
        b"abc\0" as *const u8 as *const libc::c_char,
        b"a/a/b/c/c\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 2 as libc::c_ulong != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 187 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(2) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 4 as libc::c_ulong != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 188 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(4) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 6 as libc::c_ulong != positions[2 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[2 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 189 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(6) != (positions[2])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
unsafe extern "C" fn positions_exact_match() -> greatest_test_res {
    let mut positions: [size_t; 3] = [0; 3];
    match_positions(
        b"foo\0" as *const u8 as *const libc::c_char,
        b"foo\0" as *const u8 as *const libc::c_char,
        positions.as_mut_ptr(),
    );
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 0 as libc::c_ulong != positions[0 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[0 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 197 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(0) != (positions[0])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 1 as libc::c_ulong != positions[1 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[1 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 198 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(1) != (positions[1])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.assertions = (greatest_info.assertions).wrapping_add(1);
    if 2 as libc::c_ulong != positions[2 as libc::c_int as usize] {
        fprintf(stdout, b"\nExpected: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        fprintf(stdout, b"\n     Got: \0" as *const u8 as *const libc::c_char);
        fprintf(
            stdout,
            b"%zu\0" as *const u8 as *const libc::c_char,
            positions[2 as libc::c_int as usize],
        );
        fprintf(stdout, b"\n\0" as *const u8 as *const libc::c_char);
        greatest_info
            .fail_file = b"test/test_match.c\0" as *const u8 as *const libc::c_char;
        greatest_info.fail_line = 199 as libc::c_uint;
        greatest_info
            .msg = b"(size_t)(2) != (positions[2])\0" as *const u8
            as *const libc::c_char;
        return GREATEST_TEST_RES_FAIL;
    }
    greatest_info.msg = 0 as *mut libc::c_void as *const libc::c_char;
    return GREATEST_TEST_RES_PASS;
}
pub unsafe extern "C" fn match_suite() {
    let mut res: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut res___0: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut res___1: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut res___2: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut res___3: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut res___4: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut res___5: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut res___6: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut res___7: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut res___8: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___17: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut res___9: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___20: libc::c_int = 0;
    let mut res___10: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_int = 0;
    let mut res___11: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___24: libc::c_int = 0;
    let mut res___12: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___25: libc::c_int = 0;
    let mut tmp___26: libc::c_int = 0;
    let mut res___13: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___27: libc::c_int = 0;
    let mut tmp___28: libc::c_int = 0;
    let mut res___14: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___29: libc::c_int = 0;
    let mut tmp___30: libc::c_int = 0;
    let mut res___15: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: libc::c_int = 0;
    let mut res___16: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___34: libc::c_int = 0;
    let mut res___17: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___35: libc::c_int = 0;
    let mut tmp___36: libc::c_int = 0;
    let mut res___18: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___37: libc::c_int = 0;
    let mut tmp___38: libc::c_int = 0;
    let mut res___19: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___39: libc::c_int = 0;
    let mut tmp___40: libc::c_int = 0;
    let mut res___20: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___41: libc::c_int = 0;
    let mut tmp___42: libc::c_int = 0;
    let mut res___21: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___43: libc::c_int = 0;
    let mut tmp___44: libc::c_int = 0;
    let mut res___22: greatest_test_res = GREATEST_TEST_RES_PASS;
    let mut tmp___45: libc::c_int = 0;
    let mut tmp___46: libc::c_int = 0;
    tmp___0 = greatest_test_pre(
        b"exact_match_should_return_true\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 == 1 as libc::c_int {
        tmp = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res = tmp as greatest_test_res;
        if res as libc::c_int == 0 as libc::c_int {
            res = exact_match_should_return_true();
        }
        greatest_test_post(
            b"exact_match_should_return_true\0" as *const u8 as *const libc::c_char,
            res as libc::c_int,
        );
    }
    tmp___2 = greatest_test_pre(
        b"partial_match_should_return_true\0" as *const u8 as *const libc::c_char,
    );
    if tmp___2 == 1 as libc::c_int {
        tmp___1 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___0 = tmp___1 as greatest_test_res;
        if res___0 as libc::c_int == 0 as libc::c_int {
            res___0 = partial_match_should_return_true();
        }
        greatest_test_post(
            b"partial_match_should_return_true\0" as *const u8 as *const libc::c_char,
            res___0 as libc::c_int,
        );
    }
    tmp___4 = greatest_test_pre(
        b"empty_query_should_always_match\0" as *const u8 as *const libc::c_char,
    );
    if tmp___4 == 1 as libc::c_int {
        tmp___3 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___1 = tmp___3 as greatest_test_res;
        if res___1 as libc::c_int == 0 as libc::c_int {
            res___1 = empty_query_should_always_match();
        }
        greatest_test_post(
            b"empty_query_should_always_match\0" as *const u8 as *const libc::c_char,
            res___1 as libc::c_int,
        );
    }
    tmp___6 = greatest_test_pre(
        b"non_match_should_return_false\0" as *const u8 as *const libc::c_char,
    );
    if tmp___6 == 1 as libc::c_int {
        tmp___5 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___2 = tmp___5 as greatest_test_res;
        if res___2 as libc::c_int == 0 as libc::c_int {
            res___2 = non_match_should_return_false();
        }
        greatest_test_post(
            b"non_match_should_return_false\0" as *const u8 as *const libc::c_char,
            res___2 as libc::c_int,
        );
    }
    tmp___8 = greatest_test_pre(
        b"match_with_delimiters_in_between\0" as *const u8 as *const libc::c_char,
    );
    if tmp___8 == 1 as libc::c_int {
        tmp___7 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___3 = tmp___7 as greatest_test_res;
        if res___3 as libc::c_int == 0 as libc::c_int {
            res___3 = match_with_delimiters_in_between();
        }
        greatest_test_post(
            b"match_with_delimiters_in_between\0" as *const u8 as *const libc::c_char,
            res___3 as libc::c_int,
        );
    }
    tmp___10 = greatest_test_pre(
        b"should_prefer_starts_of_words\0" as *const u8 as *const libc::c_char,
    );
    if tmp___10 == 1 as libc::c_int {
        tmp___9 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___4 = tmp___9 as greatest_test_res;
        if res___4 as libc::c_int == 0 as libc::c_int {
            res___4 = should_prefer_starts_of_words();
        }
        greatest_test_post(
            b"should_prefer_starts_of_words\0" as *const u8 as *const libc::c_char,
            res___4 as libc::c_int,
        );
    }
    tmp___12 = greatest_test_pre(
        b"should_prefer_consecutive_letters\0" as *const u8 as *const libc::c_char,
    );
    if tmp___12 == 1 as libc::c_int {
        tmp___11 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___5 = tmp___11 as greatest_test_res;
        if res___5 as libc::c_int == 0 as libc::c_int {
            res___5 = should_prefer_consecutive_letters();
        }
        greatest_test_post(
            b"should_prefer_consecutive_letters\0" as *const u8 as *const libc::c_char,
            res___5 as libc::c_int,
        );
    }
    tmp___14 = greatest_test_pre(
        b"should_prefer_contiguous_over_letter_following_period\0" as *const u8
            as *const libc::c_char,
    );
    if tmp___14 == 1 as libc::c_int {
        tmp___13 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___6 = tmp___13 as greatest_test_res;
        if res___6 as libc::c_int == 0 as libc::c_int {
            res___6 = should_prefer_contiguous_over_letter_following_period();
        }
        greatest_test_post(
            b"should_prefer_contiguous_over_letter_following_period\0" as *const u8
                as *const libc::c_char,
            res___6 as libc::c_int,
        );
    }
    tmp___16 = greatest_test_pre(
        b"should_prefer_shorter_matches\0" as *const u8 as *const libc::c_char,
    );
    if tmp___16 == 1 as libc::c_int {
        tmp___15 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___7 = tmp___15 as greatest_test_res;
        if res___7 as libc::c_int == 0 as libc::c_int {
            res___7 = should_prefer_shorter_matches();
        }
        greatest_test_post(
            b"should_prefer_shorter_matches\0" as *const u8 as *const libc::c_char,
            res___7 as libc::c_int,
        );
    }
    tmp___18 = greatest_test_pre(
        b"should_prefer_shorter_candidates\0" as *const u8 as *const libc::c_char,
    );
    if tmp___18 == 1 as libc::c_int {
        tmp___17 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___8 = tmp___17 as greatest_test_res;
        if res___8 as libc::c_int == 0 as libc::c_int {
            res___8 = should_prefer_shorter_candidates();
        }
        greatest_test_post(
            b"should_prefer_shorter_candidates\0" as *const u8 as *const libc::c_char,
            res___8 as libc::c_int,
        );
    }
    tmp___20 = greatest_test_pre(
        b"should_prefer_start_of_candidate\0" as *const u8 as *const libc::c_char,
    );
    if tmp___20 == 1 as libc::c_int {
        tmp___19 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___9 = tmp___19 as greatest_test_res;
        if res___9 as libc::c_int == 0 as libc::c_int {
            res___9 = should_prefer_start_of_candidate();
        }
        greatest_test_post(
            b"should_prefer_start_of_candidate\0" as *const u8 as *const libc::c_char,
            res___9 as libc::c_int,
        );
    }
    tmp___22 = greatest_test_pre(
        b"score_exact_match\0" as *const u8 as *const libc::c_char,
    );
    if tmp___22 == 1 as libc::c_int {
        tmp___21 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___10 = tmp___21 as greatest_test_res;
        if res___10 as libc::c_int == 0 as libc::c_int {
            res___10 = score_exact_match();
        }
        greatest_test_post(
            b"score_exact_match\0" as *const u8 as *const libc::c_char,
            res___10 as libc::c_int,
        );
    }
    tmp___24 = greatest_test_pre(
        b"score_empty_query\0" as *const u8 as *const libc::c_char,
    );
    if tmp___24 == 1 as libc::c_int {
        tmp___23 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___11 = tmp___23 as greatest_test_res;
        if res___11 as libc::c_int == 0 as libc::c_int {
            res___11 = score_empty_query();
        }
        greatest_test_post(
            b"score_empty_query\0" as *const u8 as *const libc::c_char,
            res___11 as libc::c_int,
        );
    }
    tmp___26 = greatest_test_pre(b"score_gaps\0" as *const u8 as *const libc::c_char);
    if tmp___26 == 1 as libc::c_int {
        tmp___25 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___12 = tmp___25 as greatest_test_res;
        if res___12 as libc::c_int == 0 as libc::c_int {
            res___12 = score_gaps();
        }
        greatest_test_post(
            b"score_gaps\0" as *const u8 as *const libc::c_char,
            res___12 as libc::c_int,
        );
    }
    tmp___28 = greatest_test_pre(
        b"score_consecutive\0" as *const u8 as *const libc::c_char,
    );
    if tmp___28 == 1 as libc::c_int {
        tmp___27 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___13 = tmp___27 as greatest_test_res;
        if res___13 as libc::c_int == 0 as libc::c_int {
            res___13 = score_consecutive();
        }
        greatest_test_post(
            b"score_consecutive\0" as *const u8 as *const libc::c_char,
            res___13 as libc::c_int,
        );
    }
    tmp___30 = greatest_test_pre(b"score_slash\0" as *const u8 as *const libc::c_char);
    if tmp___30 == 1 as libc::c_int {
        tmp___29 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___14 = tmp___29 as greatest_test_res;
        if res___14 as libc::c_int == 0 as libc::c_int {
            res___14 = score_slash();
        }
        greatest_test_post(
            b"score_slash\0" as *const u8 as *const libc::c_char,
            res___14 as libc::c_int,
        );
    }
    tmp___32 = greatest_test_pre(b"score_capital\0" as *const u8 as *const libc::c_char);
    if tmp___32 == 1 as libc::c_int {
        tmp___31 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___15 = tmp___31 as greatest_test_res;
        if res___15 as libc::c_int == 0 as libc::c_int {
            res___15 = score_capital();
        }
        greatest_test_post(
            b"score_capital\0" as *const u8 as *const libc::c_char,
            res___15 as libc::c_int,
        );
    }
    tmp___34 = greatest_test_pre(b"score_dot\0" as *const u8 as *const libc::c_char);
    if tmp___34 == 1 as libc::c_int {
        tmp___33 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___16 = tmp___33 as greatest_test_res;
        if res___16 as libc::c_int == 0 as libc::c_int {
            res___16 = score_dot();
        }
        greatest_test_post(
            b"score_dot\0" as *const u8 as *const libc::c_char,
            res___16 as libc::c_int,
        );
    }
    tmp___36 = greatest_test_pre(
        b"score_long_string\0" as *const u8 as *const libc::c_char,
    );
    if tmp___36 == 1 as libc::c_int {
        tmp___35 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___17 = tmp___35 as greatest_test_res;
        if res___17 as libc::c_int == 0 as libc::c_int {
            res___17 = score_long_string();
        }
        greatest_test_post(
            b"score_long_string\0" as *const u8 as *const libc::c_char,
            res___17 as libc::c_int,
        );
    }
    tmp___38 = greatest_test_pre(
        b"positions_consecutive\0" as *const u8 as *const libc::c_char,
    );
    if tmp___38 == 1 as libc::c_int {
        tmp___37 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___18 = tmp___37 as greatest_test_res;
        if res___18 as libc::c_int == 0 as libc::c_int {
            res___18 = positions_consecutive();
        }
        greatest_test_post(
            b"positions_consecutive\0" as *const u8 as *const libc::c_char,
            res___18 as libc::c_int,
        );
    }
    tmp___40 = greatest_test_pre(
        b"positions_start_of_word\0" as *const u8 as *const libc::c_char,
    );
    if tmp___40 == 1 as libc::c_int {
        tmp___39 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___19 = tmp___39 as greatest_test_res;
        if res___19 as libc::c_int == 0 as libc::c_int {
            res___19 = positions_start_of_word();
        }
        greatest_test_post(
            b"positions_start_of_word\0" as *const u8 as *const libc::c_char,
            res___19 as libc::c_int,
        );
    }
    tmp___42 = greatest_test_pre(
        b"positions_no_bonuses\0" as *const u8 as *const libc::c_char,
    );
    if tmp___42 == 1 as libc::c_int {
        tmp___41 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___20 = tmp___41 as greatest_test_res;
        if res___20 as libc::c_int == 0 as libc::c_int {
            res___20 = positions_no_bonuses();
        }
        greatest_test_post(
            b"positions_no_bonuses\0" as *const u8 as *const libc::c_char,
            res___20 as libc::c_int,
        );
    }
    tmp___44 = greatest_test_pre(
        b"positions_multiple_candidates_start_of_words\0" as *const u8
            as *const libc::c_char,
    );
    if tmp___44 == 1 as libc::c_int {
        tmp___43 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___21 = tmp___43 as greatest_test_res;
        if res___21 as libc::c_int == 0 as libc::c_int {
            res___21 = positions_multiple_candidates_start_of_words();
        }
        greatest_test_post(
            b"positions_multiple_candidates_start_of_words\0" as *const u8
                as *const libc::c_char,
            res___21 as libc::c_int,
        );
    }
    tmp___46 = greatest_test_pre(
        b"positions_exact_match\0" as *const u8 as *const libc::c_char,
    );
    if tmp___46 == 1 as libc::c_int {
        tmp___45 = _setjmp((greatest_info.jump_dest).as_mut_ptr());
        res___22 = tmp___45 as greatest_test_res;
        if res___22 as libc::c_int == 0 as libc::c_int {
            res___22 = positions_exact_match();
        }
        greatest_test_post(
            b"positions_exact_match\0" as *const u8 as *const libc::c_char,
            res___22 as libc::c_int,
        );
    }
}
pub static mut bonus_states: [[score_t; 256]; 3] = [
    [
        0 as libc::c_int as score_t,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ],
    [
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.8f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.8f64,
        0.6f64,
        0.9f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.8f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ],
    [
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.8f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.8f64,
        0.6f64,
        0.9f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.0f64,
        0.8f64,
        0.0f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.7f64,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ],
];
pub static mut bonus_index: [size_t; 256] = [
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    2 as libc::c_int as size_t,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    0 as libc::c_ulong,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    1 as libc::c_int as size_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub unsafe extern "C" fn strcasechr(
    mut s: *const libc::c_char,
    mut c: libc::c_char,
) -> *mut libc::c_char {
    let mut accept: [libc::c_char; 3] = [0; 3];
    let mut __res: libc::c_int = 0;
    let mut tmp___0: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong > 1 as libc::c_ulong {
        __res = toupper(c as libc::c_int);
    } else {
        tmp___0 = __ctype_toupper_loc();
        __res = *(*tmp___0).offset(c as libc::c_int as isize);
    }
    accept[0 as libc::c_int as usize] = c;
    accept[1 as libc::c_int as usize] = __res as libc::c_char;
    accept[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp___1 = strpbrk(s, accept.as_mut_ptr() as *const libc::c_char);
    return tmp___1;
}
pub unsafe extern "C" fn has_match(
    mut needle: *const libc::c_char,
    mut haystack: *const libc::c_char,
) -> libc::c_int {
    let mut nch: libc::c_char = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    while *needle != 0 {
        tmp = needle;
        needle = needle.offset(1);
        nch = *tmp;
        tmp___0 = strcasechr(haystack, nch);
        haystack = tmp___0 as *const libc::c_char;
        if haystack.is_null() {
            return 0 as libc::c_int;
        }
        haystack = haystack.offset(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn precompute_bonus(
    mut haystack: *const libc::c_char,
    mut match_bonus: *mut score_t,
) {
    let mut last_ch: libc::c_char = 0;
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    last_ch = '/' as i32 as libc::c_char;
    i = 0 as libc::c_int;
    while *haystack.offset(i as isize) != 0 {
        ch = *haystack.offset(i as isize);
        *match_bonus
            .offset(
                i as isize,
            ) = bonus_states[bonus_index[ch as libc::c_uchar as usize]
            as usize][last_ch as libc::c_uchar as usize];
        last_ch = ch;
        i += 1;
    }
}
unsafe extern "C" fn setup_match_struct(
    mut match___0: *mut match_struct,
    mut needle: *const libc::c_char,
    mut haystack: *const libc::c_char,
) {
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut __res: libc::c_int = 0;
    let mut tmp___2: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut i___0: libc::c_int = 0;
    let mut __res___0: libc::c_int = 0;
    let mut tmp___4: *mut *const __int32_t = 0 as *mut *const __int32_t;
    tmp = strlen(needle);
    (*match___0).needle_len = tmp as libc::c_int;
    tmp___0 = strlen(haystack);
    (*match___0).haystack_len = tmp___0 as libc::c_int;
    if (*match___0).haystack_len > 1024 as libc::c_int {
        return
    } else {
        if (*match___0).needle_len > (*match___0).haystack_len {
            return;
        }
    }
    i = 0 as libc::c_int;
    while i < (*match___0).needle_len {
        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong > 1 as libc::c_ulong {
            __res = tolower(*needle.offset(i as isize) as libc::c_int);
        } else {
            tmp___2 = __ctype_tolower_loc();
            __res = *(*tmp___2)
                .offset(*needle.offset(i as isize) as libc::c_int as isize);
        }
        (*match___0).lower_needle[i as usize] = __res as libc::c_char;
        i += 1;
    }
    i___0 = 0 as libc::c_int;
    while i___0 < (*match___0).haystack_len {
        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong > 1 as libc::c_ulong {
            __res___0 = tolower(*haystack.offset(i___0 as isize) as libc::c_int);
        } else {
            tmp___4 = __ctype_tolower_loc();
            __res___0 = *(*tmp___4)
                .offset(*haystack.offset(i___0 as isize) as libc::c_int as isize);
        }
        (*match___0).lower_haystack[i___0 as usize] = __res___0 as libc::c_char;
        i___0 += 1;
    }
    precompute_bonus(haystack, ((*match___0).match_bonus).as_mut_ptr());
}
#[inline]
unsafe extern "C" fn match_row(
    mut match___0: *const match_struct,
    mut row: libc::c_int,
    mut curr_D: *mut score_t,
    mut curr_M: *mut score_t,
    mut last_D: *const score_t,
    mut last_M: *const score_t,
) {
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lower_needle: *const libc::c_char = 0 as *const libc::c_char;
    let mut lower_haystack: *const libc::c_char = 0 as *const libc::c_char;
    let mut match_bonus: *const score_t = 0 as *const score_t;
    let mut prev_score: score_t = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut gap_score: score_t = 0.;
    let mut tmp___0: libc::c_double = 0.;
    let mut j: libc::c_int = 0;
    let mut score: score_t = 0.;
    let mut tmp___1: libc::c_float = 0.;
    let mut tmp___2: libc::c_float = 0.;
    n = (*match___0).needle_len;
    m = (*match___0).haystack_len;
    i = row;
    lower_needle = ((*match___0).lower_needle).as_ptr();
    lower_haystack = ((*match___0).lower_haystack).as_ptr();
    match_bonus = ((*match___0).match_bonus).as_ptr();
    tmp = ::std::f32::INFINITY;
    prev_score = -tmp as score_t;
    if i == n - 1 as libc::c_int {
        tmp___0 = -0.005f64;
    } else {
        tmp___0 = -0.01f64;
    }
    gap_score = tmp___0;
    j = 0 as libc::c_int;
    while j < m {
        if *lower_needle.offset(i as isize) as libc::c_int
            == *lower_haystack.offset(j as isize) as libc::c_int
        {
            tmp___1 = ::std::f32::INFINITY;
            score = -tmp___1 as score_t;
            if i == 0 {
                score = j as libc::c_double * -0.005f64
                    + *match_bonus.offset(j as isize);
            } else if j != 0 {
                if *last_M.offset((j - 1 as libc::c_int) as isize)
                    + *match_bonus.offset(j as isize)
                    > *last_D.offset((j - 1 as libc::c_int) as isize) + 1.0f64
                {
                    score = *last_M.offset((j - 1 as libc::c_int) as isize)
                        + *match_bonus.offset(j as isize);
                } else {
                    score = *last_D.offset((j - 1 as libc::c_int) as isize) + 1.0f64;
                }
            }
            *curr_D.offset(j as isize) = score;
            if score > prev_score + gap_score {
                prev_score = score;
            } else {
                prev_score += gap_score;
            }
            *curr_M.offset(j as isize) = prev_score;
        } else {
            tmp___2 = ::std::f32::INFINITY;
            *curr_D.offset(j as isize) = -tmp___2 as score_t;
            prev_score += gap_score;
            *curr_M.offset(j as isize) = prev_score;
        }
        j += 1;
    }
}
pub unsafe extern "C" fn match_0(
    mut needle: *const libc::c_char,
    mut haystack: *const libc::c_char,
) -> score_t {
    let mut tmp: libc::c_float = 0.;
    let mut match___0: match_struct = match_struct {
        needle_len: 0,
        haystack_len: 0,
        lower_needle: [0; 1024],
        lower_haystack: [0; 1024],
        match_bonus: [0.; 1024],
    };
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut tmp___0: libc::c_float = 0.;
    let mut tmp___1: libc::c_float = 0.;
    let mut D: [[score_t; 1024]; 2] = [[0.; 1024]; 2];
    let mut M: [[score_t; 1024]; 2] = [[0.; 1024]; 2];
    let mut last_D: *mut score_t = 0 as *mut score_t;
    let mut last_M: *mut score_t = 0 as *mut score_t;
    let mut curr_D: *mut score_t = 0 as *mut score_t;
    let mut curr_M: *mut score_t = 0 as *mut score_t;
    let mut i: libc::c_int = 0;
    let mut SWAP: *mut score_t = 0 as *mut score_t;
    let mut SWAP___0: *mut score_t = 0 as *mut score_t;
    if *needle == 0 {
        tmp = ::std::f32::INFINITY;
        return -tmp as score_t;
    }
    setup_match_struct(&mut match___0, needle, haystack);
    n = match___0.needle_len;
    m = match___0.haystack_len;
    if m > 1024 as libc::c_int {
        tmp___0 = ::std::f32::INFINITY;
        return -tmp___0 as score_t;
    } else {
        if n > m {
            tmp___0 = ::std::f32::INFINITY;
            return -tmp___0 as score_t;
        } else {
            if n == m {
                tmp___1 = ::std::f32::INFINITY;
                return tmp___1 as score_t;
            }
        }
    }
    last_D = (D[0 as libc::c_int as usize]).as_mut_ptr();
    last_M = (M[0 as libc::c_int as usize]).as_mut_ptr();
    curr_D = (D[1 as libc::c_int as usize]).as_mut_ptr();
    curr_M = (M[1 as libc::c_int as usize]).as_mut_ptr();
    i = 0 as libc::c_int;
    while i < n {
        match_row(
            &mut match___0 as *mut match_struct as *const match_struct,
            i,
            curr_D,
            curr_M,
            last_D as *const score_t,
            last_M as *const score_t,
        );
        SWAP = curr_D;
        curr_D = last_D;
        last_D = SWAP;
        SWAP___0 = curr_M;
        curr_M = last_M;
        last_M = SWAP___0;
        i += 1;
    }
    return *last_M.offset((m - 1 as libc::c_int) as isize);
}
pub unsafe extern "C" fn match_positions(
    mut needle: *const libc::c_char,
    mut haystack: *const libc::c_char,
    mut positions: *mut size_t,
) -> score_t {
    let mut tmp: libc::c_float = 0.;
    let mut match___0: match_struct = match_struct {
        needle_len: 0,
        haystack_len: 0,
        lower_needle: [0; 1024],
        lower_haystack: [0; 1024],
        match_bonus: [0.; 1024],
    };
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut tmp___0: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut tmp___1: libc::c_float = 0.;
    let mut D: *mut [score_t; 1024] = 0 as *mut [score_t; 1024];
    let mut M: *mut [score_t; 1024] = 0 as *mut [score_t; 1024];
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut last_D: *mut score_t = 0 as *mut score_t;
    let mut last_M: *mut score_t = 0 as *mut score_t;
    let mut curr_D: *mut score_t = 0 as *mut score_t;
    let mut curr_M: *mut score_t = 0 as *mut score_t;
    let mut i___0: libc::c_int = 0;
    let mut match_required: libc::c_int = 0;
    let mut i___1: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_float = 0.;
    let mut result: score_t = 0.;
    if *needle == 0 {
        tmp = ::std::f32::INFINITY;
        return -tmp as score_t;
    }
    setup_match_struct(&mut match___0, needle, haystack);
    n = match___0.needle_len;
    m = match___0.haystack_len;
    if m > 1024 as libc::c_int {
        tmp___0 = ::std::f32::INFINITY;
        return -tmp___0 as score_t;
    } else {
        if n > m {
            tmp___0 = ::std::f32::INFINITY;
            return -tmp___0 as score_t;
        } else {
            if n == m {
                if !positions.is_null() {
                    i = 0 as libc::c_int;
                    while i < n {
                        *positions.offset(i as isize) = i as size_t;
                        i += 1;
                    }
                }
                tmp___1 = ::std::f32::INFINITY;
                return tmp___1 as score_t;
            }
        }
    }
    tmp___2 = malloc(
        (::std::mem::size_of::<score_t>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    M = tmp___2 as *mut [score_t; 1024];
    tmp___3 = malloc(
        (::std::mem::size_of::<score_t>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    );
    D = tmp___3 as *mut [score_t; 1024];
    i___0 = 0 as libc::c_int;
    while i___0 < n {
        curr_D = &mut *(*D.offset(i___0 as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut score_t;
        curr_M = &mut *(*M.offset(i___0 as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut score_t;
        match_row(
            &mut match___0 as *mut match_struct as *const match_struct,
            i___0,
            curr_D,
            curr_M,
            last_D as *const score_t,
            last_M as *const score_t,
        );
        last_D = curr_D;
        last_M = curr_M;
        i___0 += 1;
    }
    if !positions.is_null() {
        match_required = 0 as libc::c_int;
        i___1 = n - 1 as libc::c_int;
        j = m - 1 as libc::c_int;
        while i___1 >= 0 as libc::c_int {
            let mut current_block_60: u64;
            while j >= 0 as libc::c_int {
                tmp___6 = ::std::f32::INFINITY;
                if (*D.offset(i___1 as isize))[j as usize] != -tmp___6 as score_t {
                    if match_required != 0 {
                        current_block_60 = 11698274669007305219;
                    } else if (*D.offset(i___1 as isize))[j as usize]
                            == (*M.offset(i___1 as isize))[j as usize]
                        {
                        current_block_60 = 11698274669007305219;
                    } else {
                        current_block_60 = 9859671972921157070;
                    }
                    match current_block_60 {
                        9859671972921157070 => {}
                        _ => {
                            if i___1 != 0 {
                                if j != 0 {
                                    if (*M.offset(i___1 as isize))[j as usize]
                                        == (*D
                                            .offset(
                                                (i___1 - 1 as libc::c_int) as isize,
                                            ))[(j - 1 as libc::c_int) as usize] + 1.0f64
                                    {
                                        tmp___4 = 1 as libc::c_int;
                                    } else {
                                        tmp___4 = 0 as libc::c_int;
                                    }
                                } else {
                                    tmp___4 = 0 as libc::c_int;
                                }
                            } else {
                                tmp___4 = 0 as libc::c_int;
                            }
                            match_required = tmp___4;
                            tmp___5 = j;
                            j -= 1;
                            *positions.offset(i___1 as isize) = tmp___5 as size_t;
                            break;
                        }
                    }
                }
                j -= 1;
            }
            i___1 -= 1;
        }
    }
    result = (*M
        .offset((n - 1 as libc::c_int) as isize))[(m - 1 as libc::c_int) as usize];
    free(M as *mut libc::c_void);
    free(D as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn cmpchoice(
    mut _idx1: *const libc::c_void,
    mut _idx2: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const scored_result = 0 as *const scored_result;
    let mut b: *const scored_result = 0 as *const scored_result;
    a = _idx1 as *const scored_result;
    b = _idx2 as *const scored_result;
    if (*a).score == (*b).score {
        if ((*a).str_0 as libc::c_ulong) < (*b).str_0 as libc::c_ulong {
            return -(1 as libc::c_int)
        } else {
            return 1 as libc::c_int
        }
    } else if (*a).score < (*b).score {
        return 1 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn safe_realloc(
    mut buffer: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    buffer = realloc(buffer, size);
    if buffer.is_null() {
        fprintf(
            stderr,
            b"Error: Can't allocate memory (%zu bytes)\n\0" as *const u8
                as *const libc::c_char,
            size,
        );
        abort();
    }
    return buffer;
}
pub unsafe extern "C" fn choices_fread(
    mut c: *mut choices_t,
    mut file: *mut FILE,
    mut input_delimiter: libc::c_char,
) {
    let mut buffer_start: size_t = 0;
    let mut capacity: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: size_t = 0;
    let mut line_end: *const libc::c_char = 0 as *const libc::c_char;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    buffer_start = (*c).buffer_size;
    capacity = 4096 as libc::c_int as size_t;
    while capacity <= (*c).buffer_size {
        capacity = (capacity as libc::c_ulong).wrapping_mul(2 as libc::c_ulong) as size_t
            as size_t;
    }
    tmp = safe_realloc((*c).buffer as *mut libc::c_void, capacity);
    (*c).buffer = tmp as *mut libc::c_char;
    loop {
        tmp___1 = fread(
            ((*c).buffer).offset((*c).buffer_size as isize) as *mut libc::c_void,
            1 as libc::c_int as size_t,
            capacity.wrapping_sub((*c).buffer_size),
            file,
        );
        tmp___2 = ((*c).buffer_size).wrapping_add(tmp___1);
        (*c).buffer_size = tmp___2;
        if !(tmp___2 == capacity) {
            break;
        }
        capacity = (capacity as libc::c_ulong).wrapping_mul(2 as libc::c_ulong) as size_t
            as size_t;
        tmp___0 = safe_realloc((*c).buffer as *mut libc::c_void, capacity);
        (*c).buffer = tmp___0 as *mut libc::c_char;
    }
    tmp___3 = safe_realloc(
        (*c).buffer as *mut libc::c_void,
        ((*c).buffer_size).wrapping_add(1 as libc::c_ulong),
    );
    (*c).buffer = tmp___3 as *mut libc::c_char;
    tmp___4 = (*c).buffer_size;
    (*c).buffer_size = ((*c).buffer_size).wrapping_add(1);
    *((*c).buffer).offset(tmp___4 as isize) = '\u{0}' as i32 as libc::c_char;
    line_end = ((*c).buffer).offset((*c).buffer_size as isize) as *const libc::c_char;
    line = ((*c).buffer).offset(buffer_start as isize);
    loop {
        tmp___5 = strchr(line as *const libc::c_char, input_delimiter as libc::c_int);
        nl = tmp___5;
        if !nl.is_null() {
            tmp___6 = nl;
            nl = nl.offset(1);
            *tmp___6 = '\u{0}' as i32 as libc::c_char;
        }
        if *line != 0 {
            choices_add(c, line as *const libc::c_char);
        }
        line = nl;
        if line.is_null() {
            break;
        }
        if !((line as libc::c_ulong) < line_end as libc::c_ulong) {
            break;
        }
    };
}
unsafe extern "C" fn choices_resize(mut c: *mut choices_t, mut new_capacity: size_t) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = safe_realloc(
        (*c).strings as *mut libc::c_void,
        new_capacity
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    );
    (*c).strings = tmp as *mut *const libc::c_char;
    (*c).capacity = new_capacity;
}
unsafe extern "C" fn choices_reset_search(mut c: *mut choices_t) {
    let mut tmp: size_t = 0;
    free((*c).results as *mut libc::c_void);
    tmp = 0 as libc::c_int as size_t;
    (*c).available = tmp;
    (*c).selection = tmp;
    (*c).results = 0 as *mut libc::c_void as *mut scored_result;
}
pub unsafe extern "C" fn choices_init(
    mut c: *mut choices_t,
    mut options: *mut options_t,
) {
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_long = 0;
    (*c).strings = 0 as *mut libc::c_void as *mut *const libc::c_char;
    (*c).results = 0 as *mut libc::c_void as *mut scored_result;
    (*c).buffer_size = 0 as libc::c_int as size_t;
    (*c).buffer = 0 as *mut libc::c_void as *mut libc::c_char;
    tmp = 0 as libc::c_int as size_t;
    (*c).size = tmp;
    (*c).capacity = tmp;
    choices_resize(c, 128 as libc::c_int as size_t);
    if (*options).workers != 0 {
        (*c).worker_count = (*options).workers;
    } else {
        tmp___0 = sysconf(84 as libc::c_int);
        (*c).worker_count = tmp___0 as libc::c_int as libc::c_uint;
    }
    choices_reset_search(c);
}
pub unsafe extern "C" fn choices_destroy(mut c: *mut choices_t) {
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    free((*c).buffer as *mut libc::c_void);
    (*c).buffer = 0 as *mut libc::c_void as *mut libc::c_char;
    (*c).buffer_size = 0 as libc::c_int as size_t;
    free((*c).strings as *mut libc::c_void);
    (*c).strings = 0 as *mut libc::c_void as *mut *const libc::c_char;
    tmp = 0 as libc::c_int as size_t;
    (*c).size = tmp;
    (*c).capacity = tmp;
    free((*c).results as *mut libc::c_void);
    (*c).results = 0 as *mut libc::c_void as *mut scored_result;
    tmp___0 = 0 as libc::c_int as size_t;
    (*c).selection = tmp___0;
    (*c).available = tmp___0;
}
pub unsafe extern "C" fn choices_add(
    mut c: *mut choices_t,
    mut choice: *const libc::c_char,
) {
    let mut tmp: size_t = 0;
    choices_reset_search(c);
    if (*c).size == (*c).capacity {
        choices_resize(c, ((*c).capacity).wrapping_mul(2 as libc::c_ulong));
    }
    tmp = (*c).size;
    (*c).size = ((*c).size).wrapping_add(1);
    let ref mut fresh0 = *((*c).strings).offset(tmp as isize);
    *fresh0 = choice;
}
pub unsafe extern "C" fn choices_available(mut c: *mut choices_t) -> size_t {
    return (*c).available;
}
unsafe extern "C" fn worker_get_next_batch(
    mut job: *mut search_job,
    mut start: *mut size_t,
    mut end: *mut size_t,
) {
    pthread_mutex_lock(&mut (*job).lock);
    *start = (*job).processed;
    (*job)
        .processed = ((*job).processed as libc::c_ulong)
        .wrapping_add(512 as libc::c_ulong) as size_t as size_t;
    if (*job).processed > (*(*job).choices).size {
        (*job).processed = (*(*job).choices).size;
    }
    *end = (*job).processed;
    pthread_mutex_unlock(&mut (*job).lock);
}
unsafe extern "C" fn merge2(
    mut list1: result_list,
    mut list2: result_list,
) -> result_list {
    let mut result_index: size_t = 0;
    let mut index1: size_t = 0;
    let mut index2: size_t = 0;
    let mut result: result_list = result_list {
        list: 0 as *mut scored_result,
        size: 0,
    };
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: size_t = 0;
    result_index = 0 as libc::c_int as size_t;
    index1 = 0 as libc::c_int as size_t;
    index2 = 0 as libc::c_int as size_t;
    result.size = (list1.size).wrapping_add(list2.size);
    tmp = malloc(
        (result.size)
            .wrapping_mul(::std::mem::size_of::<scored_result>() as libc::c_ulong),
    );
    result.list = tmp as *mut scored_result;
    if (result.list).is_null() {
        fprintf(
            stderr,
            b"Error: Can't allocate memory\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    while index1 < list1.size {
        if !(index2 < list2.size) {
            break;
        }
        tmp___4 = cmpchoice(
            (list1.list).offset(index1 as isize) as *const libc::c_void,
            (list2.list).offset(index2 as isize) as *const libc::c_void,
        );
        if tmp___4 < 0 as libc::c_int {
            tmp___0 = result_index;
            result_index = result_index.wrapping_add(1);
            tmp___1 = index1;
            index1 = index1.wrapping_add(1);
            *(result.list)
                .offset(tmp___0 as isize) = *(list1.list).offset(tmp___1 as isize);
        } else {
            tmp___2 = result_index;
            result_index = result_index.wrapping_add(1);
            tmp___3 = index2;
            index2 = index2.wrapping_add(1);
            *(result.list)
                .offset(tmp___2 as isize) = *(list2.list).offset(tmp___3 as isize);
        }
    }
    while index1 < list1.size {
        tmp___5 = result_index;
        result_index = result_index.wrapping_add(1);
        tmp___6 = index1;
        index1 = index1.wrapping_add(1);
        *(result.list).offset(tmp___5 as isize) = *(list1.list).offset(tmp___6 as isize);
    }
    while index2 < list2.size {
        tmp___7 = result_index;
        result_index = result_index.wrapping_add(1);
        tmp___8 = index2;
        index2 = index2.wrapping_add(1);
        *(result.list).offset(tmp___7 as isize) = *(list2.list).offset(tmp___8 as isize);
    }
    free(list1.list as *mut libc::c_void);
    free(list2.list as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn choices_search_worker(
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut w: *mut worker = 0 as *mut worker;
    let mut job: *mut search_job = 0 as *mut search_job;
    let mut c: *const choices_t = 0 as *const choices_t;
    let mut result: *mut result_list = 0 as *mut result_list;
    let mut start: size_t = 0;
    let mut end: size_t = 0;
    let mut i: size_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut step: libc::c_uint = 0;
    let mut next_worker: libc::c_uint = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: libc::c_int = 0;
    w = data as *mut worker;
    job = (*w).job;
    c = (*job).choices as *const choices_t;
    result = &mut (*w).result;
    loop {
        worker_get_next_batch(job, &mut start, &mut end);
        if start == end {
            break;
        }
        i = start;
        while i < end {
            tmp = has_match((*job).search, *((*c).strings).offset(i as isize));
            if tmp != 0 {
                let ref mut fresh1 = (*((*result).list).offset((*result).size as isize))
                    .str_0;
                *fresh1 = *((*c).strings).offset(i as isize);
                (*((*result).list).offset((*result).size as isize))
                    .score = match_0((*job).search, *((*c).strings).offset(i as isize));
                (*result).size = ((*result).size).wrapping_add(1);
            }
            i = i.wrapping_add(1);
        }
    }
    qsort(
        (*result).list as *mut libc::c_void,
        (*result).size,
        ::std::mem::size_of::<scored_result>() as libc::c_ulong,
        Some(
            cmpchoice
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    step = 0 as libc::c_uint;
    while ((*w).worker_num).wrapping_rem(((2 as libc::c_int) << step) as libc::c_uint)
        == 0
    {
        next_worker = (*w).worker_num | ((1 as libc::c_int) << step) as libc::c_uint;
        if next_worker >= (*c).worker_count {
            break;
        }
        tmp___0 = __errno_location();
        tmp___1 = pthread_join(
            (*((*job).workers).offset(next_worker as isize)).thread_id,
            0 as *mut libc::c_void as *mut *mut libc::c_void,
        );
        *tmp___0 = tmp___1;
        if tmp___1 != 0 {
            perror(b"pthread_join\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        (*w)
            .result = merge2(
            (*w).result,
            (*((*job).workers).offset(next_worker as isize)).result,
        );
        step = step.wrapping_add(1);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn choices_search(
    mut c: *mut choices_t,
    mut search: *const libc::c_char,
) {
    let mut job: *mut search_job = 0 as *mut search_job;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut workers: *mut worker = 0 as *mut worker;
    let mut i: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    choices_reset_search(c);
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<search_job>() as libc::c_ulong,
    );
    job = tmp as *mut search_job;
    (*job).search = search;
    (*job).choices = c;
    tmp___0 = pthread_mutex_init(
        &mut (*job).lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    if tmp___0 != 0 as libc::c_int {
        fprintf(
            stderr,
            b"Error: pthread_mutex_init failed\n\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    tmp___1 = calloc(
        (*c).worker_count as size_t,
        ::std::mem::size_of::<worker>() as libc::c_ulong,
    );
    (*job).workers = tmp___1 as *mut worker;
    workers = (*job).workers;
    i = ((*c).worker_count).wrapping_sub(1 as libc::c_uint) as libc::c_int;
    while i >= 0 as libc::c_int {
        let ref mut fresh2 = (*workers.offset(i as isize)).job;
        *fresh2 = job;
        (*workers.offset(i as isize)).worker_num = i as libc::c_uint;
        (*workers.offset(i as isize)).result.size = 0 as libc::c_int as size_t;
        tmp___2 = malloc(
            ((*c).size)
                .wrapping_mul(::std::mem::size_of::<scored_result>() as libc::c_ulong),
        );
        let ref mut fresh3 = (*workers.offset(i as isize)).result.list;
        *fresh3 = tmp___2 as *mut scored_result;
        tmp___3 = __errno_location();
        tmp___4 = pthread_create(
            &mut (*workers.offset(i as isize)).thread_id as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                choices_search_worker
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            workers.offset(i as isize) as *mut libc::c_void,
        );
        *tmp___3 = tmp___4;
        if tmp___4 != 0 {
            perror(b"pthread_create\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        i -= 1;
    }
    tmp___5 = pthread_join(
        (*workers.offset(0 as libc::c_int as isize)).thread_id,
        0 as *mut libc::c_void as *mut *mut libc::c_void,
    );
    if tmp___5 != 0 {
        perror(b"pthread_join\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    (*c).results = (*workers.offset(0 as libc::c_int as isize)).result.list;
    (*c).available = (*workers.offset(0 as libc::c_int as isize)).result.size;
    free(workers as *mut libc::c_void);
    pthread_mutex_destroy(&mut (*job).lock);
    free(job as *mut libc::c_void);
}
pub unsafe extern "C" fn choices_get(
    mut c: *mut choices_t,
    mut n: size_t,
) -> *const libc::c_char {
    if n < (*c).available {
        return (*((*c).results).offset(n as isize)).str_0
    } else {
        return 0 as *mut libc::c_void as *const libc::c_char
    };
}
pub unsafe extern "C" fn choices_getscore(
    mut c: *mut choices_t,
    mut n: size_t,
) -> score_t {
    return (*((*c).results).offset(n as isize)).score;
}
pub unsafe extern "C" fn choices_prev(mut c: *mut choices_t) {
    if (*c).available != 0 {
        (*c)
            .selection = ((*c).selection)
            .wrapping_add((*c).available)
            .wrapping_sub(1 as libc::c_ulong)
            .wrapping_rem((*c).available);
    }
}
pub unsafe extern "C" fn choices_next(mut c: *mut choices_t) {
    if (*c).available != 0 {
        (*c)
            .selection = ((*c).selection)
            .wrapping_add(1 as libc::c_ulong)
            .wrapping_rem((*c).available);
    }
}
static mut usage_str: *const libc::c_char = b"Usage: fzy [OPTION]...\n -l, --lines=LINES        Specify how many lines of results to show (default 10)\n -p, --prompt=PROMPT      Input prompt (default '> ')\n -q, --query=QUERY        Use QUERY as the initial search string\n -e, --show-matches=QUERY Output the sorted matches of QUERY\n -t, --tty=TTY            Specify file to use as TTY device (default /dev/tty)\n -s, --show-scores        Show the scores of each match\n -0, --read-null          Read input delimited by ASCII NUL characters\n -j, --workers NUM        Use NUM workers for searching. (default is # of CPUs)\n -i, --show-info          Show selection info line\n -h, --help     Display this help and exit\n -v, --version  Output version information and exit\n\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn usage(mut argv0: *const libc::c_char) {
    fprintf(stderr, usage_str, argv0);
}
static mut longopts: [option; 13] = [
    {
        let mut init = option {
            name: b"show-matches\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"query\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'q' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"lines\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"tty\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 't' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"prompt\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-scores\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"read-null\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: '0' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"version\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'v' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"benchmark\0" as *const u8 as *const libc::c_char,
            has_arg: 2 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'b' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"workers\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'j' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"show-info\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'i' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"help\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: 0 as *const libc::c_void as *mut libc::c_void as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
pub unsafe extern "C" fn options_init(mut options: *mut options_t) {
    (*options).benchmark = 0 as libc::c_int;
    (*options).filter = 0 as *mut libc::c_void as *const libc::c_char;
    (*options).init_search = 0 as *mut libc::c_void as *const libc::c_char;
    (*options).show_scores = 0 as libc::c_int;
    (*options).scrolloff = 1 as libc::c_uint;
    (*options).tty_filename = b"/dev/tty\0" as *const u8 as *const libc::c_char;
    (*options).num_lines = 10 as libc::c_uint;
    (*options).prompt = b"> \0" as *const u8 as *const libc::c_char;
    (*options).workers = 0 as libc::c_uint;
    (*options).input_delimiter = '\n' as i32 as libc::c_char;
    (*options).show_info = 0 as libc::c_int;
}
pub unsafe extern "C" fn options_parse(
    mut options: *mut options_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut c: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    options_init(options);
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"vhs0e:q:l:t:p:j:i\0" as *const u8 as *const libc::c_char,
            longopts.as_mut_ptr() as *const option,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            118 => {
                printf(
                    b"%s 1.0 \xC2\xA9 2014-2018 John Hawthorn\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset(0 as libc::c_int as isize),
                );
                exit(0 as libc::c_int);
            }
            115 => {
                (*options).show_scores = 1 as libc::c_int;
            }
            48 => {
                (*options).input_delimiter = '\u{0}' as i32 as libc::c_char;
            }
            113 => {
                (*options).init_search = optarg as *const libc::c_char;
            }
            101 => {
                (*options).filter = optarg as *const libc::c_char;
            }
            98 => {
                if !optarg.is_null() {
                    tmp = sscanf(
                        optarg as *const libc::c_char,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        &mut (*options).benchmark as *mut libc::c_int,
                    );
                    if tmp != 1 as libc::c_int {
                        usage(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                } else {
                    (*options).benchmark = 100 as libc::c_int;
                }
            }
            116 => {
                (*options).tty_filename = optarg as *const libc::c_char;
            }
            112 => {
                (*options).prompt = optarg as *const libc::c_char;
            }
            106 => {
                tmp___0 = sscanf(
                    optarg as *const libc::c_char,
                    b"%u\0" as *const u8 as *const libc::c_char,
                    &mut (*options).workers as *mut libc::c_uint,
                );
                if tmp___0 != 1 as libc::c_int {
                    usage(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
            }
            108 => {
                tmp___2 = strcmp(
                    optarg as *const libc::c_char,
                    b"max\0" as *const u8 as *const libc::c_char,
                );
                if tmp___2 != 0 {
                    tmp___1 = sscanf(
                        optarg as *const libc::c_char,
                        b"%d\0" as *const u8 as *const libc::c_char,
                        &mut l as *mut libc::c_int,
                    );
                    if tmp___1 != 1 as libc::c_int {
                        fprintf(
                            stderr,
                            b"Invalid format for --lines: %s\n\0" as *const u8
                                as *const libc::c_char,
                            optarg,
                        );
                        fprintf(
                            stderr,
                            b"Must be integer in range 3..\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        usage(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    } else {
                        if l < 3 as libc::c_int {
                            fprintf(
                                stderr,
                                b"Invalid format for --lines: %s\n\0" as *const u8
                                    as *const libc::c_char,
                                optarg,
                            );
                            fprintf(
                                stderr,
                                b"Must be integer in range 3..\n\0" as *const u8
                                    as *const libc::c_char,
                            );
                            usage(
                                *argv.offset(0 as libc::c_int as isize)
                                    as *const libc::c_char,
                            );
                            exit(1 as libc::c_int);
                        }
                    }
                } else {
                    l = 2147483647 as libc::c_int;
                }
                (*options).num_lines = l as libc::c_uint;
            }
            105 => {
                (*options).show_info = 1 as libc::c_int;
            }
            _ => {
                usage(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char);
                exit(0 as libc::c_int);
            }
        }
    }
    if optind != argc {
        usage(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn theft_init(mut bloom_bits: uint8_t) -> *mut theft {
    let mut t: *mut theft = 0 as *mut theft;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if bloom_bits as libc::c_int != 0 as libc::c_int {
        if (bloom_bits as libc::c_int) < 13 as libc::c_int {
            return 0 as *mut libc::c_void as *mut theft;
        }
    }
    if bloom_bits as libc::c_int > 33 as libc::c_int {
        if bloom_bits as libc::c_int != 255 as libc::c_int {
            return 0 as *mut libc::c_void as *mut theft;
        }
    }
    tmp = malloc(::std::mem::size_of::<theft>() as libc::c_ulong);
    t = tmp as *mut theft;
    if t as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut theft;
    }
    memset(
        t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<theft>() as libc::c_ulong,
    );
    (*t).mt = theft_mt_init(46725745578827501 as libc::c_long as uint64_t);
    if (*t).mt as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(t as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut theft;
    } else {
        (*t).out = stdout;
        (*t).requested_bloom_bits = bloom_bits;
        return t;
    };
}
pub unsafe extern "C" fn theft_set_seed(mut t: *mut theft, mut seed: uint64_t) {
    (*t).seed = seed;
    theft_mt_reset((*t).mt, seed);
}
pub unsafe extern "C" fn theft_random(mut t: *mut theft) -> libc::c_ulong {
    let mut ns: theft_seed = 0;
    let mut tmp: uint64_t = 0;
    tmp = theft_mt_random((*t).mt);
    ns = tmp;
    return ns;
}
pub unsafe extern "C" fn theft_random_double(mut t: *mut theft) -> libc::c_double {
    let mut tmp: libc::c_double = 0.;
    tmp = theft_mt_random_double((*t).mt);
    return tmp;
}
pub unsafe extern "C" fn theft_set_output_stream(mut t: *mut theft, mut out: *mut FILE) {
    (*t).out = out;
}
unsafe extern "C" fn check_all_args(
    mut info: *mut theft_propfun_info,
    mut all_hashable: *mut bool,
) -> bool {
    let mut ah: bool = false;
    let mut i: libc::c_int = 0;
    let mut ti: *mut theft_type_info = 0 as *mut theft_type_info;
    ah = 1 as libc::c_int != 0;
    i = 0 as libc::c_int;
    while i < (*info).arity as libc::c_int {
        ti = (*info).type_info[i as usize];
        if ::std::mem::transmute::<Option::<theft_alloc_cb>, libc::c_ulong>((*ti).alloc)
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            return 0 as libc::c_int != 0;
        }
        if ::std::mem::transmute::<Option::<theft_hash_cb>, libc::c_ulong>((*ti).hash)
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            ah = 0 as libc::c_int != 0;
        }
        i += 1;
    }
    *all_hashable = ah;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn default_progress_cb(
    mut info: *mut theft_trial_info,
    mut env: *mut libc::c_void,
) -> theft_progress_callback_res {
    return THEFT_PROGRESS_CONTINUE;
}
unsafe extern "C" fn infer_arity(mut info: *mut theft_propfun_info) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        if (*info).type_info[i as usize] as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            (*info).arity = i as uint8_t;
            break;
        } else {
            i += 1;
        }
    }
}
pub unsafe extern "C" fn theft_run(
    mut t: *mut theft,
    mut cfg: *mut theft_cfg,
) -> theft_run_res {
    let mut info: theft_propfun_info = theft_propfun_info {
        name: 0 as *const libc::c_char,
        fun: None,
        arity: 0,
        type_info: [0 as *mut theft_type_info; 10],
        always_seed_count: 0,
        always_seeds: 0 as *mut theft_seed,
    };
    let mut tmp: theft_run_res = THEFT_RUN_PASS;
    if t as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return THEFT_RUN_ERROR_BAD_ARGS
    } else {
        if cfg as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return THEFT_RUN_ERROR_BAD_ARGS;
        }
    }
    memset(
        &mut info as *mut theft_propfun_info as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<theft_propfun_info>() as libc::c_ulong,
    );
    info.name = (*cfg).name;
    info.fun = (*cfg).fun;
    memcpy(
        (info.type_info).as_mut_ptr() as *mut libc::c_void,
        ((*cfg).type_info).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[*mut theft_type_info; 10]>() as libc::c_ulong,
    );
    info.always_seed_count = (*cfg).always_seed_count;
    info.always_seeds = (*cfg).always_seeds;
    if (*cfg).seed != 0 {
        theft_set_seed(t, (*cfg).seed);
    } else {
        theft_set_seed(t, 46725745578827501 as libc::c_long as uint64_t);
    }
    if (*cfg).trials == 0 as libc::c_int {
        (*cfg).trials = 100 as libc::c_int;
    }
    tmp = theft_run_internal(
        t,
        &mut info,
        (*cfg).trials,
        (*cfg).progress_cb,
        (*cfg).env,
        (*cfg).report,
    );
    return tmp;
}
unsafe extern "C" fn theft_run_internal(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut trials: libc::c_int,
    mut cb: Option::<theft_progress_cb>,
    mut env: *mut libc::c_void,
    mut r: *mut theft_trial_report,
) -> theft_run_res {
    let mut fake_report: theft_trial_report = theft_trial_report {
        pass: 0,
        fail: 0,
        skip: 0,
        dup: 0,
    };
    let mut all_hashable: bool = false;
    let mut tmp: bool = false;
    let mut seed: theft_seed = 0;
    let mut initial_seed: theft_seed = 0;
    let mut always_seeds: libc::c_int = 0;
    let mut args: [*mut libc::c_void; 10] = [0 as *mut libc::c_void; 10];
    let mut cres___0: theft_progress_callback_res = THEFT_PROGRESS_CONTINUE;
    let mut trial: libc::c_int = 0;
    let mut ti: theft_trial_info = theft_trial_info {
        name: 0 as *const libc::c_char,
        trial: 0,
        seed: 0,
        status: THEFT_TRIAL_PASS,
        arity: 0,
        args: 0 as *mut *mut libc::c_void,
    };
    let mut gres: all_gen_res_t = ALL_GEN_OK;
    let mut tmp___0: all_gen_res_t = ALL_GEN_OK;
    let mut tmp___1: bool = false;
    if r as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        r = &mut fake_report;
    }
    memset(
        r as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<theft_trial_report>() as libc::c_ulong,
    );
    infer_arity(info);
    if (*info).arity as libc::c_int == 0 as libc::c_int {
        return THEFT_RUN_ERROR_BAD_ARGS;
    }
    if t as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return THEFT_RUN_ERROR_BAD_ARGS
    } else {
        if info as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return THEFT_RUN_ERROR_BAD_ARGS
        } else {
            if ::std::mem::transmute::<
                Option::<theft_propfun>,
                libc::c_ulong,
            >((*info).fun) == 0 as *mut libc::c_void as libc::c_ulong
            {
                return THEFT_RUN_ERROR_BAD_ARGS
            } else {
                if (*info).arity as libc::c_int == 0 as libc::c_int {
                    return THEFT_RUN_ERROR_BAD_ARGS;
                }
            }
        }
    }
    all_hashable = 0 as libc::c_int != 0;
    tmp = check_all_args(info, &mut all_hashable);
    if !tmp {
        return THEFT_RUN_ERROR_MISSING_CALLBACK;
    }
    if ::std::mem::transmute::<Option::<theft_progress_cb>, libc::c_ulong>(cb)
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        cb = Some(
            default_progress_cb
                as unsafe extern "C" fn(
                    *mut theft_trial_info,
                    *mut libc::c_void,
                ) -> theft_progress_callback_res,
        );
    }
    if all_hashable {
        if (*t).requested_bloom_bits as libc::c_int == 0 as libc::c_int {
            (*t).requested_bloom_bits = theft_bloom_recommendation(trials);
        }
        if (*t).requested_bloom_bits as libc::c_int != 255 as libc::c_int {
            (*t).bloom = theft_bloom_init((*t).requested_bloom_bits);
        }
    }
    seed = (*t).seed;
    initial_seed = (*t).seed;
    always_seeds = (*info).always_seed_count;
    if (*info).always_seeds as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        always_seeds = 0 as libc::c_int;
    }
    cres___0 = THEFT_PROGRESS_CONTINUE;
    trial = 0 as libc::c_int;
    while trial < trials {
        memset(
            args.as_mut_ptr() as *mut libc::c_void,
            255 as libc::c_int,
            ::std::mem::size_of::<[*mut libc::c_void; 10]>() as libc::c_ulong,
        );
        if cres___0 as libc::c_uint == 1 as libc::c_uint {
            break;
        }
        if trial < always_seeds {
            seed = *((*info).always_seeds).offset(trial as isize);
        } else if always_seeds > 0 as libc::c_int {
            if trial == always_seeds {
                seed = initial_seed;
            }
        }
        ti.name = (*info).name;
        ti.trial = trial;
        ti.seed = seed;
        ti.status = THEFT_TRIAL_PASS;
        ti.arity = (*info).arity;
        ti.args = args.as_mut_ptr();
        theft_set_seed(t, seed);
        tmp___0 = gen_all_args(t, info, seed, args.as_mut_ptr(), env);
        gres = tmp___0;
        match gres as libc::c_uint {
            1 => {
                ti.status = THEFT_TRIAL_SKIP;
                (*r).skip = ((*r).skip).wrapping_add(1);
                cres___0 = (Some(cb.expect("non-null function pointer")))
                    .expect("non-null function pointer")(&mut ti, env);
            }
            2 => {
                ti.status = THEFT_TRIAL_DUP;
                (*r).dup = ((*r).dup).wrapping_add(1);
                cres___0 = (Some(cb.expect("non-null function pointer")))
                    .expect("non-null function pointer")(&mut ti, env);
            }
            0 => {
                tmp___1 = run_trial(
                    t,
                    info,
                    args.as_mut_ptr(),
                    cb,
                    env,
                    r,
                    &mut ti,
                    &mut cres___0,
                );
                if !tmp___1 {
                    return THEFT_RUN_ERROR;
                }
            }
            _ => {
                ti.status = THEFT_TRIAL_ERROR;
                cres___0 = (Some(cb.expect("non-null function pointer")))
                    .expect("non-null function pointer")(&mut ti, env);
                return THEFT_RUN_ERROR;
            }
        }
        free_args(info, args.as_mut_ptr(), env);
        theft_set_seed(t, seed);
        seed = theft_random(t);
        trial += 1;
    }
    if (*r).fail > 0 as libc::c_ulong {
        return THEFT_RUN_FAIL
    } else {
        return THEFT_RUN_PASS
    };
}
unsafe extern "C" fn run_trial(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut cb: Option::<theft_progress_cb>,
    mut env: *mut libc::c_void,
    mut r: *mut theft_trial_report,
    mut ti: *mut theft_trial_info,
    mut cres___0: *mut theft_progress_callback_res,
) -> bool {
    let mut tres: theft_trial_res = THEFT_TRIAL_PASS;
    let mut tmp: theft_trial_res = THEFT_TRIAL_PASS;
    let mut tmp___0: bool = false;
    if !((*t).bloom).is_null() {
        mark_called(t, info, args, env);
    }
    tmp = call_fun(info, args);
    tres = tmp;
    (*ti).status = tres;
    match tres as libc::c_uint {
        0 => {
            (*r).pass = ((*r).pass).wrapping_add(1);
            *cres___0 = (Some(cb.expect("non-null function pointer")))
                .expect("non-null function pointer")(ti, env);
        }
        1 => {
            tmp___0 = attempt_to_shrink(t, info, args, env);
            if !tmp___0 {
                (*ti).status = THEFT_TRIAL_ERROR;
                *cres___0 = (Some(cb.expect("non-null function pointer")))
                    .expect("non-null function pointer")(ti, env);
                return 0 as libc::c_int != 0;
            }
            (*r).fail = ((*r).fail).wrapping_add(1);
            *cres___0 = report_on_failure(t, info, ti, cb, env);
        }
        2 => {
            *cres___0 = (Some(cb.expect("non-null function pointer")))
                .expect("non-null function pointer")(ti, env);
            (*r).skip = ((*r).skip).wrapping_add(1);
        }
        4 | 3 => {
            *cres___0 = (Some(cb.expect("non-null function pointer")))
                .expect("non-null function pointer")(ti, env);
            free_args(info, args, env);
            return 0 as libc::c_int != 0;
        }
        _ => {}
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn free_args(
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    let mut fcb: Option::<theft_free_cb> = None;
    i = 0 as libc::c_int;
    while i < (*info).arity as libc::c_int {
        fcb = (*(*info).type_info[i as usize]).free;
        if fcb.is_some() {
            if *args.offset(i as isize) as libc::c_ulong
                != -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong
            {
                (Some(fcb.expect("non-null function pointer")))
                    .expect("non-null function pointer")(*args.offset(i as isize), env);
            }
        }
        i += 1;
    }
}
pub unsafe extern "C" fn theft_free(mut t: *mut theft) {
    if !((*t).bloom).is_null() {
        theft_bloom_dump((*t).bloom);
        theft_bloom_free((*t).bloom);
        (*t).bloom = 0 as *mut libc::c_void as *mut theft_bloom;
    }
    theft_mt_free((*t).mt);
    free(t as *mut libc::c_void);
}
unsafe extern "C" fn call_fun(
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
) -> theft_trial_res {
    let mut res: theft_trial_res = THEFT_TRIAL_PASS;
    res = THEFT_TRIAL_ERROR;
    match (*info).arity as libc::c_int {
        1 => {
            res = ::std::mem::transmute::<
                _,
                fn(_) -> theft_trial_res,
            >(
                (Some(((*info).fun).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(*args.offset(0 as libc::c_int as isize));
        }
        2 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _) -> theft_trial_res,
            >(
                (Some(((*info).fun).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
            );
        }
        3 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _) -> theft_trial_res,
            >(
                (Some(((*info).fun).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
            );
        }
        4 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _) -> theft_trial_res,
            >(
                (Some(((*info).fun).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
            );
        }
        5 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _) -> theft_trial_res,
            >(
                (Some(((*info).fun).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
            );
        }
        6 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _, _) -> theft_trial_res,
            >(
                (Some(((*info).fun).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
                *args.offset(5 as libc::c_int as isize),
            );
        }
        7 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _, _, _) -> theft_trial_res,
            >(
                (Some(((*info).fun).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
                *args.offset(5 as libc::c_int as isize),
                *args.offset(6 as libc::c_int as isize),
            );
        }
        8 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _, _, _, _) -> theft_trial_res,
            >(
                (Some(((*info).fun).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
                *args.offset(5 as libc::c_int as isize),
                *args.offset(6 as libc::c_int as isize),
                *args.offset(7 as libc::c_int as isize),
            );
        }
        9 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _, _, _, _, _) -> theft_trial_res,
            >(
                (Some(((*info).fun).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
                *args.offset(5 as libc::c_int as isize),
                *args.offset(6 as libc::c_int as isize),
                *args.offset(7 as libc::c_int as isize),
                *args.offset(8 as libc::c_int as isize),
            );
        }
        10 => {
            res = ::std::mem::transmute::<
                _,
                fn(_, _, _, _, _, _, _, _, _, _) -> theft_trial_res,
            >(
                (Some(((*info).fun).expect("non-null function pointer")))
                    .expect("non-null function pointer"),
            )(
                *args.offset(0 as libc::c_int as isize),
                *args.offset(1 as libc::c_int as isize),
                *args.offset(2 as libc::c_int as isize),
                *args.offset(3 as libc::c_int as isize),
                *args.offset(4 as libc::c_int as isize),
                *args.offset(5 as libc::c_int as isize),
                *args.offset(6 as libc::c_int as isize),
                *args.offset(7 as libc::c_int as isize),
                *args.offset(8 as libc::c_int as isize),
                *args.offset(9 as libc::c_int as isize),
            );
        }
        _ => return THEFT_TRIAL_ERROR,
    }
    return res;
}
unsafe extern "C" fn gen_all_args(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut seed: theft_seed,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) -> all_gen_res_t {
    let mut i: libc::c_int = 0;
    let mut ti: *mut theft_type_info = 0 as *mut theft_type_info;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut j: libc::c_int = 0;
    let mut tmp___0: bool = false;
    i = 0 as libc::c_int;
    while i < (*info).arity as libc::c_int {
        ti = (*info).type_info[i as usize];
        tmp = (Some(((*ti).alloc).expect("non-null function pointer")))
            .expect("non-null function pointer")(t, seed, env);
        p = tmp;
        's_87: {
            if !(p as libc::c_ulong
                == -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong)
            {
                if !(p as libc::c_ulong
                    == -(2 as libc::c_int) as *mut libc::c_void as libc::c_ulong)
                {
                    let ref mut fresh4 = *args.offset(i as isize);
                    *fresh4 = p;
                    break 's_87;
                }
            }
            j = 0 as libc::c_int;
            while j < i {
                (Some(((*ti).free).expect("non-null function pointer")))
                    .expect("non-null function pointer")(*args.offset(j as isize), env);
                j += 1;
            }
            if p as libc::c_ulong
                == -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong
            {
                return ALL_GEN_SKIP
            } else {
                return ALL_GEN_ERROR
            }
        }
        seed = theft_random(t);
        i += 1;
    }
    if !((*t).bloom).is_null() {
        tmp___0 = check_called(t, info, args, env);
        if tmp___0 {
            return ALL_GEN_DUP;
        }
    }
    return ALL_GEN_OK;
}
unsafe extern "C" fn attempt_to_shrink(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) -> bool {
    let mut progress: bool = false;
    let mut ai: libc::c_int = 0;
    let mut ti: *mut theft_type_info = 0 as *mut theft_type_info;
    let mut rres: shrink_res = SHRINK_OK;
    let mut tmp: shrink_res = SHRINK_OK;
    progress = 0 as libc::c_int != 0;
    loop {
        progress = 0 as libc::c_int != 0;
        ai = 0 as libc::c_int;
        while ai < (*info).arity as libc::c_int {
            ti = (*info).type_info[ai as usize];
            if ((*ti).shrink).is_some() {
                tmp = attempt_to_shrink_arg(t, info, args, env, ai);
                rres = tmp;
                match rres as libc::c_uint {
                    0 => {
                        progress = 1 as libc::c_int != 0;
                    }
                    1 => {}
                    _ => return 0 as libc::c_int != 0,
                }
            }
            ai += 1;
        }
        if !progress {
            break;
        }
    }
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn attempt_to_shrink_arg(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
    mut ai: libc::c_int,
) -> shrink_res {
    let mut current_block: u64;
    let mut ti: *mut theft_type_info = 0 as *mut theft_type_info;
    let mut tactic: uint32_t = 0;
    let mut cur: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nv: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: bool = false;
    let mut res: theft_trial_res = THEFT_TRIAL_PASS;
    let mut tmp___1: theft_trial_res = THEFT_TRIAL_PASS;
    ti = (*info).type_info[ai as usize];
    tactic = 0 as libc::c_int as uint32_t;
    while tactic < 4294967295 as libc::c_uint {
        cur = *args.offset(ai as isize);
        tmp = (Some(((*ti).shrink).expect("non-null function pointer")))
            .expect("non-null function pointer")(cur, tactic, env);
        nv = tmp;
        if nv as libc::c_ulong
            == -(3 as libc::c_int) as *mut libc::c_void as libc::c_ulong
        {
            return SHRINK_DEAD_END
        } else if nv as libc::c_ulong
                == -(2 as libc::c_int) as *mut libc::c_void as libc::c_ulong
            {
            return SHRINK_ERROR
        } else {
            if !(nv as libc::c_ulong
                == -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong)
            {
                let ref mut fresh5 = *args.offset(ai as isize);
                *fresh5 = nv;
                if !((*t).bloom).is_null() {
                    tmp___0 = check_called(t, info, args, env);
                    if tmp___0 {
                        if ((*ti).free).is_some() {
                            (Some(((*ti).free).expect("non-null function pointer")))
                                .expect("non-null function pointer")(nv, env);
                        }
                        let ref mut fresh6 = *args.offset(ai as isize);
                        *fresh6 = cur;
                        current_block = 11628338766427279883;
                    } else {
                        mark_called(t, info, args, env);
                        current_block = 224731115979188411;
                    }
                } else {
                    current_block = 224731115979188411;
                }
                match current_block {
                    11628338766427279883 => {}
                    _ => {
                        tmp___1 = call_fun(info, args);
                        res = tmp___1;
                        match res as libc::c_uint {
                            2 | 0 => {
                                let ref mut fresh7 = *args.offset(ai as isize);
                                *fresh7 = cur;
                                if ((*ti).free).is_some() {
                                    (Some(((*ti).free).expect("non-null function pointer")))
                                        .expect("non-null function pointer")(nv, env);
                                }
                            }
                            1 => {
                                if ((*ti).free).is_some() {
                                    (Some(((*ti).free).expect("non-null function pointer")))
                                        .expect("non-null function pointer")(cur, env);
                                }
                                return SHRINK_OK;
                            }
                            4 | 3 => return SHRINK_ERROR,
                            _ => {}
                        }
                    }
                }
            }
            tactic = tactic.wrapping_add(1);
        }
    }
    return SHRINK_DEAD_END;
}
unsafe extern "C" fn get_arg_hash_buffer(
    mut buffer: *mut theft_hash,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*info).arity as libc::c_int {
        *buffer
            .offset(
                i as isize,
            ) = (Some(
            ((**((*info).type_info).as_mut_ptr().offset(i as isize)).hash)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(*args.offset(i as isize), env);
        i += 1;
    }
}
unsafe extern "C" fn mark_called(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) {
    let mut buffer: [theft_hash; 10] = [0; 10];
    get_arg_hash_buffer(buffer.as_mut_ptr(), info, args, env);
    theft_bloom_mark(
        (*t).bloom,
        buffer.as_mut_ptr() as *mut uint8_t,
        ((*info).arity as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<theft_hash>() as libc::c_ulong),
    );
}
unsafe extern "C" fn check_called(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut args: *mut *mut libc::c_void,
    mut env: *mut libc::c_void,
) -> bool {
    let mut buffer: [theft_hash; 10] = [0; 10];
    let mut tmp: bool = false;
    get_arg_hash_buffer(buffer.as_mut_ptr(), info, args, env);
    tmp = theft_bloom_check(
        (*t).bloom,
        buffer.as_mut_ptr() as *mut uint8_t,
        ((*info).arity as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<theft_hash>() as libc::c_ulong),
    );
    return tmp;
}
static mut cres: theft_progress_callback_res = THEFT_PROGRESS_CONTINUE;
unsafe extern "C" fn report_on_failure(
    mut t: *mut theft,
    mut info: *mut theft_propfun_info,
    mut ti: *mut theft_trial_info,
    mut cb: Option::<theft_progress_cb>,
    mut env: *mut libc::c_void,
) -> theft_progress_callback_res {
    let mut arity: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut print: Option::<theft_print_cb> = None;
    arity = (*info).arity as libc::c_int;
    if !((*info).name).is_null() {
        tmp = (*info).name;
    } else {
        tmp = b"\0" as *const u8 as *const libc::c_char;
    }
    fprintf(
        (*t).out,
        b"\n\n -- Counter-Example: %s\n\0" as *const u8 as *const libc::c_char,
        tmp,
    );
    fprintf(
        (*t).out,
        b"    Trial %u, Seed 0x%016llx\n\0" as *const u8 as *const libc::c_char,
        (*ti).trial,
        (*ti).seed,
    );
    i = 0 as libc::c_int;
    while i < arity {
        print = (*(*info).type_info[i as usize]).print;
        if print.is_some() {
            fprintf(
                (*t).out,
                b"    Argument %d:\n\0" as *const u8 as *const libc::c_char,
                i,
            );
            (Some(print.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*t).out, *((*ti).args).offset(i as isize), env);
            fprintf((*t).out, b"\n\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
    cres = (Some(cb.expect("non-null function pointer")))
        .expect("non-null function pointer")(ti, env);
    return cres;
}
pub unsafe extern "C" fn theft_bloom_init(mut bit_size2: uint8_t) -> *mut theft_bloom {
    let mut sz: size_t = 0;
    let mut b: *mut theft_bloom = 0 as *mut theft_bloom;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    sz = ((1 as libc::c_int) << bit_size2 as libc::c_int - 3 as libc::c_int) as size_t;
    tmp = malloc(
        (::std::mem::size_of::<theft_bloom>() as libc::c_ulong).wrapping_add(sz),
    );
    b = tmp as *mut theft_bloom;
    if !b.is_null() {
        (*b).size = sz;
        (*b).bit_count = bit_size2;
        memset(((*b).bits).as_mut_ptr() as *mut libc::c_void, 0 as libc::c_int, sz);
    }
    return b;
}
pub unsafe extern "C" fn theft_bloom_mark(
    mut b: *mut theft_bloom,
    mut data: *mut uint8_t,
    mut data_size: size_t,
) {
    let mut hash: uint64_t = 0;
    let mut tmp: theft_hash = 0;
    let mut bc: uint8_t = 0;
    let mut mask: uint64_t = 0;
    let mut bit_inc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut v: uint64_t = 0;
    let mut offset: size_t = 0;
    let mut bit: uint8_t = 0;
    tmp = theft_hash_onepass(data, data_size);
    hash = tmp;
    bc = (*b).bit_count;
    mask = (((1 as libc::c_int) << bc as libc::c_int) - 1 as libc::c_int) as uint64_t;
    bit_inc = (64 as libc::c_int - bc as libc::c_int) / 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int - bc as libc::c_int {
        v = (hash & mask << i) >> i;
        offset = v.wrapping_div(8 as libc::c_ulong);
        bit = ((1 as libc::c_int) << (v & 7 as libc::c_ulong)) as uint8_t;
        *((*b).bits)
            .as_mut_ptr()
            .offset(
                offset as isize,
            ) = (*((*b).bits).as_mut_ptr().offset(offset as isize) as libc::c_int
            | bit as libc::c_int) as uint8_t;
        i += bit_inc;
    }
}
pub unsafe extern "C" fn theft_bloom_check(
    mut b: *mut theft_bloom,
    mut data: *mut uint8_t,
    mut data_size: size_t,
) -> bool {
    let mut hash: uint64_t = 0;
    let mut tmp: theft_hash = 0;
    let mut bc: uint8_t = 0;
    let mut mask: uint64_t = 0;
    let mut bit_inc: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut v: uint64_t = 0;
    let mut offset: size_t = 0;
    let mut bit: uint8_t = 0;
    tmp = theft_hash_onepass(data, data_size);
    hash = tmp;
    bc = (*b).bit_count;
    mask = (((1 as libc::c_int) << bc as libc::c_int) - 1 as libc::c_int) as uint64_t;
    bit_inc = (64 as libc::c_int - bc as libc::c_int) / 4 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int - bc as libc::c_int {
        v = (hash & mask << i) >> i;
        offset = v.wrapping_div(8 as libc::c_ulong);
        bit = ((1 as libc::c_int) << (v & 7 as libc::c_ulong)) as uint8_t;
        if 0 as libc::c_int
            == *((*b).bits).as_mut_ptr().offset(offset as isize) as libc::c_int
                & bit as libc::c_int
        {
            return 0 as libc::c_int != 0;
        }
        i += bit_inc;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn theft_bloom_free(mut b: *mut theft_bloom) {
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn theft_bloom_dump(mut b: *mut theft_bloom) {
    let mut counts: [uint8_t; 256] = [0; 256];
    let mut total: size_t = 0;
    let mut row_total: uint16_t = 0;
    let mut i: size_t = 0;
    let mut count: uint8_t = 0;
    let mut tmp: uint8_t = 0;
    memset(
        counts.as_mut_ptr() as *mut libc::c_void,
        255 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 256]>() as libc::c_ulong,
    );
    total = 0 as libc::c_int as size_t;
    row_total = 0 as libc::c_int as uint16_t;
    i = 0 as libc::c_int as size_t;
    while i < (*b).size {
        tmp = get_bits_set_count(
            counts.as_mut_ptr(),
            *((*b).bits).as_mut_ptr().offset(i as isize),
        );
        count = tmp;
        total = (total as libc::c_ulong).wrapping_add(count as size_t) as size_t
            as size_t;
        row_total = (row_total as libc::c_int + count as libc::c_int) as uint16_t;
        i = i.wrapping_add(1);
    }
    if total > (*b).size {
        fprintf(
            stderr,
            b"\nWARNING: bloom filter is %zd%% full, larger bloom_bits value recommended.\n\0"
                as *const u8 as *const libc::c_char,
            (100 as libc::c_ulong)
                .wrapping_mul(total)
                .wrapping_div((8 as libc::c_ulong).wrapping_mul((*b).size)),
        );
    }
}
pub unsafe extern "C" fn theft_bloom_recommendation(mut trials: libc::c_int) -> uint8_t {
    let mut res: uint8_t = 0;
    let mut min: uint8_t = 0;
    let mut max: uint8_t = 0;
    let mut i: uint8_t = 0;
    let mut v: int32_t = 0;
    res = 17 as libc::c_int as uint8_t;
    min = 10 as libc::c_int as uint8_t;
    max = 30 as libc::c_int as uint8_t;
    i = min;
    while (i as libc::c_int) < max as libc::c_int {
        v = (1 as libc::c_int) << i as libc::c_int;
        if v > 14 as libc::c_int * trials {
            res = (i as libc::c_int + 3 as libc::c_int) as uint8_t;
            break;
        } else {
            i = (i as libc::c_int + 1 as libc::c_int) as uint8_t;
        }
    }
    return res;
}
unsafe extern "C" fn get_bits_set_count(
    mut counts: *mut uint8_t,
    mut byte: uint8_t,
) -> uint8_t {
    let mut v: uint8_t = 0;
    let mut t: uint8_t = 0;
    let mut i: uint8_t = 0;
    v = *counts.offset(byte as libc::c_int as isize);
    if v as libc::c_int != 255 as libc::c_int {
        return v;
    }
    t = 0 as libc::c_int as uint8_t;
    i = 0 as libc::c_int as uint8_t;
    while (i as libc::c_int) < 8 as libc::c_int {
        if byte as libc::c_int & (1 as libc::c_int) << i as libc::c_int != 0 {
            t = (t as libc::c_int + 1 as libc::c_int) as uint8_t;
        }
        i = (i as libc::c_int + 1 as libc::c_int) as uint8_t;
    }
    *counts.offset(byte as libc::c_int as isize) = t;
    return t;
}
pub unsafe extern "C" fn theft_mt_init(mut seed: uint64_t) -> *mut theft_mt {
    let mut mt: *mut theft_mt = 0 as *mut theft_mt;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<theft_mt>() as libc::c_ulong);
    mt = tmp as *mut theft_mt;
    if mt as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut theft_mt;
    }
    theft_mt_reset(mt, seed);
    return mt;
}
pub unsafe extern "C" fn theft_mt_free(mut mt: *mut theft_mt) {
    free(mt as *mut libc::c_void);
}
pub unsafe extern "C" fn theft_mt_reset(mut mt: *mut theft_mt, mut seed: uint64_t) {
    let mut mti: uint16_t = 0;
    (*mt).mt[0 as libc::c_int as usize] = seed;
    mti = 0 as libc::c_int as uint16_t;
    mti = 1 as libc::c_int as uint16_t;
    while (mti as libc::c_int) < 312 as libc::c_int {
        (*mt)
            .mt[mti
            as usize] = (6364136223846793005 as libc::c_ulonglong)
            .wrapping_mul(
                ((*mt).mt[(mti as libc::c_int - 1 as libc::c_int) as usize]
                    ^ (*mt).mt[(mti as libc::c_int - 1 as libc::c_int) as usize]
                        >> 62 as libc::c_int) as libc::c_ulonglong,
            )
            .wrapping_add(mti as libc::c_ulonglong) as uint64_t;
        mti = (mti as libc::c_int + 1 as libc::c_int) as uint16_t;
    }
    (*mt).mti = mti as int16_t;
}
pub unsafe extern "C" fn theft_mt_random(mut mt: *mut theft_mt) -> uint64_t {
    let mut tmp: uint64_t = 0;
    tmp = genrand64_int64(mt);
    return tmp;
}
pub unsafe extern "C" fn theft_mt_random_double(
    mut mt: *mut theft_mt,
) -> libc::c_double {
    let mut tmp: uint64_t = 0;
    tmp = genrand64_int64(mt);
    return (tmp >> 11 as libc::c_int) as libc::c_double
        * (1.0f64 / 9007199254740991.0f64);
}
static mut mag01: [uint64_t; 2] = [
    0 as libc::c_ulonglong as uint64_t,
    13043109905998158313 as libc::c_ulonglong as uint64_t,
];
unsafe extern "C" fn genrand64_int64(mut r: *mut theft_mt) -> uint64_t {
    let mut i: libc::c_int = 0;
    let mut x: uint64_t = 0;
    let mut tmp: int16_t = 0;
    if (*r).mti as libc::c_int >= 312 as libc::c_int {
        if (*r).mti as libc::c_int == 313 as libc::c_int {
            theft_mt_reset(r, 5489 as libc::c_ulonglong as uint64_t);
        }
        i = 0 as libc::c_int;
        while i < 156 as libc::c_int {
            x = ((*r).mt[i as usize] as libc::c_ulonglong
                & 18446744071562067968 as libc::c_ulonglong
                | (*r).mt[(i + 1 as libc::c_int) as usize] as libc::c_ulonglong
                    & 2147483647 as libc::c_ulonglong) as uint64_t;
            (*r)
                .mt[i
                as usize] = (*r).mt[(i + 156 as libc::c_int) as usize]
                ^ x >> 1 as libc::c_int
                ^ mag01[(x as libc::c_ulonglong & 1 as libc::c_ulonglong) as libc::c_int
                    as usize];
            i += 1;
        }
        while i < 311 as libc::c_int {
            x = ((*r).mt[i as usize] as libc::c_ulonglong
                & 18446744071562067968 as libc::c_ulonglong
                | (*r).mt[(i + 1 as libc::c_int) as usize] as libc::c_ulonglong
                    & 2147483647 as libc::c_ulonglong) as uint64_t;
            (*r)
                .mt[i
                as usize] = (*r).mt[(i + -(156 as libc::c_int)) as usize]
                ^ x >> 1 as libc::c_int
                ^ mag01[(x as libc::c_ulonglong & 1 as libc::c_ulonglong) as libc::c_int
                    as usize];
            i += 1;
        }
        x = ((*r).mt[311 as libc::c_int as usize] as libc::c_ulonglong
            & 18446744071562067968 as libc::c_ulonglong
            | (*r).mt[0 as libc::c_int as usize] as libc::c_ulonglong
                & 2147483647 as libc::c_ulonglong) as uint64_t;
        (*r)
            .mt[311 as libc::c_int
            as usize] = (*r).mt[155 as libc::c_int as usize] ^ x >> 1 as libc::c_int
            ^ mag01[(x as libc::c_ulonglong & 1 as libc::c_ulonglong) as libc::c_int
                as usize];
        (*r).mti = 0 as libc::c_int as int16_t;
    }
    tmp = (*r).mti;
    (*r).mti = ((*r).mti as libc::c_int + 1 as libc::c_int) as int16_t;
    x = (*r).mt[tmp as usize];
    x = (x as libc::c_ulonglong
        ^ (x >> 29 as libc::c_int) as libc::c_ulonglong
            & 6148914691236517205 as libc::c_ulonglong) as uint64_t;
    x = (x as libc::c_ulonglong
        ^ (x << 17 as libc::c_int) as libc::c_ulonglong
            & 8202884508482404352 as libc::c_ulonglong) as uint64_t;
    x = (x as libc::c_ulonglong
        ^ (x << 37 as libc::c_int) as libc::c_ulonglong
            & 18444473444759240704 as libc::c_ulonglong) as uint64_t;
    x ^= x >> 43 as libc::c_int;
    return x;
}
static mut fnv64_prime: libc::c_ulong = 1099511628211 as libc::c_long as uint64_t;
static mut fnv64_offset_basis: libc::c_ulong = 14695981039346656037 as libc::c_ulonglong
    as uint64_t;
pub unsafe extern "C" fn theft_hash_init(mut h: *mut theft_hasher) {
    (*h).accum = fnv64_offset_basis;
}
pub unsafe extern "C" fn theft_hash_sink(
    mut h: *mut theft_hasher,
    mut data: *mut uint8_t,
    mut bytes: size_t,
) {
    let mut a: uint64_t = 0;
    let mut i: size_t = 0;
    if h as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return
    } else {
        if data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return;
        }
    }
    a = (*h).accum;
    i = 0 as libc::c_int as size_t;
    while i < bytes {
        a = (a ^ *data.offset(i as isize) as libc::c_ulong).wrapping_mul(fnv64_prime);
        i = i.wrapping_add(1);
    }
    (*h).accum = a;
}
pub unsafe extern "C" fn theft_hash_done(mut h: *mut theft_hasher) -> theft_hash {
    let mut res: theft_hash = 0;
    res = (*h).accum;
    theft_hash_init(h);
    return res;
}
pub unsafe extern "C" fn theft_hash_onepass(
    mut data: *mut uint8_t,
    mut bytes: size_t,
) -> theft_hash {
    let mut h: theft_hasher = theft_hasher { accum: 0 };
    let mut tmp: theft_hash = 0;
    theft_hash_init(&mut h);
    theft_hash_sink(&mut h, data, bytes);
    tmp = theft_hash_done(&mut h);
    return tmp;
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
