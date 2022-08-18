use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type fftw_plan_s;
    pub type ldat;
    pub type _snd_pcm_hw_params;
    pub type _snd_pcm;
    pub type pa_defer_event;
    pub type pa_time_event;
    pub type pa_io_event;
    pub type pa_operation;
    pub type pa_context;
    pub type pa_mainloop;
    pub type pa_simple;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn log10(_: libc::c_double) -> libc::c_double;
    fn log2(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn hypot(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn fftw_execute(p_0: fftw_plan);
    fn fftw_plan_dft_r2c_1d(
        n: libc::c_int,
        in_0: *mut libc::c_double,
        out: *mut fftw_complex,
        flags: libc::c_uint,
    ) -> fftw_plan;
    fn fftw_destroy_plan(p_0: fftw_plan);
    fn fftw_alloc_real(n: size_t) -> *mut libc::c_double;
    fn fftw_alloc_complex(n: size_t) -> *mut fftw_complex;
    fn fftw_free(p_0: *mut libc::c_void);
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
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
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn signal(
        __sig: libc::c_int,
        __handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn ttyname(__fd: libc::c_int) -> *mut libc::c_char;
    fn mvprintw(
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn wgetch(_: *mut WINDOW) -> libc::c_int;
    static mut stdscr: *mut WINDOW;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn iniparser_getsecnkeys(
        d: *const dictionary,
        s: *const libc::c_char,
    ) -> libc::c_int;
    fn iniparser_getseckeys(
        d: *const dictionary,
        s: *const libc::c_char,
        keys: *mut *const libc::c_char,
    ) -> *mut *const libc::c_char;
    fn iniparser_getstring(
        d: *const dictionary,
        key: *const libc::c_char,
        def: *const libc::c_char,
    ) -> *const libc::c_char;
    fn iniparser_getint(
        d: *const dictionary,
        key: *const libc::c_char,
        notfound: libc::c_int,
    ) -> libc::c_int;
    fn iniparser_getdouble(
        d: *const dictionary,
        key: *const libc::c_char,
        notfound: libc::c_double,
    ) -> libc::c_double;
    fn iniparser_load(ininame: *const libc::c_char) -> *mut dictionary;
    fn iniparser_freedict(d: *mut dictionary);
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn shm_open(
        __name: *const libc::c_char,
        __oflag: libc::c_int,
        __mode: mode_t,
    ) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
    fn tcgetattr(__fd: libc::c_int, __termios_p: *mut termios) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn wcscat(__dest: *mut wchar_t, __src: *const wchar_t) -> *mut wchar_t;
    fn swprintf(
        __s: *mut wchar_t,
        __n: size_t,
        __format: *const wchar_t,
        _: ...
    ) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn snd_strerror(errnum: libc::c_int) -> *const libc::c_char;
    fn snd_pcm_open(
        pcm: *mut *mut snd_pcm_t,
        name: *const libc::c_char,
        stream: snd_pcm_stream_t,
        mode: libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_close(pcm: *mut snd_pcm_t) -> libc::c_int;
    fn snd_pcm_hw_params(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
    ) -> libc::c_int;
    fn snd_pcm_prepare(pcm: *mut snd_pcm_t) -> libc::c_int;
    fn snd_pcm_readi(
        pcm: *mut snd_pcm_t,
        buffer: *mut libc::c_void,
        size: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
    fn snd_pcm_get_params(
        pcm: *mut snd_pcm_t,
        buffer_size: *mut snd_pcm_uframes_t,
        period_size: *mut snd_pcm_uframes_t,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_any(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_sizeof() -> size_t;
    fn snd_pcm_hw_params_set_access(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        _access: snd_pcm_access_t,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_get_format(
        params: *const snd_pcm_hw_params_t,
        val: *mut snd_pcm_format_t,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_set_format(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: snd_pcm_format_t,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_set_channels(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: libc::c_uint,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_get_rate(
        params: *const snd_pcm_hw_params_t,
        val: *mut libc::c_uint,
        dir: *mut libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_set_rate_near(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: *mut libc::c_uint,
        dir: *mut libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_get_period_size(
        params: *const snd_pcm_hw_params_t,
        frames: *mut snd_pcm_uframes_t,
        dir: *mut libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_set_period_size_near(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: *mut snd_pcm_uframes_t,
        dir: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pa_strerror(error: libc::c_int) -> *const libc::c_char;
    fn pa_operation_unref(o: *mut pa_operation);
    fn pa_context_new(
        mainloop: *mut pa_mainloop_api,
        name: *const libc::c_char,
    ) -> *mut pa_context;
    fn pa_context_unref(c: *mut pa_context);
    fn pa_context_set_state_callback(
        c: *mut pa_context,
        cb_0: Option::<unsafe extern "C" fn(*mut pa_context, *mut libc::c_void) -> ()>,
        userdata: *mut libc::c_void,
    );
    fn pa_context_get_state(c: *const pa_context) -> pa_context_state_t;
    fn pa_context_connect(
        c: *mut pa_context,
        server: *const libc::c_char,
        flags: pa_context_flags_t,
        api: *const pa_spawn_api,
    ) -> libc::c_int;
    fn pa_context_disconnect(c: *mut pa_context);
    fn pa_context_get_server_info(
        c: *mut pa_context,
        cb_0: Option::<
            unsafe extern "C" fn(
                *mut pa_context,
                *const pa_server_info,
                *mut libc::c_void,
            ) -> (),
        >,
        userdata: *mut libc::c_void,
    ) -> *mut pa_operation;
    fn pa_mainloop_new() -> *mut pa_mainloop;
    fn pa_mainloop_free(m: *mut pa_mainloop);
    fn pa_mainloop_iterate(
        m: *mut pa_mainloop,
        block: libc::c_int,
        retval: *mut libc::c_int,
    ) -> libc::c_int;
    fn pa_mainloop_run(m: *mut pa_mainloop, retval: *mut libc::c_int) -> libc::c_int;
    fn pa_mainloop_get_api(m: *mut pa_mainloop) -> *mut pa_mainloop_api;
    fn pa_mainloop_quit(m: *mut pa_mainloop, retval: libc::c_int);
    fn pa_simple_new(
        server: *const libc::c_char,
        name: *const libc::c_char,
        dir: pa_stream_direction_t,
        dev: *const libc::c_char,
        stream_name: *const libc::c_char,
        ss_0: *const pa_sample_spec,
        map: *const pa_channel_map,
        attr: *const pa_buffer_attr,
        error: *mut libc::c_int,
    ) -> *mut pa_simple;
    fn pa_simple_free(s: *mut pa_simple);
    fn pa_simple_read(
        s: *mut pa_simple,
        data: *mut libc::c_void,
        bytes: size_t,
        error: *mut libc::c_int,
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn can_change_color() -> bool;
    fn curs_set(_: libc::c_int) -> libc::c_int;
    fn echo() -> libc::c_int;
    fn endwin() -> libc::c_int;
    fn initscr() -> *mut WINDOW;
    fn init_color(
        _: libc::c_short,
        _: libc::c_short,
        _: libc::c_short,
        _: libc::c_short,
    ) -> libc::c_int;
    fn init_pair(_: libc::c_short, _: libc::c_short, _: libc::c_short) -> libc::c_int;
    fn noecho() -> libc::c_int;
    fn start_color() -> libc::c_int;
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn wattrset(_: *mut WINDOW, _: libc::c_int) -> libc::c_int;
    fn wattr_on(_: *mut WINDOW, _: attr_t, _: *mut libc::c_void) -> libc::c_int;
    fn wbkgd(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn wclear(_: *mut WINDOW) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    fn wtimeout(_: *mut WINDOW, _: libc::c_int);
    fn use_default_colors() -> libc::c_int;
    static mut COLORS: libc::c_int;
    static mut COLOR_PAIRS: libc::c_int;
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
    fn waddnwstr(_: *mut WINDOW, _: *const wchar_t, _: libc::c_int) -> libc::c_int;
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
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type wchar_t = libc::c_int;
pub type __gnuc_va_list = __builtin_va_list;
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
pub struct __anonstruct___sigset_t_991265788 {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = __anonstruct___sigset_t_991265788;
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
pub type DIR = __dirstream;
pub type fftw_complex = [libc::c_double; 2];
pub type fftw_plan = *mut fftw_plan_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__kill_244518854 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__timer_490064738 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__rt_619254530 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigchld_284671705 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__addr_bnd_5259977 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion__bounds_401083816 {
    pub _addr_bnd: __anonstruct__addr_bnd_5259977,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigfault_1009314882 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: __anonunion__bounds_401083816,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigpoll_386613454 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigsys_44812255 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion__sifields_289080392 {
    pub _pad: [libc::c_int; 28],
    pub _kill: __anonstruct__kill_244518854,
    pub _timer: __anonstruct__timer_490064738,
    pub _rt: __anonstruct__rt_619254530,
    pub _sigchld: __anonstruct__sigchld_284671705,
    pub _sigfault: __anonstruct__sigfault_1009314882,
    pub _sigpoll: __anonstruct__sigpoll_386613454,
    pub _sigsys: __anonstruct__sigsys_44812255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_siginfo_t_292364137 {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: __anonunion__sifields_289080392,
}
pub type siginfo_t = __anonstruct_siginfo_t_292364137;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion___sigaction_handler_363639592 {
    pub sa_handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: __anonunion___sigaction_handler_363639592,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
pub type chtype = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
    pub _bkgrnd: cchar_t,
    pub _color: libc::c_int,
}
pub type cchar_t = __anonstruct_cchar_t_667532103;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_cchar_t_667532103 {
    pub attr: attr_t,
    pub chars: [wchar_t; 5],
    pub ext_color: libc::c_int,
}
pub type attr_t = chtype;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub type WINDOW = _win_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_data {
    pub FFTbassbufferSize: libc::c_int,
    pub FFTmidbufferSize: libc::c_int,
    pub FFTtreblebufferSize: libc::c_int,
    pub bass_index: libc::c_int,
    pub mid_index: libc::c_int,
    pub treble_index: libc::c_int,
    pub bass_multiplier: *mut libc::c_double,
    pub mid_multiplier: *mut libc::c_double,
    pub treble_multiplier: *mut libc::c_double,
    pub in_bass_r_raw: *mut libc::c_double,
    pub in_bass_l_raw: *mut libc::c_double,
    pub in_mid_r_raw: *mut libc::c_double,
    pub in_mid_l_raw: *mut libc::c_double,
    pub in_treble_r_raw: *mut libc::c_double,
    pub in_treble_l_raw: *mut libc::c_double,
    pub in_bass_r: *mut libc::c_double,
    pub in_bass_l: *mut libc::c_double,
    pub in_mid_r: *mut libc::c_double,
    pub in_mid_l: *mut libc::c_double,
    pub in_treble_r: *mut libc::c_double,
    pub in_treble_l: *mut libc::c_double,
    pub format: libc::c_int,
    pub rate: libc::c_uint,
    pub source: *mut libc::c_char,
    pub im: libc::c_int,
    pub channels: libc::c_uint,
    pub left: bool,
    pub right: bool,
    pub average: bool,
    pub terminate: libc::c_int,
    pub error_message: [libc::c_char; 1024],
}
pub type input_method = libc::c_uint;
pub const INPUT_MAX: input_method = 6;
pub const INPUT_SHMEM: input_method = 5;
pub const INPUT_SNDIO: input_method = 4;
pub const INPUT_PULSE: input_method = 3;
pub const INPUT_ALSA: input_method = 2;
pub const INPUT_PORTAUDIO: input_method = 1;
pub const INPUT_FIFO: input_method = 0;
pub type output_method = libc::c_uint;
pub const OUTPUT_NOT_SUPORTED: output_method = 3;
pub const OUTPUT_RAW: output_method = 2;
pub const OUTPUT_NONCURSES: output_method = 1;
pub const OUTPUT_NCURSES: output_method = 0;
pub type xaxis_scale = libc::c_uint;
pub const NOTE: xaxis_scale = 2;
pub const FREQUENCY: xaxis_scale = 1;
pub const NONE: xaxis_scale = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct config_params {
    pub color: *mut libc::c_char,
    pub bcolor: *mut libc::c_char,
    pub raw_target: *mut libc::c_char,
    pub audio_source: *mut libc::c_char,
    pub gradient_colors: *mut *mut libc::c_char,
    pub data_format: *mut libc::c_char,
    pub mono_option: *mut libc::c_char,
    pub bar_delim: libc::c_char,
    pub frame_delim: libc::c_char,
    pub monstercat: libc::c_double,
    pub integral: libc::c_double,
    pub gravity: libc::c_double,
    pub ignore: libc::c_double,
    pub sens: libc::c_double,
    pub lower_cut_off: libc::c_uint,
    pub upper_cut_off: libc::c_uint,
    pub userEQ: *mut libc::c_double,
    pub input: input_method,
    pub output: output_method,
    pub xaxis: xaxis_scale,
    pub userEQ_keys: libc::c_int,
    pub userEQ_enabled: libc::c_int,
    pub col: libc::c_int,
    pub bgcol: libc::c_int,
    pub autobars: libc::c_int,
    pub stereo: libc::c_int,
    pub is_bin: libc::c_int,
    pub ascii_range: libc::c_int,
    pub bit_format: libc::c_int,
    pub gradient: libc::c_int,
    pub gradient_count: libc::c_int,
    pub fixedbars: libc::c_int,
    pub framerate: libc::c_int,
    pub bar_width: libc::c_int,
    pub bar_spacing: libc::c_int,
    pub autosens: libc::c_int,
    pub overshoot: libc::c_int,
    pub waves: libc::c_int,
    pub fifoSample: libc::c_int,
    pub fifoSampleBits: libc::c_int,
    pub sleep_timer: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct error_s {
    pub message: [libc::c_char; 1024],
    pub length: libc::c_int,
}
pub type __ssize_t = libc::c_long;
pub type va_list___0 = __gnuc_va_list;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _dictionary_ {
    pub n: libc::c_int,
    pub size: ssize_t,
    pub val: *mut *mut libc::c_char,
    pub key: *mut *mut libc::c_char,
    pub hash: *mut libc::c_uint,
}
pub type dictionary = _dictionary_;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type int16_t = __int16_t;
pub type uint16_t = __uint16_t;
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type mode_t = __mode_t;
pub type intptr_t = libc::c_long;
pub type time_t = __time_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_rwlock_t_656928968 {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type pthread_rwlock_t = __anonunion_pthread_rwlock_t_656928968;
pub type u32_t = libc::c_uint;
pub type s16_t = libc::c_short;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_vis_t_409133603 {
    pub rwlock: pthread_rwlock_t,
    pub buf_size: u32_t,
    pub buf_index: u32_t,
    pub running: bool,
    pub rate: u32_t,
    pub updated: time_t,
    pub buffer: [s16_t; 16384],
}
pub type vis_t = __anonstruct_vis_t_409133603;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
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
pub type __int8_t = libc::c_schar;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type snd_pcm_hw_params_t = _snd_pcm_hw_params;
pub type _snd_pcm_stream = libc::c_uint;
pub const SND_PCM_STREAM_LAST: _snd_pcm_stream = 1;
pub const SND_PCM_STREAM_CAPTURE: _snd_pcm_stream = 1;
pub const SND_PCM_STREAM_PLAYBACK: _snd_pcm_stream = 0;
pub type snd_pcm_stream_t = _snd_pcm_stream;
pub type _snd_pcm_access = libc::c_uint;
pub const SND_PCM_ACCESS_LAST: _snd_pcm_access = 4;
pub const SND_PCM_ACCESS_RW_NONINTERLEAVED: _snd_pcm_access = 4;
pub const SND_PCM_ACCESS_RW_INTERLEAVED: _snd_pcm_access = 3;
pub const SND_PCM_ACCESS_MMAP_COMPLEX: _snd_pcm_access = 2;
pub const SND_PCM_ACCESS_MMAP_NONINTERLEAVED: _snd_pcm_access = 1;
pub const SND_PCM_ACCESS_MMAP_INTERLEAVED: _snd_pcm_access = 0;
pub type snd_pcm_access_t = _snd_pcm_access;
pub type _snd_pcm_format = libc::c_int;
pub const SND_PCM_FORMAT_U20: _snd_pcm_format = 27;
pub const SND_PCM_FORMAT_S20: _snd_pcm_format = 25;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME: _snd_pcm_format = 18;
pub const SND_PCM_FORMAT_FLOAT64: _snd_pcm_format = 16;
pub const SND_PCM_FORMAT_FLOAT: _snd_pcm_format = 14;
pub const SND_PCM_FORMAT_U32: _snd_pcm_format = 12;
pub const SND_PCM_FORMAT_S32: _snd_pcm_format = 10;
pub const SND_PCM_FORMAT_U24: _snd_pcm_format = 8;
pub const SND_PCM_FORMAT_S24: _snd_pcm_format = 6;
pub const SND_PCM_FORMAT_U16: _snd_pcm_format = 4;
pub const SND_PCM_FORMAT_S16: _snd_pcm_format = 2;
pub const SND_PCM_FORMAT_LAST: _snd_pcm_format = 52;
pub const SND_PCM_FORMAT_DSD_U32_BE: _snd_pcm_format = 52;
pub const SND_PCM_FORMAT_DSD_U16_BE: _snd_pcm_format = 51;
pub const SND_PCM_FORMAT_DSD_U32_LE: _snd_pcm_format = 50;
pub const SND_PCM_FORMAT_DSD_U16_LE: _snd_pcm_format = 49;
pub const SND_PCM_FORMAT_DSD_U8: _snd_pcm_format = 48;
pub const SND_PCM_FORMAT_G723_40_1B: _snd_pcm_format = 47;
pub const SND_PCM_FORMAT_G723_40: _snd_pcm_format = 46;
pub const SND_PCM_FORMAT_G723_24_1B: _snd_pcm_format = 45;
pub const SND_PCM_FORMAT_G723_24: _snd_pcm_format = 44;
pub const SND_PCM_FORMAT_U18_3BE: _snd_pcm_format = 43;
pub const SND_PCM_FORMAT_U18_3LE: _snd_pcm_format = 42;
pub const SND_PCM_FORMAT_S18_3BE: _snd_pcm_format = 41;
pub const SND_PCM_FORMAT_S18_3LE: _snd_pcm_format = 40;
pub const SND_PCM_FORMAT_U20_3BE: _snd_pcm_format = 39;
pub const SND_PCM_FORMAT_U20_3LE: _snd_pcm_format = 38;
pub const SND_PCM_FORMAT_S20_3BE: _snd_pcm_format = 37;
pub const SND_PCM_FORMAT_S20_3LE: _snd_pcm_format = 36;
pub const SND_PCM_FORMAT_U24_3BE: _snd_pcm_format = 35;
pub const SND_PCM_FORMAT_U24_3LE: _snd_pcm_format = 34;
pub const SND_PCM_FORMAT_S24_3BE: _snd_pcm_format = 33;
pub const SND_PCM_FORMAT_S24_3LE: _snd_pcm_format = 32;
pub const SND_PCM_FORMAT_SPECIAL: _snd_pcm_format = 31;
pub const SND_PCM_FORMAT_U20_BE: _snd_pcm_format = 28;
pub const SND_PCM_FORMAT_U20_LE: _snd_pcm_format = 27;
pub const SND_PCM_FORMAT_S20_BE: _snd_pcm_format = 26;
pub const SND_PCM_FORMAT_S20_LE: _snd_pcm_format = 25;
pub const SND_PCM_FORMAT_GSM: _snd_pcm_format = 24;
pub const SND_PCM_FORMAT_MPEG: _snd_pcm_format = 23;
pub const SND_PCM_FORMAT_IMA_ADPCM: _snd_pcm_format = 22;
pub const SND_PCM_FORMAT_A_LAW: _snd_pcm_format = 21;
pub const SND_PCM_FORMAT_MU_LAW: _snd_pcm_format = 20;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME_BE: _snd_pcm_format = 19;
pub const SND_PCM_FORMAT_IEC958_SUBFRAME_LE: _snd_pcm_format = 18;
pub const SND_PCM_FORMAT_FLOAT64_BE: _snd_pcm_format = 17;
pub const SND_PCM_FORMAT_FLOAT64_LE: _snd_pcm_format = 16;
pub const SND_PCM_FORMAT_FLOAT_BE: _snd_pcm_format = 15;
pub const SND_PCM_FORMAT_FLOAT_LE: _snd_pcm_format = 14;
pub const SND_PCM_FORMAT_U32_BE: _snd_pcm_format = 13;
pub const SND_PCM_FORMAT_U32_LE: _snd_pcm_format = 12;
pub const SND_PCM_FORMAT_S32_BE: _snd_pcm_format = 11;
pub const SND_PCM_FORMAT_S32_LE: _snd_pcm_format = 10;
pub const SND_PCM_FORMAT_U24_BE: _snd_pcm_format = 9;
pub const SND_PCM_FORMAT_U24_LE: _snd_pcm_format = 8;
pub const SND_PCM_FORMAT_S24_BE: _snd_pcm_format = 7;
pub const SND_PCM_FORMAT_S24_LE: _snd_pcm_format = 6;
pub const SND_PCM_FORMAT_U16_BE: _snd_pcm_format = 5;
pub const SND_PCM_FORMAT_U16_LE: _snd_pcm_format = 4;
pub const SND_PCM_FORMAT_S16_BE: _snd_pcm_format = 3;
pub const SND_PCM_FORMAT_S16_LE: _snd_pcm_format = 2;
pub const SND_PCM_FORMAT_U8: _snd_pcm_format = 1;
pub const SND_PCM_FORMAT_S8: _snd_pcm_format = 0;
pub const SND_PCM_FORMAT_UNKNOWN: _snd_pcm_format = -1;
pub type snd_pcm_format_t = _snd_pcm_format;
pub type snd_pcm_uframes_t = libc::c_ulong;
pub type snd_pcm_sframes_t = libc::c_long;
pub type snd_pcm_t = _snd_pcm;
pub type __suseconds_t = libc::c_long;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pa_sample_format = libc::c_int;
pub const PA_SAMPLE_INVALID: pa_sample_format = -1;
pub const PA_SAMPLE_MAX: pa_sample_format = 13;
pub const PA_SAMPLE_S24_32BE: pa_sample_format = 12;
pub const PA_SAMPLE_S24_32LE: pa_sample_format = 11;
pub const PA_SAMPLE_S24BE: pa_sample_format = 10;
pub const PA_SAMPLE_S24LE: pa_sample_format = 9;
pub const PA_SAMPLE_S32BE: pa_sample_format = 8;
pub const PA_SAMPLE_S32LE: pa_sample_format = 7;
pub const PA_SAMPLE_FLOAT32BE: pa_sample_format = 6;
pub const PA_SAMPLE_FLOAT32LE: pa_sample_format = 5;
pub const PA_SAMPLE_S16BE: pa_sample_format = 4;
pub const PA_SAMPLE_S16LE: pa_sample_format = 3;
pub const PA_SAMPLE_ULAW: pa_sample_format = 2;
pub const PA_SAMPLE_ALAW: pa_sample_format = 1;
pub const PA_SAMPLE_U8: pa_sample_format = 0;
pub type pa_sample_format_t = pa_sample_format;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_sample_spec {
    pub format: pa_sample_format_t,
    pub rate: uint32_t,
    pub channels: uint8_t,
}
pub type pa_context_state = libc::c_uint;
pub const PA_CONTEXT_TERMINATED: pa_context_state = 6;
pub const PA_CONTEXT_FAILED: pa_context_state = 5;
pub const PA_CONTEXT_READY: pa_context_state = 4;
pub const PA_CONTEXT_SETTING_NAME: pa_context_state = 3;
pub const PA_CONTEXT_AUTHORIZING: pa_context_state = 2;
pub const PA_CONTEXT_CONNECTING: pa_context_state = 1;
pub const PA_CONTEXT_UNCONNECTED: pa_context_state = 0;
pub type pa_context_state_t = pa_context_state;
pub type pa_context_flags = libc::c_uint;
pub const PA_CONTEXT_NOFAIL: pa_context_flags = 2;
pub const PA_CONTEXT_NOAUTOSPAWN: pa_context_flags = 1;
pub const PA_CONTEXT_NOFLAGS: pa_context_flags = 0;
pub type pa_context_flags_t = pa_context_flags;
pub type pa_stream_direction = libc::c_uint;
pub const PA_STREAM_UPLOAD: pa_stream_direction = 3;
pub const PA_STREAM_RECORD: pa_stream_direction = 2;
pub const PA_STREAM_PLAYBACK: pa_stream_direction = 1;
pub const PA_STREAM_NODIRECTION: pa_stream_direction = 0;
pub type pa_stream_direction_t = pa_stream_direction;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_buffer_attr {
    pub maxlength: uint32_t,
    pub tlength: uint32_t,
    pub prebuf: uint32_t,
    pub minreq: uint32_t,
    pub fragsize: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_spawn_api {
    pub prefork: Option::<unsafe extern "C" fn() -> ()>,
    pub postfork: Option::<unsafe extern "C" fn() -> ()>,
    pub atfork: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_mainloop_api {
    pub userdata: *mut libc::c_void,
    pub io_new: Option::<
        unsafe extern "C" fn(
            *mut pa_mainloop_api,
            libc::c_int,
            pa_io_event_flags_t,
            Option::<
                unsafe extern "C" fn(
                    *mut pa_mainloop_api,
                    *mut pa_io_event,
                    libc::c_int,
                    pa_io_event_flags_t,
                    *mut libc::c_void,
                ) -> (),
            >,
            *mut libc::c_void,
        ) -> *mut pa_io_event,
    >,
    pub io_enable: Option::<
        unsafe extern "C" fn(*mut pa_io_event, pa_io_event_flags_t) -> (),
    >,
    pub io_free: Option::<unsafe extern "C" fn(*mut pa_io_event) -> ()>,
    pub io_set_destroy: Option::<
        unsafe extern "C" fn(
            *mut pa_io_event,
            Option::<
                unsafe extern "C" fn(
                    *mut pa_mainloop_api,
                    *mut pa_io_event,
                    *mut libc::c_void,
                ) -> (),
            >,
        ) -> (),
    >,
    pub time_new: Option::<
        unsafe extern "C" fn(
            *mut pa_mainloop_api,
            *const timeval,
            Option::<
                unsafe extern "C" fn(
                    *mut pa_mainloop_api,
                    *mut pa_time_event,
                    *const timeval,
                    *mut libc::c_void,
                ) -> (),
            >,
            *mut libc::c_void,
        ) -> *mut pa_time_event,
    >,
    pub time_restart: Option::<
        unsafe extern "C" fn(*mut pa_time_event, *const timeval) -> (),
    >,
    pub time_free: Option::<unsafe extern "C" fn(*mut pa_time_event) -> ()>,
    pub time_set_destroy: Option::<
        unsafe extern "C" fn(
            *mut pa_time_event,
            Option::<
                unsafe extern "C" fn(
                    *mut pa_mainloop_api,
                    *mut pa_time_event,
                    *mut libc::c_void,
                ) -> (),
            >,
        ) -> (),
    >,
    pub defer_new: Option::<
        unsafe extern "C" fn(
            *mut pa_mainloop_api,
            Option::<
                unsafe extern "C" fn(
                    *mut pa_mainloop_api,
                    *mut pa_defer_event,
                    *mut libc::c_void,
                ) -> (),
            >,
            *mut libc::c_void,
        ) -> *mut pa_defer_event,
    >,
    pub defer_enable: Option::<
        unsafe extern "C" fn(*mut pa_defer_event, libc::c_int) -> (),
    >,
    pub defer_free: Option::<unsafe extern "C" fn(*mut pa_defer_event) -> ()>,
    pub defer_set_destroy: Option::<
        unsafe extern "C" fn(
            *mut pa_defer_event,
            Option::<
                unsafe extern "C" fn(
                    *mut pa_mainloop_api,
                    *mut pa_defer_event,
                    *mut libc::c_void,
                ) -> (),
            >,
        ) -> (),
    >,
    pub quit: Option::<unsafe extern "C" fn(*mut pa_mainloop_api, libc::c_int) -> ()>,
}
pub type pa_io_event_flags_t = pa_io_event_flags;
pub type pa_io_event_flags = libc::c_uint;
pub const PA_IO_EVENT_ERROR: pa_io_event_flags = 8;
pub const PA_IO_EVENT_HANGUP: pa_io_event_flags = 4;
pub const PA_IO_EVENT_OUTPUT: pa_io_event_flags = 2;
pub const PA_IO_EVENT_INPUT: pa_io_event_flags = 1;
pub const PA_IO_EVENT_NULL: pa_io_event_flags = 0;
pub type pa_channel_position = libc::c_int;
pub const PA_CHANNEL_POSITION_MAX: pa_channel_position = 51;
pub const PA_CHANNEL_POSITION_TOP_REAR_CENTER: pa_channel_position = 50;
pub const PA_CHANNEL_POSITION_TOP_REAR_RIGHT: pa_channel_position = 49;
pub const PA_CHANNEL_POSITION_TOP_REAR_LEFT: pa_channel_position = 48;
pub const PA_CHANNEL_POSITION_TOP_FRONT_CENTER: pa_channel_position = 47;
pub const PA_CHANNEL_POSITION_TOP_FRONT_RIGHT: pa_channel_position = 46;
pub const PA_CHANNEL_POSITION_TOP_FRONT_LEFT: pa_channel_position = 45;
pub const PA_CHANNEL_POSITION_TOP_CENTER: pa_channel_position = 44;
pub const PA_CHANNEL_POSITION_AUX31: pa_channel_position = 43;
pub const PA_CHANNEL_POSITION_AUX30: pa_channel_position = 42;
pub const PA_CHANNEL_POSITION_AUX29: pa_channel_position = 41;
pub const PA_CHANNEL_POSITION_AUX28: pa_channel_position = 40;
pub const PA_CHANNEL_POSITION_AUX27: pa_channel_position = 39;
pub const PA_CHANNEL_POSITION_AUX26: pa_channel_position = 38;
pub const PA_CHANNEL_POSITION_AUX25: pa_channel_position = 37;
pub const PA_CHANNEL_POSITION_AUX24: pa_channel_position = 36;
pub const PA_CHANNEL_POSITION_AUX23: pa_channel_position = 35;
pub const PA_CHANNEL_POSITION_AUX22: pa_channel_position = 34;
pub const PA_CHANNEL_POSITION_AUX21: pa_channel_position = 33;
pub const PA_CHANNEL_POSITION_AUX20: pa_channel_position = 32;
pub const PA_CHANNEL_POSITION_AUX19: pa_channel_position = 31;
pub const PA_CHANNEL_POSITION_AUX18: pa_channel_position = 30;
pub const PA_CHANNEL_POSITION_AUX17: pa_channel_position = 29;
pub const PA_CHANNEL_POSITION_AUX16: pa_channel_position = 28;
pub const PA_CHANNEL_POSITION_AUX15: pa_channel_position = 27;
pub const PA_CHANNEL_POSITION_AUX14: pa_channel_position = 26;
pub const PA_CHANNEL_POSITION_AUX13: pa_channel_position = 25;
pub const PA_CHANNEL_POSITION_AUX12: pa_channel_position = 24;
pub const PA_CHANNEL_POSITION_AUX11: pa_channel_position = 23;
pub const PA_CHANNEL_POSITION_AUX10: pa_channel_position = 22;
pub const PA_CHANNEL_POSITION_AUX9: pa_channel_position = 21;
pub const PA_CHANNEL_POSITION_AUX8: pa_channel_position = 20;
pub const PA_CHANNEL_POSITION_AUX7: pa_channel_position = 19;
pub const PA_CHANNEL_POSITION_AUX6: pa_channel_position = 18;
pub const PA_CHANNEL_POSITION_AUX5: pa_channel_position = 17;
pub const PA_CHANNEL_POSITION_AUX4: pa_channel_position = 16;
pub const PA_CHANNEL_POSITION_AUX3: pa_channel_position = 15;
pub const PA_CHANNEL_POSITION_AUX2: pa_channel_position = 14;
pub const PA_CHANNEL_POSITION_AUX1: pa_channel_position = 13;
pub const PA_CHANNEL_POSITION_AUX0: pa_channel_position = 12;
pub const PA_CHANNEL_POSITION_SIDE_RIGHT: pa_channel_position = 11;
pub const PA_CHANNEL_POSITION_SIDE_LEFT: pa_channel_position = 10;
pub const PA_CHANNEL_POSITION_FRONT_RIGHT_OF_CENTER: pa_channel_position = 9;
pub const PA_CHANNEL_POSITION_FRONT_LEFT_OF_CENTER: pa_channel_position = 8;
pub const PA_CHANNEL_POSITION_SUBWOOFER: pa_channel_position = 7;
pub const PA_CHANNEL_POSITION_LFE: pa_channel_position = 7;
pub const PA_CHANNEL_POSITION_REAR_RIGHT: pa_channel_position = 6;
pub const PA_CHANNEL_POSITION_REAR_LEFT: pa_channel_position = 5;
pub const PA_CHANNEL_POSITION_REAR_CENTER: pa_channel_position = 4;
pub const PA_CHANNEL_POSITION_CENTER: pa_channel_position = 3;
pub const PA_CHANNEL_POSITION_RIGHT: pa_channel_position = 2;
pub const PA_CHANNEL_POSITION_LEFT: pa_channel_position = 1;
pub const PA_CHANNEL_POSITION_FRONT_CENTER: pa_channel_position = 3;
pub const PA_CHANNEL_POSITION_FRONT_RIGHT: pa_channel_position = 2;
pub const PA_CHANNEL_POSITION_FRONT_LEFT: pa_channel_position = 1;
pub const PA_CHANNEL_POSITION_MONO: pa_channel_position = 0;
pub const PA_CHANNEL_POSITION_INVALID: pa_channel_position = -1;
pub type pa_channel_position_t = pa_channel_position;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_channel_map {
    pub channels: uint8_t,
    pub map: [pa_channel_position_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pa_server_info {
    pub user_name: *const libc::c_char,
    pub host_name: *const libc::c_char,
    pub server_version: *const libc::c_char,
    pub server_name: *const libc::c_char,
    pub sample_spec: pa_sample_spec,
    pub default_sink_name: *const libc::c_char,
    pub default_source_name: *const libc::c_char,
    pub cookie: uint32_t,
    pub channel_map: pa_channel_map,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct colors {
    pub color: libc::c_short,
    pub R: libc::c_short,
    pub G: libc::c_short,
    pub B: libc::c_short,
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
pub static mut lock: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __elision: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
};
pub static mut output_mode: libc::c_int = 0;
pub static mut should_reload: libc::c_int = 0 as libc::c_int;
pub static mut reload_colors: libc::c_int = 0 as libc::c_int;
pub static mut should_quit: libc::c_int = 0 as libc::c_int;
pub static mut p: config_params = config_params {
    color: 0 as *const libc::c_char as *mut libc::c_char,
    bcolor: 0 as *const libc::c_char as *mut libc::c_char,
    raw_target: 0 as *const libc::c_char as *mut libc::c_char,
    audio_source: 0 as *const libc::c_char as *mut libc::c_char,
    gradient_colors: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    data_format: 0 as *const libc::c_char as *mut libc::c_char,
    mono_option: 0 as *const libc::c_char as *mut libc::c_char,
    bar_delim: 0,
    frame_delim: 0,
    monstercat: 0.,
    integral: 0.,
    gravity: 0.,
    ignore: 0.,
    sens: 0.,
    lower_cut_off: 0,
    upper_cut_off: 0,
    userEQ: 0 as *const libc::c_double as *mut libc::c_double,
    input: INPUT_FIFO,
    output: OUTPUT_NCURSES,
    xaxis: NONE,
    userEQ_keys: 0,
    userEQ_enabled: 0,
    col: 0,
    bgcol: 0,
    autobars: 0,
    stereo: 0,
    is_bin: 0,
    ascii_range: 0,
    bit_format: 0,
    gradient: 0,
    gradient_count: 0,
    fixedbars: 0,
    framerate: 0,
    bar_width: 0,
    bar_spacing: 0,
    autosens: 0,
    overshoot: 0,
    waves: 0,
    fifoSample: 0,
    fifoSampleBits: 0,
    sleep_timer: 0,
};
pub static mut out_bass_l: *mut fftw_complex = 0 as *const fftw_complex
    as *mut fftw_complex;
pub static mut out_bass_r: *mut fftw_complex = 0 as *const fftw_complex
    as *mut fftw_complex;
pub static mut p_bass_l: fftw_plan = 0 as *const fftw_plan_s as *mut fftw_plan_s;
pub static mut p_bass_r: fftw_plan = 0 as *const fftw_plan_s as *mut fftw_plan_s;
pub static mut out_mid_l: *mut fftw_complex = 0 as *const fftw_complex
    as *mut fftw_complex;
pub static mut out_mid_r: *mut fftw_complex = 0 as *const fftw_complex
    as *mut fftw_complex;
pub static mut p_mid_l: fftw_plan = 0 as *const fftw_plan_s as *mut fftw_plan_s;
pub static mut p_mid_r: fftw_plan = 0 as *const fftw_plan_s as *mut fftw_plan_s;
pub static mut out_treble_l: *mut fftw_complex = 0 as *const fftw_complex
    as *mut fftw_complex;
pub static mut out_treble_r: *mut fftw_complex = 0 as *const fftw_complex
    as *mut fftw_complex;
pub static mut p_treble_l: fftw_plan = 0 as *const fftw_plan_s as *mut fftw_plan_s;
pub static mut p_treble_r: fftw_plan = 0 as *const fftw_plan_s as *mut fftw_plan_s;
pub unsafe extern "C" fn cleanup() {
    if output_mode == 0 as libc::c_int {
        cleanup_terminal_ncurses();
    } else if output_mode == 1 as libc::c_int {
        cleanup_terminal_noncurses();
    }
}
pub unsafe extern "C" fn sig_handler(mut sig_no: libc::c_int) {
    if sig_no == 10 as libc::c_int {
        should_reload = 1 as libc::c_int;
        return;
    }
    if sig_no == 12 as libc::c_int {
        reload_colors = 1 as libc::c_int;
        return;
    }
    cleanup();
    if sig_no == 2 as libc::c_int {
        printf(b"CTRL-C pressed -- goodbye\n\0" as *const u8 as *const libc::c_char);
    }
    signal(sig_no, None);
    raise(sig_no);
}
unsafe extern "C" fn is_loop_device_for_sure(mut text: *const libc::c_char) -> bool {
    let mut LOOPBACK_DEVICE_PREFIX: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    LOOPBACK_DEVICE_PREFIX = b"hw:Loopback,\0" as *const u8 as *const libc::c_char;
    tmp = strlen(LOOPBACK_DEVICE_PREFIX);
    tmp___0 = strncmp(text, LOOPBACK_DEVICE_PREFIX, tmp);
    return tmp___0 == 0 as libc::c_int;
}
unsafe extern "C" fn directory_exists(mut path: *const libc::c_char) -> bool {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut tmp: *mut DIR = 0 as *mut DIR;
    tmp = opendir(path);
    dir = tmp;
    if dir as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int != 0;
    }
    closedir(dir);
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn monstercat_filter(
    mut bars: *mut libc::c_int,
    mut number_of_bars: libc::c_int,
    mut waves: libc::c_int,
    mut monstercat: libc::c_double,
) -> *mut libc::c_int {
    let mut z: libc::c_int = 0;
    let mut m_y: libc::c_int = 0;
    let mut de: libc::c_int = 0;
    let mut _a: libc::c_double = 0.;
    let mut tmp___0: libc::c_double = 0.;
    let mut _b: libc::c_int = 0;
    let mut tmp___1: libc::c_double = 0.;
    let mut _a___0: libc::c_double = 0.;
    let mut tmp___3: libc::c_double = 0.;
    let mut _b___0: libc::c_int = 0;
    let mut tmp___4: libc::c_double = 0.;
    let mut _a___1: libc::c_double = 0.;
    let mut tmp___6: libc::c_double = 0.;
    let mut _b___1: libc::c_int = 0;
    let mut tmp___7: libc::c_double = 0.;
    let mut _a___2: libc::c_double = 0.;
    let mut tmp___9: libc::c_double = 0.;
    let mut _b___2: libc::c_int = 0;
    let mut tmp___10: libc::c_double = 0.;
    if waves > 0 as libc::c_int {
        z = 0 as libc::c_int;
        while z < number_of_bars {
            *bars
                .offset(
                    z as isize,
                ) = (*bars.offset(z as isize) as libc::c_double / 1.25f64)
                as libc::c_int;
            m_y = z - 1 as libc::c_int;
            while m_y >= 0 as libc::c_int {
                de = z - m_y;
                tmp___0 = pow(de as libc::c_double, 2 as libc::c_int as libc::c_double);
                _a = *bars.offset(z as isize) as libc::c_double - tmp___0;
                _b = *bars.offset(m_y as isize);
                if _a > _b as libc::c_double {
                    tmp___1 = _a;
                } else {
                    tmp___1 = _b as libc::c_double;
                }
                *bars.offset(m_y as isize) = tmp___1 as libc::c_int;
                m_y -= 1;
            }
            m_y = z + 1 as libc::c_int;
            while m_y < number_of_bars {
                de = m_y - z;
                tmp___3 = pow(de as libc::c_double, 2 as libc::c_int as libc::c_double);
                _a___0 = *bars.offset(z as isize) as libc::c_double - tmp___3;
                _b___0 = *bars.offset(m_y as isize);
                if _a___0 > _b___0 as libc::c_double {
                    tmp___4 = _a___0;
                } else {
                    tmp___4 = _b___0 as libc::c_double;
                }
                *bars.offset(m_y as isize) = tmp___4 as libc::c_int;
                m_y += 1;
            }
            z += 1;
        }
    } else if monstercat > 0 as libc::c_int as libc::c_double {
        z = 0 as libc::c_int;
        while z < number_of_bars {
            m_y = z - 1 as libc::c_int;
            while m_y >= 0 as libc::c_int {
                de = z - m_y;
                tmp___6 = pow(monstercat, de as libc::c_double);
                _a___1 = *bars.offset(z as isize) as libc::c_double / tmp___6;
                _b___1 = *bars.offset(m_y as isize);
                if _a___1 > _b___1 as libc::c_double {
                    tmp___7 = _a___1;
                } else {
                    tmp___7 = _b___1 as libc::c_double;
                }
                *bars.offset(m_y as isize) = tmp___7 as libc::c_int;
                m_y -= 1;
            }
            m_y = z + 1 as libc::c_int;
            while m_y < number_of_bars {
                de = m_y - z;
                tmp___9 = pow(monstercat, de as libc::c_double);
                _a___2 = *bars.offset(z as isize) as libc::c_double / tmp___9;
                _b___2 = *bars.offset(m_y as isize);
                if _a___2 > _b___2 as libc::c_double {
                    tmp___10 = _a___2;
                } else {
                    tmp___10 = _b___2 as libc::c_double;
                }
                *bars.offset(m_y as isize) = tmp___10 as libc::c_int;
                m_y += 1;
            }
            z += 1;
        }
    }
    return bars;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut configPath: [libc::c_char; 4096] = [0; 4096];
    let mut action: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    let mut usage: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut error: error_s = error_s {
        message: [0; 1024],
        length: 0,
    };
    let mut tmp: bool = false;
    let mut first: bool = false;
    let mut inAtty: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: libc::c_int = 0;
    let mut font_file: *mut FILE = 0 as *mut FILE;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bars_left: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut bars_right: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut temp_l: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut temp_r: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut bass_cut_off: libc::c_int = 0;
    let mut treble_cut_off: libc::c_int = 0;
    let mut audio: audio_data = audio_data {
        FFTbassbufferSize: 0,
        FFTmidbufferSize: 0,
        FFTtreblebufferSize: 0,
        bass_index: 0,
        mid_index: 0,
        treble_index: 0,
        bass_multiplier: 0 as *mut libc::c_double,
        mid_multiplier: 0 as *mut libc::c_double,
        treble_multiplier: 0 as *mut libc::c_double,
        in_bass_r_raw: 0 as *mut libc::c_double,
        in_bass_l_raw: 0 as *mut libc::c_double,
        in_mid_r_raw: 0 as *mut libc::c_double,
        in_mid_l_raw: 0 as *mut libc::c_double,
        in_treble_r_raw: 0 as *mut libc::c_double,
        in_treble_l_raw: 0 as *mut libc::c_double,
        in_bass_r: 0 as *mut libc::c_double,
        in_bass_l: 0 as *mut libc::c_double,
        in_mid_r: 0 as *mut libc::c_double,
        in_mid_l: 0 as *mut libc::c_double,
        in_treble_r: 0 as *mut libc::c_double,
        in_treble_l: 0 as *mut libc::c_double,
        format: 0,
        rate: 0,
        source: 0 as *mut libc::c_char,
        im: 0,
        channels: 0,
        left: false,
        right: false,
        average: false,
        terminate: 0,
        error_message: [0; 1024],
    };
    let mut tmp___7: size_t = 0;
    let mut tmp___8: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___13: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___14: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___15: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___16: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___17: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___18: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut tmp___19: libc::c_double = 0.;
    let mut i___0: libc::c_int = 0;
    let mut tmp___20: libc::c_double = 0.;
    let mut i___1: libc::c_int = 0;
    let mut tmp___21: libc::c_double = 0.;
    let mut p_thread: pthread_t = 0;
    let mut timeout_counter: libc::c_int = 0;
    let mut timeout_timer: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut thr_id: libc::c_int = 0;
    let mut tmp___22: bool = false;
    let mut tmp___23: bool = false;
    let mut tmp___24: bool = false;
    let mut tmp___25: libc::c_int = 0;
    let mut bars: [libc::c_int; 256] = [0; 256];
    let mut bars_mem: [libc::c_int; 256] = [0; 256];
    let mut bars_last: [libc::c_int; 256] = [0; 256];
    let mut previous_frame: [libc::c_int; 256] = [0; 256];
    let mut fall: [libc::c_int; 256] = [0; 256];
    let mut bars_peak: [libc::c_float; 256] = [0.; 256];
    let mut height: libc::c_int = 0;
    let mut lines: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut remainder___0: libc::c_int = 0;
    let mut fp: libc::c_int = 0;
    let mut reloadConf: bool = false;
    let mut n: libc::c_int = 0;
    let mut fptest: libc::c_int = 0;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: libc::c_int = 0;
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: libc::c_double = 0.;
    let mut tmp___30: libc::c_int = 0;
    let mut number_of_bars: libc::c_int = 0;
    let mut g: libc::c_float = 0.;
    let mut tmp___31: libc::c_double = 0.;
    let mut integral: libc::c_double = 0.;
    let mut tmp___32: libc::c_double = 0.;
    let mut tmp___33: libc::c_double = 0.;
    let mut userEQ_keys_to_bars_ratio: libc::c_double = 0.;
    let mut frequency_constant: libc::c_double = 0.;
    let mut tmp___34: libc::c_double = 0.;
    let mut cut_off_frequency: [libc::c_float; 256] = [0.; 256];
    let mut upper_cut_off_frequency: [libc::c_float; 256] = [0.; 256];
    let mut relative_cut_off: [libc::c_float; 256] = [0.; 256];
    let mut center_frequencies: [libc::c_double; 256] = [0.; 256];
    let mut FFTbuffer_lower_cut_off: [libc::c_int; 256] = [0; 256];
    let mut FFTbuffer_upper_cut_off: [libc::c_int; 256] = [0; 256];
    let mut eq: [libc::c_double; 256] = [0.; 256];
    let mut bass_cut_off_bar: libc::c_int = 0;
    let mut treble_cut_off_bar: libc::c_int = 0;
    let mut first_bar: bool = false;
    let mut first_treble_bar: libc::c_int = 0;
    let mut n___0: libc::c_int = 0;
    let mut bar_distribution_coefficient: libc::c_double = 0.;
    let mut tmp___35: libc::c_double = 0.;
    let mut tmp___36: libc::c_double = 0.;
    let mut tmp___37: libc::c_double = 0.;
    let mut tmp___38: libc::c_double = 0.;
    let mut tmp___39: libc::c_double = 0.;
    let mut tmp___40: libc::c_double = 0.;
    let mut tmp___41: libc::c_double = 0.;
    let mut x_axis_info: libc::c_int = 0;
    let mut center_frequency: libc::c_double = 0.;
    let mut n___1: libc::c_int = 0;
    let mut freq_kilohz: libc::c_float = 0.;
    let mut freq_floor: libc::c_int = 0;
    let mut resizeTerminal: bool = false;
    let mut framerate_timer: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut sleep_counter: libc::c_int = 0;
    let mut silence: bool = false;
    let mut ch: libc::c_char = 0;
    let mut sleep_mode_timer: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut tmp___42: libc::c_int = 0;
    let mut error___0: error_s = error_s {
        message: [0; 1024],
        length: 0,
    };
    let mut tmp___43: bool = false;
    let mut n___2: libc::c_int = 0;
    let mut n___3: libc::c_int = 0;
    let mut i___2: libc::c_int = 0;
    let mut tmp___44: libc::c_double = 0.;
    let mut tmp___45: libc::c_double = 0.;
    let mut tmp___46: libc::c_double = 0.;
    let mut tmp___47: libc::c_double = 0.;
    let mut tmp___48: libc::c_double = 0.;
    let mut tmp___49: libc::c_double = 0.;
    let mut senselow: bool = false;
    let mut n___4: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut div___0: libc::c_double = 0.;
    let mut rc___0: libc::c_int = 0;
    printf(
        b"%c]0;%s%c\0" as *const u8 as *const libc::c_char,
        '\u{1b}' as i32,
        b"cava\0" as *const u8 as *const libc::c_char,
        '\u{7}' as i32,
    );
    configPath[0 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    memset(
        &mut action as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    action
        .__sigaction_handler
        .sa_handler = Some(sig_handler as unsafe extern "C" fn(libc::c_int) -> ());
    sigaction(
        2 as libc::c_int,
        &mut action as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sigaction(
        15 as libc::c_int,
        &mut action as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sigaction(
        10 as libc::c_int,
        &mut action as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sigaction(
        12 as libc::c_int,
        &mut action as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    usage = b"\nUsage : cava [options]\nVisualize audio input in terminal. \n\nOptions:\n\t-p          path to config file\n\t-v          print version\n\nKeys:\n        Up        Increase sensitivity\n        Down      Decrease sensitivity\n        Left      Decrease number of bars\n        Right     Increase number of bars\n        r         Reload config\n        c         Reload colors only\n        f         Cycle foreground color\n        b         Cycle background color\n        q         Quit\n\nas of 0.4.0 all options are specified in config file, see in '/home/username/.config/cava/' \n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    loop {
        c = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"p:vh\0" as *const u8 as *const libc::c_char,
        );
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        match c {
            112 => {
                snprintf(
                    configPath.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    optarg,
                );
            }
            104 => {
                printf(b"%s\0" as *const u8 as *const libc::c_char, usage);
                return 1 as libc::c_int;
            }
            63 => {
                printf(b"%s\0" as *const u8 as *const libc::c_char, usage);
                return 1 as libc::c_int;
            }
            118 => {
                printf(b"cava 0.7.4-3-g4c0acb7\n\0" as *const u8 as *const libc::c_char);
                return 0 as libc::c_int;
            }
            _ => {
                abort();
            }
        }
    }
    loop {
        error.length = 0 as libc::c_int;
        tmp = load_config(
            configPath.as_mut_ptr(),
            &mut p,
            0 as libc::c_int != 0,
            &mut error,
        );
        if !tmp {
            fprintf(
                stderr,
                b"Error loading config. %s\0" as *const u8 as *const libc::c_char,
                (error.message).as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        first = 1 as libc::c_int != 0;
        output_mode = p.output as libc::c_int;
        if output_mode != 2 as libc::c_int {
            inAtty = 0 as libc::c_int;
            tmp___0 = ttyname(0 as libc::c_int);
            tmp___1 = strncmp(
                tmp___0 as *const libc::c_char,
                b"/dev/tty\0" as *const u8 as *const libc::c_char,
                8 as libc::c_int as size_t,
            );
            if tmp___1 == 0 as libc::c_int {
                inAtty = 1 as libc::c_int;
            } else {
                tmp___2 = ttyname(0 as libc::c_int);
                tmp___3 = strcmp(
                    tmp___2 as *const libc::c_char,
                    b"/dev/console\0" as *const u8 as *const libc::c_char,
                );
                if tmp___3 == 0 as libc::c_int {
                    inAtty = 1 as libc::c_int;
                }
            }
            tmp___4 = ttyname(0 as libc::c_int);
            tmp___5 = strncmp(
                tmp___4 as *const libc::c_char,
                b"/dev/ttys\0" as *const u8 as *const libc::c_char,
                9 as libc::c_int as size_t,
            );
            if tmp___5 == 0 as libc::c_int {
                inAtty = 0 as libc::c_int;
            }
            if inAtty != 0 {
                font_file = fopen(
                    b"/usr/local/share/consolefonts/cava.psf\0" as *const u8
                        as *const libc::c_char,
                    b"r\0" as *const u8 as *const libc::c_char,
                );
                if !font_file.is_null() {
                    fclose(font_file);
                    system(
                        b"setfont /usr/local/share/consolefonts/cava.psf  >/dev/null 2>&1\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else {
                    system(
                        b"setfont cava.psf  >/dev/null 2>&1\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                system(b"setterm -blank 0\0" as *const u8 as *const libc::c_char);
            }
            tmp___6 = getenv(b"LANG\0" as *const u8 as *const libc::c_char);
            if !tmp___6.is_null() {
                setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
            } else {
                setlocale(
                    6 as libc::c_int,
                    b"en_US.utf8\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        bass_cut_off = 150 as libc::c_int;
        treble_cut_off = 2500 as libc::c_int;
        memset(
            &mut audio as *mut audio_data as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<audio_data>() as libc::c_ulong,
        );
        tmp___7 = strlen(p.audio_source as *const libc::c_char);
        tmp___8 = malloc((1 as libc::c_ulong).wrapping_add(tmp___7));
        audio.source = tmp___8 as *mut libc::c_char;
        strcpy(audio.source, p.audio_source as *const libc::c_char);
        audio.format = -(1 as libc::c_int);
        audio.rate = 0 as libc::c_uint;
        audio.FFTbassbufferSize = 4096 as libc::c_int;
        audio.FFTmidbufferSize = 2048 as libc::c_int;
        audio.FFTtreblebufferSize = 1024 as libc::c_int;
        audio.terminate = 0 as libc::c_int;
        if p.stereo != 0 {
            audio.channels = 2 as libc::c_uint;
        }
        if p.stereo == 0 {
            audio.channels = 1 as libc::c_uint;
        }
        audio.average = 0 as libc::c_int != 0;
        audio.left = 0 as libc::c_int != 0;
        audio.right = 0 as libc::c_int != 0;
        tmp___9 = strcmp(
            p.mono_option as *const libc::c_char,
            b"average\0" as *const u8 as *const libc::c_char,
        );
        if tmp___9 == 0 as libc::c_int {
            audio.average = 1 as libc::c_int != 0;
        }
        tmp___10 = strcmp(
            p.mono_option as *const libc::c_char,
            b"left\0" as *const u8 as *const libc::c_char,
        );
        if tmp___10 == 0 as libc::c_int {
            audio.left = 1 as libc::c_int != 0;
        }
        tmp___11 = strcmp(
            p.mono_option as *const libc::c_char,
            b"right\0" as *const u8 as *const libc::c_char,
        );
        if tmp___11 == 0 as libc::c_int {
            audio.right = 1 as libc::c_int != 0;
        }
        audio.bass_index = 0 as libc::c_int;
        audio.mid_index = 0 as libc::c_int;
        audio.treble_index = 0 as libc::c_int;
        tmp___12 = malloc(
            (audio.FFTbassbufferSize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        audio.bass_multiplier = tmp___12 as *mut libc::c_double;
        tmp___13 = malloc(
            (audio.FFTmidbufferSize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        audio.mid_multiplier = tmp___13 as *mut libc::c_double;
        tmp___14 = malloc(
            (audio.FFTtreblebufferSize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        audio.treble_multiplier = tmp___14 as *mut libc::c_double;
        tmp___15 = malloc(
            ((audio.FFTbassbufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        temp_l = tmp___15 as *mut libc::c_double;
        tmp___16 = malloc(
            ((audio.FFTbassbufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
        );
        temp_r = tmp___16 as *mut libc::c_double;
        tmp___17 = malloc(
            (256 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        bars_left = tmp___17 as *mut libc::c_int;
        tmp___18 = malloc(
            (256 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        bars_right = tmp___18 as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < audio.FFTbassbufferSize {
            tmp___19 = cos(
                2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                    * i as libc::c_double
                    / (audio.FFTbassbufferSize - 1 as libc::c_int) as libc::c_double,
            );
            *(audio.bass_multiplier)
                .offset(
                    i as isize,
                ) = 0.5f64 * (1 as libc::c_int as libc::c_double - tmp___19);
            i += 1;
        }
        i___0 = 0 as libc::c_int;
        while i___0 < audio.FFTmidbufferSize {
            tmp___20 = cos(
                2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                    * i___0 as libc::c_double
                    / (audio.FFTmidbufferSize - 1 as libc::c_int) as libc::c_double,
            );
            *(audio.mid_multiplier)
                .offset(
                    i___0 as isize,
                ) = 0.5f64 * (1 as libc::c_int as libc::c_double - tmp___20);
            i___0 += 1;
        }
        i___1 = 0 as libc::c_int;
        while i___1 < audio.FFTtreblebufferSize {
            tmp___21 = cos(
                2 as libc::c_int as libc::c_double * 3.14159265358979323846f64
                    * i___1 as libc::c_double
                    / (audio.FFTtreblebufferSize - 1 as libc::c_int) as libc::c_double,
            );
            *(audio.treble_multiplier)
                .offset(
                    i___1 as isize,
                ) = 0.5f64 * (1 as libc::c_int as libc::c_double - tmp___21);
            i___1 += 1;
        }
        audio.in_bass_r = fftw_alloc_real(audio.FFTbassbufferSize as size_t);
        audio.in_bass_l = fftw_alloc_real(audio.FFTbassbufferSize as size_t);
        audio.in_bass_r_raw = fftw_alloc_real(audio.FFTbassbufferSize as size_t);
        audio.in_bass_l_raw = fftw_alloc_real(audio.FFTbassbufferSize as size_t);
        out_bass_l = fftw_alloc_complex(
            (audio.FFTbassbufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
        );
        out_bass_r = fftw_alloc_complex(
            (audio.FFTbassbufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
        );
        memset(
            out_bass_l as *mut libc::c_void,
            0 as libc::c_int,
            ((audio.FFTbassbufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
        );
        memset(
            out_bass_r as *mut libc::c_void,
            0 as libc::c_int,
            ((audio.FFTbassbufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
        );
        p_bass_l = fftw_plan_dft_r2c_1d(
            audio.FFTbassbufferSize,
            audio.in_bass_l,
            out_bass_l,
            0 as libc::c_uint,
        );
        p_bass_r = fftw_plan_dft_r2c_1d(
            audio.FFTbassbufferSize,
            audio.in_bass_r,
            out_bass_r,
            0 as libc::c_uint,
        );
        audio.in_mid_r = fftw_alloc_real(audio.FFTmidbufferSize as size_t);
        audio.in_mid_l = fftw_alloc_real(audio.FFTmidbufferSize as size_t);
        audio.in_mid_r_raw = fftw_alloc_real(audio.FFTmidbufferSize as size_t);
        audio.in_mid_l_raw = fftw_alloc_real(audio.FFTmidbufferSize as size_t);
        out_mid_l = fftw_alloc_complex(
            (audio.FFTmidbufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
        );
        out_mid_r = fftw_alloc_complex(
            (audio.FFTmidbufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
        );
        memset(
            out_mid_l as *mut libc::c_void,
            0 as libc::c_int,
            ((audio.FFTmidbufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
        );
        memset(
            out_mid_r as *mut libc::c_void,
            0 as libc::c_int,
            ((audio.FFTmidbufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
        );
        p_mid_l = fftw_plan_dft_r2c_1d(
            audio.FFTmidbufferSize,
            audio.in_mid_l,
            out_mid_l,
            0 as libc::c_uint,
        );
        p_mid_r = fftw_plan_dft_r2c_1d(
            audio.FFTmidbufferSize,
            audio.in_mid_r,
            out_mid_r,
            0 as libc::c_uint,
        );
        audio.in_treble_r = fftw_alloc_real(audio.FFTtreblebufferSize as size_t);
        audio.in_treble_l = fftw_alloc_real(audio.FFTtreblebufferSize as size_t);
        audio.in_treble_r_raw = fftw_alloc_real(audio.FFTtreblebufferSize as size_t);
        audio.in_treble_l_raw = fftw_alloc_real(audio.FFTtreblebufferSize as size_t);
        out_treble_l = fftw_alloc_complex(
            (audio.FFTtreblebufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
        );
        out_treble_r = fftw_alloc_complex(
            (audio.FFTtreblebufferSize / 2 as libc::c_int + 1 as libc::c_int) as size_t,
        );
        memset(
            out_treble_l as *mut libc::c_void,
            0 as libc::c_int,
            ((audio.FFTtreblebufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
        );
        memset(
            out_treble_r as *mut libc::c_void,
            0 as libc::c_int,
            ((audio.FFTtreblebufferSize / 2 as libc::c_int + 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fftw_complex>() as libc::c_ulong),
        );
        p_treble_l = fftw_plan_dft_r2c_1d(
            audio.FFTtreblebufferSize,
            audio.in_treble_l,
            out_treble_l,
            0 as libc::c_uint,
        );
        p_treble_r = fftw_plan_dft_r2c_1d(
            audio.FFTtreblebufferSize,
            audio.in_treble_r,
            out_treble_r,
            0 as libc::c_uint,
        );
        reset_output_buffers(&mut audio);
        timeout_counter = 0 as libc::c_int;
        timeout_timer.tv_sec = 0 as libc::c_int as __time_t;
        timeout_timer.tv_nsec = 1000000 as libc::c_int as __syscall_slong_t;
        match p.input as libc::c_uint {
            2 => {
                tmp___24 = is_loop_device_for_sure(audio.source as *const libc::c_char);
                if tmp___24 {
                    tmp___23 = directory_exists(
                        b"/sys/\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___23 {
                        tmp___22 = directory_exists(
                            b"/sys/module/snd_aloop/\0" as *const u8
                                as *const libc::c_char,
                        );
                        if !tmp___22 {
                            cleanup();
                            fprintf(
                                stderr,
                                b"Linux kernel module \"snd_aloop\" does not seem to  be loaded.\nMaybe run \"sudo modprobe snd_aloop\".\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            exit(1 as libc::c_int);
                        }
                    }
                }
                thr_id = pthread_create(
                    &mut p_thread as *mut pthread_t,
                    0 as *mut libc::c_void as *const pthread_attr_t,
                    Some(
                        input_alsa
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    &mut audio as *mut audio_data as *mut libc::c_void,
                );
                timeout_counter = 0 as libc::c_int;
                loop {
                    if !(audio.format == -(1 as libc::c_int)) {
                        if !(audio.rate == 0 as libc::c_uint) {
                            break;
                        }
                    }
                    nanosleep(
                        &mut timeout_timer as *mut timespec as *const timespec,
                        0 as *mut libc::c_void as *mut timespec,
                    );
                    timeout_counter += 1;
                    if timeout_counter > 2000 as libc::c_int {
                        cleanup();
                        fprintf(
                            stderr,
                            b"could not get rate and/or format, problems with audio thread? quiting...\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                }
            }
            0 => {
                thr_id = pthread_create(
                    &mut p_thread as *mut pthread_t,
                    0 as *mut libc::c_void as *const pthread_attr_t,
                    Some(
                        input_fifo
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    &mut audio as *mut audio_data as *mut libc::c_void,
                );
                audio.rate = p.fifoSample as libc::c_uint;
                audio.format = p.fifoSampleBits;
            }
            3 => {
                tmp___25 = strcmp(
                    audio.source as *const libc::c_char,
                    b"auto\0" as *const u8 as *const libc::c_char,
                );
                if tmp___25 == 0 as libc::c_int {
                    getPulseDefaultSink(
                        &mut audio as *mut audio_data as *mut libc::c_void,
                    );
                }
                thr_id = pthread_create(
                    &mut p_thread as *mut pthread_t,
                    0 as *mut libc::c_void as *const pthread_attr_t,
                    Some(
                        input_pulse
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    &mut audio as *mut audio_data as *mut libc::c_void,
                );
                audio.rate = 44100 as libc::c_uint;
            }
            5 => {
                thr_id = pthread_create(
                    &mut p_thread as *mut pthread_t,
                    0 as *mut libc::c_void as *const pthread_attr_t,
                    Some(
                        input_shmem
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    &mut audio as *mut audio_data as *mut libc::c_void,
                );
                timeout_counter = 0 as libc::c_int;
                while audio.rate == 0 as libc::c_uint {
                    nanosleep(
                        &mut timeout_timer as *mut timespec as *const timespec,
                        0 as *mut libc::c_void as *mut timespec,
                    );
                    timeout_counter += 1;
                    if timeout_counter > 2000 as libc::c_int {
                        cleanup();
                        fprintf(
                            stderr,
                            b"could not get rate and/or format, problems with audio thread? quiting...\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                }
            }
            _ => {
                exit(1 as libc::c_int);
            }
        }
        if p.upper_cut_off > (audio.rate).wrapping_div(2 as libc::c_uint) {
            cleanup();
            fprintf(
                stderr,
                b"higher cuttoff frequency can't be higher than sample rate / 2\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        reloadConf = 0 as libc::c_int != 0;
        while !reloadConf {
            n = 0 as libc::c_int;
            while n < 256 as libc::c_int {
                bars_last[n as usize] = 0 as libc::c_int;
                previous_frame[n as usize] = 0 as libc::c_int;
                fall[n as usize] = 0 as libc::c_int;
                bars_peak[n as usize] = 0 as libc::c_int as libc::c_float;
                bars_mem[n as usize] = 0 as libc::c_int;
                bars[n as usize] = 0 as libc::c_int;
                n += 1;
            }
            if p.xaxis as libc::c_uint == 1 as libc::c_uint {
                if p.bar_width < 4 as libc::c_int {
                    p.bar_width = 4 as libc::c_int;
                }
            }
            match output_mode {
                0 => {
                    init_terminal_ncurses(
                        p.color,
                        p.bcolor,
                        p.col,
                        p.bgcol,
                        p.gradient,
                        p.gradient_count,
                        p.gradient_colors,
                        &mut width,
                        &mut lines,
                    );
                    if p.xaxis as libc::c_uint != 0 as libc::c_uint {
                        lines -= 1;
                    }
                    height = lines * 8 as libc::c_int;
                }
                1 => {
                    get_terminal_dim_noncurses(&mut width, &mut lines);
                    if p.xaxis as libc::c_uint != 0 as libc::c_uint {
                        lines -= 1;
                    }
                    init_terminal_noncurses(
                        inAtty,
                        p.col,
                        p.bgcol,
                        width,
                        lines,
                        p.bar_width,
                    );
                    height = lines * 8 as libc::c_int;
                }
                2 => {
                    tmp___28 = strcmp(
                        p.raw_target as *const libc::c_char,
                        b"/dev/stdout\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___28 != 0 as libc::c_int {
                        tmp___27 = access(
                            p.raw_target as *const libc::c_char,
                            0 as libc::c_int,
                        );
                        if tmp___27 != -(1 as libc::c_int) {
                            fptest = open(
                                p.raw_target as *const libc::c_char,
                                2048 as libc::c_int,
                                420 as libc::c_int,
                            );
                            if fptest == -(1 as libc::c_int) {
                                fprintf(
                                    stderr,
                                    b"could not open file %s for writing\n\0" as *const u8
                                        as *const libc::c_char,
                                    p.raw_target,
                                );
                                exit(1 as libc::c_int);
                            }
                        } else {
                            printf(
                                b"creating fifo %s\n\0" as *const u8 as *const libc::c_char,
                                p.raw_target,
                            );
                            tmp___26 = mkfifo(
                                p.raw_target as *const libc::c_char,
                                436 as libc::c_int as __mode_t,
                            );
                            if tmp___26 == -(1 as libc::c_int) {
                                fprintf(
                                    stderr,
                                    b"could not create fifo %s\n\0" as *const u8
                                        as *const libc::c_char,
                                    p.raw_target,
                                );
                                exit(1 as libc::c_int);
                            }
                            fptest = open(
                                p.raw_target as *const libc::c_char,
                                2048 as libc::c_int,
                                420 as libc::c_int,
                            );
                        }
                        fp = open(
                            p.raw_target as *const libc::c_char,
                            2113 as libc::c_int,
                            420 as libc::c_int,
                        );
                    } else {
                        fp = fileno(stdout);
                        fprintf(
                            stderr,
                            b"Opening stdout\n\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    if fp == -(1 as libc::c_int) {
                        fprintf(
                            stderr,
                            b"could not open file %s for writing\n\0" as *const u8
                                as *const libc::c_char,
                            p.raw_target,
                        );
                        exit(1 as libc::c_int);
                    }
                    fprintf(
                        stderr,
                        b"open file %s for writing raw output\n\0" as *const u8
                            as *const libc::c_char,
                        p.raw_target,
                    );
                    width = 256 as libc::c_int;
                    tmp___30 = strcmp(
                        p.data_format as *const libc::c_char,
                        b"binary\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___30 == 0 as libc::c_int {
                        tmp___29 = pow(
                            2 as libc::c_int as libc::c_double,
                            p.bit_format as libc::c_double,
                        );
                        height = (tmp___29 - 1 as libc::c_int as libc::c_double)
                            as libc::c_int;
                    } else {
                        height = p.ascii_range;
                    }
                }
                _ => {
                    exit(1 as libc::c_int);
                }
            }
            if p.fixedbars != 0 {
                p.autobars = 0 as libc::c_int;
                if p.fixedbars * p.bar_width + p.fixedbars * p.bar_spacing
                    - p.bar_spacing > width
                {
                    p.autobars = 1 as libc::c_int;
                }
            }
            number_of_bars = p.fixedbars;
            if p.autobars == 1 as libc::c_int {
                number_of_bars = (width + p.bar_spacing) / (p.bar_width + p.bar_spacing);
            }
            if number_of_bars < 1 as libc::c_int {
                number_of_bars = 1 as libc::c_int;
            }
            if number_of_bars > 256 as libc::c_int {
                number_of_bars = 256 as libc::c_int;
            }
            if p.stereo != 0 {
                if number_of_bars % 2 as libc::c_int != 0 as libc::c_int {
                    number_of_bars -= 1;
                }
            }
            remainder___0 = (width - number_of_bars * p.bar_width
                - number_of_bars * p.bar_spacing + p.bar_spacing) / 2 as libc::c_int;
            if remainder___0 < 0 as libc::c_int {
                remainder___0 = 0 as libc::c_int;
            }
            tmp___31 = pow(
                (60 as libc::c_int as libc::c_float / p.framerate as libc::c_float)
                    as libc::c_double,
                2.5f64,
            );
            g = (p.gravity
                * (height as libc::c_float / 2160 as libc::c_int as libc::c_float)
                    as libc::c_double * tmp___31) as libc::c_float;
            integral = p.integral;
            if height > 320 as libc::c_int {
                tmp___32 = log10(
                    (height as libc::c_float / 10 as libc::c_int as libc::c_float)
                        as libc::c_double,
                );
                tmp___33 = sqrt(tmp___32);
                integral = p.integral * 1 as libc::c_int as libc::c_double / tmp___33;
            }
            if p.stereo != 0 {
                number_of_bars /= 2 as libc::c_int;
            }
            if p.userEQ_enabled != 0 {
                if number_of_bars > 0 as libc::c_int {
                    userEQ_keys_to_bars_ratio = p.userEQ_keys as libc::c_double
                        / number_of_bars as libc::c_double;
                }
            }
            tmp___34 = log10(
                (p.lower_cut_off as libc::c_float / p.upper_cut_off as libc::c_float)
                    as libc::c_double,
            );
            frequency_constant = tmp___34
                / (1 as libc::c_int as libc::c_float
                    / (number_of_bars as libc::c_float
                        + 1 as libc::c_int as libc::c_float)
                    - 1 as libc::c_int as libc::c_float) as libc::c_double;
            bass_cut_off_bar = -(1 as libc::c_int);
            treble_cut_off_bar = -(1 as libc::c_int);
            first_bar = 1 as libc::c_int != 0;
            first_treble_bar = 0 as libc::c_int;
            let vla = (number_of_bars + 1 as libc::c_int) as usize;
            let mut bar_buffer: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
            n___0 = 0 as libc::c_int;
            while n___0 < number_of_bars + 1 as libc::c_int {
                bar_distribution_coefficient = frequency_constant
                    * -(1 as libc::c_int) as libc::c_double;
                bar_distribution_coefficient
                    += ((n___0 as libc::c_float + 1 as libc::c_int as libc::c_float)
                        / (number_of_bars as libc::c_float
                            + 1 as libc::c_int as libc::c_float)) as libc::c_double
                        * frequency_constant;
                tmp___35 = pow(
                    10 as libc::c_int as libc::c_double,
                    bar_distribution_coefficient,
                );
                cut_off_frequency[n___0
                    as usize] = (p.upper_cut_off as libc::c_double * tmp___35)
                    as libc::c_float;
                if n___0 > 0 as libc::c_int {
                    if cut_off_frequency[(n___0 - 1 as libc::c_int) as usize]
                        >= cut_off_frequency[n___0 as usize]
                    {
                        if cut_off_frequency[(n___0 - 1 as libc::c_int) as usize]
                            > bass_cut_off as libc::c_float
                        {
                            cut_off_frequency[n___0
                                as usize] = cut_off_frequency[(n___0 - 1 as libc::c_int)
                                as usize]
                                + (cut_off_frequency[(n___0 - 1 as libc::c_int) as usize]
                                    - cut_off_frequency[(n___0 - 2 as libc::c_int) as usize]);
                        }
                    }
                }
                relative_cut_off[n___0
                    as usize] = cut_off_frequency[n___0 as usize]
                    / (audio.rate).wrapping_div(2 as libc::c_uint) as libc::c_float;
                eq[n___0
                    as usize] = pow(
                    cut_off_frequency[n___0 as usize] as libc::c_double,
                    1 as libc::c_int as libc::c_double,
                );
                tmp___36 = pow(
                    2 as libc::c_int as libc::c_double,
                    28 as libc::c_int as libc::c_double,
                );
                eq[n___0 as usize]
                    *= height as libc::c_float as libc::c_double / tmp___36;
                if p.userEQ_enabled != 0 {
                    tmp___37 = floor(
                        n___0 as libc::c_double * userEQ_keys_to_bars_ratio,
                    );
                    eq[n___0 as usize]
                        *= *(p.userEQ).offset(tmp___37 as libc::c_int as isize);
                }
                tmp___38 = log2(audio.FFTbassbufferSize as libc::c_double);
                eq[n___0 as usize] /= tmp___38;
                if cut_off_frequency[n___0 as usize] < bass_cut_off as libc::c_float {
                    *bar_buffer.as_mut_ptr().offset(n___0 as isize) = 1 as libc::c_int;
                    FFTbuffer_lower_cut_off[n___0
                        as usize] = (relative_cut_off[n___0 as usize]
                        * (audio.FFTbassbufferSize / 2 as libc::c_int) as libc::c_float)
                        as libc::c_int;
                    bass_cut_off_bar += 1;
                    treble_cut_off_bar += 1;
                    if bass_cut_off_bar > 0 as libc::c_int {
                        first_bar = 0 as libc::c_int != 0;
                    }
                    tmp___39 = log2(audio.FFTbassbufferSize as libc::c_double);
                    eq[n___0 as usize] *= tmp___39;
                } else {
                    let mut current_block_386: u64;
                    if cut_off_frequency[n___0 as usize] > bass_cut_off as libc::c_float
                    {
                        if cut_off_frequency[n___0 as usize]
                            < treble_cut_off as libc::c_float
                        {
                            *bar_buffer
                                .as_mut_ptr()
                                .offset(n___0 as isize) = 2 as libc::c_int;
                            FFTbuffer_lower_cut_off[n___0
                                as usize] = (relative_cut_off[n___0 as usize]
                                * (audio.FFTmidbufferSize / 2 as libc::c_int)
                                    as libc::c_float) as libc::c_int;
                            treble_cut_off_bar += 1;
                            if treble_cut_off_bar - bass_cut_off_bar == 1 as libc::c_int
                            {
                                first_bar = 1 as libc::c_int != 0;
                                FFTbuffer_upper_cut_off[(n___0 - 1 as libc::c_int)
                                    as usize] = (relative_cut_off[n___0 as usize]
                                    * (audio.FFTbassbufferSize / 2 as libc::c_int)
                                        as libc::c_float) as libc::c_int;
                            } else {
                                first_bar = 0 as libc::c_int != 0;
                            }
                            tmp___40 = log2(audio.FFTmidbufferSize as libc::c_double);
                            eq[n___0 as usize] *= tmp___40;
                            current_block_386 = 3105969743042996984;
                        } else {
                            current_block_386 = 6449861996653530672;
                        }
                    } else {
                        current_block_386 = 6449861996653530672;
                    }
                    match current_block_386 {
                        6449861996653530672 => {
                            *bar_buffer
                                .as_mut_ptr()
                                .offset(n___0 as isize) = 3 as libc::c_int;
                            FFTbuffer_lower_cut_off[n___0
                                as usize] = (relative_cut_off[n___0 as usize]
                                * (audio.FFTtreblebufferSize / 2 as libc::c_int)
                                    as libc::c_float) as libc::c_int;
                            first_treble_bar += 1;
                            if first_treble_bar == 1 as libc::c_int {
                                first_bar = 1 as libc::c_int != 0;
                                FFTbuffer_upper_cut_off[(n___0 - 1 as libc::c_int)
                                    as usize] = (relative_cut_off[n___0 as usize]
                                    * (audio.FFTmidbufferSize / 2 as libc::c_int)
                                        as libc::c_float) as libc::c_int;
                            } else {
                                first_bar = 0 as libc::c_int != 0;
                            }
                            tmp___41 = log2(audio.FFTtreblebufferSize as libc::c_double);
                            eq[n___0 as usize] *= tmp___41;
                        }
                        _ => {}
                    }
                }
                if n___0 > 0 as libc::c_int {
                    if !first_bar {
                        FFTbuffer_upper_cut_off[(n___0 - 1 as libc::c_int)
                            as usize] = FFTbuffer_lower_cut_off[n___0 as usize]
                            - 1 as libc::c_int;
                        if FFTbuffer_lower_cut_off[n___0 as usize]
                            <= FFTbuffer_lower_cut_off[(n___0 - 1 as libc::c_int)
                                as usize]
                        {
                            FFTbuffer_lower_cut_off[n___0
                                as usize] = FFTbuffer_lower_cut_off[(n___0
                                - 1 as libc::c_int) as usize] + 1 as libc::c_int;
                            FFTbuffer_upper_cut_off[(n___0 - 1 as libc::c_int)
                                as usize] = FFTbuffer_lower_cut_off[n___0 as usize]
                                - 1 as libc::c_int;
                            if *bar_buffer.as_mut_ptr().offset(n___0 as isize)
                                == 1 as libc::c_int
                            {
                                relative_cut_off[n___0
                                    as usize] = FFTbuffer_lower_cut_off[n___0 as usize]
                                    as libc::c_float
                                    / (audio.FFTbassbufferSize as libc::c_float
                                        / 2 as libc::c_int as libc::c_float);
                            } else if *bar_buffer.as_mut_ptr().offset(n___0 as isize)
                                    == 2 as libc::c_int
                                {
                                relative_cut_off[n___0
                                    as usize] = FFTbuffer_lower_cut_off[n___0 as usize]
                                    as libc::c_float
                                    / (audio.FFTmidbufferSize as libc::c_float
                                        / 2 as libc::c_int as libc::c_float);
                            } else if *bar_buffer.as_mut_ptr().offset(n___0 as isize)
                                    == 3 as libc::c_int
                                {
                                relative_cut_off[n___0
                                    as usize] = FFTbuffer_lower_cut_off[n___0 as usize]
                                    as libc::c_float
                                    / (audio.FFTtreblebufferSize as libc::c_float
                                        / 2 as libc::c_int as libc::c_float);
                            }
                            cut_off_frequency[n___0
                                as usize] = relative_cut_off[n___0 as usize]
                                * (audio.rate as libc::c_float
                                    / 2 as libc::c_int as libc::c_float);
                        }
                    } else if FFTbuffer_upper_cut_off[(n___0 - 1 as libc::c_int)
                            as usize]
                            <= FFTbuffer_lower_cut_off[(n___0 - 1 as libc::c_int)
                                as usize]
                        {
                        FFTbuffer_upper_cut_off[(n___0 - 1 as libc::c_int)
                            as usize] = FFTbuffer_lower_cut_off[(n___0
                            - 1 as libc::c_int) as usize] + 1 as libc::c_int;
                    }
                    upper_cut_off_frequency[(n___0 - 1 as libc::c_int)
                        as usize] = cut_off_frequency[n___0 as usize];
                    center_frequencies[(n___0 - 1 as libc::c_int)
                        as usize] = pow(
                        (cut_off_frequency[(n___0 - 1 as libc::c_int) as usize]
                            * upper_cut_off_frequency[(n___0 - 1 as libc::c_int)
                                as usize]) as libc::c_double,
                        0.5f64,
                    );
                }
                n___0 += 1;
            }
            if p.stereo != 0 {
                number_of_bars *= 2 as libc::c_int;
            }
            x_axis_info = 0 as libc::c_int;
            if p.xaxis as libc::c_uint != 0 as libc::c_uint {
                x_axis_info = 1 as libc::c_int;
                if output_mode == 1 as libc::c_int {
                    printf(
                        b"\r\x1B[%dB\0" as *const u8 as *const libc::c_char,
                        lines + 1 as libc::c_int,
                    );
                    if remainder___0 != 0 {
                        printf(
                            b"\x1B[%dC\0" as *const u8 as *const libc::c_char,
                            remainder___0,
                        );
                    }
                }
                n___1 = 0 as libc::c_int;
                while n___1 < number_of_bars {
                    if p.stereo != 0 {
                        if n___1 < number_of_bars / 2 as libc::c_int {
                            center_frequency = center_frequencies[(number_of_bars
                                / 2 as libc::c_int - 1 as libc::c_int - n___1) as usize];
                        } else {
                            center_frequency = center_frequencies[(n___1
                                - number_of_bars / 2 as libc::c_int) as usize];
                        }
                    } else {
                        center_frequency = center_frequencies[n___1 as usize];
                    }
                    freq_kilohz = (center_frequency
                        / 1000 as libc::c_int as libc::c_double) as libc::c_float;
                    freq_floor = center_frequency as libc::c_int;
                    if output_mode == 0 as libc::c_int {
                        if center_frequency < 1000 as libc::c_int as libc::c_double {
                            mvprintw(
                                lines,
                                n___1 * (p.bar_width + p.bar_spacing) + remainder___0,
                                b"%-4d\0" as *const u8 as *const libc::c_char,
                                freq_floor,
                            );
                        } else if center_frequency
                                > 1000 as libc::c_int as libc::c_double
                            {
                            if center_frequency < 10000 as libc::c_int as libc::c_double
                            {
                                mvprintw(
                                    lines,
                                    n___1 * (p.bar_width + p.bar_spacing) + remainder___0,
                                    b"%.2f\0" as *const u8 as *const libc::c_char,
                                    freq_kilohz as libc::c_double,
                                );
                            } else {
                                mvprintw(
                                    lines,
                                    n___1 * (p.bar_width + p.bar_spacing) + remainder___0,
                                    b"%.1f\0" as *const u8 as *const libc::c_char,
                                    freq_kilohz as libc::c_double,
                                );
                            }
                        } else {
                            mvprintw(
                                lines,
                                n___1 * (p.bar_width + p.bar_spacing) + remainder___0,
                                b"%.1f\0" as *const u8 as *const libc::c_char,
                                freq_kilohz as libc::c_double,
                            );
                        }
                    } else if output_mode == 1 as libc::c_int {
                        if center_frequency < 1000 as libc::c_int as libc::c_double {
                            printf(
                                b"%-4d\0" as *const u8 as *const libc::c_char,
                                freq_floor,
                            );
                        } else if center_frequency
                                > 1000 as libc::c_int as libc::c_double
                            {
                            if center_frequency < 10000 as libc::c_int as libc::c_double
                            {
                                printf(
                                    b"%.2f\0" as *const u8 as *const libc::c_char,
                                    freq_kilohz as libc::c_double,
                                );
                            } else {
                                printf(
                                    b"%.1f\0" as *const u8 as *const libc::c_char,
                                    freq_kilohz as libc::c_double,
                                );
                            }
                        } else {
                            printf(
                                b"%.1f\0" as *const u8 as *const libc::c_char,
                                freq_kilohz as libc::c_double,
                            );
                        }
                        if n___1 < number_of_bars - 1 as libc::c_int {
                            printf(
                                b"\x1B[%dC\0" as *const u8 as *const libc::c_char,
                                p.bar_width + p.bar_spacing - 4 as libc::c_int,
                            );
                        }
                    }
                    n___1 += 1;
                }
                printf(
                    b"\r\x1B[%dA\0" as *const u8 as *const libc::c_char,
                    lines + 1 as libc::c_int,
                );
            }
            resizeTerminal = 0 as libc::c_int != 0;
            framerate_timer.tv_sec = 0 as libc::c_int as __time_t;
            framerate_timer.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
            if p.framerate <= 1 as libc::c_int {
                framerate_timer
                    .tv_sec = (1 as libc::c_int as libc::c_float
                    / p.framerate as libc::c_float) as __time_t;
            } else {
                framerate_timer.tv_sec = 0 as libc::c_int as __time_t;
                framerate_timer
                    .tv_nsec = ((1 as libc::c_int as libc::c_float
                    / p.framerate as libc::c_float) as libc::c_double * 1e9f64)
                    as __syscall_slong_t;
            }
            sleep_counter = 0 as libc::c_int;
            silence = 0 as libc::c_int != 0;
            ch = '\u{0}' as i32 as libc::c_char;
            sleep_mode_timer.tv_sec = 1 as libc::c_int as __time_t;
            sleep_mode_timer.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
            while !resizeTerminal {
                if output_mode == 0 as libc::c_int {
                    tmp___42 = wgetch(stdscr);
                    ch = tmp___42 as libc::c_char;
                }
                match ch as libc::c_int {
                    65 => {
                        p.sens *= 1.05f64;
                    }
                    66 => {
                        p.sens *= 0.95f64;
                    }
                    68 => {
                        p.bar_width += 1;
                        resizeTerminal = 1 as libc::c_int != 0;
                    }
                    67 => {
                        if p.bar_width > 1 as libc::c_int {
                            p.bar_width -= 1;
                        }
                        resizeTerminal = 1 as libc::c_int != 0;
                    }
                    114 => {
                        should_reload = 1 as libc::c_int;
                    }
                    99 => {
                        reload_colors = 1 as libc::c_int;
                    }
                    102 => {
                        if p.col < 7 as libc::c_int {
                            p.col += 1;
                        } else {
                            p.col = 0 as libc::c_int;
                        }
                        resizeTerminal = 1 as libc::c_int != 0;
                    }
                    98 => {
                        if p.bgcol < 7 as libc::c_int {
                            p.bgcol += 1;
                        } else {
                            p.bgcol = 0 as libc::c_int;
                        }
                        resizeTerminal = 1 as libc::c_int != 0;
                    }
                    113 => {
                        should_reload = 1 as libc::c_int;
                        should_quit = 1 as libc::c_int;
                    }
                    _ => {}
                }
                if should_reload != 0 {
                    reloadConf = 1 as libc::c_int != 0;
                    resizeTerminal = 1 as libc::c_int != 0;
                    should_reload = 0 as libc::c_int;
                }
                if reload_colors != 0 {
                    error___0.length = 0 as libc::c_int;
                    tmp___43 = load_config(
                        configPath.as_mut_ptr(),
                        &mut p as *mut config_params as *mut libc::c_void
                            as *mut config_params,
                        1 as libc::c_int != 0,
                        &mut error___0,
                    );
                    if !tmp___43 {
                        cleanup();
                        fprintf(
                            stderr,
                            b"Error loading config. %s\0" as *const u8
                                as *const libc::c_char,
                            (error___0.message).as_mut_ptr(),
                        );
                        exit(1 as libc::c_int);
                    }
                    resizeTerminal = 1 as libc::c_int != 0;
                    reload_colors = 0 as libc::c_int;
                }
                silence = 1 as libc::c_int != 0;
                n___2 = 0 as libc::c_int;
                while n___2 < audio.FFTbassbufferSize {
                    if *(audio.in_bass_l).offset(n___2 as isize) != 0. {
                        silence = 0 as libc::c_int != 0;
                        break;
                    } else if *(audio.in_bass_r).offset(n___2 as isize) != 0. {
                        silence = 0 as libc::c_int != 0;
                        break;
                    } else {
                        n___2 += 1;
                    }
                }
                if p.sleep_timer != 0 {
                    let mut current_block_545: u64;
                    if silence {
                        if sleep_counter <= p.framerate * p.sleep_timer {
                            sleep_counter += 1;
                            current_block_545 = 12827163671827672293;
                        } else {
                            current_block_545 = 6733981545959382280;
                        }
                    } else {
                        current_block_545 = 6733981545959382280;
                    }
                    match current_block_545 {
                        6733981545959382280 => {
                            if !silence {
                                sleep_counter = 0 as libc::c_int;
                            }
                        }
                        _ => {}
                    }
                    if sleep_counter > p.framerate * p.sleep_timer {
                        nanosleep(
                            &mut sleep_mode_timer as *mut timespec as *const timespec,
                            0 as *mut libc::c_void as *mut timespec,
                        );
                        continue;
                    }
                }
                pthread_mutex_lock(&mut lock);
                fftw_execute(p_bass_l);
                fftw_execute(p_mid_l);
                fftw_execute(p_treble_l);
                if p.stereo != 0 {
                    fftw_execute(p_bass_r);
                    fftw_execute(p_mid_r);
                    fftw_execute(p_treble_r);
                    number_of_bars /= 2 as libc::c_int;
                }
                pthread_mutex_unlock(&mut lock);
                n___3 = 0 as libc::c_int;
                while n___3 < number_of_bars {
                    *temp_l.offset(n___3 as isize) = 0 as libc::c_int as libc::c_double;
                    if p.stereo != 0 {
                        *temp_r
                            .offset(n___3 as isize) = 0 as libc::c_int as libc::c_double;
                    }
                    i___2 = FFTbuffer_lower_cut_off[n___3 as usize];
                    while i___2 <= FFTbuffer_upper_cut_off[n___3 as usize] {
                        if n___3 <= bass_cut_off_bar {
                            tmp___44 = hypot(
                                (*out_bass_l
                                    .offset(i___2 as isize))[0 as libc::c_int as usize],
                                (*out_bass_l
                                    .offset(i___2 as isize))[1 as libc::c_int as usize],
                            );
                            *temp_l.offset(n___3 as isize) += tmp___44;
                            if p.stereo != 0 {
                                tmp___45 = hypot(
                                    (*out_bass_r
                                        .offset(i___2 as isize))[0 as libc::c_int as usize],
                                    (*out_bass_r
                                        .offset(i___2 as isize))[1 as libc::c_int as usize],
                                );
                                *temp_r.offset(n___3 as isize) += tmp___45;
                            }
                        } else {
                            let mut current_block_586: u64;
                            if n___3 > bass_cut_off_bar {
                                if n___3 <= treble_cut_off_bar {
                                    tmp___46 = hypot(
                                        (*out_mid_l
                                            .offset(i___2 as isize))[0 as libc::c_int as usize],
                                        (*out_mid_l
                                            .offset(i___2 as isize))[1 as libc::c_int as usize],
                                    );
                                    *temp_l.offset(n___3 as isize) += tmp___46;
                                    if p.stereo != 0 {
                                        tmp___47 = hypot(
                                            (*out_mid_r
                                                .offset(i___2 as isize))[0 as libc::c_int as usize],
                                            (*out_mid_r
                                                .offset(i___2 as isize))[1 as libc::c_int as usize],
                                        );
                                        *temp_r.offset(n___3 as isize) += tmp___47;
                                    }
                                    current_block_586 = 14657996519002068616;
                                } else {
                                    current_block_586 = 8720601981958445824;
                                }
                            } else {
                                current_block_586 = 8720601981958445824;
                            }
                            match current_block_586 {
                                8720601981958445824 => {
                                    if n___3 > treble_cut_off_bar {
                                        tmp___48 = hypot(
                                            (*out_treble_l
                                                .offset(i___2 as isize))[0 as libc::c_int as usize],
                                            (*out_treble_l
                                                .offset(i___2 as isize))[1 as libc::c_int as usize],
                                        );
                                        *temp_l.offset(n___3 as isize) += tmp___48;
                                        if p.stereo != 0 {
                                            tmp___49 = hypot(
                                                (*out_treble_r
                                                    .offset(i___2 as isize))[0 as libc::c_int as usize],
                                                (*out_treble_r
                                                    .offset(i___2 as isize))[1 as libc::c_int as usize],
                                            );
                                            *temp_r.offset(n___3 as isize) += tmp___49;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        i___2 += 1;
                    }
                    *temp_l.offset(n___3 as isize)
                        /= (FFTbuffer_upper_cut_off[n___3 as usize]
                            - FFTbuffer_lower_cut_off[n___3 as usize] + 1 as libc::c_int)
                            as libc::c_double;
                    *temp_l.offset(n___3 as isize) *= p.sens * eq[n___3 as usize];
                    if *temp_l.offset(n___3 as isize) <= p.ignore {
                        *temp_l
                            .offset(n___3 as isize) = 0 as libc::c_int as libc::c_double;
                    }
                    *bars_left
                        .offset(
                            n___3 as isize,
                        ) = *temp_l.offset(n___3 as isize) as libc::c_int;
                    if p.stereo != 0 {
                        *temp_r.offset(n___3 as isize)
                            /= (FFTbuffer_upper_cut_off[n___3 as usize]
                                - FFTbuffer_lower_cut_off[n___3 as usize]
                                + 1 as libc::c_int) as libc::c_double;
                        *temp_r.offset(n___3 as isize) *= p.sens * eq[n___3 as usize];
                        if *temp_r.offset(n___3 as isize) <= p.ignore {
                            *temp_r
                                .offset(
                                    n___3 as isize,
                                ) = 0 as libc::c_int as libc::c_double;
                        }
                        *bars_right
                            .offset(
                                n___3 as isize,
                            ) = *temp_r.offset(n___3 as isize) as libc::c_int;
                    }
                    n___3 += 1;
                }
                if p.stereo != 0 {
                    number_of_bars *= 2 as libc::c_int;
                }
                if p.monstercat != 0. {
                    if p.stereo != 0 {
                        bars_left = monstercat_filter(
                            bars_left,
                            number_of_bars / 2 as libc::c_int,
                            p.waves,
                            p.monstercat,
                        );
                        bars_right = monstercat_filter(
                            bars_right,
                            number_of_bars / 2 as libc::c_int,
                            p.waves,
                            p.monstercat,
                        );
                    } else {
                        bars_left = monstercat_filter(
                            bars_left,
                            number_of_bars,
                            p.waves,
                            p.monstercat,
                        );
                    }
                }
                senselow = 1 as libc::c_int != 0;
                n___4 = 0 as libc::c_int;
                while n___4 < number_of_bars {
                    if p.stereo != 0 {
                        if n___4 < number_of_bars / 2 as libc::c_int {
                            bars[n___4
                                as usize] = *bars_left
                                .offset(
                                    (number_of_bars / 2 as libc::c_int - n___4
                                        - 1 as libc::c_int) as isize,
                                );
                        } else {
                            bars[n___4
                                as usize] = *bars_right
                                .offset(
                                    (n___4 - number_of_bars / 2 as libc::c_int) as isize,
                                );
                        }
                    } else {
                        bars[n___4 as usize] = *bars_left.offset(n___4 as isize);
                    }
                    if g > 0 as libc::c_int as libc::c_float {
                        if bars[n___4 as usize] < bars_last[n___4 as usize] {
                            bars[n___4
                                as usize] = (bars_peak[n___4 as usize]
                                - g * fall[n___4 as usize] as libc::c_float
                                    * fall[n___4 as usize] as libc::c_float) as libc::c_int;
                            if bars[n___4 as usize] < 0 as libc::c_int {
                                bars[n___4 as usize] = 0 as libc::c_int;
                            }
                            fall[n___4 as usize] += 1;
                        } else {
                            bars_peak[n___4
                                as usize] = bars[n___4 as usize] as libc::c_float;
                            fall[n___4 as usize] = 0 as libc::c_int;
                        }
                        bars_last[n___4 as usize] = bars[n___4 as usize];
                    }
                    if p.integral > 0 as libc::c_int as libc::c_double {
                        bars[n___4
                            as usize] = (bars_mem[n___4 as usize] as libc::c_double
                            * integral + bars[n___4 as usize] as libc::c_double)
                            as libc::c_int;
                        bars_mem[n___4 as usize] = bars[n___4 as usize];
                        diff = height - bars[n___4 as usize];
                        if diff < 0 as libc::c_int {
                            diff = 0 as libc::c_int;
                        }
                        div___0 = (1 as libc::c_int / (diff + 1 as libc::c_int))
                            as libc::c_double;
                        bars_mem[n___4
                            as usize] = (bars_mem[n___4 as usize] as libc::c_double
                            * (1 as libc::c_int as libc::c_double
                                - div___0 / 20 as libc::c_int as libc::c_double))
                            as libc::c_int;
                    }
                    if output_mode != 2 as libc::c_int {
                        if bars[n___4 as usize] < 1 as libc::c_int {
                            bars[n___4 as usize] = 1 as libc::c_int;
                        }
                    }
                    if p.autosens != 0 {
                        if !silence {
                            if bars[n___4 as usize] > height {
                                if senselow {
                                    p.sens *= 0.98f64;
                                    senselow = 0 as libc::c_int != 0;
                                    first = 0 as libc::c_int != 0;
                                }
                            }
                        }
                    }
                    n___4 += 1;
                }
                if p.autosens != 0 {
                    if !silence {
                        if senselow {
                            p.sens *= 1.001f64;
                            if first {
                                p.sens *= 1.1f64;
                            }
                        }
                    }
                }
                match output_mode {
                    0 => {
                        rc___0 = draw_terminal_ncurses(
                            inAtty,
                            lines,
                            width,
                            number_of_bars,
                            p.bar_width,
                            p.bar_spacing,
                            remainder___0,
                            bars.as_mut_ptr() as *const libc::c_int,
                            previous_frame.as_mut_ptr(),
                            p.gradient,
                            x_axis_info,
                        );
                    }
                    1 => {
                        rc___0 = draw_terminal_noncurses(
                            inAtty,
                            lines,
                            width,
                            number_of_bars,
                            p.bar_width,
                            p.bar_spacing,
                            remainder___0,
                            bars.as_mut_ptr(),
                            previous_frame.as_mut_ptr(),
                            x_axis_info,
                        );
                    }
                    2 => {
                        rc___0 = print_raw_out(
                            number_of_bars,
                            fp,
                            p.is_bin,
                            p.bit_format,
                            p.ascii_range,
                            p.bar_delim,
                            p.frame_delim,
                            bars.as_mut_ptr() as *const libc::c_int,
                        );
                    }
                    _ => {
                        exit(1 as libc::c_int);
                    }
                }
                if rc___0 == -(1 as libc::c_int) {
                    resizeTerminal = 1 as libc::c_int != 0;
                }
                memcpy(
                    previous_frame.as_mut_ptr() as *mut libc::c_void,
                    bars.as_mut_ptr() as *const libc::c_void,
                    (256 as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                );
                if audio.terminate == 1 as libc::c_int {
                    cleanup();
                    fprintf(
                        stderr,
                        b"Audio thread exited unexpectedly. %s\n\0" as *const u8
                            as *const libc::c_char,
                        (audio.error_message).as_mut_ptr(),
                    );
                    exit(1 as libc::c_int);
                }
                nanosleep(
                    &mut framerate_timer as *mut timespec as *const timespec,
                    0 as *mut libc::c_void as *mut timespec,
                );
            }
        }
        audio.terminate = 1 as libc::c_int;
        pthread_join(p_thread, 0 as *mut libc::c_void as *mut *mut libc::c_void);
        if p.userEQ_enabled != 0 {
            free(p.userEQ as *mut libc::c_void);
        }
        free(audio.source as *mut libc::c_void);
        fftw_free(audio.in_bass_r as *mut libc::c_void);
        fftw_free(audio.in_bass_l as *mut libc::c_void);
        fftw_free(out_bass_r as *mut libc::c_void);
        fftw_free(out_bass_l as *mut libc::c_void);
        fftw_destroy_plan(p_bass_l);
        fftw_destroy_plan(p_bass_r);
        fftw_free(audio.in_mid_r as *mut libc::c_void);
        fftw_free(audio.in_mid_l as *mut libc::c_void);
        fftw_free(out_mid_r as *mut libc::c_void);
        fftw_free(out_mid_l as *mut libc::c_void);
        fftw_destroy_plan(p_mid_l);
        fftw_destroy_plan(p_mid_r);
        fftw_free(audio.in_treble_r as *mut libc::c_void);
        fftw_free(audio.in_treble_l as *mut libc::c_void);
        fftw_free(out_treble_r as *mut libc::c_void);
        fftw_free(out_treble_l as *mut libc::c_void);
        fftw_destroy_plan(p_treble_l);
        fftw_destroy_plan(p_treble_r);
        cleanup();
        if should_quit != 0 {
            return 0 as libc::c_int;
        }
    };
}
pub static mut smoothDef: [libc::c_double; 5] = [
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
    1 as libc::c_int as libc::c_double,
];
pub static mut default_methods: [input_method; 4] = [
    INPUT_FIFO,
    INPUT_PORTAUDIO,
    INPUT_ALSA,
    INPUT_PULSE,
];
pub static mut outputMethod: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut channels: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut xaxisScale: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut input_method_names: [*const libc::c_char; 6] = [
    b"fifo\0" as *const u8 as *const libc::c_char,
    b"portaudio\0" as *const u8 as *const libc::c_char,
    b"alsa\0" as *const u8 as *const libc::c_char,
    b"pulse\0" as *const u8 as *const libc::c_char,
    b"sndio\0" as *const u8 as *const libc::c_char,
    b"shmem\0" as *const u8 as *const libc::c_char,
];
pub static mut has_input_method: [bool; 6] = [
    1 as libc::c_int != 0,
    0 as libc::c_int != 0,
    1 as libc::c_int != 0,
    1 as libc::c_int != 0,
    0 as libc::c_int != 0,
    1 as libc::c_int != 0,
];
pub unsafe extern "C" fn input_method_by_name(
    mut str: *const libc::c_char,
) -> input_method {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        tmp = strcmp(str, input_method_names[i as usize]);
        if tmp == 0 {
            return i as input_method;
        }
        i += 1;
    }
    return INPUT_MAX;
}
pub unsafe extern "C" fn write_errorf(
    mut err: *mut libc::c_void,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut error: *mut error_s = 0 as *mut error_s;
    let mut args_0: ::std::ffi::VaListImpl;
    let mut tmp: libc::c_int = 0;
    error = err as *mut error_s;
    args_0 = args.clone();
    tmp = vsnprintf(
        ((*error).message).as_mut_ptr().offset((*error).length as isize),
        (1024 as libc::c_int - (*error).length) as size_t,
        fmt,
        args_0.as_va_list(),
    );
    (*error).length += tmp;
}
pub unsafe extern "C" fn validate_color(
    mut checkColor: *mut libc::c_char,
    mut params: *mut libc::c_void,
    mut err: *mut libc::c_void,
) -> libc::c_int {
    let mut p___0: *mut config_params = 0 as *mut config_params;
    let mut error: *mut error_s = 0 as *mut error_s;
    let mut validColor: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut __res: libc::c_int = 0;
    let mut tmp___0: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut __res___0: libc::c_int = 0;
    let mut tmp___2: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___3: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: size_t = 0;
    p___0 = params as *mut config_params;
    error = err as *mut error_s;
    validColor = 0 as libc::c_int;
    let mut current_block_74: u64;
    if *checkColor.offset(0 as libc::c_int as isize) as libc::c_int == 35 as libc::c_int
    {
        tmp___13 = strlen(checkColor as *const libc::c_char);
        if tmp___13 == 7 as libc::c_ulong {
            if (*p___0).output as libc::c_uint != 0 as libc::c_uint {
                write_errorf(
                    error as *mut libc::c_void,
                    b"hex color configured, but ncurses not set. Forcing ncurses mode.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                (*p___0).output = OUTPUT_NCURSES;
            }
            i = 1 as libc::c_int;
            while *checkColor.offset(i as isize) != 0 {
                tmp___3 = __ctype_b_loc();
                if *(*tmp___3)
                    .offset(*checkColor.offset(i as isize) as libc::c_int as isize)
                    as libc::c_int & 2048 as libc::c_int != 0
                {
                    validColor = 1 as libc::c_int;
                } else {
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_ulong
                    {
                        __res = tolower(*checkColor.offset(i as isize) as libc::c_int);
                    } else {
                        tmp___0 = __ctype_tolower_loc();
                        __res = *(*tmp___0)
                            .offset(
                                *checkColor.offset(i as isize) as libc::c_int as isize,
                            );
                    }
                    if __res >= 97 as libc::c_int {
                        if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                            > 1 as libc::c_ulong
                        {
                            __res___0 = tolower(
                                *checkColor.offset(i as isize) as libc::c_int,
                            );
                        } else {
                            tmp___2 = __ctype_tolower_loc();
                            __res___0 = *(*tmp___2)
                                .offset(
                                    *checkColor.offset(i as isize) as libc::c_int as isize,
                                );
                        }
                        if __res___0 <= 102 as libc::c_int {
                            validColor = 1 as libc::c_int;
                        } else {
                            validColor = 0 as libc::c_int;
                            break;
                        }
                    } else {
                        validColor = 0 as libc::c_int;
                        break;
                    }
                }
                i += 1;
            }
            current_block_74 = 17515716450947708786;
        } else {
            current_block_74 = 6684689397576107688;
        }
    } else {
        current_block_74 = 6684689397576107688;
    }
    match current_block_74 {
        6684689397576107688 => {
            tmp___4 = strcmp(
                checkColor as *const libc::c_char,
                b"black\0" as *const u8 as *const libc::c_char,
            );
            if tmp___4 == 0 as libc::c_int {
                validColor = 1 as libc::c_int;
            } else {
                tmp___5 = strcmp(
                    checkColor as *const libc::c_char,
                    b"red\0" as *const u8 as *const libc::c_char,
                );
                if tmp___5 == 0 as libc::c_int {
                    validColor = 1 as libc::c_int;
                } else {
                    tmp___6 = strcmp(
                        checkColor as *const libc::c_char,
                        b"green\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___6 == 0 as libc::c_int {
                        validColor = 1 as libc::c_int;
                    } else {
                        tmp___7 = strcmp(
                            checkColor as *const libc::c_char,
                            b"yellow\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___7 == 0 as libc::c_int {
                            validColor = 1 as libc::c_int;
                        } else {
                            tmp___8 = strcmp(
                                checkColor as *const libc::c_char,
                                b"blue\0" as *const u8 as *const libc::c_char,
                            );
                            if tmp___8 == 0 as libc::c_int {
                                validColor = 1 as libc::c_int;
                            } else {
                                tmp___9 = strcmp(
                                    checkColor as *const libc::c_char,
                                    b"magenta\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp___9 == 0 as libc::c_int {
                                    validColor = 1 as libc::c_int;
                                } else {
                                    tmp___10 = strcmp(
                                        checkColor as *const libc::c_char,
                                        b"cyan\0" as *const u8 as *const libc::c_char,
                                    );
                                    if tmp___10 == 0 as libc::c_int {
                                        validColor = 1 as libc::c_int;
                                    } else {
                                        tmp___11 = strcmp(
                                            checkColor as *const libc::c_char,
                                            b"white\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___11 == 0 as libc::c_int {
                                            validColor = 1 as libc::c_int;
                                        } else {
                                            tmp___12 = strcmp(
                                                checkColor as *const libc::c_char,
                                                b"default\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___12 == 0 as libc::c_int {
                                                validColor = 1 as libc::c_int;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return validColor;
}
pub unsafe extern "C" fn validate_colors(
    mut params: *mut libc::c_void,
    mut err: *mut libc::c_void,
) -> bool {
    let mut p___0: *mut config_params = 0 as *mut config_params;
    let mut error: *mut error_s = 0 as *mut error_s;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    p___0 = params as *mut config_params;
    error = err as *mut error_s;
    tmp = validate_color(
        (*p___0).color,
        p___0 as *mut libc::c_void,
        error as *mut libc::c_void,
    );
    if tmp == 0 {
        write_errorf(
            error as *mut libc::c_void,
            b"The value for 'foreground' is invalid. It can be either one of the 7 named colors or a HTML color of the form '#xxxxxx'.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___0 = validate_color(
        (*p___0).bcolor,
        p___0 as *mut libc::c_void,
        error as *mut libc::c_void,
    );
    if tmp___0 == 0 {
        write_errorf(
            error as *mut libc::c_void,
            b"The value for 'background' is invalid. It can be either one of the 7 named colors or a HTML color of the form '#xxxxxx'.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    if (*p___0).gradient != 0 {
        i = 0 as libc::c_int;
        while i < (*p___0).gradient_count {
            tmp___1 = validate_color(
                *((*p___0).gradient_colors).offset(i as isize),
                p___0 as *mut libc::c_void,
                error as *mut libc::c_void,
            );
            if tmp___1 == 0 {
                write_errorf(
                    error as *mut libc::c_void,
                    b"Gradient color %d is invalid. It must be HTML color of the form '#xxxxxx'.\n\0"
                        as *const u8 as *const libc::c_char,
                    i + 1 as libc::c_int,
                );
                return 0 as libc::c_int != 0;
            }
            i += 1;
        }
    }
    (*p___0).col = -(1 as libc::c_int);
    tmp___2 = strcmp(
        (*p___0).color as *const libc::c_char,
        b"black\0" as *const u8 as *const libc::c_char,
    );
    if tmp___2 == 0 as libc::c_int {
        (*p___0).col = 0 as libc::c_int;
    }
    tmp___3 = strcmp(
        (*p___0).color as *const libc::c_char,
        b"red\0" as *const u8 as *const libc::c_char,
    );
    if tmp___3 == 0 as libc::c_int {
        (*p___0).col = 1 as libc::c_int;
    }
    tmp___4 = strcmp(
        (*p___0).color as *const libc::c_char,
        b"green\0" as *const u8 as *const libc::c_char,
    );
    if tmp___4 == 0 as libc::c_int {
        (*p___0).col = 2 as libc::c_int;
    }
    tmp___5 = strcmp(
        (*p___0).color as *const libc::c_char,
        b"yellow\0" as *const u8 as *const libc::c_char,
    );
    if tmp___5 == 0 as libc::c_int {
        (*p___0).col = 3 as libc::c_int;
    }
    tmp___6 = strcmp(
        (*p___0).color as *const libc::c_char,
        b"blue\0" as *const u8 as *const libc::c_char,
    );
    if tmp___6 == 0 as libc::c_int {
        (*p___0).col = 4 as libc::c_int;
    }
    tmp___7 = strcmp(
        (*p___0).color as *const libc::c_char,
        b"magenta\0" as *const u8 as *const libc::c_char,
    );
    if tmp___7 == 0 as libc::c_int {
        (*p___0).col = 5 as libc::c_int;
    }
    tmp___8 = strcmp(
        (*p___0).color as *const libc::c_char,
        b"cyan\0" as *const u8 as *const libc::c_char,
    );
    if tmp___8 == 0 as libc::c_int {
        (*p___0).col = 6 as libc::c_int;
    }
    tmp___9 = strcmp(
        (*p___0).color as *const libc::c_char,
        b"white\0" as *const u8 as *const libc::c_char,
    );
    if tmp___9 == 0 as libc::c_int {
        (*p___0).col = 7 as libc::c_int;
    }
    tmp___10 = strcmp(
        (*p___0).bcolor as *const libc::c_char,
        b"black\0" as *const u8 as *const libc::c_char,
    );
    if tmp___10 == 0 as libc::c_int {
        (*p___0).bgcol = 0 as libc::c_int;
    }
    tmp___11 = strcmp(
        (*p___0).bcolor as *const libc::c_char,
        b"red\0" as *const u8 as *const libc::c_char,
    );
    if tmp___11 == 0 as libc::c_int {
        (*p___0).bgcol = 1 as libc::c_int;
    }
    tmp___12 = strcmp(
        (*p___0).bcolor as *const libc::c_char,
        b"green\0" as *const u8 as *const libc::c_char,
    );
    if tmp___12 == 0 as libc::c_int {
        (*p___0).bgcol = 2 as libc::c_int;
    }
    tmp___13 = strcmp(
        (*p___0).bcolor as *const libc::c_char,
        b"yellow\0" as *const u8 as *const libc::c_char,
    );
    if tmp___13 == 0 as libc::c_int {
        (*p___0).bgcol = 3 as libc::c_int;
    }
    tmp___14 = strcmp(
        (*p___0).bcolor as *const libc::c_char,
        b"blue\0" as *const u8 as *const libc::c_char,
    );
    if tmp___14 == 0 as libc::c_int {
        (*p___0).bgcol = 4 as libc::c_int;
    }
    tmp___15 = strcmp(
        (*p___0).bcolor as *const libc::c_char,
        b"magenta\0" as *const u8 as *const libc::c_char,
    );
    if tmp___15 == 0 as libc::c_int {
        (*p___0).bgcol = 5 as libc::c_int;
    }
    tmp___16 = strcmp(
        (*p___0).bcolor as *const libc::c_char,
        b"cyan\0" as *const u8 as *const libc::c_char,
    );
    if tmp___16 == 0 as libc::c_int {
        (*p___0).bgcol = 6 as libc::c_int;
    }
    tmp___17 = strcmp(
        (*p___0).bcolor as *const libc::c_char,
        b"white\0" as *const u8 as *const libc::c_char,
    );
    if tmp___17 == 0 as libc::c_int {
        (*p___0).bgcol = 7 as libc::c_int;
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn validate_config(
    mut p___0: *mut config_params,
    mut error: *mut error_s,
) -> bool {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: bool = false;
    (*p___0).output = OUTPUT_NOT_SUPORTED;
    tmp = strcmp(
        outputMethod as *const libc::c_char,
        b"ncurses\0" as *const u8 as *const libc::c_char,
    );
    if tmp == 0 as libc::c_int {
        (*p___0).output = OUTPUT_NCURSES;
        (*p___0).bgcol = -(1 as libc::c_int);
    }
    tmp___0 = strcmp(
        outputMethod as *const libc::c_char,
        b"noncurses\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 == 0 as libc::c_int {
        (*p___0).output = OUTPUT_NONCURSES;
        (*p___0).bgcol = 0 as libc::c_int;
    }
    tmp___3 = strcmp(
        outputMethod as *const libc::c_char,
        b"raw\0" as *const u8 as *const libc::c_char,
    );
    if tmp___3 == 0 as libc::c_int {
        (*p___0).output = OUTPUT_RAW;
        (*p___0).bar_spacing = 0 as libc::c_int;
        (*p___0).bar_width = 1 as libc::c_int;
        (*p___0).is_bin = -(1 as libc::c_int);
        tmp___2 = strcmp(
            (*p___0).data_format as *const libc::c_char,
            b"binary\0" as *const u8 as *const libc::c_char,
        );
        if tmp___2 == 0 as libc::c_int {
            (*p___0).is_bin = 1 as libc::c_int;
            if (*p___0).bit_format != 8 as libc::c_int {
                if (*p___0).bit_format != 16 as libc::c_int {
                    write_errorf(
                        error as *mut libc::c_void,
                        b"bit format  %d is not supported, supported data formats are: '8' and '16'\n\0"
                            as *const u8 as *const libc::c_char,
                        (*p___0).bit_format,
                    );
                    return 0 as libc::c_int != 0;
                }
            }
        } else {
            tmp___1 = strcmp(
                (*p___0).data_format as *const libc::c_char,
                b"ascii\0" as *const u8 as *const libc::c_char,
            );
            if tmp___1 == 0 as libc::c_int {
                (*p___0).is_bin = 0 as libc::c_int;
                if (*p___0).ascii_range < 1 as libc::c_int {
                    write_errorf(
                        error as *mut libc::c_void,
                        b"ascii max value must be a positive integer\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int != 0;
                }
            } else {
                write_errorf(
                    error as *mut libc::c_void,
                    b"data format %s is not supported, supported data formats are: 'binary' and 'ascii'\n\0"
                        as *const u8 as *const libc::c_char,
                    (*p___0).data_format,
                );
                return 0 as libc::c_int != 0;
            }
        }
    }
    if (*p___0).output as libc::c_uint == 3 as libc::c_uint {
        write_errorf(
            error as *mut libc::c_void,
            b"output method %s is not supported, supported methods are: 'ncurses', 'noncurses' and 'raw'\n\0"
                as *const u8 as *const libc::c_char,
            outputMethod,
        );
        return 0 as libc::c_int != 0;
    }
    (*p___0).xaxis = NONE;
    tmp___4 = strcmp(
        xaxisScale as *const libc::c_char,
        b"none\0" as *const u8 as *const libc::c_char,
    );
    if tmp___4 == 0 as libc::c_int {
        (*p___0).xaxis = NONE;
    }
    tmp___5 = strcmp(
        xaxisScale as *const libc::c_char,
        b"frequency\0" as *const u8 as *const libc::c_char,
    );
    if tmp___5 == 0 as libc::c_int {
        (*p___0).xaxis = FREQUENCY;
    }
    tmp___6 = strcmp(
        xaxisScale as *const libc::c_char,
        b"note\0" as *const u8 as *const libc::c_char,
    );
    if tmp___6 == 0 as libc::c_int {
        (*p___0).xaxis = NOTE;
    }
    (*p___0).stereo = -(1 as libc::c_int);
    tmp___10 = strcmp(
        channels as *const libc::c_char,
        b"mono\0" as *const u8 as *const libc::c_char,
    );
    if tmp___10 == 0 as libc::c_int {
        (*p___0).stereo = 0 as libc::c_int;
        tmp___7 = strcmp(
            (*p___0).mono_option as *const libc::c_char,
            b"average\0" as *const u8 as *const libc::c_char,
        );
        if tmp___7 != 0 as libc::c_int {
            tmp___8 = strcmp(
                (*p___0).mono_option as *const libc::c_char,
                b"left\0" as *const u8 as *const libc::c_char,
            );
            if tmp___8 != 0 as libc::c_int {
                tmp___9 = strcmp(
                    (*p___0).mono_option as *const libc::c_char,
                    b"right\0" as *const u8 as *const libc::c_char,
                );
                if tmp___9 != 0 as libc::c_int {
                    write_errorf(
                        error as *mut libc::c_void,
                        b"mono option %s is not supported, supported options are: 'average', 'left' or 'right'\n\0"
                            as *const u8 as *const libc::c_char,
                        (*p___0).mono_option,
                    );
                    return 0 as libc::c_int != 0;
                }
            }
        }
    }
    tmp___11 = strcmp(
        channels as *const libc::c_char,
        b"stereo\0" as *const u8 as *const libc::c_char,
    );
    if tmp___11 == 0 as libc::c_int {
        (*p___0).stereo = 1 as libc::c_int;
    }
    if (*p___0).stereo == -(1 as libc::c_int) {
        write_errorf(
            error as *mut libc::c_void,
            b"output channels %s is not supported, supported channelss are: 'mono' and 'stereo'\n\0"
                as *const u8 as *const libc::c_char,
            channels,
        );
        return 0 as libc::c_int != 0;
    }
    (*p___0).autobars = 1 as libc::c_int;
    if (*p___0).fixedbars > 0 as libc::c_int {
        (*p___0).autobars = 0 as libc::c_int;
    }
    if (*p___0).fixedbars > 256 as libc::c_int {
        (*p___0).fixedbars = 256 as libc::c_int;
    }
    if (*p___0).bar_width > 256 as libc::c_int {
        (*p___0).bar_width = 256 as libc::c_int;
    }
    if (*p___0).bar_width < 1 as libc::c_int {
        (*p___0).bar_width = 1 as libc::c_int;
    }
    if (*p___0).framerate < 0 as libc::c_int {
        write_errorf(
            error as *mut libc::c_void,
            b"framerate can't be negative!\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    tmp___12 = validate_colors(p___0 as *mut libc::c_void, error as *mut libc::c_void);
    if !tmp___12 {
        return 0 as libc::c_int != 0;
    }
    (*p___0).gravity /= 100 as libc::c_int as libc::c_double;
    if (*p___0).gravity < 0 as libc::c_int as libc::c_double {
        (*p___0).gravity = 0 as libc::c_int as libc::c_double;
    }
    (*p___0).integral /= 100 as libc::c_int as libc::c_double;
    if (*p___0).integral < 0 as libc::c_int as libc::c_double {
        (*p___0).integral = 0 as libc::c_int as libc::c_double;
    } else if (*p___0).integral > 1 as libc::c_int as libc::c_double {
        (*p___0).integral = 1 as libc::c_int as libc::c_double;
    }
    if (*p___0).lower_cut_off == 0 as libc::c_uint {
        (*p___0).lower_cut_off = ((*p___0).lower_cut_off).wrapping_add(1);
    }
    if (*p___0).lower_cut_off > (*p___0).upper_cut_off {
        write_errorf(
            error as *mut libc::c_void,
            b"lower cutoff frequency can't be higher than higher cutoff frequency\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int != 0;
    }
    (*p___0).sens /= 100 as libc::c_int as libc::c_double;
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn load_colors(
    mut p___0: *mut config_params,
    mut ini: *mut dictionary,
    mut err: *mut libc::c_void,
) -> bool {
    let mut error: *mut error_s = 0 as *mut error_s;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___7: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___8: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___9: *const libc::c_char = 0 as *const libc::c_char;
    error = err as *mut error_s;
    free((*p___0).color as *mut libc::c_void);
    free((*p___0).bcolor as *mut libc::c_void);
    tmp = iniparser_getstring(
        ini as *const dictionary,
        b"color:foreground\0" as *const u8 as *const libc::c_char,
        b"default\0" as *const u8 as *const libc::c_char,
    );
    (*p___0).color = strdup(tmp);
    tmp___0 = iniparser_getstring(
        ini as *const dictionary,
        b"color:background\0" as *const u8 as *const libc::c_char,
        b"default\0" as *const u8 as *const libc::c_char,
    );
    (*p___0).bcolor = strdup(tmp___0);
    (*p___0)
        .gradient = iniparser_getint(
        ini as *const dictionary,
        b"color:gradient\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if (*p___0).gradient != 0 {
        i = 0 as libc::c_int;
        while i < (*p___0).gradient_count {
            free(*((*p___0).gradient_colors).offset(i as isize) as *mut libc::c_void);
            i += 1;
        }
        (*p___0)
            .gradient_count = iniparser_getint(
            ini as *const dictionary,
            b"color:gradient_count\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int,
        );
        if (*p___0).gradient_count < 2 as libc::c_int {
            write_errorf(
                error as *mut libc::c_void,
                b"\nAtleast two colors must be given as gradient!\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        if (*p___0).gradient_count > 8 as libc::c_int {
            write_errorf(
                error as *mut libc::c_void,
                b"\nMaximum 8 colors can be specified as gradient!\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int != 0;
        }
        tmp___1 = malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul((*p___0).gradient_count as libc::c_ulong)
                .wrapping_mul(9 as libc::c_ulong),
        );
        (*p___0).gradient_colors = tmp___1 as *mut *mut libc::c_char;
        tmp___2 = iniparser_getstring(
            ini as *const dictionary,
            b"color:gradient_color_1\0" as *const u8 as *const libc::c_char,
            b"#59cc33\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh0 = *((*p___0).gradient_colors)
            .offset(0 as libc::c_int as isize);
        *fresh0 = strdup(tmp___2);
        tmp___3 = iniparser_getstring(
            ini as *const dictionary,
            b"color:gradient_color_2\0" as *const u8 as *const libc::c_char,
            b"#80cc33\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh1 = *((*p___0).gradient_colors)
            .offset(1 as libc::c_int as isize);
        *fresh1 = strdup(tmp___3);
        tmp___4 = iniparser_getstring(
            ini as *const dictionary,
            b"color:gradient_color_3\0" as *const u8 as *const libc::c_char,
            b"#a6cc33\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh2 = *((*p___0).gradient_colors)
            .offset(2 as libc::c_int as isize);
        *fresh2 = strdup(tmp___4);
        tmp___5 = iniparser_getstring(
            ini as *const dictionary,
            b"color:gradient_color_4\0" as *const u8 as *const libc::c_char,
            b"#cccc33\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh3 = *((*p___0).gradient_colors)
            .offset(3 as libc::c_int as isize);
        *fresh3 = strdup(tmp___5);
        tmp___6 = iniparser_getstring(
            ini as *const dictionary,
            b"color:gradient_color_5\0" as *const u8 as *const libc::c_char,
            b"#cca633\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh4 = *((*p___0).gradient_colors)
            .offset(4 as libc::c_int as isize);
        *fresh4 = strdup(tmp___6);
        tmp___7 = iniparser_getstring(
            ini as *const dictionary,
            b"color:gradient_color_6\0" as *const u8 as *const libc::c_char,
            b"#cc8033\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh5 = *((*p___0).gradient_colors)
            .offset(5 as libc::c_int as isize);
        *fresh5 = strdup(tmp___7);
        tmp___8 = iniparser_getstring(
            ini as *const dictionary,
            b"color:gradient_color_7\0" as *const u8 as *const libc::c_char,
            b"#cc5933\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh6 = *((*p___0).gradient_colors)
            .offset(6 as libc::c_int as isize);
        *fresh6 = strdup(tmp___8);
        tmp___9 = iniparser_getstring(
            ini as *const dictionary,
            b"color:gradient_color_8\0" as *const u8 as *const libc::c_char,
            b"#cc3333\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh7 = *((*p___0).gradient_colors)
            .offset(7 as libc::c_int as isize);
        *fresh7 = strdup(tmp___9);
    }
    return 1 as libc::c_int != 0;
}
pub unsafe extern "C" fn load_config(
    mut configPath: *mut libc::c_char,
    mut p___0: *mut config_params,
    mut colorsOnly: bool,
    mut error: *mut error_s,
) -> bool {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut configFile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut configHome: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ini: *mut dictionary = 0 as *mut dictionary;
    let mut tmp___0: bool = false;
    let mut tmp___1: bool = false;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: libc::c_double = 0.;
    let mut tmp___5: bool = false;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___10: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___11: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___12: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut sk: libc::c_int = 0;
    let mut input_method_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    let mut method: input_method = INPUT_FIFO;
    let mut tmp___16: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___17: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___18: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___19: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___20: *const libc::c_char = 0 as *const libc::c_char;
    let mut supported_methods: [libc::c_char; 255] = [0; 255];
    let mut tmp___21: libc::c_uint = 0;
    let mut i___0: libc::c_int = 0;
    let mut result: bool = false;
    let mut tmp___22: bool = false;
    if *configPath.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        configFile = b"config\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        tmp = getenv(b"XDG_CONFIG_HOME\0" as *const u8 as *const libc::c_char);
        configHome = tmp;
        if configHome as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            sprintf(
                configPath,
                b"%s/%s/\0" as *const u8 as *const libc::c_char,
                configHome,
                b"cava\0" as *const u8 as *const libc::c_char,
            );
        } else {
            configHome = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
            if configHome as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                sprintf(
                    configPath,
                    b"%s/%s/\0" as *const u8 as *const libc::c_char,
                    configHome,
                    b".config\0" as *const u8 as *const libc::c_char,
                );
                mkdir(configPath as *const libc::c_char, 511 as libc::c_int as __mode_t);
                sprintf(
                    configPath,
                    b"%s/%s/%s/\0" as *const u8 as *const libc::c_char,
                    configHome,
                    b".config\0" as *const u8 as *const libc::c_char,
                    b"cava\0" as *const u8 as *const libc::c_char,
                );
            } else {
                write_errorf(
                    error as *mut libc::c_void,
                    b"No HOME found (ERR_HOMELESS), exiting...\0" as *const u8
                        as *const libc::c_char,
                );
                return 0 as libc::c_int != 0;
            }
        }
        mkdir(configPath as *const libc::c_char, 511 as libc::c_int as __mode_t);
        strcat(configPath, configFile as *const libc::c_char);
        fp = fopen(
            configPath as *const libc::c_char,
            b"ab+\0" as *const u8 as *const libc::c_char,
        );
        if !fp.is_null() {
            fclose(fp);
        } else {
            fp = fopen(
                configPath as *const libc::c_char,
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            if !fp.is_null() {
                fclose(fp);
            } else {
                write_errorf(
                    error as *mut libc::c_void,
                    b"Unable to open or create file '%s', exiting...\n\0" as *const u8
                        as *const libc::c_char,
                    configPath,
                );
                return 0 as libc::c_int != 0;
            }
        }
    } else {
        fp = fopen(
            configPath as *const libc::c_char,
            b"rb\0" as *const u8 as *const libc::c_char,
        );
        if !fp.is_null() {
            fclose(fp);
        } else {
            write_errorf(
                error as *mut libc::c_void,
                b"Unable to open file '%s', exiting...\n\0" as *const u8
                    as *const libc::c_char,
                configPath,
            );
            return 0 as libc::c_int != 0;
        }
    }
    ini = iniparser_load(configPath as *const libc::c_char);
    if colorsOnly {
        tmp___0 = load_colors(p___0, ini, error as *mut libc::c_void);
        if !tmp___0 {
            return 0 as libc::c_int != 0;
        }
        tmp___1 = validate_colors(
            p___0 as *mut libc::c_void,
            error as *mut libc::c_void,
        );
        return tmp___1;
    }
    tmp___2 = iniparser_getstring(
        ini as *const dictionary,
        b"output:method\0" as *const u8 as *const libc::c_char,
        b"ncurses\0" as *const u8 as *const libc::c_char,
    );
    outputMethod = tmp___2 as *mut libc::c_char;
    tmp___3 = iniparser_getstring(
        ini as *const dictionary,
        b"output:xaxis\0" as *const u8 as *const libc::c_char,
        b"none\0" as *const u8 as *const libc::c_char,
    );
    xaxisScale = tmp___3 as *mut libc::c_char;
    tmp___4 = iniparser_getdouble(
        ini as *const dictionary,
        b"smoothing:monstercat\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as libc::c_double,
    );
    (*p___0).monstercat = 1.5f64 * tmp___4;
    (*p___0)
        .waves = iniparser_getint(
        ini as *const dictionary,
        b"smoothing:waves\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    (*p___0)
        .integral = iniparser_getdouble(
        ini as *const dictionary,
        b"smoothing:integral\0" as *const u8 as *const libc::c_char,
        77 as libc::c_int as libc::c_double,
    );
    (*p___0)
        .gravity = iniparser_getdouble(
        ini as *const dictionary,
        b"smoothing:gravity\0" as *const u8 as *const libc::c_char,
        100 as libc::c_int as libc::c_double,
    );
    (*p___0)
        .ignore = iniparser_getdouble(
        ini as *const dictionary,
        b"smoothing:ignore\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as libc::c_double,
    );
    tmp___5 = load_colors(p___0, ini, error as *mut libc::c_void);
    if !tmp___5 {
        return 0 as libc::c_int != 0;
    }
    (*p___0)
        .fixedbars = iniparser_getint(
        ini as *const dictionary,
        b"general:bars\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    (*p___0)
        .bar_width = iniparser_getint(
        ini as *const dictionary,
        b"general:bar_width\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
    );
    (*p___0)
        .bar_spacing = iniparser_getint(
        ini as *const dictionary,
        b"general:bar_spacing\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    (*p___0)
        .framerate = iniparser_getint(
        ini as *const dictionary,
        b"general:framerate\0" as *const u8 as *const libc::c_char,
        60 as libc::c_int,
    );
    tmp___6 = iniparser_getint(
        ini as *const dictionary,
        b"general:sensitivity\0" as *const u8 as *const libc::c_char,
        100 as libc::c_int,
    );
    (*p___0).sens = tmp___6 as libc::c_double;
    (*p___0)
        .autosens = iniparser_getint(
        ini as *const dictionary,
        b"general:autosens\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    (*p___0)
        .overshoot = iniparser_getint(
        ini as *const dictionary,
        b"general:overshoot\0" as *const u8 as *const libc::c_char,
        20 as libc::c_int,
    );
    tmp___7 = iniparser_getint(
        ini as *const dictionary,
        b"general:lower_cutoff_freq\0" as *const u8 as *const libc::c_char,
        50 as libc::c_int,
    );
    (*p___0).lower_cut_off = tmp___7 as libc::c_uint;
    tmp___8 = iniparser_getint(
        ini as *const dictionary,
        b"general:higher_cutoff_freq\0" as *const u8 as *const libc::c_char,
        10000 as libc::c_int,
    );
    (*p___0).upper_cut_off = tmp___8 as libc::c_uint;
    (*p___0)
        .sleep_timer = iniparser_getint(
        ini as *const dictionary,
        b"general:sleep_timer\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    free(channels as *mut libc::c_void);
    free((*p___0).mono_option as *mut libc::c_void);
    free((*p___0).raw_target as *mut libc::c_void);
    free((*p___0).data_format as *mut libc::c_void);
    tmp___9 = iniparser_getstring(
        ini as *const dictionary,
        b"output:channels\0" as *const u8 as *const libc::c_char,
        b"stereo\0" as *const u8 as *const libc::c_char,
    );
    channels = strdup(tmp___9);
    tmp___10 = iniparser_getstring(
        ini as *const dictionary,
        b"output:mono_option\0" as *const u8 as *const libc::c_char,
        b"average\0" as *const u8 as *const libc::c_char,
    );
    (*p___0).mono_option = strdup(tmp___10);
    tmp___11 = iniparser_getstring(
        ini as *const dictionary,
        b"output:raw_target\0" as *const u8 as *const libc::c_char,
        b"/dev/stdout\0" as *const u8 as *const libc::c_char,
    );
    (*p___0).raw_target = strdup(tmp___11);
    tmp___12 = iniparser_getstring(
        ini as *const dictionary,
        b"output:data_format\0" as *const u8 as *const libc::c_char,
        b"binary\0" as *const u8 as *const libc::c_char,
    );
    (*p___0).data_format = strdup(tmp___12);
    tmp___13 = iniparser_getint(
        ini as *const dictionary,
        b"output:bar_delimiter\0" as *const u8 as *const libc::c_char,
        59 as libc::c_int,
    );
    (*p___0).bar_delim = tmp___13 as libc::c_char;
    tmp___14 = iniparser_getint(
        ini as *const dictionary,
        b"output:frame_delimiter\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int,
    );
    (*p___0).frame_delim = tmp___14 as libc::c_char;
    (*p___0)
        .ascii_range = iniparser_getint(
        ini as *const dictionary,
        b"output:ascii_max_range\0" as *const u8 as *const libc::c_char,
        1000 as libc::c_int,
    );
    (*p___0)
        .bit_format = iniparser_getint(
        ini as *const dictionary,
        b"output:bit_format\0" as *const u8 as *const libc::c_char,
        16 as libc::c_int,
    );
    (*p___0)
        .userEQ_keys = iniparser_getsecnkeys(
        ini as *const dictionary,
        b"eq\0" as *const u8 as *const libc::c_char,
    );
    if (*p___0).userEQ_keys > 0 as libc::c_int {
        (*p___0).userEQ_enabled = 1 as libc::c_int;
        tmp___15 = calloc(
            ((*p___0).userEQ_keys + 1 as libc::c_int) as size_t,
            ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
        );
        (*p___0).userEQ = tmp___15 as *mut libc::c_double;
        let vla = (*p___0).userEQ_keys as usize;
        let mut keys: Vec::<*const libc::c_char> = ::std::vec::from_elem(
            0 as *const libc::c_char,
            vla,
        );
        iniparser_getseckeys(
            ini as *const dictionary,
            b"eq\0" as *const u8 as *const libc::c_char,
            keys.as_mut_ptr(),
        );
        sk = 0 as libc::c_int;
        while sk < (*p___0).userEQ_keys {
            *((*p___0).userEQ)
                .offset(
                    sk as isize,
                ) = iniparser_getdouble(
                ini as *const dictionary,
                *keys.as_mut_ptr().offset(sk as isize),
                1 as libc::c_int as libc::c_double,
            );
            sk += 1;
        }
    } else {
        (*p___0).userEQ_enabled = 0 as libc::c_int;
    }
    free((*p___0).audio_source as *mut libc::c_void);
    i = 0 as libc::c_int as size_t;
    while i
        < (::std::mem::size_of::<[input_method; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<input_method>() as libc::c_ulong)
            .wrapping_div(
                ((::std::mem::size_of::<[input_method; 4]>() as libc::c_ulong)
                    .wrapping_rem(::std::mem::size_of::<input_method>() as libc::c_ulong)
                    == 0) as libc::c_int as size_t,
            )
    {
        method = default_methods[i as usize];
        if has_input_method[method as usize] {
            tmp___16 = iniparser_getstring(
                ini as *const dictionary,
                b"input:method\0" as *const u8 as *const libc::c_char,
                input_method_names[method as usize],
            );
            input_method_name = tmp___16 as *mut libc::c_char;
        }
        i = i.wrapping_add(1);
    }
    (*p___0).input = input_method_by_name(input_method_name as *const libc::c_char);
    match (*p___0).input as libc::c_uint {
        2 => {
            tmp___17 = iniparser_getstring(
                ini as *const dictionary,
                b"input:source\0" as *const u8 as *const libc::c_char,
                b"hw:Loopback,1\0" as *const u8 as *const libc::c_char,
            );
            (*p___0).audio_source = strdup(tmp___17);
        }
        0 => {
            tmp___18 = iniparser_getstring(
                ini as *const dictionary,
                b"input:source\0" as *const u8 as *const libc::c_char,
                b"/tmp/mpd.fifo\0" as *const u8 as *const libc::c_char,
            );
            (*p___0).audio_source = strdup(tmp___18);
            (*p___0)
                .fifoSample = iniparser_getint(
                ini as *const dictionary,
                b"input:sample_rate\0" as *const u8 as *const libc::c_char,
                44100 as libc::c_int,
            );
            (*p___0)
                .fifoSampleBits = iniparser_getint(
                ini as *const dictionary,
                b"input:sample_bits\0" as *const u8 as *const libc::c_char,
                16 as libc::c_int,
            );
        }
        3 => {
            tmp___19 = iniparser_getstring(
                ini as *const dictionary,
                b"input:source\0" as *const u8 as *const libc::c_char,
                b"auto\0" as *const u8 as *const libc::c_char,
            );
            (*p___0).audio_source = strdup(tmp___19);
        }
        5 => {
            tmp___20 = iniparser_getstring(
                ini as *const dictionary,
                b"input:source\0" as *const u8 as *const libc::c_char,
                b"/squeezelite-00:00:00:00:00:00\0" as *const u8 as *const libc::c_char,
            );
            (*p___0).audio_source = strdup(tmp___20);
        }
        6 => {
            supported_methods[0 as libc::c_int
                as usize] = '\u{0}' as i32 as libc::c_char;
            tmp___21 = 1 as libc::c_uint;
            while !(tmp___21 >= 255 as libc::c_uint) {
                supported_methods[tmp___21 as usize] = 0 as libc::c_int as libc::c_char;
                tmp___21 = tmp___21.wrapping_add(1);
            }
            i___0 = 0 as libc::c_int;
            while i___0 < 6 as libc::c_int {
                if has_input_method[i___0 as usize] {
                    strcat(
                        supported_methods.as_mut_ptr(),
                        b"'\0" as *const u8 as *const libc::c_char,
                    );
                    strcat(
                        supported_methods.as_mut_ptr(),
                        input_method_names[i___0 as usize],
                    );
                    strcat(
                        supported_methods.as_mut_ptr(),
                        b"' \0" as *const u8 as *const libc::c_char,
                    );
                }
                i___0 += 1;
            }
            write_errorf(
                error as *mut libc::c_void,
                b"input method '%s' is not supported, supported methods are: %s\n\0"
                    as *const u8 as *const libc::c_char,
                input_method_name,
                supported_methods.as_mut_ptr(),
            );
            return 0 as libc::c_int != 0;
        }
        _ => {
            write_errorf(
                error as *mut libc::c_void,
                b"cava was built without '%s' input support\n\0" as *const u8
                    as *const libc::c_char,
                input_method_names[(*p___0).input as usize],
            );
            return 0 as libc::c_int != 0;
        }
    }
    tmp___22 = validate_config(p___0, error);
    result = tmp___22;
    iniparser_freedict(ini);
    return result;
}
pub unsafe extern "C" fn reset_output_buffers(mut data: *mut audio_data) {
    memset(
        (*data).in_bass_r as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTbassbufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_bass_l as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTbassbufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_mid_r as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTmidbufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_mid_l as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTmidbufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_treble_r as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTtreblebufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_treble_l as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTtreblebufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_bass_r_raw as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTbassbufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_bass_l_raw as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTbassbufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_mid_r_raw as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTmidbufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_mid_l_raw as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTmidbufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_treble_r_raw as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTtreblebufferSize as libc::c_ulong),
    );
    memset(
        (*data).in_treble_l_raw as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul((*data).FFTtreblebufferSize as libc::c_ulong),
    );
}
pub unsafe extern "C" fn write_to_fftw_input_buffers(
    mut frames: int16_t,
    mut buf: *mut int16_t,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut audio: *mut audio_data = 0 as *mut audio_data;
    let mut n: uint16_t = 0;
    let mut i: uint16_t = 0;
    let mut n___0: uint16_t = 0;
    let mut i___0: uint16_t = 0;
    let mut n___1: uint16_t = 0;
    let mut i___1: uint16_t = 0;
    let mut n___2: uint16_t = 0;
    let mut i___2: uint16_t = 0;
    let mut i___3: libc::c_int = 0;
    let mut i___4: libc::c_int = 0;
    let mut i___5: libc::c_int = 0;
    if frames as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    audio = data as *mut audio_data;
    n = (*audio).FFTbassbufferSize as uint16_t;
    while n as libc::c_int > frames as libc::c_int {
        i = 1 as libc::c_int as uint16_t;
        while i as libc::c_int <= frames as libc::c_int {
            *((*audio).in_bass_l_raw)
                .offset(
                    (n as libc::c_int - i as libc::c_int) as isize,
                ) = *((*audio).in_bass_l_raw)
                .offset(
                    (n as libc::c_int - i as libc::c_int - frames as libc::c_int)
                        as isize,
                );
            if (*audio).channels == 2 as libc::c_uint {
                *((*audio).in_bass_r_raw)
                    .offset(
                        (n as libc::c_int - i as libc::c_int) as isize,
                    ) = *((*audio).in_bass_r_raw)
                    .offset(
                        (n as libc::c_int - i as libc::c_int - frames as libc::c_int)
                            as isize,
                    );
            }
            i = (i as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        n = (n as libc::c_int - frames as libc::c_int) as uint16_t;
    }
    n___0 = (*audio).FFTmidbufferSize as uint16_t;
    while n___0 as libc::c_int > frames as libc::c_int {
        i___0 = 1 as libc::c_int as uint16_t;
        while i___0 as libc::c_int <= frames as libc::c_int {
            *((*audio).in_mid_l_raw)
                .offset(
                    (n___0 as libc::c_int - i___0 as libc::c_int) as isize,
                ) = *((*audio).in_mid_l_raw)
                .offset(
                    (n___0 as libc::c_int - i___0 as libc::c_int - frames as libc::c_int)
                        as isize,
                );
            if (*audio).channels == 2 as libc::c_uint {
                *((*audio).in_mid_r_raw)
                    .offset(
                        (n___0 as libc::c_int - i___0 as libc::c_int) as isize,
                    ) = *((*audio).in_mid_r_raw)
                    .offset(
                        (n___0 as libc::c_int - i___0 as libc::c_int
                            - frames as libc::c_int) as isize,
                    );
            }
            i___0 = (i___0 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        n___0 = (n___0 as libc::c_int - frames as libc::c_int) as uint16_t;
    }
    n___1 = (*audio).FFTtreblebufferSize as uint16_t;
    while n___1 as libc::c_int > frames as libc::c_int {
        i___1 = 1 as libc::c_int as uint16_t;
        while i___1 as libc::c_int <= frames as libc::c_int {
            *((*audio).in_treble_l_raw)
                .offset(
                    (n___1 as libc::c_int - i___1 as libc::c_int) as isize,
                ) = *((*audio).in_treble_l_raw)
                .offset(
                    (n___1 as libc::c_int - i___1 as libc::c_int - frames as libc::c_int)
                        as isize,
                );
            if (*audio).channels == 2 as libc::c_uint {
                *((*audio).in_treble_r_raw)
                    .offset(
                        (n___1 as libc::c_int - i___1 as libc::c_int) as isize,
                    ) = *((*audio).in_treble_r_raw)
                    .offset(
                        (n___1 as libc::c_int - i___1 as libc::c_int
                            - frames as libc::c_int) as isize,
                    );
            }
            i___1 = (i___1 as libc::c_int + 1 as libc::c_int) as uint16_t;
        }
        n___1 = (n___1 as libc::c_int - frames as libc::c_int) as uint16_t;
    }
    n___2 = (frames as libc::c_int - 1 as libc::c_int) as uint16_t;
    i___2 = 0 as libc::c_int as uint16_t;
    while (i___2 as libc::c_int) < frames as libc::c_int {
        if (*audio).channels == 1 as libc::c_uint {
            if (*audio).average {
                *((*audio).in_bass_l_raw)
                    .offset(
                        n___2 as libc::c_int as isize,
                    ) = ((*buf.offset((i___2 as libc::c_int * 2 as libc::c_int) as isize)
                    as libc::c_int
                    + *buf
                        .offset(
                            (i___2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ) as libc::c_int) / 2 as libc::c_int) as libc::c_double;
            }
            if (*audio).left {
                *((*audio).in_bass_l_raw)
                    .offset(
                        n___2 as libc::c_int as isize,
                    ) = *buf.offset((i___2 as libc::c_int * 2 as libc::c_int) as isize)
                    as libc::c_double;
            }
            if (*audio).right {
                *((*audio).in_bass_l_raw)
                    .offset(
                        n___2 as libc::c_int as isize,
                    ) = *buf
                    .offset(
                        (i___2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int)
                            as isize,
                    ) as libc::c_double;
            }
        }
        if (*audio).channels == 2 as libc::c_uint {
            *((*audio).in_bass_l_raw)
                .offset(
                    n___2 as libc::c_int as isize,
                ) = *buf.offset((i___2 as libc::c_int * 2 as libc::c_int) as isize)
                as libc::c_double;
            *((*audio).in_bass_r_raw)
                .offset(
                    n___2 as libc::c_int as isize,
                ) = *buf
                .offset(
                    (i___2 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int) as isize,
                ) as libc::c_double;
            *((*audio).in_mid_r_raw)
                .offset(
                    n___2 as libc::c_int as isize,
                ) = *((*audio).in_bass_r_raw).offset(n___2 as libc::c_int as isize);
            *((*audio).in_treble_r_raw)
                .offset(
                    n___2 as libc::c_int as isize,
                ) = *((*audio).in_bass_r_raw).offset(n___2 as libc::c_int as isize);
        }
        *((*audio).in_mid_l_raw)
            .offset(
                n___2 as libc::c_int as isize,
            ) = *((*audio).in_bass_l_raw).offset(n___2 as libc::c_int as isize);
        *((*audio).in_treble_l_raw)
            .offset(
                n___2 as libc::c_int as isize,
            ) = *((*audio).in_bass_l_raw).offset(n___2 as libc::c_int as isize);
        n___2 = (n___2 as libc::c_int - 1 as libc::c_int) as uint16_t;
        i___2 = (i___2 as libc::c_int + 1 as libc::c_int) as uint16_t;
    }
    i___3 = 0 as libc::c_int;
    while i___3 < (*audio).FFTbassbufferSize {
        *((*audio).in_bass_l)
            .offset(
                i___3 as isize,
            ) = *((*audio).bass_multiplier).offset(i___3 as isize)
            * *((*audio).in_bass_l_raw).offset(i___3 as isize);
        if (*audio).channels == 2 as libc::c_uint {
            *((*audio).in_bass_r)
                .offset(
                    i___3 as isize,
                ) = *((*audio).bass_multiplier).offset(i___3 as isize)
                * *((*audio).in_bass_r_raw).offset(i___3 as isize);
        }
        i___3 += 1;
    }
    i___4 = 0 as libc::c_int;
    while i___4 < (*audio).FFTmidbufferSize {
        *((*audio).in_mid_l)
            .offset(
                i___4 as isize,
            ) = *((*audio).mid_multiplier).offset(i___4 as isize)
            * *((*audio).in_mid_l_raw).offset(i___4 as isize);
        if (*audio).channels == 2 as libc::c_uint {
            *((*audio).in_mid_r)
                .offset(
                    i___4 as isize,
                ) = *((*audio).mid_multiplier).offset(i___4 as isize)
                * *((*audio).in_mid_r_raw).offset(i___4 as isize);
        }
        i___4 += 1;
    }
    i___5 = 0 as libc::c_int;
    while i___5 < (*audio).FFTtreblebufferSize {
        *((*audio).in_treble_l)
            .offset(
                i___5 as isize,
            ) = *((*audio).treble_multiplier).offset(i___5 as isize)
            * *((*audio).in_treble_l_raw).offset(i___5 as isize);
        if (*audio).channels == 2 as libc::c_uint {
            *((*audio).in_treble_r)
                .offset(
                    i___5 as isize,
                ) = *((*audio).treble_multiplier).offset(i___5 as isize)
                * *((*audio).in_treble_r_raw).offset(i___5 as isize);
        }
        i___5 += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn open_fifo(mut path: *const libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = open(path, 0 as libc::c_int);
    fd = tmp;
    tmp___0 = fcntl(fd, 3 as libc::c_int, 0 as libc::c_int);
    flags = tmp___0;
    fcntl(fd, 4 as libc::c_int, flags | 2048 as libc::c_int);
    return fd;
}
pub unsafe extern "C" fn input_fifo(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut audio: *mut audio_data = 0 as *mut audio_data;
    let mut SAMPLES_PER_BUFFER: libc::c_int = 0;
    let mut bytes_per_sample: libc::c_int = 0;
    let mut samples: *mut uint16_t = 0 as *mut uint16_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fd: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut time_since_last_input: libc::c_int = 0;
    let mut offset: libc::c_uint = 0;
    let mut num_read: libc::c_int = 0;
    let mut tmp___2: ssize_t = 0;
    let mut __constr_expr_0: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut i: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    audio = data as *mut audio_data;
    SAMPLES_PER_BUFFER = (*audio).FFTtreblebufferSize * 2 as libc::c_int;
    bytes_per_sample = (*audio).format / 8 as libc::c_int;
    let vla = (SAMPLES_PER_BUFFER * bytes_per_sample) as usize;
    let mut buf: Vec::<uint8_t> = ::std::vec::from_elem(0, vla);
    if bytes_per_sample == 2 as libc::c_int {
        tmp___0 = buf.as_mut_ptr() as *mut uint8_t as *mut uint16_t
            as *mut libc::c_void;
    } else {
        tmp = calloc(
            SAMPLES_PER_BUFFER as size_t,
            ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
        );
        tmp___0 = tmp;
    }
    samples = tmp___0 as *mut uint16_t;
    tmp___1 = open_fifo((*audio).source as *const libc::c_char);
    fd = tmp___1;
    while (*audio).terminate == 0 {
        time_since_last_input = 0 as libc::c_int;
        offset = 0 as libc::c_uint;
        loop {
            tmp___2 = read(
                fd,
                buf.as_mut_ptr().offset(offset as isize) as *mut libc::c_void,
                ((vla * ::std::mem::size_of::<uint8_t>()) as libc::c_ulong)
                    .wrapping_sub(offset as libc::c_ulong),
            );
            num_read = tmp___2 as libc::c_int;
            if num_read < 1 as libc::c_int {
                __constr_expr_0.tv_sec = 0 as libc::c_int as __time_t;
                __constr_expr_0.tv_nsec = 10000000 as libc::c_int as __syscall_slong_t;
                nanosleep(
                    &mut __constr_expr_0 as *mut timespec as *const timespec,
                    0 as *mut libc::c_void as *mut timespec,
                );
                time_since_last_input += 1;
                if time_since_last_input > 10 as libc::c_int {
                    reset_output_buffers(audio);
                    close(fd);
                    fd = open_fifo((*audio).source as *const libc::c_char);
                    time_since_last_input = 0 as libc::c_int;
                    offset = 0 as libc::c_uint;
                }
            } else {
                offset = offset.wrapping_add(num_read as libc::c_uint);
                time_since_last_input = 0 as libc::c_int;
            }
            if !((offset as libc::c_ulong)
                < (vla * ::std::mem::size_of::<uint8_t>()) as libc::c_ulong)
            {
                break;
            }
        }
        match bytes_per_sample {
            3 => {
                i = 0 as libc::c_int;
                while i < SAMPLES_PER_BUFFER {
                    *samples
                        .offset(
                            i as isize,
                        ) = ((*buf
                        .as_mut_ptr()
                        .offset((3 as libc::c_int * i + 2 as libc::c_int) as isize)
                        as libc::c_int) << 8 as libc::c_int
                        | *buf
                            .as_mut_ptr()
                            .offset((3 as libc::c_int * i + 1 as libc::c_int) as isize)
                            as libc::c_int) as uint16_t;
                    i += 1;
                }
            }
            4 => {
                i___0 = 0 as libc::c_int;
                while i___0 < SAMPLES_PER_BUFFER {
                    *samples
                        .offset(
                            i___0 as isize,
                        ) = ((*buf
                        .as_mut_ptr()
                        .offset((4 as libc::c_int * i___0 + 3 as libc::c_int) as isize)
                        as libc::c_int) << 8 as libc::c_int
                        | *buf
                            .as_mut_ptr()
                            .offset(
                                (4 as libc::c_int * i___0 + 2 as libc::c_int) as isize,
                            ) as libc::c_int) as uint16_t;
                    i___0 += 1;
                }
            }
            2 | _ => {}
        }
        pthread_mutex_lock(&mut lock);
        write_to_fftw_input_buffers(
            (SAMPLES_PER_BUFFER / 2 as libc::c_int) as int16_t,
            samples as *mut int16_t,
            audio as *mut libc::c_void,
        );
        pthread_mutex_unlock(&mut lock);
    }
    close(fd);
    if bytes_per_sample != 2 as libc::c_int {
        free(samples as *mut libc::c_void);
    }
    return 0 as *mut libc::c_void;
}
pub static mut rc: libc::c_int = 0;
pub unsafe extern "C" fn input_shmem(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut audio: *mut audio_data = 0 as *mut audio_data;
    let mut mmap_area: *mut vis_t = 0 as *mut vis_t;
    let mut fd: libc::c_int = 0;
    let mut mmap_count: libc::c_int = 0;
    let mut buf_frames: libc::c_int = 0;
    let mut req: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut silence_buffer: [s16_t; 16384] = [0; 16384];
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: libc::c_int = 0;
    audio = data as *mut audio_data;
    mmap_count = ::std::mem::size_of::<vis_t>() as libc::c_ulong as libc::c_int;
    req.tv_sec = 0 as libc::c_int as __time_t;
    req.tv_nsec = 0 as libc::c_int as __syscall_slong_t;
    i = 0 as libc::c_int;
    while i < 16384 as libc::c_int {
        silence_buffer[i as usize] = 0 as libc::c_int as s16_t;
        i += 1;
    }
    printf(
        b"input_shmem: source: %s\0" as *const u8 as *const libc::c_char,
        (*audio).source,
    );
    fd = shm_open(
        (*audio).source as *const libc::c_char,
        2 as libc::c_int,
        438 as libc::c_int as mode_t,
    );
    if fd < 0 as libc::c_int {
        tmp = __errno_location();
        tmp___0 = strerror(*tmp);
        printf(
            b"Could not open source '%s': %s\n\0" as *const u8 as *const libc::c_char,
            (*audio).source,
            tmp___0,
        );
        exit(1 as libc::c_int);
    } else {
        tmp___1 = mmap(
            0 as *mut libc::c_void,
            ::std::mem::size_of::<vis_t>() as libc::c_ulong,
            3 as libc::c_int,
            1 as libc::c_int,
            fd,
            0 as libc::c_int as __off_t,
        );
        mmap_area = tmp___1 as *mut vis_t;
        if mmap_area as intptr_t == -(1 as libc::c_long) {
            printf(
                b"mmap failed - check if squeezelite is running with visualization enabled\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    while (*audio).terminate == 0 {
        (*audio).rate = (*mmap_area).rate;
        buf_frames = ((*mmap_area).buf_size).wrapping_div(2 as libc::c_uint)
            as libc::c_int;
        req
            .tv_nsec = (1000000 as libc::c_uint)
            .wrapping_div((*mmap_area).rate)
            .wrapping_mul(buf_frames as u32_t) as __syscall_slong_t;
        if (*mmap_area).running {
            pthread_mutex_lock(&mut lock);
            write_to_fftw_input_buffers(
                buf_frames as int16_t,
                ((*mmap_area).buffer).as_mut_ptr(),
                audio as *mut libc::c_void,
            );
            pthread_mutex_unlock(&mut lock);
            nanosleep(
                &mut req as *mut timespec as *const timespec,
                0 as *mut libc::c_void as *mut timespec,
            );
        } else {
            write_to_fftw_input_buffers(
                buf_frames as int16_t,
                silence_buffer.as_mut_ptr(),
                audio as *mut libc::c_void,
            );
            nanosleep(
                &mut req as *mut timespec as *const timespec,
                0 as *mut libc::c_void as *mut timespec,
            );
        }
    }
    if fd > 0 as libc::c_int {
        tmp___4 = close(fd);
        if tmp___4 != 0 as libc::c_int {
            tmp___2 = __errno_location();
            tmp___3 = strerror(*tmp___2);
            printf(
                b"Could not close file descriptor %d: %s\0" as *const u8
                    as *const libc::c_char,
                fd,
                tmp___3,
            );
        }
    } else {
        printf(b"Wrong file descriptor %d\0" as *const u8 as *const libc::c_char, fd);
    }
    tmp___7 = munmap(mmap_area as *mut libc::c_void, mmap_count as size_t);
    if tmp___7 != 0 as libc::c_int {
        tmp___5 = __errno_location();
        tmp___6 = strerror(*tmp___5);
        printf(
            b"Could not munmap() area %p+%d. %s\0" as *const u8 as *const libc::c_char,
            mmap_area,
            mmap_count,
            tmp___6,
        );
    }
    return 0 as *mut libc::c_void;
}
pub static mut frame_buffer: *mut wchar_t = 0 as *const wchar_t as *mut wchar_t;
pub static mut barstring: [*mut wchar_t; 8] = [0 as *const wchar_t as *mut wchar_t; 8];
pub static mut spacestring: *mut wchar_t = 0 as *const wchar_t as *mut wchar_t;
pub static mut buf_length: libc::c_int = 0;
pub static mut ttyframe_buffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut ttybarstring: [*mut libc::c_char; 8] = [0 as *const libc::c_char
    as *mut libc::c_char; 8];
pub static mut ttyspacestring: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut ttybuf_length: libc::c_int = 0;
pub unsafe extern "C" fn setecho(
    mut fd: libc::c_int,
    mut onoff: libc::c_int,
) -> libc::c_int {
    let mut t: termios = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = tcgetattr(fd, &mut t);
    if tmp == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if onoff == 0 as libc::c_int {
        t.c_lflag &= 4294967173 as libc::c_uint;
    } else {
        t.c_lflag |= 122 as libc::c_uint;
    }
    tmp___0 = tcsetattr(fd, 0 as libc::c_int, &mut t as *mut termios as *const termios);
    if tmp___0 == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn free_terminal_noncurses() {
    let mut i: libc::c_int = 0;
    free(frame_buffer as *mut libc::c_void);
    free(ttyframe_buffer as *mut libc::c_void);
    free(spacestring as *mut libc::c_void);
    free(ttyspacestring as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        free(barstring[i as usize] as *mut libc::c_void);
        free(ttybarstring[i as usize] as *mut libc::c_void);
        i += 1;
    }
}
pub unsafe extern "C" fn init_terminal_noncurses(
    mut tty: libc::c_int,
    mut col: libc::c_int,
    mut bgcol: libc::c_int,
    mut width: libc::c_int,
    mut lines: libc::c_int,
    mut bar_width: libc::c_int,
) -> libc::c_int {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n___0: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n___1: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n___2: libc::c_int = 0;
    let mut n___3: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    free_terminal_noncurses();
    if tty != 0 {
        ttybuf_length = (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(width as libc::c_ulong)
            .wrapping_mul(lines as libc::c_ulong)
            .wrapping_mul(10 as libc::c_ulong) as libc::c_int;
        tmp = malloc(ttybuf_length as size_t);
        ttyframe_buffer = tmp as *mut libc::c_char;
        tmp___0 = malloc(
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((bar_width + 1 as libc::c_int) as libc::c_ulong),
        );
        ttyspacestring = tmp___0 as *mut libc::c_char;
        n = 0 as libc::c_int;
        while n < 8 as libc::c_int {
            tmp___1 = malloc(
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((bar_width + 1 as libc::c_int) as libc::c_ulong),
            );
            ttybarstring[n as usize] = tmp___1 as *mut libc::c_char;
            *(ttybarstring[n as usize])
                .offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
            n += 1;
        }
        *ttyspacestring
            .offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        *ttyframe_buffer
            .offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
        n___0 = 0 as libc::c_int;
        while n___0 < bar_width {
            strcat(
                ttybarstring[0 as libc::c_int as usize],
                b"H\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[1 as libc::c_int as usize],
                b"A\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[2 as libc::c_int as usize],
                b"B\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[3 as libc::c_int as usize],
                b"C\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[4 as libc::c_int as usize],
                b"D\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[5 as libc::c_int as usize],
                b"E\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[6 as libc::c_int as usize],
                b"F\0" as *const u8 as *const libc::c_char,
            );
            strcat(
                ttybarstring[7 as libc::c_int as usize],
                b"G\0" as *const u8 as *const libc::c_char,
            );
            strcat(ttyspacestring, b" \0" as *const u8 as *const libc::c_char);
            n___0 += 1;
        }
    } else if tty == 0 {
        buf_length = (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
            .wrapping_mul(width as libc::c_ulong)
            .wrapping_mul(lines as libc::c_ulong)
            .wrapping_mul(10 as libc::c_ulong) as libc::c_int;
        tmp___2 = malloc(buf_length as size_t);
        frame_buffer = tmp___2 as *mut wchar_t;
        tmp___3 = malloc(
            (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
                .wrapping_mul((bar_width + 1 as libc::c_int) as libc::c_ulong),
        );
        spacestring = tmp___3 as *mut wchar_t;
        n___1 = 0 as libc::c_int;
        while n___1 < 8 as libc::c_int {
            tmp___4 = malloc(
                (::std::mem::size_of::<wchar_t>() as libc::c_ulong)
                    .wrapping_mul((bar_width + 1 as libc::c_int) as libc::c_ulong),
            );
            barstring[n___1 as usize] = tmp___4 as *mut wchar_t;
            *(barstring[n___1 as usize])
                .offset(0 as libc::c_int as isize) = '\u{0}' as i32;
            n___1 += 1;
        }
        *spacestring.offset(0 as libc::c_int as isize) = '\u{0}' as i32;
        *frame_buffer.offset(0 as libc::c_int as isize) = '\u{0}' as i32;
        n___2 = 0 as libc::c_int;
        while n___2 < bar_width {
            wcscat(
                barstring[0 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_int; 5],
                >(b"2\0\0\05\0\0\08\0\0\08\0\0\0\0\0\0\0"))
                    .as_ptr() as *const wchar_t,
            );
            wcscat(
                barstring[1 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_int; 5],
                >(b"2\0\0\05\0\0\08\0\0\01\0\0\0\0\0\0\0"))
                    .as_ptr() as *const wchar_t,
            );
            wcscat(
                barstring[2 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_int; 5],
                >(b"2\0\0\05\0\0\08\0\0\02\0\0\0\0\0\0\0"))
                    .as_ptr() as *const wchar_t,
            );
            wcscat(
                barstring[3 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_int; 5],
                >(b"2\0\0\05\0\0\08\0\0\03\0\0\0\0\0\0\0"))
                    .as_ptr() as *const wchar_t,
            );
            wcscat(
                barstring[4 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_int; 5],
                >(b"2\0\0\05\0\0\08\0\0\04\0\0\0\0\0\0\0"))
                    .as_ptr() as *const wchar_t,
            );
            wcscat(
                barstring[5 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_int; 5],
                >(b"2\0\0\05\0\0\08\0\0\05\0\0\0\0\0\0\0"))
                    .as_ptr() as *const wchar_t,
            );
            wcscat(
                barstring[6 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_int; 5],
                >(b"2\0\0\05\0\0\08\0\0\06\0\0\0\0\0\0\0"))
                    .as_ptr() as *const wchar_t,
            );
            wcscat(
                barstring[7 as libc::c_int as usize],
                (*::std::mem::transmute::<
                    &[u8; 20],
                    &[libc::c_int; 5],
                >(b"2\0\0\05\0\0\08\0\0\07\0\0\0\0\0\0\0"))
                    .as_ptr() as *const wchar_t,
            );
            wcscat(
                spacestring,
                (*::std::mem::transmute::<
                    &[u8; 8],
                    &[libc::c_int; 2],
                >(b" \0\0\0\0\0\0\0"))
                    .as_ptr() as *const wchar_t,
            );
            n___2 += 1;
        }
    }
    col += 30 as libc::c_int;
    system(b"setterm -cursor off\0" as *const u8 as *const libc::c_char);
    system(b"setterm -blank 0\0" as *const u8 as *const libc::c_char);
    printf(b"\x1B[0m\n\0" as *const u8 as *const libc::c_char);
    system(b"clear\0" as *const u8 as *const libc::c_char);
    if col != 0 {
        printf(b"\x1B[%dm\0" as *const u8 as *const libc::c_char, col);
    }
    if bgcol != 0 as libc::c_int {
        bgcol += 40 as libc::c_int;
        printf(b"\x1B[%dm\0" as *const u8 as *const libc::c_char, bgcol);
        n___3 = lines;
        while n___3 >= 0 as libc::c_int {
            i = 0 as libc::c_int;
            while i < width {
                printf(b" \0" as *const u8 as *const libc::c_char);
                i += 1;
            }
            if n___3 != 0 as libc::c_int {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            } else {
                printf(b"\r\0" as *const u8 as *const libc::c_char);
            }
            n___3 -= 1;
        }
        printf(b"\x1B[%dA\0" as *const u8 as *const libc::c_char, lines);
    }
    setecho(0 as libc::c_int, 0 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn get_terminal_dim_noncurses(
    mut width: *mut libc::c_int,
    mut lines: *mut libc::c_int,
) {
    let mut dim: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    ioctl(1 as libc::c_int, 21523 as libc::c_ulong, &mut dim as *mut winsize);
    *lines = dim.ws_row as libc::c_int;
    *width = dim.ws_col as libc::c_int;
    system(b"clear\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn draw_terminal_noncurses(
    mut tty: libc::c_int,
    mut lines: libc::c_int,
    mut width: libc::c_int,
    mut number_of_bars: libc::c_int,
    mut bar_width: libc::c_int,
    mut bar_spacing: libc::c_int,
    mut rest: libc::c_int,
    mut bars: *mut libc::c_int,
    mut previous_frame: *mut libc::c_int,
    mut x_axis_info: libc::c_int,
) -> libc::c_int {
    let mut current_cell: libc::c_int = 0;
    let mut prev_cell: libc::c_int = 0;
    let mut same_line: libc::c_int = 0;
    let mut new_line: libc::c_int = 0;
    let mut cx: libc::c_int = 0;
    let mut dim: winsize = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let mut current_line: libc::c_int = 0;
    let mut same_bar: libc::c_int = 0;
    let mut center_adjusted: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    same_line = 0 as libc::c_int;
    new_line = 0 as libc::c_int;
    cx = 0 as libc::c_int;
    if tty == 0 {
        ioctl(1 as libc::c_int, 21523 as libc::c_ulong, &mut dim as *mut winsize);
        if x_axis_info != 0 {
            lines += 1;
        }
        if dim.ws_row as libc::c_int != lines {
            return -(1 as libc::c_int)
        } else {
            if dim.ws_col as libc::c_int != width {
                return -(1 as libc::c_int);
            }
        }
        if x_axis_info != 0 {
            lines -= 1;
        }
    }
    if tty != 0 {
        *ttyframe_buffer
            .offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    } else if tty == 0 {
        *frame_buffer.offset(0 as libc::c_int as isize) = '\u{0}' as i32;
    }
    current_line = lines - 1 as libc::c_int;
    while current_line >= 0 as libc::c_int {
        same_bar = 0 as libc::c_int;
        center_adjusted = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < number_of_bars {
            current_cell = *bars.offset(i as isize) - current_line * 8 as libc::c_int;
            prev_cell = *previous_frame.offset(i as isize)
                - current_line * 8 as libc::c_int;
            let mut current_block_107: u64;
            if current_cell < 1 as libc::c_int {
                if prev_cell < 1 as libc::c_int {
                    same_bar += 1;
                    current_block_107 = 1934991416718554651;
                } else {
                    current_block_107 = 721932016088910309;
                }
            } else {
                current_block_107 = 721932016088910309;
            }
            match current_block_107 {
                721932016088910309 => {
                    let mut current_block_106: u64;
                    if current_cell > 7 as libc::c_int {
                        if prev_cell > 7 as libc::c_int {
                            same_bar += 1;
                            current_block_106 = 13349765058737954042;
                        } else {
                            current_block_106 = 10294488031519116328;
                        }
                    } else {
                        current_block_106 = 10294488031519116328;
                    }
                    match current_block_106 {
                        10294488031519116328 => {
                            if current_cell == prev_cell {
                                same_bar += 1;
                            } else if tty != 0 {
                                if same_line > 0 as libc::c_int {
                                    tmp = snprintf(
                                        ttyframe_buffer.offset(cx as isize),
                                        (ttybuf_length - cx) as size_t,
                                        b"\x1B[%dB\0" as *const u8 as *const libc::c_char,
                                        same_line,
                                    );
                                    cx += tmp;
                                    new_line += same_line;
                                    same_line = 0 as libc::c_int;
                                }
                                if same_bar > 0 as libc::c_int {
                                    tmp___0 = snprintf(
                                        ttyframe_buffer.offset(cx as isize),
                                        (ttybuf_length - cx) as size_t,
                                        b"\x1B[%dC\0" as *const u8 as *const libc::c_char,
                                        (bar_width + bar_spacing) * same_bar,
                                    );
                                    cx += tmp___0;
                                    same_bar = 0 as libc::c_int;
                                }
                                if center_adjusted == 0 {
                                    if rest != 0 {
                                        tmp___1 = snprintf(
                                            ttyframe_buffer.offset(cx as isize),
                                            (ttybuf_length - cx) as size_t,
                                            b"\x1B[%dC\0" as *const u8 as *const libc::c_char,
                                            rest,
                                        );
                                        cx += tmp___1;
                                        center_adjusted = 1 as libc::c_int;
                                    }
                                }
                                if current_cell < 1 as libc::c_int {
                                    tmp___2 = snprintf(
                                        ttyframe_buffer.offset(cx as isize),
                                        (ttybuf_length - cx) as size_t,
                                        b"%s\0" as *const u8 as *const libc::c_char,
                                        ttyspacestring,
                                    );
                                    cx += tmp___2;
                                } else if current_cell > 7 as libc::c_int {
                                    tmp___3 = snprintf(
                                        ttyframe_buffer.offset(cx as isize),
                                        (ttybuf_length - cx) as size_t,
                                        b"%s\0" as *const u8 as *const libc::c_char,
                                        ttybarstring[0 as libc::c_int as usize],
                                    );
                                    cx += tmp___3;
                                } else {
                                    tmp___4 = snprintf(
                                        ttyframe_buffer.offset(cx as isize),
                                        (ttybuf_length - cx) as size_t,
                                        b"%s\0" as *const u8 as *const libc::c_char,
                                        ttybarstring[current_cell as usize],
                                    );
                                    cx += tmp___4;
                                }
                                if bar_spacing != 0 {
                                    tmp___5 = snprintf(
                                        ttyframe_buffer.offset(cx as isize),
                                        (ttybuf_length - cx) as size_t,
                                        b"\x1B[%dC\0" as *const u8 as *const libc::c_char,
                                        bar_spacing,
                                    );
                                    cx += tmp___5;
                                }
                            } else if tty == 0 {
                                if same_line > 0 as libc::c_int {
                                    tmp___6 = swprintf(
                                        frame_buffer.offset(cx as isize),
                                        (buf_length - cx) as size_t,
                                        (*::std::mem::transmute::<
                                            &[u8; 24],
                                            &[libc::c_int; 6],
                                        >(b"\x1B\0\0\0[\0\0\0%\0\0\0d\0\0\0B\0\0\0\0\0\0\0"))
                                            .as_ptr() as *const wchar_t,
                                        same_line,
                                    );
                                    cx += tmp___6;
                                    new_line += same_line;
                                    same_line = 0 as libc::c_int;
                                }
                                if same_bar > 0 as libc::c_int {
                                    tmp___7 = swprintf(
                                        frame_buffer.offset(cx as isize),
                                        (buf_length - cx) as size_t,
                                        (*::std::mem::transmute::<
                                            &[u8; 24],
                                            &[libc::c_int; 6],
                                        >(b"\x1B\0\0\0[\0\0\0%\0\0\0d\0\0\0C\0\0\0\0\0\0\0"))
                                            .as_ptr() as *const wchar_t,
                                        (bar_width + bar_spacing) * same_bar,
                                    );
                                    cx += tmp___7;
                                    same_bar = 0 as libc::c_int;
                                }
                                if center_adjusted == 0 {
                                    if rest != 0 {
                                        tmp___8 = swprintf(
                                            frame_buffer.offset(cx as isize),
                                            (buf_length - cx) as size_t,
                                            (*::std::mem::transmute::<
                                                &[u8; 24],
                                                &[libc::c_int; 6],
                                            >(b"\x1B\0\0\0[\0\0\0%\0\0\0d\0\0\0C\0\0\0\0\0\0\0"))
                                                .as_ptr() as *const wchar_t,
                                            rest,
                                        );
                                        cx += tmp___8;
                                        center_adjusted = 1 as libc::c_int;
                                    }
                                }
                                if current_cell < 1 as libc::c_int {
                                    tmp___9 = swprintf(
                                        frame_buffer.offset(cx as isize),
                                        (buf_length - cx) as size_t,
                                        spacestring as *const wchar_t,
                                    );
                                    cx += tmp___9;
                                } else if current_cell > 7 as libc::c_int {
                                    tmp___10 = swprintf(
                                        frame_buffer.offset(cx as isize),
                                        (buf_length - cx) as size_t,
                                        barstring[0 as libc::c_int as usize] as *const wchar_t,
                                    );
                                    cx += tmp___10;
                                } else {
                                    tmp___11 = swprintf(
                                        frame_buffer.offset(cx as isize),
                                        (buf_length - cx) as size_t,
                                        barstring[current_cell as usize] as *const wchar_t,
                                    );
                                    cx += tmp___11;
                                }
                                if bar_spacing != 0 {
                                    tmp___12 = swprintf(
                                        frame_buffer.offset(cx as isize),
                                        (buf_length - cx) as size_t,
                                        (*::std::mem::transmute::<
                                            &[u8; 24],
                                            &[libc::c_int; 6],
                                        >(b"\x1B\0\0\0[\0\0\0%\0\0\0d\0\0\0C\0\0\0\0\0\0\0"))
                                            .as_ptr() as *const wchar_t,
                                        bar_spacing,
                                    );
                                    cx += tmp___12;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            i += 1;
        }
        if same_bar != number_of_bars {
            if current_line != 0 as libc::c_int {
                if tty != 0 {
                    tmp___13 = snprintf(
                        ttyframe_buffer.offset(cx as isize),
                        (ttybuf_length - cx) as size_t,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                    cx += tmp___13;
                } else if tty == 0 {
                    tmp___14 = swprintf(
                        frame_buffer.offset(cx as isize),
                        (buf_length - cx) as size_t,
                        (*::std::mem::transmute::<
                            &[u8; 8],
                            &[libc::c_int; 2],
                        >(b"\n\0\0\0\0\0\0\0"))
                            .as_ptr() as *const wchar_t,
                    );
                    cx += tmp___14;
                }
                new_line += 1;
            }
        } else {
            same_line += 1;
        }
        current_line -= 1;
    }
    if same_line != lines {
        if tty != 0 {
            printf(
                b"%s\r\x1B[%dA\0" as *const u8 as *const libc::c_char,
                ttyframe_buffer,
                new_line,
            );
        } else if tty == 0 {
            printf(
                b"%ls\r\x1B[%dA\0" as *const u8 as *const libc::c_char,
                frame_buffer,
                new_line,
            );
        }
        fflush(stdout);
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cleanup_terminal_noncurses() {
    setecho(0 as libc::c_int, 1 as libc::c_int);
    printf(b"\x1B[0m\n\0" as *const u8 as *const libc::c_char);
    system(b"setfont  >/dev/null 2>&1\0" as *const u8 as *const libc::c_char);
    system(
        b"setfont /usr/share/consolefonts/Lat2-Fixed16.psf.gz  >/dev/null 2>&1\0"
            as *const u8 as *const libc::c_char,
    );
    system(b"setterm -cursor on\0" as *const u8 as *const libc::c_char);
    system(b"setterm -blank 10\0" as *const u8 as *const libc::c_char);
    system(b"clear\0" as *const u8 as *const libc::c_char);
}
pub static mut buf_16: int16_t = 0;
pub static mut buf_8: int8_t = 0;
pub unsafe extern "C" fn print_raw_out(
    mut bars_count: libc::c_int,
    mut fd: libc::c_int,
    mut is_binary: libc::c_int,
    mut bit_format: libc::c_int,
    mut ascii_range: libc::c_int,
    mut bar_delim: libc::c_char,
    mut frame_delim: libc::c_char,
    mut f: *const libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut f_limited: libc::c_int = 0;
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_double = 0.;
    let mut i___0: libc::c_int = 0;
    let mut f_ranged: libc::c_int = 0;
    let mut bar_height_size: libc::c_int = 0;
    let mut tmp___1: libc::c_double = 0.;
    let mut tmp___2: libc::c_double = 0.;
    if is_binary != 0 {
        i = 0 as libc::c_int;
        while i < bars_count {
            f_limited = *f.offset(i as isize);
            tmp___0 = pow(
                2 as libc::c_int as libc::c_double,
                bit_format as libc::c_double,
            );
            if f_limited as libc::c_double > tmp___0 - 1 as libc::c_int as libc::c_double
            {
                tmp = pow(
                    2 as libc::c_int as libc::c_double,
                    bit_format as libc::c_double,
                );
                f_limited = (tmp - 1 as libc::c_int as libc::c_double) as libc::c_int;
            }
            match bit_format {
                16 => {
                    buf_16 = f_limited as int16_t;
                    write(
                        fd,
                        &mut buf_16 as *mut int16_t as *const libc::c_void,
                        ::std::mem::size_of::<int16_t>() as libc::c_ulong,
                    );
                }
                8 => {
                    buf_8 = f_limited as int8_t;
                    write(
                        fd,
                        &mut buf_8 as *mut int8_t as *const libc::c_void,
                        ::std::mem::size_of::<int8_t>() as libc::c_ulong,
                    );
                }
                _ => {}
            }
            i += 1;
        }
    } else {
        i___0 = 0 as libc::c_int;
        while i___0 < bars_count {
            f_ranged = *f.offset(i___0 as isize);
            if f_ranged > ascii_range {
                f_ranged = ascii_range;
            }
            bar_height_size = 2 as libc::c_int;
            if f_ranged != 0 as libc::c_int {
                tmp___1 = log10(f_ranged as libc::c_double);
                tmp___2 = floor(tmp___1);
                bar_height_size = (bar_height_size as libc::c_double + tmp___2)
                    as libc::c_int;
            }
            let vla = bar_height_size as usize;
            let mut bar_height: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
            snprintf(
                bar_height.as_mut_ptr(),
                bar_height_size as size_t,
                b"%d\0" as *const u8 as *const libc::c_char,
                f_ranged,
            );
            write(
                fd,
                bar_height.as_mut_ptr() as *const libc::c_void,
                (bar_height_size - 1 as libc::c_int) as size_t,
            );
            write(
                fd,
                &mut bar_delim as *mut libc::c_char as *const libc::c_void,
                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            );
            i___0 += 1;
        }
        write(
            fd,
            &mut frame_delim as *mut libc::c_char as *const libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn initialize_audio_parameters(
    mut handle: *mut *mut snd_pcm_t,
    mut audio: *mut audio_data,
    mut frames: *mut snd_pcm_uframes_t,
) {
    let mut err: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut params: *mut snd_pcm_hw_params_t = 0 as *mut snd_pcm_hw_params_t;
    let mut tmp___1: size_t = 0;
    let mut tmp___2 = 0 as *mut _;
    let mut tmp___3: size_t = 0;
    let mut sample_rate: libc::c_uint = 0;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    tmp = snd_pcm_open(
        handle,
        (*audio).source as *const libc::c_char,
        SND_PCM_STREAM_CAPTURE,
        0 as libc::c_int,
    );
    err = tmp;
    if err < 0 as libc::c_int {
        tmp___0 = snd_strerror(err);
        fprintf(
            stderr,
            b"error opening stream: %s\n\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
        exit(1 as libc::c_int);
    }
    tmp___1 = snd_pcm_hw_params_sizeof();
    let mut fresh8 = ::std::vec::from_elem(0, tmp___1 as usize);
    tmp___2 = fresh8.as_mut_ptr();
    params = tmp___2 as *mut snd_pcm_hw_params_t;
    tmp___3 = snd_pcm_hw_params_sizeof();
    memset(params as *mut libc::c_void, 0 as libc::c_int, tmp___3);
    snd_pcm_hw_params_any(*handle, params);
    snd_pcm_hw_params_set_access(*handle, params, SND_PCM_ACCESS_RW_INTERLEAVED);
    snd_pcm_hw_params_set_format(*handle, params, SND_PCM_FORMAT_S16_LE);
    snd_pcm_hw_params_set_channels(*handle, params, 2 as libc::c_uint);
    sample_rate = 44100 as libc::c_uint;
    snd_pcm_hw_params_set_rate_near(
        *handle,
        params,
        &mut sample_rate,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    snd_pcm_hw_params_set_period_size_near(
        *handle,
        params,
        frames,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    err = snd_pcm_hw_params(*handle, params);
    if err < 0 as libc::c_int {
        tmp___4 = snd_strerror(err);
        fprintf(
            stderr,
            b"unable to set hw parameters: %s\n\0" as *const u8 as *const libc::c_char,
            tmp___4,
        );
        exit(1 as libc::c_int);
    }
    err = snd_pcm_prepare(*handle);
    if err < 0 as libc::c_int {
        tmp___5 = snd_strerror(err);
        fprintf(
            stderr,
            b"cannot prepare audio interface for use (%s)\n\0" as *const u8
                as *const libc::c_char,
            tmp___5,
        );
        exit(1 as libc::c_int);
    }
    snd_pcm_hw_params_get_format(
        params as *const snd_pcm_hw_params_t,
        &mut sample_rate as *mut libc::c_uint as *mut snd_pcm_format_t,
    );
    if sample_rate <= 5 as libc::c_uint {
        (*audio).format = 16 as libc::c_int;
    } else if sample_rate <= 9 as libc::c_uint {
        (*audio).format = 24 as libc::c_int;
    } else {
        (*audio).format = 32 as libc::c_int;
    }
    snd_pcm_hw_params_get_rate(
        params as *const snd_pcm_hw_params_t,
        &mut (*audio).rate,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
    snd_pcm_hw_params_get_period_size(
        params as *const snd_pcm_hw_params_t,
        frames,
        0 as *mut libc::c_void as *mut libc::c_int,
    );
}
unsafe extern "C" fn get_certain_frame(
    mut buffer: *mut libc::c_schar,
    mut buffer_index: libc::c_int,
    mut adjustment: libc::c_int,
) -> libc::c_int {
    let mut temp: libc::c_int = 0;
    let mut lo: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    temp = (*buffer.offset((buffer_index + adjustment - 1 as libc::c_int) as isize)
        as libc::c_int) << 2 as libc::c_int;
    lo = *buffer.offset((buffer_index + adjustment - 2 as libc::c_int) as isize)
        as libc::c_int >> 6 as libc::c_int;
    if lo < 0 as libc::c_int {
        tmp = abs(lo);
        lo = tmp + 1 as libc::c_int;
    }
    if temp >= 0 as libc::c_int {
        temp += lo;
    } else {
        temp -= lo;
    }
    return temp;
}
pub unsafe extern "C" fn input_alsa(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut err: libc::c_int = 0;
    let mut audio: *mut audio_data = 0 as *mut audio_data;
    let mut handle: *mut snd_pcm_t = 0 as *mut snd_pcm_t;
    let mut buffer_size: snd_pcm_uframes_t = 0;
    let mut period_size: snd_pcm_uframes_t = 0;
    let mut frames: snd_pcm_uframes_t = 0;
    let mut radj: libc::c_int = 0;
    let mut ladj: libc::c_int = 0;
    let mut buffer: *mut libc::c_schar = 0 as *mut libc::c_schar;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: snd_pcm_sframes_t = 0;
    let mut tmp___1: snd_pcm_sframes_t = 0;
    let mut i: uint16_t = 0;
    let mut tmp___2: libc::c_double = 0.;
    let mut tmp___3: snd_pcm_sframes_t = 0;
    let mut i___0: uint16_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    audio = data as *mut audio_data;
    frames = (*audio).FFTtreblebufferSize as snd_pcm_uframes_t;
    initialize_audio_parameters(&mut handle, audio, &mut frames);
    snd_pcm_get_params(handle, &mut buffer_size, &mut period_size);
    radj = (*audio).format / 4 as libc::c_int;
    ladj = (*audio).format / 8 as libc::c_int;
    let vla = period_size as usize;
    let mut buf: Vec::<int16_t> = ::std::vec::from_elem(0, vla);
    let vla_0 = period_size as usize;
    let mut buffer32: Vec::<int32_t> = ::std::vec::from_elem(0, vla_0);
    frames = period_size
        .wrapping_div(
            ((*audio).format / 8 as libc::c_int * 2 as libc::c_int) as snd_pcm_uframes_t,
        );
    tmp = malloc(period_size);
    buffer = tmp as *mut libc::c_schar;
    while (*audio).terminate == 0 {
        match (*audio).format {
            16 => {
                tmp___0 = snd_pcm_readi(
                    handle,
                    buf.as_mut_ptr() as *mut libc::c_void,
                    frames,
                );
                err = tmp___0 as libc::c_int;
            }
            32 => {
                tmp___1 = snd_pcm_readi(
                    handle,
                    buffer32.as_mut_ptr() as *mut libc::c_void,
                    frames,
                );
                err = tmp___1 as libc::c_int;
                i = 0 as libc::c_int as uint16_t;
                while (i as snd_pcm_uframes_t) < frames.wrapping_mul(2 as libc::c_ulong)
                {
                    tmp___2 = pow(
                        2 as libc::c_int as libc::c_double,
                        16 as libc::c_int as libc::c_double,
                    );
                    *buf
                        .as_mut_ptr()
                        .offset(
                            i as isize,
                        ) = (*buffer32.as_mut_ptr().offset(i as isize) as libc::c_double
                        / tmp___2) as int16_t;
                    i = (i as libc::c_int + 1 as libc::c_int) as uint16_t;
                }
            }
            _ => {
                tmp___3 = snd_pcm_readi(handle, buffer as *mut libc::c_void, frames);
                err = tmp___3 as libc::c_int;
                i___0 = 0 as libc::c_int as uint16_t;
                while (i___0 as snd_pcm_uframes_t)
                    < period_size.wrapping_mul(2 as libc::c_ulong)
                {
                    tmp___4 = get_certain_frame(buffer, i___0 as libc::c_int, ladj);
                    *buf.as_mut_ptr().offset(i___0 as isize) = tmp___4 as int16_t;
                    tmp___5 = get_certain_frame(buffer, i___0 as libc::c_int, radj);
                    *buf
                        .as_mut_ptr()
                        .offset(
                            (i___0 as libc::c_int + 1 as libc::c_int) as isize,
                        ) = tmp___5 as int16_t;
                    i___0 = (i___0 as libc::c_int + ladj * 2 as libc::c_int) as uint16_t;
                }
            }
        }
        if err == -(32 as libc::c_int) {
            snd_pcm_prepare(handle);
        } else if !(err < 0 as libc::c_int) {
            err != frames as libc::c_int;
        }
        pthread_mutex_lock(&mut lock);
        write_to_fftw_input_buffers(frames as int16_t, buf.as_mut_ptr(), data);
        pthread_mutex_unlock(&mut lock);
    }
    free(buffer as *mut libc::c_void);
    snd_pcm_close(handle);
    return 0 as *mut libc::c_void;
}
pub static mut m_pulseaudio_mainloop: *mut pa_mainloop = 0 as *const pa_mainloop
    as *mut pa_mainloop;
pub unsafe extern "C" fn cb(
    mut pulseaudio_context: *mut pa_context,
    mut i: *const pa_server_info,
    mut userdata: *mut libc::c_void,
) {
    let mut audio: *mut audio_data = 0 as *mut audio_data;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    audio = userdata as *mut audio_data;
    free((*audio).source as *mut libc::c_void);
    tmp = malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(1024 as libc::c_ulong),
    );
    (*audio).source = tmp as *mut libc::c_char;
    strcpy((*audio).source, (*i).default_sink_name);
    (*audio)
        .source = strcat(
        (*audio).source,
        b".monitor\0" as *const u8 as *const libc::c_char,
    );
    pa_context_disconnect(pulseaudio_context);
    pa_context_unref(pulseaudio_context);
    pa_mainloop_quit(m_pulseaudio_mainloop, 0 as libc::c_int);
    pa_mainloop_free(m_pulseaudio_mainloop);
}
pub unsafe extern "C" fn pulseaudio_context_state_callback(
    mut pulseaudio_context: *mut pa_context,
    mut userdata: *mut libc::c_void,
) {
    let mut tmp: pa_context_state_t = PA_CONTEXT_UNCONNECTED;
    let mut tmp___0: *mut pa_operation = 0 as *mut pa_operation;
    tmp = pa_context_get_state(pulseaudio_context as *const pa_context);
    match tmp as libc::c_uint {
        4 => {
            tmp___0 = pa_context_get_server_info(
                pulseaudio_context,
                Some(
                    cb
                        as unsafe extern "C" fn(
                            *mut pa_context,
                            *const pa_server_info,
                            *mut libc::c_void,
                        ) -> (),
                ),
                userdata,
            );
            pa_operation_unref(tmp___0);
        }
        5 => {
            printf(
                b"failed to connect to pulseaudio server\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        6 => {
            pa_mainloop_quit(m_pulseaudio_mainloop, 0 as libc::c_int);
        }
        0 | 1 | 2 | 3 | _ => {}
    };
}
pub unsafe extern "C" fn getPulseDefaultSink(mut data: *mut libc::c_void) {
    let mut audio: *mut audio_data = 0 as *mut audio_data;
    let mut mainloop_api: *mut pa_mainloop_api = 0 as *mut pa_mainloop_api;
    let mut pulseaudio_context: *mut pa_context = 0 as *mut pa_context;
    let mut ret: libc::c_int = 0;
    audio = data as *mut audio_data;
    m_pulseaudio_mainloop = pa_mainloop_new();
    mainloop_api = pa_mainloop_get_api(m_pulseaudio_mainloop);
    pulseaudio_context = pa_context_new(
        mainloop_api,
        b"cava device list\0" as *const u8 as *const libc::c_char,
    );
    pa_context_connect(
        pulseaudio_context,
        0 as *mut libc::c_void as *const libc::c_char,
        PA_CONTEXT_NOFLAGS,
        0 as *mut libc::c_void as *const pa_spawn_api,
    );
    pa_context_set_state_callback(
        pulseaudio_context,
        Some(
            pulseaudio_context_state_callback
                as unsafe extern "C" fn(*mut pa_context, *mut libc::c_void) -> (),
        ),
        audio as *mut libc::c_void,
    );
    ret = pa_mainloop_iterate(m_pulseaudio_mainloop, 0 as libc::c_int, &mut ret);
    if ret == 0 {
        printf(
            b"Could not open pulseaudio mainloop to find default device name: %d\ncheck if pulseaudio is running\n\0"
                as *const u8 as *const libc::c_char,
            ret,
        );
        exit(1 as libc::c_int);
    }
    pa_mainloop_run(m_pulseaudio_mainloop, &mut ret);
}
static mut ss: pa_sample_spec = {
    let mut init = pa_sample_spec {
        format: PA_SAMPLE_S16LE,
        rate: 44100 as libc::c_int as uint32_t,
        channels: 2 as libc::c_int as uint8_t,
    };
    init
};
pub unsafe extern "C" fn input_pulse(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut audio: *mut audio_data = 0 as *mut audio_data;
    let mut frames: uint16_t = 0;
    let mut channels___0: libc::c_int = 0;
    let mut frag_size: libc::c_int = 0;
    let mut pb: pa_buffer_attr = pa_buffer_attr {
        maxlength: 0,
        tlength: 0,
        prebuf: 0,
        minreq: 0,
        fragsize: 0,
    };
    let mut s: *mut pa_simple = 0 as *mut pa_simple;
    let mut error: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    audio = data as *mut audio_data;
    frames = (*audio).FFTtreblebufferSize as uint16_t;
    channels___0 = 2 as libc::c_int;
    let vla = (frames as libc::c_int * channels___0) as usize;
    let mut buf: Vec::<int16_t> = ::std::vec::from_elem(0, vla);
    (*audio).format = 16 as libc::c_int;
    frag_size = frames as libc::c_int * channels___0 * (*audio).format / 8 as libc::c_int
        * 2 as libc::c_int;
    pb.maxlength = -(1 as libc::c_int) as uint32_t;
    pb.tlength = 0 as libc::c_uint;
    pb.prebuf = 0 as libc::c_uint;
    pb.minreq = 0 as libc::c_uint;
    pb.fragsize = frag_size as uint32_t;
    s = 0 as *mut libc::c_void as *mut pa_simple;
    s = pa_simple_new(
        0 as *mut libc::c_void as *const libc::c_char,
        b"cava\0" as *const u8 as *const libc::c_char,
        PA_STREAM_RECORD,
        (*audio).source as *const libc::c_char,
        b"audio for cava\0" as *const u8 as *const libc::c_char,
        &ss,
        0 as *mut libc::c_void as *const pa_channel_map,
        &mut pb as *mut pa_buffer_attr as *const pa_buffer_attr,
        &mut error,
    );
    if s.is_null() {
        tmp = pa_strerror(error);
        sprintf(
            ((*audio).error_message).as_mut_ptr(),
            b"input/pulse.c: Could not open pulseaudio source: %s, %s. \t\tTo find a list of your pulseaudio sources run 'pacmd list-sources'\n\0"
                as *const u8 as *const libc::c_char,
            (*audio).source,
            tmp,
        );
        (*audio).terminate = 1 as libc::c_int;
    }
    while (*audio).terminate == 0 {
        tmp___1 = pa_simple_read(
            s,
            buf.as_mut_ptr() as *mut libc::c_void,
            (vla * ::std::mem::size_of::<int16_t>()) as libc::c_ulong,
            &mut error,
        );
        if tmp___1 < 0 as libc::c_int {
            tmp___0 = pa_strerror(error);
            sprintf(
                ((*audio).error_message).as_mut_ptr(),
                b"input/pulse.c: pa_simple_read() failed: %s\n\0" as *const u8
                    as *const libc::c_char,
                tmp___0,
            );
            (*audio).terminate = 1 as libc::c_int;
        }
        pthread_mutex_lock(&mut lock);
        write_to_fftw_input_buffers(frames as int16_t, buf.as_mut_ptr(), data);
        pthread_mutex_unlock(&mut lock);
    }
    pa_simple_free(s);
    pthread_exit(0 as *mut libc::c_void);
}
pub static mut gradient_size: libc::c_int = 64 as libc::c_int;
pub static mut bar_heights: [*const wchar_t; 8] = unsafe {
    [
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_int; 5],
        >(b"2\0\0\05\0\0\08\0\0\01\0\0\0\0\0\0\0"))
            .as_ptr() as *const wchar_t,
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_int; 5],
        >(b"2\0\0\05\0\0\08\0\0\02\0\0\0\0\0\0\0"))
            .as_ptr() as *const wchar_t,
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_int; 5],
        >(b"2\0\0\05\0\0\08\0\0\03\0\0\0\0\0\0\0"))
            .as_ptr() as *const wchar_t,
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_int; 5],
        >(b"2\0\0\05\0\0\08\0\0\04\0\0\0\0\0\0\0"))
            .as_ptr() as *const wchar_t,
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_int; 5],
        >(b"2\0\0\05\0\0\08\0\0\05\0\0\0\0\0\0\0"))
            .as_ptr() as *const wchar_t,
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_int; 5],
        >(b"2\0\0\05\0\0\08\0\0\06\0\0\0\0\0\0\0"))
            .as_ptr() as *const wchar_t,
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_int; 5],
        >(b"2\0\0\05\0\0\08\0\0\07\0\0\0\0\0\0\0"))
            .as_ptr() as *const wchar_t,
        (*::std::mem::transmute::<
            &[u8; 20],
            &[libc::c_int; 5],
        >(b"2\0\0\05\0\0\08\0\0\08\0\0\0\0\0\0\0"))
            .as_ptr() as *const wchar_t,
    ]
};
pub static mut num_bar_heights: libc::c_int = 0;
unsafe extern "C" fn parse_color(
    mut color_string: *mut libc::c_char,
    mut color: *mut colors,
) {
    let mut tmp: bool = false;
    if *color_string.offset(0 as libc::c_int as isize) as libc::c_int
        == 35 as libc::c_int
    {
        tmp = can_change_color();
        if !tmp {
            cleanup_terminal_ncurses();
            fprintf(
                stderr,
                b"Your terminal can not change color definitions, please use one of the predefined colors.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        (*color).color = -(2 as libc::c_int) as libc::c_short;
        color_string = color_string.offset(1);
        sscanf(
            color_string as *const libc::c_char,
            b"%02hx%02hx%02hx\0" as *const u8 as *const libc::c_char,
            &mut (*color).R as *mut libc::c_short,
            &mut (*color).G as *mut libc::c_short,
            &mut (*color).B as *mut libc::c_short,
        );
    }
}
unsafe extern "C" fn change_color_definition(
    mut color_number: libc::c_short,
    color_string: *mut libc::c_char,
    mut predef_color: libc::c_short,
) -> libc::c_short {
    let mut color: colors = colors {
        color: 0,
        R: 0,
        G: 0,
        B: 0,
    };
    let mut return_color_number: libc::c_short = 0;
    color.color = 0 as libc::c_int as libc::c_short;
    color.R = 0 as libc::c_int as libc::c_short;
    color.G = 0 as libc::c_int as libc::c_short;
    color.B = 0 as libc::c_int as libc::c_short;
    parse_color(color_string, &mut color);
    return_color_number = predef_color;
    if color.color as libc::c_int == -(2 as libc::c_int) {
        init_color(
            color_number,
            (color.R as libc::c_double * 1000.0f64 / 255 as libc::c_int as libc::c_double
                + 0.5f64) as libc::c_short,
            (color.G as libc::c_double * 1000.0f64 / 255 as libc::c_int as libc::c_double
                + 0.5f64) as libc::c_short,
            (color.B as libc::c_double * 1000.0f64 / 255 as libc::c_int as libc::c_double
                + 0.5f64) as libc::c_short,
        );
        return_color_number = color_number;
    }
    return return_color_number;
}
pub unsafe extern "C" fn init_terminal_ncurses(
    fg_color_string: *mut libc::c_char,
    bg_color_string: *mut libc::c_char,
    mut predef_fg_color: libc::c_int,
    mut predef_bg_color: libc::c_int,
    mut gradient: libc::c_int,
    mut gradient_count: libc::c_int,
    mut gradient_colors: *mut *mut libc::c_char,
    mut width: *mut libc::c_int,
    mut lines: *mut libc::c_int,
) {
    let mut color_pair_number: libc::c_short = 0;
    let mut bg_color_number: libc::c_short = 0;
    let mut fg_color_number: libc::c_short = 0;
    let mut next_color: [libc::c_char; 14] = [0; 14];
    let mut i: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut individual_size: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut col___0: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut col___1: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    initscr();
    curs_set(0 as libc::c_int);
    wtimeout(stdscr, 0 as libc::c_int);
    noecho();
    start_color();
    use_default_colors();
    if 0 as *const libc::c_void as libc::c_ulong
        != stdscr as *const libc::c_void as libc::c_ulong
    {
        *lines = (*stdscr)._maxy as libc::c_int + 1 as libc::c_int;
    } else {
        *lines = -(1 as libc::c_int);
    }
    if 0 as *const libc::c_void as libc::c_ulong
        != stdscr as *const libc::c_void as libc::c_ulong
    {
        *width = (*stdscr)._maxx as libc::c_int + 1 as libc::c_int;
    } else {
        *width = -(1 as libc::c_int);
    }
    wclear(stdscr);
    color_pair_number = 16 as libc::c_int as libc::c_short;
    bg_color_number = change_color_definition(
        0 as libc::c_int as libc::c_short,
        bg_color_string,
        predef_bg_color as libc::c_short,
    );
    if gradient == 0 {
        fg_color_number = change_color_definition(
            1 as libc::c_int as libc::c_short,
            fg_color_string,
            predef_fg_color as libc::c_short,
        );
        init_pair(color_pair_number, fg_color_number, bg_color_number);
    } else if gradient != 0 {
        let vla = (2 as libc::c_int * gradient_count - 1 as libc::c_int) as usize;
        let mut rgb: Vec::<[libc::c_ushort; 3]> = ::std::vec::from_elem([0; 3], vla);
        gradient_size = *lines;
        if gradient_size > COLORS {
            gradient_size = COLORS - 1 as libc::c_int;
        }
        if gradient_size > COLOR_PAIRS {
            gradient_size = COLOR_PAIRS - 1 as libc::c_int;
        }
        if gradient_size > 256 as libc::c_int {
            gradient_size = 255 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < gradient_count {
            col = (i + 1 as libc::c_int) * 2 as libc::c_int - 2 as libc::c_int;
            sscanf(
                (*gradient_colors.offset(i as isize)).offset(1 as libc::c_int as isize)
                    as *const libc::c_char,
                b"%02hx%02hx%02hx\0" as *const u8 as *const libc::c_char,
                &mut *(*rgb.as_mut_ptr().offset(col as isize))
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut libc::c_ushort,
                &mut *(*rgb.as_mut_ptr().offset(col as isize))
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize) as *mut libc::c_ushort,
                &mut *(*rgb.as_mut_ptr().offset(col as isize))
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as *mut libc::c_ushort,
            );
            i += 1;
        }
        individual_size = gradient_size / (gradient_count - 1 as libc::c_int);
        i___0 = 0 as libc::c_int;
        while i___0 < gradient_count - 1 as libc::c_int {
            col___0 = (i___0 + 1 as libc::c_int) * 2 as libc::c_int - 2 as libc::c_int;
            if i___0 == gradient_count - 1 as libc::c_int {
                col___0 = 2 as libc::c_int * (gradient_count - 1 as libc::c_int)
                    - 2 as libc::c_int;
            }
            j = 0 as libc::c_int;
            while j < individual_size {
                k = 0 as libc::c_int;
                while k < 3 as libc::c_int {
                    (*rgb
                        .as_mut_ptr()
                        .offset(
                            (col___0 + 1 as libc::c_int) as isize,
                        ))[k
                        as usize] = ((*rgb
                        .as_mut_ptr()
                        .offset(col___0 as isize))[k as usize] as libc::c_double
                        + ((*rgb
                            .as_mut_ptr()
                            .offset((col___0 + 2 as libc::c_int) as isize))[k as usize]
                            as libc::c_int
                            - (*rgb.as_mut_ptr().offset(col___0 as isize))[k as usize]
                                as libc::c_int) as libc::c_double
                            * (j as libc::c_double
                                / (individual_size as libc::c_double * 0.85f64)))
                        as libc::c_ushort;
                    if (*rgb
                        .as_mut_ptr()
                        .offset((col___0 + 1 as libc::c_int) as isize))[k as usize]
                        as libc::c_int > 255 as libc::c_int
                    {
                        (*rgb
                            .as_mut_ptr()
                            .offset(
                                col___0 as isize,
                            ))[k as usize] = 0 as libc::c_int as libc::c_ushort;
                    }
                    if j as libc::c_double > individual_size as libc::c_double * 0.85f64
                    {
                        (*rgb
                            .as_mut_ptr()
                            .offset(
                                (col___0 + 1 as libc::c_int) as isize,
                            ))[k
                            as usize] = (*rgb
                            .as_mut_ptr()
                            .offset((col___0 + 2 as libc::c_int) as isize))[k as usize];
                    }
                    k += 1;
                }
                sprintf(
                    next_color.as_mut_ptr(),
                    b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
                    (*rgb
                        .as_mut_ptr()
                        .offset(
                            (col___0 + 1 as libc::c_int) as isize,
                        ))[0 as libc::c_int as usize] as libc::c_int,
                    (*rgb
                        .as_mut_ptr()
                        .offset(
                            (col___0 + 1 as libc::c_int) as isize,
                        ))[1 as libc::c_int as usize] as libc::c_int,
                    (*rgb
                        .as_mut_ptr()
                        .offset(
                            (col___0 + 1 as libc::c_int) as isize,
                        ))[2 as libc::c_int as usize] as libc::c_int,
                );
                change_color_definition(
                    color_pair_number,
                    next_color.as_mut_ptr(),
                    color_pair_number,
                );
                init_pair(color_pair_number, color_pair_number, bg_color_number);
                color_pair_number = (color_pair_number as libc::c_int + 1 as libc::c_int)
                    as libc::c_short;
                j += 1;
            }
            i___0 += 1;
        }
        left = individual_size * (gradient_count - 1 as libc::c_int);
        col___1 = 2 as libc::c_int * gradient_count - 2 as libc::c_int;
        while left < gradient_size {
            sprintf(
                next_color.as_mut_ptr(),
                b"#%02x%02x%02x\0" as *const u8 as *const libc::c_char,
                (*rgb.as_mut_ptr().offset(col___1 as isize))[0 as libc::c_int as usize]
                    as libc::c_int,
                (*rgb.as_mut_ptr().offset(col___1 as isize))[1 as libc::c_int as usize]
                    as libc::c_int,
                (*rgb.as_mut_ptr().offset(col___1 as isize))[2 as libc::c_int as usize]
                    as libc::c_int,
            );
            change_color_definition(
                color_pair_number,
                next_color.as_mut_ptr(),
                color_pair_number,
            );
            init_pair(color_pair_number, color_pair_number, bg_color_number);
            color_pair_number = (color_pair_number as libc::c_int + 1 as libc::c_int)
                as libc::c_short;
            left += 1;
        }
        color_pair_number = (color_pair_number as libc::c_int - 1 as libc::c_int)
            as libc::c_short;
    }
    wattr_on(
        stdscr,
        (color_pair_number as chtype) << 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if bg_color_number as libc::c_int != -(1 as libc::c_int) {
        wbkgd(
            stdscr,
            (color_pair_number as chtype) << 8 as libc::c_int
                & ((1 as libc::c_uint) << 8 as libc::c_int)
                    .wrapping_sub(1 as libc::c_uint) << 8 as libc::c_int,
        );
    }
    y = 0 as libc::c_int;
    while y < *lines {
        x = 0 as libc::c_int;
        while x < *width {
            tmp = wmove(stdscr, y, x);
            if !(tmp == -(1 as libc::c_int)) {
                waddch(stdscr, ' ' as i32 as chtype);
            }
            x += 1;
        }
        y += 1;
    }
    wrefresh(stdscr);
}
pub unsafe extern "C" fn change_colors(
    mut cur_height: libc::c_int,
    mut tot_height: libc::c_int,
) {
    tot_height /= gradient_size;
    if tot_height < 1 as libc::c_int {
        tot_height = 1 as libc::c_int;
    }
    cur_height /= tot_height;
    if cur_height > gradient_size - 1 as libc::c_int {
        cur_height = gradient_size - 1 as libc::c_int;
    }
    wattr_on(
        stdscr,
        ((cur_height + 16 as libc::c_int) as chtype) << 8 as libc::c_int
            & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
                << 8 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
pub unsafe extern "C" fn get_terminal_dim_ncurses(
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    if 0 as *const libc::c_void as libc::c_ulong
        != stdscr as *const libc::c_void as libc::c_ulong
    {
        *height = (*stdscr)._maxy as libc::c_int + 1 as libc::c_int;
    } else {
        *height = -(1 as libc::c_int);
    }
    if 0 as *const libc::c_void as libc::c_ulong
        != stdscr as *const libc::c_void as libc::c_ulong
    {
        *width = (*stdscr)._maxx as libc::c_int + 1 as libc::c_int;
    } else {
        *width = -(1 as libc::c_int);
    }
    gradient_size = *height;
    wclear(stdscr);
}
pub unsafe extern "C" fn draw_terminal_ncurses(
    mut is_tty: libc::c_int,
    mut terminal_height: libc::c_int,
    mut terminal_width: libc::c_int,
    mut bars_count: libc::c_int,
    mut bar_width: libc::c_int,
    mut bar_spacing: libc::c_int,
    mut rest: libc::c_int,
    mut bars: *const libc::c_int,
    mut previous_frame: *mut libc::c_int,
    mut gradient: libc::c_int,
    mut x_axis_info: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut height: libc::c_int = 0;
    let mut max_update_y: libc::c_int = 0;
    let mut bar: libc::c_int = 0;
    let mut _a: libc::c_int = 0;
    let mut _b___0: libc::c_int = 0;
    let mut _a___1: libc::c_int = 0;
    let mut _b___1: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut bar___0: libc::c_int = 0;
    let mut cur_col: libc::c_int = 0;
    let mut f_cell: libc::c_int = 0;
    let mut f_last_cell: libc::c_int = 0;
    let mut bar_step: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut col___0: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    height = terminal_height - 1 as libc::c_int;
    if is_tty == 0 {
        if x_axis_info != 0 {
            terminal_height += 1;
        }
        's_94: {
            if !(LINES != terminal_height) {
                if !(COLS != terminal_width) {
                    break 's_94;
                }
            }
            return -(1 as libc::c_int);
        }
    }
    max_update_y = 0 as libc::c_int;
    bar = 0 as libc::c_int;
    while bar < bars_count {
        _a = max_update_y;
        _a___1 = *bars.offset(bar as isize);
        _b___1 = *previous_frame.offset(bar as isize);
        if _a___1 > _b___1 {
            tmp___0 = _a___1;
        } else {
            tmp___0 = _b___1;
        }
        _b___0 = tmp___0;
        if _a > _b___0 {
            tmp___1 = _a;
        } else {
            tmp___1 = _b___0;
        }
        max_update_y = tmp___1;
        bar += 1;
    }
    max_update_y = (max_update_y + num_bar_heights) / num_bar_heights;
    y = 0 as libc::c_int;
    while y < max_update_y {
        if gradient != 0 {
            change_colors(y, height);
        }
        bar___0 = 0 as libc::c_int;
        while bar___0 < bars_count {
            if !(*bars.offset(bar___0 as isize)
                == *previous_frame.offset(bar___0 as isize))
            {
                cur_col = bar___0 * bar_width + bar___0 * bar_spacing + rest;
                f_cell = (*bars.offset(bar___0 as isize) - 1 as libc::c_int)
                    / num_bar_heights;
                f_last_cell = (*previous_frame.offset(bar___0 as isize)
                    - 1 as libc::c_int) / num_bar_heights;
                if f_cell >= y {
                    if f_cell == y {
                        bar_step = (*bars.offset(bar___0 as isize) - 1 as libc::c_int)
                            % num_bar_heights;
                        current_block = 13678349939556791712;
                    } else if f_last_cell <= y {
                        bar_step = num_bar_heights - 1 as libc::c_int;
                        current_block = 13678349939556791712;
                    } else {
                        current_block = 1927281346075326425;
                    }
                    match current_block {
                        1927281346075326425 => {}
                        _ => {
                            col = cur_col;
                            i = 0 as libc::c_int;
                            while i < bar_width {
                                if is_tty != 0 {
                                    tmp___2 = wmove(stdscr, height - y, col);
                                    if !(tmp___2 == -(1 as libc::c_int)) {
                                        waddch(stdscr, (65 as libc::c_int + bar_step) as chtype);
                                    }
                                } else {
                                    tmp___3 = wmove(stdscr, height - y, col);
                                    if !(tmp___3 == -(1 as libc::c_int)) {
                                        waddnwstr(
                                            stdscr,
                                            bar_heights[bar_step as usize],
                                            -(1 as libc::c_int),
                                        );
                                    }
                                }
                                i += 1;
                                col += 1;
                            }
                        }
                    }
                } else if f_last_cell >= y {
                    col___0 = cur_col;
                    i___0 = 0 as libc::c_int;
                    while i___0 < bar_width {
                        tmp___4 = wmove(stdscr, height - y, col___0);
                        if !(tmp___4 == -(1 as libc::c_int)) {
                            waddch(stdscr, ' ' as i32 as chtype);
                        }
                        i___0 += 1;
                        col___0 += 1;
                    }
                }
            }
            bar___0 += 1;
        }
        y += 1;
    }
    wrefresh(stdscr);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn cleanup_terminal_ncurses() {
    echo();
    system(b"setfont  >/dev/null 2>&1\0" as *const u8 as *const libc::c_char);
    system(
        b"setfont /usr/share/consolefonts/Lat2-Fixed16.psf.gz  >/dev/null 2>&1\0"
            as *const u8 as *const libc::c_char,
    );
    system(b"setterm -blank 10  >/dev/null 2>&1\0" as *const u8 as *const libc::c_char);
    wattrset(stdscr, 0 as libc::c_int);
    endwin();
    system(b"clear\0" as *const u8 as *const libc::c_char);
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
unsafe extern "C" fn run_static_initializers() {
    num_bar_heights = (::std::mem::size_of::<[*const wchar_t; 8]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<*const wchar_t>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
