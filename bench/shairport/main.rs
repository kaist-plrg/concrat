use ::libc;
use ::c2rust_bitfields;
use ::c2rust_asm_casts;
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bio_st;
    pub type rsa_st;
    pub type bio_method_st;
    pub type ao_device;
    pub type pa_simple;
    pub type _snd_pcm_hw_params;
    pub type _snd_pcm;
    pub type _snd_mixer;
    pub type _snd_mixer_class;
    pub type _snd_mixer_elem;
    pub type _snd_mixer_selem_id;
    fn sigfillset(__set: *mut sigset_t) -> libc::c_int;
    fn sigdelset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn pthread_sigmask(
        __how: libc::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn gethostname(__name: *mut libc::c_char, __len: size_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn MD5_Init(c: *mut MD5_CTX) -> libc::c_int;
    fn MD5_Update(
        c: *mut MD5_CTX,
        data: *const libc::c_void,
        len: size_t,
    ) -> libc::c_int;
    fn MD5_Final(md: *mut libc::c_uchar, c: *mut MD5_CTX) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn lockf(__fd: libc::c_int, __cmd: libc::c_int, __len: __off_t) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vdprintf(
        __fd: libc::c_int,
        __fmt: *const libc::c_char,
        __arg: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn dprintf(__fd: libc::c_int, __fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strsep(
        __stringp: *mut *mut libc::c_char,
        __delim: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept(
        __fd: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn pthread_kill(__threadid: pthread_t, __signo: libc::c_int) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
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
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn execvp(
        __file: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(_: libc::c_int) -> !;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn getifaddrs(__ifap: *mut *mut ifaddrs) -> libc::c_int;
    fn freeifaddrs(__ifa: *mut ifaddrs);
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn BIO_set_flags(b: *mut BIO, flags: libc::c_int);
    fn BIO_new(type_0: *const BIO_METHOD) -> *mut BIO;
    fn BIO_free(a: *mut BIO) -> libc::c_int;
    fn BIO_read(b: *mut BIO, data: *mut libc::c_void, dlen: libc::c_int) -> libc::c_int;
    fn BIO_write(
        b: *mut BIO,
        data: *const libc::c_void,
        dlen: libc::c_int,
    ) -> libc::c_int;
    fn BIO_ctrl(
        bp: *mut BIO,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn BIO_push(b: *mut BIO, append: *mut BIO) -> *mut BIO;
    fn BIO_free_all(a: *mut BIO);
    fn BIO_s_mem() -> *const BIO_METHOD;
    fn BIO_new_mem_buf(buf: *const libc::c_void, len: libc::c_int) -> *mut BIO;
    fn RSA_size(rsa_0: *const RSA) -> libc::c_int;
    fn RSA_private_encrypt(
        flen: libc::c_int,
        from: *const libc::c_uchar,
        to: *mut libc::c_uchar,
        rsa_0: *mut RSA,
        padding: libc::c_int,
    ) -> libc::c_int;
    fn RSA_private_decrypt(
        flen: libc::c_int,
        from: *const libc::c_uchar,
        to: *mut libc::c_uchar,
        rsa_0: *mut RSA,
        padding: libc::c_int,
    ) -> libc::c_int;
    fn BIO_f_base64() -> *const BIO_METHOD;
    fn PEM_read_bio_RSAPrivateKey(
        bp: *mut BIO,
        x: *mut *mut RSA,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut RSA;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn mkfifo(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn AES_set_decrypt_key(
        userKey: *const libc::c_uchar,
        bits: libc::c_int,
        key: *mut AES_KEY,
    ) -> libc::c_int;
    fn AES_cbc_encrypt(
        in_0: *const libc::c_uchar,
        out: *mut libc::c_uchar,
        length: size_t,
        key: *const AES_KEY,
        ivec: *mut libc::c_uchar,
        enc: libc::c_int,
    );
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn inet_addr(__cp: *const libc::c_char) -> in_addr_t;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> libc::c_int;
    fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: libc::c_int,
    ) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn ao_initialize();
    fn ao_shutdown();
    fn ao_append_option(
        options: *mut *mut ao_option,
        key: *const libc::c_char,
        value: *const libc::c_char,
    ) -> libc::c_int;
    fn ao_open_live(
        driver_id: libc::c_int,
        format: *mut ao_sample_format,
        option: *mut ao_option,
    ) -> *mut ao_device;
    fn ao_play(
        device: *mut ao_device,
        output_samples: *mut libc::c_char,
        num_bytes: uint_32,
    ) -> libc::c_int;
    fn ao_close(device: *mut ao_device) -> libc::c_int;
    fn ao_driver_id(short_name: *const libc::c_char) -> libc::c_int;
    fn ao_default_driver_id() -> libc::c_int;
    fn pa_simple_new(
        server: *const libc::c_char,
        name: *const libc::c_char,
        dir: pa_stream_direction_t,
        dev_0: *const libc::c_char,
        stream_name: *const libc::c_char,
        ss_0: *const pa_sample_spec,
        map: *const pa_channel_map,
        attr: *const pa_buffer_attr,
        error: *mut libc::c_int,
    ) -> *mut pa_simple;
    fn pa_simple_free(s: *mut pa_simple);
    fn pa_simple_write(
        s: *mut pa_simple,
        data: *const libc::c_void,
        bytes: size_t,
        error: *mut libc::c_int,
    ) -> libc::c_int;
    fn pa_simple_drain(s: *mut pa_simple, error: *mut libc::c_int) -> libc::c_int;
    fn pa_strerror(error: libc::c_int) -> *const libc::c_char;
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
    fn snd_pcm_drain(pcm: *mut snd_pcm_t) -> libc::c_int;
    fn snd_pcm_writei(
        pcm: *mut snd_pcm_t,
        buffer: *const libc::c_void,
        size: snd_pcm_uframes_t,
    ) -> snd_pcm_sframes_t;
    fn snd_pcm_recover(
        pcm: *mut snd_pcm_t,
        err: libc::c_int,
        silent: libc::c_int,
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
    fn snd_pcm_hw_params_set_rate_near(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: *mut libc::c_uint,
        dir: *mut libc::c_int,
    ) -> libc::c_int;
    fn snd_pcm_hw_params_set_period_size_near(
        pcm: *mut snd_pcm_t,
        params: *mut snd_pcm_hw_params_t,
        val: *mut snd_pcm_uframes_t,
        dir: *mut libc::c_int,
    ) -> libc::c_int;
    fn snd_mixer_open(mixer: *mut *mut snd_mixer_t, mode: libc::c_int) -> libc::c_int;
    fn snd_mixer_close(mixer: *mut snd_mixer_t) -> libc::c_int;
    fn snd_mixer_attach(
        mixer: *mut snd_mixer_t,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn snd_mixer_load(mixer: *mut snd_mixer_t) -> libc::c_int;
    fn snd_mixer_selem_register(
        mixer: *mut snd_mixer_t,
        options: *mut snd_mixer_selem_regopt,
        classp: *mut *mut snd_mixer_class_t,
    ) -> libc::c_int;
    fn snd_mixer_find_selem(
        mixer: *mut snd_mixer_t,
        id: *const snd_mixer_selem_id_t,
    ) -> *mut snd_mixer_elem_t;
    fn snd_mixer_selem_set_playback_volume_all(
        elem: *mut snd_mixer_elem_t,
        value: libc::c_long,
    ) -> libc::c_int;
    fn snd_mixer_selem_get_playback_volume_range(
        elem: *mut snd_mixer_elem_t,
        min: *mut libc::c_long,
        max: *mut libc::c_long,
    ) -> libc::c_int;
    fn snd_mixer_selem_id_sizeof() -> size_t;
    fn snd_mixer_selem_id_set_name(
        obj: *mut snd_mixer_selem_id_t,
        val: *const libc::c_char,
    );
    fn snd_mixer_selem_id_set_index(obj: *mut snd_mixer_selem_id_t, val: libc::c_uint);
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
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___sigset_t_991265788 {
    pub __val: [libc::c_ulong; 16],
}
pub type __sigset_t = __anonstruct___sigset_t_991265788;
pub type sigset_t = __sigset_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub union __anonunion__bounds_304607062 {
    pub _addr_bnd: __anonstruct__addr_bnd_5259977,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct__sigfault_109243111 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: __anonunion__bounds_304607062,
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
pub union __anonunion__sifields_235771088 {
    pub _pad: [libc::c_int; 28],
    pub _kill: __anonstruct__kill_244518854,
    pub _timer: __anonstruct__timer_490064738,
    pub _rt: __anonstruct__rt_619254530,
    pub _sigchld: __anonstruct__sigchld_284671705,
    pub _sigfault: __anonstruct__sigfault_109243111,
    pub _sigpoll: __anonstruct__sigpoll_386613454,
    pub _sigsys: __anonstruct__sigsys_44812255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_siginfo_t_1025666667 {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: __anonunion__sifields_235771088,
}
pub type siginfo_t = __anonstruct_siginfo_t_1025666667;
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
pub type size_t = libc::c_ulong;
pub type pthread_t = libc::c_ulong;
pub type socklen_t = __socklen_t;
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
pub type uint8_t = __uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD5state_st {
    pub A: libc::c_uint,
    pub B: libc::c_uint,
    pub C: libc::c_uint,
    pub D: libc::c_uint,
    pub Nl: libc::c_uint,
    pub Nh: libc::c_uint,
    pub data: [libc::c_uint; 16],
    pub num: libc::c_uint,
}
pub type MD5_CTX = MD5state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
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
pub struct __anonstruct_audio_output_780339900 {
    pub help: Option::<unsafe extern "C" fn() -> ()>,
    pub name: *mut libc::c_char,
    pub init: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_char) -> libc::c_int,
    >,
    pub deinit: Option::<unsafe extern "C" fn() -> ()>,
    pub start: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub play: Option::<unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> ()>,
    pub stop: Option::<unsafe extern "C" fn() -> ()>,
    pub volume: Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
}
pub type audio_output = __anonstruct_audio_output_780339900;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_mdns_backend_66454024 {
    pub name: *mut libc::c_char,
    pub mdns_register: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub mdns_unregister: Option::<unsafe extern "C" fn() -> ()>,
}
pub type mdns_backend = __anonstruct_mdns_backend_66454024;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_shairport_cfg_113578856 {
    pub password: *mut libc::c_char,
    pub apname: *mut libc::c_char,
    pub hw_addr: [uint8_t; 6],
    pub port: libc::c_int,
    pub output_name: *mut libc::c_char,
    pub output: *mut audio_output,
    pub mdns_name: *mut libc::c_char,
    pub mdns: *mut mdns_backend,
    pub buffer_start_fill: libc::c_int,
    pub daemonise: libc::c_int,
    pub cmd_start: *mut libc::c_char,
    pub cmd_stop: *mut libc::c_char,
    pub cmd_blocking: libc::c_int,
    pub meta_dir: *mut libc::c_char,
    pub pidfile: *mut libc::c_char,
    pub logfile: *mut libc::c_char,
    pub errfile: *mut libc::c_char,
}
pub type shairport_cfg = __anonstruct_shairport_cfg_113578856;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type va_list___0 = __gnuc_va_list;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __suseconds_t = libc::c_long;
pub type int32_t = __int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_fd_set_356711149 {
    pub __fds_bits: [__fd_mask; 16],
}
pub type fd_set = __anonstruct_fd_set_356711149;
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
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion___in6_u_979734923 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: __anonunion___in6_u_979734923,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_metadata_273595944 {
    pub artist: *mut libc::c_char,
    pub title: *mut libc::c_char,
    pub album: *mut libc::c_char,
    pub artwork: *mut libc::c_char,
    pub comment: *mut libc::c_char,
    pub genre: *mut libc::c_char,
}
pub type metadata = __anonstruct_metadata_273595944;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_stream_cfg_797956145 {
    pub aesiv: [uint8_t; 16],
    pub aeskey: [uint8_t; 16],
    pub fmtp: [int32_t; 12],
}
pub type stream_cfg = __anonstruct_stream_cfg_797956145;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_rtsp_conn_info_364246510 {
    pub fd: libc::c_int,
    pub stream: stream_cfg,
    pub remote: sockaddr_storage,
    pub running: libc::c_int,
    pub thread: pthread_t,
}
pub type rtsp_conn_info = __anonstruct_rtsp_conn_info_364246510;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_rtsp_message_967555407 {
    pub nheaders: libc::c_int,
    pub name: [*mut libc::c_char; 16],
    pub value: [*mut libc::c_char; 16],
    pub contentlength: libc::c_int,
    pub content: *mut libc::c_char,
    pub method: [libc::c_char; 16],
    pub respcode: libc::c_int,
}
pub type rtsp_message = __anonstruct_rtsp_message_967555407;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct method_handler {
    pub method: *mut libc::c_char,
    pub handler: Option::<
        unsafe extern "C" fn(
            *mut rtsp_conn_info,
            *mut rtsp_message,
            *mut rtsp_message,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_ifa_ifu_753034692 {
    pub ifu_broadaddr: *mut sockaddr,
    pub ifu_dstaddr: *mut sockaddr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut libc::c_char,
    pub ifa_flags: libc::c_uint,
    pub ifa_addr: *mut sockaddr,
    pub ifa_netmask: *mut sockaddr,
    pub ifa_ifu: __anonunion_ifa_ifu_753034692,
    pub ifa_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_srv {
    pub priority: uint16_t,
    pub weight: uint16_t,
    pub port: uint16_t,
    pub target: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_txt {
    pub next: *mut rr_data_txt,
    pub txt: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_nsec {
    pub bitmap: [uint8_t; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_entry {
    pub name: *mut uint8_t,
    pub type_0: rr_type,
    pub ttl: uint32_t,
    pub unicast_query: libc::c_char,
    pub cache_flush: libc::c_char,
    pub rr_class: uint16_t,
    pub data: __anonunion_data_487002723,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_data_487002723 {
    pub NSEC: rr_data_nsec,
    pub SRV: rr_data_srv,
    pub TXT: rr_data_txt,
    pub PTR: rr_data_ptr,
    pub A: rr_data_a,
    pub AAAA: rr_data_aaaa,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_aaaa {
    pub addr: *mut in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_a {
    pub addr: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_data_ptr {
    pub name: *mut uint8_t,
    pub entry: *mut rr_entry,
}
pub type rr_type = libc::c_uint;
pub const RR_ANY: rr_type = 255;
pub const RR_NSEC: rr_type = 47;
pub const RR_SRV: rr_type = 33;
pub const RR_AAAA: rr_type = 28;
pub const RR_TXT: rr_type = 16;
pub const RR_PTR: rr_type = 12;
pub const RR_A: rr_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdnsd {
    pub data_lock: pthread_mutex_t,
    pub sockfd: libc::c_int,
    pub notify_pipe: [libc::c_int; 2],
    pub stop_flag: libc::c_int,
    pub group: *mut rr_group,
    pub announce: *mut rr_list,
    pub services: *mut rr_list,
    pub hostname: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_list {
    pub e: *mut rr_entry,
    pub next: *mut rr_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rr_group {
    pub name: *mut uint8_t,
    pub rr: *mut rr_list,
    pub next: *mut rr_group,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_service {
    pub entries: *mut rr_list,
}
pub type BIO = bio_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_mem_st {
    pub length: size_t,
    pub data: *mut libc::c_char,
    pub max: size_t,
    pub flags: libc::c_ulong,
}
pub type BUF_MEM = buf_mem_st;
pub type RSA = rsa_st;
pub type BIO_METHOD = bio_method_st;
pub type pem_password_cb = unsafe extern "C" fn(
    *mut libc::c_char,
    libc::c_int,
    libc::c_int,
    *mut libc::c_void,
) -> libc::c_int;
pub type __int16_t = libc::c_short;
pub type int16_t = __int16_t;
pub type seq_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes_key_st {
    pub rd_key: [libc::c_uint; 60],
    pub rounds: libc::c_int,
}
pub type AES_KEY = aes_key_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alac_file {
    pub input_buffer: *mut libc::c_uchar,
    pub input_buffer_bitaccumulator: libc::c_int,
    pub samplesize: libc::c_int,
    pub numchannels: libc::c_int,
    pub bytespersample: libc::c_int,
    pub predicterror_buffer_a: *mut int32_t,
    pub predicterror_buffer_b: *mut int32_t,
    pub outputsamples_buffer_a: *mut int32_t,
    pub outputsamples_buffer_b: *mut int32_t,
    pub uncompressed_bytes_buffer_a: *mut int32_t,
    pub uncompressed_bytes_buffer_b: *mut int32_t,
    pub setinfo_max_samples_per_frame: uint32_t,
    pub setinfo_7a: uint8_t,
    pub setinfo_sample_size: uint8_t,
    pub setinfo_rice_historymult: uint8_t,
    pub setinfo_rice_initialhistory: uint8_t,
    pub setinfo_rice_kmodifier: uint8_t,
    pub setinfo_7f: uint8_t,
    pub setinfo_80: uint16_t,
    pub setinfo_82: uint32_t,
    pub setinfo_86: uint32_t,
    pub setinfo_8a_rate: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct audio_buffer_entry {
    pub ready: libc::c_int,
    pub data: *mut libc::c_short,
}
pub type abuf_t = audio_buffer_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_biquad_t_616681636 {
    pub hist: [libc::c_double; 2],
    pub a: [libc::c_double; 2],
    pub b: [libc::c_double; 3],
}
pub type biquad_t = __anonstruct_biquad_t_616681636;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct_se_struct_24_422959015 {
    #[bitfield(name = "x", ty = "libc::c_int", bits = "0..=23")]
    pub x: [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
}
pub type __useconds_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutexattr_t_488594144 {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
pub type pthread_mutexattr_t = __anonunion_pthread_mutexattr_t_488594144;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mdns_pkt {
    pub id: uint16_t,
    pub flags: uint16_t,
    pub num_qn: uint16_t,
    pub num_ans_rr: uint16_t,
    pub num_auth_rr: uint16_t,
    pub num_add_rr: uint16_t,
    pub rr_qn: *mut rr_list,
    pub rr_ans: *mut rr_list,
    pub rr_auth: *mut rr_list,
    pub rr_add: *mut rr_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct name_comp {
    pub label: *mut uint8_t,
    pub pos: size_t,
    pub next: *mut name_comp,
}
pub type uint_32 = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ao_sample_format {
    pub bits: libc::c_int,
    pub rate: libc::c_int,
    pub channels: libc::c_int,
    pub byte_format: libc::c_int,
    pub matrix: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ao_option {
    pub key: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub next: *mut ao_option,
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
pub type snd_mixer_t = _snd_mixer;
pub type snd_mixer_class_t = _snd_mixer_class;
pub type snd_mixer_elem_t = _snd_mixer_elem;
pub type snd_mixer_selem_regopt_abstract = libc::c_uint;
pub const SND_MIXER_SABSTRACT_BASIC: snd_mixer_selem_regopt_abstract = 1;
pub const SND_MIXER_SABSTRACT_NONE: snd_mixer_selem_regopt_abstract = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snd_mixer_selem_regopt {
    pub ver: libc::c_int,
    pub abstract_0: snd_mixer_selem_regopt_abstract,
    pub device: *const libc::c_char,
    pub playback_pcm: *mut snd_pcm_t,
    pub capture_pcm: *mut snd_pcm_t,
}
pub type snd_mixer_selem_id_t = _snd_mixer_selem_id;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = __xstat(1 as libc::c_int, __path, __statbuf);
    return tmp;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    let mut tmp: libc::c_long = 0;
    tmp = strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    return tmp as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    let mut tmp: libc::c_double = 0.;
    tmp = strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    return tmp;
}
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> libc::c_int {
    return (__thread1 == __thread2) as libc::c_int;
}
static mut version: *const libc::c_char = b"1.1.1-23-gd65b8e8\0" as *const u8
    as *const libc::c_char;
static mut shutting_down: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn shairport_shutdown(mut retval: libc::c_int) {
    if shutting_down != 0 {
        return;
    }
    shutting_down = 1 as libc::c_int;
    printf(b"Shutting down...\n\0" as *const u8 as *const libc::c_char);
    mdns_unregister();
    rtsp_shutdown_stream();
    if !(config.output).is_null() {
        (Some(((*config.output).deinit).expect("non-null function pointer")))
            .expect("non-null function pointer")();
    }
    daemon_exit();
    exit(retval);
}
unsafe extern "C" fn sig_ignore(
    mut foo: libc::c_int,
    mut bar: *mut siginfo_t,
    mut baz: *mut libc::c_void,
) {}
unsafe extern "C" fn sig_shutdown(
    mut foo: libc::c_int,
    mut bar: *mut siginfo_t,
    mut baz: *mut libc::c_void,
) {
    shairport_shutdown(0 as libc::c_int);
}
unsafe extern "C" fn sig_child(
    mut foo: libc::c_int,
    mut bar: *mut siginfo_t,
    mut baz: *mut libc::c_void,
) {
    let mut pid: pid_t = 0;
    loop {
        pid = waitpid(-(1 as libc::c_int), 0 as *mut libc::c_int, 1 as libc::c_int);
        if !(pid > 0 as libc::c_int) {
            break;
        }
        if pid == mdns_pid {
            if shutting_down == 0 {
                die(
                    b"MDNS child process died unexpectedly!\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
    };
}
unsafe extern "C" fn sig_logrotate(
    mut foo: libc::c_int,
    mut bar: *mut siginfo_t,
    mut baz: *mut libc::c_void,
) {
    log_setup();
}
pub unsafe extern "C" fn usage(mut progname: *mut libc::c_char) {
    printf(b"Usage: %s [options...]\n\0" as *const u8 as *const libc::c_char, progname);
    printf(
        b"  or:  %s [options...] -- [audio output-specific options]\n\0" as *const u8
            as *const libc::c_char,
        progname,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Mandatory arguments to long options are mandatory for short options too.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Options:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"    -h, --help          show this help\n\0" as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -p, --port=PORT     set RTSP listening port\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -a, --name=NAME     set advertised name\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -k, --password=PW   require password to stream audio\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -b FILL             set how full the buffer must be before audio output\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                        starts. This value is in frames; default %d\n\0"
            as *const u8 as *const libc::c_char,
        config.buffer_start_fill,
    );
    printf(
        b"    -d, --daemon        fork (daemonise). The PID of the child process is\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                        written to stdout, unless a pidfile is used.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -P, --pidfile=FILE  write daemon's pid to FILE on startup.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                        Has no effect if -d is not specified\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -l, --log=FILE      redirect shairport's standard output to FILE\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                        If --error is not specified, it also redirects\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                        error output to FILE\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -e, --error=FILE    redirect shairport's standard error output to FILE\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -B, --on-start=COMMAND  run a shell command when playback begins\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -E, --on-stop=COMMAND   run a shell command when playback ends\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -w, --wait-cmd          block while the shell command(s) run\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -M, --meta-dir=DIR      set a directory to write metadata and album cover art to\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"    -o, --output=BACKEND    select audio output method\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"    -m, --mdns=BACKEND      force the use of BACKEND to advertise the service\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"                            if no mdns provider is specified,\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"                            shairport tries them all until one works.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    mdns_ls_backends();
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    audio_ls_outputs();
}
static mut long_options: [option; 15] = [
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
            name: b"daemon\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'd' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"pidfile\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"log\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'l' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"error\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'e' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"port\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'p' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"name\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"password\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'k' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"output\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'o' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"on-start\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'B' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"on-stop\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'E' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"wait-cmd\0" as *const u8 as *const libc::c_char,
            has_arg: 0 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'w' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"meta-dir\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'M' as i32,
        };
        init
    },
    {
        let mut init = option {
            name: b"mdns\0" as *const u8 as *const libc::c_char,
            has_arg: 1 as libc::c_int,
            flag: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_int,
            val: 'm' as i32,
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
pub unsafe extern "C" fn parse_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    setenv(
        b"POSIXLY_CORRECT\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    loop {
        opt = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"+hdvP:l:e:p:a:k:o:b:B:E:M:wm:\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr() as *const option,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if !(opt > 0 as libc::c_int) {
            break;
        }
        match opt {
            104 => {
                usage(*argv.offset(0 as libc::c_int as isize));
                exit(0 as libc::c_int);
            }
            100 => {
                config.daemonise = 1 as libc::c_int;
            }
            118 => {
                debuglev += 1;
            }
            112 => {
                config.port = atoi(optarg as *const libc::c_char);
            }
            97 => {
                config.apname = optarg;
            }
            111 => {
                config.output_name = optarg;
            }
            107 => {
                config.password = optarg;
            }
            98 => {
                config.buffer_start_fill = atoi(optarg as *const libc::c_char);
            }
            66 => {
                config.cmd_start = optarg;
            }
            69 => {
                config.cmd_stop = optarg;
            }
            119 => {
                config.cmd_blocking = 1 as libc::c_int;
            }
            77 => {
                config.meta_dir = optarg;
            }
            80 => {
                config.pidfile = optarg;
            }
            108 => {
                config.logfile = optarg;
            }
            101 => {
                config.errfile = optarg;
            }
            109 => {
                config.mdns_name = optarg;
            }
            _ => {
                usage(*argv.offset(0 as libc::c_int as isize));
                exit(1 as libc::c_int);
            }
        }
    }
    return optind;
}
pub unsafe extern "C" fn signal_setup() {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut sa: sigaction = sigaction {
        __sigaction_handler: __anonunion___sigaction_handler_363639592 {
            sa_handler: None,
        },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigfillset(&mut set);
    sigdelset(&mut set, 2 as libc::c_int);
    sigdelset(&mut set, 15 as libc::c_int);
    sigdelset(&mut set, 1 as libc::c_int);
    sigdelset(&mut set, 19 as libc::c_int);
    sigdelset(&mut set, 17 as libc::c_int);
    pthread_sigmask(
        0 as libc::c_int,
        &mut set as *mut sigset_t as *const __sigset_t,
        0 as *mut libc::c_void as *mut __sigset_t,
    );
    memset(
        &mut sa as *mut sigaction as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sigaction>() as libc::c_ulong,
    );
    sa.sa_flags = 4 as libc::c_int;
    sa
        .__sigaction_handler
        .sa_sigaction = Some(
        sig_ignore
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigaction(
        10 as libc::c_int,
        &mut sa as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sa.sa_flags = 268435460 as libc::c_int;
    sa
        .__sigaction_handler
        .sa_sigaction = Some(
        sig_shutdown
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigaction(
        2 as libc::c_int,
        &mut sa as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sigaction(
        15 as libc::c_int,
        &mut sa as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sa
        .__sigaction_handler
        .sa_sigaction = Some(
        sig_logrotate
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigaction(
        1 as libc::c_int,
        &mut sa as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
    sa
        .__sigaction_handler
        .sa_sigaction = Some(
        sig_child
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigaction(
        17 as libc::c_int,
        &mut sa as *mut sigaction as *const sigaction,
        0 as *mut libc::c_void as *mut sigaction,
    );
}
pub unsafe extern "C" fn shairport_startup_complete() {
    if config.daemonise != 0 {
        daemon_ready();
    }
}
unsafe extern "C" fn log_setup() {
    let mut log_fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut err_fd: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if !(config.logfile).is_null() {
        tmp = open(
            config.logfile as *const libc::c_char,
            1089 as libc::c_int,
            384 as libc::c_int | 256 as libc::c_int >> 3 as libc::c_int
                | 256 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        log_fd = tmp;
        if log_fd < 0 as libc::c_int {
            die(
                b"Could not open logfile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        dup2(log_fd, 1 as libc::c_int);
        setvbuf(
            stdout,
            0 as *mut libc::c_void as *mut libc::c_char,
            1 as libc::c_int,
            8192 as libc::c_int as size_t,
        );
        if (config.errfile).is_null() {
            dup2(log_fd, 2 as libc::c_int);
            setvbuf(
                stderr,
                0 as *mut libc::c_void as *mut libc::c_char,
                1 as libc::c_int,
                8192 as libc::c_int as size_t,
            );
        }
    }
    if !(config.errfile).is_null() {
        tmp___0 = open(
            config.errfile as *const libc::c_char,
            1089 as libc::c_int,
            384 as libc::c_int | 256 as libc::c_int >> 3 as libc::c_int
                | 256 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int,
        );
        err_fd = tmp___0;
        if err_fd < 0 as libc::c_int {
            die(
                b"Could not open logfile\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        dup2(err_fd, 2 as libc::c_int);
        setvbuf(
            stderr,
            0 as *mut libc::c_void as *mut libc::c_char,
            1 as libc::c_int,
            8192 as libc::c_int as size_t,
        );
    }
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut hostname: [libc::c_char; 100] = [0; 100];
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut audio_arg: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: size_t = 0;
    let mut ap_md5: [uint8_t; 16] = [0; 16];
    let mut ctx: MD5_CTX = MD5_CTX {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        Nl: 0,
        Nh: 0,
        data: [0; 16],
        num: 0,
    };
    let mut tmp___2: size_t = 0;
    printf(b"Starting Shairport %s\n\0" as *const u8 as *const libc::c_char, version);
    signal_setup();
    memset(
        &mut config as *mut shairport_cfg as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<shairport_cfg>() as libc::c_ulong,
    );
    config.buffer_start_fill = 220 as libc::c_int;
    config.port = 5002 as libc::c_int;
    gethostname(hostname.as_mut_ptr(), 100 as libc::c_int as size_t);
    tmp = malloc(120 as libc::c_int as size_t);
    config.apname = tmp as *mut libc::c_char;
    snprintf(
        config.apname,
        120 as libc::c_int as size_t,
        b"Shairport on %s\0" as *const u8 as *const libc::c_char,
        hostname.as_mut_ptr(),
    );
    tmp___0 = parse_options(argc, argv);
    audio_arg = tmp___0;
    tmp___1 = strlen(config.apname as *const libc::c_char);
    if tmp___1 > 50 as libc::c_ulong {
        die(
            b"Supplied name too long (max 50 characters)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if config.daemonise != 0 {
        daemon_init();
    }
    log_setup();
    config.output = audio_get_output(config.output_name);
    if (config.output).is_null() {
        audio_ls_outputs();
        die(
            b"Invalid audio output specified!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    (Some(((*config.output).init).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(argc - audio_arg, argv.offset(audio_arg as isize));
    MD5_Init(&mut ctx);
    tmp___2 = strlen(config.apname as *const libc::c_char);
    MD5_Update(&mut ctx, config.apname as *const libc::c_void, tmp___2);
    MD5_Final(ap_md5.as_mut_ptr(), &mut ctx);
    memcpy(
        (config.hw_addr).as_mut_ptr() as *mut libc::c_void,
        ap_md5.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[uint8_t; 6]>() as libc::c_ulong,
    );
    if !(config.meta_dir).is_null() {
        metadata_open();
    }
    rtsp_listen_loop();
    shairport_shutdown(1 as libc::c_int);
    return 1 as libc::c_int;
}
static mut lock_fd: libc::c_int = -(1 as libc::c_int);
static mut daemon_pipe: [libc::c_int; 2] = [-(1 as libc::c_int), -(1 as libc::c_int)];
pub unsafe extern "C" fn daemon_init() {
    let mut ret: libc::c_int = 0;
    let mut pid: pid_t = 0;
    let mut tmp: __pid_t = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: __pid_t = 0;
    ret = pipe(daemon_pipe.as_mut_ptr());
    if ret < 0 as libc::c_int {
        die(
            b"couldn't create a pipe?!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    tmp = fork();
    pid = tmp;
    if pid < 0 as libc::c_int {
        die(
            b"failed to fork!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if pid != 0 {
        close(daemon_pipe[1 as libc::c_int as usize]);
        tmp___0 = read(
            daemon_pipe[0 as libc::c_int as usize],
            buf.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        );
        ret = tmp___0 as libc::c_int;
        if ret < 0 as libc::c_int {
            fprintf(
                stderr,
                b"Spawning the daemon failed.\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        } else if buf[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int {
            tmp___1 = write(
                2 as libc::c_int,
                buf.as_mut_ptr() as *const libc::c_void,
                ret as size_t,
            );
            fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        } else {
            if (config.pidfile).is_null() {
                printf(b"%d\n\0" as *const u8 as *const libc::c_char, pid);
            }
            exit(0 as libc::c_int);
        }
    } else {
        close(daemon_pipe[0 as libc::c_int as usize]);
        if !(config.pidfile).is_null() {
            lock_fd = open(
                config.pidfile as *const libc::c_char,
                66 as libc::c_int,
                384 as libc::c_int,
            );
            if lock_fd < 0 as libc::c_int {
                die(
                    b"Could not open pidfile\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            ret = lockf(lock_fd, 2 as libc::c_int, 0 as libc::c_int as __off_t);
            if ret < 0 as libc::c_int {
                die(
                    b"Could not lock pidfile. Is an other instance running ?\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            tmp___2 = getpid();
            dprintf(lock_fd, b"%d\n\0" as *const u8 as *const libc::c_char, tmp___2);
        }
    };
}
pub unsafe extern "C" fn daemon_ready() {
    let mut ok: libc::c_char = 0;
    let mut tmp: ssize_t = 0;
    ok = 0 as libc::c_int as libc::c_char;
    tmp = write(
        daemon_pipe[1 as libc::c_int as usize],
        &mut ok as *mut libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    close(daemon_pipe[1 as libc::c_int as usize]);
    daemon_pipe[1 as libc::c_int as usize] = -(1 as libc::c_int);
}
pub unsafe extern "C" fn daemon_fail(
    mut format: *const libc::c_char,
    mut arg: ::std::ffi::VaList,
) {
    if daemon_pipe[1 as libc::c_int as usize] > 0 as libc::c_int {
        vdprintf(daemon_pipe[1 as libc::c_int as usize], format, arg.as_va_list());
    }
}
pub unsafe extern "C" fn daemon_exit() {
    let mut tmp: libc::c_int = 0;
    if lock_fd > 0 as libc::c_int {
        tmp = lockf(lock_fd, 0 as libc::c_int, 0 as libc::c_int as __off_t);
        close(lock_fd);
        unlink(config.pidfile as *const libc::c_char);
        lock_fd = -(1 as libc::c_int);
    }
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 4278190080 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 16711680 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 65280 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 255 as libc::c_uint) << 24 as libc::c_int;
}
static mut playing_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut please_shutdown: libc::c_int = 0 as libc::c_int;
static mut playing_thread: pthread_t = 0;
#[inline]
unsafe extern "C" fn rtsp_playing() -> libc::c_int {
    let mut tmp: pthread_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp___1 = pthread_mutex_trylock(&mut playing_mutex);
    if tmp___1 != 0 {
        tmp = pthread_self();
        tmp___0 = pthread_equal(playing_thread, tmp);
        return tmp___0;
    } else {
        pthread_mutex_unlock(&mut playing_mutex);
        return 0 as libc::c_int;
    };
}
unsafe extern "C" fn rtsp_take_player() {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = rtsp_playing();
    if tmp != 0 {
        return;
    }
    tmp___0 = pthread_mutex_trylock(&mut playing_mutex);
    if tmp___0 != 0 {
        debug(
            1 as libc::c_int,
            b"shutting down playing thread\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        please_shutdown = 1 as libc::c_int;
        pthread_kill(playing_thread, 10 as libc::c_int);
        pthread_mutex_lock(&mut playing_mutex);
    }
    playing_thread = pthread_self();
}
pub unsafe extern "C" fn rtsp_shutdown_stream() {
    rtsp_take_player();
    pthread_mutex_unlock(&mut playing_mutex);
}
static mut conns: *mut *mut rtsp_conn_info = 0 as *const libc::c_void
    as *mut libc::c_void as *mut *mut rtsp_conn_info;
static mut nconns: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn track_thread(mut conn: *mut rtsp_conn_info) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = realloc(
        conns as *mut libc::c_void,
        (::std::mem::size_of::<*mut rtsp_conn_info>() as libc::c_ulong)
            .wrapping_mul((nconns + 1 as libc::c_int) as libc::c_ulong),
    );
    conns = tmp as *mut *mut rtsp_conn_info;
    let ref mut fresh0 = *conns.offset(nconns as isize);
    *fresh0 = conn;
    nconns += 1;
}
unsafe extern "C" fn cleanup_threads() {
    let mut retval: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    debug(
        2 as libc::c_int,
        b"culling threads.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < nconns {
        if (**conns.offset(i as isize)).running == 0 as libc::c_int {
            pthread_join((**conns.offset(i as isize)).thread, &mut retval);
            free(*conns.offset(i as isize) as *mut libc::c_void);
            debug(
                2 as libc::c_int,
                b"one joined\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            nconns -= 1;
            if nconns != 0 {
                let ref mut fresh1 = *conns.offset(i as isize);
                *fresh1 = *conns.offset(nconns as isize);
            }
        } else {
            i += 1;
        }
    }
}
unsafe extern "C" fn nextline(
    mut in_0: *mut libc::c_char,
    mut inbuf: libc::c_int,
) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    out = 0 as *mut libc::c_void as *mut libc::c_char;
    while inbuf != 0 {
        if *in_0 as libc::c_int == 13 as libc::c_int {
            tmp = in_0;
            in_0 = in_0.offset(1);
            *tmp = 0 as libc::c_int as libc::c_char;
            out = in_0;
        }
        if *in_0 as libc::c_int == 10 as libc::c_int {
            tmp___0 = in_0;
            in_0 = in_0.offset(1);
            *tmp___0 = 0 as libc::c_int as libc::c_char;
            out = in_0;
        }
        if !out.is_null() {
            break;
        }
        in_0 = in_0.offset(1);
        inbuf -= 1;
    }
    return out;
}
unsafe extern "C" fn msg_init() -> *mut rtsp_message {
    let mut msg: *mut rtsp_message = 0 as *mut rtsp_message;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<rtsp_message>() as libc::c_ulong);
    msg = tmp as *mut rtsp_message;
    memset(
        msg as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rtsp_message>() as libc::c_ulong,
    );
    return msg;
}
unsafe extern "C" fn msg_add_header(
    mut msg: *mut rtsp_message,
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> libc::c_int {
    if (*msg).nheaders as libc::c_ulong
        >= (::std::mem::size_of::<[*mut libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        warn(
            b"too many headers?!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 1 as libc::c_int;
    }
    (*msg).name[(*msg).nheaders as usize] = strdup(name as *const libc::c_char);
    (*msg).value[(*msg).nheaders as usize] = strdup(value as *const libc::c_char);
    (*msg).nheaders += 1;
    return 0 as libc::c_int;
}
unsafe extern "C" fn msg_get_header(
    mut msg: *mut rtsp_message,
    mut name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*msg).nheaders {
        tmp = strcasecmp(
            (*msg).name[i as usize] as *const libc::c_char,
            name as *const libc::c_char,
        );
        if tmp == 0 {
            return (*msg).value[i as usize];
        }
        i += 1;
    }
    return 0 as *mut libc::c_void as *mut libc::c_char;
}
unsafe extern "C" fn msg_free(mut msg: *mut rtsp_message) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*msg).nheaders {
        free((*msg).name[i as usize] as *mut libc::c_void);
        free((*msg).value[i as usize] as *mut libc::c_void);
        i += 1;
    }
    if !((*msg).content).is_null() {
        free((*msg).content as *mut libc::c_void);
    }
    free(msg as *mut libc::c_void);
}
unsafe extern "C" fn msg_handle_line(
    mut pmsg: *mut *mut rtsp_message,
    mut line: *mut libc::c_char,
) -> libc::c_int {
    let mut msg: *mut rtsp_message = 0 as *mut rtsp_message;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut p___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    msg = *pmsg;
    if msg.is_null() {
        msg = msg_init();
        *pmsg = msg;
        debug(
            1 as libc::c_int,
            b"received request: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            line,
        );
        p = strtok_r(
            line,
            b" \0" as *const u8 as *const libc::c_char,
            &mut sp as *mut *mut libc::c_char,
        );
        if !p.is_null() {
            strncpy(
                ((*msg).method).as_mut_ptr(),
                p as *const libc::c_char,
                (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong),
            );
            p = strtok_r(
                0 as *mut libc::c_void as *mut libc::c_char,
                b" \0" as *const u8 as *const libc::c_char,
                &mut sp as *mut *mut libc::c_char,
            );
            if !p.is_null() {
                p = strtok_r(
                    0 as *mut libc::c_void as *mut libc::c_char,
                    b" \0" as *const u8 as *const libc::c_char,
                    &mut sp as *mut *mut libc::c_char,
                );
                if !p.is_null() {
                    tmp = strcmp(
                        p as *const libc::c_char,
                        b"RTSP/1.0\0" as *const u8 as *const libc::c_char,
                    );
                    if !(tmp != 0) {
                        return -(1 as libc::c_int);
                    }
                }
            }
        }
    } else {
        tmp___2 = strlen(line as *const libc::c_char);
        if tmp___2 != 0 {
            p___0 = strstr(
                line as *const libc::c_char,
                b": \0" as *const u8 as *const libc::c_char,
            );
            if p___0.is_null() {
                warn(
                    b"bad header: >>%s<<\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    line,
                );
            } else {
                *p___0 = 0 as libc::c_int as libc::c_char;
                p___0 = p___0.offset(2 as libc::c_int as isize);
                msg_add_header(msg, line, p___0);
                debug(
                    2 as libc::c_int,
                    b"    %s: %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    line,
                    p___0,
                );
                return -(1 as libc::c_int);
            }
        } else {
            tmp___0 = msg_get_header(
                msg,
                b"Content-Length\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            cl = tmp___0;
            if !cl.is_null() {
                tmp___1 = atoi(cl as *const libc::c_char);
                return tmp___1;
            } else {
                return 0 as libc::c_int
            }
        }
    }
    *pmsg = 0 as *mut libc::c_void as *mut rtsp_message;
    msg_free(msg);
    return 0 as libc::c_int;
}
unsafe extern "C" fn rtsp_read_request(mut fd___1: libc::c_int) -> *mut rtsp_message {
    let mut current_block: u64;
    let mut buflen: ssize_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut msg: *mut rtsp_message = 0 as *mut rtsp_message;
    let mut nread: ssize_t = 0;
    let mut inbuf: ssize_t = 0;
    let mut msg_size: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    buflen = 512 as libc::c_int as ssize_t;
    tmp = malloc((buflen + 1 as libc::c_long) as size_t);
    buf = tmp as *mut libc::c_char;
    msg = 0 as *mut libc::c_void as *mut rtsp_message;
    inbuf = 0 as libc::c_int as ssize_t;
    msg_size = -(1 as libc::c_int);
    's_40: loop {
        if !(msg_size < 0 as libc::c_int) {
            current_block = 5689316957504528238;
            break;
        }
        if please_shutdown != 0 {
            debug(
                1 as libc::c_int,
                b"RTSP shutdown requested\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            current_block = 17558555198390434851;
            break;
        } else {
            nread = read(
                fd___1,
                buf.offset(inbuf as isize) as *mut libc::c_void,
                (buflen - inbuf) as size_t,
            );
            if nread == 0 {
                debug(
                    1 as libc::c_int,
                    b"RTSP connection closed\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                current_block = 17558555198390434851;
                break;
            } else if nread < 0 as libc::c_long {
                tmp___0 = __errno_location();
                if *tmp___0 == 4 as libc::c_int {
                    continue;
                }
                perror(b"read failure\0" as *const u8 as *const libc::c_char);
                current_block = 17558555198390434851;
                break;
            } else {
                inbuf += nread;
                while msg_size < 0 as libc::c_int {
                    next = nextline(buf, inbuf as libc::c_int);
                    if next.is_null() {
                        break;
                    }
                    msg_size = msg_handle_line(&mut msg, buf);
                    if msg.is_null() {
                        warn(
                            b"no RTSP header received\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        current_block = 17558555198390434851;
                        break 's_40;
                    } else {
                        inbuf -= next.offset_from(buf) as libc::c_long;
                        if inbuf != 0 {
                            memmove(
                                buf as *mut libc::c_void,
                                next as *const libc::c_void,
                                inbuf as size_t,
                            );
                        }
                    }
                }
            }
        }
    }
    match current_block {
        5689316957504528238 => {
            if msg_size as ssize_t > buflen {
                tmp___1 = realloc(buf as *mut libc::c_void, msg_size as size_t);
                buf = tmp___1 as *mut libc::c_char;
                if buf.is_null() {
                    warn(
                        b"too much content\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    current_block = 17558555198390434851;
                } else {
                    buflen = msg_size as ssize_t;
                    current_block = 2873832966593178012;
                }
            } else {
                current_block = 2873832966593178012;
            }
            match current_block {
                17558555198390434851 => {}
                _ => {
                    loop {
                        if !(inbuf < msg_size as ssize_t) {
                            current_block = 15512526488502093901;
                            break;
                        }
                        nread = read(
                            fd___1,
                            buf.offset(inbuf as isize) as *mut libc::c_void,
                            (msg_size as ssize_t - inbuf) as size_t,
                        );
                        if nread == 0 {
                            current_block = 17558555198390434851;
                            break;
                        }
                        if nread == 4 as libc::c_long {
                            continue;
                        }
                        if nread < 0 as libc::c_long {
                            perror(
                                b"read failure\0" as *const u8 as *const libc::c_char,
                            );
                            current_block = 17558555198390434851;
                            break;
                        } else {
                            inbuf += nread;
                        }
                    }
                    match current_block {
                        17558555198390434851 => {}
                        _ => {
                            (*msg).contentlength = inbuf as libc::c_int;
                            (*msg).content = buf;
                            return msg;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    free(buf as *mut libc::c_void);
    if !msg.is_null() {
        msg_free(msg);
    }
    return 0 as *mut libc::c_void as *mut rtsp_message;
}
unsafe extern "C" fn msg_write_response(
    mut fd___1: libc::c_int,
    mut resp: *mut rtsp_message,
) {
    let mut pkt: [libc::c_char; 1024] = [0; 1024];
    let mut pktfree: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: ssize_t = 0;
    pktfree = ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
        as libc::c_int;
    p = pkt.as_mut_ptr();
    if (*resp).respcode == 200 as libc::c_int {
        tmp = b"OK\0" as *const u8 as *const libc::c_char;
    } else {
        tmp = b"Error\0" as *const u8 as *const libc::c_char;
    }
    n = snprintf(
        p,
        pktfree as size_t,
        b"RTSP/1.0 %d %s\r\n\0" as *const u8 as *const libc::c_char,
        (*resp).respcode,
        tmp,
    );
    debug(
        1 as libc::c_int,
        b"sending response: %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        pkt.as_mut_ptr(),
    );
    pktfree -= n;
    p = p.offset(n as isize);
    i = 0 as libc::c_int;
    while i < (*resp).nheaders {
        debug(
            2 as libc::c_int,
            b"    %s: %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*resp).name[i as usize],
            (*resp).value[i as usize],
        );
        n = snprintf(
            p,
            pktfree as size_t,
            b"%s: %s\r\n\0" as *const u8 as *const libc::c_char,
            (*resp).name[i as usize],
            (*resp).value[i as usize],
        );
        pktfree -= n;
        p = p.offset(n as isize);
        if pktfree <= 0 as libc::c_int {
            die(
                b"Attempted to write overlong RTSP packet\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
    }
    if pktfree < 3 as libc::c_int {
        die(
            b"Attempted to write overlong RTSP packet\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    strcpy(p, b"\r\n\0" as *const u8 as *const libc::c_char);
    tmp___0 = write(
        fd___1,
        pkt.as_mut_ptr() as *const libc::c_void,
        (p.offset_from(pkt.as_mut_ptr()) as libc::c_long + 2 as libc::c_long) as size_t,
    );
}
unsafe extern "C" fn handle_options(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    (*resp).respcode = 200 as libc::c_int;
    msg_add_header(
        resp,
        b"Public\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ANNOUNCE, SETUP, RECORD, PAUSE, FLUSH, TEARDOWN, OPTIONS, GET_PARAMETER, SET_PARAMETER\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn handle_teardown(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut tmp: libc::c_int = 0;
    tmp = rtsp_playing();
    if tmp == 0 {
        return;
    }
    (*resp).respcode = 200 as libc::c_int;
    msg_add_header(
        resp,
        b"Connection\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"close\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    please_shutdown = 1 as libc::c_int;
}
unsafe extern "C" fn handle_flush(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut tmp: libc::c_int = 0;
    tmp = rtsp_playing();
    if tmp == 0 {
        return;
    }
    player_flush();
    (*resp).respcode = 200 as libc::c_int;
}
unsafe extern "C" fn handle_setup(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut cport: libc::c_int = 0;
    let mut tport: libc::c_int = 0;
    let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sport: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut resphdr: [libc::c_char; 100] = [0; 100];
    tmp = msg_get_header(
        req,
        b"Transport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    hdr = tmp;
    if hdr.is_null() {
        return;
    }
    p = strstr(
        hdr as *const libc::c_char,
        b"control_port=\0" as *const u8 as *const libc::c_char,
    );
    if p.is_null() {
        return;
    }
    tmp___0 = strchr(p as *const libc::c_char, '=' as i32);
    p = tmp___0.offset(1 as libc::c_int as isize);
    cport = atoi(p as *const libc::c_char);
    p = strstr(
        hdr as *const libc::c_char,
        b"timing_port=\0" as *const u8 as *const libc::c_char,
    );
    if p.is_null() {
        return;
    }
    tmp___1 = strchr(p as *const libc::c_char, '=' as i32);
    p = tmp___1.offset(1 as libc::c_int as isize);
    tport = atoi(p as *const libc::c_char);
    rtsp_take_player();
    tmp___2 = rtp_setup(&mut (*conn).remote, cport, tport);
    sport = tmp___2;
    if sport == 0 {
        return;
    }
    player_play(&mut (*conn).stream);
    snprintf(
        resphdr.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        b"RTP/AVP/UDP;unicast;mode=record;server_port=%d;control_port=%d;timing_port=%d\0"
            as *const u8 as *const libc::c_char,
        sport,
        sport,
        sport,
    );
    msg_add_header(
        resp,
        b"Transport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        resphdr.as_mut_ptr(),
    );
    msg_add_header(
        resp,
        b"Session\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*resp).respcode = 200 as libc::c_int;
}
unsafe extern "C" fn handle_ignore(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    (*resp).respcode = 200 as libc::c_int;
}
unsafe extern "C" fn handle_set_parameter_parameter(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp_left: libc::c_int = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut volume___1: libc::c_float = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut progress: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    cp = (*req).content;
    cp_left = (*req).contentlength;
    while cp_left != 0 {
        if cp.is_null() {
            break;
        }
        next = nextline(cp, cp_left);
        cp_left = (cp_left as libc::c_long - next.offset_from(cp) as libc::c_long)
            as libc::c_int;
        tmp___2 = strncmp(
            cp as *const libc::c_char,
            b"volume: \0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        );
        if tmp___2 != 0 {
            tmp___1 = strncmp(
                cp as *const libc::c_char,
                b"progress: \0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as size_t,
            );
            if tmp___1 != 0 {
                tmp___0 = strlen(cp as *const libc::c_char);
                debug(
                    1 as libc::c_int,
                    b"unrecognised parameter: >>%s<< (%d)\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    cp,
                    tmp___0,
                );
            } else {
                progress = cp.offset(10 as libc::c_int as isize);
                debug(
                    1 as libc::c_int,
                    b"progress: %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    progress,
                );
            }
        } else {
            tmp = atof(cp.offset(8 as libc::c_int as isize) as *const libc::c_char);
            volume___1 = tmp as libc::c_float;
            debug(
                1 as libc::c_int,
                b"volume: %f\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                volume___1 as libc::c_double,
            );
            player_volume(volume___1 as libc::c_double);
        }
        cp = next;
    }
}
unsafe extern "C" fn handle_set_parameter_metadata(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cl: libc::c_int = 0;
    let mut off: libc::c_uint = 0;
    let mut tag: [libc::c_char; 5] = [0; 5];
    let mut vl: uint32_t = 0;
    let mut tmp: __uint32_t = 0;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    cp = (*req).content;
    cl = (*req).contentlength;
    off = 8 as libc::c_uint;
    while off < cl as libc::c_uint {
        strncpy(
            tag.as_mut_ptr(),
            cp.offset(off as isize) as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        tag[4 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        off = off.wrapping_add(4 as libc::c_uint);
        tmp = __bswap_32(*(cp.offset(off as isize) as *mut uint32_t));
        vl = tmp;
        off = (off as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<uint32_t>() as libc::c_ulong)
            as libc::c_uint;
        tmp___0 = malloc(vl.wrapping_add(1 as libc::c_uint) as size_t);
        val = tmp___0 as *mut libc::c_char;
        strncpy(val, cp.offset(off as isize) as *const libc::c_char, vl as size_t);
        *val.offset(vl as isize) = '\u{0}' as i32 as libc::c_char;
        off = off.wrapping_add(vl);
        debug(
            2 as libc::c_int,
            b"Tag: %s   Content: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            tag.as_mut_ptr(),
            val,
        );
        tmp___5 = strncmp(
            tag.as_mut_ptr() as *const libc::c_char,
            b"asal \0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
        if tmp___5 != 0 {
            tmp___4 = strncmp(
                tag.as_mut_ptr() as *const libc::c_char,
                b"asar \0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as size_t,
            );
            if tmp___4 != 0 {
                tmp___3 = strncmp(
                    tag.as_mut_ptr() as *const libc::c_char,
                    b"ascm \0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as size_t,
                );
                if tmp___3 != 0 {
                    tmp___2 = strncmp(
                        tag.as_mut_ptr() as *const libc::c_char,
                        b"asgn \0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as size_t,
                    );
                    if tmp___2 != 0 {
                        tmp___1 = strncmp(
                            tag.as_mut_ptr() as *const libc::c_char,
                            b"minm \0" as *const u8 as *const libc::c_char,
                            4 as libc::c_int as size_t,
                        );
                        if tmp___1 == 0 {
                            debug(
                                1 as libc::c_int,
                                b"META Title: %s\n\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                val,
                            );
                            metadata_set(
                                &mut player_meta.title,
                                val as *const libc::c_char,
                            );
                        }
                    } else {
                        debug(
                            1 as libc::c_int,
                            b"META Genre: %s\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            val,
                        );
                        metadata_set(&mut player_meta.genre, val as *const libc::c_char);
                    }
                } else {
                    debug(
                        1 as libc::c_int,
                        b"META Comment: %s\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        val,
                    );
                    metadata_set(&mut player_meta.comment, val as *const libc::c_char);
                }
            } else {
                debug(
                    1 as libc::c_int,
                    b"META Artist: %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    val,
                );
                metadata_set(&mut player_meta.artist, val as *const libc::c_char);
            }
        } else {
            debug(
                1 as libc::c_int,
                b"META Album: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                val,
            );
            metadata_set(&mut player_meta.album, val as *const libc::c_char);
        }
        free(val as *mut libc::c_void);
    }
    metadata_write();
}
unsafe extern "C" fn handle_set_parameter_coverart(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cl: libc::c_int = 0;
    let mut ct: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    cp = (*req).content;
    cl = (*req).contentlength;
    tmp = msg_get_header(
        req,
        b"Content-Type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    ct = tmp;
    tmp___1 = strncmp(
        ct as *const libc::c_char,
        b"image/jpeg\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int as size_t,
    );
    if tmp___1 != 0 {
        tmp___0 = strncmp(
            ct as *const libc::c_char,
            b"image/png\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as size_t,
        );
        if tmp___0 != 0 {
            metadata_cover_image(
                0 as *mut libc::c_void as *const libc::c_char,
                0 as libc::c_int,
                0 as *mut libc::c_void as *const libc::c_char,
            );
        } else {
            metadata_cover_image(
                cp as *const libc::c_char,
                cl,
                b"png\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        metadata_cover_image(
            cp as *const libc::c_char,
            cl,
            b"jpg\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn handle_set_parameter(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut ct: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    if (*req).contentlength == 0 {
        debug(
            1 as libc::c_int,
            b"received empty SET_PARAMETER request\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    tmp = msg_get_header(
        req,
        b"Content-Type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    ct = tmp;
    if !ct.is_null() {
        debug(
            2 as libc::c_int,
            b"SET_PARAMETER Content-Type: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ct,
        );
        tmp___4 = strncmp(
            ct as *const libc::c_char,
            b"application/x-dmap-tagged\0" as *const u8 as *const libc::c_char,
            25 as libc::c_int as size_t,
        );
        if tmp___4 != 0 {
            tmp___1 = strncmp(
                ct as *const libc::c_char,
                b"image/jpeg\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as size_t,
            );
            if tmp___1 != 0 {
                tmp___2 = strncmp(
                    ct as *const libc::c_char,
                    b"image/png\0" as *const u8 as *const libc::c_char,
                    9 as libc::c_int as size_t,
                );
                if tmp___2 != 0 {
                    tmp___3 = strncmp(
                        ct as *const libc::c_char,
                        b"image/none\0" as *const u8 as *const libc::c_char,
                        10 as libc::c_int as size_t,
                    );
                    if tmp___3 != 0 {
                        tmp___0 = strncmp(
                            ct as *const libc::c_char,
                            b"text/parameters\0" as *const u8 as *const libc::c_char,
                            15 as libc::c_int as size_t,
                        );
                        if tmp___0 != 0 {
                            debug(
                                1 as libc::c_int,
                                b"received unknown Content-Type %s in SET_PARAMETER request\n\0"
                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                                ct,
                            );
                        } else {
                            debug(
                                1 as libc::c_int,
                                b"received parameters in SET_PARAMETER request\n\0"
                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                            handle_set_parameter_parameter(conn, req, resp);
                        }
                    } else {
                        debug(
                            1 as libc::c_int,
                            b"received image in SET_PARAMETER request\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        handle_set_parameter_coverart(conn, req, resp);
                    }
                } else {
                    debug(
                        1 as libc::c_int,
                        b"received image in SET_PARAMETER request\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    handle_set_parameter_coverart(conn, req, resp);
                }
            } else {
                debug(
                    1 as libc::c_int,
                    b"received image in SET_PARAMETER request\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                handle_set_parameter_coverart(conn, req, resp);
            }
        } else {
            debug(
                1 as libc::c_int,
                b"received metadata tags in SET_PARAMETER request\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            handle_set_parameter_metadata(conn, req, resp);
        }
    } else {
        debug(
            1 as libc::c_int,
            b"missing Content-Type header in SET_PARAMETER request\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    (*resp).respcode = 200 as libc::c_int;
}
unsafe extern "C" fn handle_announce(
    mut conn: *mut rtsp_conn_info,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut paesiv: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prsaaeskey: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pfmtp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cp_left: libc::c_int = 0;
    let mut next: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut keylen: libc::c_int = 0;
    let mut aesiv___0: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___2: *mut uint8_t = 0 as *mut uint8_t;
    let mut rsaaeskey: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___3: *mut uint8_t = 0 as *mut uint8_t;
    let mut aeskey: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___4: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: libc::c_int = 0;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    paesiv = 0 as *mut libc::c_void as *mut libc::c_char;
    prsaaeskey = 0 as *mut libc::c_void as *mut libc::c_char;
    pfmtp = 0 as *mut libc::c_void as *mut libc::c_char;
    cp = (*req).content;
    cp_left = (*req).contentlength;
    while cp_left != 0 {
        if cp.is_null() {
            break;
        }
        next = nextline(cp, cp_left);
        cp_left = (cp_left as libc::c_long - next.offset_from(cp) as libc::c_long)
            as libc::c_int;
        tmp = strncmp(
            cp as *const libc::c_char,
            b"a=fmtp:\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as size_t,
        );
        if tmp == 0 {
            pfmtp = cp.offset(7 as libc::c_int as isize);
        }
        tmp___0 = strncmp(
            cp as *const libc::c_char,
            b"a=aesiv:\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as size_t,
        );
        if tmp___0 == 0 {
            paesiv = cp.offset(8 as libc::c_int as isize);
        }
        tmp___1 = strncmp(
            cp as *const libc::c_char,
            b"a=rsaaeskey:\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as size_t,
        );
        if tmp___1 == 0 {
            prsaaeskey = cp.offset(12 as libc::c_int as isize);
        }
        cp = next;
    }
    if paesiv.is_null() {
        warn(
            b"required params missing from announce\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    } else {
        if prsaaeskey.is_null() {
            warn(
                b"required params missing from announce\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return;
        } else {
            if pfmtp.is_null() {
                warn(
                    b"required params missing from announce\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                return;
            }
        }
    }
    tmp___2 = base64_dec(paesiv, &mut len);
    aesiv___0 = tmp___2;
    if len != 16 as libc::c_int {
        warn(
            b"client announced aeskey of %d bytes, wanted 16\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            len,
        );
        free(aesiv___0 as *mut libc::c_void);
        return;
    }
    memcpy(
        ((*conn).stream.aesiv).as_mut_ptr() as *mut libc::c_void,
        aesiv___0 as *const libc::c_void,
        16 as libc::c_int as size_t,
    );
    free(aesiv___0 as *mut libc::c_void);
    tmp___3 = base64_dec(prsaaeskey, &mut len);
    rsaaeskey = tmp___3;
    tmp___4 = rsa_apply(rsaaeskey, len, &mut keylen, 1 as libc::c_int);
    aeskey = tmp___4;
    free(rsaaeskey as *mut libc::c_void);
    if keylen != 16 as libc::c_int {
        warn(
            b"client announced rsaaeskey of %d bytes, wanted 16\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            keylen,
        );
        free(aeskey as *mut libc::c_void);
        return;
    }
    memcpy(
        ((*conn).stream.aeskey).as_mut_ptr() as *mut libc::c_void,
        aeskey as *const libc::c_void,
        16 as libc::c_int as size_t,
    );
    free(aeskey as *mut libc::c_void);
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[int32_t; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<int32_t>() as libc::c_ulong)
    {
        tmp___5 = strsep(
            &mut pfmtp as *mut *mut libc::c_char,
            b" \t\0" as *const u8 as *const libc::c_char,
        );
        (*conn).stream.fmtp[i as usize] = atoi(tmp___5 as *const libc::c_char);
        i += 1;
    }
    (*resp).respcode = 200 as libc::c_int;
}
static mut method_handlers: [method_handler; 9] = unsafe {
    [
        {
            let mut init = method_handler {
                method: b"OPTIONS\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_options
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"ANNOUNCE\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_announce
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"FLUSH\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_flush
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"TEARDOWN\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_teardown
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"SETUP\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_setup
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"GET_PARAMETER\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_ignore
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"SET_PARAMETER\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_set_parameter
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: b"RECORD\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                handler: Some(
                    handle_ignore
                        as unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                ),
            };
            init
        },
        {
            let mut init = method_handler {
                method: 0 as *const libc::c_void as *mut libc::c_void
                    as *mut libc::c_char,
                handler: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<
                        unsafe extern "C" fn(
                            *mut rtsp_conn_info,
                            *mut rtsp_message,
                            *mut rtsp_message,
                        ) -> (),
                    >,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
    ]
};
unsafe extern "C" fn apple_challenge(
    mut fd___1: libc::c_int,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) {
    let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fdsa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut sa_len: socklen_t = 0;
    let mut chall_len: libc::c_int = 0;
    let mut chall: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: *mut uint8_t = 0 as *mut uint8_t;
    let mut buf: [uint8_t; 48] = [0; 48];
    let mut bp: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: libc::c_int = 0;
    let mut sa6: *mut sockaddr_in6 = 0 as *mut sockaddr_in6;
    let mut sa: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut tmp___1: *mut uint8_t = 0 as *mut uint8_t;
    let mut buflen: libc::c_int = 0;
    let mut resplen: libc::c_int = 0;
    let mut challresp: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___2: *mut uint8_t = 0 as *mut uint8_t;
    let mut encoded: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut padding: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = msg_get_header(
        req,
        b"Apple-Challenge\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    hdr = tmp;
    if hdr.is_null() {
        return;
    }
    sa_len = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as socklen_t;
    getsockname(
        fd___1,
        &mut fdsa as *mut sockaddr_storage as *mut sockaddr,
        &mut sa_len as *mut socklen_t,
    );
    tmp___0 = base64_dec(hdr, &mut chall_len);
    chall = tmp___0;
    bp = buf.as_mut_ptr();
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[uint8_t; 48]>() as libc::c_ulong,
    );
    if chall_len > 16 as libc::c_int {
        warn(
            b"oversized Apple-Challenge!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        free(chall as *mut libc::c_void);
        return;
    }
    memcpy(bp as *mut libc::c_void, chall as *const libc::c_void, chall_len as size_t);
    free(chall as *mut libc::c_void);
    bp = bp.offset(chall_len as isize);
    if fdsa.ss_family as libc::c_int == 10 as libc::c_int {
        sa6 = &mut fdsa as *mut sockaddr_storage as *mut sockaddr_in6;
        memcpy(
            bp as *mut libc::c_void,
            ((*sa6).sin6_addr.__in6_u.__u6_addr8).as_mut_ptr() as *const libc::c_void,
            16 as libc::c_int as size_t,
        );
        bp = bp.offset(16 as libc::c_int as isize);
    } else {
        sa = &mut fdsa as *mut sockaddr_storage as *mut sockaddr_in;
        memcpy(
            bp as *mut libc::c_void,
            &mut (*sa).sin_addr.s_addr as *mut in_addr_t as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        bp = bp.offset(4 as libc::c_int as isize);
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        tmp___1 = bp;
        bp = bp.offset(1);
        *tmp___1 = config.hw_addr[i as usize];
        i += 1;
    }
    buflen = bp.offset_from(buf.as_mut_ptr()) as libc::c_long as libc::c_int;
    if buflen < 32 as libc::c_int {
        buflen = 32 as libc::c_int;
    }
    tmp___2 = rsa_apply(buf.as_mut_ptr(), buflen, &mut resplen, 0 as libc::c_int);
    challresp = tmp___2;
    tmp___3 = base64_enc(challresp, resplen);
    encoded = tmp___3;
    tmp___4 = strchr(encoded as *const libc::c_char, '=' as i32);
    padding = tmp___4;
    if !padding.is_null() {
        *padding = 0 as libc::c_int as libc::c_char;
    }
    msg_add_header(
        resp,
        b"Apple-Response\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        encoded,
    );
    free(challresp as *mut libc::c_void);
    free(encoded as *mut libc::c_void);
}
unsafe extern "C" fn make_nonce() -> *mut libc::c_char {
    let mut random___0: [uint8_t; 8] = [0; 8];
    let mut fd___1: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = open(b"/dev/random\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    fd___1 = tmp;
    if fd___1 < 0 as libc::c_int {
        die(
            b"could not open /dev/random!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    tmp___0 = read(
        fd___1,
        random___0.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[uint8_t; 8]>() as libc::c_ulong,
    );
    close(fd___1);
    tmp___1 = base64_enc(random___0.as_mut_ptr(), 8 as libc::c_int);
    return tmp___1;
}
unsafe extern "C" fn rtsp_auth(
    mut nonce: *mut *mut libc::c_char,
    mut req: *mut rtsp_message,
    mut resp: *mut rtsp_message,
) -> libc::c_int {
    let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    let mut realm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut username: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uri: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut quote: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut digest_urp: [uint8_t; 16] = [0; 16];
    let mut digest_mu: [uint8_t; 16] = [0; 16];
    let mut digest_total: [uint8_t; 16] = [0; 16];
    let mut ctx: MD5_CTX = MD5_CTX {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        Nl: 0,
        Nh: 0,
        data: [0; 16],
        num: 0,
    };
    let mut tmp___9: size_t = 0;
    let mut tmp___10: size_t = 0;
    let mut tmp___11: size_t = 0;
    let mut tmp___12: size_t = 0;
    let mut tmp___13: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 33] = [0; 33];
    let mut tmp___14: size_t = 0;
    let mut tmp___15: libc::c_int = 0;
    let mut hdrlen: libc::c_int = 0;
    let mut tmp___16: size_t = 0;
    let mut authhdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___17: *mut libc::c_void = 0 as *mut libc::c_void;
    if (config.password).is_null() {
        return 0 as libc::c_int;
    }
    if (*nonce).is_null() {
        *nonce = make_nonce();
    } else {
        tmp = msg_get_header(
            req,
            b"Authorization\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        hdr = tmp;
        if !hdr.is_null() {
            tmp___0 = strncmp(
                hdr as *const libc::c_char,
                b"Digest \0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as size_t,
            );
            if !(tmp___0 != 0) {
                tmp___1 = strstr(
                    hdr as *const libc::c_char,
                    b"realm=\"\0" as *const u8 as *const libc::c_char,
                );
                realm = tmp___1;
                tmp___2 = strstr(
                    hdr as *const libc::c_char,
                    b"username=\"\0" as *const u8 as *const libc::c_char,
                );
                username = tmp___2;
                tmp___3 = strstr(
                    hdr as *const libc::c_char,
                    b"response=\"\0" as *const u8 as *const libc::c_char,
                );
                response = tmp___3;
                tmp___4 = strstr(
                    hdr as *const libc::c_char,
                    b"uri=\"\0" as *const u8 as *const libc::c_char,
                );
                uri = tmp___4;
                if !realm.is_null() {
                    if !username.is_null() {
                        if !response.is_null() {
                            if !uri.is_null() {
                                tmp___5 = strchr(realm as *const libc::c_char, '"' as i32);
                                realm = tmp___5.offset(1 as libc::c_int as isize);
                                quote = strchr(realm as *const libc::c_char, '"' as i32);
                                if !quote.is_null() {
                                    *quote = 0 as libc::c_int as libc::c_char;
                                    tmp___6 = strchr(
                                        username as *const libc::c_char,
                                        '"' as i32,
                                    );
                                    username = tmp___6.offset(1 as libc::c_int as isize);
                                    quote = strchr(username as *const libc::c_char, '"' as i32);
                                    if !quote.is_null() {
                                        *quote = 0 as libc::c_int as libc::c_char;
                                        tmp___7 = strchr(
                                            response as *const libc::c_char,
                                            '"' as i32,
                                        );
                                        response = tmp___7.offset(1 as libc::c_int as isize);
                                        quote = strchr(response as *const libc::c_char, '"' as i32);
                                        if !quote.is_null() {
                                            *quote = 0 as libc::c_int as libc::c_char;
                                            tmp___8 = strchr(uri as *const libc::c_char, '"' as i32);
                                            uri = tmp___8.offset(1 as libc::c_int as isize);
                                            quote = strchr(uri as *const libc::c_char, '"' as i32);
                                            if !quote.is_null() {
                                                *quote = 0 as libc::c_int as libc::c_char;
                                                MD5_Init(&mut ctx);
                                                tmp___9 = strlen(username as *const libc::c_char);
                                                MD5_Update(
                                                    &mut ctx,
                                                    username as *const libc::c_void,
                                                    tmp___9,
                                                );
                                                MD5_Update(
                                                    &mut ctx,
                                                    b":\0" as *const u8 as *const libc::c_char
                                                        as *const libc::c_void,
                                                    1 as libc::c_int as size_t,
                                                );
                                                tmp___10 = strlen(realm as *const libc::c_char);
                                                MD5_Update(
                                                    &mut ctx,
                                                    realm as *const libc::c_void,
                                                    tmp___10,
                                                );
                                                MD5_Update(
                                                    &mut ctx,
                                                    b":\0" as *const u8 as *const libc::c_char
                                                        as *const libc::c_void,
                                                    1 as libc::c_int as size_t,
                                                );
                                                tmp___11 = strlen(config.password as *const libc::c_char);
                                                MD5_Update(
                                                    &mut ctx,
                                                    config.password as *const libc::c_void,
                                                    tmp___11,
                                                );
                                                MD5_Final(digest_urp.as_mut_ptr(), &mut ctx);
                                                MD5_Init(&mut ctx);
                                                tmp___12 = strlen(
                                                    ((*req).method).as_mut_ptr() as *const libc::c_char,
                                                );
                                                MD5_Update(
                                                    &mut ctx,
                                                    ((*req).method).as_mut_ptr() as *const libc::c_void,
                                                    tmp___12,
                                                );
                                                MD5_Update(
                                                    &mut ctx,
                                                    b":\0" as *const u8 as *const libc::c_char
                                                        as *const libc::c_void,
                                                    1 as libc::c_int as size_t,
                                                );
                                                tmp___13 = strlen(uri as *const libc::c_char);
                                                MD5_Update(&mut ctx, uri as *const libc::c_void, tmp___13);
                                                MD5_Final(digest_mu.as_mut_ptr(), &mut ctx);
                                                i = 0 as libc::c_int;
                                                while i < 16 as libc::c_int {
                                                    sprintf(
                                                        buf.as_mut_ptr().offset((2 as libc::c_int * i) as isize),
                                                        b"%02x\0" as *const u8 as *const libc::c_char,
                                                        digest_urp[i as usize] as libc::c_int,
                                                    );
                                                    i += 1;
                                                }
                                                MD5_Init(&mut ctx);
                                                MD5_Update(
                                                    &mut ctx,
                                                    buf.as_mut_ptr() as *const libc::c_void,
                                                    32 as libc::c_int as size_t,
                                                );
                                                MD5_Update(
                                                    &mut ctx,
                                                    b":\0" as *const u8 as *const libc::c_char
                                                        as *const libc::c_void,
                                                    1 as libc::c_int as size_t,
                                                );
                                                tmp___14 = strlen(*nonce as *const libc::c_char);
                                                MD5_Update(
                                                    &mut ctx,
                                                    *nonce as *const libc::c_void,
                                                    tmp___14,
                                                );
                                                MD5_Update(
                                                    &mut ctx,
                                                    b":\0" as *const u8 as *const libc::c_char
                                                        as *const libc::c_void,
                                                    1 as libc::c_int as size_t,
                                                );
                                                i = 0 as libc::c_int;
                                                while i < 16 as libc::c_int {
                                                    sprintf(
                                                        buf.as_mut_ptr().offset((2 as libc::c_int * i) as isize),
                                                        b"%02x\0" as *const u8 as *const libc::c_char,
                                                        digest_mu[i as usize] as libc::c_int,
                                                    );
                                                    i += 1;
                                                }
                                                MD5_Update(
                                                    &mut ctx,
                                                    buf.as_mut_ptr() as *const libc::c_void,
                                                    32 as libc::c_int as size_t,
                                                );
                                                MD5_Final(digest_total.as_mut_ptr(), &mut ctx);
                                                i = 0 as libc::c_int;
                                                while i < 16 as libc::c_int {
                                                    sprintf(
                                                        buf.as_mut_ptr().offset((2 as libc::c_int * i) as isize),
                                                        b"%02x\0" as *const u8 as *const libc::c_char,
                                                        digest_total[i as usize] as libc::c_int,
                                                    );
                                                    i += 1;
                                                }
                                                tmp___15 = strcmp(
                                                    response as *const libc::c_char,
                                                    buf.as_mut_ptr() as *const libc::c_char,
                                                );
                                                if tmp___15 == 0 {
                                                    return 0 as libc::c_int;
                                                }
                                                warn(
                                                    b"auth failed\0" as *const u8 as *const libc::c_char
                                                        as *mut libc::c_char,
                                                );
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
    (*resp).respcode = 401 as libc::c_int;
    tmp___16 = strlen(*nonce as *const libc::c_char);
    hdrlen = tmp___16.wrapping_add(40 as libc::c_ulong) as libc::c_int;
    tmp___17 = malloc(hdrlen as size_t);
    authhdr = tmp___17 as *mut libc::c_char;
    snprintf(
        authhdr,
        hdrlen as size_t,
        b"Digest realm=\"taco\", nonce=\"%s\"\0" as *const u8 as *const libc::c_char,
        *nonce,
    );
    msg_add_header(
        resp,
        b"WWW-Authenticate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        authhdr,
    );
    free(authhdr as *mut libc::c_void);
    return 1 as libc::c_int;
}
unsafe extern "C" fn rtsp_conversation_thread_func(
    mut pconn: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut set: sigset_t = sigset_t { __val: [0; 16] };
    let mut conn: *mut rtsp_conn_info = 0 as *mut rtsp_conn_info;
    let mut req: *mut rtsp_message = 0 as *mut rtsp_message;
    let mut resp: *mut rtsp_message = 0 as *mut rtsp_message;
    let mut hdr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut auth_nonce: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut mh: *mut method_handler = 0 as *mut method_handler;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    sigemptyset(&mut set);
    sigaddset(&mut set, 10 as libc::c_int);
    pthread_sigmask(
        1 as libc::c_int,
        &mut set as *mut sigset_t as *const __sigset_t,
        0 as *mut libc::c_void as *mut __sigset_t,
    );
    conn = pconn as *mut rtsp_conn_info;
    auth_nonce = 0 as *mut libc::c_void as *mut libc::c_char;
    loop {
        req = rtsp_read_request((*conn).fd);
        if req.is_null() {
            break;
        }
        resp = msg_init();
        (*resp).respcode = 400 as libc::c_int;
        apple_challenge((*conn).fd, req, resp);
        hdr = msg_get_header(
            req,
            b"CSeq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !hdr.is_null() {
            msg_add_header(
                resp,
                b"CSeq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                hdr,
            );
        }
        msg_add_header(
            resp,
            b"Audio-Jack-Status\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"connected; type=analog\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        tmp = rtsp_auth(&mut auth_nonce, req, resp);
        if !(tmp != 0) {
            mh = method_handlers.as_mut_ptr();
            while !((*mh).method).is_null() {
                tmp___0 = strcmp(
                    (*mh).method as *const libc::c_char,
                    ((*req).method).as_mut_ptr() as *const libc::c_char,
                );
                if tmp___0 == 0 {
                    (Some(((*mh).handler).expect("non-null function pointer")))
                        .expect("non-null function pointer")(conn, req, resp);
                    break;
                } else {
                    mh = mh.offset(1);
                }
            }
        }
        msg_write_response((*conn).fd, resp);
        msg_free(req);
        msg_free(resp);
    }
    debug(
        1 as libc::c_int,
        b"closing RTSP connection\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if (*conn).fd > 0 as libc::c_int {
        close((*conn).fd);
    }
    tmp___1 = rtsp_playing();
    if tmp___1 != 0 {
        rtp_shutdown();
        player_stop();
        please_shutdown = 0 as libc::c_int;
        pthread_mutex_unlock(&mut playing_mutex);
    }
    if !auth_nonce.is_null() {
        free(auth_nonce as *mut libc::c_void);
    }
    (*conn).running = 0 as libc::c_int;
    debug(
        2 as libc::c_int,
        b"terminating RTSP thread\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return 0 as *mut libc::c_void;
}
static mut string: [libc::c_char; 46] = [0; 46];
unsafe extern "C" fn format_address(mut fsa: *mut sockaddr) -> *const libc::c_char {
    let mut addr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut sa6: *mut sockaddr_in6 = 0 as *mut sockaddr_in6;
    let mut sa: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    if (*fsa).sa_family as libc::c_int == 10 as libc::c_int {
        sa6 = fsa as *mut sockaddr_in6;
        addr = &mut (*sa6).sin6_addr as *mut in6_addr as *mut libc::c_void;
    } else {
        sa = fsa as *mut sockaddr_in;
        addr = &mut (*sa).sin_addr as *mut in_addr as *mut libc::c_void;
    }
    tmp = inet_ntop(
        (*fsa).sa_family as libc::c_int,
        addr as *const libc::c_void,
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong as socklen_t,
    );
    return tmp;
}
pub unsafe extern "C" fn rtsp_listen_loop() {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut info: *mut addrinfo = 0 as *mut addrinfo;
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    let mut portstr: [libc::c_char; 6] = [0; 6];
    let mut sockfd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nsock: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut fd___1: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut yes: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut maxfd: libc::c_int = 0;
    let mut fds: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let mut acceptfd: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut conn: *mut rtsp_conn_info = 0 as *mut rtsp_conn_info;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut slen: socklen_t = 0;
    let mut rtsp_conversation_thread: pthread_t = 0;
    sockfd = 0 as *mut libc::c_void as *mut libc::c_int;
    nsock = 0 as libc::c_int;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = 1 as libc::c_int;
    hints.ai_flags = 1 as libc::c_int;
    snprintf(
        portstr.as_mut_ptr(),
        6 as libc::c_int as size_t,
        b"%d\0" as *const u8 as *const libc::c_char,
        config.port,
    );
    ret = getaddrinfo(
        0 as *mut libc::c_void as *const libc::c_char,
        portstr.as_mut_ptr() as *const libc::c_char,
        &mut hints as *mut addrinfo as *const addrinfo,
        &mut info as *mut *mut addrinfo,
    );
    if ret != 0 {
        tmp = gai_strerror(ret);
        die(
            b"getaddrinfo failed: %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            tmp,
        );
    }
    p = info;
    while !p.is_null() {
        tmp___0 = socket((*p).ai_family, (*p).ai_socktype, 6 as libc::c_int);
        fd___1 = tmp___0;
        yes = 1 as libc::c_int;
        ret = setsockopt(
            fd___1,
            1 as libc::c_int,
            2 as libc::c_int,
            &mut yes as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
        if (*p).ai_family == 10 as libc::c_int {
            tmp___1 = setsockopt(
                fd___1,
                41 as libc::c_int,
                26 as libc::c_int,
                &mut yes as *mut libc::c_int as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
            ret |= tmp___1;
        }
        if ret == 0 {
            ret = bind(fd___1, (*p).ai_addr as *const sockaddr, (*p).ai_addrlen);
        }
        if ret != 0 {
            tmp___2 = format_address((*p).ai_addr);
            debug(
                1 as libc::c_int,
                b"Failed to bind to address %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                tmp___2,
            );
        } else {
            tmp___3 = format_address((*p).ai_addr);
            debug(
                1 as libc::c_int,
                b"Bound to address %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                tmp___3,
            );
            listen(fd___1, 5 as libc::c_int);
            nsock += 1;
            tmp___4 = realloc(
                sockfd as *mut libc::c_void,
                (nsock as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
            sockfd = tmp___4 as *mut libc::c_int;
            *sockfd.offset((nsock - 1 as libc::c_int) as isize) = fd___1;
        }
        p = (*p).ai_next;
    }
    freeaddrinfo(info);
    if nsock == 0 {
        die(
            b"could not bind any listen sockets!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    maxfd = -(1 as libc::c_int);
    let fresh2 = &mut __d0;
    let fresh3;
    let fresh4 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh5 = &mut __d1;
    let fresh6;
    let fresh7 = &mut *(fds.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh2,
        fresh4) => fresh3, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh5,
        fresh7) => fresh6, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
    c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
    i = 0 as libc::c_int;
    while i < nsock {
        if *sockfd.offset(i as isize) > maxfd {
            maxfd = *sockfd.offset(i as isize);
        }
        i += 1;
    }
    mdns_register();
    printf(b"Listening for connections.\n\0" as *const u8 as *const libc::c_char);
    shairport_startup_complete();
    loop {
        tv.tv_sec = 300 as libc::c_int as __time_t;
        tv.tv_usec = 0 as libc::c_int as __suseconds_t;
        i = 0 as libc::c_int;
        while i < nsock {
            fds
                .__fds_bits[(*sockfd.offset(i as isize)
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                |= ((1 as libc::c_ulong)
                    << *sockfd.offset(i as isize)
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask;
            i += 1;
        }
        ret = select(
            maxfd + 1 as libc::c_int,
            &mut fds as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut tv as *mut timeval,
        );
        if ret < 0 as libc::c_int {
            tmp___5 = __errno_location();
            if !(*tmp___5 == 4 as libc::c_int) {
                break;
            }
        } else {
            cleanup_threads();
            acceptfd = -(1 as libc::c_int);
            i = 0 as libc::c_int;
            while i < nsock {
                if fds
                    .__fds_bits[(*sockfd.offset(i as isize)
                    / (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as usize]
                    & ((1 as libc::c_ulong)
                        << *sockfd.offset(i as isize)
                            % (8 as libc::c_int
                                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                    as libc::c_int)) as __fd_mask != 0 as libc::c_long
                {
                    acceptfd = *sockfd.offset(i as isize);
                    break;
                } else {
                    i += 1;
                }
            }
            if acceptfd < 0 as libc::c_int {
                continue;
            }
            tmp___6 = malloc(::std::mem::size_of::<rtsp_conn_info>() as libc::c_ulong);
            conn = tmp___6 as *mut rtsp_conn_info;
            memset(
                conn as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<rtsp_conn_info>() as libc::c_ulong,
            );
            slen = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong
                as socklen_t;
            debug(
                1 as libc::c_int,
                b"new RTSP connection\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            (*conn)
                .fd = accept(
                acceptfd,
                &mut (*conn).remote as *mut sockaddr_storage as *mut sockaddr,
                &mut slen as *mut socklen_t,
            );
            if (*conn).fd < 0 as libc::c_int {
                perror(
                    b"failed to accept connection\0" as *const u8 as *const libc::c_char,
                );
                free(conn as *mut libc::c_void);
            } else {
                ret = pthread_create(
                    &mut rtsp_conversation_thread as *mut pthread_t,
                    0 as *mut libc::c_void as *const pthread_attr_t,
                    Some(
                        rtsp_conversation_thread_func
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                            ) -> *mut libc::c_void,
                    ),
                    conn as *mut libc::c_void,
                );
                if ret != 0 {
                    die(
                        b"Failed to create RTSP receiver thread!\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                (*conn).thread = rtsp_conversation_thread;
                (*conn).running = 1 as libc::c_int;
                track_thread(conn);
            }
        }
    }
    perror(b"select\0" as *const u8 as *const libc::c_char);
    die(
        b"fell out of the RTSP select loop\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
static mut mdns_backends: [*mut mdns_backend; 4] = unsafe {
    [
        &mdns_external_avahi as *const mdns_backend as *mut mdns_backend,
        &mdns_external_dns_sd as *const mdns_backend as *mut mdns_backend,
        &mdns_tinysvcmdns as *const mdns_backend as *mut mdns_backend,
        0 as *const libc::c_void as *mut libc::c_void as *mut mdns_backend,
    ]
};
pub unsafe extern "C" fn mdns_register() {
    let mut mdns_apname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: *mut *mut mdns_backend = 0 as *mut *mut mdns_backend;
    let mut tmp___2: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut error___0: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    tmp = strlen(config.apname as *const libc::c_char);
    tmp___0 = malloc(tmp.wrapping_add(14 as libc::c_ulong));
    mdns_apname = tmp___0 as *mut libc::c_char;
    p = mdns_apname;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        sprintf(
            p,
            b"%02X\0" as *const u8 as *const libc::c_char,
            config.hw_addr[i as usize] as libc::c_int,
        );
        p = p.offset(2 as libc::c_int as isize);
        i += 1;
    }
    tmp___1 = p;
    p = p.offset(1);
    *tmp___1 = '@' as i32 as libc::c_char;
    strcpy(p, config.apname as *const libc::c_char);
    b = 0 as *mut libc::c_void as *mut *mut mdns_backend;
    if config.mdns_name as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        b = mdns_backends.as_mut_ptr();
        while !(*b).is_null() {
            tmp___2 = strcmp(
                (**b).name as *const libc::c_char,
                config.mdns_name as *const libc::c_char,
            );
            if tmp___2 != 0 as libc::c_int {
                b = b.offset(1);
            } else {
                tmp___3 = (Some(
                    ((**b).mdns_register).expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(mdns_apname, config.port);
                error = tmp___3;
                if error >= 0 as libc::c_int {
                    config.mdns = *b;
                }
                break;
            }
        }
        if *b as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            warn(
                b"%s mDNS backend not found\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    } else {
        b = mdns_backends.as_mut_ptr();
        while !(*b).is_null() {
            tmp___4 = (Some(((**b).mdns_register).expect("non-null function pointer")))
                .expect("non-null function pointer")(mdns_apname, config.port);
            error___0 = tmp___4;
            if error___0 >= 0 as libc::c_int {
                config.mdns = *b;
                break;
            } else {
                b = b.offset(1);
            }
        }
    }
    if config.mdns as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        die(
            b"Could not establish mDNS advertisement!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
}
pub unsafe extern "C" fn mdns_unregister() {
    if !(config.mdns).is_null() {
        (Some(((*config.mdns).mdns_unregister).expect("non-null function pointer")))
            .expect("non-null function pointer")();
    }
}
pub unsafe extern "C" fn mdns_ls_backends() {
    let mut b: *mut *mut mdns_backend = 0 as *mut *mut mdns_backend;
    b = 0 as *mut libc::c_void as *mut *mut mdns_backend;
    printf(b"Available mDNS backends: \n\0" as *const u8 as *const libc::c_char);
    b = mdns_backends.as_mut_ptr();
    while !(*b).is_null() {
        printf(b"    %s\n\0" as *const u8 as *const libc::c_char, (**b).name);
        b = b.offset(1);
    }
}
pub static mut mdns_pid: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn fork_execvp(
    mut file: *const libc::c_char,
    mut argv: *const *mut libc::c_char,
) -> libc::c_int {
    let mut execpipe: [libc::c_int; 2] = [0; 2];
    let mut pid: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: ssize_t = 0;
    let mut childErrno: libc::c_int = 0;
    let mut tmp___5: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___6: ssize_t = 0;
    pid = 0 as libc::c_int;
    tmp = pipe(execpipe.as_mut_ptr());
    if tmp < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    tmp___0 = fcntl(execpipe[1 as libc::c_int as usize], 1 as libc::c_int);
    tmp___1 = fcntl(
        execpipe[1 as libc::c_int as usize],
        2 as libc::c_int,
        tmp___0 | 1 as libc::c_int,
    );
    if tmp___1 < 0 as libc::c_int {
        close(execpipe[0 as libc::c_int as usize]);
        close(execpipe[1 as libc::c_int as usize]);
        return -(1 as libc::c_int);
    }
    pid = fork();
    if pid < 0 as libc::c_int {
        close(execpipe[0 as libc::c_int as usize]);
        close(execpipe[1 as libc::c_int as usize]);
        return -(1 as libc::c_int);
    } else if pid == 0 as libc::c_int {
        close(execpipe[0 as libc::c_int as usize]);
        execvp(file, argv);
        tmp___3 = __errno_location();
        tmp___4 = write(
            execpipe[1 as libc::c_int as usize],
            tmp___3 as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        );
        _exit(-(1 as libc::c_int));
    } else {
        close(execpipe[1 as libc::c_int as usize]);
        tmp___6 = read(
            execpipe[0 as libc::c_int as usize],
            &mut childErrno as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        );
        if tmp___6 as libc::c_ulong
            == ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        {
            tmp___5 = __errno_location();
            *tmp___5 = childErrno;
            return -(1 as libc::c_int);
        } else {
            return pid
        }
    };
}
unsafe extern "C" fn mdns_external_avahi_register(
    mut apname: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut mdns_port: [libc::c_char; 6] = [0; 6];
    let mut argv: [*mut libc::c_char; 18] = [0 as *mut libc::c_char; 18];
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut pid: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    sprintf(
        mdns_port.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        config.port,
    );
    if !(config.password).is_null() {
        tmp = b"pw=true\0" as *const u8 as *const libc::c_char;
    } else {
        tmp = b"pw=false\0" as *const u8 as *const libc::c_char;
    }
    argv[0 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
    argv[1 as libc::c_int as usize] = apname;
    argv[2 as libc::c_int
        as usize] = b"_raop._tcp\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[3 as libc::c_int as usize] = mdns_port.as_mut_ptr();
    argv[4 as libc::c_int
        as usize] = b"tp=UDP\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[5 as libc::c_int
        as usize] = b"sm=false\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[6 as libc::c_int
        as usize] = b"ek=1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[7 as libc::c_int
        as usize] = b"et=0,1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[8 as libc::c_int
        as usize] = b"cn=0,1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[9 as libc::c_int
        as usize] = b"ch=2\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[10 as libc::c_int
        as usize] = b"ss=16\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[11 as libc::c_int
        as usize] = b"sr=44100\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[12 as libc::c_int
        as usize] = b"vn=3\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[13 as libc::c_int
        as usize] = b"txtvers=1\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[14 as libc::c_int
        as usize] = b"da=true\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[15 as libc::c_int
        as usize] = b"md=0,1,2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[16 as libc::c_int as usize] = tmp as *mut libc::c_char;
    argv[17 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
    argv[0 as libc::c_int
        as usize] = b"avahi-publish-service\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    tmp___0 = fork_execvp(
        argv[0 as libc::c_int as usize] as *const libc::c_char,
        argv.as_mut_ptr() as *const *mut libc::c_char,
    );
    pid = tmp___0;
    if pid >= 0 as libc::c_int {
        mdns_pid = pid;
        return 0 as libc::c_int;
    } else {
        debug(
            1 as libc::c_int,
            b"Calling %s failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            argv[0 as libc::c_int as usize],
        );
    }
    argv[0 as libc::c_int
        as usize] = b"mDNSPublish\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    pid = fork_execvp(
        argv[0 as libc::c_int as usize] as *const libc::c_char,
        argv.as_mut_ptr() as *const *mut libc::c_char,
    );
    if pid >= 0 as libc::c_int {
        mdns_pid = pid;
        return 0 as libc::c_int;
    } else {
        debug(
            1 as libc::c_int,
            b"Calling %s failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            argv[0 as libc::c_int as usize],
        );
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn mdns_external_dns_sd_register(
    mut apname: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut mdns_port: [libc::c_char; 6] = [0; 6];
    let mut argv: [*mut libc::c_char; 20] = [0 as *mut libc::c_char; 20];
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut pid: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    sprintf(
        mdns_port.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        config.port,
    );
    if !(config.password).is_null() {
        tmp = b"pw=true\0" as *const u8 as *const libc::c_char;
    } else {
        tmp = b"pw=false\0" as *const u8 as *const libc::c_char;
    }
    argv[0 as libc::c_int
        as usize] = b"dns-sd\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[1 as libc::c_int
        as usize] = b"-R\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[2 as libc::c_int as usize] = apname;
    argv[3 as libc::c_int
        as usize] = b"_raop._tcp\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[4 as libc::c_int
        as usize] = b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[5 as libc::c_int as usize] = mdns_port.as_mut_ptr();
    argv[6 as libc::c_int
        as usize] = b"tp=UDP\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[7 as libc::c_int
        as usize] = b"sm=false\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[8 as libc::c_int
        as usize] = b"ek=1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[9 as libc::c_int
        as usize] = b"et=0,1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[10 as libc::c_int
        as usize] = b"cn=0,1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[11 as libc::c_int
        as usize] = b"ch=2\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[12 as libc::c_int
        as usize] = b"ss=16\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[13 as libc::c_int
        as usize] = b"sr=44100\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[14 as libc::c_int
        as usize] = b"vn=3\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    argv[15 as libc::c_int
        as usize] = b"txtvers=1\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[16 as libc::c_int
        as usize] = b"da=true\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[17 as libc::c_int
        as usize] = b"md=0,1,2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    argv[18 as libc::c_int as usize] = tmp as *mut libc::c_char;
    argv[19 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut libc::c_char;
    tmp___0 = fork_execvp(
        argv[0 as libc::c_int as usize] as *const libc::c_char,
        argv.as_mut_ptr() as *const *mut libc::c_char,
    );
    pid = tmp___0;
    if pid >= 0 as libc::c_int {
        mdns_pid = pid;
        return 0 as libc::c_int;
    } else {
        debug(
            1 as libc::c_int,
            b"Calling %s failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            argv[0 as libc::c_int as usize],
        );
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn kill_mdns_child() {
    if mdns_pid != 0 {
        kill(mdns_pid, 15 as libc::c_int);
    }
    mdns_pid = 0 as libc::c_int;
}
pub static mut mdns_external_avahi: mdns_backend = {
    let mut init = __anonstruct_mdns_backend_66454024 {
        name: b"external-avahi\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        mdns_register: Some(
            mdns_external_avahi_register
                as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
        ),
        mdns_unregister: Some(kill_mdns_child as unsafe extern "C" fn() -> ()),
    };
    init
};
pub static mut mdns_external_dns_sd: mdns_backend = {
    let mut init = __anonstruct_mdns_backend_66454024 {
        name: b"external-dns-sd\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        mdns_register: Some(
            mdns_external_dns_sd_register
                as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
        ),
        mdns_unregister: Some(kill_mdns_child as unsafe extern "C" fn() -> ()),
    };
    init
};
static mut svr: *mut mdnsd = 0 as *const libc::c_void as *mut libc::c_void as *mut mdnsd;
unsafe extern "C" fn mdns_tinysvcmdns_register(
    mut apname: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut ifalist: *mut ifaddrs = 0 as *mut ifaddrs;
    let mut ifa: *mut ifaddrs = 0 as *mut ifaddrs;
    let mut hostname: [libc::c_char; 106] = [0; 106];
    let mut hostend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut main_ip: uint32_t = 0;
    let mut addr: *mut in6_addr = 0 as *mut in6_addr;
    let mut ip: uint32_t = 0;
    let mut a_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp___3: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___4: *mut rr_entry = 0 as *mut rr_entry;
    let mut addr___0: *mut in6_addr = 0 as *mut in6_addr;
    let mut aaaa_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp___5: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___6: *mut rr_entry = 0 as *mut rr_entry;
    let mut txt: [*const libc::c_char; 14] = [0 as *const libc::c_char; 14];
    let mut tmp___7: *const libc::c_char = 0 as *const libc::c_char;
    let mut svc: *mut mdns_service = 0 as *mut mdns_service;
    let mut tmp___8: *mut mdns_service = 0 as *mut mdns_service;
    svr = mdnsd_start();
    if svr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        warn(
            b"tinysvcmdns: mdnsd_start() failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    gethostname(hostname.as_mut_ptr(), 99 as libc::c_int as size_t);
    hostname[99 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp = strlen(hostname.as_mut_ptr() as *const libc::c_char);
    hostend = hostname.as_mut_ptr().offset(tmp as isize);
    tmp___0 = strlen(hostname.as_mut_ptr() as *const libc::c_char);
    if tmp___0 > 6 as libc::c_ulong {
        tmp___1 = strcmp(
            hostend.offset(-(6 as libc::c_int as isize)) as *const libc::c_char,
            b".local\0" as *const u8 as *const libc::c_char,
        );
        if tmp___1 != 0 {
            strcat(
                hostname.as_mut_ptr(),
                b".local\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    tmp___2 = getifaddrs(&mut ifalist);
    if tmp___2 < 0 as libc::c_int {
        warn(
            b"tinysvcmdns: getifaddrs() failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ifa = ifalist;
    ifa = ifalist;
    while ifa as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if (*ifa).ifa_flags & 8 as libc::c_uint == 0 {
            if !((*ifa).ifa_addr).is_null() {
                if (*(*ifa).ifa_addr).sa_family as libc::c_int == 2 as libc::c_int {
                    main_ip = (*((*ifa).ifa_addr as *mut sockaddr_in)).sin_addr.s_addr;
                    mdnsd_set_hostname(
                        svr,
                        hostname.as_mut_ptr() as *const libc::c_char,
                        main_ip,
                    );
                    break;
                }
            }
        }
        if (*ifa).ifa_flags & 8 as libc::c_uint == 0 {
            if !((*ifa).ifa_addr).is_null() {
                if (*(*ifa).ifa_addr).sa_family as libc::c_int == 10 as libc::c_int {
                    addr = &mut (*((*ifa).ifa_addr as *mut sockaddr_in6)).sin6_addr;
                    mdnsd_set_hostname_v6(
                        svr,
                        hostname.as_mut_ptr() as *const libc::c_char,
                        addr,
                    );
                    break;
                }
            }
        }
        ifa = (*ifa).ifa_next;
    }
    if ifa as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        warn(
            b"tinysvcmdns: no non-loopback ipv4 or ipv6 interface found\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    ifa = (*ifa).ifa_next;
    while ifa as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if !((*ifa).ifa_flags & 8 as libc::c_uint != 0) {
            match (*(*ifa).ifa_addr).sa_family as libc::c_int {
                2 => {
                    ip = (*((*ifa).ifa_addr as *mut sockaddr_in)).sin_addr.s_addr;
                    tmp___3 = create_nlabel(
                        hostname.as_mut_ptr() as *const libc::c_char,
                    );
                    tmp___4 = rr_create_a(tmp___3, ip);
                    a_e = tmp___4;
                    mdnsd_add_rr(svr, a_e);
                }
                10 => {
                    addr___0 = &mut (*((*ifa).ifa_addr as *mut sockaddr_in6)).sin6_addr;
                    tmp___5 = create_nlabel(
                        hostname.as_mut_ptr() as *const libc::c_char,
                    );
                    tmp___6 = rr_create_aaaa(tmp___5, addr___0);
                    aaaa_e = tmp___6;
                    mdnsd_add_rr(svr, aaaa_e);
                }
                _ => {}
            }
        }
        ifa = (*ifa).ifa_next;
    }
    freeifaddrs(ifa);
    if !(config.password).is_null() {
        tmp___7 = b"pw=true\0" as *const u8 as *const libc::c_char;
    } else {
        tmp___7 = b"pw=false\0" as *const u8 as *const libc::c_char;
    }
    txt[0 as libc::c_int as usize] = b"tp=UDP\0" as *const u8 as *const libc::c_char;
    txt[1 as libc::c_int as usize] = b"sm=false\0" as *const u8 as *const libc::c_char;
    txt[2 as libc::c_int as usize] = b"ek=1\0" as *const u8 as *const libc::c_char;
    txt[3 as libc::c_int as usize] = b"et=0,1\0" as *const u8 as *const libc::c_char;
    txt[4 as libc::c_int as usize] = b"cn=0,1\0" as *const u8 as *const libc::c_char;
    txt[5 as libc::c_int as usize] = b"ch=2\0" as *const u8 as *const libc::c_char;
    txt[6 as libc::c_int as usize] = b"ss=16\0" as *const u8 as *const libc::c_char;
    txt[7 as libc::c_int as usize] = b"sr=44100\0" as *const u8 as *const libc::c_char;
    txt[8 as libc::c_int as usize] = b"vn=3\0" as *const u8 as *const libc::c_char;
    txt[9 as libc::c_int as usize] = b"txtvers=1\0" as *const u8 as *const libc::c_char;
    txt[10 as libc::c_int as usize] = b"da=true\0" as *const u8 as *const libc::c_char;
    txt[11 as libc::c_int as usize] = b"md=0,1,2\0" as *const u8 as *const libc::c_char;
    txt[12 as libc::c_int as usize] = tmp___7;
    txt[13 as libc::c_int as usize] = 0 as *mut libc::c_void as *const libc::c_char;
    tmp___8 = mdnsd_register_svc(
        svr,
        apname as *const libc::c_char,
        b"_raop._tcp.local\0" as *const u8 as *const libc::c_char,
        port as uint16_t,
        0 as *mut libc::c_void as *const libc::c_char,
        txt.as_mut_ptr(),
    );
    svc = tmp___8;
    mdns_service_destroy(svc);
    return 0 as libc::c_int;
}
unsafe extern "C" fn mdns_tinysvcmdns_unregister() {
    if !svr.is_null() {
        mdnsd_stop(svr);
        svr = 0 as *mut libc::c_void as *mut mdnsd;
    }
}
pub static mut mdns_tinysvcmdns: mdns_backend = {
    let mut init = __anonstruct_mdns_backend_66454024 {
        name: b"tinysvcmdns\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        mdns_register: Some(
            mdns_tinysvcmdns_register
                as unsafe extern "C" fn(*mut libc::c_char, libc::c_int) -> libc::c_int,
        ),
        mdns_unregister: Some(
            mdns_tinysvcmdns_unregister as unsafe extern "C" fn() -> (),
        ),
    };
    init
};
pub static mut config: shairport_cfg = shairport_cfg {
    password: 0 as *const libc::c_char as *mut libc::c_char,
    apname: 0 as *const libc::c_char as *mut libc::c_char,
    hw_addr: [0; 6],
    port: 0,
    output_name: 0 as *const libc::c_char as *mut libc::c_char,
    output: 0 as *const audio_output as *mut audio_output,
    mdns_name: 0 as *const libc::c_char as *mut libc::c_char,
    mdns: 0 as *const mdns_backend as *mut mdns_backend,
    buffer_start_fill: 0,
    daemonise: 0,
    cmd_start: 0 as *const libc::c_char as *mut libc::c_char,
    cmd_stop: 0 as *const libc::c_char as *mut libc::c_char,
    cmd_blocking: 0,
    meta_dir: 0 as *const libc::c_char as *mut libc::c_char,
    pidfile: 0 as *const libc::c_char as *mut libc::c_char,
    logfile: 0 as *const libc::c_char as *mut libc::c_char,
    errfile: 0 as *const libc::c_char as *mut libc::c_char,
};
pub static mut debuglev: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn die(mut format: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    fprintf(stderr, b"FATAL: \0" as *const u8 as *const libc::c_char);
    args_0 = args.clone();
    vfprintf(stderr, format as *const libc::c_char, args_0.as_va_list());
    if config.daemonise != 0 {
        daemon_fail(format as *const libc::c_char, args_0.as_va_list());
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    shairport_shutdown(1 as libc::c_int);
}
pub unsafe extern "C" fn warn(mut format: *mut libc::c_char, mut args: ...) {
    let mut args_0: ::std::ffi::VaListImpl;
    fprintf(stderr, b"WARNING: \0" as *const u8 as *const libc::c_char);
    args_0 = args.clone();
    vfprintf(stderr, format as *const libc::c_char, args_0.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn debug(
    mut level: libc::c_int,
    mut format: *mut libc::c_char,
    mut args: ...
) {
    let mut args_0: ::std::ffi::VaListImpl;
    if level > debuglev {
        return;
    }
    args_0 = args.clone();
    vfprintf(stderr, format as *const libc::c_char, args_0.as_va_list());
}
pub unsafe extern "C" fn base64_enc(
    mut input: *mut uint8_t,
    mut length: libc::c_int,
) -> *mut libc::c_char {
    let mut bmem: *mut BIO = 0 as *mut BIO;
    let mut b64: *mut BIO = 0 as *mut BIO;
    let mut bptr: *mut BUF_MEM = 0 as *mut BUF_MEM;
    let mut tmp: *const BIO_METHOD = 0 as *const BIO_METHOD;
    let mut tmp___0: *const BIO_METHOD = 0 as *const BIO_METHOD;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = BIO_f_base64();
    b64 = BIO_new(tmp);
    tmp___0 = BIO_s_mem();
    bmem = BIO_new(tmp___0);
    b64 = BIO_push(b64, bmem);
    BIO_set_flags(b64, 256 as libc::c_int);
    BIO_write(b64, input as *const libc::c_void, length);
    BIO_ctrl(b64, 11 as libc::c_int, 0 as libc::c_long, 0 as *mut libc::c_void);
    BIO_ctrl(
        b64,
        115 as libc::c_int,
        0 as libc::c_long,
        &mut bptr as *mut *mut BUF_MEM as *mut libc::c_char as *mut libc::c_void,
    );
    tmp___1 = malloc((*bptr).length);
    buf = tmp___1 as *mut libc::c_char;
    if (*bptr).length != 0 {
        memcpy(
            buf as *mut libc::c_void,
            (*bptr).data as *const libc::c_void,
            ((*bptr).length).wrapping_sub(1 as libc::c_ulong),
        );
        *buf
            .offset(
                ((*bptr).length).wrapping_sub(1 as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
    }
    BIO_free_all(bmem);
    return buf;
}
pub unsafe extern "C" fn base64_dec(
    mut input: *mut libc::c_char,
    mut outlen: *mut libc::c_int,
) -> *mut uint8_t {
    let mut bmem: *mut BIO = 0 as *mut BIO;
    let mut b64: *mut BIO = 0 as *mut BIO;
    let mut inlen: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *const BIO_METHOD = 0 as *const BIO_METHOD;
    let mut tmp___1: *const BIO_METHOD = 0 as *const BIO_METHOD;
    let mut tmp___2: libc::c_int = 0;
    let mut bufsize: libc::c_int = 0;
    let mut tmp___3: size_t = 0;
    let mut buf: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nread: libc::c_int = 0;
    tmp = strlen(input as *const libc::c_char);
    inlen = tmp as libc::c_int;
    tmp___0 = BIO_f_base64();
    b64 = BIO_new(tmp___0);
    BIO_set_flags(b64, 256 as libc::c_int);
    tmp___1 = BIO_s_mem();
    bmem = BIO_new(tmp___1);
    b64 = BIO_push(b64, bmem);
    BIO_write(bmem, input as *const libc::c_void, inlen);
    loop {
        tmp___2 = inlen;
        inlen += 1;
        if tmp___2 & 3 as libc::c_int == 0 {
            break;
        }
        BIO_write(
            bmem,
            b"=\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int,
        );
    }
    BIO_ctrl(bmem, 11 as libc::c_int, 0 as libc::c_long, 0 as *mut libc::c_void);
    tmp___3 = strlen(input as *const libc::c_char);
    bufsize = tmp___3
        .wrapping_mul(3 as libc::c_ulong)
        .wrapping_div(4 as libc::c_ulong)
        .wrapping_add(1 as libc::c_ulong) as libc::c_int;
    tmp___4 = malloc(bufsize as size_t);
    buf = tmp___4 as *mut uint8_t;
    nread = BIO_read(b64, buf as *mut libc::c_void, bufsize);
    BIO_free_all(bmem);
    *outlen = nread;
    return buf;
}
static mut super_secret_key: [libc::c_char; 1675] = [
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'm' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'L' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    'f' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    '+' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'Z' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'O' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    '3' as i32 as libc::c_char,
    'H' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'J' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'X' as i32 as libc::c_char,
    'q' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'z' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    '6' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'M' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    'y' as i32 as libc::c_char,
    'x' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    '2' as i32 as libc::c_char,
    'Q' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    'j' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    'G' as i32 as libc::c_char,
    'u' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'v' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '=' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'N' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'R' as i32 as libc::c_char,
    'I' as i32 as libc::c_char,
    'V' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'K' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'Y' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut rsa: *mut RSA = 0 as *const libc::c_void as *mut libc::c_void as *mut RSA;
pub unsafe extern "C" fn rsa_apply(
    mut input: *mut uint8_t,
    mut inlen: libc::c_int,
    mut outlen: *mut libc::c_int,
    mut mode: libc::c_int,
) -> *mut uint8_t {
    let mut bmem: *mut BIO = 0 as *mut BIO;
    let mut tmp: *mut BIO = 0 as *mut BIO;
    let mut out: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    if rsa.is_null() {
        tmp = BIO_new_mem_buf(
            super_secret_key.as_mut_ptr() as *const libc::c_void,
            -(1 as libc::c_int),
        );
        bmem = tmp;
        rsa = PEM_read_bio_RSAPrivateKey(
            bmem,
            0 as *mut libc::c_void as *mut *mut RSA,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<pem_password_cb>,
            >(0 as *mut libc::c_void),
            0 as *mut libc::c_void,
        );
        BIO_free(bmem);
    }
    tmp___0 = RSA_size(rsa as *const RSA);
    tmp___1 = malloc(tmp___0 as size_t);
    out = tmp___1 as *mut uint8_t;
    match mode {
        0 => {
            *outlen = RSA_private_encrypt(
                inlen,
                input as *const libc::c_uchar,
                out,
                rsa,
                1 as libc::c_int,
            );
        }
        1 => {
            *outlen = RSA_private_decrypt(
                inlen,
                input as *const libc::c_uchar,
                out,
                rsa,
                4 as libc::c_int,
            );
        }
        _ => {
            die(
                b"bad rsa mode\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    return out;
}
pub unsafe extern "C" fn command_start() {
    let mut tmp: __pid_t = 0;
    let mut tmp___0: libc::c_int = 0;
    if (config.cmd_start).is_null() {
        return;
    }
    if config.cmd_blocking == 0 {
        tmp = fork();
        if tmp != 0 {
            return;
        }
    }
    debug(
        1 as libc::c_int,
        b"running start command: %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        config.cmd_start,
    );
    tmp___0 = system(config.cmd_start as *const libc::c_char);
    if tmp___0 != 0 {
        warn(
            b"exec of external start command failed\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if config.cmd_blocking == 0 {
        exit(0 as libc::c_int);
    }
}
pub unsafe extern "C" fn command_stop() {
    let mut tmp: __pid_t = 0;
    let mut tmp___0: libc::c_int = 0;
    if (config.cmd_stop).is_null() {
        return;
    }
    if config.cmd_blocking == 0 {
        tmp = fork();
        if tmp != 0 {
            return;
        }
    }
    debug(
        1 as libc::c_int,
        b"running stop command: %s\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        config.cmd_stop,
    );
    tmp___0 = system(config.cmd_stop as *const libc::c_char);
    if tmp___0 != 0 {
        warn(
            b"exec of external stop command failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if config.cmd_blocking == 0 {
        exit(0 as libc::c_int);
    }
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 255 as libc::c_int
        | (__bsx as libc::c_int & 255 as libc::c_int) << 8 as libc::c_int) as __uint16_t;
}
#[inline]
unsafe extern "C" fn seq_diff(mut a: seq_t, mut b: seq_t) -> uint16_t {
    let mut diff: int16_t = 0;
    diff = (b as libc::c_int - a as libc::c_int) as int16_t;
    return diff as uint16_t;
}
static mut running: libc::c_int = 0 as libc::c_int;
static mut please_shutdown___0: libc::c_int = 0;
static mut rtp_client: sockaddr_storage = sockaddr_storage {
    ss_family: 0,
    __ss_padding: [0; 118],
    __ss_align: 0,
};
static mut sock: libc::c_int = 0;
static mut rtp_thread: pthread_t = 0;
unsafe extern "C" fn rtp_receiver(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut packet: [uint8_t; 2048] = [0; 2048];
    let mut pktp: *mut uint8_t = 0 as *mut uint8_t;
    let mut nread: ssize_t = 0;
    let mut plen: ssize_t = 0;
    let mut type_0: uint8_t = 0;
    let mut seqno: seq_t = 0;
    let mut tmp: __uint16_t = 0;
    while please_shutdown___0 == 0 {
        nread = recv(
            sock,
            packet.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[uint8_t; 2048]>() as libc::c_ulong,
            0 as libc::c_int,
        );
        if nread < 0 as libc::c_long {
            break;
        }
        plen = nread;
        type_0 = (packet[1 as libc::c_int as usize] as libc::c_int
            & -(129 as libc::c_int)) as uint8_t;
        if type_0 as libc::c_int == 84 as libc::c_int {
            continue;
        }
        if !(type_0 as libc::c_int == 96 as libc::c_int) {
            if !(type_0 as libc::c_int == 86 as libc::c_int) {
                warn(
                    b"Unknown RTP packet of type 0x%02X length %d\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    type_0 as libc::c_int,
                    nread,
                );
                continue;
            }
        }
        pktp = packet.as_mut_ptr();
        if type_0 as libc::c_int == 86 as libc::c_int {
            pktp = pktp.offset(4 as libc::c_int as isize);
            plen -= 4 as libc::c_long;
        }
        tmp = __bswap_16(
            *(pktp.offset(2 as libc::c_int as isize) as *mut libc::c_ushort),
        );
        seqno = tmp;
        pktp = pktp.offset(12 as libc::c_int as isize);
        plen -= 12 as libc::c_long;
        if plen >= 16 as libc::c_long {
            player_put_packet(seqno, pktp, plen as libc::c_int);
        } else {
            if type_0 as libc::c_int == 86 as libc::c_int {
                if seqno as libc::c_int == 0 as libc::c_int {
                    debug(
                        2 as libc::c_int,
                        b"resend-related request packet received, ignoring.\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                    continue;
                }
            }
            debug(
                1 as libc::c_int,
                b"Unknown RTP packet of type 0x%02X length %d seqno %d\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                type_0 as libc::c_int,
                nread,
                seqno as libc::c_int,
            );
        }
    }
    debug(
        1 as libc::c_int,
        b"RTP thread interrupted. terminating.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    close(sock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn bind_port(mut remote: *mut sockaddr_storage) -> libc::c_int {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut info: *mut addrinfo = 0 as *mut addrinfo;
    let mut ret: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut sport: libc::c_int = 0;
    let mut local: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut local_len: socklen_t = 0;
    let mut sa6: *mut sockaddr_in6 = 0 as *mut sockaddr_in6;
    let mut tmp___1: __uint16_t = 0;
    let mut sa: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut tmp___2: __uint16_t = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = (*remote).ss_family as libc::c_int;
    hints.ai_socktype = 2 as libc::c_int;
    hints.ai_flags = 1 as libc::c_int;
    tmp = getaddrinfo(
        0 as *mut libc::c_void as *const libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char,
        &mut hints as *mut addrinfo as *const addrinfo,
        &mut info as *mut *mut addrinfo,
    );
    ret = tmp;
    if ret < 0 as libc::c_int {
        tmp___0 = gai_strerror(ret);
        die(
            b"failed to get usable addrinfo?! %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            tmp___0,
        );
    }
    sock = socket(
        (*remote).ss_family as libc::c_int,
        2 as libc::c_int,
        17 as libc::c_int,
    );
    ret = bind(sock, (*info).ai_addr as *const sockaddr, (*info).ai_addrlen);
    freeaddrinfo(info);
    if ret < 0 as libc::c_int {
        die(
            b"could not bind a UDP port!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    local_len = ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as socklen_t;
    getsockname(
        sock,
        &mut local as *mut sockaddr_storage as *mut sockaddr,
        &mut local_len as *mut socklen_t,
    );
    if local.ss_family as libc::c_int == 10 as libc::c_int {
        sa6 = &mut local as *mut sockaddr_storage as *mut sockaddr_in6;
        tmp___1 = __bswap_16((*sa6).sin6_port);
        sport = tmp___1 as libc::c_int;
    } else {
        sa = &mut local as *mut sockaddr_storage as *mut sockaddr_in;
        tmp___2 = __bswap_16((*sa).sin_port);
        sport = tmp___2 as libc::c_int;
    }
    return sport;
}
pub unsafe extern "C" fn rtp_setup(
    mut remote: *mut sockaddr_storage,
    mut cport: libc::c_int,
    mut tport: libc::c_int,
) -> libc::c_int {
    let mut sa6: *mut sockaddr_in6 = 0 as *mut sockaddr_in6;
    let mut sa: *mut sockaddr_in = 0 as *mut sockaddr_in;
    let mut sport: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    if running != 0 {
        die(
            b"rtp_setup called with active stream!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    debug(
        1 as libc::c_int,
        b"rtp_setup: cport=%d tport=%d\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        cport,
        tport,
    );
    memcpy(
        &mut rtp_client as *mut sockaddr_storage as *mut libc::c_void,
        remote as *const libc::c_void,
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong,
    );
    if rtp_client.ss_family as libc::c_int == 10 as libc::c_int {
        sa6 = &mut rtp_client as *mut sockaddr_storage as *mut sockaddr_in6;
        (*sa6).sin6_port = __bswap_16(cport as __uint16_t);
    } else {
        sa = &mut rtp_client as *mut sockaddr_storage as *mut sockaddr_in;
        (*sa).sin_port = __bswap_16(cport as __uint16_t);
    }
    tmp = bind_port(remote);
    sport = tmp;
    debug(
        1 as libc::c_int,
        b"rtp listening on port %d\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        sport,
    );
    please_shutdown___0 = 0 as libc::c_int;
    pthread_create(
        &mut rtp_thread as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            rtp_receiver as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    running = 1 as libc::c_int;
    return sport;
}
pub unsafe extern "C" fn rtp_shutdown() {
    let mut retval: *mut libc::c_void = 0 as *mut libc::c_void;
    if running == 0 {
        die(
            b"rtp_shutdown called without active stream!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    debug(
        2 as libc::c_int,
        b"shutting down RTP thread\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    please_shutdown___0 = 1 as libc::c_int;
    pthread_kill(rtp_thread, 10 as libc::c_int);
    pthread_join(rtp_thread, &mut retval);
    running = 0 as libc::c_int;
}
pub unsafe extern "C" fn rtp_request_resend(mut first: seq_t, mut last: seq_t) {
    let mut tmp: uint16_t = 0;
    let mut req: [libc::c_char; 8] = [0; 8];
    if running == 0 {
        die(
            b"rtp_request_resend called without active stream!\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    tmp = seq_diff(first, last);
    debug(
        1 as libc::c_int,
        b"requesting resend on %d packets (%04X:%04X)\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        tmp as libc::c_int + 1 as libc::c_int,
        first as libc::c_int,
        last as libc::c_int,
    );
    req[0 as libc::c_int as usize] = -(128 as libc::c_int) as libc::c_char;
    req[1 as libc::c_int as usize] = -(43 as libc::c_int) as libc::c_char;
    *(req.as_mut_ptr().offset(2 as libc::c_int as isize)
        as *mut libc::c_ushort) = __bswap_16(1 as libc::c_int as __uint16_t);
    *(req.as_mut_ptr().offset(4 as libc::c_int as isize)
        as *mut libc::c_ushort) = __bswap_16(first);
    *(req.as_mut_ptr().offset(6 as libc::c_int as isize)
        as *mut libc::c_ushort) = __bswap_16(
        (last as libc::c_int - first as libc::c_int + 1 as libc::c_int) as __uint16_t,
    );
    sendto(
        sock,
        req.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
        0 as libc::c_int,
        &mut rtp_client as *mut sockaddr_storage as *mut sockaddr as *const sockaddr,
        ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as socklen_t,
    );
}
pub static mut player_meta: metadata = metadata {
    artist: 0 as *const libc::c_char as *mut libc::c_char,
    title: 0 as *const libc::c_char as *mut libc::c_char,
    album: 0 as *const libc::c_char as *mut libc::c_char,
    artwork: 0 as *const libc::c_char as *mut libc::c_char,
    comment: 0 as *const libc::c_char as *mut libc::c_char,
    genre: 0 as *const libc::c_char as *mut libc::c_char,
};
static mut fd: libc::c_int = -(1 as libc::c_int);
static mut dirty: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn metadata_set(
    mut field: *mut *mut libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut tmp: libc::c_int = 0;
    if !(*field).is_null() {
        tmp = strcmp(*field as *const libc::c_char, value);
        if tmp == 0 {
            return;
        }
        free(*field as *mut libc::c_void);
    }
    *field = strdup(value);
    dirty = 1 as libc::c_int;
}
pub unsafe extern "C" fn metadata_open() {
    let mut fn_0: [libc::c_char; 12] = [0; 12];
    let mut pl: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    if (config.meta_dir).is_null() {
        return;
    }
    fn_0[0 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    fn_0[1 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    fn_0[2 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
    fn_0[3 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    fn_0[4 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    fn_0[5 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    fn_0[6 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    fn_0[7 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    fn_0[8 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    fn_0[9 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    fn_0[10 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    fn_0[11 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    tmp = strlen(config.meta_dir as *const libc::c_char);
    tmp___0 = strlen(fn_0.as_mut_ptr() as *const libc::c_char);
    pl = tmp.wrapping_add(1 as libc::c_ulong).wrapping_add(tmp___0);
    tmp___1 = malloc(pl.wrapping_add(1 as libc::c_ulong));
    path = tmp___1 as *mut libc::c_char;
    snprintf(
        path,
        pl.wrapping_add(1 as libc::c_ulong),
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        config.meta_dir,
        fn_0.as_mut_ptr(),
    );
    tmp___2 = mkfifo(path as *const libc::c_char, 420 as libc::c_int as __mode_t);
    if tmp___2 != 0 {
        tmp___3 = __errno_location();
        if *tmp___3 != 17 as libc::c_int {
            die(
                b"Could not create metadata FIFO %s\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                path,
            );
        }
    }
    fd = open(path as *const libc::c_char, 2049 as libc::c_int);
    if fd < 0 as libc::c_int {
        debug(
            1 as libc::c_int,
            b"Could not open metadata FIFO %s. Will try again later.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            path,
        );
    }
    free(path as *mut libc::c_void);
}
unsafe extern "C" fn metadata_close() {
    close(fd);
    fd = -(1 as libc::c_int);
}
unsafe extern "C" fn print_one(
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut tmp: size_t = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: ssize_t = 0;
    let mut tmp___4: ssize_t = 0;
    tmp = strlen(name);
    tmp___0 = write(fd, name as *const libc::c_void, tmp);
    tmp___1 = write(
        fd,
        b"=\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    if !value.is_null() {
        tmp___2 = strlen(value);
        tmp___3 = write(fd, value as *const libc::c_void, tmp___2);
    }
    tmp___4 = write(
        fd,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
}
pub unsafe extern "C" fn metadata_write() {
    let mut ret: libc::c_int = 0;
    let mut tmp: ssize_t = 0;
    if fd < 0 as libc::c_int {
        metadata_open();
    }
    if fd < 0 as libc::c_int {
        return;
    }
    if dirty == 0 {
        return;
    }
    dirty = 0 as libc::c_int;
    print_one(
        b"artist\0" as *const u8 as *const libc::c_char,
        player_meta.artist as *const libc::c_char,
    );
    print_one(
        b"title\0" as *const u8 as *const libc::c_char,
        player_meta.title as *const libc::c_char,
    );
    print_one(
        b"album\0" as *const u8 as *const libc::c_char,
        player_meta.album as *const libc::c_char,
    );
    print_one(
        b"artwork\0" as *const u8 as *const libc::c_char,
        player_meta.artwork as *const libc::c_char,
    );
    print_one(
        b"genre\0" as *const u8 as *const libc::c_char,
        player_meta.genre as *const libc::c_char,
    );
    print_one(
        b"comment\0" as *const u8 as *const libc::c_char,
        player_meta.comment as *const libc::c_char,
    );
    tmp = write(
        fd,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    ret = tmp as libc::c_int;
    if ret < 1 as libc::c_int {
        metadata_close();
    }
}
pub unsafe extern "C" fn metadata_cover_image(
    mut buf: *const libc::c_char,
    mut len: libc::c_int,
    mut ext: *const libc::c_char,
) {
    let mut img_md5: [uint8_t; 16] = [0; 16];
    let mut ctx: MD5_CTX = MD5_CTX {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        Nl: 0,
        Nh: 0,
        data: [0; 16],
        num: 0,
    };
    let mut img_md5_str: [libc::c_char; 33] = [0; 33];
    let mut i: libc::c_int = 0;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut prefix: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pl: size_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cover_fd: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: ssize_t = 0;
    let mut tmp___6: size_t = 0;
    if (config.meta_dir).is_null() {
        return;
    }
    if !buf.is_null() {
        debug(
            1 as libc::c_int,
            b"Cover Art set\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        debug(
            1 as libc::c_int,
            b"Cover Art cleared\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    MD5_Init(&mut ctx);
    MD5_Update(&mut ctx, buf as *const libc::c_void, len as size_t);
    MD5_Final(img_md5.as_mut_ptr(), &mut ctx);
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        sprintf(
            &mut *img_md5_str.as_mut_ptr().offset((i * 2 as libc::c_int) as isize)
                as *mut libc::c_char,
            b"%02x\0" as *const u8 as *const libc::c_char,
            img_md5[i as usize] as libc::c_int,
        );
        i += 1;
    }
    dir = config.meta_dir;
    prefix = b"cover-\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    tmp = strlen(dir as *const libc::c_char);
    tmp___0 = strlen(prefix as *const libc::c_char);
    tmp___1 = strlen(img_md5_str.as_mut_ptr() as *const libc::c_char);
    tmp___2 = strlen(ext);
    pl = tmp
        .wrapping_add(1 as libc::c_ulong)
        .wrapping_add(tmp___0)
        .wrapping_add(tmp___1)
        .wrapping_add(1 as libc::c_ulong)
        .wrapping_add(tmp___2);
    tmp___3 = malloc(pl.wrapping_add(1 as libc::c_ulong));
    path = tmp___3 as *mut libc::c_char;
    snprintf(
        path,
        pl.wrapping_add(1 as libc::c_ulong),
        b"%s/%s%s.%s\0" as *const u8 as *const libc::c_char,
        dir,
        prefix,
        img_md5_str.as_mut_ptr(),
        ext,
    );
    tmp___4 = open(path as *const libc::c_char, 65 as libc::c_int, 384 as libc::c_int);
    cover_fd = tmp___4;
    if cover_fd < 0 as libc::c_int {
        warn(
            b"Could not open file %s for writing cover art\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            path,
        );
        return;
    }
    tmp___5 = write(cover_fd, buf as *const libc::c_void, len as size_t);
    if tmp___5 < len as ssize_t {
        warn(
            b"writing %s failed\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            path,
        );
        free(path as *mut libc::c_void);
        return;
    }
    close(cover_fd);
    debug(
        1 as libc::c_int,
        b"Cover Art file is %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        path,
    );
    tmp___6 = strlen(dir as *const libc::c_char);
    metadata_set(
        &mut player_meta.artwork,
        path.offset(tmp___6 as isize).offset(1 as libc::c_int as isize)
            as *const libc::c_char,
    );
    free(path as *mut libc::c_void);
}
static mut aesiv: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut aes: AES_KEY = AES_KEY {
    rd_key: [0; 60],
    rounds: 0,
};
static mut sampling_rate: libc::c_int = 0;
static mut frame_size: libc::c_int = 0;
static mut player_thread: pthread_t = 0;
static mut please_stop: libc::c_int = 0;
static mut decoder_info: *mut alac_file = 0 as *const alac_file as *mut alac_file;
static mut volume: libc::c_double = 1.0f64;
static mut fix_volume: libc::c_int = 65536 as libc::c_int;
static mut vol_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut audio_buffer: [abuf_t; 512] = [abuf_t {
    ready: 0,
    data: 0 as *const libc::c_short as *mut libc::c_short,
}; 512];
static mut ab_read: seq_t = 0;
static mut ab_write: seq_t = 0;
static mut ab_buffering: libc::c_int = 1 as libc::c_int;
static mut ab_synced: libc::c_int = 0 as libc::c_int;
static mut ab_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
unsafe extern "C" fn ab_resync() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        audio_buffer[i as usize].ready = 0 as libc::c_int;
        i += 1;
    }
    ab_synced = 0 as libc::c_int;
    ab_buffering = 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn seq_order(mut a: seq_t, mut b: seq_t) -> libc::c_int {
    let mut d: libc::c_short = 0;
    d = (b as libc::c_int - a as libc::c_int) as libc::c_short;
    return (d as libc::c_int > 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn alac_decode(
    mut dest: *mut libc::c_short,
    mut buf: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut packet: [libc::c_uchar; 2048] = [0; 2048];
    let mut iv: [libc::c_uchar; 16] = [0; 16];
    let mut aeslen: libc::c_int = 0;
    let mut outsize: libc::c_int = 0;
    if !(len <= 2048 as libc::c_int) {
        __assert_fail(
            b"len<=MAX_PACKET\0" as *const u8 as *const libc::c_char,
            b"player.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_uint,
            b"alac_decode\0" as *const u8 as *const libc::c_char,
        );
    }
    aeslen = len & -(16 as libc::c_int);
    memcpy(
        iv.as_mut_ptr() as *mut libc::c_void,
        aesiv as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 16]>() as libc::c_ulong,
    );
    AES_cbc_encrypt(
        buf as *const libc::c_uchar,
        packet.as_mut_ptr(),
        aeslen as size_t,
        &mut aes as *mut AES_KEY as *const AES_KEY,
        iv.as_mut_ptr(),
        0 as libc::c_int,
    );
    memcpy(
        packet.as_mut_ptr().offset(aeslen as isize) as *mut libc::c_void,
        buf.offset(aeslen as isize) as *const libc::c_void,
        (len - aeslen) as size_t,
    );
    alac_decode_frame(
        decoder_info,
        packet.as_mut_ptr(),
        dest as *mut libc::c_void,
        &mut outsize,
    );
    if !(outsize == 4 as libc::c_int * frame_size) {
        __assert_fail(
            b"outsize == FRAME_BYTES(frame_size)\0" as *const u8 as *const libc::c_char,
            b"player.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_uint,
            b"alac_decode\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn init_decoder(mut fmtp: *mut int32_t) -> libc::c_int {
    let mut alac: *mut alac_file = 0 as *mut alac_file;
    let mut sample_size: libc::c_int = 0;
    frame_size = *fmtp.offset(1 as libc::c_int as isize);
    sampling_rate = *fmtp.offset(11 as libc::c_int as isize);
    sample_size = *fmtp.offset(3 as libc::c_int as isize);
    if sample_size != 16 as libc::c_int {
        die(
            b"only 16-bit samples supported!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    alac = alac_create(sample_size, 2 as libc::c_int);
    if alac.is_null() {
        return 1 as libc::c_int;
    }
    decoder_info = alac;
    (*alac).setinfo_max_samples_per_frame = frame_size as uint32_t;
    (*alac).setinfo_7a = *fmtp.offset(2 as libc::c_int as isize) as uint8_t;
    (*alac).setinfo_sample_size = sample_size as uint8_t;
    (*alac)
        .setinfo_rice_historymult = *fmtp.offset(4 as libc::c_int as isize) as uint8_t;
    (*alac)
        .setinfo_rice_initialhistory = *fmtp.offset(5 as libc::c_int as isize)
        as uint8_t;
    (*alac).setinfo_rice_kmodifier = *fmtp.offset(6 as libc::c_int as isize) as uint8_t;
    (*alac).setinfo_7f = *fmtp.offset(7 as libc::c_int as isize) as uint8_t;
    (*alac).setinfo_80 = *fmtp.offset(8 as libc::c_int as isize) as uint16_t;
    (*alac).setinfo_82 = *fmtp.offset(9 as libc::c_int as isize) as uint32_t;
    (*alac).setinfo_86 = *fmtp.offset(10 as libc::c_int as isize) as uint32_t;
    (*alac).setinfo_8a_rate = *fmtp.offset(11 as libc::c_int as isize) as uint32_t;
    alac_allocate_buffers(alac);
    return 0 as libc::c_int;
}
unsafe extern "C" fn free_decoder() {
    alac_free(decoder_info);
}
unsafe extern "C" fn init_buffer() {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        tmp = malloc((4 as libc::c_int * (frame_size + 3 as libc::c_int)) as size_t);
        audio_buffer[i as usize].data = tmp as *mut libc::c_short;
        i += 1;
    }
    ab_resync();
}
unsafe extern "C" fn free_buffer() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 512 as libc::c_int {
        free(audio_buffer[i as usize].data as *mut libc::c_void);
        i += 1;
    }
}
pub unsafe extern "C" fn player_put_packet(
    mut seqno: seq_t,
    mut data: *mut uint8_t,
    mut len: libc::c_int,
) {
    let mut abuf: *mut abuf_t = 0 as *mut abuf_t;
    let mut buf_fill: int16_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: uint16_t = 0;
    let mut tmp___2: uint16_t = 0;
    abuf = 0 as *mut abuf_t;
    pthread_mutex_lock(&mut ab_mutex);
    if ab_synced == 0 {
        debug(
            2 as libc::c_int,
            b"syncing to first seqno %04X\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            seqno as libc::c_int,
        );
        ab_write = (seqno as libc::c_int - 1 as libc::c_int) as seq_t;
        ab_read = seqno;
        ab_synced = 1 as libc::c_int;
    }
    tmp___1 = seq_diff(ab_write, seqno);
    if tmp___1 as libc::c_int == 1 as libc::c_int {
        abuf = audio_buffer
            .as_mut_ptr()
            .offset((seqno as libc::c_int % 512 as libc::c_int) as isize);
        ab_write = seqno;
    } else {
        tmp___0 = seq_order(ab_write, seqno);
        if tmp___0 != 0 {
            rtp_request_resend(
                (ab_write as libc::c_int + 1 as libc::c_int) as seq_t,
                (seqno as libc::c_int - 1 as libc::c_int) as seq_t,
            );
            abuf = audio_buffer
                .as_mut_ptr()
                .offset((seqno as libc::c_int % 512 as libc::c_int) as isize);
            ab_write = seqno;
        } else {
            tmp = seq_order(ab_read, seqno);
            if tmp != 0 {
                abuf = audio_buffer
                    .as_mut_ptr()
                    .offset((seqno as libc::c_int % 512 as libc::c_int) as isize);
            } else {
                debug(
                    1 as libc::c_int,
                    b"late packet %04X (%04X:%04X)\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    seqno as libc::c_int,
                    ab_read as libc::c_int,
                    ab_write as libc::c_int,
                );
            }
        }
    }
    tmp___2 = seq_diff(ab_read, ab_write);
    buf_fill = tmp___2 as int16_t;
    pthread_mutex_unlock(&mut ab_mutex);
    if !abuf.is_null() {
        alac_decode((*abuf).data, data, len);
        (*abuf).ready = 1 as libc::c_int;
    }
    pthread_mutex_lock(&mut ab_mutex);
    if ab_buffering != 0 {
        if buf_fill as libc::c_int >= config.buffer_start_fill {
            debug(
                1 as libc::c_int,
                b"buffering over. starting play\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            ab_buffering = 0 as libc::c_int;
            bf_est_reset(buf_fill);
        }
    }
    pthread_mutex_unlock(&mut ab_mutex);
}
static mut lcg_prev: libc::c_ulong = 12345 as libc::c_ulong;
unsafe extern "C" fn lcg_rand() -> libc::c_short {
    lcg_prev = lcg_prev
        .wrapping_mul(69069 as libc::c_ulong)
        .wrapping_add(3 as libc::c_ulong);
    return (lcg_prev & 65535 as libc::c_ulong) as libc::c_short;
}
static mut rand_a: libc::c_short = 0;
static mut rand_b: libc::c_short = 0;
#[inline]
unsafe extern "C" fn dithered_vol(mut sample: libc::c_short) -> libc::c_short {
    let mut out: libc::c_long = 0;
    out = sample as libc::c_long * fix_volume as libc::c_long;
    if fix_volume < 65536 as libc::c_int {
        rand_b = rand_a;
        rand_a = lcg_rand();
        out += rand_a as libc::c_long;
        out -= rand_b as libc::c_long;
    }
    return (out >> 16 as libc::c_int) as libc::c_short;
}
unsafe extern "C" fn biquad_init(
    mut bq: *mut biquad_t,
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
) {
    let mut tmp: libc::c_double = 0.;
    tmp = 0.0f64;
    (*bq).hist[1 as libc::c_int as usize] = tmp;
    (*bq).hist[0 as libc::c_int as usize] = tmp;
    memcpy(
        ((*bq).a).as_mut_ptr() as *mut libc::c_void,
        a as *const libc::c_void,
        (2 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    memcpy(
        ((*bq).b).as_mut_ptr() as *mut libc::c_void,
        b as *const libc::c_void,
        (3 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
}
unsafe extern "C" fn biquad_lpf(
    mut bq: *mut biquad_t,
    mut freq: libc::c_double,
    mut Q: libc::c_double,
) {
    let mut w0: libc::c_double = 0.;
    let mut alpha: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut a_0: libc::c_double = 0.;
    let mut b: [libc::c_double; 3] = [0.; 3];
    let mut a: [libc::c_double; 2] = [0.; 2];
    let mut tmp___0: libc::c_double = 0.;
    let mut tmp___1: libc::c_double = 0.;
    let mut tmp___2: libc::c_double = 0.;
    w0 = 2.0f64 * 3.14159265358979323846f64 * freq * frame_size as libc::c_double
        / sampling_rate as libc::c_double;
    tmp = sin(w0);
    alpha = tmp / (2.0f64 * Q);
    a_0 = 1.0f64 + alpha;
    tmp___0 = cos(w0);
    b[0 as libc::c_int as usize] = (1.0f64 - tmp___0) / (2.0f64 * a_0);
    tmp___1 = cos(w0);
    b[1 as libc::c_int as usize] = (1.0f64 - tmp___1) / a_0;
    b[2 as libc::c_int as usize] = b[0 as libc::c_int as usize];
    tmp___2 = cos(w0);
    a[0 as libc::c_int as usize] = -2.0f64 * tmp___2 / a_0;
    a[1 as libc::c_int as usize] = (1 as libc::c_int as libc::c_double - alpha) / a_0;
    biquad_init(bq, a.as_mut_ptr(), b.as_mut_ptr());
}
unsafe extern "C" fn biquad_filt(
    mut bq: *mut biquad_t,
    mut in_0: libc::c_double,
) -> libc::c_double {
    let mut w: libc::c_double = 0.;
    let mut out: libc::c_double = 0.;
    w = in_0 - (*bq).a[0 as libc::c_int as usize] * (*bq).hist[0 as libc::c_int as usize]
        - (*bq).a[1 as libc::c_int as usize] * (*bq).hist[1 as libc::c_int as usize];
    out = (*bq).b[1 as libc::c_int as usize] * (*bq).hist[0 as libc::c_int as usize]
        + (*bq).b[2 as libc::c_int as usize] * (*bq).hist[1 as libc::c_int as usize]
        + (*bq).b[0 as libc::c_int as usize] * w;
    (*bq).hist[1 as libc::c_int as usize] = (*bq).hist[0 as libc::c_int as usize];
    (*bq).hist[0 as libc::c_int as usize] = w;
    return out;
}
static mut bf_playback_rate: libc::c_double = 1.0f64;
static mut bf_est_drift: libc::c_double = 0.0f64;
static mut bf_drift_lpf: biquad_t = biquad_t {
    hist: [0.; 2],
    a: [0.; 2],
    b: [0.; 3],
};
static mut bf_est_err: libc::c_double = 0.0f64;
static mut bf_last_err: libc::c_double = 0.;
static mut bf_err_lpf: biquad_t = biquad_t {
    hist: [0.; 2],
    a: [0.; 2],
    b: [0.; 3],
};
static mut bf_err_deriv_lpf: biquad_t = biquad_t {
    hist: [0.; 2],
    a: [0.; 2],
    b: [0.; 3],
};
static mut desired_fill: libc::c_double = 0.;
static mut fill_count: libc::c_int = 0;
unsafe extern "C" fn bf_est_reset(mut fill: libc::c_short) {
    biquad_lpf(&mut bf_drift_lpf, 1.0f64 / 180.0f64, 0.3f64);
    biquad_lpf(&mut bf_err_lpf, 1.0f64 / 10.0f64, 0.25f64);
    biquad_lpf(&mut bf_err_deriv_lpf, 1.0f64 / 2.0f64, 0.2f64);
    fill_count = 0 as libc::c_int;
    bf_playback_rate = 1.0f64;
    bf_last_err = 0 as libc::c_int as libc::c_double;
    bf_est_err = bf_last_err;
    fill_count = 0 as libc::c_int;
    desired_fill = fill_count as libc::c_double;
}
unsafe extern "C" fn bf_est_update(mut fill: libc::c_short) {
    let mut buf_delta: libc::c_double = 0.;
    let mut err_deriv: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut adj_error: libc::c_double = 0.;
    if fill_count < 1000 as libc::c_int {
        desired_fill += fill as libc::c_double / 1000.0f64;
        fill_count += 1;
        return;
    } else {
        if fill_count == 1000 as libc::c_int {
            debug(
                1 as libc::c_int,
                b"established desired fill of %f frames, so output chain buffered about %f frames\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                desired_fill,
                config.buffer_start_fill as libc::c_double - desired_fill,
            );
            fill_count += 1;
        }
    }
    buf_delta = fill as libc::c_double - desired_fill;
    bf_est_err = biquad_filt(&mut bf_err_lpf, buf_delta);
    tmp = biquad_filt(&mut bf_err_deriv_lpf, bf_est_err - bf_last_err);
    err_deriv = tmp;
    adj_error = 1e-4f64 * bf_est_err;
    bf_est_drift = biquad_filt(
        &mut bf_drift_lpf,
        1e-1f64 * (adj_error + err_deriv) + bf_est_drift,
    );
    debug(
        3 as libc::c_int,
        b"bf %d err %f drift %f desiring %f ed %f estd %f\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        fill as libc::c_int,
        bf_est_err,
        bf_est_drift,
        desired_fill,
        err_deriv,
        err_deriv + adj_error,
    );
    bf_playback_rate = 1.0f64 + adj_error + bf_est_drift;
    bf_last_err = bf_est_err;
}
unsafe extern "C" fn buffer_get_frame() -> *mut libc::c_short {
    let mut buf_fill: int16_t = 0;
    let mut read___0: seq_t = 0;
    let mut next: seq_t = 0;
    let mut abuf: *mut abuf_t = 0 as *mut abuf_t;
    let mut i: libc::c_int = 0;
    let mut tmp: uint16_t = 0;
    let mut tmp___0: uint16_t = 0;
    let mut curframe: *mut abuf_t = 0 as *mut abuf_t;
    abuf = 0 as *mut abuf_t;
    if ab_buffering != 0 {
        return 0 as *mut libc::c_short;
    }
    pthread_mutex_lock(&mut ab_mutex);
    tmp = seq_diff(ab_read, ab_write);
    buf_fill = tmp as int16_t;
    's_75: {
        if !((buf_fill as libc::c_int) < 1 as libc::c_int) {
            if !(ab_synced == 0) {
                break 's_75;
            }
        }
        if (buf_fill as libc::c_int) < 1 as libc::c_int {
            warn(
                b"underrun.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        ab_buffering = 1 as libc::c_int;
        pthread_mutex_unlock(&mut ab_mutex);
        return 0 as *mut libc::c_short;
    }
    if buf_fill as libc::c_int >= 512 as libc::c_int {
        warn(b"overrun.\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        ab_read = (ab_write as libc::c_int - config.buffer_start_fill) as seq_t;
    }
    read___0 = ab_read;
    ab_read = (ab_read as libc::c_int + 1 as libc::c_int) as seq_t;
    tmp___0 = seq_diff(ab_read, ab_write);
    buf_fill = tmp___0 as int16_t;
    bf_est_update(buf_fill);
    if ab_buffering == 0 {
        i = 16 as libc::c_int;
        while i < config.buffer_start_fill / 2 as libc::c_int {
            next = (ab_read as libc::c_int + i) as seq_t;
            abuf = audio_buffer
                .as_mut_ptr()
                .offset((next as libc::c_int % 512 as libc::c_int) as isize);
            if (*abuf).ready == 0 {
                rtp_request_resend(next, next);
            }
            i *= 2 as libc::c_int;
        }
    }
    curframe = audio_buffer
        .as_mut_ptr()
        .offset((read___0 as libc::c_int % 512 as libc::c_int) as isize);
    if (*curframe).ready == 0 {
        debug(
            1 as libc::c_int,
            b"missing frame %04X.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            read___0 as libc::c_int,
        );
        memset(
            (*curframe).data as *mut libc::c_void,
            0 as libc::c_int,
            (4 as libc::c_int * frame_size) as size_t,
        );
    }
    (*curframe).ready = 0 as libc::c_int;
    pthread_mutex_unlock(&mut ab_mutex);
    return (*curframe).data;
}
unsafe extern "C" fn stuff_buffer(
    mut playback_rate: libc::c_double,
    mut inptr: *mut libc::c_short,
    mut outptr: *mut libc::c_short,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut stuffsamp: libc::c_int = 0;
    let mut stuff: libc::c_int = 0;
    let mut p_stuff: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_double = 0.;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut tmp___4: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut tmp___5: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut tmp___6: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut tmp___7: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut tmp___8: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut tmp___9: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut tmp___10: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut tmp___11: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut tmp___12: *mut libc::c_short = 0 as *mut libc::c_short;
    stuffsamp = frame_size;
    stuff = 0 as libc::c_int;
    tmp = fabs(playback_rate - 1.0f64);
    tmp___0 = pow(1.0f64 - tmp, frame_size as libc::c_double);
    p_stuff = 1.0f64 - tmp___0;
    tmp___2 = rand();
    if (tmp___2 as libc::c_double)
        < p_stuff * 2147483647 as libc::c_int as libc::c_double
    {
        if playback_rate > 1.0f64 {
            stuff = -(1 as libc::c_int);
        } else {
            stuff = 1 as libc::c_int;
        }
        tmp___1 = rand();
        stuffsamp = tmp___1 % (frame_size - 1 as libc::c_int);
    }
    pthread_mutex_lock(&mut vol_mutex);
    i = 0 as libc::c_int;
    while i < stuffsamp {
        tmp___3 = outptr;
        outptr = outptr.offset(1);
        tmp___4 = inptr;
        inptr = inptr.offset(1);
        *tmp___3 = dithered_vol(*tmp___4);
        tmp___5 = outptr;
        outptr = outptr.offset(1);
        tmp___6 = inptr;
        inptr = inptr.offset(1);
        *tmp___5 = dithered_vol(*tmp___6);
        i += 1;
    }
    if stuff != 0 {
        if stuff == 1 as libc::c_int {
            debug(
                2 as libc::c_int,
                b"+++++++++\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            tmp___7 = outptr;
            outptr = outptr.offset(1);
            *tmp___7 = dithered_vol(
                (*inptr.offset(-(2 as libc::c_int) as isize) as libc::c_long
                    + *inptr.offset(0 as libc::c_int as isize) as libc::c_long
                    >> 1 as libc::c_int) as libc::c_short,
            );
            tmp___8 = outptr;
            outptr = outptr.offset(1);
            *tmp___8 = dithered_vol(
                (*inptr.offset(-(1 as libc::c_int) as isize) as libc::c_long
                    + *inptr.offset(1 as libc::c_int as isize) as libc::c_long
                    >> 1 as libc::c_int) as libc::c_short,
            );
        } else if stuff == -(1 as libc::c_int) {
            debug(
                2 as libc::c_int,
                b"---------\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            inptr = inptr.offset(1);
            inptr = inptr.offset(1);
        }
        i = stuffsamp;
        while i < frame_size + stuff {
            tmp___9 = outptr;
            outptr = outptr.offset(1);
            tmp___10 = inptr;
            inptr = inptr.offset(1);
            *tmp___9 = dithered_vol(*tmp___10);
            tmp___11 = outptr;
            outptr = outptr.offset(1);
            tmp___12 = inptr;
            inptr = inptr.offset(1);
            *tmp___11 = dithered_vol(*tmp___12);
            i += 1;
        }
    }
    pthread_mutex_unlock(&mut vol_mutex);
    return frame_size + stuff;
}
unsafe extern "C" fn player_thread_func(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut play_samples: libc::c_int = 0;
    let mut inbuf: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut outbuf: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut silence: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc((4 as libc::c_int * (frame_size + 3 as libc::c_int)) as size_t);
    outbuf = tmp as *mut libc::c_short;
    tmp___0 = malloc((4 as libc::c_int * (frame_size + 3 as libc::c_int)) as size_t);
    silence = tmp___0 as *mut libc::c_short;
    memset(
        silence as *mut libc::c_void,
        0 as libc::c_int,
        (4 as libc::c_int * (frame_size + 3 as libc::c_int)) as size_t,
    );
    while please_stop == 0 {
        inbuf = buffer_get_frame();
        if inbuf.is_null() {
            inbuf = silence;
        }
        play_samples = stuff_buffer(bf_playback_rate, inbuf, outbuf);
        (Some(((*config.output).play).expect("non-null function pointer")))
            .expect("non-null function pointer")(outbuf, play_samples);
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn player_volume(mut f: libc::c_double) {
    let mut linear_volume: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    tmp = pow(10.0f64, 0.05f64 * f);
    linear_volume = tmp;
    if ((*config.output).volume).is_some() {
        (Some(((*config.output).volume).expect("non-null function pointer")))
            .expect("non-null function pointer")(linear_volume);
    } else {
        pthread_mutex_lock(&mut vol_mutex);
        volume = linear_volume;
        fix_volume = (65536.0f64 * volume) as libc::c_int;
        pthread_mutex_unlock(&mut vol_mutex);
    };
}
pub unsafe extern "C" fn player_flush() {
    pthread_mutex_lock(&mut ab_mutex);
    ab_resync();
    pthread_mutex_unlock(&mut ab_mutex);
}
pub unsafe extern "C" fn player_play(mut stream: *mut stream_cfg) -> libc::c_int {
    if config.buffer_start_fill > 512 as libc::c_int {
        die(
            b"specified buffer starting fill %d > buffer size %d\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            config.buffer_start_fill,
            512 as libc::c_int,
        );
    }
    AES_set_decrypt_key(
        ((*stream).aeskey).as_mut_ptr() as *const libc::c_uchar,
        128 as libc::c_int,
        &mut aes,
    );
    aesiv = ((*stream).aesiv).as_mut_ptr();
    init_decoder(((*stream).fmtp).as_mut_ptr());
    init_buffer();
    please_stop = 0 as libc::c_int;
    command_start();
    (Some(((*config.output).start).expect("non-null function pointer")))
        .expect("non-null function pointer")(sampling_rate);
    pthread_create(
        &mut player_thread as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            player_thread_func
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        0 as *mut libc::c_void,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn player_stop() {
    please_stop = 1 as libc::c_int;
    pthread_join(player_thread, 0 as *mut libc::c_void as *mut *mut libc::c_void);
    (Some(((*config.output).stop).expect("non-null function pointer")))
        .expect("non-null function pointer")();
    command_stop();
    free_buffer();
    free_decoder();
}
static mut host_bigendian: libc::c_int = 0 as libc::c_int;
pub static mut se_struct_24: __anonstruct_se_struct_24_422959015 = __anonstruct_se_struct_24_422959015 {
    x: [0; 3],
    c2rust_padding: [0; 1],
};
pub unsafe extern "C" fn alac_free(mut alac: *mut alac_file) {
    if !((*alac).predicterror_buffer_a).is_null() {
        free((*alac).predicterror_buffer_a as *mut libc::c_void);
    }
    if !((*alac).predicterror_buffer_b).is_null() {
        free((*alac).predicterror_buffer_b as *mut libc::c_void);
    }
    if !((*alac).outputsamples_buffer_a).is_null() {
        free((*alac).outputsamples_buffer_a as *mut libc::c_void);
    }
    if !((*alac).outputsamples_buffer_b).is_null() {
        free((*alac).outputsamples_buffer_b as *mut libc::c_void);
    }
    if !((*alac).uncompressed_bytes_buffer_a).is_null() {
        free((*alac).uncompressed_bytes_buffer_a as *mut libc::c_void);
    }
    if !((*alac).uncompressed_bytes_buffer_b).is_null() {
        free((*alac).uncompressed_bytes_buffer_b as *mut libc::c_void);
    }
    free(alac as *mut libc::c_void);
}
pub unsafe extern "C" fn alac_allocate_buffers(mut alac: *mut alac_file) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(
        ((*alac).setinfo_max_samples_per_frame).wrapping_mul(4 as libc::c_uint) as size_t,
    );
    (*alac).predicterror_buffer_a = tmp as *mut int32_t;
    tmp___0 = malloc(
        ((*alac).setinfo_max_samples_per_frame).wrapping_mul(4 as libc::c_uint) as size_t,
    );
    (*alac).predicterror_buffer_b = tmp___0 as *mut int32_t;
    tmp___1 = malloc(
        ((*alac).setinfo_max_samples_per_frame).wrapping_mul(4 as libc::c_uint) as size_t,
    );
    (*alac).outputsamples_buffer_a = tmp___1 as *mut int32_t;
    tmp___2 = malloc(
        ((*alac).setinfo_max_samples_per_frame).wrapping_mul(4 as libc::c_uint) as size_t,
    );
    (*alac).outputsamples_buffer_b = tmp___2 as *mut int32_t;
    tmp___3 = malloc(
        ((*alac).setinfo_max_samples_per_frame).wrapping_mul(4 as libc::c_uint) as size_t,
    );
    (*alac).uncompressed_bytes_buffer_a = tmp___3 as *mut int32_t;
    tmp___4 = malloc(
        ((*alac).setinfo_max_samples_per_frame).wrapping_mul(4 as libc::c_uint) as size_t,
    );
    (*alac).uncompressed_bytes_buffer_b = tmp___4 as *mut int32_t;
}
pub unsafe extern "C" fn alac_set_info(
    mut alac: *mut alac_file,
    mut inputbuffer: *mut libc::c_char,
) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ptr = inputbuffer;
    ptr = ptr.offset(4 as libc::c_int as isize);
    ptr = ptr.offset(4 as libc::c_int as isize);
    ptr = ptr.offset(4 as libc::c_int as isize);
    ptr = ptr.offset(4 as libc::c_int as isize);
    ptr = ptr.offset(4 as libc::c_int as isize);
    ptr = ptr.offset(4 as libc::c_int as isize);
    (*alac).setinfo_max_samples_per_frame = *(ptr as *mut uint32_t);
    if host_bigendian == 0 {
        (*alac)
            .setinfo_max_samples_per_frame = ((*alac).setinfo_max_samples_per_frame
            & 255 as libc::c_uint) << 24 as libc::c_int
            | ((*alac).setinfo_max_samples_per_frame & 65280 as libc::c_uint)
                << 8 as libc::c_int
            | ((*alac).setinfo_max_samples_per_frame & 16711680 as libc::c_uint)
                >> 8 as libc::c_int
            | ((*alac).setinfo_max_samples_per_frame & 4278190080 as libc::c_uint)
                >> 24 as libc::c_int;
    }
    ptr = ptr.offset(4 as libc::c_int as isize);
    (*alac).setinfo_7a = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1);
    (*alac).setinfo_sample_size = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1);
    (*alac).setinfo_rice_historymult = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1);
    (*alac).setinfo_rice_initialhistory = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1);
    (*alac).setinfo_rice_kmodifier = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1);
    (*alac).setinfo_7f = *(ptr as *mut uint8_t);
    ptr = ptr.offset(1);
    (*alac).setinfo_80 = *(ptr as *mut uint16_t);
    if host_bigendian == 0 {
        (*alac)
            .setinfo_80 = (((*alac).setinfo_80 as libc::c_int & 255 as libc::c_int)
            << 8 as libc::c_int
            | ((*alac).setinfo_80 as libc::c_int & 65280 as libc::c_int)
                >> 8 as libc::c_int) as uint16_t;
    }
    ptr = ptr.offset(2 as libc::c_int as isize);
    (*alac).setinfo_82 = *(ptr as *mut uint32_t);
    if host_bigendian == 0 {
        (*alac)
            .setinfo_82 = ((*alac).setinfo_82 & 255 as libc::c_uint) << 24 as libc::c_int
            | ((*alac).setinfo_82 & 65280 as libc::c_uint) << 8 as libc::c_int
            | ((*alac).setinfo_82 & 16711680 as libc::c_uint) >> 8 as libc::c_int
            | ((*alac).setinfo_82 & 4278190080 as libc::c_uint) >> 24 as libc::c_int;
    }
    ptr = ptr.offset(4 as libc::c_int as isize);
    (*alac).setinfo_86 = *(ptr as *mut uint32_t);
    if host_bigendian == 0 {
        (*alac)
            .setinfo_86 = ((*alac).setinfo_86 & 255 as libc::c_uint) << 24 as libc::c_int
            | ((*alac).setinfo_86 & 65280 as libc::c_uint) << 8 as libc::c_int
            | ((*alac).setinfo_86 & 16711680 as libc::c_uint) >> 8 as libc::c_int
            | ((*alac).setinfo_86 & 4278190080 as libc::c_uint) >> 24 as libc::c_int;
    }
    ptr = ptr.offset(4 as libc::c_int as isize);
    (*alac).setinfo_8a_rate = *(ptr as *mut uint32_t);
    if host_bigendian == 0 {
        (*alac)
            .setinfo_8a_rate = ((*alac).setinfo_8a_rate & 255 as libc::c_uint)
            << 24 as libc::c_int
            | ((*alac).setinfo_8a_rate & 65280 as libc::c_uint) << 8 as libc::c_int
            | ((*alac).setinfo_8a_rate & 16711680 as libc::c_uint) >> 8 as libc::c_int
            | ((*alac).setinfo_8a_rate & 4278190080 as libc::c_uint)
                >> 24 as libc::c_int;
    }
    alac_allocate_buffers(alac);
}
unsafe extern "C" fn readbits_16(
    mut alac: *mut alac_file,
    mut bits: libc::c_int,
) -> uint32_t {
    let mut result: uint32_t = 0;
    let mut new_accumulator: libc::c_int = 0;
    result = ((*((*alac).input_buffer).offset(0 as libc::c_int as isize) as libc::c_int)
        << 16 as libc::c_int
        | (*((*alac).input_buffer).offset(1 as libc::c_int as isize) as libc::c_int)
            << 8 as libc::c_int
        | *((*alac).input_buffer).offset(2 as libc::c_int as isize) as libc::c_int)
        as uint32_t;
    result <<= (*alac).input_buffer_bitaccumulator;
    result &= 16777215 as libc::c_uint;
    result >>= 24 as libc::c_int - bits;
    new_accumulator = (*alac).input_buffer_bitaccumulator + bits;
    (*alac)
        .input_buffer = ((*alac).input_buffer)
        .offset((new_accumulator >> 3 as libc::c_int) as isize);
    (*alac).input_buffer_bitaccumulator = new_accumulator & 7 as libc::c_int;
    return result;
}
unsafe extern "C" fn readbits(
    mut alac: *mut alac_file,
    mut bits: libc::c_int,
) -> uint32_t {
    let mut result: int32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut tmp___0: uint32_t = 0;
    result = 0 as libc::c_int;
    if bits > 16 as libc::c_int {
        bits -= 16 as libc::c_int;
        tmp = readbits_16(alac, 16 as libc::c_int);
        result = (tmp << bits) as int32_t;
    }
    tmp___0 = readbits_16(alac, bits);
    result = (result as libc::c_uint | tmp___0) as int32_t;
    return result as uint32_t;
}
unsafe extern "C" fn readbit(mut alac: *mut alac_file) -> libc::c_int {
    let mut result: libc::c_int = 0;
    let mut new_accumulator: libc::c_int = 0;
    result = *((*alac).input_buffer).offset(0 as libc::c_int as isize) as libc::c_int;
    result <<= (*alac).input_buffer_bitaccumulator;
    result = result >> 7 as libc::c_int & 1 as libc::c_int;
    new_accumulator = (*alac).input_buffer_bitaccumulator + 1 as libc::c_int;
    (*alac)
        .input_buffer = ((*alac).input_buffer)
        .offset((new_accumulator / 8 as libc::c_int) as isize);
    (*alac).input_buffer_bitaccumulator = new_accumulator % 8 as libc::c_int;
    return result;
}
unsafe extern "C" fn unreadbits(mut alac: *mut alac_file, mut bits: libc::c_int) {
    let mut new_accumulator: libc::c_int = 0;
    new_accumulator = (*alac).input_buffer_bitaccumulator - bits;
    (*alac)
        .input_buffer = ((*alac).input_buffer)
        .offset((new_accumulator >> 3 as libc::c_int) as isize);
    (*alac).input_buffer_bitaccumulator = new_accumulator & 7 as libc::c_int;
    if (*alac).input_buffer_bitaccumulator < 0 as libc::c_int {
        (*alac).input_buffer_bitaccumulator *= -(1 as libc::c_int);
    }
}
unsafe extern "C" fn count_leading_zeros(mut input: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = (input as libc::c_uint).leading_zeros() as i32;
    return tmp;
}
unsafe extern "C" fn entropy_decode_value(
    mut alac: *mut alac_file,
    mut readSampleSize: libc::c_int,
    mut k: libc::c_int,
    mut rice_kmodifier_mask: libc::c_int,
) -> int32_t {
    let mut x: int32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut value: int32_t = 0;
    let mut tmp___0: uint32_t = 0;
    let mut extraBits: libc::c_int = 0;
    let mut tmp___1: uint32_t = 0;
    x = 0 as libc::c_int;
    while x <= 8 as libc::c_int {
        tmp = readbit(alac);
        if tmp == 0 {
            break;
        }
        x += 1;
    }
    if x > 8 as libc::c_int {
        tmp___0 = readbits(alac, readSampleSize);
        value = tmp___0 as int32_t;
        value = (value as libc::c_uint
            & 4294967295 as libc::c_uint >> 32 as libc::c_int - readSampleSize)
            as int32_t;
        x = value;
    } else if k != 1 as libc::c_int {
        tmp___1 = readbits(alac, k);
        extraBits = tmp___1 as libc::c_int;
        x *= ((1 as libc::c_int) << k) - 1 as libc::c_int & rice_kmodifier_mask;
        if extraBits > 1 as libc::c_int {
            x += extraBits - 1 as libc::c_int;
        } else {
            unreadbits(alac, 1 as libc::c_int);
        }
    }
    return x;
}
unsafe extern "C" fn entropy_rice_decode(
    mut alac: *mut alac_file,
    mut outputBuffer: *mut int32_t,
    mut outputSize: libc::c_int,
    mut readSampleSize: libc::c_int,
    mut rice_initialhistory: libc::c_int,
    mut rice_kmodifier: libc::c_int,
    mut rice_historymult: libc::c_int,
    mut rice_kmodifier_mask: libc::c_int,
) {
    let mut outputCount: libc::c_int = 0;
    let mut history: libc::c_int = 0;
    let mut signModifier: libc::c_int = 0;
    let mut decodedValue: int32_t = 0;
    let mut finalValue: int32_t = 0;
    let mut k: int32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut blockSize: int32_t = 0;
    let mut tmp___0: libc::c_int = 0;
    history = rice_initialhistory;
    signModifier = 0 as libc::c_int;
    outputCount = 0 as libc::c_int;
    while outputCount < outputSize {
        tmp = count_leading_zeros((history >> 9 as libc::c_int) + 3 as libc::c_int);
        k = 31 as libc::c_int - rice_kmodifier - tmp;
        if k < 0 as libc::c_int {
            k += rice_kmodifier;
        } else {
            k = rice_kmodifier;
        }
        decodedValue = entropy_decode_value(
            alac,
            readSampleSize,
            k,
            -(1 as libc::c_int),
        );
        decodedValue += signModifier;
        finalValue = (decodedValue + 1 as libc::c_int) / 2 as libc::c_int;
        if decodedValue & 1 as libc::c_int != 0 {
            finalValue *= -(1 as libc::c_int);
        }
        *outputBuffer.offset(outputCount as isize) = finalValue;
        signModifier = 0 as libc::c_int;
        history
            += decodedValue * rice_historymult
                - (history * rice_historymult >> 9 as libc::c_int);
        if decodedValue > 65535 as libc::c_int {
            history = 65535 as libc::c_int;
        }
        if history < 128 as libc::c_int {
            if (outputCount + 1 as libc::c_int) < outputSize {
                signModifier = 1 as libc::c_int;
                tmp___0 = count_leading_zeros(history);
                k = tmp___0 + (history + 16 as libc::c_int) / 64 as libc::c_int
                    - 24 as libc::c_int;
                blockSize = entropy_decode_value(
                    alac,
                    16 as libc::c_int,
                    k,
                    rice_kmodifier_mask,
                );
                if blockSize > 0 as libc::c_int {
                    memset(
                        outputBuffer.offset((outputCount + 1 as libc::c_int) as isize)
                            as *mut libc::c_void,
                        0 as libc::c_int,
                        (blockSize as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<int32_t>() as libc::c_ulong,
                            ),
                    );
                    outputCount += blockSize;
                }
                if blockSize > 65535 as libc::c_int {
                    signModifier = 0 as libc::c_int;
                }
                history = 0 as libc::c_int;
            }
        }
        outputCount += 1;
    }
}
unsafe extern "C" fn predictor_decompress_fir_adapt(
    mut error_buffer: *mut int32_t,
    mut buffer_out: *mut int32_t,
    mut output_size: libc::c_int,
    mut readsamplesize: libc::c_int,
    mut predictor_coef_table: *mut int16_t,
    mut predictor_coef_num: libc::c_int,
    mut predictor_quantitization: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut prev_value: int32_t = 0;
    let mut error_value: int32_t = 0;
    let mut i___0: libc::c_int = 0;
    let mut val: int32_t = 0;
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    let mut outval: libc::c_int = 0;
    let mut error_val: libc::c_int = 0;
    let mut predictor_num: libc::c_int = 0;
    let mut val___0: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut predictor_num___0: libc::c_int = 0;
    let mut val___1: libc::c_int = 0;
    let mut sign___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    *buffer_out = *error_buffer;
    if predictor_coef_num == 0 {
        if output_size <= 1 as libc::c_int {
            return;
        }
        memcpy(
            buffer_out.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            error_buffer.offset(1 as libc::c_int as isize) as *const libc::c_void,
            ((output_size - 1 as libc::c_int) * 4 as libc::c_int) as size_t,
        );
        return;
    }
    if predictor_coef_num == 31 as libc::c_int {
        if output_size <= 1 as libc::c_int {
            return;
        }
        i = 0 as libc::c_int;
        while i < output_size - 1 as libc::c_int {
            prev_value = *buffer_out.offset(i as isize);
            error_value = *error_buffer.offset((i + 1 as libc::c_int) as isize);
            *buffer_out
                .offset(
                    (i + 1 as libc::c_int) as isize,
                ) = prev_value + error_value << 32 as libc::c_int - readsamplesize
                >> 32 as libc::c_int - readsamplesize;
            i += 1;
        }
        return;
    }
    if predictor_coef_num > 0 as libc::c_int {
        i___0 = 0 as libc::c_int;
        while i___0 < predictor_coef_num {
            val = *buffer_out.offset(i___0 as isize)
                + *error_buffer.offset((i___0 + 1 as libc::c_int) as isize);
            val = val << 32 as libc::c_int - readsamplesize
                >> 32 as libc::c_int - readsamplesize;
            *buffer_out.offset((i___0 + 1 as libc::c_int) as isize) = val;
            i___0 += 1;
        }
    }
    if predictor_coef_num > 0 as libc::c_int {
        i = predictor_coef_num + 1 as libc::c_int;
        while i < output_size {
            sum = 0 as libc::c_int;
            error_val = *error_buffer.offset(i as isize);
            j = 0 as libc::c_int;
            while j < predictor_coef_num {
                sum
                    += (*buffer_out.offset((predictor_coef_num - j) as isize)
                        - *buffer_out.offset(0 as libc::c_int as isize))
                        * *predictor_coef_table.offset(j as isize) as int32_t;
                j += 1;
            }
            outval = ((1 as libc::c_int) << predictor_quantitization - 1 as libc::c_int)
                + sum;
            outval >>= predictor_quantitization;
            outval = outval + *buffer_out.offset(0 as libc::c_int as isize) + error_val;
            outval = outval << 32 as libc::c_int - readsamplesize
                >> 32 as libc::c_int - readsamplesize;
            *buffer_out
                .offset((predictor_coef_num + 1 as libc::c_int) as isize) = outval;
            if error_val > 0 as libc::c_int {
                predictor_num = predictor_coef_num - 1 as libc::c_int;
                while predictor_num >= 0 as libc::c_int {
                    if !(error_val > 0 as libc::c_int) {
                        break;
                    }
                    val___0 = *buffer_out.offset(0 as libc::c_int as isize)
                        - *buffer_out
                            .offset((predictor_coef_num - predictor_num) as isize);
                    if val___0 < 0 as libc::c_int {
                        tmp___0 = -(1 as libc::c_int);
                    } else {
                        if val___0 > 0 as libc::c_int {
                            tmp = 1 as libc::c_int;
                        } else {
                            tmp = 0 as libc::c_int;
                        }
                        tmp___0 = tmp;
                    }
                    sign = tmp___0;
                    *predictor_coef_table
                        .offset(
                            predictor_num as isize,
                        ) = (*predictor_coef_table.offset(predictor_num as isize)
                        as libc::c_int - sign) as int16_t;
                    val___0 *= sign;
                    error_val
                        -= (val___0 >> predictor_quantitization)
                            * (predictor_coef_num - predictor_num);
                    predictor_num -= 1;
                }
            } else if error_val < 0 as libc::c_int {
                predictor_num___0 = predictor_coef_num - 1 as libc::c_int;
                while predictor_num___0 >= 0 as libc::c_int {
                    if !(error_val < 0 as libc::c_int) {
                        break;
                    }
                    val___1 = *buffer_out.offset(0 as libc::c_int as isize)
                        - *buffer_out
                            .offset((predictor_coef_num - predictor_num___0) as isize);
                    if val___1 < 0 as libc::c_int {
                        tmp___2 = -(1 as libc::c_int);
                    } else {
                        if val___1 > 0 as libc::c_int {
                            tmp___1 = 1 as libc::c_int;
                        } else {
                            tmp___1 = 0 as libc::c_int;
                        }
                        tmp___2 = tmp___1;
                    }
                    sign___0 = -tmp___2;
                    *predictor_coef_table
                        .offset(
                            predictor_num___0 as isize,
                        ) = (*predictor_coef_table.offset(predictor_num___0 as isize)
                        as libc::c_int - sign___0) as int16_t;
                    val___1 *= sign___0;
                    error_val
                        -= (val___1 >> predictor_quantitization)
                            * (predictor_coef_num - predictor_num___0);
                    predictor_num___0 -= 1;
                }
            }
            buffer_out = buffer_out.offset(1);
            i += 1;
        }
    }
}
unsafe extern "C" fn deinterlace_16(
    mut buffer_a: *mut int32_t,
    mut buffer_b: *mut int32_t,
    mut buffer_out: *mut int16_t,
    mut numchannels: libc::c_int,
    mut numsamples: libc::c_int,
    mut interlacing_shift: uint8_t,
    mut interlacing_leftweight: uint8_t,
) {
    let mut i: libc::c_int = 0;
    let mut difference: int32_t = 0;
    let mut midright: int32_t = 0;
    let mut left: int16_t = 0;
    let mut right: int16_t = 0;
    let mut left___0: int16_t = 0;
    let mut right___0: int16_t = 0;
    if numsamples <= 0 as libc::c_int {
        return;
    }
    if interlacing_leftweight != 0 {
        i = 0 as libc::c_int;
        while i < numsamples {
            midright = *buffer_a.offset(i as isize);
            difference = *buffer_b.offset(i as isize);
            right = (midright
                - (difference * interlacing_leftweight as int32_t
                    >> interlacing_shift as libc::c_int)) as int16_t;
            left = (right as libc::c_int + difference) as int16_t;
            if host_bigendian != 0 {
                left = ((left as libc::c_int & 255 as libc::c_int) << 8 as libc::c_int
                    | (left as libc::c_int & 65280 as libc::c_int) >> 8 as libc::c_int)
                    as int16_t;
                right = ((right as libc::c_int & 255 as libc::c_int) << 8 as libc::c_int
                    | (right as libc::c_int & 65280 as libc::c_int) >> 8 as libc::c_int)
                    as int16_t;
            }
            *buffer_out.offset((i * numchannels) as isize) = left;
            *buffer_out.offset((i * numchannels + 1 as libc::c_int) as isize) = right;
            i += 1;
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < numsamples {
        left___0 = *buffer_a.offset(i as isize) as int16_t;
        right___0 = *buffer_b.offset(i as isize) as int16_t;
        if host_bigendian != 0 {
            left___0 = ((left___0 as libc::c_int & 255 as libc::c_int)
                << 8 as libc::c_int
                | (left___0 as libc::c_int & 65280 as libc::c_int) >> 8 as libc::c_int)
                as int16_t;
            right___0 = ((right___0 as libc::c_int & 255 as libc::c_int)
                << 8 as libc::c_int
                | (right___0 as libc::c_int & 65280 as libc::c_int) >> 8 as libc::c_int)
                as int16_t;
        }
        *buffer_out.offset((i * numchannels) as isize) = left___0;
        *buffer_out.offset((i * numchannels + 1 as libc::c_int) as isize) = right___0;
        i += 1;
    }
}
unsafe extern "C" fn deinterlace_24(
    mut buffer_a: *mut int32_t,
    mut buffer_b: *mut int32_t,
    mut uncompressed_bytes: libc::c_int,
    mut uncompressed_bytes_buffer_a: *mut int32_t,
    mut uncompressed_bytes_buffer_b: *mut int32_t,
    mut buffer_out: *mut libc::c_void,
    mut numchannels: libc::c_int,
    mut numsamples: libc::c_int,
    mut interlacing_shift: uint8_t,
    mut interlacing_leftweight: uint8_t,
) {
    let mut i: libc::c_int = 0;
    let mut difference: int32_t = 0;
    let mut midright: int32_t = 0;
    let mut left: int32_t = 0;
    let mut right: int32_t = 0;
    let mut mask: uint32_t = 0;
    let mut left___0: int32_t = 0;
    let mut right___0: int32_t = 0;
    let mut mask___0: uint32_t = 0;
    if numsamples <= 0 as libc::c_int {
        return;
    }
    if interlacing_leftweight != 0 {
        i = 0 as libc::c_int;
        while i < numsamples {
            midright = *buffer_a.offset(i as isize);
            difference = *buffer_b.offset(i as isize);
            right = midright
                - (difference * interlacing_leftweight as int32_t
                    >> interlacing_shift as libc::c_int);
            left = right + difference;
            if uncompressed_bytes != 0 {
                mask = !((4294967295 as libc::c_uint)
                    << uncompressed_bytes * 8 as libc::c_int);
                left <<= uncompressed_bytes * 8 as libc::c_int;
                right <<= uncompressed_bytes * 8 as libc::c_int;
                left = (left as libc::c_uint
                    | *uncompressed_bytes_buffer_a.offset(i as isize) as libc::c_uint
                        & mask) as int32_t;
                right = (right as libc::c_uint
                    | *uncompressed_bytes_buffer_b.offset(i as isize) as libc::c_uint
                        & mask) as int32_t;
            }
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int) as isize,
                ) = (left & 255 as libc::c_int) as uint8_t;
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int + 1 as libc::c_int) as isize,
                ) = (left >> 8 as libc::c_int & 255 as libc::c_int) as uint8_t;
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int + 2 as libc::c_int) as isize,
                ) = (left >> 16 as libc::c_int & 255 as libc::c_int) as uint8_t;
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int + 3 as libc::c_int) as isize,
                ) = (right & 255 as libc::c_int) as uint8_t;
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int + 4 as libc::c_int) as isize,
                ) = (right >> 8 as libc::c_int & 255 as libc::c_int) as uint8_t;
            *(buffer_out as *mut uint8_t)
                .offset(
                    (i * numchannels * 3 as libc::c_int + 5 as libc::c_int) as isize,
                ) = (right >> 16 as libc::c_int & 255 as libc::c_int) as uint8_t;
            i += 1;
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < numsamples {
        left___0 = *buffer_a.offset(i as isize);
        right___0 = *buffer_b.offset(i as isize);
        if uncompressed_bytes != 0 {
            mask___0 = !((4294967295 as libc::c_uint)
                << uncompressed_bytes * 8 as libc::c_int);
            left___0 <<= uncompressed_bytes * 8 as libc::c_int;
            right___0 <<= uncompressed_bytes * 8 as libc::c_int;
            left___0 = (left___0 as libc::c_uint
                | *uncompressed_bytes_buffer_a.offset(i as isize) as libc::c_uint
                    & mask___0) as int32_t;
            right___0 = (right___0 as libc::c_uint
                | *uncompressed_bytes_buffer_b.offset(i as isize) as libc::c_uint
                    & mask___0) as int32_t;
        }
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int) as isize,
            ) = (left___0 & 255 as libc::c_int) as uint8_t;
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int + 1 as libc::c_int) as isize,
            ) = (left___0 >> 8 as libc::c_int & 255 as libc::c_int) as uint8_t;
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int + 2 as libc::c_int) as isize,
            ) = (left___0 >> 16 as libc::c_int & 255 as libc::c_int) as uint8_t;
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int + 3 as libc::c_int) as isize,
            ) = (right___0 & 255 as libc::c_int) as uint8_t;
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int + 4 as libc::c_int) as isize,
            ) = (right___0 >> 8 as libc::c_int & 255 as libc::c_int) as uint8_t;
        *(buffer_out as *mut uint8_t)
            .offset(
                (i * numchannels * 3 as libc::c_int + 5 as libc::c_int) as isize,
            ) = (right___0 >> 16 as libc::c_int & 255 as libc::c_int) as uint8_t;
        i += 1;
    }
}
pub unsafe extern "C" fn alac_decode_frame(
    mut alac: *mut alac_file,
    mut inbuffer: *mut libc::c_uchar,
    mut outbuffer: *mut libc::c_void,
    mut outputsize: *mut libc::c_int,
) {
    let mut channels: libc::c_int = 0;
    let mut outputsamples: int32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut hassize: libc::c_int = 0;
    let mut isnotcompressed: libc::c_int = 0;
    let mut readsamplesize: libc::c_int = 0;
    let mut uncompressed_bytes: libc::c_int = 0;
    let mut ricemodifier: libc::c_int = 0;
    let mut tmp___0: uint32_t = 0;
    let mut tmp___1: uint32_t = 0;
    let mut tmp___2: uint32_t = 0;
    let mut tmp___3: uint32_t = 0;
    let mut predictor_coef_table: [int16_t; 32] = [0; 32];
    let mut predictor_coef_num: libc::c_int = 0;
    let mut prediction_type: libc::c_int = 0;
    let mut prediction_quantitization: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___4: uint32_t = 0;
    let mut tmp___5: uint32_t = 0;
    let mut tmp___6: uint32_t = 0;
    let mut tmp___7: uint32_t = 0;
    let mut tmp___8: uint32_t = 0;
    let mut i___0: libc::c_int = 0;
    let mut tmp___9: uint32_t = 0;
    let mut i___1: libc::c_int = 0;
    let mut audiobits: int32_t = 0;
    let mut tmp___10: uint32_t = 0;
    let mut i___2: libc::c_int = 0;
    let mut audiobits___0: int32_t = 0;
    let mut tmp___11: uint32_t = 0;
    let mut tmp___12: uint32_t = 0;
    let mut i___3: libc::c_int = 0;
    let mut sample: int16_t = 0;
    let mut i___4: libc::c_int = 0;
    let mut sample___0: int32_t = 0;
    let mut mask: uint32_t = 0;
    let mut hassize___0: libc::c_int = 0;
    let mut isnotcompressed___0: libc::c_int = 0;
    let mut readsamplesize___0: libc::c_int = 0;
    let mut uncompressed_bytes___0: libc::c_int = 0;
    let mut interlacing_shift: uint8_t = 0;
    let mut interlacing_leftweight: uint8_t = 0;
    let mut tmp___13: uint32_t = 0;
    let mut tmp___14: uint32_t = 0;
    let mut tmp___15: uint32_t = 0;
    let mut tmp___16: uint32_t = 0;
    let mut predictor_coef_table_a: [int16_t; 32] = [0; 32];
    let mut predictor_coef_num_a: libc::c_int = 0;
    let mut prediction_type_a: libc::c_int = 0;
    let mut prediction_quantitization_a: libc::c_int = 0;
    let mut ricemodifier_a: libc::c_int = 0;
    let mut predictor_coef_table_b: [int16_t; 32] = [0; 32];
    let mut predictor_coef_num_b: libc::c_int = 0;
    let mut prediction_type_b: libc::c_int = 0;
    let mut prediction_quantitization_b: libc::c_int = 0;
    let mut ricemodifier_b: libc::c_int = 0;
    let mut i___5: libc::c_int = 0;
    let mut tmp___17: uint32_t = 0;
    let mut tmp___18: uint32_t = 0;
    let mut tmp___19: uint32_t = 0;
    let mut tmp___20: uint32_t = 0;
    let mut tmp___21: uint32_t = 0;
    let mut tmp___22: uint32_t = 0;
    let mut tmp___23: uint32_t = 0;
    let mut tmp___24: uint32_t = 0;
    let mut tmp___25: uint32_t = 0;
    let mut tmp___26: uint32_t = 0;
    let mut tmp___27: uint32_t = 0;
    let mut tmp___28: uint32_t = 0;
    let mut i___6: libc::c_int = 0;
    let mut tmp___29: uint32_t = 0;
    let mut tmp___30: uint32_t = 0;
    let mut i___7: libc::c_int = 0;
    let mut audiobits_a: int32_t = 0;
    let mut audiobits_b: int32_t = 0;
    let mut tmp___31: uint32_t = 0;
    let mut tmp___32: uint32_t = 0;
    let mut i___8: libc::c_int = 0;
    let mut audiobits_a___0: int32_t = 0;
    let mut audiobits_b___0: int32_t = 0;
    let mut tmp___33: uint32_t = 0;
    let mut tmp___34: uint32_t = 0;
    let mut tmp___35: uint32_t = 0;
    let mut tmp___36: uint32_t = 0;
    outputsamples = (*alac).setinfo_max_samples_per_frame as int32_t;
    (*alac).input_buffer = inbuffer;
    (*alac).input_buffer_bitaccumulator = 0 as libc::c_int;
    tmp = readbits(alac, 3 as libc::c_int);
    channels = tmp as libc::c_int;
    *outputsize = outputsamples * (*alac).bytespersample;
    match channels {
        0 => {
            readbits(alac, 4 as libc::c_int);
            readbits(alac, 12 as libc::c_int);
            tmp___0 = readbits(alac, 1 as libc::c_int);
            hassize = tmp___0 as libc::c_int;
            tmp___1 = readbits(alac, 2 as libc::c_int);
            uncompressed_bytes = tmp___1 as libc::c_int;
            tmp___2 = readbits(alac, 1 as libc::c_int);
            isnotcompressed = tmp___2 as libc::c_int;
            if hassize != 0 {
                tmp___3 = readbits(alac, 32 as libc::c_int);
                outputsamples = tmp___3 as int32_t;
                *outputsize = outputsamples * (*alac).bytespersample;
            }
            readsamplesize = (*alac).setinfo_sample_size as libc::c_int
                - uncompressed_bytes * 8 as libc::c_int;
            if isnotcompressed == 0 {
                readbits(alac, 8 as libc::c_int);
                readbits(alac, 8 as libc::c_int);
                tmp___4 = readbits(alac, 4 as libc::c_int);
                prediction_type = tmp___4 as libc::c_int;
                tmp___5 = readbits(alac, 4 as libc::c_int);
                prediction_quantitization = tmp___5 as libc::c_int;
                tmp___6 = readbits(alac, 3 as libc::c_int);
                ricemodifier = tmp___6 as libc::c_int;
                tmp___7 = readbits(alac, 5 as libc::c_int);
                predictor_coef_num = tmp___7 as libc::c_int;
                i = 0 as libc::c_int;
                while i < predictor_coef_num {
                    tmp___8 = readbits(alac, 16 as libc::c_int);
                    predictor_coef_table[i as usize] = tmp___8 as int16_t;
                    i += 1;
                }
                if uncompressed_bytes != 0 {
                    i___0 = 0 as libc::c_int;
                    while i___0 < outputsamples {
                        tmp___9 = readbits(alac, uncompressed_bytes * 8 as libc::c_int);
                        *((*alac).uncompressed_bytes_buffer_a)
                            .offset(i___0 as isize) = tmp___9 as int32_t;
                        i___0 += 1;
                    }
                }
                entropy_rice_decode(
                    alac,
                    (*alac).predicterror_buffer_a,
                    outputsamples,
                    readsamplesize,
                    (*alac).setinfo_rice_initialhistory as libc::c_int,
                    (*alac).setinfo_rice_kmodifier as libc::c_int,
                    ricemodifier * (*alac).setinfo_rice_historymult as libc::c_int
                        / 4 as libc::c_int,
                    ((1 as libc::c_int) << (*alac).setinfo_rice_kmodifier as libc::c_int)
                        - 1 as libc::c_int,
                );
                if prediction_type == 0 as libc::c_int {
                    predictor_decompress_fir_adapt(
                        (*alac).predicterror_buffer_a,
                        (*alac).outputsamples_buffer_a,
                        outputsamples,
                        readsamplesize,
                        predictor_coef_table.as_mut_ptr(),
                        predictor_coef_num,
                        prediction_quantitization,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"FIXME: unhandled predicition type: %i\n\0" as *const u8
                            as *const libc::c_char,
                        prediction_type,
                    );
                }
            } else {
                if (*alac).setinfo_sample_size as libc::c_int <= 16 as libc::c_int {
                    i___1 = 0 as libc::c_int;
                    while i___1 < outputsamples {
                        tmp___10 = readbits(
                            alac,
                            (*alac).setinfo_sample_size as libc::c_int,
                        );
                        audiobits = tmp___10 as int32_t;
                        audiobits = audiobits
                            << 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int
                            >> 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int;
                        *((*alac).outputsamples_buffer_a)
                            .offset(i___1 as isize) = audiobits;
                        i___1 += 1;
                    }
                } else {
                    i___2 = 0 as libc::c_int;
                    while i___2 < outputsamples {
                        tmp___11 = readbits(alac, 16 as libc::c_int);
                        audiobits___0 = tmp___11 as int32_t;
                        audiobits___0
                            <<= (*alac).setinfo_sample_size as libc::c_int
                                - 16 as libc::c_int;
                        tmp___12 = readbits(
                            alac,
                            (*alac).setinfo_sample_size as libc::c_int
                                - 16 as libc::c_int,
                        );
                        audiobits___0 = (audiobits___0 as libc::c_uint | tmp___12)
                            as int32_t;
                        se_struct_24.set_x(audiobits___0);
                        audiobits___0 = se_struct_24.x();
                        *((*alac).outputsamples_buffer_a)
                            .offset(i___2 as isize) = audiobits___0;
                        i___2 += 1;
                    }
                }
                uncompressed_bytes = 0 as libc::c_int;
            }
            match (*alac).setinfo_sample_size as libc::c_int {
                16 => {
                    i___3 = 0 as libc::c_int;
                    while i___3 < outputsamples {
                        sample = *((*alac).outputsamples_buffer_a).offset(i___3 as isize)
                            as int16_t;
                        if host_bigendian != 0 {
                            sample = ((sample as libc::c_int & 255 as libc::c_int)
                                << 8 as libc::c_int
                                | (sample as libc::c_int & 65280 as libc::c_int)
                                    >> 8 as libc::c_int) as int16_t;
                        }
                        *(outbuffer as *mut int16_t)
                            .offset((i___3 * (*alac).numchannels) as isize) = sample;
                        i___3 += 1;
                    }
                }
                24 => {
                    i___4 = 0 as libc::c_int;
                    while i___4 < outputsamples {
                        sample___0 = *((*alac).outputsamples_buffer_a)
                            .offset(i___4 as isize);
                        if uncompressed_bytes != 0 {
                            sample___0 <<= uncompressed_bytes * 8 as libc::c_int;
                            mask = !((4294967295 as libc::c_uint)
                                << uncompressed_bytes * 8 as libc::c_int);
                            sample___0 = (sample___0 as libc::c_uint
                                | *((*alac).uncompressed_bytes_buffer_a)
                                    .offset(i___4 as isize) as libc::c_uint & mask) as int32_t;
                        }
                        *(outbuffer as *mut uint8_t)
                            .offset(
                                (i___4 * (*alac).numchannels * 3 as libc::c_int) as isize,
                            ) = (sample___0 & 255 as libc::c_int) as uint8_t;
                        *(outbuffer as *mut uint8_t)
                            .offset(
                                (i___4 * (*alac).numchannels * 3 as libc::c_int
                                    + 1 as libc::c_int) as isize,
                            ) = (sample___0 >> 8 as libc::c_int & 255 as libc::c_int)
                            as uint8_t;
                        *(outbuffer as *mut uint8_t)
                            .offset(
                                (i___4 * (*alac).numchannels * 3 as libc::c_int
                                    + 2 as libc::c_int) as isize,
                            ) = (sample___0 >> 16 as libc::c_int & 255 as libc::c_int)
                            as uint8_t;
                        i___4 += 1;
                    }
                }
                32 | 20 => {
                    fprintf(
                        stderr,
                        b"FIXME: unimplemented sample size %i\n\0" as *const u8
                            as *const libc::c_char,
                        (*alac).setinfo_sample_size as libc::c_int,
                    );
                }
                _ => {}
            }
        }
        1 => {
            readbits(alac, 4 as libc::c_int);
            readbits(alac, 12 as libc::c_int);
            tmp___13 = readbits(alac, 1 as libc::c_int);
            hassize___0 = tmp___13 as libc::c_int;
            tmp___14 = readbits(alac, 2 as libc::c_int);
            uncompressed_bytes___0 = tmp___14 as libc::c_int;
            tmp___15 = readbits(alac, 1 as libc::c_int);
            isnotcompressed___0 = tmp___15 as libc::c_int;
            if hassize___0 != 0 {
                tmp___16 = readbits(alac, 32 as libc::c_int);
                outputsamples = tmp___16 as int32_t;
                *outputsize = outputsamples * (*alac).bytespersample;
            }
            readsamplesize___0 = (*alac).setinfo_sample_size as libc::c_int
                - uncompressed_bytes___0 * 8 as libc::c_int + 1 as libc::c_int;
            if isnotcompressed___0 == 0 {
                tmp___17 = readbits(alac, 8 as libc::c_int);
                interlacing_shift = tmp___17 as uint8_t;
                tmp___18 = readbits(alac, 8 as libc::c_int);
                interlacing_leftweight = tmp___18 as uint8_t;
                tmp___19 = readbits(alac, 4 as libc::c_int);
                prediction_type_a = tmp___19 as libc::c_int;
                tmp___20 = readbits(alac, 4 as libc::c_int);
                prediction_quantitization_a = tmp___20 as libc::c_int;
                tmp___21 = readbits(alac, 3 as libc::c_int);
                ricemodifier_a = tmp___21 as libc::c_int;
                tmp___22 = readbits(alac, 5 as libc::c_int);
                predictor_coef_num_a = tmp___22 as libc::c_int;
                i___5 = 0 as libc::c_int;
                while i___5 < predictor_coef_num_a {
                    tmp___23 = readbits(alac, 16 as libc::c_int);
                    predictor_coef_table_a[i___5 as usize] = tmp___23 as int16_t;
                    i___5 += 1;
                }
                tmp___24 = readbits(alac, 4 as libc::c_int);
                prediction_type_b = tmp___24 as libc::c_int;
                tmp___25 = readbits(alac, 4 as libc::c_int);
                prediction_quantitization_b = tmp___25 as libc::c_int;
                tmp___26 = readbits(alac, 3 as libc::c_int);
                ricemodifier_b = tmp___26 as libc::c_int;
                tmp___27 = readbits(alac, 5 as libc::c_int);
                predictor_coef_num_b = tmp___27 as libc::c_int;
                i___5 = 0 as libc::c_int;
                while i___5 < predictor_coef_num_b {
                    tmp___28 = readbits(alac, 16 as libc::c_int);
                    predictor_coef_table_b[i___5 as usize] = tmp___28 as int16_t;
                    i___5 += 1;
                }
                if uncompressed_bytes___0 != 0 {
                    i___6 = 0 as libc::c_int;
                    while i___6 < outputsamples {
                        tmp___29 = readbits(
                            alac,
                            uncompressed_bytes___0 * 8 as libc::c_int,
                        );
                        *((*alac).uncompressed_bytes_buffer_a)
                            .offset(i___6 as isize) = tmp___29 as int32_t;
                        tmp___30 = readbits(
                            alac,
                            uncompressed_bytes___0 * 8 as libc::c_int,
                        );
                        *((*alac).uncompressed_bytes_buffer_b)
                            .offset(i___6 as isize) = tmp___30 as int32_t;
                        i___6 += 1;
                    }
                }
                entropy_rice_decode(
                    alac,
                    (*alac).predicterror_buffer_a,
                    outputsamples,
                    readsamplesize___0,
                    (*alac).setinfo_rice_initialhistory as libc::c_int,
                    (*alac).setinfo_rice_kmodifier as libc::c_int,
                    ricemodifier_a * (*alac).setinfo_rice_historymult as libc::c_int
                        / 4 as libc::c_int,
                    ((1 as libc::c_int) << (*alac).setinfo_rice_kmodifier as libc::c_int)
                        - 1 as libc::c_int,
                );
                if prediction_type_a == 0 as libc::c_int {
                    predictor_decompress_fir_adapt(
                        (*alac).predicterror_buffer_a,
                        (*alac).outputsamples_buffer_a,
                        outputsamples,
                        readsamplesize___0,
                        predictor_coef_table_a.as_mut_ptr(),
                        predictor_coef_num_a,
                        prediction_quantitization_a,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"FIXME: unhandled predicition type: %i\n\0" as *const u8
                            as *const libc::c_char,
                        prediction_type_a,
                    );
                }
                entropy_rice_decode(
                    alac,
                    (*alac).predicterror_buffer_b,
                    outputsamples,
                    readsamplesize___0,
                    (*alac).setinfo_rice_initialhistory as libc::c_int,
                    (*alac).setinfo_rice_kmodifier as libc::c_int,
                    ricemodifier_b * (*alac).setinfo_rice_historymult as libc::c_int
                        / 4 as libc::c_int,
                    ((1 as libc::c_int) << (*alac).setinfo_rice_kmodifier as libc::c_int)
                        - 1 as libc::c_int,
                );
                if prediction_type_b == 0 as libc::c_int {
                    predictor_decompress_fir_adapt(
                        (*alac).predicterror_buffer_b,
                        (*alac).outputsamples_buffer_b,
                        outputsamples,
                        readsamplesize___0,
                        predictor_coef_table_b.as_mut_ptr(),
                        predictor_coef_num_b,
                        prediction_quantitization_b,
                    );
                } else {
                    fprintf(
                        stderr,
                        b"FIXME: unhandled predicition type: %i\n\0" as *const u8
                            as *const libc::c_char,
                        prediction_type_b,
                    );
                }
            } else {
                if (*alac).setinfo_sample_size as libc::c_int <= 16 as libc::c_int {
                    i___7 = 0 as libc::c_int;
                    while i___7 < outputsamples {
                        tmp___31 = readbits(
                            alac,
                            (*alac).setinfo_sample_size as libc::c_int,
                        );
                        audiobits_a = tmp___31 as int32_t;
                        tmp___32 = readbits(
                            alac,
                            (*alac).setinfo_sample_size as libc::c_int,
                        );
                        audiobits_b = tmp___32 as int32_t;
                        audiobits_a = audiobits_a
                            << 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int
                            >> 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int;
                        audiobits_b = audiobits_b
                            << 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int
                            >> 32 as libc::c_int
                                - (*alac).setinfo_sample_size as libc::c_int;
                        *((*alac).outputsamples_buffer_a)
                            .offset(i___7 as isize) = audiobits_a;
                        *((*alac).outputsamples_buffer_b)
                            .offset(i___7 as isize) = audiobits_b;
                        i___7 += 1;
                    }
                } else {
                    i___8 = 0 as libc::c_int;
                    while i___8 < outputsamples {
                        tmp___33 = readbits(alac, 16 as libc::c_int);
                        audiobits_a___0 = tmp___33 as int32_t;
                        audiobits_a___0
                            <<= (*alac).setinfo_sample_size as libc::c_int
                                - 16 as libc::c_int;
                        tmp___34 = readbits(
                            alac,
                            (*alac).setinfo_sample_size as libc::c_int
                                - 16 as libc::c_int,
                        );
                        audiobits_a___0 = (audiobits_a___0 as libc::c_uint | tmp___34)
                            as int32_t;
                        se_struct_24.set_x(audiobits_a___0);
                        audiobits_a___0 = se_struct_24.x();
                        tmp___35 = readbits(alac, 16 as libc::c_int);
                        audiobits_b___0 = tmp___35 as int32_t;
                        audiobits_b___0
                            <<= (*alac).setinfo_sample_size as libc::c_int
                                - 16 as libc::c_int;
                        tmp___36 = readbits(
                            alac,
                            (*alac).setinfo_sample_size as libc::c_int
                                - 16 as libc::c_int,
                        );
                        audiobits_b___0 = (audiobits_b___0 as libc::c_uint | tmp___36)
                            as int32_t;
                        se_struct_24.set_x(audiobits_b___0);
                        audiobits_b___0 = se_struct_24.x();
                        *((*alac).outputsamples_buffer_a)
                            .offset(i___8 as isize) = audiobits_a___0;
                        *((*alac).outputsamples_buffer_b)
                            .offset(i___8 as isize) = audiobits_b___0;
                        i___8 += 1;
                    }
                }
                uncompressed_bytes___0 = 0 as libc::c_int;
                interlacing_shift = 0 as libc::c_int as uint8_t;
                interlacing_leftweight = 0 as libc::c_int as uint8_t;
            }
            match (*alac).setinfo_sample_size as libc::c_int {
                16 => {
                    deinterlace_16(
                        (*alac).outputsamples_buffer_a,
                        (*alac).outputsamples_buffer_b,
                        outbuffer as *mut int16_t,
                        (*alac).numchannels,
                        outputsamples,
                        interlacing_shift,
                        interlacing_leftweight,
                    );
                }
                24 => {
                    deinterlace_24(
                        (*alac).outputsamples_buffer_a,
                        (*alac).outputsamples_buffer_b,
                        uncompressed_bytes___0,
                        (*alac).uncompressed_bytes_buffer_a,
                        (*alac).uncompressed_bytes_buffer_b,
                        outbuffer as *mut int16_t as *mut libc::c_void,
                        (*alac).numchannels,
                        outputsamples,
                        interlacing_shift,
                        interlacing_leftweight,
                    );
                }
                32 | 20 => {
                    fprintf(
                        stderr,
                        b"FIXME: unimplemented sample size %i\n\0" as *const u8
                            as *const libc::c_char,
                        (*alac).setinfo_sample_size as libc::c_int,
                    );
                }
                _ => {}
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn alac_create(
    mut samplesize: libc::c_int,
    mut numchannels: libc::c_int,
) -> *mut alac_file {
    let mut newfile: *mut alac_file = 0 as *mut alac_file;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<alac_file>() as libc::c_ulong);
    newfile = tmp as *mut alac_file;
    memset(
        newfile as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<alac_file>() as libc::c_ulong,
    );
    (*newfile).samplesize = samplesize;
    (*newfile).numchannels = numchannels;
    (*newfile).bytespersample = samplesize / 8 as libc::c_int * numchannels;
    return newfile;
}
static mut outputs: [*mut audio_output; 6] = unsafe {
    [
        &audio_alsa as *const audio_output as *mut audio_output,
        &audio_pulse as *const audio_output as *mut audio_output,
        &audio_ao as *const audio_output as *mut audio_output,
        &audio_dummy as *const audio_output as *mut audio_output,
        &audio_pipe as *const audio_output as *mut audio_output,
        0 as *const libc::c_void as *mut libc::c_void as *mut audio_output,
    ]
};
pub unsafe extern "C" fn audio_get_output(
    mut name: *mut libc::c_char,
) -> *mut audio_output {
    let mut out: *mut *mut audio_output = 0 as *mut *mut audio_output;
    let mut tmp: libc::c_int = 0;
    if name.is_null() {
        return outputs[0 as libc::c_int as usize];
    }
    out = outputs.as_mut_ptr();
    while !(*out).is_null() {
        tmp = strcasecmp(
            name as *const libc::c_char,
            (**out).name as *const libc::c_char,
        );
        if tmp == 0 {
            return *out;
        }
        out = out.offset(1);
    }
    return 0 as *mut libc::c_void as *mut audio_output;
}
pub unsafe extern "C" fn audio_ls_outputs() {
    let mut out: *mut *mut audio_output = 0 as *mut *mut audio_output;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    printf(b"Available audio outputs:\n\0" as *const u8 as *const libc::c_char);
    out = outputs.as_mut_ptr();
    while !(*out).is_null() {
        if out as libc::c_ulong == outputs.as_mut_ptr() as libc::c_ulong {
            tmp = b" (default)\0" as *const u8 as *const libc::c_char;
        } else {
            tmp = b"\0" as *const u8 as *const libc::c_char;
        }
        printf(b"    %s%s\n\0" as *const u8 as *const libc::c_char, (**out).name, tmp);
        out = out.offset(1);
    }
    out = outputs.as_mut_ptr();
    while !(*out).is_null() {
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"Options for output %s:\n\0" as *const u8 as *const libc::c_char,
            (**out).name,
        );
        (Some(((**out).help).expect("non-null function pointer")))
            .expect("non-null function pointer")();
        out = out.offset(1);
    }
}
pub static mut Fs: libc::c_int = 0;
pub static mut starttime: libc::c_longlong = 0;
pub static mut samples_played: libc::c_longlong = 0;
unsafe extern "C" fn init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn deinit() {}
unsafe extern "C" fn start(mut sample_rate: libc::c_int) {
    Fs = sample_rate;
    starttime = 0 as libc::c_longlong;
    samples_played = 0 as libc::c_longlong;
    printf(
        b"dummy audio output started at Fs=%d Hz\n\0" as *const u8
            as *const libc::c_char,
        sample_rate,
    );
}
unsafe extern "C" fn play(mut buf: *mut libc::c_short, mut samples: libc::c_int) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut nowtime: libc::c_longlong = 0;
    let mut finishtime: libc::c_longlong = 0;
    gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
    nowtime = (tv.tv_usec as libc::c_double + 1e6f64 * tv.tv_sec as libc::c_double)
        as libc::c_longlong;
    if starttime == 0 {
        starttime = nowtime;
    }
    samples_played += samples as libc::c_longlong;
    finishtime = (starttime as libc::c_double
        + samples_played as libc::c_double * 1e6f64 / Fs as libc::c_double)
        as libc::c_longlong;
    usleep((finishtime - nowtime) as __useconds_t);
}
unsafe extern "C" fn stop() {
    printf(b"dummy audio stopped\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn help() {
    printf(
        b"    There are no options for dummy audio.\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub static mut audio_dummy: audio_output = unsafe {
    {
        let mut init = __anonstruct_audio_output_780339900 {
            help: Some(help as unsafe extern "C" fn() -> ()),
            name: b"dummy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            init: Some(
                init
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            deinit: Some(deinit as unsafe extern "C" fn() -> ()),
            start: Some(start as unsafe extern "C" fn(libc::c_int) -> ()),
            play: Some(
                play as unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> (),
            ),
            stop: Some(stop as unsafe extern "C" fn() -> ()),
            volume: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
            >(0 as *const libc::c_void as *mut libc::c_void),
        };
        init
    }
};
static mut fd___0: libc::c_int = -(1 as libc::c_int);
static mut pipename: *mut libc::c_char = 0 as *const libc::c_void as *mut libc::c_void
    as *mut libc::c_char;
static mut Fs___0: libc::c_int = 0;
static mut starttime___0: libc::c_longlong = 0;
static mut samples_played___0: libc::c_longlong = 0;
unsafe extern "C" fn stop___0() {
    close(fd___0);
    fd___0 = -(1 as libc::c_int);
}
unsafe extern "C" fn start___0(mut sample_rate: libc::c_int) {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    if fd___0 >= 0 as libc::c_int {
        stop___0();
    }
    fd___0 = open(pipename as *const libc::c_char, 2049 as libc::c_int);
    if fd___0 < 0 as libc::c_int {
        tmp = __errno_location();
        if *tmp != 6 as libc::c_int {
            perror(b"open\0" as *const u8 as *const libc::c_char);
            die(
                b"could not open specified pipe for writing\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    if fd___0 >= 0 as libc::c_int {
        close(fd___0);
        fd___0 = open(pipename as *const libc::c_char, 1 as libc::c_int);
    }
    Fs___0 = sample_rate;
    starttime___0 = 0 as libc::c_longlong;
    samples_played___0 = 0 as libc::c_longlong;
}
unsafe extern "C" fn wait_samples(mut samples: libc::c_int) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut nowtime: libc::c_longlong = 0;
    let mut finishtime: libc::c_longlong = 0;
    gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
    nowtime = (tv.tv_usec as libc::c_double + 1e6f64 * tv.tv_sec as libc::c_double)
        as libc::c_longlong;
    if starttime___0 == 0 {
        starttime___0 = nowtime;
    }
    samples_played___0 += samples as libc::c_longlong;
    finishtime = (starttime___0 as libc::c_double
        + samples_played___0 as libc::c_double * 1e6f64 / Fs___0 as libc::c_double)
        as libc::c_longlong;
    usleep((finishtime - nowtime) as __useconds_t);
}
unsafe extern "C" fn play___0(mut buf: *mut libc::c_short, mut samples: libc::c_int) {
    let mut tmp: ssize_t = 0;
    if fd___0 < 0 as libc::c_int {
        wait_samples(samples);
        if samples_played___0 > (5 as libc::c_int * Fs___0) as libc::c_longlong {
            start___0(Fs___0);
        }
        return;
    }
    tmp = write(
        fd___0,
        buf as *const libc::c_void,
        (samples * 4 as libc::c_int) as size_t,
    );
    if tmp < 0 as libc::c_long {
        stop___0();
    }
}
unsafe extern "C" fn init___0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut sb: stat = stat {
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
    if argc != 1 as libc::c_int {
        die(
            b"bad argument(s) to pipe\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    pipename = strdup(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char);
    tmp = stat(pipename as *const libc::c_char, &mut sb as *mut stat);
    if tmp < 0 as libc::c_int {
        die(
            b"could not stat() pipe\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !(sb.st_mode & 61440 as libc::c_uint == 4096 as libc::c_uint) {
        die(b"not a pipe\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn deinit___0() {
    if fd___0 >= 0 as libc::c_int {
        stop___0();
    }
    if !pipename.is_null() {
        free(pipename as *mut libc::c_void);
    }
}
unsafe extern "C" fn help___0() {
    printf(
        b"    pipe takes 1 argument: the name of the FIFO to write to.\n\0" as *const u8
            as *const libc::c_char,
    );
}
pub static mut audio_pipe: audio_output = unsafe {
    {
        let mut init = __anonstruct_audio_output_780339900 {
            help: Some(help___0 as unsafe extern "C" fn() -> ()),
            name: b"pipe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            init: Some(
                init___0
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            deinit: Some(deinit___0 as unsafe extern "C" fn() -> ()),
            start: Some(start___0 as unsafe extern "C" fn(libc::c_int) -> ()),
            play: Some(
                play___0 as unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> (),
            ),
            stop: Some(stop___0 as unsafe extern "C" fn() -> ()),
            volume: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
            >(0 as *const libc::c_void as *mut libc::c_void),
        };
        init
    }
};
#[inline]
unsafe extern "C" fn cmp_nlabel(
    mut L1: *const uint8_t,
    mut L2: *const uint8_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = strcmp(
        L1 as *mut libc::c_char as *const libc::c_char,
        L2 as *mut libc::c_char as *const libc::c_char,
    );
    return tmp;
}
#[inline]
unsafe extern "C" fn dup_nlabel(mut n: *const uint8_t) -> *mut uint8_t {
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*n.offset(0 as libc::c_int as isize) as libc::c_int <= 63 as libc::c_int) {
        __assert_fail(
            b"n[0] <= 63\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_uint,
            b"dup_nlabel\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = strdup(n as *mut libc::c_char as *const libc::c_char);
    return tmp___0 as *mut uint8_t;
}
pub unsafe extern "C" fn dup_label(mut label: *const uint8_t) -> *mut uint8_t {
    let mut len: libc::c_int = 0;
    let mut newlabel: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    len = *label as libc::c_int + 1 as libc::c_int;
    if len > 63 as libc::c_int {
        return 0 as *mut libc::c_void as *mut uint8_t;
    }
    tmp = malloc((len + 1 as libc::c_int) as size_t);
    newlabel = tmp as *mut uint8_t;
    strncpy(
        newlabel as *mut libc::c_char,
        label as *mut libc::c_char as *const libc::c_char,
        len as size_t,
    );
    *newlabel.offset(len as isize) = '\u{0}' as i32 as uint8_t;
    return newlabel;
}
pub unsafe extern "C" fn join_nlabel(
    mut n1: *const uint8_t,
    mut n2: *const uint8_t,
) -> *mut uint8_t {
    let mut len1: libc::c_int = 0;
    let mut len2: libc::c_int = 0;
    let mut s: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    if *n1.offset(0 as libc::c_int as isize) as libc::c_int <= 63 as libc::c_int {
        if !(*n2.offset(0 as libc::c_int as isize) as libc::c_int <= 63 as libc::c_int) {
            __assert_fail(
                b"n1[0] <= 63 && n2[0] <= 63\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_uint,
                b"join_nlabel\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"n1[0] <= 63 && n2[0] <= 63\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_uint,
            b"join_nlabel\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = strlen(n1 as *mut libc::c_char as *const libc::c_char);
    len1 = tmp___0 as libc::c_int;
    tmp___1 = strlen(n2 as *mut libc::c_char as *const libc::c_char);
    len2 = tmp___1 as libc::c_int;
    tmp___2 = malloc((len1 + len2 + 1 as libc::c_int) as size_t);
    s = tmp___2 as *mut uint8_t;
    strncpy(
        s as *mut libc::c_char,
        n1 as *mut libc::c_char as *const libc::c_char,
        len1 as size_t,
    );
    strncpy(
        (s as *mut libc::c_char).offset(len1 as isize),
        n2 as *mut libc::c_char as *const libc::c_char,
        len2 as size_t,
    );
    *s.offset((len1 + len2) as isize) = '\u{0}' as i32 as uint8_t;
    return s;
}
pub unsafe extern "C" fn nlabel_to_str(mut name: *const uint8_t) -> *mut libc::c_char {
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut labelp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if !(name as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_uint,
            b"nlabel_to_str\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = malloc(256 as libc::c_int as size_t);
    labelp = tmp___0 as *mut libc::c_char;
    label = labelp;
    p = name;
    while *p != 0 {
        strncpy(
            labelp,
            (p as *mut libc::c_char).offset(1 as libc::c_int as isize)
                as *const libc::c_char,
            *p as size_t,
        );
        labelp = labelp.offset(*p as libc::c_int as isize);
        *labelp = '.' as i32 as libc::c_char;
        labelp = labelp.offset(1);
        p = p.offset(*p as libc::c_int as isize);
        p = p.offset(1);
    }
    *labelp = '\u{0}' as i32 as libc::c_char;
    return label;
}
unsafe extern "C" fn label_len(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
) -> size_t {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut e: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: size_t = 0;
    e = pkt_buf.offset(pkt_len as isize);
    len = 0 as libc::c_int as size_t;
    p = pkt_buf.offset(off as isize);
    while (p as libc::c_ulong) < e as libc::c_ulong {
        if *p as libc::c_int == 0 as libc::c_int {
            return len.wrapping_add(1 as libc::c_ulong)
        } else {
            if *p as libc::c_int & 192 as libc::c_int == 192 as libc::c_int {
                return len.wrapping_add(2 as libc::c_ulong)
            } else {
                len = (len as libc::c_ulong)
                    .wrapping_add((*p as libc::c_int + 1 as libc::c_int) as size_t)
                    as size_t as size_t;
                p = p.offset(*p as libc::c_int as isize);
            }
        }
        p = p.offset(1);
    }
    return len;
}
pub unsafe extern "C" fn create_label(mut txt: *const libc::c_char) -> *mut uint8_t {
    let mut len: libc::c_int = 0;
    let mut s: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    if !(txt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"txt != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            163 as libc::c_uint,
            b"create_label\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = strlen(txt);
    len = tmp___0 as libc::c_int;
    if len > 63 as libc::c_int {
        return 0 as *mut libc::c_void as *mut uint8_t;
    }
    tmp___1 = malloc((len + 2 as libc::c_int) as size_t);
    s = tmp___1 as *mut uint8_t;
    *s.offset(0 as libc::c_int as isize) = len as uint8_t;
    strncpy(
        (s as *mut libc::c_char).offset(1 as libc::c_int as isize),
        txt,
        len as size_t,
    );
    *s.offset((len + 1 as libc::c_int) as isize) = '\u{0}' as i32 as uint8_t;
    return s;
}
pub unsafe extern "C" fn create_nlabel(mut name: *const libc::c_char) -> *mut uint8_t {
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lenpos: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    len = 0 as libc::c_int;
    if !(name as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"name != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_uint,
            b"create_nlabel\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = strlen(name);
    len = tmp___0 as libc::c_int;
    tmp___1 = malloc((len + 1 as libc::c_int + 1 as libc::c_int) as size_t);
    label = tmp___1 as *mut libc::c_char;
    if label as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut uint8_t;
    }
    strncpy(label.offset(1 as libc::c_int as isize), name, len as size_t);
    *label.offset((len + 1 as libc::c_int) as isize) = '\u{0}' as i32 as libc::c_char;
    p = label;
    e = p.offset(len as isize);
    lenpos = p;
    while (p as libc::c_ulong) < e as libc::c_ulong {
        *lenpos = 0 as libc::c_int as libc::c_char;
        tmp___2 = memchr(
            p.offset(1 as libc::c_int as isize) as *const libc::c_void,
            '.' as i32,
            (e.offset_from(p) as libc::c_long - 1 as libc::c_long) as size_t,
        );
        dot = tmp___2 as *mut libc::c_char;
        if dot as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            dot = e.offset(1 as libc::c_int as isize);
        }
        *lenpos = (dot.offset_from(p) as libc::c_long - 1 as libc::c_long)
            as libc::c_char;
        p = dot;
        lenpos = dot;
    }
    return label as *mut uint8_t;
}
unsafe extern "C" fn copy_label(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
) -> *mut uint8_t {
    let mut len: libc::c_int = 0;
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    if off > pkt_len {
        return 0 as *mut libc::c_void as *mut uint8_t;
    }
    len = *pkt_buf.offset(off as isize) as libc::c_int + 1 as libc::c_int;
    if off.wrapping_add(len as size_t) > pkt_len {
        debug(
            1 as libc::c_int,
            b"label length exceeds packet buffer\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return 0 as *mut libc::c_void as *mut uint8_t;
    }
    tmp = dup_label(pkt_buf.offset(off as isize) as *const uint8_t);
    return tmp;
}
unsafe extern "C" fn uncompress_nlabel(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
) -> *mut uint8_t {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut e: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: size_t = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut llen: size_t = 0;
    let mut p2: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut llen___0: size_t = 0;
    let mut p2___0: *mut uint8_t = 0 as *mut uint8_t;
    e = pkt_buf.offset(pkt_len as isize);
    len = 0 as libc::c_int as size_t;
    if off >= pkt_len {
        return 0 as *mut libc::c_void as *mut uint8_t;
    }
    p = pkt_buf.offset(off as isize);
    while *p != 0 {
        if !((p as libc::c_ulong) < e as libc::c_ulong) {
            break;
        }
        llen = 0 as libc::c_int as size_t;
        if *p as libc::c_int & 192 as libc::c_int == 192 as libc::c_int {
            p2 = pkt_buf
                .offset(
                    ((*p.offset(0 as libc::c_int as isize) as libc::c_int
                        & -(193 as libc::c_int)) << 8 as libc::c_int
                        | *p.offset(1 as libc::c_int as isize) as libc::c_int) as isize,
                );
            llen = (*p2 as libc::c_int + 1 as libc::c_int) as size_t;
            p = p2.offset(llen as isize).offset(-(1 as libc::c_int as isize));
        } else {
            llen = (*p as libc::c_int + 1 as libc::c_int) as size_t;
            p = p.offset(llen.wrapping_sub(1 as libc::c_ulong) as isize);
        }
        len = (len as libc::c_ulong).wrapping_add(llen) as size_t as size_t;
        p = p.offset(1);
    }
    tmp = malloc(len.wrapping_add(1 as libc::c_ulong));
    sp = tmp as *mut libc::c_char;
    str = sp;
    if str as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut uint8_t;
    }
    p = pkt_buf.offset(off as isize);
    while *p != 0 {
        if !((p as libc::c_ulong) < e as libc::c_ulong) {
            break;
        }
        llen___0 = 0 as libc::c_int as size_t;
        if *p as libc::c_int & 192 as libc::c_int == 192 as libc::c_int {
            p2___0 = pkt_buf
                .offset(
                    ((*p.offset(0 as libc::c_int as isize) as libc::c_int
                        & -(193 as libc::c_int)) << 8 as libc::c_int
                        | *p.offset(1 as libc::c_int as isize) as libc::c_int) as isize,
                );
            llen___0 = (*p2___0 as libc::c_int + 1 as libc::c_int) as size_t;
            strncpy(sp, p2___0 as *mut libc::c_char as *const libc::c_char, llen___0);
            p = p2___0.offset(llen___0 as isize).offset(-(1 as libc::c_int as isize));
        } else {
            llen___0 = (*p as libc::c_int + 1 as libc::c_int) as size_t;
            strncpy(sp, p as *mut libc::c_char as *const libc::c_char, llen___0);
            p = p.offset(llen___0.wrapping_sub(1 as libc::c_ulong) as isize);
        }
        sp = sp.offset(llen___0 as isize);
        p = p.offset(1);
    }
    *sp = '\u{0}' as i32 as libc::c_char;
    return str as *mut uint8_t;
}
pub unsafe extern "C" fn rr_get_type_name(mut type_0: rr_type) -> *const libc::c_char {
    match type_0 as libc::c_uint {
        1 => return b"A\0" as *const u8 as *const libc::c_char,
        12 => return b"PTR\0" as *const u8 as *const libc::c_char,
        16 => return b"TXT\0" as *const u8 as *const libc::c_char,
        28 => return b"AAAA\0" as *const u8 as *const libc::c_char,
        33 => return b"SRV\0" as *const u8 as *const libc::c_char,
        47 => return b"NSEC\0" as *const u8 as *const libc::c_char,
        255 => return b"ANY\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return 0 as *mut libc::c_void as *const libc::c_char;
}
pub unsafe extern "C" fn rr_entry_destroy(mut rr: *mut rr_entry) {
    let mut txt_rec: *mut rr_data_txt = 0 as *mut rr_data_txt;
    let mut next: *mut rr_data_txt = 0 as *mut rr_data_txt;
    if rr.is_null() {
        __assert_fail(
            b"rr\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_uint,
            b"rr_entry_destroy\0" as *const u8 as *const libc::c_char,
        );
    }
    match (*rr).type_0 as libc::c_uint {
        12 => {
            if !((*rr).data.PTR.name).is_null() {
                free((*rr).data.PTR.name as *mut libc::c_void);
            }
        }
        16 => {
            txt_rec = &mut (*rr).data.TXT;
            while !txt_rec.is_null() {
                next = (*txt_rec).next;
                if !((*txt_rec).txt).is_null() {
                    free((*txt_rec).txt as *mut libc::c_void);
                }
                if txt_rec as libc::c_ulong
                    != &mut (*rr).data.TXT as *mut rr_data_txt as libc::c_ulong
                {
                    free(txt_rec as *mut libc::c_void);
                }
                txt_rec = next;
            }
        }
        33 => {
            if !((*rr).data.SRV.target).is_null() {
                free((*rr).data.SRV.target as *mut libc::c_void);
            }
        }
        _ => {}
    }
    free((*rr).name as *mut libc::c_void);
    free(rr as *mut libc::c_void);
}
pub unsafe extern "C" fn rr_list_destroy(
    mut rr: *mut rr_list,
    mut destroy_items: libc::c_char,
) {
    let mut rr_next: *mut rr_list = 0 as *mut rr_list;
    while !rr.is_null() {
        rr_next = (*rr).next;
        if destroy_items != 0 {
            rr_entry_destroy((*rr).e);
        }
        free(rr as *mut libc::c_void);
        rr = rr_next;
    }
}
pub unsafe extern "C" fn rr_list_count(mut rr: *mut rr_list) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !rr.is_null() {
        i += 1;
        rr = (*rr).next;
    }
    return i;
}
pub unsafe extern "C" fn rr_list_remove(
    mut rr_head: *mut *mut rr_list,
    mut rr: *mut rr_entry,
) -> *mut rr_entry {
    let mut le: *mut rr_list = 0 as *mut rr_list;
    let mut pe: *mut rr_list = 0 as *mut rr_list;
    le = *rr_head;
    pe = 0 as *mut libc::c_void as *mut rr_list;
    while !le.is_null() {
        if (*le).e as libc::c_ulong == rr as libc::c_ulong {
            if pe as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                *rr_head = (*le).next;
                free(le as *mut libc::c_void);
                return rr;
            } else {
                (*pe).next = (*le).next;
                free(le as *mut libc::c_void);
                return rr;
            }
        }
        pe = le;
        le = (*le).next;
    }
    return 0 as *mut libc::c_void as *mut rr_entry;
}
pub unsafe extern "C" fn rr_list_append(
    mut rr_head: *mut *mut rr_list,
    mut rr: *mut rr_entry,
) -> libc::c_int {
    let mut node: *mut rr_list = 0 as *mut rr_list;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut e: *mut rr_list = 0 as *mut rr_list;
    let mut taile: *mut rr_list = 0 as *mut rr_list;
    tmp = malloc(::std::mem::size_of::<rr_list>() as libc::c_ulong);
    node = tmp as *mut rr_list;
    (*node).e = rr;
    (*node).next = 0 as *mut libc::c_void as *mut rr_list;
    if *rr_head as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        *rr_head = node;
    } else {
        e = *rr_head;
        while !e.is_null() {
            if (*e).e as libc::c_ulong == rr as libc::c_ulong {
                free(node as *mut libc::c_void);
                return 0 as libc::c_int;
            }
            if (*e).next as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                taile = e;
            }
            e = (*e).next;
        }
        (*taile).next = node;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn rr_create_a(
    mut name: *mut uint8_t,
    mut addr: uint32_t,
) -> *mut rr_entry {
    let mut rr: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<rr_entry>() as libc::c_ulong);
    rr = tmp as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    (*rr).name = name;
    (*rr).type_0 = RR_A;
    (*rr).ttl = 120 as libc::c_int as uint32_t;
    (*rr).cache_flush = 1 as libc::c_int as libc::c_char;
    (*rr).rr_class = 1 as libc::c_int as uint16_t;
    (*rr).data.A.addr = addr;
    return rr;
}
pub unsafe extern "C" fn rr_create_aaaa(
    mut name: *mut uint8_t,
    mut addr: *mut in6_addr,
) -> *mut rr_entry {
    let mut rr: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<rr_entry>() as libc::c_ulong);
    rr = tmp as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    (*rr).name = name;
    (*rr).type_0 = RR_AAAA;
    (*rr).ttl = 120 as libc::c_int as uint32_t;
    (*rr).cache_flush = 1 as libc::c_int as libc::c_char;
    (*rr).rr_class = 1 as libc::c_int as uint16_t;
    (*rr).data.AAAA.addr = addr;
    return rr;
}
pub unsafe extern "C" fn rr_create_srv(
    mut name: *mut uint8_t,
    mut port: uint16_t,
    mut target: *mut uint8_t,
) -> *mut rr_entry {
    let mut rr: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<rr_entry>() as libc::c_ulong);
    rr = tmp as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    (*rr).name = name;
    (*rr).type_0 = RR_SRV;
    (*rr).ttl = 120 as libc::c_int as uint32_t;
    (*rr).cache_flush = 1 as libc::c_int as libc::c_char;
    (*rr).rr_class = 1 as libc::c_int as uint16_t;
    (*rr).data.SRV.port = port;
    (*rr).data.SRV.target = target;
    return rr;
}
pub unsafe extern "C" fn rr_create_ptr(
    mut name: *mut uint8_t,
    mut d_rr: *mut rr_entry,
) -> *mut rr_entry {
    let mut rr: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<rr_entry>() as libc::c_ulong);
    rr = tmp as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    (*rr).name = name;
    (*rr).type_0 = RR_PTR;
    (*rr).ttl = 120 as libc::c_int as uint32_t;
    (*rr).cache_flush = 1 as libc::c_int as libc::c_char;
    (*rr).rr_class = 1 as libc::c_int as uint16_t;
    (*rr).cache_flush = 0 as libc::c_int as libc::c_char;
    (*rr).data.PTR.entry = d_rr;
    return rr;
}
pub unsafe extern "C" fn rr_create(
    mut name: *mut uint8_t,
    mut type_0: rr_type,
) -> *mut rr_entry {
    let mut rr: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<rr_entry>() as libc::c_ulong);
    rr = tmp as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    (*rr).name = name;
    (*rr).type_0 = type_0;
    (*rr).ttl = 120 as libc::c_int as uint32_t;
    (*rr).cache_flush = 1 as libc::c_int as libc::c_char;
    (*rr).rr_class = 1 as libc::c_int as uint16_t;
    return rr;
}
pub unsafe extern "C" fn rr_set_nsec(mut rr_nsec: *mut rr_entry, mut type_0: rr_type) {
    let mut tmp___2: rr_type = 0 as rr_type;
    tmp___2 = RR_NSEC;
    (*rr_nsec).type_0 = tmp___2;
    if tmp___2 as u64 == 0 {
        __assert_fail(
            b"rr_nsec->type = RR_NSEC\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_uint,
            b"rr_set_nsec\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(((type_0 as libc::c_uint).wrapping_div(8 as libc::c_uint) as libc::c_ulong)
        < ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong)
    {
        __assert_fail(
            b"(type / 8) < sizeof(rr_nsec->data.NSEC.bitmap)\0" as *const u8
                as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            441 as libc::c_uint,
            b"rr_set_nsec\0" as *const u8 as *const libc::c_char,
        );
    }
    (*rr_nsec)
        .data
        .NSEC
        .bitmap[(type_0 as libc::c_uint).wrapping_div(8 as libc::c_uint)
        as usize] = ((1 as libc::c_int)
        << (7 as libc::c_uint)
            .wrapping_sub((type_0 as libc::c_uint).wrapping_rem(8 as libc::c_uint)))
        as uint8_t;
}
pub unsafe extern "C" fn rr_add_txt(
    mut rr_txt: *mut rr_entry,
    mut txt: *const libc::c_char,
) {
    let mut txt_rec: *mut rr_data_txt = 0 as *mut rr_data_txt;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if !((*rr_txt).type_0 as libc::c_uint == 16 as libc::c_uint) {
        __assert_fail(
            b"rr_txt->type == RR_TXT\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            448 as libc::c_uint,
            b"rr_add_txt\0" as *const u8 as *const libc::c_char,
        );
    }
    txt_rec = &mut (*rr_txt).data.TXT;
    if (*txt_rec).txt as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        (*txt_rec).txt = create_label(txt);
        return;
    }
    while !((*txt_rec).next).is_null() {
        txt_rec = (*txt_rec).next;
    }
    tmp___0 = malloc(::std::mem::size_of::<rr_data_txt>() as libc::c_ulong);
    (*txt_rec).next = tmp___0 as *mut rr_data_txt;
    txt_rec = (*txt_rec).next;
    (*txt_rec).txt = create_label(txt);
    (*txt_rec).next = 0 as *mut libc::c_void as *mut rr_data_txt;
}
pub unsafe extern "C" fn rr_group_add(
    mut group: *mut *mut rr_group,
    mut rr: *mut rr_entry,
) {
    let mut g: *mut rr_group = 0 as *mut rr_group;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if !(rr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"rr != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            473 as libc::c_uint,
            b"rr_group_add\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(*group).is_null() {
        g = rr_group_find(*group, (*rr).name);
        if !g.is_null() {
            rr_list_append(&mut (*g).rr, rr);
            return;
        }
    }
    tmp___0 = malloc(::std::mem::size_of::<rr_group>() as libc::c_ulong);
    g = tmp___0 as *mut rr_group;
    memset(
        g as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_group>() as libc::c_ulong,
    );
    (*g).name = dup_nlabel((*rr).name as *const uint8_t);
    rr_list_append(&mut (*g).rr, rr);
    (*g).next = *group;
    *group = g;
}
pub unsafe extern "C" fn rr_group_find(
    mut g: *mut rr_group,
    mut name: *mut uint8_t,
) -> *mut rr_group {
    let mut tmp: libc::c_int = 0;
    while !g.is_null() {
        tmp = cmp_nlabel((*g).name as *const uint8_t, name as *const uint8_t);
        if tmp == 0 as libc::c_int {
            return g;
        }
        g = (*g).next;
    }
    return 0 as *mut libc::c_void as *mut rr_group;
}
pub unsafe extern "C" fn rr_entry_find(
    mut rr_list: *mut rr_list,
    mut name: *mut uint8_t,
    mut type_0: uint16_t,
) -> *mut rr_entry {
    let mut rr: *mut rr_list = 0 as *mut rr_list;
    let mut tmp: libc::c_int = 0;
    rr = rr_list;
    while !rr.is_null() {
        if (*(*rr).e).type_0 as libc::c_uint == type_0 as libc::c_uint {
            tmp = cmp_nlabel((*(*rr).e).name as *const uint8_t, name as *const uint8_t);
            if tmp == 0 as libc::c_int {
                return (*rr).e;
            }
        }
        rr = (*rr).next;
    }
    return 0 as *mut libc::c_void as *mut rr_entry;
}
pub unsafe extern "C" fn rr_entry_match(
    mut rr_list: *mut rr_list,
    mut entry: *mut rr_entry,
) -> *mut rr_entry {
    let mut rr: *mut rr_list = 0 as *mut rr_list;
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    rr = rr_list;
    while !rr.is_null() {
        if (*(*rr).e).type_0 as libc::c_uint == (*entry).type_0 as libc::c_uint {
            tmp___2 = cmp_nlabel(
                (*(*rr).e).name as *const uint8_t,
                (*entry).name as *const uint8_t,
            );
            if tmp___2 == 0 as libc::c_int {
                if (*entry).type_0 as libc::c_uint != 12 as libc::c_uint {
                    return (*rr).e
                } else {
                    if (*(*rr).e).data.PTR.name as libc::c_ulong
                        != 0 as *mut libc::c_void as libc::c_ulong
                    {
                        tmp = (*(*rr).e).data.PTR.name;
                    } else {
                        tmp = (*(*(*rr).e).data.PTR.entry).name;
                    }
                    if (*entry).data.PTR.name as libc::c_ulong
                        != 0 as *mut libc::c_void as libc::c_ulong
                    {
                        tmp___0 = (*entry).data.PTR.name;
                    } else {
                        tmp___0 = (*(*entry).data.PTR.entry).name;
                    }
                    tmp___1 = cmp_nlabel(
                        tmp___0 as *const uint8_t,
                        tmp as *const uint8_t,
                    );
                    if tmp___1 == 0 as libc::c_int {
                        return (*rr).e;
                    }
                }
            }
        }
        rr = (*rr).next;
    }
    return 0 as *mut libc::c_void as *mut rr_entry;
}
pub unsafe extern "C" fn rr_group_destroy(mut group: *mut rr_group) {
    let mut g: *mut rr_group = 0 as *mut rr_group;
    let mut nextg: *mut rr_group = 0 as *mut rr_group;
    g = group;
    while !g.is_null() {
        nextg = (*g).next;
        free((*g).name as *mut libc::c_void);
        rr_list_destroy((*g).rr, 1 as libc::c_int as libc::c_char);
        free(g as *mut libc::c_void);
        g = nextg;
    }
}
pub unsafe extern "C" fn mdns_write_u16(
    mut ptr: *mut uint8_t,
    v: uint16_t,
) -> *mut uint8_t {
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: *mut uint8_t = 0 as *mut uint8_t;
    tmp = ptr;
    ptr = ptr.offset(1);
    *tmp = ((v as libc::c_int >> 8 as libc::c_int) as uint8_t as libc::c_int
        & 255 as libc::c_int) as uint8_t;
    tmp___0 = ptr;
    ptr = ptr.offset(1);
    *tmp___0 = (v as libc::c_int as uint8_t as libc::c_int & 255 as libc::c_int)
        as uint8_t;
    return ptr;
}
pub unsafe extern "C" fn mdns_write_u32(
    mut ptr: *mut uint8_t,
    v: uint32_t,
) -> *mut uint8_t {
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___1: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___2: *mut uint8_t = 0 as *mut uint8_t;
    tmp = ptr;
    ptr = ptr.offset(1);
    *tmp = ((v >> 24 as libc::c_int) as uint8_t as libc::c_int & 255 as libc::c_int)
        as uint8_t;
    tmp___0 = ptr;
    ptr = ptr.offset(1);
    *tmp___0 = ((v >> 16 as libc::c_int) as uint8_t as libc::c_int & 255 as libc::c_int)
        as uint8_t;
    tmp___1 = ptr;
    ptr = ptr.offset(1);
    *tmp___1 = ((v >> 8 as libc::c_int) as uint8_t as libc::c_int & 255 as libc::c_int)
        as uint8_t;
    tmp___2 = ptr;
    ptr = ptr.offset(1);
    *tmp___2 = (v as uint8_t as libc::c_int & 255 as libc::c_int) as uint8_t;
    return ptr;
}
pub unsafe extern "C" fn mdns_read_u16(mut ptr: *const uint8_t) -> uint16_t {
    return ((*ptr.offset(0 as libc::c_int as isize) as libc::c_int & 255 as libc::c_int)
        << 8 as libc::c_int
        | *ptr.offset(1 as libc::c_int as isize) as libc::c_int & 255 as libc::c_int)
        as uint16_t;
}
pub unsafe extern "C" fn mdns_read_u32(mut ptr: *const uint8_t) -> uint32_t {
    return ((*ptr.offset(0 as libc::c_int as isize) as libc::c_int & 255 as libc::c_int)
        << 24 as libc::c_int
        | (*ptr.offset(1 as libc::c_int as isize) as libc::c_int & 255 as libc::c_int)
            << 16 as libc::c_int
        | (*ptr.offset(2 as libc::c_int as isize) as libc::c_int & 255 as libc::c_int)
            << 8 as libc::c_int
        | *ptr.offset(3 as libc::c_int as isize) as libc::c_int & 255 as libc::c_int)
        as uint32_t;
}
pub unsafe extern "C" fn mdns_init_reply(mut pkt: *mut mdns_pkt, mut id: uint16_t) {
    (*pkt).id = id;
    (*pkt)
        .flags = ((1 as libc::c_int) << 15 as libc::c_int
        | (1 as libc::c_int) << 10 as libc::c_int) as uint16_t;
    rr_list_destroy((*pkt).rr_qn, 0 as libc::c_int as libc::c_char);
    rr_list_destroy((*pkt).rr_ans, 0 as libc::c_int as libc::c_char);
    rr_list_destroy((*pkt).rr_auth, 0 as libc::c_int as libc::c_char);
    rr_list_destroy((*pkt).rr_add, 0 as libc::c_int as libc::c_char);
    (*pkt).rr_qn = 0 as *mut libc::c_void as *mut rr_list;
    (*pkt).rr_ans = 0 as *mut libc::c_void as *mut rr_list;
    (*pkt).rr_auth = 0 as *mut libc::c_void as *mut rr_list;
    (*pkt).rr_add = 0 as *mut libc::c_void as *mut rr_list;
    (*pkt).num_qn = 0 as libc::c_int as uint16_t;
    (*pkt).num_ans_rr = 0 as libc::c_int as uint16_t;
    (*pkt).num_auth_rr = 0 as libc::c_int as uint16_t;
    (*pkt).num_add_rr = 0 as libc::c_int as uint16_t;
}
pub unsafe extern "C" fn mdns_pkt_destroy(mut p: *mut mdns_pkt) {
    rr_list_destroy((*p).rr_qn, 1 as libc::c_int as libc::c_char);
    rr_list_destroy((*p).rr_ans, 1 as libc::c_int as libc::c_char);
    rr_list_destroy((*p).rr_auth, 1 as libc::c_int as libc::c_char);
    rr_list_destroy((*p).rr_add, 1 as libc::c_int as libc::c_char);
    free(p as *mut libc::c_void);
}
unsafe extern "C" fn mdns_parse_qn(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
    mut pkt: *mut mdns_pkt,
) -> size_t {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut rr: *mut rr_entry = 0 as *mut rr_entry;
    let mut name: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: uint16_t = 0;
    let mut tmp___3: uint16_t = 0;
    p = pkt_buf.offset(off as isize) as *const uint8_t;
    if !(pkt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"pkt != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            609 as libc::c_uint,
            b"mdns_parse_qn\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = malloc(::std::mem::size_of::<rr_entry>() as libc::c_ulong);
    rr = tmp___0 as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    name = uncompress_nlabel(pkt_buf, pkt_len, off);
    tmp___1 = label_len(pkt_buf, pkt_len, off);
    p = p.offset(tmp___1 as isize);
    (*rr).name = name;
    tmp___2 = mdns_read_u16(p);
    (*rr).type_0 = tmp___2 as rr_type;
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*rr)
        .unicast_query = (*p as libc::c_int & 128 as libc::c_int == 128 as libc::c_int)
        as libc::c_int as libc::c_char;
    tmp___3 = mdns_read_u16(p);
    (*rr).rr_class = (tmp___3 as libc::c_int & -(129 as libc::c_int)) as uint16_t;
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    rr_list_append(&mut (*pkt).rr_qn, rr);
    return p.offset_from(pkt_buf.offset(off as isize) as *const uint8_t) as libc::c_long
        as size_t;
}
unsafe extern "C" fn mdns_parse_rr(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
    mut pkt: *mut mdns_pkt,
) -> size_t {
    let mut p: *const uint8_t = 0 as *const uint8_t;
    let mut e: *const uint8_t = 0 as *const uint8_t;
    let mut rr: *mut rr_entry = 0 as *mut rr_entry;
    let mut name: *mut uint8_t = 0 as *mut uint8_t;
    let mut rr_data_len: size_t = 0;
    let mut txt_rec: *mut rr_data_txt = 0 as *mut rr_data_txt;
    let mut parse_error: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: uint16_t = 0;
    let mut tmp___3: uint16_t = 0;
    let mut tmp___4: uint16_t = 0;
    let mut tmp___5: uint32_t = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    p = pkt_buf.offset(off as isize) as *const uint8_t;
    e = pkt_buf.offset(pkt_len as isize) as *const uint8_t;
    rr_data_len = 0 as libc::c_int as size_t;
    parse_error = 0 as libc::c_int;
    if !(pkt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"pkt != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            642 as libc::c_uint,
            b"mdns_parse_rr\0" as *const u8 as *const libc::c_char,
        );
    }
    if off > pkt_len {
        return 0 as libc::c_int as size_t;
    }
    tmp___0 = malloc(::std::mem::size_of::<rr_entry>() as libc::c_ulong);
    rr = tmp___0 as *mut rr_entry;
    memset(
        rr as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<rr_entry>() as libc::c_ulong,
    );
    name = uncompress_nlabel(pkt_buf, pkt_len, off);
    tmp___1 = label_len(pkt_buf, pkt_len, off);
    p = p.offset(tmp___1 as isize);
    (*rr).name = name;
    tmp___2 = mdns_read_u16(p);
    (*rr).type_0 = tmp___2 as rr_type;
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*rr)
        .cache_flush = (*p as libc::c_int & 128 as libc::c_int == 128 as libc::c_int)
        as libc::c_int as libc::c_char;
    tmp___3 = mdns_read_u16(p);
    (*rr).rr_class = (tmp___3 as libc::c_int & -(129 as libc::c_int)) as uint16_t;
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*rr).ttl = mdns_read_u32(p);
    p = p.offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as isize);
    tmp___4 = mdns_read_u16(p);
    rr_data_len = tmp___4 as size_t;
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    if p.offset(rr_data_len as isize) as libc::c_ulong > e as libc::c_ulong {
        debug(
            1 as libc::c_int,
            b"rr_data_len goes beyond packet buffer: %lu > %lu\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            rr_data_len,
            e.offset_from(p) as libc::c_long,
        );
        rr_entry_destroy(rr);
        return 0 as libc::c_int as size_t;
    }
    e = p.offset(rr_data_len as isize);
    match (*rr).type_0 as libc::c_uint {
        1 => {
            if rr_data_len < ::std::mem::size_of::<uint32_t>() as libc::c_ulong {
                debug(
                    1 as libc::c_int,
                    b"invalid rr_data_len=%lu for A record\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    rr_data_len,
                );
                parse_error = 1 as libc::c_int;
            } else {
                tmp___5 = mdns_read_u32(p);
                (*rr).data.A.addr = __bswap_32(tmp___5);
                p = p
                    .offset(::std::mem::size_of::<uint32_t>() as libc::c_ulong as isize);
            }
        }
        28 => {
            if rr_data_len < ::std::mem::size_of::<in6_addr>() as libc::c_ulong {
                debug(
                    1 as libc::c_int,
                    b"invalid rr_data_len=%lu for AAAA record\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    rr_data_len,
                );
                parse_error = 1 as libc::c_int;
            } else {
                tmp___6 = malloc(::std::mem::size_of::<in6_addr>() as libc::c_ulong);
                (*rr).data.AAAA.addr = tmp___6 as *mut in6_addr;
                i = 0 as libc::c_int;
                while (i as libc::c_ulong)
                    < ::std::mem::size_of::<in6_addr>() as libc::c_ulong
                {
                    (*(*rr).data.AAAA.addr)
                        .__in6_u
                        .__u6_addr8[i as usize] = *p.offset(i as isize);
                    i += 1;
                }
                p = p
                    .offset(::std::mem::size_of::<in6_addr>() as libc::c_ulong as isize);
            }
        }
        12 => {
            (*rr)
                .data
                .PTR
                .name = uncompress_nlabel(
                pkt_buf,
                pkt_len,
                p.offset_from(pkt_buf as *const uint8_t) as libc::c_long as size_t,
            );
            if (*rr).data.PTR.name as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                debug(
                    1 as libc::c_int,
                    b"unable to parse/uncompress label for PTR name\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                parse_error = 1 as libc::c_int;
            } else {
                p = p.offset(rr_data_len as isize);
            }
        }
        16 => {
            txt_rec = &mut (*rr).data.TXT;
            if rr_data_len == 0 as libc::c_ulong {
                debug(
                    1 as libc::c_int,
                    b"WARN: rr_data_len for TXT is 0\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                (*txt_rec).txt = create_label(b"\0" as *const u8 as *const libc::c_char);
            } else {
                loop {
                    (*txt_rec)
                        .txt = copy_label(
                        pkt_buf,
                        pkt_len,
                        p.offset_from(pkt_buf as *const uint8_t) as libc::c_long
                            as size_t,
                    );
                    if (*txt_rec).txt as libc::c_ulong
                        == 0 as *mut libc::c_void as libc::c_ulong
                    {
                        debug(
                            1 as libc::c_int,
                            b"unable to copy label for TXT record\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        parse_error = 1 as libc::c_int;
                        break;
                    } else {
                        p = p
                            .offset(
                                (*((*txt_rec).txt).offset(0 as libc::c_int as isize)
                                    as libc::c_int + 1 as libc::c_int) as isize,
                            );
                        if p as libc::c_ulong >= e as libc::c_ulong {
                            break;
                        }
                        tmp___7 = malloc(
                            ::std::mem::size_of::<rr_data_txt>() as libc::c_ulong,
                        );
                        (*txt_rec).next = tmp___7 as *mut rr_data_txt;
                        txt_rec = (*txt_rec).next;
                        (*txt_rec).next = 0 as *mut libc::c_void as *mut rr_data_txt;
                    }
                }
            }
        }
        _ => {
            p = e;
        }
    }
    if parse_error != 0 {
        rr_entry_destroy(rr);
        return 0 as libc::c_int as size_t;
    }
    rr_list_append(&mut (*pkt).rr_ans, rr);
    return p.offset_from(pkt_buf.offset(off as isize) as *const uint8_t) as libc::c_long
        as size_t;
}
pub unsafe extern "C" fn mdns_parse_pkt(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
) -> *mut mdns_pkt {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut off: size_t = 0;
    let mut pkt: *mut mdns_pkt = 0 as *mut mdns_pkt;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut l: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut l___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    p = pkt_buf;
    if pkt_len < 12 as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut mdns_pkt;
    }
    tmp = malloc(::std::mem::size_of::<mdns_pkt>() as libc::c_ulong);
    pkt = tmp as *mut mdns_pkt;
    memset(
        pkt as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mdns_pkt>() as libc::c_ulong,
    );
    (*pkt).id = mdns_read_u16(p as *const uint8_t);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*pkt).flags = mdns_read_u16(p as *const uint8_t);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*pkt).num_qn = mdns_read_u16(p as *const uint8_t);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*pkt).num_ans_rr = mdns_read_u16(p as *const uint8_t);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*pkt).num_auth_rr = mdns_read_u16(p as *const uint8_t);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    (*pkt).num_add_rr = mdns_read_u16(p as *const uint8_t);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    off = p.offset_from(pkt_buf) as libc::c_long as size_t;
    i = 0 as libc::c_int;
    while i < (*pkt).num_qn as libc::c_int {
        tmp___0 = mdns_parse_qn(pkt_buf, pkt_len, off, pkt);
        l = tmp___0;
        if l == 0 {
            debug(
                1 as libc::c_int,
                b"error parsing question #%d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                i,
            );
            mdns_pkt_destroy(pkt);
            return 0 as *mut libc::c_void as *mut mdns_pkt;
        }
        off = (off as libc::c_ulong).wrapping_add(l) as size_t as size_t;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*pkt).num_ans_rr as libc::c_int {
        tmp___1 = mdns_parse_rr(pkt_buf, pkt_len, off, pkt);
        l___0 = tmp___1;
        if l___0 == 0 {
            debug(
                1 as libc::c_int,
                b"error parsing answer #%d\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                i,
            );
            mdns_pkt_destroy(pkt);
            return 0 as *mut libc::c_void as *mut mdns_pkt;
        }
        off = (off as libc::c_ulong).wrapping_add(l___0) as size_t as size_t;
        i += 1;
    }
    return pkt;
}
unsafe extern "C" fn mdns_encode_name(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
    mut name: *const uint8_t,
    mut comp: *mut name_comp,
) -> size_t {
    let mut c: *mut name_comp = 0 as *mut name_comp;
    let mut c_tail: *mut name_comp = 0 as *mut name_comp;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut len: size_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut segment_len: libc::c_int = 0;
    let mut new_c: *mut name_comp = 0 as *mut name_comp;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    c_tail = 0 as *mut libc::c_void as *mut name_comp;
    p = pkt_buf.offset(off as isize);
    len = 0 as libc::c_int as size_t;
    if !name.is_null() {
        while *name != 0 {
            c = comp;
            while !c.is_null() {
                tmp = cmp_nlabel(name, (*c).label as *const uint8_t);
                if tmp == 0 as libc::c_int {
                    mdns_write_u16(
                        p,
                        (49152 as libc::c_ulong
                            | (*c).pos & 0xffffffffffff3fff as libc::c_ulong) as uint16_t,
                    );
                    return len
                        .wrapping_add(
                            ::std::mem::size_of::<uint16_t>() as libc::c_ulong,
                        );
                }
                if (*c).next as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
                {
                    c_tail = c;
                }
                c = (*c).next;
            }
            segment_len = *name as libc::c_int + 1 as libc::c_int;
            strncpy(
                p as *mut libc::c_char,
                name as *mut libc::c_char as *const libc::c_char,
                segment_len as size_t,
            );
            tmp___0 = malloc(::std::mem::size_of::<name_comp>() as libc::c_ulong);
            new_c = tmp___0 as *mut name_comp;
            memset(
                new_c as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<name_comp>() as libc::c_ulong,
            );
            (*new_c).label = name as *mut uint8_t;
            (*new_c).pos = p.offset_from(pkt_buf) as libc::c_long as size_t;
            (*c_tail).next = new_c;
            p = p.offset(segment_len as isize);
            len = (len as libc::c_ulong).wrapping_add(segment_len as size_t) as size_t
                as size_t;
            name = name.offset(segment_len as isize);
        }
    }
    *p = '\u{0}' as i32 as uint8_t;
    len = len.wrapping_add(1);
    return len;
}
unsafe extern "C" fn mdns_encode_rr(
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
    mut off: size_t,
    mut rr: *mut rr_entry,
    mut comp: *mut name_comp,
) -> size_t {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut p_data: *mut uint8_t = 0 as *mut uint8_t;
    let mut l: size_t = 0;
    let mut txt_rec: *mut rr_data_txt = 0 as *mut rr_data_txt;
    let mut label: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: libc::c_int = 0;
    let mut tmp___1: __uint32_t = 0;
    let mut tmp___2: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___3: size_t = 0;
    let mut len: libc::c_int = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___7: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___8: *mut uint8_t = 0 as *mut uint8_t;
    p = pkt_buf.offset(off as isize);
    if !(off < pkt_len) {
        __assert_fail(
            b"off < pkt_len\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            862 as libc::c_uint,
            b"mdns_encode_rr\0" as *const u8 as *const libc::c_char,
        );
    }
    l = mdns_encode_name(pkt_buf, pkt_len, off, (*rr).name as *const uint8_t, comp);
    if !(l != 0 as libc::c_ulong) {
        __assert_fail(
            b"l != 0\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            866 as libc::c_uint,
            b"mdns_encode_rr\0" as *const u8 as *const libc::c_char,
        );
    }
    p = p.offset(l as isize);
    p = mdns_write_u16(p, (*rr).type_0 as uint16_t);
    p = mdns_write_u16(
        p,
        ((*rr).rr_class as libc::c_int & -(32769 as libc::c_int)
            | ((*rr).cache_flush as libc::c_int) << 15 as libc::c_int) as uint16_t,
    );
    p = mdns_write_u32(p, (*rr).ttl);
    p = p.offset(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize);
    p_data = p;
    match (*rr).type_0 as libc::c_uint {
        1 => {
            tmp___1 = __bswap_32((*rr).data.A.addr);
            p = mdns_write_u32(p, tmp___1);
        }
        28 => {
            i = 0 as libc::c_int;
            while (i as libc::c_ulong)
                < ::std::mem::size_of::<in6_addr>() as libc::c_ulong
            {
                tmp___2 = p;
                p = p.offset(1);
                *tmp___2 = (*(*rr).data.AAAA.addr).__in6_u.__u6_addr8[i as usize];
                i += 1;
            }
        }
        12 => {
            if !((*rr).data.PTR.name).is_null() {
                label = (*rr).data.PTR.name;
            } else {
                label = (*(*rr).data.PTR.entry).name;
            }
            tmp___3 = mdns_encode_name(
                pkt_buf,
                pkt_len,
                p.offset_from(pkt_buf) as libc::c_long as size_t,
                label as *const uint8_t,
                comp,
            );
            p = p.offset(tmp___3 as isize);
        }
        16 => {
            txt_rec = &mut (*rr).data.TXT;
            while !txt_rec.is_null() {
                len = *((*txt_rec).txt).offset(0 as libc::c_int as isize) as libc::c_int
                    + 1 as libc::c_int;
                strncpy(
                    p as *mut libc::c_char,
                    (*txt_rec).txt as *mut libc::c_char as *const libc::c_char,
                    len as size_t,
                );
                p = p.offset(len as isize);
                txt_rec = (*txt_rec).next;
            }
        }
        33 => {
            p = mdns_write_u16(p, (*rr).data.SRV.priority);
            p = mdns_write_u16(p, (*rr).data.SRV.weight);
            p = mdns_write_u16(p, (*rr).data.SRV.port);
            tmp___4 = mdns_encode_name(
                pkt_buf,
                pkt_len,
                p.offset_from(pkt_buf) as libc::c_long as size_t,
                (*rr).data.SRV.target as *const uint8_t,
                comp,
            );
            p = p.offset(tmp___4 as isize);
        }
        47 => {
            tmp___5 = mdns_encode_name(
                pkt_buf,
                pkt_len,
                p.offset_from(pkt_buf) as libc::c_long as size_t,
                (*rr).name as *const uint8_t,
                comp,
            );
            p = p.offset(tmp___5 as isize);
            tmp___6 = p;
            p = p.offset(1);
            *tmp___6 = 0 as libc::c_int as uint8_t;
            tmp___7 = p;
            p = p.offset(1);
            *tmp___7 = ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong as uint8_t;
            i = 0 as libc::c_int;
            while (i as libc::c_ulong)
                < ::std::mem::size_of::<[uint8_t; 5]>() as libc::c_ulong
            {
                tmp___8 = p;
                p = p.offset(1);
                *tmp___8 = (*rr).data.NSEC.bitmap[i as usize];
                i += 1;
            }
        }
        _ => {
            debug(
                1 as libc::c_int,
                b"unhandled rr type 0x%02x\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*rr).type_0 as libc::c_uint,
            );
        }
    }
    l = p.offset_from(p_data) as libc::c_long as size_t;
    mdns_write_u16(
        p
            .offset(-(l as isize))
            .offset(-(::std::mem::size_of::<uint16_t>() as libc::c_ulong as isize)),
        l as uint16_t,
    );
    return (p.offset_from(pkt_buf) as libc::c_long as size_t).wrapping_sub(off);
}
pub unsafe extern "C" fn mdns_encode_pkt(
    mut answer: *mut mdns_pkt,
    mut pkt_buf: *mut uint8_t,
    mut pkt_len: size_t,
) -> size_t {
    let mut comp: *mut name_comp = 0 as *mut name_comp;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut off: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut rr_set: [*mut rr_list; 3] = [0 as *mut rr_list; 3];
    let mut rr: *mut rr_list = 0 as *mut rr_list;
    let mut l: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut c: *mut name_comp = 0 as *mut name_comp;
    p = pkt_buf;
    if !(answer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"answer != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            957 as libc::c_uint,
            b"mdns_encode_pkt\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(pkt_len >= 12 as libc::c_ulong) {
        __assert_fail(
            b"pkt_len >= 12\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            958 as libc::c_uint,
            b"mdns_encode_pkt\0" as *const u8 as *const libc::c_char,
        );
    }
    if p as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int) as size_t;
    }
    if !((*answer).num_qn as libc::c_int == 0 as libc::c_int) {
        __assert_fail(
            b"answer->num_qn == 0\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            964 as libc::c_uint,
            b"mdns_encode_pkt\0" as *const u8 as *const libc::c_char,
        );
    }
    p = mdns_write_u16(p, (*answer).id);
    p = mdns_write_u16(p, (*answer).flags);
    p = mdns_write_u16(p, (*answer).num_qn);
    p = mdns_write_u16(p, (*answer).num_ans_rr);
    p = mdns_write_u16(p, (*answer).num_auth_rr);
    p = mdns_write_u16(p, (*answer).num_add_rr);
    off = p.offset_from(pkt_buf) as libc::c_long as size_t;
    tmp___2 = malloc(::std::mem::size_of::<name_comp>() as libc::c_ulong);
    comp = tmp___2 as *mut name_comp;
    if comp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return -(1 as libc::c_int) as size_t;
    }
    memset(
        comp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<name_comp>() as libc::c_ulong,
    );
    (*comp).label = b"\0" as *const u8 as *const libc::c_char as *mut uint8_t;
    (*comp).pos = 0 as libc::c_int as size_t;
    rr_set[0 as libc::c_int as usize] = (*answer).rr_ans;
    rr_set[1 as libc::c_int as usize] = (*answer).rr_auth;
    rr_set[2 as libc::c_int as usize] = (*answer).rr_add;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[*mut rr_list; 3]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut rr_list>() as libc::c_ulong)
    {
        rr = rr_set[i as usize];
        while !rr.is_null() {
            tmp___3 = mdns_encode_rr(pkt_buf, pkt_len, off, (*rr).e, comp);
            l = tmp___3;
            off = (off as libc::c_ulong).wrapping_add(l) as size_t as size_t;
            if off >= pkt_len {
                debug(
                    1 as libc::c_int,
                    b"packet buffer too small\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return -(1 as libc::c_int) as size_t;
            }
            rr = (*rr).next;
        }
        i += 1;
    }
    while !comp.is_null() {
        c = (*comp).next;
        free(comp as *mut libc::c_void);
        comp = c;
    }
    return off;
}
unsafe extern "C" fn create_recv_sock() -> libc::c_int {
    let mut sd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut on: libc::c_int = 0;
    let mut serveraddr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut mreq: ip_mreq = ip_mreq {
        imr_multiaddr: in_addr { s_addr: 0 },
        imr_interface: in_addr { s_addr: 0 },
    };
    tmp = socket(2 as libc::c_int, 2 as libc::c_int, 0 as libc::c_int);
    sd = tmp;
    if sd < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv socket(): %m\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv socket(): %m\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        return sd;
    }
    r = -(1 as libc::c_int);
    on = 1 as libc::c_int;
    r = setsockopt(
        sd,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if r < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv setsockopt(SO_REUSEADDR): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv setsockopt(SO_REUSEADDR): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        return r;
    }
    memset(
        &mut serveraddr as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    serveraddr.sin_family = 2 as libc::c_int as sa_family_t;
    serveraddr.sin_port = __bswap_16(5353 as libc::c_int as __uint16_t);
    serveraddr.sin_addr.s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
    r = bind(
        sd,
        &mut serveraddr as *mut sockaddr_in as *mut sockaddr as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if r < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv bind(): %m\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv bind(): %m\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
    }
    memset(
        &mut mreq as *mut ip_mreq as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ip_mreq>() as libc::c_ulong,
    );
    mreq.imr_interface.s_addr = __bswap_32(0 as libc::c_int as in_addr_t);
    mreq
        .imr_multiaddr
        .s_addr = inet_addr(b"224.0.0.251\0" as *const u8 as *const libc::c_char);
    r = setsockopt(
        sd,
        0 as libc::c_int,
        35 as libc::c_int,
        &mut mreq as *mut ip_mreq as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<ip_mreq>() as libc::c_ulong as socklen_t,
    );
    if r < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv setsockopt(IP_ADD_MEMBERSHIP): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv setsockopt(IP_ADD_MEMBERSHIP): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        return r;
    }
    r = setsockopt(
        sd,
        0 as libc::c_int,
        34 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if r < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv setsockopt(IP_MULTICAST_LOOP): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv setsockopt(IP_MULTICAST_LOOP): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        return r;
    }
    r = setsockopt(
        sd,
        0 as libc::c_int,
        8 as libc::c_int,
        &mut on as *mut libc::c_int as *mut libc::c_char as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    );
    if r < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"recv setsockopt(IP_PKTINFO): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"recv setsockopt(IP_PKTINFO): %m\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        return r;
    }
    return sd;
}
static mut toaddr: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
};
unsafe extern "C" fn send_packet(
    mut fd___1: libc::c_int,
    mut data: *const libc::c_void,
    mut len: size_t,
) -> ssize_t {
    let mut tmp: ssize_t = 0;
    if toaddr.sin_family as libc::c_int != 2 as libc::c_int {
        memset(
            &mut toaddr as *mut sockaddr_in as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
        );
        toaddr.sin_family = 2 as libc::c_int as sa_family_t;
        toaddr.sin_port = __bswap_16(5353 as libc::c_int as __uint16_t);
        toaddr
            .sin_addr
            .s_addr = inet_addr(b"224.0.0.251\0" as *const u8 as *const libc::c_char);
    }
    tmp = sendto(
        fd___1,
        data,
        len,
        0 as libc::c_int,
        &mut toaddr as *mut sockaddr_in as *mut sockaddr as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    return tmp;
}
unsafe extern "C" fn populate_answers(
    mut svr___0: *mut mdnsd,
    mut rr_head: *mut *mut rr_list,
    mut name: *mut uint8_t,
    mut type_0: rr_type,
) -> libc::c_int {
    let mut num_ans: libc::c_int = 0;
    let mut ans_grp: *mut rr_group = 0 as *mut rr_group;
    let mut tmp: *mut rr_group = 0 as *mut rr_group;
    let mut n: *mut rr_list = 0 as *mut rr_list;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    num_ans = 0 as libc::c_int;
    pthread_mutex_lock(&mut (*svr___0).data_lock);
    tmp = rr_group_find((*svr___0).group, name);
    ans_grp = tmp;
    if ans_grp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        pthread_mutex_unlock(&mut (*svr___0).data_lock);
        return num_ans;
    }
    n = (*ans_grp).rr;
    while !n.is_null() {
        let mut current_block_15: u64;
        if type_0 as libc::c_uint == 255 as libc::c_uint {
            if (*(*n).e).type_0 as libc::c_uint == 47 as libc::c_uint {
                current_block_15 = 11204318019117207424;
            } else {
                current_block_15 = 2968425633554183086;
            }
        } else {
            current_block_15 = 2968425633554183086;
        }
        match current_block_15 {
            2968425633554183086 => {
                let mut current_block_13: u64;
                if type_0 as libc::c_uint == (*(*n).e).type_0 as libc::c_uint {
                    current_block_13 = 14329119203590967986;
                } else if type_0 as libc::c_uint == 255 as libc::c_uint {
                    current_block_13 = 14329119203590967986;
                } else {
                    current_block_13 = 5948590327928692120;
                }
                match current_block_13 {
                    14329119203590967986 => {
                        tmp___1 = cmp_nlabel(
                            name as *const uint8_t,
                            (*(*n).e).name as *const uint8_t,
                        );
                        if tmp___1 == 0 as libc::c_int {
                            tmp___0 = rr_list_append(rr_head, (*n).e);
                            num_ans += tmp___0;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        n = (*n).next;
    }
    pthread_mutex_unlock(&mut (*svr___0).data_lock);
    return num_ans;
}
unsafe extern "C" fn add_related_rr(
    mut svr___0: *mut mdnsd,
    mut list: *mut rr_list,
    mut reply: *mut mdns_pkt,
) {
    let mut ans: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    while !list.is_null() {
        ans = (*list).e;
        match (*ans).type_0 as libc::c_uint {
            12 => {
                if (*ans).data.PTR.name as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                    tmp = (*ans).data.PTR.name;
                } else {
                    tmp = (*(*ans).data.PTR.entry).name;
                }
                tmp___0 = populate_answers(svr___0, &mut (*reply).rr_add, tmp, RR_ANY);
                (*reply)
                    .num_add_rr = ((*reply).num_add_rr as libc::c_int + tmp___0)
                    as uint16_t;
            }
            33 => {
                tmp___1 = populate_answers(
                    svr___0,
                    &mut (*reply).rr_add,
                    (*ans).data.SRV.target,
                    RR_ANY,
                );
                (*reply)
                    .num_add_rr = ((*reply).num_add_rr as libc::c_int + tmp___1)
                    as uint16_t;
                tmp___2 = populate_answers(
                    svr___0,
                    &mut (*reply).rr_add,
                    (*ans).name,
                    RR_TXT,
                );
                (*reply)
                    .num_add_rr = ((*reply).num_add_rr as libc::c_int + tmp___2)
                    as uint16_t;
            }
            28 | 1 => {
                tmp___3 = populate_answers(
                    svr___0,
                    &mut (*reply).rr_add,
                    (*ans).name,
                    RR_NSEC,
                );
                (*reply)
                    .num_add_rr = ((*reply).num_add_rr as libc::c_int + tmp___3)
                    as uint16_t;
            }
            _ => {}
        }
        list = (*list).next;
    }
}
unsafe extern "C" fn announce_srv(
    mut svr___0: *mut mdnsd,
    mut reply: *mut mdns_pkt,
    mut name: *mut uint8_t,
) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    mdns_init_reply(reply, 0 as libc::c_int as uint16_t);
    tmp = populate_answers(svr___0, &mut (*reply).rr_ans, name, RR_PTR);
    (*reply).num_ans_rr = ((*reply).num_ans_rr as libc::c_int + tmp) as uint16_t;
    tmp___0 = populate_answers(
        svr___0,
        &mut (*reply).rr_ans,
        b"\t_services\x07_dns-sd\x04_udp\x05local\0" as *const u8 as *const libc::c_char
            as *mut uint8_t,
        RR_PTR,
    );
    (*reply).num_ans_rr = ((*reply).num_ans_rr as libc::c_int + tmp___0) as uint16_t;
    add_related_rr(svr___0, (*reply).rr_ans, reply);
    add_related_rr(svr___0, (*reply).rr_add, reply);
}
unsafe extern "C" fn process_mdns_pkt(
    mut svr___0: *mut mdnsd,
    mut pkt: *mut mdns_pkt,
    mut reply: *mut mdns_pkt,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut qnl: *mut rr_list = 0 as *mut rr_list;
    let mut qn: *mut rr_entry = 0 as *mut rr_entry;
    let mut num_ans_added: libc::c_int = 0;
    let mut namestr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    let mut ans: *mut rr_list = 0 as *mut rr_list;
    let mut prev_ans: *mut rr_list = 0 as *mut rr_list;
    let mut next_ans: *mut rr_list = 0 as *mut rr_list;
    let mut known_ans: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp___2: *mut rr_entry = 0 as *mut rr_entry;
    let mut namestr___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(pkt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"pkt != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1237 as libc::c_uint,
            b"process_mdns_pkt\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*pkt).flags as libc::c_int & (1 as libc::c_int) << 15 as libc::c_int
        == 0 as libc::c_int
    {
        if (*pkt).flags as libc::c_int >> 11 as libc::c_int & 15 as libc::c_int
            == 0 as libc::c_int
        {
            mdns_init_reply(reply, (*pkt).id);
            debug(
                1 as libc::c_int,
                b"flags = %04x, qn = %d, ans = %d, add = %d\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*pkt).flags as libc::c_int,
                (*pkt).num_qn as libc::c_int,
                (*pkt).num_ans_rr as libc::c_int,
                (*pkt).num_add_rr as libc::c_int,
            );
            qnl = (*pkt).rr_qn;
            i = 0 as libc::c_int;
            while i < (*pkt).num_qn as libc::c_int {
                qn = (*qnl).e;
                num_ans_added = 0 as libc::c_int;
                tmp___0 = nlabel_to_str((*qn).name as *const uint8_t);
                namestr = tmp___0;
                tmp___1 = rr_get_type_name((*qn).type_0);
                debug(
                    1 as libc::c_int,
                    b"qn #%d: type %s (%02x) %s - \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    i,
                    tmp___1,
                    (*qn).type_0 as libc::c_uint,
                    namestr,
                );
                free(namestr as *mut libc::c_void);
                if (*qn).unicast_query != 0 {
                    debug(
                        1 as libc::c_int,
                        b"skipping unicast query\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else {
                    num_ans_added = populate_answers(
                        svr___0,
                        &mut (*reply).rr_ans,
                        (*qn).name,
                        (*qn).type_0,
                    );
                    (*reply)
                        .num_ans_rr = ((*reply).num_ans_rr as libc::c_int
                        + num_ans_added) as uint16_t;
                    debug(
                        1 as libc::c_int,
                        b"added %d answers\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        num_ans_added,
                    );
                }
                i += 1;
                qnl = (*qnl).next;
            }
            ans = 0 as *mut libc::c_void as *mut rr_list;
            prev_ans = 0 as *mut libc::c_void as *mut rr_list;
            ans = (*reply).rr_ans;
            while !ans.is_null() {
                next_ans = (*ans).next;
                tmp___2 = rr_entry_match((*pkt).rr_ans, (*ans).e);
                known_ans = tmp___2;
                if known_ans as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
                {
                    if (*known_ans).ttl
                        >= ((*(*ans).e).ttl).wrapping_div(2 as libc::c_uint)
                    {
                        tmp___3 = nlabel_to_str((*(*ans).e).name as *const uint8_t);
                        namestr___0 = tmp___3;
                        debug(
                            1 as libc::c_int,
                            b"removing answer for %s\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            namestr___0,
                        );
                        free(namestr___0 as *mut libc::c_void);
                        if prev_ans as libc::c_ulong
                            == 0 as *mut libc::c_void as libc::c_ulong
                        {
                            (*reply).rr_ans = (*ans).next;
                        } else {
                            (*prev_ans).next = (*ans).next;
                        }
                        free(ans as *mut libc::c_void);
                        ans = prev_ans;
                        (*reply)
                            .num_ans_rr = ((*reply).num_ans_rr as libc::c_int
                            - 1 as libc::c_int) as uint16_t;
                    }
                }
                prev_ans = ans;
                ans = next_ans;
            }
            add_related_rr(svr___0, (*reply).rr_ans, reply);
            add_related_rr(svr___0, (*reply).rr_add, reply);
            debug(
                1 as libc::c_int,
                b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return (*reply).num_ans_rr as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn create_pipe(mut handles: *mut libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = pipe(handles);
    return tmp;
}
pub unsafe extern "C" fn read_pipe(
    mut s: libc::c_int,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut tmp: ssize_t = 0;
    tmp = read(s, buf as *mut libc::c_void, len as size_t);
    return tmp as libc::c_int;
}
pub unsafe extern "C" fn write_pipe(
    mut s: libc::c_int,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut tmp: ssize_t = 0;
    tmp = write(s, buf as *const libc::c_void, len as size_t);
    return tmp as libc::c_int;
}
pub unsafe extern "C" fn close_pipe(mut s: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = close(s);
    return tmp;
}
unsafe extern "C" fn main_loop(mut svr___0: *mut mdnsd) {
    let mut sockfd_set: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut max_fd: libc::c_int = 0;
    let mut notify_buf: [libc::c_char; 2] = [0; 2];
    let mut pkt_buffer: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut mdns_reply: *mut mdns_pkt = 0 as *mut mdns_pkt;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let mut fromaddr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut sockaddr_size: socklen_t = 0;
    let mut recvsize: ssize_t = 0;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mdns: *mut mdns_pkt = 0 as *mut mdns_pkt;
    let mut tmp___3: *mut mdns_pkt = 0 as *mut mdns_pkt;
    let mut replylen: size_t = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut ann_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut namestr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut replylen___0: size_t = 0;
    let mut tmp___7: size_t = 0;
    let mut svc_le: *mut rr_list = 0 as *mut rr_list;
    let mut tmp___8: libc::c_int = 0;
    let mut replylen___1: size_t = 0;
    let mut tmp___9: size_t = 0;
    max_fd = (*svr___0).sockfd;
    tmp = malloc(65536 as libc::c_int as size_t);
    pkt_buffer = tmp;
    if (*svr___0).notify_pipe[0 as libc::c_int as usize] > max_fd {
        max_fd = (*svr___0).notify_pipe[0 as libc::c_int as usize];
    }
    tmp___0 = malloc(::std::mem::size_of::<mdns_pkt>() as libc::c_ulong);
    mdns_reply = tmp___0 as *mut mdns_pkt;
    memset(
        mdns_reply as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mdns_pkt>() as libc::c_ulong,
    );
    while (*svr___0).stop_flag == 0 {
        let fresh8 = &mut __d0;
        let fresh9;
        let fresh10 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
        let fresh11 = &mut __d1;
        let fresh12;
        let fresh13 = &mut *(sockfd_set.__fds_bits)
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut __fd_mask;
        asm!(
            "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh8,
            fresh10) => fresh9, inlateout("di")
            c2rust_asm_casts::AsmCast::cast_in(fresh11, fresh13) => fresh12,
            inlateout("ax") 0 as libc::c_int => _, options(preserves_flags, att_syntax)
        );
        c2rust_asm_casts::AsmCast::cast_out(fresh8, fresh10, fresh9);
        c2rust_asm_casts::AsmCast::cast_out(fresh11, fresh13, fresh12);
        sockfd_set
            .__fds_bits[((*svr___0).sockfd
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << (*svr___0).sockfd
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        sockfd_set
            .__fds_bits[((*svr___0).notify_pipe[0 as libc::c_int as usize]
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << (*svr___0).notify_pipe[0 as libc::c_int as usize]
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        select(
            max_fd + 1 as libc::c_int,
            &mut sockfd_set as *mut fd_set,
            0 as *mut libc::c_void as *mut fd_set,
            0 as *mut libc::c_void as *mut fd_set,
            0 as *mut libc::c_void as *mut timeval,
        );
        if sockfd_set
            .__fds_bits[((*svr___0).notify_pipe[0 as libc::c_int as usize]
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            & ((1 as libc::c_ulong)
                << (*svr___0).notify_pipe[0 as libc::c_int as usize]
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask != 0 as libc::c_long
        {
            read_pipe(
                (*svr___0).notify_pipe[0 as libc::c_int as usize],
                &mut notify_buf as *mut [libc::c_char; 2] as *mut libc::c_char,
                1 as libc::c_int,
            );
        } else if sockfd_set
                .__fds_bits[((*svr___0).sockfd
                / (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as usize]
                & ((1 as libc::c_ulong)
                    << (*svr___0).sockfd
                        % (8 as libc::c_int
                            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                                as libc::c_int)) as __fd_mask != 0 as libc::c_long
            {
            sockaddr_size = ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong
                as socklen_t;
            tmp___1 = recvfrom(
                (*svr___0).sockfd,
                pkt_buffer,
                65536 as libc::c_int as size_t,
                0 as libc::c_int,
                &mut fromaddr as *mut sockaddr_in as *mut sockaddr,
                &mut sockaddr_size as *mut socklen_t,
            );
            recvsize = tmp___1;
            if recvsize < 0 as libc::c_long {
                match 3 as libc::c_int {
                    3 => {
                        warn(
                            b"recv(): %m\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                    _ => {
                        debug(
                            1 as libc::c_int,
                            b"recv(): %m\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                    }
                }
            }
            tmp___2 = inet_ntoa(fromaddr.sin_addr);
            debug(
                1 as libc::c_int,
                b"data from=%s size=%ld\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                tmp___2,
                recvsize,
            );
            tmp___3 = mdns_parse_pkt(pkt_buffer as *mut uint8_t, recvsize as size_t);
            mdns = tmp___3;
            if mdns as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                tmp___5 = process_mdns_pkt(svr___0, mdns, mdns_reply);
                if tmp___5 != 0 {
                    tmp___4 = mdns_encode_pkt(
                        mdns_reply,
                        pkt_buffer as *mut uint8_t,
                        65536 as libc::c_int as size_t,
                    );
                    replylen = tmp___4;
                    send_packet(
                        (*svr___0).sockfd,
                        pkt_buffer as *const libc::c_void,
                        replylen,
                    );
                } else if (*mdns).num_qn as libc::c_int == 0 as libc::c_int {
                    debug(
                        1 as libc::c_int,
                        b"(no questions in packet)\n\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                mdns_pkt_destroy(mdns);
            }
        }
        loop {
            ann_e = 0 as *mut libc::c_void as *mut rr_entry;
            pthread_mutex_lock(&mut (*svr___0).data_lock);
            if !((*svr___0).announce).is_null() {
                ann_e = rr_list_remove(
                    &mut (*svr___0).announce,
                    (*(*svr___0).announce).e,
                );
            }
            pthread_mutex_unlock(&mut (*svr___0).data_lock);
            if ann_e.is_null() {
                break;
            }
            tmp___6 = nlabel_to_str((*ann_e).name as *const uint8_t);
            namestr = tmp___6;
            debug(
                1 as libc::c_int,
                b"sending announce for %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                namestr,
            );
            free(namestr as *mut libc::c_void);
            announce_srv(svr___0, mdns_reply, (*ann_e).name);
            if (*mdns_reply).num_ans_rr as libc::c_int > 0 as libc::c_int {
                tmp___7 = mdns_encode_pkt(
                    mdns_reply,
                    pkt_buffer as *mut uint8_t,
                    65536 as libc::c_int as size_t,
                );
                replylen___0 = tmp___7;
                send_packet(
                    (*svr___0).sockfd,
                    pkt_buffer as *const libc::c_void,
                    replylen___0,
                );
            }
        }
    }
    mdns_init_reply(mdns_reply, 0 as libc::c_int as uint16_t);
    pthread_mutex_lock(&mut (*svr___0).data_lock);
    svc_le = (*svr___0).services;
    while !svc_le.is_null() {
        (*(*svc_le).e).ttl = 0 as libc::c_int as uint32_t;
        tmp___8 = rr_list_append(&mut (*mdns_reply).rr_ans, (*svc_le).e);
        (*mdns_reply)
            .num_ans_rr = ((*mdns_reply).num_ans_rr as libc::c_int + tmp___8)
            as uint16_t;
        svc_le = (*svc_le).next;
    }
    pthread_mutex_unlock(&mut (*svr___0).data_lock);
    if (*mdns_reply).num_ans_rr as libc::c_int > 0 as libc::c_int {
        tmp___9 = mdns_encode_pkt(
            mdns_reply,
            pkt_buffer as *mut uint8_t,
            65536 as libc::c_int as size_t,
        );
        replylen___1 = tmp___9;
        send_packet((*svr___0).sockfd, pkt_buffer as *const libc::c_void, replylen___1);
    }
    mdns_init_reply(mdns_reply, 0 as libc::c_int as uint16_t);
    free(mdns_reply as *mut libc::c_void);
    free(pkt_buffer);
    close_pipe((*svr___0).sockfd);
    (*svr___0).stop_flag = 2 as libc::c_int;
}
pub unsafe extern "C" fn mdnsd_set_hostname(
    mut svr___0: *mut mdnsd,
    mut hostname: *const libc::c_char,
    mut ip: uint32_t,
) {
    let mut a_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut nsec_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp___0: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___1: *mut uint8_t = 0 as *mut uint8_t;
    a_e = 0 as *mut libc::c_void as *mut rr_entry;
    nsec_e = 0 as *mut libc::c_void as *mut rr_entry;
    if !((*svr___0).hostname as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
    {
        __assert_fail(
            b"svr->hostname == NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1501 as libc::c_uint,
            b"mdnsd_set_hostname\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = create_nlabel(hostname);
    a_e = rr_create_a(tmp___0, ip);
    tmp___1 = create_nlabel(hostname);
    nsec_e = rr_create(tmp___1, RR_NSEC);
    rr_set_nsec(nsec_e, RR_A);
    pthread_mutex_lock(&mut (*svr___0).data_lock);
    (*svr___0).hostname = create_nlabel(hostname);
    rr_group_add(&mut (*svr___0).group, a_e);
    rr_group_add(&mut (*svr___0).group, nsec_e);
    pthread_mutex_unlock(&mut (*svr___0).data_lock);
}
pub unsafe extern "C" fn mdnsd_set_hostname_v6(
    mut svr___0: *mut mdnsd,
    mut hostname: *const libc::c_char,
    mut addr: *mut in6_addr,
) {
    let mut aaaa_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut nsec_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut tmp___0: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___1: *mut uint8_t = 0 as *mut uint8_t;
    aaaa_e = 0 as *mut libc::c_void as *mut rr_entry;
    nsec_e = 0 as *mut libc::c_void as *mut rr_entry;
    if !((*svr___0).hostname as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong)
    {
        __assert_fail(
            b"svr->hostname == NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1522 as libc::c_uint,
            b"mdnsd_set_hostname_v6\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = create_nlabel(hostname);
    aaaa_e = rr_create_aaaa(tmp___0, addr);
    tmp___1 = create_nlabel(hostname);
    nsec_e = rr_create(tmp___1, RR_NSEC);
    rr_set_nsec(nsec_e, RR_AAAA);
    pthread_mutex_lock(&mut (*svr___0).data_lock);
    (*svr___0).hostname = create_nlabel(hostname);
    rr_group_add(&mut (*svr___0).group, aaaa_e);
    rr_group_add(&mut (*svr___0).group, nsec_e);
    pthread_mutex_unlock(&mut (*svr___0).data_lock);
}
pub unsafe extern "C" fn mdnsd_add_rr(mut svr___0: *mut mdnsd, mut rr: *mut rr_entry) {
    pthread_mutex_lock(&mut (*svr___0).data_lock);
    rr_group_add(&mut (*svr___0).group, rr);
    pthread_mutex_unlock(&mut (*svr___0).data_lock);
}
pub unsafe extern "C" fn mdnsd_register_svc(
    mut svr___0: *mut mdnsd,
    mut instance_name: *const libc::c_char,
    mut type_0: *const libc::c_char,
    mut port: uint16_t,
    mut hostname: *const libc::c_char,
    mut txt: *mut *const libc::c_char,
) -> *mut mdns_service {
    let mut txt_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut srv_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut ptr_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut bptr_e: *mut rr_entry = 0 as *mut rr_entry;
    let mut target: *mut uint8_t = 0 as *mut uint8_t;
    let mut inst_nlabel: *mut uint8_t = 0 as *mut uint8_t;
    let mut type_nlabel: *mut uint8_t = 0 as *mut uint8_t;
    let mut nlabel: *mut uint8_t = 0 as *mut uint8_t;
    let mut service: *mut mdns_service = 0 as *mut mdns_service;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___2: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___3: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___4: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp___5: *mut uint8_t = 0 as *mut uint8_t;
    txt_e = 0 as *mut libc::c_void as *mut rr_entry;
    srv_e = 0 as *mut libc::c_void as *mut rr_entry;
    ptr_e = 0 as *mut libc::c_void as *mut rr_entry;
    bptr_e = 0 as *mut libc::c_void as *mut rr_entry;
    tmp = malloc(::std::mem::size_of::<mdns_service>() as libc::c_ulong);
    service = tmp as *mut mdns_service;
    memset(
        service as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mdns_service>() as libc::c_ulong,
    );
    type_nlabel = create_nlabel(type_0);
    inst_nlabel = create_label(instance_name);
    nlabel = join_nlabel(inst_nlabel as *const uint8_t, type_nlabel as *const uint8_t);
    if !txt.is_null() {
        if !(*txt).is_null() {
            tmp___0 = dup_nlabel(nlabel as *const uint8_t);
            txt_e = rr_create(tmp___0, RR_TXT);
            rr_list_append(&mut (*service).entries, txt_e);
            while !(*txt).is_null() {
                rr_add_txt(txt_e, *txt);
                txt = txt.offset(1);
            }
        }
    }
    if hostname.is_null() {
        if ((*svr___0).hostname).is_null() {
            __assert_fail(
                b"hostname || svr->hostname\0" as *const u8 as *const libc::c_char,
                b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
                1569 as libc::c_uint,
                b"mdnsd_register_svc\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if !hostname.is_null() {
        tmp___2 = create_nlabel(hostname);
        target = tmp___2;
    } else {
        tmp___3 = dup_nlabel((*svr___0).hostname as *const uint8_t);
        target = tmp___3;
    }
    tmp___4 = dup_nlabel(nlabel as *const uint8_t);
    srv_e = rr_create_srv(tmp___4, port, target);
    rr_list_append(&mut (*service).entries, srv_e);
    ptr_e = rr_create_ptr(type_nlabel, srv_e);
    tmp___5 = dup_nlabel(
        b"\t_services\x07_dns-sd\x04_udp\x05local\0" as *const u8 as *const libc::c_char
            as *mut uint8_t as *const uint8_t,
    );
    bptr_e = rr_create_ptr(tmp___5, ptr_e);
    pthread_mutex_lock(&mut (*svr___0).data_lock);
    if !txt_e.is_null() {
        rr_group_add(&mut (*svr___0).group, txt_e);
    }
    rr_group_add(&mut (*svr___0).group, srv_e);
    rr_group_add(&mut (*svr___0).group, ptr_e);
    rr_group_add(&mut (*svr___0).group, bptr_e);
    rr_list_append(&mut (*svr___0).announce, ptr_e);
    rr_list_append(&mut (*svr___0).services, ptr_e);
    pthread_mutex_unlock(&mut (*svr___0).data_lock);
    free(nlabel as *mut libc::c_void);
    free(inst_nlabel as *mut libc::c_void);
    write_pipe(
        (*svr___0).notify_pipe[1 as libc::c_int as usize],
        b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    return service;
}
pub unsafe extern "C" fn mdns_service_destroy(mut srv: *mut mdns_service) {
    if !(srv as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"srv != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1610 as libc::c_uint,
            b"mdns_service_destroy\0" as *const u8 as *const libc::c_char,
        );
    }
    rr_list_destroy((*srv).entries, 0 as libc::c_int as libc::c_char);
    free(srv as *mut libc::c_void);
}
pub unsafe extern "C" fn mdnsd_start() -> *mut mdnsd {
    let mut tid: pthread_t = 0;
    let mut attr: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    let mut server: *mut mdnsd = 0 as *mut mdnsd;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = malloc(::std::mem::size_of::<mdnsd>() as libc::c_ulong);
    server = tmp as *mut mdnsd;
    memset(
        server as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mdnsd>() as libc::c_ulong,
    );
    tmp___0 = create_pipe(((*server).notify_pipe).as_mut_ptr());
    if tmp___0 != 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"pipe(): %m\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"pipe(): %m\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        free(server as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut mdnsd;
    }
    (*server).sockfd = create_recv_sock();
    if (*server).sockfd < 0 as libc::c_int {
        match 3 as libc::c_int {
            3 => {
                warn(
                    b"unable to create recv socket\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            _ => {
                debug(
                    1 as libc::c_int,
                    b"unable to create recv socket\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        free(server as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut mdnsd;
    }
    pthread_mutex_init(
        &mut (*server).data_lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    pthread_attr_init(&mut attr);
    pthread_attr_setdetachstate(&mut attr, 1 as libc::c_int);
    tmp___1 = pthread_create(
        &mut tid as *mut pthread_t,
        &mut attr as *mut pthread_attr_t as *const pthread_attr_t,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut mdnsd) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        >(Some(main_loop as unsafe extern "C" fn(*mut mdnsd) -> ())),
        server as *mut libc::c_void,
    );
    if tmp___1 != 0 as libc::c_int {
        pthread_mutex_destroy(&mut (*server).data_lock);
        free(server as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut mdnsd;
    }
    return server;
}
pub unsafe extern "C" fn mdnsd_stop(mut s: *mut mdnsd) {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if !(s as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"s != NULL\0" as *const u8 as *const libc::c_char,
            b"tinysvcmdns.c\0" as *const u8 as *const libc::c_char,
            1651 as libc::c_uint,
            b"mdnsd_stop\0" as *const u8 as *const libc::c_char,
        );
    }
    ts.tv_sec = 0 as libc::c_int as __time_t;
    ts.tv_nsec = 500000000 as libc::c_int as __syscall_slong_t;
    (*s).stop_flag = 1 as libc::c_int;
    write_pipe(
        (*s).notify_pipe[1 as libc::c_int as usize],
        b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    while (*s).stop_flag != 2 as libc::c_int {
        nanosleep(
            &mut ts as *mut timespec as *const timespec,
            0 as *mut libc::c_void as *mut timespec,
        );
    }
    close_pipe((*s).notify_pipe[0 as libc::c_int as usize]);
    close_pipe((*s).notify_pipe[1 as libc::c_int as usize]);
    pthread_mutex_destroy(&mut (*s).data_lock);
    rr_group_destroy((*s).group);
    rr_list_destroy((*s).announce, 0 as libc::c_int as libc::c_char);
    rr_list_destroy((*s).services, 0 as libc::c_int as libc::c_char);
    if !((*s).hostname).is_null() {
        free((*s).hostname as *mut libc::c_void);
    }
    free(s as *mut libc::c_void);
}
pub static mut dev: *mut ao_device = 0 as *const libc::c_void as *mut libc::c_void
    as *mut ao_device;
unsafe extern "C" fn help___1() {
    printf(
        b"    -d driver           set the output driver\n    -o name=value       set an arbitrary ao option\n    -i id               shorthand for -o id=<id>\n    -n name             shorthand for -o dev=<name> -o dsp=<name>\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn init___1(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut driver: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut ao_opts: *mut ao_option = 0 as *mut ao_option;
    let mut opt: libc::c_int = 0;
    let mut mid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fmt: ao_sample_format = ao_sample_format {
        bits: 0,
        rate: 0,
        channels: 0,
        byte_format: 0,
        matrix: 0 as *mut libc::c_char,
    };
    let mut tmp___0: libc::c_int = 0;
    ao_initialize();
    tmp = ao_default_driver_id();
    driver = tmp;
    ao_opts = 0 as *mut libc::c_void as *mut ao_option;
    optind = 1 as libc::c_int;
    argv = argv.offset(-1);
    argc += 1;
    loop {
        opt = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"d:i:n:o:\0" as *const u8 as *const libc::c_char,
        );
        if !(opt > 0 as libc::c_int) {
            break;
        }
        match opt {
            100 => {
                driver = ao_driver_id(optarg as *const libc::c_char);
                if driver < 0 as libc::c_int {
                    die(
                        b"could not find ao driver %s\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        optarg,
                    );
                }
            }
            105 => {
                ao_append_option(
                    &mut ao_opts,
                    b"id\0" as *const u8 as *const libc::c_char,
                    optarg as *const libc::c_char,
                );
            }
            110 => {
                ao_append_option(
                    &mut ao_opts,
                    b"dev\0" as *const u8 as *const libc::c_char,
                    optarg as *const libc::c_char,
                );
                ao_append_option(
                    &mut ao_opts,
                    b"dsp\0" as *const u8 as *const libc::c_char,
                    optarg as *const libc::c_char,
                );
            }
            111 => {
                mid = strchr(optarg as *const libc::c_char, '=' as i32);
                if mid.is_null() {
                    die(
                        b"Expected an = in audio option %s\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        optarg,
                    );
                }
                *mid = 0 as libc::c_int as libc::c_char;
                ao_append_option(
                    &mut ao_opts,
                    optarg as *const libc::c_char,
                    mid.offset(1 as libc::c_int as isize) as *const libc::c_char,
                );
            }
            _ => {
                help___1();
                die(
                    b"Invalid audio option -%c specified\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    opt,
                );
            }
        }
    }
    if optind < argc {
        die(
            b"Invalid audio argument: %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *argv.offset(optind as isize),
        );
    }
    memset(
        &mut fmt as *mut ao_sample_format as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ao_sample_format>() as libc::c_ulong,
    );
    fmt.bits = 16 as libc::c_int;
    fmt.rate = 44100 as libc::c_int;
    fmt.channels = 2 as libc::c_int;
    fmt.byte_format = 4 as libc::c_int;
    dev = ao_open_live(driver, &mut fmt, ao_opts);
    if !dev.is_null() {
        tmp___0 = 0 as libc::c_int;
    } else {
        tmp___0 = 1 as libc::c_int;
    }
    return tmp___0;
}
unsafe extern "C" fn deinit___1() {
    if !dev.is_null() {
        ao_close(dev);
    }
    dev = 0 as *mut libc::c_void as *mut ao_device;
    ao_shutdown();
}
unsafe extern "C" fn start___1(mut sample_rate: libc::c_int) {
    if sample_rate != 44100 as libc::c_int {
        die(
            b"unexpected sample rate!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
unsafe extern "C" fn play___1(mut buf: *mut libc::c_short, mut samples: libc::c_int) {
    ao_play(dev, buf as *mut libc::c_char, (samples * 4 as libc::c_int) as uint_32);
}
unsafe extern "C" fn stop___1() {}
pub static mut audio_ao: audio_output = unsafe {
    {
        let mut init = __anonstruct_audio_output_780339900 {
            help: Some(help___1 as unsafe extern "C" fn() -> ()),
            name: b"ao\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            init: Some(
                init___1
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            deinit: Some(deinit___1 as unsafe extern "C" fn() -> ()),
            start: Some(start___1 as unsafe extern "C" fn(libc::c_int) -> ()),
            play: Some(
                play___1 as unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> (),
            ),
            stop: Some(stop___1 as unsafe extern "C" fn() -> ()),
            volume: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
            >(0 as *const libc::c_void as *mut libc::c_void),
        };
        init
    }
};
static mut pa_dev: *mut pa_simple = 0 as *const libc::c_void as *mut libc::c_void
    as *mut pa_simple;
static mut pa_error: libc::c_int = 0;
unsafe extern "C" fn help___2() {
    printf(
        b"    -a server           set the server name\n    -s sink             set the output sink\n    -n name             set the application name, as seen by PulseAudio\n                            defaults to the access point name\n\0"
            as *const u8 as *const libc::c_char,
    );
}
static mut ss: pa_sample_spec = {
    let mut init = pa_sample_spec {
        format: PA_SAMPLE_S16LE,
        rate: 44100 as libc::c_int as uint32_t,
        channels: 2 as libc::c_int as uint8_t,
    };
    init
};
unsafe extern "C" fn init___2(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pa_server: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pa_sink: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pa_appname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut opt: libc::c_int = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    pa_server = 0 as *mut libc::c_void as *mut libc::c_char;
    pa_sink = 0 as *mut libc::c_void as *mut libc::c_char;
    pa_appname = config.apname;
    optind = 1 as libc::c_int;
    argv = argv.offset(-1);
    argc += 1;
    loop {
        opt = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"a:s:n:\0" as *const u8 as *const libc::c_char,
        );
        if !(opt > 0 as libc::c_int) {
            break;
        }
        match opt {
            97 => {
                pa_server = optarg;
            }
            115 => {
                pa_sink = optarg;
            }
            110 => {
                pa_appname = optarg;
            }
            _ => {
                help___2();
                die(
                    b"Invalid audio option -%c specified\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    opt,
                );
            }
        }
    }
    if optind < argc {
        die(
            b"Invalid audio argument: %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *argv.offset(optind as isize),
        );
    }
    pa_dev = pa_simple_new(
        pa_server as *const libc::c_char,
        pa_appname as *const libc::c_char,
        PA_STREAM_PLAYBACK,
        pa_sink as *const libc::c_char,
        b"Shairport Stream\0" as *const u8 as *const libc::c_char,
        &ss,
        0 as *mut libc::c_void as *const pa_channel_map,
        0 as *mut libc::c_void as *const pa_buffer_attr,
        &mut pa_error,
    );
    if pa_dev.is_null() {
        tmp = pa_strerror(pa_error);
        die(
            b"Could not connect to pulseaudio server: %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            tmp,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn deinit___2() {
    if !pa_dev.is_null() {
        pa_simple_free(pa_dev);
    }
    pa_dev = 0 as *mut libc::c_void as *mut pa_simple;
}
unsafe extern "C" fn start___2(mut sample_rate: libc::c_int) {
    if sample_rate != 44100 as libc::c_int {
        die(
            b"unexpected sample rate!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
unsafe extern "C" fn play___2(mut buf: *mut libc::c_short, mut samples: libc::c_int) {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    tmp___0 = pa_simple_write(
        pa_dev,
        buf as *mut libc::c_char as *const libc::c_void,
        (samples as size_t).wrapping_mul(4 as libc::c_ulong),
        &mut pa_error,
    );
    if tmp___0 < 0 as libc::c_int {
        tmp = pa_strerror(pa_error);
        fprintf(
            stderr,
            b"audio_pulse.c: pa_simple_write() failed: %s\n\0" as *const u8
                as *const libc::c_char,
            tmp,
        );
    }
}
unsafe extern "C" fn stop___2() {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    tmp___0 = pa_simple_drain(pa_dev, &mut pa_error);
    if tmp___0 < 0 as libc::c_int {
        tmp = pa_strerror(pa_error);
        fprintf(
            stderr,
            b"audio_pulse.c: pa_simple_drain() failed: %s\n\0" as *const u8
                as *const libc::c_char,
            tmp,
        );
    }
}
pub static mut audio_pulse: audio_output = unsafe {
    {
        let mut init = __anonstruct_audio_output_780339900 {
            help: Some(help___2 as unsafe extern "C" fn() -> ()),
            name: b"pulse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            init: Some(
                init___2
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            deinit: Some(deinit___2 as unsafe extern "C" fn() -> ()),
            start: Some(start___2 as unsafe extern "C" fn(libc::c_int) -> ()),
            play: Some(
                play___2 as unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> (),
            ),
            stop: Some(stop___2 as unsafe extern "C" fn() -> ()),
            volume: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
            >(0 as *const libc::c_void as *mut libc::c_void),
        };
        init
    }
};
pub static mut audio_alsa: audio_output = unsafe {
    {
        let mut init = __anonstruct_audio_output_780339900 {
            help: Some(help___3 as unsafe extern "C" fn() -> ()),
            name: b"alsa\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            init: Some(
                init___3
                    as unsafe extern "C" fn(
                        libc::c_int,
                        *mut *mut libc::c_char,
                    ) -> libc::c_int,
            ),
            deinit: Some(deinit___3 as unsafe extern "C" fn() -> ()),
            start: Some(start___3 as unsafe extern "C" fn(libc::c_int) -> ()),
            play: Some(
                play___3 as unsafe extern "C" fn(*mut libc::c_short, libc::c_int) -> (),
            ),
            stop: Some(stop___3 as unsafe extern "C" fn() -> ()),
            volume: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
            >(0 as *const libc::c_void as *mut libc::c_void),
        };
        init
    }
};
static mut alsa_handle: *mut snd_pcm_t = 0 as *const libc::c_void as *mut libc::c_void
    as *mut snd_pcm_t;
static mut alsa_params: *mut snd_pcm_hw_params_t = 0 as *const libc::c_void
    as *mut libc::c_void as *mut snd_pcm_hw_params_t;
static mut alsa_mix_handle: *mut snd_mixer_t = 0 as *const libc::c_void
    as *mut libc::c_void as *mut snd_mixer_t;
static mut alsa_mix_elem: *mut snd_mixer_elem_t = 0 as *const libc::c_void
    as *mut libc::c_void as *mut snd_mixer_elem_t;
static mut alsa_mix_sid: *mut snd_mixer_selem_id_t = 0 as *const libc::c_void
    as *mut libc::c_void as *mut snd_mixer_selem_id_t;
static mut alsa_mix_minv: libc::c_long = 0;
static mut alsa_mix_range: libc::c_long = 0;
static mut alsa_out_dev: *mut libc::c_char = b"default\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut alsa_mix_dev: *mut libc::c_char = 0 as *const libc::c_void
    as *mut libc::c_void as *mut libc::c_char;
static mut alsa_mix_ctrl: *mut libc::c_char = b"Master\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut alsa_mix_index: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn help___3() {
    printf(
        b"    -d output-device    set the output device [default*|...]\n    -t mixer-type       set the mixer type [software*|hardware]\n    -m mixer-device     set the mixer device ['output-device'*|...]\n    -c mixer-control    set the mixer control [Master*|...]\n    -i mixer-index      set the mixer index [0*|...]\n    *) default option\n\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn init___3(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut hardware_mixer: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut ret: libc::c_int = 0;
    let mut alsa_mix_maxv: libc::c_long = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut _ = 0 as *mut _;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    hardware_mixer = 0 as libc::c_int;
    optind = 1 as libc::c_int;
    argv = argv.offset(-1);
    argc += 1;
    loop {
        opt = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"d:t:m:c:i:\0" as *const u8 as *const libc::c_char,
        );
        if !(opt > 0 as libc::c_int) {
            break;
        }
        match opt {
            100 => {
                alsa_out_dev = optarg;
            }
            116 => {
                tmp = strcmp(
                    optarg as *const libc::c_char,
                    b"hardware\0" as *const u8 as *const libc::c_char,
                );
                if tmp == 0 as libc::c_int {
                    hardware_mixer = 1 as libc::c_int;
                }
            }
            109 => {
                alsa_mix_dev = optarg;
            }
            99 => {
                alsa_mix_ctrl = optarg;
            }
            105 => {
                tmp___0 = strtol(
                    optarg as *const libc::c_char,
                    0 as *mut libc::c_void as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
                alsa_mix_index = tmp___0 as libc::c_int;
            }
            _ => {
                help___3();
                die(
                    b"Invalid audio option -%c specified\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    opt,
                );
            }
        }
    }
    if optind < argc {
        die(
            b"Invalid audio argument: %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            *argv.offset(optind as isize),
        );
    }
    if hardware_mixer == 0 {
        return 0 as libc::c_int;
    }
    if alsa_mix_dev as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        alsa_mix_dev = alsa_out_dev;
    }
    audio_alsa.volume = Some(volume___0 as unsafe extern "C" fn(libc::c_double) -> ());
    ret = 0 as libc::c_int;
    tmp___1 = snd_mixer_selem_id_sizeof();
    let mut fresh14 = ::std::vec::from_elem(0, tmp___1 as usize);
    tmp___2 = fresh14.as_mut_ptr();
    alsa_mix_sid = tmp___2 as *mut snd_mixer_selem_id_t;
    tmp___3 = snd_mixer_selem_id_sizeof();
    memset(alsa_mix_sid as *mut libc::c_void, 0 as libc::c_int, tmp___3);
    snd_mixer_selem_id_set_index(alsa_mix_sid, alsa_mix_index as libc::c_uint);
    snd_mixer_selem_id_set_name(alsa_mix_sid, alsa_mix_ctrl as *const libc::c_char);
    tmp___4 = snd_mixer_open(&mut alsa_mix_handle, 0 as libc::c_int);
    if tmp___4 < 0 as libc::c_int {
        die(
            b"Failed to open mixer\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    tmp___5 = snd_mixer_attach(alsa_mix_handle, alsa_mix_dev as *const libc::c_char);
    if tmp___5 < 0 as libc::c_int {
        die(
            b"Failed to attach mixer\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    tmp___6 = snd_mixer_selem_register(
        alsa_mix_handle,
        0 as *mut libc::c_void as *mut snd_mixer_selem_regopt,
        0 as *mut libc::c_void as *mut *mut snd_mixer_class_t,
    );
    if tmp___6 < 0 as libc::c_int {
        die(
            b"Failed to register mixer element\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    ret = snd_mixer_load(alsa_mix_handle);
    if ret < 0 as libc::c_int {
        die(
            b"Failed to load mixer element\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    alsa_mix_elem = snd_mixer_find_selem(
        alsa_mix_handle,
        alsa_mix_sid as *const snd_mixer_selem_id_t,
    );
    if alsa_mix_elem.is_null() {
        die(
            b"Failed to find mixer element\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    snd_mixer_selem_get_playback_volume_range(
        alsa_mix_elem,
        &mut alsa_mix_minv,
        &mut alsa_mix_maxv,
    );
    alsa_mix_range = alsa_mix_maxv - alsa_mix_minv;
    return 0 as libc::c_int;
}
unsafe extern "C" fn deinit___3() {
    stop___3();
    if !alsa_mix_handle.is_null() {
        snd_mixer_close(alsa_mix_handle);
    }
}
unsafe extern "C" fn start___3(mut sample_rate: libc::c_int) {
    let mut ret: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut frames: snd_pcm_uframes_t = 0;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut _ = 0 as *mut _;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    if sample_rate != 44100 as libc::c_int {
        die(
            b"Unexpected sample rate!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    dir = 0 as libc::c_int;
    frames = 64 as libc::c_int as snd_pcm_uframes_t;
    ret = snd_pcm_open(
        &mut alsa_handle,
        alsa_out_dev as *const libc::c_char,
        SND_PCM_STREAM_PLAYBACK,
        0 as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        tmp = snd_strerror(ret);
        die(
            b"Alsa initialization failed: unable to open pcm device: %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            tmp,
        );
    }
    tmp___0 = snd_pcm_hw_params_sizeof();
    let mut fresh15 = ::std::vec::from_elem(0, tmp___0 as usize);
    tmp___1 = fresh15.as_mut_ptr();
    alsa_params = tmp___1 as *mut snd_pcm_hw_params_t;
    tmp___2 = snd_pcm_hw_params_sizeof();
    memset(alsa_params as *mut libc::c_void, 0 as libc::c_int, tmp___2);
    snd_pcm_hw_params_any(alsa_handle, alsa_params);
    snd_pcm_hw_params_set_access(
        alsa_handle,
        alsa_params,
        SND_PCM_ACCESS_RW_INTERLEAVED,
    );
    snd_pcm_hw_params_set_format(alsa_handle, alsa_params, SND_PCM_FORMAT_S16_LE);
    snd_pcm_hw_params_set_channels(alsa_handle, alsa_params, 2 as libc::c_uint);
    snd_pcm_hw_params_set_rate_near(
        alsa_handle,
        alsa_params,
        &mut sample_rate as *mut libc::c_int as *mut libc::c_uint,
        &mut dir,
    );
    snd_pcm_hw_params_set_period_size_near(
        alsa_handle,
        alsa_params,
        &mut frames,
        &mut dir,
    );
    ret = snd_pcm_hw_params(alsa_handle, alsa_params);
    if ret < 0 as libc::c_int {
        tmp___3 = snd_strerror(ret);
        die(
            b"unable to set hw parameters: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            tmp___3,
        );
    }
}
unsafe extern "C" fn play___3(mut buf: *mut libc::c_short, mut samples: libc::c_int) {
    let mut err: libc::c_int = 0;
    let mut tmp: snd_pcm_sframes_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    tmp = snd_pcm_writei(
        alsa_handle,
        buf as *mut libc::c_char as *const libc::c_void,
        samples as snd_pcm_uframes_t,
    );
    err = tmp as libc::c_int;
    if err < 0 as libc::c_int {
        err = snd_pcm_recover(alsa_handle, err, 0 as libc::c_int);
    }
    if err < 0 as libc::c_int {
        tmp___0 = snd_strerror(err);
        die(
            b"Failed to write to PCM device: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            tmp___0,
        );
    }
}
unsafe extern "C" fn stop___3() {
    if !alsa_handle.is_null() {
        snd_pcm_drain(alsa_handle);
        snd_pcm_close(alsa_handle);
        alsa_handle = 0 as *mut libc::c_void as *mut snd_pcm_t;
    }
}
unsafe extern "C" fn volume___0(mut vol: libc::c_double) {
    let mut alsa_volume: libc::c_long = 0;
    let mut tmp: libc::c_int = 0;
    alsa_volume = (vol * alsa_mix_range as libc::c_double
        + alsa_mix_minv as libc::c_double) as libc::c_long;
    tmp = snd_mixer_selem_set_playback_volume_all(alsa_mix_elem, alsa_volume);
    if tmp != 0 as libc::c_int {
        die(
            b"Failed to set playback volume\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
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
