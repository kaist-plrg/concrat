use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type real_pcre;
    pub type __dirstream;
    pub type lzma_internal_s;
    pub type internal_state;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn getc_unlocked(__fp: *mut FILE) -> libc::c_int;
    fn __getdelim(
        __lineptr: *mut *mut libc::c_char,
        __n: *mut size_t,
        __delimiter: libc::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn pcre_exec(
        _: *const pcre,
        _: *const pcre_extra,
        _: *const libc::c_char,
        _: libc::c_int,
        _: libc::c_int,
        _: libc::c_int,
        _: *mut libc::c_int,
        _: libc::c_int,
    ) -> libc::c_int;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn realpath(
        __name: *const libc::c_char,
        __resolved: *mut libc::c_char,
    ) -> *mut libc::c_char;
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
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut pcre_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
    fn madvise(
        __addr: *mut libc::c_void,
        __len: size_t,
        __advice: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn pthread_exit(__retval: *mut libc::c_void) -> !;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn vasprintf(
        __ptr: *mut *mut libc::c_char,
        __f: *const libc::c_char,
        __arg: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn flockfile(__stream: *mut FILE);
    fn funlockfile(__stream: *mut FILE);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn pcre_compile(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *const libc::c_char,
        _: *mut libc::c_int,
        _: *const libc::c_uchar,
    ) -> *mut pcre;
    fn pcre_study(
        _: *const pcre,
        _: libc::c_int,
        _: *mut *const libc::c_char,
    ) -> *mut pcre_extra;
    fn getpagesize() -> libc::c_int;
    fn lzma_code(strm: *mut lzma_stream, action: lzma_action) -> lzma_ret;
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_auto_decoder(
        strm: *mut lzma_stream,
        memlimit: uint64_t,
        flags: uint32_t,
    ) -> lzma_ret;
    fn inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn inflateEnd(strm: z_streamp) -> libc::c_int;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn pcre_config(_: libc::c_int, _: *mut libc::c_void) -> libc::c_int;
    fn pcre_version() -> *const libc::c_char;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
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
    fn pthread_setaffinity_np(
        __th: pthread_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn fopencookie(
        __magic_cookie: *mut libc::c_void,
        __modes: *const libc::c_char,
        __io_funcs: cookie_io_functions_t,
    ) -> *mut FILE;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn warn(__format: *const libc::c_char, _: ...);
    fn zError(_: libc::c_int) -> *const libc::c_char;
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
pub type __int32_t = libc::c_int;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ino_t = __ino_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type size_t = libc::c_ulong;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ignores {
    pub extensions: *mut *mut libc::c_char,
    pub extensions_len: size_t,
    pub names: *mut *mut libc::c_char,
    pub names_len: size_t,
    pub slash_names: *mut *mut libc::c_char,
    pub slash_names_len: size_t,
    pub regexes: *mut *mut libc::c_char,
    pub regexes_len: size_t,
    pub invert_regexes: *mut *mut libc::c_char,
    pub invert_regexes_len: size_t,
    pub slash_regexes: *mut *mut libc::c_char,
    pub slash_regexes_len: size_t,
    pub dirname: *const libc::c_char,
    pub dirname_len: size_t,
    pub abs_path: *mut libc::c_char,
    pub abs_path_len: size_t,
    pub parent: *mut ignores,
}
pub type pcre = real_pcre;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pcre_extra {
    pub flags: libc::c_ulong,
    pub study_data: *mut libc::c_void,
    pub match_limit: libc::c_ulong,
    pub callout_data: *mut libc::c_void,
    pub tables: *const libc::c_uchar,
    pub match_limit_recursion: libc::c_ulong,
    pub mark: *mut *mut libc::c_uchar,
    pub executable_jit: *mut libc::c_void,
}
pub type case_behavior = libc::c_uint;
pub const CASE_SENSITIVE_RETRY_INSENSITIVE: case_behavior = 4;
pub const CASE_SMART: case_behavior = 3;
pub const CASE_INSENSITIVE: case_behavior = 2;
pub const CASE_SENSITIVE: case_behavior = 1;
pub const CASE_DEFAULT: case_behavior = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_cli_options_661676448 {
    pub ackmate: libc::c_int,
    pub ackmate_dir_filter: *mut pcre,
    pub ackmate_dir_filter_extra: *mut pcre_extra,
    pub after: size_t,
    pub before: size_t,
    pub casing: case_behavior,
    pub file_search_string: *const libc::c_char,
    pub match_files: libc::c_int,
    pub file_search_regex: *mut pcre,
    pub file_search_regex_extra: *mut pcre_extra,
    pub color: libc::c_int,
    pub color_line_number: *mut libc::c_char,
    pub color_match: *mut libc::c_char,
    pub color_path: *mut libc::c_char,
    pub color_win_ansi: libc::c_int,
    pub column: libc::c_int,
    pub context: libc::c_int,
    pub follow_symlinks: libc::c_int,
    pub invert_match: libc::c_int,
    pub literal: libc::c_int,
    pub literal_starts_wordchar: libc::c_int,
    pub literal_ends_wordchar: libc::c_int,
    pub max_matches_per_file: size_t,
    pub max_search_depth: libc::c_int,
    pub mmap: libc::c_int,
    pub multiline: libc::c_int,
    pub one_dev: libc::c_int,
    pub only_matching: libc::c_int,
    pub path_sep: libc::c_char,
    pub path_to_ignore: libc::c_int,
    pub print_break: libc::c_int,
    pub print_count: libc::c_int,
    pub print_filename_only: libc::c_int,
    pub print_nonmatching_files: libc::c_int,
    pub print_path: libc::c_int,
    pub print_all_paths: libc::c_int,
    pub print_line_numbers: libc::c_int,
    pub print_long_lines: libc::c_int,
    pub passthrough: libc::c_int,
    pub re: *mut pcre,
    pub re_extra: *mut pcre_extra,
    pub recurse_dirs: libc::c_int,
    pub search_all_files: libc::c_int,
    pub skip_vcs_ignores: libc::c_int,
    pub search_binary_files: libc::c_int,
    pub search_zip_files: libc::c_int,
    pub search_hidden_files: libc::c_int,
    pub search_stream: libc::c_int,
    pub stats: libc::c_int,
    pub stream_line_num: size_t,
    pub match_found: libc::c_int,
    pub stdout_inode: ino_t,
    pub query: *mut libc::c_char,
    pub query_len: libc::c_int,
    pub pager: *mut libc::c_char,
    pub paths_len: libc::c_int,
    pub parallel: libc::c_int,
    pub use_thread_affinity: libc::c_int,
    pub vimgrep: libc::c_int,
    pub width: size_t,
    pub word_regexp: libc::c_int,
    pub workers: libc::c_int,
}
pub type cli_options = __anonstruct_cli_options_661676448;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_scandir_baton_t_387049375 {
    pub ig: *const ignores,
    pub base_path: *const libc::c_char,
    pub base_path_len: size_t,
    pub path_start: *const libc::c_char,
}
pub type scandir_baton_t = __anonstruct_scandir_baton_t_387049375;
pub type va_list___0 = __gnuc_va_list;
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
pub type log_level = libc::c_uint;
pub const LOG_LEVEL_NONE: log_level = 100;
pub const LOG_LEVEL_ERR: log_level = 40;
pub const LOG_LEVEL_WARN: log_level = 30;
pub const LOG_LEVEL_MSG: log_level = 20;
pub const LOG_LEVEL_DEBUG: log_level = 10;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_lang_spec_t_527861670 {
    pub name: *const libc::c_char,
    pub extensions: [*const libc::c_char; 12],
}
pub type lang_spec_t = __anonstruct_lang_spec_t_527861670;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type option_t = option;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_match_t_73278319 {
    pub start: size_t,
    pub end: size_t,
}
pub type match_t = __anonstruct_match_t_73278319;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct print_context {
    pub line: size_t,
    pub context_prev_lines: *mut *mut libc::c_char,
    pub prev_line: size_t,
    pub last_prev_line: size_t,
    pub prev_line_offset: size_t,
    pub line_preceding_current_match_offset: size_t,
    pub lines_since_last_match: size_t,
    pub last_printed_match: size_t,
    pub in_a_match: libc::c_int,
    pub printing_a_match: libc::c_int,
}
pub type DIR = __dirstream;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type off_t = __off_t;
pub type dev_t = __dev_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___wseq32_112954846 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_180959546 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_575217031 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_575217030 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_575217031,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_180959546,
    pub __annonCompField2: __anonunion____missing_field_name_575217030,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_cond_t_951761805 {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type pthread_cond_t = __anonunion_pthread_cond_t_951761805;
pub type __anonenum_ag_compression_type_847750060 = libc::c_uint;
pub const AG_XZ: __anonenum_ag_compression_type_847750060 = 4;
pub const AG_ZIP: __anonenum_ag_compression_type_847750060 = 3;
pub const AG_COMPRESS: __anonenum_ag_compression_type_847750060 = 2;
pub const AG_GZIP: __anonenum_ag_compression_type_847750060 = 1;
pub const AG_NO_COMPRESSION: __anonenum_ag_compression_type_847750060 = 0;
pub type ag_compression_type = __anonenum_ag_compression_type_847750060;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ag_stats_416475514 {
    pub total_bytes: size_t,
    pub total_files: size_t,
    pub total_matches: size_t,
    pub total_file_matches: size_t,
    pub time_start: timeval,
    pub time_end: timeval,
}
pub type ag_stats = __anonstruct_ag_stats_416475514;
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UT_hash_handle {
    pub tbl: *mut UT_hash_table,
    pub prev: *mut libc::c_void,
    pub next: *mut libc::c_void,
    pub hh_prev: *mut UT_hash_handle,
    pub hh_next: *mut UT_hash_handle,
    pub key: *mut libc::c_void,
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
pub struct work_queue_t {
    pub path: *mut libc::c_char,
    pub next: *mut work_queue_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_dirkey_t_583383409 {
    pub dev: dev_t,
    pub ino: ino_t,
}
pub type dirkey_t = __anonstruct_dirkey_t_583383409;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_symdir_t_545766974 {
    pub key: dirkey_t,
    pub hh: UT_hash_handle,
}
pub type symdir_t = __anonstruct_symdir_t_545766974;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_word_t_677777955 {
    pub as_chars: [libc::c_char; 2],
    pub as_word: uint16_t,
}
pub type word_t = __anonunion_word_t_677777955;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type __anonenum_lzma_reserved_enum_264666109 = libc::c_uint;
pub const LZMA_RESERVED_ENUM: __anonenum_lzma_reserved_enum_264666109 = 0;
pub type lzma_reserved_enum = __anonenum_lzma_reserved_enum_264666109;
pub type __anonenum_lzma_ret_557241514 = libc::c_uint;
pub const LZMA_PROG_ERROR: __anonenum_lzma_ret_557241514 = 11;
pub const LZMA_BUF_ERROR: __anonenum_lzma_ret_557241514 = 10;
pub const LZMA_DATA_ERROR: __anonenum_lzma_ret_557241514 = 9;
pub const LZMA_OPTIONS_ERROR: __anonenum_lzma_ret_557241514 = 8;
pub const LZMA_FORMAT_ERROR: __anonenum_lzma_ret_557241514 = 7;
pub const LZMA_MEMLIMIT_ERROR: __anonenum_lzma_ret_557241514 = 6;
pub const LZMA_MEM_ERROR: __anonenum_lzma_ret_557241514 = 5;
pub const LZMA_GET_CHECK: __anonenum_lzma_ret_557241514 = 4;
pub const LZMA_UNSUPPORTED_CHECK: __anonenum_lzma_ret_557241514 = 3;
pub const LZMA_NO_CHECK: __anonenum_lzma_ret_557241514 = 2;
pub const LZMA_STREAM_END: __anonenum_lzma_ret_557241514 = 1;
pub const LZMA_OK: __anonenum_lzma_ret_557241514 = 0;
pub type lzma_ret = __anonenum_lzma_ret_557241514;
pub type __anonenum_lzma_action_853458044 = libc::c_uint;
pub const LZMA_FINISH: __anonenum_lzma_action_853458044 = 3;
pub const LZMA_FULL_BARRIER: __anonenum_lzma_action_853458044 = 4;
pub const LZMA_FULL_FLUSH: __anonenum_lzma_action_853458044 = 2;
pub const LZMA_SYNC_FLUSH: __anonenum_lzma_action_853458044 = 1;
pub const LZMA_RUN: __anonenum_lzma_action_853458044 = 0;
pub type lzma_action = __anonenum_lzma_action_853458044;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_lzma_allocator_1021999983 {
    pub alloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub opaque: *mut libc::c_void,
}
pub type lzma_allocator = __anonstruct_lzma_allocator_1021999983;
pub type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_lzma_stream_354570162 {
    pub next_in: *const uint8_t,
    pub avail_in: size_t,
    pub total_in: uint64_t,
    pub next_out: *mut uint8_t,
    pub avail_out: size_t,
    pub total_out: uint64_t,
    pub allocator: *const lzma_allocator,
    pub internal: *mut lzma_internal,
    pub reserved_ptr1: *mut libc::c_void,
    pub reserved_ptr2: *mut libc::c_void,
    pub reserved_ptr3: *mut libc::c_void,
    pub reserved_ptr4: *mut libc::c_void,
    pub reserved_int1: uint64_t,
    pub reserved_int2: uint64_t,
    pub reserved_int3: size_t,
    pub reserved_int4: size_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
}
pub type lzma_stream = __anonstruct_lzma_stream_354570162;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *const Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *const libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>,
    pub zfree: Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
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
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_cpu_set_t_826868708 {
    pub __bits: [__cpu_mask; 16],
}
pub type cpu_set_t = __anonstruct_cpu_set_t_826868708;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_worker_t_846662689 {
    pub thread: pthread_t,
    pub id: libc::c_int,
}
pub type worker_t = __anonstruct_worker_t_846662689;
pub type off64_t = __off64_t;
pub type cookie_read_function_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut libc::c_char,
    size_t,
) -> __ssize_t;
pub type cookie_write_function_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_char,
    size_t,
) -> __ssize_t;
pub type cookie_seek_function_t = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut __off64_t,
    libc::c_int,
) -> libc::c_int;
pub type cookie_close_function_t = unsafe extern "C" fn(
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_cookie_io_functions_t {
    pub read: Option::<cookie_read_function_t>,
    pub write: Option::<cookie_write_function_t>,
    pub seek: Option::<cookie_seek_function_t>,
    pub close: Option::<cookie_close_function_t>,
}
pub type cookie_io_functions_t = _IO_cookie_io_functions_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_stream_340362379 {
    pub gz: z_stream,
    pub lzma: lzma_stream,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zfile {
    pub in_0: *mut FILE,
    pub logic_offset: uint64_t,
    pub decode_offset: uint64_t,
    pub actual_len: uint64_t,
    pub outbuf_start: uint32_t,
    pub ctype: ag_compression_type,
    pub stream: __anonunion_stream_340362379,
    pub inbuf: [uint8_t; 32768],
    pub outbuf: [uint8_t; 262144],
    pub eof: bool,
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
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
#[linkage = "external"]
pub unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
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
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut libc::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    let mut tmp: __ssize_t = 0;
    tmp = __getdelim(__lineptr, __n, '\n' as i32, __stream);
    return tmp;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    let mut tmp: libc::c_long = 0;
    tmp = strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    return tmp as libc::c_int;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = __xstat(1 as libc::c_int, __path, __statbuf);
    return tmp;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = __lxstat(1 as libc::c_int, __path, __statbuf);
    return tmp;
}
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = __fxstat(1 as libc::c_int, __fd, __statbuf);
    return tmp;
}
pub static mut root_ignores: *mut ignores = 0 as *const ignores as *mut ignores;
pub static mut fnmatch_flags: libc::c_int = 1 as libc::c_int;
pub static mut evil_hardcoded_ignore_files: [*const libc::c_char; 3] = [
    b".\0" as *const u8 as *const libc::c_char,
    b"..\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_void as *mut libc::c_void as *const libc::c_char,
];
pub static mut ignore_pattern_files: [*const libc::c_char; 5] = [
    b".ignore\0" as *const u8 as *const libc::c_char,
    b".gitignore\0" as *const u8 as *const libc::c_char,
    b".git/info/exclude\0" as *const u8 as *const libc::c_char,
    b".hgignore\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_void as *mut libc::c_void as *const libc::c_char,
];
pub unsafe extern "C" fn is_empty(mut ig: *mut ignores) -> libc::c_int {
    return (((*ig).extensions_len)
        .wrapping_add((*ig).names_len)
        .wrapping_add((*ig).slash_names_len)
        .wrapping_add((*ig).regexes_len)
        .wrapping_add((*ig).slash_regexes_len) == 0 as libc::c_ulong) as libc::c_int;
}
pub unsafe extern "C" fn init_ignore(
    mut parent: *mut ignores,
    mut dirname: *const libc::c_char,
    dirname_len: size_t,
) -> *mut ignores {
    let mut ig: *mut ignores = 0 as *mut ignores;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = ag_malloc(::std::mem::size_of::<ignores>() as libc::c_ulong);
    ig = tmp as *mut ignores;
    (*ig).extensions = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    (*ig).extensions_len = 0 as libc::c_int as size_t;
    (*ig).names = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    (*ig).names_len = 0 as libc::c_int as size_t;
    (*ig).slash_names = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    (*ig).slash_names_len = 0 as libc::c_int as size_t;
    (*ig).regexes = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    (*ig).regexes_len = 0 as libc::c_int as size_t;
    (*ig).invert_regexes = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    (*ig).invert_regexes_len = 0 as libc::c_int as size_t;
    (*ig).slash_regexes = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    (*ig).slash_regexes_len = 0 as libc::c_int as size_t;
    (*ig).dirname = dirname;
    (*ig).dirname_len = dirname_len;
    if !parent.is_null() {
        tmp___0 = is_empty(parent);
        if tmp___0 != 0 {
            if !((*parent).parent).is_null() {
                (*ig).parent = (*parent).parent;
            } else {
                (*ig).parent = parent;
            }
        } else {
            (*ig).parent = parent;
        }
    } else {
        (*ig).parent = parent;
    }
    let mut current_block_46: u64;
    if !parent.is_null() {
        if (*parent).abs_path_len > 0 as libc::c_ulong {
            ag_asprintf(
                &mut (*ig).abs_path as *mut *mut libc::c_char,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                (*parent).abs_path,
                dirname,
            );
            (*ig)
                .abs_path_len = ((*parent).abs_path_len)
                .wrapping_add(1 as libc::c_ulong)
                .wrapping_add(dirname_len);
            current_block_46 = 7659304154607701039;
        } else {
            current_block_46 = 6993761515533589156;
        }
    } else {
        current_block_46 = 6993761515533589156;
    }
    match current_block_46 {
        6993761515533589156 => {
            if dirname_len == 1 as libc::c_ulong {
                if *dirname.offset(0 as libc::c_int as isize) as libc::c_int
                    == 46 as libc::c_int
                {
                    tmp___1 = ag_malloc(
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    );
                    (*ig).abs_path = tmp___1 as *mut libc::c_char;
                    *((*ig).abs_path)
                        .offset(
                            0 as libc::c_int as isize,
                        ) = '\u{0}' as i32 as libc::c_char;
                    (*ig).abs_path_len = 0 as libc::c_int as size_t;
                } else {
                    ag_asprintf(
                        &mut (*ig).abs_path as *mut *mut libc::c_char,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        dirname,
                    );
                    (*ig).abs_path_len = dirname_len;
                }
            } else {
                ag_asprintf(
                    &mut (*ig).abs_path as *mut *mut libc::c_char,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    dirname,
                );
                (*ig).abs_path_len = dirname_len;
            }
        }
        _ => {}
    }
    return ig;
}
pub unsafe extern "C" fn cleanup_ignore(mut ig: *mut ignores) {
    if ig as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    free_strings((*ig).extensions, (*ig).extensions_len);
    free_strings((*ig).names, (*ig).names_len);
    free_strings((*ig).slash_names, (*ig).slash_names_len);
    free_strings((*ig).regexes, (*ig).regexes_len);
    free_strings((*ig).invert_regexes, (*ig).invert_regexes_len);
    free_strings((*ig).slash_regexes, (*ig).slash_regexes_len);
    if !((*ig).abs_path).is_null() {
        free((*ig).abs_path as *mut libc::c_void);
    }
    free(ig as *mut libc::c_void);
}
pub unsafe extern "C" fn add_ignore_pattern(
    mut ig: *mut ignores,
    mut pattern: *const libc::c_char,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut pattern_len: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut patterns_p: *mut *mut *mut libc::c_char = 0 as *mut *mut *mut libc::c_char;
    let mut patterns_len: *mut size_t = 0 as *mut size_t;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut patterns: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: *const libc::c_char = 0 as *const libc::c_char;
    tmp = strncmp(
        pattern,
        b"./\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
    if tmp == 0 as libc::c_int {
        pattern = pattern.offset(1);
    }
    tmp___0 = strlen(pattern);
    pattern_len = tmp___0 as libc::c_int;
    while pattern_len > 0 as libc::c_int {
        tmp___1 = __ctype_b_loc();
        if *(*tmp___1)
            .offset(
                *pattern.offset((pattern_len - 1 as libc::c_int) as isize) as libc::c_int
                    as isize,
            ) as libc::c_int & 8192 as libc::c_int == 0
        {
            break;
        }
        pattern_len -= 1;
    }
    if pattern_len == 0 as libc::c_int {
        log_debug(
            b"Pattern is empty. Not adding any ignores.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    tmp___4 = is_fnmatch(pattern);
    if tmp___4 != 0 {
        if *pattern.offset(0 as libc::c_int as isize) as libc::c_int == 42 as libc::c_int
        {
            if *pattern.offset(1 as libc::c_int as isize) as libc::c_int
                == 46 as libc::c_int
            {
                tmp___2 = strchr(pattern.offset(2 as libc::c_int as isize), '.' as i32);
                if !tmp___2.is_null() {
                    tmp___3 = is_fnmatch(pattern.offset(2 as libc::c_int as isize));
                    if tmp___3 != 0 {
                        current_block = 7239556964324375340;
                    } else {
                        patterns_p = &mut (*ig).extensions;
                        patterns_len = &mut (*ig).extensions_len;
                        pattern = pattern.offset(2 as libc::c_int as isize);
                        pattern_len -= 2 as libc::c_int;
                        current_block = 1924505913685386279;
                    }
                } else {
                    current_block = 7239556964324375340;
                }
            } else {
                current_block = 7239556964324375340;
            }
        } else {
            current_block = 7239556964324375340;
        }
        match current_block {
            1924505913685386279 => {}
            _ => {
                if *pattern.offset(0 as libc::c_int as isize) as libc::c_int
                    == 47 as libc::c_int
                {
                    patterns_p = &mut (*ig).slash_regexes;
                    patterns_len = &mut (*ig).slash_regexes_len;
                    pattern = pattern.offset(1);
                    pattern_len -= 1;
                } else if *pattern.offset(0 as libc::c_int as isize) as libc::c_int
                        == 33 as libc::c_int
                    {
                    patterns_p = &mut (*ig).invert_regexes;
                    patterns_len = &mut (*ig).invert_regexes_len;
                    pattern = pattern.offset(1);
                    pattern_len -= 1;
                } else {
                    patterns_p = &mut (*ig).regexes;
                    patterns_len = &mut (*ig).regexes_len;
                }
            }
        }
    } else if *pattern.offset(0 as libc::c_int as isize) as libc::c_int
            == 47 as libc::c_int
        {
        patterns_p = &mut (*ig).slash_names;
        patterns_len = &mut (*ig).slash_names_len;
        pattern = pattern.offset(1);
        pattern_len -= 1;
    } else {
        patterns_p = &mut (*ig).names;
        patterns_len = &mut (*ig).names_len;
    }
    *patterns_len = (*patterns_len).wrapping_add(1);
    tmp___5 = ag_realloc(
        *patterns_p as *mut libc::c_void,
        (*patterns_len)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    );
    patterns = tmp___5 as *mut *mut libc::c_char;
    *patterns_p = patterns;
    i = (*patterns_len).wrapping_sub(1 as libc::c_ulong) as libc::c_int;
    while i > 0 as libc::c_int {
        tmp___6 = strcmp(
            pattern,
            *patterns.offset((i - 1 as libc::c_int) as isize) as *const libc::c_char,
        );
        if tmp___6 > 0 as libc::c_int {
            break;
        }
        let ref mut fresh0 = *patterns.offset(i as isize);
        *fresh0 = *patterns.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
    }
    let ref mut fresh1 = *patterns.offset(i as isize);
    *fresh1 = ag_strndup(pattern, pattern_len as size_t);
    if ig as libc::c_ulong == root_ignores as libc::c_ulong {
        tmp___7 = b"root ignores\0" as *const u8 as *const libc::c_char;
    } else {
        tmp___7 = (*ig).abs_path as *const libc::c_char;
    }
    log_debug(
        b"added ignore pattern %s to %s\0" as *const u8 as *const libc::c_char,
        pattern,
        tmp___7,
    );
}
pub unsafe extern "C" fn load_ignore_patterns(
    mut ig: *mut ignores,
    mut path: *const libc::c_char,
) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line_len: ssize_t = 0;
    let mut line_cap: size_t = 0;
    fp = 0 as *mut libc::c_void as *mut FILE;
    fp = fopen(path, b"r\0" as *const u8 as *const libc::c_char);
    if fp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        log_debug(
            b"Skipping ignore file %s: not readable\0" as *const u8
                as *const libc::c_char,
            path,
        );
        return;
    }
    log_debug(b"Loading ignore file %s.\0" as *const u8 as *const libc::c_char, path);
    line = 0 as *mut libc::c_void as *mut libc::c_char;
    line_len = 0 as libc::c_int as ssize_t;
    line_cap = 0 as libc::c_int as size_t;
    loop {
        line_len = getline(
            &mut line as *mut *mut libc::c_char,
            &mut line_cap as *mut size_t,
            fp,
        );
        if !(line_len > 0 as libc::c_long) {
            break;
        }
        if line_len == 0 as libc::c_long {
            continue;
        }
        if *line.offset(0 as libc::c_int as isize) as libc::c_int == 10 as libc::c_int {
            continue;
        }
        if *line.offset(0 as libc::c_int as isize) as libc::c_int == 35 as libc::c_int {
            continue;
        }
        if *line.offset((line_len - 1 as libc::c_long) as isize) as libc::c_int
            == 10 as libc::c_int
        {
            *line
                .offset(
                    (line_len - 1 as libc::c_long) as isize,
                ) = '\u{0}' as i32 as libc::c_char;
        }
        add_ignore_pattern(ig, line as *const libc::c_char);
    }
    free(line as *mut libc::c_void);
    fclose(fp);
}
unsafe extern "C" fn ackmate_dir_match(
    mut dir_name: *const libc::c_char,
) -> libc::c_int {
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    if opts.ackmate_dir_filter as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    tmp = strlen(dir_name);
    tmp___0 = pcre_exec(
        opts.ackmate_dir_filter as *const pcre,
        0 as *mut libc::c_void as *const pcre_extra,
        dir_name,
        tmp as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as *mut libc::c_void as *mut libc::c_int,
        0 as libc::c_int,
    );
    return tmp___0;
}
unsafe extern "C" fn path_ignore_search(
    mut ig: *const ignores,
    mut path: *const libc::c_char,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_start_pos: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut match_pos: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut slash_filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut rv: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    match_pos = binary_search(
        filename,
        (*ig).names,
        0 as libc::c_int,
        (*ig).names_len as libc::c_int,
    );
    if match_pos >= 0 as libc::c_int {
        log_debug(
            b"file %s ignored because name matches static pattern %s\0" as *const u8
                as *const libc::c_char,
            filename,
            *((*ig).names).offset(match_pos as isize),
        );
        return 1 as libc::c_int;
    }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == 46 as libc::c_int {
        tmp = path.offset(1 as libc::c_int as isize);
    } else {
        tmp = path;
    }
    ag_asprintf(
        &mut temp as *mut *mut libc::c_char,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        tmp,
        filename,
    );
    if *temp.offset(0 as libc::c_int as isize) as libc::c_int == 47 as libc::c_int {
        temp_start_pos = 1 as libc::c_int;
    } else {
        temp_start_pos = 0 as libc::c_int;
    }
    tmp___3 = strncmp(
        temp.offset(temp_start_pos as isize) as *const libc::c_char,
        (*ig).abs_path as *const libc::c_char,
        (*ig).abs_path_len,
    );
    if tmp___3 == 0 as libc::c_int {
        slash_filename = temp
            .offset(temp_start_pos as isize)
            .offset((*ig).abs_path_len as isize);
        if *slash_filename.offset(0 as libc::c_int as isize) as libc::c_int
            == 47 as libc::c_int
        {
            slash_filename = slash_filename.offset(1);
        }
        match_pos = binary_search(
            slash_filename as *const libc::c_char,
            (*ig).names,
            0 as libc::c_int,
            (*ig).names_len as libc::c_int,
        );
        if match_pos >= 0 as libc::c_int {
            log_debug(
                b"file %s ignored because name matches static pattern %s\0" as *const u8
                    as *const libc::c_char,
                temp,
                *((*ig).names).offset(match_pos as isize),
            );
            free(temp as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        match_pos = binary_search(
            slash_filename as *const libc::c_char,
            (*ig).slash_names,
            0 as libc::c_int,
            (*ig).slash_names_len as libc::c_int,
        );
        if match_pos >= 0 as libc::c_int {
            log_debug(
                b"file %s ignored because name matches slash static pattern %s\0"
                    as *const u8 as *const libc::c_char,
                slash_filename,
                *((*ig).slash_names).offset(match_pos as isize),
            );
            free(temp as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        i = 0 as libc::c_int as size_t;
        while i < (*ig).names_len {
            tmp___0 = strstr(
                slash_filename as *const libc::c_char,
                *((*ig).names).offset(i as isize) as *const libc::c_char,
            );
            pos = tmp___0;
            let mut current_block_47: u64;
            if pos as libc::c_ulong == slash_filename as libc::c_ulong {
                current_block_47 = 2506215770721818729;
            } else if !pos.is_null() {
                if *pos.offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == 47 as libc::c_int
                {
                    current_block_47 = 2506215770721818729;
                } else {
                    current_block_47 = 1622411330066726685;
                }
            } else {
                current_block_47 = 1622411330066726685;
            }
            match current_block_47 {
                2506215770721818729 => {
                    tmp___1 = strlen(
                        *((*ig).names).offset(i as isize) as *const libc::c_char,
                    );
                    pos = pos.offset(tmp___1 as isize);
                    if *pos as libc::c_int == 0 as libc::c_int {
                        log_debug(
                            b"file %s ignored because path somewhere matches name %s\0"
                                as *const u8 as *const libc::c_char,
                            slash_filename,
                            *((*ig).names).offset(i as isize),
                        );
                        free(temp as *mut libc::c_void);
                        return 1 as libc::c_int;
                    } else {
                        if *pos as libc::c_int == 47 as libc::c_int {
                            log_debug(
                                b"file %s ignored because path somewhere matches name %s\0"
                                    as *const u8 as *const libc::c_char,
                                slash_filename,
                                *((*ig).names).offset(i as isize),
                            );
                            free(temp as *mut libc::c_void);
                            return 1 as libc::c_int;
                        }
                    }
                }
                _ => {}
            }
            log_debug(
                b"pattern %s doesn't match path %s\0" as *const u8
                    as *const libc::c_char,
                *((*ig).names).offset(i as isize),
                slash_filename,
            );
            i = i.wrapping_add(1);
        }
        i = 0 as libc::c_int as size_t;
        while i < (*ig).slash_regexes_len {
            tmp___2 = fnmatch(
                *((*ig).slash_regexes).offset(i as isize) as *const libc::c_char,
                slash_filename as *const libc::c_char,
                fnmatch_flags,
            );
            if tmp___2 == 0 as libc::c_int {
                log_debug(
                    b"file %s ignored because name matches slash regex pattern %s\0"
                        as *const u8 as *const libc::c_char,
                    slash_filename,
                    *((*ig).slash_regexes).offset(i as isize),
                );
                free(temp as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            log_debug(
                b"pattern %s doesn't match slash file %s\0" as *const u8
                    as *const libc::c_char,
                *((*ig).slash_regexes).offset(i as isize),
                slash_filename,
            );
            i = i.wrapping_add(1);
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < (*ig).invert_regexes_len {
        tmp___4 = fnmatch(
            *((*ig).invert_regexes).offset(i as isize) as *const libc::c_char,
            filename,
            fnmatch_flags,
        );
        if tmp___4 == 0 as libc::c_int {
            log_debug(
                b"file %s not ignored because name matches regex pattern !%s\0"
                    as *const u8 as *const libc::c_char,
                filename,
                *((*ig).invert_regexes).offset(i as isize),
            );
            free(temp as *mut libc::c_void);
            return 0 as libc::c_int;
        }
        log_debug(
            b"pattern !%s doesn't match file %s\0" as *const u8 as *const libc::c_char,
            *((*ig).invert_regexes).offset(i as isize),
            filename,
        );
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < (*ig).regexes_len {
        tmp___5 = fnmatch(
            *((*ig).regexes).offset(i as isize) as *const libc::c_char,
            filename,
            fnmatch_flags,
        );
        if tmp___5 == 0 as libc::c_int {
            log_debug(
                b"file %s ignored because name matches regex pattern %s\0" as *const u8
                    as *const libc::c_char,
                filename,
                *((*ig).regexes).offset(i as isize),
            );
            free(temp as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        log_debug(
            b"pattern %s doesn't match file %s\0" as *const u8 as *const libc::c_char,
            *((*ig).regexes).offset(i as isize),
            filename,
        );
        i = i.wrapping_add(1);
    }
    tmp___6 = ackmate_dir_match(temp as *const libc::c_char);
    rv = tmp___6;
    free(temp as *mut libc::c_void);
    return rv;
}
pub unsafe extern "C" fn filename_filter(
    mut path: *const libc::c_char,
    mut dir: *const dirent,
    mut baton: *mut libc::c_void,
) -> libc::c_int {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut scandir_baton: *mut scandir_baton_t = 0 as *mut scandir_baton_t;
    let mut path_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut extension: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filename_len: size_t = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut ig: *const ignores = 0 as *const ignores;
    let mut match_pos: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rv: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    filename = ((*dir).d_name).as_ptr();
    if opts.search_hidden_files == 0 {
        if *filename.offset(0 as libc::c_int as isize) as libc::c_int
            == 46 as libc::c_int
        {
            return 0 as libc::c_int;
        }
    }
    i = 0 as libc::c_int as size_t;
    while evil_hardcoded_ignore_files[i as usize] as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        tmp = strcmp(filename, evil_hardcoded_ignore_files[i as usize]);
        if tmp == 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    if opts.follow_symlinks == 0 {
        tmp___0 = is_symlink(path, dir);
        if tmp___0 != 0 {
            log_debug(
                b"File %s ignored becaused it's a symlink\0" as *const u8
                    as *const libc::c_char,
                ((*dir).d_name).as_ptr(),
            );
            return 0 as libc::c_int;
        }
    }
    tmp___1 = is_named_pipe(path, dir);
    if tmp___1 != 0 {
        log_debug(
            b"%s ignored because it's a named pipe or socket\0" as *const u8
                as *const libc::c_char,
            path,
        );
        return 0 as libc::c_int;
    }
    if opts.search_all_files != 0 {
        if opts.path_to_ignore == 0 {
            return 1 as libc::c_int;
        }
    }
    scandir_baton = baton as *mut scandir_baton_t;
    path_start = (*scandir_baton).path_start;
    tmp___2 = strchr(filename, '.' as i32);
    extension = tmp___2 as *const libc::c_char;
    if !extension.is_null() {
        if *extension.offset(1 as libc::c_int as isize) != 0 {
            extension = extension.offset(1);
        } else {
            extension = 0 as *mut libc::c_void as *const libc::c_char;
        }
    }
    filename_len = 0 as libc::c_int as size_t;
    tmp___3 = strncmp(
        filename,
        b"./\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
    if tmp___3 == 0 as libc::c_int {
        filename_len = strlen(filename);
        filename = filename.offset(1);
        filename_len = filename_len.wrapping_sub(1);
    }
    ig = (*scandir_baton).ig;
    while ig as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if !extension.is_null() {
            tmp___4 = binary_search(
                extension,
                (*ig).extensions,
                0 as libc::c_int,
                (*ig).extensions_len as libc::c_int,
            );
            match_pos = tmp___4;
            if match_pos >= 0 as libc::c_int {
                log_debug(
                    b"file %s ignored because name matches extension %s\0" as *const u8
                        as *const libc::c_char,
                    filename,
                    *((*ig).extensions).offset(match_pos as isize),
                );
                return 0 as libc::c_int;
            }
        }
        tmp___5 = path_ignore_search(ig, path_start, filename);
        if tmp___5 != 0 {
            return 0 as libc::c_int;
        }
        tmp___7 = is_directory(path, dir);
        if tmp___7 != 0 {
            if filename_len == 0 {
                filename_len = strlen(filename);
            }
            if *filename.offset(filename_len.wrapping_sub(1 as libc::c_ulong) as isize)
                as libc::c_int != 47 as libc::c_int
            {
                ag_asprintf(
                    &mut temp as *mut *mut libc::c_char,
                    b"%s/\0" as *const u8 as *const libc::c_char,
                    filename,
                );
                tmp___6 = path_ignore_search(
                    ig,
                    path_start,
                    temp as *const libc::c_char,
                );
                rv = tmp___6;
                free(temp as *mut libc::c_void);
                if rv != 0 {
                    return 0 as libc::c_int;
                }
            }
        }
        ig = (*ig).parent as *const ignores;
    }
    log_debug(b"%s not ignored\0" as *const u8 as *const libc::c_char, filename);
    return 1 as libc::c_int;
}
pub static mut print_mtx: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut log_threshold: log_level = LOG_LEVEL_ERR;
pub unsafe extern "C" fn set_log_level(mut threshold: log_level) {
    log_threshold = threshold;
}
pub unsafe extern "C" fn log_debug(mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vplog(10 as libc::c_int as libc::c_uint, fmt, args_0.as_va_list());
}
pub unsafe extern "C" fn log_msg(mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vplog(20 as libc::c_int as libc::c_uint, fmt, args_0.as_va_list());
}
pub unsafe extern "C" fn log_warn(mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vplog(30 as libc::c_int as libc::c_uint, fmt, args_0.as_va_list());
}
pub unsafe extern "C" fn log_err(mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vplog(40 as libc::c_int as libc::c_uint, fmt, args_0.as_va_list());
}
pub unsafe extern "C" fn vplog(
    level: libc::c_uint,
    mut fmt: *const libc::c_char,
    mut args: ::std::ffi::VaList,
) {
    let mut stream: *mut FILE = 0 as *mut FILE;
    if level < log_threshold as libc::c_uint {
        return;
    }
    pthread_mutex_lock(&mut print_mtx);
    stream = out_fd;
    match level {
        10 => {
            fprintf(stream, b"DEBUG: \0" as *const u8 as *const libc::c_char);
        }
        20 => {
            fprintf(stream, b"MSG: \0" as *const u8 as *const libc::c_char);
        }
        30 => {
            fprintf(stream, b"WARN: \0" as *const u8 as *const libc::c_char);
        }
        40 => {
            stream = stderr;
            fprintf(stream, b"ERR: \0" as *const u8 as *const libc::c_char);
        }
        _ => {}
    }
    vfprintf(stream, fmt, args.as_va_list());
    fprintf(stream, b"\n\0" as *const u8 as *const libc::c_char);
    pthread_mutex_unlock(&mut print_mtx);
}
pub unsafe extern "C" fn plog(
    level: libc::c_uint,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vplog(level, fmt, args_0.as_va_list());
}
pub static mut opts: cli_options = cli_options {
    ackmate: 0,
    ackmate_dir_filter: 0 as *const pcre as *mut pcre,
    ackmate_dir_filter_extra: 0 as *const pcre_extra as *mut pcre_extra,
    after: 0,
    before: 0,
    casing: CASE_DEFAULT,
    file_search_string: 0 as *const libc::c_char,
    match_files: 0,
    file_search_regex: 0 as *const pcre as *mut pcre,
    file_search_regex_extra: 0 as *const pcre_extra as *mut pcre_extra,
    color: 0,
    color_line_number: 0 as *const libc::c_char as *mut libc::c_char,
    color_match: 0 as *const libc::c_char as *mut libc::c_char,
    color_path: 0 as *const libc::c_char as *mut libc::c_char,
    color_win_ansi: 0,
    column: 0,
    context: 0,
    follow_symlinks: 0,
    invert_match: 0,
    literal: 0,
    literal_starts_wordchar: 0,
    literal_ends_wordchar: 0,
    max_matches_per_file: 0,
    max_search_depth: 0,
    mmap: 0,
    multiline: 0,
    one_dev: 0,
    only_matching: 0,
    path_sep: 0,
    path_to_ignore: 0,
    print_break: 0,
    print_count: 0,
    print_filename_only: 0,
    print_nonmatching_files: 0,
    print_path: 0,
    print_all_paths: 0,
    print_line_numbers: 0,
    print_long_lines: 0,
    passthrough: 0,
    re: 0 as *const pcre as *mut pcre,
    re_extra: 0 as *const pcre_extra as *mut pcre_extra,
    recurse_dirs: 0,
    search_all_files: 0,
    skip_vcs_ignores: 0,
    search_binary_files: 0,
    search_zip_files: 0,
    search_hidden_files: 0,
    search_stream: 0,
    stats: 0,
    stream_line_num: 0,
    match_found: 0,
    stdout_inode: 0,
    query: 0 as *const libc::c_char as *mut libc::c_char,
    query_len: 0,
    pager: 0 as *const libc::c_char as *mut libc::c_char,
    paths_len: 0,
    parallel: 0,
    use_thread_affinity: 0,
    vimgrep: 0,
    width: 0,
    word_regexp: 0,
    workers: 0,
};
pub static mut color_line_number: *const libc::c_char = b"\x1B[1;33m\0" as *const u8
    as *const libc::c_char;
pub static mut color_match: *const libc::c_char = b"\x1B[30;43m\0" as *const u8
    as *const libc::c_char;
pub static mut color_path: *const libc::c_char = b"\x1B[1;32m\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn usage() {
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Usage: ag [FILE-TYPE] [OPTIONS] PATTERN [PATH]\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"  Recursively search for PATTERN in PATH.\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(b"  Like grep or ack, but faster.\n\n\0" as *const u8 as *const libc::c_char);
    printf(b"Example:\n  ag -i foo /bar/\n\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Output Options:\n     --ackmate            Print results in AckMate-parseable format\n  -A --after [LINES]      Print lines after match (Default: 2)\n  -B --before [LINES]     Print lines before match (Default: 2)\n     --[no]break          Print newlines between matches in different files\n                          (Enabled by default)\n  -c --count              Only print the number of matches in each file.\n                          (This often differs from the number of matching lines)\n     --[no]color          Print color codes in results (Enabled by default)\n     --color-line-number  Color codes for line numbers (Default: 1;33)\n     --color-match        Color codes for result match numbers (Default: 30;43)\n     --color-path         Color codes for path names (Default: 1;32)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"     --column             Print column numbers in results\n     --[no]filename       Print file names (Enabled unless searching a single file)\n  -H --[no]heading        Print file names before each file's matches\n                          (Enabled by default)\n  -C --context [LINES]    Print lines before and after matches (Default: 2)\n     --[no]group          Same as --[no]break --[no]heading\n  -g --filename-pattern PATTERN\n                          Print filenames matching PATTERN\n  -l --files-with-matches Only print filenames that contain matches\n                          (don't print the matching lines)\n  -L --files-without-matches\n                          Only print filenames that don't contain matches\n     --print-all-files    Print headings for all files searched, even those that\n                          don't contain matches\n     --[no]numbers        Print line numbers. Default is to omit line numbers\n                          when searching streams\n  -o --only-matching      Prints only the matching part of the lines\n     --print-long-lines   Print matches on very long lines (Default: >2k characters)\n     --passthrough        When searching a stream, print all lines even if they\n                          don't match\n     --silent             Suppress all log messages, including errors\n     --stats              Print stats (files scanned, time taken, etc.)\n     --stats-only         Print stats and nothing else.\n                          (Same as --count when searching a single file)\n     --vimgrep            Print results like vim's :vimgrep /pattern/g would\n                          (it reports every match on the line)\n  -0 --null --print0      Separate filenames with null (for 'xargs -0')\n\nSearch Options:\n  -a --all-types          Search all files (doesn't include hidden files\n                          or patterns from ignore files)\n  -D --debug              Ridiculous debugging (probably not useful)\n     --depth NUM          Search up to NUM directories deep (Default: 25)\n  -f --follow             Follow symlinks\n  -F --fixed-strings      Alias for --literal for compatibility with grep\n  -G --file-search-regex  PATTERN Limit search to filenames matching PATTERN\n     --hidden             Search hidden files (obeys .*ignore files)\n  -i --ignore-case        Match case insensitively\n     --ignore PATTERN     Ignore files/directories matching PATTERN\n                          (literal file/directory names also allowed)\n     --ignore-dir NAME    Alias for --ignore for compatibility with ack.\n  -m --max-count NUM      Skip the rest of a file after NUM matches (Default: 10,000)\n     --one-device         Don't follow links to other devices.\n  -p --path-to-ignore STRING\n                          Use .ignore file at STRING\n  -Q --literal            Don't parse PATTERN as a regular expression\n  -s --case-sensitive     Match case sensitively\n  -S --smart-case         Match case insensitively unless PATTERN contains\n                          uppercase characters (Enabled by default)\n     --search-binary      Search binary files for matches\n  -t --all-text           Search all text files (doesn't include hidden files)\n  -u --unrestricted       Search all files (ignore .ignore, .gitignore, etc.;\n                          searches binary and hidden files as well)\n  -U --skip-vcs-ignores   Ignore VCS ignore files\n                          (.gitignore, .hgignore; still obey .ignore)\n  -v --invert-match\n  -w --word-regexp        Only match whole words\n  -W --width NUM          Truncate match lines after NUM characters\n  -z --search-zip         Search contents of compressed (e.g., gzip) files\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"File Types:\nThe search can be restricted to certain types of files. Example:\n  ag --html needle\n  - Searches for 'needle' in files with suffix .htm, .html, .shtml or .xhtml.\n\nFor a list of supported file types run:\n  ag --list-file-types\n\nag was originally created by Geoff Greer. More information (and the latest release)\ncan be found at http://geoff.greer.fm/ag\n\0"
            as *const u8 as *const libc::c_char,
    );
}
pub unsafe extern "C" fn print_version() {
    let mut jit: libc::c_char = 0;
    let mut lzma: libc::c_char = 0;
    let mut zlib: libc::c_char = 0;
    jit = '-' as i32 as libc::c_char;
    lzma = '-' as i32 as libc::c_char;
    zlib = '-' as i32 as libc::c_char;
    jit = '+' as i32 as libc::c_char;
    lzma = '+' as i32 as libc::c_char;
    zlib = '+' as i32 as libc::c_char;
    printf(
        b"ag version %s\n\n\0" as *const u8 as *const libc::c_char,
        b"2.2.0\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Features:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"  %cjit %clzma %czlib\n\0" as *const u8 as *const libc::c_char,
        jit as libc::c_int,
        lzma as libc::c_int,
        zlib as libc::c_int,
    );
}
pub unsafe extern "C" fn init_options() {
    let mut term: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    tmp = getenv(b"TERM\0" as *const u8 as *const libc::c_char);
    term = tmp;
    memset(
        &mut opts as *mut cli_options as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cli_options>() as libc::c_ulong,
    );
    opts.casing = CASE_DEFAULT;
    opts.color = 1 as libc::c_int;
    if !term.is_null() {
        tmp___0 = strcmp(
            term as *const libc::c_char,
            b"dumb\0" as *const u8 as *const libc::c_char,
        );
        if tmp___0 == 0 {
            opts.color = 0 as libc::c_int;
        }
    }
    opts.color_win_ansi = 0 as libc::c_int;
    opts.max_matches_per_file = 0 as libc::c_int as size_t;
    opts.max_search_depth = 25 as libc::c_int;
    opts.mmap = 1 as libc::c_int;
    opts.multiline = 1 as libc::c_int;
    opts.width = 0 as libc::c_int as size_t;
    opts.path_sep = '\n' as i32 as libc::c_char;
    opts.print_break = 1 as libc::c_int;
    opts.print_path = 0 as libc::c_int;
    opts.print_all_paths = 0 as libc::c_int;
    opts.print_line_numbers = 1 as libc::c_int;
    opts.recurse_dirs = 1 as libc::c_int;
    opts.color_path = ag_strdup(color_path);
    opts.color_match = ag_strdup(color_match);
    opts.color_line_number = ag_strdup(color_line_number);
    opts.use_thread_affinity = 1 as libc::c_int;
}
pub unsafe extern "C" fn cleanup_options() {
    free(opts.color_path as *mut libc::c_void);
    free(opts.color_match as *mut libc::c_void);
    free(opts.color_line_number as *mut libc::c_void);
    if !(opts.query).is_null() {
        free(opts.query as *mut libc::c_void);
    }
    (Some(pcre_free.expect("non-null function pointer")))
        .expect("non-null function pointer")(opts.re as *mut libc::c_void);
    if !(opts.re_extra).is_null() {
        (Some(pcre_free.expect("non-null function pointer")))
            .expect("non-null function pointer")(opts.re_extra as *mut libc::c_void);
    }
    if !(opts.ackmate_dir_filter).is_null() {
        (Some(pcre_free.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(opts.ackmate_dir_filter as *mut libc::c_void);
    }
    if !(opts.ackmate_dir_filter_extra).is_null() {
        (Some(pcre_free.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(opts.ackmate_dir_filter_extra as *mut libc::c_void);
    }
    if !(opts.file_search_regex).is_null() {
        (Some(pcre_free.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(opts.file_search_regex as *mut libc::c_void);
    }
    if !(opts.file_search_regex_extra).is_null() {
        (Some(pcre_free.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(opts.file_search_regex_extra as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn parse_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut base_paths: *mut *mut *mut libc::c_char,
    mut paths: *mut *mut *mut libc::c_char,
) {
    let mut ch: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut path_len: libc::c_int = 0;
    let mut base_path_len: libc::c_int = 0;
    let mut useless: libc::c_int = 0;
    let mut group: libc::c_int = 0;
    let mut help: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut list_file_types: libc::c_int = 0;
    let mut opt_index: libc::c_int = 0;
    let mut num_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut home_dir: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ignore_file_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut accepts_query: libc::c_int = 0;
    let mut needs_query: libc::c_int = 0;
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut rv: libc::c_int = 0;
    let mut lang_count: size_t = 0;
    let mut lang_num: size_t = 0;
    let mut has_filetype: libc::c_int = 0;
    let mut longopts_len: size_t = 0;
    let mut full_len: size_t = 0;
    let mut longopts: *mut option_t = 0 as *mut option_t;
    let mut lang_regex: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ext_index: *mut size_t = 0 as *mut size_t;
    let mut extensions: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut num_exts: size_t = 0;
    let mut base_longopts: [option_t; 92] = [option_t {
        name: 0 as *const libc::c_char,
        has_arg: 0,
        flag: 0 as *mut libc::c_int,
        val: 0,
    }; 92];
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut opt: option_t = option_t {
        name: 0 as *const libc::c_char,
        has_arg: 0,
        flag: 0 as *mut libc::c_int,
        val: 0,
    };
    let mut __constr_expr_0: option_t = option_t {
        name: 0 as *const libc::c_char,
        has_arg: 0,
        flag: 0 as *mut libc::c_int,
        val: 0,
    };
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut file_search_regex: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: libc::c_long = 0;
    let mut tmp___7: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___8: libc::c_long = 0;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___10: libc::c_long = 0;
    let mut tmp___11: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_long = 0;
    let mut tmp___14: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___20: libc::c_int = 0;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: libc::c_int = 0;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: libc::c_int = 0;
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: libc::c_int = 0;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: size_t = 0;
    let mut tmp___33: libc::c_int = 0;
    let mut pcre_opts: libc::c_int = 0;
    let mut tmp___34: libc::c_int = 0;
    let mut old_file_search_regex: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lang_index: size_t = 0;
    let mut j: libc::c_int = 0;
    let mut gitconfig_file: *mut FILE = 0 as *mut FILE;
    let mut buf_len: size_t = 0;
    let mut gitconfig_res: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___35: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___36: size_t = 0;
    let mut tmp___37: libc::c_int = 0;
    let mut config_home: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___38: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___39: size_t = 0;
    let mut tmp___40: size_t = 0;
    let mut tmp___41: libc::c_int = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___42: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___43: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___44: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___45: size_t = 0;
    let mut tmp___46: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___47: size_t = 0;
    let mut tmp___48: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___49: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___50: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___51: *mut libc::c_void = 0 as *mut libc::c_void;
    path_len = 0 as libc::c_int;
    base_path_len = 0 as libc::c_int;
    useless = 0 as libc::c_int;
    group = 1 as libc::c_int;
    help = 0 as libc::c_int;
    version = 0 as libc::c_int;
    list_file_types = 0 as libc::c_int;
    opt_index = 0 as libc::c_int;
    tmp = getenv(b"HOME\0" as *const u8 as *const libc::c_char);
    home_dir = tmp as *const libc::c_char;
    ignore_file_path = 0 as *mut libc::c_void as *mut libc::c_char;
    accepts_query = 1 as libc::c_int;
    needs_query = 1 as libc::c_int;
    lang_num = 0 as libc::c_int as size_t;
    has_filetype = 0 as libc::c_int;
    lang_regex = 0 as *mut libc::c_void as *mut libc::c_char;
    ext_index = 0 as *mut libc::c_void as *mut size_t;
    extensions = 0 as *mut libc::c_void as *mut libc::c_char;
    num_exts = 0 as libc::c_int as size_t;
    init_options();
    base_longopts[0 as libc::c_int as usize]
        .name = b"ackmate\0" as *const u8 as *const libc::c_char;
    base_longopts[0 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[0 as libc::c_int as usize].flag = &mut opts.ackmate;
    base_longopts[0 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[1 as libc::c_int as usize]
        .name = b"ackmate-dir-filter\0" as *const u8 as *const libc::c_char;
    base_longopts[1 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[1 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[1 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[2 as libc::c_int as usize]
        .name = b"affinity\0" as *const u8 as *const libc::c_char;
    base_longopts[2 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[2 as libc::c_int as usize].flag = &mut opts.use_thread_affinity;
    base_longopts[2 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[3 as libc::c_int as usize]
        .name = b"after\0" as *const u8 as *const libc::c_char;
    base_longopts[3 as libc::c_int as usize].has_arg = 2 as libc::c_int;
    base_longopts[3 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[3 as libc::c_int as usize].val = 'A' as i32;
    base_longopts[4 as libc::c_int as usize]
        .name = b"all-text\0" as *const u8 as *const libc::c_char;
    base_longopts[4 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[4 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[4 as libc::c_int as usize].val = 't' as i32;
    base_longopts[5 as libc::c_int as usize]
        .name = b"all-types\0" as *const u8 as *const libc::c_char;
    base_longopts[5 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[5 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[5 as libc::c_int as usize].val = 'a' as i32;
    base_longopts[6 as libc::c_int as usize]
        .name = b"before\0" as *const u8 as *const libc::c_char;
    base_longopts[6 as libc::c_int as usize].has_arg = 2 as libc::c_int;
    base_longopts[6 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[6 as libc::c_int as usize].val = 'B' as i32;
    base_longopts[7 as libc::c_int as usize]
        .name = b"break\0" as *const u8 as *const libc::c_char;
    base_longopts[7 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[7 as libc::c_int as usize].flag = &mut opts.print_break;
    base_longopts[7 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[8 as libc::c_int as usize]
        .name = b"case-sensitive\0" as *const u8 as *const libc::c_char;
    base_longopts[8 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[8 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[8 as libc::c_int as usize].val = 's' as i32;
    base_longopts[9 as libc::c_int as usize]
        .name = b"color\0" as *const u8 as *const libc::c_char;
    base_longopts[9 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[9 as libc::c_int as usize].flag = &mut opts.color;
    base_longopts[9 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[10 as libc::c_int as usize]
        .name = b"color-line-number\0" as *const u8 as *const libc::c_char;
    base_longopts[10 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[10 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[10 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[11 as libc::c_int as usize]
        .name = b"color-match\0" as *const u8 as *const libc::c_char;
    base_longopts[11 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[11 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[11 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[12 as libc::c_int as usize]
        .name = b"color-path\0" as *const u8 as *const libc::c_char;
    base_longopts[12 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[12 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[12 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[13 as libc::c_int as usize]
        .name = b"color-win-ansi\0" as *const u8 as *const libc::c_char;
    base_longopts[13 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[13 as libc::c_int as usize].flag = &mut opts.color_win_ansi;
    base_longopts[13 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[14 as libc::c_int as usize]
        .name = b"column\0" as *const u8 as *const libc::c_char;
    base_longopts[14 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[14 as libc::c_int as usize].flag = &mut opts.column;
    base_longopts[14 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[15 as libc::c_int as usize]
        .name = b"context\0" as *const u8 as *const libc::c_char;
    base_longopts[15 as libc::c_int as usize].has_arg = 2 as libc::c_int;
    base_longopts[15 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[15 as libc::c_int as usize].val = 'C' as i32;
    base_longopts[16 as libc::c_int as usize]
        .name = b"count\0" as *const u8 as *const libc::c_char;
    base_longopts[16 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[16 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[16 as libc::c_int as usize].val = 'c' as i32;
    base_longopts[17 as libc::c_int as usize]
        .name = b"debug\0" as *const u8 as *const libc::c_char;
    base_longopts[17 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[17 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[17 as libc::c_int as usize].val = 'D' as i32;
    base_longopts[18 as libc::c_int as usize]
        .name = b"depth\0" as *const u8 as *const libc::c_char;
    base_longopts[18 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[18 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[18 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[19 as libc::c_int as usize]
        .name = b"filename\0" as *const u8 as *const libc::c_char;
    base_longopts[19 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[19 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[19 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[20 as libc::c_int as usize]
        .name = b"filename-pattern\0" as *const u8 as *const libc::c_char;
    base_longopts[20 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[20 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[20 as libc::c_int as usize].val = 'g' as i32;
    base_longopts[21 as libc::c_int as usize]
        .name = b"file-search-regex\0" as *const u8 as *const libc::c_char;
    base_longopts[21 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[21 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[21 as libc::c_int as usize].val = 'G' as i32;
    base_longopts[22 as libc::c_int as usize]
        .name = b"files-with-matches\0" as *const u8 as *const libc::c_char;
    base_longopts[22 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[22 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[22 as libc::c_int as usize].val = 'l' as i32;
    base_longopts[23 as libc::c_int as usize]
        .name = b"files-without-matches\0" as *const u8 as *const libc::c_char;
    base_longopts[23 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[23 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[23 as libc::c_int as usize].val = 'L' as i32;
    base_longopts[24 as libc::c_int as usize]
        .name = b"fixed-strings\0" as *const u8 as *const libc::c_char;
    base_longopts[24 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[24 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[24 as libc::c_int as usize].val = 'F' as i32;
    base_longopts[25 as libc::c_int as usize]
        .name = b"follow\0" as *const u8 as *const libc::c_char;
    base_longopts[25 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[25 as libc::c_int as usize].flag = &mut opts.follow_symlinks;
    base_longopts[25 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[26 as libc::c_int as usize]
        .name = b"group\0" as *const u8 as *const libc::c_char;
    base_longopts[26 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[26 as libc::c_int as usize].flag = &mut group;
    base_longopts[26 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[27 as libc::c_int as usize]
        .name = b"heading\0" as *const u8 as *const libc::c_char;
    base_longopts[27 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[27 as libc::c_int as usize].flag = &mut opts.print_path;
    base_longopts[27 as libc::c_int as usize].val = 2 as libc::c_int;
    base_longopts[28 as libc::c_int as usize]
        .name = b"help\0" as *const u8 as *const libc::c_char;
    base_longopts[28 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[28 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[28 as libc::c_int as usize].val = 'h' as i32;
    base_longopts[29 as libc::c_int as usize]
        .name = b"hidden\0" as *const u8 as *const libc::c_char;
    base_longopts[29 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[29 as libc::c_int as usize].flag = &mut opts.search_hidden_files;
    base_longopts[29 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[30 as libc::c_int as usize]
        .name = b"ignore\0" as *const u8 as *const libc::c_char;
    base_longopts[30 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[30 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[30 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[31 as libc::c_int as usize]
        .name = b"ignore-case\0" as *const u8 as *const libc::c_char;
    base_longopts[31 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[31 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[31 as libc::c_int as usize].val = 'i' as i32;
    base_longopts[32 as libc::c_int as usize]
        .name = b"ignore-dir\0" as *const u8 as *const libc::c_char;
    base_longopts[32 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[32 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[32 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[33 as libc::c_int as usize]
        .name = b"invert-match\0" as *const u8 as *const libc::c_char;
    base_longopts[33 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[33 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[33 as libc::c_int as usize].val = 'v' as i32;
    base_longopts[34 as libc::c_int as usize]
        .name = b"line-numbers\0" as *const u8 as *const libc::c_char;
    base_longopts[34 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[34 as libc::c_int as usize].flag = &mut opts.print_line_numbers;
    base_longopts[34 as libc::c_int as usize].val = 2 as libc::c_int;
    base_longopts[35 as libc::c_int as usize]
        .name = b"list-file-types\0" as *const u8 as *const libc::c_char;
    base_longopts[35 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[35 as libc::c_int as usize].flag = &mut list_file_types;
    base_longopts[35 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[36 as libc::c_int as usize]
        .name = b"literal\0" as *const u8 as *const libc::c_char;
    base_longopts[36 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[36 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[36 as libc::c_int as usize].val = 'Q' as i32;
    base_longopts[37 as libc::c_int as usize]
        .name = b"match\0" as *const u8 as *const libc::c_char;
    base_longopts[37 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[37 as libc::c_int as usize].flag = &mut useless;
    base_longopts[37 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[38 as libc::c_int as usize]
        .name = b"max-count\0" as *const u8 as *const libc::c_char;
    base_longopts[38 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[38 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[38 as libc::c_int as usize].val = 'm' as i32;
    base_longopts[39 as libc::c_int as usize]
        .name = b"mmap\0" as *const u8 as *const libc::c_char;
    base_longopts[39 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[39 as libc::c_int as usize].flag = &mut opts.mmap;
    base_longopts[39 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[40 as libc::c_int as usize]
        .name = b"multiline\0" as *const u8 as *const libc::c_char;
    base_longopts[40 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[40 as libc::c_int as usize].flag = &mut opts.multiline;
    base_longopts[40 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[41 as libc::c_int as usize]
        .name = b"no-affinity\0" as *const u8 as *const libc::c_char;
    base_longopts[41 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[41 as libc::c_int as usize].flag = &mut opts.use_thread_affinity;
    base_longopts[41 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[42 as libc::c_int as usize]
        .name = b"noaffinity\0" as *const u8 as *const libc::c_char;
    base_longopts[42 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[42 as libc::c_int as usize].flag = &mut opts.use_thread_affinity;
    base_longopts[42 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[43 as libc::c_int as usize]
        .name = b"no-break\0" as *const u8 as *const libc::c_char;
    base_longopts[43 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[43 as libc::c_int as usize].flag = &mut opts.print_break;
    base_longopts[43 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[44 as libc::c_int as usize]
        .name = b"nobreak\0" as *const u8 as *const libc::c_char;
    base_longopts[44 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[44 as libc::c_int as usize].flag = &mut opts.print_break;
    base_longopts[44 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[45 as libc::c_int as usize]
        .name = b"no-color\0" as *const u8 as *const libc::c_char;
    base_longopts[45 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[45 as libc::c_int as usize].flag = &mut opts.color;
    base_longopts[45 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[46 as libc::c_int as usize]
        .name = b"nocolor\0" as *const u8 as *const libc::c_char;
    base_longopts[46 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[46 as libc::c_int as usize].flag = &mut opts.color;
    base_longopts[46 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[47 as libc::c_int as usize]
        .name = b"no-filename\0" as *const u8 as *const libc::c_char;
    base_longopts[47 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[47 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[47 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[48 as libc::c_int as usize]
        .name = b"nofilename\0" as *const u8 as *const libc::c_char;
    base_longopts[48 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[48 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[48 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[49 as libc::c_int as usize]
        .name = b"no-follow\0" as *const u8 as *const libc::c_char;
    base_longopts[49 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[49 as libc::c_int as usize].flag = &mut opts.follow_symlinks;
    base_longopts[49 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[50 as libc::c_int as usize]
        .name = b"nofollow\0" as *const u8 as *const libc::c_char;
    base_longopts[50 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[50 as libc::c_int as usize].flag = &mut opts.follow_symlinks;
    base_longopts[50 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[51 as libc::c_int as usize]
        .name = b"no-group\0" as *const u8 as *const libc::c_char;
    base_longopts[51 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[51 as libc::c_int as usize].flag = &mut group;
    base_longopts[51 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[52 as libc::c_int as usize]
        .name = b"nogroup\0" as *const u8 as *const libc::c_char;
    base_longopts[52 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[52 as libc::c_int as usize].flag = &mut group;
    base_longopts[52 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[53 as libc::c_int as usize]
        .name = b"no-heading\0" as *const u8 as *const libc::c_char;
    base_longopts[53 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[53 as libc::c_int as usize].flag = &mut opts.print_path;
    base_longopts[53 as libc::c_int as usize].val = 3 as libc::c_int;
    base_longopts[54 as libc::c_int as usize]
        .name = b"noheading\0" as *const u8 as *const libc::c_char;
    base_longopts[54 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[54 as libc::c_int as usize].flag = &mut opts.print_path;
    base_longopts[54 as libc::c_int as usize].val = 3 as libc::c_int;
    base_longopts[55 as libc::c_int as usize]
        .name = b"no-mmap\0" as *const u8 as *const libc::c_char;
    base_longopts[55 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[55 as libc::c_int as usize].flag = &mut opts.mmap;
    base_longopts[55 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[56 as libc::c_int as usize]
        .name = b"nommap\0" as *const u8 as *const libc::c_char;
    base_longopts[56 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[56 as libc::c_int as usize].flag = &mut opts.mmap;
    base_longopts[56 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[57 as libc::c_int as usize]
        .name = b"no-multiline\0" as *const u8 as *const libc::c_char;
    base_longopts[57 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[57 as libc::c_int as usize].flag = &mut opts.multiline;
    base_longopts[57 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[58 as libc::c_int as usize]
        .name = b"nomultiline\0" as *const u8 as *const libc::c_char;
    base_longopts[58 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[58 as libc::c_int as usize].flag = &mut opts.multiline;
    base_longopts[58 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[59 as libc::c_int as usize]
        .name = b"no-numbers\0" as *const u8 as *const libc::c_char;
    base_longopts[59 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[59 as libc::c_int as usize].flag = &mut opts.print_line_numbers;
    base_longopts[59 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[60 as libc::c_int as usize]
        .name = b"nonumbers\0" as *const u8 as *const libc::c_char;
    base_longopts[60 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[60 as libc::c_int as usize].flag = &mut opts.print_line_numbers;
    base_longopts[60 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[61 as libc::c_int as usize]
        .name = b"no-pager\0" as *const u8 as *const libc::c_char;
    base_longopts[61 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[61 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[61 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[62 as libc::c_int as usize]
        .name = b"nopager\0" as *const u8 as *const libc::c_char;
    base_longopts[62 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[62 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[62 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[63 as libc::c_int as usize]
        .name = b"no-recurse\0" as *const u8 as *const libc::c_char;
    base_longopts[63 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[63 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[63 as libc::c_int as usize].val = 'n' as i32;
    base_longopts[64 as libc::c_int as usize]
        .name = b"norecurse\0" as *const u8 as *const libc::c_char;
    base_longopts[64 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[64 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[64 as libc::c_int as usize].val = 'n' as i32;
    base_longopts[65 as libc::c_int as usize]
        .name = b"null\0" as *const u8 as *const libc::c_char;
    base_longopts[65 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[65 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[65 as libc::c_int as usize].val = '0' as i32;
    base_longopts[66 as libc::c_int as usize]
        .name = b"numbers\0" as *const u8 as *const libc::c_char;
    base_longopts[66 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[66 as libc::c_int as usize].flag = &mut opts.print_line_numbers;
    base_longopts[66 as libc::c_int as usize].val = 2 as libc::c_int;
    base_longopts[67 as libc::c_int as usize]
        .name = b"only-matching\0" as *const u8 as *const libc::c_char;
    base_longopts[67 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[67 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[67 as libc::c_int as usize].val = 'o' as i32;
    base_longopts[68 as libc::c_int as usize]
        .name = b"one-device\0" as *const u8 as *const libc::c_char;
    base_longopts[68 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[68 as libc::c_int as usize].flag = &mut opts.one_dev;
    base_longopts[68 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[69 as libc::c_int as usize]
        .name = b"pager\0" as *const u8 as *const libc::c_char;
    base_longopts[69 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[69 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[69 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[70 as libc::c_int as usize]
        .name = b"parallel\0" as *const u8 as *const libc::c_char;
    base_longopts[70 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[70 as libc::c_int as usize].flag = &mut opts.parallel;
    base_longopts[70 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[71 as libc::c_int as usize]
        .name = b"passthrough\0" as *const u8 as *const libc::c_char;
    base_longopts[71 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[71 as libc::c_int as usize].flag = &mut opts.passthrough;
    base_longopts[71 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[72 as libc::c_int as usize]
        .name = b"passthru\0" as *const u8 as *const libc::c_char;
    base_longopts[72 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[72 as libc::c_int as usize].flag = &mut opts.passthrough;
    base_longopts[72 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[73 as libc::c_int as usize]
        .name = b"path-to-ignore\0" as *const u8 as *const libc::c_char;
    base_longopts[73 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[73 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[73 as libc::c_int as usize].val = 'p' as i32;
    base_longopts[74 as libc::c_int as usize]
        .name = b"print0\0" as *const u8 as *const libc::c_char;
    base_longopts[74 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[74 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[74 as libc::c_int as usize].val = '0' as i32;
    base_longopts[75 as libc::c_int as usize]
        .name = b"print-all-files\0" as *const u8 as *const libc::c_char;
    base_longopts[75 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[75 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[75 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[76 as libc::c_int as usize]
        .name = b"print-long-lines\0" as *const u8 as *const libc::c_char;
    base_longopts[76 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[76 as libc::c_int as usize].flag = &mut opts.print_long_lines;
    base_longopts[76 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[77 as libc::c_int as usize]
        .name = b"recurse\0" as *const u8 as *const libc::c_char;
    base_longopts[77 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[77 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[77 as libc::c_int as usize].val = 'r' as i32;
    base_longopts[78 as libc::c_int as usize]
        .name = b"search-binary\0" as *const u8 as *const libc::c_char;
    base_longopts[78 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[78 as libc::c_int as usize].flag = &mut opts.search_binary_files;
    base_longopts[78 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[79 as libc::c_int as usize]
        .name = b"search-files\0" as *const u8 as *const libc::c_char;
    base_longopts[79 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[79 as libc::c_int as usize].flag = &mut opts.search_stream;
    base_longopts[79 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[80 as libc::c_int as usize]
        .name = b"search-zip\0" as *const u8 as *const libc::c_char;
    base_longopts[80 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[80 as libc::c_int as usize].flag = &mut opts.search_zip_files;
    base_longopts[80 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[81 as libc::c_int as usize]
        .name = b"silent\0" as *const u8 as *const libc::c_char;
    base_longopts[81 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[81 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[81 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[82 as libc::c_int as usize]
        .name = b"skip-vcs-ignores\0" as *const u8 as *const libc::c_char;
    base_longopts[82 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[82 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[82 as libc::c_int as usize].val = 'U' as i32;
    base_longopts[83 as libc::c_int as usize]
        .name = b"smart-case\0" as *const u8 as *const libc::c_char;
    base_longopts[83 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[83 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[83 as libc::c_int as usize].val = 'S' as i32;
    base_longopts[84 as libc::c_int as usize]
        .name = b"stats\0" as *const u8 as *const libc::c_char;
    base_longopts[84 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[84 as libc::c_int as usize].flag = &mut opts.stats;
    base_longopts[84 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[85 as libc::c_int as usize]
        .name = b"stats-only\0" as *const u8 as *const libc::c_char;
    base_longopts[85 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[85 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[85 as libc::c_int as usize].val = 0 as libc::c_int;
    base_longopts[86 as libc::c_int as usize]
        .name = b"unrestricted\0" as *const u8 as *const libc::c_char;
    base_longopts[86 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[86 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[86 as libc::c_int as usize].val = 'u' as i32;
    base_longopts[87 as libc::c_int as usize]
        .name = b"version\0" as *const u8 as *const libc::c_char;
    base_longopts[87 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[87 as libc::c_int as usize].flag = &mut version;
    base_longopts[87 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[88 as libc::c_int as usize]
        .name = b"vimgrep\0" as *const u8 as *const libc::c_char;
    base_longopts[88 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[88 as libc::c_int as usize].flag = &mut opts.vimgrep;
    base_longopts[88 as libc::c_int as usize].val = 1 as libc::c_int;
    base_longopts[89 as libc::c_int as usize]
        .name = b"width\0" as *const u8 as *const libc::c_char;
    base_longopts[89 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[89 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[89 as libc::c_int as usize].val = 'W' as i32;
    base_longopts[90 as libc::c_int as usize]
        .name = b"word-regexp\0" as *const u8 as *const libc::c_char;
    base_longopts[90 as libc::c_int as usize].has_arg = 0 as libc::c_int;
    base_longopts[90 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[90 as libc::c_int as usize].val = 'w' as i32;
    base_longopts[91 as libc::c_int as usize]
        .name = b"workers\0" as *const u8 as *const libc::c_char;
    base_longopts[91 as libc::c_int as usize].has_arg = 1 as libc::c_int;
    base_longopts[91 as libc::c_int as usize]
        .flag = 0 as *mut libc::c_void as *mut libc::c_int;
    base_longopts[91 as libc::c_int as usize].val = 0 as libc::c_int;
    lang_count = get_lang_count();
    longopts_len = (::std::mem::size_of::<[option_t; 92]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<option_t>() as libc::c_ulong);
    full_len = longopts_len.wrapping_add(lang_count).wrapping_add(1 as libc::c_ulong);
    tmp___0 = ag_malloc(
        full_len.wrapping_mul(::std::mem::size_of::<option_t>() as libc::c_ulong),
    );
    longopts = tmp___0 as *mut option_t;
    memcpy(
        longopts as *mut libc::c_void,
        base_longopts.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[option_t; 92]>() as libc::c_ulong,
    );
    tmp___1 = ag_malloc(
        (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(lang_count),
    );
    ext_index = tmp___1 as *mut size_t;
    memset(
        ext_index as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<size_t>() as libc::c_ulong).wrapping_mul(lang_count),
    );
    i = 0 as libc::c_int as size_t;
    while i < lang_count {
        opt.name = langs[i as usize].name;
        opt.has_arg = 0 as libc::c_int;
        opt.flag = 0 as *mut libc::c_void as *mut libc::c_int;
        opt.val = 0 as libc::c_int;
        *longopts.offset(i.wrapping_add(longopts_len) as isize) = opt;
        i = i.wrapping_add(1);
    }
    __constr_expr_0.name = 0 as *mut libc::c_void as *const libc::c_char;
    __constr_expr_0.has_arg = 0 as libc::c_int;
    __constr_expr_0.flag = 0 as *mut libc::c_void as *mut libc::c_int;
    __constr_expr_0.val = 0 as libc::c_int;
    *longopts
        .offset(full_len.wrapping_sub(1 as libc::c_ulong) as isize) = __constr_expr_0;
    if argc < 2 as libc::c_int {
        usage();
        cleanup_ignore(root_ignores);
        cleanup_options();
        exit(1 as libc::c_int);
    }
    tmp___2 = fileno(stdin);
    rv = fstat(tmp___2, &mut statbuf);
    if rv == 0 as libc::c_int {
        if statbuf.st_mode & 61440 as libc::c_uint == 4096 as libc::c_uint {
            opts.search_stream = 1 as libc::c_int;
        } else if statbuf.st_mode & 61440 as libc::c_uint == 32768 as libc::c_uint {
            opts.search_stream = 1 as libc::c_int;
        }
    }
    tmp___4 = fileno(stdout);
    tmp___5 = isatty(tmp___4);
    if tmp___5 == 0 {
        opts.color = 0 as libc::c_int;
        group = 0 as libc::c_int;
        tmp___3 = fileno(stdout);
        rv = fstat(tmp___3, &mut statbuf);
        if rv != 0 as libc::c_int {
            die(b"Error fstat()ing stdout\0" as *const u8 as *const libc::c_char);
        }
        opts.stdout_inode = statbuf.st_ino;
    }
    file_search_regex = 0 as *mut libc::c_void as *mut libc::c_char;
    loop {
        ch = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"A:aB:C:cDG:g:FfHhiLlm:nop:QRrSsvVtuUwW:z0\0" as *const u8
                as *const libc::c_char,
            longopts as *const option,
            &mut opt_index,
        );
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_612: u64;
        match ch {
            65 => {
                if !optarg.is_null() {
                    tmp___6 = strtol(
                        optarg as *const libc::c_char,
                        &mut num_end as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    opts.after = tmp___6 as size_t;
                    if num_end as libc::c_ulong == optarg as libc::c_ulong {
                        optind -= 1;
                        opts.after = 2 as libc::c_int as size_t;
                    } else if *num_end as libc::c_int != 0 as libc::c_int {
                        optind -= 1;
                        opts.after = 2 as libc::c_int as size_t;
                    } else {
                        tmp___7 = __errno_location();
                        if *tmp___7 == 34 as libc::c_int {
                            optind -= 1;
                            opts.after = 2 as libc::c_int as size_t;
                        }
                    }
                } else {
                    opts.after = 2 as libc::c_int as size_t;
                }
                current_block_612 = 4334755519621573660;
            }
            97 => {
                opts.search_all_files = 1 as libc::c_int;
                opts.search_binary_files = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            66 => {
                if !optarg.is_null() {
                    tmp___8 = strtol(
                        optarg as *const libc::c_char,
                        &mut num_end as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    opts.before = tmp___8 as size_t;
                    if num_end as libc::c_ulong == optarg as libc::c_ulong {
                        optind -= 1;
                        opts.before = 2 as libc::c_int as size_t;
                    } else if *num_end as libc::c_int != 0 as libc::c_int {
                        optind -= 1;
                        opts.before = 2 as libc::c_int as size_t;
                    } else {
                        tmp___9 = __errno_location();
                        if *tmp___9 == 34 as libc::c_int {
                            optind -= 1;
                            opts.before = 2 as libc::c_int as size_t;
                        }
                    }
                } else {
                    opts.before = 2 as libc::c_int as size_t;
                }
                current_block_612 = 4334755519621573660;
            }
            67 => {
                if !optarg.is_null() {
                    tmp___10 = strtol(
                        optarg as *const libc::c_char,
                        &mut num_end as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    opts.context = tmp___10 as libc::c_int;
                    if num_end as libc::c_ulong == optarg as libc::c_ulong {
                        optind -= 1;
                        opts.context = 2 as libc::c_int;
                    } else if *num_end as libc::c_int != 0 as libc::c_int {
                        optind -= 1;
                        opts.context = 2 as libc::c_int;
                    } else {
                        tmp___11 = __errno_location();
                        if *tmp___11 == 34 as libc::c_int {
                            optind -= 1;
                            opts.context = 2 as libc::c_int;
                        }
                    }
                } else {
                    opts.context = 2 as libc::c_int;
                }
                current_block_612 = 4334755519621573660;
            }
            99 => {
                opts.print_count = 1 as libc::c_int;
                opts.print_filename_only = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            68 => {
                set_log_level(LOG_LEVEL_DEBUG);
                current_block_612 = 4334755519621573660;
            }
            102 => {
                opts.follow_symlinks = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            103 => {
                accepts_query = 0 as libc::c_int;
                needs_query = accepts_query;
                opts.match_files = 1 as libc::c_int;
                current_block_612 = 1265405943498933926;
            }
            71 => {
                current_block_612 = 1265405943498933926;
            }
            72 => {
                opts.print_path = 2 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            104 => {
                help = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            105 => {
                opts.casing = CASE_INSENSITIVE;
                current_block_612 = 4334755519621573660;
            }
            76 => {
                opts.print_nonmatching_files = 1 as libc::c_int;
                opts.print_path = 2 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            108 => {
                needs_query = 0 as libc::c_int;
                opts.print_filename_only = 1 as libc::c_int;
                opts.print_path = 2 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            109 => {
                tmp___12 = atoi(optarg as *const libc::c_char);
                opts.max_matches_per_file = tmp___12 as size_t;
                current_block_612 = 4334755519621573660;
            }
            110 => {
                opts.recurse_dirs = 0 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            112 => {
                opts.path_to_ignore = 1 as libc::c_int;
                load_ignore_patterns(root_ignores, optarg as *const libc::c_char);
                current_block_612 = 4334755519621573660;
            }
            111 => {
                opts.only_matching = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            81 | 70 => {
                opts.literal = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            114 | 82 => {
                opts.recurse_dirs = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            83 => {
                opts.casing = CASE_SMART;
                current_block_612 = 4334755519621573660;
            }
            115 => {
                opts.casing = CASE_SENSITIVE;
                current_block_612 = 4334755519621573660;
            }
            116 => {
                opts.search_all_files = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            117 => {
                opts.search_binary_files = 1 as libc::c_int;
                opts.search_all_files = 1 as libc::c_int;
                opts.search_hidden_files = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            85 => {
                opts.skip_vcs_ignores = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            118 => {
                opts.invert_match = 1 as libc::c_int;
                opts.color = 0 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            86 => {
                version = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            119 => {
                opts.word_regexp = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            87 => {
                tmp___13 = strtol(
                    optarg as *const libc::c_char,
                    &mut num_end as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
                opts.width = tmp___13 as size_t;
                if num_end as libc::c_ulong == optarg as libc::c_ulong {
                    die(b"Invalid width\n\0" as *const u8 as *const libc::c_char);
                } else if *num_end as libc::c_int != 0 as libc::c_int {
                    die(b"Invalid width\n\0" as *const u8 as *const libc::c_char);
                } else {
                    tmp___14 = __errno_location();
                    if *tmp___14 == 34 as libc::c_int {
                        die(b"Invalid width\n\0" as *const u8 as *const libc::c_char);
                    }
                }
                current_block_612 = 4334755519621573660;
            }
            122 => {
                opts.search_zip_files = 1 as libc::c_int;
                current_block_612 = 4334755519621573660;
            }
            48 => {
                opts.path_sep = '\u{0}' as i32 as libc::c_char;
                current_block_612 = 4334755519621573660;
            }
            0 => {
                tmp___31 = strcmp(
                    (*longopts.offset(opt_index as isize)).name,
                    b"ackmate-dir-filter\0" as *const u8 as *const libc::c_char,
                );
                if tmp___31 == 0 as libc::c_int {
                    compile_study(
                        &mut opts.ackmate_dir_filter,
                        &mut opts.ackmate_dir_filter_extra,
                        optarg,
                        0 as libc::c_int,
                        0 as libc::c_int,
                    );
                    current_block_612 = 4334755519621573660;
                } else {
                    tmp___30 = strcmp(
                        (*longopts.offset(opt_index as isize)).name,
                        b"depth\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___30 == 0 as libc::c_int {
                        opts.max_search_depth = atoi(optarg as *const libc::c_char);
                        current_block_612 = 4334755519621573660;
                    } else {
                        tmp___29 = strcmp(
                            (*longopts.offset(opt_index as isize)).name,
                            b"filename\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___29 == 0 as libc::c_int {
                            opts.print_path = 0 as libc::c_int;
                            opts.print_line_numbers = 1 as libc::c_int;
                            current_block_612 = 4334755519621573660;
                        } else {
                            tmp___28 = strcmp(
                                (*longopts.offset(opt_index as isize)).name,
                                b"ignore-dir\0" as *const u8 as *const libc::c_char,
                            );
                            if tmp___28 == 0 as libc::c_int {
                                add_ignore_pattern(
                                    root_ignores,
                                    optarg as *const libc::c_char,
                                );
                                current_block_612 = 4334755519621573660;
                            } else {
                                tmp___27 = strcmp(
                                    (*longopts.offset(opt_index as isize)).name,
                                    b"ignore\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp___27 == 0 as libc::c_int {
                                    add_ignore_pattern(
                                        root_ignores,
                                        optarg as *const libc::c_char,
                                    );
                                    current_block_612 = 4334755519621573660;
                                } else {
                                    tmp___25 = strcmp(
                                        (*longopts.offset(opt_index as isize)).name,
                                        b"no-filename\0" as *const u8 as *const libc::c_char,
                                    );
                                    if tmp___25 == 0 as libc::c_int {
                                        opts.print_path = 4 as libc::c_int;
                                        opts.print_line_numbers = 0 as libc::c_int;
                                        current_block_612 = 4334755519621573660;
                                    } else {
                                        tmp___26 = strcmp(
                                            (*longopts.offset(opt_index as isize)).name,
                                            b"nofilename\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___26 == 0 as libc::c_int {
                                            opts.print_path = 4 as libc::c_int;
                                            opts.print_line_numbers = 0 as libc::c_int;
                                            current_block_612 = 4334755519621573660;
                                        } else {
                                            tmp___23 = strcmp(
                                                (*longopts.offset(opt_index as isize)).name,
                                                b"no-pager\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___23 == 0 as libc::c_int {
                                                out_fd = stdout;
                                                opts.pager = 0 as *mut libc::c_void as *mut libc::c_char;
                                                current_block_612 = 4334755519621573660;
                                            } else {
                                                tmp___24 = strcmp(
                                                    (*longopts.offset(opt_index as isize)).name,
                                                    b"nopager\0" as *const u8 as *const libc::c_char,
                                                );
                                                if tmp___24 == 0 as libc::c_int {
                                                    out_fd = stdout;
                                                    opts.pager = 0 as *mut libc::c_void as *mut libc::c_char;
                                                    current_block_612 = 4334755519621573660;
                                                } else {
                                                    tmp___22 = strcmp(
                                                        (*longopts.offset(opt_index as isize)).name,
                                                        b"pager\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if tmp___22 == 0 as libc::c_int {
                                                        opts.pager = optarg;
                                                        current_block_612 = 4334755519621573660;
                                                    } else {
                                                        tmp___21 = strcmp(
                                                            (*longopts.offset(opt_index as isize)).name,
                                                            b"print-all-files\0" as *const u8 as *const libc::c_char,
                                                        );
                                                        if tmp___21 == 0 as libc::c_int {
                                                            opts.print_all_paths = 1 as libc::c_int;
                                                            current_block_612 = 4334755519621573660;
                                                        } else {
                                                            tmp___20 = strcmp(
                                                                (*longopts.offset(opt_index as isize)).name,
                                                                b"workers\0" as *const u8 as *const libc::c_char,
                                                            );
                                                            if tmp___20 == 0 as libc::c_int {
                                                                opts.workers = atoi(optarg as *const libc::c_char);
                                                                current_block_612 = 4334755519621573660;
                                                            } else {
                                                                tmp___19 = strcmp(
                                                                    (*longopts.offset(opt_index as isize)).name,
                                                                    b"color-line-number\0" as *const u8 as *const libc::c_char,
                                                                );
                                                                if tmp___19 == 0 as libc::c_int {
                                                                    free(opts.color_line_number as *mut libc::c_void);
                                                                    ag_asprintf(
                                                                        &mut opts.color_line_number as *mut *mut libc::c_char,
                                                                        b"\x1B[%sm\0" as *const u8 as *const libc::c_char,
                                                                        optarg,
                                                                    );
                                                                    current_block_612 = 4334755519621573660;
                                                                } else {
                                                                    tmp___18 = strcmp(
                                                                        (*longopts.offset(opt_index as isize)).name,
                                                                        b"color-match\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                    if tmp___18 == 0 as libc::c_int {
                                                                        free(opts.color_match as *mut libc::c_void);
                                                                        ag_asprintf(
                                                                            &mut opts.color_match as *mut *mut libc::c_char,
                                                                            b"\x1B[%sm\0" as *const u8 as *const libc::c_char,
                                                                            optarg,
                                                                        );
                                                                        current_block_612 = 4334755519621573660;
                                                                    } else {
                                                                        tmp___17 = strcmp(
                                                                            (*longopts.offset(opt_index as isize)).name,
                                                                            b"color-path\0" as *const u8 as *const libc::c_char,
                                                                        );
                                                                        if tmp___17 == 0 as libc::c_int {
                                                                            free(opts.color_path as *mut libc::c_void);
                                                                            ag_asprintf(
                                                                                &mut opts.color_path as *mut *mut libc::c_char,
                                                                                b"\x1B[%sm\0" as *const u8 as *const libc::c_char,
                                                                                optarg,
                                                                            );
                                                                            current_block_612 = 4334755519621573660;
                                                                        } else {
                                                                            tmp___16 = strcmp(
                                                                                (*longopts.offset(opt_index as isize)).name,
                                                                                b"silent\0" as *const u8 as *const libc::c_char,
                                                                            );
                                                                            if tmp___16 == 0 as libc::c_int {
                                                                                set_log_level(LOG_LEVEL_NONE);
                                                                                current_block_612 = 4334755519621573660;
                                                                            } else {
                                                                                tmp___15 = strcmp(
                                                                                    (*longopts.offset(opt_index as isize)).name,
                                                                                    b"stats-only\0" as *const u8 as *const libc::c_char,
                                                                                );
                                                                                if tmp___15 == 0 as libc::c_int {
                                                                                    opts.print_filename_only = 1 as libc::c_int;
                                                                                    opts.print_path = 4 as libc::c_int;
                                                                                    opts.stats = 1 as libc::c_int;
                                                                                    current_block_612 = 4334755519621573660;
                                                                                } else if (*longopts.offset(opt_index as isize)).flag
                                                                                        as libc::c_ulong != 0 as *mut libc::c_int as libc::c_ulong
                                                                                    {
                                                                                    current_block_612 = 4334755519621573660;
                                                                                } else {
                                                                                    i = 0 as libc::c_int as size_t;
                                                                                    while i < lang_count {
                                                                                        tmp___33 = strcmp(
                                                                                            (*longopts.offset(opt_index as isize)).name,
                                                                                            langs[i as usize].name,
                                                                                        );
                                                                                        if tmp___33 == 0 as libc::c_int {
                                                                                            has_filetype = 1 as libc::c_int;
                                                                                            tmp___32 = lang_num;
                                                                                            lang_num = lang_num.wrapping_add(1);
                                                                                            *ext_index.offset(tmp___32 as isize) = i;
                                                                                            break;
                                                                                        } else {
                                                                                            i = i.wrapping_add(1);
                                                                                        }
                                                                                    }
                                                                                    if i != lang_count {
                                                                                        current_block_612 = 4334755519621573660;
                                                                                    } else {
                                                                                        log_err(
                                                                                            b"option %s does not take a value\0" as *const u8
                                                                                                as *const libc::c_char,
                                                                                            (*longopts.offset(opt_index as isize)).name,
                                                                                        );
                                                                                        current_block_612 = 140619065014995601;
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
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                current_block_612 = 140619065014995601;
            }
        }
        match current_block_612 {
            140619065014995601 => {
                usage();
                exit(1 as libc::c_int);
            }
            1265405943498933926 => {
                if !file_search_regex.is_null() {
                    log_err(
                        b"File search regex (-g or -G) already specified.\0" as *const u8
                            as *const libc::c_char,
                    );
                    usage();
                    exit(1 as libc::c_int);
                }
                file_search_regex = ag_strdup(optarg as *const libc::c_char);
            }
            _ => {}
        }
    }
    if opts.casing as libc::c_uint == 0 as libc::c_uint {
        opts.casing = CASE_SMART;
    }
    if !file_search_regex.is_null() {
        pcre_opts = 0 as libc::c_int;
        if opts.casing as libc::c_uint == 2 as libc::c_uint {
            pcre_opts |= 1 as libc::c_int;
        } else if opts.casing as libc::c_uint == 3 as libc::c_uint {
            tmp___34 = is_lowercase(file_search_regex as *const libc::c_char);
            if tmp___34 != 0 {
                pcre_opts |= 1 as libc::c_int;
            }
        }
        if opts.word_regexp != 0 {
            old_file_search_regex = file_search_regex;
            ag_asprintf(
                &mut file_search_regex as *mut *mut libc::c_char,
                b"\\b%s\\b\0" as *const u8 as *const libc::c_char,
                file_search_regex,
            );
            free(old_file_search_regex as *mut libc::c_void);
        }
        compile_study(
            &mut opts.file_search_regex,
            &mut opts.file_search_regex_extra,
            file_search_regex,
            pcre_opts,
            0 as libc::c_int,
        );
        free(file_search_regex as *mut libc::c_void);
    }
    if has_filetype != 0 {
        num_exts = combine_file_extensions(ext_index, lang_num, &mut extensions);
        lang_regex = make_lang_regex(extensions, num_exts);
        compile_study(
            &mut opts.file_search_regex,
            &mut opts.file_search_regex_extra,
            lang_regex,
            0 as libc::c_int,
            0 as libc::c_int,
        );
    }
    if !extensions.is_null() {
        free(extensions as *mut libc::c_void);
    }
    free(ext_index as *mut libc::c_void);
    if !lang_regex.is_null() {
        free(lang_regex as *mut libc::c_void);
    }
    free(longopts as *mut libc::c_void);
    argc -= optind;
    argv = argv.offset(optind as isize);
    if !(opts.pager).is_null() {
        out_fd = popen(
            opts.pager as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if out_fd.is_null() {
            perror(b"Failed to run pager\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
    }
    if help != 0 {
        usage();
        exit(0 as libc::c_int);
    }
    if version != 0 {
        print_version();
        exit(0 as libc::c_int);
    }
    if list_file_types != 0 {
        printf(
            b"The following file types are supported:\n\0" as *const u8
                as *const libc::c_char,
        );
        lang_index = 0 as libc::c_int as size_t;
        while lang_index < lang_count {
            printf(
                b"  --%s\n    \0" as *const u8 as *const libc::c_char,
                langs[lang_index as usize].name,
            );
            j = 0 as libc::c_int;
            while j < 12 as libc::c_int {
                if (langs[lang_index as usize].extensions[j as usize]).is_null() {
                    break;
                }
                printf(
                    b"  .%s\0" as *const u8 as *const libc::c_char,
                    langs[lang_index as usize].extensions[j as usize],
                );
                j += 1;
            }
            printf(b"\n\n\0" as *const u8 as *const libc::c_char);
            lang_index = lang_index.wrapping_add(1);
        }
        exit(0 as libc::c_int);
    }
    if needs_query != 0 {
        if argc == 0 as libc::c_int {
            log_err(
                b"What do you want to search for?\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    if !home_dir.is_null() {
        if opts.search_all_files == 0 {
            log_debug(
                b"Found user's home dir: %s\0" as *const u8 as *const libc::c_char,
                home_dir,
            );
            ag_asprintf(
                &mut ignore_file_path as *mut *mut libc::c_char,
                b"%s/.agignore\0" as *const u8 as *const libc::c_char,
                home_dir,
            );
            load_ignore_patterns(root_ignores, ignore_file_path as *const libc::c_char);
            free(ignore_file_path as *mut libc::c_void);
        }
    }
    if opts.skip_vcs_ignores == 0 {
        gitconfig_file = 0 as *mut libc::c_void as *mut FILE;
        buf_len = 0 as libc::c_int as size_t;
        gitconfig_res = 0 as *mut libc::c_void as *mut libc::c_char;
        gitconfig_file = popen(
            b"git config -z --path --get core.excludesfile 2>/dev/null\0" as *const u8
                as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if gitconfig_file as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            loop {
                tmp___35 = ag_realloc(
                    gitconfig_res as *mut libc::c_void,
                    buf_len.wrapping_add(65 as libc::c_ulong),
                );
                gitconfig_res = tmp___35 as *mut libc::c_char;
                tmp___36 = fread(
                    gitconfig_res.offset(buf_len as isize) as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                    64 as libc::c_int as size_t,
                    gitconfig_file,
                );
                buf_len = (buf_len as libc::c_ulong).wrapping_add(tmp___36) as size_t
                    as size_t;
                tmp___37 = feof(gitconfig_file);
                if tmp___37 != 0 {
                    break;
                }
                if !(buf_len > 0 as libc::c_ulong) {
                    break;
                }
                if !(buf_len.wrapping_rem(64 as libc::c_ulong) == 0 as libc::c_ulong) {
                    break;
                }
            }
            *gitconfig_res.offset(buf_len as isize) = '\u{0}' as i32 as libc::c_char;
            if buf_len == 0 as libc::c_ulong {
                free(gitconfig_res as *mut libc::c_void);
                tmp___38 = getenv(
                    b"XDG_CONFIG_HOME\0" as *const u8 as *const libc::c_char,
                );
                config_home = tmp___38 as *const libc::c_char;
                if !config_home.is_null() {
                    ag_asprintf(
                        &mut gitconfig_res as *mut *mut libc::c_char,
                        b"%s/%s\0" as *const u8 as *const libc::c_char,
                        config_home,
                        b"git/ignore\0" as *const u8 as *const libc::c_char,
                    );
                } else if !home_dir.is_null() {
                    ag_asprintf(
                        &mut gitconfig_res as *mut *mut libc::c_char,
                        b"%s/%s\0" as *const u8 as *const libc::c_char,
                        home_dir,
                        b".config/git/ignore\0" as *const u8 as *const libc::c_char,
                    );
                } else {
                    gitconfig_res = ag_strdup(b"\0" as *const u8 as *const libc::c_char);
                }
            }
            log_debug(
                b"global core.excludesfile: %s\0" as *const u8 as *const libc::c_char,
                gitconfig_res,
            );
            load_ignore_patterns(root_ignores, gitconfig_res as *const libc::c_char);
            free(gitconfig_res as *mut libc::c_void);
            pclose(gitconfig_file);
        }
    }
    if opts.context > 0 as libc::c_int {
        opts.before = opts.context as size_t;
        opts.after = opts.context as size_t;
    }
    if opts.ackmate != 0 {
        opts.color = 0 as libc::c_int;
        opts.print_break = 1 as libc::c_int;
        group = 1 as libc::c_int;
        opts.search_stream = 0 as libc::c_int;
    }
    if opts.vimgrep != 0 {
        opts.color = 0 as libc::c_int;
        opts.print_break = 0 as libc::c_int;
        group = 1 as libc::c_int;
        opts.search_stream = 0 as libc::c_int;
        opts.print_path = 4 as libc::c_int;
    }
    if opts.parallel != 0 {
        opts.search_stream = 0 as libc::c_int;
    }
    if !(opts.print_path != 0 as libc::c_int) {
        if !(opts.print_break == 0 as libc::c_int) {
            if group != 0 {
                opts.print_break = 1 as libc::c_int;
            } else {
                opts.print_path = 1 as libc::c_int;
                opts.print_break = 0 as libc::c_int;
            }
        }
    }
    if opts.search_stream != 0 {
        opts.print_break = 0 as libc::c_int;
        opts.print_path = 4 as libc::c_int;
        if opts.print_line_numbers != 2 as libc::c_int {
            opts.print_line_numbers = 0 as libc::c_int;
        }
    }
    let mut current_block_779: u64;
    if accepts_query != 0 {
        if argc > 0 as libc::c_int {
            if needs_query == 0 {
                tmp___39 = strlen(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                );
                if tmp___39 == 0 as libc::c_ulong {
                    opts.query = ag_strdup(b".\0" as *const u8 as *const libc::c_char);
                } else {
                    opts
                        .query = ag_strdup(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                    );
                }
            } else {
                opts
                    .query = ag_strdup(
                    *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                );
            }
            argc -= 1;
            argv = argv.offset(1);
            current_block_779 = 5182549051472977253;
        } else {
            current_block_779 = 15412668079164927588;
        }
    } else {
        current_block_779 = 15412668079164927588;
    }
    match current_block_779 {
        15412668079164927588 => {
            if needs_query == 0 {
                opts.query = ag_strdup(b".\0" as *const u8 as *const libc::c_char);
            }
        }
        _ => {}
    }
    tmp___40 = strlen(opts.query as *const libc::c_char);
    opts.query_len = tmp___40 as libc::c_int;
    log_debug(b"Query is %s\0" as *const u8 as *const libc::c_char, opts.query);
    if opts.query_len == 0 as libc::c_int {
        log_err(
            b"Error: No query. What do you want to search for?\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    tmp___41 = is_regex(opts.query as *const libc::c_char);
    if tmp___41 == 0 {
        opts.literal = 1 as libc::c_int;
    }
    path = 0 as *mut libc::c_void as *mut libc::c_char;
    base_path = 0 as *mut libc::c_void as *mut libc::c_char;
    tmp___42 = 0 as *mut libc::c_void as *mut libc::c_char;
    opts.paths_len = argc;
    if argc > 0 as libc::c_int {
        tmp___43 = ag_calloc(
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            (argc + 1 as libc::c_int) as size_t,
        );
        *paths = tmp___43 as *mut *mut libc::c_char;
        tmp___44 = ag_calloc(
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            (argc + 1 as libc::c_int) as size_t,
        );
        *base_paths = tmp___44 as *mut *mut libc::c_char;
        i = 0 as libc::c_int as size_t;
        while i < argc as size_t {
            path = ag_strdup(*argv.offset(i as isize) as *const libc::c_char);
            tmp___45 = strlen(path as *const libc::c_char);
            path_len = tmp___45 as libc::c_int;
            if path_len > 1 as libc::c_int {
                if *path.offset((path_len - 1 as libc::c_int) as isize) as libc::c_int
                    == 47 as libc::c_int
                {
                    *path
                        .offset(
                            (path_len - 1 as libc::c_int) as isize,
                        ) = '\u{0}' as i32 as libc::c_char;
                }
            }
            let ref mut fresh2 = *(*paths).offset(i as isize);
            *fresh2 = path;
            tmp___46 = ag_malloc(4096 as libc::c_int as size_t);
            tmp___42 = tmp___46 as *mut libc::c_char;
            base_path = realpath(path as *const libc::c_char, tmp___42);
            if !base_path.is_null() {
                tmp___47 = strlen(base_path as *const libc::c_char);
                base_path_len = tmp___47 as libc::c_int;
                if base_path_len > 1 as libc::c_int {
                    if *base_path.offset((base_path_len - 1 as libc::c_int) as isize)
                        as libc::c_int != 47 as libc::c_int
                    {
                        tmp___48 = ag_realloc(
                            base_path as *mut libc::c_void,
                            (base_path_len + 2 as libc::c_int) as size_t,
                        );
                        base_path = tmp___48 as *mut libc::c_char;
                        *base_path
                            .offset(base_path_len as isize) = '/' as i32 as libc::c_char;
                        *base_path
                            .offset(
                                (base_path_len + 1 as libc::c_int) as isize,
                            ) = '\u{0}' as i32 as libc::c_char;
                    }
                }
            }
            let ref mut fresh3 = *(*base_paths).offset(i as isize);
            *fresh3 = base_path;
            i = i.wrapping_add(1);
        }
        opts.search_stream = 0 as libc::c_int;
    } else {
        path = ag_strdup(b".\0" as *const u8 as *const libc::c_char);
        tmp___49 = ag_malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong),
        );
        *paths = tmp___49 as *mut *mut libc::c_char;
        tmp___50 = ag_malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong),
        );
        *base_paths = tmp___50 as *mut *mut libc::c_char;
        let ref mut fresh4 = *(*paths).offset(0 as libc::c_int as isize);
        *fresh4 = path;
        tmp___51 = ag_malloc(4096 as libc::c_int as size_t);
        tmp___42 = tmp___51 as *mut libc::c_char;
        let ref mut fresh5 = *(*base_paths).offset(0 as libc::c_int as isize);
        *fresh5 = realpath(path as *const libc::c_char, tmp___42);
        i = 1 as libc::c_int as size_t;
    }
    let ref mut fresh6 = *(*paths).offset(i as isize);
    *fresh6 = 0 as *mut libc::c_void as *mut libc::c_char;
    let ref mut fresh7 = *(*base_paths).offset(i as isize);
    *fresh7 = 0 as *mut libc::c_void as *mut libc::c_char;
}
pub static mut first_file_match: libc::c_int = 1 as libc::c_int;
pub static mut color_reset: *const libc::c_char = b"\x1B[0m\x1B[K\0" as *const u8
    as *const libc::c_char;
pub static mut truncate_marker: *const libc::c_char = b" [...]\0" as *const u8
    as *const libc::c_char;
#[thread_local]
pub static mut print_context: print_context = print_context {
    line: 0,
    context_prev_lines: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    prev_line: 0,
    last_prev_line: 0,
    prev_line_offset: 0,
    line_preceding_current_match_offset: 0,
    lines_since_last_match: 0,
    last_printed_match: 0,
    in_a_match: 0,
    printing_a_match: 0,
};
pub unsafe extern "C" fn print_init_context() {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if print_context.context_prev_lines as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        return;
    }
    tmp = ag_calloc(
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        (opts.before).wrapping_add(1 as libc::c_ulong),
    );
    print_context.context_prev_lines = tmp as *mut *mut libc::c_char;
    print_context.line = 1 as libc::c_int as size_t;
    print_context.prev_line = 0 as libc::c_int as size_t;
    print_context.last_prev_line = 0 as libc::c_int as size_t;
    print_context.prev_line_offset = 0 as libc::c_int as size_t;
    print_context.line_preceding_current_match_offset = 0 as libc::c_int as size_t;
    print_context.lines_since_last_match = 2147483647 as libc::c_int as size_t;
    print_context.last_printed_match = 0 as libc::c_int as size_t;
    print_context.in_a_match = 0 as libc::c_int;
    print_context.printing_a_match = 0 as libc::c_int;
}
pub unsafe extern "C" fn print_cleanup_context() {
    let mut i: size_t = 0;
    if print_context.context_prev_lines as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < opts.before {
        if *(print_context.context_prev_lines).offset(i as isize) as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            free(
                *(print_context.context_prev_lines).offset(i as isize)
                    as *mut libc::c_void,
            );
        }
        i = i.wrapping_add(1);
    }
    free(print_context.context_prev_lines as *mut libc::c_void);
    print_context.context_prev_lines = 0 as *mut libc::c_void as *mut *mut libc::c_char;
}
pub unsafe extern "C" fn print_context_append(
    mut line: *const libc::c_char,
    mut len: size_t,
) {
    if opts.before == 0 as libc::c_ulong {
        return;
    }
    if *(print_context.context_prev_lines).offset(print_context.last_prev_line as isize)
        as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
    {
        free(
            *(print_context.context_prev_lines)
                .offset(print_context.last_prev_line as isize) as *mut libc::c_void,
        );
    }
    let ref mut fresh8 = *(print_context.context_prev_lines)
        .offset(print_context.last_prev_line as isize);
    *fresh8 = ag_strndup(line, len);
    print_context
        .last_prev_line = (print_context.last_prev_line)
        .wrapping_add(1 as libc::c_ulong)
        .wrapping_rem(opts.before);
}
pub unsafe extern "C" fn print_trailing_context(
    mut path: *const libc::c_char,
    mut buf: *const libc::c_char,
    mut n: size_t,
) {
    let mut sep: libc::c_char = 0;
    sep = '-' as i32 as libc::c_char;
    if opts.ackmate != 0 {
        sep = ':' as i32 as libc::c_char;
    } else if opts.vimgrep != 0 {
        sep = ':' as i32 as libc::c_char;
    }
    if print_context.lines_since_last_match != 0 as libc::c_ulong {
        if print_context.lines_since_last_match <= opts.after {
            if opts.print_path == 3 as libc::c_int {
                print_path(path, ':' as i32 as libc::c_char);
            }
            print_line_number(print_context.line, sep);
            fwrite(buf as *const libc::c_void, 1 as libc::c_int as size_t, n, out_fd);
            fputc('\n' as i32, out_fd);
        }
    }
    print_context.line = (print_context.line).wrapping_add(1);
    if print_context.in_a_match == 0 {
        if print_context.lines_since_last_match < 2147483647 as libc::c_ulong {
            print_context
                .lines_since_last_match = (print_context.lines_since_last_match)
                .wrapping_add(1);
        }
    }
}
pub unsafe extern "C" fn print_path(mut path: *const libc::c_char, sep: libc::c_char) {
    if opts.print_path == 4 as libc::c_int {
        if opts.vimgrep == 0 {
            return;
        }
    }
    path = normalize_path(path);
    if opts.ackmate != 0 {
        fprintf(
            out_fd,
            b":%s%c\0" as *const u8 as *const libc::c_char,
            path,
            sep as libc::c_int,
        );
    } else if opts.vimgrep != 0 {
        fprintf(
            out_fd,
            b"%s%c\0" as *const u8 as *const libc::c_char,
            path,
            sep as libc::c_int,
        );
    } else if opts.color != 0 {
        fprintf(
            out_fd,
            b"%s%s%s%c\0" as *const u8 as *const libc::c_char,
            opts.color_path,
            path,
            color_reset,
            sep as libc::c_int,
        );
    } else {
        fprintf(
            out_fd,
            b"%s%c\0" as *const u8 as *const libc::c_char,
            path,
            sep as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn print_path_count(
    mut path: *const libc::c_char,
    sep: libc::c_char,
    count: size_t,
) {
    if *path != 0 {
        print_path(path, ':' as i32 as libc::c_char);
    }
    if opts.color != 0 {
        fprintf(
            out_fd,
            b"%s%lu%s%c\0" as *const u8 as *const libc::c_char,
            opts.color_line_number,
            count,
            color_reset,
            sep as libc::c_int,
        );
    } else {
        fprintf(
            out_fd,
            b"%lu%c\0" as *const u8 as *const libc::c_char,
            count,
            sep as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn print_line(
    mut buf: *const libc::c_char,
    mut buf_pos: size_t,
    mut prev_line_offset: size_t,
) {
    let mut write_chars: size_t = 0;
    write_chars = buf_pos
        .wrapping_sub(prev_line_offset)
        .wrapping_add(1 as libc::c_ulong);
    if opts.width > 0 as libc::c_ulong {
        if opts.width < write_chars {
            write_chars = opts.width;
        }
    }
    fwrite(
        buf.offset(prev_line_offset as isize) as *const libc::c_void,
        1 as libc::c_int as size_t,
        write_chars,
        out_fd,
    );
}
pub unsafe extern "C" fn print_binary_file_matches(mut path: *const libc::c_char) {
    path = normalize_path(path);
    print_file_separator();
    fprintf(
        out_fd,
        b"Binary file %s matches.\n\0" as *const u8 as *const libc::c_char,
        path,
    );
}
pub unsafe extern "C" fn print_file_matches(
    mut path: *const libc::c_char,
    mut buf: *const libc::c_char,
    buf_len: size_t,
    mut matches: *const match_t,
    matches_len: size_t,
) {
    let mut cur_match: size_t = 0;
    let mut lines_to_print: ssize_t = 0;
    let mut sep: libc::c_char = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut blanks_between_matches: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut start: size_t = 0;
    let mut printed_match: libc::c_int = 0;
    cur_match = 0 as libc::c_int as size_t;
    lines_to_print = 0 as libc::c_int as ssize_t;
    sep = '-' as i32 as libc::c_char;
    if opts.context != 0 {
        tmp = 1 as libc::c_int;
    } else if opts.after != 0 {
        tmp = 1 as libc::c_int;
    } else if opts.before != 0 {
        tmp = 1 as libc::c_int;
    } else {
        tmp = 0 as libc::c_int;
    }
    blanks_between_matches = tmp;
    if opts.ackmate != 0 {
        sep = ':' as i32 as libc::c_char;
    } else if opts.vimgrep != 0 {
        sep = ':' as i32 as libc::c_char;
    }
    print_file_separator();
    if opts.print_path == 0 as libc::c_int {
        opts.print_path = 2 as libc::c_int;
    } else if opts.print_path == 1 as libc::c_int {
        opts.print_path = 3 as libc::c_int;
    }
    if opts.print_path == 2 as libc::c_int {
        if opts.print_count != 0 {
            print_path_count(path, opts.path_sep, matches_len);
        } else {
            print_path(path, opts.path_sep);
        }
    }
    i = 0 as libc::c_int as size_t;
    let mut current_block_196: u64;
    while i <= buf_len {
        if !(cur_match < matches_len) {
            if !(print_context.lines_since_last_match <= opts.after) {
                break;
            }
        }
        if cur_match < matches_len {
            if i == (*matches.offset(cur_match as isize)).start {
                print_context.in_a_match = 1 as libc::c_int;
                if cur_match > 0 as libc::c_ulong {
                    if blanks_between_matches != 0 {
                        if print_context.lines_since_last_match
                            > (opts.before)
                                .wrapping_add(opts.after)
                                .wrapping_add(1 as libc::c_ulong)
                        {
                            fprintf(
                                out_fd,
                                b"--\n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
                if print_context.lines_since_last_match > 0 as libc::c_ulong {
                    if opts.before > 0 as libc::c_ulong {
                        lines_to_print = (print_context.lines_since_last_match)
                            .wrapping_sub((opts.after).wrapping_add(1 as libc::c_ulong))
                            as ssize_t;
                        if lines_to_print < 0 as libc::c_long {
                            lines_to_print = 0 as libc::c_int as ssize_t;
                        } else if lines_to_print as size_t > opts.before {
                            lines_to_print = opts.before as ssize_t;
                        }
                        j = (opts.before).wrapping_sub(lines_to_print as size_t);
                        while j < opts.before {
                            print_context
                                .prev_line = (print_context.last_prev_line)
                                .wrapping_add(j)
                                .wrapping_rem(opts.before);
                            if *(print_context.context_prev_lines)
                                .offset(print_context.prev_line as isize) as libc::c_ulong
                                != 0 as *mut libc::c_void as libc::c_ulong
                            {
                                if opts.print_path == 3 as libc::c_int {
                                    print_path(path, ':' as i32 as libc::c_char);
                                }
                                print_line_number(
                                    (print_context.line)
                                        .wrapping_sub((opts.before).wrapping_sub(j)),
                                    sep,
                                );
                                fprintf(
                                    out_fd,
                                    b"%s\n\0" as *const u8 as *const libc::c_char,
                                    *(print_context.context_prev_lines)
                                        .offset(print_context.prev_line as isize),
                                );
                            }
                            j = j.wrapping_add(1);
                        }
                    }
                }
                print_context.lines_since_last_match = 0 as libc::c_int as size_t;
            }
        }
        if cur_match < matches_len {
            if i == (*matches.offset(cur_match as isize)).end {
                cur_match = cur_match.wrapping_add(1);
                print_context.in_a_match = 0 as libc::c_int;
            }
        }
        let mut current_block_80: u64;
        if i == buf_len {
            current_block_80 = 8420278727545978749;
        } else if *buf.offset(i as isize) as libc::c_int == 10 as libc::c_int {
            current_block_80 = 8420278727545978749;
        } else {
            current_block_80 = 5793491756164225964;
        }
        match current_block_80 {
            8420278727545978749 => {
                if opts.before > 0 as libc::c_ulong {
                    print_context_append(
                        buf.offset(print_context.prev_line_offset as isize),
                        i.wrapping_sub(print_context.prev_line_offset),
                    );
                }
            }
            _ => {}
        }
        if i == buf_len {
            current_block_196 = 5105659852674189508;
        } else if *buf.offset(i as isize) as libc::c_int == 10 as libc::c_int {
            current_block_196 = 5105659852674189508;
        } else {
            current_block_196 = 13598848910332274892;
        }
        match current_block_196 {
            5105659852674189508 => {
                if print_context.lines_since_last_match == 0 as libc::c_ulong {
                    if opts.print_path == 3 as libc::c_int {
                        if opts.search_stream == 0 {
                            print_path(path, ':' as i32 as libc::c_char);
                        }
                    }
                    if opts.ackmate != 0 {
                        print_line_number(
                            print_context.line,
                            ';' as i32 as libc::c_char,
                        );
                        while print_context.last_printed_match < cur_match {
                            start = ((*matches
                                .offset(print_context.last_printed_match as isize))
                                .start)
                                .wrapping_sub(
                                    print_context.line_preceding_current_match_offset,
                                );
                            fprintf(
                                out_fd,
                                b"%lu %lu\0" as *const u8 as *const libc::c_char,
                                start,
                                ((*matches
                                    .offset(print_context.last_printed_match as isize))
                                    .end)
                                    .wrapping_sub(
                                        (*matches.offset(print_context.last_printed_match as isize))
                                            .start,
                                    ),
                            );
                            if print_context.last_printed_match
                                == cur_match.wrapping_sub(1 as libc::c_ulong)
                            {
                                fputc(':' as i32, out_fd);
                            } else {
                                fputc(',' as i32, out_fd);
                            }
                            print_context
                                .last_printed_match = (print_context.last_printed_match)
                                .wrapping_add(1);
                        }
                        print_line(buf, i, print_context.prev_line_offset);
                    } else if opts.vimgrep != 0 {
                        while print_context.last_printed_match < cur_match {
                            print_path(path, sep);
                            print_line_number(print_context.line, sep);
                            print_column_number(
                                matches,
                                print_context.last_printed_match,
                                print_context.prev_line_offset,
                                sep,
                            );
                            print_line(buf, i, print_context.prev_line_offset);
                            print_context
                                .last_printed_match = (print_context.last_printed_match)
                                .wrapping_add(1);
                        }
                    } else {
                        print_line_number(
                            print_context.line,
                            ':' as i32 as libc::c_char,
                        );
                        printed_match = 0 as libc::c_int;
                        if opts.column != 0 {
                            print_column_number(
                                matches,
                                print_context.last_printed_match,
                                print_context.prev_line_offset,
                                ':' as i32 as libc::c_char,
                            );
                        }
                        if print_context.printing_a_match != 0 {
                            if opts.color != 0 {
                                fprintf(
                                    out_fd,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    opts.color_match,
                                );
                            }
                        }
                        j = print_context.prev_line_offset;
                        while j <= i {
                            if print_context.last_printed_match < matches_len {
                                if j
                                    == (*matches
                                        .offset(print_context.last_printed_match as isize))
                                        .end
                                {
                                    if opts.color != 0 {
                                        fprintf(
                                            out_fd,
                                            b"%s\0" as *const u8 as *const libc::c_char,
                                            color_reset,
                                        );
                                    }
                                    print_context.printing_a_match = 0 as libc::c_int;
                                    print_context
                                        .last_printed_match = (print_context.last_printed_match)
                                        .wrapping_add(1);
                                    printed_match = 1 as libc::c_int;
                                    if opts.only_matching != 0 {
                                        fputc('\n' as i32, out_fd);
                                    }
                                }
                            }
                            if j < buf_len {
                                if opts.width > 0 as libc::c_ulong {
                                    if j.wrapping_sub(print_context.prev_line_offset)
                                        >= opts.width
                                    {
                                        if j < i {
                                            fputs(truncate_marker, out_fd);
                                        }
                                        fputc('\n' as i32, out_fd);
                                        j = i;
                                        print_context.last_printed_match = matches_len;
                                    }
                                }
                            }
                            if print_context.last_printed_match < matches_len {
                                if j
                                    == (*matches
                                        .offset(print_context.last_printed_match as isize))
                                        .start
                                {
                                    if opts.only_matching != 0 {
                                        if printed_match != 0 {
                                            if opts.print_path == 3 as libc::c_int {
                                                print_path(path, ':' as i32 as libc::c_char);
                                            }
                                            print_line_number(
                                                print_context.line,
                                                ':' as i32 as libc::c_char,
                                            );
                                            if opts.column != 0 {
                                                print_column_number(
                                                    matches,
                                                    print_context.last_printed_match,
                                                    print_context.prev_line_offset,
                                                    ':' as i32 as libc::c_char,
                                                );
                                            }
                                        }
                                    }
                                    if opts.color != 0 {
                                        fprintf(
                                            out_fd,
                                            b"%s\0" as *const u8 as *const libc::c_char,
                                            opts.color_match,
                                        );
                                    }
                                    print_context.printing_a_match = 1 as libc::c_int;
                                }
                            }
                            if j < buf_len {
                                let mut current_block_168: u64;
                                if opts.only_matching == 0 {
                                    current_block_168 = 4333469038864053702;
                                } else if print_context.printing_a_match != 0 {
                                    current_block_168 = 4333469038864053702;
                                } else {
                                    current_block_168 = 13665239467142187023;
                                }
                                match current_block_168 {
                                    4333469038864053702 => {
                                        if opts.width == 0 as libc::c_ulong {
                                            fputc(*buf.offset(j as isize) as libc::c_int, out_fd);
                                        } else if j.wrapping_sub(print_context.prev_line_offset)
                                                < opts.width
                                            {
                                            fputc(*buf.offset(j as isize) as libc::c_int, out_fd);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            j = j.wrapping_add(1);
                        }
                        if print_context.printing_a_match != 0 {
                            if opts.color != 0 {
                                fprintf(
                                    out_fd,
                                    b"%s\0" as *const u8 as *const libc::c_char,
                                    color_reset,
                                );
                            }
                        }
                    }
                }
                if opts.search_stream != 0 {
                    print_context.last_printed_match = 0 as libc::c_int as size_t;
                    break;
                } else {
                    print_trailing_context(
                        path,
                        buf.offset(print_context.prev_line_offset as isize),
                        i.wrapping_sub(print_context.prev_line_offset),
                    );
                    print_context.prev_line_offset = i.wrapping_add(1 as libc::c_ulong);
                    if print_context.in_a_match == 0 {
                        print_context
                            .line_preceding_current_match_offset = i
                            .wrapping_add(1 as libc::c_ulong);
                    }
                    if i == buf_len {
                        if *buf.offset(i.wrapping_sub(1 as libc::c_ulong) as isize)
                            as libc::c_int != 10 as libc::c_int
                        {
                            fputc('\n' as i32, out_fd);
                        }
                    }
                }
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    if opts.stdout_inode != 0 {
        fflush(out_fd);
    }
}
pub unsafe extern "C" fn print_line_number(mut line: size_t, sep: libc::c_char) {
    if opts.print_line_numbers == 0 {
        return;
    }
    if opts.color != 0 {
        fprintf(
            out_fd,
            b"%s%lu%s%c\0" as *const u8 as *const libc::c_char,
            opts.color_line_number,
            line,
            color_reset,
            sep as libc::c_int,
        );
    } else {
        fprintf(
            out_fd,
            b"%lu%c\0" as *const u8 as *const libc::c_char,
            line,
            sep as libc::c_int,
        );
    };
}
pub unsafe extern "C" fn print_column_number(
    mut matches: *const match_t,
    mut last_printed_match: size_t,
    mut prev_line_offset: size_t,
    sep: libc::c_char,
) {
    let mut column: size_t = 0;
    column = 0 as libc::c_int as size_t;
    if prev_line_offset <= (*matches.offset(last_printed_match as isize)).start {
        column = ((*matches.offset(last_printed_match as isize)).start)
            .wrapping_sub(prev_line_offset)
            .wrapping_add(1 as libc::c_ulong);
    }
    fprintf(
        out_fd,
        b"%lu%c\0" as *const u8 as *const libc::c_char,
        column,
        sep as libc::c_int,
    );
}
pub unsafe extern "C" fn print_file_separator() {
    if first_file_match == 0 as libc::c_int {
        if opts.print_break != 0 {
            fprintf(out_fd, b"\n\0" as *const u8 as *const libc::c_char);
        }
    }
    first_file_match = 0 as libc::c_int;
}
pub unsafe extern "C" fn normalize_path(
    mut path: *const libc::c_char,
) -> *const libc::c_char {
    let mut tmp: size_t = 0;
    tmp = strlen(path);
    if tmp < 3 as libc::c_ulong {
        return path;
    }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == 46 as libc::c_int {
        if *path.offset(1 as libc::c_int as isize) as libc::c_int == 47 as libc::c_int {
            return path.offset(2 as libc::c_int as isize);
        }
    }
    if *path.offset(0 as libc::c_int as isize) as libc::c_int == 47 as libc::c_int {
        if *path.offset(1 as libc::c_int as isize) as libc::c_int == 47 as libc::c_int {
            return path.offset(1 as libc::c_int as isize);
        }
    }
    return path;
}
pub unsafe extern "C" fn ag_scandir(
    mut dirname: *const libc::c_char,
    mut namelist: *mut *mut *mut dirent,
    mut filter: Option::<
        unsafe extern "C" fn(
            *const libc::c_char,
            *const dirent,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    mut baton: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut names: *mut *mut dirent = 0 as *mut *mut dirent;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut names_len: libc::c_int = 0;
    let mut results_len: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp_names: *mut *mut dirent = 0 as *mut *mut dirent;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    dirp = 0 as *mut libc::c_void as *mut DIR;
    names = 0 as *mut libc::c_void as *mut *mut dirent;
    names_len = 32 as libc::c_int;
    results_len = 0 as libc::c_int;
    dirp = opendir(dirname);
    if !(dirp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        tmp = malloc(
            (::std::mem::size_of::<*mut dirent>() as libc::c_ulong)
                .wrapping_mul(names_len as libc::c_ulong),
        );
        names = tmp as *mut *mut dirent;
        if !(names as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            loop {
                entry = readdir(dirp);
                if !(entry as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                    current_block = 15925075030174552612;
                    break;
                }
                tmp___0 = (Some(filter.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(dirname, entry as *const dirent, baton);
                if tmp___0 == 0 as libc::c_int {
                    continue;
                }
                if results_len >= names_len {
                    tmp_names = names;
                    names_len *= 2 as libc::c_int;
                    tmp___1 = realloc(
                        names as *mut libc::c_void,
                        (::std::mem::size_of::<*mut dirent>() as libc::c_ulong)
                            .wrapping_mul(names_len as libc::c_ulong),
                    );
                    names = tmp___1 as *mut *mut dirent;
                    if names as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
                    {
                        free(tmp_names as *mut libc::c_void);
                        current_block = 16220160324577990910;
                        break;
                    }
                }
                tmp___2 = malloc((*entry).d_reclen as size_t);
                d = tmp___2 as *mut dirent;
                if d as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    current_block = 16220160324577990910;
                    break;
                }
                memcpy(
                    d as *mut libc::c_void,
                    entry as *const libc::c_void,
                    (*entry).d_reclen as size_t,
                );
                let ref mut fresh9 = *names.offset(results_len as isize);
                *fresh9 = d;
                results_len += 1;
            }
            match current_block {
                16220160324577990910 => {}
                _ => {
                    closedir(dirp);
                    *namelist = names;
                    return results_len;
                }
            }
        }
    }
    if !dirp.is_null() {
        closedir(dirp);
    }
    if names as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        i = 0 as libc::c_int;
        while i < results_len {
            free(*names.offset(i as isize) as *mut libc::c_void);
            i += 1;
        }
        free(names as *mut libc::c_void);
    }
    return -(1 as libc::c_int);
}
pub static mut alpha_skip_lookup: [size_t; 256] = [0; 256];
pub static mut find_skip_lookup: *mut size_t = 0 as *const size_t as *mut size_t;
pub static mut h_table: [uint8_t; 65536] = [0; 65536];
pub static mut work_queue: *mut work_queue_t = 0 as *const libc::c_void
    as *mut libc::c_void as *mut work_queue_t;
pub static mut work_queue_tail: *mut work_queue_t = 0 as *const libc::c_void
    as *mut libc::c_void as *mut work_queue_t;
pub static mut done_adding_files: libc::c_int = 0 as libc::c_int;
pub static mut files_ready: pthread_cond_t = __anonunion_pthread_cond_t_951761805 {
    __data: {
        let mut init = __pthread_cond_s {
            __annonCompField1: __anonunion____missing_field_name_180959546 {
                __wseq: 0 as libc::c_ulonglong,
            },
            __annonCompField2: __anonunion____missing_field_name_575217030 {
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
pub static mut stats_mtx: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub static mut work_queue_mtx: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub static mut symhash: *mut symdir_t = 0 as *const libc::c_void as *mut libc::c_void
    as *mut symdir_t;
pub unsafe extern "C" fn search_buf(
    mut buf: *const libc::c_char,
    buf_len: size_t,
    mut dir_full_path: *const libc::c_char,
) -> ssize_t {
    let mut current_block: u64;
    let mut binary: libc::c_int = 0;
    let mut buf_offset: size_t = 0;
    let mut matches_len: size_t = 0;
    let mut matches: *mut match_t = 0 as *mut match_t;
    let mut matches_size: size_t = 0;
    let mut matches_spare: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut match_ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut offset_vector: [libc::c_int; 3] = [0; 3];
    let mut tmp___3: libc::c_int = 0;
    let mut line: *const libc::c_char = 0 as *const libc::c_char;
    let mut line_len: size_t = 0;
    let mut tmp___4: ssize_t = 0;
    let mut line_offset: size_t = 0;
    let mut rv: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut line_to_buf: size_t = 0;
    binary = -(1 as libc::c_int);
    buf_offset = 0 as libc::c_int as size_t;
    if opts.search_stream != 0 {
        binary = 0 as libc::c_int;
    } else if opts.search_binary_files == 0 {
        if opts.mmap != 0 {
            binary = is_binary(buf as *const libc::c_void, buf_len);
            if binary != 0 {
                log_debug(
                    b"File %s is binary. Skipping...\0" as *const u8
                        as *const libc::c_char,
                    dir_full_path,
                );
                return -(1 as libc::c_int) as ssize_t;
            }
        }
    }
    matches_len = 0 as libc::c_int as size_t;
    if opts.invert_match != 0 {
        matches_size = 100 as libc::c_int as size_t;
        tmp = ag_malloc(
            matches_size.wrapping_mul(::std::mem::size_of::<match_t>() as libc::c_ulong),
        );
        matches = tmp as *mut match_t;
        matches_spare = 1 as libc::c_int as size_t;
    } else {
        matches_size = 0 as libc::c_int as size_t;
        matches = 0 as *mut libc::c_void as *mut match_t;
        matches_spare = 0 as libc::c_int as size_t;
    }
    if opts.literal == 0 {
        if opts.query_len == 1 as libc::c_int {
            if *(opts.query).offset(0 as libc::c_int as isize) as libc::c_int
                == 46 as libc::c_int
            {
                matches_size = 1 as libc::c_int as size_t;
                if matches as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    tmp___0 = ag_malloc(
                        matches_size
                            .wrapping_mul(
                                ::std::mem::size_of::<match_t>() as libc::c_ulong,
                            ),
                    );
                    matches = tmp___0 as *mut match_t;
                } else {
                    matches = matches;
                }
                (*matches.offset(0 as libc::c_int as isize))
                    .start = 0 as libc::c_int as size_t;
                (*matches.offset(0 as libc::c_int as isize)).end = buf_len;
                matches_len = 1 as libc::c_int as size_t;
                current_block = 6161395505460082760;
            } else {
                current_block = 15805016273543580105;
            }
        } else {
            current_block = 15805016273543580105;
        }
    } else {
        current_block = 15805016273543580105;
    }
    match current_block {
        15805016273543580105 => {
            if opts.literal != 0 {
                match_ptr = buf;
                while buf_offset < buf_len {
                    if (opts.query_len as size_t)
                        < (2 as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                            )
                            .wrapping_sub(1 as libc::c_ulong)
                    {
                        match_ptr = boyer_moore_strnstr(
                            match_ptr,
                            opts.query as *const libc::c_char,
                            buf_len.wrapping_sub(buf_offset),
                            opts.query_len as size_t,
                            alpha_skip_lookup.as_mut_ptr() as *const size_t,
                            find_skip_lookup as *const size_t,
                            (opts.casing as libc::c_uint == 2 as libc::c_uint)
                                as libc::c_int,
                        );
                    } else if opts.query_len >= 255 as libc::c_int {
                        match_ptr = boyer_moore_strnstr(
                            match_ptr,
                            opts.query as *const libc::c_char,
                            buf_len.wrapping_sub(buf_offset),
                            opts.query_len as size_t,
                            alpha_skip_lookup.as_mut_ptr() as *const size_t,
                            find_skip_lookup as *const size_t,
                            (opts.casing as libc::c_uint == 2 as libc::c_uint)
                                as libc::c_int,
                        );
                    } else {
                        match_ptr = hash_strnstr(
                            match_ptr,
                            opts.query as *const libc::c_char,
                            buf_len.wrapping_sub(buf_offset),
                            opts.query_len as size_t,
                            h_table.as_mut_ptr(),
                            (opts.casing as libc::c_uint == 1 as libc::c_uint)
                                as libc::c_int,
                        );
                    }
                    if match_ptr as libc::c_ulong
                        == 0 as *mut libc::c_void as libc::c_ulong
                    {
                        break;
                    }
                    if opts.word_regexp != 0 {
                        start = match_ptr;
                        end = match_ptr.offset(opts.query_len as isize);
                        if !(start as libc::c_ulong == buf as libc::c_ulong) {
                            tmp___1 = is_wordchar(
                                *start.offset(-(1 as libc::c_int as isize)),
                            );
                            if !(tmp___1 != opts.literal_starts_wordchar) {
                                match_ptr = match_ptr
                                    .offset(
                                        (*find_skip_lookup.offset(0 as libc::c_int as isize))
                                            .wrapping_sub(opts.query_len as size_t)
                                            .wrapping_add(1 as libc::c_ulong) as isize,
                                    );
                                buf_offset = match_ptr.offset_from(buf) as libc::c_long
                                    as size_t;
                                continue;
                            }
                        }
                        if !(end as libc::c_ulong
                            == buf.offset(buf_len as isize) as libc::c_ulong)
                        {
                            tmp___2 = is_wordchar(*end);
                            if !(tmp___2 != opts.literal_ends_wordchar) {
                                match_ptr = match_ptr
                                    .offset(
                                        (*find_skip_lookup.offset(0 as libc::c_int as isize))
                                            .wrapping_sub(opts.query_len as size_t)
                                            .wrapping_add(1 as libc::c_ulong) as isize,
                                    );
                                buf_offset = match_ptr.offset_from(buf) as libc::c_long
                                    as size_t;
                                continue;
                            }
                        }
                    }
                    realloc_matches(
                        &mut matches,
                        &mut matches_size,
                        matches_len.wrapping_add(matches_spare),
                    );
                    (*matches.offset(matches_len as isize))
                        .start = match_ptr.offset_from(buf) as libc::c_long as size_t;
                    (*matches.offset(matches_len as isize))
                        .end = ((*matches.offset(matches_len as isize)).start)
                        .wrapping_add(opts.query_len as size_t);
                    buf_offset = (*matches.offset(matches_len as isize)).end;
                    log_debug(
                        b"Match found. File %s, offset %lu bytes.\0" as *const u8
                            as *const libc::c_char,
                        dir_full_path,
                        (*matches.offset(matches_len as isize)).start,
                    );
                    matches_len = matches_len.wrapping_add(1);
                    match_ptr = match_ptr.offset(opts.query_len as isize);
                    if !(opts.max_matches_per_file > 0 as libc::c_ulong) {
                        continue;
                    }
                    if !(matches_len >= opts.max_matches_per_file) {
                        continue;
                    }
                    log_err(
                        b"Too many matches in %s. Skipping the rest of this file.\0"
                            as *const u8 as *const libc::c_char,
                        dir_full_path,
                    );
                    break;
                }
            } else if opts.multiline != 0 {
                while buf_offset < buf_len {
                    tmp___3 = pcre_exec(
                        opts.re as *const pcre,
                        opts.re_extra as *const pcre_extra,
                        buf,
                        buf_len as libc::c_int,
                        buf_offset as libc::c_int,
                        0 as libc::c_int,
                        offset_vector.as_mut_ptr(),
                        3 as libc::c_int,
                    );
                    if !(tmp___3 >= 0 as libc::c_int) {
                        break;
                    }
                    log_debug(
                        b"Regex match found. File %s, offset %i bytes.\0" as *const u8
                            as *const libc::c_char,
                        dir_full_path,
                        offset_vector[0 as libc::c_int as usize],
                    );
                    buf_offset = offset_vector[1 as libc::c_int as usize] as size_t;
                    if offset_vector[0 as libc::c_int as usize]
                        == offset_vector[1 as libc::c_int as usize]
                    {
                        buf_offset = buf_offset.wrapping_add(1);
                        log_debug(
                            b"Regex match is of length zero. Advancing offset one byte.\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    realloc_matches(
                        &mut matches,
                        &mut matches_size,
                        matches_len.wrapping_add(matches_spare),
                    );
                    (*matches.offset(matches_len as isize))
                        .start = offset_vector[0 as libc::c_int as usize] as size_t;
                    (*matches.offset(matches_len as isize))
                        .end = offset_vector[1 as libc::c_int as usize] as size_t;
                    matches_len = matches_len.wrapping_add(1);
                    if !(opts.max_matches_per_file > 0 as libc::c_ulong) {
                        continue;
                    }
                    if !(matches_len >= opts.max_matches_per_file) {
                        continue;
                    }
                    log_err(
                        b"Too many matches in %s. Skipping the rest of this file.\0"
                            as *const u8 as *const libc::c_char,
                        dir_full_path,
                    );
                    break;
                }
            } else {
                's_455: while buf_offset < buf_len {
                    tmp___4 = buf_getline(&mut line, buf, buf_len, buf_offset);
                    line_len = tmp___4 as size_t;
                    if line.is_null() {
                        break;
                    }
                    line_offset = 0 as libc::c_int as size_t;
                    while line_offset < line_len {
                        tmp___5 = pcre_exec(
                            opts.re as *const pcre,
                            opts.re_extra as *const pcre_extra,
                            line,
                            line_len as libc::c_int,
                            line_offset as libc::c_int,
                            0 as libc::c_int,
                            offset_vector.as_mut_ptr(),
                            3 as libc::c_int,
                        );
                        rv = tmp___5;
                        if rv < 0 as libc::c_int {
                            break;
                        }
                        line_to_buf = buf_offset.wrapping_add(line_offset);
                        log_debug(
                            b"Regex match found. File %s, offset %i bytes.\0"
                                as *const u8 as *const libc::c_char,
                            dir_full_path,
                            offset_vector[0 as libc::c_int as usize],
                        );
                        line_offset = offset_vector[1 as libc::c_int as usize] as size_t;
                        if offset_vector[0 as libc::c_int as usize]
                            == offset_vector[1 as libc::c_int as usize]
                        {
                            line_offset = line_offset.wrapping_add(1);
                            log_debug(
                                b"Regex match is of length zero. Advancing offset one byte.\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        realloc_matches(
                            &mut matches,
                            &mut matches_size,
                            matches_len.wrapping_add(matches_spare),
                        );
                        (*matches.offset(matches_len as isize))
                            .start = (offset_vector[0 as libc::c_int as usize] as size_t)
                            .wrapping_add(line_to_buf);
                        (*matches.offset(matches_len as isize))
                            .end = (offset_vector[1 as libc::c_int as usize] as size_t)
                            .wrapping_add(line_to_buf);
                        matches_len = matches_len.wrapping_add(1);
                        if !(opts.max_matches_per_file > 0 as libc::c_ulong) {
                            continue;
                        }
                        if !(matches_len >= opts.max_matches_per_file) {
                            continue;
                        }
                        log_err(
                            b"Too many matches in %s. Skipping the rest of this file.\0"
                                as *const u8 as *const libc::c_char,
                            dir_full_path,
                        );
                        break 's_455;
                    }
                    buf_offset = (buf_offset as libc::c_ulong)
                        .wrapping_add(line_len.wrapping_add(1 as libc::c_ulong))
                        as size_t as size_t;
                }
            }
        }
        _ => {}
    }
    if opts.invert_match != 0 {
        matches_len = invert_matches(buf, buf_len, matches, matches_len);
    }
    if opts.stats != 0 {
        pthread_mutex_lock(&mut stats_mtx);
        stats
            .total_bytes = (stats.total_bytes as libc::c_ulong).wrapping_add(buf_len)
            as size_t as size_t;
        stats.total_files = (stats.total_files).wrapping_add(1);
        stats
            .total_matches = (stats.total_matches as libc::c_ulong)
            .wrapping_add(matches_len) as size_t as size_t;
        if matches_len > 0 as libc::c_ulong {
            stats.total_file_matches = (stats.total_file_matches).wrapping_add(1);
        }
        pthread_mutex_unlock(&mut stats_mtx);
    }
    let mut current_block_136: u64;
    if opts.print_nonmatching_files == 0 {
        if matches_len > 0 as libc::c_ulong {
            current_block_136 = 15982970901288780432;
        } else if opts.print_all_paths != 0 {
            current_block_136 = 15982970901288780432;
        } else {
            current_block_136 = 16407853667355338627;
        }
        match current_block_136 {
            16407853667355338627 => {}
            _ => {
                if binary == -(1 as libc::c_int) {
                    if opts.print_filename_only == 0 {
                        binary = is_binary(buf as *const libc::c_void, buf_len);
                    }
                }
                pthread_mutex_lock(&mut print_mtx);
                if opts.print_filename_only != 0 {
                    if opts.print_count != 0 {
                        print_path_count(dir_full_path, opts.path_sep, matches_len);
                    } else {
                        print_path(dir_full_path, opts.path_sep);
                    }
                } else if binary != 0 {
                    print_binary_file_matches(dir_full_path);
                } else {
                    print_file_matches(
                        dir_full_path,
                        buf,
                        buf_len,
                        matches as *const match_t,
                        matches_len,
                    );
                }
                pthread_mutex_unlock(&mut print_mtx);
                opts.match_found = 1 as libc::c_int;
                current_block_136 = 17855111796567036151;
            }
        }
    } else {
        current_block_136 = 16407853667355338627;
    }
    match current_block_136 {
        16407853667355338627 => {
            if opts.search_stream != 0 {
                if opts.passthrough != 0 {
                    fprintf(out_fd, b"%s\0" as *const u8 as *const libc::c_char, buf);
                } else {
                    log_debug(
                        b"No match in %s\0" as *const u8 as *const libc::c_char,
                        dir_full_path,
                    );
                }
            } else {
                log_debug(
                    b"No match in %s\0" as *const u8 as *const libc::c_char,
                    dir_full_path,
                );
            }
        }
        _ => {}
    }
    if matches_len == 0 as libc::c_ulong {
        if opts.search_stream != 0 {
            print_context_append(buf, buf_len.wrapping_sub(1 as libc::c_ulong));
        }
    }
    if matches_size > 0 as libc::c_ulong {
        free(matches as *mut libc::c_void);
    }
    return matches_len as ssize_t;
}
pub unsafe extern "C" fn search_stream(
    mut stream: *mut FILE,
    mut path: *const libc::c_char,
) -> ssize_t {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut matches_count: ssize_t = 0;
    let mut line_len: ssize_t = 0;
    let mut line_cap: size_t = 0;
    let mut i: size_t = 0;
    let mut result: ssize_t = 0;
    line = 0 as *mut libc::c_void as *mut libc::c_char;
    matches_count = 0 as libc::c_int as ssize_t;
    line_len = 0 as libc::c_int as ssize_t;
    line_cap = 0 as libc::c_int as size_t;
    print_init_context();
    i = 1 as libc::c_int as size_t;
    loop {
        line_len = getline(
            &mut line as *mut *mut libc::c_char,
            &mut line_cap as *mut size_t,
            stream,
        );
        if !(line_len > 0 as libc::c_long) {
            break;
        }
        opts.stream_line_num = i;
        result = search_buf(line as *const libc::c_char, line_len as size_t, path);
        if result > 0 as libc::c_long {
            if matches_count == -(1 as libc::c_long) {
                matches_count = 0 as libc::c_int as ssize_t;
            }
            matches_count += result;
        } else if matches_count <= 0 as libc::c_long {
            if result == -(1 as libc::c_long) {
                matches_count = -(1 as libc::c_int) as ssize_t;
            }
        }
        if *line.offset((line_len - 1 as libc::c_long) as isize) as libc::c_int
            == 10 as libc::c_int
        {
            line_len -= 1;
        }
        print_trailing_context(path, line as *const libc::c_char, line_len as size_t);
        i = i.wrapping_add(1);
    }
    free(line as *mut libc::c_void);
    print_cleanup_context();
    return matches_count;
}
pub unsafe extern "C" fn search_file(mut file_full_path: *const libc::c_char) {
    let mut current_block: u64;
    let mut fd: libc::c_int = 0;
    let mut f_len: off_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut rv: libc::c_int = 0;
    let mut matches_count: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: ssize_t = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bytes_read: ssize_t = 0;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: ssize_t = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: ssize_t = 0;
    let mut zip_type: ag_compression_type = AG_NO_COMPRESSION;
    let mut tmp___11: ag_compression_type = AG_NO_COMPRESSION;
    let mut tmp___12: ssize_t = 0;
    let mut tmp___13: ssize_t = 0;
    fd = -(1 as libc::c_int);
    f_len = 0 as libc::c_int as off_t;
    buf = 0 as *mut libc::c_void as *mut libc::c_char;
    rv = 0 as libc::c_int;
    matches_count = -(1 as libc::c_int);
    fp = 0 as *mut libc::c_void as *mut FILE;
    rv = stat(file_full_path, &mut statbuf as *mut stat);
    if rv != 0 as libc::c_int {
        log_err(
            b"Skipping %s: Error fstat()ing file.\0" as *const u8 as *const libc::c_char,
            file_full_path,
        );
    } else {
        if opts.stdout_inode != 0 as libc::c_ulong {
            if opts.stdout_inode == statbuf.st_ino {
                log_debug(
                    b"Skipping %s: stdout is redirected to it\0" as *const u8
                        as *const libc::c_char,
                    file_full_path,
                );
                current_block = 9311304539129630422;
            } else {
                current_block = 13242334135786603907;
            }
        } else {
            current_block = 13242334135786603907;
        }
        match current_block {
            9311304539129630422 => {}
            _ => {
                if !(statbuf.st_mode & 61440 as libc::c_uint == 32768 as libc::c_uint) {
                    if !(statbuf.st_mode & 61440 as libc::c_uint == 4096 as libc::c_uint)
                    {
                        log_err(
                            b"Skipping %s: Mode %u is not a file.\0" as *const u8
                                as *const libc::c_char,
                            file_full_path,
                            statbuf.st_mode,
                        );
                        current_block = 9311304539129630422;
                    } else {
                        current_block = 14576567515993809846;
                    }
                } else {
                    current_block = 14576567515993809846;
                }
                match current_block {
                    9311304539129630422 => {}
                    _ => {
                        fd = open(file_full_path, 0 as libc::c_int);
                        if fd < 0 as libc::c_int {
                            tmp = __errno_location();
                            tmp___0 = strerror(*tmp);
                            log_err(
                                b"Skipping %s: Error opening file: %s\0" as *const u8
                                    as *const libc::c_char,
                                file_full_path,
                                tmp___0,
                            );
                        } else {
                            rv = fstat(fd, &mut statbuf);
                            if rv != 0 as libc::c_int {
                                log_err(
                                    b"Skipping %s: Error fstat()ing file.\0" as *const u8
                                        as *const libc::c_char,
                                    file_full_path,
                                );
                            } else {
                                if opts.stdout_inode != 0 as libc::c_ulong {
                                    if opts.stdout_inode == statbuf.st_ino {
                                        log_debug(
                                            b"Skipping %s: stdout is redirected to it\0" as *const u8
                                                as *const libc::c_char,
                                            file_full_path,
                                        );
                                        current_block = 9311304539129630422;
                                    } else {
                                        current_block = 3275366147856559585;
                                    }
                                } else {
                                    current_block = 3275366147856559585;
                                }
                                match current_block {
                                    9311304539129630422 => {}
                                    _ => {
                                        if !(statbuf.st_mode & 61440 as libc::c_uint
                                            == 32768 as libc::c_uint)
                                        {
                                            if !(statbuf.st_mode & 61440 as libc::c_uint
                                                == 4096 as libc::c_uint)
                                            {
                                                log_err(
                                                    b"Skipping %s: Mode %u is not a file.\0" as *const u8
                                                        as *const libc::c_char,
                                                    file_full_path,
                                                    statbuf.st_mode,
                                                );
                                                current_block = 9311304539129630422;
                                            } else {
                                                current_block = 17184638872671510253;
                                            }
                                        } else {
                                            current_block = 17184638872671510253;
                                        }
                                        match current_block {
                                            9311304539129630422 => {}
                                            _ => {
                                                print_init_context();
                                                if statbuf.st_mode & 4096 as libc::c_uint != 0 {
                                                    log_debug(
                                                        b"%s is a named pipe. stream searching\0" as *const u8
                                                            as *const libc::c_char,
                                                        file_full_path,
                                                    );
                                                    fp = fdopen(fd, b"r\0" as *const u8 as *const libc::c_char);
                                                    tmp___1 = search_stream(fp, file_full_path);
                                                    matches_count = tmp___1 as libc::c_int;
                                                    fclose(fp);
                                                } else {
                                                    f_len = statbuf.st_size;
                                                    if f_len == 0 as libc::c_long {
                                                        if *(opts.query).offset(0 as libc::c_int as isize)
                                                            as libc::c_int == 46 as libc::c_int
                                                        {
                                                            if opts.query_len == 1 as libc::c_int {
                                                                if opts.literal == 0 {
                                                                    if opts.search_all_files != 0 {
                                                                        tmp___2 = search_buf(
                                                                            buf as *const libc::c_char,
                                                                            f_len as size_t,
                                                                            file_full_path,
                                                                        );
                                                                        matches_count = tmp___2 as libc::c_int;
                                                                    } else {
                                                                        log_debug(
                                                                            b"Skipping %s: file is empty.\0" as *const u8
                                                                                as *const libc::c_char,
                                                                            file_full_path,
                                                                        );
                                                                    }
                                                                } else {
                                                                    log_debug(
                                                                        b"Skipping %s: file is empty.\0" as *const u8
                                                                            as *const libc::c_char,
                                                                        file_full_path,
                                                                    );
                                                                }
                                                            } else {
                                                                log_debug(
                                                                    b"Skipping %s: file is empty.\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    file_full_path,
                                                                );
                                                            }
                                                        } else {
                                                            log_debug(
                                                                b"Skipping %s: file is empty.\0" as *const u8
                                                                    as *const libc::c_char,
                                                                file_full_path,
                                                            );
                                                        }
                                                    } else {
                                                        if opts.literal == 0 {
                                                            if f_len > 2147483647 as libc::c_long {
                                                                log_err(
                                                                    b"Skipping %s: pcre_exec() can't handle files larger than %i bytes.\0"
                                                                        as *const u8 as *const libc::c_char,
                                                                    file_full_path,
                                                                    2147483647 as libc::c_int,
                                                                );
                                                                current_block = 9311304539129630422;
                                                            } else {
                                                                current_block = 7252614138838059896;
                                                            }
                                                        } else {
                                                            current_block = 7252614138838059896;
                                                        }
                                                        match current_block {
                                                            9311304539129630422 => {}
                                                            _ => {
                                                                if opts.mmap != 0 {
                                                                    tmp___3 = mmap(
                                                                        0 as *mut libc::c_void,
                                                                        f_len as size_t,
                                                                        1 as libc::c_int,
                                                                        2 as libc::c_int,
                                                                        fd,
                                                                        0 as libc::c_int as __off_t,
                                                                    );
                                                                    buf = tmp___3 as *mut libc::c_char;
                                                                    if buf as libc::c_ulong
                                                                        == -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong
                                                                    {
                                                                        tmp___4 = __errno_location();
                                                                        tmp___5 = strerror(*tmp___4);
                                                                        log_err(
                                                                            b"File %s failed to load: %s.\0" as *const u8
                                                                                as *const libc::c_char,
                                                                            file_full_path,
                                                                            tmp___5,
                                                                        );
                                                                        current_block = 9311304539129630422;
                                                                    } else {
                                                                        madvise(
                                                                            buf as *mut libc::c_void,
                                                                            f_len as size_t,
                                                                            2 as libc::c_int,
                                                                        );
                                                                        current_block = 2472048668343472511;
                                                                    }
                                                                } else {
                                                                    tmp___6 = ag_malloc(f_len as size_t);
                                                                    buf = tmp___6 as *mut libc::c_char;
                                                                    bytes_read = 0 as libc::c_int as ssize_t;
                                                                    if opts.search_binary_files == 0 {
                                                                        tmp___7 = ag_min(
                                                                            f_len as size_t,
                                                                            512 as libc::c_int as size_t,
                                                                        );
                                                                        tmp___8 = read(fd, buf as *mut libc::c_void, tmp___7);
                                                                        bytes_read += tmp___8;
                                                                        tmp___9 = is_binary(
                                                                            buf as *const libc::c_void,
                                                                            f_len as size_t,
                                                                        );
                                                                        if tmp___9 != 0 {
                                                                            log_debug(
                                                                                b"File %s is binary. Skipping...\0" as *const u8
                                                                                    as *const libc::c_char,
                                                                                file_full_path,
                                                                            );
                                                                            current_block = 9311304539129630422;
                                                                        } else {
                                                                            current_block = 17515716450947708786;
                                                                        }
                                                                    } else {
                                                                        current_block = 17515716450947708786;
                                                                    }
                                                                    match current_block {
                                                                        9311304539129630422 => {}
                                                                        _ => {
                                                                            while bytes_read < f_len {
                                                                                tmp___10 = read(
                                                                                    fd,
                                                                                    buf.offset(bytes_read as isize) as *mut libc::c_void,
                                                                                    f_len as size_t,
                                                                                );
                                                                                bytes_read += tmp___10;
                                                                            }
                                                                            if bytes_read != f_len {
                                                                                die(
                                                                                    b"File %s read(): expected to read %u bytes but read %u\0"
                                                                                        as *const u8 as *const libc::c_char,
                                                                                    file_full_path,
                                                                                    f_len,
                                                                                    bytes_read,
                                                                                );
                                                                            }
                                                                            current_block = 2472048668343472511;
                                                                        }
                                                                    }
                                                                }
                                                                match current_block {
                                                                    9311304539129630422 => {}
                                                                    _ => {
                                                                        if opts.search_zip_files != 0 {
                                                                            tmp___11 = is_zipped(
                                                                                buf as *const libc::c_void,
                                                                                f_len as libc::c_int,
                                                                            );
                                                                            zip_type = tmp___11;
                                                                            if zip_type as libc::c_uint != 0 as libc::c_uint {
                                                                                log_debug(
                                                                                    b"%s is a compressed file. stream searching\0" as *const u8
                                                                                        as *const libc::c_char,
                                                                                    file_full_path,
                                                                                );
                                                                                fp = decompress_open(
                                                                                    fd,
                                                                                    b"r\0" as *const u8 as *const libc::c_char,
                                                                                    zip_type,
                                                                                );
                                                                                tmp___12 = search_stream(fp, file_full_path);
                                                                                matches_count = tmp___12 as libc::c_int;
                                                                                fclose(fp);
                                                                                current_block = 9311304539129630422;
                                                                            } else {
                                                                                current_block = 200744462051969938;
                                                                            }
                                                                        } else {
                                                                            current_block = 200744462051969938;
                                                                        }
                                                                        match current_block {
                                                                            9311304539129630422 => {}
                                                                            _ => {
                                                                                tmp___13 = search_buf(
                                                                                    buf as *const libc::c_char,
                                                                                    f_len as size_t,
                                                                                    file_full_path,
                                                                                );
                                                                                matches_count = tmp___13 as libc::c_int;
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
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if opts.print_nonmatching_files != 0 {
        if matches_count == 0 as libc::c_int {
            pthread_mutex_lock(&mut print_mtx);
            print_path(file_full_path, opts.path_sep);
            pthread_mutex_unlock(&mut print_mtx);
            opts.match_found = 1 as libc::c_int;
        }
    }
    print_cleanup_context();
    if buf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if opts.mmap != 0 {
            if buf as libc::c_ulong
                != -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong
            {
                munmap(buf as *mut libc::c_void, f_len as size_t);
            }
        } else {
            free(buf as *mut libc::c_void);
        }
    }
    if fd != -(1 as libc::c_int) {
        close(fd);
    }
}
pub unsafe extern "C" fn search_file_worker(
    mut i: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut queue_item: *mut work_queue_t = 0 as *mut work_queue_t;
    let mut worker_id: libc::c_int = 0;
    worker_id = *(i as *mut libc::c_int);
    log_debug(b"Worker %i started\0" as *const u8 as *const libc::c_char, worker_id);
    loop {
        pthread_mutex_lock(&mut work_queue_mtx);
        while work_queue as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            if done_adding_files != 0 {
                pthread_mutex_unlock(&mut work_queue_mtx);
                log_debug(
                    b"Worker %i finished.\0" as *const u8 as *const libc::c_char,
                    worker_id,
                );
                pthread_exit(0 as *mut libc::c_void);
            }
            pthread_cond_wait(
                &mut files_ready as *mut pthread_cond_t,
                &mut work_queue_mtx as *mut pthread_mutex_t,
            );
        }
        queue_item = work_queue;
        work_queue = (*work_queue).next;
        if work_queue as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            work_queue_tail = 0 as *mut libc::c_void as *mut work_queue_t;
        }
        pthread_mutex_unlock(&mut work_queue_mtx);
        search_file((*queue_item).path as *const libc::c_char);
        free((*queue_item).path as *mut libc::c_void);
        free(queue_item as *mut libc::c_void);
    };
}
unsafe extern "C" fn check_symloop_enter(
    mut path: *const libc::c_char,
    mut outkey: *mut dirkey_t,
) -> libc::c_int {
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut item_found: *mut symdir_t = 0 as *mut symdir_t;
    let mut new_item: *mut symdir_t = 0 as *mut symdir_t;
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut _hf_bkt: libc::c_uint = 0;
    let mut _hf_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _ha_bkt: libc::c_uint = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _hj_i___0: libc::c_uint = 0;
    let mut _hj_j___0: libc::c_uint = 0;
    let mut _hj_k___0: libc::c_uint = 0;
    let mut _hj_key___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut _he_bkt: libc::c_uint = 0;
    let mut _he_bkt_i: libc::c_uint = 0;
    let mut _he_thh: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _he_hh_nxt: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    let mut _he_new_buckets: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
    let mut _he_newbkt: *mut UT_hash_bucket = 0 as *mut UT_hash_bucket;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: libc::c_int = 0;
    item_found = 0 as *mut libc::c_void as *mut symdir_t;
    new_item = 0 as *mut libc::c_void as *mut symdir_t;
    memset(
        outkey as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<dirkey_t>() as libc::c_ulong,
    );
    (*outkey).dev = 0 as libc::c_int as dev_t;
    (*outkey).ino = 0 as libc::c_int as ino_t;
    tmp = stat(path, &mut buf as *mut stat);
    res = tmp;
    if res != 0 as libc::c_int {
        log_err(b"Error stat()ing: %s\0" as *const u8 as *const libc::c_char, path);
        return -(1 as libc::c_int);
    }
    (*outkey).dev = buf.st_dev;
    (*outkey).ino = buf.st_ino;
    item_found = 0 as *mut libc::c_void as *mut symdir_t;
    if !symhash.is_null() {
        _hj_key = outkey as *mut libc::c_uchar;
        _hf_hashv = 4276993775 as libc::c_uint;
        _hj_j = 2654435769 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::std::mem::size_of::<dirkey_t>() as libc::c_ulong as libc::c_uint;
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
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
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
        _hf_hashv = (_hf_hashv as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<dirkey_t>() as libc::c_ulong)
            as libc::c_uint;
        let mut current_block_65: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_65 = 16725018551601367397;
            }
            10 => {
                current_block_65 = 16725018551601367397;
            }
            9 => {
                current_block_65 = 16821974069940322511;
            }
            8 => {
                current_block_65 = 15626069058652981950;
            }
            7 => {
                current_block_65 = 7710529213155860982;
            }
            6 => {
                current_block_65 = 15751939872866650334;
            }
            5 => {
                current_block_65 = 10596832668479144727;
            }
            4 => {
                current_block_65 = 2888307544307979640;
            }
            3 => {
                current_block_65 = 15683981092457289569;
            }
            2 => {
                current_block_65 = 10535621665167309869;
            }
            1 => {
                current_block_65 = 15853467134859434658;
            }
            _ => {
                current_block_65 = 8835654301469918283;
            }
        }
        match current_block_65 {
            16725018551601367397 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_65 = 16821974069940322511;
            }
            _ => {}
        }
        match current_block_65 {
            16821974069940322511 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_65 = 15626069058652981950;
            }
            _ => {}
        }
        match current_block_65 {
            15626069058652981950 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_65 = 7710529213155860982;
            }
            _ => {}
        }
        match current_block_65 {
            7710529213155860982 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_65 = 15751939872866650334;
            }
            _ => {}
        }
        match current_block_65 {
            15751939872866650334 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_65 = 10596832668479144727;
            }
            _ => {}
        }
        match current_block_65 {
            10596832668479144727 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_65 = 2888307544307979640;
            }
            _ => {}
        }
        match current_block_65 {
            2888307544307979640 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_65 = 15683981092457289569;
            }
            _ => {}
        }
        match current_block_65 {
            15683981092457289569 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_65 = 10535621665167309869;
            }
            _ => {}
        }
        match current_block_65 {
            10535621665167309869 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_65 = 15853467134859434658;
            }
            _ => {}
        }
        match current_block_65 {
            15853467134859434658 => {
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
        _hf_bkt = _hf_hashv
            & ((*(*symhash).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if !((*((*(*symhash).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
            .is_null()
        {
            item_found = ((*((*(*symhash).hh.tbl).buckets).offset(_hf_bkt as isize))
                .hh_head as *mut libc::c_char)
                .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void
                as *mut symdir_t;
        } else {
            item_found = 0 as *mut libc::c_void as *mut symdir_t;
        }
        while !item_found.is_null() {
            if (*item_found).hh.keylen as libc::c_ulong
                == ::std::mem::size_of::<dirkey_t>() as libc::c_ulong
            {
                tmp___0 = memcmp(
                    (*item_found).hh.key as *const libc::c_void,
                    outkey as *const libc::c_void,
                    ::std::mem::size_of::<dirkey_t>() as libc::c_ulong,
                );
                if tmp___0 == 0 as libc::c_int {
                    break;
                }
            }
            if !((*item_found).hh.hh_next).is_null() {
                item_found = ((*item_found).hh.hh_next as *mut libc::c_char)
                    .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut symdir_t;
            } else {
                item_found = 0 as *mut libc::c_void as *mut symdir_t;
            }
        }
    }
    if !item_found.is_null() {
        return 1 as libc::c_int;
    }
    tmp___1 = ag_malloc(::std::mem::size_of::<symdir_t>() as libc::c_ulong);
    new_item = tmp___1 as *mut symdir_t;
    memcpy(
        &mut (*new_item).key as *mut dirkey_t as *mut libc::c_void,
        outkey as *const libc::c_void,
        ::std::mem::size_of::<dirkey_t>() as libc::c_ulong,
    );
    (*new_item).hh.next = 0 as *mut libc::c_void;
    (*new_item)
        .hh
        .key = &mut (*new_item).key as *mut dirkey_t as *mut libc::c_char
        as *mut libc::c_void;
    (*new_item)
        .hh
        .keylen = ::std::mem::size_of::<dirkey_t>() as libc::c_ulong as libc::c_uint;
    if symhash.is_null() {
        symhash = new_item;
        (*symhash).hh.prev = 0 as *mut libc::c_void;
        tmp___2 = malloc(::std::mem::size_of::<UT_hash_table>() as libc::c_ulong);
        (*symhash).hh.tbl = tmp___2 as *mut UT_hash_table;
        if ((*symhash).hh.tbl).is_null() {
            exit(-(1 as libc::c_int));
        }
        memset(
            (*symhash).hh.tbl as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<UT_hash_table>() as libc::c_ulong,
        );
        (*(*symhash).hh.tbl).tail = &mut (*symhash).hh;
        (*(*symhash).hh.tbl).num_buckets = 32 as libc::c_uint;
        (*(*symhash).hh.tbl).log2_num_buckets = 5 as libc::c_uint;
        (*(*symhash).hh.tbl)
            .hho = (&mut (*symhash).hh as *mut UT_hash_handle as *mut libc::c_char)
            .offset_from(symhash as *mut libc::c_char) as libc::c_long;
        tmp___3 = malloc(
            (32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong),
        );
        (*(*symhash).hh.tbl).buckets = tmp___3 as *mut UT_hash_bucket;
        if ((*(*symhash).hh.tbl).buckets).is_null() {
            exit(-(1 as libc::c_int));
        }
        memset(
            (*(*symhash).hh.tbl).buckets as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong),
        );
        (*(*symhash).hh.tbl).signature = 2685476833 as libc::c_uint;
    } else {
        (*(*(*symhash).hh.tbl).tail).next = new_item as *mut libc::c_void;
        (*new_item)
            .hh
            .prev = ((*(*symhash).hh.tbl).tail as *mut libc::c_char)
            .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void;
        (*(*symhash).hh.tbl).tail = &mut (*new_item).hh;
    }
    (*(*symhash).hh.tbl).num_items = ((*(*symhash).hh.tbl).num_items).wrapping_add(1);
    (*new_item).hh.tbl = (*symhash).hh.tbl;
    _hj_key___0 = &mut (*new_item).key as *mut dirkey_t as *mut libc::c_uchar;
    (*new_item).hh.hashv = 4276993775 as libc::c_uint;
    _hj_j___0 = 2654435769 as libc::c_uint;
    _hj_i___0 = _hj_j___0;
    _hj_k___0 = ::std::mem::size_of::<dirkey_t>() as libc::c_ulong as libc::c_uint;
    while _hj_k___0 >= 12 as libc::c_uint {
        _hj_i___0 = _hj_i___0
            .wrapping_add(
                (*_hj_key___0.offset(0 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key___0.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key___0.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key___0.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_j___0 = _hj_j___0
            .wrapping_add(
                (*_hj_key___0.offset(4 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key___0.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key___0.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key___0.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        (*new_item)
            .hh
            .hashv = ((*new_item).hh.hashv)
            .wrapping_add(
                (*_hj_key___0.offset(8 as libc::c_int as isize) as libc::c_uint)
                    .wrapping_add(
                        (*_hj_key___0.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key___0.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    )
                    .wrapping_add(
                        (*_hj_key___0.offset(11 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    ),
            );
        _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
        _hj_i___0 = _hj_i___0.wrapping_sub((*new_item).hh.hashv);
        _hj_i___0 ^= (*new_item).hh.hashv >> 13 as libc::c_int;
        _hj_j___0 = _hj_j___0.wrapping_sub((*new_item).hh.hashv);
        _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
        _hj_j___0 ^= _hj_i___0 << 8 as libc::c_int;
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i___0);
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j___0);
        (*new_item).hh.hashv ^= _hj_j___0 >> 13 as libc::c_int;
        _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
        _hj_i___0 = _hj_i___0.wrapping_sub((*new_item).hh.hashv);
        _hj_i___0 ^= (*new_item).hh.hashv >> 12 as libc::c_int;
        _hj_j___0 = _hj_j___0.wrapping_sub((*new_item).hh.hashv);
        _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
        _hj_j___0 ^= _hj_i___0 << 16 as libc::c_int;
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i___0);
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j___0);
        (*new_item).hh.hashv ^= _hj_j___0 >> 5 as libc::c_int;
        _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
        _hj_i___0 = _hj_i___0.wrapping_sub((*new_item).hh.hashv);
        _hj_i___0 ^= (*new_item).hh.hashv >> 3 as libc::c_int;
        _hj_j___0 = _hj_j___0.wrapping_sub((*new_item).hh.hashv);
        _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
        _hj_j___0 ^= _hj_i___0 << 10 as libc::c_int;
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i___0);
        (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j___0);
        (*new_item).hh.hashv ^= _hj_j___0 >> 15 as libc::c_int;
        _hj_key___0 = _hj_key___0.offset(12 as libc::c_int as isize);
        _hj_k___0 = _hj_k___0.wrapping_sub(12 as libc::c_uint);
    }
    (*new_item)
        .hh
        .hashv = ((*new_item).hh.hashv as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<dirkey_t>() as libc::c_ulong)
        as libc::c_uint;
    let mut current_block_203: u64;
    match _hj_k___0 {
        11 => {
            (*new_item)
                .hh
                .hashv = ((*new_item).hh.hashv)
                .wrapping_add(
                    (*_hj_key___0.offset(10 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_203 = 9553437138514998037;
        }
        10 => {
            current_block_203 = 9553437138514998037;
        }
        9 => {
            current_block_203 = 4425087070888421147;
        }
        8 => {
            current_block_203 = 4044299550321390264;
        }
        7 => {
            current_block_203 = 9610032191060292455;
        }
        6 => {
            current_block_203 = 14445371019284752096;
        }
        5 => {
            current_block_203 = 8322037711515268147;
        }
        4 => {
            current_block_203 = 8557604996611822219;
        }
        3 => {
            current_block_203 = 18104536936973052544;
        }
        2 => {
            current_block_203 = 8970869157841161036;
        }
        1 => {
            current_block_203 = 15119158973217614262;
        }
        _ => {
            current_block_203 = 3297745280902459415;
        }
    }
    match current_block_203 {
        9553437138514998037 => {
            (*new_item)
                .hh
                .hashv = ((*new_item).hh.hashv)
                .wrapping_add(
                    (*_hj_key___0.offset(9 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_203 = 4425087070888421147;
        }
        _ => {}
    }
    match current_block_203 {
        4425087070888421147 => {
            (*new_item)
                .hh
                .hashv = ((*new_item).hh.hashv)
                .wrapping_add(
                    (*_hj_key___0.offset(8 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_203 = 4044299550321390264;
        }
        _ => {}
    }
    match current_block_203 {
        4044299550321390264 => {
            _hj_j___0 = _hj_j___0
                .wrapping_add(
                    (*_hj_key___0.offset(7 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_203 = 9610032191060292455;
        }
        _ => {}
    }
    match current_block_203 {
        9610032191060292455 => {
            _hj_j___0 = _hj_j___0
                .wrapping_add(
                    (*_hj_key___0.offset(6 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_203 = 14445371019284752096;
        }
        _ => {}
    }
    match current_block_203 {
        14445371019284752096 => {
            _hj_j___0 = _hj_j___0
                .wrapping_add(
                    (*_hj_key___0.offset(5 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_203 = 8322037711515268147;
        }
        _ => {}
    }
    match current_block_203 {
        8322037711515268147 => {
            _hj_j___0 = _hj_j___0
                .wrapping_add(
                    *_hj_key___0.offset(4 as libc::c_int as isize) as libc::c_uint,
                );
            current_block_203 = 8557604996611822219;
        }
        _ => {}
    }
    match current_block_203 {
        8557604996611822219 => {
            _hj_i___0 = _hj_i___0
                .wrapping_add(
                    (*_hj_key___0.offset(3 as libc::c_int as isize) as libc::c_uint)
                        << 24 as libc::c_int,
                );
            current_block_203 = 18104536936973052544;
        }
        _ => {}
    }
    match current_block_203 {
        18104536936973052544 => {
            _hj_i___0 = _hj_i___0
                .wrapping_add(
                    (*_hj_key___0.offset(2 as libc::c_int as isize) as libc::c_uint)
                        << 16 as libc::c_int,
                );
            current_block_203 = 8970869157841161036;
        }
        _ => {}
    }
    match current_block_203 {
        8970869157841161036 => {
            _hj_i___0 = _hj_i___0
                .wrapping_add(
                    (*_hj_key___0.offset(1 as libc::c_int as isize) as libc::c_uint)
                        << 8 as libc::c_int,
                );
            current_block_203 = 15119158973217614262;
        }
        _ => {}
    }
    match current_block_203 {
        15119158973217614262 => {
            _hj_i___0 = _hj_i___0
                .wrapping_add(
                    *_hj_key___0.offset(0 as libc::c_int as isize) as libc::c_uint,
                );
        }
        _ => {}
    }
    _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
    _hj_i___0 = _hj_i___0.wrapping_sub((*new_item).hh.hashv);
    _hj_i___0 ^= (*new_item).hh.hashv >> 13 as libc::c_int;
    _hj_j___0 = _hj_j___0.wrapping_sub((*new_item).hh.hashv);
    _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
    _hj_j___0 ^= _hj_i___0 << 8 as libc::c_int;
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i___0);
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j___0);
    (*new_item).hh.hashv ^= _hj_j___0 >> 13 as libc::c_int;
    _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
    _hj_i___0 = _hj_i___0.wrapping_sub((*new_item).hh.hashv);
    _hj_i___0 ^= (*new_item).hh.hashv >> 12 as libc::c_int;
    _hj_j___0 = _hj_j___0.wrapping_sub((*new_item).hh.hashv);
    _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
    _hj_j___0 ^= _hj_i___0 << 16 as libc::c_int;
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i___0);
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j___0);
    (*new_item).hh.hashv ^= _hj_j___0 >> 5 as libc::c_int;
    _hj_i___0 = _hj_i___0.wrapping_sub(_hj_j___0);
    _hj_i___0 = _hj_i___0.wrapping_sub((*new_item).hh.hashv);
    _hj_i___0 ^= (*new_item).hh.hashv >> 3 as libc::c_int;
    _hj_j___0 = _hj_j___0.wrapping_sub((*new_item).hh.hashv);
    _hj_j___0 = _hj_j___0.wrapping_sub(_hj_i___0);
    _hj_j___0 ^= _hj_i___0 << 10 as libc::c_int;
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_i___0);
    (*new_item).hh.hashv = ((*new_item).hh.hashv).wrapping_sub(_hj_j___0);
    (*new_item).hh.hashv ^= _hj_j___0 >> 15 as libc::c_int;
    _ha_bkt = (*new_item).hh.hashv
        & ((*(*symhash).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
    let ref mut fresh10 = (*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize))
        .count;
    *fresh10 = (*fresh10).wrapping_add(1);
    (*new_item)
        .hh
        .hh_next = (*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize)).hh_head;
    (*new_item).hh.hh_prev = 0 as *mut libc::c_void as *mut UT_hash_handle;
    if !((*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize)).hh_head).is_null() {
        let ref mut fresh11 = (*(*((*(*symhash).hh.tbl).buckets)
            .offset(_ha_bkt as isize))
            .hh_head)
            .hh_prev;
        *fresh11 = &mut (*new_item).hh;
    }
    let ref mut fresh12 = (*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize))
        .hh_head;
    *fresh12 = &mut (*new_item).hh;
    if (*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize)).count
        >= ((*((*(*symhash).hh.tbl).buckets).offset(_ha_bkt as isize)).expand_mult)
            .wrapping_add(1 as libc::c_uint)
            .wrapping_mul(10 as libc::c_uint)
    {
        if (*(*new_item).hh.tbl).noexpand != 1 as libc::c_uint {
            tmp___4 = malloc(
                ((2 as libc::c_uint).wrapping_mul((*(*new_item).hh.tbl).num_buckets)
                    as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            );
            _he_new_buckets = tmp___4 as *mut UT_hash_bucket;
            if _he_new_buckets.is_null() {
                exit(-(1 as libc::c_int));
            }
            memset(
                _he_new_buckets as *mut libc::c_void,
                0 as libc::c_int,
                ((2 as libc::c_uint).wrapping_mul((*(*new_item).hh.tbl).num_buckets)
                    as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<UT_hash_bucket>() as libc::c_ulong,
                    ),
            );
            if (*(*new_item).hh.tbl).num_items
                & ((*(*new_item).hh.tbl).num_buckets)
                    .wrapping_mul(2 as libc::c_uint)
                    .wrapping_sub(1 as libc::c_uint) != 0
            {
                tmp___5 = 1 as libc::c_int;
            } else {
                tmp___5 = 0 as libc::c_int;
            }
            (*(*new_item).hh.tbl)
                .ideal_chain_maxlen = ((*(*new_item).hh.tbl).num_items
                >> ((*(*new_item).hh.tbl).log2_num_buckets)
                    .wrapping_add(1 as libc::c_uint))
                .wrapping_add(tmp___5 as libc::c_uint);
            (*(*new_item).hh.tbl).nonideal_items = 0 as libc::c_uint;
            _he_bkt_i = 0 as libc::c_uint;
            while _he_bkt_i < (*(*new_item).hh.tbl).num_buckets {
                _he_thh = (*((*(*new_item).hh.tbl).buckets).offset(_he_bkt_i as isize))
                    .hh_head;
                while !_he_thh.is_null() {
                    _he_hh_nxt = (*_he_thh).hh_next;
                    _he_bkt = (*_he_thh).hashv
                        & ((*(*new_item).hh.tbl).num_buckets)
                            .wrapping_mul(2 as libc::c_uint)
                            .wrapping_sub(1 as libc::c_uint);
                    _he_newbkt = _he_new_buckets.offset(_he_bkt as isize);
                    (*_he_newbkt).count = ((*_he_newbkt).count).wrapping_add(1);
                    if (*_he_newbkt).count > (*(*new_item).hh.tbl).ideal_chain_maxlen {
                        (*(*new_item).hh.tbl)
                            .nonideal_items = ((*(*new_item).hh.tbl).nonideal_items)
                            .wrapping_add(1);
                        (*_he_newbkt)
                            .expand_mult = ((*_he_newbkt).count)
                            .wrapping_div((*(*new_item).hh.tbl).ideal_chain_maxlen);
                    }
                    (*_he_thh).hh_prev = 0 as *mut libc::c_void as *mut UT_hash_handle;
                    (*_he_thh).hh_next = (*_he_newbkt).hh_head;
                    if !((*_he_newbkt).hh_head).is_null() {
                        (*(*_he_newbkt).hh_head).hh_prev = _he_thh;
                    }
                    (*_he_newbkt).hh_head = _he_thh;
                    _he_thh = _he_hh_nxt;
                }
                _he_bkt_i = _he_bkt_i.wrapping_add(1);
            }
            free((*(*new_item).hh.tbl).buckets as *mut libc::c_void);
            (*(*new_item).hh.tbl)
                .num_buckets = ((*(*new_item).hh.tbl).num_buckets)
                .wrapping_mul(2 as libc::c_uint);
            (*(*new_item).hh.tbl)
                .log2_num_buckets = ((*(*new_item).hh.tbl).log2_num_buckets)
                .wrapping_add(1);
            (*(*new_item).hh.tbl).buckets = _he_new_buckets;
            if (*(*new_item).hh.tbl).nonideal_items
                > (*(*new_item).hh.tbl).num_items >> 1 as libc::c_int
            {
                (*(*new_item).hh.tbl)
                    .ineff_expands = ((*(*new_item).hh.tbl).ineff_expands)
                    .wrapping_add(1);
            } else {
                (*(*new_item).hh.tbl).ineff_expands = 0 as libc::c_uint;
            }
            if (*(*new_item).hh.tbl).ineff_expands > 1 as libc::c_uint {
                (*(*new_item).hh.tbl).noexpand = 1 as libc::c_uint;
            }
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn check_symloop_leave(mut dirkey: *mut dirkey_t) -> libc::c_int {
    let mut item_found: *mut symdir_t = 0 as *mut symdir_t;
    let mut _hf_bkt: libc::c_uint = 0;
    let mut _hf_hashv: libc::c_uint = 0;
    let mut _hj_i: libc::c_uint = 0;
    let mut _hj_j: libc::c_uint = 0;
    let mut _hj_k: libc::c_uint = 0;
    let mut _hj_key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: libc::c_int = 0;
    let mut _hd_bkt: libc::c_uint = 0;
    let mut _hd_hh_del: *mut UT_hash_handle = 0 as *mut UT_hash_handle;
    item_found = 0 as *mut libc::c_void as *mut symdir_t;
    if (*dirkey).dev == 0 as libc::c_ulong {
        if (*dirkey).ino == 0 as libc::c_ulong {
            return -(1 as libc::c_int);
        }
    }
    item_found = 0 as *mut libc::c_void as *mut symdir_t;
    if !symhash.is_null() {
        _hj_key = dirkey as *mut libc::c_uchar;
        _hf_hashv = 4276993775 as libc::c_uint;
        _hj_j = 2654435769 as libc::c_uint;
        _hj_i = _hj_j;
        _hj_k = ::std::mem::size_of::<dirkey_t>() as libc::c_ulong as libc::c_uint;
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
                            (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                                << 16 as libc::c_int,
                        )
                        .wrapping_add(
                            (*_hj_key.offset(11 as libc::c_int as isize) as libc::c_uint)
                                << 24 as libc::c_int,
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
        _hf_hashv = (_hf_hashv as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<dirkey_t>() as libc::c_ulong)
            as libc::c_uint;
        let mut current_block_58: u64;
        match _hj_k {
            11 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(10 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_58 = 17432673760214412124;
            }
            10 => {
                current_block_58 = 17432673760214412124;
            }
            9 => {
                current_block_58 = 10137368285017592761;
            }
            8 => {
                current_block_58 = 10255565415889986909;
            }
            7 => {
                current_block_58 = 2718843859167896656;
            }
            6 => {
                current_block_58 = 9923973486983947351;
            }
            5 => {
                current_block_58 = 3495872400215169505;
            }
            4 => {
                current_block_58 = 3972608013953562677;
            }
            3 => {
                current_block_58 = 17073382754429133908;
            }
            2 => {
                current_block_58 = 4724475780081088802;
            }
            1 => {
                current_block_58 = 3328066059751196206;
            }
            _ => {
                current_block_58 = 13678349939556791712;
            }
        }
        match current_block_58 {
            17432673760214412124 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(9 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_58 = 10137368285017592761;
            }
            _ => {}
        }
        match current_block_58 {
            10137368285017592761 => {
                _hf_hashv = _hf_hashv
                    .wrapping_add(
                        (*_hj_key.offset(8 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_58 = 10255565415889986909;
            }
            _ => {}
        }
        match current_block_58 {
            10255565415889986909 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(7 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_58 = 2718843859167896656;
            }
            _ => {}
        }
        match current_block_58 {
            2718843859167896656 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(6 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_58 = 9923973486983947351;
            }
            _ => {}
        }
        match current_block_58 {
            9923973486983947351 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        (*_hj_key.offset(5 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_58 = 3495872400215169505;
            }
            _ => {}
        }
        match current_block_58 {
            3495872400215169505 => {
                _hj_j = _hj_j
                    .wrapping_add(
                        *_hj_key.offset(4 as libc::c_int as isize) as libc::c_uint,
                    );
                current_block_58 = 3972608013953562677;
            }
            _ => {}
        }
        match current_block_58 {
            3972608013953562677 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(3 as libc::c_int as isize) as libc::c_uint)
                            << 24 as libc::c_int,
                    );
                current_block_58 = 17073382754429133908;
            }
            _ => {}
        }
        match current_block_58 {
            17073382754429133908 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(2 as libc::c_int as isize) as libc::c_uint)
                            << 16 as libc::c_int,
                    );
                current_block_58 = 4724475780081088802;
            }
            _ => {}
        }
        match current_block_58 {
            4724475780081088802 => {
                _hj_i = _hj_i
                    .wrapping_add(
                        (*_hj_key.offset(1 as libc::c_int as isize) as libc::c_uint)
                            << 8 as libc::c_int,
                    );
                current_block_58 = 3328066059751196206;
            }
            _ => {}
        }
        match current_block_58 {
            3328066059751196206 => {
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
        _hf_bkt = _hf_hashv
            & ((*(*symhash).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
        if !((*((*(*symhash).hh.tbl).buckets).offset(_hf_bkt as isize)).hh_head)
            .is_null()
        {
            item_found = ((*((*(*symhash).hh.tbl).buckets).offset(_hf_bkt as isize))
                .hh_head as *mut libc::c_char)
                .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void
                as *mut symdir_t;
        } else {
            item_found = 0 as *mut libc::c_void as *mut symdir_t;
        }
        while !item_found.is_null() {
            if (*item_found).hh.keylen as libc::c_ulong
                == ::std::mem::size_of::<dirkey_t>() as libc::c_ulong
            {
                tmp = memcmp(
                    (*item_found).hh.key as *const libc::c_void,
                    dirkey as *const libc::c_void,
                    ::std::mem::size_of::<dirkey_t>() as libc::c_ulong,
                );
                if tmp == 0 as libc::c_int {
                    break;
                }
            }
            if !((*item_found).hh.hh_next).is_null() {
                item_found = ((*item_found).hh.hh_next as *mut libc::c_char)
                    .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void
                    as *mut symdir_t;
            } else {
                item_found = 0 as *mut libc::c_void as *mut symdir_t;
            }
        }
    }
    if item_found.is_null() {
        log_err(
            b"item not found! weird stuff...\n\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    let mut current_block_141: u64;
    if (*item_found).hh.prev as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
    {
        if (*item_found).hh.next as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            free((*(*symhash).hh.tbl).buckets as *mut libc::c_void);
            free((*symhash).hh.tbl as *mut libc::c_void);
            symhash = 0 as *mut libc::c_void as *mut symdir_t;
            current_block_141 = 15417752026496523887;
        } else {
            current_block_141 = 9009305615170901601;
        }
    } else {
        current_block_141 = 9009305615170901601;
    }
    match current_block_141 {
        9009305615170901601 => {
            _hd_hh_del = &mut (*item_found).hh;
            if item_found as libc::c_ulong
                == ((*(*symhash).hh.tbl).tail as *mut libc::c_char)
                    .offset(-((*(*symhash).hh.tbl).hho as isize)) as *mut libc::c_void
                    as libc::c_ulong
            {
                (*(*symhash).hh.tbl)
                    .tail = ((*item_found).hh.prev as ptrdiff_t
                    + (*(*symhash).hh.tbl).hho) as *mut UT_hash_handle;
            }
            if !((*item_found).hh.prev).is_null() {
                let ref mut fresh13 = (*(((*item_found).hh.prev as ptrdiff_t
                    + (*(*symhash).hh.tbl).hho) as *mut UT_hash_handle))
                    .next;
                *fresh13 = (*item_found).hh.next;
            } else {
                symhash = (*item_found).hh.next as *mut symdir_t;
            }
            if !((*_hd_hh_del).next).is_null() {
                let ref mut fresh14 = (*(((*_hd_hh_del).next as ptrdiff_t
                    + (*(*symhash).hh.tbl).hho) as *mut UT_hash_handle))
                    .prev;
                *fresh14 = (*_hd_hh_del).prev;
            }
            _hd_bkt = (*_hd_hh_del).hashv
                & ((*(*symhash).hh.tbl).num_buckets).wrapping_sub(1 as libc::c_uint);
            let ref mut fresh15 = (*((*(*symhash).hh.tbl).buckets)
                .offset(_hd_bkt as isize))
                .count;
            *fresh15 = (*fresh15).wrapping_sub(1);
            if (*((*(*symhash).hh.tbl).buckets).offset(_hd_bkt as isize)).hh_head
                as libc::c_ulong == _hd_hh_del as libc::c_ulong
            {
                let ref mut fresh16 = (*((*(*symhash).hh.tbl).buckets)
                    .offset(_hd_bkt as isize))
                    .hh_head;
                *fresh16 = (*_hd_hh_del).hh_next;
            }
            if !((*_hd_hh_del).hh_prev).is_null() {
                (*(*_hd_hh_del).hh_prev).hh_next = (*_hd_hh_del).hh_next;
            }
            if !((*_hd_hh_del).hh_next).is_null() {
                (*(*_hd_hh_del).hh_next).hh_prev = (*_hd_hh_del).hh_prev;
            }
            (*(*symhash).hh.tbl)
                .num_items = ((*(*symhash).hh.tbl).num_items).wrapping_sub(1);
        }
        _ => {}
    }
    free(item_found as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn search_dir(
    mut ig: *mut ignores,
    mut base_path: *const libc::c_char,
    mut path: *const libc::c_char,
    depth: libc::c_int,
    mut original_dev: dev_t,
) {
    let mut current_block: u64;
    let mut dir_list: *mut *mut dirent = 0 as *mut *mut dirent;
    let mut dir: *mut dirent = 0 as *mut dirent;
    let mut scandir_baton: scandir_baton_t = scandir_baton_t {
        ig: 0 as *const ignores,
        base_path: 0 as *const libc::c_char,
        base_path_len: 0,
        path_start: 0 as *const libc::c_char,
    };
    let mut results: libc::c_int = 0;
    let mut base_path_len: size_t = 0;
    let mut path_start: *const libc::c_char = 0 as *const libc::c_char;
    let mut dir_full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ignore_file: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut symres: libc::c_int = 0;
    let mut current_dirkey: dirkey_t = dirkey_t { dev: 0, ino: 0 };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut offset_vector: [libc::c_int; 3] = [0; 3];
    let mut rc: libc::c_int = 0;
    let mut queue_item: *mut work_queue_t = 0 as *mut work_queue_t;
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut child_ig: *mut ignores = 0 as *mut ignores;
    let mut tmp___8: size_t = 0;
    let mut tmp___9: libc::c_int = 0;
    dir_list = 0 as *mut libc::c_void as *mut *mut dirent;
    dir = 0 as *mut libc::c_void as *mut dirent;
    results = 0 as libc::c_int;
    base_path_len = 0 as libc::c_int as size_t;
    path_start = path;
    dir_full_path = 0 as *mut libc::c_void as *mut libc::c_char;
    ignore_file = 0 as *mut libc::c_void as *const libc::c_char;
    symres = check_symloop_enter(path, &mut current_dirkey);
    if symres == 1 as libc::c_int {
        log_err(
            b"Recursive directory loop: %s\0" as *const u8 as *const libc::c_char,
            path,
        );
        return;
    }
    i = 0 as libc::c_int;
    loop {
        if opts.skip_vcs_ignores != 0 {
            tmp = (i == 0 as libc::c_int) as libc::c_int;
        } else {
            tmp = (ignore_pattern_files[i as usize] as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int;
        }
        if tmp == 0 {
            break;
        }
        ignore_file = ignore_pattern_files[i as usize];
        ag_asprintf(
            &mut dir_full_path as *mut *mut libc::c_char,
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            path,
            ignore_file,
        );
        load_ignore_patterns(ig, dir_full_path as *const libc::c_char);
        free(dir_full_path as *mut libc::c_void);
        dir_full_path = 0 as *mut libc::c_void as *mut libc::c_char;
        i += 1;
    }
    if !base_path.is_null() {
        tmp___0 = strlen(base_path);
        base_path_len = tmp___0;
    } else {
        base_path_len = 0 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int;
    while (i as size_t) < base_path_len {
        if !(*path.offset(i as isize) != 0) {
            break;
        }
        if !(*base_path.offset(i as isize) as libc::c_int
            == *path.offset(i as isize) as libc::c_int)
        {
            break;
        }
        path_start = path.offset(i as isize).offset(1 as libc::c_int as isize);
        i += 1;
    }
    log_debug(
        b"search_dir: path is '%s', base_path is '%s', path_start is '%s'\0" as *const u8
            as *const libc::c_char,
        path,
        base_path,
        path_start,
    );
    scandir_baton.ig = ig as *const ignores;
    scandir_baton.base_path = base_path;
    scandir_baton.base_path_len = base_path_len;
    scandir_baton.path_start = path_start;
    results = ag_scandir(
        path,
        &mut dir_list,
        Some(
            filename_filter
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const dirent,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut scandir_baton as *mut scandir_baton_t as *mut libc::c_void,
    );
    if results == 0 as libc::c_int {
        log_debug(
            b"No results found in directory %s\0" as *const u8 as *const libc::c_char,
            path,
        );
    } else if results == -(1 as libc::c_int) {
        tmp___3 = __errno_location();
        if *tmp___3 == 20 as libc::c_int {
            if depth == 0 as libc::c_int {
                if opts.paths_len == 1 as libc::c_int {
                    if opts.print_path == 0 as libc::c_int {
                        opts.print_path = 4 as libc::c_int;
                    } else if opts.print_path == 1 as libc::c_int {
                        opts.print_path = 4 as libc::c_int;
                    }
                    if opts.only_matching != 0 {
                        if opts.print_path == 4 as libc::c_int {
                            opts.print_line_numbers = 0 as libc::c_int;
                        }
                    }
                }
            }
            search_file(path);
        } else {
            tmp___1 = __errno_location();
            tmp___2 = strerror(*tmp___1);
            log_err(
                b"Error opening directory %s: %s\0" as *const u8 as *const libc::c_char,
                path,
                tmp___2,
            );
        }
    } else {
        rc = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < results {
            queue_item = 0 as *mut libc::c_void as *mut work_queue_t;
            dir = *dir_list.offset(i as isize);
            ag_asprintf(
                &mut dir_full_path as *mut *mut libc::c_char,
                b"%s/%s\0" as *const u8 as *const libc::c_char,
                path,
                ((*dir).d_name).as_mut_ptr(),
            );
            if opts.one_dev != 0 {
                tmp___4 = lstat(
                    dir_full_path as *const libc::c_char,
                    &mut s as *mut stat,
                );
                if tmp___4 != 0 as libc::c_int {
                    log_err(
                        b"Failed to get device information for %s. Skipping...\0"
                            as *const u8 as *const libc::c_char,
                        ((*dir).d_name).as_mut_ptr(),
                    );
                    current_block = 6561497250428016462;
                } else if s.st_dev != original_dev {
                    log_debug(
                        b"File %s crosses a device boundary (is probably a mount point.) Skipping...\0"
                            as *const u8 as *const libc::c_char,
                        ((*dir).d_name).as_mut_ptr(),
                    );
                    current_block = 6561497250428016462;
                } else {
                    current_block = 726525485109251713;
                }
            } else {
                current_block = 726525485109251713;
            }
            match current_block {
                726525485109251713 => {
                    if opts.follow_symlinks == 0 {
                        tmp___5 = is_symlink(path, dir as *const dirent);
                        if tmp___5 != 0 {
                            log_debug(
                                b"File %s ignored becaused it's a symlink\0" as *const u8
                                    as *const libc::c_char,
                                ((*dir).d_name).as_mut_ptr(),
                            );
                            current_block = 6561497250428016462;
                        } else {
                            current_block = 14648606000749551097;
                        }
                    } else {
                        current_block = 14648606000749551097;
                    }
                    match current_block {
                        6561497250428016462 => {}
                        _ => {
                            tmp___9 = is_directory(path, dir as *const dirent);
                            if tmp___9 != 0 {
                                if opts.recurse_dirs != 0 {
                                    if depth < opts.max_search_depth {
                                        log_debug(
                                            b"Searching dir %s\0" as *const u8 as *const libc::c_char,
                                            dir_full_path,
                                        );
                                        tmp___8 = strlen(
                                            ((*dir).d_name).as_mut_ptr() as *const libc::c_char,
                                        );
                                        child_ig = init_ignore(
                                            ig,
                                            ((*dir).d_name).as_mut_ptr() as *const libc::c_char,
                                            tmp___8,
                                        );
                                        search_dir(
                                            child_ig,
                                            base_path,
                                            dir_full_path as *const libc::c_char,
                                            depth + 1 as libc::c_int,
                                            original_dev,
                                        );
                                        cleanup_ignore(child_ig);
                                    } else if opts.max_search_depth == -(1 as libc::c_int) {
                                        log_debug(
                                            b"Searching dir %s\0" as *const u8 as *const libc::c_char,
                                            dir_full_path,
                                        );
                                        tmp___8 = strlen(
                                            ((*dir).d_name).as_mut_ptr() as *const libc::c_char,
                                        );
                                        child_ig = init_ignore(
                                            ig,
                                            ((*dir).d_name).as_mut_ptr() as *const libc::c_char,
                                            tmp___8,
                                        );
                                        search_dir(
                                            child_ig,
                                            base_path,
                                            dir_full_path as *const libc::c_char,
                                            depth + 1 as libc::c_int,
                                            original_dev,
                                        );
                                        cleanup_ignore(child_ig);
                                    } else if opts.max_search_depth == 25 as libc::c_int {
                                        log_err(
                                            b"Skipping %s. Use the --depth option to search deeper.\0"
                                                as *const u8 as *const libc::c_char,
                                            dir_full_path,
                                        );
                                    } else {
                                        log_debug(
                                            b"Skipping %s. Use the --depth option to search deeper.\0"
                                                as *const u8 as *const libc::c_char,
                                            dir_full_path,
                                        );
                                    }
                                }
                            } else {
                                if !(opts.file_search_regex).is_null() {
                                    tmp___6 = strlen(dir_full_path as *const libc::c_char);
                                    rc = pcre_exec(
                                        opts.file_search_regex as *const pcre,
                                        0 as *mut libc::c_void as *const pcre_extra,
                                        dir_full_path as *const libc::c_char,
                                        tmp___6 as libc::c_int,
                                        0 as libc::c_int,
                                        0 as libc::c_int,
                                        offset_vector.as_mut_ptr(),
                                        3 as libc::c_int,
                                    );
                                    if rc < 0 as libc::c_int {
                                        log_debug(
                                            b"Skipping %s due to file_search_regex.\0" as *const u8
                                                as *const libc::c_char,
                                            dir_full_path,
                                        );
                                        current_block = 6561497250428016462;
                                    } else if opts.match_files != 0 {
                                        log_debug(
                                            b"match_files: file_search_regex matched for %s.\0"
                                                as *const u8 as *const libc::c_char,
                                            dir_full_path,
                                        );
                                        pthread_mutex_lock(&mut print_mtx);
                                        print_path(
                                            dir_full_path as *const libc::c_char,
                                            opts.path_sep,
                                        );
                                        pthread_mutex_unlock(&mut print_mtx);
                                        opts.match_found = 1 as libc::c_int;
                                        current_block = 6561497250428016462;
                                    } else {
                                        current_block = 13201766686570145889;
                                    }
                                } else {
                                    current_block = 13201766686570145889;
                                }
                                match current_block {
                                    6561497250428016462 => {}
                                    _ => {
                                        tmp___7 = ag_malloc(
                                            ::std::mem::size_of::<work_queue_t>() as libc::c_ulong,
                                        );
                                        queue_item = tmp___7 as *mut work_queue_t;
                                        (*queue_item).path = dir_full_path;
                                        (*queue_item)
                                            .next = 0 as *mut libc::c_void as *mut work_queue_t;
                                        pthread_mutex_lock(&mut work_queue_mtx);
                                        if work_queue_tail as libc::c_ulong
                                            == 0 as *mut libc::c_void as libc::c_ulong
                                        {
                                            work_queue = queue_item;
                                        } else {
                                            (*work_queue_tail).next = queue_item;
                                        }
                                        work_queue_tail = queue_item;
                                        pthread_cond_signal(&mut files_ready);
                                        pthread_mutex_unlock(&mut work_queue_mtx);
                                        log_debug(
                                            b"%s added to work queue\0" as *const u8
                                                as *const libc::c_char,
                                            dir_full_path,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            free(dir as *mut libc::c_void);
            dir = 0 as *mut libc::c_void as *mut dirent;
            if queue_item as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                free(dir_full_path as *mut libc::c_void);
                dir_full_path = 0 as *mut libc::c_void as *mut libc::c_char;
            }
            i += 1;
        }
    }
    check_symloop_leave(&mut current_dirkey);
    free(dir_list as *mut libc::c_void);
    dir_list = 0 as *mut libc::c_void as *mut *mut dirent;
}
pub static mut langs: [lang_spec_t; 138] = [
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"actionscript\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"as\0" as *const u8 as *const libc::c_char,
                b"mxml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"ada\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ada\0" as *const u8 as *const libc::c_char,
                b"adb\0" as *const u8 as *const libc::c_char,
                b"ads\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"asciidoc\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"adoc\0" as *const u8 as *const libc::c_char,
                b"ad\0" as *const u8 as *const libc::c_char,
                b"asc\0" as *const u8 as *const libc::c_char,
                b"asciidoc\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"apl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"apl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"asm\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"asm\0" as *const u8 as *const libc::c_char,
                b"s\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"asp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"asp\0" as *const u8 as *const libc::c_char,
                b"asa\0" as *const u8 as *const libc::c_char,
                b"aspx\0" as *const u8 as *const libc::c_char,
                b"asax\0" as *const u8 as *const libc::c_char,
                b"ashx\0" as *const u8 as *const libc::c_char,
                b"ascx\0" as *const u8 as *const libc::c_char,
                b"asmx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"aspx\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"asp\0" as *const u8 as *const libc::c_char,
                b"asa\0" as *const u8 as *const libc::c_char,
                b"aspx\0" as *const u8 as *const libc::c_char,
                b"asax\0" as *const u8 as *const libc::c_char,
                b"ashx\0" as *const u8 as *const libc::c_char,
                b"ascx\0" as *const u8 as *const libc::c_char,
                b"asmx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"batch\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"bat\0" as *const u8 as *const libc::c_char,
                b"cmd\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"bazel\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"bazel\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"bitbake\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"bb\0" as *const u8 as *const libc::c_char,
                b"bbappend\0" as *const u8 as *const libc::c_char,
                b"bbclass\0" as *const u8 as *const libc::c_char,
                b"inc\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"cc\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"c\0" as *const u8 as *const libc::c_char,
                b"h\0" as *const u8 as *const libc::c_char,
                b"xs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"cfmx\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cfc\0" as *const u8 as *const libc::c_char,
                b"cfm\0" as *const u8 as *const libc::c_char,
                b"cfml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"chpl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"chpl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"clojure\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"clj\0" as *const u8 as *const libc::c_char,
                b"cljs\0" as *const u8 as *const libc::c_char,
                b"cljc\0" as *const u8 as *const libc::c_char,
                b"cljx\0" as *const u8 as *const libc::c_char,
                b"edn\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"coffee\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"coffee\0" as *const u8 as *const libc::c_char,
                b"cjsx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"config\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"config\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"coq\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"coq\0" as *const u8 as *const libc::c_char,
                b"g\0" as *const u8 as *const libc::c_char,
                b"v\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"cpp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cpp\0" as *const u8 as *const libc::c_char,
                b"cc\0" as *const u8 as *const libc::c_char,
                b"C\0" as *const u8 as *const libc::c_char,
                b"cxx\0" as *const u8 as *const libc::c_char,
                b"m\0" as *const u8 as *const libc::c_char,
                b"hpp\0" as *const u8 as *const libc::c_char,
                b"hh\0" as *const u8 as *const libc::c_char,
                b"h\0" as *const u8 as *const libc::c_char,
                b"H\0" as *const u8 as *const libc::c_char,
                b"hxx\0" as *const u8 as *const libc::c_char,
                b"tpp\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"crystal\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cr\0" as *const u8 as *const libc::c_char,
                b"ecr\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"csharp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"cshtml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cshtml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"css\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"css\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"cython\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pyx\0" as *const u8 as *const libc::c_char,
                b"pxd\0" as *const u8 as *const libc::c_char,
                b"pxi\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"delphi\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pas\0" as *const u8 as *const libc::c_char,
                b"int\0" as *const u8 as *const libc::c_char,
                b"dfm\0" as *const u8 as *const libc::c_char,
                b"nfm\0" as *const u8 as *const libc::c_char,
                b"dof\0" as *const u8 as *const libc::c_char,
                b"dpk\0" as *const u8 as *const libc::c_char,
                b"dpr\0" as *const u8 as *const libc::c_char,
                b"dproj\0" as *const u8 as *const libc::c_char,
                b"groupproj\0" as *const u8 as *const libc::c_char,
                b"bdsgroup\0" as *const u8 as *const libc::c_char,
                b"bdsproj\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"dlang\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"d\0" as *const u8 as *const libc::c_char,
                b"di\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"dot\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"dot\0" as *const u8 as *const libc::c_char,
                b"gv\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"dts\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"dts\0" as *const u8 as *const libc::c_char,
                b"dtsi\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"ebuild\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ebuild\0" as *const u8 as *const libc::c_char,
                b"eclass\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"elisp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"el\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"elixir\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ex\0" as *const u8 as *const libc::c_char,
                b"eex\0" as *const u8 as *const libc::c_char,
                b"exs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"elm\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"elm\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"erlang\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"erl\0" as *const u8 as *const libc::c_char,
                b"hrl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"factor\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"factor\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"fortran\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"f\0" as *const u8 as *const libc::c_char,
                b"F\0" as *const u8 as *const libc::c_char,
                b"f77\0" as *const u8 as *const libc::c_char,
                b"f90\0" as *const u8 as *const libc::c_char,
                b"F90\0" as *const u8 as *const libc::c_char,
                b"f95\0" as *const u8 as *const libc::c_char,
                b"f03\0" as *const u8 as *const libc::c_char,
                b"for\0" as *const u8 as *const libc::c_char,
                b"ftn\0" as *const u8 as *const libc::c_char,
                b"fpp\0" as *const u8 as *const libc::c_char,
                b"FPP\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"fsharp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"fs\0" as *const u8 as *const libc::c_char,
                b"fsi\0" as *const u8 as *const libc::c_char,
                b"fsx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"gettext\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"po\0" as *const u8 as *const libc::c_char,
                b"pot\0" as *const u8 as *const libc::c_char,
                b"mo\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"glsl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vert\0" as *const u8 as *const libc::c_char,
                b"tesc\0" as *const u8 as *const libc::c_char,
                b"tese\0" as *const u8 as *const libc::c_char,
                b"geom\0" as *const u8 as *const libc::c_char,
                b"frag\0" as *const u8 as *const libc::c_char,
                b"comp\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"go\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"go\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"gradle\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"gradle\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"groovy\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"groovy\0" as *const u8 as *const libc::c_char,
                b"gtmpl\0" as *const u8 as *const libc::c_char,
                b"gpp\0" as *const u8 as *const libc::c_char,
                b"grunit\0" as *const u8 as *const libc::c_char,
                b"gradle\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"haml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"haml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"handlebars\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"hbs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"haskell\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"hs\0" as *const u8 as *const libc::c_char,
                b"hsig\0" as *const u8 as *const libc::c_char,
                b"lhs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"haxe\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"hx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"hh\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"h\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"html\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"htm\0" as *const u8 as *const libc::c_char,
                b"html\0" as *const u8 as *const libc::c_char,
                b"shtml\0" as *const u8 as *const libc::c_char,
                b"xhtml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"idris\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"idr\0" as *const u8 as *const libc::c_char,
                b"ipkg\0" as *const u8 as *const libc::c_char,
                b"lidr\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"ini\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ini\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"ipython\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ipynb\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"isabelle\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"thy\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"j\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ijs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"jade\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"jade\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"java\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"java\0" as *const u8 as *const libc::c_char,
                b"properties\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"jinja2\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"j2\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"js\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"es6\0" as *const u8 as *const libc::c_char,
                b"js\0" as *const u8 as *const libc::c_char,
                b"jsx\0" as *const u8 as *const libc::c_char,
                b"vue\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"json\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"json\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"jsp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"jsp\0" as *const u8 as *const libc::c_char,
                b"jspx\0" as *const u8 as *const libc::c_char,
                b"jhtm\0" as *const u8 as *const libc::c_char,
                b"jhtml\0" as *const u8 as *const libc::c_char,
                b"jspf\0" as *const u8 as *const libc::c_char,
                b"tag\0" as *const u8 as *const libc::c_char,
                b"tagf\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"julia\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"jl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"kotlin\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"kt\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"less\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"less\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"liquid\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"liquid\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"lisp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"lisp\0" as *const u8 as *const libc::c_char,
                b"lsp\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"log\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"log\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"lua\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"lua\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"m4\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m4\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"make\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"Makefiles\0" as *const u8 as *const libc::c_char,
                b"mk\0" as *const u8 as *const libc::c_char,
                b"mak\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"mako\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"mako\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"markdown\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"markdown\0" as *const u8 as *const libc::c_char,
                b"mdown\0" as *const u8 as *const libc::c_char,
                b"mdwn\0" as *const u8 as *const libc::c_char,
                b"mkdn\0" as *const u8 as *const libc::c_char,
                b"mkd\0" as *const u8 as *const libc::c_char,
                b"md\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"mason\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"mas\0" as *const u8 as *const libc::c_char,
                b"mhtml\0" as *const u8 as *const libc::c_char,
                b"mpl\0" as *const u8 as *const libc::c_char,
                b"mtxt\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"matlab\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"mathematica\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m\0" as *const u8 as *const libc::c_char,
                b"wl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"md\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"markdown\0" as *const u8 as *const libc::c_char,
                b"mdown\0" as *const u8 as *const libc::c_char,
                b"mdwn\0" as *const u8 as *const libc::c_char,
                b"mkdn\0" as *const u8 as *const libc::c_char,
                b"mkd\0" as *const u8 as *const libc::c_char,
                b"md\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"mercury\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m\0" as *const u8 as *const libc::c_char,
                b"moo\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"naccess\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"asa\0" as *const u8 as *const libc::c_char,
                b"rsa\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"nim\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"nim\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"nix\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"nix\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"objc\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m\0" as *const u8 as *const libc::c_char,
                b"h\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"objcpp\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"mm\0" as *const u8 as *const libc::c_char,
                b"h\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"ocaml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ml\0" as *const u8 as *const libc::c_char,
                b"mli\0" as *const u8 as *const libc::c_char,
                b"mll\0" as *const u8 as *const libc::c_char,
                b"mly\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"octave\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"m\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"org\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"org\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"parrot\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pir\0" as *const u8 as *const libc::c_char,
                b"pasm\0" as *const u8 as *const libc::c_char,
                b"pmc\0" as *const u8 as *const libc::c_char,
                b"ops\0" as *const u8 as *const libc::c_char,
                b"pod\0" as *const u8 as *const libc::c_char,
                b"pg\0" as *const u8 as *const libc::c_char,
                b"tg\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"pdb\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pdb\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"perl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pl\0" as *const u8 as *const libc::c_char,
                b"pm\0" as *const u8 as *const libc::c_char,
                b"pm6\0" as *const u8 as *const libc::c_char,
                b"pod\0" as *const u8 as *const libc::c_char,
                b"t\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"php\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"php\0" as *const u8 as *const libc::c_char,
                b"phpt\0" as *const u8 as *const libc::c_char,
                b"php3\0" as *const u8 as *const libc::c_char,
                b"php4\0" as *const u8 as *const libc::c_char,
                b"php5\0" as *const u8 as *const libc::c_char,
                b"phtml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"pike\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pike\0" as *const u8 as *const libc::c_char,
                b"pmod\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"plist\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"plist\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"plone\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pt\0" as *const u8 as *const libc::c_char,
                b"cpt\0" as *const u8 as *const libc::c_char,
                b"metadata\0" as *const u8 as *const libc::c_char,
                b"cpy\0" as *const u8 as *const libc::c_char,
                b"py\0" as *const u8 as *const libc::c_char,
                b"xml\0" as *const u8 as *const libc::c_char,
                b"zcml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"powershell\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ps1\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"proto\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"proto\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"ps1\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ps1\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"pug\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pug\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"puppet\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"pp\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"python\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"py\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"qml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"qml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"racket\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rkt\0" as *const u8 as *const libc::c_char,
                b"ss\0" as *const u8 as *const libc::c_char,
                b"scm\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"rake\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"Rakefile\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"razor\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"cshtml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"restructuredtext\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rst\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"rs\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"r\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"r\0" as *const u8 as *const libc::c_char,
                b"R\0" as *const u8 as *const libc::c_char,
                b"Rmd\0" as *const u8 as *const libc::c_char,
                b"Rnw\0" as *const u8 as *const libc::c_char,
                b"Rtex\0" as *const u8 as *const libc::c_char,
                b"Rrst\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"rdoc\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rdoc\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"ruby\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rb\0" as *const u8 as *const libc::c_char,
                b"rhtml\0" as *const u8 as *const libc::c_char,
                b"rjs\0" as *const u8 as *const libc::c_char,
                b"rxml\0" as *const u8 as *const libc::c_char,
                b"erb\0" as *const u8 as *const libc::c_char,
                b"rake\0" as *const u8 as *const libc::c_char,
                b"spec\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"rust\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"rs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"salt\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"sls\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"sass\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"sass\0" as *const u8 as *const libc::c_char,
                b"scss\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"scala\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"scala\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"scheme\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"scm\0" as *const u8 as *const libc::c_char,
                b"ss\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"shell\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"sh\0" as *const u8 as *const libc::c_char,
                b"bash\0" as *const u8 as *const libc::c_char,
                b"csh\0" as *const u8 as *const libc::c_char,
                b"tcsh\0" as *const u8 as *const libc::c_char,
                b"ksh\0" as *const u8 as *const libc::c_char,
                b"zsh\0" as *const u8 as *const libc::c_char,
                b"fish\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"smalltalk\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"st\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"sml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"sml\0" as *const u8 as *const libc::c_char,
                b"fun\0" as *const u8 as *const libc::c_char,
                b"mlb\0" as *const u8 as *const libc::c_char,
                b"sig\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"sql\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"sql\0" as *const u8 as *const libc::c_char,
                b"ctl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"stata\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"do\0" as *const u8 as *const libc::c_char,
                b"ado\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"stylus\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"styl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"swift\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"swift\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"tcl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"tcl\0" as *const u8 as *const libc::c_char,
                b"itcl\0" as *const u8 as *const libc::c_char,
                b"itk\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"terraform\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"tf\0" as *const u8 as *const libc::c_char,
                b"tfvars\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"tex\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"tex\0" as *const u8 as *const libc::c_char,
                b"cls\0" as *const u8 as *const libc::c_char,
                b"sty\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"thrift\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"thrift\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"tla\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"tla\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"tt\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"tt\0" as *const u8 as *const libc::c_char,
                b"tt2\0" as *const u8 as *const libc::c_char,
                b"ttml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"toml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"toml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"ts\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"ts\0" as *const u8 as *const libc::c_char,
                b"tsx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"twig\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"twig\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"vala\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vala\0" as *const u8 as *const libc::c_char,
                b"vapi\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"vb\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"bas\0" as *const u8 as *const libc::c_char,
                b"cls\0" as *const u8 as *const libc::c_char,
                b"frm\0" as *const u8 as *const libc::c_char,
                b"ctl\0" as *const u8 as *const libc::c_char,
                b"vb\0" as *const u8 as *const libc::c_char,
                b"resx\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"velocity\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vm\0" as *const u8 as *const libc::c_char,
                b"vtl\0" as *const u8 as *const libc::c_char,
                b"vsl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"verilog\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"v\0" as *const u8 as *const libc::c_char,
                b"vh\0" as *const u8 as *const libc::c_char,
                b"sv\0" as *const u8 as *const libc::c_char,
                b"svh\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"vhdl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vhd\0" as *const u8 as *const libc::c_char,
                b"vhdl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"vim\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vim\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"vue\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"vue\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"wix\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"wxi\0" as *const u8 as *const libc::c_char,
                b"wxs\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"wsdl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"wsdl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"wadl\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"wadl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"xml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"xml\0" as *const u8 as *const libc::c_char,
                b"dtd\0" as *const u8 as *const libc::c_char,
                b"xsl\0" as *const u8 as *const libc::c_char,
                b"xslt\0" as *const u8 as *const libc::c_char,
                b"xsd\0" as *const u8 as *const libc::c_char,
                b"ent\0" as *const u8 as *const libc::c_char,
                b"tld\0" as *const u8 as *const libc::c_char,
                b"plist\0" as *const u8 as *const libc::c_char,
                b"wsdl\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"yaml\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"yaml\0" as *const u8 as *const libc::c_char,
                b"yml\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"zeek\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"zeek\0" as *const u8 as *const libc::c_char,
                b"bro\0" as *const u8 as *const libc::c_char,
                b"bif\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
    {
        let mut init = __anonstruct_lang_spec_t_527861670 {
            name: b"zephir\0" as *const u8 as *const libc::c_char,
            extensions: [
                b"zep\0" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
                0 as *const libc::c_char,
            ],
        };
        init
    },
];
pub unsafe extern "C" fn get_lang_count() -> size_t {
    return (::std::mem::size_of::<[lang_spec_t; 138]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<lang_spec_t>() as libc::c_ulong);
}
pub unsafe extern "C" fn make_lang_regex(
    mut ext_array: *mut libc::c_char,
    mut num_exts: size_t,
) -> *mut libc::c_char {
    let mut regex_capacity: libc::c_int = 0;
    let mut regex: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut regex_length: libc::c_int = 0;
    let mut subsequent: libc::c_int = 0;
    let mut extension: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: size_t = 0;
    let mut extension_length: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    regex_capacity = 100 as libc::c_int;
    tmp = ag_malloc(regex_capacity as size_t);
    regex = tmp as *mut libc::c_char;
    regex_length = 3 as libc::c_int;
    subsequent = 0 as libc::c_int;
    strcpy(regex, b"\\.(\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int as size_t;
    while i < num_exts {
        extension = ext_array.offset(i.wrapping_mul(20 as libc::c_ulong) as isize);
        tmp___0 = strlen(extension as *const libc::c_char);
        extension_length = tmp___0 as libc::c_int;
        while regex_length + extension_length + 3 as libc::c_int + subsequent
            > regex_capacity
        {
            regex_capacity *= 2 as libc::c_int;
            tmp___1 = ag_realloc(regex as *mut libc::c_void, regex_capacity as size_t);
            regex = tmp___1 as *mut libc::c_char;
        }
        if subsequent != 0 {
            tmp___2 = regex_length;
            regex_length += 1;
            *regex.offset(tmp___2 as isize) = '|' as i32 as libc::c_char;
        } else {
            subsequent = 1 as libc::c_int;
        }
        strcpy(regex.offset(regex_length as isize), extension as *const libc::c_char);
        regex_length += extension_length;
        i = i.wrapping_add(1);
    }
    tmp___3 = regex_length;
    regex_length += 1;
    *regex.offset(tmp___3 as isize) = ')' as i32 as libc::c_char;
    tmp___4 = regex_length;
    regex_length += 1;
    *regex.offset(tmp___4 as isize) = '$' as i32 as libc::c_char;
    tmp___5 = regex_length;
    regex_length += 1;
    *regex.offset(tmp___5 as isize) = 0 as libc::c_int as libc::c_char;
    return regex;
}
pub unsafe extern "C" fn combine_file_extensions(
    mut extension_index: *mut size_t,
    mut len: size_t,
    mut exts: *mut *mut libc::c_char,
) -> size_t {
    let mut ext_capacity: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut num_of_extensions: size_t = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut ext: *const libc::c_char = 0 as *const libc::c_char;
    let mut pos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: size_t = 0;
    ext_capacity = 100 as libc::c_int as size_t;
    tmp = ag_malloc(ext_capacity.wrapping_mul(20 as libc::c_ulong));
    *exts = tmp as *mut libc::c_char;
    memset(
        *exts as *mut libc::c_void,
        0 as libc::c_int,
        ext_capacity.wrapping_mul(20 as libc::c_ulong),
    );
    num_of_extensions = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i < len {
        j = 0 as libc::c_int as size_t;
        ext = langs[*extension_index.offset(i as isize) as usize].extensions[j as usize];
        while !(num_of_extensions == ext_capacity) {
            pos = (*exts)
                .offset(num_of_extensions.wrapping_mul(20 as libc::c_ulong) as isize);
            tmp___0 = strlen(ext);
            strncpy(pos, ext, tmp___0);
            num_of_extensions = num_of_extensions.wrapping_add(1);
            j = j.wrapping_add(1);
            ext = langs[*extension_index.offset(i as isize) as usize]
                .extensions[j as usize];
            if ext.is_null() {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    return num_of_extensions;
}
pub static mut stats: ag_stats = ag_stats {
    total_bytes: 0,
    total_files: 0,
    total_matches: 0,
    total_file_matches: 0,
    time_start: timeval { tv_sec: 0, tv_usec: 0 },
    time_end: timeval { tv_sec: 0, tv_usec: 0 },
};
pub static mut out_fd: *mut FILE = 0 as *const libc::c_void as *mut libc::c_void
    as *mut FILE;
pub unsafe extern "C" fn ag_malloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(size);
    ptr = tmp;
    if ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        die(b"Memory allocation failed.\0" as *const u8 as *const libc::c_char);
    }
    return ptr;
}
pub unsafe extern "C" fn ag_realloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut new_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = realloc(ptr, size);
    new_ptr = tmp;
    if new_ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        die(b"Memory allocation failed.\0" as *const u8 as *const libc::c_char);
    }
    return new_ptr;
}
pub unsafe extern "C" fn ag_calloc(
    mut count: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(count, size);
    ptr = tmp;
    if ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        die(b"Memory allocation failed.\0" as *const u8 as *const libc::c_char);
    }
    return ptr;
}
pub unsafe extern "C" fn ag_strdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = strdup(s);
    str = tmp;
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        die(b"Memory allocation failed.\0" as *const u8 as *const libc::c_char);
    }
    return str;
}
pub unsafe extern "C" fn ag_strndup(
    mut s: *const libc::c_char,
    mut size: size_t,
) -> *mut libc::c_char {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    str = 0 as *mut libc::c_void as *mut libc::c_char;
    str = strndup(s, size);
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        die(b"Memory allocation failed.\0" as *const u8 as *const libc::c_char);
    }
    return str;
}
pub unsafe extern "C" fn free_strings(
    mut strs: *mut *mut libc::c_char,
    strs_len: size_t,
) {
    let mut i: size_t = 0;
    if strs as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < strs_len {
        free(*strs.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
    }
    free(strs as *mut libc::c_void);
}
pub unsafe extern "C" fn generate_alpha_skip(
    mut find: *const libc::c_char,
    mut f_len: size_t,
    mut skip_lookup: *mut size_t,
    case_sensitive: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut __res: libc::c_int = 0;
    let mut tmp___0: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut __res___0: libc::c_int = 0;
    let mut tmp___2: *mut *const __int32_t = 0 as *mut *const __int32_t;
    i = 0 as libc::c_int as size_t;
    while i < 256 as libc::c_ulong {
        *skip_lookup.offset(i as isize) = f_len;
        i = i.wrapping_add(1);
    }
    f_len = f_len.wrapping_sub(1);
    i = 0 as libc::c_int as size_t;
    while i < f_len {
        if case_sensitive != 0 {
            *skip_lookup
                .offset(
                    *find.offset(i as isize) as libc::c_uchar as libc::c_int as isize,
                ) = f_len.wrapping_sub(i);
        } else {
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_ulong
            {
                __res = tolower(*find.offset(i as isize) as libc::c_int);
            } else {
                tmp___0 = __ctype_tolower_loc();
                __res = *(*tmp___0)
                    .offset(*find.offset(i as isize) as libc::c_int as isize);
            }
            *skip_lookup
                .offset(
                    __res as libc::c_uchar as libc::c_int as isize,
                ) = f_len.wrapping_sub(i);
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_ulong
            {
                __res___0 = toupper(*find.offset(i as isize) as libc::c_int);
            } else {
                tmp___2 = __ctype_toupper_loc();
                __res___0 = *(*tmp___2)
                    .offset(*find.offset(i as isize) as libc::c_int as isize);
            }
            *skip_lookup
                .offset(
                    __res___0 as libc::c_uchar as libc::c_int as isize,
                ) = f_len.wrapping_sub(i);
        }
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn is_prefix(
    mut s: *const libc::c_char,
    s_len: size_t,
    pos: size_t,
    case_sensitive: libc::c_int,
) -> libc::c_int {
    let mut i: size_t = 0;
    let mut __res: libc::c_int = 0;
    let mut tmp___0: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut __res___0: libc::c_int = 0;
    let mut tmp___2: *mut *const __int32_t = 0 as *mut *const __int32_t;
    i = 0 as libc::c_int as size_t;
    while pos.wrapping_add(i) < s_len {
        if case_sensitive != 0 {
            if *s.offset(i as isize) as libc::c_int
                != *s.offset(i.wrapping_add(pos) as isize) as libc::c_int
            {
                return 0 as libc::c_int;
            }
        } else {
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_ulong
            {
                __res = tolower(*s.offset(i as isize) as libc::c_int);
            } else {
                tmp___0 = __ctype_tolower_loc();
                __res = *(*tmp___0)
                    .offset(*s.offset(i as isize) as libc::c_int as isize);
            }
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_ulong
            {
                __res___0 = tolower(
                    *s.offset(i.wrapping_add(pos) as isize) as libc::c_int,
                );
            } else {
                tmp___2 = __ctype_tolower_loc();
                __res___0 = *(*tmp___2)
                    .offset(
                        *s.offset(i.wrapping_add(pos) as isize) as libc::c_int as isize,
                    );
            }
            if __res != __res___0 {
                return 0 as libc::c_int;
            }
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn suffix_len(
    mut s: *const libc::c_char,
    s_len: size_t,
    pos: size_t,
    case_sensitive: libc::c_int,
) -> size_t {
    let mut i: size_t = 0;
    let mut __res: libc::c_int = 0;
    let mut tmp___0: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut __res___0: libc::c_int = 0;
    let mut tmp___2: *mut *const __int32_t = 0 as *mut *const __int32_t;
    i = 0 as libc::c_int as size_t;
    while i < pos {
        if case_sensitive != 0 {
            if *s.offset(pos.wrapping_sub(i) as isize) as libc::c_int
                != *s
                    .offset(
                        s_len.wrapping_sub(i).wrapping_sub(1 as libc::c_ulong) as isize,
                    ) as libc::c_int
            {
                break;
            }
        } else {
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_ulong
            {
                __res = tolower(*s.offset(pos.wrapping_sub(i) as isize) as libc::c_int);
            } else {
                tmp___0 = __ctype_tolower_loc();
                __res = *(*tmp___0)
                    .offset(
                        *s.offset(pos.wrapping_sub(i) as isize) as libc::c_int as isize,
                    );
            }
            if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                > 1 as libc::c_ulong
            {
                __res___0 = tolower(
                    *s
                        .offset(
                            s_len.wrapping_sub(i).wrapping_sub(1 as libc::c_ulong)
                                as isize,
                        ) as libc::c_int,
                );
            } else {
                tmp___2 = __ctype_tolower_loc();
                __res___0 = *(*tmp___2)
                    .offset(
                        *s
                            .offset(
                                s_len.wrapping_sub(i).wrapping_sub(1 as libc::c_ulong)
                                    as isize,
                            ) as libc::c_int as isize,
                    );
            }
            if __res != __res___0 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    return i;
}
pub unsafe extern "C" fn generate_find_skip(
    mut find: *const libc::c_char,
    f_len: size_t,
    mut skip_lookup: *mut *mut size_t,
    case_sensitive: libc::c_int,
) {
    let mut i: size_t = 0;
    let mut s_len: size_t = 0;
    let mut sl: *mut size_t = 0 as *mut size_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut last_prefix: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = ag_malloc(
        f_len.wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
    );
    sl = tmp as *mut size_t;
    *skip_lookup = sl;
    last_prefix = f_len;
    i = last_prefix;
    while i > 0 as libc::c_ulong {
        tmp___0 = is_prefix(find, f_len, i, case_sensitive);
        if tmp___0 != 0 {
            last_prefix = i;
        }
        *sl
            .offset(
                i.wrapping_sub(1 as libc::c_ulong) as isize,
            ) = last_prefix.wrapping_add(f_len.wrapping_sub(i));
        i = i.wrapping_sub(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < f_len {
        s_len = suffix_len(find, f_len, i, case_sensitive);
        if *find.offset(i.wrapping_sub(s_len) as isize) as libc::c_int
            != *find
                .offset(
                    f_len.wrapping_sub(1 as libc::c_ulong).wrapping_sub(s_len) as isize,
                ) as libc::c_int
        {
            *sl
                .offset(
                    f_len.wrapping_sub(1 as libc::c_ulong).wrapping_sub(s_len) as isize,
                ) = f_len
                .wrapping_sub(1 as libc::c_ulong)
                .wrapping_sub(i)
                .wrapping_add(s_len);
        }
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn ag_max(mut a: size_t, mut b: size_t) -> size_t {
    if b > a {
        return b;
    }
    return a;
}
pub unsafe extern "C" fn ag_min(mut a: size_t, mut b: size_t) -> size_t {
    if b < a {
        return b;
    }
    return a;
}
pub unsafe extern "C" fn generate_hash(
    mut find: *const libc::c_char,
    f_len: size_t,
    mut h_table___0: *mut uint8_t,
    case_sensitive: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut caps_set: libc::c_int = 0;
    let mut word: word_t = __anonunion_word_t_677777955 {
        as_chars: [0; 2],
    };
    let mut cap_index: libc::c_int = 0;
    let mut h: size_t = 0;
    i = f_len.wrapping_sub(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
        as libc::c_int;
    while i >= 0 as libc::c_int {
        caps_set = 0 as libc::c_int;
        while caps_set
            < (1 as libc::c_int) << ::std::mem::size_of::<uint16_t>() as libc::c_ulong
        {
            memcpy(
                &mut word.as_chars as *mut [libc::c_char; 2] as *mut libc::c_void,
                find.offset(i as isize) as *const libc::c_void,
                ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
            );
            cap_index = 0 as libc::c_int;
            while caps_set >> cap_index != 0 {
                if caps_set >> cap_index & 1 as libc::c_int != 0 {
                    word
                        .as_chars[cap_index
                        as usize] = (word.as_chars[cap_index as usize] as libc::c_int
                        - 32 as libc::c_int) as libc::c_char;
                }
                cap_index += 1;
            }
            h = (word.as_word as libc::c_int % 65536 as libc::c_int) as size_t;
            while *h_table___0.offset(h as isize) != 0 {
                h = h
                    .wrapping_add(1 as libc::c_ulong)
                    .wrapping_rem(65536 as libc::c_ulong);
            }
            *h_table___0.offset(h as isize) = (i + 1 as libc::c_int) as uint8_t;
            if case_sensitive != 0 {
                break;
            }
            caps_set += 1;
        }
        i -= 1;
    }
}
pub unsafe extern "C" fn boyer_moore_strnstr(
    mut s: *const libc::c_char,
    mut find: *const libc::c_char,
    s_len: size_t,
    f_len: size_t,
    mut alpha_skip_lookup___0: *const size_t,
    mut find_skip_lookup___0: *const size_t,
    case_insensitive: libc::c_int,
) -> *const libc::c_char {
    let mut i: ssize_t = 0;
    let mut pos: size_t = 0;
    let mut __res: libc::c_int = 0;
    let mut tmp___0: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    pos = f_len.wrapping_sub(1 as libc::c_ulong);
    while pos < s_len {
        i = f_len.wrapping_sub(1 as libc::c_ulong) as ssize_t;
        while i >= 0 as libc::c_long {
            if case_insensitive != 0 {
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_ulong
                {
                    __res = tolower(*s.offset(pos as isize) as libc::c_int);
                } else {
                    tmp___0 = __ctype_tolower_loc();
                    __res = *(*tmp___0)
                        .offset(*s.offset(pos as isize) as libc::c_int as isize);
                }
                tmp___1 = __res;
            } else {
                tmp___1 = *s.offset(pos as isize) as libc::c_int;
            }
            if !(tmp___1 == *find.offset(i as isize) as libc::c_int) {
                break;
            }
            pos = pos.wrapping_sub(1);
            i -= 1;
        }
        if i < 0 as libc::c_long {
            return s.offset(pos as isize).offset(1 as libc::c_int as isize);
        }
        tmp___2 = ag_max(
            *alpha_skip_lookup___0
                .offset(
                    *s.offset(pos as isize) as libc::c_uchar as libc::c_int as isize,
                ),
            *find_skip_lookup___0.offset(i as isize),
        );
        pos = (pos as libc::c_ulong).wrapping_add(tmp___2) as size_t as size_t;
    }
    return 0 as *mut libc::c_void as *const libc::c_char;
}
pub unsafe extern "C" fn hash_strnstr(
    mut s: *const libc::c_char,
    mut find: *const libc::c_char,
    s_len: size_t,
    f_len: size_t,
    mut h_table___0: *mut uint8_t,
    case_sensitive: libc::c_int,
) -> *const libc::c_char {
    let mut step: size_t = 0;
    let mut s_i: size_t = 0;
    let mut h: size_t = 0;
    let mut R: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0;
    let mut __res: libc::c_int = 0;
    let mut tmp___0: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___1: libc::c_int = 0;
    let mut i___0: size_t = 0;
    let mut R___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut s_c: libc::c_char = 0;
    let mut __res___0: libc::c_int = 0;
    let mut tmp___3: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut tmp___4: libc::c_int = 0;
    if s_len < f_len {
        return 0 as *mut libc::c_void as *const libc::c_char;
    }
    step = f_len
        .wrapping_sub(::std::mem::size_of::<uint16_t>() as libc::c_ulong)
        .wrapping_add(1 as libc::c_ulong);
    s_i = f_len.wrapping_sub(::std::mem::size_of::<uint16_t>() as libc::c_ulong);
    while s_i <= s_len.wrapping_sub(f_len) {
        h = (*(s.offset(s_i as isize) as *const uint16_t) as libc::c_int
            % 65536 as libc::c_int) as size_t;
        while *h_table___0.offset(h as isize) != 0 {
            let mut current_block_21: u64;
            R = s
                .offset(s_i as isize)
                .offset(
                    -((*h_table___0.offset(h as isize) as libc::c_int - 1 as libc::c_int)
                        as isize),
                );
            i = 0 as libc::c_int as size_t;
            loop {
                if !(i < f_len) {
                    current_block_21 = 13472856163611868459;
                    break;
                }
                if case_sensitive != 0 {
                    tmp___1 = *R.offset(i as isize) as libc::c_int;
                } else {
                    if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_ulong
                    {
                        __res = tolower(*R.offset(i as isize) as libc::c_int);
                    } else {
                        tmp___0 = __ctype_tolower_loc();
                        __res = *(*tmp___0)
                            .offset(*R.offset(i as isize) as libc::c_int as isize);
                    }
                    tmp___1 = __res;
                }
                if tmp___1 != *find.offset(i as isize) as libc::c_int {
                    current_block_21 = 1025420502087440762;
                    break;
                }
                i = i.wrapping_add(1);
            }
            match current_block_21 {
                13472856163611868459 => return R,
                _ => {
                    h = h
                        .wrapping_add(1 as libc::c_ulong)
                        .wrapping_rem(65536 as libc::c_ulong);
                }
            }
        }
        s_i = (s_i as libc::c_ulong).wrapping_add(step) as size_t as size_t;
    }
    s_i = s_i.wrapping_sub(step).wrapping_add(1 as libc::c_ulong);
    while s_i <= s_len.wrapping_sub(f_len) {
        let mut current_block_44: u64;
        R___0 = s.offset(s_i as isize);
        i___0 = 0 as libc::c_int as size_t;
        loop {
            if !(i___0 < f_len) {
                current_block_44 = 1356832168064818221;
                break;
            }
            if case_sensitive != 0 {
                tmp___4 = *R___0.offset(i___0 as isize) as libc::c_int;
            } else {
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_ulong
                {
                    __res___0 = tolower(*R___0.offset(i___0 as isize) as libc::c_int);
                } else {
                    tmp___3 = __ctype_tolower_loc();
                    __res___0 = *(*tmp___3)
                        .offset(*R___0.offset(i___0 as isize) as libc::c_int as isize);
                }
                tmp___4 = __res___0;
            }
            s_c = tmp___4 as libc::c_char;
            if s_c as libc::c_int != *find.offset(i___0 as isize) as libc::c_int {
                current_block_44 = 13211500007386619702;
                break;
            }
            i___0 = i___0.wrapping_add(1);
        }
        match current_block_44 {
            1356832168064818221 => return R___0,
            _ => {
                s_i = s_i.wrapping_add(1);
            }
        }
    }
    return 0 as *mut libc::c_void as *const libc::c_char;
}
pub unsafe extern "C" fn invert_matches(
    mut buf: *const libc::c_char,
    buf_len: size_t,
    mut matches: *mut match_t,
    mut matches_len: size_t,
) -> size_t {
    let mut i: size_t = 0;
    let mut match_read_index: size_t = 0;
    let mut inverted_match_count: size_t = 0;
    let mut inverted_match_start: size_t = 0;
    let mut last_line_end: size_t = 0;
    let mut in_inverted_match: libc::c_int = 0;
    let mut next_match: match_t = match_t { start: 0, end: 0 };
    match_read_index = 0 as libc::c_int as size_t;
    inverted_match_count = 0 as libc::c_int as size_t;
    inverted_match_start = 0 as libc::c_int as size_t;
    last_line_end = 0 as libc::c_int as size_t;
    in_inverted_match = 1 as libc::c_int;
    log_debug(
        b"Inverting %u matches.\0" as *const u8 as *const libc::c_char,
        matches_len,
    );
    if matches_len > 0 as libc::c_ulong {
        next_match = *matches.offset(0 as libc::c_int as isize);
    } else {
        next_match.start = buf_len.wrapping_add(1 as libc::c_ulong);
    }
    if matches_len == 0 as libc::c_ulong {
        (*matches.offset(0 as libc::c_int as isize)).start = 0 as libc::c_int as size_t;
        (*matches.offset(0 as libc::c_int as isize))
            .end = buf_len.wrapping_sub(1 as libc::c_ulong);
        return 1 as libc::c_int as size_t;
    }
    i = 0 as libc::c_int as size_t;
    while i < buf_len {
        if i == next_match.start {
            i = (next_match.end).wrapping_sub(1 as libc::c_ulong);
            match_read_index = match_read_index.wrapping_add(1);
            if match_read_index < matches_len {
                next_match = *matches.offset(match_read_index as isize);
            }
            if in_inverted_match != 0 {
                if last_line_end > inverted_match_start {
                    (*matches.offset(inverted_match_count as isize))
                        .start = inverted_match_start;
                    (*matches.offset(inverted_match_count as isize))
                        .end = last_line_end.wrapping_sub(1 as libc::c_ulong);
                    inverted_match_count = inverted_match_count.wrapping_add(1);
                }
            }
            in_inverted_match = 0 as libc::c_int;
        } else {
            let mut current_block_41: u64;
            if i == buf_len.wrapping_sub(1 as libc::c_ulong) {
                if in_inverted_match != 0 {
                    (*matches.offset(inverted_match_count as isize))
                        .start = inverted_match_start;
                    (*matches.offset(inverted_match_count as isize)).end = i;
                    inverted_match_count = inverted_match_count.wrapping_add(1);
                    current_block_41 = 8845338526596852646;
                } else {
                    current_block_41 = 4024880799860930278;
                }
            } else {
                current_block_41 = 4024880799860930278;
            }
            match current_block_41 {
                4024880799860930278 => {
                    if *buf.offset(i as isize) as libc::c_int == 10 as libc::c_int {
                        last_line_end = i.wrapping_add(1 as libc::c_ulong);
                        if in_inverted_match == 0 {
                            inverted_match_start = last_line_end;
                        }
                        in_inverted_match = 1 as libc::c_int;
                    }
                }
                _ => {}
            }
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as size_t;
    while i < matches_len {
        log_debug(
            b"Inverted match %i start %i end %i.\0" as *const u8 as *const libc::c_char,
            i,
            (*matches.offset(i as isize)).start,
            (*matches.offset(i as isize)).end,
        );
        i = i.wrapping_add(1);
    }
    return inverted_match_count;
}
pub unsafe extern "C" fn realloc_matches(
    mut matches: *mut *mut match_t,
    mut matches_size: *mut size_t,
    mut matches_len: size_t,
) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if matches_len < *matches_size {
        return;
    }
    if !(*matches).is_null() {
        *matches_size = (*matches_size as libc::c_ulong).wrapping_mul(2 as libc::c_ulong)
            as size_t as size_t;
    } else {
        *matches_size = 100 as libc::c_int as size_t;
    }
    tmp = ag_realloc(
        *matches as *mut libc::c_void,
        (*matches_size).wrapping_mul(::std::mem::size_of::<match_t>() as libc::c_ulong),
    );
    *matches = tmp as *mut match_t;
}
pub unsafe extern "C" fn compile_study(
    mut re: *mut *mut pcre,
    mut re_extra: *mut *mut pcre_extra,
    mut q: *mut libc::c_char,
    pcre_opts: libc::c_int,
    study_opts: libc::c_int,
) {
    let mut pcre_err: *const libc::c_char = 0 as *const libc::c_char;
    let mut pcre_err_offset: libc::c_int = 0;
    pcre_err = 0 as *mut libc::c_void as *const libc::c_char;
    pcre_err_offset = 0 as libc::c_int;
    *re = pcre_compile(
        q as *const libc::c_char,
        pcre_opts,
        &mut pcre_err,
        &mut pcre_err_offset,
        0 as *mut libc::c_void as *const libc::c_uchar,
    );
    if *re as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        die(
            b"Bad regex! pcre_compile() failed at position %i: %s\nIf you meant to search for a literal string, run ag with -Q\0"
                as *const u8 as *const libc::c_char,
            pcre_err_offset,
            pcre_err,
        );
    }
    *re_extra = pcre_study(*re as *const pcre, study_opts, &mut pcre_err);
    if *re_extra as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        log_debug(
            b"pcre_study returned nothing useful. Error: %s\0" as *const u8
                as *const libc::c_char,
            pcre_err,
        );
    }
}
pub unsafe extern "C" fn is_binary(
    mut buf: *const libc::c_void,
    buf_len: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut suspicious_bytes: size_t = 0;
    let mut total_bytes: size_t = 0;
    let mut tmp: size_t = 0;
    let mut buf_c: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut i: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    suspicious_bytes = 0 as libc::c_int as size_t;
    if buf_len > 512 as libc::c_ulong {
        tmp = 512 as libc::c_int as size_t;
    } else {
        tmp = buf_len;
    }
    total_bytes = tmp;
    buf_c = buf as *const libc::c_uchar;
    if buf_len == 0 as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if buf_len >= 3 as libc::c_ulong {
        if *buf_c.offset(0 as libc::c_int as isize) as libc::c_int == 239 as libc::c_int
        {
            if *buf_c.offset(1 as libc::c_int as isize) as libc::c_int
                == 187 as libc::c_int
            {
                if *buf_c.offset(2 as libc::c_int as isize) as libc::c_int
                    == 191 as libc::c_int
                {
                    return 0 as libc::c_int;
                }
            }
        }
    }
    if buf_len >= 5 as libc::c_ulong {
        tmp___0 = strncmp(
            buf as *const libc::c_char,
            b"%PDF-\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
        if tmp___0 == 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i < total_bytes {
        if *buf_c.offset(i as isize) as libc::c_int == 0 as libc::c_int {
            return 1 as libc::c_int
        } else {
            if (*buf_c.offset(i as isize) as libc::c_int) < 7 as libc::c_int {
                current_block = 5549600645010791694;
            } else if *buf_c.offset(i as isize) as libc::c_int > 14 as libc::c_int {
                current_block = 5549600645010791694;
            } else {
                current_block = 7573349114012108563;
            }
            match current_block {
                5549600645010791694 => {
                    if (*buf_c.offset(i as isize) as libc::c_int) < 32 as libc::c_int {
                        current_block = 1377639193732061151;
                    } else if *buf_c.offset(i as isize) as libc::c_int
                            > 127 as libc::c_int
                        {
                        current_block = 1377639193732061151;
                    } else {
                        current_block = 7573349114012108563;
                    }
                    match current_block {
                        7573349114012108563 => {}
                        _ => {
                            if *buf_c.offset(i as isize) as libc::c_int
                                > 193 as libc::c_int
                            {
                                if (*buf_c.offset(i as isize) as libc::c_int)
                                    < 224 as libc::c_int
                                {
                                    if i.wrapping_add(1 as libc::c_ulong) < total_bytes {
                                        i = i.wrapping_add(1);
                                        if *buf_c.offset(i as isize) as libc::c_int
                                            > 127 as libc::c_int
                                        {
                                            if (*buf_c.offset(i as isize) as libc::c_int)
                                                < 192 as libc::c_int
                                            {
                                                current_block = 7573349114012108563;
                                            } else {
                                                current_block = 3689906465960840878;
                                            }
                                        } else {
                                            current_block = 3689906465960840878;
                                        }
                                    } else {
                                        current_block = 4372851404596137843;
                                    }
                                } else {
                                    current_block = 4372851404596137843;
                                }
                            } else {
                                current_block = 4372851404596137843;
                            }
                            match current_block {
                                7573349114012108563 => {}
                                _ => {
                                    match current_block {
                                        4372851404596137843 => {
                                            if *buf_c.offset(i as isize) as libc::c_int
                                                > 223 as libc::c_int
                                            {
                                                if (*buf_c.offset(i as isize) as libc::c_int)
                                                    < 240 as libc::c_int
                                                {
                                                    if i.wrapping_add(2 as libc::c_ulong) < total_bytes {
                                                        i = i.wrapping_add(1);
                                                        if *buf_c.offset(i as isize) as libc::c_int
                                                            > 127 as libc::c_int
                                                        {
                                                            if (*buf_c.offset(i as isize) as libc::c_int)
                                                                < 192 as libc::c_int
                                                            {
                                                                if *buf_c
                                                                    .offset(i.wrapping_add(1 as libc::c_ulong) as isize)
                                                                    as libc::c_int > 127 as libc::c_int
                                                                {
                                                                    if (*buf_c
                                                                        .offset(i.wrapping_add(1 as libc::c_ulong) as isize)
                                                                        as libc::c_int) < 192 as libc::c_int
                                                                    {
                                                                        i = i.wrapping_add(1);
                                                                        current_block = 7573349114012108563;
                                                                    } else {
                                                                        current_block = 3689906465960840878;
                                                                    }
                                                                } else {
                                                                    current_block = 3689906465960840878;
                                                                }
                                                            } else {
                                                                current_block = 3689906465960840878;
                                                            }
                                                        } else {
                                                            current_block = 3689906465960840878;
                                                        }
                                                    } else {
                                                        current_block = 3689906465960840878;
                                                    }
                                                } else {
                                                    current_block = 3689906465960840878;
                                                }
                                            } else {
                                                current_block = 3689906465960840878;
                                            }
                                        }
                                        _ => {}
                                    }
                                    match current_block {
                                        7573349114012108563 => {}
                                        _ => {
                                            suspicious_bytes = suspicious_bytes.wrapping_add(1);
                                            if i >= 32 as libc::c_ulong {
                                                if suspicious_bytes
                                                    .wrapping_mul(100 as libc::c_ulong)
                                                    .wrapping_div(total_bytes) > 10 as libc::c_ulong
                                                {
                                                    return 1 as libc::c_int;
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
            i = i.wrapping_add(1);
        }
    }
    if suspicious_bytes.wrapping_mul(100 as libc::c_ulong).wrapping_div(total_bytes)
        > 10 as libc::c_ulong
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn is_regex(mut query: *const libc::c_char) -> libc::c_int {
    let mut regex_chars: [libc::c_char; 13] = [0; 13];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    regex_chars[0 as libc::c_int as usize] = '$' as i32 as libc::c_char;
    regex_chars[1 as libc::c_int as usize] = '(' as i32 as libc::c_char;
    regex_chars[2 as libc::c_int as usize] = ')' as i32 as libc::c_char;
    regex_chars[3 as libc::c_int as usize] = '*' as i32 as libc::c_char;
    regex_chars[4 as libc::c_int as usize] = '+' as i32 as libc::c_char;
    regex_chars[5 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    regex_chars[6 as libc::c_int as usize] = '?' as i32 as libc::c_char;
    regex_chars[7 as libc::c_int as usize] = '[' as i32 as libc::c_char;
    regex_chars[8 as libc::c_int as usize] = '\\' as i32 as libc::c_char;
    regex_chars[9 as libc::c_int as usize] = '^' as i32 as libc::c_char;
    regex_chars[10 as libc::c_int as usize] = '{' as i32 as libc::c_char;
    regex_chars[11 as libc::c_int as usize] = '|' as i32 as libc::c_char;
    regex_chars[12 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tmp = strpbrk(query, regex_chars.as_mut_ptr() as *const libc::c_char);
    return (tmp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int;
}
pub unsafe extern "C" fn is_fnmatch(mut filename: *const libc::c_char) -> libc::c_int {
    let mut fnmatch_chars: [libc::c_char; 6] = [0; 6];
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    fnmatch_chars[0 as libc::c_int as usize] = '!' as i32 as libc::c_char;
    fnmatch_chars[1 as libc::c_int as usize] = '*' as i32 as libc::c_char;
    fnmatch_chars[2 as libc::c_int as usize] = '?' as i32 as libc::c_char;
    fnmatch_chars[3 as libc::c_int as usize] = '[' as i32 as libc::c_char;
    fnmatch_chars[4 as libc::c_int as usize] = ']' as i32 as libc::c_char;
    fnmatch_chars[5 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tmp = strpbrk(filename, fnmatch_chars.as_mut_ptr() as *const libc::c_char);
    return (tmp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int;
}
pub unsafe extern "C" fn binary_search(
    mut needle: *const libc::c_char,
    mut haystack: *mut *mut libc::c_char,
    mut start: libc::c_int,
    mut end: libc::c_int,
) -> libc::c_int {
    let mut mid: libc::c_int = 0;
    let mut rc: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if start == end {
        return -(1 as libc::c_int);
    }
    mid = start + (end - start) / 2 as libc::c_int;
    rc = strcmp(needle, *haystack.offset(mid as isize) as *const libc::c_char);
    if rc < 0 as libc::c_int {
        tmp = binary_search(needle, haystack, start, mid);
        return tmp;
    } else {
        if rc > 0 as libc::c_int {
            tmp___0 = binary_search(needle, haystack, mid + 1 as libc::c_int, end);
            return tmp___0;
        }
    }
    return mid;
}
static mut wordchar_table: [libc::c_int; 256] = [0; 256];
pub unsafe extern "C" fn init_wordchar_table() {
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        ch = i as libc::c_char;
        let mut current_block_14: u64;
        if 97 as libc::c_int <= ch as libc::c_int {
            if ch as libc::c_int <= 122 as libc::c_int {
                tmp = 1 as libc::c_int;
                current_block_14 = 18317007320854588510;
            } else {
                current_block_14 = 7430350639616099220;
            }
        } else {
            current_block_14 = 7430350639616099220;
        }
        match current_block_14 {
            7430350639616099220 => {
                let mut current_block_13: u64;
                if 65 as libc::c_int <= ch as libc::c_int {
                    if ch as libc::c_int <= 90 as libc::c_int {
                        tmp = 1 as libc::c_int;
                        current_block_13 = 4495394744059808450;
                    } else {
                        current_block_13 = 17945066879964504463;
                    }
                } else {
                    current_block_13 = 17945066879964504463;
                }
                match current_block_13 {
                    17945066879964504463 => {
                        let mut current_block_12: u64;
                        if 48 as libc::c_int <= ch as libc::c_int {
                            if ch as libc::c_int <= 57 as libc::c_int {
                                tmp = 1 as libc::c_int;
                                current_block_12 = 26972500619410423;
                            } else {
                                current_block_12 = 3354907004016569123;
                            }
                        } else {
                            current_block_12 = 3354907004016569123;
                        }
                        match current_block_12 {
                            3354907004016569123 => {
                                if ch as libc::c_int == 95 as libc::c_int {
                                    tmp = 1 as libc::c_int;
                                } else {
                                    tmp = 0 as libc::c_int;
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        wordchar_table[i as usize] = tmp;
        i += 1;
    }
}
pub unsafe extern "C" fn is_wordchar(mut ch: libc::c_char) -> libc::c_int {
    return wordchar_table[ch as libc::c_uchar as usize];
}
pub unsafe extern "C" fn is_lowercase(mut s: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    i = 0 as libc::c_int;
    while *s.offset(i as isize) as libc::c_int != 0 as libc::c_int {
        if !(*s.offset(i as isize) as libc::c_int & -(128 as libc::c_int)
            == 0 as libc::c_int)
        {
            return 0 as libc::c_int
        } else {
            tmp = __ctype_b_loc();
            if *(*tmp).offset(*s.offset(i as isize) as libc::c_int as isize)
                as libc::c_int & 256 as libc::c_int != 0
            {
                return 0 as libc::c_int;
            }
        }
        i += 1;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn is_directory(
    mut path: *const libc::c_char,
    mut d: *const dirent,
) -> libc::c_int {
    let mut full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut tmp: libc::c_int = 0;
    let mut is_dir: libc::c_int = 0;
    if (*d).d_type as libc::c_int != 0 as libc::c_int {
        if (*d).d_type as libc::c_int != 10 as libc::c_int {
            return ((*d).d_type as libc::c_int == 4 as libc::c_int) as libc::c_int;
        }
    }
    ag_asprintf(
        &mut full_path as *mut *mut libc::c_char,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        path,
        ((*d).d_name).as_ptr(),
    );
    tmp = stat(full_path as *const libc::c_char, &mut s as *mut stat);
    if tmp != 0 as libc::c_int {
        free(full_path as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    is_dir = (s.st_mode & 61440 as libc::c_uint == 16384 as libc::c_uint) as libc::c_int;
    free(full_path as *mut libc::c_void);
    return is_dir;
}
pub unsafe extern "C" fn is_symlink(
    mut path: *const libc::c_char,
    mut d: *const dirent,
) -> libc::c_int {
    let mut full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut tmp: libc::c_int = 0;
    if (*d).d_type as libc::c_int != 0 as libc::c_int {
        return ((*d).d_type as libc::c_int == 10 as libc::c_int) as libc::c_int;
    }
    ag_asprintf(
        &mut full_path as *mut *mut libc::c_char,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        path,
        ((*d).d_name).as_ptr(),
    );
    tmp = lstat(full_path as *const libc::c_char, &mut s as *mut stat);
    if tmp != 0 as libc::c_int {
        free(full_path as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    free(full_path as *mut libc::c_void);
    return (s.st_mode & 61440 as libc::c_uint == 40960 as libc::c_uint) as libc::c_int;
}
pub unsafe extern "C" fn is_named_pipe(
    mut path: *const libc::c_char,
    mut d: *const dirent,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut full_path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if (*d).d_type as libc::c_int != 0 as libc::c_int {
        if (*d).d_type as libc::c_int != 10 as libc::c_int {
            if (*d).d_type as libc::c_int == 1 as libc::c_int {
                tmp = 1 as libc::c_int;
            } else if (*d).d_type as libc::c_int == 12 as libc::c_int {
                tmp = 1 as libc::c_int;
            } else {
                tmp = 0 as libc::c_int;
            }
            return tmp;
        }
    }
    ag_asprintf(
        &mut full_path as *mut *mut libc::c_char,
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        path,
        ((*d).d_name).as_ptr(),
    );
    tmp___0 = stat(full_path as *const libc::c_char, &mut s as *mut stat);
    if tmp___0 != 0 as libc::c_int {
        free(full_path as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    free(full_path as *mut libc::c_void);
    if s.st_mode & 61440 as libc::c_uint == 4096 as libc::c_uint {
        tmp___1 = 1 as libc::c_int;
    } else if s.st_mode & 61440 as libc::c_uint == 49152 as libc::c_uint {
        tmp___1 = 1 as libc::c_int;
    } else {
        tmp___1 = 0 as libc::c_int;
    }
    return tmp___1;
}
pub unsafe extern "C" fn ag_asprintf(
    mut ret: *mut *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    let mut tmp: libc::c_int = 0;
    args_0 = args.clone();
    tmp = vasprintf(ret, fmt, args_0.as_va_list());
    if tmp == -(1 as libc::c_int) {
        die(b"vasprintf returned -1\0" as *const u8 as *const libc::c_char);
    }
}
pub unsafe extern "C" fn die(mut fmt: *const libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    args_0 = args.clone();
    vplog(40 as libc::c_int as libc::c_uint, fmt, args_0.as_va_list());
    exit(2 as libc::c_int);
}
pub unsafe extern "C" fn fgetln(
    mut fp: *mut FILE,
    mut lenp: *mut size_t,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut used: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut nsize: size_t = 0;
    let mut newbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    buf = 0 as *mut libc::c_void as *mut libc::c_char;
    used = 0 as libc::c_int;
    len = 0 as libc::c_int;
    flockfile(fp);
    loop {
        c = getc_unlocked(fp);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        let mut current_block_16: u64;
        if buf.is_null() {
            current_block_16 = 18014380726844792305;
        } else if len >= used {
            current_block_16 = 18014380726844792305;
        } else {
            current_block_16 = 12124785117276362961;
        }
        match current_block_16 {
            18014380726844792305 => {
                nsize = (used + 8192 as libc::c_int) as size_t;
                tmp = realloc(buf as *mut libc::c_void, nsize);
                newbuf = tmp as *mut libc::c_char;
                if newbuf.is_null() {
                    funlockfile(fp);
                    if !buf.is_null() {
                        free(buf as *mut libc::c_void);
                    }
                    return 0 as *mut libc::c_void as *mut libc::c_char;
                }
                buf = newbuf;
                used = nsize as libc::c_int;
            }
            _ => {}
        }
        tmp___0 = len;
        len += 1;
        *buf.offset(tmp___0 as isize) = c as libc::c_char;
        if c == 10 as libc::c_int {
            break;
        }
    }
    funlockfile(fp);
    *lenp = len as size_t;
    return buf;
}
pub unsafe extern "C" fn buf_getline(
    mut line: *mut *const libc::c_char,
    mut buf: *const libc::c_char,
    buf_len: size_t,
    buf_offset: size_t,
) -> ssize_t {
    let mut cur: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: ssize_t = 0;
    cur = buf.offset(buf_offset as isize);
    i = 0 as libc::c_int as ssize_t;
    while buf_offset.wrapping_add(i as size_t) < buf_len {
        if !(*cur.offset(i as isize) as libc::c_int != 10 as libc::c_int) {
            break;
        }
        i += 1;
    }
    *line = cur;
    return i;
}
pub unsafe extern "C" fn strlcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
    mut size: size_t,
) -> size_t {
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: size_t = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_char = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    d = dst;
    s = src;
    n = size;
    if n != 0 as libc::c_ulong {
        loop {
            n = n.wrapping_sub(1);
            if !(n != 0 as libc::c_ulong) {
                break;
            }
            tmp = d;
            d = d.offset(1);
            tmp___1 = s;
            s = s.offset(1);
            tmp___0 = *tmp___1;
            *tmp = tmp___0;
            if tmp___0 as libc::c_int == 0 as libc::c_int {
                break;
            }
        }
    }
    if n == 0 as libc::c_ulong {
        if size != 0 as libc::c_ulong {
            *d = '\u{0}' as i32 as libc::c_char;
        }
        loop {
            tmp___2 = s;
            s = s.offset(1);
            if *tmp___2 == 0 {
                break;
            }
        }
    }
    return (s.offset_from(src) as libc::c_long - 1 as libc::c_long) as size_t;
}
pub static mut XZ_HEADER_MAGIC: [uint8_t; 6] = [
    253 as libc::c_int as uint8_t,
    '7' as i32 as uint8_t,
    'z' as i32 as uint8_t,
    'X' as i32 as uint8_t,
    'Z' as i32 as uint8_t,
    0 as libc::c_int as uint8_t,
];
pub static mut LZMA_HEADER_SOMETIMES: [uint8_t; 3] = [
    93 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
];
unsafe extern "C" fn decompress_zlib(
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
    mut dir_full_path: *const libc::c_char,
    mut new_buf_len: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0;
    let mut result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut result_size: size_t = 0;
    let mut pagesize: size_t = 0;
    let mut stream: z_stream = z_stream {
        next_in: 0 as *const Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *const libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp_result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = 0 as libc::c_int;
    result = 0 as *mut libc::c_void as *mut libc::c_uchar;
    result_size = 0 as libc::c_int as size_t;
    pagesize = 0 as libc::c_int as size_t;
    log_debug(
        b"Decompressing zlib file %s\0" as *const u8 as *const libc::c_char,
        dir_full_path,
    );
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = 0 as voidpf;
    stream.avail_in = 0 as libc::c_int as uInt;
    stream.next_in = 0 as *const Bytef;
    tmp = inflateInit2_(
        &mut stream,
        47 as libc::c_int,
        b"1.2.11\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if tmp != 0 as libc::c_int {
        log_err(
            b"Unable to initialize zlib: %s\0" as *const u8 as *const libc::c_char,
            stream.msg,
        );
    } else {
        stream.avail_in = buf_len as uInt;
        stream.next_in = buf as *mut Bytef as *const Bytef;
        tmp___0 = getpagesize();
        pagesize = tmp___0 as size_t;
        result_size = (buf_len as size_t)
            .wrapping_add(pagesize)
            .wrapping_sub(1 as libc::c_ulong)
            & !pagesize.wrapping_sub(1 as libc::c_ulong);
        loop {
            tmp_result = result;
            result_size = (result_size as libc::c_ulong).wrapping_mul(2 as libc::c_ulong)
                as size_t as size_t;
            tmp___1 = realloc(
                result as *mut libc::c_void,
                result_size
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ),
            );
            result = tmp___1 as *mut libc::c_uchar;
            if result as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                free(tmp_result as *mut libc::c_void);
                log_err(
                    b"Unable to allocate %d bytes to decompress file %s\0" as *const u8
                        as *const libc::c_char,
                    result_size
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                    dir_full_path,
                );
                inflateEnd(&mut stream);
                current_block = 3415156120986101164;
                break;
            } else {
                stream.avail_out = result_size.wrapping_div(2 as libc::c_ulong) as uInt;
                stream.next_out = result.offset(stream.total_out as isize);
                ret = inflate(&mut stream, 2 as libc::c_int);
                log_debug(
                    b"inflate ret = %d\0" as *const u8 as *const libc::c_char,
                    ret,
                );
                match ret {
                    -2 => {
                        log_err(
                            b"Found stream error while decompressing zlib stream: %s\0"
                                as *const u8 as *const libc::c_char,
                            stream.msg,
                        );
                        inflateEnd(&mut stream);
                        current_block = 3415156120986101164;
                        break;
                    }
                    -4 | -3 | 2 => {
                        log_err(
                            b"Found mem/data error while decompressing zlib stream: %s\0"
                                as *const u8 as *const libc::c_char,
                            stream.msg,
                        );
                        inflateEnd(&mut stream);
                        current_block = 3415156120986101164;
                        break;
                    }
                    _ => {
                        if stream.avail_out == 0 as libc::c_uint {
                            continue;
                        }
                        if !(ret == 0 as libc::c_int) {
                            current_block = 7333393191927787629;
                            break;
                        }
                    }
                }
            }
        }
        match current_block {
            3415156120986101164 => {}
            _ => {
                *new_buf_len = stream.total_out as libc::c_int;
                inflateEnd(&mut stream);
                if ret == 1 as libc::c_int {
                    return result as *mut libc::c_void;
                }
            }
        }
    }
    *new_buf_len = 0 as libc::c_int;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn decompress_lzw(
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
    mut dir_full_path: *const libc::c_char,
    mut new_buf_len: *mut libc::c_int,
) -> *mut libc::c_void {
    log_err(
        b"LZW (UNIX compress) files not yet supported: %s\0" as *const u8
            as *const libc::c_char,
        dir_full_path,
    );
    *new_buf_len = 0 as libc::c_int;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn decompress_zip(
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
    mut dir_full_path: *const libc::c_char,
    mut new_buf_len: *mut libc::c_int,
) -> *mut libc::c_void {
    log_err(
        b"Zip files not yet supported: %s\0" as *const u8 as *const libc::c_char,
        dir_full_path,
    );
    *new_buf_len = 0 as libc::c_int;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn decompress_lzma(
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
    mut dir_full_path: *const libc::c_char,
    mut new_buf_len: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut stream: lzma_stream = lzma_stream {
        next_in: 0 as *const uint8_t,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut uint8_t,
        avail_out: 0,
        total_out: 0,
        allocator: 0 as *const lzma_allocator,
        internal: 0 as *mut lzma_internal,
        reserved_ptr1: 0 as *mut libc::c_void,
        reserved_ptr2: 0 as *mut libc::c_void,
        reserved_ptr3: 0 as *mut libc::c_void,
        reserved_ptr4: 0 as *mut libc::c_void,
        reserved_int1: 0,
        reserved_int2: 0,
        reserved_int3: 0,
        reserved_int4: 0,
        reserved_enum1: LZMA_RESERVED_ENUM,
        reserved_enum2: LZMA_RESERVED_ENUM,
    };
    let mut lzrt: lzma_ret = LZMA_OK;
    let mut result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut result_size: size_t = 0;
    let mut pagesize: size_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp_result: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    stream.next_in = 0 as *mut libc::c_void as *const uint8_t;
    stream.avail_in = 0 as libc::c_int as size_t;
    stream.total_in = 0 as libc::c_int as uint64_t;
    stream.next_out = 0 as *mut libc::c_void as *mut uint8_t;
    stream.avail_out = 0 as libc::c_int as size_t;
    stream.total_out = 0 as libc::c_int as uint64_t;
    stream.allocator = 0 as *mut libc::c_void as *const lzma_allocator;
    stream.internal = 0 as *mut libc::c_void as *mut lzma_internal;
    stream.reserved_ptr1 = 0 as *mut libc::c_void;
    stream.reserved_ptr2 = 0 as *mut libc::c_void;
    stream.reserved_ptr3 = 0 as *mut libc::c_void;
    stream.reserved_ptr4 = 0 as *mut libc::c_void;
    stream.reserved_int1 = 0 as libc::c_int as uint64_t;
    stream.reserved_int2 = 0 as libc::c_int as uint64_t;
    stream.reserved_int3 = 0 as libc::c_int as size_t;
    stream.reserved_int4 = 0 as libc::c_int as size_t;
    stream.reserved_enum1 = LZMA_RESERVED_ENUM;
    stream.reserved_enum2 = LZMA_RESERVED_ENUM;
    result = 0 as *mut libc::c_void as *mut libc::c_uchar;
    result_size = 0 as libc::c_int as size_t;
    pagesize = 0 as libc::c_int as size_t;
    stream.avail_in = buf_len as size_t;
    stream.next_in = buf as *const uint8_t;
    lzrt = lzma_auto_decoder(
        &mut stream,
        -(1 as libc::c_int) as uint64_t,
        0 as libc::c_int as uint32_t,
    );
    if lzrt as libc::c_uint != 0 as libc::c_uint {
        log_err(
            b"Unable to initialize lzma_auto_decoder: %d\0" as *const u8
                as *const libc::c_char,
            lzrt as libc::c_uint,
        );
    } else {
        tmp = getpagesize();
        pagesize = tmp as size_t;
        result_size = (buf_len as size_t)
            .wrapping_add(pagesize)
            .wrapping_sub(1 as libc::c_ulong)
            & !pagesize.wrapping_sub(1 as libc::c_ulong);
        loop {
            tmp_result = result;
            result_size = (result_size as libc::c_ulong).wrapping_mul(2 as libc::c_ulong)
                as size_t as size_t;
            tmp___0 = realloc(
                result as *mut libc::c_void,
                result_size
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                    ),
            );
            result = tmp___0 as *mut libc::c_uchar;
            if result as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                free(tmp_result as *mut libc::c_void);
                log_err(
                    b"Unable to allocate %d bytes to decompress file %s\0" as *const u8
                        as *const libc::c_char,
                    result_size
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong,
                        ),
                    dir_full_path,
                );
                current_block = 4697502143554202540;
                break;
            } else {
                stream.avail_out = result_size.wrapping_div(2 as libc::c_ulong);
                stream.next_out = result.offset(stream.total_out as isize);
                lzrt = lzma_code(&mut stream, LZMA_RUN);
                log_debug(
                    b"lzma_code ret = %d\0" as *const u8 as *const libc::c_char,
                    lzrt as libc::c_uint,
                );
                match lzrt as libc::c_uint {
                    1 | 0 => {
                        if stream.avail_out == 0 as libc::c_ulong {
                            continue;
                        }
                        if !(lzrt as libc::c_uint == 0 as libc::c_uint) {
                            current_block = 4090602189656566074;
                            break;
                        }
                    }
                    _ => {
                        log_err(
                            b"Found mem/data error while decompressing xz/lzma stream: %d\0"
                                as *const u8 as *const libc::c_char,
                            lzrt as libc::c_uint,
                        );
                        current_block = 4697502143554202540;
                        break;
                    }
                }
            }
        }
        match current_block {
            4697502143554202540 => {}
            _ => {
                *new_buf_len = stream.total_out as libc::c_int;
                if lzrt as libc::c_uint == 1 as libc::c_uint {
                    lzma_end(&mut stream);
                    return result as *mut libc::c_void;
                }
            }
        }
    }
    lzma_end(&mut stream);
    *new_buf_len = 0 as libc::c_int;
    if !result.is_null() {
        free(result as *mut libc::c_void);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn decompress(
    zip_type: ag_compression_type,
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
    mut dir_full_path: *const libc::c_char,
    mut new_buf_len: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    match zip_type as libc::c_uint {
        1 => {
            tmp = decompress_zlib(buf, buf_len, dir_full_path, new_buf_len);
            return tmp;
        }
        2 => {
            tmp___0 = decompress_lzw(buf, buf_len, dir_full_path, new_buf_len);
            return tmp___0;
        }
        3 => {
            tmp___1 = decompress_zip(buf, buf_len, dir_full_path, new_buf_len);
            return tmp___1;
        }
        4 => {
            tmp___2 = decompress_lzma(buf, buf_len, dir_full_path, new_buf_len);
            return tmp___2;
        }
        0 => {
            log_err(
                b"File %s is not compressed\0" as *const u8 as *const libc::c_char,
                dir_full_path,
            );
        }
        _ => {
            log_err(
                b"Unsupported compression type: %d\0" as *const u8
                    as *const libc::c_char,
                zip_type as libc::c_uint,
            );
        }
    }
    *new_buf_len = 0 as libc::c_int;
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn is_zipped(
    mut buf: *const libc::c_void,
    buf_len: libc::c_int,
) -> ag_compression_type {
    let mut buf_c: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    buf_c = buf as *const libc::c_uchar;
    if buf_len == 0 as libc::c_int {
        return AG_NO_COMPRESSION;
    }
    if buf_len >= 2 as libc::c_int {
        if *buf_c.offset(0 as libc::c_int as isize) as libc::c_int == 31 as libc::c_int {
            if *buf_c.offset(1 as libc::c_int as isize) as libc::c_int
                == 139 as libc::c_int
            {
                log_debug(
                    b"Found gzip-based stream\0" as *const u8 as *const libc::c_char,
                );
                return AG_GZIP;
            } else {
                if *buf_c.offset(1 as libc::c_int as isize) as libc::c_int
                    == 155 as libc::c_int
                {
                    log_debug(
                        b"Found compress-based stream\0" as *const u8
                            as *const libc::c_char,
                    );
                    return AG_COMPRESS;
                }
            }
        }
    }
    if buf_len >= 4 as libc::c_int {
        if *buf_c.offset(0 as libc::c_int as isize) as libc::c_int == 80 as libc::c_int {
            if *buf_c.offset(1 as libc::c_int as isize) as libc::c_int
                == 75 as libc::c_int
            {
                if *buf_c.offset(2 as libc::c_int as isize) as libc::c_int
                    == 3 as libc::c_int
                {
                    if *buf_c.offset(3 as libc::c_int as isize) as libc::c_int
                        == 4 as libc::c_int
                    {
                        log_debug(
                            b"Found zip-based stream\0" as *const u8
                                as *const libc::c_char,
                        );
                        return AG_ZIP;
                    }
                }
            }
        }
    }
    if buf_len >= 6 as libc::c_int {
        tmp = memcmp(
            XZ_HEADER_MAGIC.as_ptr() as *const libc::c_void,
            buf_c as *const libc::c_void,
            6 as libc::c_int as size_t,
        );
        if tmp == 0 as libc::c_int {
            log_debug(b"Found xz based stream\0" as *const u8 as *const libc::c_char);
            return AG_XZ;
        }
    }
    if buf_len >= 3 as libc::c_int {
        tmp___0 = memcmp(
            LZMA_HEADER_SOMETIMES.as_ptr() as *const libc::c_void,
            buf_c as *const libc::c_void,
            3 as libc::c_int as size_t,
        );
        if tmp___0 == 0 as libc::c_int {
            log_debug(b"Found lzma-based stream\0" as *const u8 as *const libc::c_char);
            return AG_XZ;
        }
    }
    return AG_NO_COMPRESSION;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut base_paths: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut paths: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut pcre_opts: libc::c_int = 0;
    let mut study_opts: libc::c_int = 0;
    let mut workers: *mut worker_t = 0 as *mut worker_t;
    let mut workers_len: libc::c_int = 0;
    let mut num_cores: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut has_jit: libc::c_int = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut __res: libc::c_int = 0;
    let mut tmp___9: *mut *const __int32_t = 0 as *mut *const __int32_t;
    let mut word_regexp_query: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: size_t = 0;
    let mut rv: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cpu_set: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    let mut __cpu: size_t = 0;
    let mut tmp___13: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ig: *mut ignores = 0 as *mut ignores;
    let mut tmp___14: *mut ignores = 0 as *mut ignores;
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut time_diff: libc::c_double = 0.;
    base_paths = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    paths = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    pcre_opts = 2 as libc::c_int;
    study_opts = 0 as libc::c_int;
    workers = 0 as *mut libc::c_void as *mut worker_t;
    set_log_level(LOG_LEVEL_WARN);
    work_queue = 0 as *mut libc::c_void as *mut work_queue_t;
    work_queue_tail = 0 as *mut libc::c_void as *mut work_queue_t;
    root_ignores = init_ignore(
        0 as *mut libc::c_void as *mut ignores,
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as size_t,
    );
    out_fd = stdout;
    parse_options(argc, argv, &mut base_paths, &mut paths);
    tmp = pcre_version();
    log_debug(b"PCRE Version: %s\0" as *const u8 as *const libc::c_char, tmp);
    if opts.stats != 0 {
        memset(
            &mut stats as *mut ag_stats as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<ag_stats>() as libc::c_ulong,
        );
        gettimeofday(&mut stats.time_start as *mut timeval, 0 as *mut libc::c_void);
    }
    has_jit = 0 as libc::c_int;
    pcre_config(9 as libc::c_int, &mut has_jit as *mut libc::c_int as *mut libc::c_void);
    if has_jit != 0 {
        study_opts |= 1 as libc::c_int;
    }
    tmp___0 = sysconf(84 as libc::c_int);
    num_cores = tmp___0 as libc::c_int;
    if num_cores < 8 as libc::c_int {
        workers_len = num_cores;
    } else {
        workers_len = 8 as libc::c_int;
    }
    if opts.literal != 0 {
        workers_len -= 1;
    }
    if opts.workers != 0 {
        workers_len = opts.workers;
    }
    if workers_len < 1 as libc::c_int {
        workers_len = 1 as libc::c_int;
    }
    log_debug(b"Using %i workers\0" as *const u8 as *const libc::c_char, workers_len);
    done_adding_files = 0 as libc::c_int;
    tmp___1 = ag_calloc(
        workers_len as size_t,
        ::std::mem::size_of::<worker_t>() as libc::c_ulong,
    );
    workers = tmp___1 as *mut worker_t;
    tmp___2 = pthread_cond_init(
        &mut files_ready as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    if tmp___2 != 0 {
        die(b"pthread_cond_init failed!\0" as *const u8 as *const libc::c_char);
    }
    tmp___3 = pthread_mutex_init(
        &mut print_mtx,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    if tmp___3 != 0 {
        die(b"pthread_mutex_init failed!\0" as *const u8 as *const libc::c_char);
    }
    if opts.stats != 0 {
        tmp___4 = pthread_mutex_init(
            &mut stats_mtx,
            0 as *mut libc::c_void as *const pthread_mutexattr_t,
        );
        if tmp___4 != 0 {
            die(b"pthread_mutex_init failed!\0" as *const u8 as *const libc::c_char);
        }
    }
    tmp___5 = pthread_mutex_init(
        &mut work_queue_mtx,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    if tmp___5 != 0 {
        die(b"pthread_mutex_init failed!\0" as *const u8 as *const libc::c_char);
    }
    if opts.casing as libc::c_uint == 3 as libc::c_uint {
        tmp___7 = is_lowercase(opts.query as *const libc::c_char);
        if tmp___7 != 0 {
            opts.casing = CASE_INSENSITIVE;
        } else {
            opts.casing = CASE_SENSITIVE;
        }
    }
    if opts.literal != 0 {
        if opts.casing as libc::c_uint == 2 as libc::c_uint {
            c = opts.query;
            while *c as libc::c_int != 0 as libc::c_int {
                if ::std::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_ulong
                {
                    __res = tolower(*c as libc::c_int);
                } else {
                    tmp___9 = __ctype_tolower_loc();
                    __res = *(*tmp___9).offset(*c as libc::c_int as isize);
                }
                *c = __res as libc::c_char;
                c = c.offset(1);
            }
        }
        generate_alpha_skip(
            opts.query as *const libc::c_char,
            opts.query_len as size_t,
            alpha_skip_lookup.as_mut_ptr(),
            (opts.casing as libc::c_uint == 1 as libc::c_uint) as libc::c_int,
        );
        find_skip_lookup = 0 as *mut libc::c_void as *mut size_t;
        generate_find_skip(
            opts.query as *const libc::c_char,
            opts.query_len as size_t,
            &mut find_skip_lookup,
            (opts.casing as libc::c_uint == 1 as libc::c_uint) as libc::c_int,
        );
        generate_hash(
            opts.query as *const libc::c_char,
            opts.query_len as size_t,
            h_table.as_mut_ptr(),
            (opts.casing as libc::c_uint == 1 as libc::c_uint) as libc::c_int,
        );
        if opts.word_regexp != 0 {
            init_wordchar_table();
            opts
                .literal_starts_wordchar = is_wordchar(
                *(opts.query).offset(0 as libc::c_int as isize),
            );
            opts
                .literal_ends_wordchar = is_wordchar(
                *(opts.query).offset((opts.query_len - 1 as libc::c_int) as isize),
            );
        }
    } else {
        if opts.casing as libc::c_uint == 2 as libc::c_uint {
            pcre_opts |= 1 as libc::c_int;
        }
        if opts.word_regexp != 0 {
            ag_asprintf(
                &mut word_regexp_query as *mut *mut libc::c_char,
                b"\\b(?:%s)\\b\0" as *const u8 as *const libc::c_char,
                opts.query,
            );
            free(opts.query as *mut libc::c_void);
            opts.query = word_regexp_query;
            tmp___10 = strlen(opts.query as *const libc::c_char);
            opts.query_len = tmp___10 as libc::c_int;
        }
        compile_study(
            &mut opts.re,
            &mut opts.re_extra,
            opts.query,
            pcre_opts,
            study_opts,
        );
    }
    if opts.search_stream != 0 {
        search_stream(stdin, b"\0" as *const u8 as *const libc::c_char);
    } else {
        i = 0 as libc::c_int;
        while i < workers_len {
            (*workers.offset(i as isize)).id = i;
            tmp___11 = pthread_create(
                &mut (*workers.offset(i as isize)).thread as *mut pthread_t,
                0 as *mut libc::c_void as *const pthread_attr_t,
                Some(
                    search_file_worker
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                &mut (*workers.offset(i as isize)).id as *mut libc::c_int
                    as *mut libc::c_void,
            );
            rv = tmp___11;
            if rv != 0 as libc::c_int {
                tmp___12 = strerror(rv);
                die(
                    b"Error in pthread_create(): %s\0" as *const u8
                        as *const libc::c_char,
                    tmp___12,
                );
            }
            if opts.use_thread_affinity != 0 {
                libc::memset(
                    &mut cpu_set as *mut cpu_set_t as *mut libc::c_void,
                    '\u{0}' as i32,
                    ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::c_int
                        as libc::c_ulong as libc::size_t,
                );
                __cpu = (i % num_cores) as size_t;
                if __cpu.wrapping_div(8 as libc::c_ulong)
                    < ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong
                {
                    cpu_set
                        .__bits[__cpu
                        .wrapping_div(
                            (8 as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                ),
                        ) as usize]
                        |= (1 as libc::c_ulong)
                            << __cpu
                                .wrapping_rem(
                                    (8 as libc::c_ulong)
                                        .wrapping_mul(
                                            ::std::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                        ),
                                );
                }
                rv = pthread_setaffinity_np(
                    (*workers.offset(i as isize)).thread,
                    ::std::mem::size_of::<cpu_set_t>() as libc::c_ulong,
                    &mut cpu_set as *mut cpu_set_t as *const cpu_set_t,
                );
                if rv != 0 {
                    tmp___13 = strerror(rv);
                    log_err(
                        b"Error in pthread_setaffinity_np(): %s\0" as *const u8
                            as *const libc::c_char,
                        tmp___13,
                    );
                    log_err(
                        b"Performance may be affected. Use --noaffinity to suppress this message.\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else {
                    log_debug(
                        b"Thread %i set to CPU %i\0" as *const u8 as *const libc::c_char,
                        i,
                        i,
                    );
                }
            } else {
                log_debug(
                    b"Thread affinity disabled.\0" as *const u8 as *const libc::c_char,
                );
            }
            i += 1;
        }
        i = 0 as libc::c_int;
        while *paths.offset(i as isize) as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            log_debug(
                b"searching path %s for %s\0" as *const u8 as *const libc::c_char,
                *paths.offset(i as isize),
                opts.query,
            );
            symhash = 0 as *mut libc::c_void as *mut symdir_t;
            tmp___14 = init_ignore(
                root_ignores,
                b"\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int as size_t,
            );
            ig = tmp___14;
            s.st_dev = 0 as libc::c_int as __dev_t;
            s.st_ino = 0 as libc::c_ulong;
            s.st_nlink = 0 as libc::c_ulong;
            s.st_mode = 0 as libc::c_uint;
            s.st_uid = 0 as libc::c_uint;
            s.st_gid = 0 as libc::c_uint;
            s.__pad0 = 0 as libc::c_int;
            s.st_rdev = 0 as libc::c_ulong;
            s.st_size = 0 as libc::c_long;
            s.st_blksize = 0 as libc::c_long;
            s.st_blocks = 0 as libc::c_long;
            s.st_atim.tv_sec = 0 as libc::c_long;
            s.st_atim.tv_nsec = 0 as libc::c_long;
            s.st_mtim.tv_sec = 0 as libc::c_long;
            s.st_mtim.tv_nsec = 0 as libc::c_long;
            s.st_ctim.tv_sec = 0 as libc::c_long;
            s.st_ctim.tv_nsec = 0 as libc::c_long;
            s.__glibc_reserved[0 as libc::c_int as usize] = 0 as libc::c_long;
            s.__glibc_reserved[1 as libc::c_int as usize] = 0 as libc::c_long;
            s.__glibc_reserved[2 as libc::c_int as usize] = 0 as libc::c_long;
            if opts.one_dev != 0 {
                tmp___15 = lstat(
                    *paths.offset(i as isize) as *const libc::c_char,
                    &mut s as *mut stat,
                );
                if tmp___15 == -(1 as libc::c_int) {
                    log_err(
                        b"Failed to get device information for path %s. Skipping...\0"
                            as *const u8 as *const libc::c_char,
                        *paths.offset(i as isize),
                    );
                }
            }
            search_dir(
                ig,
                *base_paths.offset(i as isize) as *const libc::c_char,
                *paths.offset(i as isize) as *const libc::c_char,
                0 as libc::c_int,
                s.st_dev,
            );
            cleanup_ignore(ig);
            i += 1;
        }
        pthread_mutex_lock(&mut work_queue_mtx);
        done_adding_files = 1 as libc::c_int;
        pthread_cond_broadcast(&mut files_ready);
        pthread_mutex_unlock(&mut work_queue_mtx);
        i = 0 as libc::c_int;
        while i < workers_len {
            tmp___16 = pthread_join(
                (*workers.offset(i as isize)).thread,
                0 as *mut libc::c_void as *mut *mut libc::c_void,
            );
            if tmp___16 != 0 {
                die(b"pthread_join failed!\0" as *const u8 as *const libc::c_char);
            }
            i += 1;
        }
    }
    if opts.stats != 0 {
        gettimeofday(&mut stats.time_end as *mut timeval, 0 as *mut libc::c_void);
        time_diff = (stats.time_end.tv_sec * 1000000 as libc::c_long
            + stats.time_end.tv_usec
            - (stats.time_start.tv_sec * 1000000 as libc::c_long
                + stats.time_start.tv_usec)) as libc::c_double;
        time_diff /= 1000000 as libc::c_int as libc::c_double;
        printf(
            b"%zu matches\n%zu files contained matches\n%zu files searched\n%zu bytes searched\n%f seconds\n\0"
                as *const u8 as *const libc::c_char,
            stats.total_matches,
            stats.total_file_matches,
            stats.total_files,
            stats.total_bytes,
            time_diff,
        );
        pthread_mutex_destroy(&mut stats_mtx);
    }
    if !(opts.pager).is_null() {
        pclose(out_fd);
    }
    cleanup_options();
    pthread_cond_destroy(&mut files_ready);
    pthread_mutex_destroy(&mut work_queue_mtx);
    pthread_mutex_destroy(&mut print_mtx);
    cleanup_ignore(root_ignores);
    free(workers as *mut libc::c_void);
    i = 0 as libc::c_int;
    while *paths.offset(i as isize) as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        free(*paths.offset(i as isize) as *mut libc::c_void);
        free(*base_paths.offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    free(base_paths as *mut libc::c_void);
    free(paths as *mut libc::c_void);
    if !find_skip_lookup.is_null() {
        free(find_skip_lookup as *mut libc::c_void);
    }
    return (opts.match_found == 0) as libc::c_int;
}
static mut zfile_io: _IO_cookie_io_functions_t = unsafe {
    {
        let mut init = _IO_cookie_io_functions_t {
            read: Some(
                zfile_read
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_char,
                        size_t,
                    ) -> __ssize_t,
            ),
            write: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<cookie_write_function_t>,
            >(0 as *const libc::c_void as *mut libc::c_void),
            seek: Some(
                zfile_seek
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut __off64_t,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
            close: Some(
                zfile_close as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int,
            ),
        };
        init
    }
};
unsafe extern "C" fn zfile_cookie_init(mut cookie: *mut zfile) -> libc::c_int {
    let mut lzrc: lzma_ret = LZMA_OK;
    let mut rc: libc::c_int = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut __constr_expr_1: lzma_stream = lzma_stream {
        next_in: 0 as *const uint8_t,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut uint8_t,
        avail_out: 0,
        total_out: 0,
        allocator: 0 as *const lzma_allocator,
        internal: 0 as *mut lzma_internal,
        reserved_ptr1: 0 as *mut libc::c_void,
        reserved_ptr2: 0 as *mut libc::c_void,
        reserved_ptr3: 0 as *mut libc::c_void,
        reserved_ptr4: 0 as *mut libc::c_void,
        reserved_int1: 0,
        reserved_int2: 0,
        reserved_int3: 0,
        reserved_int4: 0,
        reserved_enum1: LZMA_RESERVED_ENUM,
        reserved_enum2: LZMA_RESERVED_ENUM,
    };
    if !((*cookie).logic_offset == 0 as libc::c_ulong) {
        __assert_fail(
            b"cookie->logic_offset == 0\0" as *const u8 as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_uint,
            b"zfile_cookie_init\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*cookie).decode_offset == 0 as libc::c_ulong) {
        __assert_fail(
            b"cookie->decode_offset == 0\0" as *const u8 as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_uint,
            b"zfile_cookie_init\0" as *const u8 as *const libc::c_char,
        );
    }
    (*cookie).actual_len = 0 as libc::c_int as uint64_t;
    match (*cookie).ctype as libc::c_uint {
        1 => {
            memset(
                &mut (*cookie).stream.gz as *mut z_stream as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<z_stream>() as libc::c_ulong,
            );
            rc = inflateInit2_(
                &mut (*cookie).stream.gz,
                47 as libc::c_int,
                b"1.2.11\0" as *const u8 as *const libc::c_char,
                ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
            );
            if rc != 0 as libc::c_int {
                tmp___1 = zError(rc);
                log_err(
                    b"Unable to initialize zlib: %s\0" as *const u8
                        as *const libc::c_char,
                    tmp___1,
                );
                return 5 as libc::c_int;
            }
            (*cookie).stream.gz.next_in = 0 as *mut libc::c_void as *mut Bytef;
            (*cookie).stream.gz.avail_in = 0 as libc::c_int as uInt;
            (*cookie).stream.gz.next_out = ((*cookie).outbuf).as_mut_ptr();
            (*cookie)
                .stream
                .gz
                .avail_out = ::std::mem::size_of::<[uint8_t; 262144]>() as libc::c_ulong
                as uInt;
        }
        4 => {
            __constr_expr_1.next_in = 0 as *mut libc::c_void as *const uint8_t;
            __constr_expr_1.avail_in = 0 as libc::c_int as size_t;
            __constr_expr_1.total_in = 0 as libc::c_int as uint64_t;
            __constr_expr_1.next_out = 0 as *mut libc::c_void as *mut uint8_t;
            __constr_expr_1.avail_out = 0 as libc::c_int as size_t;
            __constr_expr_1.total_out = 0 as libc::c_int as uint64_t;
            __constr_expr_1.allocator = 0 as *mut libc::c_void as *const lzma_allocator;
            __constr_expr_1.internal = 0 as *mut libc::c_void as *mut lzma_internal;
            __constr_expr_1.reserved_ptr1 = 0 as *mut libc::c_void;
            __constr_expr_1.reserved_ptr2 = 0 as *mut libc::c_void;
            __constr_expr_1.reserved_ptr3 = 0 as *mut libc::c_void;
            __constr_expr_1.reserved_ptr4 = 0 as *mut libc::c_void;
            __constr_expr_1.reserved_int1 = 0 as libc::c_int as uint64_t;
            __constr_expr_1.reserved_int2 = 0 as libc::c_int as uint64_t;
            __constr_expr_1.reserved_int3 = 0 as libc::c_int as size_t;
            __constr_expr_1.reserved_int4 = 0 as libc::c_int as size_t;
            __constr_expr_1.reserved_enum1 = LZMA_RESERVED_ENUM;
            __constr_expr_1.reserved_enum2 = LZMA_RESERVED_ENUM;
            (*cookie).stream.lzma = __constr_expr_1;
            lzrc = lzma_auto_decoder(
                &mut (*cookie).stream.lzma,
                -(1 as libc::c_int) as uint64_t,
                0 as libc::c_int as uint32_t,
            );
            if lzrc as libc::c_uint != 0 as libc::c_uint {
                log_err(
                    b"Unable to initialize lzma_auto_decoder: %d\0" as *const u8
                        as *const libc::c_char,
                    lzrc as libc::c_uint,
                );
                return 5 as libc::c_int;
            }
            (*cookie).stream.lzma.next_in = 0 as *mut libc::c_void as *const uint8_t;
            (*cookie).stream.lzma.avail_in = 0 as libc::c_int as size_t;
            (*cookie).stream.lzma.next_out = ((*cookie).outbuf).as_mut_ptr();
            (*cookie)
                .stream
                .lzma
                .avail_out = ::std::mem::size_of::<[uint8_t; 262144]>() as libc::c_ulong;
        }
        _ => {
            log_err(
                b"Unsupported compression type: %d\0" as *const u8
                    as *const libc::c_char,
                (*cookie).ctype as libc::c_uint,
            );
            return 22 as libc::c_int;
        }
    }
    (*cookie).outbuf_start = 0 as libc::c_int as uint32_t;
    (*cookie).eof = 0 as libc::c_int != 0;
    return 0 as libc::c_int;
}
unsafe extern "C" fn zfile_cookie_cleanup(mut cookie: *mut zfile) {
    match (*cookie).ctype as libc::c_uint {
        1 => {
            inflateEnd(&mut (*cookie).stream.gz);
        }
        4 => {
            lzma_end(&mut (*cookie).stream.lzma);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn decompress_open(
    mut fd: libc::c_int,
    mut mode: *const libc::c_char,
    mut ctype: ag_compression_type,
) -> *mut FILE {
    let mut cookie: *mut zfile = 0 as *mut zfile;
    let mut res: *mut FILE = 0 as *mut FILE;
    let mut in_0: *mut FILE = 0 as *mut FILE;
    let mut error: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    cookie = 0 as *mut libc::c_void as *mut zfile;
    res = 0 as *mut libc::c_void as *mut FILE;
    in_0 = res;
    tmp___0 = strstr(mode, b"w\0" as *const u8 as *const libc::c_char);
    if !tmp___0.is_null() {
        tmp = __errno_location();
        *tmp = 22 as libc::c_int;
    } else {
        tmp___1 = strstr(mode, b"a\0" as *const u8 as *const libc::c_char);
        if !tmp___1.is_null() {
            tmp = __errno_location();
            *tmp = 22 as libc::c_int;
        } else {
            in_0 = fdopen(fd, mode);
            if !(in_0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
                tmp___2 = malloc(::std::mem::size_of::<zfile>() as libc::c_ulong);
                cookie = tmp___2 as *mut zfile;
                if cookie as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    tmp___3 = __errno_location();
                    *tmp___3 = 12 as libc::c_int;
                } else {
                    (*cookie).in_0 = in_0;
                    (*cookie).logic_offset = 0 as libc::c_int as uint64_t;
                    (*cookie).decode_offset = 0 as libc::c_int as uint64_t;
                    (*cookie).ctype = ctype;
                    error = zfile_cookie_init(cookie);
                    if error != 0 as libc::c_int {
                        tmp___4 = __errno_location();
                        *tmp___4 = error;
                    } else {
                        res = fopencookie(cookie as *mut libc::c_void, mode, zfile_io);
                    }
                }
            }
        }
    }
    if res as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        if in_0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            fclose(in_0);
        }
        if cookie as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            free(cookie as *mut libc::c_void);
        }
    }
    return res;
}
unsafe extern "C" fn zfile_read(
    mut cookie_: *mut libc::c_void,
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> __ssize_t {
    let mut cookie: *mut zfile = 0 as *mut zfile;
    let mut nb: size_t = 0;
    let mut ignorebytes: size_t = 0;
    let mut total: ssize_t = 0;
    let mut lzret: lzma_ret = LZMA_OK;
    let mut ret: libc::c_int = 0;
    let mut inflated: size_t = 0;
    let mut left: size_t = 0;
    let mut tmp___1: *mut Bytef = 0 as *mut Bytef;
    let mut ignoreskip: size_t = 0;
    let mut _a: size_t = 0;
    let mut _b: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut toread: size_t = 0;
    let mut _a___0: size_t = 0;
    let mut _b___0: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: *mut Bytef = 0 as *mut Bytef;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: size_t = 0;
    let mut tmp___9: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___10: *mut Bytef = 0 as *mut Bytef;
    let mut tmp___11: libc::c_int = 0;
    cookie = cookie_ as *mut zfile;
    total = 0 as libc::c_int as ssize_t;
    if !(size <= 9223372036854775807 as libc::c_ulong) {
        __assert_fail(
            b"size <= SSIZE_MAX\0" as *const u8 as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_uint,
            b"zfile_read\0" as *const u8 as *const libc::c_char,
        );
    }
    if size == 0 as libc::c_ulong {
        return 0 as libc::c_int as __ssize_t;
    }
    if (*cookie).eof {
        return 0 as libc::c_int as __ssize_t;
    }
    ret = 0 as libc::c_int;
    lzret = LZMA_OK;
    ignorebytes = ((*cookie).logic_offset).wrapping_sub((*cookie).decode_offset);
    if !(ignorebytes == 0 as libc::c_ulong) {
        __assert_fail(
            b"ignorebytes == 0\0" as *const u8 as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            225 as libc::c_uint,
            b"zfile_read\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        loop {
            if (*cookie).ctype as libc::c_uint == 1 as libc::c_uint {
                tmp___4 = (*cookie).stream.gz.next_out;
            } else {
                tmp___4 = (*cookie).stream.lzma.next_out;
            }
            if !(tmp___4 as libc::c_ulong
                > &mut *((*cookie).outbuf)
                    .as_mut_ptr()
                    .offset((*cookie).outbuf_start as isize) as *mut uint8_t
                    as libc::c_ulong)
            {
                break;
            }
            if (*cookie).ctype as libc::c_uint == 1 as libc::c_uint {
                tmp___1 = (*cookie).stream.gz.next_out;
            } else {
                tmp___1 = (*cookie).stream.lzma.next_out;
            }
            left = tmp___1
                .offset_from(
                    &mut *((*cookie).outbuf)
                        .as_mut_ptr()
                        .offset((*cookie).outbuf_start as isize) as *mut uint8_t,
                ) as libc::c_long as size_t;
            _a = ignorebytes;
            _b = left;
            if _a < _b {
                tmp___2 = _a;
            } else {
                tmp___2 = _b;
            }
            ignoreskip = tmp___2;
            if ignoreskip > 0 as libc::c_ulong {
                ignorebytes = (ignorebytes as libc::c_ulong).wrapping_sub(ignoreskip)
                    as size_t as size_t;
                left = (left as libc::c_ulong).wrapping_sub(ignoreskip) as size_t
                    as size_t;
                (*cookie)
                    .outbuf_start = ((*cookie).outbuf_start as size_t)
                    .wrapping_add(ignoreskip) as uint32_t;
                (*cookie)
                    .decode_offset = ((*cookie).decode_offset as libc::c_ulong)
                    .wrapping_add(ignoreskip) as uint64_t as uint64_t;
            }
            if ignorebytes > 0 as libc::c_ulong {
                break;
            }
            _a___0 = left;
            _b___0 = size;
            if _a___0 < _b___0 {
                tmp___3 = _a___0;
            } else {
                tmp___3 = _b___0;
            }
            toread = tmp___3;
            memcpy(
                buf as *mut libc::c_void,
                &mut *((*cookie).outbuf)
                    .as_mut_ptr()
                    .offset((*cookie).outbuf_start as isize) as *mut uint8_t
                    as *const libc::c_void,
                toread,
            );
            buf = buf.offset(toread as isize);
            size = (size as libc::c_ulong).wrapping_sub(toread) as size_t as size_t;
            left = (left as libc::c_ulong).wrapping_sub(toread) as size_t as size_t;
            (*cookie)
                .outbuf_start = ((*cookie).outbuf_start as size_t).wrapping_add(toread)
                as uint32_t;
            (*cookie)
                .decode_offset = ((*cookie).decode_offset as libc::c_ulong)
                .wrapping_add(toread) as uint64_t as uint64_t;
            (*cookie)
                .logic_offset = ((*cookie).logic_offset as libc::c_ulong)
                .wrapping_add(toread) as uint64_t as uint64_t;
            total = (total as size_t).wrapping_add(toread) as ssize_t;
            if size == 0 as libc::c_ulong {
                break;
            }
        }
        if size == 0 as libc::c_ulong {
            break;
        }
        if !((*cookie).stream.gz.next_out as libc::c_ulong
            == &mut *((*cookie).outbuf)
                .as_mut_ptr()
                .offset((*cookie).outbuf_start as isize) as *mut uint8_t
                as libc::c_ulong)
        {
            __assert_fail(
                b"cookie->stream.gz.next_out == &cookie->outbuf[cookie->outbuf_start]\0"
                    as *const u8 as *const libc::c_char,
                b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                273 as libc::c_uint,
                b"zfile_read\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*cookie).ctype as libc::c_uint == 4 as libc::c_uint {
            if lzret as libc::c_uint == 1 as libc::c_uint {
                (*cookie).eof = 1 as libc::c_int != 0;
                break;
            }
        }
        if (*cookie).ctype as libc::c_uint == 1 as libc::c_uint {
            if ret == 1 as libc::c_int {
                (*cookie).eof = 1 as libc::c_int != 0;
                break;
            }
        }
        if (*cookie).ctype as libc::c_uint == 1 as libc::c_uint {
            tmp___8 = (*cookie).stream.gz.avail_in as size_t;
        } else {
            tmp___8 = (*cookie).stream.lzma.avail_in;
        }
        if tmp___8 == 0 as libc::c_ulong {
            nb = fread(
                ((*cookie).inbuf).as_mut_ptr() as *mut libc::c_void,
                1 as libc::c_int as size_t,
                ::std::mem::size_of::<[uint8_t; 32768]>() as libc::c_ulong,
                (*cookie).in_0,
            );
            tmp___6 = ferror((*cookie).in_0);
            if tmp___6 != 0 {
                warn(b"error read core\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            if nb == 0 as libc::c_ulong {
                tmp___7 = feof((*cookie).in_0);
                if tmp___7 != 0 {
                    warn(b"truncated file\0" as *const u8 as *const libc::c_char);
                    exit(1 as libc::c_int);
                }
            }
            if (*cookie).ctype as libc::c_uint == 4 as libc::c_uint {
                (*cookie).stream.lzma.avail_in = nb;
                (*cookie)
                    .stream
                    .lzma
                    .next_in = ((*cookie).inbuf).as_mut_ptr() as *const uint8_t;
            } else {
                (*cookie).stream.gz.avail_in = nb as uInt;
                (*cookie).stream.gz.next_in = ((*cookie).inbuf).as_mut_ptr();
            }
        }
        if (*cookie).ctype as libc::c_uint == 4 as libc::c_uint {
            (*cookie).stream.lzma.next_out = ((*cookie).outbuf).as_mut_ptr();
            (*cookie)
                .stream
                .lzma
                .avail_out = ::std::mem::size_of::<[uint8_t; 262144]>() as libc::c_ulong;
        } else {
            (*cookie).stream.gz.next_out = ((*cookie).outbuf).as_mut_ptr();
            (*cookie)
                .stream
                .gz
                .avail_out = ::std::mem::size_of::<[uint8_t; 262144]>() as libc::c_ulong
                as uInt;
        }
        (*cookie).outbuf_start = 0 as libc::c_int as uint32_t;
        if (*cookie).ctype as libc::c_uint == 1 as libc::c_uint {
            ret = inflate(&mut (*cookie).stream.gz, 0 as libc::c_int);
            if ret != 0 as libc::c_int {
                if ret != 1 as libc::c_int {
                    tmp___9 = zError(ret);
                    log_err(
                        b"Found mem/data error while decompressing zlib stream: %s\0"
                            as *const u8 as *const libc::c_char,
                        tmp___9,
                    );
                    return -(1 as libc::c_int) as __ssize_t;
                }
            }
        } else {
            lzret = lzma_code(&mut (*cookie).stream.lzma, LZMA_RUN);
            if lzret as libc::c_uint != 0 as libc::c_uint {
                if lzret as libc::c_uint != 1 as libc::c_uint {
                    log_err(
                        b"Found mem/data error while decompressing xz/lzma stream: %d\0"
                            as *const u8 as *const libc::c_char,
                        lzret as libc::c_uint,
                    );
                    return -(1 as libc::c_int) as __ssize_t;
                }
            }
        }
        if (*cookie).ctype as libc::c_uint == 1 as libc::c_uint {
            tmp___10 = (*cookie).stream.gz.next_out;
        } else {
            tmp___10 = (*cookie).stream.lzma.next_out;
        }
        inflated = tmp___10
            .offset_from(
                &mut *((*cookie).outbuf).as_mut_ptr().offset(0 as libc::c_int as isize)
                    as *mut uint8_t,
            ) as libc::c_long as size_t;
        (*cookie)
            .actual_len = ((*cookie).actual_len as libc::c_ulong).wrapping_add(inflated)
            as uint64_t as uint64_t;
        tmp___11 = ferror((*cookie).in_0);
        if tmp___11 != 0 {
            break;
        }
        if !(size > 0 as libc::c_ulong) {
            break;
        }
    }
    if !(total <= 9223372036854775807 as libc::c_long) {
        __assert_fail(
            b"total <= SSIZE_MAX\0" as *const u8 as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            329 as libc::c_uint,
            b"zfile_read\0" as *const u8 as *const libc::c_char,
        );
    }
    return total;
}
unsafe extern "C" fn zfile_seek(
    mut cookie_: *mut libc::c_void,
    mut offset_: *mut __off64_t,
    mut whence: libc::c_int,
) -> libc::c_int {
    let mut cookie: *mut zfile = 0 as *mut zfile;
    let mut new_offset: off64_t = 0;
    let mut offset: off64_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bsz: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut diff: size_t = 0;
    let mut _a: size_t = 0;
    let mut _b: uint64_t = 0;
    let mut tmp___0: size_t = 0;
    let mut err___0: ssize_t = 0;
    let mut tmp___1: __ssize_t = 0;
    cookie = cookie_ as *mut zfile;
    new_offset = 0 as libc::c_int as off64_t;
    offset = *offset_;
    if whence == 0 as libc::c_int {
        new_offset = offset;
    } else if whence == 1 as libc::c_int {
        new_offset = (*cookie).logic_offset as off64_t + offset;
    } else {
        return -(1 as libc::c_int)
    }
    if new_offset < 0 as libc::c_long {
        return -(1 as libc::c_int);
    }
    if new_offset < (*cookie).logic_offset as off64_t {
        if new_offset != 0 as libc::c_long {
            return -(1 as libc::c_int);
        }
    }
    if new_offset == 0 as libc::c_long {
        (*cookie).decode_offset = 0 as libc::c_int as uint64_t;
        (*cookie).logic_offset = 0 as libc::c_int as uint64_t;
        zfile_cookie_cleanup(cookie);
        zfile_cookie_init(cookie);
    } else if new_offset as uint64_t > (*cookie).logic_offset {
        bsz = 32768 as libc::c_int as size_t;
        tmp = malloc(bsz);
        buf = tmp as *mut libc::c_char;
        while new_offset as uint64_t > (*cookie).logic_offset {
            _a = bsz;
            _b = (new_offset as uint64_t).wrapping_sub((*cookie).logic_offset);
            if _a < _b {
                tmp___0 = _a;
            } else {
                tmp___0 = _b;
            }
            diff = tmp___0;
            tmp___1 = zfile_read(cookie_, buf, diff);
            err___0 = tmp___1;
            if err___0 < 0 as libc::c_long {
                free(buf as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            if !(err___0 == 0 as libc::c_long) {
                continue;
            }
            if !(*cookie).eof {
                __assert_fail(
                    b"cookie->eof\0" as *const u8 as *const libc::c_char,
                    b"src/zfile.c\0" as *const u8 as *const libc::c_char,
                    378 as libc::c_uint,
                    b"zfile_seek\0" as *const u8 as *const libc::c_char,
                );
            }
            new_offset = (*cookie).logic_offset as off64_t;
            break;
        }
        free(buf as *mut libc::c_void);
    }
    if !((*cookie).logic_offset == new_offset as uint64_t) {
        __assert_fail(
            b"cookie->logic_offset == (uint64_t)new_offset\0" as *const u8
                as *const libc::c_char,
            b"src/zfile.c\0" as *const u8 as *const libc::c_char,
            386 as libc::c_uint,
            b"zfile_seek\0" as *const u8 as *const libc::c_char,
        );
    }
    *offset_ = new_offset;
    return 0 as libc::c_int;
}
unsafe extern "C" fn zfile_close(mut cookie_: *mut libc::c_void) -> libc::c_int {
    let mut cookie: *mut zfile = 0 as *mut zfile;
    cookie = cookie_ as *mut zfile;
    zfile_cookie_cleanup(cookie);
    fclose((*cookie).in_0);
    free(cookie as *mut libc::c_void);
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
