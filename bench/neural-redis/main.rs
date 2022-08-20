use ::libc;
extern "C" {
    pub type RedisModuleCtx;
    pub type RedisModuleKey;
    pub type RedisModuleString;
    pub type RedisModuleCallReply;
    pub type RedisModuleIO;
    pub type RedisModuleType;
    pub type RedisModuleDigest;
    pub type RedisModuleBlockedClient;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn rand() -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exp(_: libc::c_double) -> libc::c_double;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type mstime_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleTypeMethods {
    pub version: uint64_t,
    pub rdb_load: Option::<
        unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int) -> *mut libc::c_void,
    >,
    pub rdb_save: Option::<
        unsafe extern "C" fn(*mut RedisModuleIO, *mut libc::c_void) -> (),
    >,
    pub aof_rewrite: Option::<
        unsafe extern "C" fn(
            *mut RedisModuleIO,
            *mut RedisModuleString,
            *mut libc::c_void,
        ) -> (),
    >,
    pub mem_usage: Option::<unsafe extern "C" fn(*mut libc::c_void) -> size_t>,
    pub digest: Option::<
        unsafe extern "C" fn(*mut RedisModuleDigest, *mut libc::c_void) -> (),
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AnnLayer {
    pub output: *mut libc::c_float,
    pub error: *mut libc::c_float,
    pub weight: *mut libc::c_float,
    pub gradient: *mut libc::c_float,
    pub sgradient: *mut libc::c_float,
    pub pgradient: *mut libc::c_float,
    pub delta: *mut libc::c_float,
    pub units: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ann {
    pub flags: libc::c_int,
    pub layers: libc::c_int,
    pub rprop_nminus: libc::c_float,
    pub rprop_nplus: libc::c_float,
    pub rprop_maxupdate: libc::c_float,
    pub rprop_minupdate: libc::c_float,
    pub learn_rate: libc::c_float,
    pub _filler_: libc::c_float,
    pub layer: *mut AnnLayer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NRDataset {
    pub len: uint32_t,
    pub maxlen: uint32_t,
    pub inputs: *mut libc::c_float,
    pub outputs: *mut libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_NRTypeObject_124612850 {
    pub id: uint64_t,
    pub training_total_steps: uint64_t,
    pub training_total_ms: uint64_t,
    pub training_max_cycles: uint64_t,
    pub training_max_ms: uint64_t,
    pub flags: uint32_t,
    pub epochs: uint32_t,
    pub nn: *mut Ann,
    pub dataset: NRDataset,
    pub test: NRDataset,
    pub dataset_error: libc::c_float,
    pub test_error: libc::c_float,
    pub test_class_error: libc::c_float,
    pub inorm: *mut libc::c_float,
    pub onorm: *mut libc::c_float,
}
pub type NRTypeObject = __anonstruct_NRTypeObject_124612850;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_NRPendingTraining_634934114 {
    pub key: *mut RedisModuleString,
    pub db_id: libc::c_int,
    pub tid: pthread_t,
    pub in_progress: libc::c_int,
    pub nr: *mut NRTypeObject,
    pub dataset_error: libc::c_float,
    pub test_error: libc::c_float,
    pub class_error: libc::c_float,
    pub curcycle: libc::c_int,
}
pub type NRPendingTraining = __anonstruct_NRPendingTraining_634934114;
pub static mut RedisModule_Alloc: Option::<
    unsafe extern "C" fn(size_t) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_Realloc: Option::<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_Free: Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
> = None;
pub static mut RedisModule_Calloc: Option::<
    unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_Strdup: Option::<
    unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
> = None;
pub static mut RedisModule_GetApi: Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
> = None;
pub static mut RedisModule_CreateCommand: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        Option::<
            unsafe extern "C" fn(
                *mut RedisModuleCtx,
                *mut *mut RedisModuleString,
                libc::c_int,
            ) -> libc::c_int,
        >,
        *const libc::c_char,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_SetModuleAttribs: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_WrongArity: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithLongLong: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_longlong) -> libc::c_int,
> = None;
pub static mut RedisModule_GetSelectedDb: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_SelectDb: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> libc::c_int,
> = None;
pub static mut RedisModule_OpenKey: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *mut RedisModuleString,
        libc::c_int,
    ) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_CloseKey: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> (),
> = None;
pub static mut RedisModule_KeyType: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
> = None;
pub static mut RedisModule_ValueLength: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> size_t,
> = None;
pub static mut RedisModule_ListPush: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        libc::c_int,
        *mut RedisModuleString,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ListPop: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, libc::c_int) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_Call: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        *const libc::c_char,
        ...
    ) -> *mut RedisModuleCallReply,
> = None;
pub static mut RedisModule_CallReplyProto: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply, *mut size_t) -> *const libc::c_char,
> = None;
pub static mut RedisModule_FreeCallReply: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply) -> (),
> = None;
pub static mut RedisModule_CallReplyType: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_int,
> = None;
pub static mut RedisModule_CallReplyInteger: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_longlong,
> = None;
pub static mut RedisModule_CallReplyLength: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply) -> size_t,
> = None;
pub static mut RedisModule_CallReplyArrayElement: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply, size_t) -> *mut RedisModuleCallReply,
> = None;
pub static mut RedisModule_CreateString: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        size_t,
    ) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_CreateStringFromLongLong: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_longlong) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_CreateStringFromString: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const RedisModuleString,
    ) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_CreateStringPrintf: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        ...
    ) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_FreeString: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleString) -> (),
> = None;
pub static mut RedisModule_StringPtrLen: Option::<
    unsafe extern "C" fn(*const RedisModuleString, *mut size_t) -> *const libc::c_char,
> = None;
pub static mut RedisModule_ReplyWithError: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *const libc::c_char) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithSimpleString: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *const libc::c_char) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithArray: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplySetArrayLength: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> (),
> = None;
pub static mut RedisModule_ReplyWithStringBuffer: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *const libc::c_char, size_t) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithString: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleString) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithNull: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithDouble: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_double) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplyWithCallReply: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleCallReply) -> libc::c_int,
> = None;
pub static mut RedisModule_StringToLongLong: Option::<
    unsafe extern "C" fn(*const RedisModuleString, *mut libc::c_longlong) -> libc::c_int,
> = None;
pub static mut RedisModule_StringToDouble: Option::<
    unsafe extern "C" fn(*const RedisModuleString, *mut libc::c_double) -> libc::c_int,
> = None;
pub static mut RedisModule_AutoMemory: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> (),
> = None;
pub static mut RedisModule_Replicate: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        *const libc::c_char,
        ...
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ReplicateVerbatim: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_CallReplyStringPtr: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply, *mut size_t) -> *const libc::c_char,
> = None;
pub static mut RedisModule_CreateStringFromCallReply: Option::<
    unsafe extern "C" fn(*mut RedisModuleCallReply) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_DeleteKey: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
> = None;
pub static mut RedisModule_StringSet: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, *mut RedisModuleString) -> libc::c_int,
> = None;
pub static mut RedisModule_StringDMA: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut size_t,
        libc::c_int,
    ) -> *mut libc::c_char,
> = None;
pub static mut RedisModule_StringTruncate: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, size_t) -> libc::c_int,
> = None;
pub static mut RedisModule_GetExpire: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> mstime_t,
> = None;
pub static mut RedisModule_SetExpire: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, mstime_t) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetAdd: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        libc::c_double,
        *mut RedisModuleString,
        *mut libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetIncrby: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        libc::c_double,
        *mut RedisModuleString,
        *mut libc::c_int,
        *mut libc::c_double,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetScore: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut RedisModuleString,
        *mut libc::c_double,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetRem: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut RedisModuleString,
        *mut libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetRangeStop: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> (),
> = None;
pub static mut RedisModule_ZsetFirstInScoreRange: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        libc::c_double,
        libc::c_double,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetLastInScoreRange: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        libc::c_double,
        libc::c_double,
        libc::c_int,
        libc::c_int,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetFirstInLexRange: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut RedisModuleString,
        *mut RedisModuleString,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetLastInLexRange: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut RedisModuleString,
        *mut RedisModuleString,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetRangeCurrentElement: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut libc::c_double,
    ) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_ZsetRangeNext: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetRangePrev: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
> = None;
pub static mut RedisModule_ZsetRangeEndReached: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
> = None;
pub static mut RedisModule_HashSet: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, libc::c_int, ...) -> libc::c_int,
> = None;
pub static mut RedisModule_HashGet: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey, libc::c_int, ...) -> libc::c_int,
> = None;
pub static mut RedisModule_IsKeysPositionRequest: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_KeyAtPos: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> (),
> = None;
pub static mut RedisModule_GetClientId: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_ulonglong,
> = None;
pub static mut RedisModule_PoolAlloc: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, size_t) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_CreateDataType: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        libc::c_int,
        *mut RedisModuleTypeMethods,
    ) -> *mut RedisModuleType,
> = None;
pub static mut RedisModule_ModuleTypeSetValue: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut RedisModuleType,
        *mut libc::c_void,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_ModuleTypeGetType: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> *mut RedisModuleType,
> = None;
pub static mut RedisModule_ModuleTypeGetValue: Option::<
    unsafe extern "C" fn(*mut RedisModuleKey) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_SaveUnsigned: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, uint64_t) -> (),
> = None;
pub static mut RedisModule_LoadUnsigned: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> uint64_t,
> = None;
pub static mut RedisModule_SaveSigned: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, int64_t) -> (),
> = None;
pub static mut RedisModule_LoadSigned: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> int64_t,
> = None;
pub static mut RedisModule_EmitAOF: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleIO,
        *const libc::c_char,
        *const libc::c_char,
        ...
    ) -> (),
> = None;
pub static mut RedisModule_SaveString: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut RedisModuleString) -> (),
> = None;
pub static mut RedisModule_SaveStringBuffer: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *const libc::c_char, size_t) -> (),
> = None;
pub static mut RedisModule_LoadString: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> *mut RedisModuleString,
> = None;
pub static mut RedisModule_LoadStringBuffer: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut size_t) -> *mut libc::c_char,
> = None;
pub static mut RedisModule_SaveDouble: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_double) -> (),
> = None;
pub static mut RedisModule_LoadDouble: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_double,
> = None;
pub static mut RedisModule_SaveFloat: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_float) -> (),
> = None;
pub static mut RedisModule_LoadFloat: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_float,
> = None;
pub static mut RedisModule_Log: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        *const libc::c_char,
        ...
    ) -> (),
> = None;
pub static mut RedisModule_LogIOError: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleIO,
        *const libc::c_char,
        *const libc::c_char,
        ...
    ) -> (),
> = None;
pub static mut RedisModule_StringAppendBuffer: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *mut RedisModuleString,
        *const libc::c_char,
        size_t,
    ) -> libc::c_int,
> = None;
pub static mut RedisModule_RetainString: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleString) -> (),
> = None;
pub static mut RedisModule_StringCompare: Option::<
    unsafe extern "C" fn(*mut RedisModuleString, *mut RedisModuleString) -> libc::c_int,
> = None;
pub static mut RedisModule_GetContextFromIO: Option::<
    unsafe extern "C" fn(*mut RedisModuleIO) -> *mut RedisModuleCtx,
> = None;
pub static mut RedisModule_BlockClient: Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        Option::<
            unsafe extern "C" fn(
                *mut RedisModuleCtx,
                *mut *mut RedisModuleString,
                libc::c_int,
            ) -> libc::c_int,
        >,
        Option::<
            unsafe extern "C" fn(
                *mut RedisModuleCtx,
                *mut *mut RedisModuleString,
                libc::c_int,
            ) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        libc::c_longlong,
    ) -> *mut RedisModuleBlockedClient,
> = None;
pub static mut RedisModule_UnblockClient: Option::<
    unsafe extern "C" fn(*mut RedisModuleBlockedClient, *mut libc::c_void) -> libc::c_int,
> = None;
pub static mut RedisModule_IsBlockedReplyRequest: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_IsBlockedTimeoutRequest: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
> = None;
pub static mut RedisModule_GetBlockedClientPrivateData: Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut libc::c_void,
> = None;
pub static mut RedisModule_AbortBlock: Option::<
    unsafe extern "C" fn(*mut RedisModuleBlockedClient) -> libc::c_int,
> = None;
pub static mut RedisModule_Milliseconds: Option::<
    unsafe extern "C" fn() -> libc::c_longlong,
> = None;
unsafe extern "C" fn RedisModule_Init(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
    mut ver: libc::c_int,
    mut apiver: libc::c_int,
) -> libc::c_int {
    let mut getapifuncptr: *mut libc::c_void = 0 as *mut libc::c_void;
    getapifuncptr = *(ctx as *mut *mut libc::c_void).offset(0 as libc::c_int as isize);
    RedisModule_GetApi = ::std::mem::transmute::<
        libc::intptr_t,
        Option::<
            unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
        >,
    >(getapifuncptr as libc::c_ulong as libc::intptr_t);
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_Alloc\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Alloc
            as *mut Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_Calloc\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Calloc
            as *mut Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_Free\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Free
            as *mut Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_Realloc\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Realloc
            as *mut Option::<
                unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_Strdup\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Strdup
            as *mut Option::<
                unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CreateCommand\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateCommand
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    Option::<
                        unsafe extern "C" fn(
                            *mut RedisModuleCtx,
                            *mut *mut RedisModuleString,
                            libc::c_int,
                        ) -> libc::c_int,
                    >,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_SetModuleAttribs\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SetModuleAttribs
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_WrongArity\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_WrongArity
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplyWithLongLong\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithLongLong
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    libc::c_longlong,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplyWithError\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithError
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplyWithSimpleString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithSimpleString
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplyWithArray\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithArray
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplySetArrayLength\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplySetArrayLength
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplyWithStringBuffer\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithStringBuffer
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplyWithString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithString
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplyWithNull\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithNull
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplyWithCallReply\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithCallReply
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleCallReply,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplyWithDouble\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplyWithDouble
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_double) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplySetArrayLength\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplySetArrayLength
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_GetSelectedDb\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_GetSelectedDb
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_SelectDb\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SelectDb
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_OpenKey\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_OpenKey
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleString,
                    libc::c_int,
                ) -> *mut libc::c_void,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CloseKey\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CloseKey
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_KeyType\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_KeyType
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ValueLength\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ValueLength
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> size_t>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ListPush\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ListPush
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ListPop\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ListPop
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_StringToLongLong\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringToLongLong
            as *mut Option::<
                unsafe extern "C" fn(
                    *const RedisModuleString,
                    *mut libc::c_longlong,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_StringToDouble\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringToDouble
            as *mut Option::<
                unsafe extern "C" fn(
                    *const RedisModuleString,
                    *mut libc::c_double,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_Call\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Call
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> *mut RedisModuleCallReply,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CallReplyProto\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyProto
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    *mut size_t,
                ) -> *const libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_FreeCallReply\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_FreeCallReply
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCallReply) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CallReplyInteger\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyInteger
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_longlong,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CallReplyType\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyType
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CallReplyLength\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyLength
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCallReply) -> size_t>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CallReplyArrayElement\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyArrayElement
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    size_t,
                ) -> *mut RedisModuleCallReply,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CallReplyStringPtr\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CallReplyStringPtr
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    *mut size_t,
                ) -> *const libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CreateStringFromCallReply\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateStringFromCallReply
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCallReply) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CreateString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateString
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    size_t,
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CreateStringFromLongLong\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateStringFromLongLong
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    libc::c_longlong,
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CreateStringFromString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateStringFromString
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const RedisModuleString,
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CreateStringPrintf\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateStringPrintf
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    ...
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_FreeString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_FreeString
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleString) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_StringPtrLen\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringPtrLen
            as *mut Option::<
                unsafe extern "C" fn(
                    *const RedisModuleString,
                    *mut size_t,
                ) -> *const libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_AutoMemory\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_AutoMemory
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_Replicate\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Replicate
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ReplicateVerbatim\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ReplicateVerbatim
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_DeleteKey\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_DeleteKey
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_StringSet\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringSet
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_StringDMA\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringDMA
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut size_t,
                    libc::c_int,
                ) -> *mut libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_StringTruncate\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringTruncate
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleKey, size_t) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_GetExpire\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_GetExpire
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> mstime_t>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_SetExpire\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SetExpire
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleKey, mstime_t) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetAdd\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetAdd
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    *mut RedisModuleString,
                    *mut libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetIncrby\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetIncrby
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    *mut RedisModuleString,
                    *mut libc::c_int,
                    *mut libc::c_double,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetScore\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetScore
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleString,
                    *mut libc::c_double,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetRem\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRem
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleString,
                    *mut libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetRangeStop\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRangeStop
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetFirstInScoreRange\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetFirstInScoreRange
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    libc::c_double,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetLastInScoreRange\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetLastInScoreRange
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    libc::c_double,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetFirstInLexRange\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetFirstInLexRange
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleString,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetLastInLexRange\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetLastInLexRange
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleString,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetRangeCurrentElement\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRangeCurrentElement
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut libc::c_double,
                ) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetRangeNext\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRangeNext
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetRangePrev\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRangePrev
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ZsetRangeEndReached\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ZsetRangeEndReached
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_HashSet\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_HashSet
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    ...
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_HashGet\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_HashGet
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    ...
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_IsKeysPositionRequest\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_IsKeysPositionRequest
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_KeyAtPos\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_KeyAtPos
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_GetClientId\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_GetClientId
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_ulonglong,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_PoolAlloc\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_PoolAlloc
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, size_t) -> *mut libc::c_void,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_CreateDataType\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_CreateDataType
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    libc::c_int,
                    *mut RedisModuleTypeMethods,
                ) -> *mut RedisModuleType,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ModuleTypeSetValue\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ModuleTypeSetValue
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleType,
                    *mut libc::c_void,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ModuleTypeGetType\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ModuleTypeGetType
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleKey) -> *mut RedisModuleType,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_ModuleTypeGetValue\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_ModuleTypeGetValue
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleKey) -> *mut libc::c_void,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_SaveUnsigned\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveUnsigned
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO, uint64_t) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_LoadUnsigned\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadUnsigned
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> uint64_t>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_SaveSigned\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveSigned
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO, int64_t) -> ()>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_LoadSigned\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadSigned
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> int64_t>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_SaveString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveString
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleIO, *mut RedisModuleString) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_SaveStringBuffer\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveStringBuffer
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *const libc::c_char,
                    size_t,
                ) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_LoadString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadString
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleIO) -> *mut RedisModuleString,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_LoadStringBuffer\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadStringBuffer
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *mut size_t,
                ) -> *mut libc::c_char,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_SaveDouble\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveDouble
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleIO, libc::c_double) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_LoadDouble\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadDouble
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_double>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_SaveFloat\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_SaveFloat
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleIO, libc::c_float) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_LoadFloat\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LoadFloat
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_float>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_EmitAOF\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_EmitAOF
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_Log\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Log
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_LogIOError\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_LogIOError
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_StringAppendBuffer\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringAppendBuffer
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleString,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_RetainString\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_RetainString
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleString) -> (),
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_StringCompare\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_StringCompare
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleString,
                    *mut RedisModuleString,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_GetContextFromIO\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_GetContextFromIO
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleIO) -> *mut RedisModuleCtx,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_BlockClient\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_BlockClient
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    Option::<
                        unsafe extern "C" fn(
                            *mut RedisModuleCtx,
                            *mut *mut RedisModuleString,
                            libc::c_int,
                        ) -> libc::c_int,
                    >,
                    Option::<
                        unsafe extern "C" fn(
                            *mut RedisModuleCtx,
                            *mut *mut RedisModuleString,
                            libc::c_int,
                        ) -> libc::c_int,
                    >,
                    Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                    libc::c_longlong,
                ) -> *mut RedisModuleBlockedClient,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_UnblockClient\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_UnblockClient
            as *mut Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleBlockedClient,
                    *mut libc::c_void,
                ) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_IsBlockedReplyRequest\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_IsBlockedReplyRequest
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_IsBlockedTimeoutRequest\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_IsBlockedTimeoutRequest
            as *mut Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_GetBlockedClientPrivateData\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_GetBlockedClientPrivateData
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut libc::c_void,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_AbortBlock\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_AbortBlock
            as *mut Option::<
                unsafe extern "C" fn(*mut RedisModuleBlockedClient) -> libc::c_int,
            > as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_GetApi.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        b"RedisModule_Milliseconds\0" as *const u8 as *const libc::c_char,
        &mut RedisModule_Milliseconds
            as *mut Option::<unsafe extern "C" fn() -> libc::c_longlong>
            as *mut *mut libc::c_void as *mut libc::c_void,
    );
    (Some(RedisModule_SetModuleAttribs.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, name, ver, apiver);
    return 0 as libc::c_int;
}
static mut NRType: *mut RedisModuleType = 0 as *const RedisModuleType
    as *mut RedisModuleType;
