use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn ksw_extz2_sse(
        km: *mut libc::c_void,
        qlen: libc::c_int,
        query: *const uint8_t,
        tlen: libc::c_int,
        target: *const uint8_t,
        m: int8_t,
        mat: *const int8_t,
        q: int8_t,
        e: int8_t,
        w: libc::c_int,
        zdrop: libc::c_int,
        end_bonus: libc::c_int,
        flag: libc::c_int,
        ez: *mut ksw_extz_t,
    );
    fn ksw_extd2_sse(
        km: *mut libc::c_void,
        qlen: libc::c_int,
        query: *const uint8_t,
        tlen: libc::c_int,
        target: *const uint8_t,
        m: int8_t,
        mat: *const int8_t,
        gapo: int8_t,
        gape: int8_t,
        gapo2: int8_t,
        gape2: int8_t,
        w: libc::c_int,
        zdrop: libc::c_int,
        end_bonus: libc::c_int,
        flag: libc::c_int,
        ez: *mut ksw_extz_t,
    );
    fn ksw_exts2_sse(
        km: *mut libc::c_void,
        qlen: libc::c_int,
        query: *const uint8_t,
        tlen: libc::c_int,
        target: *const uint8_t,
        m: int8_t,
        mat: *const int8_t,
        gapo: int8_t,
        gape: int8_t,
        gapo2: int8_t,
        noncan: int8_t,
        zdrop: libc::c_int,
        junc_bonus: int8_t,
        flag: libc::c_int,
        junc: *const uint8_t,
        ez: *mut ksw_extz_t,
    );
    fn ksw_ll_qinit(
        km: *mut libc::c_void,
        size: libc::c_int,
        qlen: libc::c_int,
        query: *const uint8_t,
        m: libc::c_int,
        mat: *const int8_t,
    ) -> *mut libc::c_void;
    fn ksw_ll_i16(
        q: *mut libc::c_void,
        tlen: libc::c_int,
        target: *const uint8_t,
        gapo: libc::c_int,
        gape: libc::c_int,
        qe: *mut libc::c_int,
        te: *mut libc::c_int,
    ) -> libc::c_int;
    fn gzdopen(fd: libc::c_int, mode: *const libc::c_char) -> gzFile;
    fn gzread(file: gzFile, buf: voidp, len: libc::c_uint) -> libc::c_int;
    fn gzclose(file: gzFile) -> libc::c_int;
    fn gzopen(_: *const libc::c_char, _: *const libc::c_char) -> gzFile;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn abort() -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn logf(_: libc::c_float) -> libc::c_float;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
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
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn freopen(
        __filename: *const libc::c_char,
        __modes: *const libc::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int8_t = __int8_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct __anonstruct_mm128_t_549813859 {
    pub x: uint64_t,
    pub y: uint64_t,
}
pub type mm128_t = __anonstruct_mm128_t_549813859;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_mm_idx_seq_t_994244248 {
    pub name: *mut libc::c_char,
    pub offset: uint64_t,
    pub len: uint32_t,
    pub is_alt: uint32_t,
}
pub type mm_idx_seq_t = __anonstruct_mm_idx_seq_t_994244248;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_bucket_s {
    pub a: mm128_v,
    pub n: int32_t,
    pub p: *mut uint64_t,
    pub h: *mut libc::c_void,
}
pub type mm128_v = __anonstruct_mm128_v_892938973;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_mm128_v_892938973 {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut mm128_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_idx_intv_s {
    pub n: int32_t,
    pub m: int32_t,
    pub a: *mut mm_idx_intv1_t,
}
pub type mm_idx_intv1_t = __anonstruct_mm_idx_intv1_t_814805217;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct_mm_idx_intv1_t_814805217 {
    pub st: int32_t,
    pub en: int32_t,
    pub max: int32_t,
    #[bitfield(name = "score", ty = "int32_t", bits = "0..=29")]
    #[bitfield(name = "strand", ty = "int32_t", bits = "30..=31")]
    pub score_strand: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_mm_idx_t_37932985 {
    pub b: int32_t,
    pub w: int32_t,
    pub k: int32_t,
    pub flag: int32_t,
    pub n_seq: uint32_t,
    pub index: int32_t,
    pub n_alt: int32_t,
    pub seq: *mut mm_idx_seq_t,
    pub S: *mut uint32_t,
    pub B: *mut mm_idx_bucket_s,
    pub I: *mut mm_idx_intv_s,
    pub km: *mut libc::c_void,
    pub h: *mut libc::c_void,
}
pub type mm_idx_t = __anonstruct_mm_idx_t_37932985;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct_mm_extra_t_49860175 {
    pub capacity: uint32_t,
    pub dp_score: int32_t,
    pub dp_max: int32_t,
    pub dp_max2: int32_t,
    #[bitfield(name = "n_ambi", ty = "uint32_t", bits = "0..=29")]
    #[bitfield(name = "trans_strand", ty = "uint32_t", bits = "30..=31")]
    pub n_ambi_trans_strand: [u8; 4],
    pub n_cigar: uint32_t,
    pub cigar: [uint32_t; 0],
}
pub type mm_extra_t = __anonstruct_mm_extra_t_49860175;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct_mm_reg1_t_435689901 {
    pub id: int32_t,
    pub cnt: int32_t,
    pub rid: int32_t,
    pub score: int32_t,
    pub qs: int32_t,
    pub qe: int32_t,
    pub rs: int32_t,
    pub re: int32_t,
    pub parent: int32_t,
    pub subsc: int32_t,
    pub as_0: int32_t,
    pub mlen: int32_t,
    pub blen: int32_t,
    pub n_sub: int32_t,
    pub score0: int32_t,
    #[bitfield(name = "mapq", ty = "uint32_t", bits = "0..=7")]
    #[bitfield(name = "split", ty = "uint32_t", bits = "8..=9")]
    #[bitfield(name = "rev", ty = "uint32_t", bits = "10..=10")]
    #[bitfield(name = "inv", ty = "uint32_t", bits = "11..=11")]
    #[bitfield(name = "sam_pri", ty = "uint32_t", bits = "12..=12")]
    #[bitfield(name = "proper_frag", ty = "uint32_t", bits = "13..=13")]
    #[bitfield(name = "pe_thru", ty = "uint32_t", bits = "14..=14")]
    #[bitfield(name = "seg_split", ty = "uint32_t", bits = "15..=15")]
    #[bitfield(name = "seg_id", ty = "uint32_t", bits = "16..=23")]
    #[bitfield(name = "split_inv", ty = "uint32_t", bits = "24..=24")]
    #[bitfield(name = "is_alt", ty = "uint32_t", bits = "25..=25")]
    #[bitfield(name = "strand_retained", ty = "uint32_t", bits = "26..=26")]
    #[bitfield(name = "dummy", ty = "uint32_t", bits = "27..=31")]
    pub mapq_split_rev_inv_sam_pri_proper_frag_pe_thru_seg_split_seg_id_split_inv_is_alt_strand_retained_dummy: [u8; 4],
    pub hash: uint32_t,
    pub div: libc::c_float,
    pub p: *mut mm_extra_t,
}
pub type mm_reg1_t = __anonstruct_mm_reg1_t_435689901;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_mm_mapopt_t_504735564 {
    pub flag: int64_t,
    pub seed: libc::c_int,
    pub sdust_thres: libc::c_int,
    pub max_qlen: libc::c_int,
    pub bw: libc::c_int,
    pub bw_long: libc::c_int,
    pub max_gap: libc::c_int,
    pub max_gap_ref: libc::c_int,
    pub max_frag_len: libc::c_int,
    pub max_chain_skip: libc::c_int,
    pub max_chain_iter: libc::c_int,
    pub min_cnt: libc::c_int,
    pub min_chain_score: libc::c_int,
    pub chain_gap_scale: libc::c_float,
    pub chain_skip_scale: libc::c_float,
    pub rmq_size_cap: libc::c_int,
    pub rmq_inner_dist: libc::c_int,
    pub rmq_rescue_size: libc::c_int,
    pub rmq_rescue_ratio: libc::c_float,
    pub mask_level: libc::c_float,
    pub mask_len: libc::c_int,
    pub pri_ratio: libc::c_float,
    pub best_n: libc::c_int,
    pub alt_drop: libc::c_float,
    pub a: libc::c_int,
    pub b: libc::c_int,
    pub q: libc::c_int,
    pub e: libc::c_int,
    pub q2: libc::c_int,
    pub e2: libc::c_int,
    pub sc_ambi: libc::c_int,
    pub noncan: libc::c_int,
    pub junc_bonus: libc::c_int,
    pub zdrop: libc::c_int,
    pub zdrop_inv: libc::c_int,
    pub end_bonus: libc::c_int,
    pub min_dp_max: libc::c_int,
    pub min_ksw_len: libc::c_int,
    pub anchor_ext_len: libc::c_int,
    pub anchor_ext_shift: libc::c_int,
    pub max_clip_ratio: libc::c_float,
    pub rank_min_len: libc::c_int,
    pub rank_frac: libc::c_float,
    pub pe_ori: libc::c_int,
    pub pe_bonus: libc::c_int,
    pub mid_occ_frac: libc::c_float,
    pub q_occ_frac: libc::c_float,
    pub min_mid_occ: int32_t,
    pub max_mid_occ: int32_t,
    pub mid_occ: int32_t,
    pub max_occ: int32_t,
    pub max_max_occ: int32_t,
    pub occ_dist: int32_t,
    pub mini_batch_size: int64_t,
    pub max_sw_mat: int64_t,
    pub cap_kalloc: int64_t,
    pub split_prefix: *const libc::c_char,
}
pub type mm_mapopt_t = __anonstruct_mm_mapopt_t_504735564;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_z_307641061 {
    pub f: libc::c_float,
    pub i: uint32_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct_ksw_extz_t_28989980 {
    #[bitfield(name = "max", ty = "uint32_t", bits = "0..=30")]
    #[bitfield(name = "zdropped", ty = "uint32_t", bits = "31..=31")]
    pub max_zdropped: [u8; 4],
    pub max_q: libc::c_int,
    pub max_t: libc::c_int,
    pub mqe: libc::c_int,
    pub mqe_t: libc::c_int,
    pub mte: libc::c_int,
    pub mte_q: libc::c_int,
    pub score: libc::c_int,
    pub m_cigar: libc::c_int,
    pub n_cigar: libc::c_int,
    pub reach_end: libc::c_int,
    pub cigar: *mut uint32_t,
}
pub type ksw_extz_t = __anonstruct_ksw_extz_t_28989980;
pub type voidp = *mut libc::c_void;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gzFile_s {
    pub have: libc::c_uint,
    pub next: *mut libc::c_uchar,
    pub pos: off_t,
}
pub type gzFile = *mut gzFile_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_bseq_file_s {
    pub fp: gzFile,
    pub ks: *mut kseq_t,
    pub s: mm_bseq1_t,
}
pub type mm_bseq1_t = __anonstruct_mm_bseq1_t_216488822;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_mm_bseq1_t_216488822 {
    pub l_seq: libc::c_int,
    pub rid: libc::c_int,
    pub name: *mut libc::c_char,
    pub seq: *mut libc::c_char,
    pub qual: *mut libc::c_char,
    pub comment: *mut libc::c_char,
}
pub type kseq_t = __anonstruct_kseq_t_745567265;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_kseq_t_745567265 {
    pub name: kstring_t,
    pub comment: kstring_t,
    pub seq: kstring_t,
    pub qual: kstring_t,
    pub last_char: libc::c_int,
    pub f: *mut kstream_t,
}
pub type kstream_t = __kstream_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __kstream_t {
    pub begin: libc::c_int,
    pub end: libc::c_int,
    #[bitfield(name = "is_eof", ty = "libc::c_int", bits = "0..=1")]
    #[bitfield(name = "bufsize", ty = "libc::c_int", bits = "2..=31")]
    pub is_eof_bufsize: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub f: gzFile,
    pub buf: *mut libc::c_uchar,
}
pub type kstring_t = __kstring_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __kstring_t {
    pub l: size_t,
    pub m: size_t,
    pub s: *mut libc::c_char,
}
pub type mm_bseq_file_t = mm_bseq_file_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_a_687107232 {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut mm_bseq1_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_a_687107233 {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut mm_bseq1_t,
}
pub type va_list___0 = __gnuc_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_mm_seg_t_669792804 {
    pub n_u: libc::c_int,
    pub n_a: libc::c_int,
    pub u: *mut uint64_t,
    pub a: *mut mm128_t,
}
pub type mm_seg_t = __anonstruct_mm_seg_t_669792804;
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_mm_idxopt_t_765247699 {
    pub k: libc::c_short,
    pub w: libc::c_short,
    pub flag: libc::c_short,
    pub bucket_bits: libc::c_short,
    pub mini_batch_size: int64_t,
    pub batch_size: uint64_t,
}
pub type mm_idxopt_t = __anonstruct_mm_idxopt_t_765247699;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_fp_221189586 {
    pub seq: *mut mm_bseq_file_s,
    pub idx: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_mm_idx_reader_t_915061758 {
    pub is_idx: libc::c_int,
    pub n_parts: libc::c_int,
    pub idx_size: int64_t,
    pub opt: mm_idxopt_t,
    pub fp_out: *mut FILE,
    pub fp: __anonunion_fp_221189586,
}
pub type mm_idx_reader_t = __anonstruct_mm_idx_reader_t_915061758;
pub type khint32_t = libc::c_uint;
pub type khint_t = khint32_t;
pub type kh_cstr_t = *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_idx_s {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint32_t,
    pub keys: *mut uint64_t,
    pub vals: *mut uint64_t,
}
pub type kh_idx_t = kh_idx_s;
pub type idxhash_t = kh_idx_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct kh_str_s {
    pub n_buckets: khint_t,
    pub size: khint_t,
    pub n_occupied: khint_t,
    pub upper_bound: khint_t,
    pub flags: *mut khint32_t,
    pub keys: *mut kh_cstr_t,
    pub vals: *mut uint32_t,
}
pub type kh_str_t = kh_str_s;
pub type mm_idx_bucket_t = mm_idx_bucket_s;
pub type mm_idx_intv_t = mm_idx_intv_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_pipeline_t_962757189 {
    pub mini_batch_size: libc::c_int,
    pub batch_size: uint64_t,
    pub sum_len: uint64_t,
    pub fp: *mut mm_bseq_file_t,
    pub mi: *mut mm_idx_t,
}
pub type pipeline_t = __anonstruct_pipeline_t_962757189;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_step_t_367897714 {
    pub n_seq: libc::c_int,
    pub seq: *mut mm_bseq1_t,
    pub a: mm128_v,
}
pub type step_t = __anonstruct_step_t_367897714;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_rsbucket_bed_t_959080191 {
    pub b: *mut mm_idx_intv1_t,
    pub e: *mut mm_idx_intv1_t,
}
pub type rsbucket_bed_t = __anonstruct_rsbucket_bed_t_959080191;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_km_stat_t_75465862 {
    pub capacity: size_t,
    pub available: size_t,
    pub n_blocks: size_t,
    pub n_cores: size_t,
    pub largest: size_t,
}
pub type km_stat_t = __anonstruct_km_stat_t_75465862;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct header_t {
    pub size: size_t,
    pub ptr: *mut header_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_kmem_t_1048322885 {
    pub par: *mut libc::c_void,
    pub min_core_size: size_t,
    pub base: header_t,
    pub loop_head: *mut header_t,
    pub core_head: *mut header_t,
}
pub type kmem_t = __anonstruct_kmem_t_1048322885;
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
pub union __anonunion____missing_field_name_465919411 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_717608301 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_717608300 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_717608301,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_465919411,
    pub __annonCompField2: __anonunion____missing_field_name_717608300,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_z_307641061___0 {
    pub f: libc::c_float,
    pub i: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_head_172264678 {
    pub p: [*mut lc_elem_s; 2],
    pub s: *mut lc_elem_s,
    pub balance: libc::c_schar,
    pub size: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lc_elem_s {
    pub y: int32_t,
    pub i: int64_t,
    pub pri: libc::c_double,
    pub head: __anonstruct_head_172264678,
}
pub type lc_elem_t = lc_elem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct krmq_itr_lc_elem {
    pub stack: [*const lc_elem_t; 64],
    pub top: *mut *const lc_elem_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_kmp_rmq_t_90781984 {
    pub cnt: size_t,
    pub n: size_t,
    pub max: size_t,
    pub buf: *mut *mut lc_elem_t,
    pub km: *mut libc::c_void,
}
pub type kmp_rmq_t = __anonstruct_kmp_rmq_t_90781984;
pub type __rlim_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ketopt_t_498465920 {
    pub ind: libc::c_int,
    pub opt: libc::c_int,
    pub arg: *mut libc::c_char,
    pub longidx: libc::c_int,
    pub i: libc::c_int,
    pub pos: libc::c_int,
    pub n_args: libc::c_int,
}
pub type ketopt_t = __anonstruct_ketopt_t_498465920;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ko_longopt_t_483049182 {
    pub name: *mut libc::c_char,
    pub has_arg: libc::c_int,
    pub val: libc::c_int,
}
pub type ko_longopt_t = __anonstruct_ko_longopt_t_483049182;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdust_buf_s {
    pub w: *mut kdq_int_t,
    pub P: perf_intv_v,
    pub res: uint64_v,
    pub km: *mut libc::c_void,
}
pub type uint64_v = __anonstruct_uint64_v_941540291;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_uint64_v_941540291 {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut uint64_t,
}
pub type perf_intv_v = __anonstruct_perf_intv_v_95612989;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_perf_intv_v_95612989 {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut perf_intv_t,
}
pub type perf_intv_t = __anonstruct_perf_intv_t_1051263395;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_perf_intv_t_1051263395 {
    pub start: libc::c_int,
    pub finish: libc::c_int,
    pub r: libc::c_int,
    pub l: libc::c_int,
}
pub type kdq_int_t = __anonstruct_kdq_int_t_502121680;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct_kdq_int_t_502121680 {
    #[bitfield(name = "front", ty = "uint64_t", bits = "0..=57")]
    #[bitfield(name = "bits", ty = "uint64_t", bits = "58..=63")]
    pub front_bits: [u8; 8],
    pub count: uint64_t,
    pub mask: uint64_t,
    pub a: *mut libc::c_int,
    pub km: *mut libc::c_void,
}
pub type sdust_buf_t = sdust_buf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mm_tbuf_s {
    pub km: *mut libc::c_void,
    pub rep_len: libc::c_int,
    pub frag_gap: libc::c_int,
}
pub type mm_tbuf_t = mm_tbuf_s;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct __anonstruct_mm_seed_t_554224803 {
    pub n: uint32_t,
    pub q_pos: uint32_t,
    #[bitfield(name = "q_span", ty = "uint32_t", bits = "0..=30")]
    #[bitfield(name = "flt", ty = "uint32_t", bits = "31..=31")]
    #[bitfield(name = "seg_id", ty = "uint32_t", bits = "32..=62")]
    #[bitfield(name = "is_tandem", ty = "uint32_t", bits = "63..=63")]
    pub q_span_flt_seg_id_is_tandem: [u8; 8],
    pub cr: *const uint64_t,
}
pub type mm_seed_t = __anonstruct_mm_seed_t_554224803;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_pipeline_t_219146015 {
    pub n_processed: libc::c_int,
    pub n_threads: libc::c_int,
    pub n_fp: libc::c_int,
    pub mini_batch_size: int64_t,
    pub opt: *const mm_mapopt_t,
    pub fp: *mut *mut mm_bseq_file_t,
    pub mi: *const mm_idx_t,
    pub str_0: kstring_t,
    pub n_parts: libc::c_int,
    pub rid_shift: *mut uint32_t,
    pub fp_split: *mut FILE,
    pub fp_parts: *mut *mut FILE,
}
pub type pipeline_t___0 = __anonstruct_pipeline_t_219146015;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_step_t_739812150 {
    pub p: *const pipeline_t___0,
    pub n_seq: libc::c_int,
    pub n_frag: libc::c_int,
    pub seq: *mut mm_bseq1_t,
    pub n_reg: *mut libc::c_int,
    pub seg_off: *mut libc::c_int,
    pub n_seg: *mut libc::c_int,
    pub rep_len: *mut libc::c_int,
    pub frag_gap: *mut libc::c_int,
    pub reg: *mut *mut mm_reg1_t,
    pub buf: *mut *mut mm_tbuf_t,
}
pub type step_t___0 = __anonstruct_step_t_739812150;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346496 {
    pub ru_maxrss: libc::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346497 {
    pub ru_ixrss: libc::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346498 {
    pub ru_idrss: libc::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346499 {
    pub ru_isrss: libc::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346500 {
    pub ru_minflt: libc::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346501 {
    pub ru_majflt: libc::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346502 {
    pub ru_nswap: libc::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346503 {
    pub ru_inblock: libc::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346504 {
    pub ru_oublock: libc::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346505 {
    pub ru_msgsnd: libc::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346506 {
    pub ru_msgrcv: libc::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346507 {
    pub ru_nsignals: libc::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346508 {
    pub ru_nvcsw: libc::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_1036346509 {
    pub ru_nivcsw: libc::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub __annonCompField3: __anonunion____missing_field_name_1036346496,
    pub __annonCompField4: __anonunion____missing_field_name_1036346497,
    pub __annonCompField5: __anonunion____missing_field_name_1036346498,
    pub __annonCompField6: __anonunion____missing_field_name_1036346499,
    pub __annonCompField7: __anonunion____missing_field_name_1036346500,
    pub __annonCompField8: __anonunion____missing_field_name_1036346501,
    pub __annonCompField9: __anonunion____missing_field_name_1036346502,
    pub __annonCompField10: __anonunion____missing_field_name_1036346503,
    pub __annonCompField11: __anonunion____missing_field_name_1036346504,
    pub __annonCompField12: __anonunion____missing_field_name_1036346505,
    pub __annonCompField13: __anonunion____missing_field_name_1036346506,
    pub __annonCompField14: __anonunion____missing_field_name_1036346507,
    pub __annonCompField15: __anonunion____missing_field_name_1036346508,
    pub __annonCompField16: __anonunion____missing_field_name_1036346509,
}
pub type __rusage_who_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_rsbucket_128x_t_473535168 {
    pub b: *mut mm128_t,
    pub e: *mut mm128_t,
}
pub type rsbucket_128x_t = __anonstruct_rsbucket_128x_t_473535168;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_rsbucket_64_t_1036346510 {
    pub b: *mut uint64_t,
    pub e: *mut uint64_t,
}
pub type rsbucket_64_t = __anonstruct_rsbucket_64_t_1036346510;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_pair_arr_t_502143112 {
    pub s: libc::c_int,
    pub rev: libc::c_int,
    pub key: uint64_t,
    pub r: *mut mm_reg1_t,
}
pub type pair_arr_t = __anonstruct_pair_arr_t_502143112;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_rsbucket_pair_t_1056124700 {
    pub b: *mut pair_arr_t,
    pub e: *mut pair_arr_t,
}
pub type rsbucket_pair_t = __anonstruct_rsbucket_pair_t_1056124700;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_sc_941540291 {
    pub n: size_t,
    pub m: size_t,
    pub a: *mut uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_tiny_queue_t_994244249 {
    pub front: libc::c_int,
    pub count: libc::c_int,
    pub a: [libc::c_int; 32],
}
pub type tiny_queue_t = __anonstruct_tiny_queue_t_994244249;
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
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    let mut tmp: libc::c_long = 0;
    tmp = strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
    return tmp;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    let mut tmp: libc::c_double = 0.;
    tmp = strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
    return tmp;
}
#[inline]
unsafe extern "C" fn mg_log2(mut x: libc::c_float) -> libc::c_float {
    let mut z: __anonunion_z_307641061 = __anonunion_z_307641061 { f: 0. };
    let mut log_2: libc::c_float = 0.;
    z.f = x;
    log_2 = (z.i >> 23 as libc::c_int & 255 as libc::c_uint)
        .wrapping_sub(128 as libc::c_uint) as libc::c_float;
    z.i &= !((255 as libc::c_int) << 23 as libc::c_int) as libc::c_uint;
    z
        .i = (z.i as libc::c_uint)
        .wrapping_add(((127 as libc::c_int) << 23 as libc::c_int) as uint32_t)
        as uint32_t as uint32_t;
    log_2 += (-0.34484843f32 * z.f + 2.02466578f32) * z.f - 0.67487759f32;
    return log_2;
}
#[inline]
unsafe extern "C" fn ksw_push_cigar(
    mut km: *mut libc::c_void,
    mut n_cigar: *mut libc::c_int,
    mut m_cigar: *mut libc::c_int,
    mut cigar: *mut uint32_t,
    mut op: uint32_t,
    mut len: libc::c_int,
) -> *mut uint32_t {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut current_block_13: u64;
    if *n_cigar == 0 as libc::c_int {
        current_block_13 = 4630497472498250929;
    } else if op
            != *cigar.offset((*n_cigar - 1 as libc::c_int) as isize) & 15 as libc::c_uint
        {
        current_block_13 = 4630497472498250929;
    } else {
        let ref mut fresh0 = *cigar.offset((*n_cigar - 1 as libc::c_int) as isize);
        *fresh0 = (*fresh0 as libc::c_uint)
            .wrapping_add((len << 4 as libc::c_int) as uint32_t) as uint32_t as uint32_t;
        current_block_13 = 12349973810996921269;
    }
    match current_block_13 {
        4630497472498250929 => {
            if *n_cigar == *m_cigar {
                if *m_cigar != 0 {
                    *m_cigar <<= 1 as libc::c_int;
                } else {
                    *m_cigar = 4 as libc::c_int;
                }
                tmp = krealloc(
                    km,
                    cigar as *mut libc::c_void,
                    (*m_cigar << 2 as libc::c_int) as size_t,
                );
                cigar = tmp as *mut uint32_t;
            }
            tmp___0 = *n_cigar;
            *n_cigar += 1;
            *cigar
                .offset(
                    tmp___0 as isize,
                ) = (len << 4 as libc::c_int) as libc::c_uint | op;
        }
        _ => {}
    }
    return cigar;
}
#[inline]
unsafe extern "C" fn ksw_reset_extz(mut ez: *mut ksw_extz_t) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    tmp___1 = -(1 as libc::c_int);
    (*ez).mte_q = tmp___1;
    tmp___0 = tmp___1;
    (*ez).mqe_t = tmp___0;
    tmp = tmp___0;
    (*ez).max_t = tmp;
    (*ez).max_q = tmp;
    (*ez).set_max(0 as libc::c_int as uint32_t);
    tmp___3 = -(1073741824 as libc::c_int);
    (*ez).mte = tmp___3;
    tmp___2 = tmp___3;
    (*ez).mqe = tmp___2;
    (*ez).score = tmp___2;
    (*ez).n_cigar = 0 as libc::c_int;
    (*ez).set_zdropped(0 as libc::c_int as uint32_t);
    (*ez).reach_end = 0 as libc::c_int;
}
unsafe extern "C" fn ksw_gen_simple_mat(
    mut m: libc::c_int,
    mut mat: *mut int8_t,
    mut a: int8_t,
    mut b: int8_t,
    mut sc_ambi: int8_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (a as libc::c_int) < 0 as libc::c_int {
        a = -(a as libc::c_int) as int8_t;
    } else {
        a = a;
    }
    if b as libc::c_int > 0 as libc::c_int {
        b = -(b as libc::c_int) as int8_t;
    } else {
        b = b;
    }
    if sc_ambi as libc::c_int > 0 as libc::c_int {
        sc_ambi = -(sc_ambi as libc::c_int) as int8_t;
    } else {
        sc_ambi = sc_ambi;
    }
    i = 0 as libc::c_int;
    while i < m - 1 as libc::c_int {
        j = 0 as libc::c_int;
        while j < m - 1 as libc::c_int {
            if i == j {
                *mat.offset((i * m + j) as isize) = a;
            } else {
                *mat.offset((i * m + j) as isize) = b;
            }
            j += 1;
        }
        *mat.offset((i * m + m - 1 as libc::c_int) as isize) = sc_ambi;
        i += 1;
    }
    j = 0 as libc::c_int;
    while j < m {
        *mat.offset(((m - 1 as libc::c_int) * m + j) as isize) = sc_ambi;
        j += 1;
    }
}
#[inline]
unsafe extern "C" fn mm_seq_rev(mut len: uint32_t, mut seq: *mut uint8_t) {
    let mut i: uint32_t = 0;
    let mut t: uint8_t = 0;
    i = 0 as libc::c_int as uint32_t;
    while i < len >> 1 as libc::c_int {
        t = *seq.offset(i as isize);
        *seq
            .offset(
                i as isize,
            ) = *seq
            .offset(len.wrapping_sub(1 as libc::c_uint).wrapping_sub(i) as isize);
        *seq.offset(len.wrapping_sub(1 as libc::c_uint).wrapping_sub(i) as isize) = t;
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn update_max_zdrop(
    mut score: int32_t,
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut max: *mut int32_t,
    mut max_i: *mut libc::c_int,
    mut max_j: *mut libc::c_int,
    mut e: libc::c_int,
    mut max_zdrop: *mut libc::c_int,
    mut pos: *mut [libc::c_int; 2],
) {
    let mut li: libc::c_int = 0;
    let mut lj: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    if score < *max {
        li = i - *max_i;
        lj = j - *max_j;
        if li > lj {
            tmp = li - lj;
        } else {
            tmp = lj - li;
        }
        diff = tmp;
        z = *max - score - diff * e;
        if z > *max_zdrop {
            *max_zdrop = z;
            (*pos.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize] = *max_i;
            (*pos.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize] = i;
            (*pos.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize] = *max_j;
            (*pos.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize] = j;
        }
    } else {
        *max = score;
        *max_i = i;
        *max_j = j;
    };
}
unsafe extern "C" fn mm_test_zdrop(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut qseq: *const uint8_t,
    mut tseq: *const uint8_t,
    mut n_cigar: uint32_t,
    mut cigar: *mut uint32_t,
    mut mat: *const int8_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut k: uint32_t = 0;
    let mut score: int32_t = 0;
    let mut max: int32_t = 0;
    let mut max_i: int32_t = 0;
    let mut max_j: int32_t = 0;
    let mut i: int32_t = 0;
    let mut j: int32_t = 0;
    let mut max_zdrop: int32_t = 0;
    let mut pos: [[libc::c_int; 2]; 2] = [[0; 2]; 2];
    let mut q_len: libc::c_int = 0;
    let mut t_len: libc::c_int = 0;
    let mut l: uint32_t = 0;
    let mut op: uint32_t = 0;
    let mut len: uint32_t = 0;
    let mut qseq2: *mut uint8_t = 0 as *mut uint8_t;
    let mut qp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut q_off: libc::c_int = 0;
    let mut t_off: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut c: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    score = 0 as libc::c_int;
    max = -(0x7fffffff as libc::c_int) - 1 as libc::c_int;
    max_i = -(1 as libc::c_int);
    max_j = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    j = 0 as libc::c_int;
    max_zdrop = 0 as libc::c_int;
    pos[0 as libc::c_int as usize][0 as libc::c_int as usize] = -(1 as libc::c_int);
    pos[0 as libc::c_int as usize][1 as libc::c_int as usize] = -(1 as libc::c_int);
    pos[1 as libc::c_int as usize][0 as libc::c_int as usize] = -(1 as libc::c_int);
    pos[1 as libc::c_int as usize][1 as libc::c_int as usize] = -(1 as libc::c_int);
    k = 0 as libc::c_int as uint32_t;
    score = 0 as libc::c_int;
    while k < n_cigar {
        op = *cigar.offset(k as isize) & 15 as libc::c_uint;
        len = *cigar.offset(k as isize) >> 4 as libc::c_int;
        if op == 0 as libc::c_uint {
            l = 0 as libc::c_int as uint32_t;
            while l < len {
                score
                    += *mat
                        .offset(
                            (*tseq.offset((i as uint32_t).wrapping_add(l) as isize)
                                as libc::c_int * 5 as libc::c_int
                                + *qseq.offset((j as uint32_t).wrapping_add(l) as isize)
                                    as libc::c_int) as isize,
                        ) as int32_t;
                update_max_zdrop(
                    score,
                    (i as uint32_t).wrapping_add(l) as libc::c_int,
                    (j as uint32_t).wrapping_add(l) as libc::c_int,
                    &mut max,
                    &mut max_i,
                    &mut max_j,
                    (*opt).e,
                    &mut max_zdrop,
                    pos.as_mut_ptr() as *mut [libc::c_int; 2],
                );
                l = l.wrapping_add(1);
            }
            i = (i as uint32_t).wrapping_add(len) as int32_t;
            j = (j as uint32_t).wrapping_add(len) as int32_t;
        } else {
            if op == 1 as libc::c_uint {
                current_block = 8172716634822002003;
            } else if op == 2 as libc::c_uint {
                current_block = 8172716634822002003;
            } else if op == 3 as libc::c_uint {
                current_block = 8172716634822002003;
            } else {
                current_block = 17184638872671510253;
            }
            match current_block {
                17184638872671510253 => {}
                _ => {
                    score = (score as uint32_t)
                        .wrapping_sub(
                            ((*opt).q as uint32_t)
                                .wrapping_add(((*opt).e as uint32_t).wrapping_mul(len)),
                        ) as int32_t;
                    if op == 1 as libc::c_uint {
                        j = (j as uint32_t).wrapping_add(len) as int32_t;
                    } else {
                        i = (i as uint32_t).wrapping_add(len) as int32_t;
                    }
                    update_max_zdrop(
                        score,
                        i,
                        j,
                        &mut max,
                        &mut max_i,
                        &mut max_j,
                        (*opt).e,
                        &mut max_zdrop,
                        pos.as_mut_ptr() as *mut [libc::c_int; 2],
                    );
                }
            }
        }
        k = k.wrapping_add(1);
    }
    q_len = pos[1 as libc::c_int as usize][1 as libc::c_int as usize]
        - pos[1 as libc::c_int as usize][0 as libc::c_int as usize];
    t_len = pos[0 as libc::c_int as usize][1 as libc::c_int as usize]
        - pos[0 as libc::c_int as usize][0 as libc::c_int as usize];
    if (*opt).flag & 3149952 as libc::c_long == 0 {
        if max_zdrop > (*opt).zdrop_inv {
            if q_len < (*opt).max_gap {
                if t_len < (*opt).max_gap {
                    tmp = kmalloc(km, q_len as size_t);
                    qseq2 = tmp as *mut uint8_t;
                    i = 0 as libc::c_int;
                    while i < q_len {
                        c = *qseq
                            .offset(
                                (pos[1 as libc::c_int as usize][1 as libc::c_int as usize]
                                    - i - 1 as libc::c_int) as isize,
                            ) as libc::c_int;
                        if c >= 4 as libc::c_int {
                            *qseq2.offset(i as isize) = 4 as libc::c_int as uint8_t;
                        } else {
                            *qseq2
                                .offset(i as isize) = (3 as libc::c_int - c) as uint8_t;
                        }
                        i += 1;
                    }
                    qp = ksw_ll_qinit(
                        km,
                        2 as libc::c_int,
                        q_len,
                        qseq2 as *const uint8_t,
                        5 as libc::c_int,
                        mat,
                    );
                    score = ksw_ll_i16(
                        qp,
                        t_len,
                        tseq
                            .offset(
                                pos[0 as libc::c_int as usize][0 as libc::c_int as usize]
                                    as isize,
                            ),
                        (*opt).q,
                        (*opt).e,
                        &mut q_off,
                        &mut t_off,
                    );
                    kfree(km, qseq2 as *mut libc::c_void);
                    kfree(km, qp);
                    if score >= (*opt).min_chain_score * (*opt).a {
                        if score >= (*opt).min_dp_max {
                            return 2 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    if max_zdrop > (*opt).zdrop {
        tmp___0 = 1 as libc::c_int;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0;
}
unsafe extern "C" fn mm_fix_cigar(
    mut r: *mut mm_reg1_t,
    mut qseq: *const uint8_t,
    mut tseq: *const uint8_t,
    mut qshift: *mut libc::c_int,
    mut tshift: *mut libc::c_int,
) {
    let mut p: *mut mm_extra_t = 0 as *mut mm_extra_t;
    let mut toff: int32_t = 0;
    let mut qoff: int32_t = 0;
    let mut to_shrink: int32_t = 0;
    let mut k: uint32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut op: uint32_t = 0;
    let mut len: uint32_t = 0;
    let mut l: libc::c_int = 0;
    let mut prev_len: libc::c_int = 0;
    let mut l___0: uint32_t = 0;
    let mut s: [uint32_t; 3] = [0; 3];
    let mut op___0: uint32_t = 0;
    let mut l___1: int32_t = 0;
    let mut tmp___1: int32_t = 0;
    let mut tmp___2: int32_t = 0;
    let mut l___2: int32_t = 0;
    p = (*r).p;
    toff = 0 as libc::c_int;
    qoff = 0 as libc::c_int;
    to_shrink = 0 as libc::c_int;
    tmp = 0 as libc::c_int;
    *tshift = tmp;
    *qshift = tmp;
    if (*p).n_cigar <= 1 as libc::c_uint {
        return;
    }
    k = 0 as libc::c_int as uint32_t;
    while k < (*p).n_cigar {
        op = *((*p).cigar).as_mut_ptr().offset(k as isize) & 15 as libc::c_uint;
        len = *((*p).cigar).as_mut_ptr().offset(k as isize) >> 4 as libc::c_int;
        if len == 0 as libc::c_uint {
            to_shrink = 1 as libc::c_int;
        }
        if op == 0 as libc::c_uint {
            toff = (toff as uint32_t).wrapping_add(len) as int32_t;
            qoff = (qoff as uint32_t).wrapping_add(len) as int32_t;
        } else {
            let mut current_block_53: u64;
            if op == 1 as libc::c_uint {
                current_block_53 = 163727351454869740;
            } else if op == 2 as libc::c_uint {
                current_block_53 = 163727351454869740;
            } else {
                if op == 3 as libc::c_uint {
                    toff = (toff as uint32_t).wrapping_add(len) as int32_t;
                }
                current_block_53 = 10380409671385728102;
            }
            match current_block_53 {
                163727351454869740 => {
                    if k > 0 as libc::c_uint {
                        if k < ((*p).n_cigar).wrapping_sub(1 as libc::c_uint) {
                            if *((*p).cigar)
                                .as_mut_ptr()
                                .offset(k.wrapping_sub(1 as libc::c_uint) as isize)
                                & 15 as libc::c_uint == 0 as libc::c_uint
                            {
                                if *((*p).cigar)
                                    .as_mut_ptr()
                                    .offset(k.wrapping_add(1 as libc::c_uint) as isize)
                                    & 15 as libc::c_uint == 0 as libc::c_uint
                                {
                                    prev_len = (*((*p).cigar)
                                        .as_mut_ptr()
                                        .offset(k.wrapping_sub(1 as libc::c_uint) as isize)
                                        >> 4 as libc::c_int) as libc::c_int;
                                    if op == 1 as libc::c_uint {
                                        l = 0 as libc::c_int;
                                        while l < prev_len {
                                            if *qseq.offset((qoff - 1 as libc::c_int - l) as isize)
                                                as libc::c_int
                                                != *qseq
                                                    .offset(
                                                        (qoff as uint32_t)
                                                            .wrapping_add(len)
                                                            .wrapping_sub(1 as libc::c_uint)
                                                            .wrapping_sub(l as uint32_t) as isize,
                                                    ) as libc::c_int
                                            {
                                                break;
                                            }
                                            l += 1;
                                        }
                                    } else {
                                        l = 0 as libc::c_int;
                                        while l < prev_len {
                                            if *tseq.offset((toff - 1 as libc::c_int - l) as isize)
                                                as libc::c_int
                                                != *tseq
                                                    .offset(
                                                        (toff as uint32_t)
                                                            .wrapping_add(len)
                                                            .wrapping_sub(1 as libc::c_uint)
                                                            .wrapping_sub(l as uint32_t) as isize,
                                                    ) as libc::c_int
                                            {
                                                break;
                                            }
                                            l += 1;
                                        }
                                    }
                                    if l > 0 as libc::c_int {
                                        let ref mut fresh1 = *((*p).cigar)
                                            .as_mut_ptr()
                                            .offset(k.wrapping_sub(1 as libc::c_uint) as isize);
                                        *fresh1 = (*fresh1 as libc::c_uint)
                                            .wrapping_sub((l << 4 as libc::c_int) as uint32_t)
                                            as uint32_t as uint32_t;
                                        let ref mut fresh2 = *((*p).cigar)
                                            .as_mut_ptr()
                                            .offset(k.wrapping_add(1 as libc::c_uint) as isize);
                                        *fresh2 = (*fresh2 as libc::c_uint)
                                            .wrapping_add((l << 4 as libc::c_int) as uint32_t)
                                            as uint32_t as uint32_t;
                                        qoff -= l;
                                        toff -= l;
                                    }
                                    if l == prev_len {
                                        to_shrink = 1 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                    if op == 1 as libc::c_uint {
                        qoff = (qoff as uint32_t).wrapping_add(len) as int32_t;
                    } else {
                        toff = (toff as uint32_t).wrapping_add(len) as int32_t;
                    }
                }
                _ => {}
            }
        }
        k = k.wrapping_add(1);
    }
    if qoff == (*r).qe - (*r).qs {
        if !(toff == (*r).re - (*r).rs) {
            __assert_fail(
                b"qoff == r->qe - r->qs && toff == r->re - r->rs\0" as *const u8
                    as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                125 as libc::c_uint,
                b"mm_fix_cigar\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"qoff == r->qe - r->qs && toff == r->re - r->rs\0" as *const u8
                as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_uint,
            b"mm_fix_cigar\0" as *const u8 as *const libc::c_char,
        );
    }
    k = 0 as libc::c_int as uint32_t;
    while k < ((*p).n_cigar).wrapping_sub(2 as libc::c_uint) {
        if *((*p).cigar).as_mut_ptr().offset(k as isize) & 15 as libc::c_uint
            > 0 as libc::c_uint
        {
            if (*((*p).cigar).as_mut_ptr().offset(k as isize) & 15 as libc::c_uint)
                .wrapping_add(
                    *((*p).cigar)
                        .as_mut_ptr()
                        .offset(k.wrapping_add(1 as libc::c_uint) as isize)
                        & 15 as libc::c_uint,
                ) == 3 as libc::c_uint
            {
                s[0 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
                s[1 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
                s[2 as libc::c_int as usize] = 0 as libc::c_int as uint32_t;
                l___0 = k;
                while l___0 < (*p).n_cigar {
                    op___0 = *((*p).cigar).as_mut_ptr().offset(l___0 as isize)
                        & 15 as libc::c_uint;
                    if op___0 == 1 as libc::c_uint {
                        s[op___0
                            as usize] = (s[op___0 as usize] as libc::c_uint)
                            .wrapping_add(
                                *((*p).cigar).as_mut_ptr().offset(l___0 as isize)
                                    >> 4 as libc::c_int,
                            ) as uint32_t as uint32_t;
                    } else if op___0 == 2 as libc::c_uint {
                        s[op___0
                            as usize] = (s[op___0 as usize] as libc::c_uint)
                            .wrapping_add(
                                *((*p).cigar).as_mut_ptr().offset(l___0 as isize)
                                    >> 4 as libc::c_int,
                            ) as uint32_t as uint32_t;
                    } else {
                        if !(*((*p).cigar).as_mut_ptr().offset(l___0 as isize)
                            >> 4 as libc::c_int == 0 as libc::c_uint)
                        {
                            break;
                        }
                        s[op___0
                            as usize] = (s[op___0 as usize] as libc::c_uint)
                            .wrapping_add(
                                *((*p).cigar).as_mut_ptr().offset(l___0 as isize)
                                    >> 4 as libc::c_int,
                            ) as uint32_t as uint32_t;
                    }
                    l___0 = l___0.wrapping_add(1);
                }
                if s[1 as libc::c_int as usize] > 0 as libc::c_uint {
                    if s[2 as libc::c_int as usize] > 0 as libc::c_uint {
                        if l___0.wrapping_sub(k) > 2 as libc::c_uint {
                            *((*p).cigar)
                                .as_mut_ptr()
                                .offset(
                                    k as isize,
                                ) = s[1 as libc::c_int as usize] << 4 as libc::c_int
                                | 1 as libc::c_uint;
                            *((*p).cigar)
                                .as_mut_ptr()
                                .offset(
                                    k.wrapping_add(1 as libc::c_uint) as isize,
                                ) = s[2 as libc::c_int as usize] << 4 as libc::c_int
                                | 2 as libc::c_uint;
                            k = (k as libc::c_uint).wrapping_add(2 as libc::c_uint)
                                as uint32_t as uint32_t;
                            while k < l___0 {
                                let ref mut fresh3 = *((*p).cigar)
                                    .as_mut_ptr()
                                    .offset(k as isize);
                                *fresh3 &= 15 as libc::c_uint;
                                k = k.wrapping_add(1);
                            }
                            to_shrink = 1 as libc::c_int;
                        }
                    }
                }
                k = l___0;
            }
        }
        k = k.wrapping_add(1);
    }
    if to_shrink != 0 {
        l___1 = 0 as libc::c_int;
        k = 0 as libc::c_int as uint32_t;
        while k < (*p).n_cigar {
            if *((*p).cigar).as_mut_ptr().offset(k as isize) >> 4 as libc::c_int
                != 0 as libc::c_uint
            {
                tmp___1 = l___1;
                l___1 += 1;
                *((*p).cigar)
                    .as_mut_ptr()
                    .offset(
                        tmp___1 as isize,
                    ) = *((*p).cigar).as_mut_ptr().offset(k as isize);
            }
            k = k.wrapping_add(1);
        }
        (*p).n_cigar = l___1 as uint32_t;
        l___1 = 0 as libc::c_int;
        k = l___1 as uint32_t;
        while k < (*p).n_cigar {
            if k == ((*p).n_cigar).wrapping_sub(1 as libc::c_uint) {
                tmp___2 = l___1;
                l___1 += 1;
                *((*p).cigar)
                    .as_mut_ptr()
                    .offset(
                        tmp___2 as isize,
                    ) = *((*p).cigar).as_mut_ptr().offset(k as isize);
            } else if *((*p).cigar).as_mut_ptr().offset(k as isize) & 15 as libc::c_uint
                    != *((*p).cigar)
                        .as_mut_ptr()
                        .offset(k.wrapping_add(1 as libc::c_uint) as isize)
                        & 15 as libc::c_uint
                {
                tmp___2 = l___1;
                l___1 += 1;
                *((*p).cigar)
                    .as_mut_ptr()
                    .offset(
                        tmp___2 as isize,
                    ) = *((*p).cigar).as_mut_ptr().offset(k as isize);
            } else {
                let ref mut fresh4 = *((*p).cigar)
                    .as_mut_ptr()
                    .offset(k.wrapping_add(1 as libc::c_uint) as isize);
                *fresh4 = (*fresh4 as libc::c_uint)
                    .wrapping_add(
                        (*((*p).cigar).as_mut_ptr().offset(k as isize)
                            >> 4 as libc::c_int) << 4 as libc::c_int,
                    ) as uint32_t as uint32_t;
            }
            k = k.wrapping_add(1);
        }
        (*p).n_cigar = l___1 as uint32_t;
    }
    let mut current_block_146: u64;
    if *((*p).cigar).as_mut_ptr().offset(0 as libc::c_int as isize) & 15 as libc::c_uint
        == 1 as libc::c_uint
    {
        current_block_146 = 10962761608837679223;
    } else if *((*p).cigar).as_mut_ptr().offset(0 as libc::c_int as isize)
            & 15 as libc::c_uint == 2 as libc::c_uint
        {
        current_block_146 = 10962761608837679223;
    } else {
        current_block_146 = 9437385368411212698;
    }
    match current_block_146 {
        10962761608837679223 => {
            l___2 = (*((*p).cigar).as_mut_ptr().offset(0 as libc::c_int as isize)
                >> 4 as libc::c_int) as int32_t;
            if *((*p).cigar).as_mut_ptr().offset(0 as libc::c_int as isize)
                & 15 as libc::c_uint == 1 as libc::c_uint
            {
                if (*r).rev() != 0 {
                    (*r).qe -= l___2;
                } else {
                    (*r).qs += l___2;
                }
                *qshift = l___2;
            } else {
                (*r).rs += l___2;
                *tshift = l___2;
            }
            (*p).n_cigar = ((*p).n_cigar).wrapping_sub(1);
            memmove(
                ((*p).cigar).as_mut_ptr() as *mut libc::c_void,
                ((*p).cigar).as_mut_ptr().offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                ((*p).n_cigar).wrapping_mul(4 as libc::c_uint) as size_t,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn mm_update_cigar_eqx(
    mut r: *mut mm_reg1_t,
    mut qseq: *const uint8_t,
    mut tseq: *const uint8_t,
) {
    let mut n_EQX: uint32_t = 0;
    let mut k: uint32_t = 0;
    let mut l: uint32_t = 0;
    let mut m: uint32_t = 0;
    let mut cap: uint32_t = 0;
    let mut toff: uint32_t = 0;
    let mut qoff: uint32_t = 0;
    let mut n_M: uint32_t = 0;
    let mut p: *mut mm_extra_t = 0 as *mut mm_extra_t;
    let mut op: uint32_t = 0;
    let mut len: uint32_t = 0;
    let mut op___0: uint32_t = 0;
    let mut len___0: uint32_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut op___1: uint32_t = 0;
    let mut len___1: uint32_t = 0;
    let mut tmp___0: uint32_t = 0;
    let mut tmp___1: uint32_t = 0;
    let mut tmp___2: uint32_t = 0;
    n_EQX = 0 as libc::c_int as uint32_t;
    toff = 0 as libc::c_int as uint32_t;
    qoff = 0 as libc::c_int as uint32_t;
    n_M = 0 as libc::c_int as uint32_t;
    if (*r).p as libc::c_ulong == 0 as *mut mm_extra_t as libc::c_ulong {
        return;
    }
    k = 0 as libc::c_int as uint32_t;
    while k < (*(*r).p).n_cigar {
        op = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize) & 15 as libc::c_uint;
        len = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize) >> 4 as libc::c_int;
        if op == 0 as libc::c_uint {
            while len > 0 as libc::c_uint {
                l = 0 as libc::c_int as uint32_t;
                while l < len {
                    if !(*qseq.offset(qoff.wrapping_add(l) as isize) as libc::c_int
                        == *tseq.offset(toff.wrapping_add(l) as isize) as libc::c_int)
                    {
                        break;
                    }
                    l = l.wrapping_add(1);
                }
                if l > 0 as libc::c_uint {
                    n_EQX = n_EQX.wrapping_add(1);
                    len = (len as libc::c_uint).wrapping_sub(l) as uint32_t as uint32_t;
                    toff = (toff as libc::c_uint).wrapping_add(l) as uint32_t
                        as uint32_t;
                    qoff = (qoff as libc::c_uint).wrapping_add(l) as uint32_t
                        as uint32_t;
                }
                l = 0 as libc::c_int as uint32_t;
                while l < len {
                    if !(*qseq.offset(qoff.wrapping_add(l) as isize) as libc::c_int
                        != *tseq.offset(toff.wrapping_add(l) as isize) as libc::c_int)
                    {
                        break;
                    }
                    l = l.wrapping_add(1);
                }
                if l > 0 as libc::c_uint {
                    n_EQX = n_EQX.wrapping_add(1);
                    len = (len as libc::c_uint).wrapping_sub(l) as uint32_t as uint32_t;
                    toff = (toff as libc::c_uint).wrapping_add(l) as uint32_t
                        as uint32_t;
                    qoff = (qoff as libc::c_uint).wrapping_add(l) as uint32_t
                        as uint32_t;
                }
            }
            n_M = n_M.wrapping_add(1);
        } else if op == 1 as libc::c_uint {
            qoff = (qoff as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
        } else if op == 2 as libc::c_uint {
            toff = (toff as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
        } else if op == 3 as libc::c_uint {
            toff = (toff as libc::c_uint).wrapping_add(len) as uint32_t as uint32_t;
        }
        k = k.wrapping_add(1);
    }
    if n_EQX == n_M {
        k = 0 as libc::c_int as uint32_t;
        while k < (*(*r).p).n_cigar {
            op___0 = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                & 15 as libc::c_uint;
            len___0 = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                >> 4 as libc::c_int;
            if op___0 == 0 as libc::c_uint {
                *((*(*r).p).cigar)
                    .as_mut_ptr()
                    .offset(
                        k as isize,
                    ) = len___0 << 4 as libc::c_int | 7 as libc::c_uint;
            }
            k = k.wrapping_add(1);
        }
        return;
    }
    cap = (((*(*r).p).n_cigar).wrapping_add(n_EQX.wrapping_sub(n_M)) as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<mm_extra_t>() as libc::c_ulong) as uint32_t;
    cap = cap.wrapping_sub(1);
    cap |= cap >> 1 as libc::c_int;
    cap |= cap >> 2 as libc::c_int;
    cap |= cap >> 4 as libc::c_int;
    cap |= cap >> 8 as libc::c_int;
    cap |= cap >> 16 as libc::c_int;
    cap = cap.wrapping_add(1);
    tmp = calloc(cap as size_t, 4 as libc::c_int as size_t);
    p = tmp as *mut mm_extra_t;
    memcpy(
        p as *mut libc::c_void,
        (*r).p as *const libc::c_void,
        ::std::mem::size_of::<mm_extra_t>() as libc::c_ulong,
    );
    (*p).capacity = cap;
    m = 0 as libc::c_int as uint32_t;
    qoff = m;
    toff = qoff;
    k = 0 as libc::c_int as uint32_t;
    while k < (*(*r).p).n_cigar {
        op___1 = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize) & 15 as libc::c_uint;
        len___1 = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize) >> 4 as libc::c_int;
        if op___1 == 0 as libc::c_uint {
            while len___1 > 0 as libc::c_uint {
                l = 0 as libc::c_int as uint32_t;
                while l < len___1 {
                    if !(*qseq.offset(qoff.wrapping_add(l) as isize) as libc::c_int
                        == *tseq.offset(toff.wrapping_add(l) as isize) as libc::c_int)
                    {
                        break;
                    }
                    l = l.wrapping_add(1);
                }
                if l > 0 as libc::c_uint {
                    tmp___0 = m;
                    m = m.wrapping_add(1);
                    *((*p).cigar)
                        .as_mut_ptr()
                        .offset(
                            tmp___0 as isize,
                        ) = l << 4 as libc::c_int | 7 as libc::c_uint;
                }
                len___1 = (len___1 as libc::c_uint).wrapping_sub(l) as uint32_t
                    as uint32_t;
                toff = (toff as libc::c_uint).wrapping_add(l) as uint32_t as uint32_t;
                qoff = (qoff as libc::c_uint).wrapping_add(l) as uint32_t as uint32_t;
                l = 0 as libc::c_int as uint32_t;
                while l < len___1 {
                    if !(*qseq.offset(qoff.wrapping_add(l) as isize) as libc::c_int
                        != *tseq.offset(toff.wrapping_add(l) as isize) as libc::c_int)
                    {
                        break;
                    }
                    l = l.wrapping_add(1);
                }
                if l > 0 as libc::c_uint {
                    tmp___1 = m;
                    m = m.wrapping_add(1);
                    *((*p).cigar)
                        .as_mut_ptr()
                        .offset(
                            tmp___1 as isize,
                        ) = l << 4 as libc::c_int | 8 as libc::c_uint;
                }
                len___1 = (len___1 as libc::c_uint).wrapping_sub(l) as uint32_t
                    as uint32_t;
                toff = (toff as libc::c_uint).wrapping_add(l) as uint32_t as uint32_t;
                qoff = (qoff as libc::c_uint).wrapping_add(l) as uint32_t as uint32_t;
            }
        } else {
            if op___1 == 1 as libc::c_uint {
                qoff = (qoff as libc::c_uint).wrapping_add(len___1) as uint32_t
                    as uint32_t;
            } else if op___1 == 2 as libc::c_uint {
                toff = (toff as libc::c_uint).wrapping_add(len___1) as uint32_t
                    as uint32_t;
            } else if op___1 == 3 as libc::c_uint {
                toff = (toff as libc::c_uint).wrapping_add(len___1) as uint32_t
                    as uint32_t;
            }
            tmp___2 = m;
            m = m.wrapping_add(1);
            *((*p).cigar)
                .as_mut_ptr()
                .offset(
                    tmp___2 as isize,
                ) = *((*(*r).p).cigar).as_mut_ptr().offset(k as isize);
        }
        k = k.wrapping_add(1);
    }
    (*p).n_cigar = m;
    free((*r).p as *mut libc::c_void);
    (*r).p = p;
}
unsafe extern "C" fn mm_update_extra(
    mut r: *mut mm_reg1_t,
    mut qseq: *const uint8_t,
    mut tseq: *const uint8_t,
    mut mat: *const int8_t,
    mut q: int8_t,
    mut e: int8_t,
    mut is_eqx: libc::c_int,
    mut log_gap: libc::c_int,
) {
    let mut k: uint32_t = 0;
    let mut l: uint32_t = 0;
    let mut qshift: int32_t = 0;
    let mut tshift: int32_t = 0;
    let mut toff: int32_t = 0;
    let mut qoff: int32_t = 0;
    let mut s: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    let mut p: *mut mm_extra_t = 0 as *mut mm_extra_t;
    let mut tmp: int32_t = 0;
    let mut op: uint32_t = 0;
    let mut len: uint32_t = 0;
    let mut n_ambi: libc::c_int = 0;
    let mut n_diff: libc::c_int = 0;
    let mut cq: libc::c_int = 0;
    let mut ct: libc::c_int = 0;
    let mut n_ambi___0: libc::c_int = 0;
    let mut tmp___0: libc::c_float = 0.;
    let mut n_ambi___1: libc::c_int = 0;
    let mut tmp___1: libc::c_float = 0.;
    toff = 0 as libc::c_int;
    qoff = 0 as libc::c_int;
    s = 0.0f64;
    max = 0.0f64;
    p = (*r).p;
    if p as libc::c_ulong == 0 as *mut mm_extra_t as libc::c_ulong {
        return;
    }
    mm_fix_cigar(r, qseq, tseq, &mut qshift, &mut tshift);
    qseq = qseq.offset(qshift as isize);
    tseq = tseq.offset(tshift as isize);
    tmp = 0 as libc::c_int;
    (*r).mlen = tmp;
    (*r).blen = tmp;
    k = 0 as libc::c_int as uint32_t;
    while k < (*p).n_cigar {
        op = *((*p).cigar).as_mut_ptr().offset(k as isize) & 15 as libc::c_uint;
        len = *((*p).cigar).as_mut_ptr().offset(k as isize) >> 4 as libc::c_int;
        if op == 0 as libc::c_uint {
            n_ambi = 0 as libc::c_int;
            n_diff = 0 as libc::c_int;
            l = 0 as libc::c_int as uint32_t;
            while l < len {
                cq = *qseq.offset((qoff as uint32_t).wrapping_add(l) as isize)
                    as libc::c_int;
                ct = *tseq.offset((toff as uint32_t).wrapping_add(l) as isize)
                    as libc::c_int;
                if ct > 3 as libc::c_int {
                    n_ambi += 1;
                } else if cq > 3 as libc::c_int {
                    n_ambi += 1;
                } else if ct != cq {
                    n_diff += 1;
                }
                s
                    += *mat.offset((ct * 5 as libc::c_int + cq) as isize)
                        as libc::c_double;
                if s < 0 as libc::c_int as libc::c_double {
                    s = 0 as libc::c_int as libc::c_double;
                } else if max > s {
                    max = max;
                } else {
                    max = s;
                }
                l = l.wrapping_add(1);
            }
            (*r)
                .blen = ((*r).blen as uint32_t)
                .wrapping_add(len.wrapping_sub(n_ambi as uint32_t)) as int32_t;
            (*r)
                .mlen = ((*r).mlen as uint32_t)
                .wrapping_add(len.wrapping_sub((n_ambi + n_diff) as uint32_t))
                as int32_t;
            (*p).set_n_ambi((*p).n_ambi() + n_ambi as uint32_t as uint32_t);
            toff = (toff as uint32_t).wrapping_add(len) as int32_t;
            qoff = (qoff as uint32_t).wrapping_add(len) as int32_t;
        } else if op == 1 as libc::c_uint {
            n_ambi___0 = 0 as libc::c_int;
            l = 0 as libc::c_int as uint32_t;
            while l < len {
                if *qseq.offset((qoff as uint32_t).wrapping_add(l) as isize)
                    as libc::c_int > 3 as libc::c_int
                {
                    n_ambi___0 += 1;
                }
                l = l.wrapping_add(1);
            }
            (*r)
                .blen = ((*r).blen as uint32_t)
                .wrapping_add(len.wrapping_sub(n_ambi___0 as uint32_t)) as int32_t;
            (*p).set_n_ambi((*p).n_ambi() + n_ambi___0 as uint32_t as uint32_t);
            if log_gap != 0 {
                tmp___0 = mg_log2((1.0f64 + len as libc::c_double) as libc::c_float);
                s
                    -= q as libc::c_double
                        + e as libc::c_double * tmp___0 as libc::c_double;
            } else {
                s -= (q as libc::c_int + e as libc::c_int) as libc::c_double;
            }
            if s < 0 as libc::c_int as libc::c_double {
                s = 0 as libc::c_int as libc::c_double;
            }
            qoff = (qoff as uint32_t).wrapping_add(len) as int32_t;
        } else if op == 2 as libc::c_uint {
            n_ambi___1 = 0 as libc::c_int;
            l = 0 as libc::c_int as uint32_t;
            while l < len {
                if *tseq.offset((toff as uint32_t).wrapping_add(l) as isize)
                    as libc::c_int > 3 as libc::c_int
                {
                    n_ambi___1 += 1;
                }
                l = l.wrapping_add(1);
            }
            (*r)
                .blen = ((*r).blen as uint32_t)
                .wrapping_add(len.wrapping_sub(n_ambi___1 as uint32_t)) as int32_t;
            (*p).set_n_ambi((*p).n_ambi() + n_ambi___1 as uint32_t as uint32_t);
            if log_gap != 0 {
                tmp___1 = mg_log2((1.0f64 + len as libc::c_double) as libc::c_float);
                s
                    -= q as libc::c_double
                        + e as libc::c_double * tmp___1 as libc::c_double;
            } else {
                s -= (q as libc::c_int + e as libc::c_int) as libc::c_double;
            }
            if s < 0 as libc::c_int as libc::c_double {
                s = 0 as libc::c_int as libc::c_double;
            }
            toff = (toff as uint32_t).wrapping_add(len) as int32_t;
        } else if op == 3 as libc::c_uint {
            toff = (toff as uint32_t).wrapping_add(len) as int32_t;
        }
        k = k.wrapping_add(1);
    }
    (*p).dp_max = (max + 0.499f64) as int32_t;
    if qoff == (*r).qe - (*r).qs {
        if !(toff == (*r).re - (*r).rs) {
            __assert_fail(
                b"qoff == r->qe - r->qs && toff == r->re - r->rs\0" as *const u8
                    as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                287 as libc::c_uint,
                b"mm_update_extra\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"qoff == r->qe - r->qs && toff == r->re - r->rs\0" as *const u8
                as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            287 as libc::c_uint,
            b"mm_update_extra\0" as *const u8 as *const libc::c_char,
        );
    }
    if is_eqx != 0 {
        mm_update_cigar_eqx(r, qseq, tseq);
    }
}
unsafe extern "C" fn mm_append_cigar(
    mut r: *mut mm_reg1_t,
    mut n_cigar: uint32_t,
    mut cigar: *mut uint32_t,
) {
    let mut p: *mut mm_extra_t = 0 as *mut mm_extra_t;
    let mut capacity: uint32_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if n_cigar == 0 as libc::c_uint {
        return;
    }
    if (*r).p as libc::c_ulong == 0 as *mut mm_extra_t as libc::c_ulong {
        capacity = (n_cigar as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<mm_extra_t>() as libc::c_ulong)
                    .wrapping_div(4 as libc::c_ulong),
            ) as uint32_t;
        capacity = capacity.wrapping_sub(1);
        capacity |= capacity >> 1 as libc::c_int;
        capacity |= capacity >> 2 as libc::c_int;
        capacity |= capacity >> 4 as libc::c_int;
        capacity |= capacity >> 8 as libc::c_int;
        capacity |= capacity >> 16 as libc::c_int;
        capacity = capacity.wrapping_add(1);
        tmp = calloc(capacity as size_t, 4 as libc::c_int as size_t);
        (*r).p = tmp as *mut mm_extra_t;
        (*(*r).p).capacity = capacity;
    } else if (((*(*r).p).n_cigar).wrapping_add(n_cigar) as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<mm_extra_t>() as libc::c_ulong)
                    .wrapping_div(4 as libc::c_ulong),
            ) > (*(*r).p).capacity as libc::c_ulong
        {
        (*(*r).p)
            .capacity = (((*(*r).p).n_cigar).wrapping_add(n_cigar) as libc::c_ulong)
            .wrapping_add(
                (::std::mem::size_of::<mm_extra_t>() as libc::c_ulong)
                    .wrapping_div(4 as libc::c_ulong),
            ) as uint32_t;
        (*(*r).p).capacity = ((*(*r).p).capacity).wrapping_sub(1);
        (*(*r).p).capacity |= (*(*r).p).capacity >> 1 as libc::c_int;
        (*(*r).p).capacity |= (*(*r).p).capacity >> 2 as libc::c_int;
        (*(*r).p).capacity |= (*(*r).p).capacity >> 4 as libc::c_int;
        (*(*r).p).capacity |= (*(*r).p).capacity >> 8 as libc::c_int;
        (*(*r).p).capacity |= (*(*r).p).capacity >> 16 as libc::c_int;
        (*(*r).p).capacity = ((*(*r).p).capacity).wrapping_add(1);
        tmp___0 = realloc(
            (*r).p as *mut libc::c_void,
            ((*(*r).p).capacity).wrapping_mul(4 as libc::c_uint) as size_t,
        );
        (*r).p = tmp___0 as *mut mm_extra_t;
    }
    p = (*r).p;
    if (*p).n_cigar > 0 as libc::c_uint {
        if *((*p).cigar)
            .as_mut_ptr()
            .offset(((*p).n_cigar).wrapping_sub(1 as libc::c_uint) as isize)
            & 15 as libc::c_uint
            == *cigar.offset(0 as libc::c_int as isize) & 15 as libc::c_uint
        {
            let ref mut fresh5 = *((*p).cigar)
                .as_mut_ptr()
                .offset(((*p).n_cigar).wrapping_sub(1 as libc::c_uint) as isize);
            *fresh5 = (*fresh5 as libc::c_uint)
                .wrapping_add(
                    (*cigar.offset(0 as libc::c_int as isize) >> 4 as libc::c_int)
                        << 4 as libc::c_int,
                ) as uint32_t as uint32_t;
            if n_cigar > 1 as libc::c_uint {
                memcpy(
                    ((*p).cigar).as_mut_ptr().offset((*p).n_cigar as isize)
                        as *mut libc::c_void,
                    cigar.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    n_cigar
                        .wrapping_sub(1 as libc::c_uint)
                        .wrapping_mul(4 as libc::c_uint) as size_t,
                );
            }
            (*p)
                .n_cigar = ((*p).n_cigar as libc::c_uint)
                .wrapping_add(n_cigar.wrapping_sub(1 as libc::c_uint)) as uint32_t
                as uint32_t;
        } else {
            memcpy(
                ((*p).cigar).as_mut_ptr().offset((*p).n_cigar as isize)
                    as *mut libc::c_void,
                cigar as *const libc::c_void,
                n_cigar.wrapping_mul(4 as libc::c_uint) as size_t,
            );
            (*p)
                .n_cigar = ((*p).n_cigar as libc::c_uint).wrapping_add(n_cigar)
                as uint32_t as uint32_t;
        }
    } else {
        memcpy(
            ((*p).cigar).as_mut_ptr().offset((*p).n_cigar as isize) as *mut libc::c_void,
            cigar as *const libc::c_void,
            n_cigar.wrapping_mul(4 as libc::c_uint) as size_t,
        );
        (*p)
            .n_cigar = ((*p).n_cigar as libc::c_uint).wrapping_add(n_cigar) as uint32_t
            as uint32_t;
    };
}
unsafe extern "C" fn mm_align_pair(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut qlen: libc::c_int,
    mut qseq: *const uint8_t,
    mut tlen: libc::c_int,
    mut tseq: *const uint8_t,
    mut junc: *const uint8_t,
    mut mat: *const int8_t,
    mut w: libc::c_int,
    mut end_bonus: libc::c_int,
    mut zdrop: libc::c_int,
    mut flag: libc::c_int,
    mut ez: *mut ksw_extz_t,
) {
    let mut i: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    if mm_dbg_flag & 8 as libc::c_int != 0 {
        fprintf(
            stderr,
            b"===> q=(%d,%d), e=(%d,%d), bw=%d, flag=%d, zdrop=%d <===\n\0" as *const u8
                as *const libc::c_char,
            (*opt).q,
            (*opt).q2,
            (*opt).e,
            (*opt).e2,
            w,
            flag,
            (*opt).zdrop,
        );
        i = 0 as libc::c_int;
        while i < tlen {
            fputc(
                *(b"ACGTN\0" as *const u8 as *const libc::c_char)
                    .offset(*tseq.offset(i as isize) as libc::c_int as isize)
                    as libc::c_int,
                stderr,
            );
            i += 1;
        }
        fputc('\n' as i32, stderr);
        i = 0 as libc::c_int;
        while i < qlen {
            fputc(
                *(b"ACGTN\0" as *const u8 as *const libc::c_char)
                    .offset(*qseq.offset(i as isize) as libc::c_int as isize)
                    as libc::c_int,
                stderr,
            );
            i += 1;
        }
        fputc('\n' as i32, stderr);
    }
    let mut current_block_29: u64;
    if (*opt).max_sw_mat > 0 as libc::c_long {
        if tlen as int64_t * qlen as int64_t > (*opt).max_sw_mat {
            ksw_reset_extz(ez);
            (*ez).set_zdropped(1 as libc::c_int as uint32_t);
            current_block_29 = 14763689060501151050;
        } else {
            current_block_29 = 4709409957122520787;
        }
    } else {
        current_block_29 = 4709409957122520787;
    }
    match current_block_29 {
        4709409957122520787 => {
            if (*opt).flag & 128 as libc::c_long != 0 {
                ksw_exts2_sse(
                    km,
                    qlen,
                    qseq,
                    tlen,
                    tseq,
                    5 as libc::c_int as int8_t,
                    mat,
                    (*opt).q as int8_t,
                    (*opt).e as int8_t,
                    (*opt).q2 as int8_t,
                    (*opt).noncan as int8_t,
                    zdrop,
                    (*opt).junc_bonus as int8_t,
                    flag,
                    junc,
                    ez,
                );
            } else if (*opt).q == (*opt).q2 {
                if (*opt).e == (*opt).e2 {
                    ksw_extz2_sse(
                        km,
                        qlen,
                        qseq,
                        tlen,
                        tseq,
                        5 as libc::c_int as int8_t,
                        mat,
                        (*opt).q as int8_t,
                        (*opt).e as int8_t,
                        w,
                        zdrop,
                        end_bonus,
                        flag,
                        ez,
                    );
                } else {
                    ksw_extd2_sse(
                        km,
                        qlen,
                        qseq,
                        tlen,
                        tseq,
                        5 as libc::c_int as int8_t,
                        mat,
                        (*opt).q as int8_t,
                        (*opt).e as int8_t,
                        (*opt).q2 as int8_t,
                        (*opt).e2 as int8_t,
                        w,
                        zdrop,
                        end_bonus,
                        flag,
                        ez,
                    );
                }
            } else {
                ksw_extd2_sse(
                    km,
                    qlen,
                    qseq,
                    tlen,
                    tseq,
                    5 as libc::c_int as int8_t,
                    mat,
                    (*opt).q as int8_t,
                    (*opt).e as int8_t,
                    (*opt).q2 as int8_t,
                    (*opt).e2 as int8_t,
                    w,
                    zdrop,
                    end_bonus,
                    flag,
                    ez,
                );
            }
        }
        _ => {}
    }
    if mm_dbg_flag & 8 as libc::c_int != 0 {
        fprintf(
            stderr,
            b"score=%d, cigar=\0" as *const u8 as *const libc::c_char,
            (*ez).score,
        );
        i___0 = 0 as libc::c_int;
        while i___0 < (*ez).n_cigar {
            fprintf(
                stderr,
                b"%d%c\0" as *const u8 as *const libc::c_char,
                *((*ez).cigar).offset(i___0 as isize) >> 4 as libc::c_int,
                *(b"MIDNSHP=XB\0" as *const u8 as *const libc::c_char)
                    .offset(
                        (*((*ez).cigar).offset(i___0 as isize) & 15 as libc::c_uint)
                            as isize,
                    ) as libc::c_int,
            );
            i___0 += 1;
        }
        fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    }
}
#[inline]
unsafe extern "C" fn mm_get_hplen_back(
    mut mi: *const mm_idx_t,
    mut rid: uint32_t,
    mut x: uint32_t,
) -> libc::c_int {
    let mut i: int64_t = 0;
    let mut off0: int64_t = 0;
    let mut off: int64_t = 0;
    let mut c: libc::c_int = 0;
    off0 = (*((*mi).seq).offset(rid as isize)).offset as int64_t;
    off = off0 + x as int64_t;
    c = (*((*mi).S).offset((off >> 3 as libc::c_int) as isize)
        >> ((off & 7 as libc::c_long) << 2 as libc::c_int) & 15 as libc::c_uint)
        as libc::c_int;
    i = off - 1 as libc::c_long;
    while i >= off0 {
        if *((*mi).S).offset((i >> 3 as libc::c_int) as isize)
            >> ((i & 7 as libc::c_long) << 2 as libc::c_int) & 15 as libc::c_uint
            != c as libc::c_uint
        {
            break;
        }
        i -= 1;
    }
    return (off - i) as libc::c_int;
}
#[inline]
unsafe extern "C" fn mm_adjust_minier(
    mut mi: *const mm_idx_t,
    mut qseq0: *const *mut uint8_t,
    mut a: *mut mm128_t,
    mut r: *mut int32_t,
    mut q: *mut int32_t,
) {
    let mut qseq: *const uint8_t = 0 as *const uint8_t;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if (*mi).flag & 1 as libc::c_int != 0 {
        qseq = *qseq0.offset(((*a).x >> 63 as libc::c_int) as isize) as *const uint8_t;
        *q = (*a).y as int32_t;
        i = *q - 1 as libc::c_int;
        c = *qseq.offset(*q as isize) as libc::c_int;
        while i > 0 as libc::c_int {
            if *qseq.offset(i as isize) as libc::c_int != c {
                break;
            }
            i -= 1;
        }
        *q = i + 1 as libc::c_int;
        c = mm_get_hplen_back(
            mi,
            ((*a).x << 1 as libc::c_int >> 33 as libc::c_int) as uint32_t,
            (*a).x as int32_t as uint32_t,
        );
        *r = (*a).x as int32_t + 1 as libc::c_int - c;
    } else {
        *r = (*a).x as int32_t - ((*mi).k >> 1 as libc::c_int);
        *q = (*a).y as int32_t - ((*mi).k >> 1 as libc::c_int);
    };
}
unsafe extern "C" fn collect_long_gaps(
    mut km: *mut libc::c_void,
    mut as1: libc::c_int,
    mut cnt1: libc::c_int,
    mut a: *mut mm128_t,
    mut min_gap: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut K: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut gap: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut gap___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    *n_ = 0 as libc::c_int;
    i = 1 as libc::c_int;
    n = 0 as libc::c_int;
    while i < cnt1 {
        gap = ((*a.offset((as1 + i) as isize)).y as int32_t as uint64_t)
            .wrapping_sub((*a.offset((as1 + i - 1 as libc::c_int) as isize)).y)
            .wrapping_sub(
                ((*a.offset((as1 + i) as isize)).x as int32_t as uint64_t)
                    .wrapping_sub((*a.offset((as1 + i - 1 as libc::c_int) as isize)).x),
            ) as libc::c_int;
        if gap < -min_gap {
            n += 1;
        } else if gap > min_gap {
            n += 1;
        }
        i += 1;
    }
    if n <= 1 as libc::c_int {
        return 0 as *mut libc::c_int;
    }
    tmp = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    K = tmp as *mut libc::c_int;
    i = 1 as libc::c_int;
    n = 0 as libc::c_int;
    while i < cnt1 {
        gap___0 = ((*a.offset((as1 + i) as isize)).y as int32_t as uint64_t)
            .wrapping_sub((*a.offset((as1 + i - 1 as libc::c_int) as isize)).y)
            .wrapping_sub(
                ((*a.offset((as1 + i) as isize)).x as int32_t as uint64_t)
                    .wrapping_sub((*a.offset((as1 + i - 1 as libc::c_int) as isize)).x),
            ) as libc::c_int;
        if gap___0 < -min_gap {
            tmp___0 = n;
            n += 1;
            *K.offset(tmp___0 as isize) = i;
        } else if gap___0 > min_gap {
            tmp___0 = n;
            n += 1;
            *K.offset(tmp___0 as isize) = i;
        }
        i += 1;
    }
    *n_ = n;
    return K;
}
unsafe extern "C" fn mm_filter_bad_seeds(
    mut km: *mut libc::c_void,
    mut as1: libc::c_int,
    mut cnt1: libc::c_int,
    mut a: *mut mm128_t,
    mut min_gap: libc::c_int,
    mut diff_thres: libc::c_int,
    mut max_ext_len: libc::c_int,
    mut max_ext_cnt: libc::c_int,
) {
    let mut max_st: libc::c_int = 0;
    let mut max_en: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut K: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut gap: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut n_ins: libc::c_int = 0;
    let mut n_del: libc::c_int = 0;
    let mut qs: libc::c_int = 0;
    let mut rs: libc::c_int = 0;
    let mut max_diff: libc::c_int = 0;
    let mut max_diff_l: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    K = collect_long_gaps(km, as1, cnt1, a, min_gap, &mut n);
    if K as libc::c_ulong == 0 as *mut libc::c_int as libc::c_ulong {
        return;
    }
    max = 0 as libc::c_int;
    max_en = -(1 as libc::c_int);
    max_st = max_en;
    k = 0 as libc::c_int;
    let mut current_block_54: u64;
    loop {
        n_ins = 0 as libc::c_int;
        n_del = 0 as libc::c_int;
        max_diff = 0 as libc::c_int;
        max_diff_l = -(1 as libc::c_int);
        if k == n {
            current_block_54 = 5757010901403000099;
        } else if k >= max_en {
            current_block_54 = 5757010901403000099;
        } else {
            current_block_54 = 15925075030174552612;
        }
        match current_block_54 {
            5757010901403000099 => {
                if max_en > 0 as libc::c_int {
                    i = *K.offset(max_st as isize);
                    while i < *K.offset(max_en as isize) {
                        (*a.offset((as1 + i) as isize))
                            .y = ((*a.offset((as1 + i) as isize)).y as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << 41 as libc::c_int) as uint64_t;
                        i += 1;
                    }
                }
                max = 0 as libc::c_int;
                max_en = -(1 as libc::c_int);
                max_st = max_en;
                if k == n {
                    break;
                }
            }
            _ => {}
        }
        i = *K.offset(k as isize);
        gap = (*a.offset((as1 + i) as isize)).y as int32_t
            - (*a.offset((as1 + i - 1 as libc::c_int) as isize)).y as int32_t
            - ((*a.offset((as1 + i) as isize)).x)
                .wrapping_sub((*a.offset((as1 + i - 1 as libc::c_int) as isize)).x)
                as int32_t;
        if gap > 0 as libc::c_int {
            n_ins += gap;
        } else {
            n_del += -gap;
        }
        qs = (*a.offset((as1 + i - 1 as libc::c_int) as isize)).y as int32_t;
        rs = (*a.offset((as1 + i - 1 as libc::c_int) as isize)).x as int32_t;
        l = k + 1 as libc::c_int;
        while l < n {
            if !(l <= k + max_ext_cnt) {
                break;
            }
            j = *K.offset(l as isize);
            if (*a.offset((as1 + j) as isize)).y as int32_t - qs > max_ext_len {
                break;
            }
            if (*a.offset((as1 + j) as isize)).x as int32_t - rs > max_ext_len {
                break;
            }
            gap = (*a.offset((as1 + j) as isize)).y as int32_t
                - (*a.offset((as1 + j - 1 as libc::c_int) as isize)).y as int32_t
                - ((*a.offset((as1 + j) as isize)).x)
                    .wrapping_sub((*a.offset((as1 + j - 1 as libc::c_int) as isize)).x)
                    as int32_t;
            if gap > 0 as libc::c_int {
                n_ins += gap;
            } else {
                n_del += -gap;
            }
            tmp = abs(n_ins - n_del);
            diff = n_ins + n_del - tmp;
            if max_diff < diff {
                max_diff = diff;
                max_diff_l = l;
            }
            l += 1;
        }
        if max_diff > diff_thres {
            if max_diff > max {
                max = max_diff;
                max_st = k;
                max_en = max_diff_l;
            }
        }
        k += 1;
    }
    kfree(km, K as *mut libc::c_void);
}
unsafe extern "C" fn mm_filter_bad_seeds_alt(
    mut km: *mut libc::c_void,
    mut as1: libc::c_int,
    mut cnt1: libc::c_int,
    mut a: *mut mm128_t,
    mut min_gap: libc::c_int,
    mut max_ext: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut K: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut gap1: libc::c_int = 0;
    let mut re1: libc::c_int = 0;
    let mut qe1: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut gap2: libc::c_int = 0;
    let mut q_span_pre: libc::c_int = 0;
    let mut rs2: libc::c_int = 0;
    let mut qs2: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut j___0: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    K = collect_long_gaps(km, as1, cnt1, a, min_gap, &mut n);
    if K as libc::c_ulong == 0 as *mut libc::c_int as libc::c_ulong {
        return;
    }
    k = 0 as libc::c_int;
    while k < n {
        i = *K.offset(k as isize);
        gap1 = (*a.offset((as1 + i) as isize)).y as int32_t
            - (*a.offset((as1 + i - 1 as libc::c_int) as isize)).y as int32_t
            - ((*a.offset((as1 + i) as isize)).x as int32_t
                - (*a.offset((as1 + i - 1 as libc::c_int) as isize)).x as int32_t);
        re1 = (*a.offset((as1 + i) as isize)).x as int32_t;
        qe1 = (*a.offset((as1 + i) as isize)).y as int32_t;
        if gap1 > 0 as libc::c_int {
            gap1 = gap1;
        } else {
            gap1 = -gap1;
        }
        l = k + 1 as libc::c_int;
        while l < n {
            j = *K.offset(l as isize);
            if (*a.offset((as1 + j) as isize)).y as int32_t - qe1 > max_ext {
                break;
            }
            if (*a.offset((as1 + j) as isize)).x as int32_t - re1 > max_ext {
                break;
            }
            gap2 = (*a.offset((as1 + j) as isize)).y as int32_t
                - (*a.offset((as1 + j - 1 as libc::c_int) as isize)).y as int32_t
                - ((*a.offset((as1 + j) as isize)).x)
                    .wrapping_sub((*a.offset((as1 + j - 1 as libc::c_int) as isize)).x)
                    as int32_t;
            q_span_pre = ((*a.offset((as1 + j - 1 as libc::c_int) as isize)).y
                >> 32 as libc::c_int & 255 as libc::c_ulong) as libc::c_int;
            rs2 = (*a.offset((as1 + j - 1 as libc::c_int) as isize)).x as int32_t
                + q_span_pre;
            qs2 = (*a.offset((as1 + j - 1 as libc::c_int) as isize)).y as int32_t
                + q_span_pre;
            if rs2 - re1 < qs2 - qe1 {
                m = rs2 - re1;
            } else {
                m = qs2 - qe1;
            }
            if gap2 > 0 as libc::c_int {
                gap2 = gap2;
            } else {
                gap2 = -gap2;
            }
            if m > gap1 + gap2 {
                break;
            }
            re1 = (*a.offset((as1 + j) as isize)).x as int32_t;
            qe1 = (*a.offset((as1 + j) as isize)).y as int32_t;
            gap1 = gap2;
            l += 1;
        }
        if l > k + 1 as libc::c_int {
            end = *K.offset((l - 1 as libc::c_int) as isize);
            j___0 = *K.offset(k as isize);
            while j___0 < end {
                (*a.offset((as1 + j___0) as isize))
                    .y = ((*a.offset((as1 + j___0) as isize)).y as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 41 as libc::c_int) as uint64_t;
                j___0 += 1;
            }
            (*a.offset((as1 + end) as isize))
                .y = ((*a.offset((as1 + end) as isize)).y as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 40 as libc::c_int) as uint64_t;
        }
        k = l;
    }
    kfree(km, K as *mut libc::c_void);
}
unsafe extern "C" fn mm_fix_bad_ends(
    mut r: *const mm_reg1_t,
    mut a: *const mm128_t,
    mut bw: libc::c_int,
    mut min_match: libc::c_int,
    mut as_0: *mut int32_t,
    mut cnt: *mut int32_t,
) {
    let mut i: int32_t = 0;
    let mut l: int32_t = 0;
    let mut m: int32_t = 0;
    let mut lq: int32_t = 0;
    let mut lr: int32_t = 0;
    let mut min: int32_t = 0;
    let mut max: int32_t = 0;
    let mut q_span: int32_t = 0;
    let mut tmp: int32_t = 0;
    let mut lq___0: int32_t = 0;
    let mut lr___0: int32_t = 0;
    let mut min___0: int32_t = 0;
    let mut max___0: int32_t = 0;
    let mut q_span___0: int32_t = 0;
    let mut tmp___0: int32_t = 0;
    *as_0 = (*r).as_0;
    *cnt = (*r).cnt;
    if (*r).cnt < 3 as libc::c_int {
        return;
    }
    l = ((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
        as int32_t;
    m = l;
    i = (*r).as_0 + 1 as libc::c_int;
    while i < (*r).as_0 + (*r).cnt - 1 as libc::c_int {
        q_span = ((*a.offset(i as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
            as int32_t;
        if (*a.offset(i as isize)).y as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 40 as libc::c_int != 0
        {
            break;
        }
        lr = (*a.offset(i as isize)).x as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).x as int32_t;
        lq = (*a.offset(i as isize)).y as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).y as int32_t;
        if lr < lq {
            min = lr;
        } else {
            min = lq;
        }
        if lr > lq {
            max = lr;
        } else {
            max = lq;
        }
        if max - min > l >> 1 as libc::c_int {
            *as_0 = i;
        }
        l += min;
        if min < q_span {
            tmp = min;
        } else {
            tmp = q_span;
        }
        m += tmp;
        if l >= bw << 1 as libc::c_int {
            break;
        }
        if m >= min_match {
            if m >= bw {
                break;
            }
        }
        if m >= (*r).mlen >> 1 as libc::c_int {
            break;
        }
        i += 1;
    }
    *cnt = (*r).as_0 + (*r).cnt - *as_0;
    l = ((*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)).y
        >> 32 as libc::c_int & 255 as libc::c_ulong) as int32_t;
    m = l;
    i = (*r).as_0 + (*r).cnt - 2 as libc::c_int;
    while i > *as_0 {
        q_span___0 = ((*a.offset((i + 1 as libc::c_int) as isize)).y >> 32 as libc::c_int
            & 255 as libc::c_ulong) as int32_t;
        if (*a.offset((i + 1 as libc::c_int) as isize)).y as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 40 as libc::c_int != 0
        {
            break;
        }
        lr___0 = (*a.offset((i + 1 as libc::c_int) as isize)).x as int32_t
            - (*a.offset(i as isize)).x as int32_t;
        lq___0 = (*a.offset((i + 1 as libc::c_int) as isize)).y as int32_t
            - (*a.offset(i as isize)).y as int32_t;
        if lr___0 < lq___0 {
            min___0 = lr___0;
        } else {
            min___0 = lq___0;
        }
        if lr___0 > lq___0 {
            max___0 = lr___0;
        } else {
            max___0 = lq___0;
        }
        if max___0 - min___0 > l >> 1 as libc::c_int {
            *cnt = i + 1 as libc::c_int - *as_0;
        }
        l += min___0;
        if min___0 < q_span___0 {
            tmp___0 = min___0;
        } else {
            tmp___0 = q_span___0;
        }
        m += tmp___0;
        if l >= bw << 1 as libc::c_int {
            break;
        }
        if m >= min_match {
            if m >= bw {
                break;
            }
        }
        if m >= (*r).mlen >> 1 as libc::c_int {
            break;
        }
        i -= 1;
    }
}
unsafe extern "C" fn mm_max_stretch(
    mut r: *const mm_reg1_t,
    mut a: *const mm128_t,
    mut as_0: *mut int32_t,
    mut cnt: *mut int32_t,
) {
    let mut i: int32_t = 0;
    let mut score: int32_t = 0;
    let mut max_score: int32_t = 0;
    let mut len: int32_t = 0;
    let mut max_i: int32_t = 0;
    let mut max_len: int32_t = 0;
    let mut lq: int32_t = 0;
    let mut lr: int32_t = 0;
    let mut q_span: int32_t = 0;
    let mut tmp: int32_t = 0;
    *as_0 = (*r).as_0;
    *cnt = (*r).cnt;
    if (*r).cnt < 2 as libc::c_int {
        return;
    }
    max_score = -(1 as libc::c_int);
    max_i = -(1 as libc::c_int);
    max_len = 0 as libc::c_int;
    score = ((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int
        & 255 as libc::c_ulong) as int32_t;
    len = 1 as libc::c_int;
    i = (*r).as_0 + 1 as libc::c_int;
    while i < (*r).as_0 + (*r).cnt {
        q_span = ((*a.offset(i as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
            as int32_t;
        lr = (*a.offset(i as isize)).x as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).x as int32_t;
        lq = (*a.offset(i as isize)).y as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).y as int32_t;
        if lq == lr {
            if lq < q_span {
                tmp = lq;
            } else {
                tmp = q_span;
            }
            score += tmp;
            len += 1;
        } else {
            if score > max_score {
                max_score = score;
                max_len = len;
                max_i = i - len;
            }
            score = q_span;
            len = 1 as libc::c_int;
        }
        i += 1;
    }
    if score > max_score {
        max_score = score;
        max_len = len;
        max_i = i - len;
    }
    *as_0 = max_i;
    *cnt = max_len;
}
unsafe extern "C" fn mm_seed_ext_score(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut mat: *const int8_t,
    mut qlen: libc::c_int,
    mut qseq0: *mut *mut uint8_t,
    mut a: *const mm128_t,
) -> libc::c_int {
    let mut qseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut tseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut q_span: libc::c_int = 0;
    let mut qs: libc::c_int = 0;
    let mut qe: libc::c_int = 0;
    let mut rs: libc::c_int = 0;
    let mut re: libc::c_int = 0;
    let mut rid: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut q_off: libc::c_int = 0;
    let mut t_off: libc::c_int = 0;
    let mut ext_len: libc::c_int = 0;
    let mut qp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    q_span = ((*a).y >> 32 as libc::c_int & 255 as libc::c_ulong) as libc::c_int;
    ext_len = (*opt).anchor_ext_len;
    rid = ((*a).x << 1 as libc::c_int >> 33 as libc::c_int) as libc::c_int;
    re = ((*a).x as uint32_t).wrapping_add(1 as libc::c_uint) as libc::c_int;
    rs = re - q_span;
    qe = ((*a).y as uint32_t).wrapping_add(1 as libc::c_uint) as libc::c_int;
    qs = qe - q_span;
    if rs - ext_len > 0 as libc::c_int {
        rs -= ext_len;
    } else {
        rs = 0 as libc::c_int;
    }
    if qs - ext_len > 0 as libc::c_int {
        qs -= ext_len;
    } else {
        qs = 0 as libc::c_int;
    }
    if re + ext_len < (*((*mi).seq).offset(rid as isize)).len as int32_t {
        re += ext_len;
    } else {
        re = (*((*mi).seq).offset(rid as isize)).len as libc::c_int;
    }
    if qe + ext_len < qlen {
        qe += ext_len;
    } else {
        qe = qlen;
    }
    tmp = kmalloc(km, (re - rs) as size_t);
    tseq = tmp as *mut uint8_t;
    if (*opt).flag as libc::c_longlong & 4294967296 as libc::c_longlong != 0 {
        qseq = (*qseq0.offset(0 as libc::c_int as isize)).offset(qs as isize);
        mm_idx_getseq2(
            mi,
            ((*a).x >> 63 as libc::c_int) as libc::c_int,
            rid as uint32_t,
            rs as uint32_t,
            re as uint32_t,
            tseq,
        );
    } else {
        qseq = (*qseq0.offset(((*a).x >> 63 as libc::c_int) as isize))
            .offset(qs as isize);
        mm_idx_getseq(mi, rid as uint32_t, rs as uint32_t, re as uint32_t, tseq);
    }
    qp = ksw_ll_qinit(
        km,
        2 as libc::c_int,
        qe - qs,
        qseq as *const uint8_t,
        5 as libc::c_int,
        mat,
    );
    score = ksw_ll_i16(
        qp,
        re - rs,
        tseq as *const uint8_t,
        (*opt).q,
        (*opt).e,
        &mut q_off,
        &mut t_off,
    );
    kfree(km, tseq as *mut libc::c_void);
    kfree(km, qp);
    return score;
}
unsafe extern "C" fn mm_fix_bad_ends_splice(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut r: *const mm_reg1_t,
    mut mat: *const int8_t,
    mut qlen: libc::c_int,
    mut qseq0: *mut *mut uint8_t,
    mut a: *const mm128_t,
    mut as1: *mut libc::c_int,
    mut cnt1: *mut libc::c_int,
) {
    let mut score: libc::c_int = 0;
    let mut log_gap: libc::c_double = 0.;
    *as1 = (*r).as_0;
    *cnt1 = (*r).cnt;
    if (*r).cnt < 3 as libc::c_int {
        return;
    }
    log_gap = log(
        ((*a.offset(((*r).as_0 + 1 as libc::c_int) as isize)).x as int32_t
            - (*a.offset((*r).as_0 as isize)).x as int32_t) as libc::c_double,
    );
    if (((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
        as libc::c_double) < log_gap + (*opt).anchor_ext_shift as libc::c_double
    {
        score = mm_seed_ext_score(
            km,
            opt,
            mi,
            mat,
            qlen,
            qseq0,
            a.offset((*r).as_0 as isize),
        );
        if (score as libc::c_double
            / *mat.offset(0 as libc::c_int as isize) as libc::c_double)
            < log_gap + (*opt).anchor_ext_shift as libc::c_double
        {
            *as1 += 1;
            *cnt1 -= 1;
        }
    }
    log_gap = log(
        ((*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)).x as int32_t
            - (*a.offset(((*r).as_0 + (*r).cnt - 2 as libc::c_int) as isize)).x
                as int32_t) as libc::c_double,
    );
    if (((*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)).y
        >> 32 as libc::c_int & 255 as libc::c_ulong) as libc::c_double)
        < log_gap + (*opt).anchor_ext_shift as libc::c_double
    {
        score = mm_seed_ext_score(
            km,
            opt,
            mi,
            mat,
            qlen,
            qseq0,
            a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize),
        );
        if (score as libc::c_double
            / *mat.offset(0 as libc::c_int as isize) as libc::c_double)
            < log_gap + (*opt).anchor_ext_shift as libc::c_double
        {
            *cnt1 -= 1;
        }
    }
}
unsafe extern "C" fn mm_align1(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut qlen: libc::c_int,
    mut qseq0: *mut *mut uint8_t,
    mut r: *mut mm_reg1_t,
    mut r2: *mut mm_reg1_t,
    mut n_a: libc::c_int,
    mut a: *mut mm128_t,
    mut ez: *mut ksw_extz_t,
    mut splice_flag: libc::c_int,
) {
    let mut current_block: u64;
    let mut is_sr: libc::c_int = 0;
    let mut is_splice: libc::c_int = 0;
    let mut rid: int32_t = 0;
    let mut rev: int32_t = 0;
    let mut as1: int32_t = 0;
    let mut cnt1: int32_t = 0;
    let mut tseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut qseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut junc: *mut uint8_t = 0 as *mut uint8_t;
    let mut i: int32_t = 0;
    let mut l: int32_t = 0;
    let mut bw: int32_t = 0;
    let mut bw_long: int32_t = 0;
    let mut dropped: int32_t = 0;
    let mut extra_flag: int32_t = 0;
    let mut rs0: int32_t = 0;
    let mut re0: int32_t = 0;
    let mut qs0: int32_t = 0;
    let mut qe0: int32_t = 0;
    let mut rs: int32_t = 0;
    let mut re: int32_t = 0;
    let mut qs: int32_t = 0;
    let mut qe: int32_t = 0;
    let mut rs1: int32_t = 0;
    let mut qs1: int32_t = 0;
    let mut re1: int32_t = 0;
    let mut qe1: int32_t = 0;
    let mut mat: [int8_t; 25] = [0; 25];
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: int32_t = 0;
    let mut tmp___4: int32_t = 0;
    let mut x: int32_t = 0;
    let mut y: int32_t = 0;
    let mut tmp___6: int32_t = 0;
    let mut x___0: int32_t = 0;
    let mut y___0: int32_t = 0;
    let mut tmp___7: int32_t = 0;
    let mut max_ext: libc::c_int = 0;
    let mut tmp___8: int32_t = 0;
    let mut tmp___10: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___11: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: int32_t = 0;
    let mut j: libc::c_int = 0;
    let mut bw1: libc::c_int = 0;
    let mut zdrop_code: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut capacity: uint32_t = 0;
    let mut tmp___20: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: int32_t = 0;
    is_sr = ((*opt).flag & 4096 as libc::c_long != 0) as libc::c_int;
    is_splice = ((*opt).flag & 128 as libc::c_long != 0) as libc::c_int;
    rid = ((*a.offset((*r).as_0 as isize)).x << 1 as libc::c_int >> 33 as libc::c_int)
        as int32_t;
    rev = ((*a.offset((*r).as_0 as isize)).x >> 63 as libc::c_int) as int32_t;
    dropped = 0 as libc::c_int;
    extra_flag = 0 as libc::c_int;
    if is_sr != 0 {
        if (*mi).flag & 1 as libc::c_int != 0 {
            __assert_fail(
                b"!(mi->flag & MM_I_HPC)\0" as *const u8 as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                583 as libc::c_uint,
                b"mm_align1\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (*r2).cnt = 0 as libc::c_int;
    if (*r).cnt == 0 as libc::c_int {
        return;
    }
    ksw_gen_simple_mat(
        5 as libc::c_int,
        mat.as_mut_ptr(),
        (*opt).a as int8_t,
        (*opt).b as int8_t,
        (*opt).sc_ambi as int8_t,
    );
    bw = ((*opt).bw as libc::c_double * 1.5f64 + 1.0f64) as libc::c_int;
    bw_long = ((*opt).bw_long as libc::c_double * 1.5f64 + 1.0f64) as libc::c_int;
    if bw_long < bw {
        bw_long = bw;
    }
    let mut current_block_40: u64;
    if is_sr != 0 {
        if (*mi).flag & 1 as libc::c_int == 0 {
            mm_max_stretch(
                r as *const mm_reg1_t,
                a as *const mm128_t,
                &mut as1,
                &mut cnt1,
            );
            rs = (*a.offset(as1 as isize)).x as int32_t + 1 as libc::c_int
                - ((*a.offset(as1 as isize)).y >> 32 as libc::c_int
                    & 255 as libc::c_ulong) as int32_t;
            qs = (*a.offset(as1 as isize)).y as int32_t + 1 as libc::c_int
                - ((*a.offset(as1 as isize)).y >> 32 as libc::c_int
                    & 255 as libc::c_ulong) as int32_t;
            re = (*a.offset((as1 + cnt1 - 1 as libc::c_int) as isize)).x as int32_t
                + 1 as libc::c_int;
            qe = (*a.offset((as1 + cnt1 - 1 as libc::c_int) as isize)).y as int32_t
                + 1 as libc::c_int;
            current_block_40 = 13281731871476506071;
        } else {
            current_block_40 = 830570840939215732;
        }
    } else {
        current_block_40 = 830570840939215732;
    }
    match current_block_40 {
        830570840939215732 => {
            if (*opt).flag & 268435456 as libc::c_long == 0 {
                if is_splice != 0 {
                    mm_fix_bad_ends_splice(
                        km,
                        opt,
                        mi,
                        r as *const mm_reg1_t,
                        mat.as_mut_ptr() as *const int8_t,
                        qlen,
                        qseq0,
                        a as *const mm128_t,
                        &mut as1,
                        &mut cnt1,
                    );
                } else {
                    mm_fix_bad_ends(
                        r as *const mm_reg1_t,
                        a as *const mm128_t,
                        (*opt).bw,
                        (*opt).min_chain_score * 2 as libc::c_int,
                        &mut as1,
                        &mut cnt1,
                    );
                }
            } else {
                as1 = (*r).as_0;
                cnt1 = (*r).cnt;
            }
            mm_filter_bad_seeds(
                km,
                as1,
                cnt1,
                a,
                10 as libc::c_int,
                40 as libc::c_int,
                (*opt).max_gap >> 1 as libc::c_int,
                10 as libc::c_int,
            );
            mm_filter_bad_seeds_alt(
                km,
                as1,
                cnt1,
                a,
                30 as libc::c_int,
                (*opt).max_gap >> 1 as libc::c_int,
            );
            mm_adjust_minier(
                mi,
                qseq0 as *const *mut uint8_t,
                a.offset(as1 as isize),
                &mut rs,
                &mut qs,
            );
            mm_adjust_minier(
                mi,
                qseq0 as *const *mut uint8_t,
                a.offset((as1 + cnt1 - 1 as libc::c_int) as isize),
                &mut re,
                &mut qe,
            );
        }
        _ => {}
    }
    if !(cnt1 > 0 as libc::c_int) {
        __assert_fail(
            b"cnt1 > 0\0" as *const u8 as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            610 as libc::c_uint,
            b"mm_align1\0" as *const u8 as *const libc::c_char,
        );
    }
    if is_splice != 0 {
        if splice_flag & 256 as libc::c_int != 0 {
            if rev != 0 {
                tmp___1 = 512 as libc::c_int;
            } else {
                tmp___1 = 256 as libc::c_int;
            }
            extra_flag |= tmp___1;
        }
        if splice_flag & 512 as libc::c_int != 0 {
            if rev != 0 {
                tmp___2 = 256 as libc::c_int;
            } else {
                tmp___2 = 512 as libc::c_int;
            }
            extra_flag |= tmp___2;
        }
        if (*opt).flag & 262144 as libc::c_long != 0 {
            extra_flag |= 1024 as libc::c_int;
        }
    }
    if is_sr != 0 {
        qs0 = 0 as libc::c_int;
        qe0 = qlen;
        l = qs;
        if l * (*opt).a + (*opt).end_bonus > (*opt).q {
            tmp___3 = (l * (*opt).a + (*opt).end_bonus - (*opt).q) / (*opt).e;
        } else {
            tmp___3 = 0 as libc::c_int;
        }
        l += tmp___3;
        if rs - l > 0 as libc::c_int {
            rs0 = rs - l;
        } else {
            rs0 = 0 as libc::c_int;
        }
        l = qlen - qe;
        if l * (*opt).a + (*opt).end_bonus > (*opt).q {
            tmp___4 = (l * (*opt).a + (*opt).end_bonus - (*opt).q) / (*opt).e;
        } else {
            tmp___4 = 0 as libc::c_int;
        }
        l += tmp___4;
        if re + l < (*((*mi).seq).offset(rid as isize)).len as int32_t {
            re0 = re + l;
        } else {
            re0 = (*((*mi).seq).offset(rid as isize)).len as int32_t;
        }
    } else {
        rs0 = (*a.offset((*r).as_0 as isize)).x as int32_t + 1 as libc::c_int
            - ((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int
                & 255 as libc::c_ulong) as int32_t;
        qs0 = (*a.offset((*r).as_0 as isize)).y as int32_t + 1 as libc::c_int
            - ((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int
                & 255 as libc::c_ulong) as int32_t;
        if rs0 < 0 as libc::c_int {
            rs0 = 0 as libc::c_int;
        }
        if !(qs0 >= 0 as libc::c_int) {
            __assert_fail(
                b"qs0 >= 0\0" as *const u8 as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                636 as libc::c_uint,
                b"mm_align1\0" as *const u8 as *const libc::c_char,
            );
        }
        qs1 = 0 as libc::c_int;
        rs1 = qs1;
        i = (*r).as_0 - 1 as libc::c_int;
        l = 0 as libc::c_int;
        while i >= 0 as libc::c_int {
            if !((*a.offset(i as isize)).x >> 32 as libc::c_int
                == (*a.offset((*r).as_0 as isize)).x >> 32 as libc::c_int)
            {
                break;
            }
            x = (*a.offset(i as isize)).x as int32_t + 1 as libc::c_int
                - ((*a.offset(i as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
                    as int32_t;
            y = (*a.offset(i as isize)).y as int32_t + 1 as libc::c_int
                - ((*a.offset(i as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
                    as int32_t;
            if x < rs0 {
                if y < qs0 {
                    l += 1;
                    if l > (*opt).min_cnt {
                        if rs0 - x > qs0 - y {
                            l = rs0 - x;
                        } else {
                            l = qs0 - y;
                        }
                        rs1 = rs0 - l;
                        qs1 = qs0 - l;
                        if rs1 < 0 as libc::c_int {
                            rs1 = 0 as libc::c_int;
                        }
                        break;
                    }
                }
            }
            i -= 1;
        }
        if qs > 0 as libc::c_int {
            if rs > 0 as libc::c_int {
                if qs < (*opt).max_gap {
                    l = qs;
                } else {
                    l = (*opt).max_gap;
                }
                if qs1 > qs - l {
                    qs1 = qs1;
                } else {
                    qs1 = qs - l;
                }
                if qs0 < qs1 {
                    qs0 = qs0;
                } else {
                    qs0 = qs1;
                }
                if l * (*opt).a > (*opt).q {
                    tmp___6 = (l * (*opt).a - (*opt).q) / (*opt).e;
                } else {
                    tmp___6 = 0 as libc::c_int;
                }
                l += tmp___6;
                if l < (*opt).max_gap {
                    l = l;
                } else {
                    l = (*opt).max_gap;
                }
                if l < rs {
                    l = l;
                } else {
                    l = rs;
                }
                if rs1 > rs - l {
                    rs1 = rs1;
                } else {
                    rs1 = rs - l;
                }
                if rs0 < rs1 {
                    rs0 = rs0;
                } else {
                    rs0 = rs1;
                }
                if rs0 < rs {
                    rs0 = rs0;
                } else {
                    rs0 = rs;
                }
            } else {
                rs0 = rs;
                qs0 = qs;
            }
        } else {
            rs0 = rs;
            qs0 = qs;
        }
        re0 = (*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)).x
            as int32_t + 1 as libc::c_int;
        qe0 = (*a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize)).y
            as int32_t + 1 as libc::c_int;
        re1 = (*((*mi).seq).offset(rid as isize)).len as int32_t;
        qe1 = qlen;
        i = (*r).as_0 + (*r).cnt;
        l = 0 as libc::c_int;
        while i < n_a {
            if !((*a.offset(i as isize)).x >> 32 as libc::c_int
                == (*a.offset((*r).as_0 as isize)).x >> 32 as libc::c_int)
            {
                break;
            }
            x___0 = (*a.offset(i as isize)).x as int32_t + 1 as libc::c_int;
            y___0 = (*a.offset(i as isize)).y as int32_t + 1 as libc::c_int;
            if x___0 > re0 {
                if y___0 > qe0 {
                    l += 1;
                    if l > (*opt).min_cnt {
                        if x___0 - re0 > y___0 - qe0 {
                            l = x___0 - re0;
                        } else {
                            l = y___0 - qe0;
                        }
                        re1 = re0 + l;
                        qe1 = qe0 + l;
                        break;
                    }
                }
            }
            i += 1;
        }
        if qe < qlen {
            if re < (*((*mi).seq).offset(rid as isize)).len as int32_t {
                if qlen - qe < (*opt).max_gap {
                    l = qlen - qe;
                } else {
                    l = (*opt).max_gap;
                }
                if qe1 < qe + l {
                    qe1 = qe1;
                } else {
                    qe1 = qe + l;
                }
                if qe0 > qe1 {
                    qe0 = qe0;
                } else {
                    qe0 = qe1;
                }
                if l * (*opt).a > (*opt).q {
                    tmp___7 = (l * (*opt).a - (*opt).q) / (*opt).e;
                } else {
                    tmp___7 = 0 as libc::c_int;
                }
                l += tmp___7;
                if l < (*opt).max_gap {
                    l = l;
                } else {
                    l = (*opt).max_gap;
                }
                if l < (*((*mi).seq).offset(rid as isize)).len as int32_t - re {
                    l = l;
                } else {
                    l = ((*((*mi).seq).offset(rid as isize)).len)
                        .wrapping_sub(re as uint32_t) as int32_t;
                }
                if re1 < re + l {
                    re1 = re1;
                } else {
                    re1 = re + l;
                }
                if re0 > re1 {
                    re0 = re0;
                } else {
                    re0 = re1;
                }
            } else {
                re0 = re;
                qe0 = qe;
            }
        } else {
            re0 = re;
            qe0 = qe;
        }
    }
    if (*a.offset((*r).as_0 as isize)).y as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 43 as libc::c_int != 0
    {
        if (*r).qs > (*r).rs {
            tmp___8 = (*r).qs - (*r).rs;
        } else {
            tmp___8 = (*r).rs - (*r).qs;
        }
        max_ext = tmp___8;
        if (*r).rs - rs0 > max_ext {
            rs0 = (*r).rs - max_ext;
        }
        if (*r).qs - qs0 > max_ext {
            qs0 = (*r).qs - max_ext;
        }
        if (*r).qe > (*r).re {
            max_ext = (*r).qe - (*r).re;
        } else {
            max_ext = (*r).re - (*r).qe;
        }
        if re0 - (*r).re > max_ext {
            re0 = (*r).re + max_ext;
        }
        if qe0 - (*r).qe > max_ext {
            qe0 = (*r).qe + max_ext;
        }
    }
    if !(re0 > rs0) {
        __assert_fail(
            b"re0 > rs0\0" as *const u8 as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            696 as libc::c_uint,
            b"mm_align1\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___10 = kmalloc(km, (re0 - rs0) as size_t);
    tseq = tmp___10 as *mut uint8_t;
    tmp___11 = kmalloc(km, (re0 - rs0) as size_t);
    junc = tmp___11 as *mut uint8_t;
    if qs > 0 as libc::c_int {
        if rs > 0 as libc::c_int {
            if (*opt).flag as libc::c_longlong & 4294967296 as libc::c_longlong != 0 {
                qseq = (*qseq0.offset(0 as libc::c_int as isize)).offset(qs0 as isize);
                mm_idx_getseq2(
                    mi,
                    rev,
                    rid as uint32_t,
                    rs0 as uint32_t,
                    rs as uint32_t,
                    tseq,
                );
            } else {
                qseq = (*qseq0.offset(rev as isize)).offset(qs0 as isize);
                mm_idx_getseq(
                    mi,
                    rid as uint32_t,
                    rs0 as uint32_t,
                    rs as uint32_t,
                    tseq,
                );
            }
            mm_idx_bed_junc(mi, rid, rs0, rs, junc);
            mm_seq_rev((qs - qs0) as uint32_t, qseq);
            mm_seq_rev((rs - rs0) as uint32_t, tseq);
            mm_seq_rev((rs - rs0) as uint32_t, junc);
            if (*r).split_inv() != 0 {
                tmp___12 = (*opt).zdrop_inv;
            } else {
                tmp___12 = (*opt).zdrop;
            }
            mm_align_pair(
                km,
                opt,
                qs - qs0,
                qseq as *const uint8_t,
                rs - rs0,
                tseq as *const uint8_t,
                junc as *const uint8_t,
                mat.as_mut_ptr() as *const int8_t,
                bw,
                (*opt).end_bonus,
                tmp___12,
                extra_flag | 64 as libc::c_int | 2 as libc::c_int | 128 as libc::c_int,
                ez,
            );
            if (*ez).n_cigar > 0 as libc::c_int {
                mm_append_cigar(r, (*ez).n_cigar as uint32_t, (*ez).cigar);
                (*(*r).p)
                    .dp_score = ((*(*r).p).dp_score as uint32_t)
                    .wrapping_add((*ez).max()) as int32_t;
            }
            if (*ez).reach_end != 0 {
                tmp___13 = (*ez).mqe_t + 1 as libc::c_int;
            } else {
                tmp___13 = (*ez).max_t + 1 as libc::c_int;
            }
            rs1 = rs - tmp___13;
            if (*ez).reach_end != 0 {
                tmp___14 = qs - qs0;
            } else {
                tmp___14 = (*ez).max_q + 1 as libc::c_int;
            }
            qs1 = qs - tmp___14;
            mm_seq_rev((qs - qs0) as uint32_t, qseq);
        } else {
            rs1 = rs;
            qs1 = qs;
        }
    } else {
        rs1 = rs;
        qs1 = qs;
    }
    re1 = rs;
    qe1 = qs;
    if qs1 >= 0 as libc::c_int {
        if !(rs1 >= 0 as libc::c_int) {
            __assert_fail(
                b"qs1 >= 0 && rs1 >= 0\0" as *const u8 as *const libc::c_char,
                b"align.c\0" as *const u8 as *const libc::c_char,
                722 as libc::c_uint,
                b"mm_align1\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"qs1 >= 0 && rs1 >= 0\0" as *const u8 as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            722 as libc::c_uint,
            b"mm_align1\0" as *const u8 as *const libc::c_char,
        );
    }
    if is_sr != 0 {
        i = cnt1 - 1 as libc::c_int;
    } else {
        i = 1 as libc::c_int;
    }
    while i < cnt1 {
        if (*a.offset((as1 + i) as isize)).y as libc::c_ulonglong
            & ((1 as libc::c_ulonglong) << 41 as libc::c_int
                | (1 as libc::c_ulonglong) << 42 as libc::c_int) != 0
        {
            if i != cnt1 - 1 as libc::c_int {
                current_block = 7727693519809412702;
            } else {
                current_block = 16070719095729554596;
            }
        } else {
            current_block = 16070719095729554596;
        }
        match current_block {
            16070719095729554596 => {
                if is_sr != 0 {
                    if (*mi).flag & 1 as libc::c_int == 0 {
                        re = (*a.offset((as1 + i) as isize)).x as int32_t
                            + 1 as libc::c_int;
                        qe = (*a.offset((as1 + i) as isize)).y as int32_t
                            + 1 as libc::c_int;
                    } else {
                        mm_adjust_minier(
                            mi,
                            qseq0 as *const *mut uint8_t,
                            a.offset((as1 + i) as isize),
                            &mut re,
                            &mut qe,
                        );
                    }
                } else {
                    mm_adjust_minier(
                        mi,
                        qseq0 as *const *mut uint8_t,
                        a.offset((as1 + i) as isize),
                        &mut re,
                        &mut qe,
                    );
                }
                re1 = re;
                qe1 = qe;
                if i == cnt1 - 1 as libc::c_int {
                    current_block = 3362767779195377783;
                } else if (*a.offset((as1 + i) as isize)).y as libc::c_ulonglong
                        & (1 as libc::c_ulonglong) << 40 as libc::c_int != 0
                    {
                    current_block = 3362767779195377783;
                } else if qe - qs >= (*opt).min_ksw_len {
                    if re - rs >= (*opt).min_ksw_len {
                        current_block = 3362767779195377783;
                    } else {
                        current_block = 7727693519809412702;
                    }
                } else {
                    current_block = 7727693519809412702;
                }
                match current_block {
                    7727693519809412702 => {}
                    _ => {
                        bw1 = bw_long;
                        if (*a.offset((as1 + i) as isize)).y as libc::c_ulonglong
                            & (1 as libc::c_ulonglong) << 40 as libc::c_int != 0
                        {
                            if qe - qs > re - rs {
                                bw1 = qe - qs;
                            } else {
                                bw1 = re - rs;
                            }
                        }
                        if (*opt).flag as libc::c_longlong
                            & 4294967296 as libc::c_longlong != 0
                        {
                            qseq = (*qseq0.offset(0 as libc::c_int as isize))
                                .offset(qs as isize);
                            mm_idx_getseq2(
                                mi,
                                rev,
                                rid as uint32_t,
                                rs as uint32_t,
                                re as uint32_t,
                                tseq,
                            );
                        } else {
                            qseq = (*qseq0.offset(rev as isize)).offset(qs as isize);
                            mm_idx_getseq(
                                mi,
                                rid as uint32_t,
                                rs as uint32_t,
                                re as uint32_t,
                                tseq,
                            );
                        }
                        mm_idx_bed_junc(mi, rid, rs, re, junc);
                        if is_sr != 0 {
                            if !(qe - qs == re - rs) {
                                __assert_fail(
                                    b"qe - qs == re - rs\0" as *const u8 as *const libc::c_char,
                                    b"align.c\0" as *const u8 as *const libc::c_char,
                                    745 as libc::c_uint,
                                    b"mm_align1\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            ksw_reset_extz(ez);
                            j = 0 as libc::c_int;
                            (*ez).score = 0 as libc::c_int;
                            while j < qe - qs {
                                if *qseq.offset(j as isize) as libc::c_int
                                    >= 4 as libc::c_int
                                {
                                    (*ez).score += (*opt).e2;
                                } else if *tseq.offset(j as isize) as libc::c_int
                                        >= 4 as libc::c_int
                                    {
                                    (*ez).score += (*opt).e2;
                                } else {
                                    if *qseq.offset(j as isize) as libc::c_int
                                        == *tseq.offset(j as isize) as libc::c_int
                                    {
                                        tmp___17 = (*opt).a;
                                    } else {
                                        tmp___17 = -(*opt).b;
                                    }
                                    (*ez).score += tmp___17;
                                }
                                j += 1;
                            }
                            (*ez)
                                .cigar = ksw_push_cigar(
                                km,
                                &mut (*ez).n_cigar,
                                &mut (*ez).m_cigar,
                                (*ez).cigar,
                                0 as libc::c_int as uint32_t,
                                qe - qs,
                            );
                        } else {
                            mm_align_pair(
                                km,
                                opt,
                                qe - qs,
                                qseq as *const uint8_t,
                                re - rs,
                                tseq as *const uint8_t,
                                junc as *const uint8_t,
                                mat.as_mut_ptr() as *const int8_t,
                                bw1,
                                -(1 as libc::c_int),
                                (*opt).zdrop,
                                extra_flag | 8 as libc::c_int,
                                ez,
                            );
                        }
                        zdrop_code = mm_test_zdrop(
                            km,
                            opt,
                            qseq as *const uint8_t,
                            tseq as *const uint8_t,
                            (*ez).n_cigar as uint32_t,
                            (*ez).cigar,
                            mat.as_mut_ptr() as *const int8_t,
                        );
                        if zdrop_code != 0 as libc::c_int {
                            if zdrop_code == 2 as libc::c_int {
                                tmp___18 = (*opt).zdrop_inv;
                            } else {
                                tmp___18 = (*opt).zdrop;
                            }
                            mm_align_pair(
                                km,
                                opt,
                                qe - qs,
                                qseq as *const uint8_t,
                                re - rs,
                                tseq as *const uint8_t,
                                junc as *const uint8_t,
                                mat.as_mut_ptr() as *const int8_t,
                                bw1,
                                -(1 as libc::c_int),
                                tmp___18,
                                extra_flag,
                                ez,
                            );
                        }
                        if (*ez).n_cigar > 0 as libc::c_int {
                            mm_append_cigar(r, (*ez).n_cigar as uint32_t, (*ez).cigar);
                        }
                        if (*ez).zdropped() != 0 {
                            if ((*r).p).is_null() {
                                if !((*ez).n_cigar == 0 as libc::c_int) {
                                    __assert_fail(
                                        b"ez->n_cigar == 0\0" as *const u8 as *const libc::c_char,
                                        b"align.c\0" as *const u8 as *const libc::c_char,
                                        763 as libc::c_uint,
                                        b"mm_align1\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                                capacity = (::std::mem::size_of::<mm_extra_t>()
                                    as libc::c_ulong)
                                    .wrapping_div(4 as libc::c_ulong) as uint32_t;
                                capacity = capacity.wrapping_sub(1);
                                capacity |= capacity >> 1 as libc::c_int;
                                capacity |= capacity >> 2 as libc::c_int;
                                capacity |= capacity >> 4 as libc::c_int;
                                capacity |= capacity >> 8 as libc::c_int;
                                capacity |= capacity >> 16 as libc::c_int;
                                capacity = capacity.wrapping_add(1);
                                tmp___20 = calloc(
                                    capacity as size_t,
                                    4 as libc::c_int as size_t,
                                );
                                (*r).p = tmp___20 as *mut mm_extra_t;
                                (*(*r).p).capacity = capacity;
                            }
                            j = i - 1 as libc::c_int;
                            while j >= 0 as libc::c_int {
                                if (*a.offset((as1 + j) as isize)).x as int32_t
                                    <= rs + (*ez).max_t
                                {
                                    break;
                                }
                                j -= 1;
                            }
                            dropped = 1 as libc::c_int;
                            if j < 0 as libc::c_int {
                                j = 0 as libc::c_int;
                            }
                            (*(*r).p)
                                .dp_score = ((*(*r).p).dp_score as uint32_t)
                                .wrapping_add((*ez).max()) as int32_t;
                            re1 = rs + ((*ez).max_t + 1 as libc::c_int);
                            qe1 = qs + ((*ez).max_q + 1 as libc::c_int);
                            if cnt1 - (j + 1 as libc::c_int) >= (*opt).min_cnt {
                                mm_split_reg(
                                    r,
                                    r2,
                                    as1 + j + 1 as libc::c_int - (*r).as_0,
                                    qlen,
                                    a,
                                    ((*opt).flag as libc::c_longlong
                                        & 4294967296 as libc::c_longlong != 0) as libc::c_int,
                                );
                                if zdrop_code == 2 as libc::c_int {
                                    (*r2).set_split_inv(1 as libc::c_int as uint32_t);
                                }
                            }
                            break;
                        } else {
                            (*(*r).p).dp_score += (*ez).score;
                            rs = re;
                            qs = qe;
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    if dropped == 0 {
        if qe < qe0 {
            if re < re0 {
                if (*opt).flag as libc::c_longlong & 4294967296 as libc::c_longlong != 0
                {
                    qseq = (*qseq0.offset(0 as libc::c_int as isize))
                        .offset(qe as isize);
                    mm_idx_getseq2(
                        mi,
                        rev,
                        rid as uint32_t,
                        re as uint32_t,
                        re0 as uint32_t,
                        tseq,
                    );
                } else {
                    qseq = (*qseq0.offset(rev as isize)).offset(qe as isize);
                    mm_idx_getseq(
                        mi,
                        rid as uint32_t,
                        re as uint32_t,
                        re0 as uint32_t,
                        tseq,
                    );
                }
                mm_idx_bed_junc(mi, rid, re, re0, junc);
                mm_align_pair(
                    km,
                    opt,
                    qe0 - qe,
                    qseq as *const uint8_t,
                    re0 - re,
                    tseq as *const uint8_t,
                    junc as *const uint8_t,
                    mat.as_mut_ptr() as *const int8_t,
                    bw,
                    (*opt).end_bonus,
                    (*opt).zdrop,
                    extra_flag | 64 as libc::c_int,
                    ez,
                );
                if (*ez).n_cigar > 0 as libc::c_int {
                    mm_append_cigar(r, (*ez).n_cigar as uint32_t, (*ez).cigar);
                    (*(*r).p)
                        .dp_score = ((*(*r).p).dp_score as uint32_t)
                        .wrapping_add((*ez).max()) as int32_t;
                }
                if (*ez).reach_end != 0 {
                    tmp___21 = (*ez).mqe_t + 1 as libc::c_int;
                } else {
                    tmp___21 = (*ez).max_t + 1 as libc::c_int;
                }
                re1 = re + tmp___21;
                if (*ez).reach_end != 0 {
                    tmp___22 = qe0 - qe;
                } else {
                    tmp___22 = (*ez).max_q + 1 as libc::c_int;
                }
                qe1 = qe + tmp___22;
            }
        }
    }
    if !(qe1 <= qlen) {
        __assert_fail(
            b"qe1 <= qlen\0" as *const u8 as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            804 as libc::c_uint,
            b"mm_align1\0" as *const u8 as *const libc::c_char,
        );
    }
    (*r).rs = rs1;
    (*r).re = re1;
    if rev == 0 {
        (*r).qs = qs1;
        (*r).qe = qe1;
    } else if (*opt).flag as libc::c_longlong & 4294967296 as libc::c_longlong != 0 {
        (*r).qs = qs1;
        (*r).qe = qe1;
    } else {
        (*r).qs = qlen - qe1;
        (*r).qe = qlen - qs1;
    }
    if !(re1 - rs1 <= re0 - rs0) {
        __assert_fail(
            b"re1 - rs1 <= re0 - rs0\0" as *const u8 as *const libc::c_char,
            b"align.c\0" as *const u8 as *const libc::c_char,
            810 as libc::c_uint,
            b"mm_align1\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*r).p).is_null() {
        if (*opt).flag as libc::c_longlong & 4294967296 as libc::c_longlong != 0 {
            mm_idx_getseq2(
                mi,
                (*r).rev() as libc::c_int,
                rid as uint32_t,
                rs1 as uint32_t,
                re1 as uint32_t,
                tseq,
            );
            qseq = (*qseq0.offset(0 as libc::c_int as isize)).offset(qs1 as isize);
        } else {
            mm_idx_getseq(mi, rid as uint32_t, rs1 as uint32_t, re1 as uint32_t, tseq);
            qseq = (*qseq0.offset((*r).rev() as libc::c_int as isize))
                .offset(qs1 as isize);
        }
        mm_update_extra(
            r,
            qseq as *const uint8_t,
            tseq as *const uint8_t,
            mat.as_mut_ptr() as *const int8_t,
            (*opt).q as int8_t,
            (*opt).e as int8_t,
            ((*opt).flag & 67108864 as libc::c_long) as libc::c_int,
            ((*opt).flag & 4096 as libc::c_long == 0) as libc::c_int,
        );
        if rev != 0 {
            if (*(*r).p).trans_strand() != 0 {
                (*(*r).p)
                    .set_trans_strand(
                        (*(*r).p).trans_strand() ^ 3 as libc::c_uint as uint32_t,
                    );
            }
        }
    }
    kfree(km, tseq as *mut libc::c_void);
    kfree(km, junc as *mut libc::c_void);
}
unsafe extern "C" fn mm_align1_inv(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut qlen: libc::c_int,
    mut qseq0: *mut *mut uint8_t,
    mut r1: *const mm_reg1_t,
    mut r2: *const mm_reg1_t,
    mut r_inv: *mut mm_reg1_t,
    mut ez: *mut ksw_extz_t,
) -> libc::c_int {
    let mut tl: libc::c_int = 0;
    let mut ql: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut q_off: libc::c_int = 0;
    let mut t_off: libc::c_int = 0;
    let mut tseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut qseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut mat: [int8_t; 25] = [0; 25];
    let mut qp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    ret = 0 as libc::c_int;
    memset(
        r_inv as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong,
    );
    if (*r1).split() & 1 as libc::c_uint == 0 {
        return 0 as libc::c_int
    } else {
        if (*r2).split() & 2 as libc::c_uint == 0 {
            return 0 as libc::c_int;
        }
    }
    if (*r1).id != (*r1).parent {
        if (*r1).parent != -(2 as libc::c_int) {
            return 0 as libc::c_int;
        }
    }
    if (*r2).id != (*r2).parent {
        if (*r2).parent != -(2 as libc::c_int) {
            return 0 as libc::c_int;
        }
    }
    if (*r1).rid != (*r2).rid {
        return 0 as libc::c_int
    } else {
        if (*r1).rev() as libc::c_int != (*r2).rev() as libc::c_int {
            return 0 as libc::c_int;
        }
    }
    if (*r1).rev() != 0 {
        ql = (*r1).qs - (*r2).qe;
    } else {
        ql = (*r2).qs - (*r1).qe;
    }
    tl = (*r2).rs - (*r1).re;
    if ql < (*opt).min_chain_score {
        return 0 as libc::c_int
    } else {
        if ql > (*opt).max_gap {
            return 0 as libc::c_int;
        }
    }
    if tl < (*opt).min_chain_score {
        return 0 as libc::c_int
    } else {
        if tl > (*opt).max_gap {
            return 0 as libc::c_int;
        }
    }
    ksw_gen_simple_mat(
        5 as libc::c_int,
        mat.as_mut_ptr(),
        (*opt).a as int8_t,
        (*opt).b as int8_t,
        (*opt).sc_ambi as int8_t,
    );
    tmp = kmalloc(km, tl as size_t);
    tseq = tmp as *mut uint8_t;
    mm_idx_getseq(
        mi,
        (*r1).rid as uint32_t,
        (*r1).re as uint32_t,
        (*r2).rs as uint32_t,
        tseq,
    );
    if (*r1).rev() != 0 {
        qseq = (*qseq0.offset(0 as libc::c_int as isize)).offset((*r2).qe as isize);
    } else {
        qseq = (*qseq0.offset(1 as libc::c_int as isize))
            .offset((qlen - (*r2).qs) as isize);
    }
    mm_seq_rev(ql as uint32_t, qseq);
    mm_seq_rev(tl as uint32_t, tseq);
    qp = ksw_ll_qinit(
        km,
        2 as libc::c_int,
        ql,
        qseq as *const uint8_t,
        5 as libc::c_int,
        mat.as_mut_ptr() as *const int8_t,
    );
    score = ksw_ll_i16(
        qp,
        tl,
        tseq as *const uint8_t,
        (*opt).q,
        (*opt).e,
        &mut q_off,
        &mut t_off,
    );
    kfree(km, qp);
    mm_seq_rev(ql as uint32_t, qseq);
    mm_seq_rev(tl as uint32_t, tseq);
    if !(score < (*opt).min_dp_max) {
        q_off = ql - (q_off + 1 as libc::c_int);
        t_off = tl - (t_off + 1 as libc::c_int);
        mm_align_pair(
            km,
            opt,
            ql - q_off,
            qseq.offset(q_off as isize) as *const uint8_t,
            tl - t_off,
            tseq.offset(t_off as isize) as *const uint8_t,
            0 as *const uint8_t,
            mat.as_mut_ptr() as *const int8_t,
            ((*opt).bw as libc::c_double * 1.5f64) as libc::c_int,
            -(1 as libc::c_int),
            (*opt).zdrop,
            64 as libc::c_int,
            ez,
        );
        if !((*ez).n_cigar == 0 as libc::c_int) {
            mm_append_cigar(r_inv, (*ez).n_cigar as uint32_t, (*ez).cigar);
            (*(*r_inv).p).dp_score = (*ez).max() as int32_t;
            (*r_inv).id = -(1 as libc::c_int);
            (*r_inv).parent = -(1 as libc::c_int);
            (*r_inv).set_inv(1 as libc::c_int as uint32_t);
            (*r_inv).set_rev(((*r1).rev() == 0) as libc::c_int as uint32_t);
            (*r_inv).rid = (*r1).rid;
            (*r_inv).div = -1.0f32;
            if (*r_inv).rev() == 0 as libc::c_uint {
                (*r_inv).qs = (*r2).qe + q_off;
                (*r_inv).qe = (*r_inv).qs + (*ez).max_q + 1 as libc::c_int;
            } else {
                (*r_inv).qe = (*r2).qs - q_off;
                (*r_inv).qs = (*r_inv).qe - ((*ez).max_q + 1 as libc::c_int);
            }
            (*r_inv).rs = (*r1).re + t_off;
            (*r_inv).re = (*r_inv).rs + (*ez).max_t + 1 as libc::c_int;
            mm_update_extra(
                r_inv,
                qseq.offset(q_off as isize) as *const uint8_t,
                tseq.offset(t_off as isize) as *const uint8_t,
                mat.as_mut_ptr() as *const int8_t,
                (*opt).q as int8_t,
                (*opt).e as int8_t,
                ((*opt).flag & 67108864 as libc::c_long) as libc::c_int,
                ((*opt).flag & 4096 as libc::c_long == 0) as libc::c_int,
            );
            ret = 1 as libc::c_int;
        }
    }
    kfree(km, tseq as *mut libc::c_void);
    return ret;
}
#[inline]
unsafe extern "C" fn mm_insert_reg(
    mut r: *const mm_reg1_t,
    mut i: libc::c_int,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut mm_reg1_t,
) -> *mut mm_reg1_t {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = realloc(
        regs as *mut libc::c_void,
        ((*n_regs + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong),
    );
    regs = tmp as *mut mm_reg1_t;
    if i + 1 as libc::c_int != *n_regs {
        memmove(
            regs.offset((i + 2 as libc::c_int) as isize) as *mut libc::c_void,
            regs.offset((i + 1 as libc::c_int) as isize) as *const libc::c_void,
            (::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong)
                .wrapping_mul((*n_regs - i - 1 as libc::c_int) as libc::c_ulong),
        );
    }
    *regs.offset((i + 1 as libc::c_int) as isize) = *r;
    *n_regs += 1;
    return regs;
}
#[inline]
unsafe extern "C" fn mm_count_gaps(
    mut r: *const mm_reg1_t,
    mut n_gap_: *mut int32_t,
    mut n_gapo_: *mut int32_t,
) {
    let mut i: uint32_t = 0;
    let mut n_gapo: int32_t = 0;
    let mut n_gap: int32_t = 0;
    let mut tmp: int32_t = 0;
    let mut op: int32_t = 0;
    let mut len: int32_t = 0;
    n_gapo = 0 as libc::c_int;
    n_gap = 0 as libc::c_int;
    tmp = -(1 as libc::c_int);
    *n_gapo_ = tmp;
    *n_gap_ = tmp;
    if (*r).p as libc::c_ulong == 0 as *mut mm_extra_t as libc::c_ulong {
        return;
    }
    i = 0 as libc::c_int as uint32_t;
    while i < (*(*r).p).n_cigar {
        op = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize) & 15 as libc::c_uint)
            as int32_t;
        len = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize) >> 4 as libc::c_int)
            as int32_t;
        if op == 1 as libc::c_int {
            n_gapo += 1;
            n_gap += len;
        } else if op == 2 as libc::c_int {
            n_gapo += 1;
            n_gap += len;
        }
        i = i.wrapping_add(1);
    }
    *n_gap_ = n_gap;
    *n_gapo_ = n_gapo;
}
pub unsafe extern "C" fn mm_event_identity(mut r: *const mm_reg1_t) -> libc::c_double {
    let mut n_gap: int32_t = 0;
    let mut n_gapo: int32_t = 0;
    if (*r).p as libc::c_ulong == 0 as *mut mm_extra_t as libc::c_ulong {
        return -1.0f32 as libc::c_double;
    }
    mm_count_gaps(r, &mut n_gap, &mut n_gapo);
    return (*r).mlen as libc::c_double
        / ((*r).blen as uint32_t)
            .wrapping_add((*(*r).p).n_ambi())
            .wrapping_sub(n_gap as uint32_t)
            .wrapping_add(n_gapo as uint32_t) as libc::c_double;
}
unsafe extern "C" fn mm_recal_max_dp(
    mut r: *const mm_reg1_t,
    mut b2: libc::c_double,
    mut match_sc: int32_t,
) -> int32_t {
    let mut i: uint32_t = 0;
    let mut n_gap: int32_t = 0;
    let mut n_gapo: int32_t = 0;
    let mut n_mis: int32_t = 0;
    let mut gap_cost: libc::c_double = 0.;
    let mut op: int32_t = 0;
    let mut len: int32_t = 0;
    let mut tmp: libc::c_float = 0.;
    n_gap = 0 as libc::c_int;
    n_gapo = 0 as libc::c_int;
    gap_cost = 0.0f64;
    if (*r).p as libc::c_ulong == 0 as *mut mm_extra_t as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int as uint32_t;
    while i < (*(*r).p).n_cigar {
        op = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize) & 15 as libc::c_uint)
            as int32_t;
        len = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize) >> 4 as libc::c_int)
            as int32_t;
        if op == 1 as libc::c_int {
            tmp = mg_log2((1.0f64 + len as libc::c_double) as libc::c_float);
            gap_cost += b2 + tmp as libc::c_double;
            n_gapo += 1;
            n_gap += len;
        } else if op == 2 as libc::c_int {
            tmp = mg_log2((1.0f64 + len as libc::c_double) as libc::c_float);
            gap_cost += b2 + tmp as libc::c_double;
            n_gapo += 1;
            n_gap += len;
        }
        i = i.wrapping_add(1);
    }
    n_mis = ((*r).blen as uint32_t)
        .wrapping_add((*(*r).p).n_ambi())
        .wrapping_sub((*r).mlen as uint32_t)
        .wrapping_sub(n_gap as uint32_t) as int32_t;
    return (match_sc as libc::c_double
        * ((*r).mlen as libc::c_double - b2 * n_mis as libc::c_double - gap_cost)
        + 0.499f64) as int32_t;
}
pub unsafe extern "C" fn mm_update_dp_max(
    mut qlen: libc::c_int,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut frac: libc::c_float,
    mut a: libc::c_int,
    mut b: libc::c_int,
) {
    let mut max: int32_t = 0;
    let mut max2: int32_t = 0;
    let mut i: int32_t = 0;
    let mut max_i: int32_t = 0;
    let mut div___0: libc::c_double = 0.;
    let mut b2: libc::c_double = 0.;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut tmp: libc::c_double = 0.;
    let mut r___0: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    max = -(1 as libc::c_int);
    max2 = -(1 as libc::c_int);
    max_i = -(1 as libc::c_int);
    if n_regs < 2 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        r = regs.offset(i as isize);
        if !((*r).p as libc::c_ulong == 0 as *mut mm_extra_t as libc::c_ulong) {
            if (*(*r).p).dp_max > max {
                max2 = max;
                max = (*(*r).p).dp_max;
                max_i = i;
            } else if (*(*r).p).dp_max > max2 {
                max2 = (*(*r).p).dp_max;
            }
        }
        i += 1;
    }
    if max_i < 0 as libc::c_int {
        return
    } else {
        if max < 0 as libc::c_int {
            return
        } else {
            if max2 < 0 as libc::c_int {
                return;
            }
        }
    }
    if (((*regs.offset(max_i as isize)).qe - (*regs.offset(max_i as isize)).qs)
        as libc::c_double) < qlen as libc::c_double * frac as libc::c_double
    {
        return;
    }
    if (max2 as libc::c_double) < max as libc::c_double * frac as libc::c_double {
        return;
    }
    tmp = mm_event_identity(regs.offset(max_i as isize) as *const mm_reg1_t);
    div___0 = 1.0f64 - tmp;
    if div___0 < 0.02f64 {
        div___0 = 0.02f64;
    }
    b2 = 0.5f64 / div___0;
    if (b2 * a as libc::c_double) < b as libc::c_double {
        b2 = a as libc::c_double / b as libc::c_double;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        r___0 = regs.offset(i as isize);
        if !((*r___0).p as libc::c_ulong == 0 as *mut mm_extra_t as libc::c_ulong) {
            (*(*r___0).p).dp_max = mm_recal_max_dp(r___0 as *const mm_reg1_t, b2, a);
            if (*(*r___0).p).dp_max < 0 as libc::c_int {
                (*(*r___0).p).dp_max = 0 as libc::c_int;
            }
        }
        i += 1;
    }
}
pub unsafe extern "C" fn mm_align_skeleton(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut qlen: libc::c_int,
    mut qstr: *const libc::c_char,
    mut n_regs_: *mut libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut a: *mut mm128_t,
) -> *mut mm_reg1_t {
    let mut current_block: u64;
    let mut i: int32_t = 0;
    let mut n_regs: int32_t = 0;
    let mut n_a: int32_t = 0;
    let mut qseq0: [*mut uint8_t; 2] = [0 as *mut uint8_t; 2];
    let mut ez: ksw_extz_t = ksw_extz_t {
        max_zdropped: [0; 4],
        max_q: 0,
        max_t: 0,
        mqe: 0,
        mqe_t: 0,
        mte: 0,
        mte_q: 0,
        score: 0,
        m_cigar: 0,
        n_cigar: 0,
        reach_end: 0,
        cigar: 0 as *mut uint32_t,
    };
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r2: mm_reg1_t = mm_reg1_t {
        id: 0,
        cnt: 0,
        rid: 0,
        score: 0,
        qs: 0,
        qe: 0,
        rs: 0,
        re: 0,
        parent: 0,
        subsc: 0,
        as_0: 0,
        mlen: 0,
        blen: 0,
        n_sub: 0,
        score0: 0,
        mapq_split_rev_inv_sam_pri_proper_frag_pe_thru_seg_split_seg_id_split_inv_is_alt_strand_retained_dummy: [0; 4],
        hash: 0,
        div: 0.,
        p: 0 as *mut mm_extra_t,
    };
    let mut s: [mm_reg1_t; 2] = [mm_reg1_t {
        id: 0,
        cnt: 0,
        rid: 0,
        score: 0,
        qs: 0,
        qe: 0,
        rs: 0,
        re: 0,
        parent: 0,
        subsc: 0,
        as_0: 0,
        mlen: 0,
        blen: 0,
        n_sub: 0,
        score0: 0,
        mapq_split_rev_inv_sam_pri_proper_frag_pe_thru_seg_split_seg_id_split_inv_is_alt_strand_retained_dummy: [0; 4],
        hash: 0,
        div: 0.,
        p: 0 as *mut mm_extra_t,
    }; 2];
    let mut s2: [mm_reg1_t; 2] = [mm_reg1_t {
        id: 0,
        cnt: 0,
        rid: 0,
        score: 0,
        qs: 0,
        qe: 0,
        rs: 0,
        re: 0,
        parent: 0,
        subsc: 0,
        as_0: 0,
        mlen: 0,
        blen: 0,
        n_sub: 0,
        score0: 0,
        mapq_split_rev_inv_sam_pri_proper_frag_pe_thru_seg_split_seg_id_split_inv_is_alt_strand_retained_dummy: [0; 4],
        hash: 0,
        div: 0.,
        p: 0 as *mut mm_extra_t,
    }; 2];
    let mut which: libc::c_int = 0;
    let mut trans_strand: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    n_regs = *n_regs_;
    tmp = kmalloc(km, (qlen * 2 as libc::c_int) as size_t);
    qseq0[0 as libc::c_int as usize] = tmp as *mut uint8_t;
    qseq0[1 as libc::c_int
        as usize] = (qseq0[0 as libc::c_int as usize]).offset(qlen as isize);
    i = 0 as libc::c_int;
    while i < qlen {
        *(qseq0[0 as libc::c_int as usize])
            .offset(
                i as isize,
            ) = seq_nt4_table[*qstr.offset(i as isize) as uint8_t as usize];
        if (*(qseq0[0 as libc::c_int as usize]).offset(i as isize) as libc::c_int)
            < 4 as libc::c_int
        {
            *(qseq0[1 as libc::c_int as usize])
                .offset(
                    (qlen - 1 as libc::c_int - i) as isize,
                ) = (3 as libc::c_int
                - *(qseq0[0 as libc::c_int as usize]).offset(i as isize) as libc::c_int)
                as uint8_t;
        } else {
            *(qseq0[1 as libc::c_int as usize])
                .offset(
                    (qlen - 1 as libc::c_int - i) as isize,
                ) = 4 as libc::c_int as uint8_t;
        }
        i += 1;
    }
    n_a = mm_squeeze_a(km, n_regs, regs, a);
    memset(
        &mut ez as *mut ksw_extz_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ksw_extz_t>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < n_regs {
        if (*opt).flag & 128 as libc::c_long != 0 {
            if (*opt).flag & 256 as libc::c_long != 0 {
                if (*opt).flag & 512 as libc::c_long != 0 {
                    s[1 as libc::c_int as usize] = *regs.offset(i as isize);
                    s[0 as libc::c_int as usize] = s[1 as libc::c_int as usize];
                    mm_align1(
                        km,
                        opt,
                        mi,
                        qlen,
                        qseq0.as_mut_ptr(),
                        &mut *s.as_mut_ptr().offset(0 as libc::c_int as isize),
                        &mut *s2.as_mut_ptr().offset(0 as libc::c_int as isize),
                        n_a,
                        a,
                        &mut ez,
                        256 as libc::c_int,
                    );
                    mm_align1(
                        km,
                        opt,
                        mi,
                        qlen,
                        qseq0.as_mut_ptr(),
                        &mut *s.as_mut_ptr().offset(1 as libc::c_int as isize),
                        &mut *s2.as_mut_ptr().offset(1 as libc::c_int as isize),
                        n_a,
                        a,
                        &mut ez,
                        512 as libc::c_int,
                    );
                    if (*s[0 as libc::c_int as usize].p).dp_score
                        > (*s[1 as libc::c_int as usize].p).dp_score
                    {
                        which = 0 as libc::c_int;
                        trans_strand = 1 as libc::c_int;
                    } else if (*s[0 as libc::c_int as usize].p).dp_score
                            < (*s[1 as libc::c_int as usize].p).dp_score
                        {
                        which = 1 as libc::c_int;
                        trans_strand = 2 as libc::c_int;
                    } else {
                        trans_strand = 3 as libc::c_int;
                        which = qlen + (*s[0 as libc::c_int as usize].p).dp_score
                            & 1 as libc::c_int;
                    }
                    if which == 0 as libc::c_int {
                        *regs.offset(i as isize) = s[0 as libc::c_int as usize];
                        r2 = s2[0 as libc::c_int as usize];
                        free(s[1 as libc::c_int as usize].p as *mut libc::c_void);
                    } else {
                        *regs.offset(i as isize) = s[1 as libc::c_int as usize];
                        r2 = s2[1 as libc::c_int as usize];
                        free(s[0 as libc::c_int as usize].p as *mut libc::c_void);
                    }
                    let ref mut fresh6 = *(*regs.offset(i as isize)).p;
                    (*fresh6).set_trans_strand(trans_strand as uint32_t);
                    current_block = 16738040538446813684;
                } else {
                    current_block = 4217058444658958322;
                }
            } else {
                current_block = 4217058444658958322;
            }
        } else {
            current_block = 4217058444658958322;
        }
        match current_block {
            4217058444658958322 => {
                mm_align1(
                    km,
                    opt,
                    mi,
                    qlen,
                    qseq0.as_mut_ptr(),
                    regs.offset(i as isize),
                    &mut r2,
                    n_a,
                    a,
                    &mut ez,
                    (*opt).flag as libc::c_int,
                );
                if (*opt).flag & 128 as libc::c_long != 0 {
                    if (*opt).flag & 256 as libc::c_long != 0 {
                        let ref mut fresh7 = *(*regs.offset(i as isize)).p;
                        (*fresh7).set_trans_strand(1 as libc::c_int as uint32_t);
                    } else {
                        let ref mut fresh8 = *(*regs.offset(i as isize)).p;
                        (*fresh8).set_trans_strand(2 as libc::c_int as uint32_t);
                    }
                }
            }
            _ => {}
        }
        if r2.cnt > 0 as libc::c_int {
            regs = mm_insert_reg(
                &mut r2 as *mut mm_reg1_t as *const mm_reg1_t,
                i,
                &mut n_regs,
                regs,
            );
        }
        if i > 0 as libc::c_int {
            if (*regs.offset(i as isize)).split_inv() != 0 {
                if (*opt).flag as libc::c_longlong & 8589934592 as libc::c_longlong == 0
                {
                    tmp___0 = mm_align1_inv(
                        km,
                        opt,
                        mi,
                        qlen,
                        qseq0.as_mut_ptr(),
                        regs.offset((i - 1 as libc::c_int) as isize) as *const mm_reg1_t,
                        regs.offset(i as isize) as *const mm_reg1_t,
                        &mut r2,
                        &mut ez,
                    );
                    if tmp___0 != 0 {
                        regs = mm_insert_reg(
                            &mut r2 as *mut mm_reg1_t as *const mm_reg1_t,
                            i,
                            &mut n_regs,
                            regs,
                        );
                        i += 1;
                    }
                }
            }
        }
        i += 1;
    }
    *n_regs_ = n_regs;
    kfree(km, qseq0[0 as libc::c_int as usize] as *mut libc::c_void);
    kfree(km, ez.cigar as *mut libc::c_void);
    mm_filter_regs(opt, qlen, n_regs_, regs);
    if (*opt).flag & 4096 as libc::c_long == 0 {
        if ((*opt).split_prefix).is_null() {
            if qlen >= (*opt).rank_min_len {
                mm_update_dp_max(
                    qlen,
                    *n_regs_,
                    regs,
                    (*opt).rank_frac,
                    (*opt).a,
                    (*opt).b,
                );
                mm_filter_regs(opt, qlen, n_regs_, regs);
            }
        }
    }
    mm_hit_sort(km, n_regs_, regs, (*opt).alt_drop);
    return regs;
}
#[inline]
unsafe extern "C" fn mm_qname_len(mut s: *const libc::c_char) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = strlen(s);
    l = tmp as libc::c_int;
    if l >= 3 as libc::c_int {
        if *s.offset((l - 1 as libc::c_int) as isize) as libc::c_int >= 48 as libc::c_int
        {
            if *s.offset((l - 1 as libc::c_int) as isize) as libc::c_int
                <= 57 as libc::c_int
            {
                if *s.offset((l - 2 as libc::c_int) as isize) as libc::c_int
                    == 47 as libc::c_int
                {
                    tmp___0 = l - 2 as libc::c_int;
                } else {
                    tmp___0 = l;
                }
            } else {
                tmp___0 = l;
            }
        } else {
            tmp___0 = l;
        }
    } else {
        tmp___0 = l;
    }
    return tmp___0;
}
#[inline]
unsafe extern "C" fn mm_qname_same(
    mut s1: *const libc::c_char,
    mut s2: *const libc::c_char,
) -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    l1 = mm_qname_len(s1);
    l2 = mm_qname_len(s2);
    if l1 == l2 {
        tmp = strncmp(s1, s2, l1 as size_t);
        if tmp == 0 as libc::c_int {
            tmp___0 = 1 as libc::c_int;
        } else {
            tmp___0 = 0 as libc::c_int;
        }
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0;
}
pub unsafe extern "C" fn ks_init(mut f: gzFile) -> *mut kstream_t {
    let mut ks: *mut kstream_t = 0 as *mut kstream_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kstream_t>() as libc::c_ulong,
    );
    ks = tmp as *mut kstream_t;
    (*ks).f = f;
    (*ks).set_bufsize(16384 as libc::c_int);
    tmp___0 = malloc(16384 as libc::c_int as size_t);
    (*ks).buf = tmp___0 as *mut libc::c_uchar;
    return ks;
}
pub unsafe extern "C" fn ks_destroy(mut ks: *mut kstream_t) {
    if ks.is_null() {
        return;
    }
    free((*ks).buf as *mut libc::c_void);
    free(ks as *mut libc::c_void);
}
pub unsafe extern "C" fn ks_getuntil2(
    mut ks: *mut kstream_t,
    mut delimiter: libc::c_int,
    mut str: *mut kstring_t,
    mut dret: *mut libc::c_int,
    mut append: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___0: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    if !dret.is_null() {
        *dret = 0 as libc::c_int;
    }
    if append != 0 {
        (*str).l = (*str).l;
    } else {
        (*str).l = 0 as libc::c_int as size_t;
    }
    if (*ks).begin >= (*ks).end {
        if (*ks).is_eof() != 0 {
            return -(1 as libc::c_int);
        }
    }
    loop {
        if (*ks).begin >= (*ks).end {
            if !((*ks).is_eof() == 0) {
                break;
            }
            (*ks).begin = 0 as libc::c_int;
            (*ks)
                .end = gzread(
                (*ks).f,
                (*ks).buf as voidp,
                (*ks).bufsize() as libc::c_uint,
            );
            if (*ks).end < (*ks).bufsize() {
                (*ks).set_is_eof(1 as libc::c_int);
            }
            if (*ks).end == 0 as libc::c_int {
                break;
            }
        }
        if delimiter == 2 as libc::c_int {
            i = (*ks).begin;
            while i < (*ks).end {
                if *((*ks).buf).offset(i as isize) as libc::c_int == 10 as libc::c_int {
                    break;
                }
                i += 1;
            }
        } else if delimiter > 2 as libc::c_int {
            i = (*ks).begin;
            while i < (*ks).end {
                if *((*ks).buf).offset(i as isize) as libc::c_int == delimiter {
                    break;
                }
                i += 1;
            }
        } else if delimiter == 0 as libc::c_int {
            i = (*ks).begin;
            while i < (*ks).end {
                tmp = __ctype_b_loc();
                if *(*tmp)
                    .offset(*((*ks).buf).offset(i as isize) as libc::c_int as isize)
                    as libc::c_int & 8192 as libc::c_int != 0
                {
                    break;
                }
                i += 1;
            }
        } else if delimiter == 1 as libc::c_int {
            i = (*ks).begin;
            while i < (*ks).end {
                tmp___0 = __ctype_b_loc();
                if *(*tmp___0)
                    .offset(*((*ks).buf).offset(i as isize) as libc::c_int as isize)
                    as libc::c_int & 8192 as libc::c_int != 0
                {
                    if *((*ks).buf).offset(i as isize) as libc::c_int
                        != 32 as libc::c_int
                    {
                        break;
                    }
                }
                i += 1;
            }
        } else {
            i = 0 as libc::c_int;
        }
        if ((*str).m).wrapping_sub((*str).l)
            < (i - (*ks).begin + 1 as libc::c_int) as size_t
        {
            (*str)
                .m = ((*str).l)
                .wrapping_add((i - (*ks).begin) as size_t)
                .wrapping_add(1 as libc::c_ulong);
            (*str).m = ((*str).m).wrapping_sub(1);
            (*str).m |= (*str).m >> 1 as libc::c_int;
            (*str).m |= (*str).m >> 2 as libc::c_int;
            (*str).m |= (*str).m >> 4 as libc::c_int;
            (*str).m |= (*str).m >> 8 as libc::c_int;
            (*str).m |= (*str).m >> 16 as libc::c_int;
            (*str).m = ((*str).m).wrapping_add(1);
            tmp___1 = realloc((*str).s as *mut libc::c_void, (*str).m);
            (*str).s = tmp___1 as *mut libc::c_char;
        }
        memcpy(
            ((*str).s).offset((*str).l as isize) as *mut libc::c_void,
            ((*ks).buf).offset((*ks).begin as isize) as *const libc::c_void,
            (i - (*ks).begin) as size_t,
        );
        (*str)
            .l = ((*str).l as libc::c_ulong).wrapping_add((i - (*ks).begin) as size_t)
            as size_t as size_t;
        (*ks).begin = i + 1 as libc::c_int;
        if !(i < (*ks).end) {
            continue;
        }
        if !dret.is_null() {
            *dret = *((*ks).buf).offset(i as isize) as libc::c_int;
        }
        break;
    }
    if (*str).s as libc::c_ulong == 0 as *mut libc::c_char as libc::c_ulong {
        (*str).m = 1 as libc::c_int as size_t;
        tmp___2 = calloc(1 as libc::c_int as size_t, 1 as libc::c_int as size_t);
        (*str).s = tmp___2 as *mut libc::c_char;
    } else if delimiter == 2 as libc::c_int {
        if (*str).l > 1 as libc::c_ulong {
            if *((*str).s).offset(((*str).l).wrapping_sub(1 as libc::c_ulong) as isize)
                as libc::c_int == 13 as libc::c_int
            {
                (*str).l = ((*str).l).wrapping_sub(1);
            }
        }
    }
    *((*str).s).offset((*str).l as isize) = '\u{0}' as i32 as libc::c_char;
    return (*str).l as libc::c_int;
}
#[inline]
unsafe extern "C" fn ks_getc(mut ks: *mut kstream_t) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if (*ks).is_eof() != 0 {
        if (*ks).begin >= (*ks).end {
            return -(1 as libc::c_int);
        }
    }
    if (*ks).begin >= (*ks).end {
        (*ks).begin = 0 as libc::c_int;
        (*ks).end = gzread((*ks).f, (*ks).buf as voidp, (*ks).bufsize() as libc::c_uint);
        if (*ks).end < (*ks).bufsize() {
            (*ks).set_is_eof(1 as libc::c_int);
        }
        if (*ks).end == 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    tmp = (*ks).begin;
    (*ks).begin += 1;
    return *((*ks).buf).offset(tmp as isize) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ks_getuntil(
    mut ks: *mut kstream_t,
    mut delimiter: libc::c_int,
    mut str: *mut kstring_t,
    mut dret: *mut libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = ks_getuntil2(ks, delimiter, str, dret, 0 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn kseq_init(mut fd: gzFile) -> *mut kseq_t {
    let mut s: *mut kseq_t = 0 as *mut kseq_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kseq_t>() as libc::c_ulong,
    );
    s = tmp as *mut kseq_t;
    (*s).f = ks_init(fd);
    return s;
}
pub unsafe extern "C" fn kseq_destroy(mut ks: *mut kseq_t) {
    if ks.is_null() {
        return;
    }
    free((*ks).name.s as *mut libc::c_void);
    free((*ks).comment.s as *mut libc::c_void);
    free((*ks).seq.s as *mut libc::c_void);
    free((*ks).qual.s as *mut libc::c_void);
    ks_destroy((*ks).f);
    free(ks as *mut libc::c_void);
}
pub unsafe extern "C" fn kseq_read(mut seq: *mut kseq_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut ks: *mut kstream_t = 0 as *mut kstream_t;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___6: libc::c_int = 0;
    ks = (*seq).f;
    if (*seq).last_char == 0 as libc::c_int {
        loop {
            c = ks_getc(ks);
            if !(c != -(1 as libc::c_int)) {
                break;
            }
            if !(c != 62 as libc::c_int) {
                break;
            }
            if !(c != 64 as libc::c_int) {
                break;
            }
        }
        if c == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        (*seq).last_char = c;
    }
    tmp___0 = 0 as libc::c_int as size_t;
    (*seq).qual.l = tmp___0;
    tmp = tmp___0;
    (*seq).seq.l = tmp;
    (*seq).comment.l = tmp;
    tmp___1 = ks_getuntil(ks, 0 as libc::c_int, &mut (*seq).name, &mut c);
    if tmp___1 < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if c != 10 as libc::c_int {
        ks_getuntil(ks, 2 as libc::c_int, &mut (*seq).comment, 0 as *mut libc::c_int);
    }
    if (*seq).seq.s as libc::c_ulong == 0 as *mut libc::c_char as libc::c_ulong {
        (*seq).seq.m = 256 as libc::c_int as size_t;
        tmp___2 = malloc((*seq).seq.m);
        (*seq).seq.s = tmp___2 as *mut libc::c_char;
    }
    loop {
        c = ks_getc(ks);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if !(c != 62 as libc::c_int) {
            break;
        }
        if !(c != 43 as libc::c_int) {
            break;
        }
        if !(c != 64 as libc::c_int) {
            break;
        }
        if c == 10 as libc::c_int {
            continue;
        }
        tmp___3 = (*seq).seq.l;
        (*seq).seq.l = ((*seq).seq.l).wrapping_add(1);
        *((*seq).seq.s).offset(tmp___3 as isize) = c as libc::c_char;
        ks_getuntil2(
            ks,
            2 as libc::c_int,
            &mut (*seq).seq,
            0 as *mut libc::c_int,
            1 as libc::c_int,
        );
    }
    if c == 62 as libc::c_int {
        (*seq).last_char = c;
    } else if c == 64 as libc::c_int {
        (*seq).last_char = c;
    }
    if ((*seq).seq.l).wrapping_add(1 as libc::c_ulong) >= (*seq).seq.m {
        (*seq).seq.m = ((*seq).seq.l).wrapping_add(2 as libc::c_ulong);
        (*seq).seq.m = ((*seq).seq.m).wrapping_sub(1);
        (*seq).seq.m |= (*seq).seq.m >> 1 as libc::c_int;
        (*seq).seq.m |= (*seq).seq.m >> 2 as libc::c_int;
        (*seq).seq.m |= (*seq).seq.m >> 4 as libc::c_int;
        (*seq).seq.m |= (*seq).seq.m >> 8 as libc::c_int;
        (*seq).seq.m |= (*seq).seq.m >> 16 as libc::c_int;
        (*seq).seq.m = ((*seq).seq.m).wrapping_add(1);
        tmp___4 = realloc((*seq).seq.s as *mut libc::c_void, (*seq).seq.m);
        (*seq).seq.s = tmp___4 as *mut libc::c_char;
    }
    *((*seq).seq.s).offset((*seq).seq.l as isize) = 0 as libc::c_int as libc::c_char;
    if c != 43 as libc::c_int {
        return (*seq).seq.l as libc::c_int;
    }
    if (*seq).qual.m < (*seq).seq.m {
        (*seq).qual.m = (*seq).seq.m;
        tmp___5 = realloc((*seq).qual.s as *mut libc::c_void, (*seq).qual.m);
        (*seq).qual.s = tmp___5 as *mut libc::c_char;
    }
    loop {
        c = ks_getc(ks);
        if !(c != -(1 as libc::c_int)) {
            break;
        }
        if !(c != 10 as libc::c_int) {
            break;
        }
    }
    if c == -(1 as libc::c_int) {
        return -(2 as libc::c_int);
    }
    loop {
        tmp___6 = ks_getuntil2(
            ks,
            2 as libc::c_int,
            &mut (*seq).qual,
            0 as *mut libc::c_int,
            1 as libc::c_int,
        );
        if !(tmp___6 >= 0 as libc::c_int) {
            break;
        }
        if !((*seq).qual.l < (*seq).seq.l) {
            break;
        }
    }
    (*seq).last_char = 0 as libc::c_int;
    if (*seq).seq.l != (*seq).qual.l {
        return -(2 as libc::c_int);
    }
    return (*seq).seq.l as libc::c_int;
}
pub static mut seq_comp_table: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    5 as libc::c_int as libc::c_uchar,
    6 as libc::c_int as libc::c_uchar,
    7 as libc::c_int as libc::c_uchar,
    8 as libc::c_int as libc::c_uchar,
    9 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    11 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    13 as libc::c_int as libc::c_uchar,
    14 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    19 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    21 as libc::c_int as libc::c_uchar,
    22 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    27 as libc::c_int as libc::c_uchar,
    28 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    37 as libc::c_int as libc::c_uchar,
    38 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    42 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    44 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    48 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    50 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    52 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    62 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    64 as libc::c_int as libc::c_uchar,
    'T' as i32 as libc::c_uchar,
    'V' as i32 as libc::c_uchar,
    'G' as i32 as libc::c_uchar,
    'H' as i32 as libc::c_uchar,
    'E' as i32 as libc::c_uchar,
    'F' as i32 as libc::c_uchar,
    'C' as i32 as libc::c_uchar,
    'D' as i32 as libc::c_uchar,
    'I' as i32 as libc::c_uchar,
    'J' as i32 as libc::c_uchar,
    'M' as i32 as libc::c_uchar,
    'L' as i32 as libc::c_uchar,
    'K' as i32 as libc::c_uchar,
    'N' as i32 as libc::c_uchar,
    'O' as i32 as libc::c_uchar,
    'P' as i32 as libc::c_uchar,
    'Q' as i32 as libc::c_uchar,
    'Y' as i32 as libc::c_uchar,
    'S' as i32 as libc::c_uchar,
    'A' as i32 as libc::c_uchar,
    'A' as i32 as libc::c_uchar,
    'B' as i32 as libc::c_uchar,
    'W' as i32 as libc::c_uchar,
    'X' as i32 as libc::c_uchar,
    'R' as i32 as libc::c_uchar,
    'Z' as i32 as libc::c_uchar,
    91 as libc::c_int as libc::c_uchar,
    92 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    94 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    't' as i32 as libc::c_uchar,
    'v' as i32 as libc::c_uchar,
    'g' as i32 as libc::c_uchar,
    'h' as i32 as libc::c_uchar,
    'e' as i32 as libc::c_uchar,
    'f' as i32 as libc::c_uchar,
    'c' as i32 as libc::c_uchar,
    'd' as i32 as libc::c_uchar,
    'i' as i32 as libc::c_uchar,
    'j' as i32 as libc::c_uchar,
    'm' as i32 as libc::c_uchar,
    'l' as i32 as libc::c_uchar,
    'k' as i32 as libc::c_uchar,
    'n' as i32 as libc::c_uchar,
    'o' as i32 as libc::c_uchar,
    'p' as i32 as libc::c_uchar,
    'q' as i32 as libc::c_uchar,
    'y' as i32 as libc::c_uchar,
    's' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'a' as i32 as libc::c_uchar,
    'b' as i32 as libc::c_uchar,
    'w' as i32 as libc::c_uchar,
    'x' as i32 as libc::c_uchar,
    'r' as i32 as libc::c_uchar,
    'z' as i32 as libc::c_uchar,
    123 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    125 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    128 as libc::c_int as libc::c_uchar,
    129 as libc::c_int as libc::c_uchar,
    130 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    132 as libc::c_int as libc::c_uchar,
    133 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    135 as libc::c_int as libc::c_uchar,
    136 as libc::c_int as libc::c_uchar,
    137 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    139 as libc::c_int as libc::c_uchar,
    140 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    143 as libc::c_int as libc::c_uchar,
    144 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    146 as libc::c_int as libc::c_uchar,
    147 as libc::c_int as libc::c_uchar,
    148 as libc::c_int as libc::c_uchar,
    149 as libc::c_int as libc::c_uchar,
    150 as libc::c_int as libc::c_uchar,
    151 as libc::c_int as libc::c_uchar,
    152 as libc::c_int as libc::c_uchar,
    153 as libc::c_int as libc::c_uchar,
    154 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    156 as libc::c_int as libc::c_uchar,
    157 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    159 as libc::c_int as libc::c_uchar,
    160 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    162 as libc::c_int as libc::c_uchar,
    163 as libc::c_int as libc::c_uchar,
    164 as libc::c_int as libc::c_uchar,
    165 as libc::c_int as libc::c_uchar,
    166 as libc::c_int as libc::c_uchar,
    167 as libc::c_int as libc::c_uchar,
    168 as libc::c_int as libc::c_uchar,
    169 as libc::c_int as libc::c_uchar,
    170 as libc::c_int as libc::c_uchar,
    171 as libc::c_int as libc::c_uchar,
    172 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    174 as libc::c_int as libc::c_uchar,
    175 as libc::c_int as libc::c_uchar,
    176 as libc::c_int as libc::c_uchar,
    177 as libc::c_int as libc::c_uchar,
    178 as libc::c_int as libc::c_uchar,
    179 as libc::c_int as libc::c_uchar,
    180 as libc::c_int as libc::c_uchar,
    181 as libc::c_int as libc::c_uchar,
    182 as libc::c_int as libc::c_uchar,
    183 as libc::c_int as libc::c_uchar,
    184 as libc::c_int as libc::c_uchar,
    185 as libc::c_int as libc::c_uchar,
    186 as libc::c_int as libc::c_uchar,
    187 as libc::c_int as libc::c_uchar,
    188 as libc::c_int as libc::c_uchar,
    189 as libc::c_int as libc::c_uchar,
    190 as libc::c_int as libc::c_uchar,
    191 as libc::c_int as libc::c_uchar,
    192 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    194 as libc::c_int as libc::c_uchar,
    195 as libc::c_int as libc::c_uchar,
    196 as libc::c_int as libc::c_uchar,
    197 as libc::c_int as libc::c_uchar,
    198 as libc::c_int as libc::c_uchar,
    199 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    201 as libc::c_int as libc::c_uchar,
    202 as libc::c_int as libc::c_uchar,
    203 as libc::c_int as libc::c_uchar,
    204 as libc::c_int as libc::c_uchar,
    205 as libc::c_int as libc::c_uchar,
    206 as libc::c_int as libc::c_uchar,
    207 as libc::c_int as libc::c_uchar,
    208 as libc::c_int as libc::c_uchar,
    209 as libc::c_int as libc::c_uchar,
    210 as libc::c_int as libc::c_uchar,
    211 as libc::c_int as libc::c_uchar,
    212 as libc::c_int as libc::c_uchar,
    213 as libc::c_int as libc::c_uchar,
    214 as libc::c_int as libc::c_uchar,
    215 as libc::c_int as libc::c_uchar,
    216 as libc::c_int as libc::c_uchar,
    217 as libc::c_int as libc::c_uchar,
    218 as libc::c_int as libc::c_uchar,
    219 as libc::c_int as libc::c_uchar,
    220 as libc::c_int as libc::c_uchar,
    221 as libc::c_int as libc::c_uchar,
    222 as libc::c_int as libc::c_uchar,
    223 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    225 as libc::c_int as libc::c_uchar,
    226 as libc::c_int as libc::c_uchar,
    227 as libc::c_int as libc::c_uchar,
    228 as libc::c_int as libc::c_uchar,
    229 as libc::c_int as libc::c_uchar,
    230 as libc::c_int as libc::c_uchar,
    231 as libc::c_int as libc::c_uchar,
    232 as libc::c_int as libc::c_uchar,
    233 as libc::c_int as libc::c_uchar,
    234 as libc::c_int as libc::c_uchar,
    235 as libc::c_int as libc::c_uchar,
    236 as libc::c_int as libc::c_uchar,
    237 as libc::c_int as libc::c_uchar,
    238 as libc::c_int as libc::c_uchar,
    239 as libc::c_int as libc::c_uchar,
    240 as libc::c_int as libc::c_uchar,
    241 as libc::c_int as libc::c_uchar,
    242 as libc::c_int as libc::c_uchar,
    243 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    245 as libc::c_int as libc::c_uchar,
    246 as libc::c_int as libc::c_uchar,
    247 as libc::c_int as libc::c_uchar,
    248 as libc::c_int as libc::c_uchar,
    249 as libc::c_int as libc::c_uchar,
    250 as libc::c_int as libc::c_uchar,
    251 as libc::c_int as libc::c_uchar,
    252 as libc::c_int as libc::c_uchar,
    253 as libc::c_int as libc::c_uchar,
    254 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
];
pub unsafe extern "C" fn mm_bseq_open(
    mut fn_0: *const libc::c_char,
) -> *mut mm_bseq_file_t {
    let mut fp: *mut mm_bseq_file_t = 0 as *mut mm_bseq_file_t;
    let mut f: gzFile = 0 as *mut gzFile_s;
    let mut tmp___0: gzFile = 0 as *mut gzFile_s;
    let mut tmp___1: gzFile = 0 as *mut gzFile_s;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    if !fn_0.is_null() {
        tmp___2 = strcmp(fn_0, b"-\0" as *const u8 as *const libc::c_char);
        if tmp___2 != 0 {
            tmp___0 = gzopen(fn_0, b"r\0" as *const u8 as *const libc::c_char);
            f = tmp___0;
        } else {
            tmp___1 = gzdopen(
                0 as libc::c_int,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            f = tmp___1;
        }
    } else {
        tmp___1 = gzdopen(0 as libc::c_int, b"r\0" as *const u8 as *const libc::c_char);
        f = tmp___1;
    }
    if f as libc::c_ulong == 0 as gzFile as libc::c_ulong {
        return 0 as *mut mm_bseq_file_t;
    }
    tmp___3 = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<mm_bseq_file_t>() as libc::c_ulong,
    );
    fp = tmp___3 as *mut mm_bseq_file_t;
    (*fp).fp = f;
    (*fp).ks = kseq_init((*fp).fp);
    return fp;
}
pub unsafe extern "C" fn mm_bseq_close(mut fp: *mut mm_bseq_file_t) {
    kseq_destroy((*fp).ks);
    gzclose((*fp).fp);
    free(fp as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn kstrdup(mut s: *const kstring_t) -> *mut libc::c_char {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(((*s).l).wrapping_add(1 as libc::c_ulong));
    t = tmp as *mut libc::c_char;
    memcpy(
        t as *mut libc::c_void,
        (*s).s as *const libc::c_void,
        ((*s).l).wrapping_add(1 as libc::c_ulong),
    );
    return t;
}
#[inline]
unsafe extern "C" fn kseq2bseq(
    mut ks: *mut kseq_t,
    mut s: *mut mm_bseq1_t,
    mut with_qual: libc::c_int,
    mut with_comment: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*ks).name.l == 0 as libc::c_ulong {
        fprintf(
            stderr,
            b"[WARNING]\x1B[1;31m empty sequence name in the input.\x1B[0m\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    (*s).name = kstrdup(&mut (*ks).name as *mut kstring_t as *const kstring_t);
    (*s).seq = kstrdup(&mut (*ks).seq as *mut kstring_t as *const kstring_t);
    i = 0 as libc::c_int;
    while i < (*ks).seq.l as libc::c_int {
        if *((*s).seq).offset(i as isize) as libc::c_int == 117 as libc::c_int {
            *((*s).seq)
                .offset(
                    i as isize,
                ) = (*((*s).seq).offset(i as isize) as libc::c_int - 1 as libc::c_int)
                as libc::c_char;
        } else if *((*s).seq).offset(i as isize) as libc::c_int == 85 as libc::c_int {
            *((*s).seq)
                .offset(
                    i as isize,
                ) = (*((*s).seq).offset(i as isize) as libc::c_int - 1 as libc::c_int)
                as libc::c_char;
        }
        i += 1;
    }
    if with_qual != 0 {
        if (*ks).qual.l != 0 {
            tmp = kstrdup(&mut (*ks).qual as *mut kstring_t as *const kstring_t);
            (*s).qual = tmp;
        } else {
            (*s).qual = 0 as *mut libc::c_char;
        }
    } else {
        (*s).qual = 0 as *mut libc::c_char;
    }
    if with_comment != 0 {
        if (*ks).comment.l != 0 {
            tmp___0 = kstrdup(&mut (*ks).comment as *mut kstring_t as *const kstring_t);
            (*s).comment = tmp___0;
        } else {
            (*s).comment = 0 as *mut libc::c_char;
        }
    } else {
        (*s).comment = 0 as *mut libc::c_char;
    }
    (*s).l_seq = (*ks).seq.l as libc::c_int;
}
pub unsafe extern "C" fn mm_bseq_read3(
    mut fp: *mut mm_bseq_file_t,
    mut chunk_size: int64_t,
    mut with_qual: libc::c_int,
    mut with_comment: libc::c_int,
    mut frag_mode: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut mm_bseq1_t {
    let mut size: int64_t = 0;
    let mut ret: libc::c_int = 0;
    let mut a: __anonstruct_a_687107232 = __anonstruct_a_687107232 {
        n: 0,
        m: 0,
        a: 0 as *mut mm_bseq1_t,
    };
    let mut ks: *mut kseq_t = 0 as *mut kseq_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: size_t = 0;
    let mut s: *mut mm_bseq1_t = 0 as *mut mm_bseq1_t;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: size_t = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: size_t = 0;
    let mut tmp___8: libc::c_int = 0;
    size = 0 as libc::c_int as int64_t;
    a.n = 0 as libc::c_int as size_t;
    a.m = 0 as libc::c_int as size_t;
    a.a = 0 as *mut mm_bseq1_t;
    ks = (*fp).ks;
    *n_ = 0 as libc::c_int;
    if !((*fp).s.seq).is_null() {
        if a.m < 256 as libc::c_ulong {
            a.m = 256 as libc::c_int as size_t;
            a.m = (a.m).wrapping_sub(1);
            a.m |= a.m >> 1 as libc::c_int;
            a.m |= a.m >> 2 as libc::c_int;
            a.m |= a.m >> 4 as libc::c_int;
            a.m |= a.m >> 8 as libc::c_int;
            a.m |= a.m >> 16 as libc::c_int;
            a.m = (a.m).wrapping_add(1);
            tmp = krealloc(
                0 as *mut libc::c_void,
                a.a as *mut libc::c_void,
                (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong).wrapping_mul(a.m),
            );
            a.a = tmp as *mut mm_bseq1_t;
        }
        if a.n == a.m {
            if a.m != 0 {
                a.m <<= 1 as libc::c_int;
            } else {
                a.m = 2 as libc::c_int as size_t;
            }
            tmp___0 = krealloc(
                0 as *mut libc::c_void,
                a.a as *mut libc::c_void,
                (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong).wrapping_mul(a.m),
            );
            a.a = tmp___0 as *mut mm_bseq1_t;
        }
        tmp___1 = a.n;
        a.n = (a.n).wrapping_add(1);
        *(a.a).offset(tmp___1 as isize) = (*fp).s;
        size = (*fp).s.l_seq as int64_t;
        memset(
            &mut (*fp).s as *mut mm_bseq1_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong,
        );
    }
    loop {
        ret = kseq_read(ks);
        if !(ret >= 0 as libc::c_int) {
            break;
        }
        if !((*ks).seq.l <= 2147483647 as libc::c_ulong) {
            __assert_fail(
                b"ks->seq.l <= INT32_MAX\0" as *const u8 as *const libc::c_char,
                b"bseq.c\0" as *const u8 as *const libc::c_char,
                95 as libc::c_uint,
                b"mm_bseq_read3\0" as *const u8 as *const libc::c_char,
            );
        }
        if a.m == 0 as libc::c_ulong {
            if a.m < 256 as libc::c_ulong {
                a.m = 256 as libc::c_int as size_t;
                a.m = (a.m).wrapping_sub(1);
                a.m |= a.m >> 1 as libc::c_int;
                a.m |= a.m >> 2 as libc::c_int;
                a.m |= a.m >> 4 as libc::c_int;
                a.m |= a.m >> 8 as libc::c_int;
                a.m |= a.m >> 16 as libc::c_int;
                a.m = (a.m).wrapping_add(1);
                tmp___3 = krealloc(
                    0 as *mut libc::c_void,
                    a.a as *mut libc::c_void,
                    (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong)
                        .wrapping_mul(a.m),
                );
                a.a = tmp___3 as *mut mm_bseq1_t;
            }
        }
        if a.n == a.m {
            if a.m != 0 {
                a.m <<= 1 as libc::c_int;
            } else {
                a.m = 2 as libc::c_int as size_t;
            }
            tmp___4 = krealloc(
                0 as *mut libc::c_void,
                a.a as *mut libc::c_void,
                (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong).wrapping_mul(a.m),
            );
            a.a = tmp___4 as *mut mm_bseq1_t;
        }
        tmp___5 = a.n;
        a.n = (a.n).wrapping_add(1);
        s = (a.a).offset(tmp___5 as isize);
        kseq2bseq(ks, s, with_qual, with_comment);
        size += (*s).l_seq as int64_t;
        if !(size >= chunk_size) {
            continue;
        }
        if frag_mode != 0 {
            if (*(a.a).offset((a.n).wrapping_sub(1 as libc::c_ulong) as isize)).l_seq
                < 1000000 as libc::c_int
            {
                loop {
                    ret = kseq_read(ks);
                    if !(ret >= 0 as libc::c_int) {
                        break;
                    }
                    kseq2bseq(ks, &mut (*fp).s, with_qual, with_comment);
                    tmp___8 = mm_qname_same(
                        (*fp).s.name as *const libc::c_char,
                        (*(a.a).offset((a.n).wrapping_sub(1 as libc::c_ulong) as isize))
                            .name as *const libc::c_char,
                    );
                    if !(tmp___8 != 0) {
                        break;
                    }
                    if a.n == a.m {
                        if a.m != 0 {
                            a.m <<= 1 as libc::c_int;
                        } else {
                            a.m = 2 as libc::c_int as size_t;
                        }
                        tmp___6 = krealloc(
                            0 as *mut libc::c_void,
                            a.a as *mut libc::c_void,
                            (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong)
                                .wrapping_mul(a.m),
                        );
                        a.a = tmp___6 as *mut mm_bseq1_t;
                    }
                    tmp___7 = a.n;
                    a.n = (a.n).wrapping_add(1);
                    *(a.a).offset(tmp___7 as isize) = (*fp).s;
                    memset(
                        &mut (*fp).s as *mut mm_bseq1_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong,
                    );
                }
            }
        }
        break;
    }
    if ret < -(1 as libc::c_int) {
        if a.n != 0 {
            fprintf(
                stderr,
                b"[WARNING]\x1B[1;31m failed to parse the FASTA/FASTQ record next to '%s'. Continue anyway.\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
                (*(a.a).offset((a.n).wrapping_sub(1 as libc::c_ulong) as isize)).name,
            );
        } else {
            fprintf(
                stderr,
                b"[WARNING]\x1B[1;31m failed to parse the first FASTA/FASTQ record. Continue anyway.\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    *n_ = a.n as libc::c_int;
    return a.a;
}
pub unsafe extern "C" fn mm_bseq_read2(
    mut fp: *mut mm_bseq_file_t,
    mut chunk_size: int64_t,
    mut with_qual: libc::c_int,
    mut frag_mode: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut mm_bseq1_t {
    let mut tmp: *mut mm_bseq1_t = 0 as *mut mm_bseq1_t;
    tmp = mm_bseq_read3(fp, chunk_size, with_qual, 0 as libc::c_int, frag_mode, n_);
    return tmp;
}
pub unsafe extern "C" fn mm_bseq_read(
    mut fp: *mut mm_bseq_file_t,
    mut chunk_size: int64_t,
    mut with_qual: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut mm_bseq1_t {
    let mut tmp: *mut mm_bseq1_t = 0 as *mut mm_bseq1_t;
    tmp = mm_bseq_read2(fp, chunk_size, with_qual, 0 as libc::c_int, n_);
    return tmp;
}
pub unsafe extern "C" fn mm_bseq_read_frag2(
    mut n_fp: libc::c_int,
    mut fp: *mut *mut mm_bseq_file_t,
    mut chunk_size: int64_t,
    mut with_qual: libc::c_int,
    mut with_comment: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut mm_bseq1_t {
    let mut i: libc::c_int = 0;
    let mut size: int64_t = 0;
    let mut a: __anonstruct_a_687107233 = __anonstruct_a_687107233 {
        n: 0,
        m: 0,
        a: 0 as *mut mm_bseq1_t,
    };
    let mut n_read: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut s: *mut mm_bseq1_t = 0 as *mut mm_bseq1_t;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: size_t = 0;
    size = 0 as libc::c_int as int64_t;
    a.n = 0 as libc::c_int as size_t;
    a.m = 0 as libc::c_int as size_t;
    a.a = 0 as *mut mm_bseq1_t;
    *n_ = 0 as libc::c_int;
    if n_fp < 1 as libc::c_int {
        return 0 as *mut mm_bseq1_t;
    }
    loop {
        n_read = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < n_fp {
            tmp = kseq_read((**fp.offset(i as isize)).ks);
            if tmp >= 0 as libc::c_int {
                n_read += 1;
            }
            i += 1;
        }
        if n_read < n_fp {
            if n_read > 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"[W::%s]\x1B[1;31m query files have different number of records; extra records skipped.\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                    b"mm_bseq_read_frag2\0" as *const u8 as *const libc::c_char,
                );
            }
            break;
        } else {
            if a.m == 0 as libc::c_ulong {
                if a.m < 256 as libc::c_ulong {
                    a.m = 256 as libc::c_int as size_t;
                    a.m = (a.m).wrapping_sub(1);
                    a.m |= a.m >> 1 as libc::c_int;
                    a.m |= a.m >> 2 as libc::c_int;
                    a.m |= a.m >> 4 as libc::c_int;
                    a.m |= a.m >> 8 as libc::c_int;
                    a.m |= a.m >> 16 as libc::c_int;
                    a.m = (a.m).wrapping_add(1);
                    tmp___0 = krealloc(
                        0 as *mut libc::c_void,
                        a.a as *mut libc::c_void,
                        (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong)
                            .wrapping_mul(a.m),
                    );
                    a.a = tmp___0 as *mut mm_bseq1_t;
                }
            }
            i = 0 as libc::c_int;
            while i < n_fp {
                if a.n == a.m {
                    if a.m != 0 {
                        a.m <<= 1 as libc::c_int;
                    } else {
                        a.m = 2 as libc::c_int as size_t;
                    }
                    tmp___1 = krealloc(
                        0 as *mut libc::c_void,
                        a.a as *mut libc::c_void,
                        (::std::mem::size_of::<mm_bseq1_t>() as libc::c_ulong)
                            .wrapping_mul(a.m),
                    );
                    a.a = tmp___1 as *mut mm_bseq1_t;
                }
                tmp___2 = a.n;
                a.n = (a.n).wrapping_add(1);
                s = (a.a).offset(tmp___2 as isize);
                kseq2bseq((**fp.offset(i as isize)).ks, s, with_qual, with_comment);
                size += (*s).l_seq as int64_t;
                i += 1;
            }
            if size >= chunk_size {
                break;
            }
        }
    }
    *n_ = a.n as libc::c_int;
    return a.a;
}
pub unsafe extern "C" fn mm_bseq_read_frag(
    mut n_fp: libc::c_int,
    mut fp: *mut *mut mm_bseq_file_t,
    mut chunk_size: int64_t,
    mut with_qual: libc::c_int,
    mut n_: *mut libc::c_int,
) -> *mut mm_bseq1_t {
    let mut tmp: *mut mm_bseq1_t = 0 as *mut mm_bseq1_t;
    tmp = mm_bseq_read_frag2(n_fp, fp, chunk_size, with_qual, 0 as libc::c_int, n_);
    return tmp;
}
pub unsafe extern "C" fn mm_bseq_eof(mut fp: *mut mm_bseq_file_t) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if (*(*(*fp).ks).f).is_eof() != 0 {
        if (*(*(*fp).ks).f).begin >= (*(*(*fp).ks).f).end {
            if (*fp).s.seq as libc::c_ulong == 0 as *mut libc::c_char as libc::c_ulong {
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
    return tmp;
}
#[inline]
unsafe extern "C" fn get_for_qpos(mut qlen: int32_t, mut a: *const mm128_t) -> int32_t {
    let mut x: int32_t = 0;
    let mut q_span: int32_t = 0;
    x = (*a).y as int32_t;
    q_span = ((*a).y >> 32 as libc::c_int & 255 as libc::c_ulong) as int32_t;
    if (*a).x >> 63 as libc::c_int != 0 {
        x = qlen - 1 as libc::c_int - (x + 1 as libc::c_int - q_span);
    }
    return x;
}
unsafe extern "C" fn get_mini_idx(
    mut qlen: libc::c_int,
    mut a: *const mm128_t,
    mut n: int32_t,
    mut mini_pos: *const uint64_t,
) -> libc::c_int {
    let mut x: int32_t = 0;
    let mut L: int32_t = 0;
    let mut R: int32_t = 0;
    let mut m: int32_t = 0;
    let mut y: int32_t = 0;
    L = 0 as libc::c_int;
    R = n - 1 as libc::c_int;
    x = get_for_qpos(qlen, a);
    while L <= R {
        m = ((L as uint64_t).wrapping_add(R as uint64_t) >> 1 as libc::c_int) as int32_t;
        y = *mini_pos.offset(m as isize) as int32_t;
        if y < x {
            L = m + 1 as libc::c_int;
        } else if y > x {
            R = m - 1 as libc::c_int;
        } else {
            return m
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn mm_est_err(
    mut mi: *const mm_idx_t,
    mut qlen: libc::c_int,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut a: *const mm128_t,
    mut n: int32_t,
    mut mini_pos: *const uint64_t,
) {
    let mut i: libc::c_int = 0;
    let mut sum_k: uint64_t = 0;
    let mut avg_k: libc::c_float = 0.;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut st: int32_t = 0;
    let mut en: int32_t = 0;
    let mut j: int32_t = 0;
    let mut k: int32_t = 0;
    let mut n_match: int32_t = 0;
    let mut n_tot: int32_t = 0;
    let mut l_ref: int32_t = 0;
    let mut tmp: *const mm128_t = 0 as *const mm128_t;
    let mut x: int32_t = 0;
    let mut tmp___0: *const mm128_t = 0 as *const mm128_t;
    let mut tmp___1: libc::c_double = 0.;
    sum_k = 0 as libc::c_int as uint64_t;
    if n == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n {
        sum_k = (sum_k as libc::c_ulong)
            .wrapping_add(
                *mini_pos.offset(i as isize) >> 32 as libc::c_int & 255 as libc::c_ulong,
            ) as uint64_t as uint64_t;
        i += 1;
    }
    avg_k = sum_k as libc::c_float / n as libc::c_float;
    i = 0 as libc::c_int;
    while i < n_regs {
        r = regs.offset(i as isize);
        (*r).div = -1.0f32;
        if !((*r).cnt == 0 as libc::c_int) {
            if (*r).rev() != 0 {
                tmp = a.offset(((*r).as_0 + (*r).cnt - 1 as libc::c_int) as isize);
            } else {
                tmp = a.offset((*r).as_0 as isize);
            }
            en = get_mini_idx(qlen, tmp, n, mini_pos);
            st = en;
            if st < 0 as libc::c_int {
                if mm_verbose >= 2 as libc::c_int {
                    fprintf(
                        stderr,
                        b"[WARNING] logic inconsistency in mm_est_err(). Please contact the developer.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                l_ref = (*((*mi).seq).offset((*r).rid as isize)).len as int32_t;
                k = 1 as libc::c_int;
                j = st + 1 as libc::c_int;
                n_match = 1 as libc::c_int;
                while j < n {
                    if !(k < (*r).cnt) {
                        break;
                    }
                    if (*r).rev() != 0 {
                        tmp___0 = a
                            .offset(
                                ((*r).as_0 + (*r).cnt - 1 as libc::c_int - k) as isize,
                            );
                    } else {
                        tmp___0 = a.offset(((*r).as_0 + k) as isize);
                    }
                    x = get_for_qpos(qlen, tmp___0);
                    if x == *mini_pos.offset(j as isize) as int32_t {
                        k += 1;
                        en = j;
                        n_match += 1;
                    }
                    j += 1;
                }
                n_tot = en - st + 1 as libc::c_int;
                if (*r).qs as libc::c_float > avg_k {
                    if (*r).rs as libc::c_float > avg_k {
                        n_tot += 1;
                    }
                }
                if (qlen - (*r).qs) as libc::c_float > avg_k {
                    if (l_ref - (*r).re) as libc::c_float > avg_k {
                        n_tot += 1;
                    }
                }
                if n_match >= n_tot {
                    (*r).div = 0.0f32;
                } else {
                    tmp___1 = pow(
                        n_match as libc::c_double / n_tot as libc::c_double,
                        1.0f64 / avg_k as libc::c_double,
                    );
                    (*r).div = (1.0f64 - tmp___1) as libc::c_float;
                }
            }
        }
        i += 1;
    }
}
static mut mm_rg_id: [libc::c_char; 256] = [0; 256];
#[inline]
unsafe extern "C" fn str_enlarge(mut s: *mut kstring_t, mut l: libc::c_int) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if ((*s).l).wrapping_add(l as size_t).wrapping_add(1 as libc::c_ulong) > (*s).m {
        (*s).m = ((*s).l).wrapping_add(l as size_t).wrapping_add(1 as libc::c_ulong);
        (*s).m = ((*s).m).wrapping_sub(1);
        (*s).m |= (*s).m >> 1 as libc::c_int;
        (*s).m |= (*s).m >> 2 as libc::c_int;
        (*s).m |= (*s).m >> 4 as libc::c_int;
        (*s).m |= (*s).m >> 8 as libc::c_int;
        (*s).m |= (*s).m >> 16 as libc::c_int;
        (*s).m = ((*s).m).wrapping_add(1);
        tmp = realloc((*s).s as *mut libc::c_void, (*s).m);
        (*s).s = tmp as *mut libc::c_char;
    }
}
#[inline]
unsafe extern "C" fn str_copy(
    mut s: *mut kstring_t,
    mut st: *const libc::c_char,
    mut en: *const libc::c_char,
) {
    str_enlarge(s, en.offset_from(st) as libc::c_long as libc::c_int);
    memcpy(
        ((*s).s).offset((*s).l as isize) as *mut libc::c_void,
        st as *const libc::c_void,
        en.offset_from(st) as libc::c_long as size_t,
    );
    (*s)
        .l = ((*s).l as libc::c_ulong)
        .wrapping_add(en.offset_from(st) as libc::c_long as size_t) as size_t as size_t;
}
unsafe extern "C" fn mm_sprintf_lite(
    mut s: *mut kstring_t,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut buf: [libc::c_char; 16] = [0; 16];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut ap: ::std::ffi::VaListImpl;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut x: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    let mut i___0: libc::c_int = 0;
    let mut l___0: libc::c_int = 0;
    let mut x___0: uint32_t = 0;
    let mut tmp___3: uint32_t = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: size_t = 0;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___7: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___8: size_t = 0;
    let mut tmp___9: size_t = 0;
    let mut tmp___10: libc::c_char = 0;
    ap = args.clone();
    p = fmt;
    q = p;
    while *p != 0 {
        if *p as libc::c_int == 37 as libc::c_int {
            if p as libc::c_ulong > q as libc::c_ulong {
                str_copy(s, q, p);
            }
            p = p.offset(1);
            if *p as libc::c_int == 100 as libc::c_int {
                l = 0 as libc::c_int;
                tmp = ap.arg::<libc::c_int>();
                c = tmp;
                if c >= 0 as libc::c_int {
                    x = c as libc::c_uint;
                } else {
                    x = -c as libc::c_uint;
                }
                loop {
                    tmp___0 = l;
                    l += 1;
                    buf[tmp___0
                        as usize] = x
                        .wrapping_rem(10 as libc::c_uint)
                        .wrapping_add(48 as libc::c_uint) as libc::c_char;
                    x = x.wrapping_div(10 as libc::c_uint);
                    if !(x > 0 as libc::c_uint) {
                        break;
                    }
                }
                if c < 0 as libc::c_int {
                    tmp___1 = l;
                    l += 1;
                    buf[tmp___1 as usize] = '-' as i32 as libc::c_char;
                }
                str_enlarge(s, l);
                i = l - 1 as libc::c_int;
                while i >= 0 as libc::c_int {
                    tmp___2 = (*s).l;
                    (*s).l = ((*s).l).wrapping_add(1);
                    *((*s).s).offset(tmp___2 as isize) = buf[i as usize];
                    i -= 1;
                }
            } else if *p as libc::c_int == 117 as libc::c_int {
                l___0 = 0 as libc::c_int;
                tmp___3 = ap.arg::<uint32_t>();
                x___0 = tmp___3;
                loop {
                    tmp___4 = l___0;
                    l___0 += 1;
                    buf[tmp___4
                        as usize] = x___0
                        .wrapping_rem(10 as libc::c_uint)
                        .wrapping_add(48 as libc::c_uint) as libc::c_char;
                    x___0 = (x___0 as libc::c_uint).wrapping_div(10 as libc::c_uint)
                        as uint32_t as uint32_t;
                    if !(x___0 > 0 as libc::c_uint) {
                        break;
                    }
                }
                str_enlarge(s, l___0);
                i___0 = l___0 - 1 as libc::c_int;
                while i___0 >= 0 as libc::c_int {
                    tmp___5 = (*s).l;
                    (*s).l = ((*s).l).wrapping_add(1);
                    *((*s).s).offset(tmp___5 as isize) = buf[i___0 as usize];
                    i___0 -= 1;
                }
            } else if *p as libc::c_int == 115 as libc::c_int {
                tmp___7 = ap.arg::<*mut libc::c_char>();
                r = tmp___7;
                tmp___8 = strlen(r as *const libc::c_char);
                str_copy(
                    s,
                    r as *const libc::c_char,
                    r.offset(tmp___8 as isize) as *const libc::c_char,
                );
            } else if *p as libc::c_int == 99 as libc::c_int {
                str_enlarge(s, 1 as libc::c_int);
                tmp___9 = (*s).l;
                (*s).l = ((*s).l).wrapping_add(1);
                tmp___10 = ap.arg::<libc::c_int>() as libc::c_char;
                *((*s).s).offset(tmp___9 as isize) = tmp___10;
            } else {
                abort();
            }
            q = p.offset(1 as libc::c_int as isize);
        }
        p = p.offset(1);
    }
    if p as libc::c_ulong > q as libc::c_ulong {
        str_copy(s, q, p);
    }
    *((*s).s).offset((*s).l as isize) = 0 as libc::c_int as libc::c_char;
}
unsafe extern "C" fn mm_escape(mut s: *mut libc::c_char) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    q = s;
    p = q;
    while *p != 0 {
        if *p as libc::c_int == 92 as libc::c_int {
            p = p.offset(1);
            if *p as libc::c_int == 116 as libc::c_int {
                tmp = q;
                q = q.offset(1);
                *tmp = '\t' as i32 as libc::c_char;
            } else if *p as libc::c_int == 92 as libc::c_int {
                tmp___0 = q;
                q = q.offset(1);
                *tmp___0 = '\\' as i32 as libc::c_char;
            }
        } else {
            tmp___1 = q;
            q = q.offset(1);
            *tmp___1 = *p;
        }
        p = p.offset(1);
    }
    *q = '\u{0}' as i32 as libc::c_char;
    return s;
}
unsafe extern "C" fn sam_write_rg_line(
    mut str: *mut kstring_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut r: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rg_line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    rg_line = 0 as *mut libc::c_char;
    memset(
        mm_rg_id.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        256 as libc::c_int as size_t,
    );
    if s as libc::c_ulong == 0 as *const libc::c_char as libc::c_ulong {
        return 0 as libc::c_int;
    }
    tmp = strstr(s, b"@RG\0" as *const u8 as *const libc::c_char);
    if tmp as libc::c_ulong != s as libc::c_ulong {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR] the read group line is not started with @RG\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        tmp___0 = strstr(s, b"\t\0" as *const u8 as *const libc::c_char);
        if tmp___0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            if mm_verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR] the read group line contained literal <tab> characters -- replace with escaped tabs: \\t\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        } else {
            tmp___1 = strlen(s);
            tmp___2 = malloc(tmp___1.wrapping_add(1 as libc::c_ulong));
            rg_line = tmp___2 as *mut libc::c_char;
            strcpy(rg_line, s);
            mm_escape(rg_line);
            p = strstr(
                rg_line as *const libc::c_char,
                b"\tID:\0" as *const u8 as *const libc::c_char,
            );
            if p as libc::c_ulong == 0 as *mut libc::c_char as libc::c_ulong {
                if mm_verbose >= 1 as libc::c_int {
                    fprintf(
                        stderr,
                        b"[ERROR] no ID within the read group line\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            } else {
                p = p.offset(4 as libc::c_int as isize);
                q = p;
                while *q != 0 {
                    if !(*q as libc::c_int != 9 as libc::c_int) {
                        break;
                    }
                    if !(*q as libc::c_int != 10 as libc::c_int) {
                        break;
                    }
                    q = q.offset(1);
                }
                if q.offset_from(p) as libc::c_long + 1 as libc::c_long
                    > 256 as libc::c_long
                {
                    if mm_verbose >= 1 as libc::c_int {
                        fprintf(
                            stderr,
                            b"[ERROR] @RG:ID is longer than 255 characters\n\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    q = p;
                    r = mm_rg_id.as_mut_ptr();
                    while *q != 0 {
                        if !(*q as libc::c_int != 9 as libc::c_int) {
                            break;
                        }
                        if !(*q as libc::c_int != 10 as libc::c_int) {
                            break;
                        }
                        tmp___3 = r;
                        r = r.offset(1);
                        *tmp___3 = *q;
                        q = q.offset(1);
                    }
                    mm_sprintf_lite(
                        str,
                        b"%s\n\0" as *const u8 as *const libc::c_char,
                        rg_line,
                    );
                    return 0 as libc::c_int;
                }
            }
        }
    }
    free(rg_line as *mut libc::c_void);
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn mm_write_sam_hdr(
    mut idx: *const mm_idx_t,
    mut rg: *const libc::c_char,
    mut ver: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut str: kstring_t = kstring_t {
        l: 0,
        m: 0,
        s: 0 as *mut libc::c_char,
    };
    let mut ret: libc::c_int = 0;
    let mut i: uint32_t = 0;
    let mut i___0: libc::c_int = 0;
    str.l = 0 as libc::c_int as size_t;
    str.m = 0 as libc::c_int as size_t;
    str.s = 0 as *mut libc::c_char;
    ret = 0 as libc::c_int;
    if !idx.is_null() {
        i = 0 as libc::c_int as uint32_t;
        while i < (*idx).n_seq {
            mm_sprintf_lite(
                &mut str as *mut kstring_t,
                b"@SQ\tSN:%s\tLN:%d\n\0" as *const u8 as *const libc::c_char,
                (*((*idx).seq).offset(i as isize)).name,
                (*((*idx).seq).offset(i as isize)).len,
            );
            i = i.wrapping_add(1);
        }
    }
    if !rg.is_null() {
        ret = sam_write_rg_line(&mut str, rg);
    }
    mm_sprintf_lite(
        &mut str as *mut kstring_t,
        b"@PG\tID:minimap2\tPN:minimap2\0" as *const u8 as *const libc::c_char,
    );
    if !ver.is_null() {
        mm_sprintf_lite(
            &mut str as *mut kstring_t,
            b"\tVN:%s\0" as *const u8 as *const libc::c_char,
            ver,
        );
    }
    if argc > 1 as libc::c_int {
        mm_sprintf_lite(
            &mut str as *mut kstring_t,
            b"\tCL:minimap2\0" as *const u8 as *const libc::c_char,
        );
        i___0 = 1 as libc::c_int;
        while i___0 < argc {
            mm_sprintf_lite(
                &mut str as *mut kstring_t,
                b" %s\0" as *const u8 as *const libc::c_char,
                *argv.offset(i___0 as isize),
            );
            i___0 += 1;
        }
    }
    mm_err_puts(str.s as *const libc::c_char);
    free(str.s as *mut libc::c_void);
    return ret;
}
unsafe extern "C" fn write_cs_core(
    mut s: *mut kstring_t,
    mut tseq: *const uint8_t,
    mut qseq: *const uint8_t,
    mut r: *const mm_reg1_t,
    mut tmp: *mut libc::c_char,
    mut no_iden: libc::c_int,
    mut write_tag: libc::c_int,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut q_off: libc::c_int = 0;
    let mut t_off: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut op: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut l_tmp: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if write_tag != 0 {
        mm_sprintf_lite(s, b"\tcs:Z:\0" as *const u8 as *const libc::c_char);
    }
    t_off = 0 as libc::c_int;
    q_off = t_off;
    i = q_off;
    while i < (*(*r).p).n_cigar as libc::c_int {
        op = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize) & 15 as libc::c_uint)
            as libc::c_int;
        len = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize) >> 4 as libc::c_int)
            as libc::c_int;
        let mut current_block_12: u64;
        if op >= 0 as libc::c_int {
            if !(op <= 3 as libc::c_int) {
                current_block_12 = 11641096599220229272;
            } else {
                current_block_12 = 12039483399334584727;
            }
        } else {
            current_block_12 = 11641096599220229272;
        }
        match current_block_12 {
            11641096599220229272 => {
                if !(op == 7 as libc::c_int) {
                    if !(op == 8 as libc::c_int) {
                        __assert_fail(
                            b"(op >= MM_CIGAR_MATCH && op <= MM_CIGAR_N_SKIP) || op == MM_CIGAR_EQ_MATCH || op == MM_CIGAR_X_MISMATCH\0"
                                as *const u8 as *const libc::c_char,
                            b"format.c\0" as *const u8 as *const libc::c_char,
                            147 as libc::c_uint,
                            b"write_cs_core\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
            _ => {}
        }
        if op == 0 as libc::c_int {
            current_block = 9505159805436168128;
        } else if op == 7 as libc::c_int {
            current_block = 9505159805436168128;
        } else if op == 8 as libc::c_int {
            current_block = 9505159805436168128;
        } else {
            if op == 1 as libc::c_int {
                j = 0 as libc::c_int;
                *tmp.offset(len as isize) = 0 as libc::c_int as libc::c_char;
                while j < len {
                    *tmp
                        .offset(
                            j as isize,
                        ) = *(b"acgtn\0" as *const u8 as *const libc::c_char)
                        .offset(
                            *qseq.offset((q_off + j) as isize) as libc::c_int as isize,
                        );
                    j += 1;
                }
                mm_sprintf_lite(s, b"+%s\0" as *const u8 as *const libc::c_char, tmp);
                q_off += len;
            } else if op == 2 as libc::c_int {
                j = 0 as libc::c_int;
                *tmp.offset(len as isize) = 0 as libc::c_int as libc::c_char;
                while j < len {
                    *tmp
                        .offset(
                            j as isize,
                        ) = *(b"acgtn\0" as *const u8 as *const libc::c_char)
                        .offset(
                            *tseq.offset((t_off + j) as isize) as libc::c_int as isize,
                        );
                    j += 1;
                }
                mm_sprintf_lite(s, b"-%s\0" as *const u8 as *const libc::c_char, tmp);
                t_off += len;
            } else {
                if !(len >= 2 as libc::c_int) {
                    __assert_fail(
                        b"len >= 2\0" as *const u8 as *const libc::c_char,
                        b"format.c\0" as *const u8 as *const libc::c_char,
                        180 as libc::c_uint,
                        b"write_cs_core\0" as *const u8 as *const libc::c_char,
                    );
                }
                mm_sprintf_lite(
                    s,
                    b"~%c%c%d%c%c\0" as *const u8 as *const libc::c_char,
                    *(b"acgtn\0" as *const u8 as *const libc::c_char)
                        .offset(*tseq.offset(t_off as isize) as libc::c_int as isize)
                        as libc::c_int,
                    *(b"acgtn\0" as *const u8 as *const libc::c_char)
                        .offset(
                            *tseq.offset((t_off + 1 as libc::c_int) as isize)
                                as libc::c_int as isize,
                        ) as libc::c_int,
                    len,
                    *(b"acgtn\0" as *const u8 as *const libc::c_char)
                        .offset(
                            *tseq.offset((t_off + len - 2 as libc::c_int) as isize)
                                as libc::c_int as isize,
                        ) as libc::c_int,
                    *(b"acgtn\0" as *const u8 as *const libc::c_char)
                        .offset(
                            *tseq.offset((t_off + len - 1 as libc::c_int) as isize)
                                as libc::c_int as isize,
                        ) as libc::c_int,
                );
                t_off += len;
            }
            current_block = 1134115459065347084;
        }
        match current_block {
            9505159805436168128 => {
                l_tmp = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < len {
                    if *qseq.offset((q_off + j) as isize) as libc::c_int
                        != *tseq.offset((t_off + j) as isize) as libc::c_int
                    {
                        if l_tmp > 0 as libc::c_int {
                            if no_iden == 0 {
                                *tmp
                                    .offset(l_tmp as isize) = 0 as libc::c_int as libc::c_char;
                                mm_sprintf_lite(
                                    s,
                                    b"=%s\0" as *const u8 as *const libc::c_char,
                                    tmp,
                                );
                            } else {
                                mm_sprintf_lite(
                                    s,
                                    b":%d\0" as *const u8 as *const libc::c_char,
                                    l_tmp,
                                );
                            }
                            l_tmp = 0 as libc::c_int;
                        }
                        mm_sprintf_lite(
                            s,
                            b"*%c%c\0" as *const u8 as *const libc::c_char,
                            *(b"acgtn\0" as *const u8 as *const libc::c_char)
                                .offset(
                                    *tseq.offset((t_off + j) as isize) as libc::c_int as isize,
                                ) as libc::c_int,
                            *(b"acgtn\0" as *const u8 as *const libc::c_char)
                                .offset(
                                    *qseq.offset((q_off + j) as isize) as libc::c_int as isize,
                                ) as libc::c_int,
                        );
                    } else {
                        tmp___1 = l_tmp;
                        l_tmp += 1;
                        *tmp
                            .offset(
                                tmp___1 as isize,
                            ) = *(b"ACGTN\0" as *const u8 as *const libc::c_char)
                            .offset(
                                *qseq.offset((q_off + j) as isize) as libc::c_int as isize,
                            );
                    }
                    j += 1;
                }
                if l_tmp > 0 as libc::c_int {
                    if no_iden == 0 {
                        *tmp.offset(l_tmp as isize) = 0 as libc::c_int as libc::c_char;
                        mm_sprintf_lite(
                            s,
                            b"=%s\0" as *const u8 as *const libc::c_char,
                            tmp,
                        );
                    } else {
                        mm_sprintf_lite(
                            s,
                            b":%d\0" as *const u8 as *const libc::c_char,
                            l_tmp,
                        );
                    }
                }
                q_off += len;
                t_off += len;
            }
            _ => {}
        }
        i += 1;
    }
    if t_off == (*r).re - (*r).rs {
        if !(q_off == (*r).qe - (*r).qs) {
            __assert_fail(
                b"t_off == r->re - r->rs && q_off == r->qe - r->qs\0" as *const u8
                    as *const libc::c_char,
                b"format.c\0" as *const u8 as *const libc::c_char,
                186 as libc::c_uint,
                b"write_cs_core\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"t_off == r->re - r->rs && q_off == r->qe - r->qs\0" as *const u8
                as *const libc::c_char,
            b"format.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_uint,
            b"write_cs_core\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn write_MD_core(
    mut s: *mut kstring_t,
    mut tseq: *const uint8_t,
    mut qseq: *const uint8_t,
    mut r: *const mm_reg1_t,
    mut tmp: *mut libc::c_char,
    mut write_tag: libc::c_int,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut q_off: libc::c_int = 0;
    let mut t_off: libc::c_int = 0;
    let mut l_MD: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut op: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    l_MD = 0 as libc::c_int;
    if write_tag != 0 {
        mm_sprintf_lite(s, b"\tMD:Z:\0" as *const u8 as *const libc::c_char);
    }
    t_off = 0 as libc::c_int;
    q_off = t_off;
    i = q_off;
    while i < (*(*r).p).n_cigar as libc::c_int {
        op = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize) & 15 as libc::c_uint)
            as libc::c_int;
        len = (*((*(*r).p).cigar).as_mut_ptr().offset(i as isize) >> 4 as libc::c_int)
            as libc::c_int;
        let mut current_block_13: u64;
        if op >= 0 as libc::c_int {
            if !(op <= 3 as libc::c_int) {
                current_block_13 = 13230376842436086001;
            } else {
                current_block_13 = 7149356873433890176;
            }
        } else {
            current_block_13 = 13230376842436086001;
        }
        match current_block_13 {
            13230376842436086001 => {
                if !(op == 7 as libc::c_int) {
                    if !(op == 8 as libc::c_int) {
                        __assert_fail(
                            b"(op >= MM_CIGAR_MATCH && op <= MM_CIGAR_N_SKIP) || op == MM_CIGAR_EQ_MATCH || op == MM_CIGAR_X_MISMATCH\0"
                                as *const u8 as *const libc::c_char,
                            b"format.c\0" as *const u8 as *const libc::c_char,
                            195 as libc::c_uint,
                            b"write_MD_core\0" as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
            _ => {}
        }
        if op == 0 as libc::c_int {
            current_block = 15525303915226111495;
        } else if op == 7 as libc::c_int {
            current_block = 15525303915226111495;
        } else if op == 8 as libc::c_int {
            current_block = 15525303915226111495;
        } else {
            if op == 1 as libc::c_int {
                q_off += len;
            } else if op == 2 as libc::c_int {
                j = 0 as libc::c_int;
                *tmp.offset(len as isize) = 0 as libc::c_int as libc::c_char;
                while j < len {
                    *tmp
                        .offset(
                            j as isize,
                        ) = *(b"ACGTN\0" as *const u8 as *const libc::c_char)
                        .offset(
                            *tseq.offset((t_off + j) as isize) as libc::c_int as isize,
                        );
                    j += 1;
                }
                mm_sprintf_lite(
                    s,
                    b"%d^%s\0" as *const u8 as *const libc::c_char,
                    l_MD,
                    tmp,
                );
                l_MD = 0 as libc::c_int;
                t_off += len;
            } else if op == 3 as libc::c_int {
                t_off += len;
            }
            current_block = 1356832168064818221;
        }
        match current_block {
            15525303915226111495 => {
                j = 0 as libc::c_int;
                while j < len {
                    if *qseq.offset((q_off + j) as isize) as libc::c_int
                        != *tseq.offset((t_off + j) as isize) as libc::c_int
                    {
                        mm_sprintf_lite(
                            s,
                            b"%d%c\0" as *const u8 as *const libc::c_char,
                            l_MD,
                            *(b"ACGTN\0" as *const u8 as *const libc::c_char)
                                .offset(
                                    *tseq.offset((t_off + j) as isize) as libc::c_int as isize,
                                ) as libc::c_int,
                        );
                        l_MD = 0 as libc::c_int;
                    } else {
                        l_MD += 1;
                    }
                    j += 1;
                }
                q_off += len;
                t_off += len;
            }
            _ => {}
        }
        i += 1;
    }
    if l_MD > 0 as libc::c_int {
        mm_sprintf_lite(s, b"%d\0" as *const u8 as *const libc::c_char, l_MD);
    }
    if t_off == (*r).re - (*r).rs {
        if !(q_off == (*r).qe - (*r).qs) {
            __assert_fail(
                b"t_off == r->re - r->rs && q_off == r->qe - r->qs\0" as *const u8
                    as *const libc::c_char,
                b"format.c\0" as *const u8 as *const libc::c_char,
                217 as libc::c_uint,
                b"write_MD_core\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"t_off == r->re - r->rs && q_off == r->qe - r->qs\0" as *const u8
                as *const libc::c_char,
            b"format.c\0" as *const u8 as *const libc::c_char,
            217 as libc::c_uint,
            b"write_MD_core\0" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn write_cs_or_MD(
    mut km: *mut libc::c_void,
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut r: *const mm_reg1_t,
    mut no_iden: libc::c_int,
    mut is_MD: libc::c_int,
    mut write_tag: libc::c_int,
    mut is_qstrand: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut qseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut tseq: *mut uint8_t = 0 as *mut uint8_t;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: int32_t = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut c: uint8_t = 0;
    if (*r).p as libc::c_ulong == 0 as *mut mm_extra_t as libc::c_ulong {
        return;
    }
    tmp___0 = kmalloc(km, ((*r).qe - (*r).qs) as size_t);
    qseq = tmp___0 as *mut uint8_t;
    tmp___1 = kmalloc(km, ((*r).re - (*r).rs) as size_t);
    tseq = tmp___1 as *mut uint8_t;
    if (*r).re - (*r).rs > (*r).qe - (*r).qs {
        tmp___2 = (*r).re - (*r).rs + 1 as libc::c_int;
    } else {
        tmp___2 = (*r).qe - (*r).qs + 1 as libc::c_int;
    }
    tmp___3 = kmalloc(km, tmp___2 as size_t);
    tmp = tmp___3 as *mut libc::c_char;
    if is_qstrand != 0 {
        mm_idx_getseq2(
            mi,
            (*r).rev() as libc::c_int,
            (*r).rid as uint32_t,
            (*r).rs as uint32_t,
            (*r).re as uint32_t,
            tseq,
        );
        i = (*r).qs;
        while i < (*r).qe {
            *qseq
                .offset(
                    (i - (*r).qs) as isize,
                ) = seq_nt4_table[*((*t).seq).offset(i as isize) as uint8_t as usize];
            i += 1;
        }
    } else {
        mm_idx_getseq(
            mi,
            (*r).rid as uint32_t,
            (*r).rs as uint32_t,
            (*r).re as uint32_t,
            tseq,
        );
        if (*r).rev() == 0 {
            i = (*r).qs;
            while i < (*r).qe {
                *qseq
                    .offset(
                        (i - (*r).qs) as isize,
                    ) = seq_nt4_table[*((*t).seq).offset(i as isize) as uint8_t
                    as usize];
                i += 1;
            }
        } else {
            i = (*r).qs;
            while i < (*r).qe {
                c = seq_nt4_table[*((*t).seq).offset(i as isize) as uint8_t as usize];
                if c as libc::c_int >= 4 as libc::c_int {
                    *qseq
                        .offset(
                            ((*r).qe - i - 1 as libc::c_int) as isize,
                        ) = 4 as libc::c_int as uint8_t;
                } else {
                    *qseq
                        .offset(
                            ((*r).qe - i - 1 as libc::c_int) as isize,
                        ) = (3 as libc::c_int - c as libc::c_int) as uint8_t;
                }
                i += 1;
            }
        }
    }
    if is_MD != 0 {
        write_MD_core(
            s,
            tseq as *const uint8_t,
            qseq as *const uint8_t,
            r,
            tmp,
            write_tag,
        );
    } else {
        write_cs_core(
            s,
            tseq as *const uint8_t,
            qseq as *const uint8_t,
            r,
            tmp,
            no_iden,
            write_tag,
        );
    }
    kfree(km, qseq as *mut libc::c_void);
    kfree(km, tseq as *mut libc::c_void);
    kfree(km, tmp as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_gen_cs_or_MD(
    mut km: *mut libc::c_void,
    mut buf: *mut *mut libc::c_char,
    mut max_len: *mut libc::c_int,
    mut mi: *const mm_idx_t,
    mut r: *const mm_reg1_t,
    mut seq: *const libc::c_char,
    mut is_MD: libc::c_int,
    mut no_iden: libc::c_int,
    mut is_qstrand: libc::c_int,
) -> libc::c_int {
    let mut t: mm_bseq1_t = mm_bseq1_t {
        l_seq: 0,
        rid: 0,
        name: 0 as *mut libc::c_char,
        seq: 0 as *mut libc::c_char,
        qual: 0 as *mut libc::c_char,
        comment: 0 as *mut libc::c_char,
    };
    let mut str: kstring_t = kstring_t {
        l: 0,
        m: 0,
        s: 0 as *mut libc::c_char,
    };
    let mut tmp: size_t = 0;
    str.s = *buf;
    str.l = 0 as libc::c_int as size_t;
    str.m = *max_len as size_t;
    tmp = strlen(seq);
    t.l_seq = tmp as libc::c_int;
    t.seq = seq as *mut libc::c_char;
    write_cs_or_MD(
        km,
        &mut str,
        mi,
        &mut t as *mut mm_bseq1_t as *const mm_bseq1_t,
        r,
        no_iden,
        is_MD,
        0 as libc::c_int,
        is_qstrand,
    );
    *max_len = str.m as libc::c_int;
    *buf = str.s;
    return str.l as libc::c_int;
}
pub unsafe extern "C" fn mm_gen_cs(
    mut km: *mut libc::c_void,
    mut buf: *mut *mut libc::c_char,
    mut max_len: *mut libc::c_int,
    mut mi: *const mm_idx_t,
    mut r: *const mm_reg1_t,
    mut seq: *const libc::c_char,
    mut no_iden: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = mm_gen_cs_or_MD(
        km,
        buf,
        max_len,
        mi,
        r,
        seq,
        0 as libc::c_int,
        no_iden,
        0 as libc::c_int,
    );
    return tmp;
}
pub unsafe extern "C" fn mm_gen_MD(
    mut km: *mut libc::c_void,
    mut buf: *mut *mut libc::c_char,
    mut max_len: *mut libc::c_int,
    mut mi: *const mm_idx_t,
    mut r: *const mm_reg1_t,
    mut seq: *const libc::c_char,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = mm_gen_cs_or_MD(
        km,
        buf,
        max_len,
        mi,
        r,
        seq,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    return tmp;
}
#[inline]
unsafe extern "C" fn write_tags(mut s: *mut kstring_t, mut r: *const mm_reg1_t) {
    let mut type_0: libc::c_int = 0;
    let mut buf: [libc::c_char; 16] = [0; 16];
    let mut div___0: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_double = 0.;
    let mut buf___0: [libc::c_char; 16] = [0; 16];
    if (*r).id == (*r).parent {
        if (*r).inv() != 0 {
            type_0 = 'I' as i32;
        } else {
            type_0 = 'P' as i32;
        }
    } else if (*r).inv() != 0 {
        type_0 = 'i' as i32;
    } else {
        type_0 = 'S' as i32;
    }
    if !((*r).p).is_null() {
        mm_sprintf_lite(
            s,
            b"\tNM:i:%d\tms:i:%d\tAS:i:%d\tnn:i:%d\0" as *const u8
                as *const libc::c_char,
            (((*r).blen - (*r).mlen) as uint32_t).wrapping_add((*(*r).p).n_ambi()),
            (*(*r).p).dp_max,
            (*(*r).p).dp_score,
            (*(*r).p).n_ambi() as libc::c_int,
        );
        if (*(*r).p).trans_strand() == 1 as libc::c_uint {
            mm_sprintf_lite(
                s,
                b"\tts:A:%c\0" as *const u8 as *const libc::c_char,
                *(b"?+-?\0" as *const u8 as *const libc::c_char)
                    .offset((*(*r).p).trans_strand() as libc::c_int as isize)
                    as libc::c_int,
            );
        } else if (*(*r).p).trans_strand() == 2 as libc::c_uint {
            mm_sprintf_lite(
                s,
                b"\tts:A:%c\0" as *const u8 as *const libc::c_char,
                *(b"?+-?\0" as *const u8 as *const libc::c_char)
                    .offset((*(*r).p).trans_strand() as libc::c_int as isize)
                    as libc::c_int,
            );
        }
    }
    mm_sprintf_lite(
        s,
        b"\ttp:A:%c\tcm:i:%d\ts1:i:%d\0" as *const u8 as *const libc::c_char,
        type_0,
        (*r).cnt,
        (*r).score,
    );
    if (*r).parent == (*r).id {
        mm_sprintf_lite(
            s,
            b"\ts2:i:%d\0" as *const u8 as *const libc::c_char,
            (*r).subsc,
        );
    }
    if !((*r).p).is_null() {
        tmp = mm_event_identity(r);
        div___0 = 1.0f64 - tmp;
        if div___0 == 0.0f64 {
            buf[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
            buf[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        } else {
            tmp___0 = mm_event_identity(r);
            snprintf(
                buf.as_mut_ptr(),
                16 as libc::c_int as size_t,
                b"%.4f\0" as *const u8 as *const libc::c_char,
                1.0f64 - tmp___0,
            );
        }
        mm_sprintf_lite(
            s,
            b"\tde:f:%s\0" as *const u8 as *const libc::c_char,
            buf.as_mut_ptr(),
        );
    } else if (*r).div >= 0.0f32 {
        if (*r).div <= 1.0f32 {
            if (*r).div == 0.0f32 {
                buf___0[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
                buf___0[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            } else {
                snprintf(
                    buf___0.as_mut_ptr(),
                    16 as libc::c_int as size_t,
                    b"%.4f\0" as *const u8 as *const libc::c_char,
                    (*r).div as libc::c_double,
                );
            }
            mm_sprintf_lite(
                s,
                b"\tdv:f:%s\0" as *const u8 as *const libc::c_char,
                buf___0.as_mut_ptr(),
            );
        }
    }
    if (*r).split() != 0 {
        mm_sprintf_lite(
            s,
            b"\tzd:i:%d\0" as *const u8 as *const libc::c_char,
            (*r).split() as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn mm_write_paf3(
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut r: *const mm_reg1_t,
    mut km: *mut libc::c_void,
    mut opt_flag: int64_t,
    mut rep_len: libc::c_int,
) {
    let mut k: uint32_t = 0;
    (*s).l = 0 as libc::c_int as size_t;
    if r as libc::c_ulong == 0 as *const mm_reg1_t as libc::c_ulong {
        mm_sprintf_lite(
            s,
            b"%s\t%d\t0\t0\t*\t*\t0\t0\t0\t0\t0\t0\0" as *const u8
                as *const libc::c_char,
            (*t).name,
            (*t).l_seq,
        );
        if rep_len >= 0 as libc::c_int {
            mm_sprintf_lite(
                s,
                b"\trl:i:%d\0" as *const u8 as *const libc::c_char,
                rep_len,
            );
        }
        return;
    }
    mm_sprintf_lite(
        s,
        b"%s\t%d\t%d\t%d\t%c\t\0" as *const u8 as *const libc::c_char,
        (*t).name,
        (*t).l_seq,
        (*r).qs,
        (*r).qe,
        *(b"+-\0" as *const u8 as *const libc::c_char)
            .offset((*r).rev() as libc::c_int as isize) as libc::c_int,
    );
    if !((*((*mi).seq).offset((*r).rid as isize)).name).is_null() {
        mm_sprintf_lite(
            s,
            b"%s\0" as *const u8 as *const libc::c_char,
            (*((*mi).seq).offset((*r).rid as isize)).name,
        );
    } else {
        mm_sprintf_lite(s, b"%d\0" as *const u8 as *const libc::c_char, (*r).rid);
    }
    mm_sprintf_lite(
        s,
        b"\t%d\0" as *const u8 as *const libc::c_char,
        (*((*mi).seq).offset((*r).rid as isize)).len,
    );
    if opt_flag as libc::c_longlong & 4294967296 as libc::c_longlong != 0 {
        if (*r).rev() != 0 {
            mm_sprintf_lite(
                s,
                b"\t%d\t%d\0" as *const u8 as *const libc::c_char,
                ((*((*mi).seq).offset((*r).rid as isize)).len)
                    .wrapping_sub((*r).re as uint32_t),
                ((*((*mi).seq).offset((*r).rid as isize)).len)
                    .wrapping_sub((*r).rs as uint32_t),
            );
        } else {
            mm_sprintf_lite(
                s,
                b"\t%d\t%d\0" as *const u8 as *const libc::c_char,
                (*r).rs,
                (*r).re,
            );
        }
    } else {
        mm_sprintf_lite(
            s,
            b"\t%d\t%d\0" as *const u8 as *const libc::c_char,
            (*r).rs,
            (*r).re,
        );
    }
    mm_sprintf_lite(
        s,
        b"\t%d\t%d\0" as *const u8 as *const libc::c_char,
        (*r).mlen,
        (*r).blen,
    );
    mm_sprintf_lite(
        s,
        b"\t%d\0" as *const u8 as *const libc::c_char,
        (*r).mapq() as libc::c_int,
    );
    write_tags(s, r);
    if rep_len >= 0 as libc::c_int {
        mm_sprintf_lite(s, b"\trl:i:%d\0" as *const u8 as *const libc::c_char, rep_len);
    }
    if !((*r).p).is_null() {
        if opt_flag & 32 as libc::c_long != 0 {
            mm_sprintf_lite(s, b"\tcg:Z:\0" as *const u8 as *const libc::c_char);
            k = 0 as libc::c_int as uint32_t;
            while k < (*(*r).p).n_cigar {
                mm_sprintf_lite(
                    s,
                    b"%d%c\0" as *const u8 as *const libc::c_char,
                    *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                        >> 4 as libc::c_int,
                    *(b"MIDNSHP=XB\0" as *const u8 as *const libc::c_char)
                        .offset(
                            (*((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                                & 15 as libc::c_uint) as isize,
                        ) as libc::c_int,
                );
                k = k.wrapping_add(1);
            }
        }
    }
    if !((*r).p).is_null() {
        if opt_flag & 16777280 as libc::c_long != 0 {
            write_cs_or_MD(
                km,
                s,
                mi,
                t,
                r,
                (opt_flag & 2048 as libc::c_long == 0) as libc::c_int,
                (opt_flag & 16777216 as libc::c_long) as libc::c_int,
                1 as libc::c_int,
                (opt_flag as libc::c_longlong & 4294967296 as libc::c_longlong != 0)
                    as libc::c_int,
            );
        }
    }
    if opt_flag & 33554432 as libc::c_long != 0 {
        if !((*t).comment).is_null() {
            mm_sprintf_lite(
                s,
                b"\t%s\0" as *const u8 as *const libc::c_char,
                (*t).comment,
            );
        }
    }
}
pub unsafe extern "C" fn mm_write_paf(
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut r: *const mm_reg1_t,
    mut km: *mut libc::c_void,
    mut opt_flag: int64_t,
) {
    mm_write_paf3(s, mi, t, r, km, opt_flag, -(1 as libc::c_int));
}
unsafe extern "C" fn sam_write_sq(
    mut s: *mut kstring_t,
    mut seq: *mut libc::c_char,
    mut l: libc::c_int,
    mut rev: libc::c_int,
    mut comp: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    if rev != 0 {
        str_enlarge(s, l);
        i = 0 as libc::c_int;
        while i < l {
            c = *seq.offset((l - 1 as libc::c_int - i) as isize) as libc::c_int;
            if c < 128 as libc::c_int {
                if comp != 0 {
                    *((*s).s)
                        .offset(
                            ((*s).l).wrapping_add(i as size_t) as isize,
                        ) = seq_comp_table[c as usize] as libc::c_char;
                } else {
                    *((*s).s)
                        .offset(
                            ((*s).l).wrapping_add(i as size_t) as isize,
                        ) = c as libc::c_char;
                }
            } else {
                *((*s).s)
                    .offset(
                        ((*s).l).wrapping_add(i as size_t) as isize,
                    ) = c as libc::c_char;
            }
            i += 1;
        }
        (*s).l = ((*s).l as libc::c_ulong).wrapping_add(l as size_t) as size_t as size_t;
    } else {
        str_copy(
            s,
            seq as *const libc::c_char,
            seq.offset(l as isize) as *const libc::c_char,
        );
    };
}
#[inline]
unsafe extern "C" fn get_sam_pri(
    mut n_regs: libc::c_int,
    mut regs: *const mm_reg1_t,
) -> *const mm_reg1_t {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_regs {
        if (*regs.offset(i as isize)).sam_pri() != 0 {
            return regs.offset(i as isize);
        }
        i += 1;
    }
    if !(n_regs == 0 as libc::c_int) {
        __assert_fail(
            b"n_regs == 0\0" as *const u8 as *const libc::c_char,
            b"format.c\0" as *const u8 as *const libc::c_char,
            359 as libc::c_uint,
            b"get_sam_pri\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as *mut libc::c_void as *const mm_reg1_t;
}
unsafe extern "C" fn write_sam_cigar(
    mut s: *mut kstring_t,
    mut sam_flag: libc::c_int,
    mut in_tag: libc::c_int,
    mut qlen: libc::c_int,
    mut r: *const mm_reg1_t,
    mut opt_flag: int64_t,
) {
    let mut k: uint32_t = 0;
    let mut clip_len: [uint32_t; 2] = [0; 2];
    let mut clip_char: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut clip_char___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if (*r).p as libc::c_ulong == 0 as *mut mm_extra_t as libc::c_ulong {
        mm_sprintf_lite(s, b"*\0" as *const u8 as *const libc::c_char);
    } else {
        if (*r).rev() != 0 {
            clip_len[0 as libc::c_int as usize] = (qlen - (*r).qe) as uint32_t;
        } else {
            clip_len[0 as libc::c_int as usize] = (*r).qs as uint32_t;
        }
        if (*r).rev() != 0 {
            clip_len[1 as libc::c_int as usize] = (*r).qs as uint32_t;
        } else {
            clip_len[1 as libc::c_int as usize] = (qlen - (*r).qe) as uint32_t;
        }
        if in_tag != 0 {
            if sam_flag & 2048 as libc::c_int != 0 {
                if opt_flag & 524288 as libc::c_long == 0 {
                    tmp = 5 as libc::c_int;
                } else {
                    tmp = 4 as libc::c_int;
                }
            } else {
                tmp = 4 as libc::c_int;
            }
            clip_char = tmp;
            mm_sprintf_lite(s, b"\tCG:B:I\0" as *const u8 as *const libc::c_char);
            if clip_len[0 as libc::c_int as usize] != 0 {
                mm_sprintf_lite(
                    s,
                    b",%u\0" as *const u8 as *const libc::c_char,
                    clip_len[0 as libc::c_int as usize] << 4 as libc::c_int
                        | clip_char as libc::c_uint,
                );
            }
            k = 0 as libc::c_int as uint32_t;
            while k < (*(*r).p).n_cigar {
                mm_sprintf_lite(
                    s,
                    b",%u\0" as *const u8 as *const libc::c_char,
                    *((*(*r).p).cigar).as_mut_ptr().offset(k as isize),
                );
                k = k.wrapping_add(1);
            }
            if clip_len[1 as libc::c_int as usize] != 0 {
                mm_sprintf_lite(
                    s,
                    b",%u\0" as *const u8 as *const libc::c_char,
                    clip_len[1 as libc::c_int as usize] << 4 as libc::c_int
                        | clip_char as libc::c_uint,
                );
            }
        } else {
            if sam_flag & 2048 as libc::c_int != 0 {
                if opt_flag & 524288 as libc::c_long == 0 {
                    tmp___0 = 'H' as i32;
                } else {
                    tmp___0 = 'S' as i32;
                }
            } else {
                tmp___0 = 'S' as i32;
            }
            clip_char___0 = tmp___0;
            if clip_len[0 as libc::c_int as usize] < qlen as uint32_t {
                if !(clip_len[1 as libc::c_int as usize] < qlen as uint32_t) {
                    __assert_fail(
                        b"clip_len[0] < qlen && clip_len[1] < qlen\0" as *const u8
                            as *const libc::c_char,
                        b"format.c\0" as *const u8 as *const libc::c_char,
                        380 as libc::c_uint,
                        b"write_sam_cigar\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                __assert_fail(
                    b"clip_len[0] < qlen && clip_len[1] < qlen\0" as *const u8
                        as *const libc::c_char,
                    b"format.c\0" as *const u8 as *const libc::c_char,
                    380 as libc::c_uint,
                    b"write_sam_cigar\0" as *const u8 as *const libc::c_char,
                );
            }
            if clip_len[0 as libc::c_int as usize] != 0 {
                mm_sprintf_lite(
                    s,
                    b"%d%c\0" as *const u8 as *const libc::c_char,
                    clip_len[0 as libc::c_int as usize],
                    clip_char___0,
                );
            }
            k = 0 as libc::c_int as uint32_t;
            while k < (*(*r).p).n_cigar {
                mm_sprintf_lite(
                    s,
                    b"%d%c\0" as *const u8 as *const libc::c_char,
                    *((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                        >> 4 as libc::c_int,
                    *(b"MIDNSHP=XB\0" as *const u8 as *const libc::c_char)
                        .offset(
                            (*((*(*r).p).cigar).as_mut_ptr().offset(k as isize)
                                & 15 as libc::c_uint) as isize,
                        ) as libc::c_int,
                );
                k = k.wrapping_add(1);
            }
            if clip_len[1 as libc::c_int as usize] != 0 {
                mm_sprintf_lite(
                    s,
                    b"%d%c\0" as *const u8 as *const libc::c_char,
                    clip_len[1 as libc::c_int as usize],
                    clip_char___0,
                );
            }
        }
    };
}
pub unsafe extern "C" fn mm_write_sam3(
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut seg_idx: libc::c_int,
    mut reg_idx: libc::c_int,
    mut n_seg: libc::c_int,
    mut n_regss: *const libc::c_int,
    mut regss: *const *const mm_reg1_t,
    mut km: *mut libc::c_void,
    mut opt_flag: int64_t,
    mut rep_len: libc::c_int,
) {
    let mut max_bam_cigar_op: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut n_regs: libc::c_int = 0;
    let mut cigar_in_tag: libc::c_int = 0;
    let mut this_rid: libc::c_int = 0;
    let mut this_pos: libc::c_int = 0;
    let mut regs: *const mm_reg1_t = 0 as *const mm_reg1_t;
    let mut r_prev: *const mm_reg1_t = 0 as *const mm_reg1_t;
    let mut r_next: *const mm_reg1_t = 0 as *const mm_reg1_t;
    let mut r: *const mm_reg1_t = 0 as *const mm_reg1_t;
    let mut tmp: *const mm_reg1_t = 0 as *const mm_reg1_t;
    let mut i: libc::c_int = 0;
    let mut next_sid: libc::c_int = 0;
    let mut prev_sid: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut n_cigar: libc::c_int = 0;
    let mut slen: libc::c_int = 0;
    let mut tlen: libc::c_int = 0;
    let mut this_pos5: libc::c_int = 0;
    let mut tmp___1: int32_t = 0;
    let mut next_pos5: libc::c_int = 0;
    let mut tmp___2: int32_t = 0;
    let mut i___0: libc::c_int = 0;
    let mut n_sa: libc::c_int = 0;
    let mut q: *const mm_reg1_t = 0 as *const mm_reg1_t;
    let mut l_M: libc::c_int = 0;
    let mut l_I: libc::c_int = 0;
    let mut l_D: libc::c_int = 0;
    let mut clip5: libc::c_int = 0;
    let mut clip3: libc::c_int = 0;
    max_bam_cigar_op = 65535 as libc::c_int;
    n_regs = *n_regss.offset(seg_idx as isize);
    cigar_in_tag = 0 as libc::c_int;
    this_rid = -(1 as libc::c_int);
    this_pos = -(1 as libc::c_int);
    regs = *regss.offset(seg_idx as isize);
    r_prev = 0 as *mut libc::c_void as *const mm_reg1_t;
    if n_regs > 0 as libc::c_int {
        if reg_idx < n_regs {
            if reg_idx >= 0 as libc::c_int {
                tmp = regs.offset(reg_idx as isize);
            } else {
                tmp = 0 as *mut libc::c_void as *const mm_reg1_t;
            }
        } else {
            tmp = 0 as *mut libc::c_void as *const mm_reg1_t;
        }
    } else {
        tmp = 0 as *mut libc::c_void as *const mm_reg1_t;
    }
    r = tmp;
    if n_seg > 1 as libc::c_int {
        next_sid = (seg_idx + 1 as libc::c_int) % n_seg;
        r_next = get_sam_pri(
            *n_regss.offset(next_sid as isize),
            *regss.offset(next_sid as isize),
        );
        if n_seg > 2 as libc::c_int {
            i = 1 as libc::c_int;
            while i <= n_seg - 1 as libc::c_int {
                prev_sid = (seg_idx + n_seg - i) % n_seg;
                if *n_regss.offset(prev_sid as isize) > 0 as libc::c_int {
                    r_prev = get_sam_pri(
                        *n_regss.offset(prev_sid as isize),
                        *regss.offset(prev_sid as isize),
                    );
                    break;
                } else {
                    i += 1;
                }
            }
        } else {
            r_prev = r_next;
        }
    } else {
        r_next = 0 as *mut libc::c_void as *const mm_reg1_t;
        r_prev = r_next;
    }
    (*s).l = 0 as libc::c_int as size_t;
    mm_sprintf_lite(s, b"%s\0" as *const u8 as *const libc::c_char, (*t).name);
    if n_seg > 1 as libc::c_int {
        tmp___0 = mm_qname_len((*t).name as *const libc::c_char);
        (*s).l = tmp___0 as size_t;
    }
    if n_seg > 1 as libc::c_int {
        flag = 1 as libc::c_int;
    } else {
        flag = 0 as libc::c_int;
    }
    if r as libc::c_ulong == 0 as *const mm_reg1_t as libc::c_ulong {
        flag |= 4 as libc::c_int;
    } else {
        if (*r).rev() != 0 {
            flag |= 16 as libc::c_int;
        }
        if (*r).parent != (*r).id {
            flag |= 256 as libc::c_int;
        } else if (*r).sam_pri() == 0 {
            flag |= 2048 as libc::c_int;
        }
    }
    if n_seg > 1 as libc::c_int {
        if !r.is_null() {
            if (*r).proper_frag() != 0 {
                flag |= 2 as libc::c_int;
            }
        }
        if seg_idx == 0 as libc::c_int {
            flag |= 64 as libc::c_int;
        } else if seg_idx == n_seg - 1 as libc::c_int {
            flag |= 128 as libc::c_int;
        }
        if r_next as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            flag |= 8 as libc::c_int;
        } else if (*r_next).rev() != 0 {
            flag |= 32 as libc::c_int;
        }
    }
    mm_sprintf_lite(s, b"\t%d\0" as *const u8 as *const libc::c_char, flag);
    if r as libc::c_ulong == 0 as *const mm_reg1_t as libc::c_ulong {
        if !r_prev.is_null() {
            this_rid = (*r_prev).rid;
            this_pos = (*r_prev).rs;
            mm_sprintf_lite(
                s,
                b"\t%s\t%d\t0\t*\0" as *const u8 as *const libc::c_char,
                (*((*mi).seq).offset(this_rid as isize)).name,
                this_pos + 1 as libc::c_int,
            );
        } else {
            mm_sprintf_lite(s, b"\t*\t0\t0\t*\0" as *const u8 as *const libc::c_char);
        }
    } else {
        this_rid = (*r).rid;
        this_pos = (*r).rs;
        mm_sprintf_lite(
            s,
            b"\t%s\t%d\t%d\t\0" as *const u8 as *const libc::c_char,
            (*((*mi).seq).offset((*r).rid as isize)).name,
            (*r).rs + 1 as libc::c_int,
            (*r).mapq() as libc::c_int,
        );
        if opt_flag & 65536 as libc::c_long != 0 {
            if !((*r).p).is_null() {
                if (*(*r).p).n_cigar > (max_bam_cigar_op - 2 as libc::c_int) as uint32_t
                {
                    n_cigar = (*(*r).p).n_cigar as libc::c_int;
                    if (*r).qs != 0 as libc::c_int {
                        n_cigar += 1;
                    }
                    if (*r).qe != (*t).l_seq {
                        n_cigar += 1;
                    }
                    if n_cigar > max_bam_cigar_op {
                        cigar_in_tag = 1 as libc::c_int;
                    }
                }
            }
        }
        if cigar_in_tag != 0 {
            if flag & 2304 as libc::c_int == 0 as libc::c_int {
                slen = (*t).l_seq;
            } else if opt_flag & 524288 as libc::c_long != 0 {
                slen = (*t).l_seq;
            } else if flag & 256 as libc::c_int != 0 {
                slen = 0 as libc::c_int;
            } else {
                slen = (*r).qe - (*r).qs;
            }
            mm_sprintf_lite(
                s,
                b"%dS%dN\0" as *const u8 as *const libc::c_char,
                slen,
                (*r).re - (*r).rs,
            );
        } else {
            write_sam_cigar(s, flag, 0 as libc::c_int, (*t).l_seq, r, opt_flag);
        }
    }
    if n_seg > 1 as libc::c_int {
        tlen = 0 as libc::c_int;
        let mut current_block_156: u64;
        if this_rid >= 0 as libc::c_int {
            if !r_next.is_null() {
                if this_rid == (*r_next).rid {
                    if !r.is_null() {
                        if (*r).rev() != 0 {
                            tmp___1 = (*r).re - 1 as libc::c_int;
                        } else {
                            tmp___1 = this_pos;
                        }
                        this_pos5 = tmp___1;
                        if (*r_next).rev() != 0 {
                            tmp___2 = (*r_next).re - 1 as libc::c_int;
                        } else {
                            tmp___2 = (*r_next).rs;
                        }
                        next_pos5 = tmp___2;
                        tlen = next_pos5 - this_pos5;
                    }
                    mm_sprintf_lite(s, b"\t=\t\0" as *const u8 as *const libc::c_char);
                } else {
                    mm_sprintf_lite(
                        s,
                        b"\t%s\t\0" as *const u8 as *const libc::c_char,
                        (*((*mi).seq).offset((*r_next).rid as isize)).name,
                    );
                }
                mm_sprintf_lite(
                    s,
                    b"%d\t\0" as *const u8 as *const libc::c_char,
                    (*r_next).rs + 1 as libc::c_int,
                );
                current_block_156 = 16974974966130203269;
            } else {
                current_block_156 = 15333263076314478521;
            }
        } else {
            current_block_156 = 15333263076314478521;
        }
        match current_block_156 {
            15333263076314478521 => {
                if !r_next.is_null() {
                    mm_sprintf_lite(
                        s,
                        b"\t%s\t%d\t\0" as *const u8 as *const libc::c_char,
                        (*((*mi).seq).offset((*r_next).rid as isize)).name,
                        (*r_next).rs + 1 as libc::c_int,
                    );
                } else if this_rid >= 0 as libc::c_int {
                    mm_sprintf_lite(
                        s,
                        b"\t=\t%d\t\0" as *const u8 as *const libc::c_char,
                        this_pos + 1 as libc::c_int,
                    );
                } else {
                    mm_sprintf_lite(
                        s,
                        b"\t*\t0\t\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            _ => {}
        }
        if tlen > 0 as libc::c_int {
            tlen += 1;
        } else if tlen < 0 as libc::c_int {
            tlen -= 1;
        }
        mm_sprintf_lite(s, b"%d\t\0" as *const u8 as *const libc::c_char, tlen);
    } else {
        mm_sprintf_lite(s, b"\t*\t0\t0\t\0" as *const u8 as *const libc::c_char);
    }
    if r as libc::c_ulong == 0 as *const mm_reg1_t as libc::c_ulong {
        sam_write_sq(s, (*t).seq, (*t).l_seq, 0 as libc::c_int, 0 as libc::c_int);
        mm_sprintf_lite(s, b"\t\0" as *const u8 as *const libc::c_char);
        if !((*t).qual).is_null() {
            sam_write_sq(s, (*t).qual, (*t).l_seq, 0 as libc::c_int, 0 as libc::c_int);
        } else {
            mm_sprintf_lite(s, b"*\0" as *const u8 as *const libc::c_char);
        }
    } else {
        let mut current_block_194: u64;
        if flag & 2304 as libc::c_int == 0 as libc::c_int {
            current_block_194 = 13322020217995181186;
        } else if opt_flag & 524288 as libc::c_long != 0 {
            current_block_194 = 13322020217995181186;
        } else {
            if flag & 256 as libc::c_int != 0 {
                mm_sprintf_lite(s, b"*\t*\0" as *const u8 as *const libc::c_char);
            } else {
                sam_write_sq(
                    s,
                    ((*t).seq).offset((*r).qs as isize),
                    (*r).qe - (*r).qs,
                    (*r).rev() as libc::c_int,
                    (*r).rev() as libc::c_int,
                );
                mm_sprintf_lite(s, b"\t\0" as *const u8 as *const libc::c_char);
                if !((*t).qual).is_null() {
                    sam_write_sq(
                        s,
                        ((*t).qual).offset((*r).qs as isize),
                        (*r).qe - (*r).qs,
                        (*r).rev() as libc::c_int,
                        0 as libc::c_int,
                    );
                } else {
                    mm_sprintf_lite(s, b"*\0" as *const u8 as *const libc::c_char);
                }
            }
            current_block_194 = 242220637564940144;
        }
        match current_block_194 {
            13322020217995181186 => {
                sam_write_sq(
                    s,
                    (*t).seq,
                    (*t).l_seq,
                    (*r).rev() as libc::c_int,
                    (*r).rev() as libc::c_int,
                );
                mm_sprintf_lite(s, b"\t\0" as *const u8 as *const libc::c_char);
                if !((*t).qual).is_null() {
                    sam_write_sq(
                        s,
                        (*t).qual,
                        (*t).l_seq,
                        (*r).rev() as libc::c_int,
                        0 as libc::c_int,
                    );
                } else {
                    mm_sprintf_lite(s, b"*\0" as *const u8 as *const libc::c_char);
                }
            }
            _ => {}
        }
    }
    if mm_rg_id[0 as libc::c_int as usize] != 0 {
        mm_sprintf_lite(
            s,
            b"\tRG:Z:%s\0" as *const u8 as *const libc::c_char,
            mm_rg_id.as_mut_ptr(),
        );
    }
    if n_seg > 2 as libc::c_int {
        mm_sprintf_lite(s, b"\tFI:i:%d\0" as *const u8 as *const libc::c_char, seg_idx);
    }
    if !r.is_null() {
        write_tags(s, r);
        if (*r).parent == (*r).id {
            if !((*r).p).is_null() {
                if n_regs > 1 as libc::c_int {
                    if !regs.is_null() {
                        if r as libc::c_ulong >= regs as libc::c_ulong {
                            if (r.offset_from(regs) as libc::c_long)
                                < n_regs as libc::c_long
                            {
                                n_sa = 0 as libc::c_int;
                                i___0 = 0 as libc::c_int;
                                while i___0 < n_regs {
                                    if i___0 as libc::c_long
                                        != r.offset_from(regs) as libc::c_long
                                    {
                                        if (*regs.offset(i___0 as isize)).parent
                                            == (*regs.offset(i___0 as isize)).id
                                        {
                                            if !((*regs.offset(i___0 as isize)).p).is_null() {
                                                n_sa += 1;
                                            }
                                        }
                                    }
                                    i___0 += 1;
                                }
                                if n_sa > 0 as libc::c_int {
                                    mm_sprintf_lite(
                                        s,
                                        b"\tSA:Z:\0" as *const u8 as *const libc::c_char,
                                    );
                                    i___0 = 0 as libc::c_int;
                                    while i___0 < n_regs {
                                        q = regs.offset(i___0 as isize);
                                        l_I = 0 as libc::c_int;
                                        l_D = 0 as libc::c_int;
                                        clip5 = 0 as libc::c_int;
                                        clip3 = 0 as libc::c_int;
                                        if !(r as libc::c_ulong == q as libc::c_ulong) {
                                            if !((*q).parent != (*q).id) {
                                                if !((*q).p as libc::c_ulong
                                                    == 0 as *mut mm_extra_t as libc::c_ulong)
                                                {
                                                    if (*q).qe - (*q).qs < (*q).re - (*q).rs {
                                                        l_M = (*q).qe - (*q).qs;
                                                        l_D = (*q).re - (*q).rs - l_M;
                                                    } else {
                                                        l_M = (*q).re - (*q).rs;
                                                        l_I = (*q).qe - (*q).qs - l_M;
                                                    }
                                                    if (*q).rev() != 0 {
                                                        clip5 = (*t).l_seq - (*q).qe;
                                                    } else {
                                                        clip5 = (*q).qs;
                                                    }
                                                    if (*q).rev() != 0 {
                                                        clip3 = (*q).qs;
                                                    } else {
                                                        clip3 = (*t).l_seq - (*q).qe;
                                                    }
                                                    mm_sprintf_lite(
                                                        s,
                                                        b"%s,%d,%c,\0" as *const u8 as *const libc::c_char,
                                                        (*((*mi).seq).offset((*q).rid as isize)).name,
                                                        (*q).rs + 1 as libc::c_int,
                                                        *(b"+-\0" as *const u8 as *const libc::c_char)
                                                            .offset((*q).rev() as libc::c_int as isize) as libc::c_int,
                                                    );
                                                    if clip5 != 0 {
                                                        mm_sprintf_lite(
                                                            s,
                                                            b"%dS\0" as *const u8 as *const libc::c_char,
                                                            clip5,
                                                        );
                                                    }
                                                    if l_M != 0 {
                                                        mm_sprintf_lite(
                                                            s,
                                                            b"%dM\0" as *const u8 as *const libc::c_char,
                                                            l_M,
                                                        );
                                                    }
                                                    if l_I != 0 {
                                                        mm_sprintf_lite(
                                                            s,
                                                            b"%dI\0" as *const u8 as *const libc::c_char,
                                                            l_I,
                                                        );
                                                    }
                                                    if l_D != 0 {
                                                        mm_sprintf_lite(
                                                            s,
                                                            b"%dD\0" as *const u8 as *const libc::c_char,
                                                            l_D,
                                                        );
                                                    }
                                                    if clip3 != 0 {
                                                        mm_sprintf_lite(
                                                            s,
                                                            b"%dS\0" as *const u8 as *const libc::c_char,
                                                            clip3,
                                                        );
                                                    }
                                                    mm_sprintf_lite(
                                                        s,
                                                        b",%d,%d;\0" as *const u8 as *const libc::c_char,
                                                        (*q).mapq() as libc::c_int,
                                                        (((*q).blen - (*q).mlen) as uint32_t)
                                                            .wrapping_add((*(*q).p).n_ambi()),
                                                    );
                                                }
                                            }
                                        }
                                        i___0 += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if !((*r).p).is_null() {
            if opt_flag & 16777280 as libc::c_long != 0 {
                write_cs_or_MD(
                    km,
                    s,
                    mi,
                    t,
                    r,
                    (opt_flag & 2048 as libc::c_long == 0) as libc::c_int,
                    (opt_flag & 16777216 as libc::c_long) as libc::c_int,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
        }
        if cigar_in_tag != 0 {
            write_sam_cigar(s, flag, 1 as libc::c_int, (*t).l_seq, r, opt_flag);
        }
    }
    if rep_len >= 0 as libc::c_int {
        mm_sprintf_lite(s, b"\trl:i:%d\0" as *const u8 as *const libc::c_char, rep_len);
    }
    if opt_flag & 33554432 as libc::c_long != 0 {
        if !((*t).comment).is_null() {
            mm_sprintf_lite(
                s,
                b"\t%s\0" as *const u8 as *const libc::c_char,
                (*t).comment,
            );
        }
    }
    *((*s).s).offset((*s).l as isize) = 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn mm_write_sam2(
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut seg_idx: libc::c_int,
    mut reg_idx: libc::c_int,
    mut n_seg: libc::c_int,
    mut n_regss: *const libc::c_int,
    mut regss: *const *const mm_reg1_t,
    mut km: *mut libc::c_void,
    mut opt_flag: int64_t,
) {
    mm_write_sam3(
        s,
        mi,
        t,
        seg_idx,
        reg_idx,
        n_seg,
        n_regss,
        regss,
        km,
        opt_flag,
        -(1 as libc::c_int),
    );
}
pub unsafe extern "C" fn mm_write_sam(
    mut s: *mut kstring_t,
    mut mi: *const mm_idx_t,
    mut t: *const mm_bseq1_t,
    mut r: *const mm_reg1_t,
    mut n_regs: libc::c_int,
    mut regs: *const mm_reg1_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_regs {
        if r as libc::c_ulong == regs.offset(i as isize) as libc::c_ulong {
            break;
        }
        i += 1;
    }
    mm_write_sam2(
        s,
        mi,
        t,
        0 as libc::c_int,
        i,
        1 as libc::c_int,
        &mut n_regs as *mut libc::c_int as *const libc::c_int,
        &mut regs as *mut *const mm_reg1_t as *const *const mm_reg1_t,
        0 as *mut libc::c_void,
        0 as libc::c_int as int64_t,
    );
}
#[inline]
unsafe extern "C" fn mm_cal_fuzzy_len(mut r: *mut mm_reg1_t, mut a: *const mm128_t) {
    let mut i: libc::c_int = 0;
    let mut tmp: int32_t = 0;
    let mut tmp___0: int32_t = 0;
    let mut span: libc::c_int = 0;
    let mut tl: libc::c_int = 0;
    let mut ql: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    tmp = 0 as libc::c_int;
    (*r).blen = tmp;
    (*r).mlen = tmp;
    if (*r).cnt <= 0 as libc::c_int {
        return;
    }
    tmp___0 = ((*a.offset((*r).as_0 as isize)).y >> 32 as libc::c_int
        & 255 as libc::c_ulong) as int32_t;
    (*r).blen = tmp___0;
    (*r).mlen = tmp___0;
    i = (*r).as_0 + 1 as libc::c_int;
    while i < (*r).as_0 + (*r).cnt {
        span = ((*a.offset(i as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
            as libc::c_int;
        tl = (*a.offset(i as isize)).x as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).x as int32_t;
        ql = (*a.offset(i as isize)).y as int32_t
            - (*a.offset((i - 1 as libc::c_int) as isize)).y as int32_t;
        if tl > ql {
            tmp___1 = tl;
        } else {
            tmp___1 = ql;
        }
        (*r).blen += tmp___1;
        let mut current_block_26: u64;
        if tl > span {
            if ql > span {
                tmp___3 = span;
                current_block_26 = 15125582407903384992;
            } else {
                current_block_26 = 15039561742039809190;
            }
        } else {
            current_block_26 = 15039561742039809190;
        }
        match current_block_26 {
            15039561742039809190 => {
                if tl < ql {
                    tmp___2 = tl;
                } else {
                    tmp___2 = ql;
                }
                tmp___3 = tmp___2;
            }
            _ => {}
        }
        (*r).mlen += tmp___3;
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn mm_reg_set_coor(
    mut r: *mut mm_reg1_t,
    mut qlen: int32_t,
    mut a: *const mm128_t,
    mut is_qstrand: libc::c_int,
) {
    let mut k: int32_t = 0;
    let mut q_span: int32_t = 0;
    k = (*r).as_0;
    q_span = ((*a.offset(k as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
        as int32_t;
    (*r).set_rev(((*a.offset(k as isize)).x >> 63 as libc::c_int) as uint32_t);
    (*r)
        .rid = ((*a.offset(k as isize)).x << 1 as libc::c_int >> 33 as libc::c_int)
        as int32_t;
    if (*a.offset(k as isize)).x as int32_t + 1 as libc::c_int > q_span {
        (*r).rs = (*a.offset(k as isize)).x as int32_t + 1 as libc::c_int - q_span;
    } else {
        (*r).rs = 0 as libc::c_int;
    }
    (*r)
        .re = (*a.offset((k + (*r).cnt - 1 as libc::c_int) as isize)).x as int32_t
        + 1 as libc::c_int;
    if (*r).rev() == 0 {
        (*r).qs = (*a.offset(k as isize)).y as int32_t + 1 as libc::c_int - q_span;
        (*r)
            .qe = (*a.offset((k + (*r).cnt - 1 as libc::c_int) as isize)).y as int32_t
            + 1 as libc::c_int;
    } else if is_qstrand != 0 {
        (*r).qs = (*a.offset(k as isize)).y as int32_t + 1 as libc::c_int - q_span;
        (*r)
            .qe = (*a.offset((k + (*r).cnt - 1 as libc::c_int) as isize)).y as int32_t
            + 1 as libc::c_int;
    } else {
        (*r)
            .qs = qlen
            - ((*a.offset((k + (*r).cnt - 1 as libc::c_int) as isize)).y as int32_t
                + 1 as libc::c_int);
        (*r)
            .qe = qlen
            - ((*a.offset(k as isize)).y as int32_t + 1 as libc::c_int - q_span);
    }
    mm_cal_fuzzy_len(r, a);
}
#[inline]
unsafe extern "C" fn hash64(mut key: uint64_t) -> uint64_t {
    key = (!key).wrapping_add(key << 21 as libc::c_int);
    key ^= key >> 24 as libc::c_int;
    key = key
        .wrapping_add(key << 3 as libc::c_int)
        .wrapping_add(key << 8 as libc::c_int);
    key ^= key >> 14 as libc::c_int;
    key = key
        .wrapping_add(key << 2 as libc::c_int)
        .wrapping_add(key << 4 as libc::c_int);
    key ^= key >> 28 as libc::c_int;
    key = (key as libc::c_ulong).wrapping_add(key << 31 as libc::c_int) as uint64_t
        as uint64_t;
    return key;
}
pub unsafe extern "C" fn mm_gen_regs(
    mut km: *mut libc::c_void,
    mut hash: uint32_t,
    mut qlen: libc::c_int,
    mut n_u: libc::c_int,
    mut u: *mut uint64_t,
    mut a: *mut mm128_t,
    mut is_qstrand: libc::c_int,
) -> *mut mm_reg1_t {
    let mut z: *mut mm128_t = 0 as *mut mm128_t;
    let mut tmp: mm128_t = mm128_t { x: 0, y: 0 };
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut h: uint32_t = 0;
    let mut tmp___1: uint64_t = 0;
    let mut tmp___2: uint64_t = 0;
    let mut tmp___3: uint64_t = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ri: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut tmp___5: int32_t = 0;
    if n_u == 0 as libc::c_int {
        return 0 as *mut mm_reg1_t;
    }
    tmp___0 = kmalloc(km, (n_u * 16 as libc::c_int) as size_t);
    z = tmp___0 as *mut mm128_t;
    k = 0 as libc::c_int;
    i = k;
    while i < n_u {
        tmp___1 = hash64((*a.offset(k as isize)).x);
        tmp___2 = hash64((*a.offset(k as isize)).y);
        tmp___3 = hash64(tmp___1.wrapping_add(tmp___2) ^ hash as libc::c_ulong);
        h = tmp___3 as uint32_t;
        (*z.offset(i as isize)).x = *u.offset(i as isize) ^ h as libc::c_ulong;
        (*z.offset(i as isize))
            .y = (k as uint64_t) << 32 as libc::c_int
            | *u.offset(i as isize) as int32_t as libc::c_ulong;
        k += *u.offset(i as isize) as int32_t;
        i += 1;
    }
    radix_sort_128x(z, z.offset(n_u as isize));
    i = 0 as libc::c_int;
    while i < n_u >> 1 as libc::c_int {
        tmp = *z.offset(i as isize);
        *z.offset(i as isize) = *z.offset((n_u - 1 as libc::c_int - i) as isize);
        *z.offset((n_u - 1 as libc::c_int - i) as isize) = tmp;
        i += 1;
    }
    tmp___4 = calloc(n_u as size_t, ::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong);
    r = tmp___4 as *mut mm_reg1_t;
    i = 0 as libc::c_int;
    while i < n_u {
        ri = r.offset(i as isize);
        (*ri).id = i;
        (*ri).parent = -(1 as libc::c_int);
        tmp___5 = ((*z.offset(i as isize)).x >> 32 as libc::c_int) as int32_t;
        (*ri).score0 = tmp___5;
        (*ri).score = tmp___5;
        (*ri).hash = (*z.offset(i as isize)).x as uint32_t;
        (*ri).cnt = (*z.offset(i as isize)).y as int32_t;
        (*ri).as_0 = ((*z.offset(i as isize)).y >> 32 as libc::c_int) as int32_t;
        (*ri).div = -1.0f32;
        mm_reg_set_coor(ri, qlen, a as *const mm128_t, is_qstrand);
        i += 1;
    }
    kfree(km, z as *mut libc::c_void);
    return r;
}
pub unsafe extern "C" fn mm_mark_alt(
    mut mi: *const mm_idx_t,
    mut n: libc::c_int,
    mut r: *mut mm_reg1_t,
) {
    let mut i: libc::c_int = 0;
    if (*mi).n_alt == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n {
        if (*((*mi).seq).offset((*r.offset(i as isize)).rid as isize)).is_alt != 0 {
            let ref mut fresh9 = *r.offset(i as isize);
            (*fresh9).set_is_alt(1 as libc::c_int as uint32_t);
        }
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn mm_alt_score(
    mut score: libc::c_int,
    mut alt_diff_frac: libc::c_float,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if score < 0 as libc::c_int {
        return score;
    }
    score = (score as libc::c_double * (1.0f64 - alt_diff_frac as libc::c_double)
        + 0.499f64) as libc::c_int;
    if score > 0 as libc::c_int {
        tmp = score;
    } else {
        tmp = 1 as libc::c_int;
    }
    return tmp;
}
pub unsafe extern "C" fn mm_split_reg(
    mut r: *mut mm_reg1_t,
    mut r2: *mut mm_reg1_t,
    mut n: libc::c_int,
    mut qlen: libc::c_int,
    mut a: *mut mm128_t,
    mut is_qstrand: libc::c_int,
) {
    if n <= 0 as libc::c_int {
        return
    } else {
        if n >= (*r).cnt {
            return;
        }
    }
    *r2 = *r;
    (*r2).id = -(1 as libc::c_int);
    (*r2).set_sam_pri(0 as libc::c_int as uint32_t);
    (*r2).p = 0 as *mut mm_extra_t;
    (*r2).set_split_inv(0 as libc::c_int as uint32_t);
    (*r2).cnt = (*r).cnt - n;
    (*r2)
        .score = (((*r).score as libc::c_float
        * ((*r2).cnt as libc::c_float / (*r).cnt as libc::c_float)) as libc::c_double
        + 0.499f64) as int32_t;
    (*r2).as_0 = (*r).as_0 + n;
    if (*r).parent == (*r).id {
        (*r2).parent = -(2 as libc::c_int);
    }
    mm_reg_set_coor(r2, qlen, a as *const mm128_t, is_qstrand);
    (*r).cnt -= (*r2).cnt;
    (*r).score -= (*r2).score;
    mm_reg_set_coor(r, qlen, a as *const mm128_t, is_qstrand);
    (*r).set_split((*r).split() | 1 as libc::c_uint as uint32_t);
    (*r2).set_split((*r2).split() | 2 as libc::c_uint as uint32_t);
}
pub unsafe extern "C" fn mm_set_parent(
    mut km: *mut libc::c_void,
    mut mask_level: libc::c_float,
    mut mask_len: libc::c_int,
    mut n: libc::c_int,
    mut r: *mut mm_reg1_t,
    mut sub_diff: libc::c_int,
    mut hard_mask_level: libc::c_int,
    mut alt_diff_frac: libc::c_float,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut w: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut cov: *mut uint64_t = 0 as *mut uint64_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ri: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut si: libc::c_int = 0;
    let mut ei: libc::c_int = 0;
    let mut n_cov: libc::c_int = 0;
    let mut uncov_len: libc::c_int = 0;
    let mut rp: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut sj: libc::c_int = 0;
    let mut ej: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut j___0: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut rp___0: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut sj___0: libc::c_int = 0;
    let mut ej___0: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut ol: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut cnt_sub: libc::c_int = 0;
    let mut sci: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    if n <= 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n {
        (*r.offset(i as isize)).id = i;
        i += 1;
    }
    tmp = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
    );
    cov = tmp as *mut uint64_t;
    tmp___0 = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    w = tmp___0 as *mut libc::c_int;
    *w.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    (*r.offset(0 as libc::c_int as isize)).parent = 0 as libc::c_int;
    i = 1 as libc::c_int;
    k = 1 as libc::c_int;
    while i < n {
        ri = r.offset(i as isize);
        si = (*ri).qs;
        ei = (*ri).qe;
        n_cov = 0 as libc::c_int;
        uncov_len = 0 as libc::c_int;
        if hard_mask_level != 0 {
            current_block = 6379818370700053507;
        } else {
            j = 0 as libc::c_int;
            while j < k {
                rp = r.offset(*w.offset(j as isize) as isize);
                sj = (*rp).qs;
                ej = (*rp).qe;
                if !(ej <= si) {
                    if !(sj >= ei) {
                        if sj < si {
                            sj = si;
                        }
                        if ej > ei {
                            ej = ei;
                        }
                        tmp___1 = n_cov;
                        n_cov += 1;
                        *cov
                            .offset(
                                tmp___1 as isize,
                            ) = (sj as uint64_t) << 32 as libc::c_int
                            | ej as libc::c_ulong;
                    }
                }
                j += 1;
            }
            if n_cov == 0 as libc::c_int {
                current_block = 696382619606876365;
            } else {
                if n_cov > 0 as libc::c_int {
                    x = si;
                    radix_sort_64(cov, cov.offset(n_cov as isize));
                    j___0 = 0 as libc::c_int;
                    while j___0 < n_cov {
                        if (*cov.offset(j___0 as isize) >> 32 as libc::c_int)
                            as libc::c_int > x
                        {
                            uncov_len = (uncov_len as uint64_t)
                                .wrapping_add(
                                    (*cov.offset(j___0 as isize) >> 32 as libc::c_int)
                                        .wrapping_sub(x as uint64_t),
                                ) as libc::c_int;
                        }
                        if *cov.offset(j___0 as isize) as int32_t > x {
                            x = *cov.offset(j___0 as isize) as int32_t;
                        } else {
                            x = x;
                        }
                        j___0 += 1;
                    }
                    if ei > x {
                        uncov_len += ei - x;
                    }
                }
                current_block = 6379818370700053507;
            }
        }
        match current_block {
            6379818370700053507 => {
                j = 0 as libc::c_int;
                while j < k {
                    rp___0 = r.offset(*w.offset(j as isize) as isize);
                    sj___0 = (*rp___0).qs;
                    ej___0 = (*rp___0).qe;
                    if !(ej___0 <= si) {
                        if !(sj___0 >= ei) {
                            if ej___0 - sj___0 < ei - si {
                                min = ej___0 - sj___0;
                            } else {
                                min = ei - si;
                            }
                            if ej___0 - sj___0 > ei - si {
                                max = ej___0 - sj___0;
                            } else {
                                max = ei - si;
                            }
                            if si < sj___0 {
                                if ei < sj___0 {
                                    tmp___3 = 0 as libc::c_int;
                                } else {
                                    if ei < ej___0 {
                                        tmp___2 = ei - sj___0;
                                    } else {
                                        tmp___2 = ej___0 - sj___0;
                                    }
                                    tmp___3 = tmp___2;
                                }
                                ol = tmp___3;
                            } else {
                                if ej___0 < si {
                                    tmp___5 = 0 as libc::c_int;
                                } else {
                                    if ej___0 < ei {
                                        tmp___4 = ej___0 - si;
                                    } else {
                                        tmp___4 = ei - si;
                                    }
                                    tmp___5 = tmp___4;
                                }
                                ol = tmp___5;
                            }
                            if ol as libc::c_float / min as libc::c_float
                                - uncov_len as libc::c_float / max as libc::c_float
                                > mask_level
                            {
                                if uncov_len <= mask_len {
                                    cnt_sub = 0 as libc::c_int;
                                    sci = (*ri).score;
                                    (*ri).parent = (*rp___0).parent;
                                    if (*rp___0).is_alt() == 0 {
                                        if (*ri).is_alt() != 0 {
                                            sci = mm_alt_score(sci, alt_diff_frac);
                                        }
                                    }
                                    if (*rp___0).subsc > sci {
                                        (*rp___0).subsc = (*rp___0).subsc;
                                    } else {
                                        (*rp___0).subsc = sci;
                                    }
                                    if (*ri).cnt >= (*rp___0).cnt {
                                        cnt_sub = 1 as libc::c_int;
                                    }
                                    if !((*rp___0).p).is_null() {
                                        if !((*ri).p).is_null() {
                                            if (*rp___0).rid != (*ri).rid {
                                                current_block = 15382095568084180913;
                                            } else if (*rp___0).rs != (*ri).rs {
                                                current_block = 15382095568084180913;
                                            } else if (*rp___0).re != (*ri).re {
                                                current_block = 15382095568084180913;
                                            } else if ol != min {
                                                current_block = 15382095568084180913;
                                            } else {
                                                current_block = 17336970397495664729;
                                            }
                                            match current_block {
                                                17336970397495664729 => {}
                                                _ => {
                                                    sci = (*(*ri).p).dp_max;
                                                    if (*rp___0).is_alt() == 0 {
                                                        if (*ri).is_alt() != 0 {
                                                            sci = mm_alt_score(sci, alt_diff_frac);
                                                        }
                                                    }
                                                    if (*(*rp___0).p).dp_max2 > sci {
                                                        (*(*rp___0).p).dp_max2 = (*(*rp___0).p).dp_max2;
                                                    } else {
                                                        (*(*rp___0).p).dp_max2 = sci;
                                                    }
                                                    if (*(*rp___0).p).dp_max - (*(*ri).p).dp_max <= sub_diff {
                                                        cnt_sub = 1 as libc::c_int;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if cnt_sub != 0 {
                                        (*rp___0).n_sub += 1;
                                    }
                                    break;
                                }
                            }
                        }
                    }
                    j += 1;
                }
            }
            _ => {}
        }
        if j == k {
            tmp___6 = k;
            k += 1;
            *w.offset(tmp___6 as isize) = i;
            (*ri).parent = i;
            (*ri).n_sub = 0 as libc::c_int;
        }
        i += 1;
    }
    kfree(km, cov as *mut libc::c_void);
    kfree(km, w as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_hit_sort(
    mut km: *mut libc::c_void,
    mut n_regs: *mut libc::c_int,
    mut r: *mut mm_reg1_t,
    mut alt_diff_frac: libc::c_float,
) {
    let mut i: int32_t = 0;
    let mut n_aux: int32_t = 0;
    let mut n: int32_t = 0;
    let mut has_cigar: int32_t = 0;
    let mut no_cigar: int32_t = 0;
    let mut aux: *mut mm128_t = 0 as *mut mm128_t;
    let mut t: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut score: libc::c_int = 0;
    let mut tmp___1: int32_t = 0;
    n = *n_regs;
    has_cigar = 0 as libc::c_int;
    no_cigar = 0 as libc::c_int;
    if n <= 1 as libc::c_int {
        return;
    }
    tmp = kmalloc(km, (n * 16 as libc::c_int) as size_t);
    aux = tmp as *mut mm128_t;
    tmp___0 = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong),
    );
    t = tmp___0 as *mut mm_reg1_t;
    n_aux = 0 as libc::c_int;
    i = n_aux;
    while i < n {
        let mut current_block_29: u64;
        if (*r.offset(i as isize)).inv() != 0 {
            current_block_29 = 11300420261032634049;
        } else if (*r.offset(i as isize)).cnt > 0 as libc::c_int {
            current_block_29 = 11300420261032634049;
        } else {
            if !((*r.offset(i as isize)).p).is_null() {
                free((*r.offset(i as isize)).p as *mut libc::c_void);
                let ref mut fresh10 = (*r.offset(i as isize)).p;
                *fresh10 = 0 as *mut mm_extra_t;
            }
            current_block_29 = 16924917904204750491;
        }
        match current_block_29 {
            11300420261032634049 => {
                if !((*r.offset(i as isize)).p).is_null() {
                    score = (*(*r.offset(i as isize)).p).dp_max;
                    has_cigar = 1 as libc::c_int;
                } else {
                    score = (*r.offset(i as isize)).score;
                    no_cigar = 1 as libc::c_int;
                }
                if (*r.offset(i as isize)).is_alt() != 0 {
                    score = mm_alt_score(score, alt_diff_frac);
                }
                (*aux.offset(n_aux as isize))
                    .x = (score as uint64_t) << 32 as libc::c_int
                    | (*r.offset(i as isize)).hash as libc::c_ulong;
                tmp___1 = n_aux;
                n_aux += 1;
                (*aux.offset(tmp___1 as isize)).y = i as uint64_t;
            }
            _ => {}
        }
        i += 1;
    }
    if !(has_cigar + no_cigar == 1 as libc::c_int) {
        __assert_fail(
            b"has_cigar + no_cigar == 1\0" as *const u8 as *const libc::c_char,
            b"hit.c\0" as *const u8 as *const libc::c_char,
            210 as libc::c_uint,
            b"mm_hit_sort\0" as *const u8 as *const libc::c_char,
        );
    }
    radix_sort_128x(aux, aux.offset(n_aux as isize));
    i = n_aux - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        *t
            .offset(
                (n_aux - 1 as libc::c_int - i) as isize,
            ) = *r.offset((*aux.offset(i as isize)).y as isize);
        i -= 1;
    }
    memcpy(
        r as *mut libc::c_void,
        t as *const libc::c_void,
        (::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong)
            .wrapping_mul(n_aux as libc::c_ulong),
    );
    *n_regs = n_aux;
    kfree(km, aux as *mut libc::c_void);
    kfree(km, t as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_set_sam_pri(
    mut n: libc::c_int,
    mut r: *mut mm_reg1_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n_pri: libc::c_int = 0;
    n_pri = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if (*r.offset(i as isize)).id == (*r.offset(i as isize)).parent {
            n_pri += 1;
            let ref mut fresh11 = *r.offset(i as isize);
            (*fresh11)
                .set_sam_pri((n_pri == 1 as libc::c_int) as libc::c_int as uint32_t);
        } else {
            let ref mut fresh12 = *r.offset(i as isize);
            (*fresh12).set_sam_pri(0 as libc::c_int as uint32_t);
        }
        i += 1;
    }
    return n_pri;
}
pub unsafe extern "C" fn mm_sync_regs(
    mut km: *mut libc::c_void,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
) {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut max_id: libc::c_int = 0;
    let mut n_tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    max_id = -(1 as libc::c_int);
    if n_regs <= 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        if max_id > (*regs.offset(i as isize)).id {
            max_id = max_id;
        } else {
            max_id = (*regs.offset(i as isize)).id;
        }
        i += 1;
    }
    n_tmp = max_id + 1 as libc::c_int;
    tmp___0 = kmalloc(
        km,
        (n_tmp as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    tmp = tmp___0 as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < n_tmp {
        *tmp.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        if (*regs.offset(i as isize)).id >= 0 as libc::c_int {
            *tmp.offset((*regs.offset(i as isize)).id as isize) = i;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        r = regs.offset(i as isize);
        (*r).id = i;
        if (*r).parent == -(2 as libc::c_int) {
            (*r).parent = i;
        } else if (*r).parent >= 0 as libc::c_int {
            if *tmp.offset((*r).parent as isize) >= 0 as libc::c_int {
                (*r).parent = *tmp.offset((*r).parent as isize);
            } else {
                (*r).parent = -(1 as libc::c_int);
            }
        } else {
            (*r).parent = -(1 as libc::c_int);
        }
        i += 1;
    }
    kfree(km, tmp as *mut libc::c_void);
    mm_set_sam_pri(n_regs, regs);
}
pub unsafe extern "C" fn mm_select_sub(
    mut km: *mut libc::c_void,
    mut pri_ratio: libc::c_float,
    mut min_diff: libc::c_int,
    mut best_n: libc::c_int,
    mut check_strand: libc::c_int,
    mut min_strand_sc: libc::c_int,
    mut n_: *mut libc::c_int,
    mut r: *mut mm_reg1_t,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut n_2nd: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if pri_ratio > 0.0f32 {
        if *n_ > 0 as libc::c_int {
            n = *n_;
            n_2nd = 0 as libc::c_int;
            k = 0 as libc::c_int;
            i = k;
            while i < n {
                p = (*r.offset(i as isize)).parent;
                if p == i {
                    tmp = k;
                    k += 1;
                    *r.offset(tmp as isize) = *r.offset(i as isize);
                } else if (*r.offset(i as isize)).inv() != 0 {
                    tmp = k;
                    k += 1;
                    *r.offset(tmp as isize) = *r.offset(i as isize);
                } else {
                    if (*r.offset(i as isize)).score as libc::c_float
                        >= (*r.offset(p as isize)).score as libc::c_float * pri_ratio
                    {
                        current_block = 794657222002170416;
                    } else if (*r.offset(i as isize)).score + min_diff
                            >= (*r.offset(p as isize)).score
                        {
                        current_block = 794657222002170416;
                    } else {
                        current_block = 7824908116513576431;
                    }
                    match current_block {
                        794657222002170416 => {
                            if n_2nd < best_n {
                                if (*r.offset(i as isize)).qs == (*r.offset(p as isize)).qs
                                {
                                    if (*r.offset(i as isize)).qe == (*r.offset(p as isize)).qe
                                    {
                                        if (*r.offset(i as isize)).rid
                                            == (*r.offset(p as isize)).rid
                                        {
                                            if (*r.offset(i as isize)).rs == (*r.offset(p as isize)).rs
                                            {
                                                if (*r.offset(i as isize)).re == (*r.offset(p as isize)).re
                                                {
                                                    if !((*r.offset(i as isize)).p).is_null() {
                                                        free((*r.offset(i as isize)).p as *mut libc::c_void);
                                                    }
                                                } else {
                                                    tmp___0 = k;
                                                    k += 1;
                                                    *r.offset(tmp___0 as isize) = *r.offset(i as isize);
                                                    n_2nd += 1;
                                                }
                                            } else {
                                                tmp___0 = k;
                                                k += 1;
                                                *r.offset(tmp___0 as isize) = *r.offset(i as isize);
                                                n_2nd += 1;
                                            }
                                        } else {
                                            tmp___0 = k;
                                            k += 1;
                                            *r.offset(tmp___0 as isize) = *r.offset(i as isize);
                                            n_2nd += 1;
                                        }
                                    } else {
                                        tmp___0 = k;
                                        k += 1;
                                        *r.offset(tmp___0 as isize) = *r.offset(i as isize);
                                        n_2nd += 1;
                                    }
                                } else {
                                    tmp___0 = k;
                                    k += 1;
                                    *r.offset(tmp___0 as isize) = *r.offset(i as isize);
                                    n_2nd += 1;
                                }
                                current_block = 5181772461570869434;
                            } else {
                                current_block = 7824908116513576431;
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        5181772461570869434 => {}
                        _ => {
                            if check_strand != 0 {
                                if n_2nd < best_n {
                                    if (*r.offset(i as isize)).score > min_strand_sc {
                                        if (*r.offset(i as isize)).rev() as libc::c_int
                                            != (*r.offset(p as isize)).rev() as libc::c_int
                                        {
                                            let ref mut fresh13 = *r.offset(i as isize);
                                            (*fresh13)
                                                .set_strand_retained(1 as libc::c_int as uint32_t);
                                            tmp___1 = k;
                                            k += 1;
                                            *r.offset(tmp___1 as isize) = *r.offset(i as isize);
                                            n_2nd += 1;
                                            current_block = 5181772461570869434;
                                        } else {
                                            current_block = 7010408897045639486;
                                        }
                                    } else {
                                        current_block = 7010408897045639486;
                                    }
                                } else {
                                    current_block = 7010408897045639486;
                                }
                            } else {
                                current_block = 7010408897045639486;
                            }
                            match current_block {
                                5181772461570869434 => {}
                                _ => {
                                    if !((*r.offset(i as isize)).p).is_null() {
                                        free((*r.offset(i as isize)).p as *mut libc::c_void);
                                    }
                                }
                            }
                        }
                    }
                }
                i += 1;
            }
            if k != n {
                mm_sync_regs(km, k, r);
            }
            *n_ = k;
        }
    }
}
pub unsafe extern "C" fn mm_filter_strand_retained(
    mut n_regs: libc::c_int,
    mut r: *mut mm_reg1_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    k = 0 as libc::c_int;
    i = k;
    while i < n_regs {
        p = (*r.offset(i as isize)).parent;
        if (*r.offset(i as isize)).strand_retained() == 0 {
            current_block = 17979430448776399916;
        } else if (*r.offset(i as isize)).div < (*r.offset(p as isize)).div * 5.0f32 {
            current_block = 17979430448776399916;
        } else if (*r.offset(i as isize)).div < 0.01f32 {
            current_block = 17979430448776399916;
        } else {
            current_block = 12349973810996921269;
        }
        match current_block {
            17979430448776399916 => {
                if k < i {
                    tmp = k;
                    k += 1;
                    *r.offset(tmp as isize) = *r.offset(i as isize);
                } else {
                    k += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    return k;
}
pub unsafe extern "C" fn mm_filter_regs(
    mut opt: *const mm_mapopt_t,
    mut qlen: libc::c_int,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut mm_reg1_t,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut flt: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    k = 0 as libc::c_int;
    i = k;
    while i < *n_regs {
        r = regs.offset(i as isize);
        flt = 0 as libc::c_int;
        if (*r).inv() == 0 {
            if (*r).seg_split() == 0 {
                if (*r).cnt < (*opt).min_cnt {
                    flt = 1 as libc::c_int;
                }
            }
        }
        if !((*r).p).is_null() {
            if (*r).mlen < (*opt).min_chain_score {
                flt = 1 as libc::c_int;
            } else if (*(*r).p).dp_max < (*opt).min_dp_max {
                flt = 1 as libc::c_int;
            } else if (*r).qs as libc::c_float
                    > qlen as libc::c_float * (*opt).max_clip_ratio
                {
                if (qlen - (*r).qe) as libc::c_float
                    > qlen as libc::c_float * (*opt).max_clip_ratio
                {
                    flt = 1 as libc::c_int;
                }
            }
            if flt != 0 {
                free((*r).p as *mut libc::c_void);
            }
        }
        if flt == 0 {
            if k < i {
                tmp = k;
                k += 1;
                *regs.offset(tmp as isize) = *regs.offset(i as isize);
            } else {
                k += 1;
            }
        }
        i += 1;
    }
    *n_regs = k;
}
pub unsafe extern "C" fn mm_squeeze_a(
    mut km: *mut libc::c_void,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut a: *mut mm128_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut as_0: libc::c_int = 0;
    let mut aux: *mut uint64_t = 0 as *mut uint64_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    as_0 = 0 as libc::c_int;
    tmp = kmalloc(km, (n_regs * 8 as libc::c_int) as size_t);
    aux = tmp as *mut uint64_t;
    i = 0 as libc::c_int;
    while i < n_regs {
        *aux
            .offset(
                i as isize,
            ) = ((*regs.offset(i as isize)).as_0 as uint64_t) << 32 as libc::c_int
            | i as libc::c_ulong;
        i += 1;
    }
    radix_sort_64(aux, aux.offset(n_regs as isize));
    i = 0 as libc::c_int;
    while i < n_regs {
        r = regs.offset(*aux.offset(i as isize) as int32_t as isize);
        if (*r).as_0 != as_0 {
            memmove(
                a.offset(as_0 as isize) as *mut libc::c_void,
                a.offset((*r).as_0 as isize) as *const libc::c_void,
                ((*r).cnt * 16 as libc::c_int) as size_t,
            );
            (*r).as_0 = as_0;
        }
        as_0 += (*r).cnt;
        i += 1;
    }
    kfree(km, aux as *mut libc::c_void);
    return as_0;
}
pub unsafe extern "C" fn mm_seg_gen(
    mut km: *mut libc::c_void,
    mut hash: uint32_t,
    mut n_segs: libc::c_int,
    mut qlens: *const libc::c_int,
    mut n_regs0: libc::c_int,
    mut regs0: *const mm_reg1_t,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut *mut mm_reg1_t,
    mut a: *const mm128_t,
) -> *mut mm_seg_t {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut acc_qlen: [libc::c_int; 256] = [0; 256];
    let mut qlen_sum: libc::c_int = 0;
    let mut seg: *mut mm_seg_t = 0 as *mut mm_seg_t;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r: *const mm_reg1_t = 0 as *const mm_reg1_t;
    let mut sid: libc::c_int = 0;
    let mut sr: *mut mm_seg_t = 0 as *mut mm_seg_t;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r___0: *const mm_reg1_t = 0 as *const mm_reg1_t;
    let mut sid___0: libc::c_int = 0;
    let mut a1: mm128_t = mm128_t { x: 0, y: 0 };
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    qlen_sum = 0 as libc::c_int;
    if !(n_segs <= 255 as libc::c_int) {
        __assert_fail(
            b"n_segs <= MM_MAX_SEG\0" as *const u8 as *const libc::c_char,
            b"hit.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_uint,
            b"mm_seg_gen\0" as *const u8 as *const libc::c_char,
        );
    }
    s = 1 as libc::c_int;
    acc_qlen[0 as libc::c_int as usize] = 0 as libc::c_int;
    while s < n_segs {
        acc_qlen[s
            as usize] = acc_qlen[(s - 1 as libc::c_int) as usize]
            + *qlens.offset((s - 1 as libc::c_int) as isize);
        s += 1;
    }
    qlen_sum = acc_qlen[(n_segs - 1 as libc::c_int) as usize]
        + *qlens.offset((n_segs - 1 as libc::c_int) as isize);
    tmp___0 = kcalloc(
        km,
        n_segs as size_t,
        ::std::mem::size_of::<mm_seg_t>() as libc::c_ulong,
    );
    seg = tmp___0 as *mut mm_seg_t;
    s = 0 as libc::c_int;
    while s < n_segs {
        tmp___1 = kmalloc(km, (n_regs0 * 8 as libc::c_int) as size_t);
        let ref mut fresh14 = (*seg.offset(s as isize)).u;
        *fresh14 = tmp___1 as *mut uint64_t;
        i = 0 as libc::c_int;
        while i < n_regs0 {
            *((*seg.offset(s as isize)).u)
                .offset(
                    i as isize,
                ) = ((*regs0.offset(i as isize)).score as uint64_t) << 32 as libc::c_int;
            i += 1;
        }
        s += 1;
    }
    i = 0 as libc::c_int;
    while i < n_regs0 {
        r = regs0.offset(i as isize);
        j = 0 as libc::c_int;
        while j < (*r).cnt {
            sid = (((*a.offset(((*r).as_0 + j) as isize)).y as libc::c_ulonglong
                & (255 as libc::c_ulonglong) << 48 as libc::c_int) >> 48 as libc::c_int)
                as libc::c_int;
            let ref mut fresh15 = *((*seg.offset(sid as isize)).u).offset(i as isize);
            *fresh15 = (*fresh15).wrapping_add(1);
            let ref mut fresh16 = (*seg.offset(sid as isize)).n_a;
            *fresh16 += 1;
            j += 1;
        }
        i += 1;
    }
    s = 0 as libc::c_int;
    while s < n_segs {
        sr = seg.offset(s as isize);
        i = 0 as libc::c_int;
        (*sr).n_u = 0 as libc::c_int;
        while i < n_regs0 {
            if *((*sr).u).offset(i as isize) as int32_t != 0 as libc::c_int {
                tmp___2 = (*sr).n_u;
                (*sr).n_u += 1;
                *((*sr).u).offset(tmp___2 as isize) = *((*sr).u).offset(i as isize);
            }
            i += 1;
        }
        tmp___3 = kmalloc(
            km,
            ((*sr).n_a as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
        );
        (*sr).a = tmp___3 as *mut mm128_t;
        (*sr).n_a = 0 as libc::c_int;
        s += 1;
    }
    i = 0 as libc::c_int;
    while i < n_regs0 {
        r___0 = regs0.offset(i as isize);
        j = 0 as libc::c_int;
        while j < (*r___0).cnt {
            sid___0 = (((*a.offset(((*r___0).as_0 + j) as isize)).y as libc::c_ulonglong
                & (255 as libc::c_ulonglong) << 48 as libc::c_int) >> 48 as libc::c_int)
                as libc::c_int;
            a1 = *a.offset(((*r___0).as_0 + j) as isize);
            if a1.x >> 63 as libc::c_int != 0 {
                tmp___4 = qlen_sum
                    - (*qlens.offset(sid___0 as isize) + acc_qlen[sid___0 as usize]);
            } else {
                tmp___4 = acc_qlen[sid___0 as usize];
            }
            a1
                .y = (a1.y as libc::c_ulong).wrapping_sub(tmp___4 as uint64_t)
                as uint64_t as uint64_t;
            tmp___5 = (*seg.offset(sid___0 as isize)).n_a;
            let ref mut fresh17 = (*seg.offset(sid___0 as isize)).n_a;
            *fresh17 += 1;
            *((*seg.offset(sid___0 as isize)).a).offset(tmp___5 as isize) = a1;
            j += 1;
        }
        i += 1;
    }
    s = 0 as libc::c_int;
    while s < n_segs {
        let ref mut fresh18 = *regs.offset(s as isize);
        *fresh18 = mm_gen_regs(
            km,
            hash,
            *qlens.offset(s as isize),
            (*seg.offset(s as isize)).n_u,
            (*seg.offset(s as isize)).u,
            (*seg.offset(s as isize)).a,
            0 as libc::c_int,
        );
        *n_regs.offset(s as isize) = (*seg.offset(s as isize)).n_u;
        i = 0 as libc::c_int;
        while i < *n_regs.offset(s as isize) {
            let ref mut fresh19 = *(*regs.offset(s as isize)).offset(i as isize);
            (*fresh19).set_seg_split(1 as libc::c_int as uint32_t);
            let ref mut fresh20 = *(*regs.offset(s as isize)).offset(i as isize);
            (*fresh20).set_seg_id(s as uint32_t);
            i += 1;
        }
        s += 1;
    }
    return seg;
}
pub unsafe extern "C" fn mm_seg_free(
    mut km: *mut libc::c_void,
    mut n_segs: libc::c_int,
    mut segs: *mut mm_seg_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n_segs {
        kfree(km, (*segs.offset(i as isize)).u as *mut libc::c_void);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < n_segs {
        kfree(km, (*segs.offset(i as isize)).a as *mut libc::c_void);
        i += 1;
    }
    kfree(km, segs as *mut libc::c_void);
}
unsafe extern "C" fn mm_set_inv_mapq(
    mut km: *mut libc::c_void,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
) {
    let mut i: libc::c_int = 0;
    let mut n_aux: libc::c_int = 0;
    let mut aux: *mut mm128_t = 0 as *mut mm128_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut inv: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut l: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    if n_regs < 3 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        if (*regs.offset(i as isize)).inv() != 0 {
            break;
        }
        i += 1;
    }
    if i == n_regs {
        return;
    }
    tmp = kmalloc(km, (n_regs * 16 as libc::c_int) as size_t);
    aux = tmp as *mut mm128_t;
    n_aux = 0 as libc::c_int;
    i = n_aux;
    while i < n_regs {
        if (*regs.offset(i as isize)).parent == i {
            (*aux.offset(n_aux as isize)).y = i as uint64_t;
            tmp___0 = n_aux;
            n_aux += 1;
            (*aux.offset(tmp___0 as isize))
                .x = ((*regs.offset(i as isize)).rid as uint64_t) << 32 as libc::c_int
                | (*regs.offset(i as isize)).rs as libc::c_ulong;
        } else if (*regs.offset(i as isize)).parent < 0 as libc::c_int {
            (*aux.offset(n_aux as isize)).y = i as uint64_t;
            tmp___0 = n_aux;
            n_aux += 1;
            (*aux.offset(tmp___0 as isize))
                .x = ((*regs.offset(i as isize)).rid as uint64_t) << 32 as libc::c_int
                | (*regs.offset(i as isize)).rs as libc::c_ulong;
        }
        i += 1;
    }
    radix_sort_128x(aux, aux.offset(n_aux as isize));
    i = 1 as libc::c_int;
    while i < n_aux - 1 as libc::c_int {
        inv = regs.offset((*aux.offset(i as isize)).y as isize);
        if (*inv).inv() != 0 {
            l = regs.offset((*aux.offset((i - 1 as libc::c_int) as isize)).y as isize);
            r = regs.offset((*aux.offset((i + 1 as libc::c_int) as isize)).y as isize);
            if ((*l).mapq() as libc::c_int) < (*r).mapq() as libc::c_int {
                (*inv).set_mapq((*l).mapq());
            } else {
                (*inv).set_mapq((*r).mapq());
            }
        }
        i += 1;
    }
    kfree(km, aux as *mut libc::c_void);
}
static mut q_coef: libc::c_float = 40.0f32;
pub unsafe extern "C" fn mm_set_mapq(
    mut km: *mut libc::c_void,
    mut n_regs: libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut min_chain_sc: libc::c_int,
    mut match_sc: libc::c_int,
    mut rep_len: libc::c_int,
    mut is_sr: libc::c_int,
) {
    let mut current_block: u64;
    let mut sum_sc: int64_t = 0;
    let mut uniq_ratio: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut mapq: libc::c_int = 0;
    let mut subsc: libc::c_int = 0;
    let mut pen_s1: libc::c_float = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut pen_cm: libc::c_float = 0.;
    let mut tmp___0: libc::c_float = 0.;
    let mut identity: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut tmp___1: libc::c_float = 0.;
    let mut mapq_alt: libc::c_int = 0;
    let mut x___0: libc::c_float = 0.;
    let mut identity___0: libc::c_float = 0.;
    let mut tmp___2: libc::c_float = 0.;
    let mut tmp___3: libc::c_float = 0.;
    let mut tmp___4: libc::c_float = 0.;
    sum_sc = 0 as libc::c_int as int64_t;
    if n_regs == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < n_regs {
        if (*regs.offset(i as isize)).parent == (*regs.offset(i as isize)).id {
            sum_sc += (*regs.offset(i as isize)).score as int64_t;
        }
        i += 1;
    }
    uniq_ratio = sum_sc as libc::c_float
        / (sum_sc + rep_len as int64_t) as libc::c_float;
    i = 0 as libc::c_int;
    while i < n_regs {
        r = regs.offset(i as isize);
        if (*r).inv() != 0 {
            (*r).set_mapq(0 as libc::c_int as uint32_t);
        } else if (*r).parent == (*r).id {
            if (*r).score > 100 as libc::c_int {
                tmp = 1.0f32;
            } else {
                tmp = 0.01f32 * (*r).score as libc::c_float;
            }
            pen_s1 = tmp * uniq_ratio;
            if (*r).cnt > 10 as libc::c_int {
                tmp___0 = 1.0f32;
            } else {
                tmp___0 = 0.1f32 * (*r).cnt as libc::c_float;
            }
            pen_cm = tmp___0;
            if pen_s1 < pen_cm {
                pen_cm = pen_s1;
            } else {
                pen_cm = pen_cm;
            }
            if (*r).subsc > min_chain_sc {
                subsc = (*r).subsc;
            } else {
                subsc = min_chain_sc;
            }
            if !((*r).p).is_null() {
                if (*(*r).p).dp_max2 > 0 as libc::c_int {
                    if (*(*r).p).dp_max > 0 as libc::c_int {
                        identity = (*r).mlen as libc::c_float
                            / (*r).blen as libc::c_float;
                        x = (*(*r).p).dp_max2 as libc::c_float * subsc as libc::c_float
                            / (*(*r).p).dp_max as libc::c_float
                            / (*r).score0 as libc::c_float;
                        tmp___1 = logf(
                            (*(*r).p).dp_max as libc::c_float / match_sc as libc::c_float,
                        );
                        mapq = (identity * pen_cm * q_coef * (1.0f32 - x * x) * tmp___1)
                            as libc::c_int;
                        if is_sr == 0 {
                            mapq_alt = (6.02f32 * identity * identity
                                * ((*(*r).p).dp_max - (*(*r).p).dp_max2) as libc::c_float
                                / match_sc as libc::c_float + 0.499f32) as libc::c_int;
                            if mapq < mapq_alt {
                                mapq = mapq;
                            } else {
                                mapq = mapq_alt;
                            }
                        }
                        current_block = 8835654301469918283;
                    } else {
                        current_block = 225200226364277581;
                    }
                } else {
                    current_block = 225200226364277581;
                }
            } else {
                current_block = 225200226364277581;
            }
            match current_block {
                225200226364277581 => {
                    x___0 = subsc as libc::c_float / (*r).score0 as libc::c_float;
                    if !((*r).p).is_null() {
                        identity___0 = (*r).mlen as libc::c_float
                            / (*r).blen as libc::c_float;
                        tmp___2 = logf(
                            (*(*r).p).dp_max as libc::c_float / match_sc as libc::c_float,
                        );
                        mapq = (identity___0 * pen_cm * q_coef * (1.0f32 - x___0)
                            * tmp___2) as libc::c_int;
                    } else {
                        tmp___3 = logf((*r).score as libc::c_float);
                        mapq = (pen_cm * q_coef * (1.0f32 - x___0) * tmp___3)
                            as libc::c_int;
                    }
                }
                _ => {}
            }
            tmp___4 = logf(((*r).n_sub + 1 as libc::c_int) as libc::c_float);
            mapq -= (4.343f32 * tmp___4 + 0.499f32) as libc::c_int;
            if mapq > 0 as libc::c_int {
                mapq = mapq;
            } else {
                mapq = 0 as libc::c_int;
            }
            if mapq < 60 as libc::c_int {
                (*r).set_mapq(mapq as uint32_t);
            } else {
                (*r).set_mapq(60 as libc::c_int as uint32_t);
            }
            if !((*r).p).is_null() {
                if (*(*r).p).dp_max > (*(*r).p).dp_max2 {
                    if (*r).mapq() == 0 as libc::c_uint {
                        (*r).set_mapq(1 as libc::c_int as uint32_t);
                    }
                }
            }
        } else {
            (*r).set_mapq(0 as libc::c_int as uint32_t);
        }
        i += 1;
    }
    mm_set_inv_mapq(km, n_regs, regs);
}
static mut __ac_HASH_UPPER: libc::c_double = 0.77f64;
#[inline]
unsafe extern "C" fn __ac_X31_hash_string(mut s: *const libc::c_char) -> khint_t {
    let mut h: khint_t = 0;
    h = *s as khint_t;
    if h != 0 {
        s = s.offset(1);
        while *s != 0 {
            h = (h << 5 as libc::c_int).wrapping_sub(h).wrapping_add(*s as khint_t);
            s = s.offset(1);
        }
    }
    return h;
}
#[inline]
unsafe extern "C" fn kh_init_idx() -> *mut kh_idx_t {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = kcalloc(
        0 as *mut libc::c_void,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kh_idx_t>() as libc::c_ulong,
    );
    return tmp as *mut kh_idx_t;
}
#[inline]
unsafe extern "C" fn kh_destroy_idx(mut h: *mut kh_idx_t) {
    if !h.is_null() {
        kfree(0 as *mut libc::c_void, (*h).keys as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, (*h).flags as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, (*h).vals as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, h as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn kh_get_idx(mut h: *const kh_idx_t, mut key: uint64_t) -> khint_t {
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = 0;
    let mut step: khint_t = 0;
    let mut tmp: khint_t = 0;
    if (*h).n_buckets != 0 {
        step = 0 as libc::c_int as khint_t;
        mask = ((*h).n_buckets).wrapping_sub(1 as libc::c_uint);
        k = (key >> 1 as libc::c_int) as khint_t;
        i = k & mask;
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 2 as libc::c_uint == 0
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 1 as libc::c_uint
                == 0
            {
                if *((*h).keys).offset(i as isize) >> 1 as libc::c_int
                    == key >> 1 as libc::c_int
                {
                    break;
                }
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if i == last {
                return (*h).n_buckets;
            }
        }
        if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 3 as libc::c_uint != 0
        {
            tmp = (*h).n_buckets;
        } else {
            tmp = i;
        }
        return tmp;
    } else {
        return 0 as libc::c_int as khint_t
    };
}
#[inline]
unsafe extern "C" fn kh_resize_idx(
    mut h: *mut kh_idx_t,
    mut new_n_buckets: khint_t,
) -> libc::c_int {
    let mut new_flags: *mut khint32_t = 0 as *mut khint32_t;
    let mut j: khint_t = 0;
    let mut tmp: khint_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: khint_t = 0;
    let mut new_keys: *mut uint64_t = 0 as *mut uint64_t;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_vals: *mut uint64_t = 0 as *mut uint64_t;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut key: uint64_t = 0;
    let mut val: uint64_t = 0;
    let mut new_mask: khint_t = 0;
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut step: khint_t = 0;
    let mut tmp___4: uint64_t = 0;
    let mut tmp___5: uint64_t = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    new_flags = 0 as *mut khint32_t;
    j = 1 as libc::c_int as khint_t;
    new_n_buckets = new_n_buckets.wrapping_sub(1);
    new_n_buckets |= new_n_buckets >> 1 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 2 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 4 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 8 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 16 as libc::c_int;
    new_n_buckets = new_n_buckets.wrapping_add(1);
    if new_n_buckets < 4 as libc::c_uint {
        new_n_buckets = 4 as libc::c_int as khint_t;
    }
    if (*h).size
        >= (new_n_buckets as libc::c_double * __ac_HASH_UPPER + 0.5f64) as khint_t
    {
        j = 0 as libc::c_int as khint_t;
    } else {
        if new_n_buckets < 16 as libc::c_uint {
            tmp = 1 as libc::c_int as khint_t;
        } else {
            tmp = new_n_buckets >> 4 as libc::c_int;
        }
        tmp___0 = kmalloc(
            0 as *mut libc::c_void,
            (tmp as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        new_flags = tmp___0 as *mut khint32_t;
        if new_flags.is_null() {
            return -(1 as libc::c_int);
        }
        if new_n_buckets < 16 as libc::c_uint {
            tmp___1 = 1 as libc::c_int as khint_t;
        } else {
            tmp___1 = new_n_buckets >> 4 as libc::c_int;
        }
        memset(
            new_flags as *mut libc::c_void,
            170 as libc::c_int,
            (tmp___1 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        if (*h).n_buckets < new_n_buckets {
            tmp___2 = krealloc(
                0 as *mut libc::c_void,
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
            );
            new_keys = tmp___2 as *mut uint64_t;
            if new_keys.is_null() {
                kfree(0 as *mut libc::c_void, new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).keys = new_keys;
            tmp___3 = krealloc(
                0 as *mut libc::c_void,
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
            );
            new_vals = tmp___3 as *mut uint64_t;
            if new_vals.is_null() {
                kfree(0 as *mut libc::c_void, new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).vals = new_vals;
        }
    }
    if j != 0 {
        j = 0 as libc::c_int as khint_t;
        while j != (*h).n_buckets {
            if *((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                >> ((j & 15 as libc::c_uint) << 1 as libc::c_int) & 3 as libc::c_uint
                == 0 as libc::c_uint
            {
                key = *((*h).keys).offset(j as isize);
                new_mask = new_n_buckets.wrapping_sub(1 as libc::c_uint);
                val = *((*h).vals).offset(j as isize);
                *((*h).flags)
                    .offset(
                        (j >> 4 as libc::c_int) as isize,
                    ) = (*((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                    as libc::c_ulong
                    | (1 as libc::c_ulong)
                        << ((j & 15 as libc::c_uint) << 1 as libc::c_int)) as khint32_t;
                loop {
                    step = 0 as libc::c_int as khint_t;
                    k = (key >> 1 as libc::c_int) as khint_t;
                    i = k & new_mask;
                    while *new_flags.offset((i >> 4 as libc::c_int) as isize)
                        >> ((i & 15 as libc::c_uint) << 1 as libc::c_int)
                        & 2 as libc::c_uint == 0
                    {
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & new_mask;
                    }
                    *new_flags
                        .offset(
                            (i >> 4 as libc::c_int) as isize,
                        ) = (*new_flags.offset((i >> 4 as libc::c_int) as isize)
                        as libc::c_ulong
                        & !((2 as libc::c_ulong)
                            << ((i & 15 as libc::c_uint) << 1 as libc::c_int)))
                        as khint32_t;
                    if i < (*h).n_buckets {
                        if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                            >> ((i & 15 as libc::c_uint) << 1 as libc::c_int)
                            & 3 as libc::c_uint == 0 as libc::c_uint
                        {
                            tmp___4 = *((*h).keys).offset(i as isize);
                            *((*h).keys).offset(i as isize) = key;
                            key = tmp___4;
                            tmp___5 = *((*h).vals).offset(i as isize);
                            *((*h).vals).offset(i as isize) = val;
                            val = tmp___5;
                            *((*h).flags)
                                .offset(
                                    (i >> 4 as libc::c_int) as isize,
                                ) = (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                                as libc::c_ulong
                                | (1 as libc::c_ulong)
                                    << ((i & 15 as libc::c_uint) << 1 as libc::c_int))
                                as khint32_t;
                        } else {
                            *((*h).keys).offset(i as isize) = key;
                            *((*h).vals).offset(i as isize) = val;
                            break;
                        }
                    } else {
                        *((*h).keys).offset(i as isize) = key;
                        *((*h).vals).offset(i as isize) = val;
                        break;
                    }
                }
            }
            j = j.wrapping_add(1);
        }
        if (*h).n_buckets > new_n_buckets {
            tmp___6 = krealloc(
                0 as *mut libc::c_void,
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
            );
            (*h).keys = tmp___6 as *mut uint64_t;
            tmp___7 = krealloc(
                0 as *mut libc::c_void,
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
            );
            (*h).vals = tmp___7 as *mut uint64_t;
        }
        kfree(0 as *mut libc::c_void, (*h).flags as *mut libc::c_void);
        (*h).flags = new_flags;
        (*h).n_buckets = new_n_buckets;
        (*h).n_occupied = (*h).size;
        (*h)
            .upper_bound = ((*h).n_buckets as libc::c_double * __ac_HASH_UPPER + 0.5f64)
            as khint_t;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn kh_put_idx(
    mut h: *mut kh_idx_t,
    mut key: uint64_t,
    mut ret: *mut libc::c_int,
) -> khint_t {
    let mut x: khint_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut site: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = 0;
    let mut step: khint_t = 0;
    if (*h).n_occupied >= (*h).upper_bound {
        if (*h).n_buckets > (*h).size << 1 as libc::c_int {
            tmp = kh_resize_idx(h, ((*h).n_buckets).wrapping_sub(1 as libc::c_uint));
            if tmp < 0 as libc::c_int {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        } else {
            tmp___0 = kh_resize_idx(h, ((*h).n_buckets).wrapping_add(1 as libc::c_uint));
            if tmp___0 < 0 as libc::c_int {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        }
    }
    mask = ((*h).n_buckets).wrapping_sub(1 as libc::c_uint);
    step = 0 as libc::c_int as khint_t;
    site = (*h).n_buckets;
    x = site;
    k = (key >> 1 as libc::c_int) as khint_t;
    i = k & mask;
    if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
        >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 2 as libc::c_uint != 0
    {
        x = i;
    } else {
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 2 as libc::c_uint == 0
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 1 as libc::c_uint
                == 0
            {
                if *((*h).keys).offset(i as isize) >> 1 as libc::c_int
                    == key >> 1 as libc::c_int
                {
                    break;
                }
            }
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 1 as libc::c_uint
                != 0
            {
                site = i;
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if !(i == last) {
                continue;
            }
            x = site;
            break;
        }
        if x == (*h).n_buckets {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 2 as libc::c_uint
                != 0
            {
                if site != (*h).n_buckets {
                    x = site;
                } else {
                    x = i;
                }
            } else {
                x = i;
            }
        }
    }
    if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 15 as libc::c_uint) << 1 as libc::c_int) & 2 as libc::c_uint != 0
    {
        *((*h).keys).offset(x as isize) = key;
        *((*h).flags)
            .offset(
                (x >> 4 as libc::c_int) as isize,
            ) = (*((*h).flags).offset((x >> 4 as libc::c_int) as isize) as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 15 as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).n_occupied = ((*h).n_occupied).wrapping_add(1);
        *ret = 1 as libc::c_int;
    } else if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
            >> ((x & 15 as libc::c_uint) << 1 as libc::c_int) & 1 as libc::c_uint != 0
        {
        *((*h).keys).offset(x as isize) = key;
        *((*h).flags)
            .offset(
                (x >> 4 as libc::c_int) as isize,
            ) = (*((*h).flags).offset((x >> 4 as libc::c_int) as isize) as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 15 as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        *ret = 2 as libc::c_int;
    } else {
        *ret = 0 as libc::c_int;
    }
    return x;
}
#[inline]
unsafe extern "C" fn kh_init_str() -> *mut kh_str_t {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = kcalloc(
        0 as *mut libc::c_void,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kh_str_t>() as libc::c_ulong,
    );
    return tmp as *mut kh_str_t;
}
#[inline]
unsafe extern "C" fn kh_destroy_str(mut h: *mut kh_str_t) {
    if !h.is_null() {
        kfree(0 as *mut libc::c_void, (*h).keys as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, (*h).flags as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, (*h).vals as *mut libc::c_void);
        kfree(0 as *mut libc::c_void, h as *mut libc::c_void);
    }
}
#[inline]
unsafe extern "C" fn kh_get_str(mut h: *const kh_str_t, mut key: kh_cstr_t) -> khint_t {
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = 0;
    let mut step: khint_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: khint_t = 0;
    if (*h).n_buckets != 0 {
        step = 0 as libc::c_int as khint_t;
        mask = ((*h).n_buckets).wrapping_sub(1 as libc::c_uint);
        k = __ac_X31_hash_string(key);
        i = k & mask;
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 2 as libc::c_uint == 0
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 1 as libc::c_uint
                == 0
            {
                tmp = strcmp(*((*h).keys).offset(i as isize), key);
                if tmp == 0 as libc::c_int {
                    break;
                }
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if i == last {
                return (*h).n_buckets;
            }
        }
        if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 3 as libc::c_uint != 0
        {
            tmp___0 = (*h).n_buckets;
        } else {
            tmp___0 = i;
        }
        return tmp___0;
    } else {
        return 0 as libc::c_int as khint_t
    };
}
#[inline]
unsafe extern "C" fn kh_resize_str(
    mut h: *mut kh_str_t,
    mut new_n_buckets: khint_t,
) -> libc::c_int {
    let mut new_flags: *mut khint32_t = 0 as *mut khint32_t;
    let mut j: khint_t = 0;
    let mut tmp: khint_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: khint_t = 0;
    let mut new_keys: *mut kh_cstr_t = 0 as *mut kh_cstr_t;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut new_vals: *mut uint32_t = 0 as *mut uint32_t;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut key: kh_cstr_t = 0 as *const libc::c_char;
    let mut val: uint32_t = 0;
    let mut new_mask: khint_t = 0;
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut step: khint_t = 0;
    let mut tmp___4: kh_cstr_t = 0 as *const libc::c_char;
    let mut tmp___5: uint32_t = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    new_flags = 0 as *mut khint32_t;
    j = 1 as libc::c_int as khint_t;
    new_n_buckets = new_n_buckets.wrapping_sub(1);
    new_n_buckets |= new_n_buckets >> 1 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 2 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 4 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 8 as libc::c_int;
    new_n_buckets |= new_n_buckets >> 16 as libc::c_int;
    new_n_buckets = new_n_buckets.wrapping_add(1);
    if new_n_buckets < 4 as libc::c_uint {
        new_n_buckets = 4 as libc::c_int as khint_t;
    }
    if (*h).size
        >= (new_n_buckets as libc::c_double * __ac_HASH_UPPER + 0.5f64) as khint_t
    {
        j = 0 as libc::c_int as khint_t;
    } else {
        if new_n_buckets < 16 as libc::c_uint {
            tmp = 1 as libc::c_int as khint_t;
        } else {
            tmp = new_n_buckets >> 4 as libc::c_int;
        }
        tmp___0 = kmalloc(
            0 as *mut libc::c_void,
            (tmp as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        new_flags = tmp___0 as *mut khint32_t;
        if new_flags.is_null() {
            return -(1 as libc::c_int);
        }
        if new_n_buckets < 16 as libc::c_uint {
            tmp___1 = 1 as libc::c_int as khint_t;
        } else {
            tmp___1 = new_n_buckets >> 4 as libc::c_int;
        }
        memset(
            new_flags as *mut libc::c_void,
            170 as libc::c_int,
            (tmp___1 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<khint32_t>() as libc::c_ulong),
        );
        if (*h).n_buckets < new_n_buckets {
            tmp___2 = krealloc(
                0 as *mut libc::c_void,
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<kh_cstr_t>() as libc::c_ulong),
            );
            new_keys = tmp___2 as *mut kh_cstr_t;
            if new_keys.is_null() {
                kfree(0 as *mut libc::c_void, new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).keys = new_keys;
            tmp___3 = krealloc(
                0 as *mut libc::c_void,
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
            new_vals = tmp___3 as *mut uint32_t;
            if new_vals.is_null() {
                kfree(0 as *mut libc::c_void, new_flags as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            (*h).vals = new_vals;
        }
    }
    if j != 0 {
        j = 0 as libc::c_int as khint_t;
        while j != (*h).n_buckets {
            if *((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                >> ((j & 15 as libc::c_uint) << 1 as libc::c_int) & 3 as libc::c_uint
                == 0 as libc::c_uint
            {
                key = *((*h).keys).offset(j as isize);
                new_mask = new_n_buckets.wrapping_sub(1 as libc::c_uint);
                val = *((*h).vals).offset(j as isize);
                *((*h).flags)
                    .offset(
                        (j >> 4 as libc::c_int) as isize,
                    ) = (*((*h).flags).offset((j >> 4 as libc::c_int) as isize)
                    as libc::c_ulong
                    | (1 as libc::c_ulong)
                        << ((j & 15 as libc::c_uint) << 1 as libc::c_int)) as khint32_t;
                loop {
                    step = 0 as libc::c_int as khint_t;
                    k = __ac_X31_hash_string(key);
                    i = k & new_mask;
                    while *new_flags.offset((i >> 4 as libc::c_int) as isize)
                        >> ((i & 15 as libc::c_uint) << 1 as libc::c_int)
                        & 2 as libc::c_uint == 0
                    {
                        step = step.wrapping_add(1);
                        i = i.wrapping_add(step) & new_mask;
                    }
                    *new_flags
                        .offset(
                            (i >> 4 as libc::c_int) as isize,
                        ) = (*new_flags.offset((i >> 4 as libc::c_int) as isize)
                        as libc::c_ulong
                        & !((2 as libc::c_ulong)
                            << ((i & 15 as libc::c_uint) << 1 as libc::c_int)))
                        as khint32_t;
                    if i < (*h).n_buckets {
                        if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                            >> ((i & 15 as libc::c_uint) << 1 as libc::c_int)
                            & 3 as libc::c_uint == 0 as libc::c_uint
                        {
                            tmp___4 = *((*h).keys).offset(i as isize);
                            let ref mut fresh21 = *((*h).keys).offset(i as isize);
                            *fresh21 = key;
                            key = tmp___4;
                            tmp___5 = *((*h).vals).offset(i as isize);
                            *((*h).vals).offset(i as isize) = val;
                            val = tmp___5;
                            *((*h).flags)
                                .offset(
                                    (i >> 4 as libc::c_int) as isize,
                                ) = (*((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                                as libc::c_ulong
                                | (1 as libc::c_ulong)
                                    << ((i & 15 as libc::c_uint) << 1 as libc::c_int))
                                as khint32_t;
                        } else {
                            let ref mut fresh22 = *((*h).keys).offset(i as isize);
                            *fresh22 = key;
                            *((*h).vals).offset(i as isize) = val;
                            break;
                        }
                    } else {
                        let ref mut fresh23 = *((*h).keys).offset(i as isize);
                        *fresh23 = key;
                        *((*h).vals).offset(i as isize) = val;
                        break;
                    }
                }
            }
            j = j.wrapping_add(1);
        }
        if (*h).n_buckets > new_n_buckets {
            tmp___6 = krealloc(
                0 as *mut libc::c_void,
                (*h).keys as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<kh_cstr_t>() as libc::c_ulong),
            );
            (*h).keys = tmp___6 as *mut kh_cstr_t;
            tmp___7 = krealloc(
                0 as *mut libc::c_void,
                (*h).vals as *mut libc::c_void,
                (new_n_buckets as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<uint32_t>() as libc::c_ulong),
            );
            (*h).vals = tmp___7 as *mut uint32_t;
        }
        kfree(0 as *mut libc::c_void, (*h).flags as *mut libc::c_void);
        (*h).flags = new_flags;
        (*h).n_buckets = new_n_buckets;
        (*h).n_occupied = (*h).size;
        (*h)
            .upper_bound = ((*h).n_buckets as libc::c_double * __ac_HASH_UPPER + 0.5f64)
            as khint_t;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn kh_put_str(
    mut h: *mut kh_str_t,
    mut key: kh_cstr_t,
    mut ret: *mut libc::c_int,
) -> khint_t {
    let mut x: khint_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut k: khint_t = 0;
    let mut i: khint_t = 0;
    let mut site: khint_t = 0;
    let mut last: khint_t = 0;
    let mut mask: khint_t = 0;
    let mut step: khint_t = 0;
    let mut tmp___1: libc::c_int = 0;
    if (*h).n_occupied >= (*h).upper_bound {
        if (*h).n_buckets > (*h).size << 1 as libc::c_int {
            tmp = kh_resize_str(h, ((*h).n_buckets).wrapping_sub(1 as libc::c_uint));
            if tmp < 0 as libc::c_int {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        } else {
            tmp___0 = kh_resize_str(h, ((*h).n_buckets).wrapping_add(1 as libc::c_uint));
            if tmp___0 < 0 as libc::c_int {
                *ret = -(1 as libc::c_int);
                return (*h).n_buckets;
            }
        }
    }
    mask = ((*h).n_buckets).wrapping_sub(1 as libc::c_uint);
    step = 0 as libc::c_int as khint_t;
    site = (*h).n_buckets;
    x = site;
    k = __ac_X31_hash_string(key);
    i = k & mask;
    if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
        >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 2 as libc::c_uint != 0
    {
        x = i;
    } else {
        last = i;
        while *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
            >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 2 as libc::c_uint == 0
        {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 1 as libc::c_uint
                == 0
            {
                tmp___1 = strcmp(*((*h).keys).offset(i as isize), key);
                if tmp___1 == 0 as libc::c_int {
                    break;
                }
            }
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 1 as libc::c_uint
                != 0
            {
                site = i;
            }
            step = step.wrapping_add(1);
            i = i.wrapping_add(step) & mask;
            if !(i == last) {
                continue;
            }
            x = site;
            break;
        }
        if x == (*h).n_buckets {
            if *((*h).flags).offset((i >> 4 as libc::c_int) as isize)
                >> ((i & 15 as libc::c_uint) << 1 as libc::c_int) & 2 as libc::c_uint
                != 0
            {
                if site != (*h).n_buckets {
                    x = site;
                } else {
                    x = i;
                }
            } else {
                x = i;
            }
        }
    }
    if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
        >> ((x & 15 as libc::c_uint) << 1 as libc::c_int) & 2 as libc::c_uint != 0
    {
        let ref mut fresh24 = *((*h).keys).offset(x as isize);
        *fresh24 = key;
        *((*h).flags)
            .offset(
                (x >> 4 as libc::c_int) as isize,
            ) = (*((*h).flags).offset((x >> 4 as libc::c_int) as isize) as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 15 as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        (*h).n_occupied = ((*h).n_occupied).wrapping_add(1);
        *ret = 1 as libc::c_int;
    } else if *((*h).flags).offset((x >> 4 as libc::c_int) as isize)
            >> ((x & 15 as libc::c_uint) << 1 as libc::c_int) & 1 as libc::c_uint != 0
        {
        let ref mut fresh25 = *((*h).keys).offset(x as isize);
        *fresh25 = key;
        *((*h).flags)
            .offset(
                (x >> 4 as libc::c_int) as isize,
            ) = (*((*h).flags).offset((x >> 4 as libc::c_int) as isize) as libc::c_ulong
            & !((3 as libc::c_ulong) << ((x & 15 as libc::c_uint) << 1 as libc::c_int)))
            as khint32_t;
        (*h).size = ((*h).size).wrapping_add(1);
        *ret = 2 as libc::c_int;
    } else {
        *ret = 0 as libc::c_int;
    }
    return x;
}
pub unsafe extern "C" fn mm_idx_init(
    mut w: libc::c_int,
    mut k: libc::c_int,
    mut b: libc::c_int,
    mut flag: libc::c_int,
) -> *mut mm_idx_t {
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if (k * 2 as libc::c_int) < b {
        b = k * 2 as libc::c_int;
    }
    if w < 1 as libc::c_int {
        w = 1 as libc::c_int;
    }
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<mm_idx_t>() as libc::c_ulong,
    );
    mi = tmp as *mut mm_idx_t;
    (*mi).w = w;
    (*mi).k = k;
    (*mi).b = b;
    (*mi).flag = flag;
    tmp___0 = calloc(
        ((1 as libc::c_int) << b) as size_t,
        ::std::mem::size_of::<mm_idx_bucket_t>() as libc::c_ulong,
    );
    (*mi).B = tmp___0 as *mut mm_idx_bucket_t;
    if mm_dbg_flag & 1 as libc::c_int == 0 {
        (*mi).km = km_init();
    }
    return mi;
}
pub unsafe extern "C" fn mm_idx_destroy(mut mi: *mut mm_idx_t) {
    let mut i: uint32_t = 0;
    if mi as libc::c_ulong == 0 as *mut mm_idx_t as libc::c_ulong {
        return;
    }
    if !((*mi).h).is_null() {
        kh_destroy_str((*mi).h as *mut kh_str_t);
    }
    if !((*mi).B).is_null() {
        i = 0 as libc::c_int as uint32_t;
        while i < (1 as libc::c_uint) << (*mi).b {
            free((*((*mi).B).offset(i as isize)).p as *mut libc::c_void);
            free((*((*mi).B).offset(i as isize)).a.a as *mut libc::c_void);
            kh_destroy_idx((*((*mi).B).offset(i as isize)).h as *mut idxhash_t);
            i = i.wrapping_add(1);
        }
    }
    if !((*mi).I).is_null() {
        i = 0 as libc::c_int as uint32_t;
        while i < (*mi).n_seq {
            free((*((*mi).I).offset(i as isize)).a as *mut libc::c_void);
            i = i.wrapping_add(1);
        }
        free((*mi).I as *mut libc::c_void);
    }
    if ((*mi).km).is_null() {
        i = 0 as libc::c_int as uint32_t;
        while i < (*mi).n_seq {
            free((*((*mi).seq).offset(i as isize)).name as *mut libc::c_void);
            i = i.wrapping_add(1);
        }
        free((*mi).seq as *mut libc::c_void);
    } else {
        km_destroy((*mi).km);
    }
    free((*mi).B as *mut libc::c_void);
    free((*mi).S as *mut libc::c_void);
    free(mi as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_idx_get(
    mut mi: *const mm_idx_t,
    mut minier: uint64_t,
    mut n: *mut libc::c_int,
) -> *const uint64_t {
    let mut mask: libc::c_int = 0;
    let mut k: khint_t = 0;
    let mut b: *mut mm_idx_bucket_t = 0 as *mut mm_idx_bucket_t;
    let mut h: *mut idxhash_t = 0 as *mut idxhash_t;
    mask = ((1 as libc::c_int) << (*mi).b) - 1 as libc::c_int;
    b = ((*mi).B).offset((minier & mask as libc::c_ulong) as isize);
    h = (*b).h as *mut idxhash_t;
    *n = 0 as libc::c_int;
    if h as libc::c_ulong == 0 as *mut idxhash_t as libc::c_ulong {
        return 0 as *const uint64_t;
    }
    k = kh_get_idx(h as *const kh_idx_t, minier >> (*mi).b << 1 as libc::c_int);
    if k == (*h).n_buckets {
        return 0 as *const uint64_t;
    }
    if *((*h).keys).offset(k as isize) & 1 as libc::c_ulong != 0 {
        *n = 1 as libc::c_int;
        return ((*h).vals).offset(k as isize) as *const uint64_t;
    } else {
        *n = *((*h).vals).offset(k as isize) as uint32_t as libc::c_int;
        return ((*b).p)
            .offset((*((*h).vals).offset(k as isize) >> 32 as libc::c_int) as isize)
            as *const uint64_t;
    };
}
pub unsafe extern "C" fn mm_idx_stat(mut mi: *const mm_idx_t) {
    let mut n: libc::c_int = 0;
    let mut n1: libc::c_int = 0;
    let mut i: uint32_t = 0;
    let mut sum: uint64_t = 0;
    let mut len: uint64_t = 0;
    let mut h: *mut idxhash_t = 0 as *mut idxhash_t;
    let mut k: khint_t = 0;
    let mut tmp: uint32_t = 0;
    let mut tmp___0: libc::c_double = 0.;
    let mut tmp___1: libc::c_double = 0.;
    let mut tmp___2: libc::c_double = 0.;
    n = 0 as libc::c_int;
    n1 = 0 as libc::c_int;
    sum = 0 as libc::c_int as uint64_t;
    len = 0 as libc::c_int as uint64_t;
    fprintf(
        stderr,
        b"[M::%s] kmer size: %d; skip: %d; is_hpc: %d; #seq: %d\n\0" as *const u8
            as *const libc::c_char,
        b"mm_idx_stat\0" as *const u8 as *const libc::c_char,
        (*mi).k,
        (*mi).w,
        (*mi).flag & 1 as libc::c_int,
        (*mi).n_seq,
    );
    i = 0 as libc::c_int as uint32_t;
    while i < (*mi).n_seq {
        len = (len as libc::c_ulong)
            .wrapping_add((*((*mi).seq).offset(i as isize)).len as uint64_t) as uint64_t
            as uint64_t;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as uint32_t;
    while i < (1 as libc::c_uint) << (*mi).b {
        if !((*((*mi).B).offset(i as isize)).h).is_null() {
            n = (n as khint_t)
                .wrapping_add(
                    (*((*((*mi).B).offset(i as isize)).h as *mut idxhash_t)).size,
                ) as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as uint32_t;
    while i < (1 as libc::c_uint) << (*mi).b {
        h = (*((*mi).B).offset(i as isize)).h as *mut idxhash_t;
        if !(h as libc::c_ulong == 0 as *mut idxhash_t as libc::c_ulong) {
            k = 0 as libc::c_int as khint_t;
            while k < (*h).n_buckets {
                if *((*h).flags).offset((k >> 4 as libc::c_int) as isize)
                    >> ((k & 15 as libc::c_uint) << 1 as libc::c_int) & 3 as libc::c_uint
                    == 0
                {
                    if *((*h).keys).offset(k as isize) & 1 as libc::c_ulong != 0 {
                        tmp = 1 as libc::c_int as uint32_t;
                    } else {
                        tmp = *((*h).vals).offset(k as isize) as uint32_t;
                    }
                    sum = (sum as libc::c_ulong).wrapping_add(tmp as uint64_t)
                        as uint64_t as uint64_t;
                    if *((*h).keys).offset(k as isize) & 1 as libc::c_ulong != 0 {
                        n1 += 1;
                    }
                }
                k = k.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    tmp___0 = cputime();
    tmp___1 = realtime();
    tmp___2 = realtime();
    fprintf(
        stderr,
        b"[M::%s::%.3f*%.2f] distinct minimizers: %d (%.2f%% are singletons); average occurrences: %.3lf; average spacing: %.3lf; total length: %ld\n\0"
            as *const u8 as *const libc::c_char,
        b"mm_idx_stat\0" as *const u8 as *const libc::c_char,
        tmp___2 - mm_realtime0,
        tmp___0 / (tmp___1 - mm_realtime0),
        n,
        100.0f64 * n1 as libc::c_double / n as libc::c_double,
        sum as libc::c_double / n as libc::c_double,
        len as libc::c_double / sum as libc::c_double,
        len as libc::c_long,
    );
}
pub unsafe extern "C" fn mm_idx_index_name(mut mi: *mut mm_idx_t) -> libc::c_int {
    let mut h: *mut kh_str_t = 0 as *mut kh_str_t;
    let mut i: uint32_t = 0;
    let mut has_dup: libc::c_int = 0;
    let mut absent: libc::c_int = 0;
    let mut k: khint_t = 0;
    has_dup = 0 as libc::c_int;
    if !((*mi).h).is_null() {
        return 0 as libc::c_int;
    }
    h = kh_init_str();
    i = 0 as libc::c_int as uint32_t;
    while i < (*mi).n_seq {
        k = kh_put_str(
            h,
            (*((*mi).seq).offset(i as isize)).name as kh_cstr_t,
            &mut absent,
        );
        if absent != 0 {
            *((*h).vals).offset(k as isize) = i;
        } else {
            has_dup = 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    (*mi).h = h as *mut libc::c_void;
    if has_dup != 0 {
        if mm_verbose >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"[WARNING] some database sequences have identical sequence names\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    return has_dup;
}
pub unsafe extern "C" fn mm_idx_name2id(
    mut mi: *const mm_idx_t,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut h: *mut kh_str_t = 0 as *mut kh_str_t;
    let mut k: khint_t = 0;
    let mut tmp: uint32_t = 0;
    h = (*mi).h as *mut kh_str_t;
    if h as libc::c_ulong == 0 as *mut kh_str_t as libc::c_ulong {
        return -(2 as libc::c_int);
    }
    k = kh_get_str(h as *const kh_str_t, name);
    if k == (*h).n_buckets {
        tmp = -(1 as libc::c_int) as uint32_t;
    } else {
        tmp = *((*h).vals).offset(k as isize);
    }
    return tmp as libc::c_int;
}
pub unsafe extern "C" fn mm_idx_getseq(
    mut mi: *const mm_idx_t,
    mut rid: uint32_t,
    mut st: uint32_t,
    mut en: uint32_t,
    mut seq: *mut uint8_t,
) -> libc::c_int {
    let mut i: uint64_t = 0;
    let mut st1: uint64_t = 0;
    let mut en1: uint64_t = 0;
    if rid >= (*mi).n_seq {
        return -(1 as libc::c_int)
    } else {
        if st >= (*((*mi).seq).offset(rid as isize)).len {
            return -(1 as libc::c_int);
        }
    }
    if en > (*((*mi).seq).offset(rid as isize)).len {
        en = (*((*mi).seq).offset(rid as isize)).len;
    }
    st1 = ((*((*mi).seq).offset(rid as isize)).offset).wrapping_add(st as uint64_t);
    en1 = ((*((*mi).seq).offset(rid as isize)).offset).wrapping_add(en as uint64_t);
    i = st1;
    while i < en1 {
        *seq
            .offset(
                i.wrapping_sub(st1) as isize,
            ) = (*((*mi).S).offset((i >> 3 as libc::c_int) as isize)
            >> ((i & 7 as libc::c_ulong) << 2 as libc::c_int) & 15 as libc::c_uint)
            as uint8_t;
        i = i.wrapping_add(1);
    }
    return en.wrapping_sub(st) as libc::c_int;
}
pub unsafe extern "C" fn mm_idx_getseq_rev(
    mut mi: *const mm_idx_t,
    mut rid: uint32_t,
    mut st: uint32_t,
    mut en: uint32_t,
    mut seq: *mut uint8_t,
) -> libc::c_int {
    let mut i: uint64_t = 0;
    let mut st1: uint64_t = 0;
    let mut en1: uint64_t = 0;
    let mut s: *const mm_idx_seq_t = 0 as *const mm_idx_seq_t;
    let mut c: uint8_t = 0;
    if rid >= (*mi).n_seq {
        return -(1 as libc::c_int)
    } else {
        if st >= (*((*mi).seq).offset(rid as isize)).len {
            return -(1 as libc::c_int);
        }
    }
    s = ((*mi).seq).offset(rid as isize) as *const mm_idx_seq_t;
    if en > (*s).len {
        en = (*s).len;
    }
    st1 = ((*s).offset).wrapping_add(((*s).len).wrapping_sub(en) as uint64_t);
    en1 = ((*s).offset).wrapping_add(((*s).len).wrapping_sub(st) as uint64_t);
    i = st1;
    while i < en1 {
        c = (*((*mi).S).offset((i >> 3 as libc::c_int) as isize)
            >> ((i & 7 as libc::c_ulong) << 2 as libc::c_int) & 15 as libc::c_uint)
            as uint8_t;
        if (c as libc::c_int) < 4 as libc::c_int {
            *seq
                .offset(
                    en1.wrapping_sub(i).wrapping_sub(1 as libc::c_ulong) as isize,
                ) = (3 as libc::c_int - c as libc::c_int) as uint8_t;
        } else {
            *seq
                .offset(
                    en1.wrapping_sub(i).wrapping_sub(1 as libc::c_ulong) as isize,
                ) = c;
        }
        i = i.wrapping_add(1);
    }
    return en.wrapping_sub(st) as libc::c_int;
}
pub unsafe extern "C" fn mm_idx_getseq2(
    mut mi: *const mm_idx_t,
    mut is_rev: libc::c_int,
    mut rid: uint32_t,
    mut st: uint32_t,
    mut en: uint32_t,
    mut seq: *mut uint8_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if is_rev != 0 {
        tmp = mm_idx_getseq_rev(mi, rid, st, en, seq);
        return tmp;
    } else {
        tmp___0 = mm_idx_getseq(mi, rid, st, en, seq);
        return tmp___0;
    };
}
pub unsafe extern "C" fn mm_idx_cal_max_occ(
    mut mi: *const mm_idx_t,
    mut f: libc::c_float,
) -> int32_t {
    let mut i: libc::c_int = 0;
    let mut n: size_t = 0;
    let mut thres: uint32_t = 0;
    let mut a: *mut khint_t = 0 as *mut khint_t;
    let mut k: khint_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut h: *mut idxhash_t = 0 as *mut idxhash_t;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: uint32_t = 0;
    n = 0 as libc::c_int as size_t;
    if f as libc::c_double <= 0.0f64 {
        return 2147483647 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << (*mi).b {
        if !((*((*mi).B).offset(i as isize)).h).is_null() {
            n = (n as libc::c_ulong)
                .wrapping_add(
                    (*((*((*mi).B).offset(i as isize)).h as *mut idxhash_t)).size
                        as size_t,
                ) as size_t as size_t;
        }
        i += 1;
    }
    tmp = malloc(n.wrapping_mul(4 as libc::c_ulong));
    a = tmp as *mut uint32_t;
    n = 0 as libc::c_int as size_t;
    i = n as libc::c_int;
    while i < (1 as libc::c_int) << (*mi).b {
        h = (*((*mi).B).offset(i as isize)).h as *mut idxhash_t;
        if !(h as libc::c_ulong == 0 as *mut idxhash_t as libc::c_ulong) {
            k = 0 as libc::c_int as khint_t;
            while k < (*h).n_buckets {
                if !(*((*h).flags).offset((k >> 4 as libc::c_int) as isize)
                    >> ((k & 15 as libc::c_uint) << 1 as libc::c_int) & 3 as libc::c_uint
                    != 0)
                {
                    tmp___0 = n;
                    n = n.wrapping_add(1);
                    if *((*h).keys).offset(k as isize) & 1 as libc::c_ulong != 0 {
                        *a.offset(tmp___0 as isize) = 1 as libc::c_int as khint_t;
                    } else {
                        *a
                            .offset(
                                tmp___0 as isize,
                            ) = *((*h).vals).offset(k as isize) as uint32_t;
                    }
                }
                k = k.wrapping_add(1);
            }
        }
        i += 1;
    }
    tmp___1 = ks_ksmall_uint32_t(
        n,
        a,
        ((1.0f64 - f as libc::c_double) * n as libc::c_double) as uint32_t as size_t,
    );
    thres = tmp___1.wrapping_add(1 as libc::c_uint);
    free(a as *mut libc::c_void);
    return thres as int32_t;
}
unsafe extern "C" fn worker_post(
    mut g: *mut libc::c_void,
    mut i: libc::c_long,
    mut tid: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut n_keys: libc::c_int = 0;
    let mut j: size_t = 0;
    let mut start_a: size_t = 0;
    let mut start_p: size_t = 0;
    let mut h: *mut idxhash_t = 0 as *mut idxhash_t;
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    let mut b: *mut mm_idx_bucket_t = 0 as *mut mm_idx_bucket_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut itr: khint_t = 0;
    let mut absent: libc::c_int = 0;
    let mut p: *mut mm128_t = 0 as *mut mm128_t;
    let mut k: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    mi = g as *mut mm_idx_t;
    b = ((*mi).B).offset(i as isize);
    if (*b).a.n == 0 as libc::c_ulong {
        return;
    }
    radix_sort_128x((*b).a.a, ((*b).a.a).offset((*b).a.n as isize));
    j = 1 as libc::c_int as size_t;
    n = 1 as libc::c_int;
    n_keys = 0 as libc::c_int;
    (*b).n = 0 as libc::c_int;
    while j <= (*b).a.n {
        let mut current_block_16: u64;
        if j == (*b).a.n {
            current_block_16 = 13708816027213318628;
        } else if (*((*b).a.a).offset(j as isize)).x >> 8 as libc::c_int
                != (*((*b).a.a).offset(j.wrapping_sub(1 as libc::c_ulong) as isize)).x
                    >> 8 as libc::c_int
            {
            current_block_16 = 13708816027213318628;
        } else {
            n += 1;
            current_block_16 = 224731115979188411;
        }
        match current_block_16 {
            13708816027213318628 => {
                n_keys += 1;
                if n > 1 as libc::c_int {
                    (*b).n += n;
                }
                n = 1 as libc::c_int;
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    h = kh_init_idx();
    kh_resize_idx(h, n_keys as khint_t);
    tmp = calloc((*b).n as size_t, 8 as libc::c_int as size_t);
    (*b).p = tmp as *mut uint64_t;
    j = 1 as libc::c_int as size_t;
    n = 1 as libc::c_int;
    start_p = 0 as libc::c_int as size_t;
    start_a = start_p;
    while j <= (*b).a.n {
        let mut current_block_54: u64;
        if j == (*b).a.n {
            current_block_54 = 948458669930030942;
        } else if (*((*b).a.a).offset(j as isize)).x >> 8 as libc::c_int
                != (*((*b).a.a).offset(j.wrapping_sub(1 as libc::c_ulong) as isize)).x
                    >> 8 as libc::c_int
            {
            current_block_54 = 948458669930030942;
        } else {
            n += 1;
            current_block_54 = 5891011138178424807;
        }
        match current_block_54 {
            948458669930030942 => {
                p = ((*b).a.a).offset(j.wrapping_sub(1 as libc::c_ulong) as isize);
                itr = kh_put_idx(
                    h,
                    (*p).x >> 8 as libc::c_int >> (*mi).b << 1 as libc::c_int,
                    &mut absent,
                );
                if absent != 0 {
                    if !(j == start_a.wrapping_add(n as size_t)) {
                        __assert_fail(
                            b"absent && j == start_a + n\0" as *const u8
                                as *const libc::c_char,
                            b"index.c\0" as *const u8 as *const libc::c_char,
                            244 as libc::c_uint,
                            b"worker_post\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    __assert_fail(
                        b"absent && j == start_a + n\0" as *const u8
                            as *const libc::c_char,
                        b"index.c\0" as *const u8 as *const libc::c_char,
                        244 as libc::c_uint,
                        b"worker_post\0" as *const u8 as *const libc::c_char,
                    );
                }
                if n == 1 as libc::c_int {
                    let ref mut fresh26 = *((*h).keys).offset(itr as isize);
                    *fresh26 |= 1 as libc::c_ulong;
                    *((*h).vals).offset(itr as isize) = (*p).y;
                } else {
                    k = 0 as libc::c_int;
                    while k < n {
                        *((*b).p)
                            .offset(
                                start_p.wrapping_add(k as size_t) as isize,
                            ) = (*((*b).a.a)
                            .offset(start_a.wrapping_add(k as size_t) as isize))
                            .y;
                        k += 1;
                    }
                    radix_sort_64(
                        ((*b).p).offset(start_p as isize),
                        ((*b).p).offset(start_p.wrapping_add(n as size_t) as isize),
                    );
                    *((*h).vals)
                        .offset(
                            itr as isize,
                        ) = start_p << 32 as libc::c_int | n as libc::c_ulong;
                    start_p = (start_p as libc::c_ulong).wrapping_add(n as size_t)
                        as size_t as size_t;
                }
                start_a = j;
                n = 1 as libc::c_int;
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    (*b).h = h as *mut libc::c_void;
    if !((*b).n == start_p as int32_t) {
        __assert_fail(
            b"b->n == (int32_t)start_p\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            260 as libc::c_uint,
            b"worker_post\0" as *const u8 as *const libc::c_char,
        );
    }
    kfree(0 as *mut libc::c_void, (*b).a.a as *mut libc::c_void);
    tmp___2 = 0 as libc::c_int as size_t;
    (*b).a.m = tmp___2;
    (*b).a.n = tmp___2;
    (*b).a.a = 0 as *mut mm128_t;
}
unsafe extern "C" fn mm_idx_post(mut mi: *mut mm_idx_t, mut n_threads: libc::c_int) {
    kt_for(
        n_threads,
        Some(
            worker_post
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_long,
                    libc::c_int,
                ) -> (),
        ),
        mi as *mut libc::c_void,
        ((1 as libc::c_int) << (*mi).b) as libc::c_long,
    );
}
unsafe extern "C" fn mm_idx_add(
    mut mi: *mut mm_idx_t,
    mut n: libc::c_int,
    mut a: *const mm128_t,
) {
    let mut i: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut p: *mut mm128_v = 0 as *mut mm128_v;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: size_t = 0;
    mask = ((1 as libc::c_int) << (*mi).b) - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        p = &mut (*((*mi).B)
            .offset(
                ((*a.offset(i as isize)).x >> 8 as libc::c_int & mask as libc::c_ulong)
                    as isize,
            ))
            .a;
        if (*p).n == (*p).m {
            if (*p).m != 0 {
                (*p).m <<= 1 as libc::c_int;
            } else {
                (*p).m = 2 as libc::c_int as size_t;
            }
            tmp = krealloc(
                0 as *mut libc::c_void,
                (*p).a as *mut libc::c_void,
                (::std::mem::size_of::<mm128_t>() as libc::c_ulong).wrapping_mul((*p).m),
            );
            (*p).a = tmp as *mut mm128_t;
        }
        tmp___0 = (*p).n;
        (*p).n = ((*p).n).wrapping_add(1);
        *((*p).a).offset(tmp___0 as isize) = *a.offset(i as isize);
        i += 1;
    }
}
unsafe extern "C" fn worker_pipeline(
    mut shared: *mut libc::c_void,
    mut step: libc::c_int,
    mut in_0: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut p: *mut pipeline_t = 0 as *mut pipeline_t;
    let mut s: *mut step_t = 0 as *mut step_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut old_m: uint32_t = 0;
    let mut m: uint32_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut sum_len: uint64_t = 0;
    let mut old_max_len: uint64_t = 0;
    let mut max_len: uint64_t = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut seq: *mut mm_idx_seq_t = 0 as *mut mm_idx_seq_t;
    let mut j: uint32_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut o: uint64_t = 0;
    let mut c: libc::c_int = 0;
    let mut tmp___5: uint32_t = 0;
    let mut s___0: *mut step_t = 0 as *mut step_t;
    let mut t: *mut mm_bseq1_t = 0 as *mut mm_bseq1_t;
    let mut s___1: *mut step_t = 0 as *mut step_t;
    p = shared as *mut pipeline_t;
    if step == 0 as libc::c_int {
        if (*p).sum_len > (*p).batch_size {
            return 0 as *mut libc::c_void;
        }
        tmp = calloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<step_t>() as libc::c_ulong,
        );
        s = tmp as *mut step_t;
        (*s)
            .seq = mm_bseq_read(
            (*p).fp,
            (*p).mini_batch_size as int64_t,
            0 as libc::c_int,
            &mut (*s).n_seq,
        );
        if !((*s).seq).is_null() {
            if !(((*(*p).mi).n_seq as uint64_t).wrapping_add((*s).n_seq as uint64_t)
                <= 4294967295 as libc::c_ulong)
            {
                __assert_fail(
                    b"(uint64_t)p->mi->n_seq + s->n_seq <= UINT32_MAX\0" as *const u8
                        as *const libc::c_char,
                    b"index.c\0" as *const u8 as *const libc::c_char,
                    313 as libc::c_uint,
                    b"worker_pipeline\0" as *const u8 as *const libc::c_char,
                );
            }
            old_m = (*(*p).mi).n_seq;
            m = ((*(*p).mi).n_seq).wrapping_add((*s).n_seq as uint32_t);
            m = m.wrapping_sub(1);
            m |= m >> 1 as libc::c_int;
            m |= m >> 2 as libc::c_int;
            m |= m >> 4 as libc::c_int;
            m |= m >> 8 as libc::c_int;
            m |= m >> 16 as libc::c_int;
            m = m.wrapping_add(1);
            old_m = old_m.wrapping_sub(1);
            old_m |= old_m >> 1 as libc::c_int;
            old_m |= old_m >> 2 as libc::c_int;
            old_m |= old_m >> 4 as libc::c_int;
            old_m |= old_m >> 8 as libc::c_int;
            old_m |= old_m >> 16 as libc::c_int;
            old_m = old_m.wrapping_add(1);
            if old_m != m {
                tmp___1 = krealloc(
                    (*(*p).mi).km,
                    (*(*p).mi).seq as *mut libc::c_void,
                    (m as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<mm_idx_seq_t>() as libc::c_ulong,
                        ),
                );
                (*(*p).mi).seq = tmp___1 as *mut mm_idx_seq_t;
            }
            if (*(*p).mi).flag & 2 as libc::c_int == 0 {
                i = 0 as libc::c_int;
                sum_len = 0 as libc::c_int as uint64_t;
                while i < (*s).n_seq {
                    sum_len = (sum_len as libc::c_ulong)
                        .wrapping_add((*((*s).seq).offset(i as isize)).l_seq as uint64_t)
                        as uint64_t as uint64_t;
                    i += 1;
                }
                old_max_len = ((*p).sum_len)
                    .wrapping_add(7 as libc::c_ulong)
                    .wrapping_div(8 as libc::c_ulong);
                max_len = ((*p).sum_len)
                    .wrapping_add(sum_len)
                    .wrapping_add(7 as libc::c_ulong)
                    .wrapping_div(8 as libc::c_ulong);
                old_max_len = old_max_len.wrapping_sub(1);
                old_max_len |= old_max_len >> 1 as libc::c_int;
                old_max_len |= old_max_len >> 2 as libc::c_int;
                old_max_len |= old_max_len >> 4 as libc::c_int;
                old_max_len |= old_max_len >> 8 as libc::c_int;
                old_max_len |= old_max_len >> 16 as libc::c_int;
                old_max_len |= old_max_len >> 32 as libc::c_int;
                old_max_len = old_max_len.wrapping_add(1);
                max_len = max_len.wrapping_sub(1);
                max_len |= max_len >> 1 as libc::c_int;
                max_len |= max_len >> 2 as libc::c_int;
                max_len |= max_len >> 4 as libc::c_int;
                max_len |= max_len >> 8 as libc::c_int;
                max_len |= max_len >> 16 as libc::c_int;
                max_len |= max_len >> 32 as libc::c_int;
                max_len = max_len.wrapping_add(1);
                if old_max_len != max_len {
                    tmp___2 = realloc(
                        (*(*p).mi).S as *mut libc::c_void,
                        max_len.wrapping_mul(4 as libc::c_ulong),
                    );
                    (*(*p).mi).S = tmp___2 as *mut uint32_t;
                    memset(
                        ((*(*p).mi).S).offset(old_max_len as isize) as *mut libc::c_void,
                        0 as libc::c_int,
                        (4 as libc::c_ulong)
                            .wrapping_mul(max_len.wrapping_sub(old_max_len)),
                    );
                }
            }
            i = 0 as libc::c_int;
            while i < (*s).n_seq {
                seq = ((*(*p).mi).seq).offset((*(*p).mi).n_seq as isize);
                if (*(*p).mi).flag & 4 as libc::c_int == 0 {
                    tmp___3 = strlen(
                        (*((*s).seq).offset(i as isize)).name as *const libc::c_char,
                    );
                    tmp___4 = kmalloc(
                        (*(*p).mi).km,
                        tmp___3.wrapping_add(1 as libc::c_ulong),
                    );
                    (*seq).name = tmp___4 as *mut libc::c_char;
                    strcpy(
                        (*seq).name,
                        (*((*s).seq).offset(i as isize)).name as *const libc::c_char,
                    );
                } else {
                    (*seq).name = 0 as *mut libc::c_char;
                }
                (*seq).len = (*((*s).seq).offset(i as isize)).l_seq as uint32_t;
                (*seq).offset = (*p).sum_len;
                (*seq).is_alt = 0 as libc::c_int as uint32_t;
                if (*(*p).mi).flag & 2 as libc::c_int == 0 {
                    j = 0 as libc::c_int as uint32_t;
                    while j < (*seq).len {
                        o = ((*p).sum_len).wrapping_add(j as uint64_t);
                        c = seq_nt4_table[*((*((*s).seq).offset(i as isize)).seq)
                            .offset(j as isize) as uint8_t as usize] as libc::c_int;
                        let ref mut fresh27 = *((*(*p).mi).S)
                            .offset((o >> 3 as libc::c_int) as isize);
                        *fresh27
                            |= (c as uint32_t)
                                << ((o & 7 as libc::c_ulong) << 2 as libc::c_int);
                        j = j.wrapping_add(1);
                    }
                }
                (*p)
                    .sum_len = ((*p).sum_len as libc::c_ulong)
                    .wrapping_add((*seq).len as uint64_t) as uint64_t as uint64_t;
                tmp___5 = (*(*p).mi).n_seq;
                (*(*p).mi).n_seq = ((*(*p).mi).n_seq).wrapping_add(1);
                (*((*s).seq).offset(i as isize)).rid = tmp___5 as libc::c_int;
                i += 1;
            }
            return s as *mut libc::c_void;
        } else {
            free(s as *mut libc::c_void);
        }
    } else if step == 1 as libc::c_int {
        s___0 = in_0 as *mut step_t;
        i = 0 as libc::c_int;
        while i < (*s___0).n_seq {
            t = ((*s___0).seq).offset(i as isize);
            if (*t).l_seq > 0 as libc::c_int {
                mm_sketch(
                    0 as *mut libc::c_void,
                    (*t).seq as *const libc::c_char,
                    (*t).l_seq,
                    (*(*p).mi).w,
                    (*(*p).mi).k,
                    (*t).rid as uint32_t,
                    (*(*p).mi).flag & 1 as libc::c_int,
                    &mut (*s___0).a,
                );
            } else if mm_verbose >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"[WARNING] the length database sequence '%s' is 0\n\0" as *const u8
                        as *const libc::c_char,
                    (*t).name,
                );
            }
            free((*t).seq as *mut libc::c_void);
            free((*t).name as *mut libc::c_void);
            i += 1;
        }
        free((*s___0).seq as *mut libc::c_void);
        (*s___0).seq = 0 as *mut mm_bseq1_t;
        return s___0 as *mut libc::c_void;
    } else {
        if step == 2 as libc::c_int {
            s___1 = in_0 as *mut step_t;
            mm_idx_add(
                (*p).mi,
                (*s___1).a.n as libc::c_int,
                (*s___1).a.a as *const mm128_t,
            );
            kfree(0 as *mut libc::c_void, (*s___1).a.a as *mut libc::c_void);
            free(s___1 as *mut libc::c_void);
        }
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn mm_idx_gen(
    mut fp: *mut mm_bseq_file_t,
    mut w: libc::c_int,
    mut k: libc::c_int,
    mut b: libc::c_int,
    mut flag: libc::c_int,
    mut mini_batch_size: libc::c_int,
    mut n_threads: libc::c_int,
    mut batch_size: uint64_t,
) -> *mut mm_idx_t {
    let mut pl: pipeline_t = pipeline_t {
        mini_batch_size: 0,
        batch_size: 0,
        sum_len: 0,
        fp: 0 as *mut mm_bseq_file_t,
        mi: 0 as *mut mm_idx_t,
    };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_double = 0.;
    let mut tmp___2: libc::c_double = 0.;
    let mut tmp___3: libc::c_double = 0.;
    let mut tmp___4: libc::c_double = 0.;
    let mut tmp___5: libc::c_double = 0.;
    let mut tmp___6: libc::c_double = 0.;
    if fp as libc::c_ulong == 0 as *mut mm_bseq_file_t as libc::c_ulong {
        return 0 as *mut mm_idx_t
    } else {
        tmp = mm_bseq_eof(fp);
        if tmp != 0 {
            return 0 as *mut mm_idx_t;
        }
    }
    memset(
        &mut pl as *mut pipeline_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pipeline_t>() as libc::c_ulong,
    );
    if (mini_batch_size as uint64_t) < batch_size {
        pl.mini_batch_size = mini_batch_size;
    } else {
        pl.mini_batch_size = batch_size as libc::c_int;
    }
    pl.batch_size = batch_size;
    pl.fp = fp;
    pl.mi = mm_idx_init(w, k, b, flag);
    if n_threads < 3 as libc::c_int {
        tmp___0 = n_threads;
    } else {
        tmp___0 = 3 as libc::c_int;
    }
    kt_pipeline(
        tmp___0,
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
    if mm_verbose >= 3 as libc::c_int {
        tmp___1 = cputime();
        tmp___2 = realtime();
        tmp___3 = realtime();
        fprintf(
            stderr,
            b"[M::%s::%.3f*%.2f] collected minimizers\n\0" as *const u8
                as *const libc::c_char,
            b"mm_idx_gen\0" as *const u8 as *const libc::c_char,
            tmp___3 - mm_realtime0,
            tmp___1 / (tmp___2 - mm_realtime0),
        );
    }
    mm_idx_post(pl.mi, n_threads);
    if mm_verbose >= 3 as libc::c_int {
        tmp___4 = cputime();
        tmp___5 = realtime();
        tmp___6 = realtime();
        fprintf(
            stderr,
            b"[M::%s::%.3f*%.2f] sorted minimizers\n\0" as *const u8
                as *const libc::c_char,
            b"mm_idx_gen\0" as *const u8 as *const libc::c_char,
            tmp___6 - mm_realtime0,
            tmp___4 / (tmp___5 - mm_realtime0),
        );
    }
    return pl.mi;
}
pub unsafe extern "C" fn mm_idx_build(
    mut fn_0: *const libc::c_char,
    mut w: libc::c_int,
    mut k: libc::c_int,
    mut flag: libc::c_int,
    mut n_threads: libc::c_int,
) -> *mut mm_idx_t {
    let mut fp: *mut mm_bseq_file_t = 0 as *mut mm_bseq_file_t;
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    fp = mm_bseq_open(fn_0);
    if fp as libc::c_ulong == 0 as *mut mm_bseq_file_t as libc::c_ulong {
        return 0 as *mut mm_idx_t;
    }
    mi = mm_idx_gen(
        fp,
        w,
        k,
        14 as libc::c_int,
        flag,
        (1 as libc::c_int) << 18 as libc::c_int,
        n_threads,
        18446744073709551615 as libc::c_ulonglong as uint64_t,
    );
    mm_bseq_close(fp);
    return mi;
}
pub unsafe extern "C" fn mm_idx_str(
    mut w: libc::c_int,
    mut k: libc::c_int,
    mut is_hpc: libc::c_int,
    mut bucket_bits: libc::c_int,
    mut n: libc::c_int,
    mut seq: *mut *const libc::c_char,
    mut name: *mut *const libc::c_char,
) -> *mut mm_idx_t {
    let mut sum_len: uint64_t = 0;
    let mut a: mm128_v = mm128_v {
        n: 0,
        m: 0,
        a: 0 as *mut mm128_t,
    };
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    let mut h: *mut kh_str_t = 0 as *mut kh_str_t;
    let mut i: libc::c_int = 0;
    let mut flag: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut mm_idx_seq_t = 0 as *mut mm_idx_seq_t;
    let mut j: uint32_t = 0;
    let mut absent: libc::c_int = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: size_t = 0;
    let mut c: libc::c_int = 0;
    let mut o: uint64_t = 0;
    sum_len = 0 as libc::c_int as uint64_t;
    a.n = 0 as libc::c_int as size_t;
    a.m = 0 as libc::c_int as size_t;
    a.a = 0 as *mut mm128_t;
    flag = 0 as libc::c_int;
    if n <= 0 as libc::c_int {
        return 0 as *mut mm_idx_t;
    }
    i = 0 as libc::c_int;
    while i < n {
        tmp = strlen(*seq.offset(i as isize));
        sum_len = (sum_len as libc::c_ulong).wrapping_add(tmp) as uint64_t as uint64_t;
        i += 1;
    }
    if is_hpc != 0 {
        flag |= 1 as libc::c_int;
    }
    if name as libc::c_ulong == 0 as *mut *const libc::c_char as libc::c_ulong {
        flag |= 4 as libc::c_int;
    }
    if bucket_bits < 0 as libc::c_int {
        bucket_bits = 14 as libc::c_int;
    }
    mi = mm_idx_init(w, k, bucket_bits, flag);
    (*mi).n_seq = n as uint32_t;
    tmp___0 = kcalloc(
        (*mi).km,
        n as size_t,
        ::std::mem::size_of::<mm_idx_seq_t>() as libc::c_ulong,
    );
    (*mi).seq = tmp___0 as *mut mm_idx_seq_t;
    tmp___1 = calloc(
        sum_len.wrapping_add(7 as libc::c_ulong).wrapping_div(8 as libc::c_ulong),
        4 as libc::c_int as size_t,
    );
    (*mi).S = tmp___1 as *mut uint32_t;
    h = kh_init_str();
    (*mi).h = h as *mut libc::c_void;
    i = 0 as libc::c_int;
    sum_len = 0 as libc::c_int as uint64_t;
    while i < n {
        s = *seq.offset(i as isize);
        p = ((*mi).seq).offset(i as isize);
        if !name.is_null() {
            if !(*name.offset(i as isize)).is_null() {
                tmp___2 = strlen(*name.offset(i as isize));
                tmp___3 = kmalloc((*mi).km, tmp___2.wrapping_add(1 as libc::c_ulong));
                (*p).name = tmp___3 as *mut libc::c_char;
                strcpy((*p).name, *name.offset(i as isize));
                kh_put_str(h, (*p).name as kh_cstr_t, &mut absent);
                if absent == 0 {
                    __assert_fail(
                        b"absent\0" as *const u8 as *const libc::c_char,
                        b"index.c\0" as *const u8 as *const libc::c_char,
                        436 as libc::c_uint,
                        b"mm_idx_str\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        (*p).offset = sum_len;
        tmp___5 = strlen(s);
        (*p).len = tmp___5 as uint32_t;
        (*p).is_alt = 0 as libc::c_int as uint32_t;
        j = 0 as libc::c_int as uint32_t;
        while j < (*p).len {
            c = seq_nt4_table[*s.offset(j as isize) as uint8_t as usize] as libc::c_int;
            o = sum_len.wrapping_add(j as uint64_t);
            let ref mut fresh28 = *((*mi).S).offset((o >> 3 as libc::c_int) as isize);
            *fresh28
                |= (c as uint32_t) << ((o & 7 as libc::c_ulong) << 2 as libc::c_int);
            j = j.wrapping_add(1);
        }
        sum_len = (sum_len as libc::c_ulong).wrapping_add((*p).len as uint64_t)
            as uint64_t as uint64_t;
        if (*p).len > 0 as libc::c_uint {
            a.n = 0 as libc::c_int as size_t;
            mm_sketch(
                0 as *mut libc::c_void,
                s,
                (*p).len as libc::c_int,
                w,
                k,
                i as uint32_t,
                is_hpc,
                &mut a,
            );
            mm_idx_add(mi, a.n as libc::c_int, a.a as *const mm128_t);
        }
        i += 1;
    }
    free(a.a as *mut libc::c_void);
    mm_idx_post(mi, 1 as libc::c_int);
    return mi;
}
pub unsafe extern "C" fn mm_idx_dump(mut fp: *mut FILE, mut mi: *const mm_idx_t) {
    let mut sum_len: uint64_t = 0;
    let mut x: [uint32_t; 5] = [0; 5];
    let mut i: uint32_t = 0;
    let mut l: uint8_t = 0;
    let mut tmp: size_t = 0;
    let mut l___0: uint8_t = 0;
    let mut b: *mut mm_idx_bucket_t = 0 as *mut mm_idx_bucket_t;
    let mut k: khint_t = 0;
    let mut h: *mut idxhash_t = 0 as *mut idxhash_t;
    let mut size: uint32_t = 0;
    let mut tmp___0: khint_t = 0;
    let mut x___0: [uint64_t; 2] = [0; 2];
    sum_len = 0 as libc::c_int as uint64_t;
    x[0 as libc::c_int as usize] = (*mi).w as uint32_t;
    x[1 as libc::c_int as usize] = (*mi).k as uint32_t;
    x[2 as libc::c_int as usize] = (*mi).b as uint32_t;
    x[3 as libc::c_int as usize] = (*mi).n_seq;
    x[4 as libc::c_int as usize] = (*mi).flag as uint32_t;
    fwrite(
        b"MMI\x02\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
        4 as libc::c_int as size_t,
        fp,
    );
    fwrite(
        x.as_mut_ptr() as *const libc::c_void,
        4 as libc::c_int as size_t,
        5 as libc::c_int as size_t,
        fp,
    );
    i = 0 as libc::c_int as uint32_t;
    while i < (*mi).n_seq {
        if !((*((*mi).seq).offset(i as isize)).name).is_null() {
            tmp = strlen((*((*mi).seq).offset(i as isize)).name as *const libc::c_char);
            l = tmp as uint8_t;
            fwrite(
                &mut l as *mut uint8_t as *const libc::c_void,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                fp,
            );
            fwrite(
                (*((*mi).seq).offset(i as isize)).name as *const libc::c_void,
                1 as libc::c_int as size_t,
                l as size_t,
                fp,
            );
        } else {
            l___0 = 0 as libc::c_int as uint8_t;
            fwrite(
                &mut l___0 as *mut uint8_t as *const libc::c_void,
                1 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                fp,
            );
        }
        fwrite(
            &mut (*((*mi).seq).offset(i as isize)).len as *mut uint32_t
                as *const libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            fp,
        );
        sum_len = (sum_len as libc::c_ulong)
            .wrapping_add((*((*mi).seq).offset(i as isize)).len as uint64_t) as uint64_t
            as uint64_t;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as uint32_t;
    while i < ((1 as libc::c_int) << (*mi).b) as uint32_t {
        b = ((*mi).B).offset(i as isize);
        h = (*b).h as *mut idxhash_t;
        if !h.is_null() {
            tmp___0 = (*h).size;
        } else {
            tmp___0 = 0 as libc::c_int as khint_t;
        }
        size = tmp___0;
        fwrite(
            &mut (*b).n as *mut int32_t as *const libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            fp,
        );
        fwrite(
            (*b).p as *const libc::c_void,
            8 as libc::c_int as size_t,
            (*b).n as size_t,
            fp,
        );
        fwrite(
            &mut size as *mut uint32_t as *const libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            fp,
        );
        if !(size == 0 as libc::c_uint) {
            k = 0 as libc::c_int as khint_t;
            while k < (*h).n_buckets {
                if !(*((*h).flags).offset((k >> 4 as libc::c_int) as isize)
                    >> ((k & 15 as libc::c_uint) << 1 as libc::c_int) & 3 as libc::c_uint
                    != 0)
                {
                    x___0[0 as libc::c_int as usize] = *((*h).keys).offset(k as isize);
                    x___0[1 as libc::c_int as usize] = *((*h).vals).offset(k as isize);
                    fwrite(
                        x___0.as_mut_ptr() as *const libc::c_void,
                        8 as libc::c_int as size_t,
                        2 as libc::c_int as size_t,
                        fp,
                    );
                }
                k = k.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    if (*mi).flag & 2 as libc::c_int == 0 {
        fwrite(
            (*mi).S as *const libc::c_void,
            4 as libc::c_int as size_t,
            sum_len.wrapping_add(7 as libc::c_ulong).wrapping_div(8 as libc::c_ulong),
            fp,
        );
    }
    fflush(fp);
}
pub unsafe extern "C" fn mm_idx_load(mut fp: *mut FILE) -> *mut mm_idx_t {
    let mut magic: [libc::c_char; 4] = [0; 4];
    let mut x: [uint32_t; 5] = [0; 5];
    let mut i: uint32_t = 0;
    let mut sum_len: uint64_t = 0;
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    let mut tmp: size_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut l: uint8_t = 0;
    let mut s: *mut mm_idx_seq_t = 0 as *mut mm_idx_seq_t;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut b: *mut mm_idx_bucket_t = 0 as *mut mm_idx_bucket_t;
    let mut j: uint32_t = 0;
    let mut size: uint32_t = 0;
    let mut k: khint_t = 0;
    let mut h: *mut idxhash_t = 0 as *mut idxhash_t;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut x___0: [uint64_t; 2] = [0; 2];
    let mut absent: libc::c_int = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    sum_len = 0 as libc::c_int as uint64_t;
    tmp = fread(
        magic.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as size_t,
        4 as libc::c_int as size_t,
        fp,
    );
    if tmp != 4 as libc::c_ulong {
        return 0 as *mut mm_idx_t;
    }
    tmp___0 = strncmp(
        magic.as_mut_ptr() as *const libc::c_char,
        b"MMI\x02\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    if tmp___0 != 0 as libc::c_int {
        return 0 as *mut mm_idx_t;
    }
    tmp___1 = fread(
        x.as_mut_ptr() as *mut libc::c_void,
        4 as libc::c_int as size_t,
        5 as libc::c_int as size_t,
        fp,
    );
    if tmp___1 != 5 as libc::c_ulong {
        return 0 as *mut mm_idx_t;
    }
    mi = mm_idx_init(
        x[0 as libc::c_int as usize] as libc::c_int,
        x[1 as libc::c_int as usize] as libc::c_int,
        x[2 as libc::c_int as usize] as libc::c_int,
        x[4 as libc::c_int as usize] as libc::c_int,
    );
    (*mi).n_seq = x[3 as libc::c_int as usize];
    tmp___2 = kcalloc(
        (*mi).km,
        (*mi).n_seq as size_t,
        ::std::mem::size_of::<mm_idx_seq_t>() as libc::c_ulong,
    );
    (*mi).seq = tmp___2 as *mut mm_idx_seq_t;
    i = 0 as libc::c_int as uint32_t;
    while i < (*mi).n_seq {
        s = ((*mi).seq).offset(i as isize);
        fread(
            &mut l as *mut uint8_t as *mut libc::c_void,
            1 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            fp,
        );
        if l != 0 {
            tmp___3 = kmalloc((*mi).km, (l as libc::c_int + 1 as libc::c_int) as size_t);
            (*s).name = tmp___3 as *mut libc::c_char;
            fread(
                (*s).name as *mut libc::c_void,
                1 as libc::c_int as size_t,
                l as size_t,
                fp,
            );
            *((*s).name)
                .offset(l as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        }
        fread(
            &mut (*s).len as *mut uint32_t as *mut libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            fp,
        );
        (*s).offset = sum_len;
        (*s).is_alt = 0 as libc::c_int as uint32_t;
        sum_len = (sum_len as libc::c_ulong).wrapping_add((*s).len as uint64_t)
            as uint64_t as uint64_t;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as uint32_t;
    while i < ((1 as libc::c_int) << (*mi).b) as uint32_t {
        b = ((*mi).B).offset(i as isize);
        fread(
            &mut (*b).n as *mut int32_t as *mut libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            fp,
        );
        tmp___4 = malloc(((*b).n * 8 as libc::c_int) as size_t);
        (*b).p = tmp___4 as *mut uint64_t;
        fread(
            (*b).p as *mut libc::c_void,
            8 as libc::c_int as size_t,
            (*b).n as size_t,
            fp,
        );
        fread(
            &mut size as *mut uint32_t as *mut libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            fp,
        );
        if !(size == 0 as libc::c_uint) {
            h = kh_init_idx();
            (*b).h = h as *mut libc::c_void;
            kh_resize_idx(h, size);
            j = 0 as libc::c_int as uint32_t;
            while j < size {
                fread(
                    x___0.as_mut_ptr() as *mut libc::c_void,
                    8 as libc::c_int as size_t,
                    2 as libc::c_int as size_t,
                    fp,
                );
                k = kh_put_idx(h, x___0[0 as libc::c_int as usize], &mut absent);
                if absent == 0 {
                    __assert_fail(
                        b"absent\0" as *const u8 as *const libc::c_char,
                        b"index.c\0" as *const u8 as *const libc::c_char,
                        547 as libc::c_uint,
                        b"mm_idx_load\0" as *const u8 as *const libc::c_char,
                    );
                }
                *((*h).vals).offset(k as isize) = x___0[1 as libc::c_int as usize];
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    if (*mi).flag & 2 as libc::c_int == 0 {
        tmp___6 = malloc(
            sum_len
                .wrapping_add(7 as libc::c_ulong)
                .wrapping_div(8 as libc::c_ulong)
                .wrapping_mul(4 as libc::c_ulong),
        );
        (*mi).S = tmp___6 as *mut uint32_t;
        fread(
            (*mi).S as *mut libc::c_void,
            4 as libc::c_int as size_t,
            sum_len.wrapping_add(7 as libc::c_ulong).wrapping_div(8 as libc::c_ulong),
            fp,
        );
    }
    return mi;
}
pub unsafe extern "C" fn mm_idx_is_idx(mut fn_0: *const libc::c_char) -> int64_t {
    let mut fd: libc::c_int = 0;
    let mut is_idx: libc::c_int = 0;
    let mut ret: int64_t = 0;
    let mut off_end: int64_t = 0;
    let mut magic: [libc::c_char; 4] = [0; 4];
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: int64_t = 0;
    is_idx = 0 as libc::c_int;
    tmp = strcmp(fn_0, b"-\0" as *const u8 as *const libc::c_char);
    if tmp == 0 as libc::c_int {
        return 0 as libc::c_int as int64_t;
    }
    fd = open(fn_0, 0 as libc::c_int);
    if fd < 0 as libc::c_int {
        return -(1 as libc::c_int) as int64_t;
    }
    off_end = lseek(fd, 0 as libc::c_int as __off_t, 2 as libc::c_int);
    if off_end >= 4 as libc::c_long {
        lseek(fd, 0 as libc::c_int as __off_t, 0 as libc::c_int);
        ret = read(
            fd,
            magic.as_mut_ptr() as *mut libc::c_void,
            4 as libc::c_int as size_t,
        );
        if ret == 4 as libc::c_long {
            tmp___0 = strncmp(
                magic.as_mut_ptr() as *const libc::c_char,
                b"MMI\x02\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as size_t,
            );
            if tmp___0 == 0 as libc::c_int {
                is_idx = 1 as libc::c_int;
            }
        }
    }
    close(fd);
    if is_idx != 0 {
        tmp___1 = off_end;
    } else {
        tmp___1 = 0 as libc::c_int as int64_t;
    }
    return tmp___1;
}
pub unsafe extern "C" fn mm_idx_reader_open(
    mut fn_0: *const libc::c_char,
    mut opt: *const mm_idxopt_t,
    mut fn_out: *const libc::c_char,
) -> *mut mm_idx_reader_t {
    let mut is_idx: int64_t = 0;
    let mut r: *mut mm_idx_reader_t = 0 as *mut mm_idx_reader_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    is_idx = mm_idx_is_idx(fn_0);
    if is_idx < 0 as libc::c_long {
        return 0 as *mut mm_idx_reader_t;
    }
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<mm_idx_reader_t>() as libc::c_ulong,
    );
    r = tmp as *mut mm_idx_reader_t;
    (*r).is_idx = is_idx as libc::c_int;
    if !opt.is_null() {
        (*r).opt = *opt;
    } else {
        mm_idxopt_init(&mut (*r).opt);
    }
    if (*r).is_idx != 0 {
        (*r).fp.idx = fopen(fn_0, b"rb\0" as *const u8 as *const libc::c_char);
        (*r).idx_size = is_idx;
    } else {
        (*r).fp.seq = mm_bseq_open(fn_0);
    }
    if !fn_out.is_null() {
        (*r).fp_out = fopen(fn_out, b"wb\0" as *const u8 as *const libc::c_char);
    }
    return r;
}
pub unsafe extern "C" fn mm_idx_reader_close(mut r: *mut mm_idx_reader_t) {
    if (*r).is_idx != 0 {
        fclose((*r).fp.idx);
    } else {
        mm_bseq_close((*r).fp.seq);
    }
    if !((*r).fp_out).is_null() {
        fclose((*r).fp_out);
    }
    free(r as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_idx_reader_read(
    mut r: *mut mm_idx_reader_t,
    mut n_threads: libc::c_int,
) -> *mut mm_idx_t {
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    let mut tmp: libc::c_int = 0;
    if (*r).is_idx != 0 {
        mi = mm_idx_load((*r).fp.idx);
        if !mi.is_null() {
            if mm_verbose >= 2 as libc::c_int {
                if (*mi).k != (*r).opt.k as int32_t {
                    fprintf(
                        stderr,
                        b"[WARNING]\x1B[1;31m Indexing parameters (-k, -w or -H) overridden by parameters used in the prebuilt index.\x1B[0m\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else if (*mi).w != (*r).opt.w as int32_t {
                    fprintf(
                        stderr,
                        b"[WARNING]\x1B[1;31m Indexing parameters (-k, -w or -H) overridden by parameters used in the prebuilt index.\x1B[0m\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                } else if (*mi).flag & 1 as libc::c_int
                        != (*r).opt.flag as libc::c_int & 1 as libc::c_int
                    {
                    fprintf(
                        stderr,
                        b"[WARNING]\x1B[1;31m Indexing parameters (-k, -w or -H) overridden by parameters used in the prebuilt index.\x1B[0m\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    } else {
        mi = mm_idx_gen(
            (*r).fp.seq,
            (*r).opt.w as libc::c_int,
            (*r).opt.k as libc::c_int,
            (*r).opt.bucket_bits as libc::c_int,
            (*r).opt.flag as libc::c_int,
            (*r).opt.mini_batch_size as libc::c_int,
            n_threads,
            (*r).opt.batch_size,
        );
    }
    if !mi.is_null() {
        if !((*r).fp_out).is_null() {
            mm_idx_dump((*r).fp_out, mi as *const mm_idx_t);
        }
        tmp = (*r).n_parts;
        (*r).n_parts += 1;
        (*mi).index = tmp;
    }
    return mi;
}
pub unsafe extern "C" fn mm_idx_reader_eof(
    mut r: *const mm_idx_reader_t,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_long = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    if (*r).is_idx != 0 {
        tmp = feof((*r).fp.idx);
        if tmp != 0 {
            tmp___1 = 1 as libc::c_int;
        } else {
            tmp___0 = ftell((*r).fp.idx);
            if tmp___0 == (*r).idx_size {
                tmp___1 = 1 as libc::c_int;
            } else {
                tmp___1 = 0 as libc::c_int;
            }
        }
        tmp___3 = tmp___1;
    } else {
        tmp___2 = mm_bseq_eof((*r).fp.seq as *mut mm_bseq_file_t);
        tmp___3 = tmp___2;
    }
    return tmp___3;
}
pub unsafe extern "C" fn mm_idx_alt_read(
    mut mi: *mut mm_idx_t,
    mut fn_0: *const libc::c_char,
) -> libc::c_int {
    let mut n_alt: libc::c_int = 0;
    let mut fp: gzFile = 0 as *mut gzFile_s;
    let mut ks: *mut kstream_t = 0 as *mut kstream_t;
    let mut str: kstring_t = kstring_t {
        l: 0,
        m: 0,
        s: 0 as *mut libc::c_char,
    };
    let mut tmp___0: gzFile = 0 as *mut gzFile_s;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: gzFile = 0 as *mut gzFile_s;
    let mut tmp___3: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut id: libc::c_int = 0;
    let mut tmp___4: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___5: libc::c_int = 0;
    n_alt = 0 as libc::c_int;
    str.l = 0 as libc::c_int as size_t;
    str.m = 0 as libc::c_int as size_t;
    str.s = 0 as *mut libc::c_char;
    if !fn_0.is_null() {
        tmp___3 = strcmp(fn_0, b"-\0" as *const u8 as *const libc::c_char);
        if tmp___3 != 0 {
            tmp___0 = gzopen(fn_0, b"r\0" as *const u8 as *const libc::c_char);
            fp = tmp___0;
        } else {
            tmp___1 = fileno(stdin);
            tmp___2 = gzdopen(tmp___1, b"r\0" as *const u8 as *const libc::c_char);
            fp = tmp___2;
        }
    } else {
        tmp___1 = fileno(stdin);
        tmp___2 = gzdopen(tmp___1, b"r\0" as *const u8 as *const libc::c_char);
        fp = tmp___2;
    }
    if fp as libc::c_ulong == 0 as gzFile as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    ks = ks_init(fp);
    if (*mi).h as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        mm_idx_index_name(mi);
    }
    loop {
        tmp___5 = ks_getuntil(ks, 2 as libc::c_int, &mut str, 0 as *mut libc::c_int);
        if !(tmp___5 >= 0 as libc::c_int) {
            break;
        }
        p = str.s;
        while *p != 0 {
            tmp___4 = __ctype_b_loc();
            if *(*tmp___4).offset(*p as libc::c_int as isize) as libc::c_int
                & 8192 as libc::c_int != 0
            {
                break;
            }
            p = p.offset(1);
        }
        *p = 0 as libc::c_int as libc::c_char;
        id = mm_idx_name2id(mi as *const mm_idx_t, str.s as *const libc::c_char);
        if id >= 0 as libc::c_int {
            (*((*mi).seq).offset(id as isize)).is_alt = 1 as libc::c_int as uint32_t;
            n_alt += 1;
        }
    }
    (*mi).n_alt = n_alt;
    if mm_verbose >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"[M::%s] found %d ALT contigs\n\0" as *const u8 as *const libc::c_char,
            b"mm_idx_alt_read\0" as *const u8 as *const libc::c_char,
            n_alt,
        );
    }
    return n_alt;
}
pub unsafe extern "C" fn rs_insertsort_bed(
    mut beg: *mut mm_idx_intv1_t,
    mut end: *mut mm_idx_intv1_t,
) {
    let mut i: *mut mm_idx_intv1_t = 0 as *mut mm_idx_intv1_t;
    let mut j: *mut mm_idx_intv1_t = 0 as *mut mm_idx_intv1_t;
    let mut tmp: mm_idx_intv1_t = mm_idx_intv1_t {
        st: 0,
        en: 0,
        max: 0,
        score_strand: [0; 4],
    };
    i = beg.offset(1 as libc::c_int as isize);
    while (i as libc::c_ulong) < end as libc::c_ulong {
        if (*i).st < (*i.offset(-(1 as libc::c_int as isize))).st {
            tmp = *i;
            j = i;
            while j as libc::c_ulong > beg as libc::c_ulong {
                if !(tmp.st < (*j.offset(-(1 as libc::c_int as isize))).st) {
                    break;
                }
                *j = *j.offset(-(1 as libc::c_int as isize));
                j = j.offset(-1);
            }
            *j = tmp;
        }
        i = i.offset(1);
    }
}
pub unsafe extern "C" fn rs_sort_bed(
    mut beg: *mut mm_idx_intv1_t,
    mut end: *mut mm_idx_intv1_t,
    mut n_bits: libc::c_int,
    mut s: libc::c_int,
) {
    let mut i: *mut mm_idx_intv1_t = 0 as *mut mm_idx_intv1_t;
    let mut size: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut k: *mut rsbucket_bed_t = 0 as *mut rsbucket_bed_t;
    let mut b: [rsbucket_bed_t; 256] = [rsbucket_bed_t {
        b: 0 as *mut mm_idx_intv1_t,
        e: 0 as *mut mm_idx_intv1_t,
    }; 256];
    let mut be: *mut rsbucket_bed_t = 0 as *mut rsbucket_bed_t;
    let mut tmp___0: *mut mm_idx_intv1_t = 0 as *mut mm_idx_intv1_t;
    let mut l: *mut rsbucket_bed_t = 0 as *mut rsbucket_bed_t;
    let mut tmp___1: mm_idx_intv1_t = mm_idx_intv1_t {
        st: 0,
        en: 0,
        max: 0,
        score_strand: [0; 4],
    };
    let mut swap: mm_idx_intv1_t = mm_idx_intv1_t {
        st: 0,
        en: 0,
        max: 0,
        score_strand: [0; 4],
    };
    let mut tmp___2: *mut mm_idx_intv1_t = 0 as *mut mm_idx_intv1_t;
    let mut tmp___3: *mut mm_idx_intv1_t = 0 as *mut mm_idx_intv1_t;
    size = (1 as libc::c_int) << n_bits;
    m = size - 1 as libc::c_int;
    be = b.as_mut_ptr().offset(size as isize);
    if !(n_bits <= 8 as libc::c_int) {
        __assert_fail(
            b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
            b"index.c\0" as *const u8 as *const libc::c_char,
            660 as libc::c_uint,
            b"rs_sort_bed\0" as *const u8 as *const libc::c_char,
        );
    }
    k = b.as_mut_ptr();
    while k as libc::c_ulong != be as libc::c_ulong {
        tmp___0 = beg;
        (*k).e = tmp___0;
        (*k).b = tmp___0;
        k = k.offset(1);
    }
    i = beg;
    while i as libc::c_ulong != end as libc::c_ulong {
        b[((*i).st >> s & m) as usize].e = (b[((*i).st >> s & m) as usize].e).offset(1);
        i = i.offset(1);
    }
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k as libc::c_ulong != be as libc::c_ulong {
        (*k)
            .e = ((*k).e)
            .offset(
                ((*k.offset(-(1 as libc::c_int as isize))).e).offset_from(beg)
                    as libc::c_long as isize,
            );
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
    }
    k = b.as_mut_ptr();
    while k as libc::c_ulong != be as libc::c_ulong {
        if (*k).b as libc::c_ulong != (*k).e as libc::c_ulong {
            l = b.as_mut_ptr().offset(((*(*k).b).st >> s & m) as isize);
            if l as libc::c_ulong != k as libc::c_ulong {
                tmp___1 = *(*k).b;
                loop {
                    swap = tmp___1;
                    tmp___1 = *(*l).b;
                    tmp___2 = (*l).b;
                    (*l).b = ((*l).b).offset(1);
                    *tmp___2 = swap;
                    l = b.as_mut_ptr().offset((tmp___1.st >> s & m) as isize);
                    if !(l as libc::c_ulong != k as libc::c_ulong) {
                        break;
                    }
                }
                tmp___3 = (*k).b;
                (*k).b = ((*k).b).offset(1);
                *tmp___3 = tmp___1;
            } else {
                (*k).b = ((*k).b).offset(1);
            }
        } else {
            k = k.offset(1);
        }
    }
    b[0 as libc::c_int as usize].b = beg;
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k as libc::c_ulong != be as libc::c_ulong {
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
    }
    if s != 0 {
        if s > n_bits {
            s -= n_bits;
        } else {
            s = 0 as libc::c_int;
        }
        k = b.as_mut_ptr();
        while k as libc::c_ulong != be as libc::c_ulong {
            if ((*k).e).offset_from((*k).b) as libc::c_long > 64 as libc::c_long {
                rs_sort_bed((*k).b, (*k).e, n_bits, s);
            } else if ((*k).e).offset_from((*k).b) as libc::c_long > 1 as libc::c_long {
                rs_insertsort_bed((*k).b, (*k).e);
            }
            k = k.offset(1);
        }
    }
}
pub unsafe extern "C" fn radix_sort_bed(
    mut beg: *mut mm_idx_intv1_t,
    mut end: *mut mm_idx_intv1_t,
) {
    if end.offset_from(beg) as libc::c_long <= 64 as libc::c_long {
        rs_insertsort_bed(beg, end);
    } else {
        rs_sort_bed(beg, end, 8 as libc::c_int, 24 as libc::c_int);
    };
}
pub unsafe extern "C" fn mm_idx_read_bed(
    mut mi: *const mm_idx_t,
    mut fn_0: *const libc::c_char,
    mut read_junc: libc::c_int,
) -> *mut mm_idx_intv_t {
    let mut fp: gzFile = 0 as *mut gzFile_s;
    let mut ks: *mut kstream_t = 0 as *mut kstream_t;
    let mut str: kstring_t = kstring_t {
        l: 0,
        m: 0,
        s: 0 as *mut libc::c_char,
    };
    let mut I: *mut mm_idx_intv_t = 0 as *mut mm_idx_intv_t;
    let mut tmp___0: gzFile = 0 as *mut gzFile_s;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: gzFile = 0 as *mut gzFile_s;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r: *mut mm_idx_intv_t = 0 as *mut mm_idx_intv_t;
    let mut t: mm_idx_intv1_t = mm_idx_intv1_t {
        st: 0,
        en: 0,
        max: 0,
        score_strand: [0; 4],
    };
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: int32_t = 0;
    let mut id: int32_t = 0;
    let mut n_blk: int32_t = 0;
    let mut c: int32_t = 0;
    let mut tmp___5: libc::c_long = 0;
    let mut tmp___6: libc::c_long = 0;
    let mut tmp___7: libc::c_long = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: *mut *const libc::c_ushort = 0 as *mut *const libc::c_ushort;
    let mut tmp___10: libc::c_long = 0;
    let mut st: int32_t = 0;
    let mut sz: int32_t = 0;
    let mut en: int32_t = 0;
    let mut tmp___11: libc::c_long = 0;
    let mut tmp___12: libc::c_long = 0;
    let mut s: mm_idx_intv1_t = mm_idx_intv1_t {
        st: 0,
        en: 0,
        max: 0,
        score_strand: [0; 4],
    };
    let mut tmp___13: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___14: libc::c_long = 0;
    let mut tmp___15: libc::c_long = 0;
    let mut tmp___16: int32_t = 0;
    let mut tmp___17: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___18: int32_t = 0;
    let mut tmp___19: libc::c_int = 0;
    str.l = 0 as libc::c_int as size_t;
    str.m = 0 as libc::c_int as size_t;
    str.s = 0 as *mut libc::c_char;
    if !fn_0.is_null() {
        tmp___3 = strcmp(fn_0, b"-\0" as *const u8 as *const libc::c_char);
        if tmp___3 != 0 {
            tmp___0 = gzopen(fn_0, b"r\0" as *const u8 as *const libc::c_char);
            fp = tmp___0;
        } else {
            tmp___1 = fileno(stdin);
            tmp___2 = gzdopen(tmp___1, b"r\0" as *const u8 as *const libc::c_char);
            fp = tmp___2;
        }
    } else {
        tmp___1 = fileno(stdin);
        tmp___2 = gzdopen(tmp___1, b"r\0" as *const u8 as *const libc::c_char);
        fp = tmp___2;
    }
    if fp as libc::c_ulong == 0 as gzFile as libc::c_ulong {
        return 0 as *mut mm_idx_intv_t;
    }
    tmp___4 = calloc(
        (*mi).n_seq as size_t,
        ::std::mem::size_of::<mm_idx_intv_t>() as libc::c_ulong,
    );
    I = tmp___4 as *mut mm_idx_intv_t;
    ks = ks_init(fp);
    loop {
        tmp___19 = ks_getuntil(ks, 2 as libc::c_int, &mut str, 0 as *mut libc::c_int);
        if !(tmp___19 >= 0 as libc::c_int) {
            break;
        }
        t.st = -(1 as libc::c_int);
        t.en = -(1 as libc::c_int);
        t.max = -(1 as libc::c_int);
        t.set_score(-(1 as libc::c_int));
        t.set_strand(0 as libc::c_int);
        id = -(1 as libc::c_int);
        n_blk = 0 as libc::c_int;
        q = str.s;
        p = q;
        i = 0 as libc::c_int;
        let mut current_block_64: u64;
        loop {
            if *p as libc::c_int == 0 as libc::c_int {
                current_block_64 = 4760236877734429204;
            } else if *p as libc::c_int == 9 as libc::c_int {
                current_block_64 = 4760236877734429204;
            } else {
                current_block_64 = 54079586644752974;
            }
            match current_block_64 {
                4760236877734429204 => {
                    c = *p as int32_t;
                    *p = 0 as libc::c_int as libc::c_char;
                    if i == 0 as libc::c_int {
                        id = mm_idx_name2id(mi, q as *const libc::c_char);
                        if id < 0 as libc::c_int {
                            break;
                        }
                    } else if i == 1 as libc::c_int {
                        tmp___5 = atol(q as *const libc::c_char);
                        t.st = tmp___5 as int32_t;
                        if t.st < 0 as libc::c_int {
                            break;
                        }
                    } else if i == 2 as libc::c_int {
                        tmp___6 = atol(q as *const libc::c_char);
                        t.en = tmp___6 as int32_t;
                        if t.en < 0 as libc::c_int {
                            break;
                        }
                    } else if i == 4 as libc::c_int {
                        tmp___7 = atol(q as *const libc::c_char);
                        t.set_score(tmp___7 as int32_t);
                    } else if i == 5 as libc::c_int {
                        if *q as libc::c_int == 43 as libc::c_int {
                            t.set_strand(1 as libc::c_int);
                        } else {
                            if *q as libc::c_int == 45 as libc::c_int {
                                tmp___8 = -(1 as libc::c_int);
                            } else {
                                tmp___8 = 0 as libc::c_int;
                            }
                            t.set_strand(tmp___8);
                        }
                    } else if i == 9 as libc::c_int {
                        tmp___9 = __ctype_b_loc();
                        if *(*tmp___9).offset(*q as libc::c_int as isize) as libc::c_int
                            & 2048 as libc::c_int == 0
                        {
                            break;
                        }
                        tmp___10 = atol(q as *const libc::c_char);
                        n_blk = tmp___10 as int32_t;
                    } else if i == 10 as libc::c_int {
                        bl = q;
                    } else if i == 11 as libc::c_int {
                        bs = q;
                        break;
                    }
                    if c == 0 as libc::c_int {
                        break;
                    }
                    i += 1;
                    q = p.offset(1 as libc::c_int as isize);
                }
                _ => {}
            }
            p = p.offset(1);
        }
        if id < 0 as libc::c_int {
            continue;
        }
        if t.st < 0 as libc::c_int {
            continue;
        }
        if t.st >= t.en {
            continue;
        }
        r = I.offset(id as isize);
        let mut current_block_114: u64;
        if i >= 11 as libc::c_int {
            if read_junc != 0 {
                tmp___11 = strtol(
                    bs as *const libc::c_char,
                    &mut bs as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
                st = tmp___11 as int32_t;
                bs = bs.offset(1);
                tmp___12 = strtol(
                    bl as *const libc::c_char,
                    &mut bl as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
                sz = tmp___12 as int32_t;
                bl = bl.offset(1);
                en = t.st + st + sz;
                i = 1 as libc::c_int;
                while i < n_blk {
                    s = t;
                    if (*r).n == (*r).m {
                        if (*r).m != 0 {
                            (*r).m += (*r).m >> 1 as libc::c_int;
                        } else {
                            (*r).m = 16 as libc::c_int;
                        }
                        tmp___13 = realloc(
                            (*r).a as *mut libc::c_void,
                            (::std::mem::size_of::<mm_idx_intv1_t>() as libc::c_ulong)
                                .wrapping_mul((*r).m as libc::c_ulong),
                        );
                        (*r).a = tmp___13 as *mut mm_idx_intv1_t;
                    }
                    tmp___14 = strtol(
                        bs as *const libc::c_char,
                        &mut bs as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    st = tmp___14 as int32_t;
                    bs = bs.offset(1);
                    tmp___15 = strtol(
                        bl as *const libc::c_char,
                        &mut bl as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    sz = tmp___15 as int32_t;
                    bl = bl.offset(1);
                    s.st = en;
                    s.en = t.st + st;
                    en = t.st + st + sz;
                    if s.en > s.st {
                        tmp___16 = (*r).n;
                        (*r).n += 1;
                        *((*r).a).offset(tmp___16 as isize) = s;
                    }
                    i += 1;
                }
                current_block_114 = 6897179874198677617;
            } else {
                current_block_114 = 2712736381204768476;
            }
        } else {
            current_block_114 = 2712736381204768476;
        }
        match current_block_114 {
            2712736381204768476 => {
                if (*r).n == (*r).m {
                    if (*r).m != 0 {
                        (*r).m += (*r).m >> 1 as libc::c_int;
                    } else {
                        (*r).m = 16 as libc::c_int;
                    }
                    tmp___17 = realloc(
                        (*r).a as *mut libc::c_void,
                        (::std::mem::size_of::<mm_idx_intv1_t>() as libc::c_ulong)
                            .wrapping_mul((*r).m as libc::c_ulong),
                    );
                    (*r).a = tmp___17 as *mut mm_idx_intv1_t;
                }
                tmp___18 = (*r).n;
                (*r).n += 1;
                *((*r).a).offset(tmp___18 as isize) = t;
            }
            _ => {}
        }
    }
    free(str.s as *mut libc::c_void);
    ks_destroy(ks);
    gzclose(fp);
    return I;
}
pub unsafe extern "C" fn mm_idx_bed_read(
    mut mi: *mut mm_idx_t,
    mut fn_0: *const libc::c_char,
    mut read_junc: libc::c_int,
) -> libc::c_int {
    let mut i: int32_t = 0;
    if (*mi).h as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        mm_idx_index_name(mi);
    }
    (*mi).I = mm_idx_read_bed(mi as *const mm_idx_t, fn_0, read_junc);
    if (*mi).I as libc::c_ulong == 0 as *mut mm_idx_intv_s as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while (i as uint32_t) < (*mi).n_seq {
        radix_sort_bed(
            (*((*mi).I).offset(i as isize)).a,
            ((*((*mi).I).offset(i as isize)).a)
                .offset((*((*mi).I).offset(i as isize)).n as isize),
        );
        i += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mm_idx_bed_junc(
    mut mi: *const mm_idx_t,
    mut ctg: int32_t,
    mut st: int32_t,
    mut en: int32_t,
    mut s: *mut uint8_t,
) -> libc::c_int {
    let mut i: int32_t = 0;
    let mut left: int32_t = 0;
    let mut right: int32_t = 0;
    let mut r: *mut mm_idx_intv_t = 0 as *mut mm_idx_intv_t;
    let mut mid: int32_t = 0;
    memset(s as *mut libc::c_void, 0 as libc::c_int, (en - st) as size_t);
    if (*mi).I as libc::c_ulong == 0 as *mut mm_idx_intv_s as libc::c_ulong {
        return -(1 as libc::c_int)
    } else {
        if ctg < 0 as libc::c_int {
            return -(1 as libc::c_int)
        } else {
            if ctg as uint32_t >= (*mi).n_seq {
                return -(1 as libc::c_int);
            }
        }
    }
    r = ((*mi).I).offset(ctg as isize);
    left = 0 as libc::c_int;
    right = (*r).n;
    while right > left {
        mid = left + (right - left >> 1 as libc::c_int);
        if (*((*r).a).offset(mid as isize)).st >= st {
            right = mid;
        } else {
            left = mid + 1 as libc::c_int;
        }
    }
    i = left;
    while i < (*r).n {
        if st <= (*((*r).a).offset(i as isize)).st {
            if en >= (*((*r).a).offset(i as isize)).en {
                if (*((*r).a).offset(i as isize)).strand() != 0 as libc::c_int {
                    if (*((*r).a).offset(i as isize)).strand() > 0 as libc::c_int {
                        *s
                            .offset(
                                ((*((*r).a).offset(i as isize)).st - st) as isize,
                            ) = (*s
                            .offset(((*((*r).a).offset(i as isize)).st - st) as isize)
                            as libc::c_int | 1 as libc::c_int) as uint8_t;
                        *s
                            .offset(
                                ((*((*r).a).offset(i as isize)).en - 1 as libc::c_int - st)
                                    as isize,
                            ) = (*s
                            .offset(
                                ((*((*r).a).offset(i as isize)).en - 1 as libc::c_int - st)
                                    as isize,
                            ) as libc::c_int | 2 as libc::c_int) as uint8_t;
                    } else {
                        *s
                            .offset(
                                ((*((*r).a).offset(i as isize)).st - st) as isize,
                            ) = (*s
                            .offset(((*((*r).a).offset(i as isize)).st - st) as isize)
                            as libc::c_int | 8 as libc::c_int) as uint8_t;
                        *s
                            .offset(
                                ((*((*r).a).offset(i as isize)).en - 1 as libc::c_int - st)
                                    as isize,
                            ) = (*s
                            .offset(
                                ((*((*r).a).offset(i as isize)).en - 1 as libc::c_int - st)
                                    as isize,
                            ) as libc::c_int | 4 as libc::c_int) as uint8_t;
                    }
                }
            }
        }
        i += 1;
    }
    return left;
}
unsafe extern "C" fn panic(mut s: *const libc::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, s);
    abort();
}
pub unsafe extern "C" fn km_init2(
    mut km_par: *mut libc::c_void,
    mut min_core_size: size_t,
) -> *mut libc::c_void {
    let mut km: *mut kmem_t = 0 as *mut kmem_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = kcalloc(
        km_par,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kmem_t>() as libc::c_ulong,
    );
    km = tmp as *mut kmem_t;
    (*km).par = km_par;
    if min_core_size > 0 as libc::c_ulong {
        (*km).min_core_size = min_core_size;
    } else {
        (*km).min_core_size = 524288 as libc::c_int as size_t;
    }
    return km as *mut libc::c_void;
}
pub unsafe extern "C" fn km_init() -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = km_init2(0 as *mut libc::c_void, 0 as libc::c_int as size_t);
    return tmp;
}
pub unsafe extern "C" fn km_destroy(mut _km: *mut libc::c_void) {
    let mut km: *mut kmem_t = 0 as *mut kmem_t;
    let mut km_par: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut p: *mut header_t = 0 as *mut header_t;
    let mut q: *mut header_t = 0 as *mut header_t;
    km = _km as *mut kmem_t;
    if km as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    km_par = (*km).par;
    p = (*km).core_head;
    while p as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        q = (*p).ptr;
        kfree(km_par, p as *mut libc::c_void);
        p = q;
    }
    kfree(km_par, km as *mut libc::c_void);
}
unsafe extern "C" fn morecore(mut km: *mut kmem_t, mut nu: size_t) -> *mut header_t {
    let mut q: *mut header_t = 0 as *mut header_t;
    let mut bytes: size_t = 0;
    let mut p: *mut size_t = 0 as *mut size_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    nu = nu
        .wrapping_add(1 as libc::c_ulong)
        .wrapping_add(((*km).min_core_size).wrapping_sub(1 as libc::c_ulong))
        .wrapping_div((*km).min_core_size)
        .wrapping_mul((*km).min_core_size);
    bytes = nu.wrapping_mul(::std::mem::size_of::<header_t>() as libc::c_ulong);
    tmp = kmalloc((*km).par, bytes);
    q = tmp as *mut header_t;
    if q.is_null() {
        panic(b"[morecore] insufficient memory\0" as *const u8 as *const libc::c_char);
    }
    (*q).ptr = (*km).core_head;
    (*q).size = nu;
    (*km).core_head = q;
    p = q.offset(1 as libc::c_int as isize) as *mut size_t;
    *p = nu.wrapping_sub(1 as libc::c_ulong);
    kfree(
        km as *mut libc::c_void,
        p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
    );
    return (*km).loop_head;
}
pub unsafe extern "C" fn kfree(mut _km: *mut libc::c_void, mut ap: *mut libc::c_void) {
    let mut p: *mut header_t = 0 as *mut header_t;
    let mut q: *mut header_t = 0 as *mut header_t;
    let mut km: *mut kmem_t = 0 as *mut kmem_t;
    km = _km as *mut kmem_t;
    if ap.is_null() {
        return;
    }
    if km as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(ap);
        return;
    }
    p = (ap as *mut size_t).offset(-(1 as libc::c_int as isize)) as *mut header_t;
    (*p).size = *(ap as *mut size_t).offset(-(1 as libc::c_int as isize));
    q = (*km).loop_head;
    loop {
        if p as libc::c_ulong > q as libc::c_ulong {
            if (p as libc::c_ulong) < (*q).ptr as libc::c_ulong {
                break;
            }
        }
        if q as libc::c_ulong >= (*q).ptr as libc::c_ulong {
            if p as libc::c_ulong > q as libc::c_ulong {
                break;
            }
            if (p as libc::c_ulong) < (*q).ptr as libc::c_ulong {
                break;
            }
        }
        q = (*q).ptr;
    }
    if p.offset((*p).size as isize) as libc::c_ulong == (*q).ptr as libc::c_ulong {
        (*p)
            .size = ((*p).size as libc::c_ulong).wrapping_add((*(*q).ptr).size) as size_t
            as size_t;
        (*p).ptr = (*(*q).ptr).ptr;
    } else if p.offset((*p).size as isize) as libc::c_ulong > (*q).ptr as libc::c_ulong {
        if (*q).ptr as libc::c_ulong >= p as libc::c_ulong {
            panic(
                b"[kfree] The end of the allocated block enters a free block.\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            (*p).ptr = (*q).ptr;
        }
    } else {
        (*p).ptr = (*q).ptr;
    }
    if q.offset((*q).size as isize) as libc::c_ulong == p as libc::c_ulong {
        (*q)
            .size = ((*q).size as libc::c_ulong).wrapping_add((*p).size) as size_t
            as size_t;
        (*q).ptr = (*p).ptr;
        (*km).loop_head = q;
    } else if q.offset((*q).size as isize) as libc::c_ulong > p as libc::c_ulong {
        if p as libc::c_ulong >= q as libc::c_ulong {
            panic(
                b"[kfree] The end of a free block enters the allocated block.\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            (*km).loop_head = p;
            (*q).ptr = p;
        }
    } else {
        (*km).loop_head = p;
        (*q).ptr = p;
    };
}
pub unsafe extern "C" fn kmalloc(
    mut _km: *mut libc::c_void,
    mut n_bytes: size_t,
) -> *mut libc::c_void {
    let mut km: *mut kmem_t = 0 as *mut kmem_t;
    let mut n_units: size_t = 0;
    let mut p: *mut header_t = 0 as *mut header_t;
    let mut q: *mut header_t = 0 as *mut header_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut header_t = 0 as *mut header_t;
    let mut tmp___1: *mut header_t = 0 as *mut header_t;
    km = _km as *mut kmem_t;
    if n_bytes == 0 as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    if km as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = malloc(n_bytes);
        return tmp;
    }
    n_units = n_bytes
        .wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<header_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<header_t>() as libc::c_ulong);
    q = (*km).loop_head;
    if q.is_null() {
        tmp___1 = &mut (*km).base;
        (*km).base.ptr = tmp___1;
        tmp___0 = tmp___1;
        (*km).loop_head = tmp___0;
        q = tmp___0;
    }
    p = (*q).ptr;
    loop {
        if (*p).size >= n_units {
            if (*p).size == n_units {
                (*q).ptr = (*p).ptr;
            } else {
                (*p)
                    .size = ((*p).size as libc::c_ulong).wrapping_sub(n_units) as size_t
                    as size_t;
                p = p.offset((*p).size as isize);
                *(p as *mut size_t) = n_units;
            }
            (*km).loop_head = q;
            return (p as *mut size_t).offset(1 as libc::c_int as isize)
                as *mut libc::c_void;
        }
        if p as libc::c_ulong == (*km).loop_head as libc::c_ulong {
            p = morecore(km, n_units);
            if p as libc::c_ulong == 0 as *mut header_t as libc::c_ulong {
                return 0 as *mut libc::c_void;
            }
        }
        q = p;
        p = (*p).ptr;
    };
}
pub unsafe extern "C" fn kcalloc(
    mut _km: *mut libc::c_void,
    mut count: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut km: *mut kmem_t = 0 as *mut kmem_t;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    km = _km as *mut kmem_t;
    if size == 0 as libc::c_ulong {
        return 0 as *mut libc::c_void
    } else {
        if count == 0 as libc::c_ulong {
            return 0 as *mut libc::c_void;
        }
    }
    if km as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = calloc(count, size);
        return tmp;
    }
    p = kmalloc(km as *mut libc::c_void, count.wrapping_mul(size));
    memset(p, 0 as libc::c_int, count.wrapping_mul(size));
    return p;
}
pub unsafe extern "C" fn krealloc(
    mut _km: *mut libc::c_void,
    mut ap: *mut libc::c_void,
    mut n_bytes: size_t,
) -> *mut libc::c_void {
    let mut km: *mut kmem_t = 0 as *mut kmem_t;
    let mut cap: size_t = 0;
    let mut p: *mut size_t = 0 as *mut size_t;
    let mut q: *mut size_t = 0 as *mut size_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    km = _km as *mut kmem_t;
    if n_bytes == 0 as libc::c_ulong {
        kfree(km as *mut libc::c_void, ap);
        return 0 as *mut libc::c_void;
    }
    if km as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp = realloc(ap, n_bytes);
        return tmp;
    }
    if ap as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = kmalloc(km as *mut libc::c_void, n_bytes);
        return tmp___0;
    }
    p = (ap as *mut size_t).offset(-(1 as libc::c_int as isize));
    cap = (*p)
        .wrapping_mul(::std::mem::size_of::<header_t>() as libc::c_ulong)
        .wrapping_sub(::std::mem::size_of::<size_t>() as libc::c_ulong);
    if cap >= n_bytes {
        return ap;
    }
    tmp___1 = kmalloc(km as *mut libc::c_void, n_bytes);
    q = tmp___1 as *mut size_t;
    memcpy(q as *mut libc::c_void, ap as *const libc::c_void, cap);
    kfree(km as *mut libc::c_void, ap);
    return q as *mut libc::c_void;
}
pub unsafe extern "C" fn km_stat(mut _km: *const libc::c_void, mut s: *mut km_stat_t) {
    let mut km: *mut kmem_t = 0 as *mut kmem_t;
    let mut p: *mut header_t = 0 as *mut header_t;
    let mut size: size_t = 0;
    km = _km as *mut kmem_t;
    memset(
        s as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<km_stat_t>() as libc::c_ulong,
    );
    if km as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return
    } else {
        if (*km).loop_head as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return;
        }
    }
    p = (*km).loop_head;
    loop {
        (*s)
            .available = ((*s).available as libc::c_ulong)
            .wrapping_add(
                ((*p).size)
                    .wrapping_mul(::std::mem::size_of::<header_t>() as libc::c_ulong),
            ) as size_t as size_t;
        if (*p).size != 0 as libc::c_ulong {
            (*s).n_blocks = ((*s).n_blocks).wrapping_add(1);
        }
        if (*p).ptr as libc::c_ulong > p as libc::c_ulong {
            if p.offset((*p).size as isize) as libc::c_ulong > (*p).ptr as libc::c_ulong
            {
                panic(
                    b"[km_stat] The end of a free block enters another free block.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        if (*p).ptr as libc::c_ulong == (*km).loop_head as libc::c_ulong {
            break;
        }
        p = (*p).ptr;
    }
    p = (*km).core_head;
    while p as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        size = ((*p).size)
            .wrapping_mul(::std::mem::size_of::<header_t>() as libc::c_ulong);
        (*s).n_cores = ((*s).n_cores).wrapping_add(1);
        (*s)
            .capacity = ((*s).capacity as libc::c_ulong).wrapping_add(size) as size_t
            as size_t;
        if (*s).largest > size {
            (*s).largest = (*s).largest;
        } else {
            (*s).largest = size;
        }
        p = (*p).ptr;
    }
}
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
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut j: libc::c_long = 0;
    if n_threads > 1 as libc::c_int {
        t.func = func;
        t.data = data;
        t.n_threads = n_threads;
        t.n = n;
        tmp = calloc(
            n_threads as size_t,
            ::std::mem::size_of::<ktf_worker_t>() as libc::c_ulong,
        );
        t.w = tmp as *mut ktf_worker_t;
        tmp___0 = calloc(
            n_threads as size_t,
            ::std::mem::size_of::<pthread_t>() as libc::c_ulong,
        );
        tid = tmp___0 as *mut pthread_t;
        i = 0 as libc::c_int;
        while i < n_threads {
            let ref mut fresh29 = (*(t.w).offset(i as isize)).t;
            *fresh29 = &mut t;
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
        free(tid as *mut libc::c_void);
        free(t.w as *mut libc::c_void);
    } else {
        j = 0 as libc::c_long;
        while j < n {
            (Some(func.expect("non-null function pointer")))
                .expect("non-null function pointer")(data, j, 0 as libc::c_int);
            j += 1;
        }
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
                __annonCompField1: __anonunion____missing_field_name_465919411 {
                    __wseq: 0,
                },
                __annonCompField2: __anonunion____missing_field_name_717608300 {
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
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut w: *mut ktp_worker_t = 0 as *mut ktp_worker_t;
    let mut tmp___0: int64_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
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
    tmp = calloc(
        n_threads as size_t,
        ::std::mem::size_of::<ktp_worker_t>() as libc::c_ulong,
    );
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
    tmp___1 = calloc(
        n_threads as size_t,
        ::std::mem::size_of::<pthread_t>() as libc::c_ulong,
    );
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
    free(tid as *mut libc::c_void);
    free(aux.workers as *mut libc::c_void);
    pthread_mutex_destroy(&mut aux.mutex);
    pthread_cond_destroy(&mut aux.cv);
}
#[inline]
unsafe extern "C" fn mg_log2___0(mut x: libc::c_float) -> libc::c_float {
    let mut z: __anonunion_z_307641061___0 = __anonunion_z_307641061___0 {
        f: 0.,
    };
    let mut log_2: libc::c_float = 0.;
    z.f = x;
    log_2 = (z.i >> 23 as libc::c_int & 255 as libc::c_uint)
        .wrapping_sub(128 as libc::c_uint) as libc::c_float;
    z.i &= !((255 as libc::c_int) << 23 as libc::c_int) as libc::c_uint;
    z
        .i = (z.i as libc::c_uint)
        .wrapping_add(((127 as libc::c_int) << 23 as libc::c_int) as uint32_t)
        as uint32_t as uint32_t;
    log_2 += (-0.34484843f32 * z.f + 2.02466578f32) * z.f - 0.67487759f32;
    return log_2;
}
unsafe extern "C" fn mg_chain_bk_end(
    mut max_drop: int32_t,
    mut z: *const mm128_t,
    mut f: *const int32_t,
    mut p: *const int64_t,
    mut t: *mut int32_t,
    mut k: int64_t,
) -> int64_t {
    let mut i: int64_t = 0;
    let mut end_i: int64_t = 0;
    let mut max_i: int64_t = 0;
    let mut max_s: int32_t = 0;
    let mut s: int32_t = 0;
    i = (*z.offset(k as isize)).y as int64_t;
    end_i = -(1 as libc::c_int) as int64_t;
    max_i = i;
    max_s = 0 as libc::c_int;
    if i < 0 as libc::c_long {
        return i
    } else {
        if *t.offset(i as isize) != 0 as libc::c_int {
            return i;
        }
    }
    loop {
        *t.offset(i as isize) = 2 as libc::c_int;
        i = *p.offset(i as isize);
        end_i = i;
        if i < 0 as libc::c_long {
            s = (*z.offset(k as isize)).x as int32_t;
        } else {
            s = (*z.offset(k as isize)).x as int32_t - *f.offset(i as isize);
        }
        if s > max_s {
            max_s = s;
            max_i = i;
        } else if max_s - s > max_drop {
            break;
        }
        if !(i >= 0 as libc::c_long) {
            break;
        }
        if !(*t.offset(i as isize) == 0 as libc::c_int) {
            break;
        }
    }
    i = (*z.offset(k as isize)).y as int64_t;
    while i >= 0 as libc::c_long {
        if !(i != end_i) {
            break;
        }
        *t.offset(i as isize) = 0 as libc::c_int;
        i = *p.offset(i as isize);
    }
    return max_i;
}
pub unsafe extern "C" fn mg_chain_backtrack(
    mut km: *mut libc::c_void,
    mut n: int64_t,
    mut f: *const int32_t,
    mut p: *const int64_t,
    mut v: *mut int32_t,
    mut t: *mut int32_t,
    mut min_cnt: int32_t,
    mut min_sc: int32_t,
    mut max_drop: int32_t,
    mut n_u_: *mut int32_t,
    mut n_v_: *mut int32_t,
) -> *mut uint64_t {
    let mut z: *mut mm128_t = 0 as *mut mm128_t;
    let mut u: *mut uint64_t = 0 as *mut uint64_t;
    let mut i: int64_t = 0;
    let mut k: int64_t = 0;
    let mut n_z: int64_t = 0;
    let mut n_v: int64_t = 0;
    let mut n_u: int32_t = 0;
    let mut tmp: int32_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: int64_t = 0;
    let mut n_v0: int64_t = 0;
    let mut end_i: int64_t = 0;
    let mut sc: int32_t = 0;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut n_v0___0: int64_t = 0;
    let mut end_i___0: int64_t = 0;
    let mut sc___0: int32_t = 0;
    let mut tmp___3: int64_t = 0;
    let mut tmp___4: int32_t = 0;
    tmp = 0 as libc::c_int;
    *n_v_ = tmp;
    *n_u_ = tmp;
    i = 0 as libc::c_int as int64_t;
    n_z = 0 as libc::c_int as int64_t;
    while i < n {
        if *f.offset(i as isize) >= min_sc {
            n_z += 1;
        }
        i += 1;
    }
    if n_z == 0 as libc::c_long {
        return 0 as *mut uint64_t;
    }
    tmp___0 = kmalloc(
        km,
        (n_z as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    );
    z = tmp___0 as *mut mm128_t;
    i = 0 as libc::c_int as int64_t;
    k = 0 as libc::c_int as int64_t;
    while i < n {
        if *f.offset(i as isize) >= min_sc {
            (*z.offset(k as isize)).x = *f.offset(i as isize) as uint64_t;
            tmp___1 = k;
            k += 1;
            (*z.offset(tmp___1 as isize)).y = i as uint64_t;
        }
        i += 1;
    }
    radix_sort_128x(z, z.offset(n_z as isize));
    memset(t as *mut libc::c_void, 0 as libc::c_int, (n * 4 as libc::c_long) as size_t);
    k = n_z - 1 as libc::c_long;
    n_u = 0 as libc::c_int;
    n_v = n_u as int64_t;
    while k >= 0 as libc::c_long {
        if *t.offset((*z.offset(k as isize)).y as isize) == 0 as libc::c_int {
            n_v0 = n_v;
            end_i = mg_chain_bk_end(max_drop, z as *const mm128_t, f, p, t, k);
            i = (*z.offset(k as isize)).y as int64_t;
            while i != end_i {
                n_v += 1;
                *t.offset(i as isize) = 1 as libc::c_int;
                i = *p.offset(i as isize);
            }
            if i < 0 as libc::c_long {
                sc = (*z.offset(k as isize)).x as int32_t;
            } else {
                sc = (*z.offset(k as isize)).x as int32_t - *f.offset(i as isize);
            }
            if sc >= min_sc {
                if n_v > n_v0 {
                    if n_v - n_v0 >= min_cnt as int64_t {
                        n_u += 1;
                    } else {
                        n_v = n_v0;
                    }
                } else {
                    n_v = n_v0;
                }
            } else {
                n_v = n_v0;
            }
        }
        k -= 1;
    }
    tmp___2 = kmalloc(
        km,
        (n_u as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
    );
    u = tmp___2 as *mut uint64_t;
    memset(t as *mut libc::c_void, 0 as libc::c_int, (n * 4 as libc::c_long) as size_t);
    k = n_z - 1 as libc::c_long;
    n_u = 0 as libc::c_int;
    n_v = n_u as int64_t;
    while k >= 0 as libc::c_long {
        if *t.offset((*z.offset(k as isize)).y as isize) == 0 as libc::c_int {
            n_v0___0 = n_v;
            end_i___0 = mg_chain_bk_end(max_drop, z as *const mm128_t, f, p, t, k);
            i = (*z.offset(k as isize)).y as int64_t;
            while i != end_i___0 {
                tmp___3 = n_v;
                n_v += 1;
                *v.offset(tmp___3 as isize) = i as int32_t;
                *t.offset(i as isize) = 1 as libc::c_int;
                i = *p.offset(i as isize);
            }
            if i < 0 as libc::c_long {
                sc___0 = (*z.offset(k as isize)).x as int32_t;
            } else {
                sc___0 = (*z.offset(k as isize)).x as int32_t - *f.offset(i as isize);
            }
            if sc___0 >= min_sc {
                if n_v > n_v0___0 {
                    if n_v - n_v0___0 >= min_cnt as int64_t {
                        tmp___4 = n_u;
                        n_u += 1;
                        *u
                            .offset(
                                tmp___4 as isize,
                            ) = (sc___0 as uint64_t) << 32 as libc::c_int
                            | (n_v - n_v0___0) as libc::c_ulong;
                    } else {
                        n_v = n_v0___0;
                    }
                } else {
                    n_v = n_v0___0;
                }
            } else {
                n_v = n_v0___0;
            }
        }
        k -= 1;
    }
    kfree(km, z as *mut libc::c_void);
    if !(n_v < 2147483647 as libc::c_long) {
        __assert_fail(
            b"n_v < INT32_MAX\0" as *const u8 as *const libc::c_char,
            b"lchain.c\0" as *const u8 as *const libc::c_char,
            73 as libc::c_uint,
            b"mg_chain_backtrack\0" as *const u8 as *const libc::c_char,
        );
    }
    *n_u_ = n_u;
    *n_v_ = n_v as int32_t;
    return u;
}
unsafe extern "C" fn compact_a(
    mut km: *mut libc::c_void,
    mut n_u: int32_t,
    mut u: *mut uint64_t,
    mut n_v: int32_t,
    mut v: *mut int32_t,
    mut a: *mut mm128_t,
) -> *mut mm128_t {
    let mut b: *mut mm128_t = 0 as *mut mm128_t;
    let mut w: *mut mm128_t = 0 as *mut mm128_t;
    let mut u2: *mut uint64_t = 0 as *mut uint64_t;
    let mut i: int64_t = 0;
    let mut j: int64_t = 0;
    let mut k: int64_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut k0: int32_t = 0;
    let mut ni: int32_t = 0;
    let mut tmp___0: int64_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut j___0: int32_t = 0;
    let mut n: int32_t = 0;
    tmp = kmalloc(
        km,
        (n_v as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    );
    b = tmp as *mut mm128_t;
    i = 0 as libc::c_int as int64_t;
    k = 0 as libc::c_int as int64_t;
    while i < n_u as int64_t {
        k0 = k as int32_t;
        ni = *u.offset(i as isize) as int32_t;
        j = 0 as libc::c_int as int64_t;
        while j < ni as int64_t {
            tmp___0 = k;
            k += 1;
            *b
                .offset(
                    tmp___0 as isize,
                ) = *a
                .offset(
                    *v
                        .offset(
                            (k0 as int64_t + (ni as int64_t - j - 1 as libc::c_long))
                                as isize,
                        ) as isize,
                );
            j += 1;
        }
        i += 1;
    }
    kfree(km, v as *mut libc::c_void);
    tmp___1 = kmalloc(
        km,
        (n_u as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    );
    w = tmp___1 as *mut mm128_t;
    k = 0 as libc::c_int as int64_t;
    i = k;
    while i < n_u as int64_t {
        (*w.offset(i as isize)).x = (*b.offset(k as isize)).x;
        (*w.offset(i as isize))
            .y = (k as uint64_t) << 32 as libc::c_int | i as libc::c_ulong;
        k += *u.offset(i as isize) as int32_t as int64_t;
        i += 1;
    }
    radix_sort_128x(w, w.offset(n_u as isize));
    tmp___2 = kmalloc(
        km,
        (n_u as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
    );
    u2 = tmp___2 as *mut uint64_t;
    k = 0 as libc::c_int as int64_t;
    i = k;
    while i < n_u as int64_t {
        j___0 = (*w.offset(i as isize)).y as int32_t;
        n = *u.offset(j___0 as isize) as int32_t;
        *u2.offset(i as isize) = *u.offset(j___0 as isize);
        memcpy(
            a.offset(k as isize) as *mut libc::c_void,
            b.offset(((*w.offset(i as isize)).y >> 32 as libc::c_int) as isize)
                as *const libc::c_void,
            (n as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
        );
        k += n as int64_t;
        i += 1;
    }
    memcpy(
        u as *mut libc::c_void,
        u2 as *const libc::c_void,
        (n_u * 8 as libc::c_int) as size_t,
    );
    memcpy(
        b as *mut libc::c_void,
        a as *const libc::c_void,
        (k as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    );
    kfree(km, a as *mut libc::c_void);
    kfree(km, w as *mut libc::c_void);
    kfree(km, u2 as *mut libc::c_void);
    return b;
}
#[inline]
unsafe extern "C" fn comput_sc(
    mut ai: *const mm128_t,
    mut aj: *const mm128_t,
    mut max_dist_x: int32_t,
    mut max_dist_y: int32_t,
    mut bw: int32_t,
    mut chn_pen_gap: libc::c_float,
    mut chn_pen_skip: libc::c_float,
    mut is_cdna: libc::c_int,
    mut n_seg: libc::c_int,
) -> int32_t {
    let mut dq: int32_t = 0;
    let mut dr: int32_t = 0;
    let mut dd: int32_t = 0;
    let mut dg: int32_t = 0;
    let mut q_span: int32_t = 0;
    let mut sc: int32_t = 0;
    let mut sidi: int32_t = 0;
    let mut sidj: int32_t = 0;
    let mut lin_pen: libc::c_float = 0.;
    let mut log_pen: libc::c_float = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut tmp___0: libc::c_float = 0.;
    dq = (*ai).y as int32_t - (*aj).y as int32_t;
    sidi = (((*ai).y as libc::c_ulonglong
        & (255 as libc::c_ulonglong) << 48 as libc::c_int) >> 48 as libc::c_int)
        as int32_t;
    sidj = (((*aj).y as libc::c_ulonglong
        & (255 as libc::c_ulonglong) << 48 as libc::c_int) >> 48 as libc::c_int)
        as int32_t;
    if dq <= 0 as libc::c_int {
        return -(0x7fffffff as libc::c_int) - 1 as libc::c_int
    } else {
        if dq > max_dist_x {
            return -(0x7fffffff as libc::c_int) - 1 as libc::c_int;
        }
    }
    dr = ((*ai).x).wrapping_sub((*aj).x) as int32_t;
    if sidi == sidj {
        if dr == 0 as libc::c_int {
            return -(0x7fffffff as libc::c_int) - 1 as libc::c_int
        } else {
            if dq > max_dist_y {
                return -(0x7fffffff as libc::c_int) - 1 as libc::c_int;
            }
        }
    }
    if dr > dq {
        dd = dr - dq;
    } else {
        dd = dq - dr;
    }
    if sidi == sidj {
        if dd > bw {
            return -(0x7fffffff as libc::c_int) - 1 as libc::c_int;
        }
    }
    if n_seg > 1 as libc::c_int {
        if is_cdna == 0 {
            if sidi == sidj {
                if dr > max_dist_y {
                    return -(0x7fffffff as libc::c_int) - 1 as libc::c_int;
                }
            }
        }
    }
    if dr < dq {
        dg = dr;
    } else {
        dg = dq;
    }
    q_span = ((*aj).y >> 32 as libc::c_int & 255 as libc::c_ulong) as int32_t;
    if q_span < dg {
        sc = q_span;
    } else {
        sc = dg;
    }
    let mut current_block_69: u64;
    if dd != 0 {
        current_block_69 = 15690485994277878412;
    } else if dg > q_span {
        current_block_69 = 15690485994277878412;
    } else {
        current_block_69 = 18038362259723567392;
    }
    match current_block_69 {
        15690485994277878412 => {
            lin_pen = chn_pen_gap * dd as libc::c_float
                + chn_pen_skip * dg as libc::c_float;
            if dd >= 1 as libc::c_int {
                tmp = mg_log2___0((dd + 1 as libc::c_int) as libc::c_float);
                log_pen = tmp;
            } else {
                log_pen = 0.0f32;
            }
            let mut current_block_68: u64;
            if is_cdna != 0 {
                current_block_68 = 5221380873865686172;
            } else if sidi != sidj {
                current_block_68 = 5221380873865686172;
            } else {
                sc -= (lin_pen + 0.5f32 * log_pen) as libc::c_int;
                current_block_68 = 12264624100856317061;
            }
            match current_block_68 {
                5221380873865686172 => {
                    let mut current_block_65: u64;
                    if sidi != sidj {
                        if dr == 0 as libc::c_int {
                            sc += 1;
                            current_block_65 = 307447392441238883;
                        } else {
                            current_block_65 = 6476307689458339582;
                        }
                    } else {
                        current_block_65 = 6476307689458339582;
                    }
                    match current_block_65 {
                        6476307689458339582 => {
                            let mut current_block_64: u64;
                            if dr > dq {
                                current_block_64 = 5562266355727715084;
                            } else if sidi != sidj {
                                current_block_64 = 5562266355727715084;
                            } else {
                                sc -= (lin_pen + 0.5f32 * log_pen) as libc::c_int;
                                current_block_64 = 13125627826496529465;
                            }
                            match current_block_64 {
                                5562266355727715084 => {
                                    if lin_pen < log_pen {
                                        tmp___0 = lin_pen;
                                    } else {
                                        tmp___0 = log_pen;
                                    }
                                    sc -= tmp___0 as libc::c_int;
                                }
                                _ => {}
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
    return sc;
}
pub unsafe extern "C" fn mg_lchain_dp(
    mut max_dist_x: libc::c_int,
    mut max_dist_y: libc::c_int,
    mut bw: libc::c_int,
    mut max_skip: libc::c_int,
    mut max_iter: libc::c_int,
    mut min_cnt: libc::c_int,
    mut min_sc: libc::c_int,
    mut chn_pen_gap: libc::c_float,
    mut chn_pen_skip: libc::c_float,
    mut is_cdna: libc::c_int,
    mut n_seg: libc::c_int,
    mut n: int64_t,
    mut a: *mut mm128_t,
    mut n_u_: *mut libc::c_int,
    mut _u: *mut *mut uint64_t,
    mut km: *mut libc::c_void,
) -> *mut mm128_t {
    let mut f: *mut int32_t = 0 as *mut int32_t;
    let mut t: *mut int32_t = 0 as *mut int32_t;
    let mut v: *mut int32_t = 0 as *mut int32_t;
    let mut n_u: int32_t = 0;
    let mut n_v: int32_t = 0;
    let mut mmax_f: int32_t = 0;
    let mut max_drop: int32_t = 0;
    let mut p: *mut int64_t = 0 as *mut int64_t;
    let mut i: int64_t = 0;
    let mut j: int64_t = 0;
    let mut max_ii: int64_t = 0;
    let mut st: int64_t = 0;
    let mut n_iter: int64_t = 0;
    let mut u: *mut uint64_t = 0 as *mut uint64_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut max_j: int64_t = 0;
    let mut end_j: int64_t = 0;
    let mut max_f: int32_t = 0;
    let mut n_skip: int32_t = 0;
    let mut sc: int32_t = 0;
    let mut max: int32_t = 0;
    let mut tmp___3: int32_t = 0;
    let mut tmp___4: *mut mm128_t = 0 as *mut mm128_t;
    mmax_f = 0 as libc::c_int;
    max_drop = bw;
    st = 0 as libc::c_int as int64_t;
    n_iter = 0 as libc::c_int as int64_t;
    if !_u.is_null() {
        *_u = 0 as *mut uint64_t;
        *n_u_ = 0 as libc::c_int;
    }
    if n == 0 as libc::c_long {
        kfree(km, a as *mut libc::c_void);
        return 0 as *mut mm128_t;
    } else {
        if a as libc::c_ulong == 0 as *mut mm128_t as libc::c_ulong {
            kfree(km, a as *mut libc::c_void);
            return 0 as *mut mm128_t;
        }
    }
    if max_dist_x < bw {
        max_dist_x = bw;
    }
    if max_dist_y < bw {
        if is_cdna == 0 {
            max_dist_y = bw;
        }
    }
    if is_cdna != 0 {
        max_drop = 2147483647 as libc::c_int;
    }
    tmp = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int64_t>() as libc::c_ulong),
    );
    p = tmp as *mut int64_t;
    tmp___0 = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    );
    f = tmp___0 as *mut int32_t;
    tmp___1 = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    );
    v = tmp___1 as *mut int32_t;
    tmp___2 = kcalloc(
        km,
        n as size_t,
        ::std::mem::size_of::<int32_t>() as libc::c_ulong,
    );
    t = tmp___2 as *mut int32_t;
    i = 0 as libc::c_int as int64_t;
    max_ii = -(1 as libc::c_int) as int64_t;
    while i < n {
        max_j = -(1 as libc::c_int) as int64_t;
        max_f = ((*a.offset(i as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
            as int32_t;
        n_skip = 0 as libc::c_int;
        while st < i {
            if !((*a.offset(i as isize)).x >> 32 as libc::c_int
                != (*a.offset(st as isize)).x >> 32 as libc::c_int)
            {
                if !((*a.offset(i as isize)).x
                    > ((*a.offset(st as isize)).x).wrapping_add(max_dist_x as uint64_t))
                {
                    break;
                }
            }
            st += 1;
        }
        if i - st > max_iter as int64_t {
            st = i - max_iter as int64_t;
        }
        j = i - 1 as libc::c_long;
        while j >= st {
            sc = comput_sc(
                a.offset(i as isize) as *const mm128_t,
                a.offset(j as isize) as *const mm128_t,
                max_dist_x,
                max_dist_y,
                bw,
                chn_pen_gap,
                chn_pen_skip,
                is_cdna,
                n_seg,
            );
            n_iter += 1;
            if !(sc == -(0x7fffffff as libc::c_int) - 1 as libc::c_int) {
                sc += *f.offset(j as isize);
                if sc > max_f {
                    max_f = sc;
                    max_j = j;
                    if n_skip > 0 as libc::c_int {
                        n_skip -= 1;
                    }
                } else if *t.offset(j as isize) == i as int32_t {
                    n_skip += 1;
                    if n_skip > max_skip {
                        break;
                    }
                }
                if *p.offset(j as isize) >= 0 as libc::c_long {
                    *t.offset(*p.offset(j as isize) as isize) = i as int32_t;
                }
            }
            j -= 1;
        }
        end_j = j;
        let mut current_block_71: u64;
        if max_ii < 0 as libc::c_long {
            current_block_71 = 10116925497236079379;
        } else if ((*a.offset(i as isize)).x)
                .wrapping_sub((*a.offset(max_ii as isize)).x)
                > max_dist_x as int64_t as uint64_t
            {
            current_block_71 = 10116925497236079379;
        } else {
            current_block_71 = 5793491756164225964;
        }
        match current_block_71 {
            10116925497236079379 => {
                max = -(0x7fffffff as libc::c_int) - 1 as libc::c_int;
                max_ii = -(1 as libc::c_int) as int64_t;
                j = i - 1 as libc::c_long;
                while j >= st {
                    if max < *f.offset(j as isize) {
                        max = *f.offset(j as isize);
                        max_ii = j;
                    }
                    j -= 1;
                }
            }
            _ => {}
        }
        if max_ii >= 0 as libc::c_long {
            if max_ii < end_j {
                tmp___3 = comput_sc(
                    a.offset(i as isize) as *const mm128_t,
                    a.offset(max_ii as isize) as *const mm128_t,
                    max_dist_x,
                    max_dist_y,
                    bw,
                    chn_pen_gap,
                    chn_pen_skip,
                    is_cdna,
                    n_seg,
                );
                if tmp___3 != -(0x7fffffff as libc::c_int) - 1 as libc::c_int {
                    if max_f < tmp___3 + *f.offset(max_ii as isize) {
                        max_f = tmp___3 + *f.offset(max_ii as isize);
                        max_j = max_ii;
                    }
                }
            }
        }
        *f.offset(i as isize) = max_f;
        *p.offset(i as isize) = max_j;
        if max_j >= 0 as libc::c_long {
            if *v.offset(max_j as isize) > max_f {
                *v.offset(i as isize) = *v.offset(max_j as isize);
            } else {
                *v.offset(i as isize) = max_f;
            }
        } else {
            *v.offset(i as isize) = max_f;
        }
        if max_ii < 0 as libc::c_long {
            max_ii = i;
        } else if ((*a.offset(i as isize)).x)
                .wrapping_sub((*a.offset(max_ii as isize)).x)
                <= max_dist_x as int64_t as uint64_t
            {
            if *f.offset(max_ii as isize) < *f.offset(i as isize) {
                max_ii = i;
            }
        }
        if mmax_f < max_f {
            mmax_f = max_f;
        }
        i += 1;
    }
    u = mg_chain_backtrack(
        km,
        n,
        f as *const int32_t,
        p as *const int64_t,
        v,
        t,
        min_cnt,
        min_sc,
        max_drop,
        &mut n_u,
        &mut n_v,
    );
    *n_u_ = n_u;
    *_u = u;
    kfree(km, p as *mut libc::c_void);
    kfree(km, f as *mut libc::c_void);
    kfree(km, t as *mut libc::c_void);
    if n_u == 0 as libc::c_int {
        kfree(km, a as *mut libc::c_void);
        kfree(km, v as *mut libc::c_void);
        return 0 as *mut mm128_t;
    }
    tmp___4 = compact_a(km, n_u, u, n_v, v, a);
    return tmp___4;
}
pub unsafe extern "C" fn krmq_find_lc_elem(
    mut root: *const lc_elem_t,
    mut x: *const lc_elem_t,
    mut cnt_: *mut libc::c_uint,
) -> *mut lc_elem_t {
    let mut p: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut cnt: libc::c_uint = 0;
    let mut cmp: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_uint = 0;
    p = root;
    cnt = 0 as libc::c_uint;
    while p as libc::c_ulong != 0 as *const lc_elem_t as libc::c_ulong {
        if (*x).y < (*p).y {
            cmp = -(1 as libc::c_int);
        } else {
            if (*x).y > (*p).y {
                tmp = 1 as libc::c_int;
            } else {
                tmp = ((*x).i > (*p).i) as libc::c_int
                    - ((*x).i < (*p).i) as libc::c_int;
            }
            cmp = tmp;
        }
        if cmp >= 0 as libc::c_int {
            if !((*p).head.p[0 as libc::c_int as usize]).is_null() {
                tmp___0 = (*(*p).head.p[0 as libc::c_int as usize]).head.size;
            } else {
                tmp___0 = 0 as libc::c_uint;
            }
            cnt = cnt.wrapping_add(tmp___0.wrapping_add(1 as libc::c_uint));
        }
        if cmp < 0 as libc::c_int {
            p = (*p).head.p[0 as libc::c_int as usize] as *const lc_elem_t;
        } else {
            if !(cmp > 0 as libc::c_int) {
                break;
            }
            p = (*p).head.p[1 as libc::c_int as usize] as *const lc_elem_t;
        }
    }
    if !cnt_.is_null() {
        *cnt_ = cnt;
    }
    return p as *mut lc_elem_t;
}
pub unsafe extern "C" fn krmq_interval_lc_elem(
    mut root: *const lc_elem_t,
    mut x: *const lc_elem_t,
    mut lower: *mut *mut lc_elem_t,
    mut upper: *mut *mut lc_elem_t,
) -> *mut lc_elem_t {
    let mut p: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut l: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut u: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut cmp: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    p = root;
    l = 0 as *const lc_elem_t;
    u = 0 as *const lc_elem_t;
    while p as libc::c_ulong != 0 as *const lc_elem_t as libc::c_ulong {
        if (*x).y < (*p).y {
            cmp = -(1 as libc::c_int);
        } else {
            if (*x).y > (*p).y {
                tmp = 1 as libc::c_int;
            } else {
                tmp = ((*x).i > (*p).i) as libc::c_int
                    - ((*x).i < (*p).i) as libc::c_int;
            }
            cmp = tmp;
        }
        if cmp < 0 as libc::c_int {
            u = p;
            p = (*p).head.p[0 as libc::c_int as usize] as *const lc_elem_t;
        } else if cmp > 0 as libc::c_int {
            l = p;
            p = (*p).head.p[1 as libc::c_int as usize] as *const lc_elem_t;
        } else {
            u = p;
            l = u;
            break;
        }
    }
    if !lower.is_null() {
        *lower = l as *mut lc_elem_t;
    }
    if !upper.is_null() {
        *upper = u as *mut lc_elem_t;
    }
    return p as *mut lc_elem_t;
}
pub unsafe extern "C" fn krmq_rmq_lc_elem(
    mut root: *const lc_elem_t,
    mut lo: *const lc_elem_t,
    mut up: *const lc_elem_t,
) -> *mut lc_elem_t {
    let mut p: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut path: [[*const lc_elem_t; 64]; 2] = [[0 as *const lc_elem_t; 64]; 2];
    let mut min: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut plen: [libc::c_int; 2] = [0; 2];
    let mut pcmp: [[libc::c_int; 64]; 2] = [[0; 64]; 2];
    let mut i: libc::c_int = 0;
    let mut cmp: libc::c_int = 0;
    let mut lca: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    p = root;
    plen[0 as libc::c_int as usize] = 0 as libc::c_int;
    plen[1 as libc::c_int as usize] = 0 as libc::c_int;
    if root as libc::c_ulong == 0 as *const lc_elem_t as libc::c_ulong {
        return 0 as *mut lc_elem_t;
    }
    while !p.is_null() {
        if (*lo).y < (*p).y {
            cmp = -(1 as libc::c_int);
        } else {
            if (*lo).y > (*p).y {
                tmp = 1 as libc::c_int;
            } else {
                tmp = ((*lo).i > (*p).i) as libc::c_int
                    - ((*lo).i < (*p).i) as libc::c_int;
            }
            cmp = tmp;
        }
        path[0 as libc::c_int as usize][plen[0 as libc::c_int as usize] as usize] = p;
        tmp___0 = plen[0 as libc::c_int as usize];
        plen[0 as libc::c_int as usize] += 1;
        pcmp[0 as libc::c_int as usize][tmp___0 as usize] = cmp;
        if cmp < 0 as libc::c_int {
            p = (*p).head.p[0 as libc::c_int as usize] as *const lc_elem_t;
        } else {
            if !(cmp > 0 as libc::c_int) {
                break;
            }
            p = (*p).head.p[1 as libc::c_int as usize] as *const lc_elem_t;
        }
    }
    p = root;
    while !p.is_null() {
        if (*up).y < (*p).y {
            cmp = -(1 as libc::c_int);
        } else {
            if (*up).y > (*p).y {
                tmp___1 = 1 as libc::c_int;
            } else {
                tmp___1 = ((*up).i > (*p).i) as libc::c_int
                    - ((*up).i < (*p).i) as libc::c_int;
            }
            cmp = tmp___1;
        }
        path[1 as libc::c_int as usize][plen[1 as libc::c_int as usize] as usize] = p;
        tmp___2 = plen[1 as libc::c_int as usize];
        plen[1 as libc::c_int as usize] += 1;
        pcmp[1 as libc::c_int as usize][tmp___2 as usize] = cmp;
        if cmp < 0 as libc::c_int {
            p = (*p).head.p[0 as libc::c_int as usize] as *const lc_elem_t;
        } else {
            if !(cmp > 0 as libc::c_int) {
                break;
            }
            p = (*p).head.p[1 as libc::c_int as usize] as *const lc_elem_t;
        }
    }
    i = 0 as libc::c_int;
    while i < plen[0 as libc::c_int as usize] {
        if !(i < plen[1 as libc::c_int as usize]) {
            break;
        }
        if path[0 as libc::c_int as usize][i as usize] as libc::c_ulong
            == path[1 as libc::c_int as usize][i as usize] as libc::c_ulong
        {
            if pcmp[0 as libc::c_int as usize][i as usize] <= 0 as libc::c_int {
                if pcmp[1 as libc::c_int as usize][i as usize] >= 0 as libc::c_int {
                    break;
                }
            }
        }
        i += 1;
    }
    if i == plen[0 as libc::c_int as usize] {
        return 0 as *mut lc_elem_t
    } else {
        if i == plen[1 as libc::c_int as usize] {
            return 0 as *mut lc_elem_t;
        }
    }
    lca = i;
    min = path[0 as libc::c_int as usize][lca as usize];
    i = lca + 1 as libc::c_int;
    while i < plen[0 as libc::c_int as usize] {
        if pcmp[0 as libc::c_int as usize][i as usize] <= 0 as libc::c_int {
            if (*path[0 as libc::c_int as usize][i as usize]).pri < (*min).pri {
                min = path[0 as libc::c_int as usize][i as usize];
            }
            if !((*path[0 as libc::c_int as usize][i as usize])
                .head
                .p[1 as libc::c_int as usize])
                .is_null()
            {
                if (*(*(*path[0 as libc::c_int as usize][i as usize])
                    .head
                    .p[1 as libc::c_int as usize])
                    .head
                    .s)
                    .pri < (*min).pri
                {
                    min = (*(*path[0 as libc::c_int as usize][i as usize])
                        .head
                        .p[1 as libc::c_int as usize])
                        .head
                        .s as *const lc_elem_t;
                }
            }
        }
        i += 1;
    }
    i = lca + 1 as libc::c_int;
    while i < plen[1 as libc::c_int as usize] {
        if pcmp[1 as libc::c_int as usize][i as usize] >= 0 as libc::c_int {
            if (*path[1 as libc::c_int as usize][i as usize]).pri < (*min).pri {
                min = path[1 as libc::c_int as usize][i as usize];
            }
            if !((*path[1 as libc::c_int as usize][i as usize])
                .head
                .p[0 as libc::c_int as usize])
                .is_null()
            {
                if (*(*(*path[1 as libc::c_int as usize][i as usize])
                    .head
                    .p[0 as libc::c_int as usize])
                    .head
                    .s)
                    .pri < (*min).pri
                {
                    min = (*(*path[1 as libc::c_int as usize][i as usize])
                        .head
                        .p[0 as libc::c_int as usize])
                        .head
                        .s as *const lc_elem_t;
                }
            }
        }
        i += 1;
    }
    return min as *mut lc_elem_t;
}
#[inline]
unsafe extern "C" fn krmq_update_min_lc_elem(
    mut p: *mut lc_elem_t,
    mut q: *const lc_elem_t,
    mut r: *const lc_elem_t,
) {
    if q.is_null() {
        (*p).head.s = p;
    } else if (*p).pri < (*(*q).head.s).pri {
        (*p).head.s = p;
    } else {
        (*p).head.s = (*q).head.s;
    }
    if r.is_null() {
        (*p).head.s = (*p).head.s;
    } else if (*(*p).head.s).pri < (*(*r).head.s).pri {
        (*p).head.s = (*p).head.s;
    } else {
        (*p).head.s = (*r).head.s;
    };
}
#[inline]
unsafe extern "C" fn krmq_rotate1_lc_elem(
    mut p: *mut lc_elem_t,
    mut dir: libc::c_int,
) -> *mut lc_elem_t {
    let mut opp: libc::c_int = 0;
    let mut q: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut s: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut size_p: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
    opp = 1 as libc::c_int - dir;
    q = (*p).head.p[opp as usize];
    s = (*p).head.s;
    size_p = (*p).head.size;
    if !((*q).head.p[dir as usize]).is_null() {
        tmp = (*(*q).head.p[dir as usize]).head.size;
    } else {
        tmp = 0 as libc::c_uint;
    }
    (*p).head.size = ((*p).head.size).wrapping_sub(((*q).head.size).wrapping_sub(tmp));
    (*q).head.size = size_p;
    krmq_update_min_lc_elem(
        p,
        (*p).head.p[dir as usize] as *const lc_elem_t,
        (*q).head.p[dir as usize] as *const lc_elem_t,
    );
    (*q).head.s = s;
    (*p).head.p[opp as usize] = (*q).head.p[dir as usize];
    (*q).head.p[dir as usize] = p;
    return q;
}
#[inline]
unsafe extern "C" fn krmq_rotate2_lc_elem(
    mut p: *mut lc_elem_t,
    mut dir: libc::c_int,
) -> *mut lc_elem_t {
    let mut b1: libc::c_int = 0;
    let mut opp: libc::c_int = 0;
    let mut q: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut r: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut s: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut size_x_dir: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
    let mut tmp___0: libc::c_schar = 0;
    opp = 1 as libc::c_int - dir;
    q = (*p).head.p[opp as usize];
    r = (*q).head.p[dir as usize];
    s = (*p).head.s;
    if !((*r).head.p[dir as usize]).is_null() {
        tmp = (*(*r).head.p[dir as usize]).head.size;
    } else {
        tmp = 0 as libc::c_uint;
    }
    size_x_dir = tmp;
    (*r).head.size = (*p).head.size;
    (*p)
        .head
        .size = ((*p).head.size).wrapping_sub(((*q).head.size).wrapping_sub(size_x_dir));
    (*q)
        .head
        .size = ((*q).head.size)
        .wrapping_sub(size_x_dir.wrapping_add(1 as libc::c_uint));
    krmq_update_min_lc_elem(
        p,
        (*p).head.p[dir as usize] as *const lc_elem_t,
        (*r).head.p[dir as usize] as *const lc_elem_t,
    );
    krmq_update_min_lc_elem(
        q,
        (*q).head.p[opp as usize] as *const lc_elem_t,
        (*r).head.p[opp as usize] as *const lc_elem_t,
    );
    (*r).head.s = s;
    (*p).head.p[opp as usize] = (*r).head.p[dir as usize];
    (*r).head.p[dir as usize] = p;
    (*q).head.p[dir as usize] = (*r).head.p[opp as usize];
    (*r).head.p[opp as usize] = q;
    if dir == 0 as libc::c_int {
        b1 = 1 as libc::c_int;
    } else {
        b1 = -(1 as libc::c_int);
    }
    if (*r).head.balance as libc::c_int == b1 {
        (*q).head.balance = 0 as libc::c_int as libc::c_schar;
        (*p).head.balance = -b1 as libc::c_schar;
    } else if (*r).head.balance as libc::c_int == 0 as libc::c_int {
        tmp___0 = 0 as libc::c_int as libc::c_schar;
        (*p).head.balance = tmp___0;
        (*q).head.balance = tmp___0;
    } else {
        (*q).head.balance = b1 as libc::c_schar;
        (*p).head.balance = 0 as libc::c_int as libc::c_schar;
    }
    (*r).head.balance = 0 as libc::c_int as libc::c_schar;
    return r;
}
pub unsafe extern "C" fn krmq_insert_lc_elem(
    mut root_: *mut *mut lc_elem_t,
    mut x: *mut lc_elem_t,
    mut cnt_: *mut libc::c_uint,
) -> *mut lc_elem_t {
    let mut stack: [libc::c_uchar; 64] = [0; 64];
    let mut path: [*mut lc_elem_t; 64] = [0 as *mut lc_elem_t; 64];
    let mut bp: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut bq: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut p: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut q: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut r: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut i: libc::c_int = 0;
    let mut which: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut path_len: libc::c_int = 0;
    let mut cnt: libc::c_uint = 0;
    let mut cmp: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_uint = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut lc_elem_s = 0 as *mut lc_elem_s;
    let mut tmp___4: libc::c_schar = 0;
    r = 0 as *mut lc_elem_t;
    which = 0 as libc::c_int;
    cnt = 0 as libc::c_uint;
    bp = *root_;
    bq = 0 as *mut lc_elem_t;
    p = bp;
    q = bq;
    path_len = 0 as libc::c_int;
    top = path_len;
    while !p.is_null() {
        if (*x).y < (*p).y {
            cmp = -(1 as libc::c_int);
        } else {
            if (*x).y > (*p).y {
                tmp = 1 as libc::c_int;
            } else {
                tmp = ((*x).i > (*p).i) as libc::c_int
                    - ((*x).i < (*p).i) as libc::c_int;
            }
            cmp = tmp;
        }
        if cmp >= 0 as libc::c_int {
            if !((*p).head.p[0 as libc::c_int as usize]).is_null() {
                tmp___0 = (*(*p).head.p[0 as libc::c_int as usize]).head.size;
            } else {
                tmp___0 = 0 as libc::c_uint;
            }
            cnt = cnt.wrapping_add(tmp___0.wrapping_add(1 as libc::c_uint));
        }
        if cmp == 0 as libc::c_int {
            if !cnt_.is_null() {
                *cnt_ = cnt;
            }
            return p;
        }
        if (*p).head.balance as libc::c_int != 0 as libc::c_int {
            bq = q;
            bp = p;
            top = 0 as libc::c_int;
        }
        tmp___1 = top;
        top += 1;
        which = (cmp > 0 as libc::c_int) as libc::c_int;
        stack[tmp___1 as usize] = which as libc::c_uchar;
        tmp___2 = path_len;
        path_len += 1;
        path[tmp___2 as usize] = p;
        q = p;
        p = (*p).head.p[which as usize];
    }
    if !cnt_.is_null() {
        *cnt_ = cnt;
    }
    (*x).head.balance = 0 as libc::c_int as libc::c_schar;
    (*x).head.size = 1 as libc::c_uint;
    tmp___3 = 0 as *mut lc_elem_s;
    (*x).head.p[1 as libc::c_int as usize] = tmp___3;
    (*x).head.p[0 as libc::c_int as usize] = tmp___3;
    (*x).head.s = x;
    if q as libc::c_ulong == 0 as *mut lc_elem_t as libc::c_ulong {
        *root_ = x;
    } else {
        (*q).head.p[which as usize] = x;
    }
    if bp as libc::c_ulong == 0 as *mut lc_elem_t as libc::c_ulong {
        return x;
    }
    i = 0 as libc::c_int;
    while i < path_len {
        (*path[i as usize]).head.size = ((*path[i as usize]).head.size).wrapping_add(1);
        i += 1;
    }
    i = path_len - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        krmq_update_min_lc_elem(
            path[i as usize],
            (*path[i as usize]).head.p[0 as libc::c_int as usize] as *const lc_elem_t,
            (*path[i as usize]).head.p[1 as libc::c_int as usize] as *const lc_elem_t,
        );
        if (*path[i as usize]).head.s as libc::c_ulong != x as libc::c_ulong {
            break;
        }
        i -= 1;
    }
    p = bp;
    top = 0 as libc::c_int;
    while p as libc::c_ulong != x as libc::c_ulong {
        if stack[top as usize] as libc::c_int == 0 as libc::c_int {
            (*p)
                .head
                .balance = ((*p).head.balance as libc::c_int - 1 as libc::c_int)
                as libc::c_schar;
        } else {
            (*p)
                .head
                .balance = ((*p).head.balance as libc::c_int + 1 as libc::c_int)
                as libc::c_schar;
        }
        p = (*p).head.p[stack[top as usize] as usize];
        top += 1;
    }
    if (*bp).head.balance as libc::c_int > -(2 as libc::c_int) {
        if ((*bp).head.balance as libc::c_int) < 2 as libc::c_int {
            return x;
        }
    }
    which = (((*bp).head.balance as libc::c_int) < 0 as libc::c_int) as libc::c_int;
    if which == 0 as libc::c_int {
        b1 = 1 as libc::c_int;
    } else {
        b1 = -(1 as libc::c_int);
    }
    q = (*bp).head.p[(1 as libc::c_int - which) as usize];
    if (*q).head.balance as libc::c_int == b1 {
        r = krmq_rotate1_lc_elem(bp, which);
        tmp___4 = 0 as libc::c_int as libc::c_schar;
        (*bp).head.balance = tmp___4;
        (*q).head.balance = tmp___4;
    } else {
        r = krmq_rotate2_lc_elem(bp, which);
    }
    if bq as libc::c_ulong == 0 as *mut lc_elem_t as libc::c_ulong {
        *root_ = r;
    } else {
        (*bq)
            .head
            .p[(bp as libc::c_ulong
            != (*bq).head.p[0 as libc::c_int as usize] as libc::c_ulong) as libc::c_int
            as usize] = r;
    }
    return x;
}
pub unsafe extern "C" fn krmq_erase_lc_elem(
    mut root_: *mut *mut lc_elem_t,
    mut x: *const lc_elem_t,
    mut cnt_: *mut libc::c_uint,
) -> *mut lc_elem_t {
    let mut p: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut path: [*mut lc_elem_t; 64] = [0 as *mut lc_elem_t; 64];
    let mut fake: lc_elem_t = lc_elem_t {
        y: 0,
        i: 0,
        pri: 0.,
        head: __anonstruct_head_172264678 {
            p: [0 as *mut lc_elem_s; 2],
            s: 0 as *mut lc_elem_s,
            balance: 0,
            size: 0,
        },
    };
    let mut dir: [libc::c_uchar; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut cmp: libc::c_int = 0;
    let mut cnt: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut which: libc::c_int = 0;
    let mut tmp___0: libc::c_uint = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_uint = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut q: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut tmp___4: libc::c_int = 0;
    let mut r: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut e: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut q___0: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut which___0: libc::c_int = 0;
    let mut other: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut r___0: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut tmp___7: libc::c_schar = 0;
    d = 0 as libc::c_int;
    cnt = 0 as libc::c_uint;
    fake = **root_;
    fake.head.p[0 as libc::c_int as usize] = *root_;
    fake.head.p[1 as libc::c_int as usize] = 0 as *mut lc_elem_s;
    if !cnt_.is_null() {
        *cnt_ = 0 as libc::c_uint;
    }
    if !x.is_null() {
        cmp = -(1 as libc::c_int);
        p = &mut fake;
        while cmp != 0 {
            which = (cmp > 0 as libc::c_int) as libc::c_int;
            if cmp > 0 as libc::c_int {
                if !((*p).head.p[0 as libc::c_int as usize]).is_null() {
                    tmp___0 = (*(*p).head.p[0 as libc::c_int as usize]).head.size;
                } else {
                    tmp___0 = 0 as libc::c_uint;
                }
                cnt = cnt.wrapping_add(tmp___0.wrapping_add(1 as libc::c_uint));
            }
            dir[d as usize] = which as libc::c_uchar;
            tmp___1 = d;
            d += 1;
            path[tmp___1 as usize] = p;
            p = (*p).head.p[which as usize];
            if p as libc::c_ulong == 0 as *mut lc_elem_t as libc::c_ulong {
                if !cnt_.is_null() {
                    *cnt_ = 0 as libc::c_uint;
                }
                return 0 as *mut lc_elem_t;
            }
            if (*x).y < (*p).y {
                cmp = -(1 as libc::c_int);
            } else {
                if (*x).y > (*p).y {
                    tmp = 1 as libc::c_int;
                } else {
                    tmp = ((*x).i > (*p).i) as libc::c_int
                        - ((*x).i < (*p).i) as libc::c_int;
                }
                cmp = tmp;
            }
        }
        if !((*p).head.p[0 as libc::c_int as usize]).is_null() {
            tmp___2 = (*(*p).head.p[0 as libc::c_int as usize]).head.size;
        } else {
            tmp___2 = 0 as libc::c_uint;
        }
        cnt = cnt.wrapping_add(tmp___2.wrapping_add(1 as libc::c_uint));
    } else {
        p = &mut fake;
        cnt = 1 as libc::c_uint;
        while !p.is_null() {
            dir[d as usize] = 0 as libc::c_int as libc::c_uchar;
            tmp___3 = d;
            d += 1;
            path[tmp___3 as usize] = p;
            p = (*p).head.p[0 as libc::c_int as usize];
        }
        d -= 1;
        p = path[d as usize];
    }
    if !cnt_.is_null() {
        *cnt_ = cnt;
    }
    i = 1 as libc::c_int;
    while i < d {
        (*path[i as usize]).head.size = ((*path[i as usize]).head.size).wrapping_sub(1);
        i += 1;
    }
    if (*p).head.p[1 as libc::c_int as usize] as libc::c_ulong
        == 0 as *mut lc_elem_s as libc::c_ulong
    {
        (*path[(d - 1 as libc::c_int) as usize])
            .head
            .p[dir[(d - 1 as libc::c_int) as usize]
            as usize] = (*p).head.p[0 as libc::c_int as usize];
    } else {
        q = (*p).head.p[1 as libc::c_int as usize];
        if (*q).head.p[0 as libc::c_int as usize] as libc::c_ulong
            == 0 as *mut lc_elem_s as libc::c_ulong
        {
            (*q)
                .head
                .p[0 as libc::c_int as usize] = (*p).head.p[0 as libc::c_int as usize];
            (*q).head.balance = (*p).head.balance;
            (*path[(d - 1 as libc::c_int) as usize])
                .head
                .p[dir[(d - 1 as libc::c_int) as usize] as usize] = q;
            path[d as usize] = q;
            tmp___4 = d;
            d += 1;
            dir[tmp___4 as usize] = 1 as libc::c_int as libc::c_uchar;
            (*q).head.size = ((*p).head.size).wrapping_sub(1 as libc::c_uint);
        } else {
            tmp___5 = d;
            d += 1;
            e = tmp___5;
            loop {
                dir[d as usize] = 0 as libc::c_int as libc::c_uchar;
                tmp___6 = d;
                d += 1;
                path[tmp___6 as usize] = q;
                r = (*q).head.p[0 as libc::c_int as usize];
                if (*r).head.p[0 as libc::c_int as usize] as libc::c_ulong
                    == 0 as *mut lc_elem_s as libc::c_ulong
                {
                    break;
                }
                q = r;
            }
            (*r)
                .head
                .p[0 as libc::c_int as usize] = (*p).head.p[0 as libc::c_int as usize];
            (*q)
                .head
                .p[0 as libc::c_int as usize] = (*r).head.p[1 as libc::c_int as usize];
            (*r)
                .head
                .p[1 as libc::c_int as usize] = (*p).head.p[1 as libc::c_int as usize];
            (*r).head.balance = (*p).head.balance;
            (*path[(e - 1 as libc::c_int) as usize])
                .head
                .p[dir[(e - 1 as libc::c_int) as usize] as usize] = r;
            path[e as usize] = r;
            dir[e as usize] = 1 as libc::c_int as libc::c_uchar;
            i = e + 1 as libc::c_int;
            while i < d {
                (*path[i as usize])
                    .head
                    .size = ((*path[i as usize]).head.size).wrapping_sub(1);
                i += 1;
            }
            (*r).head.size = ((*p).head.size).wrapping_sub(1 as libc::c_uint);
        }
    }
    i = d - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        krmq_update_min_lc_elem(
            path[i as usize],
            (*path[i as usize]).head.p[0 as libc::c_int as usize] as *const lc_elem_t,
            (*path[i as usize]).head.p[1 as libc::c_int as usize] as *const lc_elem_t,
        );
        i -= 1;
    }
    loop {
        d -= 1;
        if !(d > 0 as libc::c_int) {
            break;
        }
        q___0 = path[d as usize];
        b1 = 1 as libc::c_int;
        b2 = 2 as libc::c_int;
        which___0 = dir[d as usize] as libc::c_int;
        other = 1 as libc::c_int - which___0;
        if which___0 != 0 {
            b1 = -b1;
            b2 = -b2;
        }
        (*q___0)
            .head
            .balance = ((*q___0).head.balance as libc::c_int + b1) as libc::c_schar;
        if (*q___0).head.balance as libc::c_int == b1 {
            break;
        }
        if !((*q___0).head.balance as libc::c_int == b2) {
            continue;
        }
        r___0 = (*q___0).head.p[other as usize];
        if (*r___0).head.balance as libc::c_int == -b1 {
            (*path[(d - 1 as libc::c_int) as usize])
                .head
                .p[dir[(d - 1 as libc::c_int) as usize]
                as usize] = krmq_rotate2_lc_elem(q___0, which___0);
        } else {
            (*path[(d - 1 as libc::c_int) as usize])
                .head
                .p[dir[(d - 1 as libc::c_int) as usize]
                as usize] = krmq_rotate1_lc_elem(q___0, which___0);
            if (*r___0).head.balance as libc::c_int == 0 as libc::c_int {
                (*r___0).head.balance = -b1 as libc::c_schar;
                (*q___0).head.balance = b1 as libc::c_schar;
                break;
            } else {
                tmp___7 = 0 as libc::c_int as libc::c_schar;
                (*q___0).head.balance = tmp___7;
                (*r___0).head.balance = tmp___7;
            }
        }
    }
    *root_ = fake.head.p[0 as libc::c_int as usize];
    return p;
}
pub unsafe extern "C" fn krmq_itr_first_lc_elem(
    mut root: *const lc_elem_t,
    mut itr: *mut krmq_itr_lc_elem,
) {
    let mut p: *const lc_elem_t = 0 as *const lc_elem_t;
    (*itr).top = ((*itr).stack).as_mut_ptr().offset(-(1 as libc::c_int as isize));
    p = root;
    while !p.is_null() {
        (*itr).top = ((*itr).top).offset(1);
        *(*itr).top = p;
        p = (*p).head.p[0 as libc::c_int as usize] as *const lc_elem_t;
    }
}
pub unsafe extern "C" fn krmq_itr_find_lc_elem(
    mut root: *const lc_elem_t,
    mut x: *const lc_elem_t,
    mut itr: *mut krmq_itr_lc_elem,
) -> libc::c_int {
    let mut p: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut cmp: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    p = root;
    (*itr).top = ((*itr).stack).as_mut_ptr().offset(-(1 as libc::c_int as isize));
    while p as libc::c_ulong != 0 as *const lc_elem_t as libc::c_ulong {
        (*itr).top = ((*itr).top).offset(1);
        *(*itr).top = p;
        if (*x).y < (*p).y {
            cmp = -(1 as libc::c_int);
        } else {
            if (*x).y > (*p).y {
                tmp = 1 as libc::c_int;
            } else {
                tmp = ((*x).i > (*p).i) as libc::c_int
                    - ((*x).i < (*p).i) as libc::c_int;
            }
            cmp = tmp;
        }
        if cmp < 0 as libc::c_int {
            p = (*p).head.p[0 as libc::c_int as usize] as *const lc_elem_t;
        } else {
            if !(cmp > 0 as libc::c_int) {
                break;
            }
            p = (*p).head.p[1 as libc::c_int as usize] as *const lc_elem_t;
        }
    }
    if !p.is_null() {
        tmp___0 = 1 as libc::c_int;
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0;
}
pub unsafe extern "C" fn krmq_itr_next_bidir_lc_elem(
    mut itr: *mut krmq_itr_lc_elem,
    mut dir: libc::c_int,
) -> libc::c_int {
    let mut p: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut tmp: *mut *const lc_elem_t = 0 as *mut *const lc_elem_t;
    let mut tmp___0: libc::c_int = 0;
    if ((*itr).top as libc::c_ulong) < ((*itr).stack).as_mut_ptr() as libc::c_ulong {
        return 0 as libc::c_int;
    }
    dir = (dir != 0) as libc::c_int;
    p = (**(*itr).top).head.p[dir as usize] as *const lc_elem_t;
    if !p.is_null() {
        while !p.is_null() {
            (*itr).top = ((*itr).top).offset(1);
            *(*itr).top = p;
            p = (*p).head.p[(dir == 0) as libc::c_int as usize] as *const lc_elem_t;
        }
        return 1 as libc::c_int;
    } else {
        loop {
            tmp = (*itr).top;
            (*itr).top = ((*itr).top).offset(-1);
            p = *tmp;
            if !((*itr).top as libc::c_ulong
                >= ((*itr).stack).as_mut_ptr() as libc::c_ulong)
            {
                break;
            }
            if !(p as libc::c_ulong
                == (**(*itr).top).head.p[dir as usize] as libc::c_ulong)
            {
                break;
            }
        }
        if ((*itr).top as libc::c_ulong) < ((*itr).stack).as_mut_ptr() as libc::c_ulong {
            tmp___0 = 0 as libc::c_int;
        } else {
            tmp___0 = 1 as libc::c_int;
        }
        return tmp___0;
    };
}
#[inline]
unsafe extern "C" fn kmp_init_rmq(mut km: *mut libc::c_void) -> *mut kmp_rmq_t {
    let mut mp: *mut kmp_rmq_t = 0 as *mut kmp_rmq_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = kcalloc(
        km,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kmp_rmq_t>() as libc::c_ulong,
    );
    mp = tmp as *mut kmp_rmq_t;
    (*mp).km = km;
    return mp;
}
#[inline]
unsafe extern "C" fn kmp_alloc_rmq(mut mp: *mut kmp_rmq_t) -> *mut lc_elem_t {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    (*mp).cnt = ((*mp).cnt).wrapping_add(1);
    if (*mp).n == 0 as libc::c_ulong {
        tmp = kcalloc(
            (*mp).km,
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<lc_elem_t>() as libc::c_ulong,
        );
        return tmp as *mut lc_elem_t;
    }
    (*mp).n = ((*mp).n).wrapping_sub(1);
    return *((*mp).buf).offset((*mp).n as isize);
}
#[inline]
unsafe extern "C" fn kmp_free_rmq(mut mp: *mut kmp_rmq_t, mut p: *mut lc_elem_t) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: size_t = 0;
    (*mp).cnt = ((*mp).cnt).wrapping_sub(1);
    if (*mp).n == (*mp).max {
        if (*mp).max >= 4 as libc::c_ulong {
            (*mp)
                .max = ((*mp).max as libc::c_ulong)
                .wrapping_add((*mp).max >> 1 as libc::c_int) as size_t as size_t;
        } else {
            (*mp).max = 16 as libc::c_int as size_t;
        }
        tmp = krealloc(
            (*mp).km,
            (*mp).buf as *mut libc::c_void,
            ((*mp).max)
                .wrapping_mul(::std::mem::size_of::<*mut lc_elem_t>() as libc::c_ulong),
        );
        (*mp).buf = tmp as *mut *mut lc_elem_t;
    }
    tmp___0 = (*mp).n;
    (*mp).n = ((*mp).n).wrapping_add(1);
    let ref mut fresh30 = *((*mp).buf).offset(tmp___0 as isize);
    *fresh30 = p;
}
#[inline]
unsafe extern "C" fn comput_sc_simple(
    mut ai: *const mm128_t,
    mut aj: *const mm128_t,
    mut chn_pen_gap: libc::c_float,
    mut chn_pen_skip: libc::c_float,
    mut exact: *mut int32_t,
    mut width: *mut int32_t,
) -> int32_t {
    let mut dq: int32_t = 0;
    let mut dr: int32_t = 0;
    let mut dd: int32_t = 0;
    let mut dg: int32_t = 0;
    let mut q_span: int32_t = 0;
    let mut sc: int32_t = 0;
    let mut tmp: libc::c_int = 0;
    let mut lin_pen: libc::c_float = 0.;
    let mut log_pen: libc::c_float = 0.;
    let mut tmp___0: libc::c_float = 0.;
    dq = (*ai).y as int32_t - (*aj).y as int32_t;
    dr = ((*ai).x).wrapping_sub((*aj).x) as int32_t;
    if dr > dq {
        dd = dr - dq;
    } else {
        dd = dq - dr;
    }
    *width = dd;
    if dr < dq {
        dg = dr;
    } else {
        dg = dq;
    }
    q_span = ((*aj).y >> 32 as libc::c_int & 255 as libc::c_ulong) as int32_t;
    if q_span < dg {
        sc = q_span;
    } else {
        sc = dg;
    }
    if !exact.is_null() {
        if dd == 0 as libc::c_int {
            if dg <= q_span {
                tmp = 1 as libc::c_int;
            } else {
                tmp = 0 as libc::c_int;
            }
        } else {
            tmp = 0 as libc::c_int;
        }
        *exact = tmp;
    }
    let mut current_block_38: u64;
    if dd != 0 {
        current_block_38 = 9313074868891989557;
    } else if dq > q_span {
        current_block_38 = 9313074868891989557;
    } else {
        current_block_38 = 6417057564578538666;
    }
    match current_block_38 {
        9313074868891989557 => {
            lin_pen = chn_pen_gap * dd as libc::c_float
                + chn_pen_skip * dg as libc::c_float;
            if dd >= 1 as libc::c_int {
                tmp___0 = mg_log2___0((dd + 1 as libc::c_int) as libc::c_float);
                log_pen = tmp___0;
            } else {
                log_pen = 0.0f32;
            }
            sc -= (lin_pen + 0.5f32 * log_pen) as libc::c_int;
        }
        _ => {}
    }
    return sc;
}
pub unsafe extern "C" fn mg_lchain_rmq(
    mut max_dist: libc::c_int,
    mut max_dist_inner: libc::c_int,
    mut bw: libc::c_int,
    mut max_chn_skip: libc::c_int,
    mut cap_rmq_size: libc::c_int,
    mut min_cnt: libc::c_int,
    mut min_sc: libc::c_int,
    mut chn_pen_gap: libc::c_float,
    mut chn_pen_skip: libc::c_float,
    mut n: int64_t,
    mut a: *mut mm128_t,
    mut n_u_: *mut libc::c_int,
    mut _u: *mut *mut uint64_t,
    mut km: *mut libc::c_void,
) -> *mut mm128_t {
    let mut f: *mut int32_t = 0 as *mut int32_t;
    let mut t: *mut int32_t = 0 as *mut int32_t;
    let mut v: *mut int32_t = 0 as *mut int32_t;
    let mut n_u: int32_t = 0;
    let mut n_v: int32_t = 0;
    let mut mmax_f: int32_t = 0;
    let mut max_rmq_size: int32_t = 0;
    let mut max_drop: int32_t = 0;
    let mut p: *mut int64_t = 0 as *mut int64_t;
    let mut i: int64_t = 0;
    let mut i0: int64_t = 0;
    let mut st: int64_t = 0;
    let mut st_inner: int64_t = 0;
    let mut n_iter: int64_t = 0;
    let mut u: *mut uint64_t = 0 as *mut uint64_t;
    let mut root: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut root_inner: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut mem_mp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut mp: *mut kmp_rmq_t = 0 as *mut kmp_rmq_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut max_j: int64_t = 0;
    let mut q_span: int32_t = 0;
    let mut max_f: int32_t = 0;
    let mut s: lc_elem_t = lc_elem_t {
        y: 0,
        i: 0,
        pri: 0.,
        head: __anonstruct_head_172264678 {
            p: [0 as *mut lc_elem_s; 2],
            s: 0 as *mut lc_elem_s,
            balance: 0,
            size: 0,
        },
    };
    let mut q: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut r: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut lo: lc_elem_t = lc_elem_t {
        y: 0,
        i: 0,
        pri: 0.,
        head: __anonstruct_head_172264678 {
            p: [0 as *mut lc_elem_s; 2],
            s: 0 as *mut lc_elem_s,
            balance: 0,
            size: 0,
        },
    };
    let mut hi: lc_elem_t = lc_elem_t {
        y: 0,
        i: 0,
        pri: 0.,
        head: __anonstruct_head_172264678 {
            p: [0 as *mut lc_elem_s; 2],
            s: 0 as *mut lc_elem_s,
            balance: 0,
            size: 0,
        },
    };
    let mut j: int64_t = 0;
    let mut tmp___3: libc::c_uint = 0;
    let mut tmp___4: libc::c_uint = 0;
    let mut sc: int32_t = 0;
    let mut exact: int32_t = 0;
    let mut width: int32_t = 0;
    let mut n_skip: int32_t = 0;
    let mut j___0: int64_t = 0;
    let mut tmp___6: int32_t = 0;
    let mut lo___0: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut hi___0: *mut lc_elem_t = 0 as *mut lc_elem_t;
    let mut q___0: *const lc_elem_t = 0 as *const lc_elem_t;
    let mut width___0: int32_t = 0;
    let mut n_rmq_iter: int32_t = 0;
    let mut itr: krmq_itr_lc_elem = krmq_itr_lc_elem {
        stack: [0 as *const lc_elem_t; 64],
        top: 0 as *mut *const lc_elem_t,
    };
    let mut tmp___7: int32_t = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___10: libc::c_uint = 0;
    let mut tmp___11: *mut mm128_t = 0 as *mut mm128_t;
    mmax_f = 0 as libc::c_int;
    max_rmq_size = 0 as libc::c_int;
    max_drop = bw;
    st = 0 as libc::c_int as int64_t;
    st_inner = 0 as libc::c_int as int64_t;
    n_iter = 0 as libc::c_int as int64_t;
    root = 0 as *mut lc_elem_t;
    root_inner = 0 as *mut lc_elem_t;
    mem_mp = 0 as *mut libc::c_void;
    if !_u.is_null() {
        *_u = 0 as *mut uint64_t;
        *n_u_ = 0 as libc::c_int;
    }
    if n == 0 as libc::c_long {
        kfree(km, a as *mut libc::c_void);
        return 0 as *mut mm128_t;
    } else {
        if a as libc::c_ulong == 0 as *mut mm128_t as libc::c_ulong {
            kfree(km, a as *mut libc::c_void);
            return 0 as *mut mm128_t;
        }
    }
    if max_dist < bw {
        max_dist = bw;
    }
    if max_dist_inner <= 0 as libc::c_int {
        max_dist_inner = 0 as libc::c_int;
    } else if max_dist_inner >= max_dist {
        max_dist_inner = 0 as libc::c_int;
    }
    tmp = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int64_t>() as libc::c_ulong),
    );
    p = tmp as *mut int64_t;
    tmp___0 = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    );
    f = tmp___0 as *mut int32_t;
    tmp___1 = kcalloc(
        km,
        n as size_t,
        ::std::mem::size_of::<int32_t>() as libc::c_ulong,
    );
    t = tmp___1 as *mut int32_t;
    tmp___2 = kmalloc(
        km,
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int32_t>() as libc::c_ulong),
    );
    v = tmp___2 as *mut int32_t;
    mem_mp = km_init2(km, 65536 as libc::c_int as size_t);
    mp = kmp_init_rmq(mem_mp);
    i0 = 0 as libc::c_int as int64_t;
    i = i0;
    while i < n {
        max_j = -(1 as libc::c_int) as int64_t;
        q_span = ((*a.offset(i as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
            as int32_t;
        max_f = q_span;
        if i0 < i {
            if (*a.offset(i0 as isize)).x != (*a.offset(i as isize)).x {
                j = i0;
                while j < i {
                    q = kmp_alloc_rmq(mp);
                    (*q).y = (*a.offset(j as isize)).y as int32_t;
                    (*q).i = j;
                    (*q)
                        .pri = -(*f.offset(j as isize) as libc::c_double
                        + 0.5f64 * chn_pen_gap as libc::c_double
                            * ((*a.offset(j as isize)).x as int32_t
                                + (*a.offset(j as isize)).y as int32_t) as libc::c_double);
                    krmq_insert_lc_elem(&mut root, q, 0 as *mut libc::c_uint);
                    if max_dist_inner > 0 as libc::c_int {
                        r = kmp_alloc_rmq(mp);
                        *r = *q;
                        krmq_insert_lc_elem(&mut root_inner, r, 0 as *mut libc::c_uint);
                    }
                    j += 1;
                }
                i0 = i;
            }
        }
        while st < i {
            if !((*a.offset(i as isize)).x >> 32 as libc::c_int
                != (*a.offset(st as isize)).x >> 32 as libc::c_int)
            {
                if !((*a.offset(i as isize)).x
                    > ((*a.offset(st as isize)).x).wrapping_add(max_dist as uint64_t))
                {
                    if !root.is_null() {
                        tmp___3 = (*root).head.size;
                    } else {
                        tmp___3 = 0 as libc::c_uint;
                    }
                    if !(tmp___3 > cap_rmq_size as libc::c_uint) {
                        break;
                    }
                }
            }
            s.y = (*a.offset(st as isize)).y as int32_t;
            s.i = st;
            q = krmq_find_lc_elem(
                root as *const lc_elem_t,
                &mut s as *mut lc_elem_t as *const lc_elem_t,
                0 as *mut libc::c_uint,
            );
            if q as libc::c_ulong != 0 as *mut lc_elem_t as libc::c_ulong {
                q = krmq_erase_lc_elem(
                    &mut root,
                    q as *const lc_elem_t,
                    0 as *mut libc::c_uint,
                );
                kmp_free_rmq(mp, q);
            }
            st += 1;
        }
        if max_dist_inner > 0 as libc::c_int {
            while st_inner < i {
                if !((*a.offset(i as isize)).x >> 32 as libc::c_int
                    != (*a.offset(st_inner as isize)).x >> 32 as libc::c_int)
                {
                    if !((*a.offset(i as isize)).x
                        > ((*a.offset(st_inner as isize)).x)
                            .wrapping_add(max_dist_inner as uint64_t))
                    {
                        if !root_inner.is_null() {
                            tmp___4 = (*root_inner).head.size;
                        } else {
                            tmp___4 = 0 as libc::c_uint;
                        }
                        if !(tmp___4 > cap_rmq_size as libc::c_uint) {
                            break;
                        }
                    }
                }
                s.y = (*a.offset(st_inner as isize)).y as int32_t;
                s.i = st_inner;
                q = krmq_find_lc_elem(
                    root_inner as *const lc_elem_t,
                    &mut s as *mut lc_elem_t as *const lc_elem_t,
                    0 as *mut libc::c_uint,
                );
                if q as libc::c_ulong != 0 as *mut lc_elem_t as libc::c_ulong {
                    q = krmq_erase_lc_elem(
                        &mut root_inner,
                        q as *const lc_elem_t,
                        0 as *mut libc::c_uint,
                    );
                    kmp_free_rmq(mp, q);
                }
                st_inner += 1;
            }
        }
        lo.i = 2147483647 as libc::c_int as int64_t;
        lo.y = (*a.offset(i as isize)).y as int32_t - max_dist;
        hi.i = 0 as libc::c_int as int64_t;
        hi.y = (*a.offset(i as isize)).y as int32_t;
        q = krmq_rmq_lc_elem(
            root as *const lc_elem_t,
            &mut lo as *mut lc_elem_t as *const lc_elem_t,
            &mut hi as *mut lc_elem_t as *const lc_elem_t,
        );
        if q as libc::c_ulong != 0 as *mut lc_elem_t as libc::c_ulong {
            n_skip = 0 as libc::c_int;
            j___0 = (*q).i;
            if (*q).y >= lo.y {
                if !((*q).y <= hi.y) {
                    __assert_fail(
                        b"q->y >= lo.y && q->y <= hi.y\0" as *const u8
                            as *const libc::c_char,
                        b"lchain.c\0" as *const u8 as *const libc::c_char,
                        319 as libc::c_uint,
                        b"mg_lchain_rmq\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                __assert_fail(
                    b"q->y >= lo.y && q->y <= hi.y\0" as *const u8
                        as *const libc::c_char,
                    b"lchain.c\0" as *const u8 as *const libc::c_char,
                    319 as libc::c_uint,
                    b"mg_lchain_rmq\0" as *const u8 as *const libc::c_char,
                );
            }
            tmp___6 = comput_sc_simple(
                a.offset(i as isize) as *const mm128_t,
                a.offset(j___0 as isize) as *const mm128_t,
                chn_pen_gap,
                chn_pen_skip,
                &mut exact,
                &mut width,
            );
            sc = *f.offset(j___0 as isize) + tmp___6;
            if width <= bw {
                if sc > max_f {
                    max_f = sc;
                    max_j = j___0;
                }
            }
            if exact == 0 {
                if !root_inner.is_null() {
                    if (*a.offset(i as isize)).y as int32_t > 0 as libc::c_int {
                        s.y = (*a.offset(i as isize)).y as int32_t - 1 as libc::c_int;
                        s.i = n;
                        krmq_interval_lc_elem(
                            root_inner as *const lc_elem_t,
                            &mut s as *mut lc_elem_t as *const lc_elem_t,
                            &mut lo___0,
                            &mut hi___0,
                        );
                        if !lo___0.is_null() {
                            n_rmq_iter = 0 as libc::c_int;
                            krmq_itr_find_lc_elem(
                                root_inner as *const lc_elem_t,
                                lo___0 as *const lc_elem_t,
                                &mut itr,
                            );
                            loop {
                                if (itr.top as libc::c_ulong)
                                    < (itr.stack).as_mut_ptr() as libc::c_ulong
                                {
                                    q___0 = 0 as *const lc_elem_t;
                                } else {
                                    q___0 = *itr.top;
                                }
                                if !(q___0 as libc::c_ulong
                                    != 0 as *const lc_elem_t as libc::c_ulong)
                                {
                                    break;
                                }
                                if (*q___0).y
                                    < (*a.offset(i as isize)).y as int32_t - max_dist_inner
                                {
                                    break;
                                }
                                n_rmq_iter += 1;
                                j___0 = (*q___0).i;
                                tmp___7 = comput_sc_simple(
                                    a.offset(i as isize) as *const mm128_t,
                                    a.offset(j___0 as isize) as *const mm128_t,
                                    chn_pen_gap,
                                    chn_pen_skip,
                                    0 as *mut int32_t,
                                    &mut width___0,
                                );
                                sc = *f.offset(j___0 as isize) + tmp___7;
                                if width___0 <= bw {
                                    if sc > max_f {
                                        max_f = sc;
                                        max_j = j___0;
                                        if n_skip > 0 as libc::c_int {
                                            n_skip -= 1;
                                        }
                                    } else if *t.offset(j___0 as isize) == i as int32_t {
                                        n_skip += 1;
                                        if n_skip > max_chn_skip {
                                            break;
                                        }
                                    }
                                    if *p.offset(j___0 as isize) >= 0 as libc::c_long {
                                        *t
                                            .offset(*p.offset(j___0 as isize) as isize) = i as int32_t;
                                    }
                                }
                                tmp___8 = krmq_itr_next_bidir_lc_elem(
                                    &mut itr,
                                    0 as libc::c_int,
                                );
                                if tmp___8 == 0 {
                                    break;
                                }
                            }
                            n_iter += n_rmq_iter as int64_t;
                        }
                    }
                }
            }
        }
        if !(max_j < 0 as libc::c_long) {
            if (*a.offset(max_j as isize)).x < (*a.offset(i as isize)).x {
                if !(((*a.offset(max_j as isize)).y as int32_t)
                    < (*a.offset(i as isize)).y as int32_t)
                {
                    __assert_fail(
                        b"max_j < 0 || (a[max_j].x < a[i].x && (int32_t)a[max_j].y < (int32_t)a[i].y)\0"
                            as *const u8 as *const libc::c_char,
                        b"lchain.c\0" as *const u8 as *const libc::c_char,
                        353 as libc::c_uint,
                        b"mg_lchain_rmq\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                __assert_fail(
                    b"max_j < 0 || (a[max_j].x < a[i].x && (int32_t)a[max_j].y < (int32_t)a[i].y)\0"
                        as *const u8 as *const libc::c_char,
                    b"lchain.c\0" as *const u8 as *const libc::c_char,
                    353 as libc::c_uint,
                    b"mg_lchain_rmq\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        *f.offset(i as isize) = max_f;
        *p.offset(i as isize) = max_j;
        if max_j >= 0 as libc::c_long {
            if *v.offset(max_j as isize) > max_f {
                *v.offset(i as isize) = *v.offset(max_j as isize);
            } else {
                *v.offset(i as isize) = max_f;
            }
        } else {
            *v.offset(i as isize) = max_f;
        }
        if mmax_f < max_f {
            mmax_f = max_f;
        }
        if !root.is_null() {
            tmp___10 = (*root).head.size;
        } else {
            tmp___10 = 0 as libc::c_uint;
        }
        if (max_rmq_size as libc::c_uint) < tmp___10 {
            if !root.is_null() {
                max_rmq_size = (*root).head.size as int32_t;
            } else {
                max_rmq_size = 0 as libc::c_int;
            }
        }
        i += 1;
    }
    km_destroy(mem_mp);
    u = mg_chain_backtrack(
        km,
        n,
        f as *const int32_t,
        p as *const int64_t,
        v,
        t,
        min_cnt,
        min_sc,
        max_drop,
        &mut n_u,
        &mut n_v,
    );
    *n_u_ = n_u;
    *_u = u;
    kfree(km, p as *mut libc::c_void);
    kfree(km, f as *mut libc::c_void);
    kfree(km, t as *mut libc::c_void);
    if n_u == 0 as libc::c_int {
        kfree(km, a as *mut libc::c_void);
        kfree(km, v as *mut libc::c_void);
        return 0 as *mut mm128_t;
    }
    tmp___11 = compact_a(km, n_u, u, n_v, v, a);
    return tmp___11;
}
static mut KETOPT_INIT: ketopt_t = {
    let mut init = __anonstruct_ketopt_t_498465920 {
        ind: 1 as libc::c_int,
        opt: 0 as libc::c_int,
        arg: 0 as *const libc::c_char as *mut libc::c_char,
        longidx: -(1 as libc::c_int),
        i: 1 as libc::c_int,
        pos: 0 as libc::c_int,
        n_args: 0 as libc::c_int,
    };
    init
};
unsafe extern "C" fn ketopt_permute(
    mut argv: *mut *mut libc::c_char,
    mut j: libc::c_int,
    mut n: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = *argv.offset(j as isize);
    k = 0 as libc::c_int;
    while k < n {
        let ref mut fresh31 = *argv.offset((j - k) as isize);
        *fresh31 = *argv.offset((j - k - 1 as libc::c_int) as isize);
        k += 1;
    }
    let ref mut fresh32 = *argv.offset((j - k) as isize);
    *fresh32 = p;
}
unsafe extern "C" fn ketopt(
    mut s: *mut ketopt_t,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut permute: libc::c_int,
    mut ostr: *const libc::c_char,
    mut longopts: *const ko_longopt_t,
) -> libc::c_int {
    let mut opt: libc::c_int = 0;
    let mut i0: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n_exact: libc::c_int = 0;
    let mut n_partial: libc::c_int = 0;
    let mut o: *const ko_longopt_t = 0 as *const ko_longopt_t;
    let mut o_exact: *const ko_longopt_t = 0 as *const ko_longopt_t;
    let mut o_partial: *const ko_longopt_t = 0 as *const ko_longopt_t;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *const ko_longopt_t = 0 as *const ko_longopt_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    opt = -(1 as libc::c_int);
    if permute != 0 {
        while (*s).i < argc {
            if !(*(*argv.offset((*s).i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int != 45 as libc::c_int)
            {
                if !(*(*argv.offset((*s).i as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int == 0 as libc::c_int)
                {
                    break;
                }
            }
            (*s).i += 1;
            (*s).n_args += 1;
        }
    }
    (*s).arg = 0 as *mut libc::c_char;
    (*s).longidx = -(1 as libc::c_int);
    i0 = (*s).i;
    if (*s).i >= argc {
        (*s).ind = (*s).i - (*s).n_args;
        return -(1 as libc::c_int);
    } else {
        if *(*argv.offset((*s).i as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != 45 as libc::c_int
        {
            (*s).ind = (*s).i - (*s).n_args;
            return -(1 as libc::c_int);
        } else {
            if *(*argv.offset((*s).i as isize)).offset(1 as libc::c_int as isize)
                as libc::c_int == 0 as libc::c_int
            {
                (*s).ind = (*s).i - (*s).n_args;
                return -(1 as libc::c_int);
            }
        }
    }
    let mut current_block_115: u64;
    if *(*argv.offset((*s).i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        == 45 as libc::c_int
    {
        if *(*argv.offset((*s).i as isize)).offset(1 as libc::c_int as isize)
            as libc::c_int == 45 as libc::c_int
        {
            if *(*argv.offset((*s).i as isize)).offset(2 as libc::c_int as isize)
                as libc::c_int == 0 as libc::c_int
            {
                ketopt_permute(argv, (*s).i, (*s).n_args);
                (*s).i += 1;
                (*s).ind = (*s).i - (*s).n_args;
                return -(1 as libc::c_int);
            }
            (*s).opt = 0 as libc::c_int;
            opt = '?' as i32;
            (*s).pos = -(1 as libc::c_int);
            if !longopts.is_null() {
                n_exact = 0 as libc::c_int;
                n_partial = 0 as libc::c_int;
                o = 0 as *const ko_longopt_t;
                o_exact = 0 as *const ko_longopt_t;
                o_partial = 0 as *const ko_longopt_t;
                j = 2 as libc::c_int;
                while *(*argv.offset((*s).i as isize)).offset(j as isize) as libc::c_int
                    != 0 as libc::c_int
                {
                    if !(*(*argv.offset((*s).i as isize)).offset(j as isize)
                        as libc::c_int != 61 as libc::c_int)
                    {
                        break;
                    }
                    j += 1;
                }
                k = 0 as libc::c_int;
                while (*longopts.offset(k as isize)).name as libc::c_ulong
                    != 0 as *mut libc::c_char as libc::c_ulong
                {
                    tmp = strncmp(
                        (*argv.offset((*s).i as isize)).offset(2 as libc::c_int as isize)
                            as *const libc::c_char,
                        (*longopts.offset(k as isize)).name as *const libc::c_char,
                        (j - 2 as libc::c_int) as size_t,
                    );
                    if tmp == 0 as libc::c_int {
                        if *((*longopts.offset(k as isize)).name)
                            .offset((j - 2 as libc::c_int) as isize) as libc::c_int
                            == 0 as libc::c_int
                        {
                            n_exact += 1;
                            o_exact = longopts.offset(k as isize);
                        } else {
                            n_partial += 1;
                            o_partial = longopts.offset(k as isize);
                        }
                    }
                    k += 1;
                }
                if n_exact > 1 as libc::c_int {
                    return '?' as i32
                } else {
                    if n_exact == 0 as libc::c_int {
                        if n_partial > 1 as libc::c_int {
                            return '?' as i32;
                        }
                    }
                }
                if n_exact == 1 as libc::c_int {
                    o = o_exact;
                } else {
                    if n_partial == 1 as libc::c_int {
                        tmp___0 = o_partial;
                    } else {
                        tmp___0 = 0 as *const ko_longopt_t;
                    }
                    o = tmp___0;
                }
                if !o.is_null() {
                    opt = (*o).val;
                    (*s).opt = opt;
                    (*s)
                        .longidx = o.offset_from(longopts) as libc::c_long
                        as libc::c_int;
                    if *(*argv.offset((*s).i as isize)).offset(j as isize) as libc::c_int
                        == 61 as libc::c_int
                    {
                        (*s)
                            .arg = (*argv.offset((*s).i as isize))
                            .offset((j + 1 as libc::c_int) as isize);
                    }
                    if (*o).has_arg == 1 as libc::c_int {
                        if *(*argv.offset((*s).i as isize)).offset(j as isize)
                            as libc::c_int == 0 as libc::c_int
                        {
                            if (*s).i < argc - 1 as libc::c_int {
                                (*s).i += 1;
                                (*s).arg = *argv.offset((*s).i as isize);
                            } else {
                                opt = ':' as i32;
                            }
                        }
                    }
                }
            }
            current_block_115 = 8140372313878014523;
        } else {
            current_block_115 = 7563875733947748299;
        }
    } else {
        current_block_115 = 7563875733947748299;
    }
    match current_block_115 {
        7563875733947748299 => {
            if (*s).pos == 0 as libc::c_int {
                (*s).pos = 1 as libc::c_int;
            }
            tmp___2 = (*s).pos;
            (*s).pos += 1;
            tmp___1 = *(*argv.offset((*s).i as isize)).offset(tmp___2 as isize)
                as libc::c_int;
            (*s).opt = tmp___1;
            opt = tmp___1;
            p = strchr(ostr as *mut libc::c_char as *const libc::c_char, opt);
            if p as libc::c_ulong == 0 as *mut libc::c_char as libc::c_ulong {
                opt = '?' as i32;
            } else if *p.offset(1 as libc::c_int as isize) as libc::c_int
                    == 58 as libc::c_int
                {
                if *(*argv.offset((*s).i as isize)).offset((*s).pos as isize)
                    as libc::c_int == 0 as libc::c_int
                {
                    if (*s).i < argc - 1 as libc::c_int {
                        (*s).i += 1;
                        (*s).arg = *argv.offset((*s).i as isize);
                    } else {
                        opt = ':' as i32;
                    }
                } else {
                    (*s).arg = (*argv.offset((*s).i as isize)).offset((*s).pos as isize);
                }
                (*s).pos = -(1 as libc::c_int);
            }
        }
        _ => {}
    }
    let mut current_block_125: u64;
    if (*s).pos < 0 as libc::c_int {
        current_block_125 = 10292470290001503021;
    } else if *(*argv.offset((*s).i as isize)).offset((*s).pos as isize) as libc::c_int
            == 0 as libc::c_int
        {
        current_block_125 = 10292470290001503021;
    } else {
        current_block_125 = 17623951255125923504;
    }
    match current_block_125 {
        10292470290001503021 => {
            (*s).i += 1;
            (*s).pos = 0 as libc::c_int;
            if (*s).n_args > 0 as libc::c_int {
                j = i0;
                while j < (*s).i {
                    ketopt_permute(argv, j, (*s).n_args);
                    j += 1;
                }
            }
        }
        _ => {}
    }
    (*s).ind = (*s).i - (*s).n_args;
    return opt;
}
pub unsafe extern "C" fn liftrlimit() {
    let mut r: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    getrlimit(9 as libc::c_int, &mut r);
    r.rlim_cur = r.rlim_max;
    setrlimit(9 as libc::c_int, &mut r as *mut rlimit as *const rlimit);
}
static mut long_options: [ko_longopt_t; 63] = [
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"bucket-bits\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 300 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"mb-size\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'K' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"seed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 302 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"no-kalloc\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 303 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"print-qname\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 304 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"no-self\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 'D' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"print-seeds\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 306 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"max-chain-skip\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 307 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"min-dp-len\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 308 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"print-aln-seq\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 309 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"splice\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 310 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"cost-non-gt-ag\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'C' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"no-long-join\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 312 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"sr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 313 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"frag\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 314 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"secondary\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 315 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"cs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 2 as libc::c_int,
            val: 316 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"end-bonus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 317 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"no-pairing\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 318 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"splice-flank\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 319 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"idx-no-seq\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 320 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"end-seed-pen\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 321 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"for-only\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 322 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"rev-only\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 323 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"heap-sort\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 324 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"all-chain\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 'P' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"dual\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 326 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"max-clip-ratio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 327 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"min-occ-floor\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 328 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"MD\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 329 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"lj-min-ratio\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 330 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"score-N\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 331 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"eqx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 332 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"paf-no-hit\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 333 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"split-prefix\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 334 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"no-end-flt\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 335 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"hard-mask-level\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 336 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"cap-sw-mem\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 337 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"max-qlen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 338 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"max-chain-iter\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 339 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"junc-bed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 340 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"junc-bonus\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 341 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"sam-hit-only\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 342 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"chain-gap-scale\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 343 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"alt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 344 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"alt-drop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 345 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"mask-len\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 346 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"rmq\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 2 as libc::c_int,
            val: 347 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"qstrand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 348 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"cap-kalloc\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 349 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"q-occ-frac\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 350 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"chain-skip-scale\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 351 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"print-chains\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 352 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"no-hash-name\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 353 as libc::c_int,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 'h' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"max-intron-len\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'G' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 'V' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"min-count\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'n' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"min-chain-score\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'm' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"mask-level\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 'M' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"min-dp-score\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            has_arg: 1 as libc::c_int,
            val: 's' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: b"sam\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 'a' as i32,
        };
        init
    },
    {
        let mut init = __anonstruct_ko_longopt_t_483049182 {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            has_arg: 0 as libc::c_int,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[inline]
unsafe extern "C" fn mm_parse_num2(
    mut str: *const libc::c_char,
    mut q: *mut *mut libc::c_char,
) -> int64_t {
    let mut x: libc::c_double = 0.;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    x = strtod(str, &mut p as *mut *mut libc::c_char);
    if *p as libc::c_int == 71 as libc::c_int {
        x *= 1e9f64;
        p = p.offset(1);
    } else if *p as libc::c_int == 103 as libc::c_int {
        x *= 1e9f64;
        p = p.offset(1);
    } else if *p as libc::c_int == 77 as libc::c_int {
        x *= 1e6f64;
        p = p.offset(1);
    } else if *p as libc::c_int == 109 as libc::c_int {
        x *= 1e6f64;
        p = p.offset(1);
    } else if *p as libc::c_int == 75 as libc::c_int {
        x *= 1e3f64;
        p = p.offset(1);
    } else if *p as libc::c_int == 107 as libc::c_int {
        x *= 1e3f64;
        p = p.offset(1);
    }
    if !q.is_null() {
        *q = p;
    }
    return (x + 0.499f64) as int64_t;
}
#[inline]
unsafe extern "C" fn mm_parse_num(mut str: *const libc::c_char) -> int64_t {
    let mut tmp: int64_t = 0;
    tmp = mm_parse_num2(str, 0 as *mut *mut libc::c_char);
    return tmp;
}
#[inline]
unsafe extern "C" fn yes_or_no(
    mut opt: *mut mm_mapopt_t,
    mut flag: int64_t,
    mut long_idx: libc::c_int,
    mut arg: *const libc::c_char,
    mut yes_to_set: libc::c_int,
) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    if yes_to_set != 0 {
        tmp___1 = strcmp(arg, b"yes\0" as *const u8 as *const libc::c_char);
        if tmp___1 == 0 as libc::c_int {
            (*opt).flag |= flag;
        } else {
            tmp___2 = strcmp(arg, b"y\0" as *const u8 as *const libc::c_char);
            if tmp___2 == 0 as libc::c_int {
                (*opt).flag |= flag;
            } else {
                tmp = strcmp(arg, b"no\0" as *const u8 as *const libc::c_char);
                if tmp == 0 as libc::c_int {
                    (*opt).flag &= !flag;
                } else {
                    tmp___0 = strcmp(arg, b"n\0" as *const u8 as *const libc::c_char);
                    if tmp___0 == 0 as libc::c_int {
                        (*opt).flag &= !flag;
                    } else {
                        fprintf(
                            stderr,
                            b"[WARNING]\x1B[1;31m option '--%s' only accepts 'yes' or 'no'.\x1B[0m\n\0"
                                as *const u8 as *const libc::c_char,
                            long_options[long_idx as usize].name,
                        );
                    }
                }
            }
        }
    } else {
        tmp___5 = strcmp(arg, b"yes\0" as *const u8 as *const libc::c_char);
        if tmp___5 == 0 as libc::c_int {
            (*opt).flag &= !flag;
        } else {
            tmp___6 = strcmp(arg, b"y\0" as *const u8 as *const libc::c_char);
            if tmp___6 == 0 as libc::c_int {
                (*opt).flag &= !flag;
            } else {
                tmp___3 = strcmp(arg, b"no\0" as *const u8 as *const libc::c_char);
                if tmp___3 == 0 as libc::c_int {
                    (*opt).flag |= flag;
                } else {
                    tmp___4 = strcmp(arg, b"n\0" as *const u8 as *const libc::c_char);
                    if tmp___4 == 0 as libc::c_int {
                        (*opt).flag |= flag;
                    } else {
                        fprintf(
                            stderr,
                            b"[WARNING]\x1B[1;31m option '--%s' only accepts 'yes' or 'no'.\x1B[0m\n\0"
                                as *const u8 as *const libc::c_char,
                            long_options[long_idx as usize].name,
                        );
                    }
                }
            }
        }
    };
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut opt_str: *const libc::c_char = 0 as *const libc::c_char;
    let mut o: ketopt_t = ketopt_t {
        ind: 0,
        opt: 0,
        arg: 0 as *mut libc::c_char,
        longidx: 0,
        i: 0,
        pos: 0,
        n_args: 0,
    };
    let mut opt: mm_mapopt_t = mm_mapopt_t {
        flag: 0,
        seed: 0,
        sdust_thres: 0,
        max_qlen: 0,
        bw: 0,
        bw_long: 0,
        max_gap: 0,
        max_gap_ref: 0,
        max_frag_len: 0,
        max_chain_skip: 0,
        max_chain_iter: 0,
        min_cnt: 0,
        min_chain_score: 0,
        chain_gap_scale: 0.,
        chain_skip_scale: 0.,
        rmq_size_cap: 0,
        rmq_inner_dist: 0,
        rmq_rescue_size: 0,
        rmq_rescue_ratio: 0.,
        mask_level: 0.,
        mask_len: 0,
        pri_ratio: 0.,
        best_n: 0,
        alt_drop: 0.,
        a: 0,
        b: 0,
        q: 0,
        e: 0,
        q2: 0,
        e2: 0,
        sc_ambi: 0,
        noncan: 0,
        junc_bonus: 0,
        zdrop: 0,
        zdrop_inv: 0,
        end_bonus: 0,
        min_dp_max: 0,
        min_ksw_len: 0,
        anchor_ext_len: 0,
        anchor_ext_shift: 0,
        max_clip_ratio: 0.,
        rank_min_len: 0,
        rank_frac: 0.,
        pe_ori: 0,
        pe_bonus: 0,
        mid_occ_frac: 0.,
        q_occ_frac: 0.,
        min_mid_occ: 0,
        max_mid_occ: 0,
        mid_occ: 0,
        max_occ: 0,
        max_max_occ: 0,
        occ_dist: 0,
        mini_batch_size: 0,
        max_sw_mat: 0,
        cap_kalloc: 0,
        split_prefix: 0 as *const libc::c_char,
    };
    let mut ipt: mm_idxopt_t = mm_idxopt_t {
        k: 0,
        w: 0,
        flag: 0,
        bucket_bits: 0,
        mini_batch_size: 0,
        batch_size: 0,
    };
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut n_threads: libc::c_int = 0;
    let mut n_parts: libc::c_int = 0;
    let mut old_best_n: libc::c_int = 0;
    let mut fnw: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut rg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut junc_bed: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut alt_list: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp_help: *mut FILE = 0 as *mut FILE;
    let mut idx_rdr: *mut mm_idx_reader_t = 0 as *mut mm_idx_reader_t;
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: int64_t = 0;
    let mut tmp___3: int64_t = 0;
    let mut tmp___4: int64_t = 0;
    let mut tmp___5: libc::c_double = 0.;
    let mut tmp___6: libc::c_double = 0.;
    let mut tmp___7: int64_t = 0;
    let mut tmp___8: int64_t = 0;
    let mut tmp___9: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___10: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___11: *mut FILE = 0 as *mut FILE;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_double = 0.;
    let mut tmp___15: int64_t = 0;
    let mut tmp___16: libc::c_double = 0.;
    let mut tmp___17: libc::c_double = 0.;
    let mut tmp___18: libc::c_double = 0.;
    let mut tmp___19: int64_t = 0;
    let mut tmp___20: libc::c_double = 0.;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_int = 0;
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___24: int64_t = 0;
    let mut tmp___25: int64_t = 0;
    let mut tmp___26: libc::c_long = 0;
    let mut tmp___27: libc::c_long = 0;
    let mut x: libc::c_double = 0.;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___28: libc::c_double = 0.;
    let mut tmp___29: libc::c_long = 0;
    let mut tmp___30: libc::c_long = 0;
    let mut tmp___31: libc::c_long = 0;
    let mut tmp___32: libc::c_long = 0;
    let mut tmp___33: libc::c_long = 0;
    let mut tmp___34: libc::c_long = 0;
    let mut tmp___35: libc::c_int = 0;
    let mut tmp___36: libc::c_int = 0;
    let mut tmp___37: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___38: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut tmp___39: libc::c_int = 0;
    let mut tmp___40: libc::c_double = 0.;
    let mut tmp___41: libc::c_double = 0.;
    let mut tmp___42: libc::c_double = 0.;
    let mut tmp___43: libc::c_int = 0;
    let mut tmp___44: libc::c_long = 0;
    let mut tmp___45: libc::c_double = 0.;
    let mut tmp___46: libc::c_double = 0.;
    opt_str = b"2aSDw:k:K:t:r:f:Vv:g:G:I:d:XT:s:x:Hcp:M:n:z:A:B:O:E:m:N:Qu:R:hF:LC:yYPo:e:U:\0"
        as *const u8 as *const libc::c_char;
    o = KETOPT_INIT;
    n_threads = 3 as libc::c_int;
    old_best_n = -(1 as libc::c_int);
    fnw = 0 as *mut libc::c_char;
    rg = 0 as *mut libc::c_char;
    junc_bed = 0 as *mut libc::c_char;
    alt_list = 0 as *mut libc::c_char;
    fp_help = stderr;
    mm_verbose = 3 as libc::c_int;
    liftrlimit();
    mm_realtime0 = realtime();
    mm_set_opt(0 as *const libc::c_char, &mut ipt, &mut opt);
    loop {
        c = ketopt(
            &mut o,
            argc,
            argv,
            1 as libc::c_int,
            opt_str,
            long_options.as_mut_ptr() as *const ko_longopt_t,
        );
        if !(c >= 0 as libc::c_int) {
            break;
        }
        if c == 120 as libc::c_int {
            tmp = mm_set_opt(o.arg as *const libc::c_char, &mut ipt, &mut opt);
            if tmp < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR] unknown preset '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    o.arg,
                );
                return 1 as libc::c_int;
            }
        } else if c == 58 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR] missing option argument\n\0" as *const u8
                    as *const libc::c_char,
            );
            return 1 as libc::c_int;
        } else {
            if c == 63 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR] unknown option in \"%s\"\n\0" as *const u8
                        as *const libc::c_char,
                    *argv.offset((o.i - 1 as libc::c_int) as isize),
                );
                return 1 as libc::c_int;
            }
        }
    }
    o = KETOPT_INIT;
    loop {
        c = ketopt(
            &mut o,
            argc,
            argv,
            1 as libc::c_int,
            opt_str,
            long_options.as_mut_ptr() as *const ko_longopt_t,
        );
        if !(c >= 0 as libc::c_int) {
            break;
        }
        if c == 119 as libc::c_int {
            tmp___0 = atoi(o.arg as *const libc::c_char);
            ipt.w = tmp___0 as libc::c_short;
        } else if c == 107 as libc::c_int {
            tmp___1 = atoi(o.arg as *const libc::c_char);
            ipt.k = tmp___1 as libc::c_short;
        } else if c == 72 as libc::c_int {
            ipt.flag = (ipt.flag as libc::c_int | 1 as libc::c_int) as libc::c_short;
        } else if c == 100 as libc::c_int {
            fnw = o.arg;
        } else if c == 116 as libc::c_int {
            n_threads = atoi(o.arg as *const libc::c_char);
        } else if c == 118 as libc::c_int {
            mm_verbose = atoi(o.arg as *const libc::c_char);
        } else if c == 103 as libc::c_int {
            tmp___2 = mm_parse_num(o.arg as *const libc::c_char);
            opt.max_gap = tmp___2 as libc::c_int;
        } else if c == 71 as libc::c_int {
            tmp___3 = mm_parse_num(o.arg as *const libc::c_char);
            mm_mapopt_max_intron_len(&mut opt, tmp___3 as libc::c_int);
        } else if c == 70 as libc::c_int {
            tmp___4 = mm_parse_num(o.arg as *const libc::c_char);
            opt.max_frag_len = tmp___4 as libc::c_int;
        } else if c == 78 as libc::c_int {
            old_best_n = opt.best_n;
            opt.best_n = atoi(o.arg as *const libc::c_char);
        } else if c == 112 as libc::c_int {
            tmp___5 = atof(o.arg as *const libc::c_char);
            opt.pri_ratio = tmp___5 as libc::c_float;
        } else if c == 77 as libc::c_int {
            tmp___6 = atof(o.arg as *const libc::c_char);
            opt.mask_level = tmp___6 as libc::c_float;
        } else if c == 99 as libc::c_int {
            opt.flag |= 36 as libc::c_long;
        } else if c == 68 as libc::c_int {
            opt.flag |= 1 as libc::c_long;
        } else if c == 80 as libc::c_int {
            opt.flag |= 8388608 as libc::c_long;
        } else if c == 88 as libc::c_int {
            opt.flag |= 8389635 as libc::c_long;
        } else if c == 97 as libc::c_int {
            opt.flag |= 12 as libc::c_long;
        } else if c == 81 as libc::c_int {
            opt.flag |= 16 as libc::c_long;
        } else if c == 89 as libc::c_int {
            opt.flag |= 524288 as libc::c_long;
        } else if c == 76 as libc::c_int {
            opt.flag |= 65536 as libc::c_long;
        } else if c == 121 as libc::c_int {
            opt.flag |= 33554432 as libc::c_long;
        } else if c == 84 as libc::c_int {
            opt.sdust_thres = atoi(o.arg as *const libc::c_char);
        } else if c == 110 as libc::c_int {
            opt.min_cnt = atoi(o.arg as *const libc::c_char);
        } else if c == 109 as libc::c_int {
            opt.min_chain_score = atoi(o.arg as *const libc::c_char);
        } else if c == 65 as libc::c_int {
            opt.a = atoi(o.arg as *const libc::c_char);
        } else if c == 66 as libc::c_int {
            opt.b = atoi(o.arg as *const libc::c_char);
        } else if c == 115 as libc::c_int {
            opt.min_dp_max = atoi(o.arg as *const libc::c_char);
        } else if c == 67 as libc::c_int {
            opt.noncan = atoi(o.arg as *const libc::c_char);
        } else if c == 73 as libc::c_int {
            tmp___7 = mm_parse_num(o.arg as *const libc::c_char);
            ipt.batch_size = tmp___7 as uint64_t;
        } else if c == 75 as libc::c_int {
            opt.mini_batch_size = mm_parse_num(o.arg as *const libc::c_char);
        } else if c == 101 as libc::c_int {
            tmp___8 = mm_parse_num(o.arg as *const libc::c_char);
            opt.occ_dist = tmp___8 as int32_t;
        } else if c == 82 as libc::c_int {
            rg = o.arg;
        } else if c == 104 as libc::c_int {
            fp_help = stdout;
        } else if c == 50 as libc::c_int {
            opt.flag |= 32768 as libc::c_long;
        } else if c == 111 as libc::c_int {
            tmp___12 = strcmp(
                o.arg as *const libc::c_char,
                b"-\0" as *const u8 as *const libc::c_char,
            );
            if tmp___12 != 0 as libc::c_int {
                tmp___11 = freopen(
                    o.arg as *const libc::c_char,
                    b"wb\0" as *const u8 as *const libc::c_char,
                    stdout,
                );
                if tmp___11 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    tmp___9 = __errno_location();
                    tmp___10 = strerror(*tmp___9);
                    fprintf(
                        stderr,
                        b"[ERROR]\x1B[1;31m failed to write the output to file '%s'\x1B[0m: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        o.arg,
                        tmp___10,
                    );
                    exit(1 as libc::c_int);
                }
            }
        } else if c == 300 as libc::c_int {
            tmp___13 = atoi(o.arg as *const libc::c_char);
            ipt.bucket_bits = tmp___13 as libc::c_short;
        } else if c == 302 as libc::c_int {
            opt.seed = atoi(o.arg as *const libc::c_char);
        } else if c == 303 as libc::c_int {
            mm_dbg_flag |= 1 as libc::c_int;
        } else if c == 304 as libc::c_int {
            mm_dbg_flag |= 2 as libc::c_int;
        } else if c == 306 as libc::c_int {
            mm_dbg_flag |= 6 as libc::c_int;
            n_threads = 1 as libc::c_int;
        } else if c == 307 as libc::c_int {
            opt.max_chain_skip = atoi(o.arg as *const libc::c_char);
        } else if c == 339 as libc::c_int {
            opt.max_chain_iter = atoi(o.arg as *const libc::c_char);
        } else if c == 308 as libc::c_int {
            opt.min_ksw_len = atoi(o.arg as *const libc::c_char);
        } else if c == 309 as libc::c_int {
            mm_dbg_flag |= 10 as libc::c_int;
            n_threads = 1 as libc::c_int;
        } else if c == 310 as libc::c_int {
            opt.flag |= 128 as libc::c_long;
        } else if c == 312 as libc::c_int {
            opt.flag |= 1024 as libc::c_long;
        } else if c == 313 as libc::c_int {
            opt.flag |= 4096 as libc::c_long;
        } else if c == 317 as libc::c_int {
            opt.end_bonus = atoi(o.arg as *const libc::c_char);
        } else if c == 318 as libc::c_int {
            opt.flag |= 131072 as libc::c_long;
        } else if c == 320 as libc::c_int {
            ipt.flag = (ipt.flag as libc::c_int | 2 as libc::c_int) as libc::c_short;
        } else if c == 321 as libc::c_int {
            opt.anchor_ext_shift = atoi(o.arg as *const libc::c_char);
        } else if c == 322 as libc::c_int {
            opt.flag |= 1048576 as libc::c_long;
        } else if c == 323 as libc::c_int {
            opt.flag |= 2097152 as libc::c_long;
        } else if c == 327 as libc::c_int {
            tmp___14 = atof(o.arg as *const libc::c_char);
            opt.max_clip_ratio = tmp___14 as libc::c_float;
        } else if c == 328 as libc::c_int {
            opt.min_mid_occ = atoi(o.arg as *const libc::c_char);
        } else if c == 329 as libc::c_int {
            opt.flag |= 16777216 as libc::c_long;
        } else if c == 331 as libc::c_int {
            opt.sc_ambi = atoi(o.arg as *const libc::c_char);
        } else if c == 332 as libc::c_int {
            opt.flag |= 67108864 as libc::c_long;
        } else if c == 333 as libc::c_int {
            opt.flag |= 134217728 as libc::c_long;
        } else if c == 334 as libc::c_int {
            opt.split_prefix = o.arg as *const libc::c_char;
        } else if c == 335 as libc::c_int {
            opt.flag |= 268435456 as libc::c_long;
        } else if c == 336 as libc::c_int {
            opt.flag |= 536870912 as libc::c_long;
        } else if c == 337 as libc::c_int {
            opt.max_sw_mat = mm_parse_num(o.arg as *const libc::c_char);
        } else if c == 338 as libc::c_int {
            tmp___15 = mm_parse_num(o.arg as *const libc::c_char);
            opt.max_qlen = tmp___15 as libc::c_int;
        } else if c == 340 as libc::c_int {
            junc_bed = o.arg;
        } else if c == 341 as libc::c_int {
            opt.junc_bonus = atoi(o.arg as *const libc::c_char);
        } else if c == 342 as libc::c_int {
            opt.flag |= 1073741824 as libc::c_long;
        } else if c == 343 as libc::c_int {
            tmp___16 = atof(o.arg as *const libc::c_char);
            opt.chain_gap_scale = tmp___16 as libc::c_float;
        } else if c == 351 as libc::c_int {
            tmp___17 = atof(o.arg as *const libc::c_char);
            opt.chain_skip_scale = tmp___17 as libc::c_float;
        } else if c == 344 as libc::c_int {
            alt_list = o.arg;
        } else if c == 345 as libc::c_int {
            tmp___18 = atof(o.arg as *const libc::c_char);
            opt.alt_drop = tmp___18 as libc::c_float;
        } else if c == 346 as libc::c_int {
            tmp___19 = mm_parse_num(o.arg as *const libc::c_char);
            opt.mask_len = tmp___19 as libc::c_int;
        } else if c == 348 as libc::c_int {
            opt
                .flag = (opt.flag as libc::c_longlong | 12884901888 as libc::c_longlong)
                as int64_t;
        } else if c == 349 as libc::c_int {
            opt.cap_kalloc = mm_parse_num(o.arg as *const libc::c_char);
        } else if c == 350 as libc::c_int {
            tmp___20 = atof(o.arg as *const libc::c_char);
            opt.q_occ_frac = tmp___20 as libc::c_float;
        } else if c == 352 as libc::c_int {
            mm_dbg_flag |= 16 as libc::c_int;
        } else if c == 353 as libc::c_int {
            opt
                .flag = (opt.flag as libc::c_longlong | 17179869184 as libc::c_longlong)
                as int64_t;
        } else if c == 330 as libc::c_int {
            fprintf(
                stderr,
                b"[WARNING] \x1B[1;31m --lj-min-ratio has been deprecated.\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if c == 314 as libc::c_int {
            yes_or_no(
                &mut opt,
                8192 as libc::c_int as int64_t,
                o.longidx,
                o.arg as *const libc::c_char,
                1 as libc::c_int,
            );
        } else if c == 315 as libc::c_int {
            yes_or_no(
                &mut opt,
                16384 as libc::c_int as int64_t,
                o.longidx,
                o.arg as *const libc::c_char,
                0 as libc::c_int,
            );
        } else if c == 316 as libc::c_int {
            opt.flag |= 68 as libc::c_long;
            if o.arg as libc::c_ulong == 0 as *mut libc::c_char as libc::c_ulong {
                opt.flag &= -(2049 as libc::c_long);
            } else {
                tmp___23 = strcmp(
                    o.arg as *const libc::c_char,
                    b"short\0" as *const u8 as *const libc::c_char,
                );
                if tmp___23 == 0 as libc::c_int {
                    opt.flag &= -(2049 as libc::c_long);
                } else {
                    tmp___22 = strcmp(
                        o.arg as *const libc::c_char,
                        b"long\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___22 == 0 as libc::c_int {
                        opt.flag |= 2048 as libc::c_long;
                    } else {
                        tmp___21 = strcmp(
                            o.arg as *const libc::c_char,
                            b"none\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___21 == 0 as libc::c_int {
                            opt.flag &= -(65 as libc::c_long);
                        } else if mm_verbose >= 2 as libc::c_int {
                            fprintf(
                                stderr,
                                b"[WARNING]\x1B[1;31m --cs only takes 'short' or 'long'. Invalid values are assumed to be 'short'.\x1B[0m\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
            }
        } else if c == 319 as libc::c_int {
            yes_or_no(
                &mut opt,
                262144 as libc::c_int as int64_t,
                o.longidx,
                o.arg as *const libc::c_char,
                1 as libc::c_int,
            );
        } else if c == 324 as libc::c_int {
            yes_or_no(
                &mut opt,
                4194304 as libc::c_int as int64_t,
                o.longidx,
                o.arg as *const libc::c_char,
                1 as libc::c_int,
            );
        } else if c == 326 as libc::c_int {
            yes_or_no(
                &mut opt,
                2 as libc::c_int as int64_t,
                o.longidx,
                o.arg as *const libc::c_char,
                0 as libc::c_int,
            );
        } else if c == 347 as libc::c_int {
            yes_or_no(
                &mut opt,
                2147483648 as libc::c_longlong as int64_t,
                o.longidx,
                o.arg as *const libc::c_char,
                1 as libc::c_int,
            );
        } else if c == 83 as libc::c_int {
            opt.flag |= 2116 as libc::c_long;
            if mm_verbose >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"[WARNING]\x1B[1;31m option -S is deprecated and may be removed in future. Please use --cs=long instead.\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        } else if c == 86 as libc::c_int {
            puts(b"2.24-r1122\0" as *const u8 as *const libc::c_char);
            return 0 as libc::c_int;
        } else {
            if c == 114 as libc::c_int {
                tmp___24 = mm_parse_num2(o.arg as *const libc::c_char, &mut s);
                opt.bw = tmp___24 as libc::c_int;
                if *s as libc::c_int == 44 as libc::c_int {
                    tmp___25 = mm_parse_num2(
                        s.offset(1 as libc::c_int as isize) as *const libc::c_char,
                        &mut s,
                    );
                    opt.bw_long = tmp___25 as libc::c_int;
                }
            } else if c == 85 as libc::c_int {
                tmp___26 = strtol(
                    o.arg as *const libc::c_char,
                    &mut s as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
                opt.min_mid_occ = tmp___26 as int32_t;
                if *s as libc::c_int == 44 as libc::c_int {
                    tmp___27 = strtol(
                        s.offset(1 as libc::c_int as isize) as *const libc::c_char,
                        &mut s as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    opt.max_mid_occ = tmp___27 as int32_t;
                }
            } else if c == 102 as libc::c_int {
                x = strtod(
                    o.arg as *const libc::c_char,
                    &mut p as *mut *mut libc::c_char,
                );
                if x < 1.0f64 {
                    opt.mid_occ_frac = x as libc::c_float;
                    opt.mid_occ = 0 as libc::c_int;
                } else {
                    opt.mid_occ = (x + 0.499f64) as libc::c_int;
                }
                if *p as libc::c_int == 44 as libc::c_int {
                    tmp___28 = strtod(
                        p.offset(1 as libc::c_int as isize) as *const libc::c_char,
                        &mut p as *mut *mut libc::c_char,
                    );
                    opt.max_occ = (tmp___28 + 0.499f64) as libc::c_int;
                }
            } else if c == 117 as libc::c_int {
                if *o.arg as libc::c_int == 98 as libc::c_int {
                    opt.flag |= 768 as libc::c_long;
                } else if *o.arg as libc::c_int == 102 as libc::c_int {
                    opt.flag |= 256 as libc::c_long;
                    opt.flag &= -(513 as libc::c_long);
                } else if *o.arg as libc::c_int == 114 as libc::c_int {
                    opt.flag |= 512 as libc::c_long;
                    opt.flag &= -(257 as libc::c_long);
                } else if *o.arg as libc::c_int == 110 as libc::c_int {
                    opt.flag &= -(769 as libc::c_long);
                } else {
                    fprintf(
                        stderr,
                        b"[ERROR]\x1B[1;31m unrecognized cDNA direction\x1B[0m\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return 1 as libc::c_int;
                }
            } else if c == 122 as libc::c_int {
                tmp___29 = strtol(
                    o.arg as *const libc::c_char,
                    &mut s as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
                opt.zdrop_inv = tmp___29 as libc::c_int;
                opt.zdrop = opt.zdrop_inv;
                if *s as libc::c_int == 44 as libc::c_int {
                    tmp___30 = strtol(
                        s.offset(1 as libc::c_int as isize) as *const libc::c_char,
                        &mut s as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    opt.zdrop_inv = tmp___30 as libc::c_int;
                }
            } else if c == 79 as libc::c_int {
                tmp___31 = strtol(
                    o.arg as *const libc::c_char,
                    &mut s as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
                opt.q2 = tmp___31 as libc::c_int;
                opt.q = opt.q2;
                if *s as libc::c_int == 44 as libc::c_int {
                    tmp___32 = strtol(
                        s.offset(1 as libc::c_int as isize) as *const libc::c_char,
                        &mut s as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    opt.q2 = tmp___32 as libc::c_int;
                }
            } else if c == 69 as libc::c_int {
                tmp___33 = strtol(
                    o.arg as *const libc::c_char,
                    &mut s as *mut *mut libc::c_char,
                    10 as libc::c_int,
                );
                opt.e2 = tmp___33 as libc::c_int;
                opt.e = opt.e2;
                if *s as libc::c_int == 44 as libc::c_int {
                    tmp___34 = strtol(
                        s.offset(1 as libc::c_int as isize) as *const libc::c_char,
                        &mut s as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    opt.e2 = tmp___34 as libc::c_int;
                }
            }
        }
    }
    if opt.flag & 128 as libc::c_long != 0 {
        if opt.flag & 8192 as libc::c_long != 0 {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m --splice and --frag should not be specified at the same time.\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    if fnw.is_null() {
        if opt.flag & 4 as libc::c_long == 0 {
            ipt.flag = (ipt.flag as libc::c_int | 2 as libc::c_int) as libc::c_short;
        }
    }
    tmp___35 = mm_check_opt(
        &mut ipt as *mut mm_idxopt_t as *const mm_idxopt_t,
        &mut opt as *mut mm_mapopt_t as *const mm_mapopt_t,
    );
    if tmp___35 < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if opt.best_n == 0 as libc::c_int {
        fprintf(
            stderr,
            b"[WARNING]\x1B[1;31m changed '-N 0' to '-N %d --secondary=no'.\x1B[0m\n\0"
                as *const u8 as *const libc::c_char,
            old_best_n,
        );
        opt.best_n = old_best_n;
        opt.flag |= 16384 as libc::c_long;
    }
    's_2139: {
        if !(argc == o.ind) {
            if !(fp_help as libc::c_ulong == stdout as libc::c_ulong) {
                break 's_2139;
            }
        }
        fprintf(
            fp_help,
            b"Usage: minimap2 [options] <target.fa>|<target.idx> [query.fa] [...]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(fp_help, b"Options:\n\0" as *const u8 as *const libc::c_char);
        fprintf(fp_help, b"  Indexing:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp_help,
            b"    -H           use homopolymer-compressed k-mer (preferrable for PacBio)\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -k INT       k-mer size (no larger than 28) [%d]\n\0" as *const u8
                as *const libc::c_char,
            ipt.k as libc::c_int,
        );
        fprintf(
            fp_help,
            b"    -w INT       minimizer window size [%d]\n\0" as *const u8
                as *const libc::c_char,
            ipt.w as libc::c_int,
        );
        fprintf(
            fp_help,
            b"    -I NUM       split index for every ~NUM input bases [4G]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -d FILE      dump index to FILE []\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(fp_help, b"  Mapping:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp_help,
            b"    -f FLOAT     filter out top FLOAT fraction of repetitive minimizers [%g]\n\0"
                as *const u8 as *const libc::c_char,
            opt.mid_occ_frac as libc::c_double,
        );
        fprintf(
            fp_help,
            b"    -g NUM       stop chain enlongation if there are no minimizers in INT-bp [%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.max_gap,
        );
        fprintf(
            fp_help,
            b"    -G NUM       max intron length (effective with -xsplice; changing -r) [200k]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -F NUM       max fragment length (effective with -xsr or in the fragment mode) [800]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -r NUM[,NUM] chaining/alignment bandwidth and long-join bandwidth [%d,%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.bw,
            opt.bw_long,
        );
        fprintf(
            fp_help,
            b"    -n INT       minimal number of minimizers on a chain [%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.min_cnt,
        );
        fprintf(
            fp_help,
            b"    -m INT       minimal chaining score (matching bases minus log gap penalty) [%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.min_chain_score,
        );
        fprintf(
            fp_help,
            b"    -X           skip self and dual mappings (for the all-vs-all mode)\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -p FLOAT     min secondary-to-primary score ratio [%g]\n\0"
                as *const u8 as *const libc::c_char,
            opt.pri_ratio as libc::c_double,
        );
        fprintf(
            fp_help,
            b"    -N INT       retain at most INT secondary alignments [%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.best_n,
        );
        fprintf(fp_help, b"  Alignment:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp_help,
            b"    -A INT       matching score [%d]\n\0" as *const u8
                as *const libc::c_char,
            opt.a,
        );
        fprintf(
            fp_help,
            b"    -B INT       mismatch penalty (larger value for lower divergence) [%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.b,
        );
        fprintf(
            fp_help,
            b"    -O INT[,INT] gap open penalty [%d,%d]\n\0" as *const u8
                as *const libc::c_char,
            opt.q,
            opt.q2,
        );
        fprintf(
            fp_help,
            b"    -E INT[,INT] gap extension penalty; a k-long gap costs min{O1+k*E1,O2+k*E2} [%d,%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.e,
            opt.e2,
        );
        fprintf(
            fp_help,
            b"    -z INT[,INT] Z-drop score and inversion Z-drop score [%d,%d]\n\0"
                as *const u8 as *const libc::c_char,
            opt.zdrop,
            opt.zdrop_inv,
        );
        fprintf(
            fp_help,
            b"    -s INT       minimal peak DP alignment score [%d]\n\0" as *const u8
                as *const libc::c_char,
            opt.min_dp_max,
        );
        fprintf(
            fp_help,
            b"    -u CHAR      how to find GT-AG. f:transcript strand, b:both strands, n:don't match GT-AG [n]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(fp_help, b"  Input/Output:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp_help,
            b"    -a           output in the SAM format (PAF by default)\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -o FILE      output alignments to FILE [stdout]\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -L           write CIGAR with >65535 ops at the CG tag\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -R STR       SAM read group line in a format like '@RG\\tID:foo\\tSM:bar' []\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -c           output CIGAR in PAF\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    --cs[=STR]   output the cs tag; STR is 'short' (if absent) or 'long' [none]\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    --MD         output the MD tag\n\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    --eqx        write =/X CIGAR operators\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -Y           use soft clipping for supplementary alignments\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    -t INT       number of threads [%d]\n\0" as *const u8
                as *const libc::c_char,
            n_threads,
        );
        fprintf(
            fp_help,
            b"    -K NUM       minibatch size for mapping [500M]\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"    --version    show version number\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(fp_help, b"  Preset:\n\0" as *const u8 as *const libc::c_char);
        fprintf(
            fp_help,
            b"    -x STR       preset (always applied before other options; see minimap2.1 for details) []\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - map-pb/map-ont - PacBio CLR/Nanopore vs reference mapping\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - map-hifi - PacBio HiFi reads vs reference mapping\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - ava-pb/ava-ont - PacBio/Nanopore read overlap\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - asm5/asm10/asm20 - asm-to-ref mapping, for ~0.1/1/5%% sequence divergence\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - splice/splice:hq - long-read/Pacbio-CCS spliced alignment\n\0"
                as *const u8 as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"                 - sr - genomic short-read mapping\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            fp_help,
            b"\nSee `man ./minimap2.1' for detailed description of these and other advanced command-line options.\n\0"
                as *const u8 as *const libc::c_char,
        );
        if fp_help as libc::c_ulong == stdout as libc::c_ulong {
            tmp___36 = 0 as libc::c_int;
        } else {
            tmp___36 = 1 as libc::c_int;
        }
        return tmp___36;
    }
    if opt.flag & 4096 as libc::c_long != 0 {
        if argc - o.ind > 3 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR] incorrect input: in the sr mode, please specify no more than two query files.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
    }
    idx_rdr = mm_idx_reader_open(
        *argv.offset(o.ind as isize) as *const libc::c_char,
        &mut ipt as *mut mm_idxopt_t as *const mm_idxopt_t,
        fnw as *const libc::c_char,
    );
    if idx_rdr as libc::c_ulong == 0 as *mut mm_idx_reader_t as libc::c_ulong {
        tmp___37 = __errno_location();
        tmp___38 = strerror(*tmp___37);
        fprintf(
            stderr,
            b"[ERROR] failed to open file '%s': %s\n\0" as *const u8
                as *const libc::c_char,
            *argv.offset(o.ind as isize),
            tmp___38,
        );
        return 1 as libc::c_int;
    }
    if (*idx_rdr).is_idx == 0 {
        if fnw as libc::c_ulong == 0 as *mut libc::c_char as libc::c_ulong {
            if argc - o.ind < 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR] missing input: please specify a query file to map or option -d to keep the index\n\0"
                        as *const u8 as *const libc::c_char,
                );
                mm_idx_reader_close(idx_rdr);
                return 1 as libc::c_int;
            }
        }
    }
    if opt.best_n == 0 as libc::c_int {
        if opt.flag & 4 as libc::c_long != 0 {
            if mm_verbose >= 2 as libc::c_int {
                fprintf(
                    stderr,
                    b"[WARNING]\x1B[1;31m `-N 0' reduces alignment accuracy. Please use --secondary=no to suppress secondary alignments.\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    loop {
        mi = mm_idx_reader_read(idx_rdr, n_threads);
        if !(mi as libc::c_ulong != 0 as *mut mm_idx_t as libc::c_ulong) {
            break;
        }
        if opt.flag & 4 as libc::c_long != 0 {
            if (*mi).flag & 2 as libc::c_int != 0 {
                fprintf(
                    stderr,
                    b"[ERROR] the prebuilt index doesn't contain sequences.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                mm_idx_destroy(mi);
                mm_idx_reader_close(idx_rdr);
                return 1 as libc::c_int;
            }
        }
        if opt.flag & 8 as libc::c_long != 0 {
            if (*idx_rdr).n_parts == 1 as libc::c_int {
                tmp___39 = mm_idx_reader_eof(idx_rdr as *const mm_idx_reader_t);
                if tmp___39 != 0 {
                    if opt.split_prefix as libc::c_ulong
                        == 0 as *const libc::c_char as libc::c_ulong
                    {
                        ret = mm_write_sam_hdr(
                            mi as *const mm_idx_t,
                            rg as *const libc::c_char,
                            b"2.24-r1122\0" as *const u8 as *const libc::c_char,
                            argc,
                            argv,
                        );
                    } else {
                        ret = mm_write_sam_hdr(
                            0 as *const mm_idx_t,
                            rg as *const libc::c_char,
                            b"2.24-r1122\0" as *const u8 as *const libc::c_char,
                            argc,
                            argv,
                        );
                    }
                } else {
                    ret = mm_write_sam_hdr(
                        0 as *const mm_idx_t,
                        rg as *const libc::c_char,
                        b"2.24-r1122\0" as *const u8 as *const libc::c_char,
                        argc,
                        argv,
                    );
                    if opt.split_prefix as libc::c_ulong
                        == 0 as *const libc::c_char as libc::c_ulong
                    {
                        if mm_verbose >= 2 as libc::c_int {
                            fprintf(
                                stderr,
                                b"[WARNING]\x1B[1;31m For a multi-part index, no @SQ lines will be outputted. Please use --split-prefix.\x1B[0m\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                }
                if ret != 0 as libc::c_int {
                    mm_idx_destroy(mi);
                    mm_idx_reader_close(idx_rdr);
                    return 1 as libc::c_int;
                }
            }
        }
        if mm_verbose >= 3 as libc::c_int {
            tmp___40 = cputime();
            tmp___41 = realtime();
            tmp___42 = realtime();
            fprintf(
                stderr,
                b"[M::%s::%.3f*%.2f] loaded/built the index for %d target sequence(s)\n\0"
                    as *const u8 as *const libc::c_char,
                b"main\0" as *const u8 as *const libc::c_char,
                tmp___42 - mm_realtime0,
                tmp___40 / (tmp___41 - mm_realtime0),
                (*mi).n_seq,
            );
        }
        if argc != o.ind + 1 as libc::c_int {
            mm_mapopt_update(&mut opt, mi as *const mm_idx_t);
        }
        if mm_verbose >= 3 as libc::c_int {
            mm_idx_stat(mi as *const mm_idx_t);
        }
        if !junc_bed.is_null() {
            mm_idx_bed_read(mi, junc_bed as *const libc::c_char, 1 as libc::c_int);
        }
        if !alt_list.is_null() {
            mm_idx_alt_read(mi, alt_list as *const libc::c_char);
        }
        if argc - (o.ind + 1 as libc::c_int) == 0 as libc::c_int {
            mm_idx_destroy(mi);
        } else {
            ret = 0 as libc::c_int;
            if opt.flag & 8192 as libc::c_long == 0 {
                i = o.ind + 1 as libc::c_int;
                while i < argc {
                    ret = mm_map_file(
                        mi as *const mm_idx_t,
                        *argv.offset(i as isize) as *const libc::c_char,
                        &mut opt as *mut mm_mapopt_t as *const mm_mapopt_t,
                        n_threads,
                    );
                    if ret < 0 as libc::c_int {
                        break;
                    }
                    i += 1;
                }
            } else {
                ret = mm_map_file_frag(
                    mi as *const mm_idx_t,
                    argc - (o.ind + 1 as libc::c_int),
                    argv.offset((o.ind + 1 as libc::c_int) as isize)
                        as *mut *const libc::c_char,
                    &mut opt as *mut mm_mapopt_t as *const mm_mapopt_t,
                    n_threads,
                );
            }
            mm_idx_destroy(mi);
            if ret < 0 as libc::c_int {
                fprintf(
                    stderr,
                    b"ERROR: failed to map the query file\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    n_parts = (*idx_rdr).n_parts;
    mm_idx_reader_close(idx_rdr);
    if !(opt.split_prefix).is_null() {
        mm_split_merge(
            argc - (o.ind + 1 as libc::c_int),
            argv.offset((o.ind + 1 as libc::c_int) as isize) as *mut *const libc::c_char,
            &mut opt as *mut mm_mapopt_t as *const mm_mapopt_t,
            n_parts,
        );
    }
    tmp___43 = fflush(stdout);
    if tmp___43 == -(1 as libc::c_int) {
        perror(
            b"[ERROR] failed to write the results\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if mm_verbose >= 3 as libc::c_int {
        fprintf(
            stderr,
            b"[M::%s] Version: %s\n\0" as *const u8 as *const libc::c_char,
            b"main\0" as *const u8 as *const libc::c_char,
            b"2.24-r1122\0" as *const u8 as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"[M::%s] CMD:\0" as *const u8 as *const libc::c_char,
            b"main\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < argc {
            fprintf(
                stderr,
                b" %s\0" as *const u8 as *const libc::c_char,
                *argv.offset(i as isize),
            );
            i += 1;
        }
        tmp___44 = peakrss();
        tmp___45 = cputime();
        tmp___46 = realtime();
        fprintf(
            stderr,
            b"\n[M::%s] Real time: %.3f sec; CPU: %.3f sec; Peak RSS: %.3f GB\n\0"
                as *const u8 as *const libc::c_char,
            b"main\0" as *const u8 as *const libc::c_char,
            tmp___46 - mm_realtime0,
            tmp___45,
            tmp___44 as libc::c_double / 1024.0f64 / 1024.0f64 / 1024.0f64,
        );
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn mm_revcomp_bseq(mut s: *mut mm_bseq1_t) {
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    l = (*s).l_seq;
    i = 0 as libc::c_int;
    while i < l >> 1 as libc::c_int {
        t = *((*s).seq).offset((l - i - 1 as libc::c_int) as isize) as libc::c_int;
        *((*s).seq)
            .offset(
                (l - i - 1 as libc::c_int) as isize,
            ) = seq_comp_table[*((*s).seq).offset(i as isize) as uint8_t as usize]
            as libc::c_char;
        *((*s).seq).offset(i as isize) = seq_comp_table[t as usize] as libc::c_char;
        i += 1;
    }
    if l & 1 as libc::c_int != 0 {
        *((*s).seq)
            .offset(
                (l >> 1 as libc::c_int) as isize,
            ) = seq_comp_table[*((*s).seq).offset((l >> 1 as libc::c_int) as isize)
            as uint8_t as usize] as libc::c_char;
    }
    if !((*s).qual).is_null() {
        i = 0 as libc::c_int;
        while i < l >> 1 as libc::c_int {
            t = *((*s).qual).offset((l - i - 1 as libc::c_int) as isize) as libc::c_int;
            *((*s).qual)
                .offset(
                    (l - i - 1 as libc::c_int) as isize,
                ) = *((*s).qual).offset(i as isize);
            *((*s).qual).offset(i as isize) = t as libc::c_char;
            i += 1;
        }
    }
}
#[inline]
unsafe extern "C" fn __ac_Wang_hash(mut key: khint_t) -> khint_t {
    key = (key as libc::c_uint).wrapping_add(!(key << 15 as libc::c_int)) as khint_t
        as khint_t;
    key ^= key >> 10 as libc::c_int;
    key = (key as libc::c_uint).wrapping_add(key << 3 as libc::c_int) as khint_t
        as khint_t;
    key ^= key >> 6 as libc::c_int;
    key = (key as libc::c_uint).wrapping_add(!(key << 11 as libc::c_int)) as khint_t
        as khint_t;
    key ^= key >> 16 as libc::c_int;
    return key;
}
pub unsafe extern "C" fn mm_tbuf_init() -> *mut mm_tbuf_t {
    let mut b: *mut mm_tbuf_t = 0 as *mut mm_tbuf_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<mm_tbuf_t>() as libc::c_ulong,
    );
    b = tmp as *mut mm_tbuf_t;
    if mm_dbg_flag & 1 as libc::c_int == 0 {
        (*b).km = km_init();
    }
    return b;
}
pub unsafe extern "C" fn mm_tbuf_destroy(mut b: *mut mm_tbuf_t) {
    if b as libc::c_ulong == 0 as *mut mm_tbuf_t as libc::c_ulong {
        return;
    }
    km_destroy((*b).km);
    free(b as *mut libc::c_void);
}
pub unsafe extern "C" fn mm_tbuf_get_km(mut b: *mut mm_tbuf_t) -> *mut libc::c_void {
    return (*b).km;
}
unsafe extern "C" fn mm_dust_minier(
    mut km: *mut libc::c_void,
    mut n: libc::c_int,
    mut a: *mut mm128_t,
    mut l_seq: libc::c_int,
    mut seq: *const libc::c_char,
    mut sdust_thres: libc::c_int,
) -> libc::c_int {
    let mut n_dreg: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut dreg: *const uint64_t = 0 as *const uint64_t;
    let mut sdb: *mut sdust_buf_t = 0 as *mut sdust_buf_t;
    let mut qpos: int32_t = 0;
    let mut span: int32_t = 0;
    let mut s: int32_t = 0;
    let mut e: int32_t = 0;
    let mut v: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut ss: libc::c_int = 0;
    let mut tmp: uint64_t = 0;
    let mut ee: libc::c_int = 0;
    let mut tmp___0: uint32_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    u = 0 as libc::c_int;
    if sdust_thres <= 0 as libc::c_int {
        return n;
    }
    sdb = sdust_buf_init(km);
    dreg = sdust_core(
        seq as *const uint8_t,
        l_seq,
        sdust_thres,
        64 as libc::c_int,
        &mut n_dreg,
        sdb,
    );
    k = 0 as libc::c_int;
    j = k;
    while j < n {
        qpos = ((*a.offset(j as isize)).y as uint32_t >> 1 as libc::c_int) as int32_t;
        span = ((*a.offset(j as isize)).x & 255 as libc::c_ulong) as int32_t;
        s = qpos - (span - 1 as libc::c_int);
        e = s + span;
        while u < n_dreg {
            if !(*dreg.offset(u as isize) as int32_t <= s) {
                break;
            }
            u += 1;
        }
        if u < n_dreg {
            if ((*dreg.offset(u as isize) >> 32 as libc::c_int) as int32_t) < e {
                l = 0 as libc::c_int;
                v = u;
                while v < n_dreg {
                    if !(((*dreg.offset(v as isize) >> 32 as libc::c_int) as int32_t)
                        < e)
                    {
                        break;
                    }
                    if s > (*dreg.offset(v as isize) >> 32 as libc::c_int) as int32_t {
                        tmp = s as uint64_t;
                    } else {
                        tmp = *dreg.offset(v as isize) >> 32 as libc::c_int;
                    }
                    ss = tmp as libc::c_int;
                    if e < *dreg.offset(v as isize) as int32_t {
                        tmp___0 = e as uint32_t;
                    } else {
                        tmp___0 = *dreg.offset(v as isize) as uint32_t;
                    }
                    ee = tmp___0 as libc::c_int;
                    l += ee - ss;
                    v += 1;
                }
                if l <= span >> 1 as libc::c_int {
                    tmp___1 = k;
                    k += 1;
                    *a.offset(tmp___1 as isize) = *a.offset(j as isize);
                }
            } else {
                tmp___2 = k;
                k += 1;
                *a.offset(tmp___2 as isize) = *a.offset(j as isize);
            }
        } else {
            tmp___2 = k;
            k += 1;
            *a.offset(tmp___2 as isize) = *a.offset(j as isize);
        }
        j += 1;
    }
    sdust_buf_destroy(sdb);
    return k;
}
unsafe extern "C" fn collect_minimizers(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut n_segs: libc::c_int,
    mut qlens: *const libc::c_int,
    mut seqs: *mut *const libc::c_char,
    mut mv: *mut mm128_v,
) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    let mut j: size_t = 0;
    let mut tmp: libc::c_int = 0;
    sum = 0 as libc::c_int;
    (*mv).n = 0 as libc::c_int as size_t;
    n = 0 as libc::c_int;
    i = n;
    while i < n_segs {
        mm_sketch(
            km,
            *seqs.offset(i as isize),
            *qlens.offset(i as isize),
            (*mi).w,
            (*mi).k,
            i as uint32_t,
            (*mi).flag & 1 as libc::c_int,
            mv,
        );
        j = n as size_t;
        while j < (*mv).n {
            let ref mut fresh33 = (*((*mv).a).offset(j as isize)).y;
            *fresh33 = (*fresh33 as libc::c_ulong)
                .wrapping_add((sum << 1 as libc::c_int) as uint64_t) as uint64_t
                as uint64_t;
            j = j.wrapping_add(1);
        }
        if (*opt).sdust_thres > 0 as libc::c_int {
            tmp = mm_dust_minier(
                km,
                ((*mv).n).wrapping_sub(n as size_t) as libc::c_int,
                ((*mv).a).offset(n as isize),
                *qlens.offset(i as isize),
                *seqs.offset(i as isize),
                (*opt).sdust_thres,
            );
            (*mv).n = (n + tmp) as size_t;
        }
        sum += *qlens.offset(i as isize);
        n = (*mv).n as libc::c_int;
        i += 1;
    }
}
pub unsafe extern "C" fn ks_heapdown_heap(
    mut i: size_t,
    mut n: size_t,
    mut l: *mut mm128_t,
) {
    let mut k: size_t = 0;
    let mut tmp: mm128_t = mm128_t { x: 0, y: 0 };
    k = i;
    tmp = *l.offset(i as isize);
    loop {
        k = (k << 1 as libc::c_int).wrapping_add(1 as libc::c_ulong);
        if !(k < n) {
            break;
        }
        if k != n.wrapping_sub(1 as libc::c_ulong) {
            if (*l.offset(k as isize)).x
                > (*l.offset(k.wrapping_add(1 as libc::c_ulong) as isize)).x
            {
                k = k.wrapping_add(1);
            }
        }
        if (*l.offset(k as isize)).x > tmp.x {
            break;
        }
        *l.offset(i as isize) = *l.offset(k as isize);
        i = k;
    }
    *l.offset(i as isize) = tmp;
}
pub unsafe extern "C" fn ks_heapmake_heap(mut lsize: size_t, mut l: *mut mm128_t) {
    let mut i: size_t = 0;
    i = (lsize >> 1 as libc::c_int).wrapping_sub(1 as libc::c_ulong);
    while i != 0xffffffffffffffff as libc::c_ulong {
        ks_heapdown_heap(i, lsize, l);
        i = i.wrapping_sub(1);
    }
}
pub unsafe extern "C" fn ks_ksmall_heap(
    mut n: size_t,
    mut arr: *mut mm128_t,
    mut kk: size_t,
) -> mm128_t {
    let mut low: *mut mm128_t = 0 as *mut mm128_t;
    let mut high: *mut mm128_t = 0 as *mut mm128_t;
    let mut k: *mut mm128_t = 0 as *mut mm128_t;
    let mut ll: *mut mm128_t = 0 as *mut mm128_t;
    let mut hh: *mut mm128_t = 0 as *mut mm128_t;
    let mut mid: *mut mm128_t = 0 as *mut mm128_t;
    let mut t: mm128_t = mm128_t { x: 0, y: 0 };
    let mut t___0: mm128_t = mm128_t { x: 0, y: 0 };
    let mut t___1: mm128_t = mm128_t { x: 0, y: 0 };
    let mut t___2: mm128_t = mm128_t { x: 0, y: 0 };
    let mut t___3: mm128_t = mm128_t { x: 0, y: 0 };
    let mut t___4: mm128_t = mm128_t { x: 0, y: 0 };
    let mut t___5: mm128_t = mm128_t { x: 0, y: 0 };
    low = arr;
    high = arr.offset(n as isize).offset(-(1 as libc::c_int as isize));
    k = arr.offset(kk as isize);
    loop {
        if high as libc::c_ulong <= low as libc::c_ulong {
            return *k;
        }
        if high as libc::c_ulong
            == low.offset(1 as libc::c_int as isize) as libc::c_ulong
        {
            if (*high).x > (*low).x {
                t = *low;
                *low = *high;
                *high = t;
            }
            return *k;
        }
        mid = low
            .offset(
                (high.offset_from(low) as libc::c_long / 2 as libc::c_long) as isize,
            );
        if (*high).x > (*mid).x {
            t___0 = *mid;
            *mid = *high;
            *high = t___0;
        }
        if (*high).x > (*low).x {
            t___1 = *low;
            *low = *high;
            *high = t___1;
        }
        if (*low).x > (*mid).x {
            t___2 = *mid;
            *mid = *low;
            *low = t___2;
        }
        t___3 = *mid;
        *mid = *low.offset(1 as libc::c_int as isize);
        *low.offset(1 as libc::c_int as isize) = t___3;
        ll = low.offset(1 as libc::c_int as isize);
        hh = high;
        loop {
            loop {
                ll = ll.offset(1);
                if !((*ll).x > (*low).x) {
                    break;
                }
            }
            loop {
                hh = hh.offset(-1);
                if !((*low).x > (*hh).x) {
                    break;
                }
            }
            if (hh as libc::c_ulong) < ll as libc::c_ulong {
                break;
            }
            t___4 = *ll;
            *ll = *hh;
            *hh = t___4;
        }
        t___5 = *low;
        *low = *hh;
        *hh = t___5;
        if hh as libc::c_ulong <= k as libc::c_ulong {
            low = ll;
        }
        if hh as libc::c_ulong >= k as libc::c_ulong {
            high = hh.offset(-(1 as libc::c_int as isize));
        }
    };
}
#[inline]
unsafe extern "C" fn skip_seed(
    mut flag: libc::c_int,
    mut r: uint64_t,
    mut q: *const mm_seed_t,
    mut qname: *const libc::c_char,
    mut qlen: libc::c_int,
    mut mi: *const mm_idx_t,
    mut is_self: *mut libc::c_int,
) -> libc::c_int {
    let mut s: *const mm_idx_seq_t = 0 as *const mm_idx_seq_t;
    let mut cmp: libc::c_int = 0;
    *is_self = 0 as libc::c_int;
    if !qname.is_null() {
        if flag & 3 as libc::c_int != 0 {
            s = ((*mi).seq).offset((r >> 32 as libc::c_int) as isize)
                as *const mm_idx_seq_t;
            cmp = strcmp(qname, (*s).name as *const libc::c_char);
            if flag & 1 as libc::c_int != 0 {
                if cmp == 0 as libc::c_int {
                    if (*s).len as libc::c_int == qlen {
                        if r as uint32_t >> 1 as libc::c_int
                            == (*q).q_pos >> 1 as libc::c_int
                        {
                            return 1 as libc::c_int;
                        }
                        if r & 1 as libc::c_ulong
                            == ((*q).q_pos & 1 as libc::c_uint) as libc::c_ulong
                        {
                            *is_self = 1 as libc::c_int;
                        }
                    }
                }
            }
            if flag & 2 as libc::c_int != 0 {
                if cmp > 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
            }
        }
    }
    if flag & 3145728 as libc::c_int != 0 {
        if r & 1 as libc::c_ulong == ((*q).q_pos & 1 as libc::c_uint) as libc::c_ulong {
            if flag & 2097152 as libc::c_int != 0 {
                return 1 as libc::c_int;
            }
        } else if flag & 1048576 as libc::c_int != 0 {
            return 1 as libc::c_int
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn collect_seed_hits_heap(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut max_occ: libc::c_int,
    mut mi: *const mm_idx_t,
    mut qname: *const libc::c_char,
    mut mv: *const mm128_v,
    mut qlen: libc::c_int,
    mut n_a: *mut int64_t,
    mut rep_len: *mut libc::c_int,
    mut n_mini_pos: *mut libc::c_int,
    mut mini_pos: *mut *mut uint64_t,
) -> *mut mm128_t {
    let mut i: libc::c_int = 0;
    let mut n_m: libc::c_int = 0;
    let mut heap_size: libc::c_int = 0;
    let mut j: int64_t = 0;
    let mut n_for: int64_t = 0;
    let mut n_rev: int64_t = 0;
    let mut m: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut a: *mut mm128_t = 0 as *mut mm128_t;
    let mut heap: *mut mm128_t = 0 as *mut mm128_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut q: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut p: *mut mm128_t = 0 as *mut mm128_t;
    let mut r: uint64_t = 0;
    let mut is_self: int32_t = 0;
    let mut rpos: int32_t = 0;
    let mut tmp___1: int64_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut t: mm128_t = mm128_t { x: 0, y: 0 };
    heap_size = 0 as libc::c_int;
    n_for = 0 as libc::c_int as int64_t;
    n_rev = 0 as libc::c_int as int64_t;
    m = mm_collect_matches(
        km,
        &mut n_m,
        qlen,
        max_occ,
        (*opt).max_max_occ,
        (*opt).occ_dist,
        mi,
        mv,
        n_a,
        rep_len,
        n_mini_pos,
        mini_pos,
    );
    tmp = kmalloc(
        km,
        (n_m as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    );
    heap = tmp as *mut mm128_t;
    tmp___0 = kmalloc(
        km,
        (*n_a as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    );
    a = tmp___0 as *mut mm128_t;
    i = 0 as libc::c_int;
    heap_size = 0 as libc::c_int;
    while i < n_m {
        if (*m.offset(i as isize)).n > 0 as libc::c_uint {
            (*heap.offset(heap_size as isize))
                .x = *((*m.offset(i as isize)).cr).offset(0 as libc::c_int as isize);
            (*heap.offset(heap_size as isize)).y = (i as uint64_t) << 32 as libc::c_int;
            heap_size += 1;
        }
        i += 1;
    }
    ks_heapmake_heap(heap_size as size_t, heap);
    while heap_size > 0 as libc::c_int {
        q = m.offset(((*heap).y >> 32 as libc::c_int) as isize);
        r = (*heap).x;
        rpos = (r as uint32_t >> 1 as libc::c_int) as int32_t;
        tmp___2 = skip_seed(
            (*opt).flag as libc::c_int,
            r,
            q as *const mm_seed_t,
            qname,
            qlen,
            mi,
            &mut is_self,
        );
        if tmp___2 == 0 {
            if r & 1 as libc::c_ulong
                == ((*q).q_pos & 1 as libc::c_uint) as libc::c_ulong
            {
                tmp___1 = n_for;
                n_for += 1;
                p = a.offset(tmp___1 as isize);
                (*p)
                    .x = (r as libc::c_ulonglong
                    & 18446744069414584320 as libc::c_ulonglong
                    | rpos as libc::c_ulonglong) as uint64_t;
                (*p)
                    .y = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                    | ((*q).q_pos >> 1 as libc::c_int) as libc::c_ulong;
            } else {
                n_rev += 1;
                p = a.offset((*n_a - n_rev) as isize);
                (*p)
                    .x = ((1 as libc::c_ulonglong) << 63 as libc::c_int
                    | r as libc::c_ulonglong & 18446744069414584320 as libc::c_ulonglong
                    | rpos as libc::c_ulonglong) as uint64_t;
                (*p)
                    .y = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                    | (qlen as uint32_t)
                        .wrapping_sub(
                            ((*q).q_pos >> 1 as libc::c_int)
                                .wrapping_add(1 as libc::c_uint)
                                .wrapping_sub((*q).q_span()),
                        )
                        .wrapping_sub(1 as libc::c_uint) as libc::c_ulong;
            }
            (*p).y |= ((*q).seg_id() as uint64_t) << 48 as libc::c_int;
            if (*q).is_tandem() != 0 {
                (*p)
                    .y = ((*p).y as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 42 as libc::c_int) as uint64_t;
            }
            if is_self != 0 {
                (*p)
                    .y = ((*p).y as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 43 as libc::c_int) as uint64_t;
            }
        }
        if ((*heap).y as uint32_t) < ((*q).n).wrapping_sub(1 as libc::c_uint) {
            let ref mut fresh34 = (*heap.offset(0 as libc::c_int as isize)).y;
            *fresh34 = (*fresh34).wrapping_add(1);
            (*heap.offset(0 as libc::c_int as isize))
                .x = *((*m
                .offset(
                    ((*heap.offset(0 as libc::c_int as isize)).y >> 32 as libc::c_int)
                        as isize,
                ))
                .cr)
                .offset(
                    (*heap.offset(0 as libc::c_int as isize)).y as uint32_t as isize,
                );
        } else {
            *heap
                .offset(
                    0 as libc::c_int as isize,
                ) = *heap.offset((heap_size - 1 as libc::c_int) as isize);
            heap_size -= 1;
        }
        ks_heapdown_heap(0 as libc::c_int as size_t, heap_size as size_t, heap);
    }
    kfree(km, m as *mut libc::c_void);
    kfree(km, heap as *mut libc::c_void);
    j = 0 as libc::c_int as int64_t;
    while j < n_rev >> 1 as libc::c_int {
        t = *a.offset((*n_a - 1 as libc::c_long - j) as isize);
        *a
            .offset(
                (*n_a - 1 as libc::c_long - j) as isize,
            ) = *a.offset((*n_a - (n_rev - j)) as isize);
        *a.offset((*n_a - (n_rev - j)) as isize) = t;
        j += 1;
    }
    if *n_a > n_for + n_rev {
        memmove(
            a.offset(n_for as isize) as *mut libc::c_void,
            a.offset(*n_a as isize).offset(-(n_rev as isize)) as *const libc::c_void,
            (n_rev as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
        );
        *n_a = n_for + n_rev;
    }
    return a;
}
unsafe extern "C" fn collect_seed_hits(
    mut km: *mut libc::c_void,
    mut opt: *const mm_mapopt_t,
    mut max_occ: libc::c_int,
    mut mi: *const mm_idx_t,
    mut qname: *const libc::c_char,
    mut mv: *const mm128_v,
    mut qlen: libc::c_int,
    mut n_a: *mut int64_t,
    mut rep_len: *mut libc::c_int,
    mut n_mini_pos: *mut libc::c_int,
    mut mini_pos: *mut *mut uint64_t,
) -> *mut mm128_t {
    let mut i: libc::c_int = 0;
    let mut n_m: libc::c_int = 0;
    let mut m: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut a: *mut mm128_t = 0 as *mut mm128_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut q: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut r: *const uint64_t = 0 as *const uint64_t;
    let mut k: uint32_t = 0;
    let mut is_self: int32_t = 0;
    let mut rpos: int32_t = 0;
    let mut p: *mut mm128_t = 0 as *mut mm128_t;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: int64_t = 0;
    let mut len: int32_t = 0;
    m = mm_collect_matches(
        km,
        &mut n_m,
        qlen,
        max_occ,
        (*opt).max_max_occ,
        (*opt).occ_dist,
        mi,
        mv,
        n_a,
        rep_len,
        n_mini_pos,
        mini_pos,
    );
    tmp = kmalloc(
        km,
        (*n_a as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    );
    a = tmp as *mut mm128_t;
    i = 0 as libc::c_int;
    *n_a = 0 as libc::c_int as int64_t;
    while i < n_m {
        q = m.offset(i as isize);
        r = (*q).cr;
        k = 0 as libc::c_int as uint32_t;
        while k < (*q).n {
            rpos = (*r.offset(k as isize) as uint32_t >> 1 as libc::c_int) as int32_t;
            tmp___0 = skip_seed(
                (*opt).flag as libc::c_int,
                *r.offset(k as isize),
                q as *const mm_seed_t,
                qname,
                qlen,
                mi,
                &mut is_self,
            );
            if !(tmp___0 != 0) {
                tmp___1 = *n_a;
                *n_a += 1;
                p = a.offset(tmp___1 as isize);
                if *r.offset(k as isize) & 1 as libc::c_ulong
                    == ((*q).q_pos & 1 as libc::c_uint) as libc::c_ulong
                {
                    (*p)
                        .x = (*r.offset(k as isize) as libc::c_ulonglong
                        & 18446744069414584320 as libc::c_ulonglong
                        | rpos as libc::c_ulonglong) as uint64_t;
                    (*p)
                        .y = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                        | ((*q).q_pos >> 1 as libc::c_int) as libc::c_ulong;
                } else if (*opt).flag as libc::c_longlong
                        & 4294967296 as libc::c_longlong == 0
                    {
                    (*p)
                        .x = ((1 as libc::c_ulonglong) << 63 as libc::c_int
                        | *r.offset(k as isize) as libc::c_ulonglong
                            & 18446744069414584320 as libc::c_ulonglong
                        | rpos as libc::c_ulonglong) as uint64_t;
                    (*p)
                        .y = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                        | (qlen as uint32_t)
                            .wrapping_sub(
                                ((*q).q_pos >> 1 as libc::c_int)
                                    .wrapping_add(1 as libc::c_uint)
                                    .wrapping_sub((*q).q_span()),
                            )
                            .wrapping_sub(1 as libc::c_uint) as libc::c_ulong;
                } else {
                    len = (*((*mi).seq)
                        .offset((*r.offset(k as isize) >> 32 as libc::c_int) as isize))
                        .len as int32_t;
                    (*p)
                        .x = ((1 as libc::c_ulonglong) << 63 as libc::c_int
                        | *r.offset(k as isize) as libc::c_ulonglong
                            & 18446744069414584320 as libc::c_ulonglong
                        | (len as uint32_t)
                            .wrapping_sub(
                                ((rpos + 1 as libc::c_int) as uint32_t)
                                    .wrapping_sub((*q).q_span()),
                            )
                            .wrapping_sub(1 as libc::c_uint) as libc::c_ulonglong)
                        as uint64_t;
                    (*p)
                        .y = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                        | ((*q).q_pos >> 1 as libc::c_int) as libc::c_ulong;
                }
                (*p).y |= ((*q).seg_id() as uint64_t) << 48 as libc::c_int;
                if (*q).is_tandem() != 0 {
                    (*p)
                        .y = ((*p).y as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << 42 as libc::c_int) as uint64_t;
                }
                if is_self != 0 {
                    (*p)
                        .y = ((*p).y as libc::c_ulonglong
                        | (1 as libc::c_ulonglong) << 43 as libc::c_int) as uint64_t;
                }
            }
            k = k.wrapping_add(1);
        }
        i += 1;
    }
    kfree(km, m as *mut libc::c_void);
    radix_sort_128x(a, a.offset(*n_a as isize));
    return a;
}
unsafe extern "C" fn chain_post(
    mut opt: *const mm_mapopt_t,
    mut max_chain_gap_ref: libc::c_int,
    mut mi: *const mm_idx_t,
    mut km: *mut libc::c_void,
    mut qlen: libc::c_int,
    mut n_segs: libc::c_int,
    mut qlens: *const libc::c_int,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut a: *mut mm128_t,
) {
    if (*opt).flag & 8388608 as libc::c_long == 0 {
        mm_set_parent(
            km,
            (*opt).mask_level,
            (*opt).mask_len,
            *n_regs,
            regs,
            (*opt).a * 2 as libc::c_int + (*opt).b,
            ((*opt).flag & 536870912 as libc::c_long) as libc::c_int,
            (*opt).alt_drop,
        );
        if n_segs <= 1 as libc::c_int {
            mm_select_sub(
                km,
                (*opt).pri_ratio,
                (*mi).k * 2 as libc::c_int,
                (*opt).best_n,
                1 as libc::c_int,
                ((*opt).max_gap as libc::c_double * 0.8f64) as libc::c_int,
                n_regs,
                regs,
            );
        } else {
            mm_select_sub_multi(
                km,
                (*opt).pri_ratio,
                0.2f32,
                0.7f32,
                max_chain_gap_ref,
                (*mi).k * 2 as libc::c_int,
                (*opt).best_n,
                n_segs,
                qlens,
                n_regs,
                regs,
            );
        }
    }
}
unsafe extern "C" fn align_regs(
    mut opt: *const mm_mapopt_t,
    mut mi: *const mm_idx_t,
    mut km: *mut libc::c_void,
    mut qlen: libc::c_int,
    mut seq: *const libc::c_char,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut mm_reg1_t,
    mut a: *mut mm128_t,
) -> *mut mm_reg1_t {
    if (*opt).flag & 4 as libc::c_long == 0 {
        return regs;
    }
    regs = mm_align_skeleton(km, opt, mi, qlen, seq, n_regs, regs, a);
    if (*opt).flag & 8388608 as libc::c_long == 0 {
        mm_set_parent(
            km,
            (*opt).mask_level,
            (*opt).mask_len,
            *n_regs,
            regs,
            (*opt).a * 2 as libc::c_int + (*opt).b,
            ((*opt).flag & 536870912 as libc::c_long) as libc::c_int,
            (*opt).alt_drop,
        );
        mm_select_sub(
            km,
            (*opt).pri_ratio,
            (*mi).k * 2 as libc::c_int,
            (*opt).best_n,
            0 as libc::c_int,
            ((*opt).max_gap as libc::c_double * 0.8f64) as libc::c_int,
            n_regs,
            regs,
        );
        mm_set_sam_pri(*n_regs, regs);
    }
    return regs;
}
pub unsafe extern "C" fn mm_map_frag(
    mut mi: *const mm_idx_t,
    mut n_segs: libc::c_int,
    mut qlens: *const libc::c_int,
    mut seqs: *mut *const libc::c_char,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut *mut mm_reg1_t,
    mut b: *mut mm_tbuf_t,
    mut opt: *const mm_mapopt_t,
    mut qname: *const libc::c_char,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut rep_len: libc::c_int = 0;
    let mut qlen_sum: libc::c_int = 0;
    let mut n_regs0: libc::c_int = 0;
    let mut n_mini_pos: libc::c_int = 0;
    let mut max_chain_gap_qry: libc::c_int = 0;
    let mut max_chain_gap_ref: libc::c_int = 0;
    let mut is_splice: libc::c_int = 0;
    let mut is_sr: libc::c_int = 0;
    let mut hash: uint32_t = 0;
    let mut n_a: int64_t = 0;
    let mut u: *mut uint64_t = 0 as *mut uint64_t;
    let mut mini_pos: *mut uint64_t = 0 as *mut uint64_t;
    let mut a: *mut mm128_t = 0 as *mut mm128_t;
    let mut mv: mm128_v = mm128_v {
        n: 0,
        m: 0,
        a: 0 as *mut mm128_t,
    };
    let mut regs0: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut kmst: km_stat_t = km_stat_t {
        capacity: 0,
        available: 0,
        n_blocks: 0,
        n_cores: 0,
        largest: 0,
    };
    let mut chn_pen_gap: libc::c_float = 0.;
    let mut chn_pen_skip: libc::c_float = 0.;
    let mut tmp: khint_t = 0;
    let mut tmp___0: khint_t = 0;
    let mut tmp___1: khint_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut st: int32_t = 0;
    let mut en: int32_t = 0;
    let mut i___0: int32_t = 0;
    let mut rechain: libc::c_int = 0;
    let mut n_chained_segs: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut max_i: libc::c_int = 0;
    let mut max_off: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut seg: *mut mm_seg_t = 0 as *mut mm_seg_t;
    is_splice = ((*opt).flag & 128 as libc::c_long != 0) as libc::c_int;
    is_sr = ((*opt).flag & 4096 as libc::c_long != 0) as libc::c_int;
    mv.n = 0 as libc::c_int as size_t;
    mv.m = 0 as libc::c_int as size_t;
    mv.a = 0 as *mut mm128_t;
    i = 0 as libc::c_int;
    qlen_sum = 0 as libc::c_int;
    while i < n_segs {
        qlen_sum += *qlens.offset(i as isize);
        *n_regs.offset(i as isize) = 0 as libc::c_int;
        let ref mut fresh35 = *regs.offset(i as isize);
        *fresh35 = 0 as *mut mm_reg1_t;
        i += 1;
    }
    if qlen_sum == 0 as libc::c_int {
        return
    } else {
        if n_segs <= 0 as libc::c_int {
            return
        } else {
            if n_segs > 255 as libc::c_int {
                return;
            }
        }
    }
    if (*opt).max_qlen > 0 as libc::c_int {
        if qlen_sum > (*opt).max_qlen {
            return;
        }
    }
    if !qname.is_null() {
        if (*opt).flag as libc::c_longlong & 17179869184 as libc::c_longlong == 0 {
            tmp = __ac_X31_hash_string(qname);
            hash = tmp;
        } else {
            hash = 0 as libc::c_int as uint32_t;
        }
    } else {
        hash = 0 as libc::c_int as uint32_t;
    }
    tmp___0 = __ac_Wang_hash(qlen_sum as khint_t);
    tmp___1 = __ac_Wang_hash((*opt).seed as khint_t);
    hash ^= tmp___0.wrapping_add(tmp___1);
    hash = __ac_Wang_hash(hash);
    collect_minimizers((*b).km, opt, mi, n_segs, qlens, seqs, &mut mv);
    if (*opt).q_occ_frac > 0.0f32 {
        mm_seed_mz_flt((*b).km, &mut mv, (*opt).mid_occ, (*opt).q_occ_frac);
    }
    if (*opt).flag & 4194304 as libc::c_long != 0 {
        a = collect_seed_hits_heap(
            (*b).km,
            opt,
            (*opt).mid_occ,
            mi,
            qname,
            &mut mv as *mut mm128_v as *const mm128_v,
            qlen_sum,
            &mut n_a,
            &mut rep_len,
            &mut n_mini_pos,
            &mut mini_pos,
        );
    } else {
        a = collect_seed_hits(
            (*b).km,
            opt,
            (*opt).mid_occ,
            mi,
            qname,
            &mut mv as *mut mm128_v as *const mm128_v,
            qlen_sum,
            &mut n_a,
            &mut rep_len,
            &mut n_mini_pos,
            &mut mini_pos,
        );
    }
    if mm_dbg_flag & 4 as libc::c_int != 0 {
        fprintf(stderr, b"RS\t%d\n\0" as *const u8 as *const libc::c_char, rep_len);
        i = 0 as libc::c_int;
        while (i as int64_t) < n_a {
            if i == 0 as libc::c_int {
                tmp___2 = 0 as libc::c_int;
            } else {
                tmp___2 = (*a.offset(i as isize)).y as int32_t
                    - (*a.offset((i - 1 as libc::c_int) as isize)).y as int32_t
                    - ((*a.offset(i as isize)).x as int32_t
                        - (*a.offset((i - 1 as libc::c_int) as isize)).x as int32_t);
            }
            fprintf(
                stderr,
                b"SD\t%s\t%d\t%c\t%d\t%d\t%d\n\0" as *const u8 as *const libc::c_char,
                (*((*mi).seq)
                    .offset(
                        ((*a.offset(i as isize)).x << 1 as libc::c_int
                            >> 33 as libc::c_int) as isize,
                    ))
                    .name,
                (*a.offset(i as isize)).x as int32_t,
                *(b"+-\0" as *const u8 as *const libc::c_char)
                    .offset(((*a.offset(i as isize)).x >> 63 as libc::c_int) as isize)
                    as libc::c_int,
                (*a.offset(i as isize)).y as int32_t,
                ((*a.offset(i as isize)).y >> 32 as libc::c_int & 255 as libc::c_ulong)
                    as int32_t,
                tmp___2,
            );
            i += 1;
        }
    }
    if is_sr != 0 {
        if qlen_sum > (*opt).max_gap {
            max_chain_gap_qry = qlen_sum;
        } else {
            max_chain_gap_qry = (*opt).max_gap;
        }
    } else {
        max_chain_gap_qry = (*opt).max_gap;
    }
    if (*opt).max_gap_ref > 0 as libc::c_int {
        max_chain_gap_ref = (*opt).max_gap_ref;
    } else if (*opt).max_frag_len > 0 as libc::c_int {
        max_chain_gap_ref = (*opt).max_frag_len - qlen_sum;
        if max_chain_gap_ref < (*opt).max_gap {
            max_chain_gap_ref = (*opt).max_gap;
        }
    } else {
        max_chain_gap_ref = (*opt).max_gap;
    }
    chn_pen_gap = ((*opt).chain_gap_scale as libc::c_double * 0.01f64
        * (*mi).k as libc::c_double) as libc::c_float;
    chn_pen_skip = ((*opt).chain_skip_scale as libc::c_double * 0.01f64
        * (*mi).k as libc::c_double) as libc::c_float;
    if (*opt).flag as libc::c_longlong & 2147483648 as libc::c_longlong != 0 {
        a = mg_lchain_rmq(
            (*opt).max_gap,
            (*opt).rmq_inner_dist,
            (*opt).bw,
            (*opt).max_chain_skip,
            (*opt).rmq_size_cap,
            (*opt).min_cnt,
            (*opt).min_chain_score,
            chn_pen_gap,
            chn_pen_skip,
            n_a,
            a,
            &mut n_regs0,
            &mut u,
            (*b).km,
        );
    } else {
        a = mg_lchain_dp(
            max_chain_gap_ref,
            max_chain_gap_qry,
            (*opt).bw,
            (*opt).max_chain_skip,
            (*opt).max_chain_iter,
            (*opt).min_cnt,
            (*opt).min_chain_score,
            chn_pen_gap,
            chn_pen_skip,
            is_splice,
            n_segs,
            n_a,
            a,
            &mut n_regs0,
            &mut u,
            (*b).km,
        );
    }
    if (*opt).bw_long > (*opt).bw {
        if (*opt).flag & 5248 as libc::c_long == 0 as libc::c_long {
            if n_segs == 1 as libc::c_int {
                if n_regs0 > 1 as libc::c_int {
                    st = (*a.offset(0 as libc::c_int as isize)).y as int32_t;
                    en = (*a
                        .offset(
                            (*u.offset(0 as libc::c_int as isize) as int32_t
                                - 1 as libc::c_int) as isize,
                        ))
                        .y as int32_t;
                    let mut current_block_100: u64;
                    if qlen_sum - (en - st) > (*opt).rmq_rescue_size {
                        current_block_100 = 16756062771064377642;
                    } else if (en - st) as libc::c_float
                            > qlen_sum as libc::c_float * (*opt).rmq_rescue_ratio
                        {
                        current_block_100 = 16756062771064377642;
                    } else {
                        current_block_100 = 7416055328783156979;
                    }
                    match current_block_100 {
                        16756062771064377642 => {
                            i___0 = 0 as libc::c_int;
                            n_a = 0 as libc::c_int as int64_t;
                            while i___0 < n_regs0 {
                                n_a += *u.offset(i___0 as isize) as int32_t as int64_t;
                                i___0 += 1;
                            }
                            kfree((*b).km, u as *mut libc::c_void);
                            radix_sort_128x(a, a.offset(n_a as isize));
                            a = mg_lchain_rmq(
                                (*opt).max_gap,
                                (*opt).rmq_inner_dist,
                                (*opt).bw_long,
                                (*opt).max_chain_skip,
                                (*opt).rmq_size_cap,
                                (*opt).min_cnt,
                                (*opt).min_chain_score,
                                chn_pen_gap,
                                chn_pen_skip,
                                n_a,
                                a,
                                &mut n_regs0,
                                &mut u,
                                (*b).km,
                            );
                        }
                        _ => {}
                    }
                    current_block = 9587810615301548814;
                } else {
                    current_block = 5492826956404566150;
                }
            } else {
                current_block = 5492826956404566150;
            }
        } else {
            current_block = 5492826956404566150;
        }
    } else {
        current_block = 5492826956404566150;
    }
    match current_block {
        5492826956404566150 => {
            if (*opt).max_occ > (*opt).mid_occ {
                if rep_len > 0 as libc::c_int {
                    if (*opt).flag as libc::c_longlong & 2147483648 as libc::c_longlong
                        == 0
                    {
                        rechain = 0 as libc::c_int;
                        if n_regs0 > 0 as libc::c_int {
                            n_chained_segs = 1 as libc::c_int;
                            max = 0 as libc::c_int;
                            max_i = -(1 as libc::c_int);
                            max_off = -(1 as libc::c_int);
                            off = 0 as libc::c_int;
                            i = 0 as libc::c_int;
                            while i < n_regs0 {
                                if max
                                    < (*u.offset(i as isize) >> 32 as libc::c_int)
                                        as libc::c_int
                                {
                                    max = (*u.offset(i as isize) >> 32 as libc::c_int)
                                        as libc::c_int;
                                    max_i = i;
                                    max_off = off;
                                }
                                off = (off as uint32_t)
                                    .wrapping_add(*u.offset(i as isize) as uint32_t)
                                    as libc::c_int;
                                i += 1;
                            }
                            i = 1 as libc::c_int;
                            while i < *u.offset(max_i as isize) as int32_t {
                                if (*a.offset((max_off + i) as isize)).y
                                    as libc::c_ulonglong
                                    & (255 as libc::c_ulonglong) << 48 as libc::c_int
                                    != (*a.offset((max_off + i - 1 as libc::c_int) as isize)).y
                                        as libc::c_ulonglong
                                        & (255 as libc::c_ulonglong) << 48 as libc::c_int
                                {
                                    n_chained_segs += 1;
                                }
                                i += 1;
                            }
                            if n_chained_segs < n_segs {
                                rechain = 1 as libc::c_int;
                            }
                        } else {
                            rechain = 1 as libc::c_int;
                        }
                        if rechain != 0 {
                            kfree((*b).km, a as *mut libc::c_void);
                            kfree((*b).km, u as *mut libc::c_void);
                            kfree((*b).km, mini_pos as *mut libc::c_void);
                            if (*opt).flag & 4194304 as libc::c_long != 0 {
                                a = collect_seed_hits_heap(
                                    (*b).km,
                                    opt,
                                    (*opt).max_occ,
                                    mi,
                                    qname,
                                    &mut mv as *mut mm128_v as *const mm128_v,
                                    qlen_sum,
                                    &mut n_a,
                                    &mut rep_len,
                                    &mut n_mini_pos,
                                    &mut mini_pos,
                                );
                            } else {
                                a = collect_seed_hits(
                                    (*b).km,
                                    opt,
                                    (*opt).max_occ,
                                    mi,
                                    qname,
                                    &mut mv as *mut mm128_v as *const mm128_v,
                                    qlen_sum,
                                    &mut n_a,
                                    &mut rep_len,
                                    &mut n_mini_pos,
                                    &mut mini_pos,
                                );
                            }
                            a = mg_lchain_dp(
                                max_chain_gap_ref,
                                max_chain_gap_qry,
                                (*opt).bw,
                                (*opt).max_chain_skip,
                                (*opt).max_chain_iter,
                                (*opt).min_cnt,
                                (*opt).min_chain_score,
                                chn_pen_gap,
                                chn_pen_skip,
                                is_splice,
                                n_segs,
                                n_a,
                                a,
                                &mut n_regs0,
                                &mut u,
                                (*b).km,
                            );
                        }
                    }
                }
            }
        }
        _ => {}
    }
    (*b).frag_gap = max_chain_gap_ref;
    (*b).rep_len = rep_len;
    regs0 = mm_gen_regs(
        (*b).km,
        hash,
        qlen_sum,
        n_regs0,
        u,
        a,
        ((*opt).flag as libc::c_longlong & 4294967296 as libc::c_longlong != 0)
            as libc::c_int,
    );
    if (*mi).n_alt != 0 {
        mm_mark_alt(mi, n_regs0, regs0);
        mm_hit_sort((*b).km, &mut n_regs0, regs0, (*opt).alt_drop);
    }
    if mm_dbg_flag & 20 as libc::c_int != 0 {
        j = 0 as libc::c_int;
        while j < n_regs0 {
            i = (*regs0.offset(j as isize)).as_0;
            while i < (*regs0.offset(j as isize)).as_0 + (*regs0.offset(j as isize)).cnt
            {
                if i == (*regs0.offset(j as isize)).as_0 {
                    tmp___3 = 0 as libc::c_int;
                } else {
                    tmp___3 = (*a.offset(i as isize)).y as int32_t
                        - (*a.offset((i - 1 as libc::c_int) as isize)).y as int32_t
                        - ((*a.offset(i as isize)).x as int32_t
                            - (*a.offset((i - 1 as libc::c_int) as isize)).x as int32_t);
                }
                fprintf(
                    stderr,
                    b"CN\t%d\t%s\t%d\t%c\t%d\t%d\t%d\n\0" as *const u8
                        as *const libc::c_char,
                    j,
                    (*((*mi).seq)
                        .offset(
                            ((*a.offset(i as isize)).x << 1 as libc::c_int
                                >> 33 as libc::c_int) as isize,
                        ))
                        .name,
                    (*a.offset(i as isize)).x as int32_t,
                    *(b"+-\0" as *const u8 as *const libc::c_char)
                        .offset(
                            ((*a.offset(i as isize)).x >> 63 as libc::c_int) as isize,
                        ) as libc::c_int,
                    (*a.offset(i as isize)).y as int32_t,
                    ((*a.offset(i as isize)).y >> 32 as libc::c_int
                        & 255 as libc::c_ulong) as int32_t,
                    tmp___3,
                );
                i += 1;
            }
            j += 1;
        }
    }
    chain_post(
        opt,
        max_chain_gap_ref,
        mi,
        (*b).km,
        qlen_sum,
        n_segs,
        qlens,
        &mut n_regs0,
        regs0,
        a,
    );
    if is_sr == 0 {
        if (*opt).flag as libc::c_longlong & 4294967296 as libc::c_longlong == 0 {
            mm_est_err(
                mi,
                qlen_sum,
                n_regs0,
                regs0,
                a as *const mm128_t,
                n_mini_pos,
                mini_pos as *const uint64_t,
            );
            n_regs0 = mm_filter_strand_retained(n_regs0, regs0);
        }
    }
    if n_segs == 1 as libc::c_int {
        regs0 = align_regs(
            opt,
            mi,
            (*b).km,
            *qlens.offset(0 as libc::c_int as isize),
            *seqs.offset(0 as libc::c_int as isize),
            &mut n_regs0,
            regs0,
            a,
        );
        tmp___4 = realloc(
            regs0 as *mut libc::c_void,
            (::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong)
                .wrapping_mul(n_regs0 as libc::c_ulong),
        );
        regs0 = tmp___4 as *mut mm_reg1_t;
        mm_set_mapq(
            (*b).km,
            n_regs0,
            regs0,
            (*opt).min_chain_score,
            (*opt).a,
            rep_len,
            is_sr,
        );
        *n_regs.offset(0 as libc::c_int as isize) = n_regs0;
        let ref mut fresh36 = *regs.offset(0 as libc::c_int as isize);
        *fresh36 = regs0;
    } else {
        seg = mm_seg_gen(
            (*b).km,
            hash,
            n_segs,
            qlens,
            n_regs0,
            regs0 as *const mm_reg1_t,
            n_regs,
            regs,
            a as *const mm128_t,
        );
        free(regs0 as *mut libc::c_void);
        i = 0 as libc::c_int;
        while i < n_segs {
            mm_set_parent(
                (*b).km,
                (*opt).mask_level,
                (*opt).mask_len,
                *n_regs.offset(i as isize),
                *regs.offset(i as isize),
                (*opt).a * 2 as libc::c_int + (*opt).b,
                ((*opt).flag & 536870912 as libc::c_long) as libc::c_int,
                (*opt).alt_drop,
            );
            let ref mut fresh37 = *regs.offset(i as isize);
            *fresh37 = align_regs(
                opt,
                mi,
                (*b).km,
                *qlens.offset(i as isize),
                *seqs.offset(i as isize),
                n_regs.offset(i as isize),
                *regs.offset(i as isize),
                (*seg.offset(i as isize)).a,
            );
            mm_set_mapq(
                (*b).km,
                *n_regs.offset(i as isize),
                *regs.offset(i as isize),
                (*opt).min_chain_score,
                (*opt).a,
                rep_len,
                is_sr,
            );
            i += 1;
        }
        mm_seg_free((*b).km, n_segs, seg);
        if n_segs == 2 as libc::c_int {
            if (*opt).pe_ori >= 0 as libc::c_int {
                if (*opt).flag & 4 as libc::c_long != 0 {
                    mm_pair(
                        (*b).km,
                        max_chain_gap_ref,
                        (*opt).pe_bonus,
                        (*opt).a * 2 as libc::c_int + (*opt).b,
                        (*opt).a,
                        qlens,
                        n_regs,
                        regs,
                    );
                }
            }
        }
    }
    kfree((*b).km, mv.a as *mut libc::c_void);
    kfree((*b).km, a as *mut libc::c_void);
    kfree((*b).km, u as *mut libc::c_void);
    kfree((*b).km, mini_pos as *mut libc::c_void);
    if !((*b).km).is_null() {
        km_stat((*b).km as *const libc::c_void, &mut kmst);
        if mm_dbg_flag & 2 as libc::c_int != 0 {
            fprintf(
                stderr,
                b"QM\t%s\t%d\tcap=%ld,nCore=%ld,largest=%ld\n\0" as *const u8
                    as *const libc::c_char,
                qname,
                qlen_sum,
                kmst.capacity,
                kmst.n_cores,
                kmst.largest,
            );
        }
        let mut current_block_218: u64;
        if kmst.largest > ((1 as libc::c_uint) << 28 as libc::c_int) as size_t {
            current_block_218 = 2378930188562487258;
        } else if (*opt).cap_kalloc > 0 as libc::c_long {
            if kmst.capacity > (*opt).cap_kalloc as size_t {
                current_block_218 = 2378930188562487258;
            } else {
                current_block_218 = 7147782476509834402;
            }
        } else {
            current_block_218 = 7147782476509834402;
        }
        match current_block_218 {
            2378930188562487258 => {
                if mm_dbg_flag & 2 as libc::c_int != 0 {
                    fprintf(
                        stderr,
                        b"[W::%s] reset thread-local memory after read %s\n\0"
                            as *const u8 as *const libc::c_char,
                        b"mm_map_frag\0" as *const u8 as *const libc::c_char,
                        qname,
                    );
                }
                km_destroy((*b).km);
                (*b).km = km_init();
            }
            _ => {}
        }
    }
}
pub unsafe extern "C" fn mm_map(
    mut mi: *const mm_idx_t,
    mut qlen: libc::c_int,
    mut seq: *const libc::c_char,
    mut n_regs: *mut libc::c_int,
    mut b: *mut mm_tbuf_t,
    mut opt: *const mm_mapopt_t,
    mut qname: *const libc::c_char,
) -> *mut mm_reg1_t {
    let mut regs: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    mm_map_frag(
        mi,
        1 as libc::c_int,
        &mut qlen as *mut libc::c_int as *const libc::c_int,
        &mut seq,
        n_regs,
        &mut regs,
        b,
        opt,
        qname,
    );
    return regs;
}
unsafe extern "C" fn worker_for(
    mut _data: *mut libc::c_void,
    mut i: libc::c_long,
    mut tid: libc::c_int,
) {
    let mut s: *mut step_t___0 = 0 as *mut step_t___0;
    let mut qlens: [libc::c_int; 255] = [0; 255];
    let mut j: libc::c_int = 0;
    let mut off: libc::c_int = 0;
    let mut pe_ori: libc::c_int = 0;
    let mut qseqs: [*const libc::c_char; 255] = [0 as *const libc::c_char; 255];
    let mut t: libc::c_double = 0.;
    let mut b: *mut mm_tbuf_t = 0 as *mut mm_tbuf_t;
    let mut k: libc::c_int = 0;
    let mut t___0: libc::c_int = 0;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut tmp___0: libc::c_double = 0.;
    s = _data as *mut step_t___0;
    off = *((*s).seg_off).offset(i as isize);
    pe_ori = (*(*(*s).p).opt).pe_ori;
    t = 0.0f64;
    b = *((*s).buf).offset(tid as isize);
    if !(*((*s).n_seg).offset(i as isize) <= 255 as libc::c_int) {
        __assert_fail(
            b"s->n_seg[i] <= MM_MAX_SEG\0" as *const u8 as *const libc::c_char,
            b"map.c\0" as *const u8 as *const libc::c_char,
            420 as libc::c_uint,
            b"worker_for\0" as *const u8 as *const libc::c_char,
        );
    }
    if mm_dbg_flag & 2 as libc::c_int != 0 {
        fprintf(
            stderr,
            b"QR\t%s\t%d\t%d\n\0" as *const u8 as *const libc::c_char,
            (*((*s).seq).offset(off as isize)).name,
            tid,
            (*((*s).seq).offset(off as isize)).l_seq,
        );
        t = realtime();
    }
    j = 0 as libc::c_int;
    while j < *((*s).n_seg).offset(i as isize) {
        if *((*s).n_seg).offset(i as isize) == 2 as libc::c_int {
            let mut current_block_19: u64;
            if j == 0 as libc::c_int {
                if pe_ori >> 1 as libc::c_int & 1 as libc::c_int != 0 {
                    mm_revcomp_bseq(((*s).seq).offset((off + j) as isize));
                    current_block_19 = 10652014663920648156;
                } else {
                    current_block_19 = 1135735465937608776;
                }
            } else {
                current_block_19 = 1135735465937608776;
            }
            match current_block_19 {
                1135735465937608776 => {
                    if j == 1 as libc::c_int {
                        if pe_ori & 1 as libc::c_int != 0 {
                            mm_revcomp_bseq(((*s).seq).offset((off + j) as isize));
                        }
                    }
                }
                _ => {}
            }
        }
        qlens[j as usize] = (*((*s).seq).offset((off + j) as isize)).l_seq;
        qseqs[j
            as usize] = (*((*s).seq).offset((off + j) as isize)).seq
            as *const libc::c_char;
        j += 1;
    }
    if (*(*(*s).p).opt).flag & 131072 as libc::c_long != 0 {
        j = 0 as libc::c_int;
        while j < *((*s).n_seg).offset(i as isize) {
            mm_map_frag(
                (*(*s).p).mi,
                1 as libc::c_int,
                &mut *qlens.as_mut_ptr().offset(j as isize) as *mut libc::c_int
                    as *const libc::c_int,
                &mut *qseqs.as_mut_ptr().offset(j as isize),
                ((*s).n_reg).offset((off + j) as isize),
                ((*s).reg).offset((off + j) as isize),
                b,
                (*(*s).p).opt,
                (*((*s).seq).offset((off + j) as isize)).name as *const libc::c_char,
            );
            *((*s).rep_len).offset((off + j) as isize) = (*b).rep_len;
            *((*s).frag_gap).offset((off + j) as isize) = (*b).frag_gap;
            j += 1;
        }
    } else {
        mm_map_frag(
            (*(*s).p).mi,
            *((*s).n_seg).offset(i as isize),
            qlens.as_mut_ptr() as *const libc::c_int,
            qseqs.as_mut_ptr(),
            ((*s).n_reg).offset(off as isize),
            ((*s).reg).offset(off as isize),
            b,
            (*(*s).p).opt,
            (*((*s).seq).offset(off as isize)).name as *const libc::c_char,
        );
        j = 0 as libc::c_int;
        while j < *((*s).n_seg).offset(i as isize) {
            *((*s).rep_len).offset((off + j) as isize) = (*b).rep_len;
            *((*s).frag_gap).offset((off + j) as isize) = (*b).frag_gap;
            j += 1;
        }
    }
    j = 0 as libc::c_int;
    while j < *((*s).n_seg).offset(i as isize) {
        if *((*s).n_seg).offset(i as isize) == 2 as libc::c_int {
            let mut current_block_55: u64;
            if j == 0 as libc::c_int {
                if pe_ori >> 1 as libc::c_int & 1 as libc::c_int != 0 {
                    current_block_55 = 11845342122146411189;
                } else {
                    current_block_55 = 9765189940802733743;
                }
            } else {
                current_block_55 = 9765189940802733743;
            }
            match current_block_55 {
                9765189940802733743 => {
                    if j == 1 as libc::c_int {
                        if pe_ori & 1 as libc::c_int != 0 {
                            current_block_55 = 11845342122146411189;
                        } else {
                            current_block_55 = 6072622540298447352;
                        }
                    } else {
                        current_block_55 = 6072622540298447352;
                    }
                }
                _ => {}
            }
            match current_block_55 {
                11845342122146411189 => {
                    mm_revcomp_bseq(((*s).seq).offset((off + j) as isize));
                    k = 0 as libc::c_int;
                    while k < *((*s).n_reg).offset((off + j) as isize) {
                        r = (*((*s).reg).offset((off + j) as isize)).offset(k as isize);
                        t___0 = (*r).qs;
                        (*r).qs = qlens[j as usize] - (*r).qe;
                        (*r).qe = qlens[j as usize] - t___0;
                        (*r).set_rev(((*r).rev() == 0) as libc::c_int as uint32_t);
                        k += 1;
                    }
                }
                _ => {}
            }
        }
        j += 1;
    }
    if mm_dbg_flag & 2 as libc::c_int != 0 {
        tmp___0 = realtime();
        fprintf(
            stderr,
            b"QT\t%s\t%d\t%.6f\n\0" as *const u8 as *const libc::c_char,
            (*((*s).seq).offset(off as isize)).name,
            tid,
            tmp___0 - t,
        );
    }
}
unsafe extern "C" fn merge_hits(mut s: *mut step_t___0) {
    let mut f: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut k0: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut max_seg: libc::c_int = 0;
    let mut n_reg_part: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rep_len_part: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut frag_gap_part: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut qlens: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut km: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fp: *mut *mut FILE = 0 as *mut *mut FILE;
    let mut opt: *const mm_mapopt_t = 0 as *const mm_mapopt_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut rep_len: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut capacity: uint32_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut r___0: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    max_seg = 0 as libc::c_int;
    fp = (*(*s).p).fp_parts;
    opt = (*(*s).p).opt;
    km = km_init();
    f = 0 as libc::c_int;
    while f < (*s).n_frag {
        if max_seg > *((*s).n_seg).offset(f as isize) {
            max_seg = max_seg;
        } else {
            max_seg = *((*s).n_seg).offset(f as isize);
        }
        f += 1;
    }
    tmp = calloc(
        (max_seg + (*(*s).p).n_parts * 3 as libc::c_int) as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    qlens = tmp as *mut libc::c_int;
    n_reg_part = qlens.offset(max_seg as isize);
    rep_len_part = n_reg_part.offset((*(*s).p).n_parts as isize);
    frag_gap_part = rep_len_part.offset((*(*s).p).n_parts as isize);
    f = 0 as libc::c_int;
    k0 = 0 as libc::c_int;
    k = k0;
    while f < (*s).n_frag {
        k0 = k;
        i = 0 as libc::c_int;
        while i < *((*s).n_seg).offset(f as isize) {
            rep_len = 0 as libc::c_int;
            *qlens.offset(i as isize) = (*((*s).seq).offset(k as isize)).l_seq;
            j = 0 as libc::c_int;
            *((*s).n_reg).offset(k as isize) = 0 as libc::c_int;
            while j < (*(*s).p).n_parts {
                mm_err_fread(
                    n_reg_part.offset(j as isize) as *mut libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    1 as libc::c_int as size_t,
                    *fp.offset(j as isize),
                );
                mm_err_fread(
                    rep_len_part.offset(j as isize) as *mut libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    1 as libc::c_int as size_t,
                    *fp.offset(j as isize),
                );
                mm_err_fread(
                    frag_gap_part.offset(j as isize) as *mut libc::c_void,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    1 as libc::c_int as size_t,
                    *fp.offset(j as isize),
                );
                *((*s).n_reg).offset(k as isize) += *n_reg_part.offset(j as isize);
                if rep_len < *rep_len_part.offset(j as isize) {
                    rep_len = *rep_len_part.offset(j as isize);
                }
                j += 1;
            }
            tmp___0 = calloc(
                *((*s).n_reg).offset(k as isize) as size_t,
                ::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong,
            );
            let ref mut fresh38 = *((*s).reg).offset(k as isize);
            *fresh38 = tmp___0 as *mut mm_reg1_t;
            j = 0 as libc::c_int;
            l = 0 as libc::c_int;
            while j < (*(*s).p).n_parts {
                t = 0 as libc::c_int;
                while t < *n_reg_part.offset(j as isize) {
                    r = (*((*s).reg).offset(k as isize)).offset(l as isize);
                    mm_err_fread(
                        r as *mut libc::c_void,
                        ::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong,
                        1 as libc::c_int as size_t,
                        *fp.offset(j as isize),
                    );
                    (*r)
                        .rid = ((*r).rid as uint32_t)
                        .wrapping_add(*((*(*s).p).rid_shift).offset(j as isize))
                        as int32_t;
                    if (*opt).flag & 4 as libc::c_long != 0 {
                        mm_err_fread(
                            &mut capacity as *mut uint32_t as *mut libc::c_void,
                            4 as libc::c_int as size_t,
                            1 as libc::c_int as size_t,
                            *fp.offset(j as isize),
                        );
                        tmp___1 = calloc(capacity as size_t, 4 as libc::c_int as size_t);
                        (*r).p = tmp___1 as *mut mm_extra_t;
                        (*(*r).p).capacity = capacity;
                        mm_err_fread(
                            (*r).p as *mut libc::c_void,
                            (*(*r).p).capacity as size_t,
                            4 as libc::c_int as size_t,
                            *fp.offset(j as isize),
                        );
                    }
                    t += 1;
                    l += 1;
                }
                j += 1;
            }
            if (*opt).flag & 4096 as libc::c_long == 0 {
                if (*((*s).seq).offset(k as isize)).l_seq >= (*opt).rank_min_len {
                    mm_update_dp_max(
                        (*((*s).seq).offset(k as isize)).l_seq,
                        *((*s).n_reg).offset(k as isize),
                        *((*s).reg).offset(k as isize),
                        (*opt).rank_frac,
                        (*opt).a,
                        (*opt).b,
                    );
                }
            }
            j = 0 as libc::c_int;
            while j < *((*s).n_reg).offset(k as isize) {
                r___0 = (*((*s).reg).offset(k as isize)).offset(j as isize);
                if !((*r___0).p).is_null() {
                    (*(*r___0).p).dp_max2 = 0 as libc::c_int;
                }
                (*r___0).subsc = 0 as libc::c_int;
                (*r___0).n_sub = 0 as libc::c_int;
                j += 1;
            }
            mm_hit_sort(
                km,
                ((*s).n_reg).offset(k as isize),
                *((*s).reg).offset(k as isize),
                (*opt).alt_drop,
            );
            mm_set_parent(
                km,
                (*opt).mask_level,
                (*opt).mask_len,
                *((*s).n_reg).offset(k as isize),
                *((*s).reg).offset(k as isize),
                (*opt).a * 2 as libc::c_int + (*opt).b,
                ((*opt).flag & 536870912 as libc::c_long) as libc::c_int,
                (*opt).alt_drop,
            );
            if (*opt).flag & 8388608 as libc::c_long == 0 {
                mm_select_sub(
                    km,
                    (*opt).pri_ratio,
                    (*(*(*s).p).mi).k * 2 as libc::c_int,
                    (*opt).best_n,
                    0 as libc::c_int,
                    ((*opt).max_gap as libc::c_double * 0.8f64) as libc::c_int,
                    ((*s).n_reg).offset(k as isize),
                    *((*s).reg).offset(k as isize),
                );
                mm_set_sam_pri(
                    *((*s).n_reg).offset(k as isize),
                    *((*s).reg).offset(k as isize),
                );
            }
            mm_set_mapq(
                km,
                *((*s).n_reg).offset(k as isize),
                *((*s).reg).offset(k as isize),
                (*opt).min_chain_score,
                (*opt).a,
                rep_len,
                ((*opt).flag & 4096 as libc::c_long != 0) as libc::c_int,
            );
            i += 1;
            k += 1;
        }
        if *((*s).n_seg).offset(f as isize) == 2 as libc::c_int {
            if (*opt).pe_ori >= 0 as libc::c_int {
                if (*opt).flag & 4 as libc::c_long != 0 {
                    mm_pair(
                        km,
                        *frag_gap_part.offset(0 as libc::c_int as isize),
                        (*opt).pe_bonus,
                        (*opt).a * 2 as libc::c_int + (*opt).b,
                        (*opt).a,
                        qlens as *const libc::c_int,
                        ((*s).n_reg).offset(k0 as isize),
                        ((*s).reg).offset(k0 as isize),
                    );
                }
            }
        }
        f += 1;
    }
    free(qlens as *mut libc::c_void);
    km_destroy(km);
}
unsafe extern "C" fn worker_pipeline___0(
    mut shared: *mut libc::c_void,
    mut step: libc::c_int,
    mut in_0: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: *mut pipeline_t___0 = 0 as *mut pipeline_t___0;
    let mut with_qual: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut with_comment: libc::c_int = 0;
    let mut frag_mode: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut s: *mut step_t___0 = 0 as *mut step_t___0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut km: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut s___0: *mut step_t___0 = 0 as *mut step_t___0;
    let mut mi: *const mm_idx_t = 0 as *const mm_idx_t;
    let mut seg_st: libc::c_int = 0;
    let mut seg_en: libc::c_int = 0;
    let mut t: *mut mm_bseq1_t = 0 as *mut mm_bseq1_t;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut r___0: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut tmp___9: libc::c_double = 0.;
    let mut tmp___10: libc::c_double = 0.;
    let mut tmp___11: libc::c_double = 0.;
    p = shared as *mut pipeline_t___0;
    if step == 0 as libc::c_int {
        if (*(*p).opt).flag & 8 as libc::c_long != 0 {
            if (*(*p).opt).flag & 16 as libc::c_long == 0 {
                tmp = 1 as libc::c_int;
            } else {
                tmp = 0 as libc::c_int;
            }
        } else {
            tmp = 0 as libc::c_int;
        }
        with_qual = tmp;
        with_comment = ((*(*p).opt).flag & 33554432 as libc::c_long != 0) as libc::c_int;
        if (*p).n_fp > 1 as libc::c_int {
            tmp___0 = 1 as libc::c_int;
        } else if (*(*p).opt).flag & 8192 as libc::c_long != 0 {
            tmp___0 = 1 as libc::c_int;
        } else {
            tmp___0 = 0 as libc::c_int;
        }
        frag_mode = tmp___0;
        tmp___1 = calloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<step_t___0>() as libc::c_ulong,
        );
        s = tmp___1 as *mut step_t___0;
        if (*p).n_fp > 1 as libc::c_int {
            (*s)
                .seq = mm_bseq_read_frag2(
                (*p).n_fp,
                (*p).fp,
                (*p).mini_batch_size,
                with_qual,
                with_comment,
                &mut (*s).n_seq,
            );
        } else {
            (*s)
                .seq = mm_bseq_read3(
                *((*p).fp).offset(0 as libc::c_int as isize),
                (*p).mini_batch_size,
                with_qual,
                with_comment,
                frag_mode,
                &mut (*s).n_seq,
            );
        }
        if !((*s).seq).is_null() {
            (*s).p = p as *const pipeline_t___0;
            i = 0 as libc::c_int;
            while i < (*s).n_seq {
                tmp___2 = (*p).n_processed;
                (*p).n_processed += 1;
                (*((*s).seq).offset(i as isize)).rid = tmp___2;
                i += 1;
            }
            tmp___3 = calloc(
                (*p).n_threads as size_t,
                ::std::mem::size_of::<*mut mm_tbuf_t>() as libc::c_ulong,
            );
            (*s).buf = tmp___3 as *mut *mut mm_tbuf_t;
            i = 0 as libc::c_int;
            while i < (*p).n_threads {
                let ref mut fresh39 = *((*s).buf).offset(i as isize);
                *fresh39 = mm_tbuf_init();
                i += 1;
            }
            tmp___4 = calloc(
                (5 as libc::c_int * (*s).n_seq) as size_t,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            );
            (*s).n_reg = tmp___4 as *mut libc::c_int;
            (*s).seg_off = ((*s).n_reg).offset((*s).n_seq as isize);
            (*s).n_seg = ((*s).seg_off).offset((*s).n_seq as isize);
            (*s).rep_len = ((*s).n_seg).offset((*s).n_seq as isize);
            (*s).frag_gap = ((*s).rep_len).offset((*s).n_seq as isize);
            tmp___5 = calloc(
                (*s).n_seq as size_t,
                ::std::mem::size_of::<*mut mm_reg1_t>() as libc::c_ulong,
            );
            (*s).reg = tmp___5 as *mut *mut mm_reg1_t;
            i = 1 as libc::c_int;
            j = 0 as libc::c_int;
            while i <= (*s).n_seq {
                if i == (*s).n_seq {
                    *((*s).n_seg).offset((*s).n_frag as isize) = i - j;
                    tmp___6 = (*s).n_frag;
                    (*s).n_frag += 1;
                    *((*s).seg_off).offset(tmp___6 as isize) = j;
                    j = i;
                } else if frag_mode == 0 {
                    *((*s).n_seg).offset((*s).n_frag as isize) = i - j;
                    tmp___6 = (*s).n_frag;
                    (*s).n_frag += 1;
                    *((*s).seg_off).offset(tmp___6 as isize) = j;
                    j = i;
                } else {
                    tmp___7 = mm_qname_same(
                        (*((*s).seq).offset((i - 1 as libc::c_int) as isize)).name
                            as *const libc::c_char,
                        (*((*s).seq).offset(i as isize)).name as *const libc::c_char,
                    );
                    if tmp___7 == 0 {
                        *((*s).n_seg).offset((*s).n_frag as isize) = i - j;
                        tmp___6 = (*s).n_frag;
                        (*s).n_frag += 1;
                        *((*s).seg_off).offset(tmp___6 as isize) = j;
                        j = i;
                    }
                }
                i += 1;
            }
            return s as *mut libc::c_void;
        } else {
            free(s as *mut libc::c_void);
        }
    } else if step == 1 as libc::c_int {
        if (*p).n_parts > 0 as libc::c_int {
            merge_hits(in_0 as *mut step_t___0);
        } else {
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
                (*(in_0 as *mut step_t___0)).n_frag as libc::c_long,
            );
        }
        return in_0;
    } else {
        if step == 2 as libc::c_int {
            km = 0 as *mut libc::c_void;
            s___0 = in_0 as *mut step_t___0;
            mi = (*p).mi;
            i = 0 as libc::c_int;
            while i < (*p).n_threads {
                mm_tbuf_destroy(*((*s___0).buf).offset(i as isize));
                i += 1;
            }
            free((*s___0).buf as *mut libc::c_void);
            if (*(*p).opt).flag & 64 as libc::c_long != 0 {
                if mm_dbg_flag & 1 as libc::c_int == 0 {
                    km = km_init();
                }
            }
            k = 0 as libc::c_int;
            while k < (*s___0).n_frag {
                seg_st = *((*s___0).seg_off).offset(k as isize);
                seg_en = *((*s___0).seg_off).offset(k as isize)
                    + *((*s___0).n_seg).offset(k as isize);
                i = seg_st;
                while i < seg_en {
                    t = ((*s___0).seq).offset(i as isize);
                    let mut current_block_149: u64;
                    if !((*(*p).opt).split_prefix).is_null() {
                        if (*p).n_parts == 0 as libc::c_int {
                            mm_err_fwrite(
                                ((*s___0).n_reg).offset(i as isize) as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                1 as libc::c_int as size_t,
                                (*p).fp_split,
                            );
                            mm_err_fwrite(
                                ((*s___0).rep_len).offset(i as isize)
                                    as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                1 as libc::c_int as size_t,
                                (*p).fp_split,
                            );
                            mm_err_fwrite(
                                ((*s___0).frag_gap).offset(i as isize)
                                    as *const libc::c_void,
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                                1 as libc::c_int as size_t,
                                (*p).fp_split,
                            );
                            j = 0 as libc::c_int;
                            while j < *((*s___0).n_reg).offset(i as isize) {
                                r = (*((*s___0).reg).offset(i as isize)).offset(j as isize);
                                mm_err_fwrite(
                                    r as *const libc::c_void,
                                    ::std::mem::size_of::<mm_reg1_t>() as libc::c_ulong,
                                    1 as libc::c_int as size_t,
                                    (*p).fp_split,
                                );
                                if (*(*p).opt).flag & 4 as libc::c_long != 0 {
                                    mm_err_fwrite(
                                        &mut (*(*r).p).capacity as *mut uint32_t
                                            as *const libc::c_void,
                                        4 as libc::c_int as size_t,
                                        1 as libc::c_int as size_t,
                                        (*p).fp_split,
                                    );
                                    mm_err_fwrite(
                                        (*r).p as *const libc::c_void,
                                        (*(*r).p).capacity as size_t,
                                        4 as libc::c_int as size_t,
                                        (*p).fp_split,
                                    );
                                }
                                j += 1;
                            }
                            current_block_149 = 16981061190961355901;
                        } else {
                            current_block_149 = 2082541250129836153;
                        }
                    } else {
                        current_block_149 = 2082541250129836153;
                    }
                    match current_block_149 {
                        2082541250129836153 => {
                            if *((*s___0).n_reg).offset(i as isize) > 0 as libc::c_int {
                                j = 0 as libc::c_int;
                                while j < *((*s___0).n_reg).offset(i as isize) {
                                    let mut current_block_138: u64;
                                    r___0 = (*((*s___0).reg).offset(i as isize))
                                        .offset(j as isize);
                                    if (*r___0).sam_pri() != 0 {
                                        if !((*r___0).id == (*r___0).parent) {
                                            __assert_fail(
                                                b"!r->sam_pri || r->id == r->parent\0" as *const u8
                                                    as *const libc::c_char,
                                                b"map.c\0" as *const u8 as *const libc::c_char,
                                                588 as libc::c_uint,
                                                b"worker_pipeline\0" as *const u8 as *const libc::c_char,
                                            );
                                        }
                                    }
                                    if (*(*p).opt).flag & 16384 as libc::c_long != 0 {
                                        if (*r___0).id != (*r___0).parent {
                                            current_block_138 = 10567489114655037681;
                                        } else {
                                            current_block_138 = 2956972668325154207;
                                        }
                                    } else {
                                        current_block_138 = 2956972668325154207;
                                    }
                                    match current_block_138 {
                                        2956972668325154207 => {
                                            if (*(*p).opt).flag & 8 as libc::c_long != 0 {
                                                mm_write_sam3(
                                                    &mut (*p).str_0,
                                                    mi,
                                                    t as *const mm_bseq1_t,
                                                    i - seg_st,
                                                    j,
                                                    *((*s___0).n_seg).offset(k as isize),
                                                    ((*s___0).n_reg).offset(seg_st as isize)
                                                        as *const libc::c_int,
                                                    ((*s___0).reg).offset(seg_st as isize)
                                                        as *const *const mm_reg1_t,
                                                    km,
                                                    (*(*p).opt).flag,
                                                    *((*s___0).rep_len).offset(i as isize),
                                                );
                                            } else {
                                                mm_write_paf3(
                                                    &mut (*p).str_0,
                                                    mi,
                                                    t as *const mm_bseq1_t,
                                                    r___0 as *const mm_reg1_t,
                                                    km,
                                                    (*(*p).opt).flag,
                                                    *((*s___0).rep_len).offset(i as isize),
                                                );
                                            }
                                            mm_err_puts((*p).str_0.s as *const libc::c_char);
                                        }
                                        _ => {}
                                    }
                                    j += 1;
                                }
                            } else {
                                let mut current_block_147: u64;
                                if (*(*p).opt).flag & 134217728 as libc::c_long != 0 {
                                    current_block_147 = 12506246967550130847;
                                } else if (*(*p).opt).flag & 8 as libc::c_long != 0 {
                                    if (*(*p).opt).flag & 1073741824 as libc::c_long == 0 {
                                        current_block_147 = 12506246967550130847;
                                    } else {
                                        current_block_147 = 13454018412769612774;
                                    }
                                } else {
                                    current_block_147 = 13454018412769612774;
                                }
                                match current_block_147 {
                                    12506246967550130847 => {
                                        if (*(*p).opt).flag & 8 as libc::c_long != 0 {
                                            mm_write_sam3(
                                                &mut (*p).str_0,
                                                mi,
                                                t as *const mm_bseq1_t,
                                                i - seg_st,
                                                -(1 as libc::c_int),
                                                *((*s___0).n_seg).offset(k as isize),
                                                ((*s___0).n_reg).offset(seg_st as isize)
                                                    as *const libc::c_int,
                                                ((*s___0).reg).offset(seg_st as isize)
                                                    as *const *const mm_reg1_t,
                                                km,
                                                (*(*p).opt).flag,
                                                *((*s___0).rep_len).offset(i as isize),
                                            );
                                        } else {
                                            mm_write_paf3(
                                                &mut (*p).str_0,
                                                mi,
                                                t as *const mm_bseq1_t,
                                                0 as *const mm_reg1_t,
                                                0 as *mut libc::c_void,
                                                (*(*p).opt).flag,
                                                *((*s___0).rep_len).offset(i as isize),
                                            );
                                        }
                                        mm_err_puts((*p).str_0.s as *const libc::c_char);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
                i = seg_st;
                while i < seg_en {
                    j = 0 as libc::c_int;
                    while j < *((*s___0).n_reg).offset(i as isize) {
                        free(
                            (*(*((*s___0).reg).offset(i as isize)).offset(j as isize)).p
                                as *mut libc::c_void,
                        );
                        j += 1;
                    }
                    free(*((*s___0).reg).offset(i as isize) as *mut libc::c_void);
                    free((*((*s___0).seq).offset(i as isize)).seq as *mut libc::c_void);
                    free((*((*s___0).seq).offset(i as isize)).name as *mut libc::c_void);
                    if !((*((*s___0).seq).offset(i as isize)).qual).is_null() {
                        free(
                            (*((*s___0).seq).offset(i as isize)).qual
                                as *mut libc::c_void,
                        );
                    }
                    if !((*((*s___0).seq).offset(i as isize)).comment).is_null() {
                        free(
                            (*((*s___0).seq).offset(i as isize)).comment
                                as *mut libc::c_void,
                        );
                    }
                    i += 1;
                }
                k += 1;
            }
            free((*s___0).reg as *mut libc::c_void);
            free((*s___0).n_reg as *mut libc::c_void);
            free((*s___0).seq as *mut libc::c_void);
            km_destroy(km);
            if mm_verbose >= 3 as libc::c_int {
                tmp___9 = cputime();
                tmp___10 = realtime();
                tmp___11 = realtime();
                fprintf(
                    stderr,
                    b"[M::%s::%.3f*%.2f] mapped %d sequences\n\0" as *const u8
                        as *const libc::c_char,
                    b"worker_pipeline\0" as *const u8 as *const libc::c_char,
                    tmp___11 - mm_realtime0,
                    tmp___9 / (tmp___10 - mm_realtime0),
                    (*s___0).n_seq,
                );
            }
            free(s___0 as *mut libc::c_void);
        }
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn open_bseqs(
    mut n: libc::c_int,
    mut fn_0: *mut *const libc::c_char,
) -> *mut *mut mm_bseq_file_t {
    let mut fp: *mut *mut mm_bseq_file_t = 0 as *mut *mut mm_bseq_file_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut mm_bseq_file_t = 0 as *mut mm_bseq_file_t;
    tmp = calloc(
        n as size_t,
        ::std::mem::size_of::<*mut mm_bseq_file_t>() as libc::c_ulong,
    );
    fp = tmp as *mut *mut mm_bseq_file_t;
    i = 0 as libc::c_int;
    while i < n {
        tmp___2 = mm_bseq_open(*fn_0.offset(i as isize));
        let ref mut fresh40 = *fp.offset(i as isize);
        *fresh40 = tmp___2;
        if tmp___2 as libc::c_ulong == 0 as *mut mm_bseq_file_t as libc::c_ulong {
            if mm_verbose >= 1 as libc::c_int {
                tmp___0 = __errno_location();
                tmp___1 = strerror(*tmp___0);
                fprintf(
                    stderr,
                    b"ERROR: failed to open file '%s': %s\n\0" as *const u8
                        as *const libc::c_char,
                    *fn_0.offset(i as isize),
                    tmp___1,
                );
            }
            j = 0 as libc::c_int;
            while j < i {
                mm_bseq_close(*fp.offset(j as isize));
                j += 1;
            }
            free(fp as *mut libc::c_void);
            return 0 as *mut *mut mm_bseq_file_t;
        }
        i += 1;
    }
    return fp;
}
pub unsafe extern "C" fn mm_map_file_frag(
    mut idx: *const mm_idx_t,
    mut n_segs: libc::c_int,
    mut fn_0: *mut *const libc::c_char,
    mut opt: *const mm_mapopt_t,
    mut n_threads: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pl_threads: libc::c_int = 0;
    let mut pl: pipeline_t___0 = pipeline_t___0 {
        n_processed: 0,
        n_threads: 0,
        n_fp: 0,
        mini_batch_size: 0,
        opt: 0 as *const mm_mapopt_t,
        fp: 0 as *mut *mut mm_bseq_file_t,
        mi: 0 as *const mm_idx_t,
        str_0: kstring_t {
            l: 0,
            m: 0,
            s: 0 as *mut libc::c_char,
        },
        n_parts: 0,
        rid_shift: 0 as *mut uint32_t,
        fp_split: 0 as *mut FILE,
        fp_parts: 0 as *mut *mut FILE,
    };
    let mut tmp: libc::c_int = 0;
    if n_segs < 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    memset(
        &mut pl as *mut pipeline_t___0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pipeline_t___0>() as libc::c_ulong,
    );
    pl.n_fp = n_segs;
    pl.fp = open_bseqs(pl.n_fp, fn_0);
    if pl.fp as libc::c_ulong == 0 as *mut *mut mm_bseq_file_t as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    pl.opt = opt;
    pl.mi = idx;
    if n_threads > 1 as libc::c_int {
        pl.n_threads = n_threads;
    } else {
        pl.n_threads = 1 as libc::c_int;
    }
    pl.mini_batch_size = (*opt).mini_batch_size;
    if !((*opt).split_prefix).is_null() {
        pl.fp_split = mm_split_init((*opt).split_prefix, idx);
    }
    if n_threads == 1 as libc::c_int {
        pl_threads = 1 as libc::c_int;
    } else {
        if (*opt).flag & 32768 as libc::c_long != 0 {
            tmp = 3 as libc::c_int;
        } else {
            tmp = 2 as libc::c_int;
        }
        pl_threads = tmp;
    }
    kt_pipeline(
        pl_threads,
        Some(
            worker_pipeline___0
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        &mut pl as *mut pipeline_t___0 as *mut libc::c_void,
        3 as libc::c_int,
    );
    free(pl.str_0.s as *mut libc::c_void);
    if !(pl.fp_split).is_null() {
        fclose(pl.fp_split);
    }
    i = 0 as libc::c_int;
    while i < pl.n_fp {
        mm_bseq_close(*(pl.fp).offset(i as isize));
        i += 1;
    }
    free(pl.fp as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mm_map_file(
    mut idx: *const mm_idx_t,
    mut fn_0: *const libc::c_char,
    mut opt: *const mm_mapopt_t,
    mut n_threads: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = mm_map_file_frag(idx, 1 as libc::c_int, &mut fn_0, opt, n_threads);
    return tmp;
}
pub unsafe extern "C" fn mm_split_merge(
    mut n_segs: libc::c_int,
    mut fn_0: *mut *const libc::c_char,
    mut opt: *const mm_mapopt_t,
    mut n_split_idx: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pl: pipeline_t___0 = pipeline_t___0 {
        n_processed: 0,
        n_threads: 0,
        n_fp: 0,
        mini_batch_size: 0,
        opt: 0 as *const mm_mapopt_t,
        fp: 0 as *mut *mut mm_bseq_file_t,
        mi: 0 as *const mm_idx_t,
        str_0: kstring_t {
            l: 0,
            m: 0,
            s: 0 as *mut libc::c_char,
        },
        n_parts: 0,
        rid_shift: 0 as *mut uint32_t,
        fp_split: 0 as *mut FILE,
        fp_parts: 0 as *mut *mut FILE,
    };
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if n_segs < 1 as libc::c_int {
        return -(1 as libc::c_int)
    } else {
        if n_split_idx < 1 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    memset(
        &mut pl as *mut pipeline_t___0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pipeline_t___0>() as libc::c_ulong,
    );
    pl.n_fp = n_segs;
    pl.fp = open_bseqs(pl.n_fp, fn_0);
    if pl.fp as libc::c_ulong == 0 as *mut *mut mm_bseq_file_t as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    pl.opt = opt;
    pl.mini_batch_size = (*opt).mini_batch_size;
    pl.n_parts = n_split_idx;
    tmp = calloc(
        pl.n_parts as size_t,
        ::std::mem::size_of::<*mut FILE>() as libc::c_ulong,
    );
    pl.fp_parts = tmp as *mut *mut FILE;
    tmp___0 = calloc(
        pl.n_parts as size_t,
        ::std::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
    pl.rid_shift = tmp___0 as *mut uint32_t;
    mi = mm_split_merge_prep(
        (*opt).split_prefix,
        n_split_idx,
        pl.fp_parts,
        pl.rid_shift,
    );
    pl.mi = mi as *const mm_idx_t;
    if pl.mi as libc::c_ulong == 0 as *const mm_idx_t as libc::c_ulong {
        free(pl.fp_parts as *mut libc::c_void);
        free(pl.rid_shift as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    i = n_split_idx - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        *(pl.rid_shift)
            .offset(
                i as isize,
            ) = *(pl.rid_shift).offset((i - 1 as libc::c_int) as isize);
        i -= 1;
    }
    *(pl.rid_shift).offset(0 as libc::c_int as isize) = 0 as libc::c_int as uint32_t;
    i = 1 as libc::c_int;
    while i < n_split_idx {
        let ref mut fresh41 = *(pl.rid_shift).offset(i as isize);
        *fresh41 = (*fresh41 as libc::c_uint)
            .wrapping_add(*(pl.rid_shift).offset((i - 1 as libc::c_int) as isize))
            as uint32_t as uint32_t;
        i += 1;
    }
    if (*opt).flag & 8 as libc::c_long != 0 {
        i = 0 as libc::c_int;
        while i < (*pl.mi).n_seq as int32_t {
            printf(
                b"@SQ\tSN:%s\tLN:%d\n\0" as *const u8 as *const libc::c_char,
                (*((*pl.mi).seq).offset(i as isize)).name,
                (*((*pl.mi).seq).offset(i as isize)).len,
            );
            i += 1;
        }
    }
    kt_pipeline(
        2 as libc::c_int,
        Some(
            worker_pipeline___0
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
        &mut pl as *mut pipeline_t___0 as *mut libc::c_void,
        3 as libc::c_int,
    );
    free(pl.str_0.s as *mut libc::c_void);
    mm_idx_destroy(mi);
    free(pl.rid_shift as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < n_split_idx {
        fclose(*(pl.fp_parts).offset(i as isize));
        i += 1;
    }
    free(pl.fp_parts as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < pl.n_fp {
        mm_bseq_close(*(pl.fp).offset(i as isize));
        i += 1;
    }
    free(pl.fp as *mut libc::c_void);
    mm_split_rm_tmp((*opt).split_prefix, n_split_idx);
    return 0 as libc::c_int;
}
pub static mut mm_realtime0: libc::c_double = 0.;
pub static mut mm_verbose: libc::c_int = 1 as libc::c_int;
pub static mut mm_dbg_flag: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn cputime() -> libc::c_double {
    let mut r: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        __annonCompField3: __anonunion____missing_field_name_1036346496 {
            ru_maxrss: 0,
        },
        __annonCompField4: __anonunion____missing_field_name_1036346497 {
            ru_ixrss: 0,
        },
        __annonCompField5: __anonunion____missing_field_name_1036346498 {
            ru_idrss: 0,
        },
        __annonCompField6: __anonunion____missing_field_name_1036346499 {
            ru_isrss: 0,
        },
        __annonCompField7: __anonunion____missing_field_name_1036346500 {
            ru_minflt: 0,
        },
        __annonCompField8: __anonunion____missing_field_name_1036346501 {
            ru_majflt: 0,
        },
        __annonCompField9: __anonunion____missing_field_name_1036346502 {
            ru_nswap: 0,
        },
        __annonCompField10: __anonunion____missing_field_name_1036346503 {
            ru_inblock: 0,
        },
        __annonCompField11: __anonunion____missing_field_name_1036346504 {
            ru_oublock: 0,
        },
        __annonCompField12: __anonunion____missing_field_name_1036346505 {
            ru_msgsnd: 0,
        },
        __annonCompField13: __anonunion____missing_field_name_1036346506 {
            ru_msgrcv: 0,
        },
        __annonCompField14: __anonunion____missing_field_name_1036346507 {
            ru_nsignals: 0,
        },
        __annonCompField15: __anonunion____missing_field_name_1036346508 {
            ru_nvcsw: 0,
        },
        __annonCompField16: __anonunion____missing_field_name_1036346509 {
            ru_nivcsw: 0,
        },
    };
    getrusage(0 as libc::c_int, &mut r);
    return (r.ru_utime.tv_sec + r.ru_stime.tv_sec) as libc::c_double
        + 1e-6f64 * (r.ru_utime.tv_usec + r.ru_stime.tv_usec) as libc::c_double;
}
pub unsafe extern "C" fn peakrss() -> libc::c_long {
    let mut r: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        __annonCompField3: __anonunion____missing_field_name_1036346496 {
            ru_maxrss: 0,
        },
        __annonCompField4: __anonunion____missing_field_name_1036346497 {
            ru_ixrss: 0,
        },
        __annonCompField5: __anonunion____missing_field_name_1036346498 {
            ru_idrss: 0,
        },
        __annonCompField6: __anonunion____missing_field_name_1036346499 {
            ru_isrss: 0,
        },
        __annonCompField7: __anonunion____missing_field_name_1036346500 {
            ru_minflt: 0,
        },
        __annonCompField8: __anonunion____missing_field_name_1036346501 {
            ru_majflt: 0,
        },
        __annonCompField9: __anonunion____missing_field_name_1036346502 {
            ru_nswap: 0,
        },
        __annonCompField10: __anonunion____missing_field_name_1036346503 {
            ru_inblock: 0,
        },
        __annonCompField11: __anonunion____missing_field_name_1036346504 {
            ru_oublock: 0,
        },
        __annonCompField12: __anonunion____missing_field_name_1036346505 {
            ru_msgsnd: 0,
        },
        __annonCompField13: __anonunion____missing_field_name_1036346506 {
            ru_msgrcv: 0,
        },
        __annonCompField14: __anonunion____missing_field_name_1036346507 {
            ru_nsignals: 0,
        },
        __annonCompField15: __anonunion____missing_field_name_1036346508 {
            ru_nvcsw: 0,
        },
        __annonCompField16: __anonunion____missing_field_name_1036346509 {
            ru_nivcsw: 0,
        },
    };
    getrusage(0 as libc::c_int, &mut r);
    return r.__annonCompField3.ru_maxrss * 1024 as libc::c_long;
}
pub unsafe extern "C" fn realtime() -> libc::c_double {
    let mut tp: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tp as *mut timeval, 0 as *mut libc::c_void);
    return tp.tv_sec as libc::c_double + tp.tv_usec as libc::c_double * 1e-6f64;
}
pub unsafe extern "C" fn mm_err_puts(mut str: *const libc::c_char) {
    let mut ret: libc::c_int = 0;
    ret = puts(str);
    if ret == -(1 as libc::c_int) {
        perror(
            b"[ERROR] failed to write the results\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn mm_err_fwrite(
    mut p: *const libc::c_void,
    mut size: size_t,
    mut nitems: size_t,
    mut fp: *mut FILE,
) {
    let mut ret: libc::c_int = 0;
    let mut tmp: size_t = 0;
    tmp = fwrite(p, size, nitems, fp);
    ret = tmp as libc::c_int;
    if ret == -(1 as libc::c_int) {
        perror(b"[ERROR] failed to write data\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn mm_err_fread(
    mut p: *mut libc::c_void,
    mut size: size_t,
    mut nitems: size_t,
    mut fp: *mut FILE,
) {
    let mut ret: libc::c_int = 0;
    let mut tmp: size_t = 0;
    tmp = fread(p, size, nitems, fp);
    ret = tmp as libc::c_int;
    if ret == -(1 as libc::c_int) {
        perror(b"[ERROR] failed to read data\0" as *const u8 as *const libc::c_char);
        exit(1 as libc::c_int);
    }
}
pub unsafe extern "C" fn rs_insertsort_128x(
    mut beg: *mut mm128_t,
    mut end: *mut mm128_t,
) {
    let mut i: *mut mm128_t = 0 as *mut mm128_t;
    let mut j: *mut mm128_t = 0 as *mut mm128_t;
    let mut tmp: mm128_t = mm128_t { x: 0, y: 0 };
    i = beg.offset(1 as libc::c_int as isize);
    while (i as libc::c_ulong) < end as libc::c_ulong {
        if (*i).x < (*i.offset(-(1 as libc::c_int as isize))).x {
            tmp = *i;
            j = i;
            while j as libc::c_ulong > beg as libc::c_ulong {
                if !(tmp.x < (*j.offset(-(1 as libc::c_int as isize))).x) {
                    break;
                }
                *j = *j.offset(-(1 as libc::c_int as isize));
                j = j.offset(-1);
            }
            *j = tmp;
        }
        i = i.offset(1);
    }
}
pub unsafe extern "C" fn rs_sort_128x(
    mut beg: *mut mm128_t,
    mut end: *mut mm128_t,
    mut n_bits: libc::c_int,
    mut s: libc::c_int,
) {
    let mut i: *mut mm128_t = 0 as *mut mm128_t;
    let mut size: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut k: *mut rsbucket_128x_t = 0 as *mut rsbucket_128x_t;
    let mut b: [rsbucket_128x_t; 256] = [rsbucket_128x_t {
        b: 0 as *mut mm128_t,
        e: 0 as *mut mm128_t,
    }; 256];
    let mut be: *mut rsbucket_128x_t = 0 as *mut rsbucket_128x_t;
    let mut tmp___0: *mut mm128_t = 0 as *mut mm128_t;
    let mut l: *mut rsbucket_128x_t = 0 as *mut rsbucket_128x_t;
    let mut tmp___1: mm128_t = mm128_t { x: 0, y: 0 };
    let mut swap: mm128_t = mm128_t { x: 0, y: 0 };
    let mut tmp___2: *mut mm128_t = 0 as *mut mm128_t;
    let mut tmp___3: *mut mm128_t = 0 as *mut mm128_t;
    size = (1 as libc::c_int) << n_bits;
    m = size - 1 as libc::c_int;
    be = b.as_mut_ptr().offset(size as isize);
    if !(n_bits <= 8 as libc::c_int) {
        __assert_fail(
            b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
            b"misc.c\0" as *const u8 as *const libc::c_char,
            156 as libc::c_uint,
            b"rs_sort_128x\0" as *const u8 as *const libc::c_char,
        );
    }
    k = b.as_mut_ptr();
    while k as libc::c_ulong != be as libc::c_ulong {
        tmp___0 = beg;
        (*k).e = tmp___0;
        (*k).b = tmp___0;
        k = k.offset(1);
    }
    i = beg;
    while i as libc::c_ulong != end as libc::c_ulong {
        b[((*i).x >> s & m as libc::c_ulong) as usize]
            .e = (b[((*i).x >> s & m as libc::c_ulong) as usize].e).offset(1);
        i = i.offset(1);
    }
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k as libc::c_ulong != be as libc::c_ulong {
        (*k)
            .e = ((*k).e)
            .offset(
                ((*k.offset(-(1 as libc::c_int as isize))).e).offset_from(beg)
                    as libc::c_long as isize,
            );
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
    }
    k = b.as_mut_ptr();
    while k as libc::c_ulong != be as libc::c_ulong {
        if (*k).b as libc::c_ulong != (*k).e as libc::c_ulong {
            l = b.as_mut_ptr().offset(((*(*k).b).x >> s & m as libc::c_ulong) as isize);
            if l as libc::c_ulong != k as libc::c_ulong {
                tmp___1 = *(*k).b;
                loop {
                    swap = tmp___1;
                    tmp___1 = *(*l).b;
                    tmp___2 = (*l).b;
                    (*l).b = ((*l).b).offset(1);
                    *tmp___2 = swap;
                    l = b
                        .as_mut_ptr()
                        .offset((tmp___1.x >> s & m as libc::c_ulong) as isize);
                    if !(l as libc::c_ulong != k as libc::c_ulong) {
                        break;
                    }
                }
                tmp___3 = (*k).b;
                (*k).b = ((*k).b).offset(1);
                *tmp___3 = tmp___1;
            } else {
                (*k).b = ((*k).b).offset(1);
            }
        } else {
            k = k.offset(1);
        }
    }
    b[0 as libc::c_int as usize].b = beg;
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k as libc::c_ulong != be as libc::c_ulong {
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
    }
    if s != 0 {
        if s > n_bits {
            s -= n_bits;
        } else {
            s = 0 as libc::c_int;
        }
        k = b.as_mut_ptr();
        while k as libc::c_ulong != be as libc::c_ulong {
            if ((*k).e).offset_from((*k).b) as libc::c_long > 64 as libc::c_long {
                rs_sort_128x((*k).b, (*k).e, n_bits, s);
            } else if ((*k).e).offset_from((*k).b) as libc::c_long > 1 as libc::c_long {
                rs_insertsort_128x((*k).b, (*k).e);
            }
            k = k.offset(1);
        }
    }
}
pub unsafe extern "C" fn radix_sort_128x(mut beg: *mut mm128_t, mut end: *mut mm128_t) {
    if end.offset_from(beg) as libc::c_long <= 64 as libc::c_long {
        rs_insertsort_128x(beg, end);
    } else {
        rs_sort_128x(beg, end, 8 as libc::c_int, 56 as libc::c_int);
    };
}
pub unsafe extern "C" fn rs_insertsort_64(
    mut beg: *mut uint64_t,
    mut end: *mut uint64_t,
) {
    let mut i: *mut uint64_t = 0 as *mut uint64_t;
    let mut j: *mut uint64_t = 0 as *mut uint64_t;
    let mut tmp: uint64_t = 0;
    i = beg.offset(1 as libc::c_int as isize);
    while (i as libc::c_ulong) < end as libc::c_ulong {
        if *i < *i.offset(-(1 as libc::c_int as isize)) {
            tmp = *i;
            j = i;
            while j as libc::c_ulong > beg as libc::c_ulong {
                if !(tmp < *j.offset(-(1 as libc::c_int as isize))) {
                    break;
                }
                *j = *j.offset(-(1 as libc::c_int as isize));
                j = j.offset(-1);
            }
            *j = tmp;
        }
        i = i.offset(1);
    }
}
pub unsafe extern "C" fn rs_sort_64(
    mut beg: *mut uint64_t,
    mut end: *mut uint64_t,
    mut n_bits: libc::c_int,
    mut s: libc::c_int,
) {
    let mut i: *mut uint64_t = 0 as *mut uint64_t;
    let mut size: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut k: *mut rsbucket_64_t = 0 as *mut rsbucket_64_t;
    let mut b: [rsbucket_64_t; 256] = [rsbucket_64_t {
        b: 0 as *mut uint64_t,
        e: 0 as *mut uint64_t,
    }; 256];
    let mut be: *mut rsbucket_64_t = 0 as *mut rsbucket_64_t;
    let mut tmp___0: *mut uint64_t = 0 as *mut uint64_t;
    let mut l: *mut rsbucket_64_t = 0 as *mut rsbucket_64_t;
    let mut tmp___1: uint64_t = 0;
    let mut swap: uint64_t = 0;
    let mut tmp___2: *mut uint64_t = 0 as *mut uint64_t;
    let mut tmp___3: *mut uint64_t = 0 as *mut uint64_t;
    size = (1 as libc::c_int) << n_bits;
    m = size - 1 as libc::c_int;
    be = b.as_mut_ptr().offset(size as isize);
    if !(n_bits <= 8 as libc::c_int) {
        __assert_fail(
            b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
            b"misc.c\0" as *const u8 as *const libc::c_char,
            159 as libc::c_uint,
            b"rs_sort_64\0" as *const u8 as *const libc::c_char,
        );
    }
    k = b.as_mut_ptr();
    while k as libc::c_ulong != be as libc::c_ulong {
        tmp___0 = beg;
        (*k).e = tmp___0;
        (*k).b = tmp___0;
        k = k.offset(1);
    }
    i = beg;
    while i as libc::c_ulong != end as libc::c_ulong {
        b[(*i >> s & m as libc::c_ulong) as usize]
            .e = (b[(*i >> s & m as libc::c_ulong) as usize].e).offset(1);
        i = i.offset(1);
    }
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k as libc::c_ulong != be as libc::c_ulong {
        (*k)
            .e = ((*k).e)
            .offset(
                ((*k.offset(-(1 as libc::c_int as isize))).e).offset_from(beg)
                    as libc::c_long as isize,
            );
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
    }
    k = b.as_mut_ptr();
    while k as libc::c_ulong != be as libc::c_ulong {
        if (*k).b as libc::c_ulong != (*k).e as libc::c_ulong {
            l = b.as_mut_ptr().offset((*(*k).b >> s & m as libc::c_ulong) as isize);
            if l as libc::c_ulong != k as libc::c_ulong {
                tmp___1 = *(*k).b;
                loop {
                    swap = tmp___1;
                    tmp___1 = *(*l).b;
                    tmp___2 = (*l).b;
                    (*l).b = ((*l).b).offset(1);
                    *tmp___2 = swap;
                    l = b
                        .as_mut_ptr()
                        .offset((tmp___1 >> s & m as libc::c_ulong) as isize);
                    if !(l as libc::c_ulong != k as libc::c_ulong) {
                        break;
                    }
                }
                tmp___3 = (*k).b;
                (*k).b = ((*k).b).offset(1);
                *tmp___3 = tmp___1;
            } else {
                (*k).b = ((*k).b).offset(1);
            }
        } else {
            k = k.offset(1);
        }
    }
    b[0 as libc::c_int as usize].b = beg;
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k as libc::c_ulong != be as libc::c_ulong {
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
    }
    if s != 0 {
        if s > n_bits {
            s -= n_bits;
        } else {
            s = 0 as libc::c_int;
        }
        k = b.as_mut_ptr();
        while k as libc::c_ulong != be as libc::c_ulong {
            if ((*k).e).offset_from((*k).b) as libc::c_long > 64 as libc::c_long {
                rs_sort_64((*k).b, (*k).e, n_bits, s);
            } else if ((*k).e).offset_from((*k).b) as libc::c_long > 1 as libc::c_long {
                rs_insertsort_64((*k).b, (*k).e);
            }
            k = k.offset(1);
        }
    }
}
pub unsafe extern "C" fn radix_sort_64(mut beg: *mut uint64_t, mut end: *mut uint64_t) {
    if end.offset_from(beg) as libc::c_long <= 64 as libc::c_long {
        rs_insertsort_64(beg, end);
    } else {
        rs_sort_64(beg, end, 8 as libc::c_int, 56 as libc::c_int);
    };
}
pub unsafe extern "C" fn ks_heapdown_uint32_t(
    mut i: size_t,
    mut n: size_t,
    mut l: *mut uint32_t,
) {
    let mut k: size_t = 0;
    let mut tmp: uint32_t = 0;
    k = i;
    tmp = *l.offset(i as isize);
    loop {
        k = (k << 1 as libc::c_int).wrapping_add(1 as libc::c_ulong);
        if !(k < n) {
            break;
        }
        if k != n.wrapping_sub(1 as libc::c_ulong) {
            if *l.offset(k as isize)
                < *l.offset(k.wrapping_add(1 as libc::c_ulong) as isize)
            {
                k = k.wrapping_add(1);
            }
        }
        if *l.offset(k as isize) < tmp {
            break;
        }
        *l.offset(i as isize) = *l.offset(k as isize);
        i = k;
    }
    *l.offset(i as isize) = tmp;
}
pub unsafe extern "C" fn ks_heapmake_uint32_t(mut lsize: size_t, mut l: *mut uint32_t) {
    let mut i: size_t = 0;
    i = (lsize >> 1 as libc::c_int).wrapping_sub(1 as libc::c_ulong);
    while i != 0xffffffffffffffff as libc::c_ulong {
        ks_heapdown_uint32_t(i, lsize, l);
        i = i.wrapping_sub(1);
    }
}
pub unsafe extern "C" fn ks_ksmall_uint32_t(
    mut n: size_t,
    mut arr: *mut uint32_t,
    mut kk: size_t,
) -> uint32_t {
    let mut low: *mut uint32_t = 0 as *mut uint32_t;
    let mut high: *mut uint32_t = 0 as *mut uint32_t;
    let mut k: *mut uint32_t = 0 as *mut uint32_t;
    let mut ll: *mut uint32_t = 0 as *mut uint32_t;
    let mut hh: *mut uint32_t = 0 as *mut uint32_t;
    let mut mid: *mut uint32_t = 0 as *mut uint32_t;
    let mut t: uint32_t = 0;
    let mut t___0: uint32_t = 0;
    let mut t___1: uint32_t = 0;
    let mut t___2: uint32_t = 0;
    let mut t___3: uint32_t = 0;
    let mut t___4: uint32_t = 0;
    let mut t___5: uint32_t = 0;
    low = arr;
    high = arr.offset(n as isize).offset(-(1 as libc::c_int as isize));
    k = arr.offset(kk as isize);
    loop {
        if high as libc::c_ulong <= low as libc::c_ulong {
            return *k;
        }
        if high as libc::c_ulong
            == low.offset(1 as libc::c_int as isize) as libc::c_ulong
        {
            if *high < *low {
                t = *low;
                *low = *high;
                *high = t;
            }
            return *k;
        }
        mid = low
            .offset(
                (high.offset_from(low) as libc::c_long / 2 as libc::c_long) as isize,
            );
        if *high < *mid {
            t___0 = *mid;
            *mid = *high;
            *high = t___0;
        }
        if *high < *low {
            t___1 = *low;
            *low = *high;
            *high = t___1;
        }
        if *low < *mid {
            t___2 = *mid;
            *mid = *low;
            *low = t___2;
        }
        t___3 = *mid;
        *mid = *low.offset(1 as libc::c_int as isize);
        *low.offset(1 as libc::c_int as isize) = t___3;
        ll = low.offset(1 as libc::c_int as isize);
        hh = high;
        loop {
            loop {
                ll = ll.offset(1);
                if !(*ll < *low) {
                    break;
                }
            }
            loop {
                hh = hh.offset(-1);
                if !(*low < *hh) {
                    break;
                }
            }
            if (hh as libc::c_ulong) < ll as libc::c_ulong {
                break;
            }
            t___4 = *ll;
            *ll = *hh;
            *hh = t___4;
        }
        t___5 = *low;
        *low = *hh;
        *hh = t___5;
        if hh as libc::c_ulong <= k as libc::c_ulong {
            low = ll;
        }
        if hh as libc::c_ulong >= k as libc::c_ulong {
            high = hh.offset(-(1 as libc::c_int as isize));
        }
    };
}
pub unsafe extern "C" fn ks_heapdown_uint64_t(
    mut i: size_t,
    mut n: size_t,
    mut l: *mut uint64_t,
) {
    let mut k: size_t = 0;
    let mut tmp: uint64_t = 0;
    k = i;
    tmp = *l.offset(i as isize);
    loop {
        k = (k << 1 as libc::c_int).wrapping_add(1 as libc::c_ulong);
        if !(k < n) {
            break;
        }
        if k != n.wrapping_sub(1 as libc::c_ulong) {
            if *l.offset(k as isize)
                < *l.offset(k.wrapping_add(1 as libc::c_ulong) as isize)
            {
                k = k.wrapping_add(1);
            }
        }
        if *l.offset(k as isize) < tmp {
            break;
        }
        *l.offset(i as isize) = *l.offset(k as isize);
        i = k;
    }
    *l.offset(i as isize) = tmp;
}
pub unsafe extern "C" fn ks_heapmake_uint64_t(mut lsize: size_t, mut l: *mut uint64_t) {
    let mut i: size_t = 0;
    i = (lsize >> 1 as libc::c_int).wrapping_sub(1 as libc::c_ulong);
    while i != 0xffffffffffffffff as libc::c_ulong {
        ks_heapdown_uint64_t(i, lsize, l);
        i = i.wrapping_sub(1);
    }
}
pub unsafe extern "C" fn ks_ksmall_uint64_t(
    mut n: size_t,
    mut arr: *mut uint64_t,
    mut kk: size_t,
) -> uint64_t {
    let mut low: *mut uint64_t = 0 as *mut uint64_t;
    let mut high: *mut uint64_t = 0 as *mut uint64_t;
    let mut k: *mut uint64_t = 0 as *mut uint64_t;
    let mut ll: *mut uint64_t = 0 as *mut uint64_t;
    let mut hh: *mut uint64_t = 0 as *mut uint64_t;
    let mut mid: *mut uint64_t = 0 as *mut uint64_t;
    let mut t: uint64_t = 0;
    let mut t___0: uint64_t = 0;
    let mut t___1: uint64_t = 0;
    let mut t___2: uint64_t = 0;
    let mut t___3: uint64_t = 0;
    let mut t___4: uint64_t = 0;
    let mut t___5: uint64_t = 0;
    low = arr;
    high = arr.offset(n as isize).offset(-(1 as libc::c_int as isize));
    k = arr.offset(kk as isize);
    loop {
        if high as libc::c_ulong <= low as libc::c_ulong {
            return *k;
        }
        if high as libc::c_ulong
            == low.offset(1 as libc::c_int as isize) as libc::c_ulong
        {
            if *high < *low {
                t = *low;
                *low = *high;
                *high = t;
            }
            return *k;
        }
        mid = low
            .offset(
                (high.offset_from(low) as libc::c_long / 2 as libc::c_long) as isize,
            );
        if *high < *mid {
            t___0 = *mid;
            *mid = *high;
            *high = t___0;
        }
        if *high < *low {
            t___1 = *low;
            *low = *high;
            *high = t___1;
        }
        if *low < *mid {
            t___2 = *mid;
            *mid = *low;
            *low = t___2;
        }
        t___3 = *mid;
        *mid = *low.offset(1 as libc::c_int as isize);
        *low.offset(1 as libc::c_int as isize) = t___3;
        ll = low.offset(1 as libc::c_int as isize);
        hh = high;
        loop {
            loop {
                ll = ll.offset(1);
                if !(*ll < *low) {
                    break;
                }
            }
            loop {
                hh = hh.offset(-1);
                if !(*low < *hh) {
                    break;
                }
            }
            if (hh as libc::c_ulong) < ll as libc::c_ulong {
                break;
            }
            t___4 = *ll;
            *ll = *hh;
            *hh = t___4;
        }
        t___5 = *low;
        *low = *hh;
        *hh = t___5;
        if hh as libc::c_ulong <= k as libc::c_ulong {
            low = ll;
        }
        if hh as libc::c_ulong >= k as libc::c_ulong {
            high = hh.offset(-(1 as libc::c_int as isize));
        }
    };
}
pub unsafe extern "C" fn mm_idxopt_init(mut opt: *mut mm_idxopt_t) {
    memset(
        opt as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mm_idxopt_t>() as libc::c_ulong,
    );
    (*opt).k = 15 as libc::c_int as libc::c_short;
    (*opt).w = 10 as libc::c_int as libc::c_short;
    (*opt).flag = 0 as libc::c_int as libc::c_short;
    (*opt).bucket_bits = 14 as libc::c_int as libc::c_short;
    (*opt).mini_batch_size = 50000000 as libc::c_int as int64_t;
    (*opt).batch_size = 4000000000 as libc::c_ulonglong as uint64_t;
}
pub unsafe extern "C" fn mm_mapopt_init(mut opt: *mut mm_mapopt_t) {
    memset(
        opt as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<mm_mapopt_t>() as libc::c_ulong,
    );
    (*opt).seed = 11 as libc::c_int;
    (*opt).mid_occ_frac = 2e-4f32;
    (*opt).min_mid_occ = 10 as libc::c_int;
    (*opt).max_mid_occ = 1000000 as libc::c_int;
    (*opt).sdust_thres = 0 as libc::c_int;
    (*opt).q_occ_frac = 0.01f32;
    (*opt).min_cnt = 3 as libc::c_int;
    (*opt).min_chain_score = 40 as libc::c_int;
    (*opt).bw = 500 as libc::c_int;
    (*opt).bw_long = 20000 as libc::c_int;
    (*opt).max_gap = 5000 as libc::c_int;
    (*opt).max_gap_ref = -(1 as libc::c_int);
    (*opt).max_chain_skip = 25 as libc::c_int;
    (*opt).max_chain_iter = 5000 as libc::c_int;
    (*opt).rmq_inner_dist = 1000 as libc::c_int;
    (*opt).rmq_size_cap = 100000 as libc::c_int;
    (*opt).rmq_rescue_size = 1000 as libc::c_int;
    (*opt).rmq_rescue_ratio = 0.1f32;
    (*opt).chain_gap_scale = 0.8f32;
    (*opt).chain_skip_scale = 0.0f32;
    (*opt).max_max_occ = 4095 as libc::c_int;
    (*opt).occ_dist = 500 as libc::c_int;
    (*opt).mask_level = 0.5f32;
    (*opt).mask_len = 2147483647 as libc::c_int;
    (*opt).pri_ratio = 0.8f32;
    (*opt).best_n = 5 as libc::c_int;
    (*opt).alt_drop = 0.15f32;
    (*opt).a = 2 as libc::c_int;
    (*opt).b = 4 as libc::c_int;
    (*opt).q = 4 as libc::c_int;
    (*opt).e = 2 as libc::c_int;
    (*opt).q2 = 24 as libc::c_int;
    (*opt).e2 = 1 as libc::c_int;
    (*opt).sc_ambi = 1 as libc::c_int;
    (*opt).zdrop = 400 as libc::c_int;
    (*opt).zdrop_inv = 200 as libc::c_int;
    (*opt).end_bonus = -(1 as libc::c_int);
    (*opt).min_dp_max = (*opt).min_chain_score * (*opt).a;
    (*opt).min_ksw_len = 200 as libc::c_int;
    (*opt).anchor_ext_len = 20 as libc::c_int;
    (*opt).anchor_ext_shift = 6 as libc::c_int;
    (*opt).max_clip_ratio = 1.0f32;
    (*opt).mini_batch_size = 500000000 as libc::c_int as int64_t;
    (*opt).max_sw_mat = 100000000 as libc::c_int as int64_t;
    (*opt).cap_kalloc = 1000000000 as libc::c_int as int64_t;
    (*opt).rank_min_len = 500 as libc::c_int;
    (*opt).rank_frac = 0.9f32;
    (*opt).pe_ori = 0 as libc::c_int;
    (*opt).pe_bonus = 33 as libc::c_int;
}
pub unsafe extern "C" fn mm_mapopt_update(
    mut opt: *mut mm_mapopt_t,
    mut mi: *const mm_idx_t,
) {
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_double = 0.;
    let mut tmp___1: libc::c_double = 0.;
    if (*opt).flag & 256 as libc::c_long != 0 {
        (*opt).flag |= 128 as libc::c_long;
    } else if (*opt).flag & 512 as libc::c_long != 0 {
        (*opt).flag |= 128 as libc::c_long;
    }
    if (*opt).mid_occ <= 0 as libc::c_int {
        (*opt).mid_occ = mm_idx_cal_max_occ(mi, (*opt).mid_occ_frac);
        if (*opt).mid_occ < (*opt).min_mid_occ {
            (*opt).mid_occ = (*opt).min_mid_occ;
        }
        if (*opt).max_mid_occ > (*opt).min_mid_occ {
            if (*opt).mid_occ > (*opt).max_mid_occ {
                (*opt).mid_occ = (*opt).max_mid_occ;
            }
        }
    }
    if (*opt).bw_long < (*opt).bw {
        (*opt).bw_long = (*opt).bw;
    }
    if mm_verbose >= 3 as libc::c_int {
        tmp = cputime();
        tmp___0 = realtime();
        tmp___1 = realtime();
        fprintf(
            stderr,
            b"[M::%s::%.3f*%.2f] mid_occ = %d\n\0" as *const u8 as *const libc::c_char,
            b"mm_mapopt_update\0" as *const u8 as *const libc::c_char,
            tmp___1 - mm_realtime0,
            tmp / (tmp___0 - mm_realtime0),
            (*opt).mid_occ,
        );
    }
}
pub unsafe extern "C" fn mm_mapopt_max_intron_len(
    mut opt: *mut mm_mapopt_t,
    mut max_intron_len: libc::c_int,
) {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if (*opt).flag & 128 as libc::c_long != 0 {
        if max_intron_len > 0 as libc::c_int {
            tmp___0 = max_intron_len;
            (*opt).bw_long = tmp___0;
            tmp = tmp___0;
            (*opt).bw = tmp;
            (*opt).max_gap_ref = tmp;
        }
    }
}
pub unsafe extern "C" fn mm_set_opt(
    mut preset: *const libc::c_char,
    mut io: *mut mm_idxopt_t,
    mut mo: *mut mm_mapopt_t,
) -> libc::c_int {
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
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: libc::c_int = 0;
    let mut tmp___20: libc::c_int = 0;
    let mut tmp___21: libc::c_int = 0;
    let mut tmp___22: libc::c_int = 0;
    if preset as libc::c_ulong == 0 as *const libc::c_char as libc::c_ulong {
        mm_idxopt_init(io);
        mm_mapopt_init(mo);
    } else {
        tmp___22 = strcmp(preset, b"map-ont\0" as *const u8 as *const libc::c_char);
        if !(tmp___22 == 0 as libc::c_int) {
            tmp___21 = strcmp(preset, b"ava-ont\0" as *const u8 as *const libc::c_char);
            if tmp___21 == 0 as libc::c_int {
                (*io).flag = 0 as libc::c_int as libc::c_short;
                (*io).k = 15 as libc::c_int as libc::c_short;
                (*io).w = 5 as libc::c_int as libc::c_short;
                (*mo).flag |= 8389635 as libc::c_long;
                (*mo).min_chain_score = 100 as libc::c_int;
                (*mo).pri_ratio = 0.0f32;
                (*mo).max_chain_skip = 25 as libc::c_int;
                tmp = 2000 as libc::c_int;
                (*mo).bw_long = tmp;
                (*mo).bw = tmp;
                (*mo).occ_dist = 0 as libc::c_int;
            } else {
                tmp___19 = strcmp(
                    preset,
                    b"map10k\0" as *const u8 as *const libc::c_char,
                );
                if tmp___19 == 0 as libc::c_int {
                    (*io)
                        .flag = ((*io).flag as libc::c_int | 1 as libc::c_int)
                        as libc::c_short;
                    (*io).k = 19 as libc::c_int as libc::c_short;
                } else {
                    tmp___20 = strcmp(
                        preset,
                        b"map-pb\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___20 == 0 as libc::c_int {
                        (*io)
                            .flag = ((*io).flag as libc::c_int | 1 as libc::c_int)
                            as libc::c_short;
                        (*io).k = 19 as libc::c_int as libc::c_short;
                    } else {
                        tmp___18 = strcmp(
                            preset,
                            b"ava-pb\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___18 == 0 as libc::c_int {
                            (*io)
                                .flag = ((*io).flag as libc::c_int | 1 as libc::c_int)
                                as libc::c_short;
                            (*io).k = 19 as libc::c_int as libc::c_short;
                            (*io).w = 5 as libc::c_int as libc::c_short;
                            (*mo).flag |= 8389635 as libc::c_long;
                            (*mo).min_chain_score = 100 as libc::c_int;
                            (*mo).pri_ratio = 0.0f32;
                            (*mo).max_chain_skip = 25 as libc::c_int;
                            (*mo).bw_long = (*mo).bw;
                            (*mo).occ_dist = 0 as libc::c_int;
                        } else {
                            tmp___16 = strcmp(
                                preset,
                                b"map-hifi\0" as *const u8 as *const libc::c_char,
                            );
                            let mut current_block_173: u64;
                            if tmp___16 == 0 as libc::c_int {
                                current_block_173 = 4222888326580758960;
                            } else {
                                tmp___17 = strcmp(
                                    preset,
                                    b"map-ccs\0" as *const u8 as *const libc::c_char,
                                );
                                if tmp___17 == 0 as libc::c_int {
                                    current_block_173 = 4222888326580758960;
                                } else {
                                    tmp___15 = strncmp(
                                        preset,
                                        b"asm\0" as *const u8 as *const libc::c_char,
                                        3 as libc::c_int as size_t,
                                    );
                                    if tmp___15 == 0 as libc::c_int {
                                        (*io).flag = 0 as libc::c_int as libc::c_short;
                                        (*io).k = 19 as libc::c_int as libc::c_short;
                                        (*io).w = 19 as libc::c_int as libc::c_short;
                                        (*mo).bw = 1000 as libc::c_int;
                                        (*mo).bw_long = 100000 as libc::c_int;
                                        (*mo).max_gap = 10000 as libc::c_int;
                                        (*mo)
                                            .flag = ((*mo).flag as libc::c_longlong
                                            | 2147483648 as libc::c_longlong) as int64_t;
                                        (*mo).min_mid_occ = 50 as libc::c_int;
                                        (*mo).max_mid_occ = 500 as libc::c_int;
                                        (*mo).min_dp_max = 200 as libc::c_int;
                                        (*mo).best_n = 50 as libc::c_int;
                                        tmp___5 = strcmp(
                                            preset,
                                            b"asm5\0" as *const u8 as *const libc::c_char,
                                        );
                                        if tmp___5 == 0 as libc::c_int {
                                            (*mo).a = 1 as libc::c_int;
                                            (*mo).b = 19 as libc::c_int;
                                            (*mo).q = 39 as libc::c_int;
                                            (*mo).q2 = 81 as libc::c_int;
                                            (*mo).e = 3 as libc::c_int;
                                            (*mo).e2 = 1 as libc::c_int;
                                            tmp___0 = 200 as libc::c_int;
                                            (*mo).zdrop_inv = tmp___0;
                                            (*mo).zdrop = tmp___0;
                                        } else {
                                            tmp___4 = strcmp(
                                                preset,
                                                b"asm10\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___4 == 0 as libc::c_int {
                                                (*mo).a = 1 as libc::c_int;
                                                (*mo).b = 9 as libc::c_int;
                                                (*mo).q = 16 as libc::c_int;
                                                (*mo).q2 = 41 as libc::c_int;
                                                (*mo).e = 2 as libc::c_int;
                                                (*mo).e2 = 1 as libc::c_int;
                                                tmp___1 = 200 as libc::c_int;
                                                (*mo).zdrop_inv = tmp___1;
                                                (*mo).zdrop = tmp___1;
                                            } else {
                                                tmp___3 = strcmp(
                                                    preset,
                                                    b"asm20\0" as *const u8 as *const libc::c_char,
                                                );
                                                if tmp___3 == 0 as libc::c_int {
                                                    (*mo).a = 1 as libc::c_int;
                                                    (*mo).b = 4 as libc::c_int;
                                                    (*mo).q = 6 as libc::c_int;
                                                    (*mo).q2 = 26 as libc::c_int;
                                                    (*mo).e = 2 as libc::c_int;
                                                    (*mo).e2 = 1 as libc::c_int;
                                                    tmp___2 = 200 as libc::c_int;
                                                    (*mo).zdrop_inv = tmp___2;
                                                    (*mo).zdrop = tmp___2;
                                                    (*io).w = 10 as libc::c_int as libc::c_short;
                                                } else {
                                                    return -(1 as libc::c_int)
                                                }
                                            }
                                        }
                                    } else {
                                        tmp___13 = strcmp(
                                            preset,
                                            b"short\0" as *const u8 as *const libc::c_char,
                                        );
                                        let mut current_block_169: u64;
                                        if tmp___13 == 0 as libc::c_int {
                                            current_block_169 = 14694235730511432552;
                                        } else {
                                            tmp___14 = strcmp(
                                                preset,
                                                b"sr\0" as *const u8 as *const libc::c_char,
                                            );
                                            if tmp___14 == 0 as libc::c_int {
                                                current_block_169 = 14694235730511432552;
                                            } else {
                                                tmp___11 = strncmp(
                                                    preset,
                                                    b"splice\0" as *const u8 as *const libc::c_char,
                                                    6 as libc::c_int as size_t,
                                                );
                                                if !(tmp___11 == 0 as libc::c_int) {
                                                    tmp___12 = strcmp(
                                                        preset,
                                                        b"cdna\0" as *const u8 as *const libc::c_char,
                                                    );
                                                    if !(tmp___12 == 0 as libc::c_int) {
                                                        return -(1 as libc::c_int);
                                                    }
                                                }
                                                (*io).flag = 0 as libc::c_int as libc::c_short;
                                                (*io).k = 15 as libc::c_int as libc::c_short;
                                                (*io).w = 5 as libc::c_int as libc::c_short;
                                                (*mo).flag |= 263040 as libc::c_long;
                                                (*mo).max_sw_mat = 0 as libc::c_int as int64_t;
                                                (*mo).max_gap = 2000 as libc::c_int;
                                                tmp___9 = 200000 as libc::c_int;
                                                (*mo).bw_long = tmp___9;
                                                tmp___8 = tmp___9;
                                                (*mo).bw = tmp___8;
                                                (*mo).max_gap_ref = tmp___8;
                                                (*mo).a = 1 as libc::c_int;
                                                (*mo).b = 2 as libc::c_int;
                                                (*mo).q = 2 as libc::c_int;
                                                (*mo).e = 1 as libc::c_int;
                                                (*mo).q2 = 32 as libc::c_int;
                                                (*mo).e2 = 0 as libc::c_int;
                                                (*mo).noncan = 9 as libc::c_int;
                                                (*mo).junc_bonus = 9 as libc::c_int;
                                                (*mo).zdrop = 200 as libc::c_int;
                                                (*mo).zdrop_inv = 100 as libc::c_int;
                                                tmp___10 = strcmp(
                                                    preset,
                                                    b"splice:hq\0" as *const u8 as *const libc::c_char,
                                                );
                                                if tmp___10 == 0 as libc::c_int {
                                                    (*mo).junc_bonus = 5 as libc::c_int;
                                                    (*mo).b = 4 as libc::c_int;
                                                    (*mo).q = 6 as libc::c_int;
                                                    (*mo).q2 = 24 as libc::c_int;
                                                }
                                                current_block_169 = 10945915984064580713;
                                            }
                                        }
                                        match current_block_169 {
                                            14694235730511432552 => {
                                                (*io).flag = 0 as libc::c_int as libc::c_short;
                                                (*io).k = 21 as libc::c_int as libc::c_short;
                                                (*io).w = 11 as libc::c_int as libc::c_short;
                                                (*mo).flag |= 4255744 as libc::c_long;
                                                (*mo).pe_ori = 1 as libc::c_int;
                                                (*mo).a = 2 as libc::c_int;
                                                (*mo).b = 8 as libc::c_int;
                                                (*mo).q = 12 as libc::c_int;
                                                (*mo).e = 2 as libc::c_int;
                                                (*mo).q2 = 24 as libc::c_int;
                                                (*mo).e2 = 1 as libc::c_int;
                                                tmp___6 = 100 as libc::c_int;
                                                (*mo).zdrop_inv = tmp___6;
                                                (*mo).zdrop = tmp___6;
                                                (*mo).end_bonus = 10 as libc::c_int;
                                                (*mo).max_frag_len = 800 as libc::c_int;
                                                (*mo).max_gap = 100 as libc::c_int;
                                                tmp___7 = 100 as libc::c_int;
                                                (*mo).bw_long = tmp___7;
                                                (*mo).bw = tmp___7;
                                                (*mo).pri_ratio = 0.5f32;
                                                (*mo).min_cnt = 2 as libc::c_int;
                                                (*mo).min_chain_score = 25 as libc::c_int;
                                                (*mo).min_dp_max = 40 as libc::c_int;
                                                (*mo).best_n = 20 as libc::c_int;
                                                (*mo).mid_occ = 1000 as libc::c_int;
                                                (*mo).max_occ = 5000 as libc::c_int;
                                                (*mo).mini_batch_size = 50000000 as libc::c_int as int64_t;
                                            }
                                            _ => {}
                                        }
                                    }
                                    current_block_173 = 12608488225262500095;
                                }
                            }
                            match current_block_173 {
                                4222888326580758960 => {
                                    (*io).flag = 0 as libc::c_int as libc::c_short;
                                    (*io).k = 19 as libc::c_int as libc::c_short;
                                    (*io).w = 19 as libc::c_int as libc::c_short;
                                    (*mo).max_gap = 10000 as libc::c_int;
                                    (*mo).a = 1 as libc::c_int;
                                    (*mo).b = 4 as libc::c_int;
                                    (*mo).q = 6 as libc::c_int;
                                    (*mo).q2 = 26 as libc::c_int;
                                    (*mo).e = 2 as libc::c_int;
                                    (*mo).e2 = 1 as libc::c_int;
                                    (*mo).occ_dist = 500 as libc::c_int;
                                    (*mo).min_mid_occ = 50 as libc::c_int;
                                    (*mo).max_mid_occ = 500 as libc::c_int;
                                    (*mo).min_dp_max = 200 as libc::c_int;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mm_check_opt(
    mut io: *const mm_idxopt_t,
    mut mo: *const mm_mapopt_t,
) -> libc::c_int {
    if (*mo).bw > (*mo).bw_long {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m with '-rNUM1,NUM2', NUM1 (%d) can't be larger than NUM2 (%d)\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
                (*mo).bw,
                (*mo).bw_long,
            );
        }
        return -(8 as libc::c_int);
    }
    if (*mo).flag as libc::c_longlong & 2147483648 as libc::c_longlong != 0 {
        if (*mo).flag & 4224 as libc::c_long != 0 {
            if mm_verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR]\x1B[1;31m --rmq doesn't work with --sr or --splice\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            return -(7 as libc::c_int);
        }
    }
    if !((*mo).split_prefix).is_null() {
        if (*mo).flag & 16777280 as libc::c_long != 0 {
            if mm_verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR]\x1B[1;31m --cs or --MD doesn't work with --split-prefix\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            return -(6 as libc::c_int);
        }
    }
    's_115: {
        if !((*io).k as libc::c_int <= 0 as libc::c_int) {
            if !((*io).w as libc::c_int <= 0 as libc::c_int) {
                break 's_115;
            }
        }
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m -k and -w must be positive\x1B[0m\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(5 as libc::c_int);
    }
    if (*mo).best_n < 0 as libc::c_int {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m -N must be no less than 0\x1B[0m\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(4 as libc::c_int);
    }
    if (*mo).best_n == 0 as libc::c_int {
        if mm_verbose >= 2 as libc::c_int {
            fprintf(
                stderr,
                b"[WARNING]\x1B[1;31m '-N 0' reduces mapping accuracy. Please use '--secondary=no' instead.\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    's_189: {
        if !((*mo).pri_ratio < 0.0f32) {
            if !((*mo).pri_ratio > 1.0f32) {
                break 's_189;
            }
        }
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m -p must be within 0 and 1 (including 0 and 1)\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(4 as libc::c_int);
    }
    if (*mo).flag & 1048576 as libc::c_long != 0 {
        if (*mo).flag & 2097152 as libc::c_long != 0 {
            if mm_verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR]\x1B[1;31m --for-only and --rev-only can't be applied at the same time\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            return -(3 as libc::c_int);
        }
    }
    's_251: {
        if !((*mo).e <= 0 as libc::c_int) {
            if !((*mo).q <= 0 as libc::c_int) {
                break 's_251;
            }
        }
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m -O and -E must be positive\x1B[0m\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    let mut current_block_60: u64;
    if (*mo).q != (*mo).q2 {
        current_block_60 = 17008118637718713538;
    } else if (*mo).e != (*mo).e2 {
        current_block_60 = 17008118637718713538;
    } else {
        current_block_60 = 13303144130133872306;
    }
    match current_block_60 {
        17008118637718713538 => 's_294: {
            if (*mo).e > (*mo).e2 {
                if (*mo).q + (*mo).e < (*mo).q2 + (*mo).e2 {
                    break 's_294;
                }
            }
            if mm_verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR]\x1B[1;31m dual gap penalties violating E1>E2 and O1+E1<O2+E2\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            return -(2 as libc::c_int);
        }
        _ => {}
    }
    if (*mo).q + (*mo).e + ((*mo).q2 + (*mo).e2) > 127 as libc::c_int {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m scoring system violating ({-O}+{-E})+({-O2}+{-E2}) <= 127\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    if (*mo).zdrop < (*mo).zdrop_inv {
        if mm_verbose >= 1 as libc::c_int {
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m Z-drop should not be less than inversion-Z-drop\x1B[0m\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return -(5 as libc::c_int);
    }
    if (*mo).flag & 16384 as libc::c_long != 0 {
        if (*mo).flag & 8388608 as libc::c_long != 0 {
            if mm_verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR]\x1B[1;31m -X/-P and --secondary=no can't be applied at the same time\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            return -(5 as libc::c_int);
        }
    }
    if (*mo).flag as libc::c_longlong & 4294967296 as libc::c_longlong != 0 {
        's_412: {
            if !((*mo).flag & 8328 as libc::c_long != 0) {
                if !((*io).flag as libc::c_int & 1 as libc::c_int != 0) {
                    break 's_412;
                }
            }
            if mm_verbose >= 1 as libc::c_int {
                fprintf(
                    stderr,
                    b"[ERROR]\x1B[1;31m --qstrand doesn't work with -a, -H, --frag or --splice\x1B[0m\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            return -(5 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn mm_select_sub_multi(
    mut km: *mut libc::c_void,
    mut pri_ratio: libc::c_float,
    mut pri1: libc::c_float,
    mut pri2: libc::c_float,
    mut max_gap_ref: libc::c_int,
    mut min_diff: libc::c_int,
    mut best_n: libc::c_int,
    mut n_segs: libc::c_int,
    mut qlens: *const libc::c_int,
    mut n_: *mut libc::c_int,
    mut r: *mut mm_reg1_t,
) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut n_2nd: libc::c_int = 0;
    let mut max_dist: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut to_keep: libc::c_int = 0;
    let mut p: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut q: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut is_par_both: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut is_chi_both: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    if pri_ratio > 0.0f32 {
        if *n_ > 0 as libc::c_int {
            n = *n_;
            n_2nd = 0 as libc::c_int;
            if n_segs == 2 as libc::c_int {
                tmp = *qlens.offset(0 as libc::c_int as isize)
                    + *qlens.offset(1 as libc::c_int as isize) + max_gap_ref;
            } else {
                tmp = 0 as libc::c_int;
            }
            max_dist = tmp;
            k = 0 as libc::c_int;
            i = k;
            while i < n {
                to_keep = 0 as libc::c_int;
                if (*r.offset(i as isize)).parent == i {
                    to_keep = 1 as libc::c_int;
                } else if (*r.offset(i as isize)).score + min_diff
                        >= (*r.offset((*r.offset(i as isize)).parent as isize)).score
                    {
                    to_keep = 1 as libc::c_int;
                } else {
                    p = r.offset((*r.offset(i as isize)).parent as isize);
                    q = r.offset(i as isize);
                    if (*p).rev() as libc::c_int == (*q).rev() as libc::c_int {
                        if (*p).rid == (*q).rid {
                            if (*q).re - (*p).rs < max_dist {
                                if (*p).re - (*q).rs < max_dist {
                                    if (*q).score as libc::c_float
                                        >= (*p).score as libc::c_float * pri1
                                    {
                                        to_keep = 1 as libc::c_int;
                                    }
                                    current_block = 317151059986244064;
                                } else {
                                    current_block = 1051055268789358430;
                                }
                            } else {
                                current_block = 1051055268789358430;
                            }
                        } else {
                            current_block = 1051055268789358430;
                        }
                    } else {
                        current_block = 1051055268789358430;
                    }
                    match current_block {
                        317151059986244064 => {}
                        _ => {
                            if n_segs == 2 as libc::c_int {
                                if (*p).qs < *qlens.offset(0 as libc::c_int as isize) {
                                    if (*p).qe > *qlens.offset(0 as libc::c_int as isize) {
                                        tmp___0 = 1 as libc::c_int;
                                    } else {
                                        tmp___0 = 0 as libc::c_int;
                                    }
                                } else {
                                    tmp___0 = 0 as libc::c_int;
                                }
                            } else {
                                tmp___0 = 0 as libc::c_int;
                            }
                            is_par_both = tmp___0;
                            if n_segs == 2 as libc::c_int {
                                if (*q).qs < *qlens.offset(0 as libc::c_int as isize) {
                                    if (*q).qe > *qlens.offset(0 as libc::c_int as isize) {
                                        tmp___1 = 1 as libc::c_int;
                                    } else {
                                        tmp___1 = 0 as libc::c_int;
                                    }
                                } else {
                                    tmp___1 = 0 as libc::c_int;
                                }
                            } else {
                                tmp___1 = 0 as libc::c_int;
                            }
                            is_chi_both = tmp___1;
                            let mut current_block_54: u64;
                            if is_chi_both != 0 {
                                current_block_54 = 150383757401579151;
                            } else if is_chi_both == is_par_both {
                                current_block_54 = 150383757401579151;
                            } else {
                                if (*q).score as libc::c_float
                                    >= (*p).score as libc::c_float * pri2
                                {
                                    to_keep = 1 as libc::c_int;
                                }
                                current_block_54 = 15594603006322722090;
                            }
                            match current_block_54 {
                                150383757401579151 => {
                                    if (*q).score as libc::c_float
                                        >= (*p).score as libc::c_float * pri_ratio
                                    {
                                        to_keep = 1 as libc::c_int;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                if to_keep != 0 {
                    if (*r.offset(i as isize)).parent != i {
                        tmp___2 = n_2nd;
                        n_2nd += 1;
                        if tmp___2 >= best_n {
                            to_keep = 0 as libc::c_int;
                        }
                    }
                }
                if to_keep != 0 {
                    tmp___3 = k;
                    k += 1;
                    *r.offset(tmp___3 as isize) = *r.offset(i as isize);
                } else if !((*r.offset(i as isize)).p).is_null() {
                    free((*r.offset(i as isize)).p as *mut libc::c_void);
                }
                i += 1;
            }
            if k != n {
                mm_sync_regs(km, k, r);
            }
            *n_ = k;
        }
    }
}
pub unsafe extern "C" fn mm_set_pe_thru(
    mut qlens: *const libc::c_int,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut *mut mm_reg1_t,
) {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n_pri: [libc::c_int; 2] = [0; 2];
    let mut pri: [libc::c_int; 2] = [0; 2];
    let mut p: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut q: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut tmp: uint32_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    n_pri[1 as libc::c_int as usize] = 0 as libc::c_int;
    n_pri[0 as libc::c_int as usize] = n_pri[1 as libc::c_int as usize];
    pri[1 as libc::c_int as usize] = -(1 as libc::c_int);
    pri[0 as libc::c_int as usize] = pri[1 as libc::c_int as usize];
    s = 0 as libc::c_int;
    while s < 2 as libc::c_int {
        i = 0 as libc::c_int;
        while i < *n_regs.offset(s as isize) {
            if (*(*regs.offset(s as isize)).offset(i as isize)).id
                == (*(*regs.offset(s as isize)).offset(i as isize)).parent
            {
                n_pri[s as usize] += 1;
                pri[s as usize] = i;
            }
            i += 1;
        }
        s += 1;
    }
    if n_pri[0 as libc::c_int as usize] == 1 as libc::c_int {
        if n_pri[1 as libc::c_int as usize] == 1 as libc::c_int {
            p = (*regs.offset(0 as libc::c_int as isize))
                .offset(pri[0 as libc::c_int as usize] as isize);
            q = (*regs.offset(1 as libc::c_int as isize))
                .offset(pri[1 as libc::c_int as usize] as isize);
            if (*p).rid == (*q).rid {
                if (*p).rev() as libc::c_int == (*q).rev() as libc::c_int {
                    tmp___0 = abs((*p).rs - (*q).rs);
                    if tmp___0 < 3 as libc::c_int {
                        tmp___1 = abs((*p).re - (*q).re);
                        if tmp___1 < 3 as libc::c_int {
                            let mut current_block_30: u64;
                            if (*p).qs == 0 as libc::c_int {
                                if *qlens.offset(1 as libc::c_int as isize) - (*q).qe
                                    == 0 as libc::c_int
                                {
                                    tmp = 1 as libc::c_int as uint32_t;
                                    (*q).set_pe_thru(tmp);
                                    (*p).set_pe_thru(tmp);
                                    current_block_30 = 1538046216550696469;
                                } else {
                                    current_block_30 = 16486210603295471691;
                                }
                            } else {
                                current_block_30 = 16486210603295471691;
                            }
                            match current_block_30 {
                                16486210603295471691 => {
                                    if (*q).qs == 0 as libc::c_int {
                                        if *qlens.offset(0 as libc::c_int as isize) - (*p).qe
                                            == 0 as libc::c_int
                                        {
                                            tmp = 1 as libc::c_int as uint32_t;
                                            (*q).set_pe_thru(tmp);
                                            (*p).set_pe_thru(tmp);
                                        }
                                    }
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
pub unsafe extern "C" fn rs_insertsort_pair(
    mut beg: *mut pair_arr_t,
    mut end: *mut pair_arr_t,
) {
    let mut i: *mut pair_arr_t = 0 as *mut pair_arr_t;
    let mut j: *mut pair_arr_t = 0 as *mut pair_arr_t;
    let mut tmp: pair_arr_t = pair_arr_t {
        s: 0,
        rev: 0,
        key: 0,
        r: 0 as *mut mm_reg1_t,
    };
    i = beg.offset(1 as libc::c_int as isize);
    while (i as libc::c_ulong) < end as libc::c_ulong {
        if (*i).key < (*i.offset(-(1 as libc::c_int as isize))).key {
            tmp = *i;
            j = i;
            while j as libc::c_ulong > beg as libc::c_ulong {
                if !(tmp.key < (*j.offset(-(1 as libc::c_int as isize))).key) {
                    break;
                }
                *j = *j.offset(-(1 as libc::c_int as isize));
                j = j.offset(-1);
            }
            *j = tmp;
        }
        i = i.offset(1);
    }
}
pub unsafe extern "C" fn rs_sort_pair(
    mut beg: *mut pair_arr_t,
    mut end: *mut pair_arr_t,
    mut n_bits: libc::c_int,
    mut s: libc::c_int,
) {
    let mut i: *mut pair_arr_t = 0 as *mut pair_arr_t;
    let mut size: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut k: *mut rsbucket_pair_t = 0 as *mut rsbucket_pair_t;
    let mut b: [rsbucket_pair_t; 256] = [rsbucket_pair_t {
        b: 0 as *mut pair_arr_t,
        e: 0 as *mut pair_arr_t,
    }; 256];
    let mut be: *mut rsbucket_pair_t = 0 as *mut rsbucket_pair_t;
    let mut tmp___0: *mut pair_arr_t = 0 as *mut pair_arr_t;
    let mut l: *mut rsbucket_pair_t = 0 as *mut rsbucket_pair_t;
    let mut tmp___1: pair_arr_t = pair_arr_t {
        s: 0,
        rev: 0,
        key: 0,
        r: 0 as *mut mm_reg1_t,
    };
    let mut swap: pair_arr_t = pair_arr_t {
        s: 0,
        rev: 0,
        key: 0,
        r: 0 as *mut mm_reg1_t,
    };
    let mut tmp___2: *mut pair_arr_t = 0 as *mut pair_arr_t;
    let mut tmp___3: *mut pair_arr_t = 0 as *mut pair_arr_t;
    size = (1 as libc::c_int) << n_bits;
    m = size - 1 as libc::c_int;
    be = b.as_mut_ptr().offset(size as isize);
    if !(n_bits <= 8 as libc::c_int) {
        __assert_fail(
            b"n_bits <= RS_MAX_BITS\0" as *const u8 as *const libc::c_char,
            b"pe.c\0" as *const u8 as *const libc::c_char,
            74 as libc::c_uint,
            b"rs_sort_pair\0" as *const u8 as *const libc::c_char,
        );
    }
    k = b.as_mut_ptr();
    while k as libc::c_ulong != be as libc::c_ulong {
        tmp___0 = beg;
        (*k).e = tmp___0;
        (*k).b = tmp___0;
        k = k.offset(1);
    }
    i = beg;
    while i as libc::c_ulong != end as libc::c_ulong {
        b[((*i).key >> s & m as libc::c_ulong) as usize]
            .e = (b[((*i).key >> s & m as libc::c_ulong) as usize].e).offset(1);
        i = i.offset(1);
    }
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k as libc::c_ulong != be as libc::c_ulong {
        (*k)
            .e = ((*k).e)
            .offset(
                ((*k.offset(-(1 as libc::c_int as isize))).e).offset_from(beg)
                    as libc::c_long as isize,
            );
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
    }
    k = b.as_mut_ptr();
    while k as libc::c_ulong != be as libc::c_ulong {
        if (*k).b as libc::c_ulong != (*k).e as libc::c_ulong {
            l = b
                .as_mut_ptr()
                .offset(((*(*k).b).key >> s & m as libc::c_ulong) as isize);
            if l as libc::c_ulong != k as libc::c_ulong {
                tmp___1 = *(*k).b;
                loop {
                    swap = tmp___1;
                    tmp___1 = *(*l).b;
                    tmp___2 = (*l).b;
                    (*l).b = ((*l).b).offset(1);
                    *tmp___2 = swap;
                    l = b
                        .as_mut_ptr()
                        .offset((tmp___1.key >> s & m as libc::c_ulong) as isize);
                    if !(l as libc::c_ulong != k as libc::c_ulong) {
                        break;
                    }
                }
                tmp___3 = (*k).b;
                (*k).b = ((*k).b).offset(1);
                *tmp___3 = tmp___1;
            } else {
                (*k).b = ((*k).b).offset(1);
            }
        } else {
            k = k.offset(1);
        }
    }
    b[0 as libc::c_int as usize].b = beg;
    k = b.as_mut_ptr().offset(1 as libc::c_int as isize);
    while k as libc::c_ulong != be as libc::c_ulong {
        (*k).b = (*k.offset(-(1 as libc::c_int as isize))).e;
        k = k.offset(1);
    }
    if s != 0 {
        if s > n_bits {
            s -= n_bits;
        } else {
            s = 0 as libc::c_int;
        }
        k = b.as_mut_ptr();
        while k as libc::c_ulong != be as libc::c_ulong {
            if ((*k).e).offset_from((*k).b) as libc::c_long > 64 as libc::c_long {
                rs_sort_pair((*k).b, (*k).e, n_bits, s);
            } else if ((*k).e).offset_from((*k).b) as libc::c_long > 1 as libc::c_long {
                rs_insertsort_pair((*k).b, (*k).e);
            }
            k = k.offset(1);
        }
    }
}
pub unsafe extern "C" fn radix_sort_pair(
    mut beg: *mut pair_arr_t,
    mut end: *mut pair_arr_t,
) {
    if end.offset_from(beg) as libc::c_long <= 64 as libc::c_long {
        rs_insertsort_pair(beg, end);
    } else {
        rs_sort_pair(beg, end, 8 as libc::c_int, 56 as libc::c_int);
    };
}
pub unsafe extern "C" fn mm_pair(
    mut km: *mut libc::c_void,
    mut max_gap_ref: libc::c_int,
    mut pe_bonus: libc::c_int,
    mut sub_diff: libc::c_int,
    mut match_sc: libc::c_int,
    mut qlens: *const libc::c_int,
    mut n_regs: *mut libc::c_int,
    mut regs: *mut *mut mm_reg1_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut last: [libc::c_int; 2] = [0; 2];
    let mut dp_thres: libc::c_int = 0;
    let mut segs: libc::c_int = 0;
    let mut max_idx: [libc::c_int; 2] = [0; 2];
    let mut max: int64_t = 0;
    let mut a: *mut pair_arr_t = 0 as *mut pair_arr_t;
    let mut sc: __anonstruct_sc_941540291 = __anonstruct_sc_941540291 {
        n: 0,
        m: 0,
        a: 0 as *mut uint64_t,
    };
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut max___0: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut q: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut r: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut score: int64_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: size_t = 0;
    let mut n_sub: libc::c_int = 0;
    let mut mapq_pe: libc::c_int = 0;
    let mut r___0: [*mut mm_reg1_t; 2] = [0 as *mut mm_reg1_t; 2];
    let mut tmp___3: uint32_t = 0;
    let mut p: *mut mm_reg1_t = 0 as *mut mm_reg1_t;
    let mut mapq_pe_alt: libc::c_int = 0;
    let mut tmp___4: libc::c_float = 0.;
    segs = 0 as libc::c_int;
    sc.n = 0 as libc::c_int as size_t;
    sc.m = 0 as libc::c_int as size_t;
    sc.a = 0 as *mut uint64_t;
    tmp = kmalloc(
        km,
        ((*n_regs.offset(0 as libc::c_int as isize)
            + *n_regs.offset(1 as libc::c_int as isize)) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<pair_arr_t>() as libc::c_ulong),
    );
    a = tmp as *mut pair_arr_t;
    n = 0 as libc::c_int;
    s = n;
    dp_thres = 0 as libc::c_int;
    while s < 2 as libc::c_int {
        max___0 = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < *n_regs.offset(s as isize) {
            (*a.offset(n as isize)).s = s;
            let ref mut fresh42 = (*a.offset(n as isize)).r;
            *fresh42 = (*regs.offset(s as isize)).offset(i as isize);
            (*a.offset(n as isize))
                .rev = (*(*a.offset(n as isize)).r).rev() as libc::c_int;
            (*a.offset(n as isize))
                .key = ((*(*a.offset(n as isize)).r).rid as uint64_t)
                << 32 as libc::c_int
                | ((*(*a.offset(n as isize)).r).rs << 1 as libc::c_int) as libc::c_ulong
                | (s ^ (*a.offset(n as isize)).rev) as libc::c_ulong;
            if max___0 > (*(*(*a.offset(n as isize)).r).p).dp_max {
                max___0 = max___0;
            } else {
                max___0 = (*(*(*a.offset(n as isize)).r).p).dp_max;
            }
            n += 1;
            segs |= (1 as libc::c_int) << s;
            i += 1;
        }
        dp_thres += max___0;
        s += 1;
    }
    if segs != 3 as libc::c_int {
        kfree(km, a as *mut libc::c_void);
        return;
    }
    dp_thres -= pe_bonus;
    if dp_thres < 0 as libc::c_int {
        dp_thres = 0 as libc::c_int;
    }
    radix_sort_pair(a, a.offset(n as isize));
    max = -(1 as libc::c_int) as int64_t;
    max_idx[1 as libc::c_int as usize] = -(1 as libc::c_int);
    max_idx[0 as libc::c_int as usize] = max_idx[1 as libc::c_int as usize];
    last[1 as libc::c_int as usize] = -(1 as libc::c_int);
    last[0 as libc::c_int as usize] = last[1 as libc::c_int as usize];
    if sc.m < n as size_t {
        sc.m = n as size_t;
        sc.m = (sc.m).wrapping_sub(1);
        sc.m |= sc.m >> 1 as libc::c_int;
        sc.m |= sc.m >> 2 as libc::c_int;
        sc.m |= sc.m >> 4 as libc::c_int;
        sc.m |= sc.m >> 8 as libc::c_int;
        sc.m |= sc.m >> 16 as libc::c_int;
        sc.m = (sc.m).wrapping_add(1);
        tmp___0 = krealloc(
            km,
            sc.a as *mut libc::c_void,
            (::std::mem::size_of::<uint64_t>() as libc::c_ulong).wrapping_mul(sc.m),
        );
        sc.a = tmp___0 as *mut uint64_t;
    }
    i = 0 as libc::c_int;
    while i < n {
        if (*a.offset(i as isize)).key & 1 as libc::c_ulong != 0 {
            if !(last[(*a.offset(i as isize)).rev as usize] < 0 as libc::c_int) {
                r = (*a.offset(i as isize)).r;
                q = (*a.offset(last[(*a.offset(i as isize)).rev as usize] as isize)).r;
                if !((*r).rid != (*q).rid) {
                    if !((*r).rs - (*q).re > max_gap_ref) {
                        j = last[(*a.offset(i as isize)).rev as usize];
                        while j >= 0 as libc::c_int {
                            if !((*a.offset(j as isize)).rev
                                != (*a.offset(i as isize)).rev)
                            {
                                if !((*a.offset(j as isize)).s == (*a.offset(i as isize)).s)
                                {
                                    q = (*a.offset(j as isize)).r;
                                    if (*r).rid != (*q).rid {
                                        break;
                                    }
                                    if (*r).rs - (*q).re > max_gap_ref {
                                        break;
                                    }
                                    if !((*(*r).p).dp_max + (*(*q).p).dp_max < dp_thres) {
                                        score = (((*(*r).p).dp_max + (*(*q).p).dp_max) as int64_t)
                                            << 32 as libc::c_int
                                            | ((*r).hash).wrapping_add((*q).hash) as libc::c_long;
                                        if score > max {
                                            max = score;
                                            max_idx[(*a.offset(j as isize)).s as usize] = j;
                                            max_idx[(*a.offset(i as isize)).s as usize] = i;
                                        }
                                        if sc.n == sc.m {
                                            if sc.m != 0 {
                                                sc.m <<= 1 as libc::c_int;
                                            } else {
                                                sc.m = 2 as libc::c_int as size_t;
                                            }
                                            tmp___1 = krealloc(
                                                km,
                                                sc.a as *mut libc::c_void,
                                                (::std::mem::size_of::<uint64_t>() as libc::c_ulong)
                                                    .wrapping_mul(sc.m),
                                            );
                                            sc.a = tmp___1 as *mut uint64_t;
                                        }
                                        tmp___2 = sc.n;
                                        sc.n = (sc.n).wrapping_add(1);
                                        *(sc.a).offset(tmp___2 as isize) = score as uint64_t;
                                    }
                                }
                            }
                            j -= 1;
                        }
                    }
                }
            }
        } else {
            last[(*a.offset(i as isize)).rev as usize] = i;
        }
        i += 1;
    }
    if sc.n > 1 as libc::c_ulong {
        radix_sort_64(sc.a, (sc.a).offset(sc.n as isize));
    }
    if sc.n > 0 as libc::c_ulong {
        if max > 0 as libc::c_long {
            n_sub = 0 as libc::c_int;
            r___0[0 as libc::c_int
                as usize] = (*a.offset(max_idx[0 as libc::c_int as usize] as isize)).r;
            r___0[1 as libc::c_int
                as usize] = (*a.offset(max_idx[1 as libc::c_int as usize] as isize)).r;
            tmp___3 = 1 as libc::c_int as uint32_t;
            (*r___0[1 as libc::c_int as usize]).set_proper_frag(tmp___3);
            (*r___0[0 as libc::c_int as usize]).set_proper_frag(tmp___3);
            s = 0 as libc::c_int;
            while s < 2 as libc::c_int {
                if (*r___0[s as usize]).id != (*r___0[s as usize]).parent {
                    p = (*regs.offset(s as isize))
                        .offset((*r___0[s as usize]).parent as isize);
                    i = 0 as libc::c_int;
                    while i < *n_regs.offset(s as isize) {
                        if (*(*regs.offset(s as isize)).offset(i as isize)).parent
                            == (*p).id
                        {
                            (*(*regs.offset(s as isize)).offset(i as isize))
                                .parent = (*r___0[s as usize]).id;
                        }
                        i += 1;
                    }
                    (*p).set_mapq(0 as libc::c_int as uint32_t);
                }
                if (*r___0[s as usize]).sam_pri() == 0 {
                    i = 0 as libc::c_int;
                    while i < *n_regs.offset(s as isize) {
                        let ref mut fresh43 = *(*regs.offset(s as isize))
                            .offset(i as isize);
                        (*fresh43).set_sam_pri(0 as libc::c_int as uint32_t);
                        i += 1;
                    }
                    (*r___0[s as usize]).set_sam_pri(1 as libc::c_int as uint32_t);
                }
                s += 1;
            }
            if (*r___0[0 as libc::c_int as usize]).mapq() as libc::c_int
                > (*r___0[1 as libc::c_int as usize]).mapq() as libc::c_int
            {
                mapq_pe = (*r___0[0 as libc::c_int as usize]).mapq() as libc::c_int;
            } else {
                mapq_pe = (*r___0[1 as libc::c_int as usize]).mapq() as libc::c_int;
            }
            i = 0 as libc::c_int;
            while i < sc.n as libc::c_int {
                if (*(sc.a).offset(i as isize) >> 32 as libc::c_int)
                    .wrapping_add(sub_diff as uint64_t)
                    >= max as uint64_t >> 32 as libc::c_int
                {
                    n_sub += 1;
                }
                i += 1;
            }
            if sc.n > 1 as libc::c_ulong {
                tmp___4 = logf(n_sub as libc::c_float);
                mapq_pe_alt = (6.02f32
                    * ((max >> 32 as libc::c_int) as uint64_t)
                        .wrapping_sub(
                            *(sc.a)
                                .offset((sc.n).wrapping_sub(2 as libc::c_ulong) as isize)
                                >> 32 as libc::c_int,
                        ) as libc::c_float / match_sc as libc::c_float
                    - 4.343f32 * tmp___4) as libc::c_int;
                if mapq_pe < mapq_pe_alt {
                    mapq_pe = mapq_pe;
                } else {
                    mapq_pe = mapq_pe_alt;
                }
            }
            if (*r___0[0 as libc::c_int as usize]).mapq() < mapq_pe as uint32_t {
                (*r___0[0 as libc::c_int as usize])
                    .set_mapq(
                        (0.2f32
                            * (*r___0[0 as libc::c_int as usize]).mapq() as libc::c_float
                            + 0.8f32 * mapq_pe as libc::c_float + 0.499f32)
                            as libc::c_int as uint32_t,
                    );
            }
            if (*r___0[1 as libc::c_int as usize]).mapq() < mapq_pe as uint32_t {
                (*r___0[1 as libc::c_int as usize])
                    .set_mapq(
                        (0.2f32
                            * (*r___0[1 as libc::c_int as usize]).mapq() as libc::c_float
                            + 0.8f32 * mapq_pe as libc::c_float + 0.499f32)
                            as libc::c_int as uint32_t,
                    );
            }
            if sc.n == 1 as libc::c_ulong {
                if (*r___0[0 as libc::c_int as usize]).mapq() < 2 as libc::c_uint {
                    (*r___0[0 as libc::c_int as usize])
                        .set_mapq(2 as libc::c_int as uint32_t);
                }
                if (*r___0[1 as libc::c_int as usize]).mapq() < 2 as libc::c_uint {
                    (*r___0[1 as libc::c_int as usize])
                        .set_mapq(2 as libc::c_int as uint32_t);
                }
            } else if max as uint64_t >> 32 as libc::c_int
                    > *(sc.a).offset((sc.n).wrapping_sub(2 as libc::c_ulong) as isize)
                        >> 32 as libc::c_int
                {
                if (*r___0[0 as libc::c_int as usize]).mapq() < 1 as libc::c_uint {
                    (*r___0[0 as libc::c_int as usize])
                        .set_mapq(1 as libc::c_int as uint32_t);
                }
                if (*r___0[1 as libc::c_int as usize]).mapq() < 1 as libc::c_uint {
                    (*r___0[1 as libc::c_int as usize])
                        .set_mapq(1 as libc::c_int as uint32_t);
                }
            }
        }
    }
    kfree(km, a as *mut libc::c_void);
    kfree(km, sc.a as *mut libc::c_void);
    mm_set_pe_thru(qlens, n_regs, regs);
}
#[inline]
unsafe extern "C" fn kdq_init_int(mut km: *mut libc::c_void) -> *mut kdq_int_t {
    let mut q: *mut kdq_int_t = 0 as *mut kdq_int_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = kcalloc(
        km,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<kdq_int_t>() as libc::c_ulong,
    );
    q = tmp as *mut kdq_int_t;
    (*q).set_bits(2 as libc::c_int as uint64_t);
    (*q)
        .mask = ((1 as libc::c_ulonglong) << (*q).bits() as libc::c_int)
        .wrapping_sub(1 as libc::c_ulonglong) as uint64_t;
    tmp___0 = kmalloc(
        km,
        (((1 as libc::c_int) << (*q).bits() as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*q).a = tmp___0 as *mut libc::c_int;
    (*q).km = km;
    return q;
}
#[inline]
unsafe extern "C" fn kdq_destroy_int(mut q: *mut kdq_int_t) {
    if q as libc::c_ulong == 0 as *mut kdq_int_t as libc::c_ulong {
        return;
    }
    kfree((*q).km, (*q).a as *mut libc::c_void);
    kfree((*q).km, q as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn kdq_resize_int(
    mut q: *mut kdq_int_t,
    mut new_bits: libc::c_int,
) -> libc::c_int {
    let mut new_size: size_t = 0;
    let mut old_size: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    new_size = ((1 as libc::c_ulonglong) << new_bits) as size_t;
    old_size = ((1 as libc::c_ulonglong) << (*q).bits() as libc::c_int) as size_t;
    if new_size < (*q).count {
        i = 0 as libc::c_int;
        while i < 64 as libc::c_int {
            if (1 as libc::c_ulonglong) << i > (*q).count as libc::c_ulonglong {
                break;
            }
            i += 1;
        }
        new_bits = i;
        new_size = ((1 as libc::c_ulonglong) << new_bits) as size_t;
    }
    if new_bits as uint64_t == (*q).bits() {
        return (*q).bits() as libc::c_int;
    }
    if new_bits as uint64_t > (*q).bits() {
        tmp = krealloc(
            (*q).km,
            (*q).a as *mut libc::c_void,
            ((1 as libc::c_ulonglong) << new_bits)
                .wrapping_mul(
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        as libc::c_ulonglong,
                ) as size_t,
        );
        (*q).a = tmp as *mut libc::c_int;
    }
    if (*q).front().wrapping_add((*q).count) <= old_size {
        if (*q).front().wrapping_add((*q).count) > new_size {
            memmove(
                (*q).a as *mut libc::c_void,
                ((*q).a).offset(new_size as isize) as *const libc::c_void,
                (*q)
                    .front()
                    .wrapping_add((*q).count)
                    .wrapping_sub(new_size)
                    .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
            );
        }
    } else {
        memmove(
            ((*q).a)
                .offset(
                    new_size.wrapping_sub(old_size.wrapping_sub((*q).front())) as isize,
                ) as *mut libc::c_void,
            ((*q).a).offset((*q).front() as isize) as *const libc::c_void,
            old_size
                .wrapping_sub((*q).front())
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*q).set_front(new_size.wrapping_sub(old_size.wrapping_sub((*q).front())));
    }
    (*q).set_bits(new_bits as uint64_t);
    (*q)
        .mask = ((1 as libc::c_ulonglong) << (*q).bits() as libc::c_int)
        .wrapping_sub(1 as libc::c_ulonglong) as uint64_t;
    if (new_bits as uint64_t) < (*q).bits() {
        tmp___0 = krealloc(
            (*q).km,
            (*q).a as *mut libc::c_void,
            ((1 as libc::c_ulonglong) << new_bits)
                .wrapping_mul(
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                        as libc::c_ulonglong,
                ) as size_t,
        );
        (*q).a = tmp___0 as *mut libc::c_int;
    }
    return (*q).bits() as libc::c_int;
}
#[inline]
unsafe extern "C" fn kdq_push_int(mut q: *mut kdq_int_t, mut v: libc::c_int) {
    let mut tmp: uint64_t = 0;
    if (*q).count as libc::c_ulonglong
        == (1 as libc::c_ulonglong) << (*q).bits() as libc::c_int
    {
        kdq_resize_int(q, (*q).bits().wrapping_add(1 as libc::c_ulong) as libc::c_int);
    }
    tmp = (*q).count;
    (*q).count = ((*q).count).wrapping_add(1);
    *((*q).a).offset((tmp.wrapping_add((*q).front()) & (*q).mask) as isize) = v;
}
#[inline]
unsafe extern "C" fn kdq_shift_int(mut q: *mut kdq_int_t) -> *mut libc::c_int {
    let mut d: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp: uint64_t = 0;
    d = 0 as *mut libc::c_int;
    if (*q).count == 0 as libc::c_ulong {
        return 0 as *mut libc::c_int;
    }
    tmp = (*q).front();
    (*q).set_front((*q).front() + 1);
    d = ((*q).a).offset(tmp as isize);
    (*q).set_front((*q).front() & (*q).mask as uint64_t);
    (*q).count = ((*q).count).wrapping_sub(1);
    return d;
}
pub unsafe extern "C" fn sdust_buf_init(mut km: *mut libc::c_void) -> *mut sdust_buf_t {
    let mut buf: *mut sdust_buf_t = 0 as *mut sdust_buf_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = kcalloc(
        km,
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<sdust_buf_t>() as libc::c_ulong,
    );
    buf = tmp as *mut sdust_buf_t;
    (*buf).km = km;
    (*buf).w = kdq_init_int((*buf).km);
    kdq_resize_int((*buf).w, 8 as libc::c_int);
    return buf;
}
pub unsafe extern "C" fn sdust_buf_destroy(mut buf: *mut sdust_buf_t) {
    if buf as libc::c_ulong == 0 as *mut sdust_buf_t as libc::c_ulong {
        return;
    }
    kdq_destroy_int((*buf).w);
    kfree((*buf).km, (*buf).P.a as *mut libc::c_void);
    kfree((*buf).km, (*buf).res.a as *mut libc::c_void);
    kfree((*buf).km, buf as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn shift_window(
    mut t: libc::c_int,
    mut w: *mut kdq_int_t,
    mut T: libc::c_int,
    mut W: libc::c_int,
    mut L: *mut libc::c_int,
    mut rw: *mut libc::c_int,
    mut rv: *mut libc::c_int,
    mut cw: *mut libc::c_int,
    mut cv: *mut libc::c_int,
) {
    let mut s: libc::c_int = 0;
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    if (*w).count as libc::c_int >= W - 3 as libc::c_int + 1 as libc::c_int {
        tmp = kdq_shift_int(w);
        s = *tmp;
        let ref mut fresh44 = *cw.offset(s as isize);
        *fresh44 -= 1;
        *rw -= *cw.offset(s as isize);
        if *L > (*w).count as libc::c_int {
            *L -= 1;
            let ref mut fresh45 = *cv.offset(s as isize);
            *fresh45 -= 1;
            *rv -= *cv.offset(s as isize);
        }
    }
    kdq_push_int(w, t);
    *L += 1;
    tmp___0 = *cw.offset(t as isize);
    let ref mut fresh46 = *cw.offset(t as isize);
    *fresh46 += 1;
    *rw += tmp___0;
    tmp___1 = *cv.offset(t as isize);
    let ref mut fresh47 = *cv.offset(t as isize);
    *fresh47 += 1;
    *rv += tmp___1;
    if *cv.offset(t as isize) * 10 as libc::c_int > T << 1 as libc::c_int {
        loop {
            s = *((*w).a)
                .offset(
                    ((*w).front().wrapping_add(((*w).count).wrapping_sub(*L as uint64_t))
                        & (*w).mask) as isize,
                );
            let ref mut fresh48 = *cv.offset(s as isize);
            *fresh48 -= 1;
            *rv -= *cv.offset(s as isize);
            *L -= 1;
            if !(s != t) {
                break;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn save_masked_regions(
    mut km: *mut libc::c_void,
    mut res: *mut uint64_v,
    mut P: *mut perf_intv_v,
    mut start: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut saved: libc::c_int = 0;
    let mut p: *mut perf_intv_t = 0 as *mut perf_intv_t;
    let mut s: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: size_t = 0;
    saved = 0 as libc::c_int;
    if (*P).n == 0 as libc::c_ulong {
        return
    } else {
        if (*((*P).a).offset(((*P).n).wrapping_sub(1 as libc::c_ulong) as isize)).start
            >= start
        {
            return;
        }
    }
    p = ((*P).a).offset(((*P).n).wrapping_sub(1 as libc::c_ulong) as isize);
    if (*res).n != 0 {
        s = (*((*res).a).offset(((*res).n).wrapping_sub(1 as libc::c_ulong) as isize)
            >> 32 as libc::c_int) as libc::c_int;
        f = *((*res).a).offset(((*res).n).wrapping_sub(1 as libc::c_ulong) as isize)
            as uint32_t as libc::c_int;
        if (*p).start <= f {
            saved = 1 as libc::c_int;
            if f > (*p).finish {
                tmp = f;
            } else {
                tmp = (*p).finish;
            }
            *((*res).a)
                .offset(
                    ((*res).n).wrapping_sub(1 as libc::c_ulong) as isize,
                ) = (s as uint64_t) << 32 as libc::c_int | tmp as libc::c_ulong;
        }
    }
    if saved == 0 {
        if (*res).n == (*res).m {
            if (*res).m != 0 {
                (*res).m <<= 1 as libc::c_int;
            } else {
                (*res).m = 2 as libc::c_int as size_t;
            }
            tmp___0 = krealloc(
                km,
                (*res).a as *mut libc::c_void,
                (::std::mem::size_of::<uint64_t>() as libc::c_ulong)
                    .wrapping_mul((*res).m),
            );
            (*res).a = tmp___0 as *mut uint64_t;
        }
        tmp___1 = (*res).n;
        (*res).n = ((*res).n).wrapping_add(1);
        *((*res).a)
            .offset(
                tmp___1 as isize,
            ) = ((*p).start as uint64_t) << 32 as libc::c_int
            | (*p).finish as libc::c_ulong;
    }
    i = ((*P).n).wrapping_sub(1 as libc::c_ulong) as libc::c_int;
    while i >= 0 as libc::c_int {
        if !((*((*P).a).offset(i as isize)).start < start) {
            break;
        }
        i -= 1;
    }
    (*P).n = (i + 1 as libc::c_int) as size_t;
}
unsafe extern "C" fn find_perfect(
    mut km: *mut libc::c_void,
    mut P: *mut perf_intv_v,
    mut w: *const kdq_int_t,
    mut T: libc::c_int,
    mut start: libc::c_int,
    mut L: libc::c_int,
    mut rv: libc::c_int,
    mut cv: *const libc::c_int,
) {
    let mut c: [libc::c_int; 64] = [0; 64];
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut max_r: libc::c_int = 0;
    let mut max_l: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut new_r: libc::c_int = 0;
    let mut new_l: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut p: *mut perf_intv_t = 0 as *mut perf_intv_t;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    r = rv;
    max_r = 0 as libc::c_int;
    max_l = 0 as libc::c_int;
    memcpy(
        c.as_mut_ptr() as *mut libc::c_void,
        cv as *const libc::c_void,
        (((1 as libc::c_int) << ((3 as libc::c_int) << 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    i = ((*w).count as libc::c_long - L as libc::c_long - 1 as libc::c_long)
        as libc::c_int;
    while i >= 0 as libc::c_int {
        t = *((*w).a)
            .offset(((*w).front().wrapping_add(i as uint64_t) & (*w).mask) as isize);
        tmp = c[t as usize];
        c[t as usize] += 1;
        r += tmp;
        new_r = r;
        new_l = ((*w).count).wrapping_sub(i as uint64_t).wrapping_sub(1 as libc::c_ulong)
            as libc::c_int;
        if new_r * 10 as libc::c_int > T * new_l {
            j = 0 as libc::c_int;
            while j < (*P).n as libc::c_int {
                if !((*((*P).a).offset(j as isize)).start >= i + start) {
                    break;
                }
                p = ((*P).a).offset(j as isize);
                if max_r == 0 as libc::c_int {
                    max_r = (*p).r;
                    max_l = (*p).l;
                } else if (*p).r * max_l > max_r * (*p).l {
                    max_r = (*p).r;
                    max_l = (*p).l;
                }
                j += 1;
            }
            let mut current_block_45: u64;
            if max_r == 0 as libc::c_int {
                current_block_45 = 13181179965151232395;
            } else if new_r * max_l >= max_r * new_l {
                current_block_45 = 13181179965151232395;
            } else {
                current_block_45 = 6717214610478484138;
            }
            match current_block_45 {
                13181179965151232395 => {
                    max_r = new_r;
                    max_l = new_l;
                    if (*P).n == (*P).m {
                        if (*P).m < ((*P).n).wrapping_add(1 as libc::c_ulong) {
                            (*P).m = ((*P).n).wrapping_add(1 as libc::c_ulong);
                            (*P).m = ((*P).m).wrapping_sub(1);
                            (*P).m |= (*P).m >> 1 as libc::c_int;
                            (*P).m |= (*P).m >> 2 as libc::c_int;
                            (*P).m |= (*P).m >> 4 as libc::c_int;
                            (*P).m |= (*P).m >> 8 as libc::c_int;
                            (*P).m |= (*P).m >> 16 as libc::c_int;
                            (*P).m = ((*P).m).wrapping_add(1);
                            tmp___0 = krealloc(
                                km,
                                (*P).a as *mut libc::c_void,
                                (::std::mem::size_of::<perf_intv_t>() as libc::c_ulong)
                                    .wrapping_mul((*P).m),
                            );
                            (*P).a = tmp___0 as *mut perf_intv_t;
                        }
                    }
                    memmove(
                        ((*P).a).offset((j + 1 as libc::c_int) as isize)
                            as *mut libc::c_void,
                        ((*P).a).offset(j as isize) as *const libc::c_void,
                        ((*P).n)
                            .wrapping_sub(j as size_t)
                            .wrapping_mul(
                                ::std::mem::size_of::<perf_intv_t>() as libc::c_ulong,
                            ),
                    );
                    (*P).n = ((*P).n).wrapping_add(1);
                    (*((*P).a).offset(j as isize)).start = i + start;
                    (*((*P).a).offset(j as isize))
                        .finish = ((*w).count)
                        .wrapping_add(2 as libc::c_ulong)
                        .wrapping_add(start as uint64_t) as libc::c_int;
                    (*((*P).a).offset(j as isize)).r = new_r;
                    (*((*P).a).offset(j as isize)).l = new_l;
                }
                _ => {}
            }
        }
        i -= 1;
    }
}
pub unsafe extern "C" fn sdust_core(
    mut seq: *const uint8_t,
    mut l_seq: libc::c_int,
    mut T: libc::c_int,
    mut W: libc::c_int,
    mut n: *mut libc::c_int,
    mut buf: *mut sdust_buf_t,
) -> *const uint64_t {
    let mut rv: libc::c_int = 0;
    let mut rw: libc::c_int = 0;
    let mut L: libc::c_int = 0;
    let mut cv: [libc::c_int; 64] = [0; 64];
    let mut cw: [libc::c_int; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: uint64_t = 0;
    let mut tmp___1: size_t = 0;
    let mut b: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    rv = 0 as libc::c_int;
    rw = 0 as libc::c_int;
    L = 0 as libc::c_int;
    tmp = 0 as libc::c_int as size_t;
    (*buf).res.n = tmp;
    (*buf).P.n = tmp;
    tmp___0 = 0 as libc::c_int as uint64_t;
    (*(*buf).w).count = tmp___0;
    (*(*buf).w).set_front(tmp___0);
    memset(
        cv.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (((1 as libc::c_int) << ((3 as libc::c_int) << 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    memset(
        cw.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (((1 as libc::c_int) << ((3 as libc::c_int) << 1 as libc::c_int))
            as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    if l_seq < 0 as libc::c_int {
        tmp___1 = strlen(seq as *const libc::c_char);
        l_seq = tmp___1 as libc::c_int;
    }
    t = 0 as libc::c_uint;
    l = t as libc::c_int;
    i = l;
    while i <= l_seq {
        if i < l_seq {
            tmp___2 = seq_nt4_table[*seq.offset(i as isize) as usize] as libc::c_int;
        } else {
            tmp___2 = 4 as libc::c_int;
        }
        b = tmp___2;
        if b < 4 as libc::c_int {
            l += 1;
            t = (t << 2 as libc::c_int | b as libc::c_uint)
                & (((1 as libc::c_int) << ((3 as libc::c_int) << 1 as libc::c_int))
                    - 1 as libc::c_int) as libc::c_uint;
            if l >= 3 as libc::c_int {
                if l - W > 0 as libc::c_int {
                    tmp___3 = l - W;
                } else {
                    tmp___3 = 0 as libc::c_int;
                }
                start = tmp___3 + (i + 1 as libc::c_int - l);
                save_masked_regions((*buf).km, &mut (*buf).res, &mut (*buf).P, start);
                shift_window(
                    t as libc::c_int,
                    (*buf).w,
                    T,
                    W,
                    &mut L,
                    &mut rw,
                    &mut rv,
                    cw.as_mut_ptr(),
                    cv.as_mut_ptr(),
                );
                if rw * 10 as libc::c_int > L * T {
                    find_perfect(
                        (*buf).km,
                        &mut (*buf).P,
                        (*buf).w as *const kdq_int_t,
                        T,
                        start,
                        L,
                        rv,
                        cv.as_mut_ptr() as *const libc::c_int,
                    );
                }
            }
        } else {
            if l - W + 1 as libc::c_int > 0 as libc::c_int {
                tmp___4 = l - W + 1 as libc::c_int;
            } else {
                tmp___4 = 0 as libc::c_int;
            }
            start = tmp___4 + (i + 1 as libc::c_int - l);
            while (*buf).P.n != 0 {
                tmp___5 = start;
                start += 1;
                save_masked_regions((*buf).km, &mut (*buf).res, &mut (*buf).P, tmp___5);
            }
            t = 0 as libc::c_uint;
            l = t as libc::c_int;
        }
        i += 1;
    }
    *n = (*buf).res.n as libc::c_int;
    return (*buf).res.a as *const uint64_t;
}
pub unsafe extern "C" fn sdust(
    mut km: *mut libc::c_void,
    mut seq: *const uint8_t,
    mut l_seq: libc::c_int,
    mut T: libc::c_int,
    mut W: libc::c_int,
    mut n: *mut libc::c_int,
) -> *mut uint64_t {
    let mut ret: *mut uint64_t = 0 as *mut uint64_t;
    let mut buf: *mut sdust_buf_t = 0 as *mut sdust_buf_t;
    let mut tmp: *const uint64_t = 0 as *const uint64_t;
    buf = sdust_buf_init(km);
    tmp = sdust_core(seq, l_seq, T, W, n, buf);
    ret = tmp as *mut uint64_t;
    (*buf).res.a = 0 as *mut uint64_t;
    sdust_buf_destroy(buf);
    return ret;
}
pub unsafe extern "C" fn mm_seed_mz_flt(
    mut km: *mut libc::c_void,
    mut mv: *mut mm128_v,
    mut q_occ_max: int32_t,
    mut q_occ_frac: libc::c_float,
) {
    let mut a: *mut mm128_t = 0 as *mut mm128_t;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut st: size_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cnt: int32_t = 0;
    let mut tmp___0: size_t = 0;
    if (*mv).n <= q_occ_max as size_t {
        return
    } else {
        if q_occ_frac <= 0.0f32 {
            return
        } else {
            if q_occ_max <= 0 as libc::c_int {
                return;
            }
        }
    }
    tmp = kmalloc(
        km,
        ((*mv).n).wrapping_mul(::std::mem::size_of::<mm128_t>() as libc::c_ulong),
    );
    a = tmp as *mut mm128_t;
    i = 0 as libc::c_int as size_t;
    while i < (*mv).n {
        (*a.offset(i as isize)).x = (*((*mv).a).offset(i as isize)).x;
        (*a.offset(i as isize)).y = i;
        i = i.wrapping_add(1);
    }
    radix_sort_128x(a, a.offset((*mv).n as isize));
    st = 0 as libc::c_int as size_t;
    i = 1 as libc::c_int as size_t;
    while i <= (*mv).n {
        let mut current_block_30: u64;
        if i == (*mv).n {
            current_block_30 = 13580986381134744345;
        } else if (*a.offset(i as isize)).x != (*a.offset(st as isize)).x {
            current_block_30 = 13580986381134744345;
        } else {
            current_block_30 = 4775909272756257391;
        }
        match current_block_30 {
            13580986381134744345 => {
                cnt = i.wrapping_sub(st) as int32_t;
                if cnt > q_occ_max {
                    if cnt as libc::c_float > (*mv).n as libc::c_float * q_occ_frac {
                        j = st;
                        while j < i {
                            (*((*mv).a).offset((*a.offset(j as isize)).y as isize))
                                .x = 0 as libc::c_int as uint64_t;
                            j = j.wrapping_add(1);
                        }
                    }
                }
                st = i;
            }
            _ => {}
        }
        i = i.wrapping_add(1);
    }
    kfree(km, a as *mut libc::c_void);
    j = 0 as libc::c_int as size_t;
    i = j;
    while i < (*mv).n {
        if (*((*mv).a).offset(i as isize)).x != 0 as libc::c_ulong {
            tmp___0 = j;
            j = j.wrapping_add(1);
            *((*mv).a).offset(tmp___0 as isize) = *((*mv).a).offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    (*mv).n = j;
}
pub unsafe extern "C" fn mm_seed_collect_all(
    mut km: *mut libc::c_void,
    mut mi: *const mm_idx_t,
    mut mv: *const mm128_v,
    mut n_m_: *mut int32_t,
) -> *mut mm_seed_t {
    let mut m: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut i: size_t = 0;
    let mut k: int32_t = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cr: *const uint64_t = 0 as *const uint64_t;
    let mut q: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut p: *mut mm128_t = 0 as *mut mm128_t;
    let mut q_pos: uint32_t = 0;
    let mut q_span: uint32_t = 0;
    let mut t: libc::c_int = 0;
    let mut tmp___0: int32_t = 0;
    let mut tmp___1: uint32_t = 0;
    tmp = kmalloc(
        km,
        ((*mv).n).wrapping_mul(::std::mem::size_of::<mm_seed_t>() as libc::c_ulong),
    );
    m = tmp as *mut mm_seed_t;
    k = 0 as libc::c_int;
    i = k as size_t;
    while i < (*mv).n {
        p = ((*mv).a).offset(i as isize);
        q_pos = (*p).y as uint32_t;
        q_span = ((*p).x & 255 as libc::c_ulong) as uint32_t;
        cr = mm_idx_get(mi, (*p).x >> 8 as libc::c_int, &mut t);
        if !(t == 0 as libc::c_int) {
            tmp___0 = k;
            k += 1;
            q = m.offset(tmp___0 as isize);
            (*q).q_pos = q_pos;
            (*q).set_q_span(q_span);
            (*q).cr = cr;
            (*q).n = t as uint32_t;
            (*q).set_seg_id(((*p).y >> 32 as libc::c_int) as uint32_t);
            tmp___1 = 0 as libc::c_int as uint32_t;
            (*q).set_flt(tmp___1);
            (*q).set_is_tandem(tmp___1);
            if i > 0 as libc::c_ulong {
                if (*p).x >> 8 as libc::c_int
                    == (*((*mv).a).offset(i.wrapping_sub(1 as libc::c_ulong) as isize)).x
                        >> 8 as libc::c_int
                {
                    (*q).set_is_tandem(1 as libc::c_int as uint32_t);
                }
            }
            if i < ((*mv).n).wrapping_sub(1 as libc::c_ulong) {
                if (*p).x >> 8 as libc::c_int
                    == (*((*mv).a).offset(i.wrapping_add(1 as libc::c_ulong) as isize)).x
                        >> 8 as libc::c_int
                {
                    (*q).set_is_tandem(1 as libc::c_int as uint32_t);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    *n_m_ = k;
    return m;
}
pub unsafe extern "C" fn mm_seed_select(
    mut n: int32_t,
    mut a: *mut mm_seed_t,
    mut len: libc::c_int,
    mut max_occ: libc::c_int,
    mut max_max_occ: libc::c_int,
    mut dist: libc::c_int,
) {
    let mut i: int32_t = 0;
    let mut last0: int32_t = 0;
    let mut m: int32_t = 0;
    let mut b: [uint64_t; 128] = [0; 128];
    let mut ps: int32_t = 0;
    let mut tmp: uint32_t = 0;
    let mut pe: int32_t = 0;
    let mut tmp___0: uint32_t = 0;
    let mut j: int32_t = 0;
    let mut k: int32_t = 0;
    let mut st: int32_t = 0;
    let mut en: int32_t = 0;
    let mut max_high_occ: int32_t = 0;
    if n == 0 as libc::c_int {
        return
    } else {
        if n == 1 as libc::c_int {
            return;
        }
    }
    m = 0 as libc::c_int;
    i = m;
    while i < n {
        if (*a.offset(i as isize)).n > max_occ as uint32_t {
            m += 1;
        }
        i += 1;
    }
    if m == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    last0 = -(1 as libc::c_int);
    while i <= n {
        let mut current_block_72: u64;
        if i == n {
            current_block_72 = 739414145545070446;
        } else if (*a.offset(i as isize)).n <= max_occ as uint32_t {
            current_block_72 = 739414145545070446;
        } else {
            current_block_72 = 10067844863897285902;
        }
        match current_block_72 {
            739414145545070446 => {
                if i - last0 > 1 as libc::c_int {
                    if last0 < 0 as libc::c_int {
                        tmp = 0 as libc::c_int as uint32_t;
                    } else {
                        tmp = (*a.offset(last0 as isize)).q_pos >> 1 as libc::c_int;
                    }
                    ps = tmp as int32_t;
                    if i == n {
                        tmp___0 = len as uint32_t;
                    } else {
                        tmp___0 = (*a.offset(i as isize)).q_pos >> 1 as libc::c_int;
                    }
                    pe = tmp___0 as int32_t;
                    st = last0 + 1 as libc::c_int;
                    en = i;
                    max_high_occ = ((pe - ps) as libc::c_double / dist as libc::c_double
                        + 0.499f64) as int32_t;
                    if max_high_occ > 0 as libc::c_int {
                        if max_high_occ > 128 as libc::c_int {
                            max_high_occ = 128 as libc::c_int;
                        }
                        j = st;
                        k = 0 as libc::c_int;
                        while j < en {
                            if !(k < max_high_occ) {
                                break;
                            }
                            b[k
                                as usize] = ((*a.offset(j as isize)).n as uint64_t)
                                << 32 as libc::c_int | j as libc::c_ulong;
                            j += 1;
                            k += 1;
                        }
                        ks_heapmake_uint64_t(k as size_t, b.as_mut_ptr());
                        while j < en {
                            if (*a.offset(j as isize)).n
                                < (b[0 as libc::c_int as usize] >> 32 as libc::c_int)
                                    as int32_t as uint32_t
                            {
                                b[0 as libc::c_int
                                    as usize] = ((*a.offset(j as isize)).n as uint64_t)
                                    << 32 as libc::c_int | j as libc::c_ulong;
                                ks_heapdown_uint64_t(
                                    0 as libc::c_int as size_t,
                                    k as size_t,
                                    b.as_mut_ptr(),
                                );
                            }
                            j += 1;
                        }
                        j = 0 as libc::c_int;
                        while j < k {
                            let ref mut fresh49 = *a
                                .offset(b[j as usize] as uint32_t as isize);
                            (*fresh49).set_flt(1 as libc::c_int as uint32_t);
                            j += 1;
                        }
                    }
                    j = st;
                    while j < en {
                        let ref mut fresh50 = *a.offset(j as isize);
                        (*fresh50)
                            .set_flt((*fresh50).flt() ^ 1 as libc::c_uint as uint32_t);
                        j += 1;
                    }
                    j = st;
                    while j < en {
                        if (*a.offset(j as isize)).n > max_max_occ as uint32_t {
                            let ref mut fresh51 = *a.offset(j as isize);
                            (*fresh51).set_flt(1 as libc::c_int as uint32_t);
                        }
                        j += 1;
                    }
                }
                last0 = i;
            }
            _ => {}
        }
        i += 1;
    }
}
pub unsafe extern "C" fn mm_collect_matches(
    mut km: *mut libc::c_void,
    mut _n_m: *mut libc::c_int,
    mut qlen: libc::c_int,
    mut max_occ: libc::c_int,
    mut max_max_occ: libc::c_int,
    mut dist: libc::c_int,
    mut mi: *const mm_idx_t,
    mut mv: *const mm128_v,
    mut n_a: *mut int64_t,
    mut rep_len: *mut libc::c_int,
    mut n_mini_pos: *mut libc::c_int,
    mut mini_pos: *mut *mut uint64_t,
) -> *mut mm_seed_t {
    let mut rep_st: libc::c_int = 0;
    let mut rep_en: libc::c_int = 0;
    let mut n_m: libc::c_int = 0;
    let mut n_m0: libc::c_int = 0;
    let mut i: size_t = 0;
    let mut m: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut q: *mut mm_seed_t = 0 as *mut mm_seed_t;
    let mut en: libc::c_int = 0;
    let mut st: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    rep_st = 0 as libc::c_int;
    rep_en = 0 as libc::c_int;
    *n_mini_pos = 0 as libc::c_int;
    tmp = kmalloc(
        km,
        ((*mv).n).wrapping_mul(::std::mem::size_of::<uint64_t>() as libc::c_ulong),
    );
    *mini_pos = tmp as *mut uint64_t;
    m = mm_seed_collect_all(km, mi, mv, &mut n_m0);
    let mut current_block_14: u64;
    if dist > 0 as libc::c_int {
        if max_max_occ > max_occ {
            mm_seed_select(n_m0, m, qlen, max_occ, max_max_occ, dist);
            current_block_14 = 7175849428784450219;
        } else {
            current_block_14 = 4173974453063809438;
        }
    } else {
        current_block_14 = 4173974453063809438;
    }
    match current_block_14 {
        4173974453063809438 => {
            i = 0 as libc::c_int as size_t;
            while i < n_m0 as size_t {
                if (*m.offset(i as isize)).n > max_occ as uint32_t {
                    let ref mut fresh52 = *m.offset(i as isize);
                    (*fresh52).set_flt(1 as libc::c_int as uint32_t);
                }
                i = i.wrapping_add(1);
            }
        }
        _ => {}
    }
    i = 0 as libc::c_int as size_t;
    n_m = 0 as libc::c_int;
    *rep_len = 0 as libc::c_int;
    *n_a = 0 as libc::c_int as int64_t;
    while i < n_m0 as size_t {
        q = m.offset(i as isize);
        if (*q).flt() != 0 {
            en = ((*q).q_pos >> 1 as libc::c_int).wrapping_add(1 as libc::c_uint)
                as libc::c_int;
            st = (en as uint32_t).wrapping_sub((*q).q_span()) as libc::c_int;
            if st > rep_en {
                *rep_len += rep_en - rep_st;
                rep_st = st;
                rep_en = en;
            } else {
                rep_en = en;
            }
        } else {
            *n_a += (*q).n as int64_t;
            tmp___0 = *n_mini_pos;
            *n_mini_pos += 1;
            *(*mini_pos)
                .offset(
                    tmp___0 as isize,
                ) = ((*q).q_span() as uint64_t) << 32 as libc::c_int
                | ((*q).q_pos >> 1 as libc::c_int) as libc::c_ulong;
            tmp___1 = n_m;
            n_m += 1;
            *m.offset(tmp___1 as isize) = *q;
        }
        i = i.wrapping_add(1);
    }
    *rep_len += rep_en - rep_st;
    *_n_m = n_m;
    return m;
}
pub static mut seq_nt4_table: [libc::c_uchar; 256] = [
    0 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    2 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    3 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
];
#[inline]
unsafe extern "C" fn hash64___0(mut key: uint64_t, mut mask: uint64_t) -> uint64_t {
    key = (!key).wrapping_add(key << 21 as libc::c_int) & mask;
    key ^= key >> 24 as libc::c_int;
    key = key.wrapping_add(key << 3 as libc::c_int).wrapping_add(key << 8 as libc::c_int)
        & mask;
    key ^= key >> 14 as libc::c_int;
    key = key.wrapping_add(key << 2 as libc::c_int).wrapping_add(key << 4 as libc::c_int)
        & mask;
    key ^= key >> 28 as libc::c_int;
    key = key.wrapping_add(key << 31 as libc::c_int) & mask;
    return key;
}
#[inline]
unsafe extern "C" fn tq_push(mut q: *mut tiny_queue_t, mut x: libc::c_int) {
    let mut tmp: libc::c_int = 0;
    tmp = (*q).count;
    (*q).count += 1;
    (*q).a[(tmp + (*q).front & 31 as libc::c_int) as usize] = x;
}
#[inline]
unsafe extern "C" fn tq_shift(mut q: *mut tiny_queue_t) -> libc::c_int {
    let mut x: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    if (*q).count == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    tmp = (*q).front;
    (*q).front += 1;
    x = (*q).a[tmp as usize];
    (*q).front &= 31 as libc::c_int;
    (*q).count -= 1;
    return x;
}
pub unsafe extern "C" fn mm_sketch(
    mut km: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: libc::c_int,
    mut w: libc::c_int,
    mut k: libc::c_int,
    mut rid: uint32_t,
    mut is_hpc: libc::c_int,
    mut p: *mut mm128_v,
) {
    let mut shift1: uint64_t = 0;
    let mut mask: uint64_t = 0;
    let mut kmer: [uint64_t; 2] = [0; 2];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut buf_pos: libc::c_int = 0;
    let mut min_pos: libc::c_int = 0;
    let mut kmer_span: libc::c_int = 0;
    let mut buf: [mm128_t; 256] = [mm128_t { x: 0, y: 0 }; 256];
    let mut min: mm128_t = mm128_t { x: 0, y: 0 };
    let mut tq: tiny_queue_t = tiny_queue_t {
        front: 0,
        count: 0,
        a: [0; 32],
    };
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut c: libc::c_int = 0;
    let mut info: mm128_t = mm128_t { x: 0, y: 0 };
    let mut z: libc::c_int = 0;
    let mut skip_len: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: uint64_t = 0;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___6: size_t = 0;
    let mut tmp___7: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___8: size_t = 0;
    let mut tmp___9: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___10: size_t = 0;
    let mut tmp___11: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___12: size_t = 0;
    let mut tmp___13: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___14: size_t = 0;
    let mut tmp___15: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___16: size_t = 0;
    shift1 = (2 as libc::c_int * (k - 1 as libc::c_int)) as uint64_t;
    mask = ((1 as libc::c_ulonglong) << 2 as libc::c_int * k)
        .wrapping_sub(1 as libc::c_ulonglong) as uint64_t;
    kmer[0 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    kmer[1 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    kmer_span = 0 as libc::c_int;
    min.x = 18446744073709551615 as libc::c_ulonglong as uint64_t;
    min.y = 18446744073709551615 as libc::c_ulonglong as uint64_t;
    if len > 0 as libc::c_int {
        if w > 0 as libc::c_int {
            if w < 256 as libc::c_int {
                if k > 0 as libc::c_int {
                    if !(k <= 28 as libc::c_int) {
                        __assert_fail(
                            b"len > 0 && (w > 0 && w < 256) && (k > 0 && k <= 28)\0"
                                as *const u8 as *const libc::c_char,
                            b"sketch.c\0" as *const u8 as *const libc::c_char,
                            84 as libc::c_uint,
                            b"mm_sketch\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    __assert_fail(
                        b"len > 0 && (w > 0 && w < 256) && (k > 0 && k <= 28)\0"
                            as *const u8 as *const libc::c_char,
                        b"sketch.c\0" as *const u8 as *const libc::c_char,
                        84 as libc::c_uint,
                        b"mm_sketch\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                __assert_fail(
                    b"len > 0 && (w > 0 && w < 256) && (k > 0 && k <= 28)\0" as *const u8
                        as *const libc::c_char,
                    b"sketch.c\0" as *const u8 as *const libc::c_char,
                    84 as libc::c_uint,
                    b"mm_sketch\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            __assert_fail(
                b"len > 0 && (w > 0 && w < 256) && (k > 0 && k <= 28)\0" as *const u8
                    as *const libc::c_char,
                b"sketch.c\0" as *const u8 as *const libc::c_char,
                84 as libc::c_uint,
                b"mm_sketch\0" as *const u8 as *const libc::c_char,
            );
        }
    } else {
        __assert_fail(
            b"len > 0 && (w > 0 && w < 256) && (k > 0 && k <= 28)\0" as *const u8
                as *const libc::c_char,
            b"sketch.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_uint,
            b"mm_sketch\0" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        buf.as_mut_ptr() as *mut libc::c_void,
        255 as libc::c_int,
        (w * 16 as libc::c_int) as size_t,
    );
    memset(
        &mut tq as *mut tiny_queue_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<tiny_queue_t>() as libc::c_ulong,
    );
    if (*p).m < ((*p).n).wrapping_add((len / w) as size_t) {
        (*p).m = ((*p).n).wrapping_add((len / w) as size_t);
        (*p).m = ((*p).m).wrapping_sub(1);
        (*p).m |= (*p).m >> 1 as libc::c_int;
        (*p).m |= (*p).m >> 2 as libc::c_int;
        (*p).m |= (*p).m >> 4 as libc::c_int;
        (*p).m |= (*p).m >> 8 as libc::c_int;
        (*p).m |= (*p).m >> 16 as libc::c_int;
        (*p).m = ((*p).m).wrapping_add(1);
        tmp___0 = krealloc(
            km,
            (*p).a as *mut libc::c_void,
            (::std::mem::size_of::<mm128_t>() as libc::c_ulong).wrapping_mul((*p).m),
        );
        (*p).a = tmp___0 as *mut mm128_t;
    }
    min_pos = 0 as libc::c_int;
    buf_pos = min_pos;
    l = buf_pos;
    i = l;
    while i < len {
        let mut current_block_244: u64;
        c = seq_nt4_table[*str.offset(i as isize) as uint8_t as usize] as libc::c_int;
        info.x = 18446744073709551615 as libc::c_ulonglong as uint64_t;
        info.y = 18446744073709551615 as libc::c_ulonglong as uint64_t;
        if c < 4 as libc::c_int {
            if is_hpc != 0 {
                skip_len = 1 as libc::c_int;
                if (i + 1 as libc::c_int) < len {
                    if seq_nt4_table[*str.offset((i + 1 as libc::c_int) as isize)
                        as uint8_t as usize] as libc::c_int == c
                    {
                        skip_len = 2 as libc::c_int;
                        while i + skip_len < len {
                            if seq_nt4_table[*str.offset((i + skip_len) as isize)
                                as uint8_t as usize] as libc::c_int != c
                            {
                                break;
                            }
                            skip_len += 1;
                        }
                        i += skip_len - 1 as libc::c_int;
                    }
                }
                tq_push(&mut tq, skip_len);
                kmer_span += skip_len;
                if tq.count > k {
                    tmp___1 = tq_shift(&mut tq);
                    kmer_span -= tmp___1;
                }
            } else if (l + 1 as libc::c_int) < k {
                kmer_span = l + 1 as libc::c_int;
            } else {
                kmer_span = k;
            }
            kmer[0 as libc::c_int
                as usize] = (kmer[0 as libc::c_int as usize] << 2 as libc::c_int
                | c as libc::c_ulong) & mask;
            kmer[1 as libc::c_int
                as usize] = ((kmer[1 as libc::c_int as usize] >> 2 as libc::c_int)
                as libc::c_ulonglong
                | (3 as libc::c_ulonglong ^ c as libc::c_ulonglong) << shift1)
                as uint64_t;
            if kmer[0 as libc::c_int as usize] == kmer[1 as libc::c_int as usize] {
                current_block_244 = 18252959353284554359;
            } else {
                if kmer[0 as libc::c_int as usize] < kmer[1 as libc::c_int as usize] {
                    z = 0 as libc::c_int;
                } else {
                    z = 1 as libc::c_int;
                }
                l += 1;
                if l >= k {
                    if kmer_span < 256 as libc::c_int {
                        tmp___2 = hash64___0(kmer[z as usize], mask);
                        info
                            .x = tmp___2 << 8 as libc::c_int
                            | kmer_span as libc::c_ulong;
                        info
                            .y = (rid as uint64_t) << 32 as libc::c_int
                            | ((i as uint32_t) << 1 as libc::c_int) as libc::c_ulong
                            | z as libc::c_ulong;
                    }
                }
                current_block_244 = 9879896046554623444;
            }
        } else {
            l = 0 as libc::c_int;
            tq.front = 0 as libc::c_int;
            tq.count = tq.front;
            kmer_span = 0 as libc::c_int;
            current_block_244 = 9879896046554623444;
        }
        match current_block_244 {
            9879896046554623444 => {
                buf[buf_pos as usize] = info;
                if l == w + k - 1 as libc::c_int {
                    if min.x as libc::c_ulonglong
                        != 18446744073709551615 as libc::c_ulonglong
                    {
                        j = buf_pos + 1 as libc::c_int;
                        while j < w {
                            if min.x == buf[j as usize].x {
                                if buf[j as usize].y != min.y {
                                    if (*p).n == (*p).m {
                                        if (*p).m != 0 {
                                            (*p).m <<= 1 as libc::c_int;
                                        } else {
                                            (*p).m = 2 as libc::c_int as size_t;
                                        }
                                        tmp___3 = krealloc(
                                            km,
                                            (*p).a as *mut libc::c_void,
                                            (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                                .wrapping_mul((*p).m),
                                        );
                                        (*p).a = tmp___3 as *mut mm128_t;
                                    }
                                    tmp___4 = (*p).n;
                                    (*p).n = ((*p).n).wrapping_add(1);
                                    *((*p).a).offset(tmp___4 as isize) = buf[j as usize];
                                }
                            }
                            j += 1;
                        }
                        j = 0 as libc::c_int;
                        while j < buf_pos {
                            if min.x == buf[j as usize].x {
                                if buf[j as usize].y != min.y {
                                    if (*p).n == (*p).m {
                                        if (*p).m != 0 {
                                            (*p).m <<= 1 as libc::c_int;
                                        } else {
                                            (*p).m = 2 as libc::c_int as size_t;
                                        }
                                        tmp___5 = krealloc(
                                            km,
                                            (*p).a as *mut libc::c_void,
                                            (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                                .wrapping_mul((*p).m),
                                        );
                                        (*p).a = tmp___5 as *mut mm128_t;
                                    }
                                    tmp___6 = (*p).n;
                                    (*p).n = ((*p).n).wrapping_add(1);
                                    *((*p).a).offset(tmp___6 as isize) = buf[j as usize];
                                }
                            }
                            j += 1;
                        }
                    }
                }
                if info.x <= min.x {
                    if l >= w + k {
                        if min.x as libc::c_ulonglong
                            != 18446744073709551615 as libc::c_ulonglong
                        {
                            if (*p).n == (*p).m {
                                if (*p).m != 0 {
                                    (*p).m <<= 1 as libc::c_int;
                                } else {
                                    (*p).m = 2 as libc::c_int as size_t;
                                }
                                tmp___7 = krealloc(
                                    km,
                                    (*p).a as *mut libc::c_void,
                                    (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                        .wrapping_mul((*p).m),
                                );
                                (*p).a = tmp___7 as *mut mm128_t;
                            }
                            tmp___8 = (*p).n;
                            (*p).n = ((*p).n).wrapping_add(1);
                            *((*p).a).offset(tmp___8 as isize) = min;
                        }
                    }
                    min = info;
                    min_pos = buf_pos;
                } else if buf_pos == min_pos {
                    if l >= w + k - 1 as libc::c_int {
                        if min.x as libc::c_ulonglong
                            != 18446744073709551615 as libc::c_ulonglong
                        {
                            if (*p).n == (*p).m {
                                if (*p).m != 0 {
                                    (*p).m <<= 1 as libc::c_int;
                                } else {
                                    (*p).m = 2 as libc::c_int as size_t;
                                }
                                tmp___9 = krealloc(
                                    km,
                                    (*p).a as *mut libc::c_void,
                                    (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                        .wrapping_mul((*p).m),
                                );
                                (*p).a = tmp___9 as *mut mm128_t;
                            }
                            tmp___10 = (*p).n;
                            (*p).n = ((*p).n).wrapping_add(1);
                            *((*p).a).offset(tmp___10 as isize) = min;
                        }
                    }
                    j = buf_pos + 1 as libc::c_int;
                    min.x = 18446744073709551615 as libc::c_ulonglong as uint64_t;
                    while j < w {
                        if min.x >= buf[j as usize].x {
                            min = buf[j as usize];
                            min_pos = j;
                        }
                        j += 1;
                    }
                    j = 0 as libc::c_int;
                    while j <= buf_pos {
                        if min.x >= buf[j as usize].x {
                            min = buf[j as usize];
                            min_pos = j;
                        }
                        j += 1;
                    }
                    if l >= w + k - 1 as libc::c_int {
                        if min.x as libc::c_ulonglong
                            != 18446744073709551615 as libc::c_ulonglong
                        {
                            j = buf_pos + 1 as libc::c_int;
                            while j < w {
                                if min.x == buf[j as usize].x {
                                    if min.y != buf[j as usize].y {
                                        if (*p).n == (*p).m {
                                            if (*p).m != 0 {
                                                (*p).m <<= 1 as libc::c_int;
                                            } else {
                                                (*p).m = 2 as libc::c_int as size_t;
                                            }
                                            tmp___11 = krealloc(
                                                km,
                                                (*p).a as *mut libc::c_void,
                                                (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                                    .wrapping_mul((*p).m),
                                            );
                                            (*p).a = tmp___11 as *mut mm128_t;
                                        }
                                        tmp___12 = (*p).n;
                                        (*p).n = ((*p).n).wrapping_add(1);
                                        *((*p).a).offset(tmp___12 as isize) = buf[j as usize];
                                    }
                                }
                                j += 1;
                            }
                            j = 0 as libc::c_int;
                            while j <= buf_pos {
                                if min.x == buf[j as usize].x {
                                    if min.y != buf[j as usize].y {
                                        if (*p).n == (*p).m {
                                            if (*p).m != 0 {
                                                (*p).m <<= 1 as libc::c_int;
                                            } else {
                                                (*p).m = 2 as libc::c_int as size_t;
                                            }
                                            tmp___13 = krealloc(
                                                km,
                                                (*p).a as *mut libc::c_void,
                                                (::std::mem::size_of::<mm128_t>() as libc::c_ulong)
                                                    .wrapping_mul((*p).m),
                                            );
                                            (*p).a = tmp___13 as *mut mm128_t;
                                        }
                                        tmp___14 = (*p).n;
                                        (*p).n = ((*p).n).wrapping_add(1);
                                        *((*p).a).offset(tmp___14 as isize) = buf[j as usize];
                                    }
                                }
                                j += 1;
                            }
                        }
                    }
                }
                buf_pos += 1;
                if buf_pos == w {
                    buf_pos = 0 as libc::c_int;
                }
            }
            _ => {}
        }
        i += 1;
    }
    if min.x as libc::c_ulonglong != 18446744073709551615 as libc::c_ulonglong {
        if (*p).n == (*p).m {
            if (*p).m != 0 {
                (*p).m <<= 1 as libc::c_int;
            } else {
                (*p).m = 2 as libc::c_int as size_t;
            }
            tmp___15 = krealloc(
                km,
                (*p).a as *mut libc::c_void,
                (::std::mem::size_of::<mm128_t>() as libc::c_ulong).wrapping_mul((*p).m),
            );
            (*p).a = tmp___15 as *mut mm128_t;
        }
        tmp___16 = (*p).n;
        (*p).n = ((*p).n).wrapping_add(1);
        *((*p).a).offset(tmp___16 as isize) = min;
    }
}
pub unsafe extern "C" fn mm_split_init(
    mut prefix: *const libc::c_char,
    mut mi: *const mm_idx_t,
) -> *mut FILE {
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut i: uint32_t = 0;
    let mut k: uint32_t = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: uint32_t = 0;
    let mut tmp___3: size_t = 0;
    k = (*mi).k as uint32_t;
    tmp = strlen(prefix);
    tmp___0 = calloc(tmp.wrapping_add(10 as libc::c_ulong), 1 as libc::c_int as size_t);
    fn_0 = tmp___0 as *mut libc::c_char;
    sprintf(
        fn_0,
        b"%s.%.4d.tmp\0" as *const u8 as *const libc::c_char,
        prefix,
        (*mi).index,
    );
    fp = fopen(fn_0 as *const libc::c_char, b"wb\0" as *const u8 as *const libc::c_char);
    if fp as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        if mm_verbose >= 1 as libc::c_int {
            tmp___1 = __errno_location();
            tmp___2 = strerror(*tmp___1);
            fprintf(
                stderr,
                b"[ERROR]\x1B[1;31m failed to write to temporary file '%s'\x1B[0m: %s\n\0"
                    as *const u8 as *const libc::c_char,
                fn_0,
                tmp___2,
            );
        }
        exit(1 as libc::c_int);
    }
    mm_err_fwrite(
        &mut k as *mut uint32_t as *const libc::c_void,
        4 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        fp,
    );
    mm_err_fwrite(
        &(*mi).n_seq as *const uint32_t as *const libc::c_void,
        4 as libc::c_int as size_t,
        1 as libc::c_int as size_t,
        fp,
    );
    i = 0 as libc::c_int as uint32_t;
    while i < (*mi).n_seq {
        tmp___3 = strlen((*((*mi).seq).offset(i as isize)).name as *const libc::c_char);
        l = tmp___3 as uint32_t;
        mm_err_fwrite(
            &mut l as *mut uint32_t as *const libc::c_void,
            1 as libc::c_int as size_t,
            4 as libc::c_int as size_t,
            fp,
        );
        mm_err_fwrite(
            (*((*mi).seq).offset(i as isize)).name as *const libc::c_void,
            1 as libc::c_int as size_t,
            l as size_t,
            fp,
        );
        mm_err_fwrite(
            &mut (*((*mi).seq).offset(i as isize)).len as *mut uint32_t
                as *const libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            fp,
        );
        i = i.wrapping_add(1);
    }
    free(fn_0 as *mut libc::c_void);
    return fp;
}
pub unsafe extern "C" fn mm_split_merge_prep(
    mut prefix: *const libc::c_char,
    mut n_splits: libc::c_int,
    mut fp: *mut *mut FILE,
    mut n_seq_part: *mut uint32_t,
) -> *mut mm_idx_t {
    let mut mi: *mut mm_idx_t = 0 as *mut mm_idx_t;
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut FILE = 0 as *mut FILE;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut k: uint32_t = 0;
    let mut l: uint32_t = 0;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    mi = 0 as *mut mm_idx_t;
    if n_splits < 1 as libc::c_int {
        return 0 as *mut mm_idx_t;
    }
    tmp = strlen(prefix);
    tmp___0 = calloc(
        tmp.wrapping_add(10 as libc::c_ulong),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    );
    fn_0 = tmp___0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < n_splits {
        sprintf(fn_0, b"%s.%.4d.tmp\0" as *const u8 as *const libc::c_char, prefix, i);
        tmp___3 = fopen(
            fn_0 as *const libc::c_char,
            b"rb\0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh53 = *fp.offset(i as isize);
        *fresh53 = tmp___3;
        if tmp___3 as libc::c_ulong == 0 as *mut FILE as libc::c_ulong {
            if mm_verbose >= 1 as libc::c_int {
                tmp___1 = __errno_location();
                tmp___2 = strerror(*tmp___1);
                fprintf(
                    stderr,
                    b"ERROR: failed to open temporary file '%s': %s\n\0" as *const u8
                        as *const libc::c_char,
                    fn_0,
                    tmp___2,
                );
            }
            j = 0 as libc::c_int;
            while j < i {
                fclose(*fp.offset(j as isize));
                j += 1;
            }
            free(fn_0 as *mut libc::c_void);
            return 0 as *mut mm_idx_t;
        }
        i += 1;
    }
    free(fn_0 as *mut libc::c_void);
    tmp___4 = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<mm_idx_t>() as libc::c_ulong,
    );
    mi = tmp___4 as *mut mm_idx_t;
    i = 0 as libc::c_int;
    while i < n_splits {
        mm_err_fread(
            &mut (*mi).k as *mut int32_t as *mut libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            *fp.offset(i as isize),
        );
        mm_err_fread(
            n_seq_part.offset(i as isize) as *mut libc::c_void,
            4 as libc::c_int as size_t,
            1 as libc::c_int as size_t,
            *fp.offset(i as isize),
        );
        (*mi)
            .n_seq = ((*mi).n_seq as libc::c_uint)
            .wrapping_add(*n_seq_part.offset(i as isize)) as uint32_t as uint32_t;
        i += 1;
    }
    tmp___5 = calloc(
        (*mi).n_seq as size_t,
        ::std::mem::size_of::<mm_idx_seq_t>() as libc::c_ulong,
    );
    (*mi).seq = tmp___5 as *mut mm_idx_seq_t;
    j = 0 as libc::c_int;
    i = j;
    while i < n_splits {
        k = 0 as libc::c_int as uint32_t;
        while k < *n_seq_part.offset(i as isize) {
            mm_err_fread(
                &mut l as *mut uint32_t as *mut libc::c_void,
                1 as libc::c_int as size_t,
                4 as libc::c_int as size_t,
                *fp.offset(i as isize),
            );
            tmp___6 = calloc(
                l.wrapping_add(1 as libc::c_uint) as size_t,
                1 as libc::c_int as size_t,
            );
            let ref mut fresh54 = (*((*mi).seq).offset(j as isize)).name;
            *fresh54 = tmp___6 as *mut libc::c_char;
            mm_err_fread(
                (*((*mi).seq).offset(j as isize)).name as *mut libc::c_void,
                1 as libc::c_int as size_t,
                l as size_t,
                *fp.offset(i as isize),
            );
            mm_err_fread(
                &mut (*((*mi).seq).offset(j as isize)).len as *mut uint32_t
                    as *mut libc::c_void,
                4 as libc::c_int as size_t,
                1 as libc::c_int as size_t,
                *fp.offset(i as isize),
            );
            k = k.wrapping_add(1);
            j += 1;
        }
        i += 1;
    }
    return mi;
}
pub unsafe extern "C" fn mm_split_rm_tmp(
    mut prefix: *const libc::c_char,
    mut n_splits: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut fn_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = strlen(prefix);
    tmp___0 = calloc(
        tmp.wrapping_add(10 as libc::c_ulong),
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
    );
    fn_0 = tmp___0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < n_splits {
        sprintf(fn_0, b"%s.%.4d.tmp\0" as *const u8 as *const libc::c_char, prefix, i);
        remove(fn_0 as *const libc::c_char);
        i += 1;
    }
    free(fn_0 as *mut libc::c_void);
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
