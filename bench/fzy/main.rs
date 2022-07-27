use std::arch::asm;

use ::c2rust_asm_casts;
use ::libc;
use c2rust_asm_casts::AsmCastTrait;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::std::ffi::VaList) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn pselect(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn signal(
        __sig: libc::c_int,
        __handler: Option<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: Option<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    );
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn memmove(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
        -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type score_t = libc::c_double;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_tty_t_116520144 {
    pub fdin: libc::c_int,
    pub fout: *mut FILE,
    pub original_termios: termios,
    pub fgcolor: libc::c_int,
    pub maxwidth: size_t,
    pub maxheight: size_t,
}
pub type tty_t = __anonstruct_tty_t_116520144;
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
pub struct __anonstruct_tty_interface_t_641585985 {
    pub tty: *mut tty_t,
    pub choices: *mut choices_t,
    pub options: *mut options_t,
    pub search: [libc::c_char; 4097],
    pub last_search: [libc::c_char; 4097],
    pub cursor: size_t,
    pub ambiguous_key_pending: libc::c_int,
    pub input: [libc::c_char; 32],
    pub exit: libc::c_int,
}
pub type tty_interface_t = __anonstruct_tty_interface_t_641585985;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct match_struct {
    pub needle_len: libc::c_int,
    pub haystack_len: libc::c_int,
    pub lower_needle: [libc::c_char; 1024],
    pub lower_haystack: [libc::c_char; 1024],
    pub match_bonus: [score_t; 1024],
}
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type va_list___0 = __gnuc_va_list;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___sigset_t_991265788 {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = __anonstruct___sigset_t_991265788;
pub type sigset_t = __sigset_t;
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_fd_set_356711149 {
    pub fds_bits: [__fd_mask; 16],
}
pub type fd_set = __anonstruct_fd_set_356711149;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_keybinding_t_984224788 {
    pub key: *const libc::c_char,
    pub action: Option<unsafe extern "C" fn(*mut tty_interface_t) -> ()>,
}
pub type keybinding_t = __anonstruct_keybinding_t_984224788;
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
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut options: options_t = options_t {
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
    let mut choices: choices_t = choices_t {
        buffer: 0 as *mut libc::c_char,
        buffer_size: 0,
        capacity: 0,
        size: 0,
        strings: 0 as *mut *const libc::c_char,
        results: 0 as *mut scored_result,
        available: 0,
        selection: 0,
        worker_count: 0,
    };
    let mut i: libc::c_int = 0;
    let mut i___0: size_t = 0;
    let mut tmp: score_t = 0.;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tty: tty_t = tty_t {
        fdin: 0,
        fout: 0 as *mut FILE,
        original_termios: termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0,
        },
        fgcolor: 0,
        maxwidth: 0,
        maxheight: 0,
    };
    let mut tmp___3: libc::c_int = 0;
    let mut num_lines_adjustment: libc::c_int = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut tty_interface: tty_interface_t = tty_interface_t {
        tty: 0 as *mut tty_t,
        choices: 0 as *mut choices_t,
        options: 0 as *mut options_t,
        search: [0; 4097],
        last_search: [0; 4097],
        cursor: 0,
        ambiguous_key_pending: 0,
        input: [0; 32],
        exit: 0,
    };
    ret = 0 as libc::c_int;
    options_parse(&mut options, argc, argv);
    choices_init(&mut choices, &mut options);
    if options.benchmark != 0 {
        if (options.filter).is_null() {
            fprintf(
                stderr,
                b"Must specify -e/--show-matches with --benchmark\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        choices_fread(&mut choices, stdin, options.input_delimiter);
        i = 0 as libc::c_int;
        while i < options.benchmark {
            choices_search(&mut choices, options.filter);
            i += 1;
        }
    } else if !(options.filter).is_null() {
        choices_fread(&mut choices, stdin, options.input_delimiter);
        choices_search(&mut choices, options.filter);
        i___0 = 0 as libc::c_int as size_t;
        loop {
            tmp___1 = choices_available(&mut choices);
            if !(i___0 < tmp___1) {
                break;
            }
            if options.show_scores != 0 {
                tmp = choices_getscore(&mut choices, i___0);
                printf(b"%f\t\0" as *const u8 as *const libc::c_char, tmp);
            }
            tmp___0 = choices_get(&mut choices, i___0);
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, tmp___0);
            i___0 = i___0.wrapping_add(1);
        }
    } else {
        tmp___2 = isatty(0 as libc::c_int);
        if tmp___2 != 0 {
            choices_fread(&mut choices, stdin, options.input_delimiter);
        }
        tty_init(&mut tty, options.tty_filename);
        tmp___3 = isatty(0 as libc::c_int);
        if tmp___3 == 0 {
            choices_fread(&mut choices, stdin, options.input_delimiter);
        }
        if options.num_lines as size_t > choices.size {
            options.num_lines = choices.size as libc::c_uint;
        }
        num_lines_adjustment = 1 as libc::c_int;
        if options.show_info != 0 {
            num_lines_adjustment += 1;
        }
        tmp___5 = tty_getheight(&mut tty);
        if (options.num_lines).wrapping_add(num_lines_adjustment as libc::c_uint) as size_t
            > tmp___5
        {
            tmp___4 = tty_getheight(&mut tty);
            options.num_lines =
                tmp___4.wrapping_sub(num_lines_adjustment as size_t) as libc::c_uint;
        }
        tty_interface_init(&mut tty_interface, &mut tty, &mut choices, &mut options);
        ret = tty_interface_run(&mut tty_interface);
    }
    choices_destroy(&mut choices);
    return ret;
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
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.8f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.8f64, 0.6f64, 0.9f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.8f64, 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
    ],
    [
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.8f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.8f64, 0.6f64, 0.9f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.8f64, 0.0f64, 0.7f64, 0.7f64,
        0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64,
        0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64, 0.7f64,
        0.7f64, 0.7f64, 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.,
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
        *match_bonus.offset(i as isize) = bonus_states
            [bonus_index[ch as libc::c_uchar as usize] as usize][last_ch as libc::c_uchar as usize];
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
        return;
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
            __res = *(*tmp___2).offset(*needle.offset(i as isize) as libc::c_int as isize);
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
            __res___0 =
                *(*tmp___4).offset(*haystack.offset(i___0 as isize) as libc::c_int as isize);
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
                score = j as libc::c_double * -0.005f64 + *match_bonus.offset(j as isize);
            } else if j != 0 {
                if *last_M.offset((j - 1 as libc::c_int) as isize) + *match_bonus.offset(j as isize)
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
                        current_block_60 = 8217158745379213294;
                    } else if (*D.offset(i___1 as isize))[j as usize]
                        == (*M.offset(i___1 as isize))[j as usize]
                    {
                        current_block_60 = 8217158745379213294;
                    } else {
                        current_block_60 = 9859671972921157070;
                    }
                    match current_block_60 {
                        9859671972921157070 => {}
                        _ => {
                            if i___1 != 0 {
                                if j != 0 {
                                    if (*M.offset(i___1 as isize))[j as usize]
                                        == (*D.offset((i___1 - 1 as libc::c_int) as isize))
                                            [(j - 1 as libc::c_int) as usize]
                                            + 1.0f64
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
    result = (*M.offset((n - 1 as libc::c_int) as isize))[(m - 1 as libc::c_int) as usize];
    free(M as *mut libc::c_void);
    free(D as *mut libc::c_void);
    return result;
}
pub unsafe extern "C" fn tty_reset(mut tty: *mut tty_t) {
    tcsetattr(
        (*tty).fdin,
        0 as libc::c_int,
        &mut (*tty).original_termios as *mut termios as *const termios,
    );
}
pub unsafe extern "C" fn tty_close(mut tty: *mut tty_t) {
    tty_reset(tty);
    fclose((*tty).fout);
    close((*tty).fdin);
}
unsafe extern "C" fn handle_sigwinch(mut sig: libc::c_int) {}
pub unsafe extern "C" fn tty_init(mut tty: *mut tty_t, mut tty_filename: *const libc::c_char) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut new_termios: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut tmp___1: libc::c_int = 0;
    (*tty).fdin = open(tty_filename, 0 as libc::c_int);
    if (*tty).fdin < 0 as libc::c_int {
        perror(b"Failed to open tty\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    (*tty).fout = fopen(tty_filename, b"w\0" as *const u8 as *const libc::c_char);
    if ((*tty).fout).is_null() {
        perror(b"Failed to open tty\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    tmp = setvbuf(
        (*tty).fout,
        0 as *mut libc::c_void as *mut libc::c_char,
        0 as libc::c_int,
        4096 as libc::c_int as size_t,
    );
    if tmp != 0 {
        perror(b"setvbuf\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    tmp___0 = tcgetattr((*tty).fdin, &mut (*tty).original_termios);
    if tmp___0 != 0 {
        perror(b"tcgetattr\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
    new_termios = (*tty).original_termios;
    new_termios.c_iflag &= 4294967039 as libc::c_uint;
    new_termios.c_lflag &= 4294967284 as libc::c_uint;
    tmp___1 = tcsetattr(
        (*tty).fdin,
        0 as libc::c_int,
        &mut new_termios as *mut termios as *const termios,
    );
    if tmp___1 != 0 {
        perror(b"tcsetattr\0" as *const u8 as *const libc::c_char);
    }
    tty_getwinsz(tty);
    tty_setnormal(tty);
    signal(
        28 as libc::c_int,
        Some(handle_sigwinch as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
pub unsafe extern "C" fn tty_getwinsz(mut tty: *mut tty_t) {
    let mut ws: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = fileno((*tty).fout);
    tmp___0 = ioctl(tmp, 21523 as libc::c_ulong, &mut ws as *mut winsize);
    if tmp___0 == -(1 as libc::c_int) {
        (*tty).maxwidth = 80 as libc::c_int as size_t;
        (*tty).maxheight = 25 as libc::c_int as size_t;
    } else {
        (*tty).maxwidth = ws.ws_col as size_t;
        (*tty).maxheight = ws.ws_row as size_t;
    };
}
pub unsafe extern "C" fn tty_getchar(mut tty: *mut tty_t) -> libc::c_char {
    let mut ch: libc::c_char = 0;
    let mut size: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    tmp = read(
        (*tty).fdin,
        &mut ch as *mut libc::c_char as *mut libc::c_void,
        1 as libc::c_int as size_t,
    );
    size = tmp as libc::c_int;
    if size < 0 as libc::c_int {
        perror(b"error reading from tty\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    } else if size == 0 as libc::c_int {
        exit(1 as libc::c_int);
    } else {
        return ch;
    };
}
pub unsafe extern "C" fn tty_input_ready(
    mut tty: *mut tty_t,
    mut timeout: libc::c_long,
    mut return_on_signal: libc::c_int,
) -> libc::c_int {
    let mut readfs: fd_set = fd_set { fds_bits: [0; 16] };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let mut ts: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut mask: sigset_t = sigset_t { __val: [0; 16] };
    let mut err: libc::c_int = 0;
    let mut tmp: *mut sigset_t = 0 as *mut sigset_t;
    let mut tmp___0: *mut timespec = 0 as *mut timespec;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh3 = &mut __d1;
    let fresh4;
    let fresh5 = &mut *(readfs.fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh0,
        fresh2) => fresh1, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh3,
        fresh5) => fresh4, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh0, fresh2, fresh1);
    c2rust_asm_casts::AsmCast::cast_out(fresh3, fresh5, fresh4);
    readfs.fds_bits[((*tty).fdin
        / (8 as libc::c_int * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize] |= ((1 as libc::c_ulong)
        << (*tty).fdin
            % (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as __fd_mask;
    ts.tv_sec = timeout / 1000 as libc::c_long;
    ts.tv_nsec = timeout % 1000 as libc::c_long * 1000000 as libc::c_long;
    sigemptyset(&mut mask);
    if return_on_signal == 0 {
        sigaddset(&mut mask, 28 as libc::c_int);
    }
    if return_on_signal != 0 {
        tmp = 0 as *mut libc::c_void as *mut sigset_t;
    } else {
        tmp = &mut mask;
    }
    if timeout < 0 as libc::c_long {
        tmp___0 = 0 as *mut libc::c_void as *mut timespec;
    } else {
        tmp___0 = &mut ts;
    }
    tmp___1 = pselect(
        (*tty).fdin + 1 as libc::c_int,
        &mut readfs as *mut fd_set,
        0 as *mut libc::c_void as *mut fd_set,
        0 as *mut libc::c_void as *mut fd_set,
        tmp___0 as *const timespec,
        tmp as *const __sigset_t,
    );
    err = tmp___1;
    if err < 0 as libc::c_int {
        tmp___2 = __errno_location();
        if *tmp___2 == 4 as libc::c_int {
            return 0 as libc::c_int;
        } else {
            perror(b"select\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    } else {
        return (readfs.fds_bits[((*tty).fdin
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << (*tty).fdin
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
                as __fd_mask
            != 0 as libc::c_long) as libc::c_int;
    };
}
unsafe extern "C" fn tty_sgr(mut tty: *mut tty_t, mut code: libc::c_int) {
    tty_printf(
        tty,
        b"%c%c%im\0" as *const u8 as *const libc::c_char,
        27 as libc::c_int,
        '[' as i32,
        code,
    );
}
pub unsafe extern "C" fn tty_setfg(mut tty: *mut tty_t, mut fg: libc::c_int) {
    if (*tty).fgcolor != fg {
        tty_sgr(tty, 30 as libc::c_int + fg);
        (*tty).fgcolor = fg;
    }
}
pub unsafe extern "C" fn tty_setinvert(mut tty: *mut tty_t) {
    tty_sgr(tty, 7 as libc::c_int);
}
pub unsafe extern "C" fn tty_setunderline(mut tty: *mut tty_t) {
    tty_sgr(tty, 4 as libc::c_int);
}
pub unsafe extern "C" fn tty_setnormal(mut tty: *mut tty_t) {
    tty_sgr(tty, 0 as libc::c_int);
    (*tty).fgcolor = 9 as libc::c_int;
}
pub unsafe extern "C" fn tty_setnowrap(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%c?7l\0" as *const u8 as *const libc::c_char,
        27 as libc::c_int,
        '[' as i32,
    );
}
pub unsafe extern "C" fn tty_setwrap(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%c?7h\0" as *const u8 as *const libc::c_char,
        27 as libc::c_int,
        '[' as i32,
    );
}
pub unsafe extern "C" fn tty_newline(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%cK\n\0" as *const u8 as *const libc::c_char,
        27 as libc::c_int,
        '[' as i32,
    );
}
pub unsafe extern "C" fn tty_clearline(mut tty: *mut tty_t) {
    tty_printf(
        tty,
        b"%c%cK\0" as *const u8 as *const libc::c_char,
        27 as libc::c_int,
        '[' as i32,
    );
}
pub unsafe extern "C" fn tty_setcol(mut tty: *mut tty_t, mut col: libc::c_int) {
    tty_printf(
        tty,
        b"%c%c%iG\0" as *const u8 as *const libc::c_char,
        27 as libc::c_int,
        '[' as i32,
        col + 1 as libc::c_int,
    );
}
pub unsafe extern "C" fn tty_moveup(mut tty: *mut tty_t, mut i: libc::c_int) {
    tty_printf(
        tty,
        b"%c%c%iA\0" as *const u8 as *const libc::c_char,
        27 as libc::c_int,
        '[' as i32,
        i,
    );
}
pub unsafe extern "C" fn tty_printf(
    mut tty: *mut tty_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf((*tty).fout, fmt, args_0.as_va_list());
}
pub unsafe extern "C" fn tty_putc(mut tty: *mut tty_t, mut c: libc::c_char) {
    fputc(c as libc::c_int, (*tty).fout);
}
pub unsafe extern "C" fn tty_flush(mut tty: *mut tty_t) {
    fflush((*tty).fout);
}
pub unsafe extern "C" fn tty_getwidth(mut tty: *mut tty_t) -> size_t {
    return (*tty).maxwidth;
}
pub unsafe extern "C" fn tty_getheight(mut tty: *mut tty_t) -> size_t {
    return (*tty).maxheight;
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
            return -(1 as libc::c_int);
        } else {
            return 1 as libc::c_int;
        }
    } else if (*a).score < (*b).score {
        return 1 as libc::c_int;
    } else {
        return -(1 as libc::c_int);
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
            b"Error: Can't allocate memory (%zu bytes)\n\0" as *const u8 as *const libc::c_char,
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
        capacity = (capacity as libc::c_ulong).wrapping_mul(2 as libc::c_ulong) as size_t as size_t;
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
        capacity = (capacity as libc::c_ulong).wrapping_mul(2 as libc::c_ulong) as size_t as size_t;
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
    }
}
unsafe extern "C" fn choices_resize(mut c: *mut choices_t, mut new_capacity: size_t) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = safe_realloc(
        (*c).strings as *mut libc::c_void,
        new_capacity.wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
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
pub unsafe extern "C" fn choices_init(mut c: *mut choices_t, mut options: *mut options_t) {
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
pub unsafe extern "C" fn choices_add(mut c: *mut choices_t, mut choice: *const libc::c_char) {
    let mut tmp: size_t = 0;
    choices_reset_search(c);
    if (*c).size == (*c).capacity {
        choices_resize(c, ((*c).capacity).wrapping_mul(2 as libc::c_ulong));
    }
    tmp = (*c).size;
    (*c).size = ((*c).size).wrapping_add(1);
    let ref mut fresh6 = *((*c).strings).offset(tmp as isize);
    *fresh6 = choice;
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
    (*job).processed =
        ((*job).processed as libc::c_ulong).wrapping_add(512 as libc::c_ulong) as size_t as size_t;
    if (*job).processed > (*(*job).choices).size {
        (*job).processed = (*(*job).choices).size;
    }
    *end = (*job).processed;
    pthread_mutex_unlock(&mut (*job).lock);
}
unsafe extern "C" fn merge2(mut list1: result_list, mut list2: result_list) -> result_list {
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
    tmp =
        malloc((result.size).wrapping_mul(::std::mem::size_of::<scored_result>() as libc::c_ulong));
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
            *(result.list).offset(tmp___0 as isize) = *(list1.list).offset(tmp___1 as isize);
        } else {
            tmp___2 = result_index;
            result_index = result_index.wrapping_add(1);
            tmp___3 = index2;
            index2 = index2.wrapping_add(1);
            *(result.list).offset(tmp___2 as isize) = *(list2.list).offset(tmp___3 as isize);
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
unsafe extern "C" fn choices_search_worker(mut data: *mut libc::c_void) -> *mut libc::c_void {
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
                let ref mut fresh7 = (*((*result).list).offset((*result).size as isize)).str_0;
                *fresh7 = *((*c).strings).offset(i as isize);
                (*((*result).list).offset((*result).size as isize)).score =
                    match_0((*job).search, *((*c).strings).offset(i as isize));
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
                as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        ),
    );
    step = 0 as libc::c_uint;
    while ((*w).worker_num).wrapping_rem(((2 as libc::c_int) << step) as libc::c_uint) == 0 {
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
        (*w).result = merge2(
            (*w).result,
            (*((*job).workers).offset(next_worker as isize)).result,
        );
        step = step.wrapping_add(1);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn choices_search(mut c: *mut choices_t, mut search: *const libc::c_char) {
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
        let ref mut fresh8 = (*workers.offset(i as isize)).job;
        *fresh8 = job;
        (*workers.offset(i as isize)).worker_num = i as libc::c_uint;
        (*workers.offset(i as isize)).result.size = 0 as libc::c_int as size_t;
        tmp___2 = malloc(
            ((*c).size).wrapping_mul(::std::mem::size_of::<scored_result>() as libc::c_ulong),
        );
        let ref mut fresh9 = (*workers.offset(i as isize)).result.list;
        *fresh9 = tmp___2 as *mut scored_result;
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
pub unsafe extern "C" fn choices_get(mut c: *mut choices_t, mut n: size_t) -> *const libc::c_char {
    if n < (*c).available {
        return (*((*c).results).offset(n as isize)).str_0;
    } else {
        return 0 as *mut libc::c_void as *const libc::c_char;
    };
}
pub unsafe extern "C" fn choices_getscore(mut c: *mut choices_t, mut n: size_t) -> score_t {
    return (*((*c).results).offset(n as isize)).score;
}
pub unsafe extern "C" fn choices_prev(mut c: *mut choices_t) {
    if (*c).available != 0 {
        (*c).selection = ((*c).selection)
            .wrapping_add((*c).available)
            .wrapping_sub(1 as libc::c_ulong)
            .wrapping_rem((*c).available);
    }
}
pub unsafe extern "C" fn choices_next(mut c: *mut choices_t) {
    if (*c).available != 0 {
        (*c).selection = ((*c).selection)
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
                        usage(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char);
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
                    usage(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char);
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
                            b"Must be integer in range 3..\n\0" as *const u8 as *const libc::c_char,
                        );
                        usage(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char);
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
                            usage(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char);
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
unsafe extern "C" fn isprint_unicode(mut c: libc::c_char) -> libc::c_int {
    let mut tmp: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___0: libc::c_int = 0;
    tmp = __ctype_b_loc();
    if *(*tmp).offset(c as libc::c_int as isize) as libc::c_int & 16384 as libc::c_int != 0 {
        tmp___0 = 1 as libc::c_int;
    } else if c as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        tmp___0 = 1 as libc::c_int;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0;
}
unsafe extern "C" fn is_boundary(mut c: libc::c_char) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if !(c as libc::c_int) & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        tmp = 1 as libc::c_int;
    } else if c as libc::c_int & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        tmp = 1 as libc::c_int;
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp;
}
unsafe extern "C" fn clear(mut state: *mut tty_interface_t) {
    let mut tty: *mut tty_t = 0 as *mut tty_t;
    let mut line: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    tty = (*state).tty;
    tty_setcol(tty, 0 as libc::c_int);
    line = 0 as libc::c_int as size_t;
    loop {
        tmp = line;
        line = line.wrapping_add(1);
        if (*(*state).options).show_info != 0 {
            tmp___0 = 1 as libc::c_int;
        } else {
            tmp___0 = 0 as libc::c_int;
        }
        if !(tmp < ((*(*state).options).num_lines).wrapping_add(tmp___0 as libc::c_uint) as size_t)
        {
            break;
        }
        tty_newline(tty);
    }
    tty_clearline(tty);
    if (*(*state).options).num_lines > 0 as libc::c_uint {
        tty_moveup(tty, line.wrapping_sub(1 as libc::c_ulong) as libc::c_int);
    }
    tty_flush(tty);
}
unsafe extern "C" fn draw_match(
    mut state: *mut tty_interface_t,
    mut choice: *const libc::c_char,
    mut selected: libc::c_int,
) {
    let mut tty: *mut tty_t = 0 as *mut tty_t;
    let mut options: *mut options_t = 0 as *mut options_t;
    let mut search: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut positions: [size_t; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut score: score_t = 0.;
    let mut tmp___0: score_t = 0.;
    let mut tmp___1: libc::c_float = 0.;
    let mut i___0: size_t = 0;
    let mut p: size_t = 0;
    tty = (*state).tty;
    options = (*state).options;
    search = ((*state).last_search).as_mut_ptr();
    tmp = strlen(search as *const libc::c_char);
    n = tmp as libc::c_int;
    i = 0 as libc::c_int;
    while i < n + 1 as libc::c_int {
        if !(i < 1024 as libc::c_int) {
            break;
        }
        positions[i as usize] = -(1 as libc::c_int) as size_t;
        i += 1;
    }
    tmp___0 = match_positions(
        search as *const libc::c_char,
        choice,
        &mut *positions.as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    score = tmp___0;
    if (*options).show_scores != 0 {
        tmp___1 = ::std::f32::INFINITY;
        if score == -tmp___1 as score_t {
            tty_printf(tty, b"(     ) \0" as *const u8 as *const libc::c_char);
        } else {
            tty_printf(
                tty,
                b"(%5.2f) \0" as *const u8 as *const libc::c_char,
                score,
            );
        }
    }
    if selected != 0 {
        tty_setinvert(tty);
    }
    tty_setnowrap(tty);
    i___0 = 0 as libc::c_int as size_t;
    p = 0 as libc::c_int as size_t;
    while *choice.offset(i___0 as isize) as libc::c_int != 0 as libc::c_int {
        if positions[p as usize] == i___0 {
            tty_setfg(tty, 3 as libc::c_int);
            p = p.wrapping_add(1);
        } else {
            tty_setfg(tty, 9 as libc::c_int);
        }
        if *choice.offset(i___0 as isize) as libc::c_int == 10 as libc::c_int {
            tty_putc(tty, ' ' as i32 as libc::c_char);
        } else {
            tty_printf(
                tty,
                b"%c\0" as *const u8 as *const libc::c_char,
                *choice.offset(i___0 as isize) as libc::c_int,
            );
        }
        i___0 = i___0.wrapping_add(1);
    }
    tty_setwrap(tty);
    tty_setnormal(tty);
}
unsafe extern "C" fn draw(mut state: *mut tty_interface_t) {
    let mut tty: *mut tty_t = 0 as *mut tty_t;
    let mut choices: *mut choices_t = 0 as *mut choices_t;
    let mut options: *mut options_t = 0 as *mut options_t;
    let mut num_lines: libc::c_uint = 0;
    let mut start: size_t = 0;
    let mut current_selection: size_t = 0;
    let mut available: size_t = 0;
    let mut tmp: size_t = 0;
    let mut i: size_t = 0;
    let mut choice: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut i___0: size_t = 0;
    tty = (*state).tty;
    choices = (*state).choices;
    options = (*state).options;
    num_lines = (*options).num_lines;
    start = 0 as libc::c_int as size_t;
    current_selection = (*choices).selection;
    if current_selection.wrapping_add((*options).scrolloff as size_t) >= num_lines as size_t {
        start = current_selection
            .wrapping_add((*options).scrolloff as size_t)
            .wrapping_sub(num_lines as size_t)
            .wrapping_add(1 as libc::c_ulong);
        tmp = choices_available(choices);
        available = tmp;
        if start.wrapping_add(num_lines as size_t) >= available {
            if available > 0 as libc::c_ulong {
                start = available.wrapping_sub(num_lines as size_t);
            }
        }
    }
    tty_setcol(tty, 0 as libc::c_int);
    tty_printf(
        tty,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        (*options).prompt,
        ((*state).search).as_mut_ptr(),
    );
    tty_clearline(tty);
    if (*options).show_info != 0 {
        tty_printf(
            tty,
            b"\n[%lu/%lu]\0" as *const u8 as *const libc::c_char,
            (*choices).available,
            (*choices).size,
        );
        tty_clearline(tty);
    }
    i = start;
    while i < start.wrapping_add(num_lines as size_t) {
        tty_printf(tty, b"\n\0" as *const u8 as *const libc::c_char);
        tty_clearline(tty);
        tmp___0 = choices_get(choices, i);
        choice = tmp___0;
        if !choice.is_null() {
            draw_match(state, choice, (i == (*choices).selection) as libc::c_int);
        }
        i = i.wrapping_add(1);
    }
    if num_lines.wrapping_add((*options).show_info as libc::c_uint) != 0 {
        tty_moveup(
            tty,
            num_lines.wrapping_add((*options).show_info as libc::c_uint) as libc::c_int,
        );
    }
    tty_setcol(tty, 0 as libc::c_int);
    fputs((*options).prompt, (*tty).fout);
    i___0 = 0 as libc::c_int as size_t;
    while i___0 < (*state).cursor {
        fputc((*state).search[i___0 as usize] as libc::c_int, (*tty).fout);
        i___0 = i___0.wrapping_add(1);
    }
    tty_flush(tty);
}
unsafe extern "C" fn update_search(mut state: *mut tty_interface_t) {
    choices_search(
        (*state).choices,
        ((*state).search).as_mut_ptr() as *const libc::c_char,
    );
    strcpy(
        ((*state).last_search).as_mut_ptr(),
        ((*state).search).as_mut_ptr() as *const libc::c_char,
    );
}
unsafe extern "C" fn update_state(mut state: *mut tty_interface_t) {
    let mut tmp: libc::c_int = 0;
    tmp = strcmp(
        ((*state).last_search).as_mut_ptr() as *const libc::c_char,
        ((*state).search).as_mut_ptr() as *const libc::c_char,
    );
    if tmp != 0 {
        update_search(state);
        draw(state);
    }
}
unsafe extern "C" fn action_emit(mut state: *mut tty_interface_t) {
    let mut selection: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    update_state(state);
    clear(state);
    tty_close((*state).tty);
    tmp = choices_get((*state).choices, (*(*state).choices).selection);
    selection = tmp;
    if !selection.is_null() {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, selection);
    } else {
        printf(
            b"%s\n\0" as *const u8 as *const libc::c_char,
            ((*state).search).as_mut_ptr(),
        );
    }
    (*state).exit = 0 as libc::c_int;
}
unsafe extern "C" fn action_del_char(mut state: *mut tty_interface_t) {
    let mut length: size_t = 0;
    let mut tmp: size_t = 0;
    let mut original_cursor: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = strlen(((*state).search).as_mut_ptr() as *const libc::c_char);
    length = tmp;
    if (*state).cursor == 0 as libc::c_ulong {
        return;
    }
    original_cursor = (*state).cursor;
    loop {
        (*state).cursor = ((*state).cursor).wrapping_sub(1);
        tmp___0 = is_boundary((*state).search[(*state).cursor as usize]);
        if tmp___0 != 0 {
            break;
        }
        if (*state).cursor == 0 {
            break;
        }
    }
    memmove(
        &mut *((*state).search)
            .as_mut_ptr()
            .offset((*state).cursor as isize) as *mut libc::c_char as *mut libc::c_void,
        &mut *((*state).search)
            .as_mut_ptr()
            .offset(original_cursor as isize) as *mut libc::c_char as *const libc::c_void,
        length
            .wrapping_sub(original_cursor)
            .wrapping_add(1 as libc::c_ulong),
    );
}
unsafe extern "C" fn action_del_word(mut state: *mut tty_interface_t) {
    let mut original_cursor: size_t = 0;
    let mut cursor: size_t = 0;
    let mut tmp: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___1: size_t = 0;
    original_cursor = (*state).cursor;
    cursor = (*state).cursor;
    while cursor != 0 {
        tmp = __ctype_b_loc();
        if *(*tmp).offset(
            (*state).search[cursor.wrapping_sub(1 as libc::c_ulong) as usize] as libc::c_int
                as isize,
        ) as libc::c_int
            & 8192 as libc::c_int
            == 0
        {
            break;
        }
        cursor = cursor.wrapping_sub(1);
    }
    while cursor != 0 {
        tmp___0 = __ctype_b_loc();
        if *(*tmp___0).offset(
            (*state).search[cursor.wrapping_sub(1 as libc::c_ulong) as usize] as libc::c_int
                as isize,
        ) as libc::c_int
            & 8192 as libc::c_int
            != 0
        {
            break;
        }
        cursor = cursor.wrapping_sub(1);
    }
    tmp___1 = strlen(((*state).search).as_mut_ptr() as *const libc::c_char);
    memmove(
        &mut *((*state).search).as_mut_ptr().offset(cursor as isize) as *mut libc::c_char
            as *mut libc::c_void,
        &mut *((*state).search)
            .as_mut_ptr()
            .offset(original_cursor as isize) as *mut libc::c_char as *const libc::c_void,
        tmp___1
            .wrapping_sub(original_cursor)
            .wrapping_add(1 as libc::c_ulong),
    );
    (*state).cursor = cursor;
}
unsafe extern "C" fn action_del_all(mut state: *mut tty_interface_t) {
    let mut tmp: size_t = 0;
    tmp = strlen(((*state).search).as_mut_ptr() as *const libc::c_char);
    memmove(
        ((*state).search).as_mut_ptr() as *mut libc::c_void,
        &mut *((*state).search)
            .as_mut_ptr()
            .offset((*state).cursor as isize) as *mut libc::c_char as *const libc::c_void,
        tmp.wrapping_sub((*state).cursor)
            .wrapping_add(1 as libc::c_ulong),
    );
    (*state).cursor = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn action_prev(mut state: *mut tty_interface_t) {
    update_state(state);
    choices_prev((*state).choices);
}
unsafe extern "C" fn action_ignore(mut state: *mut tty_interface_t) {}
unsafe extern "C" fn action_next(mut state: *mut tty_interface_t) {
    update_state(state);
    choices_next((*state).choices);
}
unsafe extern "C" fn action_left(mut state: *mut tty_interface_t) {
    let mut tmp: libc::c_int = 0;
    if (*state).cursor > 0 as libc::c_ulong {
        (*state).cursor = ((*state).cursor).wrapping_sub(1);
        loop {
            tmp = is_boundary((*state).search[(*state).cursor as usize]);
            if tmp != 0 {
                break;
            }
            if (*state).cursor == 0 {
                break;
            }
            (*state).cursor = ((*state).cursor).wrapping_sub(1);
        }
    }
}
unsafe extern "C" fn action_right(mut state: *mut tty_interface_t) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    tmp___0 = strlen(((*state).search).as_mut_ptr() as *const libc::c_char);
    if (*state).cursor < tmp___0 {
        (*state).cursor = ((*state).cursor).wrapping_add(1);
        loop {
            tmp = is_boundary((*state).search[(*state).cursor as usize]);
            if tmp != 0 {
                break;
            }
            (*state).cursor = ((*state).cursor).wrapping_add(1);
        }
    }
}
unsafe extern "C" fn action_beginning(mut state: *mut tty_interface_t) {
    (*state).cursor = 0 as libc::c_int as size_t;
}
unsafe extern "C" fn action_end(mut state: *mut tty_interface_t) {
    (*state).cursor = strlen(((*state).search).as_mut_ptr() as *const libc::c_char);
}
unsafe extern "C" fn action_pageup(mut state: *mut tty_interface_t) {
    let mut i: size_t = 0;
    update_state(state);
    i = 0 as libc::c_int as size_t;
    while i < (*(*state).options).num_lines as size_t {
        if !((*(*state).choices).selection > 0 as libc::c_ulong) {
            break;
        }
        choices_prev((*state).choices);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn action_pagedown(mut state: *mut tty_interface_t) {
    let mut i: size_t = 0;
    update_state(state);
    i = 0 as libc::c_int as size_t;
    while i < (*(*state).options).num_lines as size_t {
        if !((*(*state).choices).selection
            < ((*(*state).choices).available).wrapping_sub(1 as libc::c_ulong))
        {
            break;
        }
        choices_next((*state).choices);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn action_autocomplete(mut state: *mut tty_interface_t) {
    let mut current_selection: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    update_state(state);
    tmp = choices_get((*state).choices, (*(*state).choices).selection);
    current_selection = tmp;
    if !current_selection.is_null() {
        tmp___0 = choices_get((*state).choices, (*(*state).choices).selection);
        strncpy(
            ((*state).search).as_mut_ptr(),
            tmp___0,
            4096 as libc::c_int as size_t,
        );
        (*state).cursor = strlen(((*state).search).as_mut_ptr() as *const libc::c_char);
    }
}
unsafe extern "C" fn action_exit(mut state: *mut tty_interface_t) {
    clear(state);
    tty_close((*state).tty);
    (*state).exit = 1 as libc::c_int;
}
unsafe extern "C" fn append_search(mut state: *mut tty_interface_t, mut ch: libc::c_char) {
    let mut search: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut search_size: size_t = 0;
    let mut tmp: size_t = 0;
    search = ((*state).search).as_mut_ptr();
    tmp = strlen(search as *const libc::c_char);
    search_size = tmp;
    if search_size < 4096 as libc::c_ulong {
        memmove(
            search.offset(((*state).cursor).wrapping_add(1 as libc::c_ulong) as isize)
                as *mut libc::c_void,
            search.offset((*state).cursor as isize) as *const libc::c_void,
            search_size
                .wrapping_sub((*state).cursor)
                .wrapping_add(1 as libc::c_ulong),
        );
        *search.offset((*state).cursor as isize) = ch;
        (*state).cursor = ((*state).cursor).wrapping_add(1);
    }
}
pub unsafe extern "C" fn tty_interface_init(
    mut state: *mut tty_interface_t,
    mut tty: *mut tty_t,
    mut choices: *mut choices_t,
    mut options: *mut options_t,
) {
    (*state).tty = tty;
    (*state).choices = choices;
    (*state).options = options;
    (*state).ambiguous_key_pending = 0 as libc::c_int;
    strcpy(
        ((*state).input).as_mut_ptr(),
        b"\0" as *const u8 as *const libc::c_char,
    );
    strcpy(
        ((*state).search).as_mut_ptr(),
        b"\0" as *const u8 as *const libc::c_char,
    );
    strcpy(
        ((*state).last_search).as_mut_ptr(),
        b"\0" as *const u8 as *const libc::c_char,
    );
    (*state).exit = -(1 as libc::c_int);
    if !((*options).init_search).is_null() {
        strncpy(
            ((*state).search).as_mut_ptr(),
            (*options).init_search,
            4096 as libc::c_int as size_t,
        );
    }
    (*state).cursor = strlen(((*state).search).as_mut_ptr() as *const libc::c_char);
    update_search(state);
}
static mut __constr_expr_0: [libc::c_char; 2] = [
    8 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_1: [libc::c_char; 2] = [
    23 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_2: [libc::c_char; 2] = [
    21 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_3: [libc::c_char; 2] = [
    9 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_4: [libc::c_char; 2] = [
    3 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_5: [libc::c_char; 2] = [
    4 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_6: [libc::c_char; 2] = [
    7 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_7: [libc::c_char; 2] = [
    13 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_8: [libc::c_char; 2] = [
    16 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_9: [libc::c_char; 2] = [
    14 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_10: [libc::c_char; 2] = [
    11 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_11: [libc::c_char; 2] = [
    10 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_12: [libc::c_char; 2] = [
    1 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut __constr_expr_13: [libc::c_char; 2] = [
    5 as libc::c_int as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut keybindings: [keybinding_t; 33] = unsafe {
    [
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B\0" as *const u8 as *const libc::c_char,
                action: Some(action_exit as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x7F\0" as *const u8 as *const libc::c_char,
                action: Some(action_del_char as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_0.as_ptr(),
                action: Some(action_del_char as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_1.as_ptr(),
                action: Some(action_del_word as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_2.as_ptr(),
                action: Some(action_del_all as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_3.as_ptr(),
                action: Some(
                    action_autocomplete as unsafe extern "C" fn(*mut tty_interface_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_4.as_ptr(),
                action: Some(action_exit as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_5.as_ptr(),
                action: Some(action_exit as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_6.as_ptr(),
                action: Some(action_exit as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_7.as_ptr(),
                action: Some(action_emit as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_8.as_ptr(),
                action: Some(action_prev as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_9.as_ptr(),
                action: Some(action_next as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_10.as_ptr(),
                action: Some(action_prev as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_11.as_ptr(),
                action: Some(action_next as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_12.as_ptr(),
                action: Some(action_beginning as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: __constr_expr_13.as_ptr(),
                action: Some(action_end as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1BOD\0" as *const u8 as *const libc::c_char,
                action: Some(action_left as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[D\0" as *const u8 as *const libc::c_char,
                action: Some(action_left as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1BOC\0" as *const u8 as *const libc::c_char,
                action: Some(action_right as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[C\0" as *const u8 as *const libc::c_char,
                action: Some(action_right as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[1~\0" as *const u8 as *const libc::c_char,
                action: Some(action_beginning as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[H\0" as *const u8 as *const libc::c_char,
                action: Some(action_beginning as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[4~\0" as *const u8 as *const libc::c_char,
                action: Some(action_end as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[F\0" as *const u8 as *const libc::c_char,
                action: Some(action_end as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[A\0" as *const u8 as *const libc::c_char,
                action: Some(action_prev as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1BOA\0" as *const u8 as *const libc::c_char,
                action: Some(action_prev as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[B\0" as *const u8 as *const libc::c_char,
                action: Some(action_next as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1BOB\0" as *const u8 as *const libc::c_char,
                action: Some(action_next as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[5~\0" as *const u8 as *const libc::c_char,
                action: Some(action_pageup as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[6~\0" as *const u8 as *const libc::c_char,
                action: Some(action_pagedown as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[200~\0" as *const u8 as *const libc::c_char,
                action: Some(action_ignore as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: b"\x1B[201~\0" as *const u8 as *const libc::c_char,
                action: Some(action_ignore as unsafe extern "C" fn(*mut tty_interface_t) -> ()),
            };
            init
        },
        {
            let mut init = __anonstruct_keybinding_t_984224788 {
                key: 0 as *const libc::c_void as *mut libc::c_void as *const libc::c_char,
                action: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option<unsafe extern "C" fn(*mut tty_interface_t) -> ()>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
    ]
};
unsafe extern "C" fn handle_input(
    mut state: *mut tty_interface_t,
    mut s: *const libc::c_char,
    mut handle_ambiguous_key: libc::c_int,
) {
    let mut input: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut found_keybinding: libc::c_int = 0;
    let mut in_middle: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    (*state).ambiguous_key_pending = 0 as libc::c_int;
    input = ((*state).input).as_mut_ptr();
    strcat(((*state).input).as_mut_ptr(), s);
    found_keybinding = -(1 as libc::c_int);
    in_middle = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while !(keybindings[i as usize].key).is_null() {
        tmp___1 = strcmp(input as *const libc::c_char, keybindings[i as usize].key);
        if tmp___1 != 0 {
            tmp = strlen(((*state).input).as_mut_ptr() as *const libc::c_char);
            tmp___0 = strncmp(
                input as *const libc::c_char,
                keybindings[i as usize].key,
                tmp,
            );
            if tmp___0 == 0 {
                in_middle = 1 as libc::c_int;
            }
        } else {
            found_keybinding = i;
        }
        i += 1;
    }
    if found_keybinding != -(1 as libc::c_int) {
        if in_middle == 0 {
            (Some(
                ((*keybindings.as_ptr().offset(found_keybinding as isize)).action)
                    .expect("non-null function pointer"),
            ))
            .expect("non-null function pointer")(state);
            strcpy(input, b"\0" as *const u8 as *const libc::c_char);
            return;
        } else {
            if handle_ambiguous_key != 0 {
                (Some(
                    ((*keybindings.as_ptr().offset(found_keybinding as isize)).action)
                        .expect("non-null function pointer"),
                ))
                .expect("non-null function pointer")(state);
                strcpy(input, b"\0" as *const u8 as *const libc::c_char);
                return;
            }
        }
    }
    if found_keybinding != -(1 as libc::c_int) {
        if in_middle != 0 {
            (*state).ambiguous_key_pending = 1 as libc::c_int;
            return;
        }
    }
    if in_middle != 0 {
        return;
    }
    i___0 = 0 as libc::c_int;
    while *input.offset(i___0 as isize) != 0 {
        tmp___2 = isprint_unicode(*input.offset(i___0 as isize));
        if tmp___2 != 0 {
            append_search(state, *input.offset(i___0 as isize));
        }
        i___0 += 1;
    }
    strcpy(input, b"\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn tty_interface_run(mut state: *mut tty_interface_t) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut s: [libc::c_char; 2] = [0; 2];
    let mut tmp___0: libc::c_char = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut s___0: [libc::c_char; 1] = [0; 1];
    draw(state);
    loop {
        loop {
            loop {
                tmp = tty_input_ready((*state).tty, -(1 as libc::c_long), 1 as libc::c_int);
                if tmp != 0 {
                    break;
                }
                draw(state);
            }
            tmp___0 = tty_getchar((*state).tty);
            s[0 as libc::c_int as usize] = tmp___0;
            s[1 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
            handle_input(
                state,
                s.as_mut_ptr() as *const libc::c_char,
                0 as libc::c_int,
            );
            if (*state).exit >= 0 as libc::c_int {
                return (*state).exit;
            }
            draw(state);
            if (*state).ambiguous_key_pending != 0 {
                tmp___1 = 25 as libc::c_int;
            } else {
                tmp___1 = 0 as libc::c_int;
            }
            tmp___2 = tty_input_ready((*state).tty, tmp___1 as libc::c_long, 0 as libc::c_int);
            if tmp___2 == 0 {
                break;
            }
        }
        if (*state).ambiguous_key_pending != 0 {
            s___0[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
            handle_input(
                state,
                s___0.as_mut_ptr() as *const libc::c_char,
                1 as libc::c_int,
            );
            if (*state).exit >= 0 as libc::c_int {
                return (*state).exit;
            }
        }
        update_state(state);
    }
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