pub static mut NRNextId: uint64_t = 1 as libc::c_int as uint64_t;
static mut NRPendingTrainingMutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
static mut NRTrainings: [NRPendingTraining; 32] = [NRPendingTraining {
    key: 0 as *const RedisModuleString as *mut RedisModuleString,
    db_id: 0,
    tid: 0,
    in_progress: 0,
    nr: 0 as *const NRTypeObject as *mut NRTypeObject,
    dataset_error: 0.,
    test_error: 0.,
    class_error: 0.,
    curcycle: 0,
}; 32];
static mut NRPendingTrainingCount: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn NRMilliseconds() -> libc::c_longlong {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ust: libc::c_longlong = 0;
    gettimeofday(&mut tv as *mut timeval, 0 as *mut libc::c_void);
    ust = tv.tv_sec as libc::c_longlong * 1000000 as libc::c_longlong;
    ust += tv.tv_usec as libc::c_longlong;
    return ust / 1000 as libc::c_longlong;
}
pub unsafe extern "C" fn createNRTypeObject(
    mut flags: libc::c_int,
    mut layers: *mut libc::c_int,
    mut numlayers: libc::c_int,
    mut dset_len: libc::c_int,
    mut test_len: libc::c_int,
) -> *mut NRTypeObject {
    let mut o: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: uint64_t = 0;
    let mut ilen: libc::c_int = 0;
    let mut olen: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut j: libc::c_int = 0;
    let mut j___0: libc::c_int = 0;
    tmp = (Some(RedisModule_Calloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<NRTypeObject>() as libc::c_ulong,
    );
    o = tmp as *mut NRTypeObject;
    tmp___0 = NRNextId;
    NRNextId = NRNextId.wrapping_add(1);
    (*o).id = tmp___0;
    (*o).flags = flags as uint32_t;
    (*o).nn = AnnCreateNet(numlayers, layers);
    (*o).dataset.maxlen = dset_len as uint32_t;
    (*o).test.maxlen = test_len as uint32_t;
    ilen = (*((*(*o).nn).layer).offset(((*(*o).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    olen = (*((*(*o).nn).layer).offset(0 as libc::c_int as isize)).units;
    tmp___1 = (Some(RedisModule_Calloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        1 as libc::c_int as size_t,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong),
    );
    (*o).inorm = tmp___1 as *mut libc::c_float;
    tmp___2 = (Some(RedisModule_Calloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        1 as libc::c_int as size_t,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong),
    );
    (*o).onorm = tmp___2 as *mut libc::c_float;
    j = 0 as libc::c_int;
    while j < ilen {
        *((*o).inorm).offset(j as isize) = 1 as libc::c_int as libc::c_float;
        j += 1;
    }
    j___0 = 0 as libc::c_int;
    while j___0 < olen {
        *((*o).onorm).offset(j___0 as isize) = 1 as libc::c_int as libc::c_float;
        j___0 += 1;
    }
    return o;
}
pub unsafe extern "C" fn NRTypeInsertData(
    mut o: *mut NRTypeObject,
    mut inputs: *mut libc::c_float,
    mut outputs: *mut libc::c_float,
    mut target_ds: libc::c_int,
) {
    let mut target: *mut NRDataset = 0 as *mut NRDataset;
    let mut fill_a: libc::c_float = 0.;
    let mut fill_b: libc::c_float = 0.;
    let mut r: libc::c_double = 0.;
    let mut tmp: libc::c_int = 0;
    let mut sumlen: libc::c_double = 0.;
    let mut idx: size_t = 0;
    let mut j: libc::c_int = 0;
    let mut numin: libc::c_int = 0;
    let mut numout: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    target = 0 as *mut libc::c_void as *mut NRDataset;
    if (*o).dataset.maxlen == 0 as libc::c_uint {
        if (*o).test.maxlen == 0 as libc::c_uint {
            return;
        }
    }
    if target_ds == 1 as libc::c_int {
        target = &mut (*o).dataset;
    } else if target_ds == 2 as libc::c_int {
        target = &mut (*o).test;
    }
    if (*o).dataset.maxlen == 0 as libc::c_uint {
        target = &mut (*o).test;
    } else if (*o).test.maxlen == 0 as libc::c_uint {
        target = &mut (*o).dataset;
    }
    if target as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        let mut current_block_33: u64;
        if (*o).dataset.len != (*o).dataset.maxlen {
            current_block_33 = 10336135577553507410;
        } else if (*o).test.len != (*o).dataset.len {
            current_block_33 = 10336135577553507410;
        } else {
            tmp = rand();
            r = (tmp / 2147483647 as libc::c_int) as libc::c_double;
            sumlen = ((*o).dataset.maxlen).wrapping_add((*o).test.maxlen)
                as libc::c_double;
            if r < (*o).dataset.maxlen as libc::c_double / sumlen {
                target = &mut (*o).dataset;
            } else {
                target = &mut (*o).test;
            }
            current_block_33 = 7245201122033322888;
        }
        match current_block_33 {
            10336135577553507410 => {
                fill_a = (*o).dataset.len as libc::c_float
                    / (*o).dataset.maxlen as libc::c_float;
                fill_b = (*o).test.len as libc::c_float
                    / (*o).test.maxlen as libc::c_float;
                if fill_a <= fill_b {
                    target = &mut (*o).dataset;
                } else {
                    target = &mut (*o).test;
                }
            }
            _ => {}
        }
    }
    numin = (*((*(*o).nn).layer).offset(((*(*o).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    numout = (*((*(*o).nn).layer).offset(0 as libc::c_int as isize)).units;
    if (*target).maxlen == (*target).len {
        tmp___0 = rand();
        idx = (tmp___0 as libc::c_uint).wrapping_rem((*target).maxlen) as size_t;
    } else {
        idx = (*target).len as size_t;
        (*target).len = ((*target).len).wrapping_add(1);
        tmp___1 = (Some(RedisModule_Realloc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*target).inputs as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(numin as libc::c_ulong)
                .wrapping_mul((*target).len as libc::c_ulong),
        );
        (*target).inputs = tmp___1 as *mut libc::c_float;
        tmp___2 = (Some(RedisModule_Realloc.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*target).outputs as *mut libc::c_void,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(numout as libc::c_ulong)
                .wrapping_mul((*target).len as libc::c_ulong),
        );
        (*target).outputs = tmp___2 as *mut libc::c_float;
    }
    j = 0 as libc::c_int;
    while j < numin {
        *((*target).inputs)
            .offset(
                idx.wrapping_mul(numin as size_t).wrapping_add(j as size_t) as isize,
            ) = *inputs.offset(j as isize);
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < numout {
        *((*target).outputs)
            .offset(
                idx.wrapping_mul(numout as size_t).wrapping_add(j as size_t) as isize,
            ) = *outputs.offset(j as isize);
        j += 1;
    }
}
pub unsafe extern "C" fn NRDatasetFree(mut dset: *mut NRDataset) {
    (Some(RedisModule_Free.expect("non-null function pointer")))
        .expect("non-null function pointer")((*dset).inputs as *mut libc::c_void);
    (Some(RedisModule_Free.expect("non-null function pointer")))
        .expect("non-null function pointer")((*dset).outputs as *mut libc::c_void);
}
pub unsafe extern "C" fn NRTypeReleaseObject(mut o: *mut NRTypeObject) {
    AnnFree((*o).nn);
    NRDatasetFree(&mut (*o).dataset);
    NRDatasetFree(&mut (*o).test);
    (Some(RedisModule_Free.expect("non-null function pointer")))
        .expect("non-null function pointer")((*o).inorm as *mut libc::c_void);
    (Some(RedisModule_Free.expect("non-null function pointer")))
        .expect("non-null function pointer")((*o).onorm as *mut libc::c_void);
    (Some(RedisModule_Free.expect("non-null function pointer")))
        .expect("non-null function pointer")(o as *mut libc::c_void);
}
pub unsafe extern "C" fn NRClone(
    mut o: *mut NRTypeObject,
    mut newid: libc::c_int,
) -> *mut NRTypeObject {
    let mut copy: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: uint64_t = 0;
    let mut ilen: libc::c_int = 0;
    let mut olen: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___6: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = (Some(RedisModule_Calloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<NRTypeObject>() as libc::c_ulong,
    );
    copy = tmp as *mut NRTypeObject;
    *copy = *o;
    if newid != 0 {
        tmp___0 = NRNextId;
        NRNextId = NRNextId.wrapping_add(1);
        (*copy).id = tmp___0;
    }
    (*copy).nn = AnnClone((*o).nn);
    (*copy).dataset = (*o).dataset;
    (*copy).test = (*o).test;
    ilen = (*((*(*o).nn).layer).offset(((*(*o).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    olen = (*((*(*o).nn).layer).offset(0 as libc::c_int as isize)).units;
    tmp___1 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong)
            .wrapping_mul((*o).dataset.len as libc::c_ulong),
    );
    (*copy).dataset.inputs = tmp___1 as *mut libc::c_float;
    tmp___2 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong)
            .wrapping_mul((*o).dataset.len as libc::c_ulong),
    );
    (*copy).dataset.outputs = tmp___2 as *mut libc::c_float;
    tmp___3 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong)
            .wrapping_mul((*o).test.len as libc::c_ulong),
    );
    (*copy).test.inputs = tmp___3 as *mut libc::c_float;
    tmp___4 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong)
            .wrapping_mul((*o).test.len as libc::c_ulong),
    );
    (*copy).test.outputs = tmp___4 as *mut libc::c_float;
    memcpy(
        (*copy).dataset.inputs as *mut libc::c_void,
        (*o).dataset.inputs as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong)
            .wrapping_mul((*o).dataset.len as libc::c_ulong),
    );
    memcpy(
        (*copy).dataset.outputs as *mut libc::c_void,
        (*o).dataset.outputs as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong)
            .wrapping_mul((*o).dataset.len as libc::c_ulong),
    );
    memcpy(
        (*copy).test.inputs as *mut libc::c_void,
        (*o).test.inputs as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong)
            .wrapping_mul((*o).test.len as libc::c_ulong),
    );
    memcpy(
        (*copy).test.outputs as *mut libc::c_void,
        (*o).test.outputs as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong)
            .wrapping_mul((*o).test.len as libc::c_ulong),
    );
    tmp___5 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong),
    );
    (*copy).inorm = tmp___5 as *mut libc::c_float;
    tmp___6 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong),
    );
    (*copy).onorm = tmp___6 as *mut libc::c_float;
    memcpy(
        (*copy).inorm as *mut libc::c_void,
        (*o).inorm as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong),
    );
    memcpy(
        (*copy).onorm as *mut libc::c_void,
        (*o).onorm as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong),
    );
    return copy;
}
pub unsafe extern "C" fn NRTransferWeights(
    mut ctx: *mut RedisModuleCtx,
    mut dst: *mut NRTypeObject,
    mut src: *mut NRTypeObject,
) {
    let mut ilen: libc::c_int = 0;
    let mut olen: libc::c_int = 0;
    if (*dst).id != (*src).id {
        (Some(RedisModule_Log.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"warning\0" as *const u8 as *const libc::c_char,
            b"NSTransferWeight(): source and destination neural network IDs don't match. This is unexpected, probably a bug inside the module. Weights not transferred back to the origina NN.\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    AnnFree((*dst).nn);
    (*dst).nn = AnnClone((*src).nn);
    (*dst).training_total_steps = (*src).training_total_steps;
    (*dst).training_total_ms = (*src).training_total_ms;
    (*dst).dataset_error = (*src).dataset_error;
    (*dst).test_error = (*src).test_error;
    (*dst).test_class_error = (*src).test_class_error;
    (*dst).flags
        |= (*src).flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
    ilen = (*((*(*src).nn).layer)
        .offset(((*(*src).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    olen = (*((*(*src).nn).layer).offset(0 as libc::c_int as isize)).units;
    memcpy(
        (*dst).inorm as *mut libc::c_void,
        (*src).inorm as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong),
    );
    memcpy(
        (*dst).onorm as *mut libc::c_void,
        (*src).onorm as *const libc::c_void,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong),
    );
}
pub unsafe extern "C" fn NRTrainingThreadMain(
    mut arg: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut pt: *mut NRPendingTraining = 0 as *mut NRPendingTraining;
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut training_iterations: libc::c_int = 0;
    let mut train_error: libc::c_float = 0.;
    let mut test_error: libc::c_float = 0.;
    let mut class_error: libc::c_float = 0.;
    let mut past_train_error: libc::c_float = 0.;
    let mut past_test_error: libc::c_float = 0.;
    let mut auto_stop: libc::c_int = 0;
    let mut backtrack: libc::c_int = 0;
    let mut cycles: uint64_t = 0;
    let mut start: libc::c_longlong = 0;
    let mut tmp: libc::c_longlong = 0;
    let mut cycle_time: libc::c_longlong = 0;
    let mut overfitting_count: libc::c_int = 0;
    let mut overfitting_limit: libc::c_int = 0;
    let mut best_test_error: libc::c_float = 0.;
    let mut ilen: libc::c_int = 0;
    let mut olen: libc::c_int = 0;
    let mut imax: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut omax: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut inputs: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut outputs: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut j: uint32_t = 0;
    let mut i___1: libc::c_int = 0;
    let mut tmp___0: libc::c_double = 0.;
    let mut tmp___1: libc::c_double = 0.;
    let mut i___2: libc::c_int = 0;
    let mut tmp___2: libc::c_double = 0.;
    let mut tmp___3: libc::c_double = 0.;
    let mut i___3: libc::c_int = 0;
    let mut i___4: libc::c_int = 0;
    let mut j___0: uint32_t = 0;
    let mut i___5: libc::c_int = 0;
    let mut i___6: libc::c_int = 0;
    let mut j___1: uint32_t = 0;
    let mut i___7: libc::c_int = 0;
    let mut i___8: libc::c_int = 0;
    let mut saved: *mut Ann = 0 as *mut Ann;
    let mut saved_error: libc::c_float = 0.;
    let mut saved_train_error: libc::c_float = 0.;
    let mut saved_class_error: libc::c_float = 0.;
    let mut cycle_start: libc::c_longlong = 0;
    let mut tmp___4: libc::c_longlong = 0;
    let mut tmp___5: libc::c_longlong = 0;
    let mut total_time: libc::c_longlong = 0;
    let mut tmp___6: libc::c_longlong = 0;
    let mut tmp___7: libc::c_longlong = 0;
    pt = arg as *mut NRPendingTraining;
    nr = (*pt).nr;
    training_iterations = 1 as libc::c_int;
    train_error = 0 as libc::c_int as libc::c_float;
    test_error = 0 as libc::c_int as libc::c_float;
    class_error = 0 as libc::c_int as libc::c_float;
    past_train_error = (1.0f64 / 0.0f64) as libc::c_float;
    past_test_error = (1.0f64 / 0.0f64) as libc::c_float;
    auto_stop = ((*nr).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint)
        as libc::c_int;
    backtrack = ((*nr).flags & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint)
        as libc::c_int;
    cycles = 0 as libc::c_int as uint64_t;
    tmp = NRMilliseconds();
    start = tmp;
    overfitting_count = 0 as libc::c_int;
    overfitting_limit = 5 as libc::c_int;
    best_test_error = (1.0f64 / 0.0f64) as libc::c_float;
    (*nr).flags &= !((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
    if (*nr).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
        if (*nr).dataset.len != 0 {
            ilen = (*((*(*nr).nn).layer)
                .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
                .units - 1 as libc::c_int;
            olen = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize)).units;
            imax = (*nr).inorm;
            omax = (*nr).onorm;
            inputs = (*nr).dataset.inputs;
            outputs = (*nr).dataset.outputs;
            i = 0 as libc::c_int;
            while i < ilen {
                *imax.offset(i as isize) = 1 as libc::c_int as libc::c_float;
                i += 1;
            }
            i___0 = 0 as libc::c_int;
            while i___0 < olen {
                *omax.offset(i___0 as isize) = 1 as libc::c_int as libc::c_float;
                i___0 += 1;
            }
            j = 0 as libc::c_int as uint32_t;
            while j < (*nr).dataset.len {
                i___1 = 0 as libc::c_int;
                while i___1 < ilen {
                    tmp___1 = fabs(*inputs.offset(i___1 as isize) as libc::c_double);
                    if tmp___1 > *imax.offset(i___1 as isize) as libc::c_double {
                        tmp___0 = fabs(*inputs.offset(i___1 as isize) as libc::c_double);
                        *imax.offset(i___1 as isize) = tmp___0 as libc::c_float;
                    }
                    i___1 += 1;
                }
                i___2 = 0 as libc::c_int;
                while i___2 < olen {
                    tmp___3 = fabs(*outputs.offset(i___2 as isize) as libc::c_double);
                    if tmp___3 > *omax.offset(i___2 as isize) as libc::c_double {
                        tmp___2 = fabs(
                            *outputs.offset(i___2 as isize) as libc::c_double,
                        );
                        *omax.offset(i___2 as isize) = tmp___2 as libc::c_float;
                    }
                    i___2 += 1;
                }
                inputs = inputs.offset(ilen as isize);
                outputs = outputs.offset(olen as isize);
                j = j.wrapping_add(1);
            }
            i___3 = 0 as libc::c_int;
            while i___3 < ilen {
                if *imax.offset(i___3 as isize) != 1 as libc::c_int as libc::c_float {
                    *imax
                        .offset(
                            i___3 as isize,
                        ) = (*imax.offset(i___3 as isize) as libc::c_double * 1.2f64)
                        as libc::c_float;
                }
                i___3 += 1;
            }
            i___4 = 0 as libc::c_int;
            while i___4 < olen {
                if *omax.offset(i___4 as isize) != 1 as libc::c_int as libc::c_float {
                    *omax
                        .offset(
                            i___4 as isize,
                        ) = (*omax.offset(i___4 as isize) as libc::c_double * 1.2f64)
                        as libc::c_float;
                }
                i___4 += 1;
            }
            inputs = (*nr).dataset.inputs;
            outputs = (*nr).dataset.outputs;
            j___0 = 0 as libc::c_int as uint32_t;
            while j___0 < (*nr).dataset.len {
                i___5 = 0 as libc::c_int;
                while i___5 < ilen {
                    *inputs.offset(i___5 as isize)
                        /= *((*nr).inorm).offset(i___5 as isize);
                    i___5 += 1;
                }
                if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
                    == 0
                {
                    i___6 = 0 as libc::c_int;
                    while i___6 < olen {
                        *outputs.offset(i___6 as isize)
                            /= *((*nr).onorm).offset(i___6 as isize);
                        i___6 += 1;
                    }
                }
                inputs = inputs.offset(ilen as isize);
                outputs = outputs.offset(olen as isize);
                j___0 = j___0.wrapping_add(1);
            }
            inputs = (*nr).test.inputs;
            outputs = (*nr).test.outputs;
            j___1 = 0 as libc::c_int as uint32_t;
            while j___1 < (*nr).test.len {
                i___7 = 0 as libc::c_int;
                while i___7 < ilen {
                    *inputs.offset(i___7 as isize)
                        /= *((*nr).inorm).offset(i___7 as isize);
                    i___7 += 1;
                }
                if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
                    == 0
                {
                    i___8 = 0 as libc::c_int;
                    while i___8 < olen {
                        *outputs.offset(i___8 as isize)
                            /= *((*nr).onorm).offset(i___8 as isize);
                        i___8 += 1;
                    }
                }
                inputs = inputs.offset(ilen as isize);
                outputs = outputs.offset(olen as isize);
                j___1 = j___1.wrapping_add(1);
            }
        }
    }
    saved = 0 as *mut libc::c_void as *mut Ann;
    let mut current_block_159: u64;
    loop {
        tmp___4 = NRMilliseconds();
        cycle_start = tmp___4;
        train_error = AnnTrain(
            (*nr).nn,
            (*nr).dataset.inputs,
            (*nr).dataset.outputs,
            0 as libc::c_int as libc::c_float,
            training_iterations,
            (*nr).dataset.len as libc::c_int,
            0 as libc::c_int,
        );
        tmp___5 = NRMilliseconds();
        cycle_time = tmp___5 - cycle_start;
        (*nr)
            .training_total_steps = ((*nr).training_total_steps as libc::c_ulong)
            .wrapping_add(
                ((*nr).dataset.len).wrapping_mul(training_iterations as uint32_t)
                    as uint64_t,
            ) as uint64_t as uint64_t;
        if auto_stop != 0 {
            AnnTestError(
                (*nr).nn,
                (*nr).test.inputs,
                (*nr).test.outputs,
                (*nr).test.len as libc::c_int,
                &mut test_error,
                &mut class_error,
            );
            if train_error < past_train_error {
                if test_error > past_test_error {
                    overfitting_count += 1;
                    if overfitting_count == overfitting_limit {
                        (*nr).flags
                            |= ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
                        break;
                    } else {
                        current_block_159 = 14648249180243006330;
                    }
                } else {
                    current_block_159 = 9966943614572473686;
                }
            } else {
                current_block_159 = 9966943614572473686;
            }
            match current_block_159 {
                9966943614572473686 => {
                    if overfitting_count > 0 as libc::c_int {
                        overfitting_count -= 1;
                    }
                }
                _ => {}
            }
            if backtrack != 0 {
                let mut current_block_134: u64;
                if saved as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                    current_block_134 = 7423896114345324554;
                } else if test_error < saved_error {
                    current_block_134 = 7423896114345324554;
                } else {
                    current_block_134 = 17336970397495664729;
                }
                match current_block_134 {
                    7423896114345324554 => {
                        saved_error = test_error;
                        saved_train_error = train_error;
                        saved_class_error = class_error;
                        if !saved.is_null() {
                            AnnFree(saved);
                        }
                        saved = AnnClone((*nr).nn);
                    }
                    _ => {}
                }
            }
            if test_error < best_test_error {
                overfitting_count = 0 as libc::c_int;
                best_test_error = test_error;
            }
            if (train_error as libc::c_double) < 0.000000000000001f64 {
                if (test_error as libc::c_double) < 0.000000000000001f64 {
                    break;
                }
            }
        }
        cycles = cycles.wrapping_add(1);
        tmp___6 = NRMilliseconds();
        total_time = tmp___6 - start;
        if (*nr).training_max_cycles != 0 {
            if cycles == (*nr).training_max_cycles {
                break;
            }
        }
        if (*nr).training_max_ms != 0 {
            if total_time > (*nr).training_max_ms as libc::c_longlong {
                break;
            }
        }
        if total_time > 10000 as libc::c_longlong {
            if cycle_time < 100 as libc::c_longlong {
                training_iterations += 1;
            }
        }
        past_train_error = train_error;
        past_test_error = test_error;
        pthread_mutex_lock(&mut NRPendingTrainingMutex);
        (*pt).dataset_error = train_error;
        (*pt).test_error = test_error;
        if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
            (*pt).class_error = class_error;
        }
        (*pt).curcycle = cycles as libc::c_int;
        pthread_mutex_unlock(&mut NRPendingTrainingMutex);
    }
    if auto_stop == 0 {
        AnnTestError(
            (*nr).nn,
            (*nr).test.inputs,
            (*nr).test.outputs,
            (*nr).test.len as libc::c_int,
            &mut test_error,
            &mut class_error,
        );
    }
    if auto_stop != 0 {
        if backtrack != 0 {
            let mut current_block_172: u64;
            if !saved.is_null() {
                if saved_error < test_error {
                    AnnFree((*nr).nn);
                    (*nr).nn = saved;
                    test_error = saved_error;
                    train_error = saved_train_error;
                    class_error = saved_class_error;
                    current_block_172 = 14217554310789445237;
                } else {
                    current_block_172 = 1877225584336526460;
                }
            } else {
                current_block_172 = 1877225584336526460;
            }
            match current_block_172 {
                1877225584336526460 => {
                    if !saved.is_null() {
                        AnnFree(saved);
                    }
                }
                _ => {}
            }
        }
    }
    if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        (*nr).test_class_error = class_error;
    }
    (*nr).dataset_error = train_error;
    (*nr).test_error = test_error;
    tmp___7 = NRMilliseconds();
    (*nr)
        .training_total_ms = ((*nr).training_total_ms as libc::c_ulonglong)
        .wrapping_add((tmp___7 - start) as libc::c_ulonglong) as uint64_t;
    pthread_mutex_lock(&mut NRPendingTrainingMutex);
    (*pt).in_progress = 0 as libc::c_int;
    pthread_mutex_unlock(&mut NRPendingTrainingMutex);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn NRStartTraining(
    mut ctx: *mut RedisModuleCtx,
    mut key: *mut RedisModuleString,
    mut dbid: libc::c_int,
    mut nr: *mut NRTypeObject,
) -> libc::c_int {
    let mut pt: *mut NRPendingTraining = 0 as *mut NRPendingTraining;
    let mut tmp: libc::c_int = 0;
    pthread_mutex_lock(&mut NRPendingTrainingMutex);
    if NRPendingTrainingCount == 32 as libc::c_int {
        pthread_mutex_unlock(&mut NRPendingTrainingMutex);
        return 1 as libc::c_int;
    }
    pt = &mut *NRTrainings.as_mut_ptr().offset(NRPendingTrainingCount as isize)
        as *mut NRPendingTraining;
    (*pt)
        .key = (Some(
        RedisModule_CreateStringFromString.expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(ctx, key as *const RedisModuleString);
    (Some(RedisModule_RetainString.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, (*pt).key);
    (*pt).db_id = dbid;
    (*pt).in_progress = 1 as libc::c_int;
    (*pt).nr = NRClone(nr, 0 as libc::c_int);
    (*pt).dataset_error = 0 as libc::c_int as libc::c_float;
    (*pt).test_error = 0 as libc::c_int as libc::c_float;
    (*pt).class_error = 0 as libc::c_int as libc::c_float;
    (*pt).curcycle = 0 as libc::c_int;
    tmp = pthread_create(
        &mut (*pt).tid as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(
            NRTrainingThreadMain
                as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        ),
        pt as *mut libc::c_void,
    );
    if tmp != 0 as libc::c_int {
        (Some(RedisModule_Log.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"warning\0" as *const u8 as *const libc::c_char,
            b"Unable to create a new pthread in NRStartTraining()\0" as *const u8
                as *const libc::c_char,
        );
        (Some(RedisModule_FreeString.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx, (*pt).key);
        (*pt).key = 0 as *mut libc::c_void as *mut RedisModuleString;
        NRTypeReleaseObject((*pt).nr);
        pthread_mutex_unlock(&mut NRPendingTrainingMutex);
        return 1 as libc::c_int;
    }
    NRPendingTrainingCount += 1;
    (*nr).flags |= 1 as libc::c_uint;
    (*nr).flags &= !((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint;
    pthread_mutex_unlock(&mut NRPendingTrainingMutex);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRCollectThreads(mut ctx: *mut RedisModuleCtx) -> libc::c_int {
    let mut collected: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pt: *mut NRPendingTraining = 0 as *mut NRPendingTraining;
    let mut orig_id: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut key: *mut RedisModuleKey = 0 as *mut RedisModuleKey;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut RedisModuleType = 0 as *mut RedisModuleType;
    collected = 0 as libc::c_int;
    pthread_mutex_lock(&mut NRPendingTrainingMutex);
    j = 0 as libc::c_int;
    while j < NRPendingTrainingCount {
        pt = &mut *NRTrainings.as_mut_ptr().offset(j as isize) as *mut NRPendingTraining;
        if (*pt).in_progress == 0 as libc::c_int {
            tmp = (Some(RedisModule_GetSelectedDb.expect("non-null function pointer")))
                .expect("non-null function pointer")(ctx);
            orig_id = tmp;
            if orig_id != (*pt).db_id {
                (Some(RedisModule_SelectDb.expect("non-null function pointer")))
                    .expect("non-null function pointer")(ctx, (*pt).db_id);
            }
            tmp___0 = (Some(RedisModule_OpenKey.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                (*pt).key,
                1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
            );
            key = tmp___0 as *mut RedisModuleKey;
            tmp___2 = (Some(
                RedisModule_ModuleTypeGetType.expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(key);
            if tmp___2 as libc::c_ulong == NRType as libc::c_ulong {
                tmp___1 = (Some(
                    RedisModule_ModuleTypeGetValue.expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(key);
                nr = tmp___1 as *mut NRTypeObject;
                if (*nr).id == (*(*pt).nr).id {
                    NRTransferWeights(ctx, nr, (*pt).nr);
                    (*nr).flags &= 4294967294 as libc::c_uint;
                }
                (Some(RedisModule_FreeString.expect("non-null function pointer")))
                    .expect("non-null function pointer")(ctx, (*pt).key);
                (*pt).key = 0 as *mut libc::c_void as *mut RedisModuleString;
                NRTypeReleaseObject((*pt).nr);
                NRPendingTrainingCount -= 1;
                memcpy(
                    &mut *NRTrainings.as_mut_ptr().offset(j as isize)
                        as *mut NRPendingTraining as *mut libc::c_void,
                    &mut *NRTrainings
                        .as_mut_ptr()
                        .offset((j + 1 as libc::c_int) as isize)
                        as *mut NRPendingTraining as *const libc::c_void,
                    ((NRPendingTrainingCount - j) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<NRPendingTraining>() as libc::c_ulong,
                        ),
                );
            }
            if orig_id != (*pt).db_id {
                (Some(RedisModule_SelectDb.expect("non-null function pointer")))
                    .expect("non-null function pointer")(ctx, orig_id);
            }
            collected += 1;
        }
        j += 1;
    }
    pthread_mutex_unlock(&mut NRPendingTrainingMutex);
    return collected;
}
pub unsafe extern "C" fn NRCreate_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut dset_size: libc::c_longlong = 0;
    let mut test_size: libc::c_longlong = 0;
    let mut layers: [libc::c_int; 32] = [0; 32];
    let mut num_layers: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut nntype: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut stop: libc::c_int = 0;
    let mut u: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut units: libc::c_longlong = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut o: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___9: *const libc::c_char = 0 as *const libc::c_char;
    let mut v: libc::c_longlong = 0;
    let mut lastarg: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut tmp___13: libc::c_int = 0;
    let mut tmp___14: libc::c_int = 0;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut key: *mut RedisModuleKey = 0 as *mut RedisModuleKey;
    let mut tmp___17: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut type_0: libc::c_int = 0;
    let mut tmp___18: libc::c_int = 0;
    let mut tmp___19: libc::c_int = 0;
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp___20: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp___21: size_t = 0;
    dset_size = 0 as libc::c_longlong;
    test_size = 0 as libc::c_longlong;
    num_layers = 0 as libc::c_int;
    flags = 0 as libc::c_int;
    (Some(RedisModule_AutoMemory.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx);
    NRCollectThreads(ctx);
    if argc < 6 as libc::c_int {
        tmp = (Some(RedisModule_WrongArity.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx);
        return tmp;
    }
    tmp___0 = (Some(RedisModule_StringPtrLen.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        *argv.offset(2 as libc::c_int as isize) as *const RedisModuleString,
        0 as *mut libc::c_void as *mut size_t,
    );
    nntype = tmp___0;
    tmp___3 = strcasecmp(nntype, b"classifier\0" as *const u8 as *const libc::c_char);
    if tmp___3 != 0 {
        tmp___2 = strcasecmp(nntype, b"regressor\0" as *const u8 as *const libc::c_char);
        if tmp___2 != 0 {
            tmp___1 = (Some(
                RedisModule_ReplyWithError.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                b"ERR invalid neural network type. Must be CLASSIFIER or REGRESSOR\0"
                    as *const u8 as *const libc::c_char,
            );
            return tmp___1;
        } else {
            flags |= (1 as libc::c_int) << 1 as libc::c_int;
        }
    } else {
        flags |= (1 as libc::c_int) << 2 as libc::c_int;
    }
    j = 3 as libc::c_int;
    stop = 0 as libc::c_int;
    while j < argc {
        tmp___4 = (Some(RedisModule_StringPtrLen.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            *argv.offset(j as isize) as *const RedisModuleString,
            0 as *mut libc::c_void as *mut size_t,
        );
        u = tmp___4;
        tmp___5 = strcmp(u, b"->\0" as *const u8 as *const libc::c_char);
        if tmp___5 == 0 {
            stop = 1 as libc::c_int;
            j += 1;
        } else {
            tmp___7 = (Some(
                RedisModule_StringToLongLong.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(*argv.offset(j as isize) as *const RedisModuleString, &mut units);
            if tmp___7 != 0 as libc::c_int {
                tmp___6 = (Some(
                    RedisModule_ReplyWithError.expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    ctx,
                    b"ERR invalid units count\0" as *const u8 as *const libc::c_char,
                );
                return tmp___6;
            } else {
                if units <= 0 as libc::c_longlong {
                    tmp___6 = (Some(
                        RedisModule_ReplyWithError.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        ctx,
                        b"ERR invalid units count\0" as *const u8 as *const libc::c_char,
                    );
                    return tmp___6;
                }
            }
            tmp___8 = num_layers;
            num_layers += 1;
            layers[tmp___8 as usize] = units as libc::c_int;
            j += 1;
            if stop != 0 {
                break;
            }
        }
    }
    i = 0 as libc::c_int;
    while i < num_layers / 2 as libc::c_int {
        t = layers[i as usize];
        layers[i as usize] = layers[(num_layers - 1 as libc::c_int - i) as usize];
        layers[(num_layers - 1 as libc::c_int - i) as usize] = t;
        i += 1;
    }
    while j < argc {
        tmp___9 = (Some(RedisModule_StringPtrLen.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            *argv.offset(j as isize) as *const RedisModuleString,
            0 as *mut libc::c_void as *mut size_t,
        );
        o = tmp___9;
        lastarg = (j == argc - 1 as libc::c_int) as libc::c_int;
        tmp___15 = strcasecmp(o, b"dataset\0" as *const u8 as *const libc::c_char);
        let mut current_block_80: u64;
        if tmp___15 != 0 {
            tmp___16 = strcasecmp(o, b"test\0" as *const u8 as *const libc::c_char);
            if tmp___16 != 0 {
                current_block_80 = 695661582440953144;
            } else {
                current_block_80 = 3211957994008561118;
            }
        } else {
            current_block_80 = 3211957994008561118;
        }
        match current_block_80 {
            3211957994008561118 => {
                if lastarg == 0 {
                    tmp___11 = (Some(
                        RedisModule_StringToLongLong.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        *argv.offset((j + 1 as libc::c_int) as isize)
                            as *const RedisModuleString,
                        &mut v,
                    );
                    if tmp___11 != 0 as libc::c_int {
                        tmp___10 = (Some(
                            RedisModule_ReplyWithError
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            ctx,
                            b"ERR invalid dataset size\0" as *const u8
                                as *const libc::c_char,
                        );
                        return tmp___10;
                    } else {
                        if v < 0 as libc::c_longlong {
                            tmp___10 = (Some(
                                RedisModule_ReplyWithError
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                ctx,
                                b"ERR invalid dataset size\0" as *const u8
                                    as *const libc::c_char,
                            );
                            return tmp___10;
                        }
                    }
                    tmp___12 = strcasecmp(
                        o,
                        b"dataset\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp___12 != 0 {
                        test_size = v;
                    } else {
                        dset_size = v;
                    }
                    j += 1;
                    current_block_80 = 2472048668343472511;
                } else {
                    current_block_80 = 695661582440953144;
                }
            }
            _ => {}
        }
        match current_block_80 {
            695661582440953144 => {
                tmp___14 = strcasecmp(
                    o,
                    b"normalize\0" as *const u8 as *const libc::c_char,
                );
                if tmp___14 != 0 {
                    tmp___13 = (Some(
                        RedisModule_ReplyWithError.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        ctx,
                        b"ERR Syntax error in NR.CREATE\0" as *const u8
                            as *const libc::c_char,
                    );
                    return tmp___13;
                } else {
                    flags |= (1 as libc::c_int) << 3 as libc::c_int;
                }
            }
            _ => {}
        }
        j += 1;
    }
    tmp___17 = (Some(RedisModule_OpenKey.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
    );
    key = tmp___17 as *mut RedisModuleKey;
    tmp___18 = (Some(RedisModule_KeyType.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    type_0 = tmp___18;
    if type_0 != 0 as libc::c_int {
        tmp___19 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(ctx, b"ERR the key name is busy\0" as *const u8 as *const libc::c_char);
        return tmp___19;
    }
    tmp___20 = createNRTypeObject(
        flags,
        layers.as_mut_ptr(),
        num_layers,
        dset_size as libc::c_int,
        test_size as libc::c_int,
    );
    nr = tmp___20;
    (Some(RedisModule_ModuleTypeSetValue.expect("non-null function pointer")))
        .expect("non-null function pointer")(key, NRType, nr as *mut libc::c_void);
    tmp___21 = AnnCountWeights((*nr).nn);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, tmp___21 as libc::c_longlong);
    (Some(RedisModule_ReplicateVerbatim.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRGenericRun_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
    mut output_class: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut key: *mut RedisModuleKey = 0 as *mut RedisModuleKey;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut RedisModuleType = 0 as *mut RedisModuleType;
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: libc::c_int = 0;
    let mut ilen: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut input: libc::c_double = 0.;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut olen: libc::c_int = 0;
    let mut max: libc::c_float = 0.;
    let mut max_class: libc::c_int = 0;
    let mut j___0: libc::c_int = 0;
    let mut output: libc::c_float = 0.;
    let mut j___1: libc::c_int = 0;
    let mut output___0: libc::c_float = 0.;
    (Some(RedisModule_AutoMemory.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx);
    NRCollectThreads(ctx);
    if argc < 3 as libc::c_int {
        tmp = (Some(RedisModule_WrongArity.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx);
        return tmp;
    }
    tmp___0 = (Some(RedisModule_OpenKey.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, *argv.offset(1 as libc::c_int as isize), 1 as libc::c_int);
    key = tmp___0 as *mut RedisModuleKey;
    tmp___2 = (Some(RedisModule_ModuleTypeGetType.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    if tmp___2 as libc::c_ulong != NRType as libc::c_ulong {
        tmp___1 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___1;
    }
    tmp___3 = (Some(RedisModule_ModuleTypeGetValue.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    nr = tmp___3 as *mut NRTypeObject;
    if output_class != 0 {
        if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint == 0 {
            tmp___4 = (Some(
                RedisModule_ReplyWithError.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                b"ERR you can't call NR.CLASS with a regressor network. Use this command with a classifier network\0"
                    as *const u8 as *const libc::c_char,
            );
            return tmp___4;
        }
    }
    ilen = (*((*(*nr).nn).layer)
        .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    if argc != ilen + 2 as libc::c_int {
        tmp___5 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"ERR number of arguments does not match the number of inputs in the neural network\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___5;
    }
    j = 0 as libc::c_int;
    while j < ilen {
        tmp___7 = (Some(RedisModule_StringToDouble.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            *argv.offset((j + 2 as libc::c_int) as isize) as *const RedisModuleString,
            &mut input,
        );
        if tmp___7 != 0 as libc::c_int {
            tmp___6 = (Some(
                RedisModule_ReplyWithError.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                b"ERR invalid neural network input: must be a valid float precision floating point number\0"
                    as *const u8 as *const libc::c_char,
            );
            return tmp___6;
        }
        if (*nr).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0 {
            input /= *((*nr).inorm).offset(j as isize) as libc::c_double;
        }
        *((*((*(*nr).nn).layer).offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
            .output)
            .offset(j as isize) = input as libc::c_float;
        j += 1;
    }
    AnnSimulate((*nr).nn);
    olen = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize)).units;
    if output_class != 0 {
        max = *((*((*(*nr).nn).layer).offset(0 as libc::c_int as isize)).output)
            .offset(0 as libc::c_int as isize);
        max_class = 0 as libc::c_int;
        j___0 = 1 as libc::c_int;
        while j___0 < olen {
            output = *((*((*(*nr).nn).layer).offset(0 as libc::c_int as isize)).output)
                .offset(j___0 as isize);
            if output > max {
                max = output;
                max_class = j___0;
            }
            j___0 += 1;
        }
        (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx, max_class as libc::c_longlong);
    } else {
        (Some(RedisModule_ReplyWithArray.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx, olen as libc::c_long);
        j___1 = 0 as libc::c_int;
        while j___1 < olen {
            output___0 = *((*((*(*nr).nn).layer).offset(0 as libc::c_int as isize))
                .output)
                .offset(j___1 as isize);
            if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint
                == 0
            {
                if (*nr).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint
                    != 0
                {
                    output___0 *= *((*nr).onorm).offset(j___1 as isize);
                }
            }
            (Some(RedisModule_ReplyWithDouble.expect("non-null function pointer")))
                .expect("non-null function pointer")(ctx, output___0 as libc::c_double);
            j___1 += 1;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRRun_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = NRGenericRun_RedisCommand(ctx, argv, argc, 0 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn NRClass_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = NRGenericRun_RedisCommand(ctx, argv, argc, 1 as libc::c_int);
    return tmp;
}
pub unsafe extern "C" fn NRObserve_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut key: *mut RedisModuleKey = 0 as *mut RedisModuleKey;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut RedisModuleType = 0 as *mut RedisModuleType;
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ilen: libc::c_int = 0;
    let mut olen: libc::c_int = 0;
    let mut oargs: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut target: libc::c_int = 0;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut sep: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___10: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    let mut inputs: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut tmp___13: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut outputs: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut tmp___14: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut j: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    let mut tmp___15: libc::c_int = 0;
    let mut tmp___16: libc::c_int = 0;
    let mut classid: libc::c_int = 0;
    let mut tmp___17: libc::c_int = 0;
    (Some(RedisModule_AutoMemory.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx);
    NRCollectThreads(ctx);
    if argc < 3 as libc::c_int {
        tmp = (Some(RedisModule_WrongArity.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx);
        return tmp;
    }
    tmp___0 = (Some(RedisModule_OpenKey.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
    );
    key = tmp___0 as *mut RedisModuleKey;
    tmp___2 = (Some(RedisModule_ModuleTypeGetType.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    if tmp___2 as libc::c_ulong != NRType as libc::c_ulong {
        tmp___1 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___1;
    }
    tmp___3 = (Some(RedisModule_ModuleTypeGetValue.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    nr = tmp___3 as *mut NRTypeObject;
    ilen = (*((*(*nr).nn).layer)
        .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    olen = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize)).units;
    if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        tmp___4 = 1 as libc::c_int;
    } else {
        tmp___4 = olen;
    }
    oargs = tmp___4;
    target = 0 as libc::c_int;
    tmp___7 = (Some(RedisModule_StringPtrLen.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        *argv.offset((argc - 1 as libc::c_int) as isize) as *const RedisModuleString,
        0 as *mut libc::c_void as *mut size_t,
    );
    tmp___8 = strcasecmp(tmp___7, b"train\0" as *const u8 as *const libc::c_char);
    if tmp___8 != 0 {
        tmp___5 = (Some(RedisModule_StringPtrLen.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            *argv.offset((argc - 1 as libc::c_int) as isize) as *const RedisModuleString,
            0 as *mut libc::c_void as *mut size_t,
        );
        tmp___6 = strcasecmp(tmp___5, b"test\0" as *const u8 as *const libc::c_char);
        if tmp___6 == 0 {
            target = 2 as libc::c_int;
            argc -= 1;
        }
    } else {
        target = 1 as libc::c_int;
        argc -= 1;
    }
    if argc != oargs + ilen + 3 as libc::c_int {
        tmp___9 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"ERR number of arguments does not match the number of inputs and outputs in the neural network\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___9;
    }
    tmp___10 = (Some(RedisModule_StringPtrLen.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        *argv.offset((ilen + 2 as libc::c_int) as isize) as *const RedisModuleString,
        0 as *mut libc::c_void as *mut size_t,
    );
    sep = tmp___10;
    tmp___12 = strcmp(sep, b"->\0" as *const u8 as *const libc::c_char);
    if tmp___12 != 0 {
        tmp___11 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"ERR no '->' separtor in the correct position between inputs and outputs: are you sure your training data is correct?\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___11;
    }
    tmp___13 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(ilen as libc::c_ulong),
    );
    inputs = tmp___13 as *mut libc::c_float;
    tmp___14 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(olen as libc::c_ulong),
    );
    outputs = tmp___14 as *mut libc::c_float;
    j = 2 as libc::c_int;
    while j < argc {
        if !(j == ilen + 2 as libc::c_int) {
            tmp___16 = (Some(
                RedisModule_StringToDouble.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(*argv.offset(j as isize) as *const RedisModuleString, &mut val);
            if tmp___16 != 0 as libc::c_int {
                (Some(RedisModule_Free.expect("non-null function pointer")))
                    .expect("non-null function pointer")(inputs as *mut libc::c_void);
                (Some(RedisModule_Free.expect("non-null function pointer")))
                    .expect("non-null function pointer")(outputs as *mut libc::c_void);
                tmp___15 = (Some(
                    RedisModule_ReplyWithError.expect("non-null function pointer"),
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    ctx,
                    b"ERR invalid neural network input: must be a valid float precision floating point number\0"
                        as *const u8 as *const libc::c_char,
                );
                return tmp___15;
            }
            if j < ilen + 2 as libc::c_int {
                *inputs.offset((j - 2 as libc::c_int) as isize) = val as libc::c_float;
            } else if (*nr).flags
                    & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
                {
                classid = val as libc::c_int;
                if classid as libc::c_double != val {
                    (Some(RedisModule_Free.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(inputs as *mut libc::c_void);
                    (Some(RedisModule_Free.expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(outputs as *mut libc::c_void);
                    tmp___17 = (Some(
                        RedisModule_ReplyWithError.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        ctx,
                        b"ERR classifier network output must be an integer in the range from 0 to outputs-1.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return tmp___17;
                } else {
                    if val >= olen as libc::c_double {
                        (Some(RedisModule_Free.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(inputs as *mut libc::c_void);
                        (Some(RedisModule_Free.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(outputs as *mut libc::c_void);
                        tmp___17 = (Some(
                            RedisModule_ReplyWithError
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            ctx,
                            b"ERR classifier network output must be an integer in the range from 0 to outputs-1.\0"
                                as *const u8 as *const libc::c_char,
                        );
                        return tmp___17;
                    } else {
                        if val < 0 as libc::c_int as libc::c_double {
                            (Some(RedisModule_Free.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(inputs as *mut libc::c_void);
                            (Some(RedisModule_Free.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(outputs as *mut libc::c_void);
                            tmp___17 = (Some(
                                RedisModule_ReplyWithError
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                ctx,
                                b"ERR classifier network output must be an integer in the range from 0 to outputs-1.\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            return tmp___17;
                        }
                    }
                }
                memset(
                    outputs as *mut libc::c_void,
                    0 as libc::c_int,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(olen as libc::c_ulong),
                );
                *outputs.offset(classid as isize) = 1 as libc::c_int as libc::c_float;
            } else {
                *outputs
                    .offset(
                        (j - ilen - 3 as libc::c_int) as isize,
                    ) = val as libc::c_float;
            }
        }
        j += 1;
    }
    NRTypeInsertData(nr, inputs, outputs, target);
    (Some(RedisModule_Free.expect("non-null function pointer")))
        .expect("non-null function pointer")(inputs as *mut libc::c_void);
    (Some(RedisModule_Free.expect("non-null function pointer")))
        .expect("non-null function pointer")(outputs as *mut libc::c_void);
    (Some(RedisModule_ReplyWithArray.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, 2 as libc::c_long);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, (*nr).dataset.len as libc::c_longlong);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, (*nr).test.len as libc::c_longlong);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRTrain_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut key: *mut RedisModuleKey = 0 as *mut RedisModuleKey;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut RedisModuleType = 0 as *mut RedisModuleType;
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut o: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut v: libc::c_longlong = 0;
    let mut lastarg: libc::c_int = 0;
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
    (Some(RedisModule_AutoMemory.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx);
    NRCollectThreads(ctx);
    if argc < 2 as libc::c_int {
        tmp = (Some(RedisModule_WrongArity.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx);
        return tmp;
    }
    tmp___0 = (Some(RedisModule_OpenKey.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
    );
    key = tmp___0 as *mut RedisModuleKey;
    tmp___2 = (Some(RedisModule_ModuleTypeGetType.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    if tmp___2 as libc::c_ulong != NRType as libc::c_ulong {
        tmp___1 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___1;
    }
    tmp___3 = (Some(RedisModule_ModuleTypeGetValue.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    nr = tmp___3 as *mut NRTypeObject;
    if (*nr).flags & 1 as libc::c_uint != 0 {
        tmp___4 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"ERR neural network training already in progress\0" as *const u8
                as *const libc::c_char,
        );
        return tmp___4;
    }
    (*nr).training_max_cycles = 0 as libc::c_int as uint64_t;
    (*nr).training_max_ms = 10000 as libc::c_int as uint64_t;
    (*nr).flags
        &= !((1 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
    j = 2 as libc::c_int;
    while j < argc {
        tmp___5 = (Some(RedisModule_StringPtrLen.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            *argv.offset(j as isize) as *const RedisModuleString,
            0 as *mut libc::c_void as *mut size_t,
        );
        o = tmp___5;
        lastarg = (j == argc - 1 as libc::c_int) as libc::c_int;
        tmp___14 = strcasecmp(o, b"autostop\0" as *const u8 as *const libc::c_char);
        if tmp___14 != 0 {
            tmp___13 = strcasecmp(o, b"backtrack\0" as *const u8 as *const libc::c_char);
            if tmp___13 != 0 {
                tmp___12 = strcasecmp(
                    o,
                    b"maxcycles\0" as *const u8 as *const libc::c_char,
                );
                let mut current_block_53: u64;
                if tmp___12 != 0 {
                    current_block_53 = 12390465908509322549;
                } else if lastarg == 0 {
                    j += 1;
                    tmp___7 = (Some(
                        RedisModule_StringToLongLong.expect("non-null function pointer"),
                    ))
                        .expect(
                            "non-null function pointer",
                        )(*argv.offset(j as isize) as *const RedisModuleString, &mut v);
                    if tmp___7 != 0 as libc::c_int {
                        tmp___6 = (Some(
                            RedisModule_ReplyWithError
                                .expect("non-null function pointer"),
                        ))
                            .expect(
                                "non-null function pointer",
                            )(
                            ctx,
                            b"ERR invalid number of cycles\0" as *const u8
                                as *const libc::c_char,
                        );
                        return tmp___6;
                    }
                    (*nr).training_max_cycles = v as uint64_t;
                    current_block_53 = 9627623479216730126;
                } else {
                    current_block_53 = 12390465908509322549;
                }
                match current_block_53 {
                    12390465908509322549 => {
                        tmp___11 = strcasecmp(
                            o,
                            b"maxtime\0" as *const u8 as *const libc::c_char,
                        );
                        if tmp___11 != 0 {
                            tmp___10 = (Some(
                                RedisModule_ReplyWithError
                                    .expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                ctx,
                                b"ERR Syntax error in NR.TRAIN\0" as *const u8
                                    as *const libc::c_char,
                            );
                            return tmp___10;
                        } else {
                            if lastarg == 0 {
                                j += 1;
                                tmp___9 = (Some(
                                    RedisModule_StringToLongLong
                                        .expect("non-null function pointer"),
                                ))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    *argv.offset(j as isize) as *const RedisModuleString,
                                    &mut v,
                                );
                                if tmp___9 != 0 as libc::c_int {
                                    tmp___8 = (Some(
                                        RedisModule_ReplyWithError
                                            .expect("non-null function pointer"),
                                    ))
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        ctx,
                                        b"ERR invalid number of milliseconds of time\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                    return tmp___8;
                                }
                                (*nr).training_max_ms = v as uint64_t;
                            } else {
                                tmp___10 = (Some(
                                    RedisModule_ReplyWithError
                                        .expect("non-null function pointer"),
                                ))
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    ctx,
                                    b"ERR Syntax error in NR.TRAIN\0" as *const u8
                                        as *const libc::c_char,
                                );
                                return tmp___10;
                            }
                        }
                    }
                    _ => {}
                }
            } else {
                (*nr).flags |= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint;
            }
        } else {
            (*nr).flags |= ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint;
        }
        j += 1;
    }
    if (*nr).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint != 0 {
        if (*nr).test.len == 0 as libc::c_uint {
            tmp___15 = (Some(
                RedisModule_ReplyWithError.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                b"ERR Can't start training with AUTOSTOP option: overfitting detection requires a non zero length testing dataset\0"
                    as *const u8 as *const libc::c_char,
            );
            return tmp___15;
        }
    }
    tmp___18 = (Some(RedisModule_GetSelectedDb.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx);
    tmp___19 = NRStartTraining(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        tmp___18,
        nr,
    );
    if tmp___19 == 1 as libc::c_int {
        tmp___16 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"ERR Can't train the neural network: too many NNs already training\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___16;
    } else {
        tmp___17 = (Some(
            RedisModule_ReplyWithSimpleString.expect("non-null function pointer"),
        ))
            .expect(
                "non-null function pointer",
            )(ctx, b"Training has started\0" as *const u8 as *const libc::c_char);
        return tmp___17;
    };
}
pub unsafe extern "C" fn NRReset_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut key: *mut RedisModuleKey = 0 as *mut RedisModuleKey;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut RedisModuleType = 0 as *mut RedisModuleType;
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: uint64_t = 0;
    let mut tmp___5: libc::c_int = 0;
    (Some(RedisModule_AutoMemory.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx);
    NRCollectThreads(ctx);
    if argc != 2 as libc::c_int {
        tmp = (Some(RedisModule_WrongArity.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx);
        return tmp;
    }
    tmp___0 = (Some(RedisModule_OpenKey.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        *argv.offset(1 as libc::c_int as isize),
        1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
    );
    key = tmp___0 as *mut RedisModuleKey;
    tmp___2 = (Some(RedisModule_ModuleTypeGetType.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    if tmp___2 as libc::c_ulong != NRType as libc::c_ulong {
        tmp___1 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___1;
    }
    tmp___3 = (Some(RedisModule_ModuleTypeGetValue.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    nr = tmp___3 as *mut NRTypeObject;
    tmp___4 = NRNextId;
    NRNextId = NRNextId.wrapping_add(1);
    (*nr).id = tmp___4;
    (*nr).training_total_steps = 0 as libc::c_int as uint64_t;
    (*nr).training_total_ms = 0 as libc::c_int as uint64_t;
    (*nr).training_max_cycles = 0 as libc::c_int as uint64_t;
    (*nr).training_max_ms = 0 as libc::c_int as uint64_t;
    (*nr).dataset_error = 0 as libc::c_int as libc::c_float;
    (*nr).test_error = 0 as libc::c_int as libc::c_float;
    (*nr).test_class_error = 0 as libc::c_int as libc::c_float;
    AnnSetRandomWeights((*nr).nn);
    tmp___5 = (Some(
        RedisModule_ReplyWithSimpleString.expect("non-null function pointer"),
    ))
        .expect(
            "non-null function pointer",
        )(ctx, b"OK\0" as *const u8 as *const libc::c_char);
    return tmp___5;
}
pub unsafe extern "C" fn NRInfo_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut tmp: libc::c_int = 0;
    let mut key: *mut RedisModuleKey = 0 as *mut RedisModuleKey;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut RedisModuleType = 0 as *mut RedisModuleType;
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fields: libc::c_int = 0;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut tmp___5: uint64_t = 0;
    let mut tmp___6: *const libc::c_char = 0 as *const libc::c_char;
    (Some(RedisModule_AutoMemory.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx);
    NRCollectThreads(ctx);
    if argc != 2 as libc::c_int {
        tmp = (Some(RedisModule_WrongArity.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx);
        return tmp;
    }
    tmp___0 = (Some(RedisModule_OpenKey.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, *argv.offset(1 as libc::c_int as isize), 1 as libc::c_int);
    key = tmp___0 as *mut RedisModuleKey;
    tmp___2 = (Some(RedisModule_ModuleTypeGetType.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    if tmp___2 as libc::c_ulong != NRType as libc::c_ulong {
        tmp___1 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___1;
    }
    tmp___3 = (Some(RedisModule_ModuleTypeGetValue.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    nr = tmp___3 as *mut NRTypeObject;
    fields = 15 as libc::c_int;
    if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        fields += 1;
    }
    (Some(RedisModule_ReplyWithArray.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, (fields * 2 as libc::c_int) as libc::c_long);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"id\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, (*nr).id as libc::c_longlong);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"type\0" as *const u8 as *const libc::c_char);
    if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        tmp___4 = b"classifier\0" as *const u8 as *const libc::c_char;
    } else {
        tmp___4 = b"regressor\0" as *const u8 as *const libc::c_char;
    }
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, tmp___4);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"auto-normalization\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        ((*nr).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_uint != 0)
            as libc::c_int as libc::c_longlong,
    );
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"training\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        ((*nr).flags & 1 as libc::c_uint != 0) as libc::c_int as libc::c_longlong,
    );
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"layout\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithArray.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, (*(*nr).nn).layers as libc::c_long);
    i = (*(*nr).nn).layers - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        units = (*((*(*nr).nn).layer).offset(i as isize)).units;
        if i != 0 as libc::c_int {
            units -= 1;
        }
        (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx, units as libc::c_longlong);
        i -= 1;
    }
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"training-dataset-maxlen\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, (*nr).dataset.maxlen as libc::c_longlong);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"training-dataset-len\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, (*nr).dataset.len as libc::c_longlong);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"test-dataset-maxlen\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, (*nr).test.maxlen as libc::c_longlong);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"test-dataset-len\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, (*nr).test.len as libc::c_longlong);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"training-total-steps\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, (*nr).training_total_steps as libc::c_longlong);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"training-total-cycles\0" as *const u8 as *const libc::c_char);
    if (*nr).dataset.len != 0 {
        tmp___5 = ((*nr).training_total_steps)
            .wrapping_div((*nr).dataset.len as uint64_t);
    } else {
        tmp___5 = 0 as libc::c_int as uint64_t;
    }
    (Some(RedisModule_ReplyWithLongLong.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, tmp___5 as libc::c_longlong);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"training-total-seconds\0" as *const u8 as *const libc::c_char);
    snprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%.02f\0" as *const u8 as *const libc::c_char,
        ((*nr).training_total_ms as libc::c_float / 1000 as libc::c_int as libc::c_float)
            as libc::c_double,
    );
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, buf.as_mut_ptr() as *const libc::c_char);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"dataset-error\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithDouble.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, (*nr).dataset_error as libc::c_double);
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"test-error\0" as *const u8 as *const libc::c_char);
    (Some(RedisModule_ReplyWithDouble.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, (*nr).test_error as libc::c_double);
    if (*nr).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0 {
        (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(ctx, b"classification-errors-perc\0" as *const u8 as *const libc::c_char);
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"%.02f\0" as *const u8 as *const libc::c_char,
            (*nr).test_class_error as libc::c_double,
        );
        (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(ctx, buf.as_mut_ptr() as *const libc::c_char);
    }
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, b"overfitting-detected\0" as *const u8 as *const libc::c_char);
    if (*nr).flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint != 0 {
        tmp___6 = b"yes\0" as *const u8 as *const libc::c_char;
    } else {
        tmp___6 = b"no\0" as *const u8 as *const libc::c_char;
    }
    (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, tmp___6);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRThreads_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut pt: *mut NRPendingTraining = 0 as *mut NRPendingTraining;
    let mut keyname: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___0: *const libc::c_char = 0 as *const libc::c_char;
    (Some(RedisModule_AutoMemory.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx);
    NRCollectThreads(ctx);
    if argc != 1 as libc::c_int {
        tmp = (Some(RedisModule_WrongArity.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx);
        return tmp;
    }
    pthread_mutex_lock(&mut NRPendingTrainingMutex);
    (Some(RedisModule_ReplyWithArray.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, NRPendingTrainingCount as libc::c_long);
    j = 0 as libc::c_int;
    while j < NRPendingTrainingCount {
        pt = &mut *NRTrainings.as_mut_ptr().offset(j as isize) as *mut NRPendingTraining;
        tmp___0 = (Some(RedisModule_StringPtrLen.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*pt).key as *const RedisModuleString,
            0 as *mut libc::c_void as *mut size_t,
        );
        keyname = tmp___0;
        snprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"nn_id=%llu cycle=%d key=%s db=%d maxtime=%llu maxcycles=%llu trainerr=%f testerr=%f classerr=%f\0"
                as *const u8 as *const libc::c_char,
            (*(*pt).nr).id as libc::c_ulonglong,
            (*pt).curcycle,
            keyname,
            (*pt).db_id,
            (*(*pt).nr).training_max_ms as libc::c_ulonglong,
            (*(*pt).nr).training_max_cycles as libc::c_ulonglong,
            (*pt).dataset_error as libc::c_double,
            (*pt).test_error as libc::c_double,
            (*pt).class_error as libc::c_double,
        );
        (Some(RedisModule_ReplyWithSimpleString.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(ctx, buf.as_mut_ptr() as *const libc::c_char);
        j += 1;
    }
    pthread_mutex_unlock(&mut NRPendingTrainingMutex);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRGetdata_RedisCommand(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut key: *mut RedisModuleKey = 0 as *mut RedisModuleKey;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: *mut RedisModuleType = 0 as *mut RedisModuleType;
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ilen: libc::c_int = 0;
    let mut olen: libc::c_int = 0;
    let mut target: *mut NRDataset = 0 as *mut NRDataset;
    let mut idx: libc::c_longlong = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut input: libc::c_double = 0.;
    let mut j___0: libc::c_int = 0;
    let mut output: libc::c_double = 0.;
    (Some(RedisModule_AutoMemory.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx);
    NRCollectThreads(ctx);
    if argc != 4 as libc::c_int {
        tmp = (Some(RedisModule_WrongArity.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx);
        return tmp;
    }
    tmp___0 = (Some(RedisModule_OpenKey.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(ctx, *argv.offset(1 as libc::c_int as isize), 1 as libc::c_int);
    key = tmp___0 as *mut RedisModuleKey;
    tmp___2 = (Some(RedisModule_ModuleTypeGetType.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    if tmp___2 as libc::c_ulong != NRType as libc::c_ulong {
        tmp___1 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            ctx,
            b"WRONGTYPE Operation against a key holding the wrong kind of value\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___1;
    }
    tmp___3 = (Some(RedisModule_ModuleTypeGetValue.expect("non-null function pointer")))
        .expect("non-null function pointer")(key);
    nr = tmp___3 as *mut NRTypeObject;
    ilen = (*((*(*nr).nn).layer)
        .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int;
    olen = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize)).units;
    target = 0 as *mut libc::c_void as *mut NRDataset;
    tmp___7 = (Some(RedisModule_StringPtrLen.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        *argv.offset(2 as libc::c_int as isize) as *const RedisModuleString,
        0 as *mut libc::c_void as *mut size_t,
    );
    tmp___8 = strcasecmp(tmp___7, b"train\0" as *const u8 as *const libc::c_char);
    if tmp___8 != 0 {
        tmp___5 = (Some(RedisModule_StringPtrLen.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            *argv.offset(2 as libc::c_int as isize) as *const RedisModuleString,
            0 as *mut libc::c_void as *mut size_t,
        );
        tmp___6 = strcasecmp(tmp___5, b"test\0" as *const u8 as *const libc::c_char);
        if tmp___6 != 0 {
            tmp___4 = (Some(
                RedisModule_ReplyWithError.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                b"ERR please specify as source either TRAIN or TEST\0" as *const u8
                    as *const libc::c_char,
            );
            return tmp___4;
        } else {
            target = &mut (*nr).test;
        }
    } else {
        target = &mut (*nr).dataset;
    }
    tmp___11 = (Some(RedisModule_StringToLongLong.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(*argv.offset(3 as libc::c_int as isize) as *const RedisModuleString, &mut idx);
    if tmp___11 != 0 as libc::c_int {
        tmp___9 = (Some(RedisModule_ReplyWithError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(ctx, b"ERR invalid row specified\0" as *const u8 as *const libc::c_char);
        return tmp___9;
    } else {
        if idx < 0 as libc::c_longlong {
            tmp___9 = (Some(
                RedisModule_ReplyWithError.expect("non-null function pointer"),
            ))
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                b"ERR invalid row specified\0" as *const u8 as *const libc::c_char,
            );
            return tmp___9;
        } else {
            if idx >= (*target).maxlen as libc::c_longlong {
                tmp___10 = (Some(
                    RedisModule_ReplyWithNull.expect("non-null function pointer"),
                ))
                    .expect("non-null function pointer")(ctx);
                return tmp___10;
            }
        }
    }
    (Some(RedisModule_ReplyWithArray.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, 2 as libc::c_long);
    (Some(RedisModule_ReplyWithArray.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, ilen as libc::c_long);
    j = 0 as libc::c_int;
    while j < ilen {
        input = *((*target).inputs)
            .offset((ilen as libc::c_longlong * idx + j as libc::c_longlong) as isize)
            as libc::c_double;
        (Some(RedisModule_ReplyWithDouble.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx, input);
        j += 1;
    }
    (Some(RedisModule_ReplyWithArray.expect("non-null function pointer")))
        .expect("non-null function pointer")(ctx, olen as libc::c_long);
    j___0 = 0 as libc::c_int;
    while j___0 < olen {
        output = *((*target).outputs)
            .offset(
                (olen as libc::c_longlong * idx + j___0 as libc::c_longlong) as isize,
            ) as libc::c_double;
        (Some(RedisModule_ReplyWithDouble.expect("non-null function pointer")))
            .expect("non-null function pointer")(ctx, output);
        j___0 += 1;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn NRTypeRdbSaveDataset(
    mut rdb: *mut RedisModuleIO,
    mut ds: *mut NRDataset,
    mut ilen: uint32_t,
    mut olen: uint32_t,
) {
    let mut j: uint32_t = 0;
    let mut j___0: uint32_t = 0;
    (Some(RedisModule_SaveUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*ds).len as uint64_t);
    (Some(RedisModule_SaveUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*ds).maxlen as uint64_t);
    j = 0 as libc::c_int as uint32_t;
    while j < ilen.wrapping_mul((*ds).len) {
        (Some(RedisModule_SaveFloat.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(rdb, *((*ds).inputs).offset(j as isize));
        j = j.wrapping_add(1);
    }
    j___0 = 0 as libc::c_int as uint32_t;
    while j___0 < olen.wrapping_mul((*ds).len) {
        (Some(RedisModule_SaveFloat.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(rdb, *((*ds).outputs).offset(j___0 as isize));
        j___0 = j___0.wrapping_add(1);
    }
}
pub unsafe extern "C" fn NRTypeRdbSave(
    mut rdb: *mut RedisModuleIO,
    mut value: *mut libc::c_void,
) {
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut j: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut j___0: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut i___1: libc::c_int = 0;
    let mut ilen: uint32_t = 0;
    let mut olen: uint32_t = 0;
    let mut j___1: uint32_t = 0;
    let mut j___2: uint32_t = 0;
    nr = value as *mut NRTypeObject;
    (Some(RedisModule_SaveUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*(*nr).nn).layers as uint64_t);
    j = 0 as libc::c_int;
    while j < (*(*nr).nn).layers {
        units = (*((*(*nr).nn).layer).offset(j as isize)).units;
        if j != 0 as libc::c_int {
            units -= 1;
        }
        (Some(RedisModule_SaveUnsigned.expect("non-null function pointer")))
            .expect("non-null function pointer")(rdb, units as uint64_t);
        j += 1;
    }
    (Some(RedisModule_SaveUnsigned.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        rdb,
        ((*nr).flags
            & ((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint) as uint64_t,
    );
    (Some(RedisModule_SaveUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*nr).id);
    (Some(RedisModule_SaveUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*nr).training_total_steps);
    (Some(RedisModule_SaveUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*nr).training_total_ms);
    (Some(RedisModule_SaveUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*nr).training_max_cycles);
    (Some(RedisModule_SaveUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*nr).training_max_ms);
    (Some(RedisModule_SaveFloat.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*nr).dataset_error);
    (Some(RedisModule_SaveFloat.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*nr).test_error);
    (Some(RedisModule_SaveFloat.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb, (*nr).test_class_error);
    j___0 = 1 as libc::c_int;
    while j___0 < (*(*nr).nn).layers {
        weights = (*((*(*nr).nn).layer).offset(j___0 as isize)).units
            * (*((*(*nr).nn).layer).offset((j___0 - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            (Some(RedisModule_SaveFloat.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                rdb,
                *((*((*(*nr).nn).layer).offset(j___0 as isize)).weight)
                    .offset(i as isize),
            );
            i += 1;
        }
        i___0 = 0 as libc::c_int;
        while i___0 < weights {
            (Some(RedisModule_SaveFloat.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                rdb,
                *((*((*(*nr).nn).layer).offset(j___0 as isize)).delta)
                    .offset(i___0 as isize),
            );
            i___0 += 1;
        }
        i___1 = 0 as libc::c_int;
        while i___1 < weights {
            (Some(RedisModule_SaveFloat.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                rdb,
                *((*((*(*nr).nn).layer).offset(j___0 as isize)).pgradient)
                    .offset(i___1 as isize),
            );
            i___1 += 1;
        }
        j___0 += 1;
    }
    ilen = ((*((*(*nr).nn).layer)
        .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int) as uint32_t;
    olen = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize)).units as uint32_t;
    j___1 = 0 as libc::c_int as uint32_t;
    while j___1 < ilen {
        (Some(RedisModule_SaveFloat.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(rdb, *((*nr).inorm).offset(j___1 as isize));
        j___1 = j___1.wrapping_add(1);
    }
    j___2 = 0 as libc::c_int as uint32_t;
    while j___2 < olen {
        (Some(RedisModule_SaveFloat.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(rdb, *((*nr).onorm).offset(j___2 as isize));
        j___2 = j___2.wrapping_add(1);
    }
    NRTypeRdbSaveDataset(rdb, &mut (*nr).dataset, ilen, olen);
    NRTypeRdbSaveDataset(rdb, &mut (*nr).test, ilen, olen);
}
pub unsafe extern "C" fn NRTypeRdbLoadDataset(
    mut rdb: *mut RedisModuleIO,
    mut ds: *mut NRDataset,
    mut ilen: uint32_t,
    mut olen: uint32_t,
) {
    let mut tmp: uint64_t = 0;
    let mut tmp___0: uint64_t = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut j: uint32_t = 0;
    let mut j___0: uint32_t = 0;
    tmp = (Some(RedisModule_LoadUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb);
    (*ds).len = tmp as uint32_t;
    tmp___0 = (Some(RedisModule_LoadUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb);
    (*ds).maxlen = tmp___0 as uint32_t;
    if (*ds).len == 0 as libc::c_uint {
        return;
    }
    tmp___1 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (ilen.wrapping_mul((*ds).len) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    (*ds).inputs = tmp___1 as *mut libc::c_float;
    tmp___2 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (olen.wrapping_mul((*ds).len) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::c_ulong),
    );
    (*ds).outputs = tmp___2 as *mut libc::c_float;
    j = 0 as libc::c_int as uint32_t;
    while j < ilen.wrapping_mul((*ds).len) {
        *((*ds).inputs)
            .offset(
                j as isize,
            ) = (Some(RedisModule_LoadFloat.expect("non-null function pointer")))
            .expect("non-null function pointer")(rdb);
        j = j.wrapping_add(1);
    }
    j___0 = 0 as libc::c_int as uint32_t;
    while j___0 < olen.wrapping_mul((*ds).len) {
        *((*ds).outputs)
            .offset(
                j___0 as isize,
            ) = (Some(RedisModule_LoadFloat.expect("non-null function pointer")))
            .expect("non-null function pointer")(rdb);
        j___0 = j___0.wrapping_add(1);
    }
}
pub unsafe extern "C" fn NRTypeRdbLoad(
    mut rdb: *mut RedisModuleIO,
    mut encver: libc::c_int,
) -> *mut libc::c_void {
    let mut numlayers: uint64_t = 0;
    let mut tmp: uint64_t = 0;
    let mut layers: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut j: uint32_t = 0;
    let mut tmp___1: uint64_t = 0;
    let mut flags: uint32_t = 0;
    let mut tmp___2: uint64_t = 0;
    let mut nr: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut tmp___3: *mut NRTypeObject = 0 as *mut NRTypeObject;
    let mut j___0: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut i___1: libc::c_int = 0;
    let mut ilen: uint32_t = 0;
    let mut olen: uint32_t = 0;
    let mut j___1: uint32_t = 0;
    let mut j___2: uint32_t = 0;
    if encver != 2 as libc::c_int {
        (Some(RedisModule_LogIOError.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            rdb,
            b"warning\0" as *const u8 as *const libc::c_char,
            b"Sorry the Neural Redis module only supports RDB files written with the encoding version %d. This file has encoding version %d, and was likely written by a previous version of this module that is now deprecated. Once the module will be stable we'll start supporting older versions of the encodings, in case we switch to newer encodings.\0"
                as *const u8 as *const libc::c_char,
            2 as libc::c_int,
            encver,
        );
        return 0 as *mut libc::c_void;
    }
    tmp = (Some(RedisModule_LoadUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb);
    numlayers = tmp;
    tmp___0 = (Some(RedisModule_Alloc.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(numlayers),
    );
    layers = tmp___0 as *mut libc::c_int;
    j = 0 as libc::c_int as uint32_t;
    while (j as uint64_t) < numlayers {
        tmp___1 = (Some(RedisModule_LoadUnsigned.expect("non-null function pointer")))
            .expect("non-null function pointer")(rdb);
        *layers.offset(j as isize) = tmp___1 as libc::c_int;
        j = j.wrapping_add(1);
    }
    tmp___2 = (Some(RedisModule_LoadUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb);
    flags = tmp___2 as uint32_t;
    tmp___3 = createNRTypeObject(
        flags as libc::c_int,
        layers,
        numlayers as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    nr = tmp___3;
    (Some(RedisModule_Free.expect("non-null function pointer")))
        .expect("non-null function pointer")(layers as *mut libc::c_void);
    (*nr)
        .id = (Some(RedisModule_LoadUnsigned.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb);
    (*nr)
        .training_total_steps = (Some(
        RedisModule_LoadUnsigned.expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(rdb);
    (*nr)
        .training_total_ms = (Some(
        RedisModule_LoadUnsigned.expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(rdb);
    (*nr)
        .training_max_cycles = (Some(
        RedisModule_LoadUnsigned.expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(rdb);
    (*nr)
        .training_max_ms = (Some(
        RedisModule_LoadUnsigned.expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(rdb);
    (*nr)
        .dataset_error = (Some(
        RedisModule_LoadFloat.expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(rdb);
    (*nr)
        .test_error = (Some(RedisModule_LoadFloat.expect("non-null function pointer")))
        .expect("non-null function pointer")(rdb);
    (*nr)
        .test_class_error = (Some(
        RedisModule_LoadFloat.expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(rdb);
    j___0 = 1 as libc::c_int;
    while j___0 < (*(*nr).nn).layers {
        weights = (*((*(*nr).nn).layer).offset(j___0 as isize)).units
            * (*((*(*nr).nn).layer).offset((j___0 - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*(*nr).nn).layer).offset(j___0 as isize)).weight)
                .offset(
                    i as isize,
                ) = (Some(RedisModule_LoadFloat.expect("non-null function pointer")))
                .expect("non-null function pointer")(rdb);
            i += 1;
        }
        i___0 = 0 as libc::c_int;
        while i___0 < weights {
            *((*((*(*nr).nn).layer).offset(j___0 as isize)).delta)
                .offset(
                    i___0 as isize,
                ) = (Some(RedisModule_LoadFloat.expect("non-null function pointer")))
                .expect("non-null function pointer")(rdb);
            i___0 += 1;
        }
        i___1 = 0 as libc::c_int;
        while i___1 < weights {
            *((*((*(*nr).nn).layer).offset(j___0 as isize)).pgradient)
                .offset(
                    i___1 as isize,
                ) = (Some(RedisModule_LoadFloat.expect("non-null function pointer")))
                .expect("non-null function pointer")(rdb);
            i___1 += 1;
        }
        j___0 += 1;
    }
    ilen = ((*((*(*nr).nn).layer)
        .offset(((*(*nr).nn).layers - 1 as libc::c_int) as isize))
        .units - 1 as libc::c_int) as uint32_t;
    olen = (*((*(*nr).nn).layer).offset(0 as libc::c_int as isize)).units as uint32_t;
    j___1 = 0 as libc::c_int as uint32_t;
    while j___1 < ilen {
        *((*nr).inorm)
            .offset(
                j___1 as isize,
            ) = (Some(RedisModule_LoadFloat.expect("non-null function pointer")))
            .expect("non-null function pointer")(rdb);
        j___1 = j___1.wrapping_add(1);
    }
    j___2 = 0 as libc::c_int as uint32_t;
    while j___2 < olen {
        *((*nr).onorm)
            .offset(
                j___2 as isize,
            ) = (Some(RedisModule_LoadFloat.expect("non-null function pointer")))
            .expect("non-null function pointer")(rdb);
        j___2 = j___2.wrapping_add(1);
    }
    NRTypeRdbLoadDataset(rdb, &mut (*nr).dataset, ilen, olen);
    NRTypeRdbLoadDataset(rdb, &mut (*nr).test, ilen, olen);
    return nr as *mut libc::c_void;
}
pub unsafe extern "C" fn NRTypeAofRewrite(
    mut aof: *mut RedisModuleIO,
    mut key: *mut RedisModuleString,
    mut value: *mut libc::c_void,
) {}
pub unsafe extern "C" fn NRTypeFree(mut value: *mut libc::c_void) {
    NRTypeReleaseObject(value as *mut NRTypeObject);
}
pub unsafe extern "C" fn RedisModule_OnLoad(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut RedisModuleString,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tm: RedisModuleTypeMethods = RedisModuleTypeMethods {
        version: 0,
        rdb_load: None,
        rdb_save: None,
        aof_rewrite: None,
        mem_usage: None,
        digest: None,
        free: None,
    };
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    tmp = RedisModule_Init(
        ctx,
        b"neuralredis\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if tmp == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    tm.version = 1 as libc::c_int as uint64_t;
    tm
        .rdb_load = Some(
        NRTypeRdbLoad
            as unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int) -> *mut libc::c_void,
    );
    tm
        .rdb_save = Some(
        NRTypeRdbSave
            as unsafe extern "C" fn(*mut RedisModuleIO, *mut libc::c_void) -> (),
    );
    tm
        .aof_rewrite = Some(
        NRTypeAofRewrite
            as unsafe extern "C" fn(
                *mut RedisModuleIO,
                *mut RedisModuleString,
                *mut libc::c_void,
            ) -> (),
    );
    tm.mem_usage = None;
    tm.digest = None;
    tm.free = Some(NRTypeFree as unsafe extern "C" fn(*mut libc::c_void) -> ());
    NRType = (Some(RedisModule_CreateDataType.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        b"neural-NN\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        &mut tm,
    );
    if NRType as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 1 as libc::c_int;
    }
    tmp___0 = (Some(RedisModule_CreateCommand.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        b"nr.create\0" as *const u8 as *const libc::c_char,
        Some(
            NRCreate_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"write deny-oom\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if tmp___0 == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    tmp___1 = (Some(RedisModule_CreateCommand.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        b"nr.run\0" as *const u8 as *const libc::c_char,
        Some(
            NRRun_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"readonly\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if tmp___1 == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    tmp___2 = (Some(RedisModule_CreateCommand.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        b"nr.class\0" as *const u8 as *const libc::c_char,
        Some(
            NRClass_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"readonly\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if tmp___2 == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    tmp___3 = (Some(RedisModule_CreateCommand.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        b"nr.observe\0" as *const u8 as *const libc::c_char,
        Some(
            NRObserve_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"write deny-oom\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if tmp___3 == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    tmp___4 = (Some(RedisModule_CreateCommand.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        b"nr.info\0" as *const u8 as *const libc::c_char,
        Some(
            NRInfo_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"readonly\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if tmp___4 == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    tmp___5 = (Some(RedisModule_CreateCommand.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        b"nr.train\0" as *const u8 as *const libc::c_char,
        Some(
            NRTrain_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"write\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if tmp___5 == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    tmp___6 = (Some(RedisModule_CreateCommand.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        b"nr.reset\0" as *const u8 as *const libc::c_char,
        Some(
            NRReset_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"write\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if tmp___6 == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    tmp___7 = (Some(RedisModule_CreateCommand.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        b"nr.threads\0" as *const u8 as *const libc::c_char,
        Some(
            NRThreads_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if tmp___7 == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    tmp___8 = (Some(RedisModule_CreateCommand.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        ctx,
        b"nr.getdata\0" as *const u8 as *const libc::c_char,
        Some(
            NRGetdata_RedisCommand
                as unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut RedisModuleString,
                    libc::c_int,
                ) -> libc::c_int,
        ),
        b"readonly\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    );
    if tmp___8 == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn sigmoid(mut x: libc::c_float) -> libc::c_float {
    let mut tmp: libc::c_double = 0.;
    tmp = exp(-x as libc::c_double);
    return (1 as libc::c_int as libc::c_float as libc::c_double
        / (1 as libc::c_int as libc::c_double + tmp)) as libc::c_float;
}
pub unsafe extern "C" fn relu(mut x: libc::c_float) -> libc::c_float {
    let mut tmp: libc::c_float = 0.;
    if x > 0 as libc::c_int as libc::c_float {
        tmp = x;
    } else {
        tmp = 0 as libc::c_int as libc::c_float;
    }
    return tmp;
}
pub unsafe extern "C" fn AnnResetLayer(mut layer: *mut AnnLayer) {
    (*layer).units = 0 as libc::c_int;
    (*layer).output = 0 as *mut libc::c_void as *mut libc::c_float;
    (*layer).error = 0 as *mut libc::c_void as *mut libc::c_float;
    (*layer).weight = 0 as *mut libc::c_void as *mut libc::c_float;
    (*layer).gradient = 0 as *mut libc::c_void as *mut libc::c_float;
    (*layer).pgradient = 0 as *mut libc::c_void as *mut libc::c_float;
    (*layer).delta = 0 as *mut libc::c_void as *mut libc::c_float;
    (*layer).sgradient = 0 as *mut libc::c_void as *mut libc::c_float;
}
pub unsafe extern "C" fn AnnAlloc(mut layers: libc::c_int) -> *mut Ann {
    let mut net: *mut Ann = 0 as *mut Ann;
    let mut i: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut AnnLayer = 0 as *mut AnnLayer;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(::std::mem::size_of::<Ann>() as libc::c_ulong);
    net = tmp as *mut Ann;
    if net as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut Ann;
    }
    tmp___1 = malloc(
        (::std::mem::size_of::<AnnLayer>() as libc::c_ulong)
            .wrapping_mul(layers as libc::c_ulong),
    );
    tmp___0 = tmp___1 as *mut AnnLayer;
    (*net).layer = tmp___0;
    if tmp___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        free(net as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut Ann;
    }
    (*net).layers = layers;
    (*net).flags = 0 as libc::c_int;
    (*net).rprop_nminus = 0.5f64 as libc::c_float;
    (*net).rprop_nplus = 1.2f64 as libc::c_float;
    (*net).rprop_maxupdate = 50 as libc::c_int as libc::c_float;
    (*net).rprop_minupdate = 0.000001f64 as libc::c_float;
    i = 0 as libc::c_int;
    while i < layers {
        AnnResetLayer(((*net).layer).offset(i as isize));
        i += 1;
    }
    return net;
}
pub unsafe extern "C" fn AnnFreeLayer(mut layer: *mut AnnLayer) {
    free((*layer).output as *mut libc::c_void);
    free((*layer).error as *mut libc::c_void);
    free((*layer).weight as *mut libc::c_void);
    free((*layer).gradient as *mut libc::c_void);
    free((*layer).pgradient as *mut libc::c_void);
    free((*layer).delta as *mut libc::c_void);
    free((*layer).sgradient as *mut libc::c_void);
    AnnResetLayer(layer);
}
pub unsafe extern "C" fn AnnFree(mut net: *mut Ann) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*net).layers {
        AnnFreeLayer(((*net).layer).offset(i as isize));
        i += 1;
    }
    free((*net).layer as *mut libc::c_void);
    free(net as *mut libc::c_void);
}
pub unsafe extern "C" fn AnnInitLayer(
    mut net: *mut Ann,
    mut i: libc::c_int,
    mut units: libc::c_int,
    mut bias: libc::c_int,
) -> libc::c_int {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___5: *mut libc::c_void = 0 as *mut libc::c_void;
    if bias != 0 {
        units += 1;
    }
    tmp = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(units as libc::c_ulong),
    );
    let ref mut fresh0 = (*((*net).layer).offset(i as isize)).output;
    *fresh0 = tmp as *mut libc::c_float;
    tmp___0 = malloc(
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(units as libc::c_ulong),
    );
    let ref mut fresh1 = (*((*net).layer).offset(i as isize)).error;
    *fresh1 = tmp___0 as *mut libc::c_float;
    if i != 0 {
        tmp___1 = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        let ref mut fresh2 = (*((*net).layer).offset(i as isize)).weight;
        *fresh2 = tmp___1 as *mut libc::c_float;
        tmp___2 = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        let ref mut fresh3 = (*((*net).layer).offset(i as isize)).gradient;
        *fresh3 = tmp___2 as *mut libc::c_float;
        tmp___3 = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        let ref mut fresh4 = (*((*net).layer).offset(i as isize)).pgradient;
        *fresh4 = tmp___3 as *mut libc::c_float;
        tmp___4 = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        let ref mut fresh5 = (*((*net).layer).offset(i as isize)).delta;
        *fresh5 = tmp___4 as *mut libc::c_float;
        tmp___5 = malloc(
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        let ref mut fresh6 = (*((*net).layer).offset(i as isize)).sgradient;
        *fresh6 = tmp___5 as *mut libc::c_float;
    }
    (*((*net).layer).offset(i as isize)).units = units;
    if (*((*net).layer).offset(i as isize)).output as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        AnnFreeLayer(((*net).layer).offset(i as isize));
        AnnResetLayer(((*net).layer).offset(i as isize));
        return 1 as libc::c_int;
    } else {
        if (*((*net).layer).offset(i as isize)).error as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            AnnFreeLayer(((*net).layer).offset(i as isize));
            AnnResetLayer(((*net).layer).offset(i as isize));
            return 1 as libc::c_int;
        } else {
            if i != 0 {
                if (*((*net).layer).offset(i as isize)).weight as libc::c_ulong
                    == 0 as *mut libc::c_void as libc::c_ulong
                {
                    AnnFreeLayer(((*net).layer).offset(i as isize));
                    AnnResetLayer(((*net).layer).offset(i as isize));
                    return 1 as libc::c_int;
                }
            }
            if i != 0 {
                if (*((*net).layer).offset(i as isize)).gradient as libc::c_ulong
                    == 0 as *mut libc::c_void as libc::c_ulong
                {
                    AnnFreeLayer(((*net).layer).offset(i as isize));
                    AnnResetLayer(((*net).layer).offset(i as isize));
                    return 1 as libc::c_int;
                }
            }
            if i != 0 {
                if (*((*net).layer).offset(i as isize)).pgradient as libc::c_ulong
                    == 0 as *mut libc::c_void as libc::c_ulong
                {
                    AnnFreeLayer(((*net).layer).offset(i as isize));
                    AnnResetLayer(((*net).layer).offset(i as isize));
                    return 1 as libc::c_int;
                }
            }
            if i != 0 {
                if (*((*net).layer).offset(i as isize)).sgradient as libc::c_ulong
                    == 0 as *mut libc::c_void as libc::c_ulong
                {
                    AnnFreeLayer(((*net).layer).offset(i as isize));
                    AnnResetLayer(((*net).layer).offset(i as isize));
                    return 1 as libc::c_int;
                }
            }
            if i != 0 {
                if (*((*net).layer).offset(i as isize)).delta as libc::c_ulong
                    == 0 as *mut libc::c_void as libc::c_ulong
                {
                    AnnFreeLayer(((*net).layer).offset(i as isize));
                    AnnResetLayer(((*net).layer).offset(i as isize));
                    return 1 as libc::c_int;
                }
            }
        }
    }
    memset(
        (*((*net).layer).offset(i as isize)).output as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(units as libc::c_ulong),
    );
    memset(
        (*((*net).layer).offset(i as isize)).error as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
            .wrapping_mul(units as libc::c_ulong),
    );
    if i != 0 {
        memset(
            (*((*net).layer).offset(i as isize)).weight as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        memset(
            (*((*net).layer).offset(i as isize)).gradient as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        memset(
            (*((*net).layer).offset(i as isize)).pgradient as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        memset(
            (*((*net).layer).offset(i as isize)).delta as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
        memset(
            (*((*net).layer).offset(i as isize)).sgradient as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(units as libc::c_ulong)
                .wrapping_mul(
                    (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                        as libc::c_ulong,
                ),
        );
    }
    if bias != 0 {
        *((*((*net).layer).offset(i as isize)).output)
            .offset(
                (units - 1 as libc::c_int) as isize,
            ) = 1 as libc::c_int as libc::c_float;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn AnnClone(mut net: *mut Ann) -> *mut Ann {
    let mut copy: *mut Ann = 0 as *mut Ann;
    let mut j: libc::c_int = 0;
    let mut ldst: *mut AnnLayer = 0 as *mut AnnLayer;
    let mut lsrc: *mut AnnLayer = 0 as *mut AnnLayer;
    let mut units: libc::c_int = 0;
    let mut bias: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    copy = AnnAlloc((*net).layers);
    if copy as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut Ann;
    }
    j = 0 as libc::c_int;
    while j < (*net).layers {
        units = (*((*net).layer).offset(j as isize)).units;
        bias = (j > 0 as libc::c_int) as libc::c_int;
        tmp = AnnInitLayer(copy, j, units - bias, bias);
        if tmp != 0 {
            AnnFree(copy);
            return 0 as *mut libc::c_void as *mut Ann;
        }
        lsrc = ((*net).layer).offset(j as isize);
        ldst = ((*copy).layer).offset(j as isize);
        if !((*lsrc).output).is_null() {
            memcpy(
                (*ldst).output as *mut libc::c_void,
                (*lsrc).output as *const libc::c_void,
                (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                    .wrapping_mul(units as libc::c_ulong),
            );
        }
        if !((*lsrc).error).is_null() {
            memcpy(
                (*ldst).error as *mut libc::c_void,
                (*lsrc).error as *const libc::c_void,
                (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                    .wrapping_mul(units as libc::c_ulong),
            );
        }
        if j != 0 {
            weights = (*((*net).layer).offset(j as isize)).units
                * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
            if !((*lsrc).weight).is_null() {
                memcpy(
                    (*ldst).weight as *mut libc::c_void,
                    (*lsrc).weight as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(weights as libc::c_ulong),
                );
            }
            if !((*lsrc).gradient).is_null() {
                memcpy(
                    (*ldst).gradient as *mut libc::c_void,
                    (*lsrc).gradient as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(weights as libc::c_ulong),
                );
            }
            if !((*lsrc).pgradient).is_null() {
                memcpy(
                    (*ldst).pgradient as *mut libc::c_void,
                    (*lsrc).pgradient as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(weights as libc::c_ulong),
                );
            }
            if !((*lsrc).delta).is_null() {
                memcpy(
                    (*ldst).delta as *mut libc::c_void,
                    (*lsrc).delta as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(weights as libc::c_ulong),
                );
            }
            if !((*lsrc).sgradient).is_null() {
                memcpy(
                    (*ldst).sgradient as *mut libc::c_void,
                    (*lsrc).sgradient as *const libc::c_void,
                    (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                        .wrapping_mul(weights as libc::c_ulong),
                );
            }
        }
        j += 1;
    }
    (*copy).rprop_nminus = (*net).rprop_nminus;
    (*copy).rprop_nplus = (*net).rprop_nplus;
    (*copy).rprop_maxupdate = (*net).rprop_maxupdate;
    (*copy).rprop_minupdate = (*net).rprop_minupdate;
    (*copy).flags = (*net).flags;
    return copy;
}
pub unsafe extern "C" fn AnnCreateNet(
    mut layers: libc::c_int,
    mut units: *mut libc::c_int,
) -> *mut Ann {
    let mut net: *mut Ann = 0 as *mut Ann;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    net = AnnAlloc(layers);
    if net as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut Ann;
    }
    i = 0 as libc::c_int;
    while i < layers {
        tmp = AnnInitLayer(
            net,
            i,
            *units.offset(i as isize),
            (i > 0 as libc::c_int) as libc::c_int,
        );
        if tmp != 0 {
            AnnFree(net);
            return 0 as *mut libc::c_void as *mut Ann;
        }
        i += 1;
    }
    AnnSetRandomWeights(net);
    AnnSetDeltas(net, 0.1f64 as libc::c_float);
    (*net).learn_rate = 0.1f64 as libc::c_float;
    return net;
}
pub unsafe extern "C" fn AnnCountWeights(mut net: *mut Ann) -> size_t {
    let mut weights: size_t = 0;
    let mut i: libc::c_int = 0;
    let mut nextunits: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    weights = 0 as libc::c_int as size_t;
    i = (*net).layers - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        nextunits = (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units;
        units = (*((*net).layer).offset(i as isize)).units;
        if i > 1 as libc::c_int {
            nextunits -= 1;
        }
        weights = (weights as libc::c_ulong).wrapping_add((units * nextunits) as size_t)
            as size_t as size_t;
        i -= 1;
    }
    return weights;
}
pub unsafe extern "C" fn AnnCreateNet4(
    mut iunits: libc::c_int,
    mut hunits: libc::c_int,
    mut hunits2: libc::c_int,
    mut ounits: libc::c_int,
) -> *mut Ann {
    let mut units: [libc::c_int; 4] = [0; 4];
    let mut tmp: *mut Ann = 0 as *mut Ann;
    units[0 as libc::c_int as usize] = ounits;
    units[1 as libc::c_int as usize] = hunits2;
    units[2 as libc::c_int as usize] = hunits;
    units[3 as libc::c_int as usize] = iunits;
    tmp = AnnCreateNet(4 as libc::c_int, units.as_mut_ptr());
    return tmp;
}
pub unsafe extern "C" fn AnnCreateNet3(
    mut iunits: libc::c_int,
    mut hunits: libc::c_int,
    mut ounits: libc::c_int,
) -> *mut Ann {
    let mut units: [libc::c_int; 3] = [0; 3];
    let mut tmp: *mut Ann = 0 as *mut Ann;
    units[0 as libc::c_int as usize] = ounits;
    units[1 as libc::c_int as usize] = hunits;
    units[2 as libc::c_int as usize] = iunits;
    tmp = AnnCreateNet(3 as libc::c_int, units.as_mut_ptr());
    return tmp;
}
pub unsafe extern "C" fn AnnCreateNet2(
    mut iunits: libc::c_int,
    mut ounits: libc::c_int,
) -> *mut Ann {
    let mut units: [libc::c_int; 2] = [0; 2];
    let mut tmp: *mut Ann = 0 as *mut Ann;
    units[0 as libc::c_int as usize] = ounits;
    units[1 as libc::c_int as usize] = iunits;
    tmp = AnnCreateNet(2 as libc::c_int, units.as_mut_ptr());
    return tmp;
}
pub unsafe extern "C" fn AnnSimulate(mut net: *mut Ann) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nextunits: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut A: libc::c_float = 0.;
    let mut w: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut o: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut tmp: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut tmp___0: *mut libc::c_float = 0 as *mut libc::c_float;
    i = (*net).layers - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        nextunits = (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units;
        units = (*((*net).layer).offset(i as isize)).units;
        if i > 1 as libc::c_int {
            nextunits -= 1;
        }
        j = 0 as libc::c_int;
        while j < nextunits {
            A = 0 as libc::c_int as libc::c_float;
            w = ((*((*net).layer).offset(i as isize)).weight)
                .offset((j * units) as isize);
            o = (*((*net).layer).offset(i as isize)).output;
            k = 0 as libc::c_int;
            while k < units {
                tmp = w;
                w = w.offset(1);
                tmp___0 = o;
                o = o.offset(1);
                A += *tmp * *tmp___0;
                k += 1;
            }
            *((*((*net).layer).offset((i - 1 as libc::c_int) as isize)).output)
                .offset(j as isize) = sigmoid(A);
            j += 1;
        }
        i -= 1;
    }
}
pub unsafe extern "C" fn Ann2Tcl(mut net: *mut Ann) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nextunits: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut W: libc::c_float = 0.;
    printf(b"proc ann input {\n\0" as *const u8 as *const libc::c_char);
    printf(b"    set output {\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*((*net).layer).offset(0 as libc::c_int as isize)).units {
        printf(b"0 \0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
    i = (*net).layers - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        nextunits = (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units;
        units = (*((*net).layer).offset(i as isize)).units;
        if i > 1 as libc::c_int {
            nextunits -= 1;
        }
        j = 0 as libc::c_int;
        while j < nextunits {
            if i == 1 as libc::c_int {
                printf(b"    lset output %d \0" as *const u8 as *const libc::c_char, j);
            } else {
                printf(
                    b"    set O_%d_%d\0" as *const u8 as *const libc::c_char,
                    i - 1 as libc::c_int,
                    j,
                );
            }
            printf(b" [expr { \\\n\0" as *const u8 as *const libc::c_char);
            k = 0 as libc::c_int;
            while k < units {
                W = *((*((*net).layer).offset(i as isize)).weight)
                    .offset(
                        (j * (*((*net).layer).offset(i as isize)).units + k) as isize,
                    );
                let mut current_block_29: u64;
                if i > 1 as libc::c_int {
                    if k == units - 1 as libc::c_int {
                        printf(
                            b"        (%.9f)\0" as *const u8 as *const libc::c_char,
                            W as libc::c_double,
                        );
                        current_block_29 = 5689316957504528238;
                    } else {
                        current_block_29 = 1803712249039311133;
                    }
                } else {
                    current_block_29 = 1803712249039311133;
                }
                match current_block_29 {
                    1803712249039311133 => {
                        if i == (*net).layers - 1 as libc::c_int {
                            printf(
                                b"        (%.9f*[lindex $input %d])\0" as *const u8
                                    as *const libc::c_char,
                                W as libc::c_double,
                                k,
                            );
                        } else {
                            printf(
                                b"        (%.9f*$O_%d_%d)\0" as *const u8
                                    as *const libc::c_char,
                                W as libc::c_double,
                                i,
                                k,
                            );
                        }
                    }
                    _ => {}
                }
                if (k + 1 as libc::c_int) < units {
                    printf(b"+ \\\n\0" as *const u8 as *const libc::c_char);
                }
                k += 1;
            }
            printf(b"}]\n\0" as *const u8 as *const libc::c_char);
            if i == 1 as libc::c_int {
                printf(
                    b"    lset output %d [expr {1/(1+exp(-[lindex $output %d]))}]\n\0"
                        as *const u8 as *const libc::c_char,
                    j,
                    j,
                );
            } else {
                printf(
                    b"    lset O_%d_%d [expr {1/(1+exp(-$O_%d_%d))}]\n\0" as *const u8
                        as *const libc::c_char,
                    i - 1 as libc::c_int,
                    j,
                    i - 1 as libc::c_int,
                    j,
                );
            }
            j += 1;
        }
        i -= 1;
    }
    printf(b"    return $output\n\0" as *const u8 as *const libc::c_char);
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn AnnPrint(mut net: *mut Ann) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut layertype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut targets: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*net).layers {
        layertype = b"Hidden\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        if i == 0 as libc::c_int {
            layertype = b"Output\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        if i == (*net).layers - 1 as libc::c_int {
            layertype = b"Input\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        printf(
            b"%s layer %d, units %d\n\0" as *const u8 as *const libc::c_char,
            layertype,
            i,
            (*((*net).layer).offset(i as isize)).units,
        );
        if i != 0 {
            targets = (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units
                - (i - 1 as libc::c_int > 0 as libc::c_int) as libc::c_int;
            printf(b"\tW\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                printf(b"(\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < targets {
                    printf(
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *((*((*net).layer).offset(i as isize)).weight)
                            .offset(
                                (k * (*((*net).layer).offset(i as isize)).units + j)
                                    as isize,
                            ) as libc::c_double,
                    );
                    if k != targets - 1 as libc::c_int {
                        printf(b" \0" as *const u8 as *const libc::c_char);
                    }
                    k += 1;
                }
                printf(b") \0" as *const u8 as *const libc::c_char);
                j += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(b"\tg\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                printf(b"[\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < targets {
                    printf(
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *((*((*net).layer).offset(i as isize)).gradient)
                            .offset(
                                (k * (*((*net).layer).offset(i as isize)).units + j)
                                    as isize,
                            ) as libc::c_double,
                    );
                    if k != targets - 1 as libc::c_int {
                        printf(b" \0" as *const u8 as *const libc::c_char);
                    }
                    k += 1;
                }
                printf(b"] \0" as *const u8 as *const libc::c_char);
                j += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(b"\tG\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                printf(b"[\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < targets {
                    printf(
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *((*((*net).layer).offset(i as isize)).sgradient)
                            .offset(
                                (k * (*((*net).layer).offset(i as isize)).units + j)
                                    as isize,
                            ) as libc::c_double,
                    );
                    if k != targets - 1 as libc::c_int {
                        printf(b" \0" as *const u8 as *const libc::c_char);
                    }
                    k += 1;
                }
                printf(b"] \0" as *const u8 as *const libc::c_char);
                j += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(b"\tP\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                printf(b"[\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < targets {
                    printf(
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *((*((*net).layer).offset(i as isize)).pgradient)
                            .offset(
                                (k * (*((*net).layer).offset(i as isize)).units + j)
                                    as isize,
                            ) as libc::c_double,
                    );
                    if k != targets - 1 as libc::c_int {
                        printf(b" \0" as *const u8 as *const libc::c_char);
                    }
                    k += 1;
                }
                printf(b"] \0" as *const u8 as *const libc::c_char);
                j += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            printf(b"\tD\0" as *const u8 as *const libc::c_char);
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                printf(b"|\0" as *const u8 as *const libc::c_char);
                k = 0 as libc::c_int;
                while k < targets {
                    printf(
                        b"%f\0" as *const u8 as *const libc::c_char,
                        *((*((*net).layer).offset(i as isize)).delta)
                            .offset(
                                (k * (*((*net).layer).offset(i as isize)).units + j)
                                    as isize,
                            ) as libc::c_double,
                    );
                    if k != targets - 1 as libc::c_int {
                        printf(b" \0" as *const u8 as *const libc::c_char);
                    }
                    k += 1;
                }
                printf(b"| \0" as *const u8 as *const libc::c_char);
                j += 1;
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
        j = 0 as libc::c_int;
        while j < (*((*net).layer).offset(i as isize)).units {
            printf(
                b"\tO: %f \0" as *const u8 as *const libc::c_char,
                *((*((*net).layer).offset(i as isize)).output).offset(j as isize)
                    as libc::c_double,
            );
            j += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        printf(b"\tE /\0" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int;
        while j < (*((*net).layer).offset(i as isize)).units {
            printf(
                b"%f \0" as *const u8 as *const libc::c_char,
                *((*((*net).layer).offset(i as isize)).error).offset(j as isize)
                    as libc::c_double,
            );
            j += 1;
        }
        printf(b"/\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}
pub unsafe extern "C" fn AnnGlobalError(
    mut net: *mut Ann,
    mut desired: *mut libc::c_float,
) -> libc::c_float {
    let mut e: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut outputs: libc::c_int = 0;
    outputs = (*((*net).layer).offset(0 as libc::c_int as isize)).units;
    e = 0 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < outputs {
        t = *desired.offset(i as isize)
            - *((*((*net).layer).offset(0 as libc::c_int as isize)).output)
                .offset(i as isize);
        e += t * t;
        i += 1;
    }
    return (0.5f64 * e as libc::c_double) as libc::c_float;
}
pub unsafe extern "C" fn AnnSetInput(mut net: *mut Ann, mut input: *mut libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut inputs: libc::c_int = 0;
    inputs = (*((*net).layer).offset(((*net).layers - 1 as libc::c_int) as isize)).units
        - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < inputs {
        *((*((*net).layer).offset(((*net).layers - 1 as libc::c_int) as isize)).output)
            .offset(i as isize) = *input.offset(i as isize);
        i += 1;
    }
}
pub unsafe extern "C" fn AnnSimulateError(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desired: *mut libc::c_float,
) -> libc::c_float {
    let mut tmp: libc::c_float = 0.;
    AnnSetInput(net, input);
    AnnSimulate(net);
    tmp = AnnGlobalError(net, desired);
    return tmp;
}
pub unsafe extern "C" fn AnnCalculateOutputError(
    mut net: *mut Ann,
    mut desired: *mut libc::c_float,
) {
    let mut units: libc::c_int = 0;
    let mut factor: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    units = (*((*net).layer).offset(0 as libc::c_int as isize)).units;
    factor = 2 as libc::c_int as libc::c_float / units as libc::c_float;
    j = 0 as libc::c_int;
    while j < units {
        *((*((*net).layer).offset(0 as libc::c_int as isize)).error)
            .offset(
                j as isize,
            ) = factor
            * (*((*((*net).layer).offset(0 as libc::c_int as isize)).output)
                .offset(j as isize) - *desired.offset(j as isize));
        j += 1;
    }
}
pub unsafe extern "C" fn AnnCalculateGradientsTrivial(
    mut net: *mut Ann,
    mut desired: *mut libc::c_float,
) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut layers: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    let mut t: libc::c_float = 0.;
    let mut e1: libc::c_float = 0.;
    let mut e2: libc::c_float = 0.;
    layers = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        units = (*((*net).layer).offset(j as isize)).units;
        weights = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            AnnSimulate(net);
            e1 = AnnGlobalError(net, desired);
            t = *((*((*net).layer).offset(j as isize)).weight).offset(i as isize);
            *((*((*net).layer).offset(j as isize)).weight)
                .offset(
                    i as isize,
                ) = (*((*((*net).layer).offset(j as isize)).weight).offset(i as isize)
                as libc::c_double + 0.001f64) as libc::c_float;
            AnnSimulate(net);
            e2 = AnnGlobalError(net, desired);
            *((*((*net).layer).offset(j as isize)).weight).offset(i as isize) = t;
            *((*((*net).layer).offset(j as isize)).gradient)
                .offset(
                    i as isize,
                ) = ((e2 - e1) as libc::c_double / 0.001f64) as libc::c_float;
            i += 1;
        }
        j += 1;
    }
}
pub unsafe extern "C" fn AnnCalculateGradients(
    mut net: *mut Ann,
    mut desired: *mut libc::c_float,
) {
    let mut j: libc::c_int = 0;
    let mut layers: libc::c_int = 0;
    let mut layer: *mut AnnLayer = 0 as *mut AnnLayer;
    let mut prev_layer: *mut AnnLayer = 0 as *mut AnnLayer;
    let mut i: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut prevunits: libc::c_int = 0;
    let mut error_signal: libc::c_float = 0.;
    let mut ei: libc::c_float = 0.;
    let mut oi: libc::c_float = 0.;
    let mut derivative: libc::c_float = 0.;
    let mut k: libc::c_int = 0;
    let mut g: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut w: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut o: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut e: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut tmp: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut tmp___0: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut tmp___1: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut tmp___2: *mut libc::c_float = 0 as *mut libc::c_float;
    layers = (*net).layers - 1 as libc::c_int;
    AnnCalculateOutputError(net, desired);
    j = 0 as libc::c_int;
    while j < layers {
        layer = ((*net).layer).offset(j as isize);
        prev_layer = ((*net).layer).offset((j + 1 as libc::c_int) as isize);
        units = (*layer).units;
        prevunits = (*prev_layer).units;
        if j > 1 as libc::c_int {
            units -= 1;
        }
        i = 0 as libc::c_int;
        while i < prevunits {
            *((*prev_layer).error)
                .offset(i as isize) = 0 as libc::c_int as libc::c_float;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < units {
            ei = *((*layer).error).offset(i as isize);
            oi = *((*layer).output).offset(i as isize);
            derivative = oi * (1 as libc::c_int as libc::c_float - oi);
            error_signal = ei * derivative;
            g = ((*prev_layer).gradient).offset((i * prevunits) as isize);
            w = ((*prev_layer).weight).offset((i * prevunits) as isize);
            o = (*prev_layer).output;
            e = (*prev_layer).error;
            k = 0 as libc::c_int;
            while k < prevunits {
                tmp = g;
                g = g.offset(1);
                tmp___0 = o;
                o = o.offset(1);
                *tmp = error_signal * *tmp___0;
                k += 1;
            }
            k = 0 as libc::c_int;
            while k < prevunits {
                tmp___1 = e;
                e = e.offset(1);
                tmp___2 = w;
                w = w.offset(1);
                *tmp___1 += error_signal * *tmp___2;
                k += 1;
            }
            i += 1;
        }
        j += 1;
    }
}
pub unsafe extern "C" fn AnnSetDeltas(mut net: *mut Ann, mut val: libc::c_float) {
    let mut j: libc::c_int = 0;
    let mut layers: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    layers = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        units = (*((*net).layer).offset(j as isize)).units;
        weights = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*net).layer).offset(j as isize)).delta).offset(i as isize) = val;
            i += 1;
        }
        j += 1;
    }
}
pub unsafe extern "C" fn AnnResetSgradient(mut net: *mut Ann) {
    let mut j: libc::c_int = 0;
    let mut layers: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    layers = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        units = (*((*net).layer).offset(j as isize)).units;
        weights = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        memset(
            (*((*net).layer).offset(j as isize)).sgradient as *mut libc::c_void,
            0 as libc::c_int,
            (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_mul(weights as libc::c_ulong),
        );
        j += 1;
    }
}
pub unsafe extern "C" fn AnnSetRandomWeights(mut net: *mut Ann) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < (*net).layers {
        k = 0 as libc::c_int;
        while k < (*((*net).layer).offset((i - 1 as libc::c_int) as isize)).units {
            j = 0 as libc::c_int;
            while j < (*((*net).layer).offset(i as isize)).units {
                tmp = rand();
                *((*((*net).layer).offset(i as isize)).weight)
                    .offset(
                        (k * (*((*net).layer).offset(i as isize)).units + j) as isize,
                    ) = (-0.05f64
                    + 0.1f64
                        * (tmp as libc::c_double
                            / (2147483647 as libc::c_int as libc::c_double + 1.0f64)))
                    as libc::c_float;
                j += 1;
            }
            k += 1;
        }
        i += 1;
    }
}
pub unsafe extern "C" fn AnnScaleWeights(mut net: *mut Ann, mut factor: libc::c_float) {
    let mut j: libc::c_int = 0;
    let mut layers: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    layers = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        units = (*((*net).layer).offset(j as isize)).units;
        weights = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*net).layer).offset(j as isize)).weight).offset(i as isize) *= factor;
            i += 1;
        }
        j += 1;
    }
}
pub unsafe extern "C" fn AnnUpdateSgradient(mut net: *mut Ann) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut layers: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    layers = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        units = (*((*net).layer).offset(j as isize)).units;
        weights = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*net).layer).offset(j as isize)).sgradient).offset(i as isize)
                += *((*((*net).layer).offset(j as isize)).gradient).offset(i as isize);
            i += 1;
        }
        j += 1;
    }
}
pub unsafe extern "C" fn sign(mut n: libc::c_float) -> libc::c_float {
    if n > 0 as libc::c_int as libc::c_float {
        return 1 as libc::c_int as libc::c_float;
    }
    if n < 0 as libc::c_int as libc::c_float {
        return -(1 as libc::c_int) as libc::c_float;
    }
    return 0 as libc::c_int as libc::c_float;
}
pub unsafe extern "C" fn AnnAdjustWeightsResilientBP(mut net: *mut Ann) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut layers: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    let mut t: libc::c_float = 0.;
    let mut delta: libc::c_float = 0.;
    let mut wdelta: libc::c_float = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut past_wdelta: libc::c_float = 0.;
    let mut tmp___0: libc::c_float = 0.;
    let mut wdelta___0: libc::c_float = 0.;
    let mut tmp___1: libc::c_float = 0.;
    layers = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        units = (*((*net).layer).offset(j as isize)).units;
        weights = units * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units
            - (j - 1 as libc::c_int > 0 as libc::c_int) as libc::c_int;
        i = 0 as libc::c_int;
        while i < weights {
            t = *((*((*net).layer).offset(j as isize)).pgradient).offset(i as isize)
                * *((*((*net).layer).offset(j as isize)).sgradient).offset(i as isize);
            delta = *((*((*net).layer).offset(j as isize)).delta).offset(i as isize);
            if t > 0 as libc::c_int as libc::c_float {
                if delta * (*net).rprop_nplus < (*net).rprop_maxupdate {
                    delta *= (*net).rprop_nplus;
                } else {
                    delta = (*net).rprop_maxupdate;
                }
                tmp = sign(
                    *((*((*net).layer).offset(j as isize)).sgradient).offset(i as isize),
                );
                wdelta = -tmp * delta;
                *((*((*net).layer).offset(j as isize)).weight).offset(i as isize)
                    += wdelta;
                *((*((*net).layer).offset(j as isize)).delta).offset(i as isize) = delta;
                *((*((*net).layer).offset(j as isize)).pgradient)
                    .offset(
                        i as isize,
                    ) = *((*((*net).layer).offset(j as isize)).sgradient)
                    .offset(i as isize);
            } else if t < 0 as libc::c_int as libc::c_float {
                tmp___0 = sign(
                    *((*((*net).layer).offset(j as isize)).pgradient).offset(i as isize),
                );
                past_wdelta = -tmp___0 * delta;
                if delta * (*net).rprop_nminus > (*net).rprop_minupdate {
                    delta *= (*net).rprop_nminus;
                } else {
                    delta = (*net).rprop_minupdate;
                }
                *((*((*net).layer).offset(j as isize)).weight).offset(i as isize)
                    -= past_wdelta;
                *((*((*net).layer).offset(j as isize)).delta).offset(i as isize) = delta;
                *((*((*net).layer).offset(j as isize)).pgradient)
                    .offset(i as isize) = 0 as libc::c_int as libc::c_float;
            } else {
                tmp___1 = sign(
                    *((*((*net).layer).offset(j as isize)).sgradient).offset(i as isize),
                );
                wdelta___0 = -tmp___1 * delta;
                *((*((*net).layer).offset(j as isize)).weight).offset(i as isize)
                    += wdelta___0;
                *((*((*net).layer).offset(j as isize)).pgradient)
                    .offset(
                        i as isize,
                    ) = *((*((*net).layer).offset(j as isize)).sgradient)
                    .offset(i as isize);
            }
            i += 1;
        }
        j += 1;
    }
}
pub unsafe extern "C" fn AnnResilientBPEpoch(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desired: *mut libc::c_float,
    mut setlen: libc::c_int,
) -> libc::c_float {
    let mut error: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut inputs: libc::c_int = 0;
    let mut outputs: libc::c_int = 0;
    let mut tmp: libc::c_float = 0.;
    error = 0 as libc::c_int as libc::c_float;
    inputs = (*((*net).layer).offset(((*net).layers - 1 as libc::c_int) as isize)).units
        - 1 as libc::c_int;
    outputs = (*((*net).layer).offset(0 as libc::c_int as isize)).units;
    AnnResetSgradient(net);
    j = 0 as libc::c_int;
    while j < setlen {
        tmp = AnnSimulateError(net, input, desired);
        error += tmp;
        AnnCalculateGradients(net, desired);
        AnnUpdateSgradient(net);
        input = input.offset(inputs as isize);
        desired = desired.offset(outputs as isize);
        j += 1;
    }
    AnnAdjustWeightsResilientBP(net);
    return error / setlen as libc::c_float;
}
pub unsafe extern "C" fn AnnUpdateDeltasGD(mut net: *mut Ann) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut layers: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    layers = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        units = (*((*net).layer).offset(j as isize)).units;
        weights = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*net).layer).offset(j as isize)).delta).offset(i as isize)
                += *((*((*net).layer).offset(j as isize)).gradient).offset(i as isize);
            i += 1;
        }
        j += 1;
    }
}
pub unsafe extern "C" fn AnnAdjustWeights(mut net: *mut Ann, mut setlen: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut layers: libc::c_int = 0;
    let mut units: libc::c_int = 0;
    let mut weights: libc::c_int = 0;
    layers = (*net).layers;
    j = 1 as libc::c_int;
    while j < layers {
        units = (*((*net).layer).offset(j as isize)).units;
        weights = units
            * (*((*net).layer).offset((j - 1 as libc::c_int) as isize)).units;
        i = 0 as libc::c_int;
        while i < weights {
            *((*((*net).layer).offset(j as isize)).weight).offset(i as isize)
                -= (*net).learn_rate / setlen as libc::c_float
                    * *((*((*net).layer).offset(j as isize)).delta).offset(i as isize);
            i += 1;
        }
        j += 1;
    }
}
pub unsafe extern "C" fn AnnGDEpoch(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desidered: *mut libc::c_float,
    mut setlen: libc::c_int,
) -> libc::c_float {
    let mut error: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut inputs: libc::c_int = 0;
    let mut outputs: libc::c_int = 0;
    let mut tmp: libc::c_float = 0.;
    error = 0 as libc::c_int as libc::c_float;
    inputs = (*((*net).layer).offset(((*net).layers - 1 as libc::c_int) as isize)).units
        - 1 as libc::c_int;
    outputs = (*((*net).layer).offset(0 as libc::c_int as isize)).units;
    j = 0 as libc::c_int;
    while j < setlen {
        AnnSetDeltas(net, 0 as libc::c_int as libc::c_float);
        tmp = AnnSimulateError(net, input, desidered);
        error += tmp;
        AnnCalculateGradients(net, desidered);
        AnnUpdateDeltasGD(net);
        input = input.offset(inputs as isize);
        desidered = desidered.offset(outputs as isize);
        AnnAdjustWeights(net, setlen);
        j += 1;
    }
    return error / setlen as libc::c_float;
}
pub unsafe extern "C" fn AnnTestClassError(
    mut net: *mut Ann,
    mut desired: *mut libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut outputs: libc::c_int = 0;
    let mut classid: libc::c_int = 0;
    let mut outid: libc::c_int = 0;
    let mut max: libc::c_float = 0.;
    let mut o: libc::c_float = 0.;
    outputs = (*((*net).layer).offset(0 as libc::c_int as isize)).units;
    max = 0 as libc::c_int as libc::c_float;
    classid = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < outputs {
        if *desired.offset(i as isize) == 1 as libc::c_int as libc::c_float {
            break;
        }
        i += 1;
    }
    classid = i;
    max = *((*((*net).layer).offset(0 as libc::c_int as isize)).output)
        .offset(0 as libc::c_int as isize);
    outid = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < outputs {
        o = *((*((*net).layer).offset(0 as libc::c_int as isize)).output)
            .offset(i as isize);
        if o > max {
            outid = i;
            max = o;
        }
        i += 1;
    }
    return (outid != classid) as libc::c_int;
}
pub unsafe extern "C" fn AnnTestError(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desired: *mut libc::c_float,
    mut setlen: libc::c_int,
    mut avgerr: *mut libc::c_float,
    mut classerr: *mut libc::c_float,
) {
    let mut error: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut inputs: libc::c_int = 0;
    let mut outputs: libc::c_int = 0;
    let mut class_errors: libc::c_int = 0;
    let mut tmp: libc::c_float = 0.;
    let mut tmp___0: libc::c_int = 0;
    error = 0 as libc::c_int as libc::c_float;
    inputs = (*((*net).layer).offset(((*net).layers - 1 as libc::c_int) as isize)).units
        - 1 as libc::c_int;
    outputs = (*((*net).layer).offset(0 as libc::c_int as isize)).units;
    class_errors = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < setlen {
        tmp = AnnSimulateError(net, input, desired);
        error += tmp;
        if !classerr.is_null() {
            tmp___0 = AnnTestClassError(net, desired);
            class_errors += tmp___0;
        }
        input = input.offset(inputs as isize);
        desired = desired.offset(outputs as isize);
        j += 1;
    }
    if !avgerr.is_null() {
        *avgerr = error / setlen as libc::c_float;
    }
    if !classerr.is_null() {
        *classerr = class_errors as libc::c_float * 100 as libc::c_int as libc::c_float
            / setlen as libc::c_float;
    }
}
pub unsafe extern "C" fn AnnTrainWithAlgoFunc(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desired: *mut libc::c_float,
    mut maxerr: libc::c_float,
    mut maxepochs: libc::c_int,
    mut setlen: libc::c_int,
    mut algo_func: Option::<
        unsafe extern "C" fn(
            *mut Ann,
            *mut libc::c_float,
            *mut libc::c_float,
            libc::c_int,
        ) -> libc::c_float,
    >,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_float = 0.;
    let mut tmp: libc::c_int = 0;
    i = 0 as libc::c_int;
    e = maxerr + 1 as libc::c_int as libc::c_float;
    loop {
        tmp = i;
        i += 1;
        if !(tmp < maxepochs) {
            break;
        }
        if !(e >= maxerr) {
            break;
        }
        e = (Some(algo_func.expect("non-null function pointer")))
            .expect("non-null function pointer")(net, input, desired, setlen);
    }
    return e;
}
pub unsafe extern "C" fn AnnTrain(
    mut net: *mut Ann,
    mut input: *mut libc::c_float,
    mut desired: *mut libc::c_float,
    mut maxerr: libc::c_float,
    mut maxepochs: libc::c_int,
    mut setlen: libc::c_int,
    mut algo: libc::c_int,
) -> libc::c_float {
    let mut algo_func: Option::<
        unsafe extern "C" fn(
            *mut Ann,
            *mut libc::c_float,
            *mut libc::c_float,
            libc::c_int,
        ) -> libc::c_float,
    > = None;
    let mut tmp: libc::c_float = 0.;
    if algo == 0 as libc::c_int {
        algo_func = Some(
            AnnResilientBPEpoch
                as unsafe extern "C" fn(
                    *mut Ann,
                    *mut libc::c_float,
                    *mut libc::c_float,
                    libc::c_int,
                ) -> libc::c_float,
        );
    } else if algo == 1 as libc::c_int {
        algo_func = Some(
            AnnGDEpoch
                as unsafe extern "C" fn(
                    *mut Ann,
                    *mut libc::c_float,
                    *mut libc::c_float,
                    libc::c_int,
                ) -> libc::c_float,
        );
    } else {
        return -(1 as libc::c_int) as libc::c_float
    }
    tmp = AnnTrainWithAlgoFunc(
        net,
        input,
        desired,
        maxerr,
        maxepochs,
        setlen,
        algo_func,
    );
    return tmp;
}
