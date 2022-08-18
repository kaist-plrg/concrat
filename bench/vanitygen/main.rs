use ::libc;
use libc::memset;
use libc::strcpy;
use libc::snprintf;
use libc::memcpy;
use libc::memmove;
use std::arch::asm;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type bignum_ctx;
    pub type ec_group_st;
    pub type ec_method_st;
    pub type ec_key_st;
    pub type _cl_mem;
    pub type _cl_event;
    pub type _cl_kernel;
    pub type _cl_program;
    pub type _cl_command_queue;
    pub type _cl_context;
    pub type _cl_device_id;
    pub type evp_pkey_ctx_st;
    pub type engine_st;
    pub type _cl_platform_id;
    pub type real_pcre;
    pub type bn_blinding_st;
    pub type ASN1_VALUE_st;
    pub type evp_pkey_asn1_method_st;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn __fprintf_chk(
        __stream: *mut FILE,
        __flag: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __printf_chk(
        __flag: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __fgets_chk(
        __s: *mut libc::c_char,
        __size: size_t,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __fgets_alias(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __fgets_chk_warn(
        __s: *mut libc::c_char,
        __size: size_t,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __fread_chk(
        __ptr: *mut libc::c_void,
        __ptrlen: size_t,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __fread_alias(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __fread_chk_warn(
        __ptr: *mut libc::c_void,
        __ptrlen: size_t,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok_r(
        __s: *mut libc::c_char,
        __delim: *const libc::c_char,
        __save_ptr: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn BN_CTX_new() -> *mut BN_CTX;
    fn BN_CTX_free(c: *mut BN_CTX);
    fn BN_init(_: *mut BIGNUM);
    fn BN_clear_free(a: *mut BIGNUM);
    fn BN_bin2bn(
        s: *const libc::c_uchar,
        len: libc::c_int,
        ret: *mut BIGNUM,
    ) -> *mut BIGNUM;
    fn BN_sub(r: *mut BIGNUM, a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
    fn BN_set_word(a: *mut BIGNUM, w: libc::c_ulong) -> libc::c_int;
    fn BN_get_word(a: *const BIGNUM) -> libc::c_ulong;
    fn BN_cmp(a: *const BIGNUM, b: *const BIGNUM) -> libc::c_int;
    fn BN_mod_inverse(
        ret: *mut BIGNUM,
        a: *const BIGNUM,
        n: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> *mut BIGNUM;
    fn BN_MONT_CTX_new() -> *mut BN_MONT_CTX;
    fn BN_mod_mul_montgomery(
        r: *mut BIGNUM,
        a: *const BIGNUM,
        b: *const BIGNUM,
        mont: *mut BN_MONT_CTX,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn BN_MONT_CTX_free(mont: *mut BN_MONT_CTX);
    fn BN_MONT_CTX_set(
        mont: *mut BN_MONT_CTX,
        mod_0: *const BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn bn_expand2(a: *mut BIGNUM, words: libc::c_int) -> *mut BIGNUM;
    fn EC_GROUP_get0_generator(group: *const EC_GROUP) -> *const EC_POINT;
    fn EC_GROUP_get_order(
        group: *const EC_GROUP,
        order: *mut BIGNUM,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_POINT_new(group: *const EC_GROUP) -> *mut EC_POINT;
    fn EC_POINT_copy(dst: *mut EC_POINT, src: *const EC_POINT) -> libc::c_int;
    fn EC_POINT_add(
        group: *const EC_GROUP,
        r: *mut EC_POINT,
        a: *const EC_POINT,
        b: *const EC_POINT,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_POINT_make_affine(
        group: *const EC_GROUP,
        point: *mut EC_POINT,
        ctx: *mut BN_CTX,
    ) -> libc::c_int;
    fn EC_POINTs_make_affine(
        group: *const EC_GROUP,
        num: size_t,
        points: *mut *mut EC_POINT,
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
    fn EC_KEY_get0_public_key(key: *const EC_KEY) -> *const EC_POINT;
    fn EC_KEY_generate_key(key: *mut EC_KEY) -> libc::c_int;
    fn EVP_MD_CTX_create() -> *mut EVP_MD_CTX;
    fn EVP_MD_CTX_destroy(ctx: *mut EVP_MD_CTX);
    fn EVP_DigestInit_ex(
        ctx: *mut EVP_MD_CTX,
        type_0: *const EVP_MD,
        impl_0: *mut ENGINE,
    ) -> libc::c_int;
    fn EVP_DigestUpdate(
        ctx: *mut EVP_MD_CTX,
        d: *const libc::c_void,
        cnt: size_t,
    ) -> libc::c_int;
    fn EVP_DigestFinal_ex(
        ctx: *mut EVP_MD_CTX,
        md: *mut libc::c_uchar,
        s: *mut libc::c_uint,
    ) -> libc::c_int;
    fn EVP_md5() -> *const EVP_MD;
    fn clGetPlatformIDs(
        num_entries: cl_uint,
        platforms: *mut cl_platform_id,
        num_platforms: *mut cl_uint,
    ) -> cl_int;
    fn clGetPlatformInfo(
        platform: cl_platform_id,
        param_name: cl_platform_info,
        param_value_size: size_t,
        param_value: *mut libc::c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
    fn clGetDeviceIDs(
        platform: cl_platform_id,
        device_type: cl_device_type,
        num_entries: cl_uint,
        devices: *mut cl_device_id,
        num_devices: *mut cl_uint,
    ) -> cl_int;
    fn clGetDeviceInfo(
        device: cl_device_id,
        param_name: cl_device_info,
        param_value_size: size_t,
        param_value: *mut libc::c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
    fn clCreateContext(
        properties: *const cl_context_properties,
        num_devices: cl_uint,
        devices: *const cl_device_id,
        pfn_notify: Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *const libc::c_void,
                size_t,
                *mut libc::c_void,
            ) -> (),
        >,
        user_data: *mut libc::c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_context;
    fn clReleaseContext(context: cl_context) -> cl_int;
    fn clReleaseCommandQueue(command_queue: cl_command_queue) -> cl_int;
    fn clCreateBuffer(
        context: cl_context,
        flags: cl_mem_flags,
        size: size_t,
        host_ptr: *mut libc::c_void,
        errcode_ret: *mut cl_int,
    ) -> cl_mem;
    fn clRetainMemObject(memobj: cl_mem) -> cl_int;
    fn clReleaseMemObject(memobj: cl_mem) -> cl_int;
    fn clCreateProgramWithSource(
        context: cl_context,
        count: cl_uint,
        strings: *mut *const libc::c_char,
        lengths: *const size_t,
        errcode_ret: *mut cl_int,
    ) -> cl_program;
    fn clCreateProgramWithBinary(
        context: cl_context,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        lengths: *const size_t,
        binaries: *mut *const libc::c_uchar,
        binary_status: *mut cl_int,
        errcode_ret: *mut cl_int,
    ) -> cl_program;
    fn clReleaseProgram(program: cl_program) -> cl_int;
    fn clBuildProgram(
        program: cl_program,
        num_devices: cl_uint,
        device_list: *const cl_device_id,
        options: *const libc::c_char,
        pfn_notify: Option::<unsafe extern "C" fn(cl_program, *mut libc::c_void) -> ()>,
        user_data: *mut libc::c_void,
    ) -> cl_int;
    fn clGetProgramInfo(
        program: cl_program,
        param_name: cl_program_info,
        param_value_size: size_t,
        param_value: *mut libc::c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
    fn clGetProgramBuildInfo(
        program: cl_program,
        device: cl_device_id,
        param_name: cl_program_build_info,
        param_value_size: size_t,
        param_value: *mut libc::c_void,
        param_value_size_ret: *mut size_t,
    ) -> cl_int;
    fn clCreateKernel(
        program: cl_program,
        kernel_name: *const libc::c_char,
        errcode_ret: *mut cl_int,
    ) -> cl_kernel;
    fn clReleaseKernel(kernel: cl_kernel) -> cl_int;
    fn clSetKernelArg(
        kernel: cl_kernel,
        arg_index: cl_uint,
        arg_size: size_t,
        arg_value: *const libc::c_void,
    ) -> cl_int;
    fn clWaitForEvents(num_events: cl_uint, event_list: *const cl_event) -> cl_int;
    fn clReleaseEvent(event: cl_event) -> cl_int;
    fn clEnqueueWriteBuffer(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_write: cl_bool,
        offset: size_t,
        size: size_t,
        ptr: *const libc::c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    fn clEnqueueMapBuffer(
        command_queue: cl_command_queue,
        buffer: cl_mem,
        blocking_map: cl_bool,
        map_flags: cl_map_flags,
        offset: size_t,
        size: size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
        errcode_ret: *mut cl_int,
    ) -> *mut libc::c_void;
    fn clEnqueueUnmapMemObject(
        command_queue: cl_command_queue,
        memobj: cl_mem,
        mapped_ptr: *mut libc::c_void,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    fn clEnqueueNDRangeKernel(
        command_queue: cl_command_queue,
        kernel: cl_kernel,
        work_dim: cl_uint,
        global_work_offset: *const size_t,
        global_work_size: *const size_t,
        local_work_size: *const size_t,
        num_events_in_wait_list: cl_uint,
        event_wait_list: *const cl_event,
        event: *mut cl_event,
    ) -> cl_int;
    fn clCreateCommandQueue(
        context: cl_context,
        device: cl_device_id,
        properties: cl_command_queue_properties,
        errcode_ret: *mut cl_int,
    ) -> cl_command_queue;
    fn __errno_location() -> *mut libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pthread_self() -> pthread_t;
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
    fn BN_new() -> *mut BIGNUM;
    fn BN_copy(a: *mut BIGNUM, b: *const BIGNUM) -> *mut BIGNUM;
    fn BN_bn2bin(a: *const BIGNUM, to: *mut libc::c_uchar) -> libc::c_int;
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
    fn EC_POINT_point2oct(
        group: *const EC_GROUP,
        p: *const EC_POINT,
        form: point_conversion_form_t,
        buf: *mut libc::c_uchar,
        len: size_t,
        ctx: *mut BN_CTX,
    ) -> size_t;
    fn EC_KEY_new_by_curve_name(nid: libc::c_int) -> *mut EC_KEY;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ec_point_st {
    pub meth: *const EC_METHOD,
    pub X: BIGNUM,
    pub Y: BIGNUM,
    pub Z: BIGNUM,
    pub Z_is_one: libc::c_int,
}
pub type EC_METHOD = ec_method_st;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _vg_ocl_context_s {
    pub base: vg_exec_context_t,
    pub voc_ocldid: cl_device_id,
    pub voc_oclctx: cl_context,
    pub voc_oclcmdq: cl_command_queue,
    pub voc_oclprog: cl_program,
    pub voc_init_func: Option::<
        unsafe extern "C" fn(*mut _vg_ocl_context_s) -> libc::c_int,
    >,
    pub voc_rekey_func: Option::<
        unsafe extern "C" fn(*mut _vg_ocl_context_s) -> libc::c_int,
    >,
    pub voc_check_func: Option::<
        unsafe extern "C" fn(*mut _vg_ocl_context_s, libc::c_int) -> libc::c_int,
    >,
    pub voc_quirks: libc::c_int,
    pub voc_nslots: libc::c_int,
    pub voc_oclkernel: [[cl_kernel; 3]; 2],
    pub voc_oclkrnwait: [cl_event; 2],
    pub voc_args: [[cl_mem; 6]; 2],
    pub voc_arg_size: [[size_t; 6]; 2],
    pub voc_pattern_rewrite: libc::c_int,
    pub voc_pattern_alloc: libc::c_int,
    pub voc_verify_func: [vg_ocl_check_t; 3],
    pub voc_ocl_thread: pthread_t,
    pub voc_lock: pthread_mutex_t,
    pub voc_wait: pthread_cond_t,
    pub voc_ocl_slot: libc::c_int,
    pub voc_ocl_rows: libc::c_int,
    pub voc_ocl_cols: libc::c_int,
    pub voc_ocl_invsize: libc::c_int,
    pub voc_halt: libc::c_int,
    pub voc_dump_done: libc::c_int,
}
pub type vg_ocl_check_t = Option::<
    unsafe extern "C" fn(*mut _vg_ocl_context_s, libc::c_int) -> libc::c_int,
>;
pub type cl_mem = *mut _cl_mem;
pub type cl_event = *mut _cl_event;
pub type cl_kernel = *mut _cl_kernel;
pub type cl_program = *mut _cl_program;
pub type cl_command_queue = *mut _cl_command_queue;
pub type cl_context = *mut _cl_context;
pub type cl_device_id = *mut _cl_device_id;
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
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
pub type int32_t = __int32_t;
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
pub type ENGINE = engine_st;
pub type EVP_MD = env_md_st;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = libc::c_long;
pub type cl_int = int32_t;
pub type cl_uint = uint32_t;
pub type cl_ulong = uint64_t;
pub type cl_platform_id = *mut _cl_platform_id;
pub type cl_bool = cl_uint;
pub type cl_bitfield = cl_ulong;
pub type cl_device_type = cl_bitfield;
pub type cl_platform_info = cl_uint;
pub type cl_device_info = cl_uint;
pub type cl_command_queue_properties = cl_bitfield;
pub type cl_context_properties = intptr_t;
pub type cl_mem_flags = cl_bitfield;
pub type cl_map_flags = cl_bitfield;
pub type cl_program_info = cl_uint;
pub type cl_program_build_info = cl_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_vg_elf32_header_t_838073139 {
    pub e_ident: [libc::c_uchar; 16],
    pub e_type: uint16_t,
    pub e_machine: uint16_t,
    pub e_version: uint32_t,
    pub e_entry: uint32_t,
    pub e_phoff: uint32_t,
    pub e_shoff: uint32_t,
    pub e_flags: uint32_t,
    pub e_ehsize: uint16_t,
    pub e_phentsize: uint16_t,
    pub e_phnum: uint16_t,
    pub e_shentsize: uint16_t,
    pub e_shnum: uint16_t,
    pub e_shstrndx: uint16_t,
}
pub type vg_elf32_header_t = __anonstruct_vg_elf32_header_t_838073139;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_vg_elf32_shdr_t_293933665 {
    pub sh_name: uint32_t,
    pub sh_type: uint32_t,
    pub sh_flags: uint32_t,
    pub sh_addr: uint32_t,
    pub sh_offset: uint32_t,
    pub sh_size: uint32_t,
    pub sh_link: uint32_t,
    pub sh_info: uint32_t,
    pub sh_addralign: uint32_t,
    pub sh_entsize: uint32_t,
}
pub type vg_elf32_shdr_t = __anonstruct_vg_elf32_shdr_t_293933665;
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
#[inline(always)]
unsafe extern "C" fn fprintf(
    mut __stream: *mut FILE,
    mut __fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut tmp___0: libc::c_int = 0;
    return tmp___0;
}
#[inline(always)]
unsafe extern "C" fn printf(
    mut __fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut tmp___0: libc::c_int = 0;
    return tmp___0;
}
#[inline(always)]
unsafe extern "C" fn fgets(
    mut __s: *mut libc::c_char,
    mut __n: libc::c_int,
    mut __stream: *mut FILE,
) -> *mut libc::c_char {
    let mut tmp___0: libc::c_ulong = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_ulong = 0;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: libc::c_ulong = 0;
    let mut tmp___6: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp___5 = (if 1 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as _;
    if tmp___5 != 0xffffffffffffffff as libc::c_ulong {
        tmp___0 = (if 1 as libc::c_int & 2 == 0 { -1isize } else { 0isize })
            as _;
        tmp___1 = __fgets_chk(__s, tmp___0, __n, __stream);
        return tmp___1;
    }
    tmp___6 = __fgets_alias(__s, __n, __stream);
    return tmp___6;
}
#[inline(always)]
unsafe extern "C" fn fread(
    mut __ptr: *mut libc::c_void,
    mut __size: size_t,
    mut __n: size_t,
    mut __stream: *mut FILE,
) -> libc::c_ulong {
    let mut tmp___0: libc::c_ulong = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: libc::c_ulong = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: libc::c_ulong = 0;
    let mut tmp___5: libc::c_ulong = 0;
    let mut tmp___6: size_t = 0;
    tmp___5 = (if 0 as libc::c_int & 2 == 0 { -1isize } else { 0isize }) as _;
    if tmp___5 != 0xffffffffffffffff as libc::c_ulong {
        tmp___0 = (if 0 as libc::c_int & 2 == 0 { -1isize } else { 0isize })
            as _;
        tmp___1 = __fread_chk(__ptr, tmp___0, __size, __n, __stream);
        return tmp___1;
    }
    tmp___6 = __fread_alias(__ptr, __size, __n, __stream);
    return tmp___6;
}
#[inline]
unsafe extern "C" fn pthread_equal(
    mut __thread1: pthread_t,
    mut __thread2: pthread_t,
) -> libc::c_int {
    return (__thread1 == __thread2) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    let mut tmp___0: libc::c_long = 0;
    tmp___0 = strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    return tmp___0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    let mut tmp___0: libc::c_double = 0.;
    tmp___0 = strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    return tmp___0;
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
    let mut tmp___0: libc::c_int = 0;
    if (*rootp).ar_root as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = 1 as libc::c_int;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0;
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
    let mut tmp___0: *mut avl_item_t = 0 as *mut avl_item_t;
    tmp___0 = (*itemp).ai_left;
    (*itemp).ai_left = (*tmp___0).ai_right;
    if !((*itemp).ai_left).is_null() {
        (*(*itemp).ai_left).ai_up = itemp;
    }
    (*tmp___0).ai_right = itemp;
    if !((*itemp).ai_up).is_null() {
        if (*(*itemp).ai_up).ai_left as libc::c_ulong == itemp as libc::c_ulong {
            (*(*itemp).ai_up).ai_left = tmp___0;
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
            (*(*itemp).ai_up).ai_right = tmp___0;
        }
    } else {
        (*rootp).ar_root = tmp___0;
    }
    (*tmp___0).ai_up = (*itemp).ai_up;
    (*itemp).ai_up = tmp___0;
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
    let mut tmp___0: *mut avl_item_t = 0 as *mut avl_item_t;
    tmp___0 = (*itemp).ai_right;
    (*itemp).ai_right = (*tmp___0).ai_left;
    if !((*itemp).ai_right).is_null() {
        (*(*itemp).ai_right).ai_up = itemp;
    }
    (*tmp___0).ai_left = itemp;
    if !((*itemp).ai_up).is_null() {
        if (*(*itemp).ai_up).ai_right as libc::c_ulong == itemp as libc::c_ulong {
            (*(*itemp).ai_up).ai_right = tmp___0;
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
            (*(*itemp).ai_up).ai_left = tmp___0;
        }
    } else {
        (*rootp).ar_root = tmp___0;
    }
    (*tmp___0).ai_up = (*itemp).ai_up;
    (*itemp).ai_up = tmp___0;
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
    let mut tmp___0: *mut _avl_item_s = 0 as *mut _avl_item_s;
    parentp = (*itemp).ai_up;
    tmp___0 = 0 as *mut libc::c_void as *mut _avl_item_s;
    (*itemp).ai_right = tmp___0;
    (*itemp).ai_left = tmp___0;
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
    let mut tmp___0: libc::c_int = 0;
    cmpres = strcmp((*a).pattern, (*b).pattern);
    if cmpres == 0 {
        if (*a).reward < (*b).reward {
            cmpres = -(1 as libc::c_int);
        } else {
            if (*a).reward > (*b).reward {
                tmp___0 = 1 as libc::c_int;
            } else {
                tmp___0 = 0 as libc::c_int;
            }
            cmpres = tmp___0;
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
    let mut tmp___0: libc::c_int = 0;
    tmp___0 = strcmp((*a).pubkey_hex, (*b).pubkey_hex);
    return tmp___0;
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
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: size_t = 0;
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
    tmp___0 = strlen(pfx);
    tmp___1 = strlen(comment);
    tmp___2 = malloc(
        (::std::mem::size_of::<workitem_t>() as libc::c_ulong)
            .wrapping_add(tmp___0)
            .wrapping_add(tmp___1)
            .wrapping_add(2 as libc::c_ulong),
    );
    wip = tmp___2 as *mut workitem_t;
    memset(
        wip as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<workitem_t>() as libc::c_ulong as _,
    );
    avl_item_init(&mut (*wip).avlent);
    (*wip)
        .pattern = wip.offset(1 as libc::c_int as isize) as *mut libc::c_char
        as *const libc::c_char;
    strcpy((*wip).pattern as *mut libc::c_char, pfx);
    tmp___3 = strlen((*wip).pattern);
    (*wip)
        .comment = ((*wip).pattern)
        .offset(tmp___3.wrapping_add(1 as libc::c_ulong) as isize);
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if (*a).reward == (*b).reward {
        tmp___0 = strcmp((*a).pattern, (*b).pattern);
        if tmp___0 != 0 {
            tmp___1 = 0 as libc::c_int;
        } else {
            tmp___1 = 1 as libc::c_int;
        }
    } else {
        tmp___1 = 0 as libc::c_int;
    }
    return tmp___1;
}
unsafe extern "C" fn server_pubkeybatch_equal(
    mut ctxp: *mut server_context_t,
    mut a: *mut pubkeybatch_t,
    mut b: *mut pubkeybatch_t,
) -> libc::c_int {
    let mut wipa: *mut workitem_t = 0 as *mut workitem_t;
    let mut wipb: *mut workitem_t = 0 as *mut workitem_t;
    let mut tmp___0: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    if (*a).nitems != (*b).nitems {
        return 0 as libc::c_int;
    }
    tmp___0 = EC_KEY_get0_group((*ctxp).dummy_key as *const EC_KEY);
    tmp___1 = EC_POINT_cmp(
        tmp___0,
        (*a).pubkey as *const EC_POINT,
        (*b).pubkey as *const EC_POINT,
        0 as *mut libc::c_void as *mut BN_CTX,
    );
    if tmp___1 != 0 {
        return 0 as libc::c_int;
    }
    wipa = workitem_avl_first(&mut (*a).items);
    wipb = workitem_avl_first(&mut (*b).items);
    while !wipa.is_null() {
        if wipb.is_null() {
            break;
        }
        tmp___2 = server_workitem_equal(wipa, wipb);
        if tmp___2 == 0 {
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
    let mut tmp___0: size_t = 0;
    let mut addrlen: libc::c_int = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp___0 = strlen(url);
    urllen = tmp___0 as libc::c_int;
    tmp___1 = strlen(credit_addr);
    addrlen = tmp___1 as libc::c_int;
    tmp___2 = malloc(
        (::std::mem::size_of::<server_context_t>() as libc::c_ulong)
            .wrapping_add(urllen as libc::c_ulong)
            .wrapping_add(addrlen as libc::c_ulong)
            .wrapping_add(2 as libc::c_ulong),
    );
    ctxp = tmp___2 as *mut server_context_t;
    memset(
        ctxp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_context_t>() as libc::c_ulong as _,
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
    tmp___3 = malloc((urllen + 9 as libc::c_int) as size_t);
    (*ctxp).getwork = tmp___3 as *mut libc::c_char;
    tmp___4 = malloc((urllen + 7 as libc::c_int) as size_t);
    (*ctxp).submit = tmp___4 as *mut libc::c_char;
    if *url.offset((urllen - 1 as libc::c_int) as isize) as libc::c_int
        == 47 as libc::c_int
    {
        snprintf(
            (*ctxp).getwork,
            (urllen + 9 as libc::c_int) as size_t as _,
            b"%sgetWork\0" as *const u8 as *const libc::c_char,
            url,
        );
        snprintf(
            (*ctxp).submit,
            (urllen + 7 as libc::c_int) as size_t as _,
            b"%ssolve\0" as *const u8 as *const libc::c_char,
            url,
        );
    } else {
        snprintf(
            (*ctxp).getwork,
            (urllen + 9 as libc::c_int) as size_t as _,
            b"%s/getWork\0" as *const u8 as *const libc::c_char,
            url,
        );
        snprintf(
            (*ctxp).submit,
            (urllen + 7 as libc::c_int) as size_t as _,
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
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    pbatch = 0 as *mut libc::c_void as *mut pubkeybatch_t;
    pbatch = pubkeybatch_avl_search(
        &mut (*reqp).items,
        (*wip).pubkey as *const EC_POINT,
        (*reqp).group,
    );
    if pbatch as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = malloc(::std::mem::size_of::<pubkeybatch_t>() as libc::c_ulong);
        pbatch = tmp___0 as *mut pubkeybatch_t;
        if pbatch as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return -(1 as libc::c_int);
        }
        memset(
            pbatch as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<pubkeybatch_t>() as libc::c_ulong as _,
        );
        avl_item_init(&mut (*pbatch).avlent);
        avl_root_init(&mut (*pbatch).items);
        (*pbatch).total_value = 0 as libc::c_int as libc::c_double;
        (*pbatch).pubkey = (*wip).pubkey;
        tmp___1 = EC_POINT_point2hex(
            (*reqp).group,
            (*wip).pubkey as *const EC_POINT,
            POINT_CONVERSION_UNCOMPRESSED,
            0 as *mut libc::c_void as *mut BN_CTX,
        );
        (*pbatch).pubkey_hex = tmp___1 as *const libc::c_char;
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
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
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
                ((*reqp).part_end).wrapping_sub((*reqp).part_off) as _,
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
        tmp___0 = realloc((*reqp).part_buf as *mut libc::c_void, (*reqp).part_size);
        (*reqp).part_buf = tmp___0 as *mut libc::c_char;
        if ((*reqp).part_buf).is_null() {
            fprintf(stderr, b"Out of memory\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int as size_t;
        }
    }
    memcpy(
        ((*reqp).part_buf).offset((*reqp).part_end as isize) as *mut libc::c_void,
        buf as *const libc::c_void,
        len as _,
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
        tmp___1 = server_workitem_add(reqp, wip);
        if !(tmp___1 != 0) {
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
    let mut tmp___0: libc::c_int = 0;
    if (*reqp).part_buf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        free((*reqp).part_buf as *mut libc::c_void);
    }
    tmp___0 = avl_root_empty(&mut (*reqp).items);
    if tmp___0 == 0 {
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
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut _curl_opt: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: CURLcode = CURLE_OK;
    let mut _curl_opt___0: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: CURLcode = CURLE_OK;
    let mut _curl_opt___1: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: CURLcode = CURLE_OK;
    let mut _curl_opt___2: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: CURLcode = CURLE_OK;
    let mut _curl_opt___3: libc::c_int = 0;
    let mut tmp___10: CURLcode = CURLE_OK;
    let mut tmp___11: *const libc::c_char = 0 as *const libc::c_char;
    tmp___0 = malloc(::std::mem::size_of::<server_request_t>() as libc::c_ulong);
    reqp = tmp___0 as *mut server_request_t;
    memset(
        reqp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_request_t>() as libc::c_ulong as _,
    );
    (*reqp).group = EC_KEY_get0_group((*ctxp).dummy_key as *const EC_KEY);
    creq = curl_easy_init();
    _curl_opt = 10002 as libc::c_int;
    tmp___3 = curl_easy_setopt(creq, _curl_opt as CURLoption, (*ctxp).getwork);
    if tmp___3 as u64 != 0 {
        fprintf(
            stderr,
            b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        _curl_opt___0 = 41 as libc::c_int;
        tmp___5 = curl_easy_setopt(
            creq,
            _curl_opt___0 as CURLoption,
            ((*ctxp).verbose > 1 as libc::c_int) as libc::c_int,
        );
        if tmp___5 as u64 != 0 {
            fprintf(
                stderr,
                b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        } else {
            _curl_opt___1 = 20011 as libc::c_int;
            tmp___7 = curl_easy_setopt(
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
            if tmp___7 as u64 != 0 {
                fprintf(
                    stderr,
                    b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            } else {
                _curl_opt___2 = 52 as libc::c_int;
                tmp___9 = curl_easy_setopt(
                    creq,
                    _curl_opt___2 as CURLoption,
                    1 as libc::c_int,
                );
                if tmp___9 as u64 != 0 {
                    fprintf(
                        stderr,
                        b"Failed to set up libcurl\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                } else {
                    _curl_opt___3 = 10001 as libc::c_int;
                    tmp___10 = curl_easy_setopt(creq, _curl_opt___3 as CURLoption, reqp);
                    if tmp___10 as u64 != 0 {
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
                        tmp___11 = curl_easy_strerror(res);
                        fprintf(
                            stderr,
                            b"Get work request failed: %s\n\0" as *const u8
                                as *const libc::c_char,
                            tmp___11,
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
    let mut tmp___0: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut _curl_opt: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: CURLcode = CURLE_OK;
    let mut _curl_opt___0: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: CURLcode = CURLE_OK;
    let mut _curl_opt___1: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: CURLcode = CURLE_OK;
    let mut tmp___8: *const libc::c_char = 0 as *const libc::c_char;
    tmp___0 = EC_KEY_get0_group((*ctxp).dummy_key as *const EC_KEY);
    pubhex = EC_POINT_point2hex(
        tmp___0,
        (*work).pubkey as *const EC_POINT,
        POINT_CONVERSION_UNCOMPRESSED,
        0 as *mut libc::c_void as *mut BN_CTX,
    );
    snprintf(
        urlbuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as _,
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
    tmp___3 = curl_easy_setopt(creq, _curl_opt as CURLoption, urlbuf.as_mut_ptr());
    if tmp___3 as u64 != 0 {
        fprintf(
            stderr,
            b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    } else {
        _curl_opt___0 = 41 as libc::c_int;
        tmp___5 = curl_easy_setopt(
            creq,
            _curl_opt___0 as CURLoption,
            ((*ctxp).verbose > 1 as libc::c_int) as libc::c_int,
        );
        if tmp___5 as u64 != 0 {
            fprintf(
                stderr,
                b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        } else {
            _curl_opt___1 = 52 as libc::c_int;
            tmp___7 = curl_easy_setopt(
                creq,
                _curl_opt___1 as CURLoption,
                1 as libc::c_int,
            );
            if tmp___7 as u64 != 0 {
                fprintf(
                    stderr,
                    b"Failed to set up libcurl\n\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            res = curl_easy_perform(creq);
            if res as libc::c_uint != 0 as libc::c_uint {
                tmp___8 = curl_easy_strerror(res);
                fprintf(
                    stderr,
                    b"Submission failed: %s\n\0" as *const u8 as *const libc::c_char,
                    tmp___8,
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
    let mut tmp___0: *const BIGNUM = 0 as *const BIGNUM;
    vg_output_match_console(vcp, pkey, pattern);
    pthread_mutex_lock(&mut soln_lock);
    free_soln();
    soln_pattern = strdup(pattern);
    tmp___0 = EC_KEY_get0_private_key(pkey as *const EC_KEY);
    soln_private_key = BN_bn2hex(tmp___0);
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
    let mut tmp___0: *mut workitem_t = 0 as *mut workitem_t;
    res = 0 as libc::c_int;
    pthread_mutex_lock(&mut soln_lock);
    if soln_private_key as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = workitem_avl_search(
            &mut (*pbatch).items,
            soln_pattern as *const libc::c_char,
        );
        wip = tmp___0;
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
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: libc::c_long = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut wip: *mut workitem_t = 0 as *mut workitem_t;
    let mut tmp___7: libc::c_int = 0;
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
                tmp___0 = strtol(
                    optarg as *const libc::c_char,
                    &mut pend as *mut *mut libc::c_char,
                    0 as libc::c_int,
                );
                ncols = tmp___0 as libc::c_int;
                if !pend.is_null() {
                    if *pend as libc::c_int == 120 as libc::c_int {
                        tmp___1 = strtol(
                            pend.offset(1 as libc::c_int as isize)
                                as *const libc::c_char,
                            0 as *mut libc::c_void as *mut *mut libc::c_char,
                            0 as libc::c_int,
                        );
                        nrows = tmp___1 as libc::c_int;
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
                tmp___2 = ndevstrs;
                ndevstrs += 1;
                devstrs[tmp___2 as usize] = optarg;
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
    tmp___3 = vg_b58_decode_check(
        credit_addr,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
    );
    if tmp___3 == 0 {
        fprintf(
            stderr,
            b"ERROR: Invalid reward address specified\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    scp = server_context_new(url, credit_addr);
    (*scp).verbose = verbose;
    tmp___4 = server_context_getwork(scp);
    if tmp___4 != 0 {
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
        tmp___5 = avl_root_empty(&mut (*scp).items);
        if tmp___5 != 0 {
            server_context_getwork(scp);
        }
        pkb = most_valuable_pkb(scp);
        if !pkb.is_null() {
            if !active_pkb.is_null() {
                tmp___6 = server_pubkeybatch_equal(scp, active_pkb, pkb);
                if tmp___6 != 0 {
                    pkb = active_pkb;
                }
            }
        }
        if thread_started != 0 {
            let mut current_block_159: u64;
            if active_pkb.is_null() {
                current_block_159 = 15221755894416026288;
            } else if pkb as libc::c_ulong != active_pkb as libc::c_ulong {
                current_block_159 = 15221755894416026288;
            } else {
                current_block_159 = 939350892795860671;
            }
            match current_block_159 {
                15221755894416026288 => {
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
                tmp___7 = vg_context_add_patterns(
                    vcp,
                    &mut (*wip).pattern as *mut *const libc::c_char,
                    1 as libc::c_int,
                );
                if tmp___7 != 0 {
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
pub unsafe extern "C" fn vg_exec_downgrade_lock(mut vxcp: *mut vg_exec_context_t) {}
pub unsafe extern "C" fn vg_exec_upgrade_lock(
    mut vxcp: *mut vg_exec_context_t,
) -> libc::c_int {
    return 0 as libc::c_int;
}
static mut tmp: [libc::c_char; 64] = [0; 64];
unsafe extern "C" fn vg_ocl_strerror(mut ret: cl_int) -> *const libc::c_char {
    match ret {
        0 => return b"CL_SUCCESS\0" as *const u8 as *const libc::c_char,
        -1 => return b"CL_DEVICE_NOT_FOUND\0" as *const u8 as *const libc::c_char,
        -2 => return b"CL_DEVICE_NOT_AVAILABLE\0" as *const u8 as *const libc::c_char,
        -3 => return b"CL_COMPILER_NOT_AVAILABLE\0" as *const u8 as *const libc::c_char,
        -4 => {
            return b"CL_MEM_OBJECT_ALLOCATION_FAILURE\0" as *const u8
                as *const libc::c_char;
        }
        -5 => return b"CL_OUT_OF_RESOURCES\0" as *const u8 as *const libc::c_char,
        -6 => return b"CL_OUT_OF_HOST_MEMORY\0" as *const u8 as *const libc::c_char,
        -7 => {
            return b"CL_PROFILING_INFO_NOT_AVAILABLE\0" as *const u8
                as *const libc::c_char;
        }
        -8 => return b"CL_MEM_COPY_OVERLAP\0" as *const u8 as *const libc::c_char,
        -9 => return b"CL_IMAGE_FORMAT_MISMATCH\0" as *const u8 as *const libc::c_char,
        -10 => {
            return b"CL_IMAGE_FORMAT_NOT_SUPPORTED\0" as *const u8 as *const libc::c_char;
        }
        -11 => return b"CL_BUILD_PROGRAM_FAILURE\0" as *const u8 as *const libc::c_char,
        -12 => return b"CL_MAP_FAILURE\0" as *const u8 as *const libc::c_char,
        -13 => {
            return b"CL_MISALIGNED_SUB_BUFFER_OFFSET\0" as *const u8
                as *const libc::c_char;
        }
        -14 => {
            return b"CL_EXEC_STATUS_ERROR_FOR_EVENTS_IN_WAIT_LIST\0" as *const u8
                as *const libc::c_char;
        }
        -30 => return b"CL_INVALID_VALUE\0" as *const u8 as *const libc::c_char,
        -31 => return b"CL_INVALID_DEVICE_TYPE\0" as *const u8 as *const libc::c_char,
        -32 => return b"CL_INVALID_PLATFORM\0" as *const u8 as *const libc::c_char,
        -33 => return b"CL_INVALID_DEVICE\0" as *const u8 as *const libc::c_char,
        -34 => return b"CL_INVALID_CONTEXT\0" as *const u8 as *const libc::c_char,
        -35 => {
            return b"CL_INVALID_QUEUE_PROPERTIES\0" as *const u8 as *const libc::c_char;
        }
        -36 => return b"CL_INVALID_COMMAND_QUEUE\0" as *const u8 as *const libc::c_char,
        -37 => return b"CL_INVALID_HOST_PTR\0" as *const u8 as *const libc::c_char,
        -38 => return b"CL_INVALID_MEM_OBJECT\0" as *const u8 as *const libc::c_char,
        -39 => {
            return b"CL_INVALID_IMAGE_FORMAT_DESCRIPTOR\0" as *const u8
                as *const libc::c_char;
        }
        -40 => return b"CL_INVALID_IMAGE_SIZE\0" as *const u8 as *const libc::c_char,
        -41 => return b"CL_INVALID_SAMPLER\0" as *const u8 as *const libc::c_char,
        -42 => return b"CL_INVALID_BINARY\0" as *const u8 as *const libc::c_char,
        -43 => return b"CL_INVALID_BUILD_OPTIONS\0" as *const u8 as *const libc::c_char,
        -44 => return b"CL_INVALID_PROGRAM\0" as *const u8 as *const libc::c_char,
        -45 => {
            return b"CL_INVALID_PROGRAM_EXECUTABLE\0" as *const u8 as *const libc::c_char;
        }
        -46 => return b"CL_INVALID_KERNEL_NAME\0" as *const u8 as *const libc::c_char,
        -47 => {
            return b"CL_INVALID_KERNEL_DEFINITION\0" as *const u8 as *const libc::c_char;
        }
        -48 => return b"CL_INVALID_KERNEL\0" as *const u8 as *const libc::c_char,
        -49 => return b"CL_INVALID_ARG_INDEX\0" as *const u8 as *const libc::c_char,
        -50 => return b"CL_INVALID_ARG_VALUE\0" as *const u8 as *const libc::c_char,
        -51 => return b"CL_INVALID_ARG_SIZE\0" as *const u8 as *const libc::c_char,
        -52 => return b"CL_INVALID_KERNEL_ARGS\0" as *const u8 as *const libc::c_char,
        -53 => return b"CL_INVALID_WORK_DIMENSION\0" as *const u8 as *const libc::c_char,
        -54 => return b"CL_INVALID_WORK_GROUP_SIZE\0" as *const u8 as *const libc::c_char,
        -55 => return b"CL_INVALID_WORK_ITEM_SIZE\0" as *const u8 as *const libc::c_char,
        -56 => return b"CL_INVALID_GLOBAL_OFFSET\0" as *const u8 as *const libc::c_char,
        -57 => return b"CL_INVALID_EVENT_WAIT_LIST\0" as *const u8 as *const libc::c_char,
        -58 => return b"CL_INVALID_EVENT\0" as *const u8 as *const libc::c_char,
        -59 => return b"CL_INVALID_OPERATION\0" as *const u8 as *const libc::c_char,
        -60 => return b"CL_INVALID_GL_OBJECT\0" as *const u8 as *const libc::c_char,
        -61 => return b"CL_INVALID_BUFFER_SIZE\0" as *const u8 as *const libc::c_char,
        -62 => return b"CL_INVALID_MIP_LEVEL\0" as *const u8 as *const libc::c_char,
        -63 => {
            return b"CL_INVALID_GLOBAL_WORK_SIZE\0" as *const u8 as *const libc::c_char;
        }
        -64 => return b"CL_INVALID_PROPERTY\0" as *const u8 as *const libc::c_char,
        _ => {
            snprintf(
                tmp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as _,
                b"Unknown code %d\0" as *const u8 as *const libc::c_char,
                ret,
            );
            return tmp.as_mut_ptr() as *const libc::c_char;
        }
    };
}
static mut platform_str: [libc::c_char; 1024] = [0; 1024];
unsafe extern "C" fn vg_ocl_platform_getstr(
    mut pid: cl_platform_id,
    mut param: cl_platform_info,
) -> *const libc::c_char {
    let mut ret: cl_int = 0;
    let mut size_ret: size_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    ret = clGetPlatformInfo(
        pid,
        param,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        platform_str.as_mut_ptr() as *mut libc::c_void,
        &mut size_ret,
    );
    if ret != 0 as libc::c_int {
        tmp___0 = vg_ocl_strerror(ret);
        snprintf(
            platform_str.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as _,
            b"clGetPlatformInfo(%d): %s\0" as *const u8 as *const libc::c_char,
            param,
            tmp___0,
        );
    }
    return platform_str.as_mut_ptr() as *const libc::c_char;
}
unsafe extern "C" fn vg_ocl_device_getplatform(mut did: cl_device_id) -> cl_platform_id {
    let mut ret: cl_int = 0;
    let mut val: cl_platform_id = 0 as *mut _cl_platform_id;
    let mut size_ret: size_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    ret = clGetDeviceInfo(
        did,
        4145 as libc::c_int as cl_device_info,
        ::std::mem::size_of::<cl_platform_id>() as libc::c_ulong,
        &mut val as *mut cl_platform_id as *mut libc::c_void,
        &mut size_ret,
    );
    if ret != 0 as libc::c_int {
        tmp___0 = vg_ocl_strerror(ret);
        fprintf(
            stderr,
            b"clGetDeviceInfo(CL_DEVICE_PLATFORM): %s\0" as *const u8
                as *const libc::c_char,
            tmp___0,
        );
    }
    return val;
}
unsafe extern "C" fn vg_ocl_device_gettype(mut did: cl_device_id) -> cl_device_type {
    let mut ret: cl_int = 0;
    let mut val: cl_device_type = 0;
    let mut size_ret: size_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    ret = clGetDeviceInfo(
        did,
        4096 as libc::c_int as cl_device_info,
        ::std::mem::size_of::<cl_device_type>() as libc::c_ulong,
        &mut val as *mut cl_device_type as *mut libc::c_void,
        &mut size_ret,
    );
    if ret != 0 as libc::c_int {
        tmp___0 = vg_ocl_strerror(ret);
        fprintf(
            stderr,
            b"clGetDeviceInfo(CL_DEVICE_TYPE): %s\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
    }
    return val;
}
static mut device_str: [libc::c_char; 1024] = [0; 1024];
unsafe extern "C" fn vg_ocl_device_getstr(
    mut did: cl_device_id,
    mut param: cl_device_info,
) -> *const libc::c_char {
    let mut ret: cl_int = 0;
    let mut size_ret: size_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    ret = clGetDeviceInfo(
        did,
        param,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        device_str.as_mut_ptr() as *mut libc::c_void,
        &mut size_ret,
    );
    if ret != 0 as libc::c_int {
        tmp___0 = vg_ocl_strerror(ret);
        snprintf(
            device_str.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as _,
            b"clGetDeviceInfo(%d): %s\0" as *const u8 as *const libc::c_char,
            param,
            tmp___0,
        );
    }
    return device_str.as_mut_ptr() as *const libc::c_char;
}
unsafe extern "C" fn vg_ocl_device_getsizet(
    mut did: cl_device_id,
    mut param: cl_device_info,
) -> size_t {
    let mut ret: cl_int = 0;
    let mut val: size_t = 0;
    let mut size_ret: size_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    ret = clGetDeviceInfo(
        did,
        param,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
        &mut val as *mut size_t as *mut libc::c_void,
        &mut size_ret,
    );
    if ret != 0 as libc::c_int {
        tmp___0 = vg_ocl_strerror(ret);
        fprintf(
            stderr,
            b"clGetDeviceInfo(%d): %s\0" as *const u8 as *const libc::c_char,
            param,
            tmp___0,
        );
    }
    return val;
}
unsafe extern "C" fn vg_ocl_device_getulong(
    mut did: cl_device_id,
    mut param: cl_device_info,
) -> cl_ulong {
    let mut ret: cl_int = 0;
    let mut val: cl_ulong = 0;
    let mut size_ret: size_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    ret = clGetDeviceInfo(
        did,
        param,
        ::std::mem::size_of::<cl_ulong>() as libc::c_ulong,
        &mut val as *mut cl_ulong as *mut libc::c_void,
        &mut size_ret,
    );
    if ret != 0 as libc::c_int {
        tmp___0 = vg_ocl_strerror(ret);
        fprintf(
            stderr,
            b"clGetDeviceInfo(%d): %s\0" as *const u8 as *const libc::c_char,
            param,
            tmp___0,
        );
    }
    return val;
}
unsafe extern "C" fn vg_ocl_device_getuint(
    mut did: cl_device_id,
    mut param: cl_device_info,
) -> cl_uint {
    let mut ret: cl_int = 0;
    let mut val: cl_uint = 0;
    let mut size_ret: size_t = 0;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    ret = clGetDeviceInfo(
        did,
        param,
        ::std::mem::size_of::<cl_uint>() as libc::c_ulong,
        &mut val as *mut cl_uint as *mut libc::c_void,
        &mut size_ret,
    );
    if ret != 0 as libc::c_int {
        tmp___0 = vg_ocl_strerror(ret);
        fprintf(
            stderr,
            b"clGetDeviceInfo(%d): %s\0" as *const u8 as *const libc::c_char,
            param,
            tmp___0,
        );
    }
    return val;
}
pub unsafe extern "C" fn vg_ocl_dump_info(mut vocp: *mut vg_ocl_context_t) {
    let mut did: cl_device_id = 0 as *mut _cl_device_id;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: cl_uint = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: cl_ulong = 0;
    let mut tmp___9: cl_ulong = 0;
    if !((*vocp).base.vxc_vc).is_null() {
        if (*(*vocp).base.vxc_vc).vc_verbose < 1 as libc::c_int {
            return;
        }
    }
    if (*vocp).voc_dump_done != 0 {
        return;
    }
    did = (*vocp).voc_ocldid;
    tmp___0 = vg_ocl_device_getstr(did, 4139 as libc::c_int as cl_device_info);
    fprintf(stderr, b"Device: %s\n\0" as *const u8 as *const libc::c_char, tmp___0);
    tmp___1 = vg_ocl_device_getuint(did, 4097 as libc::c_int as cl_device_info);
    tmp___2 = vg_ocl_device_getstr(did, 4140 as libc::c_int as cl_device_info);
    fprintf(
        stderr,
        b"Vendor: %s (%04x)\n\0" as *const u8 as *const libc::c_char,
        tmp___2,
        tmp___1,
    );
    tmp___3 = vg_ocl_device_getstr(did, 4141 as libc::c_int as cl_device_info);
    fprintf(stderr, b"Driver: %s\n\0" as *const u8 as *const libc::c_char, tmp___3);
    tmp___4 = vg_ocl_device_getstr(did, 4142 as libc::c_int as cl_device_info);
    fprintf(stderr, b"Profile: %s\n\0" as *const u8 as *const libc::c_char, tmp___4);
    tmp___5 = vg_ocl_device_getstr(did, 4143 as libc::c_int as cl_device_info);
    fprintf(stderr, b"Version: %s\n\0" as *const u8 as *const libc::c_char, tmp___5);
    tmp___6 = vg_ocl_device_getsizet(did, 4098 as libc::c_int as cl_device_info);
    fprintf(
        stderr,
        b"Max compute units: %zd\n\0" as *const u8 as *const libc::c_char,
        tmp___6,
    );
    tmp___7 = vg_ocl_device_getsizet(did, 4100 as libc::c_int as cl_device_info);
    fprintf(
        stderr,
        b"Max workgroup size: %zd\n\0" as *const u8 as *const libc::c_char,
        tmp___7,
    );
    tmp___8 = vg_ocl_device_getulong(did, 4127 as libc::c_int as cl_device_info);
    fprintf(
        stderr,
        b"Global memory: %ld\n\0" as *const u8 as *const libc::c_char,
        tmp___8,
    );
    tmp___9 = vg_ocl_device_getulong(did, 4112 as libc::c_int as cl_device_info);
    fprintf(
        stderr,
        b"Max allocation: %ld\n\0" as *const u8 as *const libc::c_char,
        tmp___9,
    );
    (*vocp).voc_dump_done = 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_ocl_error(
    mut vocp: *mut vg_ocl_context_t,
    mut code: libc::c_int,
    mut desc: *const libc::c_char,
) {
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    tmp___0 = vg_ocl_strerror(code);
    err = tmp___0;
    if !desc.is_null() {
        fprintf(stderr, b"%s: %s\n\0" as *const u8 as *const libc::c_char, desc, err);
    } else {
        fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, err);
    }
    if !vocp.is_null() {
        if !((*vocp).voc_ocldid).is_null() {
            vg_ocl_dump_info(vocp);
        }
    }
}
unsafe extern "C" fn vg_ocl_buildlog(
    mut vocp: *mut vg_ocl_context_t,
    mut prog: cl_program,
) {
    let mut logbufsize: size_t = 0;
    let mut logsize: size_t = 0;
    let mut log___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut off: libc::c_int = 0;
    let mut ret: cl_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    off = 0 as libc::c_int;
    ret = clGetProgramBuildInfo(
        prog,
        (*vocp).voc_ocldid,
        4483 as libc::c_int as cl_program_build_info,
        0 as libc::c_int as size_t,
        0 as *mut libc::c_void,
        &mut logbufsize,
    );
    if ret != 0 as libc::c_int {
        vg_ocl_error(
            0 as *mut libc::c_void as *mut vg_ocl_context_t,
            ret,
            b"clGetProgramBuildInfo\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    tmp___0 = malloc(logbufsize);
    log___0 = tmp___0 as *mut libc::c_char;
    if log___0.is_null() {
        fprintf(
            stderr,
            b"Could not allocate build log buffer\n\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    ret = clGetProgramBuildInfo(
        prog,
        (*vocp).voc_ocldid,
        4483 as libc::c_int as cl_program_build_info,
        logbufsize,
        log___0 as *mut libc::c_void,
        &mut logsize,
    );
    if ret != 0 as libc::c_int {
        vg_ocl_error(
            0 as *mut libc::c_void as *mut vg_ocl_context_t,
            ret,
            b"clGetProgramBuildInfo\0" as *const u8 as *const libc::c_char,
        );
    } else {
        *log___0
            .offset(
                logbufsize.wrapping_sub(1 as libc::c_ulong) as isize,
            ) = '\u{0}' as i32 as libc::c_char;
        off = logsize.wrapping_sub(1 as libc::c_ulong) as libc::c_int;
        while off >= 0 as libc::c_int {
            if *log___0.offset(off as isize) as libc::c_int != 13 as libc::c_int {
                if *log___0.offset(off as isize) as libc::c_int != 10 as libc::c_int {
                    if *log___0.offset(off as isize) as libc::c_int != 32 as libc::c_int
                    {
                        if *log___0.offset(off as isize) as libc::c_int
                            != 9 as libc::c_int
                        {
                            if *log___0.offset(off as isize) as libc::c_int
                                != 0 as libc::c_int
                            {
                                break;
                            }
                        }
                    }
                }
            }
            *log___0.offset(off as isize) = '\u{0}' as i32 as libc::c_char;
            off -= 1;
        }
        off = 0 as libc::c_int;
        while (off as size_t) < logbufsize {
            if *log___0.offset(off as isize) as libc::c_int != 13 as libc::c_int {
                if *log___0.offset(off as isize) as libc::c_int != 10 as libc::c_int {
                    break;
                }
            }
            off += 1;
        }
        fprintf(
            stderr,
            b"Build log:\n%s\n\0" as *const u8 as *const libc::c_char,
            log___0.offset(off as isize),
        );
    }
    free(log___0 as *mut libc::c_void);
}
unsafe extern "C" fn vg_ocl_get_quirks(mut vocp: *mut vg_ocl_context_t) -> libc::c_int {
    let mut vend: uint32_t = 0;
    let mut dvn: *const libc::c_char = 0 as *const libc::c_char;
    let mut quirks: libc::c_uint = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: cl_device_type = 0;
    quirks = 0 as libc::c_uint;
    quirks |= 1 as libc::c_uint;
    vend = vg_ocl_device_getuint(
        (*vocp).voc_ocldid,
        4097 as libc::c_int as cl_device_info,
    );
    match vend {
        4318 => {
            quirks &= 4294967294 as libc::c_uint;
            quirks |= 2 as libc::c_uint;
            quirks |= 32 as libc::c_uint;
        }
        4098 => {
            tmp___2 = vg_ocl_device_gettype((*vocp).voc_ocldid);
            if tmp___2 & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0 {
                quirks |= 4 as libc::c_uint;
                quirks |= 8 as libc::c_uint;
                dvn = vg_ocl_device_getstr(
                    (*vocp).voc_ocldid,
                    4144 as libc::c_int as cl_device_info,
                );
                if !dvn.is_null() {
                    tmp___0 = strstr(
                        dvn,
                        b"cl_amd_media_ops\0" as *const u8 as *const libc::c_char,
                    );
                    if !tmp___0.is_null() {
                        quirks |= 16 as libc::c_uint;
                    }
                }
                dvn = vg_ocl_device_getstr(
                    (*vocp).voc_ocldid,
                    4139 as libc::c_int as cl_device_info,
                );
                tmp___1 = strcmp(
                    dvn,
                    b"ATI RV710\0" as *const u8 as *const libc::c_char,
                );
                if tmp___1 == 0 {
                    quirks &= 4294967264 as libc::c_uint;
                    quirks |= 128 as libc::c_uint;
                }
            }
        }
        _ => {}
    }
    return quirks as libc::c_int;
}
unsafe extern "C" fn vg_ocl_create_kernel(
    mut vocp: *mut vg_ocl_context_t,
    mut knum: libc::c_int,
    mut func: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut krn: cl_kernel = 0 as *mut _cl_kernel;
    let mut ret: cl_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        krn = clCreateKernel((*vocp).voc_oclprog, func, &mut ret);
        if krn.is_null() {
            fprintf(
                stderr,
                b"clCreateKernel(%d): \0" as *const u8 as *const libc::c_char,
                i,
            );
            vg_ocl_error(vocp, ret, 0 as *mut libc::c_void as *const libc::c_char);
            loop {
                i -= 1;
                if !(i >= 0 as libc::c_int) {
                    break;
                }
                clReleaseKernel((*vocp).voc_oclkernel[i as usize][knum as usize]);
                (*vocp)
                    .voc_oclkernel[i
                    as usize][knum as usize] = 0 as *mut libc::c_void as cl_kernel;
            }
            return 0 as libc::c_int;
        }
        (*vocp).voc_oclkernel[i as usize][knum as usize] = krn;
        (*vocp).voc_oclkrnwait[i as usize] = 0 as *mut libc::c_void as cl_event;
        i += 1;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_ocl_hash_program(
    mut vocp: *mut vg_ocl_context_t,
    mut opts: *const libc::c_char,
    mut program: *const libc::c_char,
    mut size: size_t,
    mut hash_out: *mut libc::c_uchar,
) {
    let mut mdctx: *mut EVP_MD_CTX = 0 as *mut EVP_MD_CTX;
    let mut pid: cl_platform_id = 0 as *mut _cl_platform_id;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const EVP_MD = 0 as *const EVP_MD;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    mdctx = EVP_MD_CTX_create();
    tmp___0 = EVP_md5();
    EVP_DigestInit_ex(mdctx, tmp___0, 0 as *mut libc::c_void as *mut ENGINE);
    pid = vg_ocl_device_getplatform((*vocp).voc_ocldid);
    str = vg_ocl_platform_getstr(pid, 2306 as libc::c_int as cl_platform_info);
    tmp___1 = strlen(str);
    EVP_DigestUpdate(
        mdctx,
        str as *const libc::c_void,
        tmp___1.wrapping_add(1 as libc::c_ulong),
    );
    str = vg_ocl_platform_getstr(pid, 2305 as libc::c_int as cl_platform_info);
    tmp___2 = strlen(str);
    EVP_DigestUpdate(
        mdctx,
        str as *const libc::c_void,
        tmp___2.wrapping_add(1 as libc::c_ulong),
    );
    str = vg_ocl_device_getstr(
        (*vocp).voc_ocldid,
        4139 as libc::c_int as cl_device_info,
    );
    tmp___3 = strlen(str);
    EVP_DigestUpdate(
        mdctx,
        str as *const libc::c_void,
        tmp___3.wrapping_add(1 as libc::c_ulong),
    );
    if !opts.is_null() {
        tmp___4 = strlen(opts);
        EVP_DigestUpdate(
            mdctx,
            opts as *const libc::c_void,
            tmp___4.wrapping_add(1 as libc::c_ulong),
        );
    }
    if size != 0 {
        EVP_DigestUpdate(mdctx, program as *const libc::c_void, size);
    }
    EVP_DigestFinal_ex(mdctx, hash_out, 0 as *mut libc::c_void as *mut libc::c_uint);
    EVP_MD_CTX_destroy(mdctx);
}
unsafe extern "C" fn vg_ocl_amd_patch_inner(
    mut binary: *mut libc::c_uchar,
    mut size: size_t,
) -> libc::c_int {
    let mut ehp: *mut vg_elf32_header_t = 0 as *mut vg_elf32_header_t;
    let mut shp: *mut vg_elf32_shdr_t = 0 as *mut vg_elf32_shdr_t;
    let mut nshp: *mut vg_elf32_shdr_t = 0 as *mut vg_elf32_shdr_t;
    let mut instr: *mut uint32_t = 0 as *mut uint32_t;
    let mut off: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut txt2idx: libc::c_int = 0;
    let mut patched: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    ehp = binary as *mut vg_elf32_header_t;
    if size < ::std::mem::size_of::<vg_elf32_header_t>() as libc::c_ulong {
        return 0 as libc::c_int
    } else {
        tmp___0 = memcmp(
            ((*ehp).e_ident).as_mut_ptr() as *const libc::c_void,
            b"\x7FELF\x01\x01\x01d\0" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            8 as libc::c_int as size_t,
        );
        if tmp___0 != 0 {
            return 0 as libc::c_int
        } else {
            if (*ehp).e_shoff == 0 {
                return 0 as libc::c_int;
            }
        }
    }
    off = ((*ehp).e_shoff)
        .wrapping_add(
            ((*ehp).e_shstrndx as libc::c_int * (*ehp).e_shentsize as libc::c_int)
                as uint32_t,
        ) as size_t;
    nshp = binary.offset(off as isize) as *mut vg_elf32_shdr_t;
    if off.wrapping_add(::std::mem::size_of::<vg_elf32_shdr_t>() as libc::c_ulong) > size
    {
        return 0 as libc::c_int;
    }
    shp = binary.offset((*ehp).e_shoff as isize) as *mut vg_elf32_shdr_t;
    n = 0 as libc::c_int;
    txt2idx = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*ehp).e_shnum as libc::c_int {
        off = ((*nshp).sh_offset).wrapping_add((*shp.offset(i as isize)).sh_name)
            as size_t;
        if !(off.wrapping_add(6 as libc::c_ulong) >= size) {
            tmp___1 = memcmp(
                binary.offset(off as isize) as *const libc::c_void,
                b".text\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                6 as libc::c_int as size_t,
            );
            if !(tmp___1 != 0) {
                n += 1;
                if n == 2 as libc::c_int {
                    txt2idx = i;
                }
            }
        }
        i += 1;
    }
    if n != 2 as libc::c_int {
        return 0 as libc::c_int;
    }
    off = (*shp.offset(txt2idx as isize)).sh_offset as size_t;
    instr = binary.offset(off as isize) as *mut uint32_t;
    n = ((*shp.offset(txt2idx as isize)).sh_size).wrapping_div(4 as libc::c_uint)
        as libc::c_int;
    patched = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if *instr.offset(i as isize) & 33558528 as libc::c_uint == 0 as libc::c_uint {
            if *instr.offset((i + 1 as libc::c_int) as isize)
                & 2416177152 as libc::c_uint == 106496 as libc::c_uint
            {
                let ref mut fresh0 = *instr.offset((i + 1 as libc::c_int) as isize);
                *fresh0 ^= 90112 as libc::c_uint;
                patched += 1;
            }
        }
        i += 2 as libc::c_int;
    }
    return patched;
}
unsafe extern "C" fn vg_ocl_amd_patch(
    mut vocp: *mut vg_ocl_context_t,
    mut binary: *mut libc::c_uchar,
    mut size: size_t,
) -> libc::c_int {
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    let mut ehp: *mut vg_elf32_header_t = 0 as *mut vg_elf32_header_t;
    let mut ptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut offset: size_t = 0;
    let mut ninner: libc::c_int = 0;
    let mut nrun: libc::c_int = 0;
    let mut npatched: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: libc::c_int = 0;
    vcp = (*vocp).base.vxc_vc;
    offset = 1 as libc::c_int as size_t;
    ninner = 0 as libc::c_int;
    npatched = 0 as libc::c_int;
    ehp = binary as *mut vg_elf32_header_t;
    if size < ::std::mem::size_of::<vg_elf32_header_t>() as libc::c_ulong {
        return 0 as libc::c_int
    } else {
        tmp___0 = memcmp(
            ((*ehp).e_ident).as_mut_ptr() as *const libc::c_void,
            b"\x7FELF\x01\x01\x01\0\0" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            8 as libc::c_int as size_t,
        );
        if tmp___0 != 0 {
            return 0 as libc::c_int
        } else {
            if (*ehp).e_shoff == 0 {
                return 0 as libc::c_int;
            }
        }
    }
    offset = 1 as libc::c_int as size_t;
    while offset < size.wrapping_sub(8 as libc::c_ulong) {
        tmp___1 = memchr(
            binary.offset(offset as isize) as *const libc::c_void,
            127 as libc::c_int,
            size.wrapping_sub(offset),
        );
        ptr = tmp___1 as *mut libc::c_uchar;
        if ptr.is_null() {
            return npatched;
        }
        offset = ptr.offset_from(binary) as libc::c_long as size_t;
        ehp = ptr as *mut vg_elf32_header_t;
        if size.wrapping_sub(offset)
            < ::std::mem::size_of::<vg_elf32_header_t>() as libc::c_ulong
        {
            offset = offset.wrapping_add(1);
        } else {
            tmp___2 = memcmp(
                ((*ehp).e_ident).as_mut_ptr() as *const libc::c_void,
                b"\x7FELF\x01\x01\x01d\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                8 as libc::c_int as size_t,
            );
            if tmp___2 != 0 {
                offset = offset.wrapping_add(1);
            } else if (*ehp).e_shoff == 0 {
                offset = offset.wrapping_add(1);
            } else {
                ninner += 1;
                nrun = vg_ocl_amd_patch_inner(ptr, size.wrapping_sub(offset));
                npatched += nrun;
                if (*vcp).vc_verbose > 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"AMD BFI_INT: patched %d instructions in kernel %d\n\0"
                            as *const u8 as *const libc::c_char,
                        nrun,
                        ninner,
                    );
                }
                npatched += 1;
                offset = offset.wrapping_add(1);
            }
        }
    }
    return npatched;
}
unsafe extern "C" fn vg_ocl_load_program(
    mut vcp: *mut vg_context_t,
    mut vocp: *mut vg_ocl_context_t,
    mut filename: *const libc::c_char,
    mut opts: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut kfp: *mut FILE = 0 as *mut FILE;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut fromsource: libc::c_int = 0;
    let mut patched: libc::c_int = 0;
    let mut sz: size_t = 0;
    let mut szr: size_t = 0;
    let mut prog: cl_program = 0 as *mut _cl_program;
    let mut ret: cl_int = 0;
    let mut sts: cl_int = 0;
    let mut prog_hash: [libc::c_uchar; 16] = [0; 16];
    let mut bin_name: [libc::c_char; 64] = [0; 64];
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    fromsource = 0 as libc::c_int;
    patched = 0 as libc::c_int;
    if (*vcp).vc_verbose > 1 as libc::c_int {
        if !opts.is_null() {
            tmp___0 = opts;
        } else {
            tmp___0 = b"\0" as *const u8 as *const libc::c_char;
        }
        fprintf(
            stderr,
            b"OpenCL compiler flags: %s\n\0" as *const u8 as *const libc::c_char,
            tmp___0,
        );
    }
    sz = 131072 as libc::c_int as size_t;
    tmp___1 = malloc(sz);
    buf = tmp___1 as *mut libc::c_char;
    if buf.is_null() {
        fprintf(
            stderr,
            b"Could not allocate program buffer\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    kfp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if kfp.is_null() {
        tmp___2 = __errno_location();
        tmp___3 = strerror(*tmp___2);
        fprintf(
            stderr,
            b"Error loading kernel file '%s': %s\n\0" as *const u8
                as *const libc::c_char,
            filename,
            tmp___3,
        );
        free(buf as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    tmp___4 = fread(buf as *mut libc::c_void, 1 as libc::c_int as size_t, sz, kfp);
    len = tmp___4 as libc::c_int;
    fclose(kfp);
    if len == 0 {
        fprintf(
            stderr,
            b"Short read on CL kernel\n\0" as *const u8 as *const libc::c_char,
        );
        free(buf as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    vg_ocl_hash_program(
        vocp,
        opts,
        buf as *const libc::c_char,
        len as size_t,
        prog_hash.as_mut_ptr(),
    );
    snprintf(
        bin_name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as _,
        b"%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x.oclbin\0"
            as *const u8 as *const libc::c_char,
        prog_hash[0 as libc::c_int as usize] as libc::c_int,
        prog_hash[1 as libc::c_int as usize] as libc::c_int,
        prog_hash[2 as libc::c_int as usize] as libc::c_int,
        prog_hash[3 as libc::c_int as usize] as libc::c_int,
        prog_hash[4 as libc::c_int as usize] as libc::c_int,
        prog_hash[5 as libc::c_int as usize] as libc::c_int,
        prog_hash[6 as libc::c_int as usize] as libc::c_int,
        prog_hash[7 as libc::c_int as usize] as libc::c_int,
        prog_hash[8 as libc::c_int as usize] as libc::c_int,
        prog_hash[9 as libc::c_int as usize] as libc::c_int,
        prog_hash[10 as libc::c_int as usize] as libc::c_int,
        prog_hash[11 as libc::c_int as usize] as libc::c_int,
        prog_hash[12 as libc::c_int as usize] as libc::c_int,
        prog_hash[13 as libc::c_int as usize] as libc::c_int,
        prog_hash[14 as libc::c_int as usize] as libc::c_int,
        prog_hash[15 as libc::c_int as usize] as libc::c_int,
    );
    if (*vocp).voc_quirks & 128 as libc::c_int != 0 {
        kfp = 0 as *mut libc::c_void as *mut FILE;
        if (*vcp).vc_verbose > 1 as libc::c_int {
            fprintf(
                stderr,
                b"Binary OpenCL programs disabled\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        kfp = fopen(
            bin_name.as_mut_ptr() as *const libc::c_char,
            b"rb\0" as *const u8 as *const libc::c_char,
        );
    }
    if kfp.is_null() {
        fromsource = 1 as libc::c_int;
        sz = len as size_t;
        prog = clCreateProgramWithSource(
            (*vocp).voc_oclctx,
            1 as libc::c_int as cl_uint,
            &mut buf as *mut *mut libc::c_char as *mut *const libc::c_char,
            &mut sz as *mut size_t as *const size_t,
            &mut ret,
        );
        current_block = 5372832139739605200;
    } else {
        if (*vcp).vc_verbose > 1 as libc::c_int {
            fprintf(
                stderr,
                b"Loading kernel binary %s\n\0" as *const u8 as *const libc::c_char,
                bin_name.as_mut_ptr(),
            );
        }
        szr = 0 as libc::c_int as size_t;
        loop {
            tmp___7 = feof(kfp);
            if tmp___7 != 0 {
                break;
            }
            tmp___5 = fread(
                buf.offset(szr as isize) as *mut libc::c_void,
                1 as libc::c_int as size_t,
                sz.wrapping_sub(szr),
                kfp,
            );
            len = tmp___5 as libc::c_int;
            if len == 0 {
                fprintf(
                    stderr,
                    b"Short read on CL kernel binary\n\0" as *const u8
                        as *const libc::c_char,
                );
                fclose(kfp);
                free(buf as *mut libc::c_void);
                return 0 as libc::c_int;
            }
            szr = (szr as libc::c_ulong).wrapping_add(len as size_t) as size_t as size_t;
            if szr == sz {
                tmp___6 = realloc(
                    buf as *mut libc::c_void,
                    sz.wrapping_mul(2 as libc::c_ulong),
                );
                tbuf = tmp___6 as *mut libc::c_char;
                if tbuf.is_null() {
                    fprintf(
                        stderr,
                        b"Could not expand CL kernel binary buffer\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    fclose(kfp);
                    free(buf as *mut libc::c_void);
                    return 0 as libc::c_int;
                }
                buf = tbuf;
                sz = (sz as libc::c_ulong).wrapping_mul(2 as libc::c_ulong) as size_t
                    as size_t;
            }
        }
        fclose(kfp);
        current_block = 8890902903995130642;
    }
    loop {
        match current_block {
            8890902903995130642 => {
                prog = clCreateProgramWithBinary(
                    (*vocp).voc_oclctx,
                    1 as libc::c_int as cl_uint,
                    &mut (*vocp).voc_ocldid as *mut cl_device_id as *const cl_device_id,
                    &mut szr as *mut size_t as *const size_t,
                    &mut buf as *mut *mut libc::c_char as *mut *const libc::c_uchar,
                    &mut sts,
                    &mut ret,
                );
                current_block = 5372832139739605200;
            }
            _ => {
                free(buf as *mut libc::c_void);
                if prog.is_null() {
                    vg_ocl_error(
                        vocp,
                        ret,
                        b"clCreateProgramWithSource\0" as *const u8
                            as *const libc::c_char,
                    );
                    return 0 as libc::c_int;
                }
                if (*vcp).vc_verbose > 0 as libc::c_int {
                    if fromsource != 0 {
                        if patched == 0 {
                            fprintf(
                                stderr,
                                b"Compiling kernel, can take minutes...\0" as *const u8
                                    as *const libc::c_char,
                            );
                            fflush(stderr);
                        }
                    }
                }
                ret = clBuildProgram(
                    prog,
                    1 as libc::c_int as cl_uint,
                    &mut (*vocp).voc_ocldid as *mut cl_device_id as *const cl_device_id,
                    opts,
                    ::std::mem::transmute::<
                        *mut libc::c_void,
                        Option::<
                            unsafe extern "C" fn(cl_program, *mut libc::c_void) -> (),
                        >,
                    >(0 as *mut libc::c_void),
                    0 as *mut libc::c_void,
                );
                if ret != 0 as libc::c_int {
                    if (*vcp).vc_verbose > 0 as libc::c_int {
                        if fromsource != 0 {
                            if patched == 0 {
                                fprintf(
                                    stderr,
                                    b"failure.\n\0" as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                    }
                    vg_ocl_error(
                        0 as *mut libc::c_void as *mut vg_ocl_context_t,
                        ret,
                        b"clBuildProgram\0" as *const u8 as *const libc::c_char,
                    );
                } else if (*vcp).vc_verbose > 0 as libc::c_int {
                    if fromsource != 0 {
                        if patched == 0 {
                            fprintf(
                                stderr,
                                b"done!\n\0" as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
                if ret != 0 as libc::c_int {
                    vg_ocl_buildlog(vocp, prog);
                } else if (*vcp).vc_verbose > 1 as libc::c_int {
                    if fromsource != 0 {
                        if patched == 0 {
                            vg_ocl_buildlog(vocp, prog);
                        }
                    }
                }
                if ret != 0 as libc::c_int {
                    vg_ocl_dump_info(vocp);
                    clReleaseProgram(prog);
                    return 0 as libc::c_int;
                }
                if !(fromsource != 0) {
                    current_block = 13988745531296555351;
                    break;
                }
                if !((*vocp).voc_quirks & 128 as libc::c_int == 0) {
                    current_block = 13988745531296555351;
                    break;
                }
                ret = clGetProgramInfo(
                    prog,
                    4453 as libc::c_int as cl_program_info,
                    ::std::mem::size_of::<size_t>() as libc::c_ulong,
                    &mut szr as *mut size_t as *mut libc::c_void,
                    &mut sz,
                );
                if ret != 0 as libc::c_int {
                    vg_ocl_error(
                        vocp,
                        ret,
                        b"WARNING: clGetProgramInfo(BINARY_SIZES)\0" as *const u8
                            as *const libc::c_char,
                    );
                    current_block = 13988745531296555351;
                    break;
                } else if sz == 0 as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"WARNING: zero-length CL kernel binary\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    current_block = 13988745531296555351;
                    break;
                } else {
                    tmp___8 = malloc(szr);
                    buf = tmp___8 as *mut libc::c_char;
                    if buf.is_null() {
                        fprintf(
                            stderr,
                            b"WARNING: Could not allocate %zd bytes for CL binary\n\0"
                                as *const u8 as *const libc::c_char,
                            szr,
                        );
                        current_block = 13988745531296555351;
                        break;
                    } else {
                        ret = clGetProgramInfo(
                            prog,
                            4454 as libc::c_int as cl_program_info,
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                            &mut buf as *mut *mut libc::c_char as *mut libc::c_void,
                            &mut sz,
                        );
                        if ret != 0 as libc::c_int {
                            vg_ocl_error(
                                vocp,
                                ret,
                                b"WARNING: clGetProgramInfo(BINARIES)\0" as *const u8
                                    as *const libc::c_char,
                            );
                            free(buf as *mut libc::c_void);
                            current_block = 13988745531296555351;
                            break;
                        } else {
                            if !((*vocp).voc_quirks & 16 as libc::c_int != 0) {
                                current_block = 4485073238441121731;
                                break;
                            }
                            if !(patched == 0) {
                                current_block = 4485073238441121731;
                                break;
                            }
                            patched = vg_ocl_amd_patch(
                                vocp,
                                buf as *mut libc::c_uchar,
                                szr,
                            );
                            if patched > 0 as libc::c_int {
                                if (*vcp).vc_verbose > 1 as libc::c_int {
                                    fprintf(
                                        stderr,
                                        b"AMD BFI_INT patch complete\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                }
                                clReleaseProgram(prog);
                                current_block = 8890902903995130642;
                            } else {
                                fprintf(
                                    stderr,
                                    b"WARNING: AMD BFI_INT patching failed\n\0" as *const u8
                                        as *const libc::c_char,
                                );
                                if patched < 0 as libc::c_int {
                                    current_block = 1604201581803946138;
                                    break;
                                } else {
                                    current_block = 4485073238441121731;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        4485073238441121731 => {
            kfp = fopen(
                bin_name.as_mut_ptr() as *const libc::c_char,
                b"wb\0" as *const u8 as *const libc::c_char,
            );
            if kfp.is_null() {
                tmp___9 = __errno_location();
                tmp___10 = strerror(*tmp___9);
                fprintf(
                    stderr,
                    b"WARNING: could not save CL kernel binary: %s\n\0" as *const u8
                        as *const libc::c_char,
                    tmp___10,
                );
            } else {
                sz = fwrite(
                    buf as *const libc::c_void,
                    1 as libc::c_int as size_t,
                    szr,
                    kfp,
                );
                fclose(kfp);
                if sz != szr {
                    fprintf(
                        stderr,
                        b"WARNING: short write on CL kernel binary file: expected %zd, got %zd\n\0"
                            as *const u8 as *const libc::c_char,
                        szr,
                        sz,
                    );
                    unlink(bin_name.as_mut_ptr() as *const libc::c_char);
                }
            }
            free(buf as *mut libc::c_void);
        }
        1604201581803946138 => {
            free(buf as *mut libc::c_void);
        }
        _ => {}
    }
    (*vocp).voc_oclprog = prog;
    tmp___11 = vg_ocl_create_kernel(
        vocp,
        0 as libc::c_int,
        b"ec_add_grid\0" as *const u8 as *const libc::c_char,
    );
    if tmp___11 != 0 {
        tmp___12 = vg_ocl_create_kernel(
            vocp,
            1 as libc::c_int,
            b"heap_invert\0" as *const u8 as *const libc::c_char,
        );
        if tmp___12 == 0 {
            clReleaseProgram((*vocp).voc_oclprog);
            (*vocp).voc_oclprog = 0 as *mut libc::c_void as cl_program;
            return 0 as libc::c_int;
        }
    } else {
        clReleaseProgram((*vocp).voc_oclprog);
        (*vocp).voc_oclprog = 0 as *mut libc::c_void as cl_program;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_ocl_context_callback(
    mut errinfo: *const libc::c_char,
    mut private_info: *const libc::c_void,
    mut cb: size_t,
    mut user_data: *mut libc::c_void,
) {
    fprintf(
        stderr,
        b"vg_ocl_context_callback error: %s\n\0" as *const u8 as *const libc::c_char,
        errinfo,
    );
}
unsafe extern "C" fn vg_ocl_init(
    mut vcp: *mut vg_context_t,
    mut vocp: *mut vg_ocl_context_t,
    mut did: cl_device_id,
    mut safe_mode: libc::c_int,
) -> libc::c_int {
    let mut ret: cl_int = 0;
    let mut optbuf: [libc::c_char; 128] = [0; 128];
    let mut end: libc::c_int = 0;
    let mut yesbuf: [libc::c_char; 16] = [0; 16];
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    end = 0 as libc::c_int;
    memset(
        vocp as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<vg_ocl_context_t>() as libc::c_ulong as _,
    );
    vg_exec_context_init(vcp, &mut (*vocp).base);
    (*vocp)
        .base
        .vxc_threadfunc = Some(
        vg_opencl_loop
            as unsafe extern "C" fn(*mut vg_exec_context_t) -> *mut libc::c_void,
    );
    pthread_mutex_init(
        &mut (*vocp).voc_lock,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    pthread_cond_init(
        &mut (*vocp).voc_wait as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    (*vocp).voc_ocl_slot = -(1 as libc::c_int);
    (*vocp).voc_ocldid = did;
    if (*vcp).vc_verbose > 1 as libc::c_int {
        vg_ocl_dump_info(vocp);
    }
    (*vocp).voc_quirks = vg_ocl_get_quirks(vocp);
    if (*vocp).voc_quirks & 64 as libc::c_int != 0 {
        if (*vcp).vc_verbose > 0 as libc::c_int {
            printf(b"Type 'yes' to continue: \0" as *const u8 as *const libc::c_char);
            fflush(stdout);
            tmp___0 = fgets(
                yesbuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong
                    as libc::c_int,
                stdin,
            );
            if !tmp___0.is_null() {
                tmp___1 = strncmp(
                    yesbuf.as_mut_ptr() as *const libc::c_char,
                    b"yes\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as size_t,
                );
                if tmp___1 != 0 {
                    exit(1 as libc::c_int);
                }
            } else {
                exit(1 as libc::c_int);
            }
        }
    }
    (*vocp)
        .voc_oclctx = clCreateContext(
        0 as *mut libc::c_void as *const cl_context_properties,
        1 as libc::c_int as cl_uint,
        &mut did as *mut cl_device_id as *const cl_device_id,
        Some(
            vg_ocl_context_callback
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_void,
                    size_t,
                    *mut libc::c_void,
                ) -> (),
        ),
        0 as *mut libc::c_void,
        &mut ret,
    );
    if ((*vocp).voc_oclctx).is_null() {
        vg_ocl_error(
            vocp,
            ret,
            b"clCreateContext\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    (*vocp)
        .voc_oclcmdq = clCreateCommandQueue(
        (*vocp).voc_oclctx,
        (*vocp).voc_ocldid,
        0 as libc::c_int as cl_command_queue_properties,
        &mut ret,
    );
    if ((*vocp).voc_oclcmdq).is_null() {
        vg_ocl_error(
            vocp,
            ret,
            b"clCreateCommandQueue\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if safe_mode != 0 {
        (*vocp).voc_quirks &= -(32 as libc::c_int);
    }
    end = 0 as libc::c_int;
    optbuf[end as usize] = '\u{0}' as i32 as libc::c_char;
    if (*vocp).voc_quirks & 1 as libc::c_int != 0 {
        tmp___2 = snprintf(
            optbuf.as_mut_ptr().offset(end as isize),
            (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                .wrapping_sub(end as libc::c_ulong) as _,
            b"-DDEEP_PREPROC_UNROLL \0" as *const u8 as *const libc::c_char,
        );
        end += tmp___2;
    }
    if (*vocp).voc_quirks & 2 as libc::c_int != 0 {
        tmp___3 = snprintf(
            optbuf.as_mut_ptr().offset(end as isize),
            (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                .wrapping_sub(end as libc::c_ulong) as _,
            b"-DPRAGMA_UNROLL \0" as *const u8 as *const libc::c_char,
        );
        end += tmp___3;
    }
    if (*vocp).voc_quirks & 4 as libc::c_int != 0 {
        tmp___4 = snprintf(
            optbuf.as_mut_ptr().offset(end as isize),
            (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                .wrapping_sub(end as libc::c_ulong) as _,
            b"-DVERY_EXPENSIVE_BRANCHES \0" as *const u8 as *const libc::c_char,
        );
        end += tmp___4;
    }
    if (*vocp).voc_quirks & 8 as libc::c_int != 0 {
        tmp___5 = snprintf(
            optbuf.as_mut_ptr().offset(end as isize),
            (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                .wrapping_sub(end as libc::c_ulong) as _,
            b"-DDEEP_VLIW \0" as *const u8 as *const libc::c_char,
        );
        end += tmp___5;
    }
    if (*vocp).voc_quirks & 16 as libc::c_int != 0 {
        tmp___6 = snprintf(
            optbuf.as_mut_ptr().offset(end as isize),
            (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                .wrapping_sub(end as libc::c_ulong) as _,
            b"-DAMD_BFI_INT \0" as *const u8 as *const libc::c_char,
        );
        end += tmp___6;
    }
    if (*vocp).voc_quirks & 32 as libc::c_int != 0 {
        tmp___7 = snprintf(
            optbuf.as_mut_ptr().offset(end as isize),
            (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                .wrapping_sub(end as libc::c_ulong) as _,
            b"-cl-nv-verbose \0" as *const u8 as *const libc::c_char,
        );
        end += tmp___7;
    }
    tmp___8 = vg_ocl_load_program(
        vcp,
        vocp,
        b"calc_addrs.cl\0" as *const u8 as *const libc::c_char,
        optbuf.as_mut_ptr() as *const libc::c_char,
    );
    if tmp___8 == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_ocl_del(mut vocp: *mut vg_ocl_context_t) {
    vg_ocl_free_args(vocp);
    if !((*vocp).voc_oclprog).is_null() {
        clReleaseProgram((*vocp).voc_oclprog);
        (*vocp).voc_oclprog = 0 as *mut libc::c_void as cl_program;
    }
    if !((*vocp).voc_oclcmdq).is_null() {
        clReleaseCommandQueue((*vocp).voc_oclcmdq);
        (*vocp).voc_oclcmdq = 0 as *mut libc::c_void as cl_command_queue;
    }
    if !((*vocp).voc_oclctx).is_null() {
        clReleaseContext((*vocp).voc_oclctx);
        (*vocp).voc_oclctx = 0 as *mut libc::c_void as cl_context;
    }
    pthread_cond_destroy(&mut (*vocp).voc_wait);
    pthread_mutex_destroy(&mut (*vocp).voc_lock);
    vg_exec_context_del(&mut (*vocp).base);
}
static mut vg_ocl_arg_map: [[libc::c_int; 8]; 6] = [
    [2 as libc::c_int, 0 as libc::c_int, -(1 as libc::c_int), 0, 0, 0, 0, 0],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        -(1 as libc::c_int),
        0,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        2 as libc::c_int,
        1 as libc::c_int,
        -(1 as libc::c_int),
        0,
        0,
        0,
    ],
    [0 as libc::c_int, 2 as libc::c_int, -(1 as libc::c_int), 0, 0, 0, 0, 0],
    [0 as libc::c_int, 3 as libc::c_int, -(1 as libc::c_int), 0, 0, 0, 0, 0],
    [2 as libc::c_int, 3 as libc::c_int, -(1 as libc::c_int), 0, 0, 0, 0, 0],
];
unsafe extern "C" fn vg_ocl_kernel_arg_alloc(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
    mut arg: libc::c_int,
    mut size: size_t,
    mut host: libc::c_int,
) -> libc::c_int {
    let mut clbuf: cl_mem = 0 as *mut _cl_mem;
    let mut ret: cl_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut knum: libc::c_int = 0;
    let mut karg: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut current_block_6: u64;
        if i != slot {
            if slot >= 0 as libc::c_int {
                current_block_6 = 6174466495384644892;
            } else {
                current_block_6 = 10879442775620481940;
            }
        } else {
            current_block_6 = 10879442775620481940;
        }
        match current_block_6 {
            10879442775620481940 => {
                if !((*vocp).voc_args[i as usize][arg as usize]).is_null() {
                    clReleaseMemObject((*vocp).voc_args[i as usize][arg as usize]);
                    (*vocp)
                        .voc_args[i
                        as usize][arg as usize] = 0 as *mut libc::c_void as cl_mem;
                    (*vocp)
                        .voc_arg_size[i
                        as usize][arg as usize] = 0 as libc::c_int as size_t;
                }
            }
            _ => {}
        }
        i += 1;
    }
    if host != 0 {
        tmp___0 = (1 as libc::c_int) << 4 as libc::c_int;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    clbuf = clCreateBuffer(
        (*vocp).voc_oclctx,
        (1 as libc::c_int | tmp___0) as cl_mem_flags,
        size,
        0 as *mut libc::c_void,
        &mut ret,
    );
    if clbuf.is_null() {
        fprintf(
            stderr,
            b"clCreateBuffer(%d,%d): \0" as *const u8 as *const libc::c_char,
            slot,
            arg,
        );
        vg_ocl_error(vocp, ret, 0 as *mut libc::c_void as *const libc::c_char);
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut current_block_36: u64;
        if i != slot {
            if slot >= 0 as libc::c_int {
                current_block_36 = 17200130555324103459;
            } else {
                current_block_36 = 2719512138335094285;
            }
        } else {
            current_block_36 = 2719512138335094285;
        }
        match current_block_36 {
            2719512138335094285 => {
                clRetainMemObject(clbuf);
                (*vocp).voc_args[i as usize][arg as usize] = clbuf;
                (*vocp).voc_arg_size[i as usize][arg as usize] = size;
                j = 0 as libc::c_int;
                while vg_ocl_arg_map[arg as usize][j as usize] >= 0 as libc::c_int {
                    knum = vg_ocl_arg_map[arg as usize][j as usize];
                    karg = vg_ocl_arg_map[arg as usize][(j + 1 as libc::c_int) as usize];
                    ret = clSetKernelArg(
                        (*vocp).voc_oclkernel[i as usize][knum as usize],
                        karg as cl_uint,
                        ::std::mem::size_of::<cl_mem>() as libc::c_ulong,
                        &mut clbuf as *mut cl_mem as *const libc::c_void,
                    );
                    if ret != 0 {
                        fprintf(
                            stderr,
                            b"clSetKernelArg(%d,%d): \0" as *const u8
                                as *const libc::c_char,
                            knum,
                            karg,
                        );
                        vg_ocl_error(
                            vocp,
                            ret,
                            0 as *mut libc::c_void as *const libc::c_char,
                        );
                        return 0 as libc::c_int;
                    }
                    j += 2 as libc::c_int;
                }
            }
            _ => {}
        }
        i += 1;
    }
    clReleaseMemObject(clbuf);
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_ocl_copyout_arg(
    mut vocp: *mut vg_ocl_context_t,
    mut wslot: libc::c_int,
    mut arg: libc::c_int,
    mut buffer: *mut libc::c_void,
    mut size: size_t,
) -> libc::c_int {
    let mut slot: cl_int = 0;
    let mut ret: cl_int = 0;
    if wslot < 0 as libc::c_int {
        slot = 0 as libc::c_int;
    } else {
        slot = wslot;
    }
    if slot >= 0 as libc::c_int {
        if !(slot < 2 as libc::c_int) {
            __assert_fail(
                b"(slot >= 0) && (slot < MAX_SLOT)\0" as *const u8
                    as *const libc::c_char,
                b"oclengine.c\0" as *const u8 as *const libc::c_char,
                1048 as libc::c_uint,
                b"vg_ocl_copyout_arg\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"(slot >= 0) && (slot < MAX_SLOT)\0" as *const u8 as *const libc::c_char,
            b"oclengine.c\0" as *const u8 as *const libc::c_char,
            1048 as libc::c_uint,
            b"vg_ocl_copyout_arg\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(size <= (*vocp).voc_arg_size[slot as usize][arg as usize]) {
        __assert_fail(
            b"size <= vocp->voc_arg_size[slot][arg]\0" as *const u8
                as *const libc::c_char,
            b"oclengine.c\0" as *const u8 as *const libc::c_char,
            1049 as libc::c_uint,
            b"vg_ocl_copyout_arg\0" as *const u8 as *const libc::c_char,
        );
    }
    ret = clEnqueueWriteBuffer(
        (*vocp).voc_oclcmdq,
        (*vocp).voc_args[slot as usize][arg as usize],
        1 as libc::c_int as cl_bool,
        0 as libc::c_int as size_t,
        size,
        buffer as *const libc::c_void,
        0 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *const cl_event,
        0 as *mut libc::c_void as *mut cl_event,
    );
    if ret != 0 {
        fprintf(
            stderr,
            b"clEnqueueWriteBuffer(%d): \0" as *const u8 as *const libc::c_char,
            arg,
        );
        vg_ocl_error(vocp, ret, 0 as *mut libc::c_void as *const libc::c_char);
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_ocl_map_arg_buffer(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
    mut arg: libc::c_int,
    mut rw: libc::c_int,
) -> *mut libc::c_void {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ret: cl_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    if slot >= 0 as libc::c_int {
        if !(slot < 2 as libc::c_int) {
            __assert_fail(
                b"(slot >= 0) && (slot < MAX_SLOT)\0" as *const u8
                    as *const libc::c_char,
                b"oclengine.c\0" as *const u8 as *const libc::c_char,
                1075 as libc::c_uint,
                b"vg_ocl_map_arg_buffer\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"(slot >= 0) && (slot < MAX_SLOT)\0" as *const u8 as *const libc::c_char,
            b"oclengine.c\0" as *const u8 as *const libc::c_char,
            1075 as libc::c_uint,
            b"vg_ocl_map_arg_buffer\0" as *const u8 as *const libc::c_char,
        );
    }
    if rw == 2 as libc::c_int {
        tmp___2 = 1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int;
    } else {
        if rw != 0 {
            tmp___1 = (1 as libc::c_int) << 1 as libc::c_int;
        } else {
            tmp___1 = 1 as libc::c_int;
        }
        tmp___2 = tmp___1;
    }
    buf = clEnqueueMapBuffer(
        (*vocp).voc_oclcmdq,
        (*vocp).voc_args[slot as usize][arg as usize],
        1 as libc::c_int as cl_bool,
        tmp___2 as cl_map_flags,
        0 as libc::c_int as size_t,
        (*vocp).voc_arg_size[slot as usize][arg as usize],
        0 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *const cl_event,
        0 as *mut libc::c_void as *mut cl_event,
        &mut ret,
    );
    if buf.is_null() {
        fprintf(
            stderr,
            b"clEnqueueMapBuffer(%d): \0" as *const u8 as *const libc::c_char,
            arg,
        );
        vg_ocl_error(vocp, ret, 0 as *mut libc::c_void as *const libc::c_char);
        return 0 as *mut libc::c_void;
    }
    return buf;
}
unsafe extern "C" fn vg_ocl_unmap_arg_buffer(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
    mut arg: libc::c_int,
    mut buf: *mut libc::c_void,
) {
    let mut ret: cl_int = 0;
    let mut ev: cl_event = 0 as *mut _cl_event;
    if slot >= 0 as libc::c_int {
        if !(slot < 2 as libc::c_int) {
            __assert_fail(
                b"(slot >= 0) && (slot < MAX_SLOT)\0" as *const u8
                    as *const libc::c_char,
                b"oclengine.c\0" as *const u8 as *const libc::c_char,
                1101 as libc::c_uint,
                b"vg_ocl_unmap_arg_buffer\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"(slot >= 0) && (slot < MAX_SLOT)\0" as *const u8 as *const libc::c_char,
            b"oclengine.c\0" as *const u8 as *const libc::c_char,
            1101 as libc::c_uint,
            b"vg_ocl_unmap_arg_buffer\0" as *const u8 as *const libc::c_char,
        );
    }
    ret = clEnqueueUnmapMemObject(
        (*vocp).voc_oclcmdq,
        (*vocp).voc_args[slot as usize][arg as usize],
        buf,
        0 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *const cl_event,
        &mut ev,
    );
    if ret != 0 as libc::c_int {
        fprintf(
            stderr,
            b"clEnqueueUnmapMemObject(%d): \0" as *const u8 as *const libc::c_char,
            arg,
        );
        vg_ocl_error(vocp, ret, 0 as *mut libc::c_void as *const libc::c_char);
        return;
    }
    ret = clWaitForEvents(
        1 as libc::c_int as cl_uint,
        &mut ev as *mut cl_event as *const cl_event,
    );
    clReleaseEvent(ev);
    if ret != 0 as libc::c_int {
        fprintf(
            stderr,
            b"clWaitForEvent(clUnmapMemObject,%d): \0" as *const u8
                as *const libc::c_char,
            arg,
        );
        vg_ocl_error(vocp, ret, 0 as *mut libc::c_void as *const libc::c_char);
    }
}
pub unsafe extern "C" fn vg_ocl_kernel_int_arg(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
    mut arg: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut ret: cl_int = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut current_block_7: u64;
        if i != slot {
            if slot >= 0 as libc::c_int {
                current_block_7 = 2680634958049926135;
            } else {
                current_block_7 = 820271813250567934;
            }
        } else {
            current_block_7 = 820271813250567934;
        }
        match current_block_7 {
            820271813250567934 => {
                ret = clSetKernelArg(
                    (*vocp).voc_oclkernel[i as usize][2 as libc::c_int as usize],
                    arg as cl_uint,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    &mut value as *mut libc::c_int as *const libc::c_void,
                );
                if ret != 0 {
                    fprintf(
                        stderr,
                        b"clSetKernelArg(%d): \0" as *const u8 as *const libc::c_char,
                        arg,
                    );
                    vg_ocl_error(
                        vocp,
                        ret,
                        0 as *mut libc::c_void as *const libc::c_char,
                    );
                    return 0 as libc::c_int;
                }
            }
            _ => {}
        }
        i += 1;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn vg_ocl_kernel_buffer_arg(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
    mut arg: libc::c_int,
    mut value: *mut libc::c_void,
    mut size: size_t,
) -> libc::c_int {
    let mut ret: cl_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut knum: libc::c_int = 0;
    let mut karg: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        let mut current_block_13: u64;
        if i != slot {
            if slot >= 0 as libc::c_int {
                current_block_13 = 3622095589389319910;
            } else {
                current_block_13 = 15619007995458559411;
            }
        } else {
            current_block_13 = 15619007995458559411;
        }
        match current_block_13 {
            15619007995458559411 => {
                j = 0 as libc::c_int;
                while vg_ocl_arg_map[arg as usize][j as usize] >= 0 as libc::c_int {
                    knum = vg_ocl_arg_map[arg as usize][j as usize];
                    karg = vg_ocl_arg_map[arg as usize][(j + 1 as libc::c_int) as usize];
                    ret = clSetKernelArg(
                        (*vocp).voc_oclkernel[i as usize][knum as usize],
                        karg as cl_uint,
                        size,
                        value as *const libc::c_void,
                    );
                    if ret != 0 {
                        fprintf(
                            stderr,
                            b"clSetKernelArg(%d,%d): \0" as *const u8
                                as *const libc::c_char,
                            knum,
                            karg,
                        );
                        vg_ocl_error(
                            vocp,
                            ret,
                            0 as *mut libc::c_void as *const libc::c_char,
                        );
                        return 0 as libc::c_int;
                    }
                    j += 2 as libc::c_int;
                }
            }
            _ => {}
        }
        i += 1;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_ocl_free_args(mut vocp: *mut vg_ocl_context_t) {
    let mut i: libc::c_int = 0;
    let mut arg: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        arg = 0 as libc::c_int;
        while arg < 6 as libc::c_int {
            if !((*vocp).voc_args[i as usize][arg as usize]).is_null() {
                clReleaseMemObject((*vocp).voc_args[i as usize][arg as usize]);
                (*vocp)
                    .voc_args[i
                    as usize][arg as usize] = 0 as *mut libc::c_void as cl_mem;
                (*vocp)
                    .voc_arg_size[i as usize][arg as usize] = 0 as libc::c_int as size_t;
            }
            arg += 1;
        }
        i += 1;
    }
}
pub unsafe extern "C" fn vg_ocl_kernel_dead(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
) -> libc::c_int {
    return ((*vocp).voc_oclkrnwait[slot as usize] as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn vg_ocl_kernel_start(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
    mut ncol: libc::c_int,
    mut nrow: libc::c_int,
    mut invsize: libc::c_int,
) -> libc::c_int {
    let mut val: cl_int = 0;
    let mut ret: cl_int = 0;
    let mut ev: cl_event = 0 as *mut _cl_event;
    let mut globalws: [size_t; 2] = [0; 2];
    let mut invws: size_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    globalws[0 as libc::c_int as usize] = ncol as size_t;
    globalws[1 as libc::c_int as usize] = nrow as size_t;
    invws = (ncol * nrow / invsize) as size_t;
    if !((*vocp).voc_oclkrnwait[slot as usize]).is_null() {
        __assert_fail(
            b"!vocp->voc_oclkrnwait[slot]\0" as *const u8 as *const libc::c_char,
            b"oclengine.c\0" as *const u8 as *const libc::c_char,
            1203 as libc::c_uint,
            b"vg_ocl_kernel_start\0" as *const u8 as *const libc::c_char,
        );
    }
    if invsize & invsize - 1 as libc::c_int == 0 {
        if !(invsize > 1 as libc::c_int) {
            __assert_fail(
                b"is_pow2(invsize) && (invsize > 1)\0" as *const u8
                    as *const libc::c_char,
                b"oclengine.c\0" as *const u8 as *const libc::c_char,
                1206 as libc::c_uint,
                b"vg_ocl_kernel_start\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"is_pow2(invsize) && (invsize > 1)\0" as *const u8 as *const libc::c_char,
            b"oclengine.c\0" as *const u8 as *const libc::c_char,
            1206 as libc::c_uint,
            b"vg_ocl_kernel_start\0" as *const u8 as *const libc::c_char,
        );
    }
    val = invsize;
    ret = clSetKernelArg(
        (*vocp).voc_oclkernel[slot as usize][1 as libc::c_int as usize],
        1 as libc::c_int as cl_uint,
        ::std::mem::size_of::<cl_int>() as libc::c_ulong,
        &mut val as *mut cl_int as *const libc::c_void,
    );
    if ret != 0 as libc::c_int {
        vg_ocl_error(
            vocp,
            ret,
            b"clSetKernelArg(ncol)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    ret = clEnqueueNDRangeKernel(
        (*vocp).voc_oclcmdq,
        (*vocp).voc_oclkernel[slot as usize][0 as libc::c_int as usize],
        2 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *const size_t,
        globalws.as_mut_ptr() as *const size_t,
        0 as *mut libc::c_void as *const size_t,
        0 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *const cl_event,
        &mut ev,
    );
    if ret != 0 as libc::c_int {
        vg_ocl_error(
            vocp,
            ret,
            b"clEnqueueNDRange(0)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    ret = clWaitForEvents(
        1 as libc::c_int as cl_uint,
        &mut ev as *mut cl_event as *const cl_event,
    );
    clReleaseEvent(ev);
    if ret != 0 as libc::c_int {
        vg_ocl_error(
            vocp,
            ret,
            b"clWaitForEvents(NDRange,0)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ((*vocp).voc_verify_func[0 as libc::c_int as usize]).is_some() {
        tmp___2 = (Some(
            (*((*vocp).voc_verify_func).as_mut_ptr().offset(0 as libc::c_int as isize))
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(vocp, slot);
        if tmp___2 == 0 {
            fprintf(
                stderr,
                b"ERROR: Kernel 0 failed verification test\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    ret = clEnqueueNDRangeKernel(
        (*vocp).voc_oclcmdq,
        (*vocp).voc_oclkernel[slot as usize][1 as libc::c_int as usize],
        1 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *const size_t,
        &mut invws as *mut size_t as *const size_t,
        0 as *mut libc::c_void as *const size_t,
        0 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *const cl_event,
        &mut ev,
    );
    if ret != 0 as libc::c_int {
        vg_ocl_error(
            vocp,
            ret,
            b"clEnqueueNDRange(1)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    ret = clWaitForEvents(
        1 as libc::c_int as cl_uint,
        &mut ev as *mut cl_event as *const cl_event,
    );
    clReleaseEvent(ev);
    if ret != 0 as libc::c_int {
        vg_ocl_error(
            vocp,
            ret,
            b"clWaitForEvents(NDRange,1)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ((*vocp).voc_verify_func[1 as libc::c_int as usize]).is_some() {
        tmp___3 = (Some(
            (*((*vocp).voc_verify_func).as_mut_ptr().offset(1 as libc::c_int as isize))
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(vocp, slot);
        if tmp___3 == 0 {
            fprintf(
                stderr,
                b"ERROR: Kernel 1 failed verification test\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    ret = clEnqueueNDRangeKernel(
        (*vocp).voc_oclcmdq,
        (*vocp).voc_oclkernel[slot as usize][2 as libc::c_int as usize],
        2 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *const size_t,
        globalws.as_mut_ptr() as *const size_t,
        0 as *mut libc::c_void as *const size_t,
        0 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *const cl_event,
        &mut ev,
    );
    if ret != 0 as libc::c_int {
        vg_ocl_error(
            vocp,
            ret,
            b"clEnqueueNDRange(2)\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    (*vocp).voc_oclkrnwait[slot as usize] = ev;
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_ocl_kernel_wait(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
) -> libc::c_int {
    let mut ev: cl_event = 0 as *mut _cl_event;
    let mut ret: cl_int = 0;
    ev = (*vocp).voc_oclkrnwait[slot as usize];
    (*vocp).voc_oclkrnwait[slot as usize] = 0 as *mut libc::c_void as cl_event;
    if !ev.is_null() {
        ret = clWaitForEvents(
            1 as libc::c_int as cl_uint,
            &mut ev as *mut cl_event as *const cl_event,
        );
        clReleaseEvent(ev);
        if ret != 0 as libc::c_int {
            vg_ocl_error(
                vocp,
                ret,
                b"clWaitForEvents(NDRange,e)\0" as *const u8 as *const libc::c_char,
            );
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn vg_ocl_get_bignum_raw(
    mut bn: *mut BIGNUM,
    mut buf: *const libc::c_uchar,
) {
    if !(4 as libc::c_int <= (*bn).dmax) {
        bn_expand2(bn, 4 as libc::c_int);
    }
    memcpy(
        (*bn).d as *mut libc::c_void,
        buf as *const libc::c_void,
        32 as libc::c_int as size_t as _,
    );
    (*bn)
        .top = (32 as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn vg_ocl_put_bignum_raw(
    mut buf: *mut libc::c_uchar,
    mut bn: *const BIGNUM,
) {
    let mut bnlen: libc::c_int = 0;
    bnlen = ((*bn).top as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        as libc::c_int;
    if bnlen >= 32 as libc::c_int {
        memcpy(
            buf as *mut libc::c_void,
            (*bn).d as *const libc::c_void,
            32 as libc::c_int as size_t as _,
        );
    } else {
        memcpy(
            buf as *mut libc::c_void,
            (*bn).d as *const libc::c_void,
            bnlen as size_t as _,
        );
        memset(
            buf.offset(bnlen as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_int - bnlen) as size_t as _,
        );
    };
}
unsafe extern "C" fn vg_ocl_get_bignum_tpa(
    mut bn: *mut BIGNUM,
    mut buf: *const libc::c_uchar,
    mut cell: libc::c_int,
) {
    let mut bnbuf: [libc::c_uchar; 32] = [0; 32];
    let mut start: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    start = cell / 128 as libc::c_int * 1024 as libc::c_int + cell % 128 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        memcpy(
            bnbuf.as_mut_ptr().offset((i * 4 as libc::c_int) as isize)
                as *mut libc::c_void,
            buf.offset((4 as libc::c_int * (start + i * 128 as libc::c_int)) as isize)
                as *const libc::c_void,
            4 as libc::c_int as size_t as _,
        );
        i += 1;
    }
    vg_ocl_get_bignum_raw(bn, bnbuf.as_mut_ptr() as *const libc::c_uchar);
}
static mut mont_one: [libc::c_uchar; 5] = [
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    209 as libc::c_int as libc::c_uchar,
];
#[inline]
unsafe extern "C" fn vg_ocl_get_point(
    mut ppnt: *mut EC_POINT,
    mut buf: *const libc::c_uchar,
) {
    vg_ocl_get_bignum_raw(&mut (*ppnt).X, buf);
    vg_ocl_get_bignum_raw(&mut (*ppnt).Y, buf.offset(32 as libc::c_int as isize));
    if (*ppnt).Z_is_one == 0 {
        (*ppnt).Z_is_one = 1 as libc::c_int;
        BN_bin2bn(
            mont_one.as_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 5]>() as libc::c_ulong as libc::c_int,
            &mut (*ppnt).Z,
        );
    }
}
#[inline]
unsafe extern "C" fn vg_ocl_put_point(
    mut buf: *mut libc::c_uchar,
    mut ppnt: *const EC_POINT,
) {
    if (*ppnt).Z_is_one == 0 {
        __assert_fail(
            b"ppnt->Z_is_one\0" as *const u8 as *const libc::c_char,
            b"oclengine.c\0" as *const u8 as *const libc::c_char,
            1368 as libc::c_uint,
            b"vg_ocl_put_point\0" as *const u8 as *const libc::c_char,
        );
    }
    vg_ocl_put_bignum_raw(buf, &(*ppnt).X);
    vg_ocl_put_bignum_raw(buf.offset(32 as libc::c_int as isize), &(*ppnt).Y);
}
unsafe extern "C" fn vg_ocl_put_point_tpa(
    mut buf: *mut libc::c_uchar,
    mut cell: libc::c_int,
    mut ppnt: *const EC_POINT,
) {
    let mut pntbuf: [libc::c_uchar; 64] = [0; 64];
    let mut start: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    vg_ocl_put_point(pntbuf.as_mut_ptr(), ppnt);
    start = 2 as libc::c_int * cell / 128 as libc::c_int * 1024 as libc::c_int
        + cell % 64 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        memcpy(
            buf.offset((4 as libc::c_int * (start + i * 128 as libc::c_int)) as isize)
                as *mut libc::c_void,
            pntbuf.as_mut_ptr().offset((i * 4 as libc::c_int) as isize)
                as *const libc::c_void,
            4 as libc::c_int as size_t as _,
        );
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        memcpy(
            buf
                .offset(
                    (4 as libc::c_int
                        * (start + 64 as libc::c_int + i * 128 as libc::c_int)) as isize,
                ) as *mut libc::c_void,
            pntbuf
                .as_mut_ptr()
                .offset(32 as libc::c_int as isize)
                .offset((i * 4 as libc::c_int) as isize) as *const libc::c_void,
            4 as libc::c_int as size_t as _,
        );
        i += 1;
    }
}
unsafe extern "C" fn vg_ocl_get_point_tpa(
    mut ppnt: *mut EC_POINT,
    mut buf: *const libc::c_uchar,
    mut cell: libc::c_int,
) {
    let mut pntbuf: [libc::c_uchar; 64] = [0; 64];
    let mut start: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    start = 2 as libc::c_int * cell / 128 as libc::c_int * 1024 as libc::c_int
        + cell % 64 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        memcpy(
            pntbuf.as_mut_ptr().offset((i * 4 as libc::c_int) as isize)
                as *mut libc::c_void,
            buf.offset((4 as libc::c_int * (start + i * 128 as libc::c_int)) as isize)
                as *const libc::c_void,
            4 as libc::c_int as size_t as _,
        );
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        memcpy(
            pntbuf
                .as_mut_ptr()
                .offset(32 as libc::c_int as isize)
                .offset((i * 4 as libc::c_int) as isize) as *mut libc::c_void,
            buf
                .offset(
                    (4 as libc::c_int
                        * (start + 64 as libc::c_int + i * 128 as libc::c_int)) as isize,
                ) as *const libc::c_void,
            4 as libc::c_int as size_t as _,
        );
        i += 1;
    }
    vg_ocl_get_point(ppnt, pntbuf.as_mut_ptr() as *const libc::c_uchar);
}
pub unsafe extern "C" fn show_elapsed(
    mut tv: *mut timeval,
    mut place: *const libc::c_char,
) {
    let mut now: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut delta: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut now as *mut timeval, 0 as *mut libc::c_void);
    delta.tv_sec = now.tv_sec - (*tv).tv_sec;
    delta.tv_usec = now.tv_usec - (*tv).tv_usec;
    if delta.tv_usec < 0 as libc::c_long {
        delta.tv_sec -= 1;
        delta.tv_usec += 1000000 as libc::c_long;
    }
    fprintf(
        stderr,
        b"%s spent %ld.%06lds\n\0" as *const u8 as *const libc::c_char,
        place,
        delta.tv_sec,
        delta.tv_usec,
    );
}
unsafe extern "C" fn vg_ocl_gethash_check(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
) -> libc::c_int {
    let mut vxcp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    let mut test_func: Option::<
        unsafe extern "C" fn(*mut vg_exec_context_t) -> libc::c_int,
    > = None;
    let mut ocl_hashes_out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut round___0: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    vxcp = &mut (*vocp).base;
    vcp = (*vocp).base.vxc_vc;
    test_func = (*vcp).vc_test;
    res = 0 as libc::c_int;
    tmp___0 = vg_ocl_map_arg_buffer(vocp, slot, 0 as libc::c_int, 0 as libc::c_int);
    ocl_hashes_out = tmp___0 as *mut libc::c_uchar;
    if ocl_hashes_out.is_null() {
        fprintf(
            stderr,
            b"ERROR: Could not map hash result buffer for slot %d\n\0" as *const u8
                as *const libc::c_char,
            slot,
        );
        return 2 as libc::c_int;
    }
    round___0 = (*vocp).voc_ocl_cols * (*vocp).voc_ocl_rows;
    i = 0 as libc::c_int;
    while i < round___0 {
        memcpy(
            &mut *((*vxcp).vxc_binres).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            ocl_hashes_out.offset((20 as libc::c_int * i) as isize)
                as *const libc::c_void,
            20 as libc::c_int as size_t as _,
        );
        res = (Some(test_func.expect("non-null function pointer")))
            .expect("non-null function pointer")(vxcp);
        if res != 0 {
            break;
        }
        i += 1;
        (*vxcp).vxc_delta += 1;
    }
    vg_ocl_unmap_arg_buffer(
        vocp,
        slot,
        0 as libc::c_int,
        ocl_hashes_out as *mut libc::c_void,
    );
    return res;
}
unsafe extern "C" fn vg_ocl_gethash_init(
    mut vocp: *mut vg_ocl_context_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp___0 = vg_ocl_create_kernel(
        vocp,
        2 as libc::c_int,
        b"hash_ec_point_get\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 == 0 {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*vocp).voc_nslots {
        tmp___1 = vg_ocl_kernel_arg_alloc(
            vocp,
            i,
            0 as libc::c_int,
            (20 as libc::c_int * (*vocp).voc_ocl_rows * (*vocp).voc_ocl_cols) as size_t,
            1 as libc::c_int,
        );
        if tmp___1 == 0 {
            return 0 as libc::c_int;
        }
        i += 1;
    }
    (*vocp)
        .voc_rekey_func = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut _vg_ocl_context_s) -> libc::c_int>,
    >(0 as *mut libc::c_void);
    (*vocp)
        .voc_check_func = Some(
        vg_ocl_gethash_check
            as unsafe extern "C" fn(*mut vg_ocl_context_t, libc::c_int) -> libc::c_int,
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_ocl_prefix_rekey(
    mut vocp: *mut vg_ocl_context_t,
) -> libc::c_int {
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    let mut ocl_targets_in: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ocl_found_out: *mut uint32_t = 0 as *mut uint32_t;
    let mut i: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    vcp = (*vocp).base.vxc_vc;
    i = 0 as libc::c_int;
    while i < (*vocp).voc_nslots {
        tmp___0 = vg_ocl_map_arg_buffer(vocp, i, 0 as libc::c_int, 1 as libc::c_int);
        ocl_found_out = tmp___0 as *mut uint32_t;
        if ocl_found_out.is_null() {
            fprintf(
                stderr,
                b"ERROR: Could not map result buffer for slot %d (rekey)\n\0"
                    as *const u8 as *const libc::c_char,
                i,
            );
            return -(1 as libc::c_int);
        }
        *ocl_found_out.offset(0 as libc::c_int as isize) = 4294967295 as libc::c_uint;
        vg_ocl_unmap_arg_buffer(
            vocp,
            i,
            0 as libc::c_int,
            ocl_found_out as *mut libc::c_void,
        );
        i += 1;
    }
    if (*vocp).voc_pattern_rewrite != 0 {
        i = vg_context_hash160_sort(vcp, 0 as *mut libc::c_void);
        if i == 0 {
            return 0 as libc::c_int;
        }
        if i > (*vocp).voc_pattern_alloc {
            tmp___1 = vg_ocl_kernel_arg_alloc(
                vocp,
                -(1 as libc::c_int),
                5 as libc::c_int,
                (40 as libc::c_int * i) as size_t,
                0 as libc::c_int,
            );
            if tmp___1 == 0 {
                return -(1 as libc::c_int);
            }
            (*vocp).voc_pattern_alloc = i;
        }
        tmp___2 = vg_ocl_map_arg_buffer(
            vocp,
            0 as libc::c_int,
            5 as libc::c_int,
            1 as libc::c_int,
        );
        ocl_targets_in = tmp___2 as *mut libc::c_uchar;
        if ocl_targets_in.is_null() {
            fprintf(
                stderr,
                b"ERROR: Could not map hash target buffer\n\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        vg_context_hash160_sort(vcp, ocl_targets_in as *mut libc::c_void);
        vg_ocl_unmap_arg_buffer(
            vocp,
            0 as libc::c_int,
            5 as libc::c_int,
            ocl_targets_in as *mut libc::c_void,
        );
        vg_ocl_kernel_int_arg(vocp, -(1 as libc::c_int), 4 as libc::c_int, i);
        (*vocp).voc_pattern_rewrite = 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_ocl_prefix_check(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
) -> libc::c_int {
    let mut vxcp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    let mut test_func: Option::<
        unsafe extern "C" fn(*mut vg_exec_context_t) -> libc::c_int,
    > = None;
    let mut ocl_found_out: *mut uint32_t = 0 as *mut uint32_t;
    let mut found_delta: uint32_t = 0;
    let mut orig_delta: libc::c_int = 0;
    let mut tablesize: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    vxcp = &mut (*vocp).base;
    vcp = (*vocp).base.vxc_vc;
    test_func = (*vcp).vc_test;
    res = 0 as libc::c_int;
    tmp___0 = vg_ocl_map_arg_buffer(vocp, slot, 0 as libc::c_int, 2 as libc::c_int);
    ocl_found_out = tmp___0 as *mut uint32_t;
    if ocl_found_out.is_null() {
        fprintf(
            stderr,
            b"ERROR: Could not map result buffer for slot %d\n\0" as *const u8
                as *const libc::c_char,
            slot,
        );
        return 2 as libc::c_int;
    }
    found_delta = *ocl_found_out.offset(0 as libc::c_int as isize);
    if found_delta != 4294967295 as libc::c_uint {
        orig_delta = (*vxcp).vxc_delta;
        (*vxcp)
            .vxc_delta = ((*vxcp).vxc_delta as uint32_t).wrapping_add(found_delta)
            as libc::c_int;
        vg_exec_context_calc_address(vxcp);
        res = 0 as libc::c_int;
        tmp___1 = memcmp(
            ((*vxcp).vxc_binres).as_mut_ptr().offset(1 as libc::c_int as isize)
                as *const libc::c_void,
            ocl_found_out.offset(2 as libc::c_int as isize) as *const libc::c_void,
            20 as libc::c_int as size_t,
        );
        if tmp___1 == 0 {
            res = (Some(test_func.expect("non-null function pointer")))
                .expect("non-null function pointer")(vxcp);
        }
        if res == 0 as libc::c_int {
            tablesize = *ocl_found_out.offset(2 as libc::c_int as isize) as libc::c_int;
            fprintf(
                stderr,
                b"Match idx: %d\n\0" as *const u8 as *const libc::c_char,
                *ocl_found_out.offset(1 as libc::c_int as isize),
            );
            fprintf(stderr, b"CPU hash: \0" as *const u8 as *const libc::c_char);
            fdumphex(
                stderr,
                ((*vxcp).vxc_binres).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *const libc::c_uchar,
                20 as libc::c_int as size_t,
            );
            fprintf(stderr, b"GPU hash: \0" as *const u8 as *const libc::c_char);
            fdumphex(
                stderr,
                ocl_found_out.offset(2 as libc::c_int as isize) as *mut libc::c_uchar
                    as *const libc::c_uchar,
                20 as libc::c_int as size_t,
            );
            fprintf(
                stderr,
                b"Found delta: %d Start delta: %d\n\0" as *const u8
                    as *const libc::c_char,
                found_delta,
                orig_delta,
            );
            res = 1 as libc::c_int;
        }
    } else {
        (*vxcp).vxc_delta += (*vocp).voc_ocl_cols * (*vocp).voc_ocl_rows;
    }
    vg_ocl_unmap_arg_buffer(
        vocp,
        slot,
        0 as libc::c_int,
        ocl_found_out as *mut libc::c_void,
    );
    return res;
}
unsafe extern "C" fn vg_ocl_prefix_init(mut vocp: *mut vg_ocl_context_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp___0 = vg_ocl_create_kernel(
        vocp,
        2 as libc::c_int,
        b"hash_ec_point_search_prefix\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 == 0 {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*vocp).voc_nslots {
        tmp___1 = vg_ocl_kernel_arg_alloc(
            vocp,
            i,
            0 as libc::c_int,
            28 as libc::c_int as size_t,
            1 as libc::c_int,
        );
        if tmp___1 == 0 {
            return 0 as libc::c_int;
        }
        i += 1;
    }
    (*vocp)
        .voc_rekey_func = Some(
        vg_ocl_prefix_rekey as unsafe extern "C" fn(*mut vg_ocl_context_t) -> libc::c_int,
    );
    (*vocp)
        .voc_check_func = Some(
        vg_ocl_prefix_check
            as unsafe extern "C" fn(*mut vg_ocl_context_t, libc::c_int) -> libc::c_int,
    );
    (*vocp).voc_pattern_rewrite = 1 as libc::c_int;
    (*vocp).voc_pattern_alloc = 0 as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn vg_ocl_config_pattern(
    mut vocp: *mut vg_ocl_context_t,
) -> libc::c_int {
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    let mut i: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    vcp = (*vocp).base.vxc_vc;
    i = vg_context_hash160_sort(vcp, 0 as *mut libc::c_void);
    if i > 0 as libc::c_int {
        if (*vcp).vc_verbose > 1 as libc::c_int {
            fprintf(
                stderr,
                b"Using OpenCL prefix matcher\n\0" as *const u8 as *const libc::c_char,
            );
        }
        tmp___0 = vg_ocl_prefix_init(vocp);
        return tmp___0;
    }
    if (*vcp).vc_verbose > 0 as libc::c_int {
        fprintf(
            stderr,
            b"WARNING: Using CPU pattern matcher\n\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___1 = vg_ocl_gethash_init(vocp);
    return tmp___1;
}
static mut raw_modulus: [libc::c_uchar; 32] = [
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    254 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    252 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
];
unsafe extern "C" fn vg_ocl_verify_temporary(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
    mut z_inverted: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut vxcp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut point_tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut z_heap: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ocl_points_in: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ocl_strides_in: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pgroup: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut ppr: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut ppc: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut pps: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut ppt: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut bnz: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnez: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnm: BIGNUM = BIGNUM {
        d: 0 as *mut libc::c_ulong,
        top: 0,
        dmax: 0,
        neg: 0,
        flags: 0,
    };
    let mut bnzc: *mut BIGNUM = 0 as *mut BIGNUM;
    let mut bnctx: *mut BN_CTX = 0 as *mut BN_CTX;
    let mut bnmont: *mut BN_MONT_CTX = 0 as *mut BN_MONT_CTX;
    let mut ret: libc::c_int = 0;
    let mut mismatches: libc::c_int = 0;
    let mut mm_r: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut bx: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___6: *const BIGNUM = 0 as *const BIGNUM;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    vxcp = &mut (*vocp).base;
    point_tmp = 0 as *mut libc::c_void as *mut libc::c_uchar;
    z_heap = 0 as *mut libc::c_void as *mut libc::c_uchar;
    ocl_points_in = 0 as *mut libc::c_void as *mut libc::c_uchar;
    ocl_strides_in = 0 as *mut libc::c_void as *mut libc::c_uchar;
    ppr = 0 as *mut libc::c_void as *mut EC_POINT;
    ppc = 0 as *mut libc::c_void as *mut EC_POINT;
    pps = 0 as *mut libc::c_void as *mut EC_POINT;
    ppt = 0 as *mut libc::c_void as *mut EC_POINT;
    bnctx = 0 as *mut libc::c_void as *mut BN_CTX;
    ret = 0 as libc::c_int;
    mismatches = 0 as libc::c_int;
    BN_init(&mut bnz);
    BN_init(&mut bnez);
    BN_init(&mut bnm);
    bnctx = BN_CTX_new();
    bnmont = BN_MONT_CTX_new();
    pgroup = EC_KEY_get0_group((*vxcp).vxc_key as *const EC_KEY);
    ppr = EC_POINT_new(pgroup);
    ppc = EC_POINT_new(pgroup);
    pps = EC_POINT_new(pgroup);
    ppt = EC_POINT_new(pgroup);
    if bnctx.is_null() {
        fprintf(stderr, b"ERROR: out of memory\n\0" as *const u8 as *const libc::c_char);
    } else if bnmont.is_null() {
        fprintf(stderr, b"ERROR: out of memory\n\0" as *const u8 as *const libc::c_char);
    } else if ppr.is_null() {
        fprintf(stderr, b"ERROR: out of memory\n\0" as *const u8 as *const libc::c_char);
    } else if ppc.is_null() {
        fprintf(stderr, b"ERROR: out of memory\n\0" as *const u8 as *const libc::c_char);
    } else if pps.is_null() {
        fprintf(stderr, b"ERROR: out of memory\n\0" as *const u8 as *const libc::c_char);
    } else if ppt.is_null() {
        fprintf(stderr, b"ERROR: out of memory\n\0" as *const u8 as *const libc::c_char);
    } else {
        BN_bin2bn(
            raw_modulus.as_ptr(),
            ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong as libc::c_int,
            &mut bnm,
        );
        BN_MONT_CTX_set(bnmont, &mut bnm as *mut BIGNUM as *const BIGNUM, bnctx);
        if z_inverted != 0 {
            bnzc = &mut bnez;
        } else {
            bnzc = &mut (*pps).Z;
        }
        tmp___0 = vg_ocl_map_arg_buffer(vocp, slot, 1 as libc::c_int, 0 as libc::c_int);
        z_heap = tmp___0 as *mut libc::c_uchar;
        tmp___1 = vg_ocl_map_arg_buffer(vocp, slot, 2 as libc::c_int, 0 as libc::c_int);
        point_tmp = tmp___1 as *mut libc::c_uchar;
        tmp___2 = vg_ocl_map_arg_buffer(vocp, slot, 3 as libc::c_int, 0 as libc::c_int);
        ocl_points_in = tmp___2 as *mut libc::c_uchar;
        tmp___3 = vg_ocl_map_arg_buffer(vocp, slot, 4 as libc::c_int, 0 as libc::c_int);
        ocl_strides_in = tmp___3 as *mut libc::c_uchar;
        if z_heap.is_null() {
            fprintf(
                stderr,
                b"ERROR: could not map OpenCL point buffers\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else if point_tmp.is_null() {
            fprintf(
                stderr,
                b"ERROR: could not map OpenCL point buffers\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else if ocl_points_in.is_null() {
            fprintf(
                stderr,
                b"ERROR: could not map OpenCL point buffers\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else if ocl_strides_in.is_null() {
            fprintf(
                stderr,
                b"ERROR: could not map OpenCL point buffers\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            y = 0 as libc::c_int;
            while y < (*vocp).voc_ocl_rows {
                vg_ocl_get_point(
                    ppr,
                    ocl_strides_in.offset((64 as libc::c_int * y) as isize)
                        as *const libc::c_uchar,
                );
                bx = y * (*vocp).voc_ocl_cols;
                mm_r = 0 as libc::c_int;
                x = 0 as libc::c_int;
                while x < (*vocp).voc_ocl_cols {
                    vg_ocl_get_point_tpa(ppc, ocl_points_in as *const libc::c_uchar, x);
                    if (*ppr).Z_is_one != 0 {
                        if (*ppc).Z_is_one == 0 {
                            __assert_fail(
                                b"ppr->Z_is_one && ppc->Z_is_one\0" as *const u8
                                    as *const libc::c_char,
                                b"oclengine.c\0" as *const u8 as *const libc::c_char,
                                1718 as libc::c_uint,
                                b"vg_ocl_verify_temporary\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    } else {
                        __assert_fail(
                            b"ppr->Z_is_one && ppc->Z_is_one\0" as *const u8
                                as *const libc::c_char,
                            b"oclengine.c\0" as *const u8 as *const libc::c_char,
                            1718 as libc::c_uint,
                            b"vg_ocl_verify_temporary\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    EC_POINT_add(
                        pgroup,
                        pps,
                        ppc as *const EC_POINT,
                        ppr as *const EC_POINT,
                        bnctx,
                    );
                    if (*pps).Z_is_one != 0 {
                        __assert_fail(
                            b"!pps->Z_is_one\0" as *const u8 as *const libc::c_char,
                            b"oclengine.c\0" as *const u8 as *const libc::c_char,
                            1720 as libc::c_uint,
                            b"vg_ocl_verify_temporary\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    vg_ocl_get_point_tpa(ppt, point_tmp as *const libc::c_uchar, bx + x);
                    vg_ocl_get_bignum_tpa(
                        &mut bnz,
                        z_heap as *const libc::c_uchar,
                        bx + x,
                    );
                    if z_inverted != 0 {
                        BN_mod_inverse(
                            &mut bnez,
                            &mut (*pps).Z as *mut BIGNUM as *const BIGNUM,
                            &mut bnm as *mut BIGNUM as *const BIGNUM,
                            bnctx,
                        );
                        BN_mod_mul_montgomery(
                            &mut bnez,
                            &mut bnez as *mut BIGNUM as *const BIGNUM,
                            &mut (*bnmont).RR as *mut BIGNUM as *const BIGNUM,
                            bnmont,
                            bnctx,
                        );
                        BN_mod_mul_montgomery(
                            &mut bnez,
                            &mut bnez as *mut BIGNUM as *const BIGNUM,
                            &mut (*bnmont).RR as *mut BIGNUM as *const BIGNUM,
                            bnmont,
                            bnctx,
                        );
                    }
                    tmp___10 = BN_cmp(
                        &mut (*ppt).X as *mut BIGNUM as *const BIGNUM,
                        &mut (*pps).X as *mut BIGNUM as *const BIGNUM,
                    );
                    if tmp___10 != 0 {
                        current_block = 4273054258691719101;
                    } else {
                        tmp___11 = BN_cmp(
                            &mut (*ppt).Y as *mut BIGNUM as *const BIGNUM,
                            &mut (*pps).Y as *mut BIGNUM as *const BIGNUM,
                        );
                        if tmp___11 != 0 {
                            current_block = 4273054258691719101;
                        } else {
                            tmp___12 = BN_cmp(
                                &mut bnz as *mut BIGNUM as *const BIGNUM,
                                bnzc as *const BIGNUM,
                            );
                            if tmp___12 != 0 {
                                current_block = 4273054258691719101;
                            } else {
                                current_block = 11674240781755647963;
                            }
                        }
                    }
                    match current_block {
                        4273054258691719101 => {
                            if mismatches == 0 {
                                fprintf(
                                    stderr,
                                    b"Base privkey: \0" as *const u8 as *const libc::c_char,
                                );
                                tmp___6 = EC_KEY_get0_private_key(
                                    (*vxcp).vxc_key as *const EC_KEY,
                                );
                                fdumpbn(stderr, tmp___6);
                            }
                            mismatches += 1;
                            fprintf(
                                stderr,
                                b"Mismatch for kernel %d, offset %d (%d,%d)\n\0"
                                    as *const u8 as *const libc::c_char,
                                z_inverted,
                                bx + x,
                                y,
                                x,
                            );
                            if mm_r == 0 {
                                mm_r = 1 as libc::c_int;
                                fprintf(
                                    stderr,
                                    b"Row X   : \0" as *const u8 as *const libc::c_char,
                                );
                                fdumpbn(
                                    stderr,
                                    &mut (*ppr).X as *mut BIGNUM as *const BIGNUM,
                                );
                                fprintf(
                                    stderr,
                                    b"Row Y   : \0" as *const u8 as *const libc::c_char,
                                );
                                fdumpbn(
                                    stderr,
                                    &mut (*ppr).Y as *mut BIGNUM as *const BIGNUM,
                                );
                            }
                            fprintf(
                                stderr,
                                b"Column X: \0" as *const u8 as *const libc::c_char,
                            );
                            fdumpbn(
                                stderr,
                                &mut (*ppc).X as *mut BIGNUM as *const BIGNUM,
                            );
                            fprintf(
                                stderr,
                                b"Column Y: \0" as *const u8 as *const libc::c_char,
                            );
                            fdumpbn(
                                stderr,
                                &mut (*ppc).Y as *mut BIGNUM as *const BIGNUM,
                            );
                            tmp___7 = BN_cmp(
                                &mut (*ppt).X as *mut BIGNUM as *const BIGNUM,
                                &mut (*pps).X as *mut BIGNUM as *const BIGNUM,
                            );
                            if tmp___7 != 0 {
                                fprintf(
                                    stderr,
                                    b"Expect X: \0" as *const u8 as *const libc::c_char,
                                );
                                fdumpbn(
                                    stderr,
                                    &mut (*pps).X as *mut BIGNUM as *const BIGNUM,
                                );
                                fprintf(
                                    stderr,
                                    b"Device X: \0" as *const u8 as *const libc::c_char,
                                );
                                fdumpbn(
                                    stderr,
                                    &mut (*ppt).X as *mut BIGNUM as *const BIGNUM,
                                );
                            }
                            tmp___8 = BN_cmp(
                                &mut (*ppt).Y as *mut BIGNUM as *const BIGNUM,
                                &mut (*pps).Y as *mut BIGNUM as *const BIGNUM,
                            );
                            if tmp___8 != 0 {
                                fprintf(
                                    stderr,
                                    b"Expect Y: \0" as *const u8 as *const libc::c_char,
                                );
                                fdumpbn(
                                    stderr,
                                    &mut (*pps).Y as *mut BIGNUM as *const BIGNUM,
                                );
                                fprintf(
                                    stderr,
                                    b"Device Y: \0" as *const u8 as *const libc::c_char,
                                );
                                fdumpbn(
                                    stderr,
                                    &mut (*ppt).Y as *mut BIGNUM as *const BIGNUM,
                                );
                            }
                            tmp___9 = BN_cmp(
                                &mut bnz as *mut BIGNUM as *const BIGNUM,
                                bnzc as *const BIGNUM,
                            );
                            if tmp___9 != 0 {
                                fprintf(
                                    stderr,
                                    b"Expect Z: \0" as *const u8 as *const libc::c_char,
                                );
                                fdumpbn(stderr, bnzc as *const BIGNUM);
                                fprintf(
                                    stderr,
                                    b"Device Z: \0" as *const u8 as *const libc::c_char,
                                );
                                fdumpbn(stderr, &mut bnz as *mut BIGNUM as *const BIGNUM);
                            }
                        }
                        _ => {}
                    }
                    x += 1;
                }
                y += 1;
            }
            ret = (mismatches == 0) as libc::c_int;
        }
    }
    if !z_heap.is_null() {
        vg_ocl_unmap_arg_buffer(
            vocp,
            slot,
            1 as libc::c_int,
            z_heap as *mut libc::c_void,
        );
    }
    if !point_tmp.is_null() {
        vg_ocl_unmap_arg_buffer(
            vocp,
            slot,
            2 as libc::c_int,
            point_tmp as *mut libc::c_void,
        );
    }
    if !ocl_points_in.is_null() {
        vg_ocl_unmap_arg_buffer(
            vocp,
            slot,
            3 as libc::c_int,
            ocl_points_in as *mut libc::c_void,
        );
    }
    if !ocl_strides_in.is_null() {
        vg_ocl_unmap_arg_buffer(
            vocp,
            slot,
            4 as libc::c_int,
            ocl_strides_in as *mut libc::c_void,
        );
    }
    if !ppr.is_null() {
        EC_POINT_free(ppr);
    }
    if !ppc.is_null() {
        EC_POINT_free(ppc);
    }
    if !pps.is_null() {
        EC_POINT_free(pps);
    }
    if !ppt.is_null() {
        EC_POINT_free(ppt);
    }
    BN_clear_free(&mut bnz);
    BN_clear_free(&mut bnez);
    BN_clear_free(&mut bnm);
    if !bnmont.is_null() {
        BN_MONT_CTX_free(bnmont);
    }
    if !bnctx.is_null() {
        BN_CTX_free(bnctx);
    }
    return ret;
}
unsafe extern "C" fn vg_ocl_verify_k0(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
) -> libc::c_int {
    let mut tmp___0: libc::c_int = 0;
    tmp___0 = vg_ocl_verify_temporary(vocp, slot, 0 as libc::c_int);
    return tmp___0;
}
unsafe extern "C" fn vg_ocl_verify_k1(
    mut vocp: *mut vg_ocl_context_t,
    mut slot: libc::c_int,
) -> libc::c_int {
    let mut tmp___0: libc::c_int = 0;
    tmp___0 = vg_ocl_verify_temporary(vocp, slot, 1 as libc::c_int);
    return tmp___0;
}
unsafe extern "C" fn vg_opencl_thread(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut vocp: *mut vg_ocl_context_t = 0 as *mut vg_ocl_context_t;
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    let mut halt: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    let mut rows: libc::c_int = 0;
    let mut cols: libc::c_int = 0;
    let mut invsize: libc::c_int = 0;
    let mut idleu: libc::c_ulonglong = 0;
    let mut busyu: libc::c_ulonglong = 0;
    let mut pidle: libc::c_double = 0.;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tvt: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tvd: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut idle: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut busy: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    vocp = arg as *mut vg_ocl_context_t;
    vcp = (*vocp).base.vxc_vc;
    halt = 0 as libc::c_int;
    slot = -(1 as libc::c_int);
    memset(
        &mut idle as *mut timeval as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as _,
    );
    memset(
        &mut busy as *mut timeval as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<timeval>() as libc::c_ulong as _,
    );
    's_57: loop {
        pthread_mutex_lock(&mut (*vocp).voc_lock);
        if halt != 0 {
            halt = 0 as libc::c_int;
            (*vocp).voc_halt = 1 as libc::c_int;
        }
        if slot != -(1 as libc::c_int) {
            if !((*vocp).voc_ocl_slot == slot) {
                __assert_fail(
                    b"vocp->voc_ocl_slot == slot\0" as *const u8 as *const libc::c_char,
                    b"oclengine.c\0" as *const u8 as *const libc::c_char,
                    1838 as libc::c_uint,
                    b"vg_opencl_thread\0" as *const u8 as *const libc::c_char,
                );
            }
            (*vocp).voc_ocl_slot = -(1 as libc::c_int);
            slot = -(1 as libc::c_int);
            pthread_cond_signal(&mut (*vocp).voc_wait);
        }
        if (*vocp).voc_ocl_slot == -(1 as libc::c_int) {
            gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
            while (*vocp).voc_ocl_slot == -(1 as libc::c_int) {
                if (*vocp).voc_halt != 0 {
                    break 's_57;
                }
                pthread_cond_wait(
                    &mut (*vocp).voc_wait as *mut pthread_cond_t,
                    &mut (*vocp).voc_lock as *mut pthread_mutex_t,
                );
            }
            gettimeofday(&mut tvt as *mut timeval, 0 as *mut libc::c_void);
            tvd.tv_sec = tvt.tv_sec - tv.tv_sec;
            tvd.tv_usec = tvt.tv_usec - tv.tv_usec;
            if tvd.tv_usec < 0 as libc::c_long {
                tvd.tv_sec -= 1;
                tvd.tv_usec += 1000000 as libc::c_long;
            }
            idle.tv_sec = tvd.tv_sec + idle.tv_sec;
            idle.tv_usec = tvd.tv_usec + idle.tv_usec;
            if idle.tv_usec >= 1000000 as libc::c_long {
                idle.tv_sec += 1;
                idle.tv_usec -= 1000000 as libc::c_long;
            }
        }
        slot = (*vocp).voc_ocl_slot;
        rows = (*vocp).voc_ocl_rows;
        cols = (*vocp).voc_ocl_cols;
        invsize = (*vocp).voc_ocl_invsize;
        pthread_mutex_unlock(&mut (*vocp).voc_lock);
        gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
        tmp___1 = vg_ocl_kernel_start(vocp, slot, cols, rows, invsize);
        if tmp___1 == 0 {
            halt = 1 as libc::c_int;
        }
        tmp___2 = vg_ocl_kernel_wait(vocp, slot);
        if tmp___2 == 0 {
            halt = 1 as libc::c_int;
        }
        if (*vcp).vc_verbose > 1 as libc::c_int {
            gettimeofday(&mut tvt as *mut timeval, 0 as *mut libc::c_void);
            tvd.tv_sec = tvt.tv_sec - tv.tv_sec;
            tvd.tv_usec = tvt.tv_usec - tv.tv_usec;
            if tvd.tv_usec < 0 as libc::c_long {
                tvd.tv_sec -= 1;
                tvd.tv_usec += 1000000 as libc::c_long;
            }
            busy.tv_sec = tvd.tv_sec + busy.tv_sec;
            busy.tv_usec = tvd.tv_usec + busy.tv_usec;
            if busy.tv_usec >= 1000000 as libc::c_long {
                busy.tv_sec += 1;
                busy.tv_usec -= 1000000 as libc::c_long;
            }
            if busy.tv_sec + idle.tv_sec > 1 as libc::c_long {
                idleu = (1000000 as libc::c_long * idle.tv_sec + idle.tv_usec)
                    as libc::c_ulonglong;
                busyu = (1000000 as libc::c_long * busy.tv_sec + busy.tv_usec)
                    as libc::c_ulonglong;
                pidle = idleu as libc::c_double
                    / idleu.wrapping_add(busyu) as libc::c_double;
                if pidle > 0.01f64 {
                    fprintf(
                        stderr,
                        b"\rGPU idle: %.2f%%                                                              \n\0"
                            as *const u8 as *const libc::c_char,
                        100 as libc::c_int as libc::c_double * pidle,
                    );
                }
                memset(
                    &mut idle as *mut timeval as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<timeval>() as libc::c_ulong as _,
                );
                memset(
                    &mut busy as *mut timeval as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<timeval>() as libc::c_ulong as _,
                );
            }
        }
    }
    pthread_mutex_unlock(&mut (*vocp).voc_lock);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn vg_opencl_loop(
    mut arg: *mut vg_exec_context_t,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut vocp: *mut vg_ocl_context_t = 0 as *mut vg_ocl_context_t;
    let mut i: libc::c_int = 0;
    let mut round___0: libc::c_int = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut pattern_generation: libc::c_int = 0;
    let mut rekey_max: libc::c_ulong = 0;
    let mut npoints: libc::c_ulong = 0;
    let mut rekey_at: libc::c_ulong = 0;
    let mut pkey: *mut EC_KEY = 0 as *mut EC_KEY;
    let mut pgroup: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut pgen: *const EC_POINT = 0 as *const EC_POINT;
    let mut ppbase: *mut *mut EC_POINT = 0 as *mut *mut EC_POINT;
    let mut pprow: *mut *mut EC_POINT = 0 as *mut *mut EC_POINT;
    let mut pbatchinc: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut poffset: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut pseek: *mut EC_POINT = 0 as *mut EC_POINT;
    let mut ocl_points_in: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ocl_strides_in: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vcp: *mut vg_context_t = 0 as *mut vg_context_t;
    let mut vxcp: *mut vg_exec_context_t = 0 as *mut vg_exec_context_t;
    let mut slot: libc::c_int = 0;
    let mut nslots: libc::c_int = 0;
    let mut slot_busy: libc::c_int = 0;
    let mut slot_done: libc::c_int = 0;
    let mut halt: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut output_interval: libc::c_int = 0;
    let mut tvstart: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: *const BIGNUM = 0 as *const BIGNUM;
    let mut tmp___10: *const EC_POINT = 0 as *const EC_POINT;
    let mut tmp___11: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: *mut libc::c_void = 0 as *mut libc::c_void;
    vocp = arg as *mut vg_ocl_context_t;
    rekey_max = 100000000 as libc::c_ulong;
    pkey = 0 as *mut libc::c_void as *mut EC_KEY;
    ppbase = 0 as *mut libc::c_void as *mut *mut EC_POINT;
    pbatchinc = 0 as *mut libc::c_void as *mut EC_POINT;
    poffset = 0 as *mut libc::c_void as *mut EC_POINT;
    pseek = 0 as *mut libc::c_void as *mut EC_POINT;
    vcp = (*vocp).base.vxc_vc;
    vxcp = &mut (*vocp).base;
    slot_busy = 0 as libc::c_int;
    slot_done = 0 as libc::c_int;
    halt = 0 as libc::c_int;
    c = 0 as libc::c_int;
    output_interval = 1000 as libc::c_int;
    pkey = (*vxcp).vxc_key;
    pgroup = EC_KEY_get0_group(pkey as *const EC_KEY);
    pgen = EC_GROUP_get0_generator(pgroup);
    round___0 = (*vocp).voc_ocl_rows * (*vocp).voc_ocl_cols;
    if (*vcp).vc_remove_on_match == 0 {
        if (*vcp).vc_chance >= 1.0f32 as libc::c_double {
            if (*vcp).vc_chance < round___0 as libc::c_double {
                if (*vcp).vc_verbose > 0 as libc::c_int {
                    fprintf(
                        stderr,
                        b"WARNING: low pattern difficulty\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    fprintf(
                        stderr,
                        b"WARNING: better match throughput is possible using vanitygen on the CPU\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    slot = 0 as libc::c_int;
    nslots = 2 as libc::c_int;
    (*vocp).voc_nslots = nslots;
    nrows = (*vocp).voc_ocl_rows;
    ncols = (*vocp).voc_ocl_cols;
    tmp___0 = malloc(
        ((nrows + ncols) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut EC_POINT>() as libc::c_ulong),
    );
    ppbase = tmp___0 as *mut *mut EC_POINT;
    if ppbase.is_null() {
        current_block = 2684075921598956718;
    } else {
        i = 0 as libc::c_int;
        loop {
            if !(i < nrows + ncols) {
                current_block = 1434579379687443766;
                break;
            }
            let ref mut fresh1 = *ppbase.offset(i as isize);
            *fresh1 = EC_POINT_new(pgroup);
            if (*ppbase.offset(i as isize)).is_null() {
                current_block = 2684075921598956718;
                break;
            }
            i += 1;
        }
        match current_block {
            2684075921598956718 => {}
            _ => {
                pprow = ppbase.offset(ncols as isize);
                pbatchinc = EC_POINT_new(pgroup);
                poffset = EC_POINT_new(pgroup);
                pseek = EC_POINT_new(pgroup);
                if pbatchinc.is_null() {
                    current_block = 2684075921598956718;
                } else if poffset.is_null() {
                    current_block = 2684075921598956718;
                } else if pseek.is_null() {
                    current_block = 2684075921598956718;
                } else {
                    BN_set_word(&mut (*vxcp).vxc_bntmp, ncols as libc::c_ulong);
                    EC_POINT_mul(
                        pgroup,
                        pbatchinc,
                        &mut (*vxcp).vxc_bntmp as *mut BIGNUM as *const BIGNUM,
                        0 as *mut libc::c_void as *const EC_POINT,
                        0 as *mut libc::c_void as *const BIGNUM,
                        (*vxcp).vxc_bnctx,
                    );
                    EC_POINT_make_affine(pgroup, pbatchinc, (*vxcp).vxc_bnctx);
                    BN_set_word(&mut (*vxcp).vxc_bntmp, round___0 as libc::c_ulong);
                    EC_POINT_mul(
                        pgroup,
                        poffset,
                        &mut (*vxcp).vxc_bntmp as *mut BIGNUM as *const BIGNUM,
                        0 as *mut libc::c_void as *const EC_POINT,
                        0 as *mut libc::c_void as *const BIGNUM,
                        (*vxcp).vxc_bnctx,
                    );
                    EC_POINT_make_affine(pgroup, poffset, (*vxcp).vxc_bnctx);
                    tmp___1 = vg_ocl_config_pattern(vocp);
                    if tmp___1 == 0 {
                        current_block = 2684075921598956718;
                    } else {
                        i = 0 as libc::c_int;
                        loop {
                            if !(i < nslots) {
                                current_block = 5597585068398118923;
                                break;
                            }
                            tmp___2 = vg_ocl_kernel_arg_alloc(
                                vocp,
                                i,
                                4 as libc::c_int,
                                (64 as libc::c_int * nrows) as size_t,
                                1 as libc::c_int,
                            );
                            if tmp___2 == 0 {
                                current_block = 2684075921598956718;
                                break;
                            }
                            i += 1;
                        }
                        match current_block {
                            2684075921598956718 => {}
                            _ => {
                                tmp___3 = vg_ocl_kernel_arg_alloc(
                                    vocp,
                                    -(1 as libc::c_int),
                                    1 as libc::c_int,
                                    (64 as libc::c_int * round___0 + 4095 as libc::c_int
                                        & -(4096 as libc::c_int)) as size_t,
                                    0 as libc::c_int,
                                );
                                if tmp___3 != 0 {
                                    tmp___4 = vg_ocl_kernel_arg_alloc(
                                        vocp,
                                        -(1 as libc::c_int),
                                        2 as libc::c_int,
                                        (64 as libc::c_int * round___0 + 4095 as libc::c_int
                                            & -(4096 as libc::c_int)) as size_t,
                                        0 as libc::c_int,
                                    );
                                    if tmp___4 != 0 {
                                        tmp___5 = vg_ocl_kernel_arg_alloc(
                                            vocp,
                                            -(1 as libc::c_int),
                                            3 as libc::c_int,
                                            (64 as libc::c_int * ncols + 4095 as libc::c_int
                                                & -(4096 as libc::c_int)) as size_t,
                                            1 as libc::c_int,
                                        );
                                        if tmp___5 == 0 {
                                            current_block = 2684075921598956718;
                                        } else {
                                            npoints = 0 as libc::c_ulong;
                                            rekey_at = 0 as libc::c_ulong;
                                            (*vxcp)
                                                .vxc_binres[0 as libc::c_int
                                                as usize] = (*vcp).vc_addrtype as libc::c_uchar;
                                            tmp___6 = pthread_create(
                                                &mut (*vocp).voc_ocl_thread as *mut pthread_t,
                                                0 as *mut libc::c_void as *const pthread_attr_t,
                                                Some(
                                                    vg_opencl_thread
                                                        as unsafe extern "C" fn(
                                                            *mut libc::c_void,
                                                        ) -> *mut libc::c_void,
                                                ),
                                                vocp as *mut libc::c_void,
                                            );
                                            if tmp___6 != 0 {
                                                current_block = 2684075921598956718;
                                            } else {
                                                gettimeofday(
                                                    &mut tvstart as *mut timeval,
                                                    0 as *mut libc::c_void,
                                                );
                                                '_l_rekey: loop {
                                                    if ((*vocp).voc_rekey_func).is_some() {
                                                        tmp___7 = (Some(
                                                            ((*vocp).voc_rekey_func).expect("non-null function pointer"),
                                                        ))
                                                            .expect("non-null function pointer")(vocp);
                                                        match tmp___7 {
                                                            1 => {}
                                                            0 => {
                                                                current_block = 1587619384396752891;
                                                                break;
                                                            }
                                                            _ => {
                                                                current_block = 2684075921598956718;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                    vg_exec_context_upgrade_lock(vxcp);
                                                    pattern_generation = (*vcp).vc_pattern_generation;
                                                    EC_KEY_generate_key(pkey);
                                                    npoints = 0 as libc::c_ulong;
                                                    EC_GROUP_get_order(
                                                        pgroup,
                                                        &mut (*vxcp).vxc_bntmp,
                                                        (*vxcp).vxc_bnctx,
                                                    );
                                                    tmp___8 = EC_KEY_get0_private_key(pkey as *const EC_KEY);
                                                    BN_sub(
                                                        &mut (*vxcp).vxc_bntmp2,
                                                        &mut (*vxcp).vxc_bntmp as *mut BIGNUM as *const BIGNUM,
                                                        tmp___8,
                                                    );
                                                    rekey_at = BN_get_word(
                                                        &mut (*vxcp).vxc_bntmp2 as *mut BIGNUM as *const BIGNUM,
                                                    );
                                                    if rekey_at as libc::c_ulonglong
                                                        == 18446744073709551615 as libc::c_ulonglong
                                                    {
                                                        rekey_at = rekey_max;
                                                    } else if rekey_at > rekey_max {
                                                        rekey_at = rekey_max;
                                                    }
                                                    if !(rekey_at > 0 as libc::c_ulong) {
                                                        __assert_fail(
                                                            b"rekey_at > 0\0" as *const u8 as *const libc::c_char,
                                                            b"oclengine.c\0" as *const u8 as *const libc::c_char,
                                                            2040 as libc::c_uint,
                                                            b"vg_opencl_loop\0" as *const u8 as *const libc::c_char,
                                                        );
                                                    }
                                                    tmp___10 = EC_KEY_get0_public_key(pkey as *const EC_KEY);
                                                    EC_POINT_copy(
                                                        *ppbase.offset(0 as libc::c_int as isize),
                                                        tmp___10,
                                                    );
                                                    vg_exec_context_downgrade_lock(vxcp);
                                                    if !((*vcp).vc_pubkey_base).is_null() {
                                                        EC_POINT_add(
                                                            pgroup,
                                                            *ppbase.offset(0 as libc::c_int as isize),
                                                            *ppbase.offset(0 as libc::c_int as isize)
                                                                as *const EC_POINT,
                                                            (*vcp).vc_pubkey_base as *const EC_POINT,
                                                            (*vxcp).vxc_bnctx,
                                                        );
                                                    }
                                                    i = 1 as libc::c_int;
                                                    while i < ncols {
                                                        EC_POINT_add(
                                                            pgroup,
                                                            *ppbase.offset(i as isize),
                                                            *ppbase.offset((i - 1 as libc::c_int) as isize)
                                                                as *const EC_POINT,
                                                            pgen,
                                                            (*vxcp).vxc_bnctx,
                                                        );
                                                        i += 1;
                                                    }
                                                    EC_POINTs_make_affine(
                                                        pgroup,
                                                        ncols as size_t,
                                                        ppbase,
                                                        (*vxcp).vxc_bnctx,
                                                    );
                                                    tmp___11 = vg_ocl_map_arg_buffer(
                                                        vocp,
                                                        0 as libc::c_int,
                                                        3 as libc::c_int,
                                                        1 as libc::c_int,
                                                    );
                                                    ocl_points_in = tmp___11 as *mut libc::c_uchar;
                                                    if ocl_points_in.is_null() {
                                                        fprintf(
                                                            stderr,
                                                            b"ERROR: Could not map column buffer\n\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        current_block = 2684075921598956718;
                                                        break;
                                                    } else {
                                                        i = 0 as libc::c_int;
                                                        while i < ncols {
                                                            vg_ocl_put_point_tpa(
                                                                ocl_points_in,
                                                                i,
                                                                *ppbase.offset(i as isize) as *const EC_POINT,
                                                            );
                                                            i += 1;
                                                        }
                                                        vg_ocl_unmap_arg_buffer(
                                                            vocp,
                                                            0 as libc::c_int,
                                                            3 as libc::c_int,
                                                            ocl_points_in as *mut libc::c_void,
                                                        );
                                                        EC_POINT_copy(
                                                            *pprow.offset(0 as libc::c_int as isize),
                                                            pgen,
                                                        );
                                                        i = 1 as libc::c_int;
                                                        while i < nrows {
                                                            EC_POINT_add(
                                                                pgroup,
                                                                *pprow.offset(i as isize),
                                                                *pprow.offset((i - 1 as libc::c_int) as isize)
                                                                    as *const EC_POINT,
                                                                pbatchinc as *const EC_POINT,
                                                                (*vxcp).vxc_bnctx,
                                                            );
                                                            i += 1;
                                                        }
                                                        EC_POINTs_make_affine(
                                                            pgroup,
                                                            nrows as size_t,
                                                            pprow,
                                                            (*vxcp).vxc_bnctx,
                                                        );
                                                        (*vxcp).vxc_delta = 1 as libc::c_int;
                                                        npoints = 1 as libc::c_ulong;
                                                        slot = 0 as libc::c_int;
                                                        slot_busy = 0 as libc::c_int;
                                                        slot_done = 0 as libc::c_int;
                                                        loop {
                                                            if slot_done != 0 {
                                                                if !(rekey_at > 0 as libc::c_ulong) {
                                                                    __assert_fail(
                                                                        b"rekey_at > 0\0" as *const u8 as *const libc::c_char,
                                                                        b"oclengine.c\0" as *const u8 as *const libc::c_char,
                                                                        2096 as libc::c_uint,
                                                                        b"vg_opencl_loop\0" as *const u8 as *const libc::c_char,
                                                                    );
                                                                }
                                                                slot_done = 0 as libc::c_int;
                                                                tmp___13 = (Some(
                                                                    ((*vocp).voc_check_func).expect("non-null function pointer"),
                                                                ))
                                                                    .expect("non-null function pointer")(vocp, slot);
                                                                match tmp___13 {
                                                                    1 => {
                                                                        rekey_at = 0 as libc::c_ulong;
                                                                    }
                                                                    2 => {
                                                                        halt = 1 as libc::c_int;
                                                                    }
                                                                    _ => {}
                                                                }
                                                                c += round___0;
                                                                if halt == 0 {
                                                                    if c >= output_interval {
                                                                        output_interval = vg_output_timing(vcp, c, &mut tvstart);
                                                                        c = 0 as libc::c_int;
                                                                    }
                                                                }
                                                                vg_exec_context_yield(vxcp);
                                                                if ((*vocp).voc_rekey_func).is_some() {
                                                                    if pattern_generation != (*vcp).vc_pattern_generation {
                                                                        (*vocp).voc_pattern_rewrite = 1 as libc::c_int;
                                                                        rekey_at = 0 as libc::c_ulong;
                                                                    }
                                                                }
                                                            }
                                                            if (*vcp).vc_halt != 0 {
                                                                halt = 1 as libc::c_int;
                                                            }
                                                            if halt != 0 {
                                                                current_block = 1587619384396752891;
                                                                break '_l_rekey;
                                                            }
                                                            if npoints.wrapping_add(round___0 as libc::c_ulong)
                                                                < rekey_at
                                                            {
                                                                if npoints > 1 as libc::c_ulong {
                                                                    i = 0 as libc::c_int;
                                                                    while i < nrows {
                                                                        EC_POINT_add(
                                                                            pgroup,
                                                                            *pprow.offset(i as isize),
                                                                            *pprow.offset(i as isize) as *const EC_POINT,
                                                                            poffset as *const EC_POINT,
                                                                            (*vxcp).vxc_bnctx,
                                                                        );
                                                                        i += 1;
                                                                    }
                                                                    EC_POINTs_make_affine(
                                                                        pgroup,
                                                                        nrows as size_t,
                                                                        pprow,
                                                                        (*vxcp).vxc_bnctx,
                                                                    );
                                                                }
                                                                tmp___14 = vg_ocl_map_arg_buffer(
                                                                    vocp,
                                                                    slot,
                                                                    4 as libc::c_int,
                                                                    1 as libc::c_int,
                                                                );
                                                                ocl_strides_in = tmp___14 as *mut libc::c_uchar;
                                                                if ocl_strides_in.is_null() {
                                                                    fprintf(
                                                                        stderr,
                                                                        b"ERROR: Could not map row buffer for slot %d\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        slot,
                                                                    );
                                                                    current_block = 2684075921598956718;
                                                                    break '_l_rekey;
                                                                } else {
                                                                    memset(
                                                                        ocl_strides_in as *mut libc::c_void,
                                                                        0 as libc::c_int,
                                                                        (64 as libc::c_int * nrows) as size_t as _,
                                                                    );
                                                                    i = 0 as libc::c_int;
                                                                    while i < nrows {
                                                                        vg_ocl_put_point(
                                                                            ocl_strides_in.offset((64 as libc::c_int * i) as isize),
                                                                            *pprow.offset(i as isize) as *const EC_POINT,
                                                                        );
                                                                        i += 1;
                                                                    }
                                                                    vg_ocl_unmap_arg_buffer(
                                                                        vocp,
                                                                        slot,
                                                                        4 as libc::c_int,
                                                                        ocl_strides_in as *mut libc::c_void,
                                                                    );
                                                                    npoints = npoints.wrapping_add(round___0 as libc::c_ulong);
                                                                    pthread_mutex_lock(&mut (*vocp).voc_lock);
                                                                    while (*vocp).voc_ocl_slot != -(1 as libc::c_int) {
                                                                        if slot_busy == 0 {
                                                                            __assert_fail(
                                                                                b"slot_busy\0" as *const u8 as *const libc::c_char,
                                                                                b"oclengine.c\0" as *const u8 as *const libc::c_char,
                                                                                2166 as libc::c_uint,
                                                                                b"vg_opencl_loop\0" as *const u8 as *const libc::c_char,
                                                                            );
                                                                        }
                                                                        pthread_cond_wait(
                                                                            &mut (*vocp).voc_wait as *mut pthread_cond_t,
                                                                            &mut (*vocp).voc_lock as *mut pthread_mutex_t,
                                                                        );
                                                                    }
                                                                    if (*vocp).voc_halt != 0 {
                                                                        pthread_mutex_unlock(&mut (*vocp).voc_lock);
                                                                        halt = 1 as libc::c_int;
                                                                        current_block = 1587619384396752891;
                                                                        break '_l_rekey;
                                                                    } else {
                                                                        (*vocp).voc_ocl_slot = slot;
                                                                        pthread_cond_signal(&mut (*vocp).voc_wait);
                                                                        pthread_mutex_unlock(&mut (*vocp).voc_lock);
                                                                        slot_done = slot_busy;
                                                                        slot_busy = 1 as libc::c_int;
                                                                        slot = (slot + 1 as libc::c_int) % nslots;
                                                                    }
                                                                }
                                                            } else {
                                                                if slot_busy != 0 {
                                                                    pthread_mutex_lock(&mut (*vocp).voc_lock);
                                                                    while (*vocp).voc_ocl_slot != -(1 as libc::c_int) {
                                                                        if !((*vocp).voc_ocl_slot
                                                                            == (slot + nslots - 1 as libc::c_int) % nslots)
                                                                        {
                                                                            __assert_fail(
                                                                                b"vocp->voc_ocl_slot == ((slot + nslots - 1) % nslots)\0"
                                                                                    as *const u8 as *const libc::c_char,
                                                                                b"oclengine.c\0" as *const u8 as *const libc::c_char,
                                                                                2189 as libc::c_uint,
                                                                                b"vg_opencl_loop\0" as *const u8 as *const libc::c_char,
                                                                            );
                                                                        }
                                                                        pthread_cond_wait(
                                                                            &mut (*vocp).voc_wait as *mut pthread_cond_t,
                                                                            &mut (*vocp).voc_lock as *mut pthread_mutex_t,
                                                                        );
                                                                    }
                                                                    pthread_mutex_unlock(&mut (*vocp).voc_lock);
                                                                    slot_busy = 0 as libc::c_int;
                                                                    slot_done = 1 as libc::c_int;
                                                                }
                                                                if rekey_at == 0 {
                                                                    break;
                                                                }
                                                                if !(slot_done == 0) {
                                                                    continue;
                                                                }
                                                                if npoints.wrapping_add(round___0 as libc::c_ulong)
                                                                    >= rekey_at
                                                                {
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        current_block = 2684075921598956718;
                                    }
                                } else {
                                    current_block = 2684075921598956718;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        2684075921598956718 => {
            fprintf(
                stderr,
                b"ERROR: allocation failure?\n\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {}
    }
    if halt != 0 {
        if (*vcp).vc_verbose > 1 as libc::c_int {
            printf(b"Halting...\0" as *const u8 as *const libc::c_char);
            fflush(stdout);
        }
        pthread_mutex_lock(&mut (*vocp).voc_lock);
        (*vocp).voc_halt = 1 as libc::c_int;
        pthread_cond_signal(&mut (*vocp).voc_wait);
        while (*vocp).voc_ocl_slot != -(1 as libc::c_int) {
            if slot_busy == 0 {
                __assert_fail(
                    b"slot_busy\0" as *const u8 as *const libc::c_char,
                    b"oclengine.c\0" as *const u8 as *const libc::c_char,
                    2221 as libc::c_uint,
                    b"vg_opencl_loop\0" as *const u8 as *const libc::c_char,
                );
            }
            pthread_cond_wait(
                &mut (*vocp).voc_wait as *mut pthread_cond_t,
                &mut (*vocp).voc_lock as *mut pthread_mutex_t,
            );
        }
        slot_busy = 0 as libc::c_int;
        pthread_mutex_unlock(&mut (*vocp).voc_lock);
        pthread_join(
            (*vocp).voc_ocl_thread,
            0 as *mut libc::c_void as *mut *mut libc::c_void,
        );
        if (*vcp).vc_verbose > 1 as libc::c_int {
            printf(b"done!\n\0" as *const u8 as *const libc::c_char);
        }
    }
    vg_exec_context_yield(vxcp);
    if !ppbase.is_null() {
        i = 0 as libc::c_int;
        while i < nrows + ncols {
            if !(*ppbase.offset(i as isize)).is_null() {
                EC_POINT_free(*ppbase.offset(i as isize));
            }
            i += 1;
        }
        free(ppbase as *mut libc::c_void);
    }
    if !pbatchinc.is_null() {
        EC_POINT_free(pbatchinc);
    }
    vg_ocl_free_args(vocp);
    (*vocp).voc_halt = 0 as libc::c_int;
    (*vocp).voc_ocl_slot = -(1 as libc::c_int);
    vg_context_thread_exit(vcp);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn get_device_list(
    mut pid: cl_platform_id,
    mut list_out: *mut *mut cl_device_id,
) -> libc::c_int {
    let mut nd: cl_uint = 0;
    let mut res: cl_int = 0;
    let mut ids: *mut cl_device_id = 0 as *mut cl_device_id;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    res = clGetDeviceIDs(
        pid,
        4294967295 as libc::c_uint as cl_device_type,
        0 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *mut cl_device_id,
        &mut nd,
    );
    if res != 0 as libc::c_int {
        vg_ocl_error(
            0 as *mut libc::c_void as *mut vg_ocl_context_t,
            res,
            b"clGetDeviceIDs(0)\0" as *const u8 as *const libc::c_char,
        );
        *list_out = 0 as *mut libc::c_void as *mut cl_device_id;
        return -(1 as libc::c_int);
    }
    if nd != 0 {
        tmp___0 = malloc(
            (nd as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cl_device_id>() as libc::c_ulong),
        );
        ids = tmp___0 as *mut cl_device_id;
        if ids as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            fprintf(
                stderr,
                b"Could not allocate device ID list\n\0" as *const u8
                    as *const libc::c_char,
            );
            *list_out = 0 as *mut libc::c_void as *mut cl_device_id;
            return -(1 as libc::c_int);
        }
        res = clGetDeviceIDs(
            pid,
            4294967295 as libc::c_uint as cl_device_type,
            nd,
            ids,
            0 as *mut libc::c_void as *mut cl_uint,
        );
        if res != 0 as libc::c_int {
            vg_ocl_error(
                0 as *mut libc::c_void as *mut vg_ocl_context_t,
                res,
                b"clGetDeviceIDs(n)\0" as *const u8 as *const libc::c_char,
            );
            free(ids as *mut libc::c_void);
            *list_out = 0 as *mut libc::c_void as *mut cl_device_id;
            return -(1 as libc::c_int);
        }
        *list_out = ids;
    }
    return nd as libc::c_int;
}
unsafe extern "C" fn show_devices(
    mut pid: cl_platform_id,
    mut ids: *mut cl_device_id,
    mut nd: libc::c_int,
    mut base: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut nbuf: [libc::c_char; 128] = [0; 128];
    let mut vbuf: [libc::c_char; 128] = [0; 128];
    let mut len: size_t = 0;
    let mut res: cl_int = 0;
    i = 0 as libc::c_int;
    while i < nd {
        res = clGetDeviceInfo(
            *ids.offset(i as isize),
            4139 as libc::c_int as cl_device_info,
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            nbuf.as_mut_ptr() as *mut libc::c_void,
            &mut len,
        );
        if !(res != 0 as libc::c_int) {
            if len >= ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong {
                len = (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong);
            }
            nbuf[len as usize] = '\u{0}' as i32 as libc::c_char;
            res = clGetDeviceInfo(
                *ids.offset(i as isize),
                4140 as libc::c_int as cl_device_info,
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                vbuf.as_mut_ptr() as *mut libc::c_void,
                &mut len,
            );
            if !(res != 0 as libc::c_int) {
                if len >= ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong {
                    len = (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_ulong);
                }
                vbuf[len as usize] = '\u{0}' as i32 as libc::c_char;
                fprintf(
                    stderr,
                    b"  %d: [%s] %s\n\0" as *const u8 as *const libc::c_char,
                    i + base,
                    vbuf.as_mut_ptr(),
                    nbuf.as_mut_ptr(),
                );
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn get_device(
    mut pid: cl_platform_id,
    mut num: libc::c_int,
) -> cl_device_id {
    let mut nd: libc::c_int = 0;
    let mut id: cl_device_id = 0 as *mut _cl_device_id;
    let mut ids: *mut cl_device_id = 0 as *mut cl_device_id;
    nd = get_device_list(pid, &mut ids);
    if nd < 0 as libc::c_int {
        return 0 as *mut libc::c_void as cl_device_id;
    }
    if nd == 0 {
        fprintf(
            stderr,
            b"No OpenCL devices found\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void as cl_device_id;
    }
    if num < 0 as libc::c_int {
        if nd == 1 as libc::c_int {
            num = 0 as libc::c_int;
        } else {
            num = nd;
        }
    }
    if num < nd {
        id = *ids.offset(num as isize);
        free(ids as *mut libc::c_void);
        return id;
    }
    free(ids as *mut libc::c_void);
    return 0 as *mut libc::c_void as cl_device_id;
}
unsafe extern "C" fn get_platform_list(
    mut list_out: *mut *mut cl_platform_id,
) -> libc::c_int {
    let mut np: cl_uint = 0;
    let mut res: cl_int = 0;
    let mut ids: *mut cl_platform_id = 0 as *mut cl_platform_id;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    res = clGetPlatformIDs(
        0 as libc::c_int as cl_uint,
        0 as *mut libc::c_void as *mut cl_platform_id,
        &mut np,
    );
    if res != 0 as libc::c_int {
        vg_ocl_error(
            0 as *mut libc::c_void as *mut vg_ocl_context_t,
            res,
            b"clGetPlatformIDs(0)\0" as *const u8 as *const libc::c_char,
        );
        *list_out = 0 as *mut libc::c_void as *mut cl_platform_id;
        return -(1 as libc::c_int);
    }
    if np != 0 {
        tmp___0 = malloc(
            (np as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cl_platform_id>() as libc::c_ulong),
        );
        ids = tmp___0 as *mut cl_platform_id;
        if ids as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            fprintf(
                stderr,
                b"Could not allocate platform ID list\n\0" as *const u8
                    as *const libc::c_char,
            );
            *list_out = 0 as *mut libc::c_void as *mut cl_platform_id;
            return -(1 as libc::c_int);
        }
        res = clGetPlatformIDs(np, ids, 0 as *mut libc::c_void as *mut cl_uint);
        if res != 0 as libc::c_int {
            vg_ocl_error(
                0 as *mut libc::c_void as *mut vg_ocl_context_t,
                res,
                b"clGetPlatformIDs(n)\0" as *const u8 as *const libc::c_char,
            );
            free(ids as *mut libc::c_void);
            *list_out = 0 as *mut libc::c_void as *mut cl_platform_id;
            return -(1 as libc::c_int);
        }
        *list_out = ids;
    }
    return np as libc::c_int;
}
pub unsafe extern "C" fn show_platforms(
    mut ids: *mut cl_platform_id,
    mut np: libc::c_int,
    mut base: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut nbuf: [libc::c_char; 128] = [0; 128];
    let mut vbuf: [libc::c_char; 128] = [0; 128];
    let mut len: size_t = 0;
    let mut res: cl_int = 0;
    i = 0 as libc::c_int;
    while i < np {
        res = clGetPlatformInfo(
            *ids.offset(i as isize),
            2306 as libc::c_int as cl_platform_info,
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            nbuf.as_mut_ptr() as *mut libc::c_void,
            &mut len,
        );
        if res != 0 as libc::c_int {
            vg_ocl_error(
                0 as *mut libc::c_void as *mut vg_ocl_context_t,
                res,
                b"clGetPlatformInfo(NAME)\0" as *const u8 as *const libc::c_char,
            );
        } else {
            if len >= ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong {
                len = (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_ulong);
            }
            nbuf[len as usize] = '\u{0}' as i32 as libc::c_char;
            res = clGetPlatformInfo(
                *ids.offset(i as isize),
                2307 as libc::c_int as cl_platform_info,
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                vbuf.as_mut_ptr() as *mut libc::c_void,
                &mut len,
            );
            if res != 0 as libc::c_int {
                vg_ocl_error(
                    0 as *mut libc::c_void as *mut vg_ocl_context_t,
                    res,
                    b"clGetPlatformInfo(VENDOR)\0" as *const u8 as *const libc::c_char,
                );
            } else {
                if len >= ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong {
                    len = (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_ulong);
                }
                vbuf[len as usize] = '\u{0}' as i32 as libc::c_char;
                fprintf(
                    stderr,
                    b"%d: [%s] %s\n\0" as *const u8 as *const libc::c_char,
                    i + base,
                    vbuf.as_mut_ptr(),
                    nbuf.as_mut_ptr(),
                );
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn get_platform(mut num: libc::c_int) -> cl_platform_id {
    let mut np: libc::c_int = 0;
    let mut id: cl_platform_id = 0 as *mut _cl_platform_id;
    let mut ids: *mut cl_platform_id = 0 as *mut cl_platform_id;
    np = get_platform_list(&mut ids);
    if np < 0 as libc::c_int {
        return 0 as *mut libc::c_void as cl_platform_id;
    }
    if np == 0 {
        fprintf(
            stderr,
            b"No OpenCL platforms available\n\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void as cl_platform_id;
    }
    if num < 0 as libc::c_int {
        if np == 1 as libc::c_int {
            num = 0 as libc::c_int;
        } else {
            num = np;
        }
    }
    if num < np {
        id = *ids.offset(num as isize);
        free(ids as *mut libc::c_void);
        return id;
    }
    free(ids as *mut libc::c_void);
    return 0 as *mut libc::c_void as cl_platform_id;
}
pub unsafe extern "C" fn vg_ocl_enumerate_devices() {
    let mut pids: *mut cl_platform_id = 0 as *mut cl_platform_id;
    let mut dids: *mut cl_device_id = 0 as *mut cl_device_id;
    let mut np: libc::c_int = 0;
    let mut nd: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    np = get_platform_list(&mut pids);
    if np == 0 {
        fprintf(
            stderr,
            b"No OpenCL platforms available\n\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    fprintf(
        stderr,
        b"Available OpenCL platforms:\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < np {
        show_platforms(pids.offset(i as isize), 1 as libc::c_int, i);
        nd = get_device_list(*pids.offset(i as isize), &mut dids);
        if nd == 0 {
            fprintf(stderr, b"  -- No devices\n\0" as *const u8 as *const libc::c_char);
        } else {
            show_devices(*pids.offset(i as isize), dids, nd, 0 as libc::c_int);
        }
        i += 1;
    }
}
unsafe extern "C" fn get_opencl_device(
    mut platformidx: libc::c_int,
    mut deviceidx: libc::c_int,
) -> cl_device_id {
    let mut pid: cl_platform_id = 0 as *mut _cl_platform_id;
    let mut did: cl_device_id = 0 as *mut _cl_device_id;
    did = 0 as *mut libc::c_void as cl_device_id;
    pid = get_platform(platformidx);
    if !pid.is_null() {
        did = get_device(pid, deviceidx);
        if !did.is_null() {
            return did;
        }
    }
    return 0 as *mut libc::c_void as cl_device_id;
}
pub unsafe extern "C" fn vg_ocl_context_new(
    mut vcp: *mut vg_context_t,
    mut platformidx: libc::c_int,
    mut deviceidx: libc::c_int,
    mut safe_mode: libc::c_int,
    mut verify: libc::c_int,
    mut worksize: libc::c_int,
    mut nthreads: libc::c_int,
    mut nrows: libc::c_int,
    mut ncols: libc::c_int,
    mut invsize: libc::c_int,
) -> *mut vg_ocl_context_t {
    let mut did: cl_device_id = 0 as *mut _cl_device_id;
    let mut round___0: libc::c_int = 0;
    let mut full_threads: libc::c_int = 0;
    let mut wsmult: libc::c_int = 0;
    let mut memsize: cl_ulong = 0;
    let mut allocsize: cl_ulong = 0;
    let mut vocp: *mut vg_ocl_context_t = 0 as *mut vg_ocl_context_t;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: cl_device_type = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: cl_device_type = 0;
    did = get_opencl_device(platformidx, deviceidx);
    if did.is_null() {
        return 0 as *mut vg_ocl_context_t;
    }
    tmp___0 = malloc(::std::mem::size_of::<vg_ocl_context_t>() as libc::c_ulong);
    vocp = tmp___0 as *mut vg_ocl_context_t;
    if vocp.is_null() {
        return 0 as *mut libc::c_void as *mut vg_ocl_context_t;
    }
    tmp___1 = vg_ocl_init(vcp, vocp, did, safe_mode);
    if tmp___1 == 0 {
        free(vocp as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut vg_ocl_context_t;
    }
    if verify != 0 {
        if (*vcp).vc_verbose > 0 as libc::c_int {
            fprintf(
                stderr,
                b"WARNING: Hardware verification mode enabled\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if nthreads == 0 {
            nthreads = 1 as libc::c_int;
        }
        (*vocp)
            .voc_verify_func[0 as libc::c_int
            as usize] = Some(
            vg_ocl_verify_k0
                as unsafe extern "C" fn(
                    *mut vg_ocl_context_t,
                    libc::c_int,
                ) -> libc::c_int,
        );
        (*vocp)
            .voc_verify_func[1 as libc::c_int
            as usize] = Some(
            vg_ocl_verify_k1
                as unsafe extern "C" fn(
                    *mut vg_ocl_context_t,
                    libc::c_int,
                ) -> libc::c_int,
        );
    }
    if nthreads == 0 {
        tmp___3 = vg_ocl_device_gettype((*vocp).voc_ocldid);
        if tmp___3 & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
            nthreads = 1 as libc::c_int;
        } else {
            tmp___2 = vg_ocl_device_getsizet(
                (*vocp).voc_ocldid,
                4100 as libc::c_int as cl_device_info,
            );
            nthreads = tmp___2 as libc::c_int;
        }
    }
    tmp___4 = vg_ocl_device_getsizet(
        (*vocp).voc_ocldid,
        4098 as libc::c_int as cl_device_info,
    );
    full_threads = tmp___4 as libc::c_int;
    full_threads *= nthreads;
    if worksize == 0 {
        tmp___5 = vg_ocl_device_gettype((*vocp).voc_ocldid);
        if tmp___5 & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0 {
            worksize = 2048 as libc::c_int;
        } else {
            worksize = 256 as libc::c_int;
        }
    }
    if ncols == 0 {
        memsize = vg_ocl_device_getulong(
            (*vocp).voc_ocldid,
            4127 as libc::c_int as cl_device_info,
        );
        allocsize = vg_ocl_device_getulong(
            (*vocp).voc_ocldid,
            4112 as libc::c_int as cl_device_info,
        );
        memsize = (memsize as libc::c_ulong).wrapping_div(2 as libc::c_ulong) as cl_ulong
            as cl_ulong;
        ncols = full_threads;
        nrows = 2 as libc::c_int;
        while ncols > nrows {
            if ncols & 1 as libc::c_int != 0 {
                break;
            }
            ncols /= 2 as libc::c_int;
            nrows *= 2 as libc::c_int;
        }
        wsmult = 1 as libc::c_int;
        loop {
            if !(worksize == 0) {
                if !(wsmult * 2 as libc::c_int <= worksize) {
                    break;
                }
            }
            if !(((ncols * nrows * 2 as libc::c_int * 128 as libc::c_int) as cl_ulong)
                < memsize)
            {
                break;
            }
            if !(((ncols * nrows * 2 as libc::c_int * 64 as libc::c_int) as cl_ulong)
                < allocsize)
            {
                break;
            }
            if ncols > nrows {
                nrows *= 2 as libc::c_int;
            } else {
                ncols *= 2 as libc::c_int;
            }
            wsmult *= 2 as libc::c_int;
        }
    }
    round___0 = nrows * ncols;
    if invsize == 0 {
        invsize = 2 as libc::c_int;
        while round___0 % (invsize << 1 as libc::c_int) == 0 {
            if !(round___0 / invsize > full_threads) {
                break;
            }
            invsize <<= 1 as libc::c_int;
        }
    }
    if (*vcp).vc_verbose > 1 as libc::c_int {
        fprintf(
            stderr,
            b"Grid size: %dx%d\n\0" as *const u8 as *const libc::c_char,
            ncols,
            nrows,
        );
        fprintf(
            stderr,
            b"Modular inverse: %d threads, %d ops each\n\0" as *const u8
                as *const libc::c_char,
            round___0 / invsize,
            invsize,
        );
    }
    if !(round___0 % invsize != 0) {
        if !(invsize & invsize - 1 as libc::c_int != 0) {
            if !(invsize < 2 as libc::c_int) {
                (*vocp).voc_ocl_rows = nrows;
                (*vocp).voc_ocl_cols = ncols;
                (*vocp).voc_ocl_invsize = invsize;
                return vocp;
            }
        }
    }
    if (*vcp).vc_verbose <= 1 as libc::c_int {
        fprintf(
            stderr,
            b"Grid size: %dx%d\n\0" as *const u8 as *const libc::c_char,
            ncols,
            nrows,
        );
        fprintf(
            stderr,
            b"Modular inverse: %d threads, %d ops each\n\0" as *const u8
                as *const libc::c_char,
            round___0 / invsize,
            invsize,
        );
    }
    if round___0 % invsize != 0 {
        fprintf(
            stderr,
            b"Modular inverse work size must evenly divide points\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        fprintf(
            stderr,
            b"Modular inverse work per task (%d) must be a power of 2\n\0" as *const u8
                as *const libc::c_char,
            invsize,
        );
    }
    vg_ocl_context_free(vocp);
    return 0 as *mut libc::c_void as *mut vg_ocl_context_t;
}
pub unsafe extern "C" fn vg_ocl_context_new_from_devstr(
    mut vcp: *mut vg_context_t,
    mut devstr: *const libc::c_char,
    mut safemode: libc::c_int,
    mut verify: libc::c_int,
) -> *mut vg_ocl_context_t {
    let mut platformidx: libc::c_int = 0;
    let mut deviceidx: libc::c_int = 0;
    let mut worksize: libc::c_int = 0;
    let mut nthreads: libc::c_int = 0;
    let mut nrows: libc::c_int = 0;
    let mut ncols: libc::c_int = 0;
    let mut invsize: libc::c_int = 0;
    let mut dsd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut part: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut part2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut param: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: libc::c_long = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: *mut vg_ocl_context_t = 0 as *mut vg_ocl_context_t;
    worksize = 0 as libc::c_int;
    nthreads = 0 as libc::c_int;
    nrows = 0 as libc::c_int;
    ncols = 0 as libc::c_int;
    invsize = 0 as libc::c_int;
    dsd = strdup(devstr);
    if dsd.is_null() {
        return 0 as *mut libc::c_void as *mut vg_ocl_context_t;
    }
    save = 0 as *mut libc::c_void as *mut libc::c_char;
    part = strtok_r(
        dsd,
        b",\0" as *const u8 as *const libc::c_char,
        &mut save as *mut *mut libc::c_char,
    );
    part2 = strchr(part as *const libc::c_char, ':' as i32);
    if part2.is_null() {
        fprintf(
            stderr,
            b"Invalid device specifier '%s'\n\0" as *const u8 as *const libc::c_char,
            part,
        );
        free(dsd as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut vg_ocl_context_t;
    }
    *part2 = '\u{0}' as i32 as libc::c_char;
    platformidx = atoi(part as *const libc::c_char);
    deviceidx = atoi(part2.offset(1 as libc::c_int as isize) as *const libc::c_char);
    loop {
        part = strtok_r(
            0 as *mut libc::c_void as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
            &mut save as *mut *mut libc::c_char,
        );
        if !(part as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        param = strchr(part as *const libc::c_char, '=' as i32);
        if param.is_null() {
            fprintf(
                stderr,
                b"Unrecognized parameter '%s'\n\0" as *const u8 as *const libc::c_char,
                part,
            );
        } else {
            *param = '\u{0}' as i32 as libc::c_char;
            param = param.offset(1);
            tmp___4 = strcmp(
                part as *const libc::c_char,
                b"grid\0" as *const u8 as *const libc::c_char,
            );
            if tmp___4 != 0 {
                tmp___3 = strcmp(
                    part as *const libc::c_char,
                    b"invsize\0" as *const u8 as *const libc::c_char,
                );
                if tmp___3 != 0 {
                    tmp___2 = strcmp(
                        part as *const libc::c_char,
                        b"threads\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___2 != 0 {
                        fprintf(
                            stderr,
                            b"Unrecognized parameter '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            part,
                        );
                    } else {
                        nthreads = atoi(param as *const libc::c_char);
                        if !(nthreads == 0 as libc::c_int) {
                            continue;
                        }
                        fprintf(
                            stderr,
                            b"Invalid thread count '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            param,
                        );
                    }
                } else {
                    invsize = atoi(param as *const libc::c_char);
                    if invsize == 0 {
                        fprintf(
                            stderr,
                            b"Invalid modular inverse size '%s'\n\0" as *const u8
                                as *const libc::c_char,
                            param,
                        );
                    } else {
                        if !(invsize & invsize - 1 as libc::c_int != 0) {
                            continue;
                        }
                        fprintf(
                            stderr,
                            b"Modular inverse size %d must be a power of 2\n\0"
                                as *const u8 as *const libc::c_char,
                            invsize,
                        );
                        invsize = 0 as libc::c_int;
                    }
                }
            } else {
                tmp___0 = strtol(
                    param as *const libc::c_char,
                    &mut part2 as *mut *mut libc::c_char,
                    0 as libc::c_int,
                );
                ncols = tmp___0 as libc::c_int;
                if !part2.is_null() {
                    if *part2 as libc::c_int == 120 as libc::c_int {
                        tmp___1 = strtol(
                            part2.offset(1 as libc::c_int as isize)
                                as *const libc::c_char,
                            0 as *mut libc::c_void as *mut *mut libc::c_char,
                            0 as libc::c_int,
                        );
                        nrows = tmp___1 as libc::c_int;
                    }
                }
                if nrows == 0 {
                    fprintf(
                        stderr,
                        b"Invalid grid size '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        param,
                    );
                    nrows = 0 as libc::c_int;
                    ncols = 0 as libc::c_int;
                } else {
                    if !(ncols == 0) {
                        continue;
                    }
                    fprintf(
                        stderr,
                        b"Invalid grid size '%s'\n\0" as *const u8
                            as *const libc::c_char,
                        param,
                    );
                    nrows = 0 as libc::c_int;
                    ncols = 0 as libc::c_int;
                }
            }
        }
    }
    free(dsd as *mut libc::c_void);
    tmp___5 = vg_ocl_context_new(
        vcp,
        platformidx,
        deviceidx,
        safemode,
        verify,
        worksize,
        nthreads,
        nrows,
        ncols,
        invsize,
    );
    return tmp___5;
}
pub unsafe extern "C" fn vg_ocl_context_free(mut vocp: *mut vg_ocl_context_t) {
    vg_ocl_del(vocp);
    free(vocp as *mut libc::c_void);
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
    let mut tmp___0: *mut _avl_item_s = 0 as *mut _avl_item_s;
    parentp = (*itemp).ai_up;
    tmp___0 = 0 as *mut libc::c_void as *mut _avl_item_s;
    (*itemp).ai_right = tmp___0;
    (*itemp).ai_left = tmp___0;
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
    let mut tmp___0: *mut EC_KEY = 0 as *mut EC_KEY;
    tmp___0 = EC_KEY_new_by_curve_name(714 as libc::c_int);
    return tmp___0;
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
        ::std::mem::size_of::<vg_exec_context_t>() as libc::c_ulong as _,
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
    let mut tmp___0: *const BIGNUM = 0 as *const BIGNUM;
    if (*vxcp).vxc_delta != 0 {
        BN_clear(&mut (*vxcp).vxc_bntmp);
        BN_set_word(&mut (*vxcp).vxc_bntmp, (*vxcp).vxc_delta as libc::c_ulong);
        tmp___0 = EC_KEY_get0_private_key((*vxcp).vxc_key as *const EC_KEY);
        BN_add(
            &mut (*vxcp).vxc_bntmp2,
            tmp___0,
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
    let mut tmp___0: *const EC_POINT = 0 as *const EC_POINT;
    let mut tmp___1: size_t = 0;
    vg_exec_context_consolidate_key(vxcp);
    pgroup = EC_KEY_get0_group((*vxcp).vxc_key as *const EC_KEY);
    pubkey = EC_POINT_new(pgroup);
    tmp___0 = EC_KEY_get0_public_key((*vxcp).vxc_key as *const EC_KEY);
    EC_POINT_copy(pubkey, tmp___0);
    if !((*(*vxcp).vxc_vc).vc_pubkey_base).is_null() {
        EC_POINT_add(
            pgroup,
            pubkey,
            pubkey as *const EC_POINT,
            (*(*vxcp).vxc_vc).vc_pubkey_base as *const EC_POINT,
            (*vxcp).vxc_bnctx,
        );
    }
    tmp___1 = EC_POINT_point2oct(
        pgroup,
        pubkey as *const EC_POINT,
        POINT_CONVERSION_UNCOMPRESSED,
        eckey_buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_uchar; 96]>() as libc::c_ulong,
        (*vxcp).vxc_bnctx,
    );
    len = tmp___1 as libc::c_int;
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
        20 as libc::c_int as size_t as _,
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
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
        ::std::mem::size_of::<timeval>() as libc::c_ulong as _,
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
        tmp___0 = pthread_equal((*tip).ti_thread, me);
        if tmp___0 != 0 {
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
        tmp___1 = malloc(::std::mem::size_of::<timing_info_t>() as libc::c_ulong);
        mytip = tmp___1 as *mut timing_info_t;
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
    let mut tmp___0: libc::c_int = 0;
    pthread_mutex_lock(&mut timing_mutex);
    me = pthread_self();
    ptip = &mut (*vcp).vc_timing_head;
    tip = *ptip;
    while tip as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = pthread_equal((*tip).ti_thread, me);
        if tmp___0 == 0 {
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
        rem as size_t as _,
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
                rem as size_t as _,
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
                    rem as size_t as _,
                    b"[%d%% in %e%s]\0" as *const u8 as *const libc::c_char,
                    (100 as libc::c_int as libc::c_double * targ) as libc::c_int,
                    time___0,
                    unit,
                );
            } else {
                p = snprintf(
                    &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                    rem as size_t as _,
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
                rem as size_t as _,
                b"[Found %lld/%ld]\0" as *const u8 as *const libc::c_char,
                (*vcp).vc_found,
                (*vcp).vc_npatterns_start,
            );
        } else {
            p = snprintf(
                &mut *linebuf.as_mut_ptr().offset(p as isize) as *mut libc::c_char,
                rem as size_t as _,
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
            rem as size_t as _,
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
    let mut tmp___0: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___1: *const EC_POINT = 0 as *const EC_POINT;
    let mut tmp___2: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___3: *const EC_POINT = 0 as *const EC_POINT;
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
        tmp___0 = EC_KEY_get0_group(pkey as *const EC_KEY);
        ppnt = EC_POINT_new(tmp___0);
        tmp___1 = EC_KEY_get0_public_key(pkey as *const EC_KEY);
        EC_POINT_copy(ppnt, tmp___1);
        tmp___2 = EC_KEY_get0_group(pkey as *const EC_KEY);
        EC_POINT_add(
            tmp___2,
            ppnt,
            ppnt as *const EC_POINT,
            (*vcp).vc_pubkey_base as *const EC_POINT,
            0 as *mut libc::c_void as *mut BN_CTX,
        );
        free_ppnt = 1 as libc::c_int;
        keytype = b"PrivkeyPart\0" as *const u8 as *const libc::c_char;
    } else {
        tmp___3 = EC_KEY_get0_public_key(pkey as *const EC_KEY);
        ppnt = tmp___3 as *mut EC_POINT;
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
        current_block_62 = 4411413170459848840;
    } else if (*vcp).vc_verbose > 0 as libc::c_int {
        current_block_62 = 4411413170459848840;
    } else {
        current_block_62 = 5235537862154438448;
    }
    match current_block_62 {
        4411413170459848840 => {
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
    let mut tmp___0: libc::c_int = 0;
    (*vcp).vc_pattern_generation += 1;
    tmp___0 = (Some(((*vcp).vc_add_patterns).expect("non-null function pointer")))
        .expect("non-null function pointer")(vcp, patterns, npatterns);
    return tmp___0;
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
    let mut tmp___0: libc::c_int = 0;
    if ((*vcp).vc_hash160_sort).is_none() {
        return 0 as libc::c_int;
    }
    tmp___0 = (Some(((*vcp).vc_hash160_sort).expect("non-null function pointer")))
        .expect("non-null function pointer")(vcp, buf);
    return tmp___0;
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
    let mut tmp___0: size_t = 0;
    let mut tmp___1: *const BIGNUM = 0 as *const BIGNUM;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_ulong = 0;
    let mut tmp___4: *const BIGNUM = 0 as *const BIGNUM;
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
    tmp___0 = strlen(pfx);
    p = tmp___0 as libc::c_int;
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
            current_block = 4620475327522584562;
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
                        current_block = 4620475327522584562;
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
            tmp___1 = BN_value_one();
            BN_sub(&mut bnceil, &mut bntmp as *mut BIGNUM as *const BIGNUM, tmp___1);
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
                    tmp___2 = BN_cmp(
                        bnap as *const BIGNUM,
                        &mut bnbase as *mut BIGNUM as *const BIGNUM,
                    );
                    if !(tmp___2 > 0 as libc::c_int) {
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
                tmp___3 = BN_get_word(bnap as *const BIGNUM);
                b58ceil = tmp___3 as libc::c_int;
                if b58pow - (p - zero_prefix) < 6 as libc::c_int {
                    fprintf(
                        stderr,
                        b"Prefix '%s' is too long\n\0" as *const u8
                            as *const libc::c_char,
                        pfx,
                    );
                    current_block = 4620475327522584562;
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
                    tmp___4 = BN_value_one();
                    BN_sub(
                        &mut bntmp2,
                        &mut bntmp as *mut BIGNUM as *const BIGNUM,
                        tmp___4,
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
                        tmp___6 = BN_cmp(
                            &mut bnceil as *mut BIGNUM as *const BIGNUM,
                            bnlow2 as *const BIGNUM,
                        );
                        if tmp___6 < 0 as libc::c_int {
                            check_upper = 0 as libc::c_int;
                            BN_free(bnhigh2);
                            bnhigh2 = 0 as *mut libc::c_void as *mut BIGNUM;
                            BN_free(bnlow2);
                            bnlow2 = 0 as *mut libc::c_void as *mut BIGNUM;
                        } else {
                            tmp___5 = BN_cmp(
                                &mut bnceil as *mut BIGNUM as *const BIGNUM,
                                bnhigh2 as *const BIGNUM,
                            );
                            if tmp___5 < 0 as libc::c_int {
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
                4620475327522584562 => {}
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
                            current_block = 18080013446798038906;
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
                                    current_block = 18080013446798038906;
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
                                18080013446798038906 => {}
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
                                    let ref mut fresh2 = *result
                                        .offset(0 as libc::c_int as isize);
                                    *fresh2 = bnlow;
                                    let ref mut fresh3 = *result
                                        .offset(1 as libc::c_int as isize);
                                    *fresh3 = bnhigh;
                                    let ref mut fresh4 = *result
                                        .offset(2 as libc::c_int as isize);
                                    *fresh4 = bnlow2;
                                    let ref mut fresh5 = *result
                                        .offset(3 as libc::c_int as isize);
                                    *fresh5 = bnhigh2;
                                    bnlow = 0 as *mut libc::c_void as *mut BIGNUM;
                                    bnhigh = 0 as *mut libc::c_void as *mut BIGNUM;
                                    bnlow2 = 0 as *mut libc::c_void as *mut BIGNUM;
                                    bnhigh2 = 0 as *mut libc::c_void as *mut BIGNUM;
                                    ret = 0 as libc::c_int;
                                    current_block = 4620475327522584562;
                                }
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        4620475327522584562 => {}
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
    let ref mut fresh6 = *ranges.offset(0 as libc::c_int as isize);
    *fresh6 = 0 as *mut libc::c_void as *mut BIGNUM;
    let ref mut fresh7 = *ranges.offset(1 as libc::c_int as isize);
    *fresh7 = 0 as *mut libc::c_void as *mut BIGNUM;
    if !(*ranges.offset(2 as libc::c_int as isize)).is_null() {
        BN_free(*ranges.offset(2 as libc::c_int as isize));
        BN_free(*ranges.offset(3 as libc::c_int as isize));
        let ref mut fresh8 = *ranges.offset(2 as libc::c_int as isize);
        *fresh8 = 0 as *mut libc::c_void as *mut BIGNUM;
        let ref mut fresh9 = *ranges.offset(3 as libc::c_int as isize);
        *fresh9 = 0 as *mut libc::c_void as *mut BIGNUM;
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    itemp = (*rootp).ar_root;
    while !itemp.is_null() {
        vp = (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
        tmp___1 = BN_cmp((*vp).vp_low as *const BIGNUM, targ as *const BIGNUM);
        if tmp___1 > 0 as libc::c_int {
            itemp = (*itemp).ai_left;
        } else {
            tmp___0 = BN_cmp((*vp).vp_high as *const BIGNUM, targ as *const BIGNUM);
            if tmp___0 < 0 as libc::c_int {
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    itemp = 0 as *mut libc::c_void as *mut avl_item_t;
    ptrp = &mut (*rootp).ar_root;
    while !(*ptrp).is_null() {
        itemp = *ptrp;
        vp = (itemp as *mut libc::c_uchar)
            .offset(
                -(&mut (*(0 as *mut libc::c_uchar as *mut vg_prefix_t)).vp_item
                    as *mut avl_item_t as size_t as isize),
            ) as *mut vg_prefix_t;
        tmp___1 = BN_cmp(
            (*vp).vp_low as *const BIGNUM,
            (*vpnew).vp_high as *const BIGNUM,
        );
        if tmp___1 > 0 as libc::c_int {
            ptrp = &mut (*itemp).ai_left;
        } else {
            tmp___0 = BN_cmp(
                (*vp).vp_high as *const BIGNUM,
                (*vpnew).vp_low as *const BIGNUM,
            );
            if tmp___0 < 0 as libc::c_int {
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
    let mut tmp___0: libc::c_int = 0;
    vcpp = vcp as *mut vg_prefix_context_t;
    npfx_left = 0 as libc::c_ulong;
    loop {
        tmp___0 = avl_root_empty(&mut (*vcpp).vcp_avlroot);
        if tmp___0 != 0 {
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
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
            tmp___0 = prefix_case_iter_init(&mut caseiter, *patterns.offset(i as isize));
            if tmp___0 == 0 {
                fprintf(
                    stderr,
                    b"Prefix '%s' is too long\n\0" as *const u8 as *const libc::c_char,
                    *patterns.offset(i as isize),
                );
                current_block = 597003643462067142;
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
                    tmp___1 = prefix_case_iter_next(&mut caseiter);
                    if tmp___1 == 0 {
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
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
        tmp___0 = vg_exec_context_upgrade_lock(vxcp);
        if tmp___0 != 0 {
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
            tmp___1 = avl_root_empty(&mut (*vcpp).vcp_avlroot);
            if tmp___1 == 0 {
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
    tmp___2 = avl_root_empty(&mut (*vcpp).vcp_avlroot);
    if tmp___2 != 0 {
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
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
                    (20 as libc::c_int - ncopy) as size_t as _,
                );
            }
            memcpy(
                cbuf.offset((20 as libc::c_int - ncopy) as isize) as *mut libc::c_void,
                bnbuf.as_mut_ptr().offset(nskip as isize) as *const libc::c_void,
                ncopy as size_t as _,
            );
            cbuf = cbuf.offset(20 as libc::c_int as isize);
            nbytes = BN_bn2bin((*vp).vp_high as *const BIGNUM, bnbuf.as_mut_ptr());
            if nbytes >= 24 as libc::c_int {
                ncopy = 20 as libc::c_int;
            } else {
                if nbytes > 4 as libc::c_int {
                    tmp___1 = nbytes - 4 as libc::c_int;
                } else {
                    tmp___1 = 0 as libc::c_int;
                }
                ncopy = tmp___1;
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
                    (20 as libc::c_int - ncopy) as size_t as _,
                );
            }
            memcpy(
                cbuf.offset((20 as libc::c_int - ncopy) as isize) as *mut libc::c_void,
                bnbuf.as_mut_ptr().offset(nskip as isize) as *const libc::c_void,
                ncopy as size_t as _,
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
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp___0 = malloc(::std::mem::size_of::<vg_prefix_context_t>() as libc::c_ulong);
    vcpp = tmp___0 as *mut vg_prefix_context_t;
    if !vcpp.is_null() {
        memset(
            vcpp as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<vg_prefix_context_t>() as libc::c_ulong as _,
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
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
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
        tmp___0 = malloc(
            (3 as libc::c_ulong)
                .wrapping_mul(count)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
        mem = tmp___0 as *mut *mut libc::c_void;
        if mem.is_null() {
            return 0 as libc::c_int;
        }
        i = 0 as libc::c_ulong;
        while i < (*vcrp).base.vc_npatterns {
            let ref mut fresh10 = *mem.offset(i as isize);
            *fresh10 = *((*vcrp).vcr_regex).offset(i as isize) as *mut libc::c_void;
            let ref mut fresh11 = *mem.offset(count.wrapping_add(i) as isize);
            *fresh11 = *((*vcrp).vcr_regex_extra).offset(i as isize)
                as *mut libc::c_void;
            let ref mut fresh12 = *mem
                .offset(
                    (2 as libc::c_ulong).wrapping_mul(count).wrapping_add(i) as isize,
                );
            *fresh12 = *((*vcrp).vcr_regex_pat).offset(i as isize) as *mut libc::c_void;
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
        let ref mut fresh13 = *((*vcrp).vcr_regex).offset(nres as isize);
        *fresh13 = pcre_compile(
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
            let ref mut fresh14 = *((*vcrp).vcr_regex_extra).offset(nres as isize);
            *fresh14 = pcre_study(
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
                let ref mut fresh15 = *((*vcrp).vcr_regex_pat).offset(nres as isize);
                *fresh15 = *patterns.offset(i as isize);
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
    let mut tmp___0: libc::c_ulong = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
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
        4 as libc::c_int as size_t as _,
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
        tmp___0 = BN_get_word(&mut bnrem as *mut BIGNUM as *const BIGNUM);
        d = tmp___0 as libc::c_int;
        p -= 1;
        b58[p as usize] = *vg_b58_alphabet.offset(d as isize);
    }
    loop {
        tmp___1 = zpfx;
        zpfx -= 1;
        if tmp___1 == 0 {
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
                    tmp___2 = vg_exec_context_upgrade_lock(vxcp);
                    if tmp___2 != 0 {
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
                                let ref mut fresh16 = *((*vcrp).vcr_regex)
                                    .offset(i as isize);
                                *fresh16 = *((*vcrp).vcr_regex).offset(nres as isize);
                                let ref mut fresh17 = *((*vcrp).vcr_regex_extra)
                                    .offset(i as isize);
                                *fresh17 = *((*vcrp).vcr_regex_extra).offset(nres as isize);
                                let ref mut fresh18 = *((*vcrp).vcr_regex_pat)
                                    .offset(i as isize);
                                *fresh18 = *((*vcrp).vcr_regex_pat).offset(nres as isize);
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
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp___0 = malloc(::std::mem::size_of::<vg_regex_context_t>() as libc::c_ulong);
    vcrp = tmp___0 as *mut vg_regex_context_t;
    if !vcrp.is_null() {
        memset(
            vcrp as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<vg_regex_context_t>() as libc::c_ulong as _,
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
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    buf = BN_bn2hex(bn);
    if !buf.is_null() {
        tmp___0 = buf as *const libc::c_char;
    } else {
        tmp___0 = b"0\0" as *const u8 as *const libc::c_char;
    }
    fprintf(fp, b"%s\n\0" as *const u8 as *const libc::c_char, tmp___0);
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
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_ulong = 0;
    let mut tmp___2: libc::c_int = 0;
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
    tmp___0 = malloc(brlen as size_t);
    binres = tmp___0 as *mut libc::c_uchar;
    memcpy(binres as *mut libc::c_void, buf as *const libc::c_void, len as _);
    SHA256(binres as *const libc::c_uchar, len, hash1.as_mut_ptr());
    SHA256(
        hash1.as_mut_ptr() as *const libc::c_uchar,
        ::std::mem::size_of::<[libc::c_uchar; 32]>() as libc::c_ulong,
        hash2.as_mut_ptr(),
    );
    memcpy(
        binres.offset(len as isize) as *mut libc::c_void,
        hash2.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as size_t as _,
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
        tmp___1 = BN_get_word(&mut bnrem as *mut BIGNUM as *const BIGNUM);
        d = tmp___1 as libc::c_int;
        p -= 1;
        *binres
            .offset(p as isize) = *vg_b58_alphabet.offset(d as isize) as libc::c_uchar;
    }
    loop {
        tmp___2 = zpfx;
        zpfx -= 1;
        if tmp___2 == 0 {
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
        (brlen - p) as size_t as _,
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
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: libc::c_int = 0;
    xbuf = 0 as *mut libc::c_void as *mut libc::c_uchar;
    res = 0 as libc::c_int;
    BN_init(&mut bn);
    BN_init(&mut bnw);
    BN_init(&mut bnbase);
    BN_set_word(&mut bnbase, 58 as libc::c_ulong);
    bnctx = BN_CTX_new();
    tmp___0 = strlen(input);
    l = tmp___0 as libc::c_int;
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
                            current_block = 8367009238430249258;
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
            tmp___1 = BN_num_bits(&mut bn as *mut BIGNUM as *const BIGNUM);
            c = (tmp___1 + 7 as libc::c_int) / 8 as libc::c_int;
            l = zpfx + c;
            if !(l < 5 as libc::c_int) {
                tmp___2 = malloc(l as size_t);
                xbuf = tmp___2 as *mut libc::c_uchar;
                if !xbuf.is_null() {
                    if zpfx != 0 {
                        memset(
                            xbuf as *mut libc::c_void,
                            0 as libc::c_int,
                            zpfx as size_t as _,
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
                    tmp___3 = memcmp(
                        hash2.as_mut_ptr() as *const libc::c_void,
                        xbuf.offset(l as isize) as *const libc::c_void,
                        4 as libc::c_int as size_t,
                    );
                    if !(tmp___3 != 0) {
                        if len != 0 {
                            if len > l as size_t {
                                len = l as size_t;
                            }
                            memcpy(buf, xbuf as *const libc::c_void, len as _);
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
    let mut tmp___0: libc::c_uint = 0;
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    binres[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    tmp___0 = 1 as libc::c_uint;
    while !(tmp___0 >= 21 as libc::c_uint) {
        binres[tmp___0 as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp___0 = tmp___0.wrapping_add(1);
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
    let mut tmp___0: libc::c_uint = 0;
    let mut hash1: [libc::c_uchar; 32] = [0; 32];
    eckey_buf = script_buf.as_mut_ptr().offset(2 as libc::c_int as isize);
    binres[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    tmp___0 = 1 as libc::c_uint;
    while !(tmp___0 >= 21 as libc::c_uint) {
        binres[tmp___0 as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp___0 = tmp___0.wrapping_add(1);
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
    let mut tmp___0: libc::c_int = 0;
    bn = EC_KEY_get0_private_key(pkey);
    eckey_buf[0 as libc::c_int as usize] = addrtype as libc::c_uchar;
    tmp___0 = BN_num_bits(bn);
    nbytes = (tmp___0 + 7 as libc::c_int) / 8 as libc::c_int;
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
            (32 as libc::c_int - nbytes) as size_t as _,
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    pgroup = EC_KEY_get0_group(pkey as *const EC_KEY);
    ppnt = EC_POINT_new(pgroup);
    if !ppnt.is_null() {
        tmp___0 = EC_KEY_set_private_key(pkey, bnpriv);
        if tmp___0 != 0 {
            tmp___1 = EC_POINT_mul(
                pgroup,
                ppnt,
                bnpriv,
                0 as *mut libc::c_void as *const EC_POINT,
                0 as *mut libc::c_void as *const BIGNUM,
                0 as *mut libc::c_void as *mut BN_CTX,
            );
            if tmp___1 != 0 {
                tmp___2 = EC_KEY_set_public_key(pkey, ppnt as *const EC_POINT);
                if tmp___2 != 0 {
                    tmp___3 = 1 as libc::c_int;
                } else {
                    tmp___3 = 0 as libc::c_int;
                }
            } else {
                tmp___3 = 0 as libc::c_int;
            }
        } else {
            tmp___3 = 0 as libc::c_int;
        }
    } else {
        tmp___3 = 0 as libc::c_int;
    }
    res = tmp___3;
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
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___5: libc::c_int = 0;
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
                current_block = 11377157378814796093;
            } else {
                current_block = 15768484401365413375;
            }
        } else {
            current_block = 15768484401365413375;
        }
        match current_block {
            11377157378814796093 => {}
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
                                    current_block = 11377157378814796093;
                                } else {
                                    current_block = 5330834795799507926;
                                }
                            } else {
                                current_block = 5330834795799507926;
                            }
                            match current_block {
                                11377157378814796093 => {}
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
                                        tmp___0 = strlen(pass);
                                        PKCS5_PBKDF2_HMAC(
                                            pass,
                                            tmp___0.wrapping_add(1 as libc::c_ulong) as libc::c_int,
                                            salt as *const libc::c_uchar,
                                            salt_len,
                                            (*params).iterations,
                                            pbkdf_digest,
                                            (*cipher).key_len + (*cipher).iv_len + hmac_keylen,
                                            keymaterial.as_mut_ptr(),
                                        );
                                        tmp___1 = EVP_CipherInit(
                                            ctx,
                                            cipher,
                                            keymaterial.as_mut_ptr() as *const libc::c_uchar,
                                            keymaterial.as_mut_ptr().offset((*cipher).key_len as isize)
                                                as *const libc::c_uchar,
                                            enc,
                                        );
                                        if tmp___1 == 0 {
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
                                            tmp___2 = EVP_CipherUpdate(
                                                ctx,
                                                data_out.offset(opos as isize),
                                                &mut oincr,
                                                data_in.offset(ipos as isize) as *const libc::c_uchar,
                                                nbytes,
                                            );
                                            if tmp___2 == 0 {
                                                current_block = 8232204334090222504;
                                            } else {
                                                opos += oincr;
                                                olen -= oincr;
                                                oincr = olen;
                                                tmp___3 = EVP_CipherFinal(
                                                    ctx,
                                                    data_out.offset(opos as isize),
                                                    &mut oincr,
                                                );
                                                if tmp___3 == 0 {
                                                    current_block = 8232204334090222504;
                                                } else {
                                                    opos += oincr;
                                                    if hmac_len != 0 {
                                                        hlen = ::std::mem::size_of::<[libc::c_uchar; 64]>()
                                                            as libc::c_ulong as libc::c_uint;
                                                        if enc != 0 {
                                                            tmp___4 = data_in;
                                                        } else {
                                                            tmp___4 = data_out;
                                                        }
                                                        HMAC(
                                                            hmac_digest,
                                                            keymaterial
                                                                .as_mut_ptr()
                                                                .offset((*cipher).key_len as isize)
                                                                .offset((*cipher).iv_len as isize) as *const libc::c_void,
                                                            hmac_keylen,
                                                            tmp___4 as *const libc::c_uchar,
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
                                                                hmac_len as size_t as _,
                                                            );
                                                            current_block = 9199578309995299736;
                                                        } else {
                                                            tmp___5 = memcmp(
                                                                hmac.as_mut_ptr() as *const libc::c_void,
                                                                data_in
                                                                    .offset(1 as libc::c_int as isize)
                                                                    .offset(ciphertext_len as isize) as *const libc::c_void,
                                                                hmac_len as size_t,
                                                            );
                                                            if tmp___5 != 0 {
                                                                current_block = 8232204334090222504;
                                                            } else {
                                                                current_block = 9199578309995299736;
                                                            }
                                                        }
                                                    } else {
                                                        current_block = 9199578309995299736;
                                                    }
                                                    match current_block {
                                                        8232204334090222504 => {}
                                                        _ => {
                                                            if enc != 0 {
                                                                if opos != 1 as libc::c_int + ciphertext_len {
                                                                    fprintf(
                                                                        stderr,
                                                                        b"ERROR: plaintext size mismatch\n\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                    current_block = 11377157378814796093;
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
                                                                current_block = 11377157378814796093;
                                                            } else {
                                                                current_block = 17769492591016358583;
                                                            }
                                                            match current_block {
                                                                11377157378814796093 => {}
                                                                _ => {
                                                                    ret = opos;
                                                                    current_block = 11377157378814796093;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            match current_block {
                                                11377157378814796093 => {}
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: size_t = 0;
    if keytype & 1 as libc::c_int != 0 {
        restype = 79 as libc::c_int;
    } else {
        restype = 32 as libc::c_int;
    }
    privkey = EC_KEY_get0_private_key(pkey);
    tmp___0 = BN_num_bits(privkey);
    nbytes = (tmp___0 + 7 as libc::c_int) / 8 as libc::c_int;
    if nbytes < 32 as libc::c_int {
        memset(
            ecpriv.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (32 as libc::c_int - nbytes) as size_t as _,
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
    tmp___1 = strlen(out as *const libc::c_char);
    nbytes = tmp___1 as libc::c_int;
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
    let mut tmp___0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp___1: libc::c_int = 0;
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
        tmp___0 = ecpriv.as_mut_ptr();
    } else {
        tmp___0 = 0 as *mut libc::c_void as *mut libc::c_uchar;
    }
    tmp___1 = vg_protect_crypt(
        -(1 as libc::c_int),
        ecenc.as_mut_ptr().offset(1 as libc::c_int as isize),
        res - 1 as libc::c_int,
        tmp___0,
        pass,
        0 as libc::c_int,
    );
    if tmp___1 == 0 {
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut BIO_METHOD = 0 as *mut BIO_METHOD;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *const EVP_CIPHER = 0 as *const EVP_CIPHER;
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
            tmp___0 = EVP_PKEY_set1_EC_KEY(evp_key, pkey_copy);
            if !(tmp___0 == 0) {
                pkcs8 = EVP_PKEY2PKCS8(evp_key);
                if !pkcs8.is_null() {
                    tmp___1 = BIO_s_mem();
                    bio = BIO_new(tmp___1);
                    if !bio.is_null() {
                        if pass.is_null() {
                            res = PEM_write_bio_PKCS8_PRIV_KEY_INFO(bio, pkcs8);
                            current_block = 2232869372362427478;
                        } else {
                            tmp___2 = strlen(pass);
                            tmp___3 = EVP_aes_256_cbc();
                            pkcs8_enc = PKCS8_encrypt(
                                -(1 as libc::c_int),
                                tmp___3,
                                pass,
                                tmp___2 as libc::c_int,
                                0 as *mut libc::c_void as *mut libc::c_uchar,
                                0 as libc::c_int,
                                4096 as libc::c_int,
                                pkcs8,
                            );
                            if pkcs8_enc.is_null() {
                                current_block = 1013675548128967757;
                            } else {
                                res = PEM_write_bio_PKCS8(bio, pkcs8_enc);
                                current_block = 2232869372362427478;
                            }
                        }
                        match current_block {
                            1013675548128967757 => {}
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
                                        res as size_t as _,
                                    );
                                    *out.offset(res as isize) = '\u{0}' as i32 as libc::c_char;
                                } else {
                                    memcpy(
                                        out as *mut libc::c_void,
                                        (*memptr).data as *const libc::c_void,
                                        (outlen - 1 as libc::c_int) as size_t as _,
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
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___3: *const EC_GROUP = 0 as *const EC_GROUP;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: *mut EC_KEY = 0 as *mut EC_KEY;
    pkey_in = 0 as *mut libc::c_void as *mut EC_KEY;
    test_key = 0 as *mut libc::c_void as *mut EC_KEY;
    evp_key = 0 as *mut libc::c_void as *mut EVP_PKEY;
    pkcs8 = 0 as *mut libc::c_void as *mut PKCS8_PRIV_KEY_INFO;
    pkcs8_enc = 0 as *mut libc::c_void as *mut X509_SIG;
    bio = 0 as *mut libc::c_void as *mut BIO;
    res = 0 as libc::c_int;
    tmp___0 = strlen(pem_in);
    bio = BIO_new_mem_buf(
        pem_in as *mut libc::c_char as *mut libc::c_void,
        tmp___0 as libc::c_int,
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
            tmp___1 = strlen(pass);
            pkcs8 = PKCS8_decrypt(pkcs8_enc, pass, tmp___1 as libc::c_int);
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
                        tmp___2 = EC_KEY_get0_group(test_key as *const EC_KEY);
                        tmp___3 = EC_KEY_get0_group(pkey_in as *const EC_KEY);
                        tmp___4 = EC_GROUP_cmp(
                            tmp___3,
                            tmp___2,
                            0 as *mut libc::c_void as *mut BN_CTX,
                        );
                        if !(tmp___4 != 0) {
                            tmp___5 = EC_KEY_copy(pkey, pkey_in as *const EC_KEY);
                            if !tmp___5.is_null() {
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    tmp___0 = vg_decode_privkey(input, pkey, addrtype);
    if tmp___0 != 0 {
        return 1 as libc::c_int;
    }
    tmp___2 = vg_protect_decode_privkey(
        pkey,
        addrtype,
        input,
        0 as *mut libc::c_void as *const libc::c_char,
    );
    if tmp___2 != 0 {
        if pass.is_null() {
            return -(1 as libc::c_int);
        }
        tmp___1 = vg_protect_decode_privkey(pkey, addrtype, input, pass);
        return tmp___1;
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
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp___0 = EVP_read_pw_string(
        buf,
        size as libc::c_int,
        b"Enter new password:\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    if tmp___0 != 0 {
        tmp___1 = 0 as libc::c_int;
    } else {
        tmp___1 = 1 as libc::c_int;
    }
    return tmp___1;
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
    let mut tmp___0: libc::c_uint = 0;
    let mut crackunit: *const libc::c_char = 0 as *const libc::c_char;
    let mut char_complexity: libc::c_int = 0;
    let mut crackops: libc::c_double = 0.;
    let mut cracktime: libc::c_double = 0.;
    let mut weak: libc::c_int = 0;
    let mut rate: libc::c_int = 0;
    let mut weak_threshold: libc::c_int = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *const libc::c_char = 0 as *const libc::c_char;
    classes[0 as libc::c_int as usize] = 0 as libc::c_int;
    tmp___0 = 1 as libc::c_uint;
    while !(tmp___0 >= 6 as libc::c_uint) {
        classes[tmp___0 as usize] = 0 as libc::c_int;
        tmp___0 = tmp___0.wrapping_add(1);
    }
    crackunit = b"seconds\0" as *const u8 as *const libc::c_char;
    char_complexity = 0 as libc::c_int;
    rate = 250000000 as libc::c_int;
    weak_threshold = 31536000 as libc::c_int;
    tmp___1 = strlen(pass);
    len = tmp___1 as libc::c_int;
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
            current_block = 6239531672976302653;
        } else {
            current_block = 571769854332864405;
        }
    } else {
        current_block = 571769854332864405;
    }
    match current_block {
        571769854332864405 => {
            if verbose > 1 as libc::c_int {
                current_block = 6239531672976302653;
            } else {
                current_block = 11071260907632769126;
            }
        }
        _ => {}
    }
    match current_block {
        6239531672976302653 => {
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
                                    current_block = 11953140830320187989;
                                }
                            } else {
                                current_block = 11953140830320187989;
                            }
                        } else {
                            current_block = 11953140830320187989;
                        }
                    } else {
                        current_block = 11953140830320187989;
                    }
                } else {
                    current_block = 11953140830320187989;
                }
            } else {
                current_block = 11953140830320187989;
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
                                        current_block_76 = 3933019160833971941;
                                    } else if classes[1 as libc::c_int as usize] == 0 {
                                        current_block_76 = 3933019160833971941;
                                    } else {
                                        fprintf(
                                            stderr,
                                            b"WARNING: Password contains only letters\n\0" as *const u8
                                                as *const libc::c_char,
                                        );
                                        current_block_76 = 5722677567366458307;
                                    }
                                    match current_block_76 {
                                        3933019160833971941 => {
                                            if classes[0 as libc::c_int as usize] != 0 {
                                                tmp___2 = b"lower\0" as *const u8 as *const libc::c_char;
                                            } else {
                                                tmp___2 = b"upper\0" as *const u8 as *const libc::c_char;
                                            }
                                            fprintf(
                                                stderr,
                                                b"WARNING: Password contains only %scase letters\n\0"
                                                    as *const u8 as *const libc::c_char,
                                                tmp___2,
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
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = 1 as libc::c_int;
    buf = 0 as *mut libc::c_void as *mut libc::c_char;
    blksize = 16384 as libc::c_int;
    nalloc = 16 as libc::c_int;
    npatterns = 0 as libc::c_int;
    tmp___0 = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nalloc as libc::c_ulong),
    );
    patterns = tmp___0 as *mut *mut libc::c_char;
    count = 0 as libc::c_int;
    pos = 0 as libc::c_int;
    loop {
        obuf = buf;
        tmp___1 = malloc(blksize as size_t);
        buf = tmp___1 as *mut libc::c_char;
        if buf.is_null() {
            ret = 0 as libc::c_int;
            break;
        } else {
            if pos < count {
                memcpy(
                    buf as *mut libc::c_void,
                    obuf.offset(pos as isize) as *const libc::c_void,
                    (count - pos) as size_t as _,
                );
            }
            pos = count - pos;
            tmp___2 = fread(
                buf.offset(pos as isize) as *mut libc::c_void,
                1 as libc::c_int as size_t,
                (blksize - pos) as size_t,
                fp,
            );
            count = tmp___2 as libc::c_int;
            if count < 0 as libc::c_int {
                tmp___3 = __errno_location();
                tmp___4 = strerror(*tmp___3);
                fprintf(
                    stderr,
                    b"Error reading file: %s\n\0" as *const u8 as *const libc::c_char,
                    tmp___4,
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
                    current_block_40 = 6015050597719278903;
                } else if *buf.offset(pos as isize) as libc::c_int == 10 as libc::c_int {
                    current_block_40 = 6015050597719278903;
                } else {
                    if pat.is_null() {
                        pat = buf.offset(pos as isize);
                    }
                    current_block_40 = 12199444798915819164;
                }
                match current_block_40 {
                    6015050597719278903 => {
                        *buf.offset(pos as isize) = '\u{0}' as i32 as libc::c_char;
                        if !pat.is_null() {
                            if npatterns == nalloc {
                                nalloc *= 2 as libc::c_int;
                                tmp___5 = realloc(
                                    patterns as *mut libc::c_void,
                                    (::std::mem::size_of::<*mut libc::c_char>()
                                        as libc::c_ulong)
                                        .wrapping_mul(nalloc as libc::c_ulong),
                                );
                                patterns = tmp___5 as *mut *mut libc::c_char;
                            }
                            let ref mut fresh19 = *patterns.offset(npatterns as isize);
                            *fresh19 = pat;
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
