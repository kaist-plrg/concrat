use ::libc;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bignum_ctx;
    pub type ec_group_st;
    pub type ec_point_st;
    pub type ec_key_st;
    pub type _vg_ocl_context_s;
    pub type real_pcre;
    pub type bn_blinding_st;
    pub type engine_st;
    pub type ASN1_VALUE_st;
    pub type evp_pkey_ctx_st;
    pub type evp_pkey_asn1_method_st;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn CRYPTO_free(ptr: *mut libc::c_void);
    fn BN_bn2hex(a: *const BIGNUM) -> *mut libc::c_char;
    fn EC_POINT_free(point: *mut EC_POINT);
    fn EC_POINT_point2hex(
        _: *const EC_GROUP,
        _: *const EC_POINT,
        form: point_conversion_form_t,
        _: *mut BN_CTX,
    ) -> *mut libc::c_char;
    fn EC_POINT_hex2point(
        _: *const EC_GROUP,
        _: *const libc::c_char,
        _: *mut EC_POINT,
        _: *mut BN_CTX,
    ) -> *mut EC_POINT;
    fn EC_POINT_cmp(
        group: *const EC_GROUP,
        a: *const EC_POINT,
        b: *const EC_POINT,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_KEY_free(key: *mut EC_KEY);
    fn EC_KEY_get0_group(key: *const EC_KEY) -> *const EC_GROUP;
    fn EC_KEY_get0_private_key(key: *const EC_KEY) -> *const BIGNUM;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn curl_easy_strerror(_: CURLcode) -> *const libc::c_char;
    fn curl_easy_init() -> *mut libc::c_void;
    fn curl_easy_setopt(curl: *mut libc::c_void, option: CURLoption, _: ...) -> CURLcode;
    fn curl_easy_perform(curl: *mut libc::c_void) -> CURLcode;
    fn curl_easy_cleanup(curl: *mut libc::c_void);
    static mut optarg: *mut libc::c_char;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    fn vg_ocl_context_new(
        vcp: *mut vg_context_t,
        platformidx: libc::c_int,
        deviceidx: libc::c_int,
        safe_mode: libc::c_int,
        verify: libc::c_int,
        worksize: libc::c_int,
        nthreads: libc::c_int,
        nrows: libc::c_int,
        ncols: libc::c_int,
        invsize: libc::c_int,
    ) -> *mut vg_ocl_context_t;
    fn vg_ocl_context_new_from_devstr(
        vcp: *mut vg_context_t,
        devstr: *const libc::c_char,
        safemode: libc::c_int,
        verify: libc::c_int,
    ) -> *mut vg_ocl_context_t;
    fn vg_ocl_enumerate_devices();
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
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
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn SHA256(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn RIPEMD160(
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn BN_value_one() -> *const BIGNUM;
    fn BN_CTX_new() -> *mut BN_CTX;
    fn BN_CTX_free(c: *mut BN_CTX);
    fn BN_new() -> *mut BIGNUM;
    fn BN_init(_: *mut BIGNUM);
    fn BN_clear_free(a: *mut BIGNUM);
    fn BN_copy(a: *mut BIGNUM, b: *const BIGNUM) -> *mut BIGNUM;
    fn BN_bin2bn(
        s: *const libc::c_uchar,
        len: libc::c_int,
        ret: *mut BIGNUM,
    ) -> *mut BIGNUM;
    fn BN_bn2bin(a: *const BIGNUM, to: *mut libc::c_uchar) -> libc::c_int;
    fn BN_sub(r: *mut BIGNUM, a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
    fn BN_add(r: *mut BIGNUM, a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
    fn BN_mul(
        r: *mut BIGNUM,
        a: *const BIGNUM,
        b: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn BN_div(
        dv: *mut BIGNUM,
        rem: *mut BIGNUM,
        m: *const BIGNUM,
        d: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn BN_set_word(a: *mut BIGNUM, w: libc::c_ulong) -> libc::c_int;
    fn BN_get_word(a: *const BIGNUM) -> libc::c_ulong;
    fn BN_cmp(a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
    fn BN_free(a: *mut BIGNUM);
    fn BN_lshift(r: *mut BIGNUM, a: *const BIGNUM, n: libc::c_int) -> libc::c_int;
    fn BN_exp(
        r: *mut BIGNUM,
        a: *const BIGNUM,
        p: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn BN_clear(a: *mut BIGNUM);
    fn BN_set_bit(a: *mut BIGNUM, n: libc::c_int) -> libc::c_int;
    fn BN_bn2dec(a: *const BIGNUM) -> *mut libc::c_char;
    fn EC_POINT_new(group: *const EC_GROUP) -> *mut EC_POINT;
    fn EC_POINT_copy(dst: *mut EC_POINT, src: *const EC_POINT) -> libc::c_int;
    fn EC_POINT_point2oct(
        group: *const EC_GROUP,
        p: *const EC_POINT,
        form: point_conversion_form_t,
        buf: *mut libc::c_uchar,
        len: size_t,
        ctx: *mut BN_CTX,
    ) -> size_t;
    fn EC_POINT_add(
        group: *const EC_GROUP,
        r: *mut EC_POINT,
        a: *const EC_POINT,
        b: *const EC_POINT,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_KEY_new_by_curve_name(nid: libc::c_int) -> *mut EC_KEY;
    fn EC_KEY_get0_public_key(key: *const EC_KEY) -> *const EC_POINT;
    fn EC_KEY_precompute_mult(key: *mut EC_KEY, ctx: *mut BN_CTX) -> libc::c_int;
    fn EC_KEY_check_key(key: *const EC_KEY) -> libc::c_int;
    fn i2d_ECPrivateKey(key: *mut EC_KEY, out: *mut *mut libc::c_uchar) -> libc::c_int;
    fn i2o_ECPublicKey(key: *mut EC_KEY, out: *mut *mut libc::c_uchar) -> libc::c_int;
    static mut pcre_free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    fn pcre_compile(
        _: *const libc::c_char,
        _: libc::c_int,
        _: *mut *const libc::c_char,
        _: *mut libc::c_int,
        _: *const libc::c_uchar,
    ) -> *mut pcre;
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
    fn pcre_study(
        _: *const pcre,
        _: libc::c_int,
        _: *mut *const libc::c_char,
    ) -> *mut pcre_extra;
    fn __errno_location() -> *mut libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn OPENSSL_cleanse(ptr: *mut libc::c_void, len: size_t);
    fn BN_num_bits(a: *const BIGNUM) -> libc::c_int;
    fn BIO_new(type_0: *mut BIO_METHOD) -> *mut BIO;
    fn BIO_free(a: *mut BIO) -> libc::c_int;
    fn BIO_ctrl(
        bp: *mut BIO,
        cmd: libc::c_int,
        larg: libc::c_long,
        parg: *mut libc::c_void,
    ) -> libc::c_long;
    fn BIO_s_mem() -> *mut BIO_METHOD;
    fn BIO_new_mem_buf(buf: *mut libc::c_void, len: libc::c_int) -> *mut BIO;
    fn EVP_read_pw_string(
        buf: *mut libc::c_char,
        length: libc::c_int,
        prompt: *const libc::c_char,
        verify: libc::c_int,
    ) -> libc::c_int;
    fn EVP_CipherInit(
        ctx: *mut EVP_CIPHER_CTX,
        cipher: *const EVP_CIPHER,
        key: *const libc::c_uchar,
        iv: *const libc::c_uchar,
        enc: libc::c_int,
    ) -> libc::c_int;
    fn EVP_CipherUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut libc::c_uchar,
        outl: *mut libc::c_int,
        in_0: *const libc::c_uchar,
        inl: libc::c_int,
    ) -> libc::c_int;
    fn EVP_CipherFinal(
        ctx: *mut EVP_CIPHER_CTX,
        outm: *mut libc::c_uchar,
        outl: *mut libc::c_int,
    ) -> libc::c_int;
    fn EVP_CIPHER_CTX_new() -> *mut EVP_CIPHER_CTX;
    fn EVP_CIPHER_CTX_free(a: *mut EVP_CIPHER_CTX);
    fn EVP_CIPHER_CTX_set_padding(
        c: *mut EVP_CIPHER_CTX,
        pad: libc::c_int,
    ) -> libc::c_int;
    fn EVP_sha256() -> *const EVP_MD;
    fn EVP_aes_256_cbc() -> *const EVP_CIPHER;
    fn EVP_PKEY_set1_EC_KEY(pkey: *mut EVP_PKEY, key: *mut ec_key_st) -> libc::c_int;
    fn EVP_PKEY_get1_EC_KEY(pkey: *mut EVP_PKEY) -> *mut ec_key_st;
    fn EVP_PKEY_new() -> *mut EVP_PKEY;
    fn EVP_PKEY_free(pkey: *mut EVP_PKEY);
    fn PKCS5_PBKDF2_HMAC(
        pass: *const libc::c_char,
        passlen: libc::c_int,
        salt: *const libc::c_uchar,
        saltlen: libc::c_int,
        iter: libc::c_int,
        digest: *const EVP_MD,
        keylen: libc::c_int,
        out: *mut libc::c_uchar,
    ) -> libc::c_int;
    fn HMAC(
        evp_md: *const EVP_MD,
        key: *const libc::c_void,
        key_len: libc::c_int,
        d: *const libc::c_uchar,
        n: size_t,
        md: *mut libc::c_uchar,
        md_len: *mut libc::c_uint,
    ) -> *mut libc::c_uchar;
    fn RAND_bytes(buf: *mut libc::c_uchar, num: libc::c_int) -> libc::c_int;
    fn EC_GROUP_cmp(
        a: *const EC_GROUP,
        b: *const EC_GROUP,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_POINT_mul(
        group: *const EC_GROUP,
        r: *mut EC_POINT,
        n: *const BIGNUM,
        q: *const EC_POINT,
        m: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_KEY_copy(dst: *mut EC_KEY, src: *const EC_KEY) -> *mut EC_KEY;
    fn EC_KEY_dup(src: *const EC_KEY) -> *mut EC_KEY;
    fn EC_KEY_set_private_key(key: *mut EC_KEY, prv: *const BIGNUM) -> libc::c_int;
    fn EC_KEY_set_public_key(key: *mut EC_KEY, pub_0: *const EC_POINT) -> libc::c_int;
    fn X509_SIG_free(a: *mut X509_SIG);
    fn PKCS8_PRIV_KEY_INFO_free(a: *mut PKCS8_PRIV_KEY_INFO);
    fn EVP_PKCS82PKEY(p8: *mut PKCS8_PRIV_KEY_INFO) -> *mut EVP_PKEY;
    fn EVP_PKEY2PKCS8(pkey: *mut EVP_PKEY) -> *mut PKCS8_PRIV_KEY_INFO;
    fn PEM_read_bio_PKCS8(
        bp: *mut BIO,
        x: *mut *mut X509_SIG,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut X509_SIG;
    fn PEM_write_bio_PKCS8(bp: *mut BIO, x: *mut X509_SIG) -> libc::c_int;
    fn PEM_read_bio_PKCS8_PRIV_KEY_INFO(
        bp: *mut BIO,
        x: *mut *mut PKCS8_PRIV_KEY_INFO,
        cb: Option::<pem_password_cb>,
        u: *mut libc::c_void,
    ) -> *mut PKCS8_PRIV_KEY_INFO;
    fn PEM_write_bio_PKCS8_PRIV_KEY_INFO(
        bp: *mut BIO,
        x: *mut PKCS8_PRIV_KEY_INFO,
    ) -> libc::c_int;
    fn PKCS8_decrypt(
        p8: *mut X509_SIG,
        pass: *const libc::c_char,
        passlen: libc::c_int,
    ) -> *mut PKCS8_PRIV_KEY_INFO;
    fn PKCS8_encrypt(
        pbe_nid: libc::c_int,
        cipher: *const EVP_CIPHER,
        pass: *const libc::c_char,
        passlen: libc::c_int,
        salt: *mut libc::c_uchar,
        saltlen: libc::c_int,
        iter: libc::c_int,
        p8: *mut PKCS8_PRIV_KEY_INFO,
    ) -> *mut X509_SIG;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub union __anonunion____missing_field_name_622077928 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_119281978 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_119281977 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_119281978,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_622077928,
    pub __annonCompField2: __anonunion____missing_field_name_119281977,
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bignum_st {
    pub d: *mut libc::c_ulong,
    pub top: libc::c_int,
    pub dmax: libc::c_int,
    pub neg: libc::c_int,
    pub flags: libc::c_int,
}
pub type BIGNUM = bignum_st;
pub type BN_CTX = bignum_ctx;
pub type __anonenum_point_conversion_form_t_134566212 = libc::c_uint;
pub const POINT_CONVERSION_HYBRID: __anonenum_point_conversion_form_t_134566212 = 6;
pub const POINT_CONVERSION_UNCOMPRESSED: __anonenum_point_conversion_form_t_134566212 = 4;
pub const POINT_CONVERSION_COMPRESSED: __anonenum_point_conversion_form_t_134566212 = 2;
pub type point_conversion_form_t = __anonenum_point_conversion_form_t_134566212;
pub type EC_GROUP = ec_group_st;
pub type EC_POINT = ec_point_st;
pub type EC_KEY = ec_key_st;
pub type CURL = ();
pub type __anonenum_CURLcode_1040171027 = libc::c_uint;
pub const CURL_LAST: __anonenum_CURLcode_1040171027 = 96;
pub const CURLE_HTTP3: __anonenum_CURLcode_1040171027 = 95;
pub const CURLE_AUTH_ERROR: __anonenum_CURLcode_1040171027 = 94;
pub const CURLE_RECURSIVE_API_CALL: __anonenum_CURLcode_1040171027 = 93;
pub const CURLE_HTTP2_STREAM: __anonenum_CURLcode_1040171027 = 92;
pub const CURLE_SSL_INVALIDCERTSTATUS: __anonenum_CURLcode_1040171027 = 91;
pub const CURLE_SSL_PINNEDPUBKEYNOTMATCH: __anonenum_CURLcode_1040171027 = 90;
pub const CURLE_NO_CONNECTION_AVAILABLE: __anonenum_CURLcode_1040171027 = 89;
pub const CURLE_CHUNK_FAILED: __anonenum_CURLcode_1040171027 = 88;
pub const CURLE_FTP_BAD_FILE_LIST: __anonenum_CURLcode_1040171027 = 87;
pub const CURLE_RTSP_SESSION_ERROR: __anonenum_CURLcode_1040171027 = 86;
pub const CURLE_RTSP_CSEQ_ERROR: __anonenum_CURLcode_1040171027 = 85;
pub const CURLE_FTP_PRET_FAILED: __anonenum_CURLcode_1040171027 = 84;
pub const CURLE_SSL_ISSUER_ERROR: __anonenum_CURLcode_1040171027 = 83;
pub const CURLE_SSL_CRL_BADFILE: __anonenum_CURLcode_1040171027 = 82;
pub const CURLE_AGAIN: __anonenum_CURLcode_1040171027 = 81;
pub const CURLE_SSL_SHUTDOWN_FAILED: __anonenum_CURLcode_1040171027 = 80;
pub const CURLE_SSH: __anonenum_CURLcode_1040171027 = 79;
pub const CURLE_REMOTE_FILE_NOT_FOUND: __anonenum_CURLcode_1040171027 = 78;
pub const CURLE_SSL_CACERT_BADFILE: __anonenum_CURLcode_1040171027 = 77;
pub const CURLE_CONV_REQD: __anonenum_CURLcode_1040171027 = 76;
pub const CURLE_CONV_FAILED: __anonenum_CURLcode_1040171027 = 75;
pub const CURLE_TFTP_NOSUCHUSER: __anonenum_CURLcode_1040171027 = 74;
pub const CURLE_REMOTE_FILE_EXISTS: __anonenum_CURLcode_1040171027 = 73;
pub const CURLE_TFTP_UNKNOWNID: __anonenum_CURLcode_1040171027 = 72;
pub const CURLE_TFTP_ILLEGAL: __anonenum_CURLcode_1040171027 = 71;
pub const CURLE_REMOTE_DISK_FULL: __anonenum_CURLcode_1040171027 = 70;
pub const CURLE_TFTP_PERM: __anonenum_CURLcode_1040171027 = 69;
pub const CURLE_TFTP_NOTFOUND: __anonenum_CURLcode_1040171027 = 68;
pub const CURLE_LOGIN_DENIED: __anonenum_CURLcode_1040171027 = 67;
pub const CURLE_SSL_ENGINE_INITFAILED: __anonenum_CURLcode_1040171027 = 66;
pub const CURLE_SEND_FAIL_REWIND: __anonenum_CURLcode_1040171027 = 65;
pub const CURLE_USE_SSL_FAILED: __anonenum_CURLcode_1040171027 = 64;
pub const CURLE_FILESIZE_EXCEEDED: __anonenum_CURLcode_1040171027 = 63;
pub const CURLE_LDAP_INVALID_URL: __anonenum_CURLcode_1040171027 = 62;
pub const CURLE_BAD_CONTENT_ENCODING: __anonenum_CURLcode_1040171027 = 61;
pub const CURLE_PEER_FAILED_VERIFICATION: __anonenum_CURLcode_1040171027 = 60;
pub const CURLE_SSL_CIPHER: __anonenum_CURLcode_1040171027 = 59;
pub const CURLE_SSL_CERTPROBLEM: __anonenum_CURLcode_1040171027 = 58;
pub const CURLE_OBSOLETE57: __anonenum_CURLcode_1040171027 = 57;
pub const CURLE_RECV_ERROR: __anonenum_CURLcode_1040171027 = 56;
pub const CURLE_SEND_ERROR: __anonenum_CURLcode_1040171027 = 55;
pub const CURLE_SSL_ENGINE_SETFAILED: __anonenum_CURLcode_1040171027 = 54;
pub const CURLE_SSL_ENGINE_NOTFOUND: __anonenum_CURLcode_1040171027 = 53;
pub const CURLE_GOT_NOTHING: __anonenum_CURLcode_1040171027 = 52;
pub const CURLE_OBSOLETE51: __anonenum_CURLcode_1040171027 = 51;
pub const CURLE_OBSOLETE50: __anonenum_CURLcode_1040171027 = 50;
pub const CURLE_TELNET_OPTION_SYNTAX: __anonenum_CURLcode_1040171027 = 49;
pub const CURLE_UNKNOWN_OPTION: __anonenum_CURLcode_1040171027 = 48;
pub const CURLE_TOO_MANY_REDIRECTS: __anonenum_CURLcode_1040171027 = 47;
pub const CURLE_OBSOLETE46: __anonenum_CURLcode_1040171027 = 46;
pub const CURLE_INTERFACE_FAILED: __anonenum_CURLcode_1040171027 = 45;
pub const CURLE_OBSOLETE44: __anonenum_CURLcode_1040171027 = 44;
pub const CURLE_BAD_FUNCTION_ARGUMENT: __anonenum_CURLcode_1040171027 = 43;
pub const CURLE_ABORTED_BY_CALLBACK: __anonenum_CURLcode_1040171027 = 42;
pub const CURLE_FUNCTION_NOT_FOUND: __anonenum_CURLcode_1040171027 = 41;
pub const CURLE_OBSOLETE40: __anonenum_CURLcode_1040171027 = 40;
pub const CURLE_LDAP_SEARCH_FAILED: __anonenum_CURLcode_1040171027 = 39;
pub const CURLE_LDAP_CANNOT_BIND: __anonenum_CURLcode_1040171027 = 38;
pub const CURLE_FILE_COULDNT_READ_FILE: __anonenum_CURLcode_1040171027 = 37;
pub const CURLE_BAD_DOWNLOAD_RESUME: __anonenum_CURLcode_1040171027 = 36;
pub const CURLE_SSL_CONNECT_ERROR: __anonenum_CURLcode_1040171027 = 35;
pub const CURLE_HTTP_POST_ERROR: __anonenum_CURLcode_1040171027 = 34;
pub const CURLE_RANGE_ERROR: __anonenum_CURLcode_1040171027 = 33;
pub const CURLE_OBSOLETE32: __anonenum_CURLcode_1040171027 = 32;
pub const CURLE_FTP_COULDNT_USE_REST: __anonenum_CURLcode_1040171027 = 31;
pub const CURLE_FTP_PORT_FAILED: __anonenum_CURLcode_1040171027 = 30;
pub const CURLE_OBSOLETE29: __anonenum_CURLcode_1040171027 = 29;
pub const CURLE_OPERATION_TIMEDOUT: __anonenum_CURLcode_1040171027 = 28;
pub const CURLE_OUT_OF_MEMORY: __anonenum_CURLcode_1040171027 = 27;
pub const CURLE_READ_ERROR: __anonenum_CURLcode_1040171027 = 26;
pub const CURLE_UPLOAD_FAILED: __anonenum_CURLcode_1040171027 = 25;
pub const CURLE_OBSOLETE24: __anonenum_CURLcode_1040171027 = 24;
pub const CURLE_WRITE_ERROR: __anonenum_CURLcode_1040171027 = 23;
pub const CURLE_HTTP_RETURNED_ERROR: __anonenum_CURLcode_1040171027 = 22;
pub const CURLE_QUOTE_ERROR: __anonenum_CURLcode_1040171027 = 21;
pub const CURLE_OBSOLETE20: __anonenum_CURLcode_1040171027 = 20;
pub const CURLE_FTP_COULDNT_RETR_FILE: __anonenum_CURLcode_1040171027 = 19;
pub const CURLE_PARTIAL_FILE: __anonenum_CURLcode_1040171027 = 18;
pub const CURLE_FTP_COULDNT_SET_TYPE: __anonenum_CURLcode_1040171027 = 17;
pub const CURLE_HTTP2: __anonenum_CURLcode_1040171027 = 16;
pub const CURLE_FTP_CANT_GET_HOST: __anonenum_CURLcode_1040171027 = 15;
pub const CURLE_FTP_WEIRD_227_FORMAT: __anonenum_CURLcode_1040171027 = 14;
pub const CURLE_FTP_WEIRD_PASV_REPLY: __anonenum_CURLcode_1040171027 = 13;
pub const CURLE_FTP_ACCEPT_TIMEOUT: __anonenum_CURLcode_1040171027 = 12;
pub const CURLE_FTP_WEIRD_PASS_REPLY: __anonenum_CURLcode_1040171027 = 11;
pub const CURLE_FTP_ACCEPT_FAILED: __anonenum_CURLcode_1040171027 = 10;
pub const CURLE_REMOTE_ACCESS_DENIED: __anonenum_CURLcode_1040171027 = 9;
pub const CURLE_WEIRD_SERVER_REPLY: __anonenum_CURLcode_1040171027 = 8;
pub const CURLE_COULDNT_CONNECT: __anonenum_CURLcode_1040171027 = 7;
pub const CURLE_COULDNT_RESOLVE_HOST: __anonenum_CURLcode_1040171027 = 6;
pub const CURLE_COULDNT_RESOLVE_PROXY: __anonenum_CURLcode_1040171027 = 5;
pub const CURLE_NOT_BUILT_IN: __anonenum_CURLcode_1040171027 = 4;
pub const CURLE_URL_MALFORMAT: __anonenum_CURLcode_1040171027 = 3;
pub const CURLE_FAILED_INIT: __anonenum_CURLcode_1040171027 = 2;
pub const CURLE_UNSUPPORTED_PROTOCOL: __anonenum_CURLcode_1040171027 = 1;
pub const CURLE_OK: __anonenum_CURLcode_1040171027 = 0;
pub type CURLcode = __anonenum_CURLcode_1040171027;
pub type __anonenum_CURLoption_714703655 = libc::c_uint;
pub const CURLOPT_LASTENTRY: __anonenum_CURLoption_714703655 = 10290;
pub const CURLOPT_SASL_AUTHZID: __anonenum_CURLoption_714703655 = 10289;
pub const CURLOPT_MAXAGE_CONN: __anonenum_CURLoption_714703655 = 288;
pub const CURLOPT_ALTSVC: __anonenum_CURLoption_714703655 = 10287;
pub const CURLOPT_ALTSVC_CTRL: __anonenum_CURLoption_714703655 = 286;
pub const CURLOPT_HTTP09_ALLOWED: __anonenum_CURLoption_714703655 = 285;
pub const CURLOPT_TRAILERDATA: __anonenum_CURLoption_714703655 = 10284;
pub const CURLOPT_TRAILERFUNCTION: __anonenum_CURLoption_714703655 = 20283;
pub const CURLOPT_CURLU: __anonenum_CURLoption_714703655 = 10282;
pub const CURLOPT_UPKEEP_INTERVAL_MS: __anonenum_CURLoption_714703655 = 281;
pub const CURLOPT_UPLOAD_BUFFERSIZE: __anonenum_CURLoption_714703655 = 280;
pub const CURLOPT_DOH_URL: __anonenum_CURLoption_714703655 = 10279;
pub const CURLOPT_DISALLOW_USERNAME_IN_URL: __anonenum_CURLoption_714703655 = 278;
pub const CURLOPT_PROXY_TLS13_CIPHERS: __anonenum_CURLoption_714703655 = 10277;
pub const CURLOPT_TLS13_CIPHERS: __anonenum_CURLoption_714703655 = 10276;
pub const CURLOPT_DNS_SHUFFLE_ADDRESSES: __anonenum_CURLoption_714703655 = 275;
pub const CURLOPT_HAPROXYPROTOCOL: __anonenum_CURLoption_714703655 = 274;
pub const CURLOPT_RESOLVER_START_DATA: __anonenum_CURLoption_714703655 = 10273;
pub const CURLOPT_RESOLVER_START_FUNCTION: __anonenum_CURLoption_714703655 = 20272;
pub const CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS: __anonenum_CURLoption_714703655 = 271;
pub const CURLOPT_TIMEVALUE_LARGE: __anonenum_CURLoption_714703655 = 30270;
pub const CURLOPT_MIMEPOST: __anonenum_CURLoption_714703655 = 10269;
pub const CURLOPT_SSH_COMPRESSION: __anonenum_CURLoption_714703655 = 268;
pub const CURLOPT_SOCKS5_AUTH: __anonenum_CURLoption_714703655 = 267;
pub const CURLOPT_REQUEST_TARGET: __anonenum_CURLoption_714703655 = 10266;
pub const CURLOPT_SUPPRESS_CONNECT_HEADERS: __anonenum_CURLoption_714703655 = 265;
pub const CURLOPT_ABSTRACT_UNIX_SOCKET: __anonenum_CURLoption_714703655 = 10264;
pub const CURLOPT_PROXY_PINNEDPUBLICKEY: __anonenum_CURLoption_714703655 = 10263;
pub const CURLOPT_PRE_PROXY: __anonenum_CURLoption_714703655 = 10262;
pub const CURLOPT_PROXY_SSL_OPTIONS: __anonenum_CURLoption_714703655 = 261;
pub const CURLOPT_PROXY_CRLFILE: __anonenum_CURLoption_714703655 = 10260;
pub const CURLOPT_PROXY_SSL_CIPHER_LIST: __anonenum_CURLoption_714703655 = 10259;
pub const CURLOPT_PROXY_KEYPASSWD: __anonenum_CURLoption_714703655 = 10258;
pub const CURLOPT_PROXY_SSLKEYTYPE: __anonenum_CURLoption_714703655 = 10257;
pub const CURLOPT_PROXY_SSLKEY: __anonenum_CURLoption_714703655 = 10256;
pub const CURLOPT_PROXY_SSLCERTTYPE: __anonenum_CURLoption_714703655 = 10255;
pub const CURLOPT_PROXY_SSLCERT: __anonenum_CURLoption_714703655 = 10254;
pub const CURLOPT_PROXY_TLSAUTH_TYPE: __anonenum_CURLoption_714703655 = 10253;
pub const CURLOPT_PROXY_TLSAUTH_PASSWORD: __anonenum_CURLoption_714703655 = 10252;
pub const CURLOPT_PROXY_TLSAUTH_USERNAME: __anonenum_CURLoption_714703655 = 10251;
pub const CURLOPT_PROXY_SSLVERSION: __anonenum_CURLoption_714703655 = 250;
pub const CURLOPT_PROXY_SSL_VERIFYHOST: __anonenum_CURLoption_714703655 = 249;
pub const CURLOPT_PROXY_SSL_VERIFYPEER: __anonenum_CURLoption_714703655 = 248;
pub const CURLOPT_PROXY_CAPATH: __anonenum_CURLoption_714703655 = 10247;
pub const CURLOPT_PROXY_CAINFO: __anonenum_CURLoption_714703655 = 10246;
pub const CURLOPT_KEEP_SENDING_ON_ERROR: __anonenum_CURLoption_714703655 = 245;
pub const CURLOPT_TCP_FASTOPEN: __anonenum_CURLoption_714703655 = 244;
pub const CURLOPT_CONNECT_TO: __anonenum_CURLoption_714703655 = 10243;
pub const CURLOPT_TFTP_NO_OPTIONS: __anonenum_CURLoption_714703655 = 242;
pub const CURLOPT_STREAM_DEPENDS_E: __anonenum_CURLoption_714703655 = 10241;
pub const CURLOPT_STREAM_DEPENDS: __anonenum_CURLoption_714703655 = 10240;
pub const CURLOPT_STREAM_WEIGHT: __anonenum_CURLoption_714703655 = 239;
pub const CURLOPT_DEFAULT_PROTOCOL: __anonenum_CURLoption_714703655 = 10238;
pub const CURLOPT_PIPEWAIT: __anonenum_CURLoption_714703655 = 237;
pub const CURLOPT_SERVICE_NAME: __anonenum_CURLoption_714703655 = 10236;
pub const CURLOPT_PROXY_SERVICE_NAME: __anonenum_CURLoption_714703655 = 10235;
pub const CURLOPT_PATH_AS_IS: __anonenum_CURLoption_714703655 = 234;
pub const CURLOPT_SSL_FALSESTART: __anonenum_CURLoption_714703655 = 233;
pub const CURLOPT_SSL_VERIFYSTATUS: __anonenum_CURLoption_714703655 = 232;
pub const CURLOPT_UNIX_SOCKET_PATH: __anonenum_CURLoption_714703655 = 10231;
pub const CURLOPT_PINNEDPUBLICKEY: __anonenum_CURLoption_714703655 = 10230;
pub const CURLOPT_HEADEROPT: __anonenum_CURLoption_714703655 = 229;
pub const CURLOPT_PROXYHEADER: __anonenum_CURLoption_714703655 = 10228;
pub const CURLOPT_EXPECT_100_TIMEOUT_MS: __anonenum_CURLoption_714703655 = 227;
pub const CURLOPT_SSL_ENABLE_ALPN: __anonenum_CURLoption_714703655 = 226;
pub const CURLOPT_SSL_ENABLE_NPN: __anonenum_CURLoption_714703655 = 225;
pub const CURLOPT_LOGIN_OPTIONS: __anonenum_CURLoption_714703655 = 10224;
pub const CURLOPT_DNS_LOCAL_IP6: __anonenum_CURLoption_714703655 = 10223;
pub const CURLOPT_DNS_LOCAL_IP4: __anonenum_CURLoption_714703655 = 10222;
pub const CURLOPT_DNS_INTERFACE: __anonenum_CURLoption_714703655 = 10221;
pub const CURLOPT_XOAUTH2_BEARER: __anonenum_CURLoption_714703655 = 10220;
pub const CURLOPT_XFERINFOFUNCTION: __anonenum_CURLoption_714703655 = 20219;
pub const CURLOPT_SASL_IR: __anonenum_CURLoption_714703655 = 218;
pub const CURLOPT_MAIL_AUTH: __anonenum_CURLoption_714703655 = 10217;
pub const CURLOPT_SSL_OPTIONS: __anonenum_CURLoption_714703655 = 216;
pub const CURLOPT_TCP_KEEPINTVL: __anonenum_CURLoption_714703655 = 215;
pub const CURLOPT_TCP_KEEPIDLE: __anonenum_CURLoption_714703655 = 214;
pub const CURLOPT_TCP_KEEPALIVE: __anonenum_CURLoption_714703655 = 213;
pub const CURLOPT_ACCEPTTIMEOUT_MS: __anonenum_CURLoption_714703655 = 212;
pub const CURLOPT_DNS_SERVERS: __anonenum_CURLoption_714703655 = 10211;
pub const CURLOPT_GSSAPI_DELEGATION: __anonenum_CURLoption_714703655 = 210;
pub const CURLOPT_CLOSESOCKETDATA: __anonenum_CURLoption_714703655 = 10209;
pub const CURLOPT_CLOSESOCKETFUNCTION: __anonenum_CURLoption_714703655 = 20208;
pub const CURLOPT_TRANSFER_ENCODING: __anonenum_CURLoption_714703655 = 207;
pub const CURLOPT_TLSAUTH_TYPE: __anonenum_CURLoption_714703655 = 10206;
pub const CURLOPT_TLSAUTH_PASSWORD: __anonenum_CURLoption_714703655 = 10205;
pub const CURLOPT_TLSAUTH_USERNAME: __anonenum_CURLoption_714703655 = 10204;
pub const CURLOPT_RESOLVE: __anonenum_CURLoption_714703655 = 10203;
pub const CURLOPT_FNMATCH_DATA: __anonenum_CURLoption_714703655 = 10202;
pub const CURLOPT_CHUNK_DATA: __anonenum_CURLoption_714703655 = 10201;
pub const CURLOPT_FNMATCH_FUNCTION: __anonenum_CURLoption_714703655 = 20200;
pub const CURLOPT_CHUNK_END_FUNCTION: __anonenum_CURLoption_714703655 = 20199;
pub const CURLOPT_CHUNK_BGN_FUNCTION: __anonenum_CURLoption_714703655 = 20198;
pub const CURLOPT_WILDCARDMATCH: __anonenum_CURLoption_714703655 = 197;
pub const CURLOPT_INTERLEAVEFUNCTION: __anonenum_CURLoption_714703655 = 20196;
pub const CURLOPT_INTERLEAVEDATA: __anonenum_CURLoption_714703655 = 10195;
pub const CURLOPT_RTSP_SERVER_CSEQ: __anonenum_CURLoption_714703655 = 194;
pub const CURLOPT_RTSP_CLIENT_CSEQ: __anonenum_CURLoption_714703655 = 193;
pub const CURLOPT_RTSP_TRANSPORT: __anonenum_CURLoption_714703655 = 10192;
pub const CURLOPT_RTSP_STREAM_URI: __anonenum_CURLoption_714703655 = 10191;
pub const CURLOPT_RTSP_SESSION_ID: __anonenum_CURLoption_714703655 = 10190;
pub const CURLOPT_RTSP_REQUEST: __anonenum_CURLoption_714703655 = 189;
pub const CURLOPT_FTP_USE_PRET: __anonenum_CURLoption_714703655 = 188;
pub const CURLOPT_MAIL_RCPT: __anonenum_CURLoption_714703655 = 10187;
pub const CURLOPT_MAIL_FROM: __anonenum_CURLoption_714703655 = 10186;
pub const CURLOPT_SSH_KEYDATA: __anonenum_CURLoption_714703655 = 10185;
pub const CURLOPT_SSH_KEYFUNCTION: __anonenum_CURLoption_714703655 = 20184;
pub const CURLOPT_SSH_KNOWNHOSTS: __anonenum_CURLoption_714703655 = 10183;
pub const CURLOPT_REDIR_PROTOCOLS: __anonenum_CURLoption_714703655 = 182;
pub const CURLOPT_PROTOCOLS: __anonenum_CURLoption_714703655 = 181;
pub const CURLOPT_SOCKS5_GSSAPI_NEC: __anonenum_CURLoption_714703655 = 180;
pub const CURLOPT_SOCKS5_GSSAPI_SERVICE: __anonenum_CURLoption_714703655 = 10179;
pub const CURLOPT_TFTP_BLKSIZE: __anonenum_CURLoption_714703655 = 178;
pub const CURLOPT_NOPROXY: __anonenum_CURLoption_714703655 = 10177;
pub const CURLOPT_PROXYPASSWORD: __anonenum_CURLoption_714703655 = 10176;
pub const CURLOPT_PROXYUSERNAME: __anonenum_CURLoption_714703655 = 10175;
pub const CURLOPT_PASSWORD: __anonenum_CURLoption_714703655 = 10174;
pub const CURLOPT_USERNAME: __anonenum_CURLoption_714703655 = 10173;
pub const CURLOPT_CERTINFO: __anonenum_CURLoption_714703655 = 172;
pub const CURLOPT_ADDRESS_SCOPE: __anonenum_CURLoption_714703655 = 171;
pub const CURLOPT_ISSUERCERT: __anonenum_CURLoption_714703655 = 10170;
pub const CURLOPT_CRLFILE: __anonenum_CURLoption_714703655 = 10169;
pub const CURLOPT_SEEKDATA: __anonenum_CURLoption_714703655 = 10168;
pub const CURLOPT_SEEKFUNCTION: __anonenum_CURLoption_714703655 = 20167;
pub const CURLOPT_PROXY_TRANSFER_MODE: __anonenum_CURLoption_714703655 = 166;
pub const CURLOPT_COPYPOSTFIELDS: __anonenum_CURLoption_714703655 = 10165;
pub const CURLOPT_OPENSOCKETDATA: __anonenum_CURLoption_714703655 = 10164;
pub const CURLOPT_OPENSOCKETFUNCTION: __anonenum_CURLoption_714703655 = 20163;
pub const CURLOPT_SSH_HOST_PUBLIC_KEY_MD5: __anonenum_CURLoption_714703655 = 10162;
pub const CURLOPT_POSTREDIR: __anonenum_CURLoption_714703655 = 161;
pub const CURLOPT_NEW_DIRECTORY_PERMS: __anonenum_CURLoption_714703655 = 160;
pub const CURLOPT_NEW_FILE_PERMS: __anonenum_CURLoption_714703655 = 159;
pub const CURLOPT_HTTP_CONTENT_DECODING: __anonenum_CURLoption_714703655 = 158;
pub const CURLOPT_HTTP_TRANSFER_DECODING: __anonenum_CURLoption_714703655 = 157;
pub const CURLOPT_CONNECTTIMEOUT_MS: __anonenum_CURLoption_714703655 = 156;
pub const CURLOPT_TIMEOUT_MS: __anonenum_CURLoption_714703655 = 155;
pub const CURLOPT_FTP_SSL_CCC: __anonenum_CURLoption_714703655 = 154;
pub const CURLOPT_SSH_PRIVATE_KEYFILE: __anonenum_CURLoption_714703655 = 10153;
pub const CURLOPT_SSH_PUBLIC_KEYFILE: __anonenum_CURLoption_714703655 = 10152;
pub const CURLOPT_SSH_AUTH_TYPES: __anonenum_CURLoption_714703655 = 151;
pub const CURLOPT_SSL_SESSIONID_CACHE: __anonenum_CURLoption_714703655 = 150;
pub const CURLOPT_SOCKOPTDATA: __anonenum_CURLoption_714703655 = 10149;
pub const CURLOPT_SOCKOPTFUNCTION: __anonenum_CURLoption_714703655 = 20148;
pub const CURLOPT_FTP_ALTERNATIVE_TO_USER: __anonenum_CURLoption_714703655 = 10147;
pub const CURLOPT_MAX_RECV_SPEED_LARGE: __anonenum_CURLoption_714703655 = 30146;
pub const CURLOPT_MAX_SEND_SPEED_LARGE: __anonenum_CURLoption_714703655 = 30145;
pub const CURLOPT_CONV_FROM_UTF8_FUNCTION: __anonenum_CURLoption_714703655 = 20144;
pub const CURLOPT_CONV_TO_NETWORK_FUNCTION: __anonenum_CURLoption_714703655 = 20143;
pub const CURLOPT_CONV_FROM_NETWORK_FUNCTION: __anonenum_CURLoption_714703655 = 20142;
pub const CURLOPT_CONNECT_ONLY: __anonenum_CURLoption_714703655 = 141;
pub const CURLOPT_LOCALPORTRANGE: __anonenum_CURLoption_714703655 = 140;
pub const CURLOPT_LOCALPORT: __anonenum_CURLoption_714703655 = 139;
pub const CURLOPT_FTP_FILEMETHOD: __anonenum_CURLoption_714703655 = 138;
pub const CURLOPT_FTP_SKIP_PASV_IP: __anonenum_CURLoption_714703655 = 137;
pub const CURLOPT_IGNORE_CONTENT_LENGTH: __anonenum_CURLoption_714703655 = 136;
pub const CURLOPT_COOKIELIST: __anonenum_CURLoption_714703655 = 10135;
pub const CURLOPT_FTP_ACCOUNT: __anonenum_CURLoption_714703655 = 10134;
pub const CURLOPT_IOCTLDATA: __anonenum_CURLoption_714703655 = 10131;
pub const CURLOPT_IOCTLFUNCTION: __anonenum_CURLoption_714703655 = 20130;
pub const CURLOPT_FTPSSLAUTH: __anonenum_CURLoption_714703655 = 129;
pub const CURLOPT_TCP_NODELAY: __anonenum_CURLoption_714703655 = 121;
pub const CURLOPT_POSTFIELDSIZE_LARGE: __anonenum_CURLoption_714703655 = 30120;
pub const CURLOPT_USE_SSL: __anonenum_CURLoption_714703655 = 119;
pub const CURLOPT_NETRC_FILE: __anonenum_CURLoption_714703655 = 10118;
pub const CURLOPT_MAXFILESIZE_LARGE: __anonenum_CURLoption_714703655 = 30117;
pub const CURLOPT_RESUME_FROM_LARGE: __anonenum_CURLoption_714703655 = 30116;
pub const CURLOPT_INFILESIZE_LARGE: __anonenum_CURLoption_714703655 = 30115;
pub const CURLOPT_MAXFILESIZE: __anonenum_CURLoption_714703655 = 114;
pub const CURLOPT_IPRESOLVE: __anonenum_CURLoption_714703655 = 113;
pub const CURLOPT_FTP_RESPONSE_TIMEOUT: __anonenum_CURLoption_714703655 = 112;
pub const CURLOPT_PROXYAUTH: __anonenum_CURLoption_714703655 = 111;
pub const CURLOPT_FTP_CREATE_MISSING_DIRS: __anonenum_CURLoption_714703655 = 110;
pub const CURLOPT_SSL_CTX_DATA: __anonenum_CURLoption_714703655 = 10109;
pub const CURLOPT_SSL_CTX_FUNCTION: __anonenum_CURLoption_714703655 = 20108;
pub const CURLOPT_HTTPAUTH: __anonenum_CURLoption_714703655 = 107;
pub const CURLOPT_FTP_USE_EPRT: __anonenum_CURLoption_714703655 = 106;
pub const CURLOPT_UNRESTRICTED_AUTH: __anonenum_CURLoption_714703655 = 105;
pub const CURLOPT_HTTP200ALIASES: __anonenum_CURLoption_714703655 = 10104;
pub const CURLOPT_PRIVATE: __anonenum_CURLoption_714703655 = 10103;
pub const CURLOPT_ACCEPT_ENCODING: __anonenum_CURLoption_714703655 = 10102;
pub const CURLOPT_PROXYTYPE: __anonenum_CURLoption_714703655 = 101;
pub const CURLOPT_SHARE: __anonenum_CURLoption_714703655 = 10100;
pub const CURLOPT_NOSIGNAL: __anonenum_CURLoption_714703655 = 99;
pub const CURLOPT_BUFFERSIZE: __anonenum_CURLoption_714703655 = 98;
pub const CURLOPT_CAPATH: __anonenum_CURLoption_714703655 = 10097;
pub const CURLOPT_COOKIESESSION: __anonenum_CURLoption_714703655 = 96;
pub const CURLOPT_DEBUGDATA: __anonenum_CURLoption_714703655 = 10095;
pub const CURLOPT_DEBUGFUNCTION: __anonenum_CURLoption_714703655 = 20094;
pub const CURLOPT_PREQUOTE: __anonenum_CURLoption_714703655 = 10093;
pub const CURLOPT_DNS_CACHE_TIMEOUT: __anonenum_CURLoption_714703655 = 92;
pub const CURLOPT_DNS_USE_GLOBAL_CACHE: __anonenum_CURLoption_714703655 = 91;
pub const CURLOPT_SSLENGINE_DEFAULT: __anonenum_CURLoption_714703655 = 90;
pub const CURLOPT_SSLENGINE: __anonenum_CURLoption_714703655 = 10089;
pub const CURLOPT_SSLKEYTYPE: __anonenum_CURLoption_714703655 = 10088;
pub const CURLOPT_SSLKEY: __anonenum_CURLoption_714703655 = 10087;
pub const CURLOPT_SSLCERTTYPE: __anonenum_CURLoption_714703655 = 10086;
pub const CURLOPT_FTP_USE_EPSV: __anonenum_CURLoption_714703655 = 85;
pub const CURLOPT_HTTP_VERSION: __anonenum_CURLoption_714703655 = 84;
pub const CURLOPT_SSL_CIPHER_LIST: __anonenum_CURLoption_714703655 = 10083;
pub const CURLOPT_COOKIEJAR: __anonenum_CURLoption_714703655 = 10082;
pub const CURLOPT_SSL_VERIFYHOST: __anonenum_CURLoption_714703655 = 81;
pub const CURLOPT_HTTPGET: __anonenum_CURLoption_714703655 = 80;
pub const CURLOPT_HEADERFUNCTION: __anonenum_CURLoption_714703655 = 20079;
pub const CURLOPT_CONNECTTIMEOUT: __anonenum_CURLoption_714703655 = 78;
pub const CURLOPT_EGDSOCKET: __anonenum_CURLoption_714703655 = 10077;
pub const CURLOPT_RANDOM_FILE: __anonenum_CURLoption_714703655 = 10076;
pub const CURLOPT_FORBID_REUSE: __anonenum_CURLoption_714703655 = 75;
pub const CURLOPT_FRESH_CONNECT: __anonenum_CURLoption_714703655 = 74;
pub const CURLOPT_OBSOLETE72: __anonenum_CURLoption_714703655 = 72;
pub const CURLOPT_MAXCONNECTS: __anonenum_CURLoption_714703655 = 71;
pub const CURLOPT_TELNETOPTIONS: __anonenum_CURLoption_714703655 = 10070;
pub const CURLOPT_FILETIME: __anonenum_CURLoption_714703655 = 69;
pub const CURLOPT_MAXREDIRS: __anonenum_CURLoption_714703655 = 68;
pub const CURLOPT_CAINFO: __anonenum_CURLoption_714703655 = 10065;
pub const CURLOPT_SSL_VERIFYPEER: __anonenum_CURLoption_714703655 = 64;
pub const CURLOPT_KRBLEVEL: __anonenum_CURLoption_714703655 = 10063;
pub const CURLOPT_INTERFACE: __anonenum_CURLoption_714703655 = 10062;
pub const CURLOPT_HTTPPROXYTUNNEL: __anonenum_CURLoption_714703655 = 61;
pub const CURLOPT_POSTFIELDSIZE: __anonenum_CURLoption_714703655 = 60;
pub const CURLOPT_PROXYPORT: __anonenum_CURLoption_714703655 = 59;
pub const CURLOPT_AUTOREFERER: __anonenum_CURLoption_714703655 = 58;
pub const CURLOPT_PROGRESSDATA: __anonenum_CURLoption_714703655 = 10057;
pub const CURLOPT_PROGRESSFUNCTION: __anonenum_CURLoption_714703655 = 20056;
pub const CURLOPT_PUT: __anonenum_CURLoption_714703655 = 54;
pub const CURLOPT_TRANSFERTEXT: __anonenum_CURLoption_714703655 = 53;
pub const CURLOPT_FOLLOWLOCATION: __anonenum_CURLoption_714703655 = 52;
pub const CURLOPT_NETRC: __anonenum_CURLoption_714703655 = 51;
pub const CURLOPT_APPEND: __anonenum_CURLoption_714703655 = 50;
pub const CURLOPT_DIRLISTONLY: __anonenum_CURLoption_714703655 = 48;
pub const CURLOPT_POST: __anonenum_CURLoption_714703655 = 47;
pub const CURLOPT_UPLOAD: __anonenum_CURLoption_714703655 = 46;
pub const CURLOPT_FAILONERROR: __anonenum_CURLoption_714703655 = 45;
pub const CURLOPT_NOBODY: __anonenum_CURLoption_714703655 = 44;
pub const CURLOPT_NOPROGRESS: __anonenum_CURLoption_714703655 = 43;
pub const CURLOPT_HEADER: __anonenum_CURLoption_714703655 = 42;
pub const CURLOPT_VERBOSE: __anonenum_CURLoption_714703655 = 41;
pub const CURLOPT_OBSOLETE40: __anonenum_CURLoption_714703655 = 10040;
pub const CURLOPT_POSTQUOTE: __anonenum_CURLoption_714703655 = 10039;
pub const CURLOPT_STDERR: __anonenum_CURLoption_714703655 = 10037;
pub const CURLOPT_CUSTOMREQUEST: __anonenum_CURLoption_714703655 = 10036;
pub const CURLOPT_TIMEVALUE: __anonenum_CURLoption_714703655 = 34;
pub const CURLOPT_TIMECONDITION: __anonenum_CURLoption_714703655 = 33;
pub const CURLOPT_SSLVERSION: __anonenum_CURLoption_714703655 = 32;
pub const CURLOPT_COOKIEFILE: __anonenum_CURLoption_714703655 = 10031;
pub const CURLOPT_HEADERDATA: __anonenum_CURLoption_714703655 = 10029;
pub const CURLOPT_QUOTE: __anonenum_CURLoption_714703655 = 10028;
pub const CURLOPT_CRLF: __anonenum_CURLoption_714703655 = 27;
pub const CURLOPT_KEYPASSWD: __anonenum_CURLoption_714703655 = 10026;
pub const CURLOPT_SSLCERT: __anonenum_CURLoption_714703655 = 10025;
pub const CURLOPT_HTTPPOST: __anonenum_CURLoption_714703655 = 10024;
pub const CURLOPT_HTTPHEADER: __anonenum_CURLoption_714703655 = 10023;
pub const CURLOPT_COOKIE: __anonenum_CURLoption_714703655 = 10022;
pub const CURLOPT_RESUME_FROM: __anonenum_CURLoption_714703655 = 21;
pub const CURLOPT_LOW_SPEED_TIME: __anonenum_CURLoption_714703655 = 20;
pub const CURLOPT_LOW_SPEED_LIMIT: __anonenum_CURLoption_714703655 = 19;
pub const CURLOPT_USERAGENT: __anonenum_CURLoption_714703655 = 10018;
pub const CURLOPT_FTPPORT: __anonenum_CURLoption_714703655 = 10017;
pub const CURLOPT_REFERER: __anonenum_CURLoption_714703655 = 10016;
pub const CURLOPT_POSTFIELDS: __anonenum_CURLoption_714703655 = 10015;
pub const CURLOPT_INFILESIZE: __anonenum_CURLoption_714703655 = 14;
pub const CURLOPT_TIMEOUT: __anonenum_CURLoption_714703655 = 13;
pub const CURLOPT_READFUNCTION: __anonenum_CURLoption_714703655 = 20012;
pub const CURLOPT_WRITEFUNCTION: __anonenum_CURLoption_714703655 = 20011;
pub const CURLOPT_ERRORBUFFER: __anonenum_CURLoption_714703655 = 10010;
pub const CURLOPT_READDATA: __anonenum_CURLoption_714703655 = 10009;
pub const CURLOPT_RANGE: __anonenum_CURLoption_714703655 = 10007;
pub const CURLOPT_PROXYUSERPWD: __anonenum_CURLoption_714703655 = 10006;
pub const CURLOPT_USERPWD: __anonenum_CURLoption_714703655 = 10005;
pub const CURLOPT_PROXY: __anonenum_CURLoption_714703655 = 10004;
pub const CURLOPT_PORT: __anonenum_CURLoption_714703655 = 3;
pub const CURLOPT_URL: __anonenum_CURLoption_714703655 = 10002;
pub const CURLOPT_WRITEDATA: __anonenum_CURLoption_714703655 = 10001;
pub type CURLoption = __anonenum_CURLoption_714703655;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_context_s {
    pub vc_addrtype: libc::c_int,
    pub vc_privtype: libc::c_int,
    pub vc_npatterns: libc::c_ulong,
    pub vc_npatterns_start: libc::c_ulong,
    pub vc_found: libc::c_ulonglong,
    pub vc_pattern_generation: libc::c_int,
    pub vc_chance: libc::c_double,
    pub vc_result_file: *const libc::c_char,
    pub vc_key_protect_pass: *const libc::c_char,
    pub vc_remove_on_match: libc::c_int,
    pub vc_only_one: libc::c_int,
    pub vc_verbose: libc::c_int,
    pub vc_format: vg_format,
    pub vc_pubkeytype: libc::c_int,
    pub vc_pubkey_base: *mut EC_POINT,
    pub vc_halt: libc::c_int,
    pub vc_threads: *mut vg_exec_context_t,
    pub vc_thread_excl: libc::c_int,
    pub vc_free: Option::<unsafe extern "C" fn(*mut vg_context_t) -> ()>,
    pub vc_add_patterns: Option::<
        unsafe extern "C" fn(
            *mut vg_context_t,
            *mut *const libc::c_char,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub vc_clear_all_patterns: Option::<unsafe extern "C" fn(*mut vg_context_t) -> ()>,
    pub vc_test: Option::<unsafe extern "C" fn(*mut vg_exec_context_t) -> libc::c_int>,
    pub vc_hash160_sort: Option::<
        unsafe extern "C" fn(*mut vg_context_t, *mut libc::c_void) -> libc::c_int,
    >,
    pub vc_timing_total: libc::c_ulonglong,
    pub vc_timing_prevfound: libc::c_ulonglong,
    pub vc_timing_sincelast: libc::c_ulonglong,
    pub vc_timing_head: *mut _timing_info_s,
    pub vc_output_error: Option::<
        unsafe extern "C" fn(*mut vg_context_t, *const libc::c_char) -> (),
    >,
    pub vc_output_match: Option::<
        unsafe extern "C" fn(*mut vg_context_t, *mut EC_KEY, *const libc::c_char) -> (),
    >,
    pub vc_output_timing: Option::<
        unsafe extern "C" fn(
            *mut vg_context_t,
            libc::c_double,
            libc::c_ulonglong,
            libc::c_ulonglong,
        ) -> (),
    >,
}
pub type vg_context_t = _vg_context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _timing_info_s {
    pub ti_next: *mut _timing_info_s,
    pub ti_thread: pthread_t,
    pub ti_last_rate: libc::c_ulong,
    pub ti_hist_time: [libc::c_ulonglong; 5],
    pub ti_hist_work: [libc::c_ulong; 5],
    pub ti_hist_last: libc::c_int,
}
pub type vg_exec_context_t = _vg_exec_context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_exec_context_s {
    pub vxc_vc: *mut vg_context_t,
    pub vxc_bnctx: *mut BN_CTX,
    pub vxc_key: *mut EC_KEY,
    pub vxc_delta: libc::c_int,
    pub vxc_binres: [libc::c_uchar; 28],
    pub vxc_bntarg: BIGNUM,
    pub vxc_bnbase: BIGNUM,
    pub vxc_bntmp: BIGNUM,
    pub vxc_bntmp2: BIGNUM,
    pub vxc_threadfunc: Option::<
        unsafe extern "C" fn(*mut vg_exec_context_t) -> *mut libc::c_void,
    >,
    pub vxc_pthread: pthread_t,
    pub vxc_thread_active: libc::c_int,
    pub vxc_next: *mut _vg_exec_context_s,
    pub vxc_lockmode: libc::c_int,
    pub vxc_stop: libc::c_int,
}
pub type vg_format = libc::c_uint;
pub const VCF_SCRIPT: vg_format = 1;
pub const VCF_PUBKEY: vg_format = 0;
pub type vg_ocl_context_t = _vg_ocl_context_s;
pub type __anonenum_avl_balance_t_281414157 = libc::c_uint;
pub const RIGHT: __anonenum_avl_balance_t_281414157 = 2;
pub const LEFT: __anonenum_avl_balance_t_281414157 = 0;
pub const CENT: __anonenum_avl_balance_t_281414157 = 1;
pub type avl_balance_t = __anonenum_avl_balance_t_281414157;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _avl_item_s {
    pub ai_left: *mut _avl_item_s,
    pub ai_right: *mut _avl_item_s,
    pub ai_up: *mut _avl_item_s,
    pub ai_balance: avl_balance_t,
    pub ai_indexed: libc::c_int,
}
pub type avl_item_t = _avl_item_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _avl_root_s {
    pub ar_root: *mut avl_item_t,
}
pub type avl_root_t = _avl_root_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct workitem_s {
    pub avlent: avl_item_t,
    pub pattern: *const libc::c_char,
    pub comment: *const libc::c_char,
    pub pubkey: *mut EC_POINT,
    pub addrtype: libc::c_int,
    pub difficulty: libc::c_double,
    pub reward: libc::c_double,
    pub value: libc::c_double,
}
pub type workitem_t = workitem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pubkeybatch_s {
    pub avlent: avl_item_t,
    pub pubkey: *mut EC_POINT,
    pub pubkey_hex: *const libc::c_char,
    pub items: avl_root_t,
    pub nitems: libc::c_int,
    pub total_value: libc::c_double,
}
pub type pubkeybatch_t = pubkeybatch_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_request_s {
    pub request_status: libc::c_int,
    pub group: *const EC_GROUP,
    pub part_buf: *mut libc::c_char,
    pub part_off: size_t,
    pub part_end: size_t,
    pub part_size: size_t,
    pub items: avl_root_t,
    pub nitems: libc::c_int,
}
pub type server_request_t = server_request_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_context_s {
    pub dummy_key: *mut EC_KEY,
    pub url: *const libc::c_char,
    pub credit_addr: *const libc::c_char,
    pub getwork: *mut libc::c_char,
    pub submit: *mut libc::c_char,
    pub verbose: libc::c_int,
    pub items: avl_root_t,
    pub nitems: libc::c_int,
}
pub type server_context_t = server_context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
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
pub type timing_info_t = _timing_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_prefix_s {
    pub vp_item: avl_item_t,
    pub vp_sibling: *mut _vg_prefix_s,
    pub vp_pattern: *const libc::c_char,
    pub vp_low: *mut BIGNUM,
    pub vp_high: *mut BIGNUM,
}
pub type vg_prefix_t = _vg_prefix_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _prefix_case_iter_s {
    pub ci_prefix: [libc::c_char; 32],
    pub ci_case_map: [libc::c_char; 32],
    pub ci_nbits: libc::c_char,
    pub ci_value: libc::c_int,
}
pub type prefix_case_iter_t = _prefix_case_iter_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_prefix_context_s {
    pub base: vg_context_t,
    pub vcp_avlroot: avl_root_t,
    pub vcp_difficulty: BIGNUM,
    pub vcp_caseinsensitive: libc::c_int,
}
pub type vg_prefix_context_t = _vg_prefix_context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_regex_context_s {
    pub base: vg_context_t,
    pub vcr_regex: *mut *mut pcre,
    pub vcr_regex_extra: *mut *mut pcre_extra,
    pub vcr_regex_pat: *mut *const libc::c_char,
    pub vcr_nalloc: libc::c_ulong,
}
pub type vg_regex_context_t = _vg_regex_context_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_string_st {
    pub length: libc::c_int,
    pub type_0: libc::c_int,
    pub data: *mut libc::c_uchar,
    pub flags: libc::c_long,
}
pub type ASN1_INTEGER = asn1_string_st;
pub type ASN1_ENUMERATED = asn1_string_st;
pub type ASN1_BIT_STRING = asn1_string_st;
pub type ASN1_OCTET_STRING = asn1_string_st;
pub type ASN1_PRINTABLESTRING = asn1_string_st;
pub type ASN1_T61STRING = asn1_string_st;
pub type ASN1_IA5STRING = asn1_string_st;
pub type ASN1_GENERALSTRING = asn1_string_st;
pub type ASN1_UNIVERSALSTRING = asn1_string_st;
pub type ASN1_BMPSTRING = asn1_string_st;
pub type ASN1_UTCTIME = asn1_string_st;
pub type ASN1_GENERALIZEDTIME = asn1_string_st;
pub type ASN1_VISIBLESTRING = asn1_string_st;
pub type ASN1_UTF8STRING = asn1_string_st;
pub type ASN1_STRING = asn1_string_st;
pub type ASN1_BOOLEAN = libc::c_int;
pub type BN_BLINDING = bn_blinding_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn_mont_ctx_st {
    pub ri: libc::c_int,
    pub RR: BIGNUM,
    pub N: BIGNUM,
    pub Ni: BIGNUM,
    pub n0: [libc::c_ulong; 2],
    pub flags: libc::c_int,
}
pub type BN_MONT_CTX = bn_mont_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bn_gencb_st {
    pub ver: libc::c_uint,
    pub arg: *mut libc::c_void,
    pub cb: __anonunion_cb_888073939,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_cb_888073939 {
    pub cb_1: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
    >,
    pub cb_2: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut BN_GENCB) -> libc::c_int,
    >,
}
pub type BN_GENCB = bn_gencb_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf_mem_st {
    pub length: size_t,
    pub data: *mut libc::c_char,
    pub max: size_t,
}
pub type BUF_MEM = buf_mem_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_cipher_st {
    pub nid: libc::c_int,
    pub block_size: libc::c_int,
    pub key_len: libc::c_int,
    pub iv_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            *const libc::c_uchar,
            *const libc::c_uchar,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub do_cipher: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            *mut libc::c_uchar,
            *const libc::c_uchar,
            size_t,
        ) -> libc::c_int,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut EVP_CIPHER_CTX) -> libc::c_int>,
    pub ctx_size: libc::c_int,
    pub set_asn1_parameters: Option::<
        unsafe extern "C" fn(*mut EVP_CIPHER_CTX, *mut ASN1_TYPE) -> libc::c_int,
    >,
    pub get_asn1_parameters: Option::<
        unsafe extern "C" fn(*mut EVP_CIPHER_CTX, *mut ASN1_TYPE) -> libc::c_int,
    >,
    pub ctrl: Option::<
        unsafe extern "C" fn(
            *mut EVP_CIPHER_CTX,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub app_data: *mut libc::c_void,
}
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_cipher_ctx_st {
    pub cipher: *const EVP_CIPHER,
    pub engine: *mut ENGINE,
    pub encrypt: libc::c_int,
    pub buf_len: libc::c_int,
    pub oiv: [libc::c_uchar; 16],
    pub iv: [libc::c_uchar; 16],
    pub buf: [libc::c_uchar; 32],
    pub num: libc::c_int,
    pub app_data: *mut libc::c_void,
    pub key_len: libc::c_int,
    pub flags: libc::c_ulong,
    pub cipher_data: *mut libc::c_void,
    pub final_used: libc::c_int,
    pub block_mask: libc::c_int,
    pub final_0: [libc::c_uchar; 32],
}
pub type ENGINE = engine_st;
pub type EVP_CIPHER = evp_cipher_st;
pub type ASN1_TYPE = asn1_type_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_type_st {
    pub type_0: libc::c_int,
    pub value: __anonunion_value_401497255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_value_401497255 {
    pub ptr: *mut libc::c_char,
    pub boolean: ASN1_BOOLEAN,
    pub asn1_string: *mut ASN1_STRING,
    pub object: *mut ASN1_OBJECT,
    pub integer: *mut ASN1_INTEGER,
    pub enumerated: *mut ASN1_ENUMERATED,
    pub bit_string: *mut ASN1_BIT_STRING,
    pub octet_string: *mut ASN1_OCTET_STRING,
    pub printablestring: *mut ASN1_PRINTABLESTRING,
    pub t61string: *mut ASN1_T61STRING,
    pub ia5string: *mut ASN1_IA5STRING,
    pub generalstring: *mut ASN1_GENERALSTRING,
    pub bmpstring: *mut ASN1_BMPSTRING,
    pub universalstring: *mut ASN1_UNIVERSALSTRING,
    pub utctime: *mut ASN1_UTCTIME,
    pub generalizedtime: *mut ASN1_GENERALIZEDTIME,
    pub visiblestring: *mut ASN1_VISIBLESTRING,
    pub utf8string: *mut ASN1_UTF8STRING,
    pub set: *mut ASN1_STRING,
    pub sequence: *mut ASN1_STRING,
    pub asn1_value: *mut ASN1_VALUE,
}
pub type ASN1_VALUE = ASN1_VALUE_st;
pub type ASN1_OBJECT = asn1_object_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct asn1_object_st {
    pub sn: *const libc::c_char,
    pub ln: *const libc::c_char,
    pub nid: libc::c_int,
    pub length: libc::c_int,
    pub data: *const libc::c_uchar,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env_md_st {
    pub type_0: libc::c_int,
    pub pkey_type: libc::c_int,
    pub md_size: libc::c_int,
    pub flags: libc::c_ulong,
    pub init: Option::<unsafe extern "C" fn(*mut EVP_MD_CTX) -> libc::c_int>,
    pub update: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const libc::c_void, size_t) -> libc::c_int,
    >,
    pub final_0: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *mut libc::c_uchar) -> libc::c_int,
    >,
    pub copy: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const EVP_MD_CTX) -> libc::c_int,
    >,
    pub cleanup: Option::<unsafe extern "C" fn(*mut EVP_MD_CTX) -> libc::c_int>,
    pub sign: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            *mut libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub verify: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub required_pkey_type: [libc::c_int; 5],
    pub block_size: libc::c_int,
    pub ctx_size: libc::c_int,
    pub md_ctrl: Option::<
        unsafe extern "C" fn(
            *mut EVP_MD_CTX,
            libc::c_int,
            libc::c_int,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
}
pub type EVP_MD_CTX = env_md_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct env_md_ctx_st {
    pub digest: *const EVP_MD,
    pub engine: *mut ENGINE,
    pub flags: libc::c_ulong,
    pub md_data: *mut libc::c_void,
    pub pctx: *mut EVP_PKEY_CTX,
    pub update: Option::<
        unsafe extern "C" fn(*mut EVP_MD_CTX, *const libc::c_void, size_t) -> libc::c_int,
    >,
}
pub type EVP_PKEY_CTX = evp_pkey_ctx_st;
pub type EVP_MD = env_md_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evp_pkey_st {
    pub type_0: libc::c_int,
    pub save_type: libc::c_int,
    pub references: libc::c_int,
    pub ameth: *const EVP_PKEY_ASN1_METHOD,
    pub engine: *mut ENGINE,
    pub pkey: __anonunion_pkey_1024245030,
    pub save_parameters: libc::c_int,
    pub attributes: *mut stack_st_X509_ATTRIBUTE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_X509_ATTRIBUTE {
    pub stack: _STACK,
}
pub type _STACK = stack_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st {
    pub num: libc::c_int,
    pub data: *mut *mut libc::c_char,
    pub sorted: libc::c_int,
    pub num_alloc: libc::c_int,
    pub comp: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pkey_1024245030 {
    pub ptr: *mut libc::c_char,
    pub rsa: *mut rsa_st,
    pub dsa: *mut dsa_st,
    pub dh: *mut dh_st,
    pub ec: *mut ec_key_st,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dh_st {
    pub pad: libc::c_int,
    pub version: libc::c_int,
    pub p: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub length: libc::c_long,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub q: *mut BIGNUM,
    pub j: *mut BIGNUM,
    pub seed: *mut libc::c_uchar,
    pub seedlen: libc::c_int,
    pub counter: *mut BIGNUM,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DH_METHOD,
    pub engine: *mut ENGINE,
}
pub type DH_METHOD = dh_method;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dh_method {
    pub name: *const libc::c_char,
    pub generate_key: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub compute_key: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, *const BIGNUM, *mut DH) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *const DH,
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut DH) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub generate_params: Option::<
        unsafe extern "C" fn(
            *mut DH,
            libc::c_int,
            libc::c_int,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
}
pub type DH = dh_st;
pub type CRYPTO_EX_DATA = crypto_ex_data_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct crypto_ex_data_st {
    pub sk: *mut stack_st_void,
    pub dummy: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_st_void {
    pub stack: _STACK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub write_params: libc::c_int,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub g: *mut BIGNUM,
    pub pub_key: *mut BIGNUM,
    pub priv_key: *mut BIGNUM,
    pub kinv: *mut BIGNUM,
    pub r: *mut BIGNUM,
    pub flags: libc::c_int,
    pub method_mont_p: *mut BN_MONT_CTX,
    pub references: libc::c_int,
    pub ex_data: CRYPTO_EX_DATA,
    pub meth: *const DSA_METHOD,
    pub engine: *mut ENGINE,
}
pub type DSA_METHOD = dsa_method;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsa_method {
    pub name: *const libc::c_char,
    pub dsa_do_sign: Option::<
        unsafe extern "C" fn(*const libc::c_uchar, libc::c_int, *mut DSA) -> *mut DSA_SIG,
    >,
    pub dsa_sign_setup: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BN_CTX,
            *mut *mut BIGNUM,
            *mut *mut BIGNUM,
        ) -> libc::c_int,
    >,
    pub dsa_do_verify: Option::<
        unsafe extern "C" fn(
            *const libc::c_uchar,
            libc::c_int,
            *mut DSA_SIG,
            *mut DSA,
        ) -> libc::c_int,
    >,
    pub dsa_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            *mut BIGNUM,
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub dsa_paramgen: Option::<
        unsafe extern "C" fn(
            *mut DSA,
            libc::c_int,
            *const libc::c_uchar,
            libc::c_int,
            *mut libc::c_int,
            *mut libc::c_ulong,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
    pub dsa_keygen: Option::<unsafe extern "C" fn(*mut DSA) -> libc::c_int>,
}
pub type DSA = dsa_st;
pub type DSA_SIG = DSA_SIG_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DSA_SIG_st {
    pub r: *mut BIGNUM,
    pub s: *mut BIGNUM,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_st {
    pub pad: libc::c_int,
    pub version: libc::c_long,
    pub meth: *const RSA_METHOD,
    pub engine: *mut ENGINE,
    pub n: *mut BIGNUM,
    pub e: *mut BIGNUM,
    pub d: *mut BIGNUM,
    pub p: *mut BIGNUM,
    pub q: *mut BIGNUM,
    pub dmp1: *mut BIGNUM,
    pub dmq1: *mut BIGNUM,
    pub iqmp: *mut BIGNUM,
    pub ex_data: CRYPTO_EX_DATA,
    pub references: libc::c_int,
    pub flags: libc::c_int,
    pub _method_mod_n: *mut BN_MONT_CTX,
    pub _method_mod_p: *mut BN_MONT_CTX,
    pub _method_mod_q: *mut BN_MONT_CTX,
    pub bignum_data: *mut libc::c_char,
    pub blinding: *mut BN_BLINDING,
    pub mt_blinding: *mut BN_BLINDING,
}
pub type RSA_METHOD = rsa_meth_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rsa_meth_st {
    pub name: *const libc::c_char,
    pub rsa_pub_enc: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_pub_dec: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_priv_enc: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_priv_dec: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            *mut libc::c_uchar,
            *mut RSA,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub rsa_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut BIGNUM,
            *const BIGNUM,
            *mut RSA,
            *mut BN_CTX,
        ) -> libc::c_int,
    >,
    pub bn_mod_exp: Option::<
        unsafe extern "C" fn(
            *mut BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *const BIGNUM,
            *mut BN_CTX,
            *mut BN_MONT_CTX,
        ) -> libc::c_int,
    >,
    pub init: Option::<unsafe extern "C" fn(*mut RSA) -> libc::c_int>,
    pub finish: Option::<unsafe extern "C" fn(*mut RSA) -> libc::c_int>,
    pub flags: libc::c_int,
    pub app_data: *mut libc::c_char,
    pub rsa_sign: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *mut libc::c_uchar,
            *mut libc::c_uint,
            *const RSA,
        ) -> libc::c_int,
    >,
    pub rsa_verify: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_uchar,
            libc::c_uint,
            *const libc::c_uchar,
            libc::c_uint,
            *const RSA,
        ) -> libc::c_int,
    >,
    pub rsa_keygen: Option::<
        unsafe extern "C" fn(
            *mut RSA,
            libc::c_int,
            *mut BIGNUM,
            *mut BN_GENCB,
        ) -> libc::c_int,
    >,
}
pub type RSA = rsa_st;
pub type EVP_PKEY_ASN1_METHOD = evp_pkey_asn1_method_st;
pub type EVP_PKEY = evp_pkey_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_algor_st {
    pub algorithm: *mut ASN1_OBJECT,
    pub parameter: *mut ASN1_TYPE,
}
pub type X509_ALGOR = X509_algor_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pkcs8_priv_key_info_st {
    pub broken: libc::c_int,
    pub version: *mut ASN1_INTEGER,
    pub pkeyalg: *mut X509_ALGOR,
    pub pkey: *mut ASN1_TYPE,
    pub attributes: *mut stack_st_X509_ATTRIBUTE,
}
pub type PKCS8_PRIV_KEY_INFO = pkcs8_priv_key_info_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bio_st {
    pub method: *mut BIO_METHOD,
    pub callback: Option::<
        unsafe extern "C" fn(
            *mut bio_st,
            libc::c_int,
            *const libc::c_char,
            libc::c_int,
            libc::c_long,
            libc::c_long,
        ) -> libc::c_long,
    >,
    pub cb_arg: *mut libc::c_char,
    pub init: libc::c_int,
    pub shutdown: libc::c_int,
    pub flags: libc::c_int,
    pub retry_reason: libc::c_int,
    pub num: libc::c_int,
    pub ptr: *mut libc::c_void,
    pub next_bio: *mut bio_st,
    pub prev_bio: *mut bio_st,
    pub references: libc::c_int,
    pub num_read: libc::c_ulong,
    pub num_write: libc::c_ulong,
    pub ex_data: CRYPTO_EX_DATA,
}
pub type BIO_METHOD = bio_method_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bio_method_st {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub bwrite: Option::<
        unsafe extern "C" fn(*mut BIO, *const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub bread: Option::<
        unsafe extern "C" fn(*mut BIO, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub bputs: Option::<
        unsafe extern "C" fn(*mut BIO, *const libc::c_char) -> libc::c_int,
    >,
    pub bgets: Option::<
        unsafe extern "C" fn(*mut BIO, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub ctrl: Option::<
        unsafe extern "C" fn(
            *mut BIO,
            libc::c_int,
            libc::c_long,
            *mut libc::c_void,
        ) -> libc::c_long,
    >,
    pub create: Option::<unsafe extern "C" fn(*mut BIO) -> libc::c_int>,
    pub destroy: Option::<unsafe extern "C" fn(*mut BIO) -> libc::c_int>,
    pub callback_ctrl: Option::<
        unsafe extern "C" fn(
            *mut BIO,
            libc::c_int,
            Option::<bio_info_cb>,
        ) -> libc::c_long,
    >,
}
pub type bio_info_cb = unsafe extern "C" fn(
    *mut bio_st,
    libc::c_int,
    *const libc::c_char,
    libc::c_int,
    libc::c_long,
    libc::c_long,
) -> ();
pub type BIO = bio_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct X509_sig_st {
    pub algor: *mut X509_ALGOR,
    pub digest: *mut ASN1_OCTET_STRING,
}
pub type X509_SIG = X509_sig_st;
pub type pem_password_cb = unsafe extern "C" fn(
    *mut libc::c_char,
    libc::c_int,
    libc::c_int,
    *mut libc::c_void,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_vg_protkey_parameters_t_1070732568 {
    pub mode: libc::c_int,
    pub iterations: libc::c_int,
    pub pbkdf_hash_getter: Option::<unsafe extern "C" fn() -> *const EVP_MD>,
    pub cipher_getter: Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
}
pub type vg_protkey_parameters_t = __anonstruct_vg_protkey_parameters_t_1070732568;
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> libc::c_int {
    return (__thread1 == __thread2) as libc::c_int;
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
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_long() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_curl_off_t() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_string() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_write_callback() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_resolver_start_callback() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_read_cb() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_ioctl_cb() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_sockopt_cb() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_opensocket_cb() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_progress_cb() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_debug_cb() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_ssl_ctx_cb() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_conv_cb() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_seek_cb() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_cb_data() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_error_buffer() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_FILE() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_postfields() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_curl_httpost() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_curl_mimepost() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_curl_slist() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline(never)]
unsafe extern "C" fn _curl_easy_setopt_err_CURLSH() {
    asm!("", options(preserves_flags, att_syntax));
}
#[inline]
unsafe extern "C" fn avl_root_init(mut rootp: *mut avl_root_t) {
    (*rootp).ar_root = 0 as *mut libc::c_void as *mut avl_item_t;
}
#[inline]
unsafe extern "C" fn avl_root_empty(mut rootp: *mut avl_root_t) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if (*rootp).ar_root as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = 1 as libc::c_int;
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp;
}
#[inline]
unsafe extern "C" fn avl_item_init(mut itemp: *mut avl_item_t) {
    (*itemp).ai_left = 0 as *mut libc::c_void as *mut _avl_item_s;
    (*itemp).ai_right = 0 as *mut libc::c_void as *mut _avl_item_s;
    (*itemp).ai_up = 0 as *mut libc::c_void as *mut _avl_item_s;
    (*itemp).ai_balance = CENT;
    (*itemp).ai_indexed = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn _avl_rotate_ll(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut tmp: *mut avl_item_t = 0 as *mut avl_item_t;
    tmp = (*itemp).ai_left;
    (*itemp).ai_left = (*tmp).ai_right;
    if !((*itemp).ai_left).is_null() {
        (*(*itemp).ai_left).ai_up = itemp;
    }
    (*tmp).ai_right = itemp;
    if !((*itemp).ai_up).is_null() {
        if (*(*itemp).ai_up).ai_left as libc::c_ulong == itemp as libc::c_ulong {
            (*(*itemp).ai_up).ai_left = tmp;
        } else {
            if !((*(*itemp).ai_up).ai_right as libc::c_ulong == itemp as libc::c_ulong) {
                __assert_fail(
                    b"itemp->ai_up->ai_right == itemp\0" as *const u8
                        as *const libc::c_char,
                    b"avl.h\0" as *const u8 as *const libc::c_char,
                    89 as libc::c_uint,
                    b"_avl_rotate_ll\0" as *const u8 as *const libc::c_char,
                );
            }
            (*(*itemp).ai_up).ai_right = tmp;
        }
    } else {
        (*rootp).ar_root = tmp;
    }
    (*tmp).ai_up = (*itemp).ai_up;
    (*itemp).ai_up = tmp;
}
#[inline]
unsafe extern "C" fn _avl_rotate_lr(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut rcp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut rlcp: *mut avl_item_t = 0 as *mut avl_item_t;
    rcp = (*itemp).ai_left;
    rlcp = (*rcp).ai_right;
    if !((*itemp).ai_up).is_null() {
        if itemp as libc::c_ulong == (*(*itemp).ai_up).ai_left as libc::c_ulong {
            (*(*itemp).ai_up).ai_left = rlcp;
        } else {
            if !(itemp as libc::c_ulong == (*(*itemp).ai_up).ai_right as libc::c_ulong) {
                __assert_fail(
                    b"itemp == itemp->ai_up->ai_right\0" as *const u8
                        as *const libc::c_char,
                    b"avl.h\0" as *const u8 as *const libc::c_char,
                    109 as libc::c_uint,
                    b"_avl_rotate_lr\0" as *const u8 as *const libc::c_char,
                );
            }
            (*(*itemp).ai_up).ai_right = rlcp;
        }
    } else {
        (*rootp).ar_root = rlcp;
    }
    (*rlcp).ai_up = (*itemp).ai_up;
    (*rcp).ai_right = (*rlcp).ai_left;
    if !((*rcp).ai_right).is_null() {
        (*(*rcp).ai_right).ai_up = rcp;
    }
    (*itemp).ai_left = (*rlcp).ai_right;
    if !((*itemp).ai_left).is_null() {
        (*(*itemp).ai_left).ai_up = itemp;
    }
    (*rlcp).ai_left = rcp;
    (*rlcp).ai_right = itemp;
    (*rcp).ai_up = rlcp;
    (*itemp).ai_up = rlcp;
}
#[inline]
unsafe extern "C" fn _avl_rotate_rr(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut tmp: *mut avl_item_t = 0 as *mut avl_item_t;
    tmp = (*itemp).ai_right;
    (*itemp).ai_right = (*tmp).ai_left;
    if !((*itemp).ai_right).is_null() {
        (*(*itemp).ai_right).ai_up = itemp;
    }
    (*tmp).ai_left = itemp;
    if !((*itemp).ai_up).is_null() {
        if (*(*itemp).ai_up).ai_right as libc::c_ulong == itemp as libc::c_ulong {
            (*(*itemp).ai_up).ai_right = tmp;
        } else {
            if !((*(*itemp).ai_up).ai_left as libc::c_ulong == itemp as libc::c_ulong) {
                __assert_fail(
                    b"itemp->ai_up->ai_left == itemp\0" as *const u8
                        as *const libc::c_char,
                    b"avl.h\0" as *const u8 as *const libc::c_char,
                    142 as libc::c_uint,
                    b"_avl_rotate_rr\0" as *const u8 as *const libc::c_char,
                );
            }
            (*(*itemp).ai_up).ai_left = tmp;
        }
    } else {
        (*rootp).ar_root = tmp;
    }
    (*tmp).ai_up = (*itemp).ai_up;
    (*itemp).ai_up = tmp;
}
#[inline]
unsafe extern "C" fn _avl_rotate_rl(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut rcp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut rlcp: *mut avl_item_t = 0 as *mut avl_item_t;
    rcp = (*itemp).ai_right;
    rlcp = (*rcp).ai_left;
    if !((*itemp).ai_up).is_null() {
        if itemp as libc::c_ulong == (*(*itemp).ai_up).ai_right as libc::c_ulong {
            (*(*itemp).ai_up).ai_right = rlcp;
        } else {
            if !(itemp as libc::c_ulong == (*(*itemp).ai_up).ai_left as libc::c_ulong) {
                __assert_fail(
                    b"itemp == itemp->ai_up->ai_left\0" as *const u8
                        as *const libc::c_char,
                    b"avl.h\0" as *const u8 as *const libc::c_char,
                    162 as libc::c_uint,
                    b"_avl_rotate_rl\0" as *const u8 as *const libc::c_char,
                );
            }
            (*(*itemp).ai_up).ai_left = rlcp;
        }
    } else {
        (*rootp).ar_root = rlcp;
    }
    (*rlcp).ai_up = (*itemp).ai_up;
    (*rcp).ai_left = (*rlcp).ai_right;
    if !((*rcp).ai_left).is_null() {
        (*(*rcp).ai_left).ai_up = rcp;
    }
    (*itemp).ai_right = (*rlcp).ai_left;
    if !((*itemp).ai_right).is_null() {
        (*(*itemp).ai_right).ai_up = itemp;
    }
    (*rlcp).ai_right = rcp;
    (*rlcp).ai_left = itemp;
    (*rcp).ai_up = rlcp;
    (*itemp).ai_up = rlcp;
}
unsafe extern "C" fn avl_delete_fix(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
    mut parentp: *mut avl_item_t,
) {
    let mut childp: *mut avl_item_t = 0 as *mut avl_item_t;
    if (*parentp).ai_left as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        if (*parentp).ai_right as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            if !(itemp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
                __assert_fail(
                    b"itemp == NULL\0" as *const u8 as *const libc::c_char,
                    b"avl.h\0" as *const u8 as *const libc::c_char,
                    188 as libc::c_uint,
                    b"avl_delete_fix\0" as *const u8 as *const libc::c_char,
                );
            }
            (*parentp).ai_balance = CENT;
            itemp = parentp;
            parentp = (*itemp).ai_up;
        }
    }
    while !parentp.is_null() {
        if itemp as libc::c_ulong == (*parentp).ai_right as libc::c_ulong {
            itemp = (*parentp).ai_left;
            if (*parentp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                if (*itemp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                    _avl_rotate_ll(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    parentp = itemp;
                } else if (*itemp).ai_balance as libc::c_uint == 1 as libc::c_uint {
                    _avl_rotate_ll(rootp, parentp);
                    (*itemp).ai_balance = RIGHT;
                    (*parentp).ai_balance = LEFT;
                    break;
                } else {
                    childp = (*itemp).ai_right;
                    _avl_rotate_lr(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    if (*childp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                        (*itemp).ai_balance = LEFT;
                    }
                    if (*childp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                        (*parentp).ai_balance = RIGHT;
                    }
                    (*childp).ai_balance = CENT;
                    parentp = childp;
                }
            } else if (*parentp).ai_balance as libc::c_uint == 1 as libc::c_uint {
                (*parentp).ai_balance = LEFT;
                break;
            } else {
                (*parentp).ai_balance = CENT;
            }
        } else {
            itemp = (*parentp).ai_right;
            if (*parentp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                if (*itemp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                    _avl_rotate_rr(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    parentp = itemp;
                } else if (*itemp).ai_balance as libc::c_uint == 1 as libc::c_uint {
                    _avl_rotate_rr(rootp, parentp);
                    (*itemp).ai_balance = LEFT;
                    (*parentp).ai_balance = RIGHT;
                    break;
                } else {
                    childp = (*itemp).ai_left;
                    _avl_rotate_rl(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    if (*childp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                        (*parentp).ai_balance = LEFT;
                    }
                    if (*childp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                        (*itemp).ai_balance = RIGHT;
                    }
                    (*childp).ai_balance = CENT;
                    parentp = childp;
                }
            } else if (*parentp).ai_balance as libc::c_uint == 1 as libc::c_uint {
                (*parentp).ai_balance = RIGHT;
                break;
            } else {
                (*parentp).ai_balance = CENT;
            }
        }
        itemp = parentp;
        parentp = (*itemp).ai_up;
    }
}
unsafe extern "C" fn avl_insert_fix(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut childp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut parentp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut tmp: *mut _avl_item_s = 0 as *mut _avl_item_s;
    parentp = (*itemp).ai_up;
    tmp = 0 as *mut libc::c_void as *mut _avl_item_s;
    (*itemp).ai_right = tmp;
    (*itemp).ai_left = tmp;
    if (*itemp).ai_indexed != 0 {
        __assert_fail(
            b"!itemp->ai_indexed\0" as *const u8 as *const libc::c_char,
            b"avl.h\0" as *const u8 as *const libc::c_char,
            275 as libc::c_uint,
            b"avl_insert_fix\0" as *const u8 as *const libc::c_char,
        );
    }
    (*itemp).ai_indexed = 1 as libc::c_int;
    while !parentp.is_null() {
        if itemp as libc::c_ulong == (*parentp).ai_left as libc::c_ulong {
            if (*parentp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                if (*itemp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                    _avl_rotate_ll(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    break;
                } else {
                    if !((*itemp).ai_balance as libc::c_uint != 1 as libc::c_uint) {
                        __assert_fail(
                            b"itemp->ai_balance != CENT\0" as *const u8
                                as *const libc::c_char,
                            b"avl.h\0" as *const u8 as *const libc::c_char,
                            290 as libc::c_uint,
                            b"avl_insert_fix\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    childp = (*itemp).ai_right;
                    _avl_rotate_lr(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    if (*childp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                        (*itemp).ai_balance = LEFT;
                    }
                    if (*childp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                        (*parentp).ai_balance = RIGHT;
                    }
                    (*childp).ai_balance = CENT;
                    break;
                }
            } else if (*parentp).ai_balance as libc::c_uint == 1 as libc::c_uint {
                (*parentp).ai_balance = LEFT;
            } else {
                (*parentp).ai_balance = CENT;
                return;
            }
        } else if (*parentp).ai_balance as libc::c_uint == 2 as libc::c_uint {
            if (*itemp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                _avl_rotate_rr(rootp, parentp);
                (*itemp).ai_balance = CENT;
                (*parentp).ai_balance = CENT;
                break;
            } else {
                if !((*itemp).ai_balance as libc::c_uint != 1 as libc::c_uint) {
                    __assert_fail(
                        b"itemp->ai_balance != CENT\0" as *const u8
                            as *const libc::c_char,
                        b"avl.h\0" as *const u8 as *const libc::c_char,
                        316 as libc::c_uint,
                        b"avl_insert_fix\0" as *const u8 as *const libc::c_char,
                    );
                }
                childp = (*itemp).ai_left;
                _avl_rotate_rl(rootp, parentp);
                (*itemp).ai_balance = CENT;
                (*parentp).ai_balance = CENT;
                if (*childp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                    (*parentp).ai_balance = LEFT;
                }
                if (*childp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                    (*itemp).ai_balance = RIGHT;
                }
                (*childp).ai_balance = CENT;
                break;
            }
        } else if (*parentp).ai_balance as libc::c_uint == 1 as libc::c_uint {
            (*parentp).ai_balance = RIGHT;
        } else {
            (*parentp).ai_balance = CENT;
            break;
        }
        itemp = parentp;
        parentp = (*itemp).ai_up;
    }
}
#[inline]
unsafe extern "C" fn avl_first(mut rootp: *mut avl_root_t) -> *mut avl_item_t {
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    itemp = (*rootp).ar_root;
    if !itemp.is_null() {
        while !((*itemp).ai_left).is_null() {
            itemp = (*itemp).ai_left;
        }
    }
    return itemp;
}
#[inline]
unsafe extern "C" fn avl_next(mut itemp: *mut avl_item_t) -> *mut avl_item_t {
    if !((*itemp).ai_right).is_null() {
        itemp = (*itemp).ai_right;
        while !((*itemp).ai_left).is_null() {
            itemp = (*itemp).ai_left;
        }
        return itemp;
    }
    while !((*itemp).ai_up).is_null() {
        if !(itemp as libc::c_ulong == (*(*itemp).ai_up).ai_right as libc::c_ulong) {
            break;
        }
        itemp = (*itemp).ai_up;
    }
    if ((*itemp).ai_up).is_null() {
        return 0 as *mut libc::c_void as *mut avl_item_t;
    }
    return (*itemp).ai_up;
}
unsafe extern "C" fn avl_remove(mut rootp: *mut avl_root_t, mut itemp: *mut avl_item_t) {
    let mut relocp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut replacep: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut parentp: *mut avl_item_t = 0 as *mut avl_item_t;
    parentp = 0 as *mut libc::c_void as *mut avl_item_t;
    if (*itemp).ai_indexed == 0 {
        __assert_fail(
            b"itemp->ai_indexed\0" as *const u8 as *const libc::c_char,
            b"avl.h\0" as *const u8 as *const libc::c_char,
            376 as libc::c_uint,
            b"avl_remove\0" as *const u8 as *const libc::c_char,
        );
    }
    (*itemp).ai_indexed = 0 as libc::c_int;
    's_113: {
        if !((*itemp).ai_left as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong)
        {
            if !((*itemp).ai_right as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong)
            {
                break 's_113;
            }
        }
        parentp = (*itemp).ai_up;
        replacep = (*itemp).ai_left;
        if replacep as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            replacep = (*itemp).ai_right;
        }
        if replacep as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            (*replacep).ai_up = parentp;
        }
        if parentp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            (*rootp).ar_root = replacep;
        } else {
            if itemp as libc::c_ulong == (*parentp).ai_left as libc::c_ulong {
                (*parentp).ai_left = replacep;
            } else {
                (*parentp).ai_right = replacep;
            }
            avl_delete_fix(rootp, replacep, parentp);
        }
        return;
    }
    relocp = avl_next(itemp);
    if relocp.is_null() {
        __assert_fail(
            b"relocp\0" as *const u8 as *const libc::c_char,
            b"avl.h\0" as *const u8 as *const libc::c_char,
            405 as libc::c_uint,
            b"avl_remove\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*relocp).ai_up as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"relocp->ai_up != NULL\0" as *const u8 as *const libc::c_char,
            b"avl.h\0" as *const u8 as *const libc::c_char,
            406 as libc::c_uint,
            b"avl_remove\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*relocp).ai_left as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"relocp->ai_left == NULL\0" as *const u8 as *const libc::c_char,
            b"avl.h\0" as *const u8 as *const libc::c_char,
            407 as libc::c_uint,
            b"avl_remove\0" as *const u8 as *const libc::c_char,
        );
    }
    replacep = (*relocp).ai_right;
    (*relocp).ai_left = (*itemp).ai_left;
    if (*relocp).ai_left as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*(*relocp).ai_left).ai_up = relocp;
    }
    if (*itemp).ai_up as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        (*rootp).ar_root = relocp;
    } else if itemp as libc::c_ulong == (*(*itemp).ai_up).ai_left as libc::c_ulong {
        (*(*itemp).ai_up).ai_left = relocp;
    } else {
        (*(*itemp).ai_up).ai_right = relocp;
    }
    if relocp as libc::c_ulong == (*(*relocp).ai_up).ai_left as libc::c_ulong {
        if !((*relocp).ai_up as libc::c_ulong != itemp as libc::c_ulong) {
            __assert_fail(
                b"relocp->ai_up != itemp\0" as *const u8 as *const libc::c_char,
                b"avl.h\0" as *const u8 as *const libc::c_char,
                421 as libc::c_uint,
                b"avl_remove\0" as *const u8 as *const libc::c_char,
            );
        }
        (*(*relocp).ai_up).ai_left = replacep;
        parentp = (*relocp).ai_up;
        if replacep as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            (*replacep).ai_up = (*relocp).ai_up;
        }
        (*relocp).ai_right = (*itemp).ai_right;
    } else {
        if !((*relocp).ai_up as libc::c_ulong == itemp as libc::c_ulong) {
            __assert_fail(
                b"relocp->ai_up == itemp\0" as *const u8 as *const libc::c_char,
                b"avl.h\0" as *const u8 as *const libc::c_char,
                428 as libc::c_uint,
                b"avl_remove\0" as *const u8 as *const libc::c_char,
            );
        }
        (*relocp).ai_right = replacep;
        parentp = relocp;
    }
    if (*relocp).ai_right as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*(*relocp).ai_right).ai_up = relocp;
    }
    (*relocp).ai_up = (*itemp).ai_up;
    (*relocp).ai_balance = (*itemp).ai_balance;
    avl_delete_fix(rootp, replacep, parentp);
}
pub static mut version: *const libc::c_char = b"0.22\0" as *const u8
    as *const libc::c_char;
pub static mut debug: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn workitem_cmp(
    mut a: *mut workitem_t,
    mut b: *mut workitem_t,
) -> libc::c_int {
    let mut cmpres: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    cmpres = strcmp((*a).pattern, (*b).pattern);
    if cmpres == 0 {
        if (*a).reward < (*b).reward {
            cmpres = -(1 as libc::c_int);
        } else {
            if (*a).reward > (*b).reward {
                tmp = 1 as libc::c_int;
            } else {
                tmp = 0 as libc::c_int;
            }
            cmpres = tmp;
        }
    }
    return cmpres;
}
unsafe extern "C" fn workitem_avl_search(
    mut rootp: *mut avl_root_t,
    mut pattern: *const libc::c_char,
) -> *mut workitem_t {
    let mut vp: *mut workitem_t = 0 as *mut workitem_t;
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut cmpres: libc::c_int = 0;
    itemp = (*rootp).ar_root;
    while !itemp.is_null() {
        vp = (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut workitem_t)).avlent
                    as *mut avl_item_t as size_t as isize),
            ) as *mut workitem_t;
        cmpres = strcmp((*vp).pattern, pattern);
        if cmpres > 0 as libc::c_int {
            itemp = (*itemp).ai_left;
        } else if cmpres < 0 as libc::c_int {
            itemp = (*itemp).ai_right;
        } else {
            return vp
        }
    }
    return 0 as *mut libc::c_void as *mut workitem_t;
}
unsafe extern "C" fn workitem_avl_insert(
    mut rootp: *mut avl_root_t,
    mut vpnew: *mut workitem_t,
) -> *mut workitem_t {
    let mut vp: *mut workitem_t = 0 as *mut workitem_t;
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut ptrp: *mut *mut avl_item_t = 0 as *mut *mut avl_item_t;
    let mut cmpres: libc::c_int = 0;
    itemp = 0 as *mut libc::c_void as *mut avl_item_t;
    ptrp = &mut (*rootp).ar_root;
    while !(*ptrp).is_null() {
        itemp = *ptrp;
        vp = (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut workitem_t)).avlent
                    as *mut avl_item_t as size_t as isize),
            ) as *mut workitem_t;
        cmpres = workitem_cmp(vp, vpnew);
        if cmpres > 0 as libc::c_int {
            ptrp = &mut (*itemp).ai_left;
        } else if cmpres < 0 as libc::c_int {
            ptrp = &mut (*itemp).ai_right;
        } else {
            return vp
        }
    }
    (*vpnew).avlent.ai_up = itemp;
    itemp = &mut (*vpnew).avlent;
    *ptrp = itemp;
    avl_insert_fix(rootp, itemp);
    return 0 as *mut libc::c_void as *mut workitem_t;
}
unsafe extern "C" fn workitem_avl_first(mut rootp: *mut avl_root_t) -> *mut workitem_t {
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    itemp = avl_first(rootp);
    if !itemp.is_null() {
        return (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut workitem_t)).avlent
                    as *mut avl_item_t as size_t as isize),
            ) as *mut workitem_t;
    }
    return 0 as *mut libc::c_void as *mut workitem_t;
}
unsafe extern "C" fn workitem_avl_next(mut vp: *mut workitem_t) -> *mut workitem_t {
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    itemp = &mut (*vp).avlent;
    itemp = avl_next(itemp);
    if !itemp.is_null() {
        return (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut workitem_t)).avlent
                    as *mut avl_item_t as size_t as isize),
            ) as *mut workitem_t;
    }
    return 0 as *mut libc::c_void as *mut workitem_t;
}
pub unsafe extern "C" fn server_workitem_free(mut wip: *mut workitem_t) {
    if !((*wip).pubkey).is_null() {
        EC_POINT_free((*wip).pubkey);
    }
    free(wip as *mut libc::c_void);
}
pub unsafe extern "C" fn server_batch_free(mut pbatch: *mut pubkeybatch_t) {
    let mut wip: *mut workitem_t = 0 as *mut workitem_t;
    loop {
        wip = workitem_avl_first(&mut (*pbatch).items);
        if !(wip as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        if (*wip).pubkey as libc::c_ulong == (*pbatch).pubkey as libc::c_ulong {
            (*wip).pubkey = 0 as *mut libc::c_void as *mut EC_POINT;
        }
        avl_remove(&mut (*pbatch).items, &mut (*wip).avlent);
        server_workitem_free(wip);
    }
    if !((*pbatch).pubkey).is_null() {
        EC_POINT_free((*pbatch).pubkey);
    }
    if !((*pbatch).pubkey_hex).is_null() {
        CRYPTO_free((*pbatch).pubkey_hex as *mut libc::c_char as *mut libc::c_void);
    }
    free(pbatch as *mut libc::c_void);
}
unsafe extern "C" fn pubkeybatch_cmp(
    mut a: *mut pubkeybatch_t,
    mut b: *mut pubkeybatch_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = strcmp((*a).pubkey_hex, (*b).pubkey_hex);
    return tmp;
}
unsafe extern "C" fn pubkeybatch_avl_search(
    mut rootp: *mut avl_root_t,
    mut pubkey: *const EC_POINT,
    mut pgroup: *const EC_GROUP,
) -> *mut pubkeybatch_t {
    let mut pubkey_hex: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vp: *mut pubkeybatch_t = 0 as *mut pubkeybatch_t;
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut cmpres: libc::c_int = 0;
    itemp = (*rootp).ar_root;
    pubkey_hex = EC_POINT_point2hex(
        pgroup,
        pubkey,
        POINT_CONVERSION_UNCOMPRESSED,
        0 as *mut libc::c_void as *mut BN_CTX,
    );
    while !itemp.is_null() {
        vp = (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut pubkeybatch_t)).avlent
                    as *mut avl_item_t as size_t as isize),
            ) as *mut pubkeybatch_t;
        cmpres = strcmp(pubkey_hex as *const libc::c_char, (*vp).pubkey_hex);
        if cmpres > 0 as libc::c_int {
            itemp = (*itemp).ai_left;
        } else if cmpres < 0 as libc::c_int {
            itemp = (*itemp).ai_right;
        } else {
            CRYPTO_free(pubkey_hex as *mut libc::c_void);
            return vp;
        }
    }
    CRYPTO_free(pubkey_hex as *mut libc::c_void);
    return 0 as *mut libc::c_void as *mut pubkeybatch_t;
}
unsafe extern "C" fn pubkeybatch_avl_insert(
    mut rootp: *mut avl_root_t,
    mut vpnew: *mut pubkeybatch_t,
) -> *mut pubkeybatch_t {
    let mut vp: *mut pubkeybatch_t = 0 as *mut pubkeybatch_t;
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut ptrp: *mut *mut avl_item_t = 0 as *mut *mut avl_item_t;
    let mut cmpres: libc::c_int = 0;
    itemp = 0 as *mut libc::c_void as *mut avl_item_t;
    ptrp = &mut (*rootp).ar_root;
    while !(*ptrp).is_null() {
        itemp = *ptrp;
        vp = (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut pubkeybatch_t)).avlent
                    as *mut avl_item_t as size_t as isize),
            ) as *mut pubkeybatch_t;
        cmpres = pubkeybatch_cmp(vpnew, vp);
        if cmpres > 0 as libc::c_int {
            ptrp = &mut (*itemp).ai_left;
        } else if cmpres < 0 as libc::c_int {
            ptrp = &mut (*itemp).ai_right;
        } else {
            return vp
        }
    }
    (*vpnew).avlent.ai_up = itemp;
    itemp = &mut (*vpnew).avlent;
    *ptrp = itemp;
    avl_insert_fix(rootp, itemp);
    return 0 as *mut libc::c_void as *mut pubkeybatch_t;
}
unsafe extern "C" fn pubkeybatch_avl_first(
    mut rootp: *mut avl_root_t,
) -> *mut pubkeybatch_t {
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    itemp = avl_first(rootp);
    if !itemp.is_null() {
        return (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut pubkeybatch_t)).avlent
                    as *mut avl_item_t as size_t as isize),
            ) as *mut pubkeybatch_t;
    }
    return 0 as *mut libc::c_void as *mut pubkeybatch_t;
}
unsafe extern "C" fn pubkeybatch_avl_next(
    mut vp: *mut pubkeybatch_t,
) -> *mut pubkeybatch_t {
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    itemp = &mut (*vp).avlent;
    itemp = avl_next(itemp);
    if !itemp.is_null() {
        return (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut pubkeybatch_t)).avlent
                    as *mut avl_item_t as size_t as isize),
            ) as *mut pubkeybatch_t;
    }
    return 0 as *mut libc::c_void as *mut pubkeybatch_t;
}
unsafe extern "C" fn server_workitem_new(
    mut reqp: *mut server_request_t,
    mut pfx: *const libc::c_char,
    mut pubkey_s: *const libc::c_char,
    mut addrtype_s: *const libc::c_char,
    mut reward_s: *const libc::c_char,
    mut comment: *const libc::c_char,
) -> *mut workitem_t {
    let mut wip: *mut workitem_t = 0 as *mut workitem_t;
    let mut pubkey: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut addrtype: libc::c_int = 0;
    let mut reward: libc::c_double = 0.;
    let mut difficulty: libc::c_double = 0.;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: size_t = 0;
    addrtype = atoi(addrtype_s);
    if addrtype < 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut workitem_t
    } else {
        if addrtype > 255 as libc::c_int {
            return 0 as *mut libc::c_void as *mut workitem_t;
        }
    }
    reward = strtod(reward_s, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    if reward < 0.0f64 {
        return 0 as *mut libc::c_void as *mut workitem_t;
    }
    difficulty = vg_prefix_get_difficulty(addrtype, pfx);
    if difficulty == 0.0f64 {
        return 0 as *mut libc::c_void as *mut workitem_t;
    }
    pubkey = EC_POINT_hex2point(
        (*reqp).group,
        pubkey_s,
        0 as *mut libc::c_void as *mut EC_POINT,
        0 as *mut libc::c_void as *mut BN_CTX,
    );
    if pubkey as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut workitem_t;
    }
    tmp = strlen(pfx);
    tmp___0 = strlen(comment);
    tmp___1 = malloc(
        (::std::mem::size_of::<workitem_t>() as libc::c_ulong)
            .wrapping_add(tmp)
            .wrapping_add(tmp___0)
            .wrapping_add(2 as libc::c_ulong),
    );
    wip = tmp___1 as *mut workitem_t;
    memset(
        wip as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<workitem_t>() as libc::c_ulong,
    );
    avl_item_init(&mut (*wip).avlent);
    (*wip)
        .pattern = wip.offset(1 as libc::c_int as isize) as *mut libc::c_char
        as *const libc::c_char;
    strcpy((*wip).pattern as *mut libc::c_char, pfx);
    tmp___2 = strlen((*wip).pattern);
    (*wip)
        .comment = ((*wip).pattern)
        .offset(tmp___2.wrapping_add(1 as libc::c_ulong) as isize);
    strcpy((*wip).comment as *mut libc::c_char, comment);
    (*wip).pubkey = pubkey;
    (*wip).addrtype = addrtype;
    (*wip).difficulty = difficulty;
    (*wip).reward = reward;
    (*wip).value = reward * 1000000000.0f64 / difficulty;
    return wip;
}
unsafe extern "C" fn server_workitem_equal(
    mut a: *mut workitem_t,
    mut b: *mut workitem_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if (*a).reward == (*b).reward {
        tmp = strcmp((*a).pattern, (*b).pattern);
        if tmp != 0 {
            tmp___0 = 0 as libc::c_int;
        } else {
            tmp___0 = 1 as libc::c_int;
        }
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0;
}
unsafe extern "C" fn server_pubkeybatch_equal(
    mut ctxp: *mut server_context_t,
    mut a: *mut pubkeybatch_t,
    mut b: *mut pubkeybatch_t,
) -> libc::c_int {
    let mut wipa: *mut workitem_t = 0 as *mut workitem_t;
    let mut wipb: *mut workitem_t = 0 as *mut workitem_t;
    let mut tmp: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if (*a).nitems != (*b).nitems {
        return 0 as libc::c_int;
    }
    tmp = EC_KEY_get0_group((*ctxp).dummy_key as *const EC_KEY);
    tmp___0 = EC_POINT_cmp(
        tmp,
        (*a).pubkey as *const EC_POINT,
        (*b).pubkey as *const EC_POINT,
        0 as *mut libc::c_void as *mut BN_CTX,
    );
    if tmp___0 != 0 {
        return 0 as libc::c_int;
    }
    wipa = workitem_avl_first(&mut (*a).items);
    wipb = workitem_avl_first(&mut (*b).items);
    while !wipa.is_null() {
        if wipb.is_null() {
            break;
        }
        tmp___1 = server_workitem_equal(wipa, wipb);
        if tmp___1 == 0 {
            return 0 as libc::c_int;
        }
        wipa = workitem_avl_next(wipa);
        wipb = workitem_avl_next(wipb);
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn server_context_free(mut ctxp: *mut server_context_t) {
    if !((*ctxp).dummy_key).is_null() {
        EC_KEY_free((*ctxp).dummy_key);
    }
    if !((*ctxp).getwork).is_null() {
        free((*ctxp).getwork as *mut libc::c_void);
    }
    if !((*ctxp).submit).is_null() {
        free((*ctxp).submit as *mut libc::c_void);
    }
    free(ctxp as *mut libc::c_void);
}
pub unsafe extern "C" fn server_context_new(
    mut url: *const libc::c_char,
    mut credit_addr: *const libc::c_char,
) -> *mut server_context_t {
    let mut ctxp: *mut server_context_t = 0 as *mut server_context_t;
    let mut urllen: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut addrlen: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = strlen(url);
    urllen = tmp as libc::c_int;
    tmp___0 = strlen(credit_addr);
    addrlen = tmp___0 as libc::c_int;
    tmp___1 = malloc(
        (::std::mem::size_of::<server_context_t>() as libc::c_ulong)
            .wrapping_add(urllen as libc::c_ulong)
            .wrapping_add(addrlen as libc::c_ulong)
            .wrapping_add(2 as libc::c_ulong),
    );
    ctxp = tmp___1 as *mut server_context_t;
    memset(
        ctxp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_context_t>() as libc::c_ulong,
    );
    avl_root_init(&mut (*ctxp).items);
    (*ctxp).url = ctxp.offset(1 as libc::c_int as isize) as *const libc::c_char;
    (*ctxp)
        .credit_addr = ((*ctxp).url)
        .offset(urllen as isize)
        .offset(1 as libc::c_int as isize);
    strcpy((*ctxp).url as *mut libc::c_char, url);
    strcpy((*ctxp).credit_addr as *mut libc::c_char, credit_addr);
    (*ctxp).dummy_key = vg_exec_context_new_key();
    tmp___2 = malloc((urllen + 9 as libc::c_int) as size_t);
    (*ctxp).getwork = tmp___2 as *mut libc::c_char;
    tmp___3 = malloc((urllen + 7 as libc::c_int) as size_t);
    (*ctxp).submit = tmp___3 as *mut libc::c_char;
    if *url.offset((urllen - 1 as libc::c_int) as isize) as libc::c_int
        == 47 as libc::c_int
    {
        snprintf(
            (*ctxp).getwork,
            (urllen + 9 as libc::c_int) as size_t,
            b"%sgetWork\0" as *const u8 as *const libc::c_char,
            url,
        );
        snprintf(
            (*ctxp).submit,
            (urllen + 7 as libc::c_int) as size_t,
            b"%ssolve\0" as *const u8 as *const libc::c_char,
            url,
        );
    } else {
        snprintf(
            (*ctxp).getwork,
            (urllen + 9 as libc::c_int) as size_t,
            b"%s/getWork\0" as *const u8 as *const libc::c_char,
            url,
        );
        snprintf(
            (*ctxp).submit,
            (urllen + 7 as libc::c_int) as size_t,
            b"%s/solve\0" as *const u8 as *const libc::c_char,
            url,
        );
    }
    return ctxp;
}
pub unsafe extern "C" fn server_workitem_add(
    mut reqp: *mut server_request_t,
    mut wip: *mut workitem_t,
) -> libc::c_int {
    let mut xwip: *mut workitem_t = 0 as *mut workitem_t;
    let mut pbatch: *mut pubkeybatch_t = 0 as *mut pubkeybatch_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    pbatch = 0 as *mut libc::c_void as *mut pubkeybatch_t;
    pbatch = pubkeybatch_avl_search(
        &mut (*reqp).items,
        (*wip).pubkey as *const EC_POINT,
        (*reqp).group,
    );
    if pbatch as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = malloc(::std::mem::size_of::<pubkeybatch_t>() as libc::c_ulong);
        pbatch = tmp as *mut pubkeybatch_t;
        if pbatch as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        memset(
            pbatch as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<pubkeybatch_t>() as libc::c_ulong,
        );
        avl_item_init(&mut (*pbatch).avlent);
        avl_root_init(&mut (*pbatch).items);
        (*pbatch).total_value = 0 as libc::c_int as libc::c_double;
        (*pbatch).pubkey = (*wip).pubkey;
        tmp___0 = EC_POINT_point2hex(
            (*reqp).group,
            (*wip).pubkey as *const EC_POINT,
            POINT_CONVERSION_UNCOMPRESSED,
            0 as *mut libc::c_void as *mut BN_CTX,
        );
        (*pbatch).pubkey_hex = tmp___0 as *const libc::c_char;
        pubkeybatch_avl_insert(&mut (*reqp).items, pbatch);
        (*reqp).nitems += 1;
    }
    xwip = workitem_avl_insert(&mut (*pbatch).items, wip);
    if !xwip.is_null() {
        return -(1 as libc::c_int);
    }
    if !((*wip).pubkey).is_null() {
        if (*wip).pubkey as libc::c_ulong != (*pbatch).pubkey as libc::c_ulong {
            EC_POINT_free((*wip).pubkey);
        }
    }
    (*wip).pubkey = (*pbatch).pubkey;
    (*pbatch).nitems += 1;
    (*pbatch).total_value += (*wip).value;
    return 0 as libc::c_int;
}
unsafe extern "C" fn server_body_reader(
    mut buf: *const libc::c_char,
    mut elemsize: size_t,
    mut len: size_t,
    mut param: *mut libc::c_void,
) -> size_t {
    let mut reqp: *mut server_request_t = 0 as *mut server_request_t;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pfx: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pubkey_s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut addrtype_s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut reward_s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut comment: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wip: *mut workitem_t = 0 as *mut workitem_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    reqp = param as *mut server_request_t;
    if len == 0 {
        return 0 as libc::c_int as size_t;
    }
    if (*reqp).part_size
        < ((*reqp).part_end).wrapping_add(len).wrapping_add(1 as libc::c_ulong)
    {
        if (*reqp).part_off > 0 as libc::c_ulong {
            memmove(
                (*reqp).part_buf as *mut libc::c_void,
                ((*reqp).part_buf).offset((*reqp).part_off as isize)
                    as *const libc::c_void,
                ((*reqp).part_end).wrapping_sub((*reqp).part_off),
            );
            (*reqp)
                .part_end = ((*reqp).part_end as libc::c_ulong)
                .wrapping_sub((*reqp).part_off) as size_t as size_t;
            (*reqp).part_off = 0 as libc::c_int as size_t;
        }
    }
    if (*reqp).part_size
        < ((*reqp).part_end).wrapping_add(len).wrapping_add(1 as libc::c_ulong)
    {
        if (*reqp).part_size == 0 as libc::c_ulong {
            (*reqp).part_size = 4096 as libc::c_int as size_t;
        }
        while (*reqp).part_size
            < ((*reqp).part_end).wrapping_add(len).wrapping_add(1 as libc::c_ulong)
        {
            (*reqp)
                .part_size = ((*reqp).part_size as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong) as size_t as size_t;
            if (*reqp).part_size > 1048576 as libc::c_ulong {
                fprintf(
                    stderr,
                    b"Line too long from server\0" as *const u8 as *const libc::c_char,
                );
                (*reqp).request_status = 0 as libc::c_int;
                return 0 as libc::c_int as size_t;
            }
        }
        tmp = realloc((*reqp).part_buf as *mut libc::c_void, (*reqp).part_size);
        (*reqp).part_buf = tmp as *mut libc::c_char;
        if ((*reqp).part_buf).is_null() {
            fprintf(stderr, b"Out of memory\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int as size_t;
        }
    }
    memcpy(
        ((*reqp).part_buf).offset((*reqp).part_end as isize) as *mut libc::c_void,
        buf as *const libc::c_void,
        len,
    );
    (*reqp)
        .part_end = ((*reqp).part_end as libc::c_ulong).wrapping_add(len) as size_t
        as size_t;
    *((*reqp).part_buf)
        .offset((*reqp).part_end as isize) = '\u{0}' as i32 as libc::c_char;
    line = ((*reqp).part_buf).offset((*reqp).part_off as isize);
    loop {
        sep = strchr(line as *const libc::c_char, '\n' as i32);
        if sep.is_null() {
            break;
        }
        pfx = line;
        *sep = '\u{0}' as i32 as libc::c_char;
        line = sep.offset(1 as libc::c_int as isize);
        sep = strchr(pfx as *const libc::c_char, ':' as i32);
        if sep.is_null() {
            continue;
        }
        *sep = '\u{0}' as i32 as libc::c_char;
        sep = sep.offset(1);
        pubkey_s = sep;
        sep = strchr(sep as *const libc::c_char, ':' as i32);
        if sep.is_null() {
            continue;
        }
        *sep = '\u{0}' as i32 as libc::c_char;
        sep = sep.offset(1);
        addrtype_s = sep;
        sep = strchr(sep as *const libc::c_char, ':' as i32);
        if sep.is_null() {
            continue;
        }
        *sep = '\u{0}' as i32 as libc::c_char;
        sep = sep.offset(1);
        reward_s = sep;
        sep = strchr(sep as *const libc::c_char, ';' as i32);
        if sep.is_null() {
            continue;
        }
        *sep = '\u{0}' as i32 as libc::c_char;
        sep = sep.offset(1);
        comment = sep;
        wip = server_workitem_new(
            reqp,
            pfx as *const libc::c_char,
            pubkey_s as *const libc::c_char,
            addrtype_s as *const libc::c_char,
            reward_s as *const libc::c_char,
            comment as *const libc::c_char,
        );
        if wip.is_null() {
            continue;
        }
        tmp___0 = server_workitem_add(reqp, wip);
        if !(tmp___0 != 0) {
            continue;
        }
        server_workitem_free(wip);
    }
    (*reqp).part_off = line.offset_from((*reqp).part_buf) as libc::c_long as size_t;
    if (*reqp).part_off == (*reqp).part_end {
        (*reqp).part_off = 0 as libc::c_int as size_t;
        (*reqp).part_end = 0 as libc::c_int as size_t;
    }
    return len;
}
pub unsafe extern "C" fn dump_work(mut work: *mut avl_root_t) {
    let mut pbatch: *mut pubkeybatch_t = 0 as *mut pubkeybatch_t;
    let mut wip: *mut workitem_t = 0 as *mut workitem_t;
    printf(b"Available bounties:\n\0" as *const u8 as *const libc::c_char);
    pbatch = pubkeybatch_avl_first(work);
    while pbatch as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        wip = workitem_avl_first(&mut (*pbatch).items);
        while wip as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            printf(
                b"Pattern: \"%s\" Reward: %f Value: %f BTC/Gkey\n\0" as *const u8
                    as *const libc::c_char,
                (*wip).pattern,
                (*wip).reward,
                (*wip).value,
            );
            wip = workitem_avl_next(wip);
        }
        if (*pbatch).nitems > 1 as libc::c_int {
            printf(
                b"Batch of %d, total value: %f BTC/Gkey\n\0" as *const u8
                    as *const libc::c_char,
                (*pbatch).nitems,
                (*pbatch).total_value,
            );
        }
        pbatch = pubkeybatch_avl_next(pbatch);
    }
}
pub unsafe extern "C" fn free_pkb_tree(
    mut rootp: *mut avl_root_t,
    mut save_pkb: *mut pubkeybatch_t,
) {
    let mut pkb: *mut pubkeybatch_t = 0 as *mut pubkeybatch_t;
    loop {
        pkb = pubkeybatch_avl_first(rootp);
        if !(pkb as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        avl_remove(rootp, &mut (*pkb).avlent);
        if pkb as libc::c_ulong != save_pkb as libc::c_ulong {
            server_batch_free(pkb);
        }
    };
}
pub unsafe extern "C" fn server_request_free(mut reqp: *mut server_request_t) {
    let mut tmp: libc::c_int = 0;
    if (*reqp).part_buf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*reqp).part_buf as *mut libc::c_void);
    }
    tmp = avl_root_empty(&mut (*reqp).items);
    if tmp == 0 {
        free_pkb_tree(&mut (*reqp).items, 0 as *mut libc::c_void as *mut pubkeybatch_t);
    }
    free(reqp as *mut libc::c_void);
}
pub unsafe extern "C" fn server_context_getwork(
    mut ctxp: *mut server_context_t,
) -> libc::c_int {
    let mut res: CURLcode = CURLE_OK;
    let mut reqp: *mut server_request_t = 0 as *mut server_request_t;
    let mut creq: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _curl_opt: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: CURLcode = CURLE_OK;
    let mut _curl_opt___0: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: CURLcode = CURLE_OK;
    let mut _curl_opt___1: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: CURLcode = CURLE_OK;
    let mut _curl_opt___2: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: CURLcode = CURLE_OK;
    let mut _curl_opt___3: libc::c_int = 0;
    let mut tmp___9: CURLcode = CURLE_OK;
    let mut tmp___10: *const libc::c_char = 0 as *const libc::c_char;
    tmp = malloc(::std::mem::size_of::<server_request_t>() as libc::c_ulong);
    reqp = tmp as *mut server_request_t;
    memset(
        reqp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_request_t>() as libc::c_ulong,
    );
    (*reqp).group = EC_KEY_get0_group((*ctxp).dummy_key as *const EC_KEY);
    creq = curl_easy_init();
    _curl_opt = 10002 as libc::c_int;
    tmp___2 = curl_easy_setopt(creq, _curl_opt as CURLoption, (*ctxp).getwork);
    if tmp___2 as u64 != 0 {
        fprintf(
            stderr,
            b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        _curl_opt___0 = 41 as libc::c_int;
        tmp___4 = curl_easy_setopt(
            creq,
            _curl_opt___0 as CURLoption,
            ((*ctxp).verbose > 1 as libc::c_int) as libc::c_int,
        );
        if tmp___4 as u64 != 0 {
            fprintf(
                stderr,
                b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        } else {
            _curl_opt___1 = 20011 as libc::c_int;
            tmp___6 = curl_easy_setopt(
                creq,
                _curl_opt___1 as CURLoption,
                Some(
                    server_body_reader
                        as unsafe extern "C" fn(
                            *const libc::c_char,
                            size_t,
                            size_t,
                            *mut libc::c_void,
                        ) -> size_t,
                ),
            );
            if tmp___6 as u64 != 0 {
                fprintf(
                    stderr,
                    b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            } else {
                _curl_opt___2 = 52 as libc::c_int;
                tmp___8 = curl_easy_setopt(
                    creq,
                    _curl_opt___2 as CURLoption,
                    1 as libc::c_int,
                );
                if tmp___8 as u64 != 0 {
                    fprintf(
                        stderr,
                        b"Failed to set up libcurl\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                } else {
                    _curl_opt___3 = 10001 as libc::c_int;
                    tmp___9 = curl_easy_setopt(creq, _curl_opt___3 as CURLoption, reqp);
                    if tmp___9 as u64 != 0 {
                        fprintf(
                            stderr,
                            b"Failed to set up libcurl\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    res = curl_easy_perform(creq);
                    curl_easy_cleanup(creq);
                    if res as libc::c_uint != 0 as libc::c_uint {
                        tmp___10 = curl_easy_strerror(res);
                        fprintf(
                            stderr,
                            b"Get work request failed: %s\n\0" as *const u8
                                as *const libc::c_char,
                            tmp___10,
                        );
                        server_request_free(reqp);
                        return -(1 as libc::c_int);
                    }
                    (*ctxp).items.ar_root = (*reqp).items.ar_root;
                    return 0 as libc::c_int;
                }
            }
        }
    };
}
pub unsafe extern "C" fn server_context_submit_solution(
    mut ctxp: *mut server_context_t,
    mut work: *mut workitem_t,
    mut privkey: *const libc::c_char,
) -> libc::c_int {
    let mut urlbuf: [libc::c_char; 8192] = [0; 8192];
    let mut pubhex: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut creq: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut res: CURLcode = CURLE_OK;
    let mut tmp: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut _curl_opt: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: CURLcode = CURLE_OK;
    let mut _curl_opt___0: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: CURLcode = CURLE_OK;
    let mut _curl_opt___1: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: CURLcode = CURLE_OK;
    let mut tmp___7: *const libc::c_char = 0 as *const libc::c_char;
    tmp = EC_KEY_get0_group((*ctxp).dummy_key as *const EC_KEY);
    pubhex = EC_POINT_point2hex(
        tmp,
        (*work).pubkey as *const EC_POINT,
        POINT_CONVERSION_UNCOMPRESSED,
        0 as *mut libc::c_void as *mut BN_CTX,
    );
    snprintf(
        urlbuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong,
        b"%s?key=%s%%3A%s&privateKey=%s&bitcoinAddress=%s\0" as *const u8
            as *const libc::c_char,
        (*ctxp).submit,
        (*work).pattern,
        pubhex,
        privkey,
        (*ctxp).credit_addr,
    );
    CRYPTO_free(pubhex as *mut libc::c_void);
    creq = curl_easy_init();
    _curl_opt = 10002 as libc::c_int;
    tmp___2 = curl_easy_setopt(creq, _curl_opt as CURLoption, urlbuf.as_mut_ptr());
    if tmp___2 as u64 != 0 {
        fprintf(
            stderr,
            b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        _curl_opt___0 = 41 as libc::c_int;
        tmp___4 = curl_easy_setopt(
            creq,
            _curl_opt___0 as CURLoption,
            ((*ctxp).verbose > 1 as libc::c_int) as libc::c_int,
        );
        if tmp___4 as u64 != 0 {
            fprintf(
                stderr,
                b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        } else {
            _curl_opt___1 = 52 as libc::c_int;
            tmp___6 = curl_easy_setopt(
                creq,
                _curl_opt___1 as CURLoption,
                1 as libc::c_int,
            );
            if tmp___6 as u64 != 0 {
                fprintf(
                    stderr,
                    b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            res = curl_easy_perform(creq);
            if res as libc::c_uint != 0 as libc::c_uint {
                tmp___7 = curl_easy_strerror(res);
                fprintf(
                    stderr,
                    b"Submission failed: %s\n\0" as *const u8 as *const libc::c_char,
                    tmp___7,
                );
                curl_easy_cleanup(creq);
                return -(1 as libc::c_int);
            }
            curl_easy_cleanup(creq);
            return 0 as libc::c_int;
        }
    };
}
static mut soln_lock: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut soln_cond: pthread_cond_t = __anonunion_pthread_cond_t_951761805 {
    __data: __pthread_cond_s {
        __annonCompField1: __anonunion____missing_field_name_622077928 {
            __wseq: 0,
        },
        __annonCompField2: __anonunion____missing_field_name_119281977 {
            __g1_start: 0,
        },
        __g_refs: [0; 2],
        __g_size: [0; 2],
        __g1_orig_size: 0,
        __wrefs: 0,
        __g_signals: [0; 2],
    },
};
static mut soln_pattern: *mut libc::c_char = 0 as *const libc::c_void
    as *mut libc::c_void as *mut libc::c_char;
static mut soln_private_key: *mut libc::c_char = 0 as *const libc::c_void
    as *mut libc::c_void as *mut libc::c_char;
pub unsafe extern "C" fn free_soln() {
    if !soln_pattern.is_null() {
        free(soln_pattern as *mut libc::c_void);
        soln_pattern = 0 as *mut libc::c_void as *mut libc::c_char;
    }
    if !soln_private_key.is_null() {
        CRYPTO_free(soln_private_key as *mut libc::c_void);
        soln_private_key = 0 as *mut libc::c_void as *mut libc::c_char;
    }
}
pub unsafe extern "C" fn output_match_work_complete(
    mut vcp: *mut vg_context_t,
    mut pkey: *mut EC_KEY,
    mut pattern: *const libc::c_char,
) {
    let mut tmp: *const BIGNUM = 0 as *const BIGNUM;
    vg_output_match_console(vcp, pkey, pattern);
    pthread_mutex_lock(&mut soln_lock);
    free_soln();
    soln_pattern = strdup(pattern);
    tmp = EC_KEY_get0_private_key(pkey as *const EC_KEY);
    soln_private_key = BN_bn2hex(tmp);
    (*vcp).vc_halt = 1 as libc::c_int;
    pthread_cond_broadcast(&mut soln_cond);
    pthread_mutex_unlock(&mut soln_lock);
}
pub unsafe extern "C" fn check_solution(
    mut scp: *mut server_context_t,
    mut pbatch: *mut pubkeybatch_t,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut wip: *mut workitem_t = 0 as *mut workitem_t;
    let mut tmp: *mut workitem_t = 0 as *mut workitem_t;
    res = 0 as libc::c_int;
    pthread_mutex_lock(&mut soln_lock);
    if soln_private_key as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = workitem_avl_search(
            &mut (*pbatch).items,
            soln_pattern as *const libc::c_char,
        );
        wip = tmp;
        if !(wip as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            __assert_fail(
                b"wip != NULL\0" as *const u8 as *const libc::c_char,
                b"oclvanityminer.c\0" as *const u8 as *const libc::c_char,
                712 as libc::c_uint,
                b"check_solution\0" as *const u8 as *const libc::c_char,
            );
        }
        avl_remove(&mut (*pbatch).items, &mut (*wip).avlent);
        (*pbatch).nitems -= 1;
        (*pbatch).total_value -= (*wip).value;
        server_context_submit_solution(
            scp,
            wip,
            soln_private_key as *const libc::c_char,
        );
        if (*wip).pubkey as libc::c_ulong == (*pbatch).pubkey as libc::c_ulong {
            (*wip).pubkey = 0 as *mut libc::c_void as *mut EC_POINT;
        }
        server_workitem_free(wip);
        free_soln();
        res = 1 as libc::c_int;
    }
    pthread_mutex_unlock(&mut soln_lock);
    return res;
}
pub unsafe extern "C" fn most_valuable_pkb(
    mut scp: *mut server_context_t,
) -> *mut pubkeybatch_t {
    let mut pbatch: *mut pubkeybatch_t = 0 as *mut pubkeybatch_t;
    let mut res: *mut pubkeybatch_t = 0 as *mut pubkeybatch_t;
    res = 0 as *mut libc::c_void as *mut pubkeybatch_t;
    pbatch = pubkeybatch_avl_first(&mut (*scp).items);
    while pbatch as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if res.is_null() {
            res = pbatch;
        } else if (*res).total_value < (*pbatch).total_value {
            res = pbatch;
        }
        pbatch = pubkeybatch_avl_next(pbatch);
    }
    return res;
}
pub unsafe extern "C" fn usage(mut name: *const libc::c_char) {
    fprintf(
        stderr,
        b"oclVanityMiner %s (OpenSSL 1.0.1u  22 Sep 2016)\nUsage: %s -u <URL> -a <credit address>\nOrganized vanity address mining client using OpenCL.  Contacts the specified\nbounty pool server, downloads a list of active bounties, and attempts to\ngenerate the address with the best difficulty to reward ratio.  Maintains\ncontact with the bounty pool server and periodically refreshes the bounty\nlist.\nBy default, if no device is specified, and the system has exactly one OpenCL\ndevice, it will be selected automatically, otherwise if the system has\nmultiple OpenCL devices and no device is specified, an error will be\nreported.  To use multiple devices simultaneously, specify the -D option for\neach device.\n\nOptions:\n-u <URL>      Bounty pool URL\n-a <address>  Credit address for completed work\n-i <interval> Set server polling interval in seconds (default 90)\n-v            Verbose output\n-q            Quiet output\n-p <platform> Select OpenCL platform\n-d <device>   Select OpenCL device\n-D <devstr>   Use OpenCL device, identified by device string\n              Form: <platform>:<devicenumber>[,<options>]\n              Example: 0:0,grid=1024x1024\n-S            Safe mode, disable OpenCL loop unrolling optimizations\n-w <worksize> Set work items per thread in a work unit\n-t <threads>  Set target thread count per multiprocessor\n-g <x>x<y>    Set grid size\n-b <invsize>  Set modular inverse ops per thread\n-V            Enable kernel/OpenCL/hardware verification (SLOW)\n\0"
            as *const u8 as *const libc::c_char,
        version,
        name,
    );
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut url: *const libc::c_char = 0 as *const libc::c_char;
    let mut credit_addr: *const libc::c_char = 0 as *const libc::c_char;
    let mut opt: libc::c_int = 0;
    let mut platformidx: libc::c_int = 0;
    let mut deviceidx: libc::c_int = 0;
    let mut pend: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut verbose: libc::c_int = 0;
    let mut interval: libc::c_int = 0;
    let mut nthreads: libc::c_int = 0;
    let mut worksize: libc::c_int = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut invsize: libc::c_int = 0;
    let mut verify_mode: libc::c_int = 0;
    let mut safe_mode: libc::c_int = 0;
    let mut devstrs: [*mut libc::c_char; 32] = [0 as *mut libc::c_char; 32];
    let mut ndevstrs: libc::c_int = 0;
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    let mut vocp: *mut vg_ocl_context_t = 0 as *mut vg_ocl_context_t;
    let mut res: libc::c_int = 0;
    let mut thread_started: libc::c_int = 0;
    let mut active_pkb: *mut pubkeybatch_t = 0 as *mut pubkeybatch_t;
    let mut active_pkb_value: libc::c_float = 0.;
    let mut scp: *mut server_context_t = 0 as *mut server_context_t;
    let mut pkb: *mut pubkeybatch_t = 0 as *mut pubkeybatch_t;
    let mut was_sleeping: libc::c_int = 0;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut sleepy: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut tmp: libc::c_long = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut wip: *mut workitem_t = 0 as *mut workitem_t;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    url = 0 as *mut libc::c_void as *const libc::c_char;
    credit_addr = 0 as *mut libc::c_void as *const libc::c_char;
    platformidx = -(1 as libc::c_int);
    deviceidx = -(1 as libc::c_int);
    verbose = 1 as libc::c_int;
    interval = 90 as libc::c_int;
    nthreads = 0 as libc::c_int;
    worksize = 0 as libc::c_int;
    nrows = 0 as libc::c_int;
    ncols = 0 as libc::c_int;
    invsize = 0 as libc::c_int;
    verify_mode = 0 as libc::c_int;
    safe_mode = 0 as libc::c_int;
    ndevstrs = 0 as libc::c_int;
    vcp = 0 as *mut libc::c_void as *mut vg_context_t;
    vocp = 0 as *mut libc::c_void as *mut vg_ocl_context_t;
    thread_started = 0 as libc::c_int;
    active_pkb = 0 as *mut libc::c_void as *mut pubkeybatch_t;
    active_pkb_value = 0 as libc::c_int as libc::c_float;
    scp = 0 as *mut libc::c_void as *mut server_context_t;
    was_sleeping = 0 as libc::c_int;
    pthread_mutex_init(
        &mut soln_lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    pthread_cond_init(
        &mut soln_cond as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    if argc == 1 as libc::c_int {
        usage(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char);
        return 1 as libc::c_int;
    }
    loop {
        opt = getopt(
            argc,
            argv as *const *mut libc::c_char,
            b"u:a:vqp:d:w:t:g:b:VD:Sh?i:\0" as *const u8 as *const libc::c_char,
        );
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        match opt {
            117 => {
                url = optarg as *const libc::c_char;
            }
            97 => {
                credit_addr = optarg as *const libc::c_char;
            }
            118 => {
                verbose = 2 as libc::c_int;
            }
            113 => {
                verbose = 0 as libc::c_int;
            }
            105 => {
                interval = atoi(optarg as *const libc::c_char);
                if interval < 10 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Invalid interval '%s'\n\0" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    return 1 as libc::c_int;
                }
            }
            112 => {
                platformidx = atoi(optarg as *const libc::c_char);
            }
            100 => {
                deviceidx = atoi(optarg as *const libc::c_char);
            }
            119 => {
                worksize = atoi(optarg as *const libc::c_char);
                if worksize == 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Invalid work size '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    return 1 as libc::c_int;
                }
            }
            116 => {
                nthreads = atoi(optarg as *const libc::c_char);
                if nthreads == 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Invalid thread count '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    return 1 as libc::c_int;
                }
            }
            103 => {
                nrows = 0 as libc::c_int;
                tmp = strtol(
                    optarg as *const libc::c_char,
                    &mut pend as *mut *mut libc::c_char,
                    0 as libc::c_int,
                );
                ncols = tmp as libc::c_int;
                if !pend.is_null() {
                    if *pend as libc::c_int == 120 as libc::c_int {
                        tmp___0 = strtol(
                            pend.offset(1 as libc::c_int as isize)
                                as *const libc::c_char,
                            0 as *mut libc::c_void as *mut *mut libc::c_char,
                            0 as libc::c_int,
                        );
                        nrows = tmp___0 as libc::c_int;
                    }
                }
                if nrows == 0 {
                    fprintf(
                        stderr,
                        b"Invalid grid size '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    return 1 as libc::c_int;
                } else {
                    if ncols == 0 {
                        fprintf(
                            stderr,
                            b"Invalid grid size '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            optarg,
                        );
                        return 1 as libc::c_int;
                    }
                }
            }
            98 => {
                invsize = atoi(optarg as *const libc::c_char);
                if invsize == 0 {
                    fprintf(
                        stderr,
                        b"Invalid modular inverse size '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        optarg,
                    );
                    return 1 as libc::c_int;
                }
                if invsize & invsize - 1 as libc::c_int != 0 {
                    fprintf(
                        stderr,
                        b"Modular inverse size must be a power of 2\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
            }
            86 => {
                verify_mode = 1 as libc::c_int;
            }
            83 => {
                safe_mode = 1 as libc::c_int;
            }
            68 => {
                if ndevstrs >= 32 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Too many OpenCL devices (limit %d)\n\0" as *const u8
                            as *const libc::c_char,
                        32 as libc::c_int,
                    );
                    return 1 as libc::c_int;
                }
                tmp___1 = ndevstrs;
                ndevstrs += 1;
                devstrs[tmp___1 as usize] = optarg;
            }
            _ => {
                usage(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char);
                return 1 as libc::c_int;
            }
        }
    }
    curl_easy_init();
    vcp = vg_prefix_context_new(0 as libc::c_int, 128 as libc::c_int, 0 as libc::c_int);
    (*vcp).vc_verbose = verbose;
    (*vcp)
        .vc_output_match = Some(
        output_match_work_complete
            as unsafe extern "C" fn(
                *mut vg_context_t,
                *mut EC_KEY,
                *const libc::c_char,
            ) -> (),
    );
    (*vcp)
        .vc_output_timing = Some(
        vg_output_timing_console
            as unsafe extern "C" fn(
                *mut vg_context_t,
                libc::c_double,
                libc::c_ulonglong,
                libc::c_ulonglong,
            ) -> (),
    );
    if url.is_null() {
        fprintf(
            stderr,
            b"ERROR: No server URL specified\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if credit_addr.is_null() {
        fprintf(
            stderr,
            b"ERROR: No reward address specified\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    tmp___2 = vg_b58_decode_check(
        credit_addr,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
    );
    if tmp___2 == 0 {
        fprintf(
            stderr,
            b"ERROR: Invalid reward address specified\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    scp = server_context_new(url, credit_addr);
    (*scp).verbose = verbose;
    tmp___3 = server_context_getwork(scp);
    if tmp___3 != 0 {
        return 1 as libc::c_int;
    }
    res = 0 as libc::c_int;
    if ndevstrs != 0 {
        opt = 0 as libc::c_int;
        while opt < ndevstrs {
            vocp = vg_ocl_context_new_from_devstr(
                vcp,
                devstrs[opt as usize] as *const libc::c_char,
                safe_mode,
                verify_mode,
            );
            if vocp.is_null() {
                fprintf(
                    stderr,
                    b"Could not open device '%s', ignoring\n\0" as *const u8
                        as *const libc::c_char,
                    devstrs[opt as usize],
                );
            } else {
                res += 1;
            }
            opt += 1;
        }
    } else {
        vocp = vg_ocl_context_new(
            vcp,
            platformidx,
            deviceidx,
            safe_mode,
            verify_mode,
            worksize,
            nthreads,
            nrows,
            ncols,
            invsize,
        );
        if !vocp.is_null() {
            res += 1;
        }
    }
    if res == 0 {
        vg_ocl_enumerate_devices();
        return 1 as libc::c_int;
    }
    if verbose > 1 as libc::c_int {
        dump_work(&mut (*scp).items);
    }
    loop {
        tmp___4 = avl_root_empty(&mut (*scp).items);
        if tmp___4 != 0 {
            server_context_getwork(scp);
        }
        pkb = most_valuable_pkb(scp);
        if !pkb.is_null() {
            if !active_pkb.is_null() {
                tmp___5 = server_pubkeybatch_equal(scp, active_pkb, pkb);
                if tmp___5 != 0 {
                    pkb = active_pkb;
                }
            }
        }
        if thread_started != 0 {
            let mut current_block_159: u64;
            if active_pkb.is_null() {
                current_block_159 = 2424895904571516763;
            } else if pkb as libc::c_ulong != active_pkb as libc::c_ulong {
                current_block_159 = 2424895904571516763;
            } else {
                current_block_159 = 939350892795860671;
            }
            match current_block_159 {
                2424895904571516763 => {
                    vg_context_stop_threads(vcp);
                    thread_started = 0 as libc::c_int;
                    if !active_pkb.is_null() {
                        check_solution(scp, active_pkb);
                        active_pkb = 0 as *mut libc::c_void as *mut pubkeybatch_t;
                    }
                    vg_context_clear_all_patterns(vcp);
                    if verbose > 1 as libc::c_int {
                        dump_work(&mut (*scp).items);
                    }
                }
                _ => {}
            }
        }
        if pkb.is_null() {
            if was_sleeping == 0 {
                fprintf(
                    stderr,
                    b"No work available, sleeping\n\0" as *const u8
                        as *const libc::c_char,
                );
                was_sleeping = 1 as libc::c_int;
            }
        } else if active_pkb.is_null() {
            was_sleeping = 0 as libc::c_int;
            active_pkb_value = 0 as libc::c_int as libc::c_float;
            (*vcp).vc_pubkey_base = (*pkb).pubkey;
            wip = workitem_avl_first(&mut (*pkb).items);
            while wip as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                fprintf(
                    stderr,
                    b"Searching for pattern: \"%s\" Reward: %f Value: %f BTC/Gkey\n\0"
                        as *const u8 as *const libc::c_char,
                    (*wip).pattern,
                    (*wip).reward,
                    (*wip).value,
                );
                (*vcp).vc_addrtype = (*wip).addrtype;
                tmp___6 = vg_context_add_patterns(
                    vcp,
                    &mut (*wip).pattern as *mut *const libc::c_char,
                    1 as libc::c_int,
                );
                if tmp___6 != 0 {
                    active_pkb_value = (active_pkb_value as libc::c_double
                        + (*wip).value) as libc::c_float;
                } else {
                    fprintf(
                        stderr,
                        b"WARNING: could not add pattern\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if (*vcp).vc_npatterns == 0 {
                    __assert_fail(
                        b"vcp->vc_npatterns\0" as *const u8 as *const libc::c_char,
                        b"oclvanityminer.c\0" as *const u8 as *const libc::c_char,
                        1042 as libc::c_uint,
                        b"main\0" as *const u8 as *const libc::c_char,
                    );
                }
                wip = workitem_avl_next(wip);
            }
            fprintf(
                stderr,
                b"\nTotal value for current work: %f BTC/Gkey\n\0" as *const u8
                    as *const libc::c_char,
                active_pkb_value as libc::c_double,
            );
            res = vg_context_start_threads(vcp);
            if res != 0 {
                return 1 as libc::c_int;
            }
            thread_started = 1 as libc::c_int;
            active_pkb = pkb;
        }
        gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
        sleepy.tv_sec = tv.tv_sec;
        sleepy.tv_nsec = tv.tv_usec * 1000 as libc::c_long;
        sleepy.tv_sec += interval as __time_t;
        pthread_mutex_lock(&mut soln_lock);
        res = 0 as libc::c_int;
        if soln_private_key.is_null() {
            res = pthread_cond_timedwait(
                &mut soln_cond as *mut pthread_cond_t,
                &mut soln_lock as *mut pthread_mutex_t,
                &mut sleepy as *mut timespec as *const timespec,
            );
        }
        pthread_mutex_unlock(&mut soln_lock);
        if res == 0 as libc::c_int {
            tmp___8 = check_solution(scp, active_pkb);
            if tmp___8 != 0 {
                active_pkb = 0 as *mut libc::c_void as *mut pubkeybatch_t;
            }
        } else if res == 110 as libc::c_int {
            free_pkb_tree(&mut (*scp).items, active_pkb);
        }
    };
}
unsafe extern "C" fn avl_delete_fix___0(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
    mut parentp: *mut avl_item_t,
) {
    let mut childp: *mut avl_item_t = 0 as *mut avl_item_t;
    if (*parentp).ai_left as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        if (*parentp).ai_right as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            if !(itemp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
                __assert_fail(
                    b"itemp == NULL\0" as *const u8 as *const libc::c_char,
                    b"avl.h\0" as *const u8 as *const libc::c_char,
                    188 as libc::c_uint,
                    b"avl_delete_fix\0" as *const u8 as *const libc::c_char,
                );
            }
            (*parentp).ai_balance = CENT;
            itemp = parentp;
            parentp = (*itemp).ai_up;
        }
    }
    while !parentp.is_null() {
        if itemp as libc::c_ulong == (*parentp).ai_right as libc::c_ulong {
            itemp = (*parentp).ai_left;
            if (*parentp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                if (*itemp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                    _avl_rotate_ll(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    parentp = itemp;
                } else if (*itemp).ai_balance as libc::c_uint == 1 as libc::c_uint {
                    _avl_rotate_ll(rootp, parentp);
                    (*itemp).ai_balance = RIGHT;
                    (*parentp).ai_balance = LEFT;
                    break;
                } else {
                    childp = (*itemp).ai_right;
                    _avl_rotate_lr(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    if (*childp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                        (*itemp).ai_balance = LEFT;
                    }
                    if (*childp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                        (*parentp).ai_balance = RIGHT;
                    }
                    (*childp).ai_balance = CENT;
                    parentp = childp;
                }
            } else if (*parentp).ai_balance as libc::c_uint == 1 as libc::c_uint {
                (*parentp).ai_balance = LEFT;
                break;
            } else {
                (*parentp).ai_balance = CENT;
            }
        } else {
            itemp = (*parentp).ai_right;
            if (*parentp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                if (*itemp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                    _avl_rotate_rr(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    parentp = itemp;
                } else if (*itemp).ai_balance as libc::c_uint == 1 as libc::c_uint {
                    _avl_rotate_rr(rootp, parentp);
                    (*itemp).ai_balance = LEFT;
                    (*parentp).ai_balance = RIGHT;
                    break;
                } else {
                    childp = (*itemp).ai_left;
                    _avl_rotate_rl(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    if (*childp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                        (*parentp).ai_balance = LEFT;
                    }
                    if (*childp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                        (*itemp).ai_balance = RIGHT;
                    }
                    (*childp).ai_balance = CENT;
                    parentp = childp;
                }
            } else if (*parentp).ai_balance as libc::c_uint == 1 as libc::c_uint {
                (*parentp).ai_balance = RIGHT;
                break;
            } else {
                (*parentp).ai_balance = CENT;
            }
        }
        itemp = parentp;
        parentp = (*itemp).ai_up;
    }
}
unsafe extern "C" fn avl_insert_fix___0(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut childp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut parentp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut tmp: *mut _avl_item_s = 0 as *mut _avl_item_s;
    parentp = (*itemp).ai_up;
    tmp = 0 as *mut libc::c_void as *mut _avl_item_s;
    (*itemp).ai_right = tmp;
    (*itemp).ai_left = tmp;
    if (*itemp).ai_indexed != 0 {
        __assert_fail(
            b"!itemp->ai_indexed\0" as *const u8 as *const libc::c_char,
            b"avl.h\0" as *const u8 as *const libc::c_char,
            275 as libc::c_uint,
            b"avl_insert_fix\0" as *const u8 as *const libc::c_char,
        );
    }
    (*itemp).ai_indexed = 1 as libc::c_int;
    while !parentp.is_null() {
        if itemp as libc::c_ulong == (*parentp).ai_left as libc::c_ulong {
            if (*parentp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                if (*itemp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                    _avl_rotate_ll(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    break;
                } else {
                    if !((*itemp).ai_balance as libc::c_uint != 1 as libc::c_uint) {
                        __assert_fail(
                            b"itemp->ai_balance != CENT\0" as *const u8
                                as *const libc::c_char,
                            b"avl.h\0" as *const u8 as *const libc::c_char,
                            290 as libc::c_uint,
                            b"avl_insert_fix\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    childp = (*itemp).ai_right;
                    _avl_rotate_lr(rootp, parentp);
                    (*itemp).ai_balance = CENT;
                    (*parentp).ai_balance = CENT;
                    if (*childp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                        (*itemp).ai_balance = LEFT;
                    }
                    if (*childp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                        (*parentp).ai_balance = RIGHT;
                    }
                    (*childp).ai_balance = CENT;
                    break;
                }
            } else if (*parentp).ai_balance as libc::c_uint == 1 as libc::c_uint {
                (*parentp).ai_balance = LEFT;
            } else {
                (*parentp).ai_balance = CENT;
                return;
            }
        } else if (*parentp).ai_balance as libc::c_uint == 2 as libc::c_uint {
            if (*itemp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                _avl_rotate_rr(rootp, parentp);
                (*itemp).ai_balance = CENT;
                (*parentp).ai_balance = CENT;
                break;
            } else {
                if !((*itemp).ai_balance as libc::c_uint != 1 as libc::c_uint) {
                    __assert_fail(
                        b"itemp->ai_balance != CENT\0" as *const u8
                            as *const libc::c_char,
                        b"avl.h\0" as *const u8 as *const libc::c_char,
                        316 as libc::c_uint,
                        b"avl_insert_fix\0" as *const u8 as *const libc::c_char,
                    );
                }
                childp = (*itemp).ai_left;
                _avl_rotate_rl(rootp, parentp);
                (*itemp).ai_balance = CENT;
                (*parentp).ai_balance = CENT;
                if (*childp).ai_balance as libc::c_uint == 2 as libc::c_uint {
                    (*parentp).ai_balance = LEFT;
                }
                if (*childp).ai_balance as libc::c_uint == 0 as libc::c_uint {
                    (*itemp).ai_balance = RIGHT;
                }
                (*childp).ai_balance = CENT;
                break;
            }
        } else if (*parentp).ai_balance as libc::c_uint == 1 as libc::c_uint {
            (*parentp).ai_balance = RIGHT;
        } else {
            (*parentp).ai_balance = CENT;
            break;
        }
        itemp = parentp;
        parentp = (*itemp).ai_up;
    }
}
unsafe extern "C" fn avl_remove___0(
    mut rootp: *mut avl_root_t,
    mut itemp: *mut avl_item_t,
) {
    let mut relocp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut replacep: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut parentp: *mut avl_item_t = 0 as *mut avl_item_t;
    parentp = 0 as *mut libc::c_void as *mut avl_item_t;
    if (*itemp).ai_indexed == 0 {
        __assert_fail(
            b"itemp->ai_indexed\0" as *const u8 as *const libc::c_char,
            b"avl.h\0" as *const u8 as *const libc::c_char,
            376 as libc::c_uint,
            b"avl_remove\0" as *const u8 as *const libc::c_char,
        );
    }
    (*itemp).ai_indexed = 0 as libc::c_int;
    's_113: {
        if !((*itemp).ai_left as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong)
        {
            if !((*itemp).ai_right as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong)
            {
                break 's_113;
            }
        }
        parentp = (*itemp).ai_up;
        replacep = (*itemp).ai_left;
        if replacep as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            replacep = (*itemp).ai_right;
        }
        if replacep as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            (*replacep).ai_up = parentp;
        }
        if parentp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            (*rootp).ar_root = replacep;
        } else {
            if itemp as libc::c_ulong == (*parentp).ai_left as libc::c_ulong {
                (*parentp).ai_left = replacep;
            } else {
                (*parentp).ai_right = replacep;
            }
            avl_delete_fix___0(rootp, replacep, parentp);
        }
        return;
    }
    relocp = avl_next(itemp);
    if relocp.is_null() {
        __assert_fail(
            b"relocp\0" as *const u8 as *const libc::c_char,
            b"avl.h\0" as *const u8 as *const libc::c_char,
            405 as libc::c_uint,
            b"avl_remove\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*relocp).ai_up as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"relocp->ai_up != NULL\0" as *const u8 as *const libc::c_char,
            b"avl.h\0" as *const u8 as *const libc::c_char,
            406 as libc::c_uint,
            b"avl_remove\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*relocp).ai_left as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"relocp->ai_left == NULL\0" as *const u8 as *const libc::c_char,
            b"avl.h\0" as *const u8 as *const libc::c_char,
            407 as libc::c_uint,
            b"avl_remove\0" as *const u8 as *const libc::c_char,
        );
    }
    replacep = (*relocp).ai_right;
    (*relocp).ai_left = (*itemp).ai_left;
    if (*relocp).ai_left as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*(*relocp).ai_left).ai_up = relocp;
    }
    if (*itemp).ai_up as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        (*rootp).ar_root = relocp;
    } else if itemp as libc::c_ulong == (*(*itemp).ai_up).ai_left as libc::c_ulong {
        (*(*itemp).ai_up).ai_left = relocp;
    } else {
        (*(*itemp).ai_up).ai_right = relocp;
    }
    if relocp as libc::c_ulong == (*(*relocp).ai_up).ai_left as libc::c_ulong {
        if !((*relocp).ai_up as libc::c_ulong != itemp as libc::c_ulong) {
            __assert_fail(
                b"relocp->ai_up != itemp\0" as *const u8 as *const libc::c_char,
                b"avl.h\0" as *const u8 as *const libc::c_char,
                421 as libc::c_uint,
                b"avl_remove\0" as *const u8 as *const libc::c_char,
            );
        }
        (*(*relocp).ai_up).ai_left = replacep;
        parentp = (*relocp).ai_up;
        if replacep as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            (*replacep).ai_up = (*relocp).ai_up;
        }
        (*relocp).ai_right = (*itemp).ai_right;
    } else {
        if !((*relocp).ai_up as libc::c_ulong == itemp as libc::c_ulong) {
            __assert_fail(
                b"relocp->ai_up == itemp\0" as *const u8 as *const libc::c_char,
                b"avl.h\0" as *const u8 as *const libc::c_char,
                428 as libc::c_uint,
                b"avl_remove\0" as *const u8 as *const libc::c_char,
            );
        }
        (*relocp).ai_right = replacep;
        parentp = relocp;
    }
    if (*relocp).ai_right as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*(*relocp).ai_right).ai_up = relocp;
    }
    (*relocp).ai_up = (*itemp).ai_up;
    (*relocp).ai_balance = (*itemp).ai_balance;
    avl_delete_fix___0(rootp, replacep, parentp);
}
pub unsafe extern "C" fn vg_exec_context_new_key() -> *mut EC_KEY {
    let mut tmp: *mut EC_KEY = 0 as *mut EC_KEY;
    tmp = EC_KEY_new_by_curve_name(714 as libc::c_int);
    return tmp;
}
static mut vg_thread_lock: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut vg_thread_rdcond: pthread_cond_t = __anonunion_pthread_cond_t_951761805 {
    __data: {
        let mut init = __pthread_cond_s {
            __annonCompField1: __anonunion____missing_field_name_622077928 {
                __wseq: 0 as libc::c_ulonglong,
            },
            __annonCompField2: __anonunion____missing_field_name_119281977 {
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
static mut vg_thread_wrcond: pthread_cond_t = __anonunion_pthread_cond_t_951761805 {
    __data: {
        let mut init = __pthread_cond_s {
            __annonCompField1: __anonunion____missing_field_name_622077928 {
                __wseq: 0 as libc::c_ulonglong,
            },
            __annonCompField2: __anonunion____missing_field_name_119281977 {
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
static mut vg_thread_upcond: pthread_cond_t = __anonunion_pthread_cond_t_951761805 {
    __data: {
        let mut init = __pthread_cond_s {
            __annonCompField1: __anonunion____missing_field_name_622077928 {
                __wseq: 0 as libc::c_ulonglong,
            },
            __annonCompField2: __anonunion____missing_field_name_119281977 {
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
unsafe extern "C" fn __vg_exec_context_yield(mut vxcp: *mut vg_exec_context_t) {
    (*vxcp).vxc_lockmode = 0 as libc::c_int;
    while (*(*vxcp).vxc_vc).vc_thread_excl != 0 {
        if (*vxcp).vxc_stop != 0 {
            if (*(*vxcp).vxc_vc).vc_thread_excl == 0 {
                __assert_fail(
                    b"vxcp->vxc_vc->vc_thread_excl\0" as *const u8
                        as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    64 as libc::c_uint,
                    b"__vg_exec_context_yield\0" as *const u8 as *const libc::c_char,
                );
            }
            (*vxcp).vxc_stop = 0 as libc::c_int;
            pthread_cond_signal(&mut vg_thread_upcond);
        }
        pthread_cond_wait(
            &mut vg_thread_rdcond as *mut pthread_cond_t,
            &mut vg_thread_lock as *mut pthread_mutex_t,
        );
    }
    if (*vxcp).vxc_stop != 0 {
        __assert_fail(
            b"!vxcp->vxc_stop\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_uint,
            b"__vg_exec_context_yield\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*vxcp).vxc_lockmode != 0 {
        __assert_fail(
            b"!vxcp->vxc_lockmode\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_uint,
            b"__vg_exec_context_yield\0" as *const u8 as *const libc::c_char,
        );
    }
    (*vxcp).vxc_lockmode = 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_exec_context_upgrade_lock(
    mut vxcp: *mut vg_exec_context_t,
) -> libc::c_int {
    let mut tp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    let mut tmp___5: libc::c_int = 0;
    if (*vxcp).vxc_lockmode == 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    pthread_mutex_lock(&mut vg_thread_lock);
    if !((*vxcp).vxc_lockmode == 1 as libc::c_int) {
        __assert_fail(
            b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_uint,
            b"vg_exec_context_upgrade_lock\0" as *const u8 as *const libc::c_char,
        );
    }
    (*vxcp).vxc_lockmode = 0 as libc::c_int;
    vcp = (*vxcp).vxc_vc;
    tmp___5 = (*vcp).vc_thread_excl;
    (*vcp).vc_thread_excl += 1;
    if tmp___5 != 0 {
        if (*vxcp).vxc_stop == 0 {
            __assert_fail(
                b"vxcp->vxc_stop\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                91 as libc::c_uint,
                b"vg_exec_context_upgrade_lock\0" as *const u8 as *const libc::c_char,
            );
        }
        (*vxcp).vxc_stop = 0 as libc::c_int;
        pthread_cond_signal(&mut vg_thread_upcond);
        pthread_cond_wait(
            &mut vg_thread_wrcond as *mut pthread_cond_t,
            &mut vg_thread_lock as *mut pthread_mutex_t,
        );
        tp = (*vcp).vc_threads;
        while tp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            if (*tp).vxc_lockmode != 0 {
                __assert_fail(
                    b"!tp->vxc_lockmode\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    97 as libc::c_uint,
                    b"vg_exec_context_upgrade_lock\0" as *const u8 as *const libc::c_char,
                );
            }
            if (*tp).vxc_stop != 0 {
                __assert_fail(
                    b"!tp->vxc_stop\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    98 as libc::c_uint,
                    b"vg_exec_context_upgrade_lock\0" as *const u8 as *const libc::c_char,
                );
            }
            tp = (*tp).vxc_next;
        }
    } else {
        tp = (*vcp).vc_threads;
        while tp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            if (*tp).vxc_lockmode != 0 {
                if !((*tp).vxc_lockmode != 2 as libc::c_int) {
                    __assert_fail(
                        b"tp->vxc_lockmode != 2\0" as *const u8 as *const libc::c_char,
                        b"pattern.c\0" as *const u8 as *const libc::c_char,
                        104 as libc::c_uint,
                        b"vg_exec_context_upgrade_lock\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                (*tp).vxc_stop = 1 as libc::c_int;
            }
            tp = (*tp).vxc_next;
        }
        loop {
            tp = (*vcp).vc_threads;
            while tp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                if (*tp).vxc_lockmode != 0 {
                    if !((*tp).vxc_lockmode != 2 as libc::c_int) {
                        __assert_fail(
                            b"tp->vxc_lockmode != 2\0" as *const u8
                                as *const libc::c_char,
                            b"pattern.c\0" as *const u8 as *const libc::c_char,
                            114 as libc::c_uint,
                            b"vg_exec_context_upgrade_lock\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    pthread_cond_wait(
                        &mut vg_thread_upcond as *mut pthread_cond_t,
                        &mut vg_thread_lock as *mut pthread_mutex_t,
                    );
                    break;
                } else {
                    tp = (*tp).vxc_next;
                }
            }
            if tp.is_null() {
                break;
            }
        }
    }
    (*vxcp).vxc_lockmode = 2 as libc::c_int;
    pthread_mutex_unlock(&mut vg_thread_lock);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_exec_context_downgrade_lock(
    mut vxcp: *mut vg_exec_context_t,
) {
    pthread_mutex_lock(&mut vg_thread_lock);
    if !((*vxcp).vxc_lockmode == 2 as libc::c_int) {
        __assert_fail(
            b"vxcp->vxc_lockmode == 2\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_uint,
            b"vg_exec_context_downgrade_lock\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*vxcp).vxc_stop != 0 {
        __assert_fail(
            b"!vxcp->vxc_stop\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            133 as libc::c_uint,
            b"vg_exec_context_downgrade_lock\0" as *const u8 as *const libc::c_char,
        );
    }
    (*(*vxcp).vxc_vc).vc_thread_excl -= 1;
    if (*(*vxcp).vxc_vc).vc_thread_excl == 0 {
        (*vxcp).vxc_lockmode = 1 as libc::c_int;
        pthread_cond_broadcast(&mut vg_thread_rdcond);
        pthread_mutex_unlock(&mut vg_thread_lock);
        return;
    }
    pthread_cond_signal(&mut vg_thread_wrcond);
    __vg_exec_context_yield(vxcp);
    pthread_mutex_unlock(&mut vg_thread_lock);
}
pub unsafe extern "C" fn vg_exec_context_init(
    mut vcp: *mut vg_context_t,
    mut vxcp: *mut vg_exec_context_t,
) -> libc::c_int {
    pthread_mutex_lock(&mut vg_thread_lock);
    memset(
        vxcp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<vg_exec_context_t>() as libc::c_ulong,
    );
    (*vxcp).vxc_vc = vcp;
    BN_init(&mut (*vxcp).vxc_bntarg);
    BN_init(&mut (*vxcp).vxc_bnbase);
    BN_init(&mut (*vxcp).vxc_bntmp);
    BN_init(&mut (*vxcp).vxc_bntmp2);
    BN_set_word(&mut (*vxcp).vxc_bnbase, 58 as libc::c_ulong);
    (*vxcp).vxc_bnctx = BN_CTX_new();
    if ((*vxcp).vxc_bnctx).is_null() {
        __assert_fail(
            b"vxcp->vxc_bnctx\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_uint,
            b"vg_exec_context_init\0" as *const u8 as *const libc::c_char,
        );
    }
    (*vxcp).vxc_key = vg_exec_context_new_key();
    if ((*vxcp).vxc_key).is_null() {
        __assert_fail(
            b"vxcp->vxc_key\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_uint,
            b"vg_exec_context_init\0" as *const u8 as *const libc::c_char,
        );
    }
    EC_KEY_precompute_mult((*vxcp).vxc_key, (*vxcp).vxc_bnctx);
    (*vxcp).vxc_lockmode = 0 as libc::c_int;
    (*vxcp).vxc_stop = 0 as libc::c_int;
    (*vxcp).vxc_next = (*vcp).vc_threads;
    (*vcp).vc_threads = vxcp;
    __vg_exec_context_yield(vxcp);
    pthread_mutex_unlock(&mut vg_thread_lock);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_exec_context_del(mut vxcp: *mut vg_exec_context_t) {
    let mut tp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut pprev: *mut *mut vg_exec_context_t = 0 as *mut *mut vg_exec_context_t;
    if (*vxcp).vxc_lockmode == 2 as libc::c_int {
        vg_exec_context_downgrade_lock(vxcp);
    }
    pthread_mutex_lock(&mut vg_thread_lock);
    if !((*vxcp).vxc_lockmode == 1 as libc::c_int) {
        __assert_fail(
            b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_uint,
            b"vg_exec_context_del\0" as *const u8 as *const libc::c_char,
        );
    }
    (*vxcp).vxc_lockmode = 0 as libc::c_int;
    pprev = &mut (*(*vxcp).vxc_vc).vc_threads;
    tp = *pprev;
    while tp as libc::c_ulong != vxcp as libc::c_ulong {
        if !(tp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        pprev = &mut (*tp).vxc_next;
        tp = *pprev;
    }
    if !(tp as libc::c_ulong == vxcp as libc::c_ulong) {
        __assert_fail(
            b"tp == vxcp\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_uint,
            b"vg_exec_context_del\0" as *const u8 as *const libc::c_char,
        );
    }
    *pprev = (*tp).vxc_next;
    if (*tp).vxc_stop != 0 {
        pthread_cond_signal(&mut vg_thread_upcond);
    }
    BN_clear_free(&mut (*vxcp).vxc_bntarg);
    BN_clear_free(&mut (*vxcp).vxc_bnbase);
    BN_clear_free(&mut (*vxcp).vxc_bntmp);
    BN_clear_free(&mut (*vxcp).vxc_bntmp2);
    BN_CTX_free((*vxcp).vxc_bnctx);
    (*vxcp).vxc_bnctx = 0 as *mut libc::c_void as *mut BN_CTX;
    pthread_mutex_unlock(&mut vg_thread_lock);
}
pub unsafe extern "C" fn vg_exec_context_yield(mut vxcp: *mut vg_exec_context_t) {
    if (*vxcp).vxc_lockmode == 2 as libc::c_int {
        vg_exec_context_downgrade_lock(vxcp);
    } else if (*vxcp).vxc_stop != 0 {
        if !((*vxcp).vxc_lockmode == 1 as libc::c_int) {
            __assert_fail(
                b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_uint,
                b"vg_exec_context_yield\0" as *const u8 as *const libc::c_char,
            );
        }
        pthread_mutex_lock(&mut vg_thread_lock);
        __vg_exec_context_yield(vxcp);
        pthread_mutex_unlock(&mut vg_thread_lock);
    }
    if !((*vxcp).vxc_lockmode == 1 as libc::c_int) {
        __assert_fail(
            b"vxcp->vxc_lockmode == 1\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            221 as libc::c_uint,
            b"vg_exec_context_yield\0" as *const u8 as *const libc::c_char,
        );
    }
}
pub unsafe extern "C" fn vg_exec_context_consolidate_key(
    mut vxcp: *mut vg_exec_context_t,
) {
    let mut tmp: *const BIGNUM = 0 as *const BIGNUM;
    if (*vxcp).vxc_delta != 0 {
        BN_clear(&mut (*vxcp).vxc_bntmp);
        BN_set_word(&mut (*vxcp).vxc_bntmp, (*vxcp).vxc_delta as libc::c_ulong);
        tmp = EC_KEY_get0_private_key((*vxcp).vxc_key as *const EC_KEY);
        BN_add(
            &mut (*vxcp).vxc_bntmp2,
            tmp,
            &mut (*vxcp).vxc_bntmp as *mut BIGNUM as *const BIGNUM,
        );
        vg_set_privkey(
            &mut (*vxcp).vxc_bntmp2 as *mut BIGNUM as *const BIGNUM,
            (*vxcp).vxc_key,
        );
        (*vxcp).vxc_delta = 0 as libc::c_int;
    }
}
pub unsafe extern "C" fn vg_exec_context_calc_address(mut vxcp: *mut vg_exec_context_t) {
    let mut pubkey: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut pgroup: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut eckey_buf: [libc::c_uchar; 96] = [0; 96];
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    let mut hash2: [libc::c_uchar; 20] = [0; 20];
    let mut len: libc::c_int = 0;
    let mut tmp: *const EC_POINT = 0 as *const EC_POINT;
    let mut tmp___0: size_t = 0;
    vg_exec_context_consolidate_key(vxcp);
    pgroup = EC_KEY_get0_group((*vxcp).vxc_key as *const EC_KEY);
    pubkey = EC_POINT_new(pgroup);
    tmp = EC_KEY_get0_public_key((*vxcp).vxc_key as *const EC_KEY);
    EC_POINT_copy(pubkey, tmp);
    if !((*(*vxcp).vxc_vc).vc_pubkey_base).is_null() {
        EC_POINT_add(
            pgroup,
            pubkey,
            pubkey as *const EC_POINT,
            (*(*vxcp).vxc_vc).vc_pubkey_base as *const EC_POINT,
            (*vxcp).vxc_bnctx,
        );
    }
    tmp___0 = EC_POINT_point2oct(
        pgroup,
        pubkey as *const EC_POINT,
        POINT_CONVERSION_UNCOMPRESSED,
        eckey_buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 96]>() as libc::c_ulong,
        (*vxcp).vxc_bnctx,
    );
    len = tmp___0 as libc::c_int;
    SHA256(
        eckey_buf.as_mut_ptr() as *const libc::c_uchar,
        len as size_t,
        hash1.as_mut_ptr(),
    );
    RIPEMD160(
        hash1.as_mut_ptr() as *const libc::c_uchar,
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        hash2.as_mut_ptr(),
    );
    memcpy(
        &mut *((*vxcp).vxc_binres).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut libc::c_uchar as *mut libc::c_void,
        hash2.as_mut_ptr() as *const libc::c_void,
        20 as libc::c_int as size_t,
    );
    EC_POINT_free(pubkey);
}
static mut timing_mutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub unsafe extern "C" fn vg_output_timing(
    mut vcp: *mut vg_context_t,
    mut cycle: libc::c_int,
    mut last: *mut timeval,
) -> libc::c_int {
    let mut me: pthread_t = 0;
    let mut tvnow: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tip: *mut timing_info_t = 0 as *mut timing_info_t;
    let mut mytip: *mut timing_info_t = 0 as *mut timing_info_t;
    let mut rate: libc::c_ulonglong = 0;
    let mut myrate: libc::c_ulonglong = 0;
    let mut mytime: libc::c_ulonglong = 0;
    let mut total: libc::c_ulonglong = 0;
    let mut sincelast: libc::c_ulonglong = 0;
    let mut p: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    myrate = 0 as libc::c_ulonglong;
    gettimeofday(&mut tvnow as *mut timeval, 0 as *mut libc::c_void);
    tv.tv_sec = tvnow.tv_sec - (*last).tv_sec;
    tv.tv_usec = tvnow.tv_usec - (*last).tv_usec;
    if tv.tv_usec < 0 as libc::c_long {
        tv.tv_sec -= 1;
        tv.tv_usec += 1000000 as libc::c_long;
    }
    memcpy(
        last as *mut libc::c_void,
        &mut tvnow as *mut timeval as *const libc::c_void,
        ::std::mem::size_of::<timeval>() as libc::c_ulong,
    );
    mytime = (tv.tv_usec as libc::c_ulonglong)
        .wrapping_add(
            (1000000 as libc::c_ulonglong).wrapping_mul(tv.tv_sec as libc::c_ulonglong),
        );
    if mytime == 0 {
        mytime = 1 as libc::c_ulonglong;
    }
    rate = 0 as libc::c_ulonglong;
    pthread_mutex_lock(&mut timing_mutex);
    me = pthread_self();
    tip = (*vcp).vc_timing_head;
    mytip = 0 as *mut libc::c_void as *mut timing_info_t;
    while tip as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = pthread_equal((*tip).ti_thread, me);
        if tmp != 0 {
            mytip = tip;
            p = ((*tip).ti_hist_last + 1 as libc::c_int) % 5 as libc::c_int;
            (*tip).ti_hist_time[p as usize] = mytime;
            (*tip).ti_hist_work[p as usize] = cycle as libc::c_ulong;
            (*tip).ti_hist_last = p;
            mytime = 0 as libc::c_ulonglong;
            myrate = 0 as libc::c_ulonglong;
            i = 0 as libc::c_int;
            while i < 5 as libc::c_int {
                mytime = mytime.wrapping_add((*tip).ti_hist_time[i as usize]);
                myrate = myrate
                    .wrapping_add((*tip).ti_hist_work[i as usize] as libc::c_ulonglong);
                i += 1;
            }
            myrate = myrate
                .wrapping_mul(1000000 as libc::c_ulonglong)
                .wrapping_div(mytime);
            (*tip).ti_last_rate = myrate as libc::c_ulong;
            rate = rate.wrapping_add(myrate);
        } else {
            rate = rate.wrapping_add((*tip).ti_last_rate as libc::c_ulonglong);
        }
        tip = (*tip).ti_next;
    }
    if mytip.is_null() {
        tmp___0 = malloc(::std::mem::size_of::<timing_info_t>() as libc::c_ulong);
        mytip = tmp___0 as *mut timing_info_t;
        (*mytip).ti_next = (*vcp).vc_timing_head;
        (*mytip).ti_thread = me;
        (*vcp).vc_timing_head = mytip;
        (*mytip).ti_hist_last = 0 as libc::c_int;
        (*mytip).ti_hist_time[0 as libc::c_int as usize] = mytime;
        (*mytip).ti_hist_work[0 as libc::c_int as usize] = cycle as libc::c_ulong;
        i = 1 as libc::c_int;
        while i < 5 as libc::c_int {
            (*mytip).ti_hist_time[i as usize] = 1 as libc::c_ulonglong;
            (*mytip).ti_hist_work[i as usize] = 0 as libc::c_ulong;
            i += 1;
        }
        myrate = (cycle as libc::c_ulonglong)
            .wrapping_mul(1000000 as libc::c_ulonglong)
            .wrapping_div(mytime);
        (*mytip).ti_last_rate = myrate as libc::c_ulong;
        rate = rate.wrapping_add(myrate);
    }
    (*vcp)
        .vc_timing_total = ((*vcp).vc_timing_total)
        .wrapping_add(cycle as libc::c_ulonglong);
    if (*vcp).vc_timing_prevfound != (*vcp).vc_found {
        (*vcp).vc_timing_prevfound = (*vcp).vc_found;
        (*vcp).vc_timing_sincelast = 0 as libc::c_ulonglong;
    }
    (*vcp)
        .vc_timing_sincelast = ((*vcp).vc_timing_sincelast)
        .wrapping_add(cycle as libc::c_ulonglong);
    if mytip as libc::c_ulong != (*vcp).vc_timing_head as libc::c_ulong {
        pthread_mutex_unlock(&mut timing_mutex);
        return myrate as libc::c_int;
    }
    total = (*vcp).vc_timing_total;
    sincelast = (*vcp).vc_timing_sincelast;
    pthread_mutex_unlock(&mut timing_mutex);
    (Some(((*vcp).vc_output_timing).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(vcp, sincelast as libc::c_double, rate, total);
    return myrate as libc::c_int;
}
pub unsafe extern "C" fn vg_context_thread_exit(mut vcp: *mut vg_context_t) {
    let mut tip: *mut timing_info_t = 0 as *mut timing_info_t;
    let mut ptip: *mut *mut timing_info_t = 0 as *mut *mut timing_info_t;
    let mut me: pthread_t = 0;
    let mut tmp: libc::c_int = 0;
    pthread_mutex_lock(&mut timing_mutex);
    me = pthread_self();
    ptip = &mut (*vcp).vc_timing_head;
    tip = *ptip;
    while tip as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = pthread_equal((*tip).ti_thread, me);
        if tmp == 0 {
            ptip = &mut (*tip).ti_next;
            tip = *ptip;
        } else {
            *ptip = (*tip).ti_next;
            free(tip as *mut libc::c_void);
            break;
        }
    }
    pthread_mutex_unlock(&mut timing_mutex);
}
unsafe extern "C" fn vg_timing_info_free(mut vcp: *mut vg_context_t) {
    let mut tp: *mut timing_info_t = 0 as *mut timing_info_t;
    while (*vcp).vc_timing_head as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        tp = (*vcp).vc_timing_head;
        (*vcp).vc_timing_head = (*tp).ti_next;
        free(tp as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn vg_output_timing_console(
    mut vcp: *mut vg_context_t,
    mut count: libc::c_double,
    mut rate: libc::c_ulonglong,
    mut total: libc::c_ulonglong,
) {
    let mut prob: libc::c_double = 0.;
    let mut time___0: libc::c_double = 0.;
    let mut targ: libc::c_double = 0.;
    let mut unit: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut linebuf: [libc::c_char; 80] = [0; 80];
    let mut rem: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut targs: [libc::c_double; 6] = [0.; 6];
    let mut tmp___0: libc::c_double = 0.;
    let mut tmp___2: libc::c_double = 0.;
    targs[0 as libc::c_int as usize] = 0.5f64;
    targs[1 as libc::c_int as usize] = 0.75f64;
    targs[2 as libc::c_int as usize] = 0.8f64;
    targs[3 as libc::c_int as usize] = 0.9f64;
    targs[4 as libc::c_int as usize] = 0.95f64;
    targs[5 as libc::c_int as usize] = 1.0f64;
    targ = rate as libc::c_double;
    unit = b"key/s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if targ > 1000 as libc::c_int as libc::c_double {
        unit = b"Kkey/s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        targ /= 1000.0f64;
        if targ > 1000 as libc::c_int as libc::c_double {
            unit = b"Mkey/s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            targ /= 1000.0f64;
        }
    }
    rem = ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int;
    p = snprintf(
        linebuf.as_mut_ptr(),
        rem as size_t,
        b"[%.2f %s][total %lld]\0" as *const u8 as *const libc::c_char,
        targ,
        unit,
        total,
    );
    if !(p > 0 as libc::c_int) {
        __assert_fail(
            b"p > 0\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_uint,
            b"vg_output_timing_console\0" as *const u8 as *const libc::c_char,
        );
    }
    rem -= p;
    if rem < 0 as libc::c_int {
        rem = 0 as libc::c_int;
    }
    if (*vcp).vc_chance >= 1.0f64 {
        tmp___0 = exp(-count / (*vcp).vc_chance);
        prob = 1.0f32 as libc::c_double - tmp___0;
        if prob <= 0.999f64 {
            p = snprintf(
                &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                rem as size_t,
                b"[Prob %.1f%%]\0" as *const u8 as *const libc::c_char,
                prob * 100 as libc::c_int as libc::c_double,
            );
            if !(p > 0 as libc::c_int) {
                __assert_fail(
                    b"p > 0\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    432 as libc::c_uint,
                    b"vg_output_timing_console\0" as *const u8 as *const libc::c_char,
                );
            }
            rem -= p;
            if rem < 0 as libc::c_int {
                rem = 0 as libc::c_int;
            }
            p = (::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
                .wrapping_sub(rem as libc::c_ulong) as libc::c_int;
        }
        i = 0 as libc::c_int;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_double; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
        {
            targ = targs[i as usize];
            if targ < 1.0f64 {
                if prob <= targ {
                    break;
                }
            }
            i += 1;
        }
        if targ < 1.0f64 {
            tmp___2 = log(1.0f64 - targ);
            time___0 = (-(*vcp).vc_chance * tmp___2 - count) / rate as libc::c_double;
            unit = b"s\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            if time___0 > 60 as libc::c_int as libc::c_double {
                time___0 /= 60 as libc::c_int as libc::c_double;
                unit = b"min\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
                if time___0 > 60 as libc::c_int as libc::c_double {
                    time___0 /= 60 as libc::c_int as libc::c_double;
                    unit = b"h\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    if time___0 > 24 as libc::c_int as libc::c_double {
                        time___0 /= 24 as libc::c_int as libc::c_double;
                        unit = b"d\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        if time___0 > 365 as libc::c_int as libc::c_double {
                            time___0 /= 365 as libc::c_int as libc::c_double;
                            unit = b"y\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char;
                        }
                    }
                }
            }
            if time___0 > 1000000 as libc::c_int as libc::c_double {
                p = snprintf(
                    &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                    rem as size_t,
                    b"[%d%% in %e%s]\0" as *const u8 as *const libc::c_char,
                    (100 as libc::c_int as libc::c_double * targ) as libc::c_int,
                    time___0,
                    unit,
                );
            } else {
                p = snprintf(
                    &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                    rem as size_t,
                    b"[%d%% in %.1f%s]\0" as *const u8 as *const libc::c_char,
                    (100 as libc::c_int as libc::c_double * targ) as libc::c_int,
                    time___0,
                    unit,
                );
            }
            if !(p > 0 as libc::c_int) {
                __assert_fail(
                    b"p > 0\0" as *const u8 as *const libc::c_char,
                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                    475 as libc::c_uint,
                    b"vg_output_timing_console\0" as *const u8 as *const libc::c_char,
                );
            }
            rem -= p;
            if rem < 0 as libc::c_int {
                rem = 0 as libc::c_int;
            }
            p = (::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
                .wrapping_sub(rem as libc::c_ulong) as libc::c_int;
        }
    }
    if (*vcp).vc_found != 0 {
        if (*vcp).vc_remove_on_match != 0 {
            p = snprintf(
                &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                rem as size_t,
                b"[Found %lld/%ld]\0" as *const u8 as *const libc::c_char,
                (*vcp).vc_found,
                (*vcp).vc_npatterns_start,
            );
        } else {
            p = snprintf(
                &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                rem as size_t,
                b"[Found %lld]\0" as *const u8 as *const libc::c_char,
                (*vcp).vc_found,
            );
        }
        if !(p > 0 as libc::c_int) {
            __assert_fail(
                b"p > 0\0" as *const u8 as *const libc::c_char,
                b"pattern.c\0" as *const u8 as *const libc::c_char,
                490 as libc::c_uint,
                b"vg_output_timing_console\0" as *const u8 as *const libc::c_char,
            );
        }
        rem -= p;
        if rem < 0 as libc::c_int {
            rem = 0 as libc::c_int;
        }
    }
    if rem != 0 {
        memset(
            &mut *linebuf
                .as_mut_ptr()
                .offset(
                    (::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
                        .wrapping_sub(rem as libc::c_ulong) as isize,
                ) as *mut libc::c_char as *mut libc::c_void,
            32 as libc::c_int,
            rem as size_t,
        );
        linebuf[(::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong) as usize] = '\u{0}' as i32 as libc::c_char;
    }
    printf(b"\r%s\0" as *const u8 as *const libc::c_char, linebuf.as_mut_ptr());
    fflush(stdout);
}
pub unsafe extern "C" fn vg_output_match_console(
    mut vcp: *mut vg_context_t,
    mut pkey: *mut EC_KEY,
    mut pattern: *const libc::c_char,
) {
    let mut key_buf: [libc::c_uchar; 512] = [0; 512];
    let mut pend: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut addr_buf: [libc::c_char; 64] = [0; 64];
    let mut addr2_buf: [libc::c_char; 64] = [0; 64];
    let mut privkey_buf: [libc::c_char; 128] = [0; 128];
    let mut keytype: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: libc::c_int = 0;
    let mut isscript: libc::c_int = 0;
    let mut ppnt: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut free_ppnt: libc::c_int = 0;
    let mut tmp: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___0: *const EC_POINT = 0 as *const EC_POINT;
    let mut tmp___1: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___2: *const EC_POINT = 0 as *const EC_POINT;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___8: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___9: *const BIGNUM = 0 as *const BIGNUM;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tmp___10: *mut FILE = 0 as *mut FILE;
    let mut tmp___11: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    keytype = b"Privkey\0" as *const u8 as *const libc::c_char;
    isscript = ((*vcp).vc_format as libc::c_uint == 1 as libc::c_uint) as libc::c_int;
    free_ppnt = 0 as libc::c_int;
    if !((*vcp).vc_pubkey_base).is_null() {
        tmp = EC_KEY_get0_group(pkey as *const EC_KEY);
        ppnt = EC_POINT_new(tmp);
        tmp___0 = EC_KEY_get0_public_key(pkey as *const EC_KEY);
        EC_POINT_copy(ppnt, tmp___0);
        tmp___1 = EC_KEY_get0_group(pkey as *const EC_KEY);
        EC_POINT_add(
            tmp___1,
            ppnt,
            ppnt as *const EC_POINT,
            (*vcp).vc_pubkey_base as *const EC_POINT,
            0 as *mut libc::c_void as *mut BN_CTX,
        );
        free_ppnt = 1 as libc::c_int;
        keytype = b"PrivkeyPart\0" as *const u8 as *const libc::c_char;
    } else {
        tmp___2 = EC_KEY_get0_public_key(pkey as *const EC_KEY);
        ppnt = tmp___2 as *mut EC_POINT;
    }
    tmp___6 = EC_KEY_check_key(pkey as *const EC_KEY);
    if tmp___6 == 0 {
        __assert_fail(
            b"EC_KEY_check_key(pkey)\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            530 as libc::c_uint,
            b"vg_output_match_console\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___7 = EC_KEY_get0_group(pkey as *const EC_KEY);
    vg_encode_address(
        ppnt as *const EC_POINT,
        tmp___7,
        (*vcp).vc_pubkeytype,
        addr_buf.as_mut_ptr(),
    );
    if isscript != 0 {
        tmp___8 = EC_KEY_get0_group(pkey as *const EC_KEY);
        vg_encode_script_address(
            ppnt as *const EC_POINT,
            tmp___8,
            (*vcp).vc_addrtype,
            addr2_buf.as_mut_ptr(),
        );
    }
    if !((*vcp).vc_key_protect_pass).is_null() {
        len = vg_protect_encode_privkey(
            privkey_buf.as_mut_ptr(),
            pkey as *const EC_KEY,
            (*vcp).vc_privtype,
            -(1 as libc::c_int),
            (*vcp).vc_key_protect_pass,
        );
        if len != 0 {
            keytype = b"Protkey\0" as *const u8 as *const libc::c_char;
        } else {
            fprintf(
                stderr,
                b"ERROR: could not password-protect key\n\0" as *const u8
                    as *const libc::c_char,
            );
            (*vcp).vc_key_protect_pass = 0 as *mut libc::c_void as *const libc::c_char;
        }
    }
    if ((*vcp).vc_key_protect_pass).is_null() {
        vg_encode_privkey(
            pkey as *const EC_KEY,
            (*vcp).vc_privtype,
            privkey_buf.as_mut_ptr(),
        );
    }
    if ((*vcp).vc_result_file).is_null() {
        printf(
            b"\r%79s\rPattern: %s\n\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            pattern,
        );
    } else if (*vcp).vc_verbose > 0 as libc::c_int {
        printf(
            b"\r%79s\rPattern: %s\n\0" as *const u8 as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
            pattern,
        );
    }
    if (*vcp).vc_verbose > 0 as libc::c_int {
        if (*vcp).vc_verbose > 1 as libc::c_int {
            pend = key_buf.as_mut_ptr();
            len = i2o_ECPublicKey(pkey, &mut pend);
            printf(b"Pubkey (hex): \0" as *const u8 as *const libc::c_char);
            dumphex(key_buf.as_mut_ptr() as *const libc::c_uchar, len as size_t);
            printf(b"Privkey (hex): \0" as *const u8 as *const libc::c_char);
            tmp___9 = EC_KEY_get0_private_key(pkey as *const EC_KEY);
            dumpbn(tmp___9);
            pend = key_buf.as_mut_ptr();
            len = i2d_ECPrivateKey(pkey, &mut pend);
            printf(b"Privkey (ASN1): \0" as *const u8 as *const libc::c_char);
            dumphex(key_buf.as_mut_ptr() as *const libc::c_uchar, len as size_t);
        }
    }
    let mut current_block_62: u64;
    if ((*vcp).vc_result_file).is_null() {
        current_block_62 = 9915023510721421386;
    } else if (*vcp).vc_verbose > 0 as libc::c_int {
        current_block_62 = 9915023510721421386;
    } else {
        current_block_62 = 5235537862154438448;
    }
    match current_block_62 {
        9915023510721421386 => {
            if isscript != 0 {
                printf(
                    b"P2SHAddress: %s\n\0" as *const u8 as *const libc::c_char,
                    addr2_buf.as_mut_ptr(),
                );
            }
            printf(
                b"Address: %s\n%s: %s\n\0" as *const u8 as *const libc::c_char,
                addr_buf.as_mut_ptr(),
                keytype,
                privkey_buf.as_mut_ptr(),
            );
        }
        _ => {}
    }
    if !((*vcp).vc_result_file).is_null() {
        tmp___10 = fopen(
            (*vcp).vc_result_file,
            b"a\0" as *const u8 as *const libc::c_char,
        );
        fp = tmp___10;
        if fp.is_null() {
            tmp___11 = __errno_location();
            tmp___12 = strerror(*tmp___11);
            fprintf(
                stderr,
                b"ERROR: could not open result file: %s\n\0" as *const u8
                    as *const libc::c_char,
                tmp___12,
            );
        } else {
            fprintf(fp, b"Pattern: %s\n\0" as *const u8 as *const libc::c_char, pattern);
            if isscript != 0 {
                fprintf(
                    fp,
                    b"P2SHAddress: %s\n\0" as *const u8 as *const libc::c_char,
                    addr2_buf.as_mut_ptr(),
                );
            }
            fprintf(
                fp,
                b"Address: %s\n%s: %s\n\0" as *const u8 as *const libc::c_char,
                addr_buf.as_mut_ptr(),
                keytype,
                privkey_buf.as_mut_ptr(),
            );
            fclose(fp);
        }
    }
    if free_ppnt != 0 {
        EC_POINT_free(ppnt);
    }
}
pub unsafe extern "C" fn vg_context_free(mut vcp: *mut vg_context_t) {
    vg_timing_info_free(vcp);
    (Some(((*vcp).vc_free).expect("non-null function pointer")))
        .expect("non-null function pointer")(vcp);
}
pub unsafe extern "C" fn vg_context_add_patterns(
    mut vcp: *mut vg_context_t,
    patterns: *mut *const libc::c_char,
    mut npatterns: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    (*vcp).vc_pattern_generation += 1;
    tmp = (Some(((*vcp).vc_add_patterns).expect("non-null function pointer")))
        .expect("non-null function pointer")(vcp, patterns, npatterns);
    return tmp;
}
pub unsafe extern "C" fn vg_context_clear_all_patterns(mut vcp: *mut vg_context_t) {
    (Some(((*vcp).vc_clear_all_patterns).expect("non-null function pointer")))
        .expect("non-null function pointer")(vcp);
    (*vcp).vc_pattern_generation += 1;
}
pub unsafe extern "C" fn vg_context_hash160_sort(
    mut vcp: *mut vg_context_t,
    mut buf: *mut libc::c_void,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if ((*vcp).vc_hash160_sort).is_none() {
        return 0 as libc::c_int;
    }
    tmp = (Some(((*vcp).vc_hash160_sort).expect("non-null function pointer")))
        .expect("non-null function pointer")(vcp, buf);
    return tmp;
}
pub unsafe extern "C" fn vg_context_start_threads(
    mut vcp: *mut vg_context_t,
) -> libc::c_int {
    let mut vxcp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut res: libc::c_int = 0;
    vxcp = (*vcp).vc_threads;
    while vxcp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        res = pthread_create(
            &mut (*vxcp).vxc_pthread as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*mut vg_exec_context_t) -> *mut libc::c_void,
                >,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
            >((*vxcp).vxc_threadfunc),
            vxcp as *mut libc::c_void,
        );
        if res != 0 {
            fprintf(
                stderr,
                b"ERROR: could not create thread: %d\n\0" as *const u8
                    as *const libc::c_char,
                res,
            );
            vg_context_stop_threads(vcp);
            return -(1 as libc::c_int);
        }
        (*vxcp).vxc_thread_active = 1 as libc::c_int;
        vxcp = (*vxcp).vxc_next;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn vg_context_stop_threads(mut vcp: *mut vg_context_t) {
    (*vcp).vc_halt = 1 as libc::c_int;
    vg_context_wait_for_completion(vcp);
    (*vcp).vc_halt = 0 as libc::c_int;
}
pub unsafe extern "C" fn vg_context_wait_for_completion(mut vcp: *mut vg_context_t) {
    let mut vxcp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    vxcp = (*vcp).vc_threads;
    while vxcp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if !((*vxcp).vxc_thread_active == 0) {
            pthread_join(
                (*vxcp).vxc_pthread,
                0 as *mut libc::c_void as *mut *mut libc::c_void,
            );
            (*vxcp).vxc_thread_active = 0 as libc::c_int;
        }
        vxcp = (*vxcp).vxc_next;
    }
}
unsafe extern "C" fn get_prefix_ranges(
    mut addrtype: libc::c_int,
    mut pfx: *const libc::c_char,
    mut result: *mut *mut BIGNUM,
    mut bnctx: *mut BN_CTX,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut zero_prefix: libc::c_int = 0;
    let mut check_upper: libc::c_int = 0;
    let mut b58pow: libc::c_int = 0;
    let mut b58ceil: libc::c_int = 0;
    let mut b58top: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut bntarg: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnceil: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnfloor: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnbase: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnap: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnbp: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bntp: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnhigh: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnlow: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnhigh2: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnlow2: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bntmp: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bntmp2: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut tmp: size_t = 0;
    let mut tmp___0: *const BIGNUM = 0 as *const BIGNUM;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_ulong = 0;
    let mut tmp___3: *const BIGNUM = 0 as *const BIGNUM;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
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
    zero_prefix = 0 as libc::c_int;
    check_upper = 0 as libc::c_int;
    b58top = 0 as libc::c_int;
    ret = -(1 as libc::c_int);
    bnhigh = 0 as *mut libc::c_void as *mut BIGNUM;
    bnlow = 0 as *mut libc::c_void as *mut BIGNUM;
    bnhigh2 = 0 as *mut libc::c_void as *mut BIGNUM;
    bnlow2 = 0 as *mut libc::c_void as *mut BIGNUM;
    BN_init(&mut bntarg);
    BN_init(&mut bnceil);
    BN_init(&mut bnfloor);
    BN_init(&mut bnbase);
    BN_init(&mut bntmp);
    BN_init(&mut bntmp2);
    BN_set_word(&mut bnbase, 58 as libc::c_ulong);
    tmp = strlen(pfx);
    p = tmp as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        if !(i < p) {
            current_block = 10891380440665537214;
            break;
        }
        c = vg_b58_reverse_map[*pfx.offset(i as isize) as libc::c_int as usize]
            as libc::c_int;
        if c == -(1 as libc::c_int) {
            fprintf(
                stderr,
                b"Invalid character '%c' in prefix '%s'\n\0" as *const u8
                    as *const libc::c_char,
                *pfx.offset(i as isize) as libc::c_int,
                pfx,
            );
            current_block = 7776949735822190393;
            break;
        } else {
            if i == zero_prefix {
                if c == 0 as libc::c_int {
                    zero_prefix += 1;
                    if zero_prefix > 19 as libc::c_int {
                        fprintf(
                            stderr,
                            b"Prefix '%s' is too long\n\0" as *const u8
                                as *const libc::c_char,
                            pfx,
                        );
                        current_block = 7776949735822190393;
                        break;
                    }
                } else {
                    b58top = c;
                    BN_set_word(&mut bntarg, c as libc::c_ulong);
                }
            } else {
                BN_set_word(&mut bntmp2, c as libc::c_ulong);
                BN_mul(
                    &mut bntmp,
                    &mut bntarg as *mut BIGNUM as *const BIGNUM,
                    &mut bnbase as *mut BIGNUM as *const BIGNUM,
                    bnctx,
                );
                BN_add(
                    &mut bntarg,
                    &mut bntmp as *mut BIGNUM as *const BIGNUM,
                    &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                );
            }
            i += 1;
        }
    }
    match current_block {
        10891380440665537214 => {
            BN_clear(&mut bntmp);
            BN_set_bit(&mut bntmp, 200 as libc::c_int - zero_prefix * 8 as libc::c_int);
            tmp___0 = BN_value_one();
            BN_sub(&mut bnceil, &mut bntmp as *mut BIGNUM as *const BIGNUM, tmp___0);
            BN_set_bit(
                &mut bnfloor,
                192 as libc::c_int - zero_prefix * 8 as libc::c_int,
            );
            bnlow = BN_new();
            bnhigh = BN_new();
            if b58top != 0 {
                BN_copy(&mut bntmp, &mut bnceil as *mut BIGNUM as *const BIGNUM);
                bnap = &mut bntmp;
                bnbp = &mut bntmp2;
                b58pow = 0 as libc::c_int;
                loop {
                    tmp___1 = BN_cmp(
                        bnap as *const BIGNUM,
                        &mut bnbase as *mut BIGNUM as *const BIGNUM,
                    );
                    if !(tmp___1 > 0 as libc::c_int) {
                        break;
                    }
                    b58pow += 1;
                    BN_div(
                        bnbp,
                        0 as *mut libc::c_void as *mut BIGNUM,
                        bnap as *const BIGNUM,
                        &mut bnbase as *mut BIGNUM as *const BIGNUM,
                        bnctx,
                    );
                    bntp = bnap;
                    bnap = bnbp;
                    bnbp = bntp;
                }
                tmp___2 = BN_get_word(bnap as *const BIGNUM);
                b58ceil = tmp___2 as libc::c_int;
                if b58pow - (p - zero_prefix) < 6 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Prefix '%s' is too long\n\0" as *const u8
                            as *const libc::c_char,
                        pfx,
                    );
                    current_block = 7776949735822190393;
                } else {
                    BN_set_word(
                        &mut bntmp2,
                        (b58pow - (p - zero_prefix)) as libc::c_ulong,
                    );
                    BN_exp(
                        &mut bntmp,
                        &mut bnbase as *mut BIGNUM as *const BIGNUM,
                        &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                        bnctx,
                    );
                    BN_mul(
                        bnlow,
                        &mut bntmp as *mut BIGNUM as *const BIGNUM,
                        &mut bntarg as *mut BIGNUM as *const BIGNUM,
                        bnctx,
                    );
                    tmp___3 = BN_value_one();
                    BN_sub(
                        &mut bntmp2,
                        &mut bntmp as *mut BIGNUM as *const BIGNUM,
                        tmp___3,
                    );
                    BN_add(
                        bnhigh,
                        bnlow as *const BIGNUM,
                        &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                    );
                    if b58top <= b58ceil {
                        check_upper = 1 as libc::c_int;
                        bnlow2 = BN_new();
                        bnhigh2 = BN_new();
                        BN_mul(
                            bnlow2,
                            bnlow as *const BIGNUM,
                            &mut bnbase as *mut BIGNUM as *const BIGNUM,
                            bnctx,
                        );
                        BN_mul(
                            &mut bntmp2,
                            bnhigh as *const BIGNUM,
                            &mut bnbase as *mut BIGNUM as *const BIGNUM,
                            bnctx,
                        );
                        BN_set_word(&mut bntmp, 57 as libc::c_ulong);
                        BN_add(
                            bnhigh2,
                            &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                            &mut bntmp as *mut BIGNUM as *const BIGNUM,
                        );
                        tmp___5 = BN_cmp(
                            &mut bnceil as *mut BIGNUM as *const BIGNUM,
                            bnlow2 as *const BIGNUM,
                        );
                        if tmp___5 < 0 as libc::c_int {
                            check_upper = 0 as libc::c_int;
                            BN_free(bnhigh2);
                            bnhigh2 = 0 as *mut libc::c_void as *mut BIGNUM;
                            BN_free(bnlow2);
                            bnlow2 = 0 as *mut libc::c_void as *mut BIGNUM;
                        } else {
                            tmp___4 = BN_cmp(
                                &mut bnceil as *mut BIGNUM as *const BIGNUM,
                                bnhigh2 as *const BIGNUM,
                            );
                            if tmp___4 < 0 as libc::c_int {
                                BN_copy(
                                    bnhigh2,
                                    &mut bnceil as *mut BIGNUM as *const BIGNUM,
                                );
                            }
                        }
                        tmp___8 = BN_cmp(
                            &mut bnfloor as *mut BIGNUM as *const BIGNUM,
                            bnhigh as *const BIGNUM,
                        );
                        if tmp___8 >= 0 as libc::c_int {
                            if check_upper == 0 {
                                __assert_fail(
                                    b"check_upper\0" as *const u8 as *const libc::c_char,
                                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                                    820 as libc::c_uint,
                                    b"get_prefix_ranges\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            check_upper = 0 as libc::c_int;
                            BN_free(bnhigh);
                            bnhigh = bnhigh2;
                            bnhigh2 = 0 as *mut libc::c_void as *mut BIGNUM;
                            BN_free(bnlow);
                            bnlow = bnlow2;
                            bnlow2 = 0 as *mut libc::c_void as *mut BIGNUM;
                        } else {
                            tmp___7 = BN_cmp(
                                &mut bnfloor as *mut BIGNUM as *const BIGNUM,
                                bnlow as *const BIGNUM,
                            );
                            if tmp___7 > 0 as libc::c_int {
                                BN_copy(
                                    bnlow,
                                    &mut bnfloor as *mut BIGNUM as *const BIGNUM,
                                );
                            }
                        }
                    }
                    current_block = 7301440000599063274;
                }
            } else {
                BN_copy(bnhigh, &mut bnceil as *mut BIGNUM as *const BIGNUM);
                BN_clear(bnlow);
                current_block = 7301440000599063274;
            }
            match current_block {
                7776949735822190393 => {}
                _ => {
                    BN_clear(&mut bntmp);
                    BN_set_word(&mut bntmp, addrtype as libc::c_ulong);
                    BN_lshift(
                        &mut bntmp2,
                        &mut bntmp as *mut BIGNUM as *const BIGNUM,
                        192 as libc::c_int,
                    );
                    if check_upper != 0 {
                        tmp___10 = BN_cmp(
                            &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                            bnhigh2 as *const BIGNUM,
                        );
                        if tmp___10 > 0 as libc::c_int {
                            check_upper = 0 as libc::c_int;
                            BN_free(bnhigh2);
                            bnhigh2 = 0 as *mut libc::c_void as *mut BIGNUM;
                            BN_free(bnlow2);
                            bnlow2 = 0 as *mut libc::c_void as *mut BIGNUM;
                        } else {
                            tmp___9 = BN_cmp(
                                &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                                bnlow2 as *const BIGNUM,
                            );
                            if tmp___9 > 0 as libc::c_int {
                                BN_copy(
                                    bnlow2,
                                    &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                                );
                            }
                        }
                    }
                    tmp___12 = BN_cmp(
                        &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                        bnhigh as *const BIGNUM,
                    );
                    if tmp___12 > 0 as libc::c_int {
                        if check_upper == 0 {
                            current_block = 13551661895636571695;
                        } else {
                            check_upper = 0 as libc::c_int;
                            BN_free(bnhigh);
                            bnhigh = bnhigh2;
                            bnhigh2 = 0 as *mut libc::c_void as *mut BIGNUM;
                            BN_free(bnlow);
                            bnlow = bnlow2;
                            bnlow2 = 0 as *mut libc::c_void as *mut BIGNUM;
                            current_block = 8225018548522317130;
                        }
                    } else {
                        tmp___11 = BN_cmp(
                            &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                            bnlow as *const BIGNUM,
                        );
                        if tmp___11 > 0 as libc::c_int {
                            BN_copy(bnlow, &mut bntmp2 as *mut BIGNUM as *const BIGNUM);
                        }
                        current_block = 8225018548522317130;
                    }
                    match current_block {
                        8225018548522317130 => {
                            BN_set_word(
                                &mut bntmp,
                                (addrtype + 1 as libc::c_int) as libc::c_ulong,
                            );
                            BN_lshift(
                                &mut bntmp2,
                                &mut bntmp as *mut BIGNUM as *const BIGNUM,
                                192 as libc::c_int,
                            );
                            if check_upper != 0 {
                                tmp___14 = BN_cmp(
                                    &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                                    bnlow2 as *const BIGNUM,
                                );
                                if tmp___14 < 0 as libc::c_int {
                                    check_upper = 0 as libc::c_int;
                                    BN_free(bnhigh2);
                                    bnhigh2 = 0 as *mut libc::c_void as *mut BIGNUM;
                                    BN_free(bnlow2);
                                    bnlow2 = 0 as *mut libc::c_void as *mut BIGNUM;
                                } else {
                                    tmp___13 = BN_cmp(
                                        &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                                        bnhigh2 as *const BIGNUM,
                                    );
                                    if tmp___13 < 0 as libc::c_int {
                                        BN_copy(
                                            bnlow2,
                                            &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                                        );
                                    }
                                }
                            }
                            tmp___16 = BN_cmp(
                                &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                                bnlow as *const BIGNUM,
                            );
                            if tmp___16 < 0 as libc::c_int {
                                if check_upper == 0 {
                                    current_block = 13551661895636571695;
                                } else {
                                    check_upper = 0 as libc::c_int;
                                    BN_free(bnhigh);
                                    bnhigh = bnhigh2;
                                    bnhigh2 = 0 as *mut libc::c_void as *mut BIGNUM;
                                    BN_free(bnlow);
                                    bnlow = bnlow2;
                                    bnlow2 = 0 as *mut libc::c_void as *mut BIGNUM;
                                    current_block = 7315983924538012637;
                                }
                            } else {
                                tmp___15 = BN_cmp(
                                    &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                                    bnhigh as *const BIGNUM,
                                );
                                if tmp___15 < 0 as libc::c_int {
                                    BN_copy(
                                        bnhigh,
                                        &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                                    );
                                }
                                current_block = 7315983924538012637;
                            }
                            match current_block {
                                13551661895636571695 => {}
                                _ => {
                                    if check_upper == 0 {
                                        if bnlow2 as libc::c_ulong
                                            == 0 as *mut libc::c_void as libc::c_ulong
                                        {
                                            if !(bnhigh2 as libc::c_ulong
                                                == 0 as *mut libc::c_void as libc::c_ulong)
                                            {
                                                __assert_fail(
                                                    b"check_upper || ((bnlow2 == NULL) && (bnhigh2 == NULL))\0"
                                                        as *const u8 as *const libc::c_char,
                                                    b"pattern.c\0" as *const u8 as *const libc::c_char,
                                                    903 as libc::c_uint,
                                                    b"get_prefix_ranges\0" as *const u8 as *const libc::c_char,
                                                );
                                            }
                                        } else {
                                            __assert_fail(
                                                b"check_upper || ((bnlow2 == NULL) && (bnhigh2 == NULL))\0"
                                                    as *const u8 as *const libc::c_char,
                                                b"pattern.c\0" as *const u8 as *const libc::c_char,
                                                903 as libc::c_uint,
                                                b"get_prefix_ranges\0" as *const u8 as *const libc::c_char,
                                            );
                                        }
                                    }
                                    let ref mut fresh0 = *result
                                        .offset(0 as libc::c_int as isize);
                                    *fresh0 = bnlow;
                                    let ref mut fresh1 = *result
                                        .offset(1 as libc::c_int as isize);
                                    *fresh1 = bnhigh;
                                    let ref mut fresh2 = *result
                                        .offset(2 as libc::c_int as isize);
                                    *fresh2 = bnlow2;
                                    let ref mut fresh3 = *result
                                        .offset(3 as libc::c_int as isize);
                                    *fresh3 = bnhigh2;
                                    bnlow = 0 as *mut libc::c_void as *mut BIGNUM;
                                    bnhigh = 0 as *mut libc::c_void as *mut BIGNUM;
                                    bnlow2 = 0 as *mut libc::c_void as *mut BIGNUM;
                                    bnhigh2 = 0 as *mut libc::c_void as *mut BIGNUM;
                                    ret = 0 as libc::c_int;
                                    current_block = 7776949735822190393;
                                }
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        7776949735822190393 => {}
                        _ => {
                            ret = -(2 as libc::c_int);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    BN_clear_free(&mut bntarg);
    BN_clear_free(&mut bnceil);
    BN_clear_free(&mut bnfloor);
    BN_clear_free(&mut bnbase);
    BN_clear_free(&mut bntmp);
    BN_clear_free(&mut bntmp2);
    if !bnhigh.is_null() {
        BN_free(bnhigh);
    }
    if !bnlow.is_null() {
        BN_free(bnlow);
    }
    if !bnhigh2.is_null() {
        BN_free(bnhigh2);
    }
    if !bnlow2.is_null() {
        BN_free(bnlow2);
    }
    return ret;
}
unsafe extern "C" fn free_ranges(mut ranges: *mut *mut BIGNUM) {
    BN_free(*ranges.offset(0 as libc::c_int as isize));
    BN_free(*ranges.offset(1 as libc::c_int as isize));
    let ref mut fresh4 = *ranges.offset(0 as libc::c_int as isize);
    *fresh4 = 0 as *mut libc::c_void as *mut BIGNUM;
    let ref mut fresh5 = *ranges.offset(1 as libc::c_int as isize);
    *fresh5 = 0 as *mut libc::c_void as *mut BIGNUM;
    if !(*ranges.offset(2 as libc::c_int as isize)).is_null() {
        BN_free(*ranges.offset(2 as libc::c_int as isize));
        BN_free(*ranges.offset(3 as libc::c_int as isize));
        let ref mut fresh6 = *ranges.offset(2 as libc::c_int as isize);
        *fresh6 = 0 as *mut libc::c_void as *mut BIGNUM;
        let ref mut fresh7 = *ranges.offset(3 as libc::c_int as isize);
        *fresh7 = 0 as *mut libc::c_void as *mut BIGNUM;
    }
}
pub static mut vpk_nwords: libc::c_int = 0;
unsafe extern "C" fn vg_prefix_free(mut vp: *mut vg_prefix_t) {
    if !((*vp).vp_low).is_null() {
        BN_free((*vp).vp_low);
    }
    if !((*vp).vp_high).is_null() {
        BN_free((*vp).vp_high);
    }
    free(vp as *mut libc::c_void);
}
unsafe extern "C" fn vg_prefix_avl_search(
    mut rootp: *mut avl_root_t,
    mut targ: *mut BIGNUM,
) -> *mut vg_prefix_t {
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    itemp = (*rootp).ar_root;
    while !itemp.is_null() {
        vp = (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
        tmp___0 = BN_cmp((*vp).vp_low as *const BIGNUM, targ as *const BIGNUM);
        if tmp___0 > 0 as libc::c_int {
            itemp = (*itemp).ai_left;
        } else {
            tmp = BN_cmp((*vp).vp_high as *const BIGNUM, targ as *const BIGNUM);
            if tmp < 0 as libc::c_int {
                itemp = (*itemp).ai_right;
            } else {
                return vp
            }
        }
    }
    return 0 as *mut libc::c_void as *mut vg_prefix_t;
}
unsafe extern "C" fn vg_prefix_avl_insert(
    mut rootp: *mut avl_root_t,
    mut vpnew: *mut vg_prefix_t,
) -> *mut vg_prefix_t {
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    let mut ptrp: *mut *mut avl_item_t = 0 as *mut *mut avl_item_t;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    itemp = 0 as *mut libc::c_void as *mut avl_item_t;
    ptrp = &mut (*rootp).ar_root;
    while !(*ptrp).is_null() {
        itemp = *ptrp;
        vp = (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
        tmp___0 = BN_cmp(
            (*vp).vp_low as *const BIGNUM,
            (*vpnew).vp_high as *const BIGNUM,
        );
        if tmp___0 > 0 as libc::c_int {
            ptrp = &mut (*itemp).ai_left;
        } else {
            tmp = BN_cmp(
                (*vp).vp_high as *const BIGNUM,
                (*vpnew).vp_low as *const BIGNUM,
            );
            if tmp < 0 as libc::c_int {
                ptrp = &mut (*itemp).ai_right;
            } else {
                return vp
            }
        }
    }
    (*vpnew).vp_item.ai_up = itemp;
    itemp = &mut (*vpnew).vp_item;
    *ptrp = itemp;
    avl_insert_fix___0(rootp, itemp);
    return 0 as *mut libc::c_void as *mut vg_prefix_t;
}
unsafe extern "C" fn vg_prefix_first(mut rootp: *mut avl_root_t) -> *mut vg_prefix_t {
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    itemp = avl_first(rootp);
    if !itemp.is_null() {
        return (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
    }
    return 0 as *mut libc::c_void as *mut vg_prefix_t;
}
unsafe extern "C" fn vg_prefix_next(mut vp: *mut vg_prefix_t) -> *mut vg_prefix_t {
    let mut itemp: *mut avl_item_t = 0 as *mut avl_item_t;
    itemp = &mut (*vp).vp_item;
    itemp = avl_next(itemp);
    if !itemp.is_null() {
        return (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
    }
    return 0 as *mut libc::c_void as *mut vg_prefix_t;
}
unsafe extern "C" fn vg_prefix_add(
    mut rootp: *mut avl_root_t,
    mut pattern: *const libc::c_char,
    mut low: *mut BIGNUM,
    mut high: *mut BIGNUM,
) -> *mut vg_prefix_t {
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut vp2: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp___2 = BN_cmp(low as *const BIGNUM, high as *const BIGNUM);
    if !(tmp___2 < 0 as libc::c_int) {
        __assert_fail(
            b"BN_cmp(low, high) < 0\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            1047 as libc::c_uint,
            b"vg_prefix_add\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___3 = malloc(::std::mem::size_of::<vg_prefix_t>() as libc::c_ulong);
    vp = tmp___3 as *mut vg_prefix_t;
    if !vp.is_null() {
        avl_item_init(&mut (*vp).vp_item);
        (*vp).vp_sibling = 0 as *mut libc::c_void as *mut _vg_prefix_s;
        (*vp).vp_pattern = pattern;
        (*vp).vp_low = low;
        (*vp).vp_high = high;
        vp2 = vg_prefix_avl_insert(rootp, vp);
        if vp2 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            fprintf(
                stderr,
                b"Prefix '%s' ignored, overlaps '%s'\n\0" as *const u8
                    as *const libc::c_char,
                pattern,
                (*vp2).vp_pattern,
            );
            vg_prefix_free(vp);
            vp = 0 as *mut libc::c_void as *mut vg_prefix_t;
        }
    }
    return vp;
}
unsafe extern "C" fn vg_prefix_delete(
    mut rootp: *mut avl_root_t,
    mut vp: *mut vg_prefix_t,
) {
    let mut sibp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut delp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    avl_remove___0(rootp, &mut (*vp).vp_item);
    sibp = (*vp).vp_sibling;
    while !sibp.is_null() {
        if !(sibp as libc::c_ulong != vp as libc::c_ulong) {
            break;
        }
        avl_remove___0(rootp, &mut (*sibp).vp_item);
        delp = sibp;
        sibp = (*sibp).vp_sibling;
        vg_prefix_free(delp);
    }
    vg_prefix_free(vp);
}
unsafe extern "C" fn vg_prefix_add_ranges(
    mut rootp: *mut avl_root_t,
    mut pattern: *const libc::c_char,
    mut ranges: *mut *mut BIGNUM,
    mut master: *mut vg_prefix_t,
) -> *mut vg_prefix_t {
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut vp2: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    vp2 = 0 as *mut libc::c_void as *mut vg_prefix_t;
    if (*ranges.offset(0 as libc::c_int as isize)).is_null() {
        __assert_fail(
            b"ranges[0]\0" as *const u8 as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            1089 as libc::c_uint,
            b"vg_prefix_add_ranges\0" as *const u8 as *const libc::c_char,
        );
    }
    vp = vg_prefix_add(
        rootp,
        pattern,
        *ranges.offset(0 as libc::c_int as isize),
        *ranges.offset(1 as libc::c_int as isize),
    );
    if vp.is_null() {
        return 0 as *mut libc::c_void as *mut vg_prefix_t;
    }
    if !(*ranges.offset(2 as libc::c_int as isize)).is_null() {
        vp2 = vg_prefix_add(
            rootp,
            pattern,
            *ranges.offset(2 as libc::c_int as isize),
            *ranges.offset(3 as libc::c_int as isize),
        );
        if vp2.is_null() {
            vg_prefix_delete(rootp, vp);
            return 0 as *mut libc::c_void as *mut vg_prefix_t;
        }
    }
    if master.is_null() {
        (*vp).vp_sibling = vp2;
        if !vp2.is_null() {
            (*vp2).vp_sibling = vp;
        }
    } else if !vp2.is_null() {
        (*vp).vp_sibling = vp2;
        if !((*master).vp_sibling).is_null() {
            (*vp2).vp_sibling = (*master).vp_sibling;
        } else {
            (*vp2).vp_sibling = master;
        }
        (*master).vp_sibling = vp;
    } else {
        if !((*master).vp_sibling).is_null() {
            (*vp).vp_sibling = (*master).vp_sibling;
        } else {
            (*vp).vp_sibling = master;
        }
        (*master).vp_sibling = vp;
    }
    return vp;
}
unsafe extern "C" fn vg_prefix_range_sum(
    mut vp: *mut vg_prefix_t,
    mut result: *mut BIGNUM,
    mut tmp1: *mut BIGNUM,
) {
    let mut startp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    startp = vp;
    BN_clear(result);
    loop {
        BN_sub(tmp1, (*vp).vp_high as *const BIGNUM, (*vp).vp_low as *const BIGNUM);
        BN_add(result, result as *const BIGNUM, tmp1 as *const BIGNUM);
        vp = (*vp).vp_sibling;
        if vp.is_null() {
            break;
        }
        if !(vp as libc::c_ulong != startp as libc::c_ulong) {
            break;
        }
    };
}
static mut b58_case_map: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
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
unsafe extern "C" fn prefix_case_iter_init(
    mut cip: *mut prefix_case_iter_t,
    mut pfx: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    (*cip).ci_nbits = 0 as libc::c_int as libc::c_char;
    (*cip).ci_value = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while *pfx.offset(i as isize) != 0 {
        if i as libc::c_ulong
            > ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
        if b58_case_map[*pfx.offset(i as isize) as libc::c_int as usize] == 0 {
            (*cip).ci_prefix[i as usize] = *pfx.offset(i as isize);
        } else if b58_case_map[*pfx.offset(i as isize) as libc::c_int as usize]
                as libc::c_int == 2 as libc::c_int
            {
            (*cip)
                .ci_prefix[i
                as usize] = (*pfx.offset(i as isize) as libc::c_int ^ 32 as libc::c_int)
                as libc::c_char;
        } else {
            (*cip)
                .ci_prefix[i
                as usize] = (*pfx.offset(i as isize) as libc::c_int | 32 as libc::c_int)
                as libc::c_char;
            (*cip)
                .ci_case_map[(*cip).ci_nbits as libc::c_int
                as usize] = i as libc::c_char;
            (*cip)
                .ci_nbits = ((*cip).ci_nbits as libc::c_int + 1 as libc::c_int)
                as libc::c_char;
        }
        i += 1;
    }
    (*cip).ci_prefix[i as usize] = '\u{0}' as i32 as libc::c_char;
    return 1 as libc::c_int;
}
unsafe extern "C" fn prefix_case_iter_next(
    mut cip: *mut prefix_case_iter_t,
) -> libc::c_int {
    let mut val: libc::c_ulong = 0;
    let mut max: libc::c_ulong = 0;
    let mut mask: libc::c_ulong = 0;
    let mut i: libc::c_int = 0;
    let mut nbits: libc::c_int = 0;
    nbits = (*cip).ci_nbits as libc::c_int;
    max = ((1 as libc::c_ulong) << nbits).wrapping_sub(1 as libc::c_ulong);
    val = ((*cip).ci_value + 1 as libc::c_int) as libc::c_ulong;
    if val > max {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    mask = 1 as libc::c_ulong;
    while i < nbits {
        if val & mask != 0 {
            (*cip)
                .ci_prefix[(*cip).ci_case_map[i as usize] as libc::c_int
                as usize] = ((*cip)
                .ci_prefix[(*cip).ci_case_map[i as usize] as libc::c_int as usize]
                as libc::c_int & 223 as libc::c_int) as libc::c_char;
        } else {
            (*cip)
                .ci_prefix[(*cip).ci_case_map[i as usize] as libc::c_int
                as usize] = ((*cip)
                .ci_prefix[(*cip).ci_case_map[i as usize] as libc::c_int as usize]
                as libc::c_int | 32 as libc::c_int) as libc::c_char;
        }
        i += 1;
        mask <<= 1 as libc::c_int;
    }
    (*cip).ci_value = val as libc::c_int;
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_prefix_context_set_case_insensitive(
    mut vcp: *mut vg_context_t,
    mut caseinsensitive: libc::c_int,
) {
    (*(vcp as *mut vg_prefix_context_t)).vcp_caseinsensitive = caseinsensitive;
}
unsafe extern "C" fn vg_prefix_context_clear_all_patterns(mut vcp: *mut vg_context_t) {
    let mut vcpp: *mut vg_prefix_context_t = 0 as *mut vg_prefix_context_t;
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut npfx_left: libc::c_ulong = 0;
    let mut tmp: libc::c_int = 0;
    vcpp = vcp as *mut vg_prefix_context_t;
    npfx_left = 0 as libc::c_ulong;
    loop {
        tmp = avl_root_empty(&mut (*vcpp).vcp_avlroot);
        if tmp != 0 {
            break;
        }
        vp = ((*vcpp).vcp_avlroot.ar_root as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
        vg_prefix_delete(&mut (*vcpp).vcp_avlroot, vp);
        npfx_left = npfx_left.wrapping_add(1);
    }
    if !(npfx_left == (*vcpp).base.vc_npatterns) {
        __assert_fail(
            b"npfx_left == vcpp->base.vc_npatterns\0" as *const u8
                as *const libc::c_char,
            b"pattern.c\0" as *const u8 as *const libc::c_char,
            1233 as libc::c_uint,
            b"vg_prefix_context_clear_all_patterns\0" as *const u8 as *const libc::c_char,
        );
    }
    (*vcpp).base.vc_npatterns = 0 as libc::c_ulong;
    (*vcpp).base.vc_npatterns_start = 0 as libc::c_ulong;
    (*vcpp).base.vc_found = 0 as libc::c_ulonglong;
    BN_clear(&mut (*vcpp).vcp_difficulty);
}
unsafe extern "C" fn vg_prefix_context_free(mut vcp: *mut vg_context_t) {
    let mut vcpp: *mut vg_prefix_context_t = 0 as *mut vg_prefix_context_t;
    vcpp = vcp as *mut vg_prefix_context_t;
    vg_prefix_context_clear_all_patterns(vcp);
    BN_clear_free(&mut (*vcpp).vcp_difficulty);
    free(vcpp as *mut libc::c_void);
}
unsafe extern "C" fn vg_prefix_context_next_difficulty(
    mut vcpp: *mut vg_prefix_context_t,
    mut bntmp: *mut BIGNUM,
    mut bntmp2: *mut BIGNUM,
    mut bnctx: *mut BN_CTX,
) {
    let mut dbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    BN_clear(bntmp);
    BN_set_bit(bntmp, 192 as libc::c_int);
    BN_div(
        bntmp2,
        0 as *mut libc::c_void as *mut BIGNUM,
        bntmp as *const BIGNUM,
        &mut (*vcpp).vcp_difficulty as *mut BIGNUM as *const BIGNUM,
        bnctx,
    );
    dbuf = BN_bn2dec(bntmp2 as *const BIGNUM);
    if (*vcpp).base.vc_verbose > 0 as libc::c_int {
        if (*vcpp).base.vc_npatterns > 1 as libc::c_ulong {
            fprintf(
                stderr,
                b"Next match difficulty: %s (%ld prefixes)\n\0" as *const u8
                    as *const libc::c_char,
                dbuf,
                (*vcpp).base.vc_npatterns,
            );
        } else {
            fprintf(
                stderr,
                b"Difficulty: %s\n\0" as *const u8 as *const libc::c_char,
                dbuf,
            );
        }
    }
    (*vcpp).base.vc_chance = atof(dbuf as *const libc::c_char);
    CRYPTO_free(dbuf as *mut libc::c_void);
}
unsafe extern "C" fn vg_prefix_context_add_patterns(
    mut vcp: *mut vg_context_t,
    patterns: *mut *const libc::c_char,
    mut npatterns: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut vcpp: *mut vg_prefix_context_t = 0 as *mut vg_prefix_context_t;
    let mut caseiter: prefix_case_iter_t = prefix_case_iter_t {
        ci_prefix: [0; 32],
        ci_case_map: [0; 32],
        ci_nbits: 0,
        ci_value: 0,
    };
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut vp2: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut bnctx: *mut BN_CTX = 0 as *mut BN_CTX;
    let mut bntmp: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bntmp2: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bntmp3: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut ranges: [*mut BIGNUM; 4] = [0 as *mut BIGNUM; 4];
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut impossible: libc::c_int = 0;
    let mut case_impossible: libc::c_int = 0;
    let mut npfx: libc::c_ulong = 0;
    let mut dbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut ats: *const libc::c_char = 0 as *const libc::c_char;
    let mut bw: *const libc::c_char = 0 as *const libc::c_char;
    vcpp = vcp as *mut vg_prefix_context_t;
    ret = 0 as libc::c_int;
    impossible = 0 as libc::c_int;
    bnctx = BN_CTX_new();
    BN_init(&mut bntmp);
    BN_init(&mut bntmp2);
    BN_init(&mut bntmp3);
    npfx = 0 as libc::c_ulong;
    i = 0 as libc::c_int;
    while i < npatterns {
        if (*vcpp).vcp_caseinsensitive == 0 {
            vp = 0 as *mut libc::c_void as *mut vg_prefix_t;
            ret = get_prefix_ranges(
                (*vcpp).base.vc_addrtype,
                *patterns.offset(i as isize),
                ranges.as_mut_ptr(),
                bnctx,
            );
            if ret == 0 {
                vp = vg_prefix_add_ranges(
                    &mut (*vcpp).vcp_avlroot,
                    *patterns.offset(i as isize),
                    ranges.as_mut_ptr(),
                    0 as *mut libc::c_void as *mut vg_prefix_t,
                );
            }
            current_block = 6476622998065200121;
        } else {
            tmp = prefix_case_iter_init(&mut caseiter, *patterns.offset(i as isize));
            if tmp == 0 {
                fprintf(
                    stderr,
                    b"Prefix '%s' is too long\n\0" as *const u8 as *const libc::c_char,
                    *patterns.offset(i as isize),
                );
                current_block = 11881057843494000624;
            } else {
                if caseiter.ci_nbits as libc::c_int > 16 as libc::c_int {
                    fprintf(
                        stderr,
                        b"WARNING: Prefix '%s' has 2^%d case-varied derivatives\n\0"
                            as *const u8 as *const libc::c_char,
                        *patterns.offset(i as isize),
                        caseiter.ci_nbits as libc::c_int,
                    );
                }
                case_impossible = 0 as libc::c_int;
                vp = 0 as *mut libc::c_void as *mut vg_prefix_t;
                loop {
                    ret = get_prefix_ranges(
                        (*vcpp).base.vc_addrtype,
                        (caseiter.ci_prefix).as_mut_ptr() as *const libc::c_char,
                        ranges.as_mut_ptr(),
                        bnctx,
                    );
                    if ret == -(2 as libc::c_int) {
                        case_impossible += 1;
                        ret = 0 as libc::c_int;
                    } else {
                        if ret != 0 {
                            break;
                        }
                        vp2 = vg_prefix_add_ranges(
                            &mut (*vcpp).vcp_avlroot,
                            *patterns.offset(i as isize),
                            ranges.as_mut_ptr(),
                            vp,
                        );
                        if vp2.is_null() {
                            ret = -(1 as libc::c_int);
                            break;
                        } else if vp.is_null() {
                            vp = vp2;
                        }
                    }
                    tmp___0 = prefix_case_iter_next(&mut caseiter);
                    if tmp___0 == 0 {
                        break;
                    }
                }
                if vp.is_null() {
                    if case_impossible != 0 {
                        ret = -(2 as libc::c_int);
                    }
                }
                if ret != 0 {
                    if !vp.is_null() {
                        vg_prefix_delete(&mut (*vcpp).vcp_avlroot, vp);
                        vp = 0 as *mut libc::c_void as *mut vg_prefix_t;
                    }
                }
                current_block = 6476622998065200121;
            }
        }
        match current_block {
            6476622998065200121 => {
                if ret == -(2 as libc::c_int) {
                    fprintf(
                        stderr,
                        b"Prefix '%s' not possible\n\0" as *const u8
                            as *const libc::c_char,
                        *patterns.offset(i as isize),
                    );
                    impossible += 1;
                }
                if !vp.is_null() {
                    npfx = npfx.wrapping_add(1);
                    vg_prefix_range_sum(vp, &mut bntmp, &mut bntmp2);
                    BN_add(
                        &mut bntmp2,
                        &mut (*vcpp).vcp_difficulty as *mut BIGNUM as *const BIGNUM,
                        &mut bntmp as *mut BIGNUM as *const BIGNUM,
                    );
                    BN_copy(
                        &mut (*vcpp).vcp_difficulty,
                        &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                    );
                    if (*vcp).vc_verbose > 1 as libc::c_int {
                        BN_clear(&mut bntmp2);
                        BN_set_bit(&mut bntmp2, 192 as libc::c_int);
                        BN_div(
                            &mut bntmp3,
                            0 as *mut libc::c_void as *mut BIGNUM,
                            &mut bntmp2 as *mut BIGNUM as *const BIGNUM,
                            &mut bntmp as *mut BIGNUM as *const BIGNUM,
                            bnctx,
                        );
                        dbuf = BN_bn2dec(&mut bntmp3 as *mut BIGNUM as *const BIGNUM);
                        fprintf(
                            stderr,
                            b"Prefix difficulty: %20s %s\n\0" as *const u8
                                as *const libc::c_char,
                            dbuf,
                            *patterns.offset(i as isize),
                        );
                        CRYPTO_free(dbuf as *mut libc::c_void);
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    (*vcpp).base.vc_npatterns = ((*vcpp).base.vc_npatterns).wrapping_add(npfx);
    (*vcpp)
        .base
        .vc_npatterns_start = ((*vcpp).base.vc_npatterns_start).wrapping_add(npfx);
    if npfx == 0 {
        if impossible != 0 {
            ats = b"bitcoin\0" as *const u8 as *const libc::c_char;
            bw = b"\"1\"\0" as *const u8 as *const libc::c_char;
            match (*vcpp).base.vc_addrtype {
                5 => {
                    ats = b"bitcoin script\0" as *const u8 as *const libc::c_char;
                    bw = b"\"3\"\0" as *const u8 as *const libc::c_char;
                }
                111 => {
                    ats = b"testnet\0" as *const u8 as *const libc::c_char;
                    bw = b"\"m\" or \"n\"\0" as *const u8 as *const libc::c_char;
                }
                52 => {
                    ats = b"namecoin\0" as *const u8 as *const libc::c_char;
                    bw = b"\"M\" or \"N\"\0" as *const u8 as *const libc::c_char;
                }
                _ => {}
            }
            fprintf(
                stderr,
                b"Hint: valid %s addresses begin with %s\n\0" as *const u8
                    as *const libc::c_char,
                ats,
                bw,
            );
        }
    }
    if npfx != 0 {
        vg_prefix_context_next_difficulty(vcpp, &mut bntmp, &mut bntmp2, bnctx);
    }
    ret = (npfx != 0 as libc::c_ulong) as libc::c_int;
    BN_clear_free(&mut bntmp);
    BN_clear_free(&mut bntmp2);
    BN_clear_free(&mut bntmp3);
    BN_CTX_free(bnctx);
    return ret;
}
pub unsafe extern "C" fn vg_prefix_get_difficulty(
    mut addrtype: libc::c_int,
    mut pattern: *const libc::c_char,
) -> libc::c_double {
    let mut bnctx: *mut BN_CTX = 0 as *mut BN_CTX;
    let mut result: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bntmp: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut ranges: [*mut BIGNUM; 4] = [0 as *mut BIGNUM; 4];
    let mut dbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut diffret: libc::c_double = 0.;
    diffret = 0.0f64;
    bnctx = BN_CTX_new();
    BN_init(&mut result);
    BN_init(&mut bntmp);
    ret = get_prefix_ranges(addrtype, pattern, ranges.as_mut_ptr(), bnctx);
    if ret == 0 as libc::c_int {
        BN_sub(
            &mut bntmp,
            ranges[1 as libc::c_int as usize] as *const BIGNUM,
            ranges[0 as libc::c_int as usize] as *const BIGNUM,
        );
        BN_add(
            &mut result,
            &mut result as *mut BIGNUM as *const BIGNUM,
            &mut bntmp as *mut BIGNUM as *const BIGNUM,
        );
        if !(ranges[2 as libc::c_int as usize]).is_null() {
            BN_sub(
                &mut bntmp,
                ranges[3 as libc::c_int as usize] as *const BIGNUM,
                ranges[2 as libc::c_int as usize] as *const BIGNUM,
            );
            BN_add(
                &mut result,
                &mut result as *mut BIGNUM as *const BIGNUM,
                &mut bntmp as *mut BIGNUM as *const BIGNUM,
            );
        }
        free_ranges(ranges.as_mut_ptr());
        BN_clear(&mut bntmp);
        BN_set_bit(&mut bntmp, 192 as libc::c_int);
        BN_div(
            &mut result,
            0 as *mut libc::c_void as *mut BIGNUM,
            &mut bntmp as *mut BIGNUM as *const BIGNUM,
            &mut result as *mut BIGNUM as *const BIGNUM,
            bnctx,
        );
        dbuf = BN_bn2dec(&mut result as *mut BIGNUM as *const BIGNUM);
        diffret = strtod(
            dbuf as *const libc::c_char,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
        );
        CRYPTO_free(dbuf as *mut libc::c_void);
    }
    BN_clear_free(&mut result);
    BN_clear_free(&mut bntmp);
    BN_CTX_free(bnctx);
    return diffret;
}
unsafe extern "C" fn vg_prefix_test(mut vxcp: *mut vg_exec_context_t) -> libc::c_int {
    let mut vcpp: *mut vg_prefix_context_t = 0 as *mut vg_prefix_context_t;
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    vcpp = (*vxcp).vxc_vc as *mut vg_prefix_context_t;
    res = 0 as libc::c_int;
    BN_bin2bn(
        ((*vxcp).vxc_binres).as_mut_ptr() as *const libc::c_uchar,
        25 as libc::c_int,
        &mut (*vxcp).vxc_bntarg,
    );
    loop {
        vp = vg_prefix_avl_search(&mut (*vcpp).vcp_avlroot, &mut (*vxcp).vxc_bntarg);
        if vp.is_null() {
            break;
        }
        tmp = vg_exec_context_upgrade_lock(vxcp);
        if tmp != 0 {
            continue;
        }
        vg_exec_context_consolidate_key(vxcp);
        (Some(((*vcpp).base.vc_output_match).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(&mut (*vcpp).base, (*vxcp).vxc_key, (*vp).vp_pattern);
        (*vcpp).base.vc_found = ((*vcpp).base.vc_found).wrapping_add(1);
        if (*vcpp).base.vc_only_one != 0 {
            return 2 as libc::c_int;
        }
        if (*vcpp).base.vc_remove_on_match != 0 {
            vg_prefix_range_sum(vp, &mut (*vxcp).vxc_bntarg, &mut (*vxcp).vxc_bntmp);
            BN_sub(
                &mut (*vxcp).vxc_bntmp,
                &mut (*vcpp).vcp_difficulty as *mut BIGNUM as *const BIGNUM,
                &mut (*vxcp).vxc_bntarg as *mut BIGNUM as *const BIGNUM,
            );
            BN_copy(
                &mut (*vcpp).vcp_difficulty,
                &mut (*vxcp).vxc_bntmp as *mut BIGNUM as *const BIGNUM,
            );
            vg_prefix_delete(&mut (*vcpp).vcp_avlroot, vp);
            (*vcpp).base.vc_npatterns = ((*vcpp).base.vc_npatterns).wrapping_sub(1);
            tmp___0 = avl_root_empty(&mut (*vcpp).vcp_avlroot);
            if tmp___0 == 0 {
                vg_prefix_context_next_difficulty(
                    vcpp,
                    &mut (*vxcp).vxc_bntmp,
                    &mut (*vxcp).vxc_bntmp2,
                    (*vxcp).vxc_bnctx,
                );
            }
            (*vcpp).base.vc_pattern_generation += 1;
        }
        res = 1 as libc::c_int;
        break;
    }
    tmp___1 = avl_root_empty(&mut (*vcpp).vcp_avlroot);
    if tmp___1 != 0 {
        return 2 as libc::c_int;
    }
    return res;
}
unsafe extern "C" fn vg_prefix_hash160_sort(
    mut vcp: *mut vg_context_t,
    mut buf: *mut libc::c_void,
) -> libc::c_int {
    let mut vcpp: *mut vg_prefix_context_t = 0 as *mut vg_prefix_context_t;
    let mut vp: *mut vg_prefix_t = 0 as *mut vg_prefix_t;
    let mut cbuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bnbuf: [libc::c_uchar; 25] = [0; 25];
    let mut nbytes: libc::c_int = 0;
    let mut ncopy: libc::c_int = 0;
    let mut nskip: libc::c_int = 0;
    let mut npfx: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    vcpp = vcp as *mut vg_prefix_context_t;
    cbuf = buf as *mut libc::c_uchar;
    npfx = 0 as libc::c_int;
    vp = vg_prefix_first(&mut (*vcpp).vcp_avlroot);
    while vp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        npfx += 1;
        if !buf.is_null() {
            nbytes = BN_bn2bin((*vp).vp_low as *const BIGNUM, bnbuf.as_mut_ptr());
            if nbytes >= 24 as libc::c_int {
                ncopy = 20 as libc::c_int;
            } else {
                if nbytes > 4 as libc::c_int {
                    tmp = nbytes - 4 as libc::c_int;
                } else {
                    tmp = 0 as libc::c_int;
                }
                ncopy = tmp;
            }
            if nbytes >= 24 as libc::c_int {
                nskip = nbytes - 24 as libc::c_int;
            } else {
                nskip = 0 as libc::c_int;
            }
            if ncopy < 20 as libc::c_int {
                memset(
                    cbuf as *mut libc::c_void,
                    0 as libc::c_int,
                    (20 as libc::c_int - ncopy) as size_t,
                );
            }
            memcpy(
                cbuf.offset((20 as libc::c_int - ncopy) as isize) as *mut libc::c_void,
                bnbuf.as_mut_ptr().offset(nskip as isize) as *const libc::c_void,
                ncopy as size_t,
            );
            cbuf = cbuf.offset(20 as libc::c_int as isize);
            nbytes = BN_bn2bin((*vp).vp_high as *const BIGNUM, bnbuf.as_mut_ptr());
            if nbytes >= 24 as libc::c_int {
                ncopy = 20 as libc::c_int;
            } else {
                if nbytes > 4 as libc::c_int {
                    tmp___0 = nbytes - 4 as libc::c_int;
                } else {
                    tmp___0 = 0 as libc::c_int;
                }
                ncopy = tmp___0;
            }
            if nbytes >= 24 as libc::c_int {
                nskip = nbytes - 24 as libc::c_int;
            } else {
                nskip = 0 as libc::c_int;
            }
            if ncopy < 20 as libc::c_int {
                memset(
                    cbuf as *mut libc::c_void,
                    0 as libc::c_int,
                    (20 as libc::c_int - ncopy) as size_t,
                );
            }
            memcpy(
                cbuf.offset((20 as libc::c_int - ncopy) as isize) as *mut libc::c_void,
                bnbuf.as_mut_ptr().offset(nskip as isize) as *const libc::c_void,
                ncopy as size_t,
            );
            cbuf = cbuf.offset(20 as libc::c_int as isize);
        }
        vp = vg_prefix_next(vp);
    }
    return npfx;
}
pub unsafe extern "C" fn vg_prefix_context_new(
    mut addrtype: libc::c_int,
    mut privtype: libc::c_int,
    mut caseinsensitive: libc::c_int,
) -> *mut vg_context_t {
    let mut vcpp: *mut vg_prefix_context_t = 0 as *mut vg_prefix_context_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<vg_prefix_context_t>() as libc::c_ulong);
    vcpp = tmp as *mut vg_prefix_context_t;
    if !vcpp.is_null() {
        memset(
            vcpp as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<vg_prefix_context_t>() as libc::c_ulong,
        );
        (*vcpp).base.vc_addrtype = addrtype;
        (*vcpp).base.vc_privtype = privtype;
        (*vcpp).base.vc_npatterns = 0 as libc::c_ulong;
        (*vcpp).base.vc_npatterns_start = 0 as libc::c_ulong;
        (*vcpp).base.vc_found = 0 as libc::c_ulonglong;
        (*vcpp).base.vc_chance = 0.0f64;
        (*vcpp)
            .base
            .vc_free = Some(
            vg_prefix_context_free as unsafe extern "C" fn(*mut vg_context_t) -> (),
        );
        (*vcpp)
            .base
            .vc_add_patterns = Some(
            vg_prefix_context_add_patterns
                as unsafe extern "C" fn(
                    *mut vg_context_t,
                    *mut *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        );
        (*vcpp)
            .base
            .vc_clear_all_patterns = Some(
            vg_prefix_context_clear_all_patterns
                as unsafe extern "C" fn(*mut vg_context_t) -> (),
        );
        (*vcpp)
            .base
            .vc_test = Some(
            vg_prefix_test as unsafe extern "C" fn(*mut vg_exec_context_t) -> libc::c_int,
        );
        (*vcpp)
            .base
            .vc_hash160_sort = Some(
            vg_prefix_hash160_sort
                as unsafe extern "C" fn(
                    *mut vg_context_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
        );
        avl_root_init(&mut (*vcpp).vcp_avlroot);
        BN_init(&mut (*vcpp).vcp_difficulty);
        (*vcpp).vcp_caseinsensitive = caseinsensitive;
    }
    return &mut (*vcpp).base;
}
unsafe extern "C" fn vg_regex_context_add_patterns(
    mut vcp: *mut vg_context_t,
    patterns: *mut *const libc::c_char,
    mut npatterns: libc::c_int,
) -> libc::c_int {
    let mut vcrp: *mut vg_regex_context_t = 0 as *mut vg_regex_context_t;
    let mut pcre_errptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut pcre_erroffset: libc::c_int = 0;
    let mut i: libc::c_ulong = 0;
    let mut nres: libc::c_ulong = 0;
    let mut count: libc::c_ulong = 0;
    let mut mem: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut spaces: *const libc::c_char = 0 as *const libc::c_char;
    vcrp = vcp as *mut vg_regex_context_t;
    if npatterns == 0 {
        return 1 as libc::c_int;
    }
    if npatterns as libc::c_ulong
        > ((*vcrp).vcr_nalloc).wrapping_sub((*vcrp).base.vc_npatterns)
    {
        count = (npatterns as libc::c_ulong).wrapping_add((*vcrp).base.vc_npatterns);
        if count < (2 as libc::c_ulong).wrapping_mul((*vcrp).vcr_nalloc) {
            count = (2 as libc::c_ulong).wrapping_mul((*vcrp).vcr_nalloc);
        }
        if count < 16 as libc::c_ulong {
            count = 16 as libc::c_ulong;
        }
        tmp = malloc(
            (3 as libc::c_ulong)
                .wrapping_mul(count)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
        mem = tmp as *mut *mut libc::c_void;
        if mem.is_null() {
            return 0 as libc::c_int;
        }
        i = 0 as libc::c_ulong;
        while i < (*vcrp).base.vc_npatterns {
            let ref mut fresh8 = *mem.offset(i as isize);
            *fresh8 = *((*vcrp).vcr_regex).offset(i as isize) as *mut libc::c_void;
            let ref mut fresh9 = *mem.offset(count.wrapping_add(i) as isize);
            *fresh9 = *((*vcrp).vcr_regex_extra).offset(i as isize) as *mut libc::c_void;
            let ref mut fresh10 = *mem
                .offset(
                    (2 as libc::c_ulong).wrapping_mul(count).wrapping_add(i) as isize,
                );
            *fresh10 = *((*vcrp).vcr_regex_pat).offset(i as isize) as *mut libc::c_void;
            i = i.wrapping_add(1);
        }
        if (*vcrp).vcr_nalloc != 0 {
            free((*vcrp).vcr_regex as *mut libc::c_void);
        }
        (*vcrp).vcr_regex = mem as *mut *mut pcre;
        (*vcrp).vcr_regex_extra = mem.offset(count as isize) as *mut *mut pcre_extra;
        (*vcrp)
            .vcr_regex_pat = mem
            .offset((2 as libc::c_ulong).wrapping_mul(count) as isize)
            as *mut *const libc::c_char;
        (*vcrp).vcr_nalloc = count;
    }
    nres = (*vcrp).base.vc_npatterns;
    i = 0 as libc::c_ulong;
    while i < npatterns as libc::c_ulong {
        let ref mut fresh11 = *((*vcrp).vcr_regex).offset(nres as isize);
        *fresh11 = pcre_compile(
            *patterns.offset(i as isize),
            0 as libc::c_int,
            &mut pcre_errptr,
            &mut pcre_erroffset,
            0 as *mut libc::c_void as *const libc::c_uchar,
        );
        if (*((*vcrp).vcr_regex).offset(nres as isize)).is_null() {
            spaces = b"                \0" as *const u8 as *const libc::c_char;
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                *patterns.offset(i as isize),
            );
            while pcre_erroffset > 16 as libc::c_int {
                fprintf(stderr, b"%s\0" as *const u8 as *const libc::c_char, spaces);
                pcre_erroffset -= 16 as libc::c_int;
            }
            if pcre_erroffset > 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    spaces.offset((16 as libc::c_int - pcre_erroffset) as isize),
                );
            }
            fprintf(
                stderr,
                b"^\nRegex error: %s\n\0" as *const u8 as *const libc::c_char,
                pcre_errptr,
            );
        } else {
            let ref mut fresh12 = *((*vcrp).vcr_regex_extra).offset(nres as isize);
            *fresh12 = pcre_study(
                *((*vcrp).vcr_regex).offset(nres as isize) as *const pcre,
                0 as libc::c_int,
                &mut pcre_errptr,
            );
            if !pcre_errptr.is_null() {
                fprintf(
                    stderr,
                    b"Regex error: %s\n\0" as *const u8 as *const libc::c_char,
                    pcre_errptr,
                );
                (Some(pcre_free.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(*((*vcrp).vcr_regex).offset(nres as isize) as *mut libc::c_void);
            } else {
                let ref mut fresh13 = *((*vcrp).vcr_regex_pat).offset(nres as isize);
                *fresh13 = *patterns.offset(i as isize);
                nres = nres.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    if nres == (*vcrp).base.vc_npatterns {
        return 0 as libc::c_int;
    }
    (*vcrp)
        .base
        .vc_npatterns_start = ((*vcrp).base.vc_npatterns_start)
        .wrapping_add(nres.wrapping_sub((*vcrp).base.vc_npatterns));
    (*vcrp).base.vc_npatterns = nres;
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_regex_context_clear_all_patterns(mut vcp: *mut vg_context_t) {
    let mut vcrp: *mut vg_regex_context_t = 0 as *mut vg_regex_context_t;
    let mut i: libc::c_int = 0;
    vcrp = vcp as *mut vg_regex_context_t;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*vcrp).base.vc_npatterns {
        if !(*((*vcrp).vcr_regex_extra).offset(i as isize)).is_null() {
            (Some(pcre_free.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(*((*vcrp).vcr_regex_extra).offset(i as isize) as *mut libc::c_void);
        }
        (Some(pcre_free.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(*((*vcrp).vcr_regex).offset(i as isize) as *mut libc::c_void);
        i += 1;
    }
    (*vcrp).base.vc_npatterns = 0 as libc::c_ulong;
    (*vcrp).base.vc_npatterns_start = 0 as libc::c_ulong;
    (*vcrp).base.vc_found = 0 as libc::c_ulonglong;
}
unsafe extern "C" fn vg_regex_context_free(mut vcp: *mut vg_context_t) {
    let mut vcrp: *mut vg_regex_context_t = 0 as *mut vg_regex_context_t;
    vcrp = vcp as *mut vg_regex_context_t;
    vg_regex_context_clear_all_patterns(vcp);
    if (*vcrp).vcr_nalloc != 0 {
        free((*vcrp).vcr_regex as *mut libc::c_void);
    }
    free(vcrp as *mut libc::c_void);
}
unsafe extern "C" fn vg_regex_test(mut vxcp: *mut vg_exec_context_t) -> libc::c_int {
    let mut vcrp: *mut vg_regex_context_t = 0 as *mut vg_regex_context_t;
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    let mut hash2: [libc::c_uchar; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut zpfx: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut nres: libc::c_int = 0;
    let mut re_vec: [libc::c_int; 9] = [0; 9];
    let mut b58: [libc::c_char; 40] = [0; 40];
    let mut bnrem: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bn: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bndiv: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnptmp: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut res: libc::c_int = 0;
    let mut re: *mut pcre = 0 as *mut pcre;
    let mut tmp: libc::c_ulong = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    vcrp = (*vxcp).vxc_vc as *mut vg_regex_context_t;
    res = 0 as libc::c_int;
    BN_init(&mut bnrem);
    SHA256(
        ((*vxcp).vxc_binres).as_mut_ptr() as *const libc::c_uchar,
        21 as libc::c_int as size_t,
        hash1.as_mut_ptr(),
    );
    SHA256(
        hash1.as_mut_ptr() as *const libc::c_uchar,
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        hash2.as_mut_ptr(),
    );
    memcpy(
        &mut *((*vxcp).vxc_binres).as_mut_ptr().offset(21 as libc::c_int as isize)
            as *mut libc::c_uchar as *mut libc::c_void,
        hash2.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as size_t,
    );
    bn = &mut (*vxcp).vxc_bntmp;
    bndiv = &mut (*vxcp).vxc_bntmp2;
    BN_bin2bn(
        ((*vxcp).vxc_binres).as_mut_ptr() as *const libc::c_uchar,
        25 as libc::c_int,
        bn,
    );
    zpfx = 0 as libc::c_int;
    while zpfx < 25 as libc::c_int {
        if !((*vxcp).vxc_binres[zpfx as usize] as libc::c_int == 0 as libc::c_int) {
            break;
        }
        zpfx += 1;
    }
    p = (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_ulong) as libc::c_int;
    b58[p as usize] = '\u{0}' as i32 as libc::c_char;
    while !((*bn).top == 0 as libc::c_int) {
        BN_div(
            bndiv,
            &mut bnrem,
            bn as *const BIGNUM,
            &mut (*vxcp).vxc_bnbase as *mut BIGNUM as *const BIGNUM,
            (*vxcp).vxc_bnctx,
        );
        bnptmp = bn;
        bn = bndiv;
        bndiv = bnptmp;
        tmp = BN_get_word(&mut bnrem as *mut BIGNUM as *const BIGNUM);
        d = tmp as libc::c_int;
        p -= 1;
        b58[p as usize] = *vg_b58_alphabet.offset(d as isize);
    }
    loop {
        tmp___0 = zpfx;
        zpfx -= 1;
        if tmp___0 == 0 {
            break;
        }
        p -= 1;
        b58[p as usize] = *vg_b58_alphabet.offset(0 as libc::c_int as isize);
    }
    '_restart_loop: loop {
        nres = (*vcrp).base.vc_npatterns as libc::c_int;
        if nres == 0 {
            res = 2 as libc::c_int;
            break;
        } else {
            i = 0 as libc::c_int;
            loop {
                if !(i < nres) {
                    break '_restart_loop;
                }
                d = pcre_exec(
                    *((*vcrp).vcr_regex).offset(i as isize) as *const pcre,
                    *((*vcrp).vcr_regex_extra).offset(i as isize) as *const pcre_extra,
                    &mut *b58.as_mut_ptr().offset(p as isize) as *mut libc::c_char
                        as *const libc::c_char,
                    (::std::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_ulong)
                        .wrapping_sub(p as libc::c_ulong) as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    re_vec.as_mut_ptr(),
                    (::std::mem::size_of::<[libc::c_int; 9]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) as libc::c_int,
                );
                if d <= 0 as libc::c_int {
                    if d != -(1 as libc::c_int) {
                        fprintf(
                            stderr,
                            b"PCRE error: %d\n\0" as *const u8 as *const libc::c_char,
                            d,
                        );
                        res = 2 as libc::c_int;
                        break '_restart_loop;
                    }
                } else {
                    re = *((*vcrp).vcr_regex).offset(i as isize);
                    tmp___1 = vg_exec_context_upgrade_lock(vxcp);
                    if tmp___1 != 0 {
                        if i as libc::c_ulong >= (*vcrp).base.vc_npatterns {
                            break;
                        }
                        if *((*vcrp).vcr_regex).offset(i as isize) as libc::c_ulong
                            != re as libc::c_ulong
                        {
                            break;
                        }
                    }
                    vg_exec_context_consolidate_key(vxcp);
                    (Some(
                        ((*vcrp).base.vc_output_match)
                            .expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut (*vcrp).base,
                        (*vxcp).vxc_key,
                        *((*vcrp).vcr_regex_pat).offset(i as isize),
                    );
                    (*vcrp).base.vc_found = ((*vcrp).base.vc_found).wrapping_add(1);
                    if (*vcrp).base.vc_only_one != 0 {
                        res = 2 as libc::c_int;
                        break '_restart_loop;
                    } else {
                        if (*vcrp).base.vc_remove_on_match != 0 {
                            (Some(pcre_free.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(
                                *((*vcrp).vcr_regex).offset(i as isize) as *mut libc::c_void,
                            );
                            if !(*((*vcrp).vcr_regex_extra).offset(i as isize)).is_null()
                            {
                                (Some(pcre_free.expect("non-null function pointer")))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    *((*vcrp).vcr_regex_extra).offset(i as isize)
                                        as *mut libc::c_void,
                                );
                            }
                            nres -= 1;
                            (*vcrp).base.vc_npatterns = nres as libc::c_ulong;
                            if nres == 0 {
                                res = 2 as libc::c_int;
                                break '_restart_loop;
                            } else {
                                let ref mut fresh14 = *((*vcrp).vcr_regex)
                                    .offset(i as isize);
                                *fresh14 = *((*vcrp).vcr_regex).offset(nres as isize);
                                let ref mut fresh15 = *((*vcrp).vcr_regex_extra)
                                    .offset(i as isize);
                                *fresh15 = *((*vcrp).vcr_regex_extra).offset(nres as isize);
                                let ref mut fresh16 = *((*vcrp).vcr_regex_pat)
                                    .offset(i as isize);
                                *fresh16 = *((*vcrp).vcr_regex_pat).offset(nres as isize);
                                (*vcrp).base.vc_npatterns = nres as libc::c_ulong;
                                (*vcrp).base.vc_pattern_generation += 1;
                            }
                        }
                        res = 1 as libc::c_int;
                    }
                }
                i += 1;
            }
        }
    }
    BN_clear_free(&mut bnrem);
    return res;
}
pub unsafe extern "C" fn vg_regex_context_new(
    mut addrtype: libc::c_int,
    mut privtype: libc::c_int,
) -> *mut vg_context_t {
    let mut vcrp: *mut vg_regex_context_t = 0 as *mut vg_regex_context_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<vg_regex_context_t>() as libc::c_ulong);
    vcrp = tmp as *mut vg_regex_context_t;
    if !vcrp.is_null() {
        memset(
            vcrp as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<vg_regex_context_t>() as libc::c_ulong,
        );
        (*vcrp).base.vc_addrtype = addrtype;
        (*vcrp).base.vc_privtype = privtype;
        (*vcrp).base.vc_npatterns = 0 as libc::c_ulong;
        (*vcrp).base.vc_npatterns_start = 0 as libc::c_ulong;
        (*vcrp).base.vc_found = 0 as libc::c_ulonglong;
        (*vcrp).base.vc_chance = 0.0f64;
        (*vcrp)
            .base
            .vc_free = Some(
            vg_regex_context_free as unsafe extern "C" fn(*mut vg_context_t) -> (),
        );
        (*vcrp)
            .base
            .vc_add_patterns = Some(
            vg_regex_context_add_patterns
                as unsafe extern "C" fn(
                    *mut vg_context_t,
                    *mut *const libc::c_char,
                    libc::c_int,
                ) -> libc::c_int,
        );
        (*vcrp)
            .base
            .vc_clear_all_patterns = Some(
            vg_regex_context_clear_all_patterns
                as unsafe extern "C" fn(*mut vg_context_t) -> (),
        );
        (*vcrp)
            .base
            .vc_test = Some(
            vg_regex_test as unsafe extern "C" fn(*mut vg_exec_context_t) -> libc::c_int,
        );
        (*vcrp)
            .base
            .vc_hash160_sort = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<
                unsafe extern "C" fn(*mut vg_context_t, *mut libc::c_void) -> libc::c_int,
            >,
        >(0 as *mut libc::c_void);
        (*vcrp).vcr_regex = 0 as *mut libc::c_void as *mut *mut pcre;
        (*vcrp).vcr_nalloc = 0 as libc::c_ulong;
    }
    return &mut (*vcrp).base;
}
pub static mut vg_b58_alphabet: *const libc::c_char = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz\0"
    as *const u8 as *const libc::c_char;
pub static mut vg_b58_reverse_map: [libc::c_schar; 256] = [
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    0 as libc::c_int as libc::c_schar,
    1 as libc::c_int as libc::c_schar,
    2 as libc::c_int as libc::c_schar,
    3 as libc::c_int as libc::c_schar,
    4 as libc::c_int as libc::c_schar,
    5 as libc::c_int as libc::c_schar,
    6 as libc::c_int as libc::c_schar,
    7 as libc::c_int as libc::c_schar,
    8 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    9 as libc::c_int as libc::c_schar,
    10 as libc::c_int as libc::c_schar,
    11 as libc::c_int as libc::c_schar,
    12 as libc::c_int as libc::c_schar,
    13 as libc::c_int as libc::c_schar,
    14 as libc::c_int as libc::c_schar,
    15 as libc::c_int as libc::c_schar,
    16 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    17 as libc::c_int as libc::c_schar,
    18 as libc::c_int as libc::c_schar,
    19 as libc::c_int as libc::c_schar,
    20 as libc::c_int as libc::c_schar,
    21 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    22 as libc::c_int as libc::c_schar,
    23 as libc::c_int as libc::c_schar,
    24 as libc::c_int as libc::c_schar,
    25 as libc::c_int as libc::c_schar,
    26 as libc::c_int as libc::c_schar,
    27 as libc::c_int as libc::c_schar,
    28 as libc::c_int as libc::c_schar,
    29 as libc::c_int as libc::c_schar,
    30 as libc::c_int as libc::c_schar,
    31 as libc::c_int as libc::c_schar,
    32 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    33 as libc::c_int as libc::c_schar,
    34 as libc::c_int as libc::c_schar,
    35 as libc::c_int as libc::c_schar,
    36 as libc::c_int as libc::c_schar,
    37 as libc::c_int as libc::c_schar,
    38 as libc::c_int as libc::c_schar,
    39 as libc::c_int as libc::c_schar,
    40 as libc::c_int as libc::c_schar,
    41 as libc::c_int as libc::c_schar,
    42 as libc::c_int as libc::c_schar,
    43 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    44 as libc::c_int as libc::c_schar,
    45 as libc::c_int as libc::c_schar,
    46 as libc::c_int as libc::c_schar,
    47 as libc::c_int as libc::c_schar,
    48 as libc::c_int as libc::c_schar,
    49 as libc::c_int as libc::c_schar,
    50 as libc::c_int as libc::c_schar,
    51 as libc::c_int as libc::c_schar,
    52 as libc::c_int as libc::c_schar,
    53 as libc::c_int as libc::c_schar,
    54 as libc::c_int as libc::c_schar,
    55 as libc::c_int as libc::c_schar,
    56 as libc::c_int as libc::c_schar,
    57 as libc::c_int as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
    -(1 as libc::c_int) as libc::c_schar,
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
pub unsafe extern "C" fn fdumphex(
    mut fp: *mut FILE,
    mut src: *const libc::c_uchar,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < len {
        fprintf(
            fp,
            b"%02x\0" as *const u8 as *const libc::c_char,
            *src.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn fdumpbn(mut fp: *mut FILE, mut bn: *const BIGNUM) {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    buf = BN_bn2hex(bn);
    if !buf.is_null() {
        tmp = buf as *const libc::c_char;
    } else {
        tmp = b"0\0" as *const u8 as *const libc::c_char;
    }
    fprintf(fp, b"%s\n\0" as *const u8 as *const libc::c_char, tmp);
    if !buf.is_null() {
        CRYPTO_free(buf as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn dumphex(mut src: *const libc::c_uchar, mut len: size_t) {
    fdumphex(stdout, src, len);
}
pub unsafe extern "C" fn dumpbn(mut bn: *const BIGNUM) {
    fdumpbn(stdout, bn);
}
pub unsafe extern "C" fn vg_b58_encode_check(
    mut buf: *mut libc::c_void,
    mut len: size_t,
    mut result: *mut libc::c_char,
) {
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    let mut hash2: [libc::c_uchar; 32] = [0; 32];
    let mut d: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut bnctx: *mut BN_CTX = 0 as *mut BN_CTX;
    let mut bn: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bndiv: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bntmp: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bna: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnb: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnbase: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnrem: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut binres: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut brlen: libc::c_int = 0;
    let mut zpfx: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_ulong = 0;
    let mut tmp___1: libc::c_int = 0;
    bnctx = BN_CTX_new();
    BN_init(&mut bna);
    BN_init(&mut bnb);
    BN_init(&mut bnbase);
    BN_init(&mut bnrem);
    BN_set_word(&mut bnbase, 58 as libc::c_ulong);
    bn = &mut bna;
    bndiv = &mut bnb;
    brlen = (2 as libc::c_ulong).wrapping_mul(len).wrapping_add(4 as libc::c_ulong)
        as libc::c_int;
    tmp = malloc(brlen as size_t);
    binres = tmp as *mut libc::c_uchar;
    memcpy(binres as *mut libc::c_void, buf as *const libc::c_void, len);
    SHA256(binres as *const libc::c_uchar, len, hash1.as_mut_ptr());
    SHA256(
        hash1.as_mut_ptr() as *const libc::c_uchar,
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        hash2.as_mut_ptr(),
    );
    memcpy(
        binres.offset(len as isize) as *mut libc::c_void,
        hash2.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as size_t,
    );
    BN_bin2bn(
        binres as *const libc::c_uchar,
        len.wrapping_add(4 as libc::c_ulong) as libc::c_int,
        bn,
    );
    zpfx = 0 as libc::c_int;
    while (zpfx as size_t) < len.wrapping_add(4 as libc::c_ulong) {
        if !(*binres.offset(zpfx as isize) as libc::c_int == 0 as libc::c_int) {
            break;
        }
        zpfx += 1;
    }
    p = brlen;
    while !((*bn).top == 0 as libc::c_int) {
        BN_div(
            bndiv,
            &mut bnrem,
            bn as *const BIGNUM,
            &mut bnbase as *mut BIGNUM as *const BIGNUM,
            bnctx,
        );
        bntmp = bn;
        bn = bndiv;
        bndiv = bntmp;
        tmp___0 = BN_get_word(&mut bnrem as *mut BIGNUM as *const BIGNUM);
        d = tmp___0 as libc::c_int;
        p -= 1;
        *binres
            .offset(p as isize) = *vg_b58_alphabet.offset(d as isize) as libc::c_uchar;
    }
    loop {
        tmp___1 = zpfx;
        zpfx -= 1;
        if tmp___1 == 0 {
            break;
        }
        p -= 1;
        *binres
            .offset(
                p as isize,
            ) = *vg_b58_alphabet.offset(0 as libc::c_int as isize) as libc::c_uchar;
    }
    memcpy(
        result as *mut libc::c_void,
        binres.offset(p as isize) as *const libc::c_void,
        (brlen - p) as size_t,
    );
    *result.offset((brlen - p) as isize) = '\u{0}' as i32 as libc::c_char;
    free(binres as *mut libc::c_void);
    BN_clear_free(&mut bna);
    BN_clear_free(&mut bnb);
    BN_clear_free(&mut bnbase);
    BN_clear_free(&mut bnrem);
    BN_CTX_free(bnctx);
}
pub unsafe extern "C" fn vg_b58_decode_check(
    mut input: *const libc::c_char,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut xbuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut bn: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnw: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnbase: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnctx: *mut BN_CTX = 0 as *mut BN_CTX;
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    let mut hash2: [libc::c_uchar; 32] = [0; 32];
    let mut zpfx: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: libc::c_int = 0;
    xbuf = 0 as *mut libc::c_void as *mut libc::c_uchar;
    res = 0 as libc::c_int;
    BN_init(&mut bn);
    BN_init(&mut bnw);
    BN_init(&mut bnbase);
    BN_set_word(&mut bnbase, 58 as libc::c_ulong);
    bnctx = BN_CTX_new();
    tmp = strlen(input);
    l = tmp as libc::c_int;
    i = 0 as libc::c_int;
    loop {
        if !(i < l) {
            current_block = 7172762164747879670;
            break;
        }
        if !(*input.offset(i as isize) as libc::c_int == 13 as libc::c_int) {
            if !(*input.offset(i as isize) as libc::c_int == 10 as libc::c_int) {
                if !(*input.offset(i as isize) as libc::c_int == 32 as libc::c_int) {
                    if !(*input.offset(i as isize) as libc::c_int == 9 as libc::c_int) {
                        c = vg_b58_reverse_map[*input.offset(i as isize) as libc::c_int
                            as usize] as libc::c_int;
                        if c < 0 as libc::c_int {
                            current_block = 628418260863286395;
                            break;
                        }
                        BN_clear(&mut bnw);
                        BN_set_word(&mut bnw, c as libc::c_ulong);
                        BN_mul(
                            &mut bn,
                            &mut bn as *mut BIGNUM as *const BIGNUM,
                            &mut bnbase as *mut BIGNUM as *const BIGNUM,
                            bnctx,
                        );
                        BN_add(
                            &mut bn,
                            &mut bn as *mut BIGNUM as *const BIGNUM,
                            &mut bnw as *mut BIGNUM as *const BIGNUM,
                        );
                    }
                }
            }
        }
        i += 1;
    }
    match current_block {
        7172762164747879670 => {
            i = 0 as libc::c_int;
            zpfx = 0 as libc::c_int;
            while *input.offset(i as isize) != 0 {
                if !(*input.offset(i as isize) as libc::c_int == 13 as libc::c_int) {
                    if !(*input.offset(i as isize) as libc::c_int == 10 as libc::c_int) {
                        if !(*input.offset(i as isize) as libc::c_int
                            == 32 as libc::c_int)
                        {
                            if !(*input.offset(i as isize) as libc::c_int
                                == 9 as libc::c_int)
                            {
                                if *input.offset(i as isize) as libc::c_int
                                    != *vg_b58_alphabet.offset(0 as libc::c_int as isize)
                                        as libc::c_int
                                {
                                    break;
                                }
                                zpfx += 1;
                            }
                        }
                    }
                }
                i += 1;
            }
            tmp___0 = BN_num_bits(&mut bn as *mut BIGNUM as *const BIGNUM);
            c = (tmp___0 + 7 as libc::c_int) / 8 as libc::c_int;
            l = zpfx + c;
            if !(l < 5 as libc::c_int) {
                tmp___1 = malloc(l as size_t);
                xbuf = tmp___1 as *mut libc::c_uchar;
                if !xbuf.is_null() {
                    if zpfx != 0 {
                        memset(
                            xbuf as *mut libc::c_void,
                            0 as libc::c_int,
                            zpfx as size_t,
                        );
                    }
                    if c != 0 {
                        BN_bn2bin(
                            &mut bn as *mut BIGNUM as *const BIGNUM,
                            xbuf.offset(zpfx as isize),
                        );
                    }
                    l -= 4 as libc::c_int;
                    SHA256(
                        xbuf as *const libc::c_uchar,
                        l as size_t,
                        hash1.as_mut_ptr(),
                    );
                    SHA256(
                        hash1.as_mut_ptr() as *const libc::c_uchar,
                        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
                        hash2.as_mut_ptr(),
                    );
                    tmp___2 = memcmp(
                        hash2.as_mut_ptr() as *const libc::c_void,
                        xbuf.offset(l as isize) as *const libc::c_void,
                        4 as libc::c_int as size_t,
                    );
                    if !(tmp___2 != 0) {
                        if len != 0 {
                            if len > l as size_t {
                                len = l as size_t;
                            }
                            memcpy(buf, xbuf as *const libc::c_void, len);
                        }
                        res = l;
                    }
                }
            }
        }
        _ => {}
    }
    if !xbuf.is_null() {
        free(xbuf as *mut libc::c_void);
    }
    BN_clear_free(&mut bn);
    BN_clear_free(&mut bnw);
    BN_clear_free(&mut bnbase);
    BN_CTX_free(bnctx);
    return res;
}
pub unsafe extern "C" fn vg_encode_address(
    mut ppoint: *const EC_POINT,
    mut pgroup: *const EC_GROUP,
    mut addrtype: libc::c_int,
    mut result: *mut libc::c_char,
) {
    let mut eckey_buf: [libc::c_uchar; 128] = [0; 128];
    let mut pend: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut binres: [libc::c_uchar; 21] = [0; 21];
    let mut tmp: libc::c_uint = 0;
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    binres[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 21 as libc::c_uint) {
        binres[tmp as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp = tmp.wrapping_add(1);
    }
    pend = eckey_buf.as_mut_ptr();
    EC_POINT_point2oct(
        pgroup,
        ppoint,
        POINT_CONVERSION_UNCOMPRESSED,
        eckey_buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong,
        0 as *mut libc::c_void as *mut BN_CTX,
    );
    pend = eckey_buf.as_mut_ptr().offset(65 as libc::c_int as isize);
    binres[0 as libc::c_int as usize] = addrtype as libc::c_uchar;
    SHA256(
        eckey_buf.as_mut_ptr() as *const libc::c_uchar,
        pend.offset_from(eckey_buf.as_mut_ptr()) as libc::c_long as size_t,
        hash1.as_mut_ptr(),
    );
    RIPEMD160(
        hash1.as_mut_ptr() as *const libc::c_uchar,
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        &mut *binres.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    vg_b58_encode_check(
        binres.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 21]>() as libc::c_ulong,
        result,
    );
}
pub unsafe extern "C" fn vg_encode_script_address(
    mut ppoint: *const EC_POINT,
    mut pgroup: *const EC_GROUP,
    mut addrtype: libc::c_int,
    mut result: *mut libc::c_char,
) {
    let mut script_buf: [libc::c_uchar; 69] = [0; 69];
    let mut eckey_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut binres: [libc::c_uchar; 21] = [0; 21];
    let mut tmp: libc::c_uint = 0;
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    eckey_buf = script_buf.as_mut_ptr().offset(2 as libc::c_int as isize);
    binres[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 21 as libc::c_uint) {
        binres[tmp as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp = tmp.wrapping_add(1);
    }
    script_buf[0 as libc::c_int as usize] = 81 as libc::c_int as libc::c_uchar;
    script_buf[1 as libc::c_int as usize] = 65 as libc::c_int as libc::c_uchar;
    script_buf[67 as libc::c_int as usize] = 81 as libc::c_int as libc::c_uchar;
    script_buf[68 as libc::c_int as usize] = 174 as libc::c_int as libc::c_uchar;
    EC_POINT_point2oct(
        pgroup,
        ppoint,
        POINT_CONVERSION_UNCOMPRESSED,
        eckey_buf,
        65 as libc::c_int as size_t,
        0 as *mut libc::c_void as *mut BN_CTX,
    );
    binres[0 as libc::c_int as usize] = addrtype as libc::c_uchar;
    SHA256(
        script_buf.as_mut_ptr() as *const libc::c_uchar,
        69 as libc::c_int as size_t,
        hash1.as_mut_ptr(),
    );
    RIPEMD160(
        hash1.as_mut_ptr() as *const libc::c_uchar,
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        &mut *binres.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    vg_b58_encode_check(
        binres.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 21]>() as libc::c_ulong,
        result,
    );
}
pub unsafe extern "C" fn vg_encode_privkey(
    mut pkey: *const EC_KEY,
    mut addrtype: libc::c_int,
    mut result: *mut libc::c_char,
) {
    let mut eckey_buf: [libc::c_uchar; 128] = [0; 128];
    let mut bn: *const BIGNUM = 0 as *const BIGNUM;
    let mut nbytes: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    bn = EC_KEY_get0_private_key(pkey);
    eckey_buf[0 as libc::c_int as usize] = addrtype as libc::c_uchar;
    tmp = BN_num_bits(bn);
    nbytes = (tmp + 7 as libc::c_int) / 8 as libc::c_int;
    if !(nbytes <= 32 as libc::c_int) {
        __assert_fail(
            b"nbytes <= 32\0" as *const u8 as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_uint,
            b"vg_encode_privkey\0" as *const u8 as *const libc::c_char,
        );
    }
    if nbytes < 32 as libc::c_int {
        memset(
            eckey_buf.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_int - nbytes) as size_t,
        );
    }
    BN_bn2bin(
        bn,
        &mut *eckey_buf.as_mut_ptr().offset((33 as libc::c_int - nbytes) as isize),
    );
    vg_b58_encode_check(
        eckey_buf.as_mut_ptr() as *mut libc::c_void,
        33 as libc::c_int as size_t,
        result,
    );
}
pub unsafe extern "C" fn vg_set_privkey(
    mut bnpriv: *const BIGNUM,
    mut pkey: *mut EC_KEY,
) -> libc::c_int {
    let mut pgroup: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut ppnt: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    pgroup = EC_KEY_get0_group(pkey as *const EC_KEY);
    ppnt = EC_POINT_new(pgroup);
    if !ppnt.is_null() {
        tmp = EC_KEY_set_private_key(pkey, bnpriv);
        if tmp != 0 {
            tmp___0 = EC_POINT_mul(
                pgroup,
                ppnt,
                bnpriv,
                0 as *mut libc::c_void as *const EC_POINT,
                0 as *mut libc::c_void as *const BIGNUM,
                0 as *mut libc::c_void as *mut BN_CTX,
            );
            if tmp___0 != 0 {
                tmp___1 = EC_KEY_set_public_key(pkey, ppnt as *const EC_POINT);
                if tmp___1 != 0 {
                    tmp___2 = 1 as libc::c_int;
                } else {
                    tmp___2 = 0 as libc::c_int;
                }
            } else {
                tmp___2 = 0 as libc::c_int;
            }
        } else {
            tmp___2 = 0 as libc::c_int;
        }
    } else {
        tmp___2 = 0 as libc::c_int;
    }
    res = tmp___2;
    if !ppnt.is_null() {
        EC_POINT_free(ppnt);
    }
    if res == 0 {
        return 0 as libc::c_int;
    }
    tmp___6 = EC_KEY_check_key(pkey as *const EC_KEY);
    if tmp___6 == 0 {
        __assert_fail(
            b"EC_KEY_check_key(pkey)\0" as *const u8 as *const libc::c_char,
            b"util.c\0" as *const u8 as *const libc::c_char,
            330 as libc::c_uint,
            b"vg_set_privkey\0" as *const u8 as *const libc::c_char,
        );
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_decode_privkey(
    mut b58encoded: *const libc::c_char,
    mut pkey: *mut EC_KEY,
    mut addrtype: *mut libc::c_int,
) -> libc::c_int {
    let mut bnpriv: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut ecpriv: [libc::c_uchar; 48] = [0; 48];
    let mut res: libc::c_int = 0;
    res = vg_b58_decode_check(
        b58encoded,
        ecpriv.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 48]>() as libc::c_ulong,
    );
    if res != 33 as libc::c_int {
        return 0 as libc::c_int;
    }
    BN_init(&mut bnpriv);
    BN_bin2bn(
        ecpriv.as_mut_ptr().offset(1 as libc::c_int as isize) as *const libc::c_uchar,
        res - 1 as libc::c_int,
        &mut bnpriv,
    );
    res = vg_set_privkey(&mut bnpriv as *mut BIGNUM as *const BIGNUM, pkey);
    BN_clear_free(&mut bnpriv);
    *addrtype = ecpriv[0 as libc::c_int as usize] as libc::c_int;
    return 1 as libc::c_int;
}
static mut protkey_parameters: [vg_protkey_parameters_t; 17] = unsafe {
    [
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 4096 as libc::c_int,
                pbkdf_hash_getter: Some(
                    EVP_sha256 as unsafe extern "C" fn() -> *const EVP_MD,
                ),
                cipher_getter: Some(
                    EVP_aes_256_cbc as unsafe extern "C" fn() -> *const EVP_CIPHER,
                ),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 0 as libc::c_int,
                iterations: 0 as libc::c_int,
                pbkdf_hash_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_MD>,
                >(0 as *const libc::c_void as *mut libc::c_void),
                cipher_getter: ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<unsafe extern "C" fn() -> *const EVP_CIPHER>,
                >(0 as *const libc::c_void as *mut libc::c_void),
            };
            init
        },
        {
            let mut init = __anonstruct_vg_protkey_parameters_t_1070732568 {
                mode: 1 as libc::c_int,
                iterations: 4096 as libc::c_int,
                pbkdf_hash_getter: Some(
                    EVP_sha256 as unsafe extern "C" fn() -> *const EVP_MD,
                ),
                cipher_getter: Some(
                    EVP_aes_256_cbc as unsafe extern "C" fn() -> *const EVP_CIPHER,
                ),
            };
            init
        },
    ]
};
unsafe extern "C" fn vg_protect_crypt(
    mut parameter_group: libc::c_int,
    mut data_in: *mut libc::c_uchar,
    mut data_in_len: libc::c_int,
    mut data_out: *mut libc::c_uchar,
    mut pass: *const libc::c_char,
    mut enc: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ctx: *mut EVP_CIPHER_CTX = 0 as *mut EVP_CIPHER_CTX;
    let mut salt: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut keymaterial: [libc::c_uchar; 144] = [0; 144];
    let mut hmac: [libc::c_uchar; 64] = [0; 64];
    let mut hmac_len: libc::c_int = 0;
    let mut hmac_keylen: libc::c_int = 0;
    let mut salt_len: libc::c_int = 0;
    let mut plaintext_len: libc::c_int = 0;
    let mut ciphertext_len: libc::c_int = 0;
    let mut pkcs7_padding: libc::c_int = 0;
    let mut params: *const vg_protkey_parameters_t = 0 as *const vg_protkey_parameters_t;
    let mut cipher: *const EVP_CIPHER = 0 as *const EVP_CIPHER;
    let mut pbkdf_digest: *const EVP_MD = 0 as *const EVP_MD;
    let mut hmac_digest: *const EVP_MD = 0 as *const EVP_MD;
    let mut hlen: libc::c_uint = 0;
    let mut opos: libc::c_int = 0;
    let mut olen: libc::c_int = 0;
    let mut oincr: libc::c_int = 0;
    let mut nbytes: libc::c_int = 0;
    let mut ipos: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___4: libc::c_int = 0;
    ctx = 0 as *mut libc::c_void as *mut EVP_CIPHER_CTX;
    hmac_len = 0 as libc::c_int;
    hmac_keylen = 0 as libc::c_int;
    plaintext_len = 32 as libc::c_int;
    pkcs7_padding = 1 as libc::c_int;
    ret = 0 as libc::c_int;
    ctx = EVP_CIPHER_CTX_new();
    if !ctx.is_null() {
        if parameter_group < 0 as libc::c_int {
            if enc != 0 {
                parameter_group = 0 as libc::c_int;
            } else {
                parameter_group = *data_in.offset(0 as libc::c_int as isize)
                    as libc::c_int;
            }
            current_block = 15768484401365413375;
        } else if enc == 0 {
            if parameter_group
                != *data_in.offset(0 as libc::c_int as isize) as libc::c_int
            {
                current_block = 15934876590625249380;
            } else {
                current_block = 15768484401365413375;
            }
        } else {
            current_block = 15768484401365413375;
        }
        match current_block {
            15934876590625249380 => {}
            _ => {
                if !(parameter_group as libc::c_ulong
                    > (::std::mem::size_of::<[vg_protkey_parameters_t; 17]>()
                        as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<vg_protkey_parameters_t>()
                                as libc::c_ulong,
                        ))
                {
                    params = &*protkey_parameters
                        .as_ptr()
                        .offset(parameter_group as isize)
                        as *const vg_protkey_parameters_t;
                    if !((*params).iterations == 0) {
                        if !((*params).pbkdf_hash_getter).is_none() {
                            pbkdf_digest = (Some(
                                ((*params).pbkdf_hash_getter)
                                    .expect("non-null function pointer"),
                            ))
                                .expect("non-null function pointer")();
                            cipher = (Some(
                                ((*params).cipher_getter)
                                    .expect("non-null function pointer"),
                            ))
                                .expect("non-null function pointer")();
                            if (*params).mode == 0 as libc::c_int {
                                salt_len = 4 as libc::c_int;
                                hmac_len = 8 as libc::c_int;
                                hmac_keylen = 16 as libc::c_int;
                                ciphertext_len = (plaintext_len + (*cipher).block_size
                                    - 1 as libc::c_int) / (*cipher).block_size
                                    * (*cipher).block_size;
                                pkcs7_padding = 0 as libc::c_int;
                                hmac_digest = EVP_sha256();
                            } else {
                                salt_len = 8 as libc::c_int;
                                ciphertext_len = (plaintext_len + (*cipher).block_size)
                                    / (*cipher).block_size * (*cipher).block_size;
                                hmac_digest = 0 as *mut libc::c_void as *const EVP_MD;
                            }
                            if enc == 0 {
                                if data_in_len
                                    != 1 as libc::c_int + ciphertext_len + hmac_len + salt_len
                                {
                                    current_block = 15934876590625249380;
                                } else {
                                    current_block = 5330834795799507926;
                                }
                            } else {
                                current_block = 5330834795799507926;
                            }
                            match current_block {
                                15934876590625249380 => {}
                                _ => {
                                    if pass.is_null() {
                                        ret = plaintext_len;
                                    } else if data_out.is_null() {
                                        ret = plaintext_len;
                                    } else {
                                        if enc == 0 {
                                            salt = data_in
                                                .offset(1 as libc::c_int as isize)
                                                .offset(ciphertext_len as isize)
                                                .offset(hmac_len as isize);
                                        } else if salt_len != 0 {
                                            salt = data_out
                                                .offset(1 as libc::c_int as isize)
                                                .offset(ciphertext_len as isize)
                                                .offset(hmac_len as isize);
                                            RAND_bytes(salt, salt_len);
                                        } else {
                                            salt = 0 as *mut libc::c_void as *mut libc::c_uchar;
                                        }
                                        tmp = strlen(pass);
                                        PKCS5_PBKDF2_HMAC(
                                            pass,
                                            tmp.wrapping_add(1 as libc::c_ulong) as libc::c_int,
                                            salt as *const libc::c_uchar,
                                            salt_len,
                                            (*params).iterations,
                                            pbkdf_digest,
                                            (*cipher).key_len + (*cipher).iv_len + hmac_keylen,
                                            keymaterial.as_mut_ptr(),
                                        );
                                        tmp___0 = EVP_CipherInit(
                                            ctx,
                                            cipher,
                                            keymaterial.as_mut_ptr() as *const libc::c_uchar,
                                            keymaterial.as_mut_ptr().offset((*cipher).key_len as isize)
                                                as *const libc::c_uchar,
                                            enc,
                                        );
                                        if tmp___0 == 0 {
                                            fprintf(
                                                stderr,
                                                b"ERROR: could not configure cipher\n\0" as *const u8
                                                    as *const libc::c_char,
                                            );
                                        } else {
                                            if pkcs7_padding == 0 {
                                                EVP_CIPHER_CTX_set_padding(ctx, 0 as libc::c_int);
                                            }
                                            if enc == 0 {
                                                opos = 0 as libc::c_int;
                                                olen = plaintext_len;
                                                nbytes = ciphertext_len;
                                                ipos = 1 as libc::c_int;
                                            } else {
                                                *data_out
                                                    .offset(
                                                        0 as libc::c_int as isize,
                                                    ) = parameter_group as libc::c_uchar;
                                                opos = 1 as libc::c_int;
                                                olen = 1 as libc::c_int + ciphertext_len + hmac_len
                                                    + salt_len - opos;
                                                nbytes = plaintext_len;
                                                ipos = 0 as libc::c_int;
                                            }
                                            oincr = olen;
                                            tmp___1 = EVP_CipherUpdate(
                                                ctx,
                                                data_out.offset(opos as isize),
                                                &mut oincr,
                                                data_in.offset(ipos as isize) as *const libc::c_uchar,
                                                nbytes,
                                            );
                                            if tmp___1 == 0 {
                                                current_block = 10522923313019172727;
                                            } else {
                                                opos += oincr;
                                                olen -= oincr;
                                                oincr = olen;
                                                tmp___2 = EVP_CipherFinal(
                                                    ctx,
                                                    data_out.offset(opos as isize),
                                                    &mut oincr,
                                                );
                                                if tmp___2 == 0 {
                                                    current_block = 10522923313019172727;
                                                } else {
                                                    opos += oincr;
                                                    if hmac_len != 0 {
                                                        hlen = ::std::mem::size_of::<[libc::c_uchar; 64]>()
                                                            as libc::c_ulong as libc::c_uint;
                                                        if enc != 0 {
                                                            tmp___3 = data_in;
                                                        } else {
                                                            tmp___3 = data_out;
                                                        }
                                                        HMAC(
                                                            hmac_digest,
                                                            keymaterial
                                                                .as_mut_ptr()
                                                                .offset((*cipher).key_len as isize)
                                                                .offset((*cipher).iv_len as isize) as *const libc::c_void,
                                                            hmac_keylen,
                                                            tmp___3 as *const libc::c_uchar,
                                                            plaintext_len as size_t,
                                                            hmac.as_mut_ptr(),
                                                            &mut hlen,
                                                        );
                                                        if enc != 0 {
                                                            memcpy(
                                                                data_out
                                                                    .offset(1 as libc::c_int as isize)
                                                                    .offset(ciphertext_len as isize) as *mut libc::c_void,
                                                                hmac.as_mut_ptr() as *const libc::c_void,
                                                                hmac_len as size_t,
                                                            );
                                                            current_block = 9199578309995299736;
                                                        } else {
                                                            tmp___4 = memcmp(
                                                                hmac.as_mut_ptr() as *const libc::c_void,
                                                                data_in
                                                                    .offset(1 as libc::c_int as isize)
                                                                    .offset(ciphertext_len as isize) as *const libc::c_void,
                                                                hmac_len as size_t,
                                                            );
                                                            if tmp___4 != 0 {
                                                                current_block = 10522923313019172727;
                                                            } else {
                                                                current_block = 9199578309995299736;
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 9199578309995299736;
                                                    }
                                                    match current_block {
                                                        10522923313019172727 => {}
                                                        _ => {
                                                            if enc != 0 {
                                                                if opos != 1 as libc::c_int + ciphertext_len {
                                                                    fprintf(
                                                                        stderr,
                                                                        b"ERROR: plaintext size mismatch\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                    current_block = 15934876590625249380;
                                                                } else {
                                                                    opos += hmac_len + salt_len;
                                                                    current_block = 17769492591016358583;
                                                                }
                                                            } else if opos != plaintext_len {
                                                                fprintf(
                                                                    stderr,
                                                                    b"ERROR: plaintext size mismatch\n\0" as *const u8
                                                                        as *const libc::c_char,
                                                                );
                                                                current_block = 15934876590625249380;
                                                            } else {
                                                                current_block = 17769492591016358583;
                                                            }
                                                            match current_block {
                                                                15934876590625249380 => {}
                                                                _ => {
                                                                    ret = opos;
                                                                    current_block = 15934876590625249380;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            match current_block {
                                                15934876590625249380 => {}
                                                _ => {
                                                    fprintf(
                                                        stderr,
                                                        b"ERROR: Invalid password\n\0" as *const u8
                                                            as *const libc::c_char,
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
    }
    OPENSSL_cleanse(
        hmac.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
    );
    OPENSSL_cleanse(
        keymaterial.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 144]>() as libc::c_ulong,
    );
    if !ctx.is_null() {
        EVP_CIPHER_CTX_free(ctx);
    }
    return ret;
}
pub unsafe extern "C" fn vg_protect_encode_privkey(
    mut out: *mut libc::c_char,
    mut pkey: *const EC_KEY,
    mut keytype: libc::c_int,
    mut parameter_group: libc::c_int,
    mut pass: *const libc::c_char,
) -> libc::c_int {
    let mut ecpriv: [libc::c_uchar; 64] = [0; 64];
    let mut ecenc: [libc::c_uchar; 128] = [0; 128];
    let mut privkey: *const BIGNUM = 0 as *const BIGNUM;
    let mut nbytes: libc::c_int = 0;
    let mut restype: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    if keytype & 1 as libc::c_int != 0 {
        restype = 79 as libc::c_int;
    } else {
        restype = 32 as libc::c_int;
    }
    privkey = EC_KEY_get0_private_key(pkey);
    tmp = BN_num_bits(privkey);
    nbytes = (tmp + 7 as libc::c_int) / 8 as libc::c_int;
    if nbytes < 32 as libc::c_int {
        memset(
            ecpriv.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_int - nbytes) as size_t,
        );
    }
    BN_bn2bin(
        privkey,
        ecpriv.as_mut_ptr().offset(32 as libc::c_int as isize).offset(-(nbytes as isize)),
    );
    nbytes = vg_protect_crypt(
        parameter_group,
        ecpriv.as_mut_ptr(),
        32 as libc::c_int,
        &mut *ecenc.as_mut_ptr().offset(1 as libc::c_int as isize),
        pass,
        1 as libc::c_int,
    );
    if nbytes <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    OPENSSL_cleanse(
        ecpriv.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
    );
    ecenc[0 as libc::c_int as usize] = restype as libc::c_uchar;
    vg_b58_encode_check(
        ecenc.as_mut_ptr() as *mut libc::c_void,
        (nbytes + 1 as libc::c_int) as size_t,
        out,
    );
    tmp___0 = strlen(out as *const libc::c_char);
    nbytes = tmp___0 as libc::c_int;
    return nbytes;
}
pub unsafe extern "C" fn vg_protect_decode_privkey(
    mut pkey: *mut EC_KEY,
    mut keytype: *mut libc::c_int,
    mut encoded: *const libc::c_char,
    mut pass: *const libc::c_char,
) -> libc::c_int {
    let mut ecpriv: [libc::c_uchar; 64] = [0; 64];
    let mut ecenc: [libc::c_uchar; 128] = [0; 128];
    let mut bn: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut restype: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___0: libc::c_int = 0;
    res = vg_b58_decode_check(
        encoded,
        ecenc.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong,
    );
    if res < 2 as libc::c_int {
        return 0 as libc::c_int
    } else {
        if res as libc::c_ulong
            > ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong
        {
            return 0 as libc::c_int;
        }
    }
    match ecenc[0 as libc::c_int as usize] as libc::c_int {
        32 => {
            restype = 128 as libc::c_int;
        }
        79 => {
            restype = 239 as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    }
    if !pkey.is_null() {
        tmp = ecpriv.as_mut_ptr();
    } else {
        tmp = 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    tmp___0 = vg_protect_crypt(
        -(1 as libc::c_int),
        ecenc.as_mut_ptr().offset(1 as libc::c_int as isize),
        res - 1 as libc::c_int,
        tmp,
        pass,
        0 as libc::c_int,
    );
    if tmp___0 == 0 {
        return 0 as libc::c_int;
    }
    res = 1 as libc::c_int;
    if !pkey.is_null() {
        BN_init(&mut bn);
        BN_bin2bn(
            ecpriv.as_mut_ptr() as *const libc::c_uchar,
            32 as libc::c_int,
            &mut bn,
        );
        res = vg_set_privkey(&mut bn as *mut BIGNUM as *const BIGNUM, pkey);
        BN_clear_free(&mut bn);
        OPENSSL_cleanse(
            ecpriv.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
        );
    }
    *keytype = restype;
    return res;
}
pub unsafe extern "C" fn vg_pkcs8_encode_privkey(
    mut out: *mut libc::c_char,
    mut outlen: libc::c_int,
    mut pkey: *const EC_KEY,
    mut pass: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut pkey_copy: *mut EC_KEY = 0 as *mut EC_KEY;
    let mut evp_key: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut pkcs8: *mut PKCS8_PRIV_KEY_INFO = 0 as *mut PKCS8_PRIV_KEY_INFO;
    let mut pkcs8_enc: *mut X509_SIG = 0 as *mut X509_SIG;
    let mut memptr: *mut BUF_MEM = 0 as *mut BUF_MEM;
    let mut bio: *mut BIO = 0 as *mut BIO;
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut BIO_METHOD = 0 as *mut BIO_METHOD;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *const EVP_CIPHER = 0 as *const EVP_CIPHER;
    pkey_copy = 0 as *mut libc::c_void as *mut EC_KEY;
    evp_key = 0 as *mut libc::c_void as *mut EVP_PKEY;
    pkcs8 = 0 as *mut libc::c_void as *mut PKCS8_PRIV_KEY_INFO;
    pkcs8_enc = 0 as *mut libc::c_void as *mut X509_SIG;
    bio = 0 as *mut libc::c_void as *mut BIO;
    res = 0 as libc::c_int;
    pkey_copy = EC_KEY_dup(pkey);
    if !pkey_copy.is_null() {
        evp_key = EVP_PKEY_new();
        if !evp_key.is_null() {
            tmp = EVP_PKEY_set1_EC_KEY(evp_key, pkey_copy);
            if !(tmp == 0) {
                pkcs8 = EVP_PKEY2PKCS8(evp_key);
                if !pkcs8.is_null() {
                    tmp___0 = BIO_s_mem();
                    bio = BIO_new(tmp___0);
                    if !bio.is_null() {
                        if pass.is_null() {
                            res = PEM_write_bio_PKCS8_PRIV_KEY_INFO(bio, pkcs8);
                            current_block = 2232869372362427478;
                        } else {
                            tmp___1 = strlen(pass);
                            tmp___2 = EVP_aes_256_cbc();
                            pkcs8_enc = PKCS8_encrypt(
                                -(1 as libc::c_int),
                                tmp___2,
                                pass,
                                tmp___1 as libc::c_int,
                                0 as *mut libc::c_void as *mut libc::c_uchar,
                                0 as libc::c_int,
                                4096 as libc::c_int,
                                pkcs8,
                            );
                            if pkcs8_enc.is_null() {
                                current_block = 5970610568327127476;
                            } else {
                                res = PEM_write_bio_PKCS8(bio, pkcs8_enc);
                                current_block = 2232869372362427478;
                            }
                        }
                        match current_block {
                            5970610568327127476 => {}
                            _ => {
                                BIO_ctrl(
                                    bio,
                                    115 as libc::c_int,
                                    0 as libc::c_long,
                                    &mut memptr as *mut *mut BUF_MEM as *mut libc::c_char
                                        as *mut libc::c_void,
                                );
                                res = (*memptr).length as libc::c_int;
                                if res < outlen {
                                    memcpy(
                                        out as *mut libc::c_void,
                                        (*memptr).data as *const libc::c_void,
                                        res as size_t,
                                    );
                                    *out.offset(res as isize) = '\u{0}' as i32 as libc::c_char;
                                } else {
                                    memcpy(
                                        out as *mut libc::c_void,
                                        (*memptr).data as *const libc::c_void,
                                        (outlen - 1 as libc::c_int) as size_t,
                                    );
                                    *out
                                        .offset(
                                            (outlen - 1 as libc::c_int) as isize,
                                        ) = '\u{0}' as i32 as libc::c_char;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !bio.is_null() {
        BIO_free(bio);
    }
    if !pkey_copy.is_null() {
        EC_KEY_free(pkey_copy);
    }
    if !evp_key.is_null() {
        EVP_PKEY_free(evp_key);
    }
    if !pkcs8.is_null() {
        PKCS8_PRIV_KEY_INFO_free(pkcs8);
    }
    if !pkcs8_enc.is_null() {
        X509_SIG_free(pkcs8_enc);
    }
    return res;
}
pub unsafe extern "C" fn vg_pkcs8_decode_privkey(
    mut pkey: *mut EC_KEY,
    mut pem_in: *const libc::c_char,
    mut pass: *const libc::c_char,
) -> libc::c_int {
    let mut pkey_in: *mut EC_KEY = 0 as *mut EC_KEY;
    let mut test_key: *mut EC_KEY = 0 as *mut EC_KEY;
    let mut evp_key: *mut EVP_PKEY = 0 as *mut EVP_PKEY;
    let mut pkcs8: *mut PKCS8_PRIV_KEY_INFO = 0 as *mut PKCS8_PRIV_KEY_INFO;
    let mut pkcs8_enc: *mut X509_SIG = 0 as *mut X509_SIG;
    let mut bio: *mut BIO = 0 as *mut BIO;
    let mut res: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___2: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut EC_KEY = 0 as *mut EC_KEY;
    pkey_in = 0 as *mut libc::c_void as *mut EC_KEY;
    test_key = 0 as *mut libc::c_void as *mut EC_KEY;
    evp_key = 0 as *mut libc::c_void as *mut EVP_PKEY;
    pkcs8 = 0 as *mut libc::c_void as *mut PKCS8_PRIV_KEY_INFO;
    pkcs8_enc = 0 as *mut libc::c_void as *mut X509_SIG;
    bio = 0 as *mut libc::c_void as *mut BIO;
    res = 0 as libc::c_int;
    tmp = strlen(pem_in);
    bio = BIO_new_mem_buf(
        pem_in as *mut libc::c_char as *mut libc::c_void,
        tmp as libc::c_int,
    );
    if !bio.is_null() {
        pkcs8_enc = PEM_read_bio_PKCS8(
            bio,
            0 as *mut libc::c_void as *mut *mut X509_SIG,
            ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<pem_password_cb>,
            >(0 as *mut libc::c_void),
            0 as *mut libc::c_void,
        );
        if !pkcs8_enc.is_null() {
            if pass.is_null() {
                return -(1 as libc::c_int);
            }
            tmp___0 = strlen(pass);
            pkcs8 = PKCS8_decrypt(pkcs8_enc, pass, tmp___0 as libc::c_int);
        } else {
            BIO_ctrl(bio, 1 as libc::c_int, 0 as libc::c_long, 0 as *mut libc::c_void);
            pkcs8 = PEM_read_bio_PKCS8_PRIV_KEY_INFO(
                bio,
                0 as *mut libc::c_void as *mut *mut PKCS8_PRIV_KEY_INFO,
                ::std::mem::transmute::<
                    *mut libc::c_void,
                    Option::<pem_password_cb>,
                >(0 as *mut libc::c_void),
                0 as *mut libc::c_void,
            );
        }
        if !pkcs8.is_null() {
            evp_key = EVP_PKCS82PKEY(pkcs8);
            if !evp_key.is_null() {
                pkey_in = EVP_PKEY_get1_EC_KEY(evp_key);
                if !pkey_in.is_null() {
                    test_key = EC_KEY_new_by_curve_name(714 as libc::c_int);
                    if !test_key.is_null() {
                        tmp___1 = EC_KEY_get0_group(test_key as *const EC_KEY);
                        tmp___2 = EC_KEY_get0_group(pkey_in as *const EC_KEY);
                        tmp___3 = EC_GROUP_cmp(
                            tmp___2,
                            tmp___1,
                            0 as *mut libc::c_void as *mut BN_CTX,
                        );
                        if !(tmp___3 != 0) {
                            tmp___4 = EC_KEY_copy(pkey, pkey_in as *const EC_KEY);
                            if !tmp___4.is_null() {
                                res = 1 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    if !bio.is_null() {
        BIO_free(bio);
    }
    if !test_key.is_null() {
        EC_KEY_free(pkey_in);
    }
    if !evp_key.is_null() {
        EVP_PKEY_free(evp_key);
    }
    if !pkcs8.is_null() {
        PKCS8_PRIV_KEY_INFO_free(pkcs8);
    }
    if !pkcs8_enc.is_null() {
        X509_SIG_free(pkcs8_enc);
    }
    return res;
}
pub unsafe extern "C" fn vg_decode_privkey_any(
    mut pkey: *mut EC_KEY,
    mut addrtype: *mut libc::c_int,
    mut input: *const libc::c_char,
    mut pass: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = vg_decode_privkey(input, pkey, addrtype);
    if tmp != 0 {
        return 1 as libc::c_int;
    }
    tmp___1 = vg_protect_decode_privkey(
        pkey,
        addrtype,
        input,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    if tmp___1 != 0 {
        if pass.is_null() {
            return -(1 as libc::c_int);
        }
        tmp___0 = vg_protect_decode_privkey(pkey, addrtype, input, pass);
        return tmp___0;
    }
    res = vg_pkcs8_decode_privkey(pkey, input, pass);
    if res > 0 as libc::c_int {
        *addrtype = 128 as libc::c_int;
    }
    return res;
}
pub unsafe extern "C" fn vg_read_password(
    mut buf: *mut libc::c_char,
    mut size: size_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = EVP_read_pw_string(
        buf,
        size as libc::c_int,
        b"Enter new password:\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    if tmp != 0 {
        tmp___0 = 0 as libc::c_int;
    } else {
        tmp___0 = 1 as libc::c_int;
    }
    return tmp___0;
}
static mut ascii_class: [libc::c_uchar; 128] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn vg_check_password_complexity(
    mut pass: *const libc::c_char,
    mut verbose: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut classes: [libc::c_int; 6] = [0; 6];
    let mut tmp: libc::c_uint = 0;
    let mut crackunit: *const libc::c_char = 0 as *const libc::c_char;
    let mut char_complexity: libc::c_int = 0;
    let mut crackops: libc::c_double = 0.;
    let mut cracktime: libc::c_double = 0.;
    let mut weak: libc::c_int = 0;
    let mut rate: libc::c_int = 0;
    let mut weak_threshold: libc::c_int = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *const libc::c_char = 0 as *const libc::c_char;
    classes[0 as libc::c_int as usize] = 0 as libc::c_int;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 6 as libc::c_uint) {
        classes[tmp as usize] = 0 as libc::c_int;
        tmp = tmp.wrapping_add(1);
    }
    crackunit = b"seconds\0" as *const u8 as *const libc::c_char;
    char_complexity = 0 as libc::c_int;
    rate = 250000000 as libc::c_int;
    weak_threshold = 31536000 as libc::c_int;
    tmp___0 = strlen(pass);
    len = tmp___0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        if *pass.offset(i as isize) as libc::c_ulong
            > ::std::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong
        {
            classes[5 as libc::c_int as usize] += 1;
        } else if ascii_class[*pass.offset(i as isize) as libc::c_int as usize] != 0 {
            classes[(ascii_class[*pass.offset(i as isize) as libc::c_int as usize]
                as libc::c_int - 1 as libc::c_int) as usize] += 1;
        }
        i += 1;
    }
    if classes[0 as libc::c_int as usize] != 0 {
        char_complexity += 26 as libc::c_int;
    }
    if classes[1 as libc::c_int as usize] != 0 {
        char_complexity += 26 as libc::c_int;
    }
    if classes[2 as libc::c_int as usize] != 0 {
        char_complexity += 10 as libc::c_int;
    }
    if classes[3 as libc::c_int as usize] != 0 {
        char_complexity += 14 as libc::c_int;
    }
    if classes[4 as libc::c_int as usize] != 0 {
        char_complexity += 19 as libc::c_int;
    }
    if classes[5 as libc::c_int as usize] != 0 {
        char_complexity += 32 as libc::c_int;
    }
    crackops = pow(char_complexity as libc::c_double, len as libc::c_double);
    cracktime = crackops
        * (1 as libc::c_int as libc::c_double
            - 1 as libc::c_int as libc::c_double / 2.7182818284590452354f64)
        / rate as libc::c_double;
    weak = (cracktime < weak_threshold as libc::c_double) as libc::c_int;
    if cracktime > 60.0f64 {
        cracktime /= 60.0f64;
        crackunit = b"minutes\0" as *const u8 as *const libc::c_char;
        if cracktime > 60.0f64 {
            cracktime /= 60.0f64;
            crackunit = b"hours\0" as *const u8 as *const libc::c_char;
            if cracktime > 24.0f64 {
                cracktime /= 24 as libc::c_int as libc::c_double;
                crackunit = b"days\0" as *const u8 as *const libc::c_char;
                if cracktime > 365.0f64 {
                    cracktime /= 365.0f64;
                    crackunit = b"years\0" as *const u8 as *const libc::c_char;
                }
            }
        }
    }
    if weak != 0 {
        if verbose > 0 as libc::c_int {
            current_block = 4617120963215599469;
        } else {
            current_block = 3247444318569577738;
        }
    } else {
        current_block = 3247444318569577738;
    }
    match current_block {
        3247444318569577738 => {
            if verbose > 1 as libc::c_int {
                current_block = 4617120963215599469;
            } else {
                current_block = 11071260907632769126;
            }
        }
        _ => {}
    }
    match current_block {
        4617120963215599469 => {
            if cracktime < 1.0f64 {
                fprintf(
                    stderr,
                    b"Estimated password crack time: >1 %s\n\0" as *const u8
                        as *const libc::c_char,
                    crackunit,
                );
            } else if cracktime < 1000000 as libc::c_int as libc::c_double {
                fprintf(
                    stderr,
                    b"Estimated password crack time: %.1f %s\n\0" as *const u8
                        as *const libc::c_char,
                    cracktime,
                    crackunit,
                );
            } else {
                fprintf(
                    stderr,
                    b"Estimated password crack time: %e %s\n\0" as *const u8
                        as *const libc::c_char,
                    cracktime,
                    crackunit,
                );
            }
            if classes[0 as libc::c_int as usize] == 0 {
                if classes[1 as libc::c_int as usize] == 0 {
                    if classes[2 as libc::c_int as usize] != 0 {
                        if classes[3 as libc::c_int as usize] == 0 {
                            if classes[4 as libc::c_int as usize] == 0 {
                                if classes[5 as libc::c_int as usize] == 0 {
                                    fprintf(
                                        stderr,
                                        b"WARNING: Password contains only numbers\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    current_block = 11071260907632769126;
                                } else {
                                    current_block = 1289005050726867966;
                                }
                            } else {
                                current_block = 1289005050726867966;
                            }
                        } else {
                            current_block = 1289005050726867966;
                        }
                    } else {
                        current_block = 1289005050726867966;
                    }
                } else {
                    current_block = 1289005050726867966;
                }
            } else {
                current_block = 1289005050726867966;
            }
            match current_block {
                11071260907632769126 => {}
                _ => {
                    if classes[2 as libc::c_int as usize] == 0 {
                        if classes[3 as libc::c_int as usize] == 0 {
                            if classes[4 as libc::c_int as usize] == 0 {
                                if classes[5 as libc::c_int as usize] == 0 {
                                    let mut current_block_76: u64;
                                    if classes[0 as libc::c_int as usize] == 0 {
                                        current_block_76 = 14807673733814441108;
                                    } else if classes[1 as libc::c_int as usize] == 0 {
                                        current_block_76 = 14807673733814441108;
                                    } else {
                                        fprintf(
                                            stderr,
                                            b"WARNING: Password contains only letters\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block_76 = 5722677567366458307;
                                    }
                                    match current_block_76 {
                                        14807673733814441108 => {
                                            if classes[0 as libc::c_int as usize] != 0 {
                                                tmp___1 = b"lower\0" as *const u8 as *const libc::c_char;
                                            } else {
                                                tmp___1 = b"upper\0" as *const u8 as *const libc::c_char;
                                            }
                                            fprintf(
                                                stderr,
                                                b"WARNING: Password contains only %scase letters\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                tmp___1,
                                            );
                                        }
                                        _ => {}
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
    return (weak == 0) as libc::c_int;
}
pub unsafe extern "C" fn vg_read_file(
    mut fp: *mut FILE,
    mut result: *mut *mut *mut libc::c_char,
    mut rescount: *mut libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut patterns: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut blksize: libc::c_int = 0;
    let mut nalloc: libc::c_int = 0;
    let mut npatterns: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = 1 as libc::c_int;
    buf = 0 as *mut libc::c_void as *mut libc::c_char;
    blksize = 16384 as libc::c_int;
    nalloc = 16 as libc::c_int;
    npatterns = 0 as libc::c_int;
    tmp = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nalloc as libc::c_ulong),
    );
    patterns = tmp as *mut *mut libc::c_char;
    count = 0 as libc::c_int;
    pos = 0 as libc::c_int;
    loop {
        obuf = buf;
        tmp___0 = malloc(blksize as size_t);
        buf = tmp___0 as *mut libc::c_char;
        if buf.is_null() {
            ret = 0 as libc::c_int;
            break;
        } else {
            if pos < count {
                memcpy(
                    buf as *mut libc::c_void,
                    obuf.offset(pos as isize) as *const libc::c_void,
                    (count - pos) as size_t,
                );
            }
            pos = count - pos;
            tmp___1 = fread(
                buf.offset(pos as isize) as *mut libc::c_void,
                1 as libc::c_int as size_t,
                (blksize - pos) as size_t,
                fp,
            );
            count = tmp___1 as libc::c_int;
            if count < 0 as libc::c_int {
                tmp___2 = __errno_location();
                tmp___3 = strerror(*tmp___2);
                fprintf(
                    stderr,
                    b"Error reading file: %s\n\0" as *const u8 as *const libc::c_char,
                    tmp___3,
                );
                ret = 0 as libc::c_int;
            }
            if count <= 0 as libc::c_int {
                break;
            }
            count += pos;
            pat = buf;
            while pos < count {
                let mut current_block_40: u64;
                if *buf.offset(pos as isize) as libc::c_int == 13 as libc::c_int {
                    current_block_40 = 14727503967324681556;
                } else if *buf.offset(pos as isize) as libc::c_int == 10 as libc::c_int {
                    current_block_40 = 14727503967324681556;
                } else {
                    if pat.is_null() {
                        pat = buf.offset(pos as isize);
                    }
                    current_block_40 = 12199444798915819164;
                }
                match current_block_40 {
                    14727503967324681556 => {
                        *buf.offset(pos as isize) = '\u{0}' as i32 as libc::c_char;
                        if !pat.is_null() {
                            if npatterns == nalloc {
                                nalloc *= 2 as libc::c_int;
                                tmp___4 = realloc(
                                    patterns as *mut libc::c_void,
                                    (::std::mem::size_of::<*mut libc::c_char>()
                                        as libc::c_ulong)
                                        .wrapping_mul(nalloc as libc::c_ulong),
                                );
                                patterns = tmp___4 as *mut *mut libc::c_char;
                            }
                            let ref mut fresh17 = *patterns.offset(npatterns as isize);
                            *fresh17 = pat;
                            npatterns += 1;
                            pat = 0 as *mut libc::c_void as *mut libc::c_char;
                        }
                    }
                    _ => {}
                }
                pos += 1;
            }
            if !pat.is_null() {
                pos = pat.offset_from(buf) as libc::c_long as libc::c_int;
            } else {
                pos = count;
            }
        }
    }
    *result = patterns;
    *rescount = npatterns;
    return ret;
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
    vpk_nwords = (25 as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
