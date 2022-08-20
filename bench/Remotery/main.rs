use ::libc;
use ::c2rust_asm_casts;
use std::arch::asm;
use c2rust_asm_casts::AsmCastTrait;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rand() -> libc::c_int;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn signal(
        __sig: libc::c_int,
        __handler: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    ) -> __sighandler_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn clock() -> clock_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn mbstowcs(__pwcs: *mut wchar_t, __s: *const libc::c_char, __n: size_t) -> size_t;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
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
    fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn pthread_key_delete(__key: pthread_key_t) -> libc::c_int;
    fn pthread_getspecific(__key: pthread_key_t) -> *mut libc::c_void;
    fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const libc::c_void,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off_t) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(__fd: libc::c_int, __addr: *const sockaddr, __len: socklen_t) -> libc::c_int;
    fn send(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
    ) -> ssize_t;
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
    fn shutdown(__fd: libc::c_int, __how: libc::c_int) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type rmtU32 = libc::c_uint;
pub type rmtPStr = *const libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Remotery {
    pub server: *mut Server,
    pub timer: usTimer,
    pub mq_to_rmt_thread: *mut rmtMessageQueue,
    pub thread: *mut rmtThread,
    pub string_table: *mut StringTable,
    pub logfile: *mut FILE,
    pub map_message_queue_fn: Option::<
        unsafe extern "C" fn(*mut Remotery, *mut Message) -> (),
    >,
    pub map_message_queue_data: *mut libc::c_void,
    pub threadProfilers: *mut ThreadProfilers,
    pub propertyMutex: rmtMutex,
    pub rootProperty: rmtProperty,
    pub propertyAllocator: *mut ObjectAllocator,
    pub propertyFrame: rmtU32,
}
pub type ObjectAllocator = __anonstruct_ObjectAllocator_929114526;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_ObjectAllocator_929114526 {
    pub object_size: rmtU32,
    pub constructor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> rmtError>,
    pub destructor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub nb_free: rmtAtomicS32,
    pub nb_inuse: rmtAtomicS32,
    pub nb_allocated: rmtAtomicS32,
    pub first_free: *mut ObjectLink,
}
pub type ObjectLink = ObjectLink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ObjectLink_s {
    pub next: *mut ObjectLink_s,
}
pub type rmtAtomicS32 = rmtS32;
pub type rmtS32 = libc::c_int;
pub type rmtError = libc::c_uint;
pub const RMT_ERROR_CUDA_UNKNOWN: rmtError = 43;
pub const RMT_ERROR_OPENGL_ERROR: rmtError = 42;
pub const RMT_ERROR_D3D11_FAILED_TO_CREATE_QUERY: rmtError = 41;
pub const RMT_ERROR_ERROR_NOT_READY: rmtError = 40;
pub const RMT_ERROR_CUDA_OUT_OF_MEMORY: rmtError = 39;
pub const RMT_ERROR_CUDA_INVALID_HANDLE: rmtError = 38;
pub const RMT_ERROR_CUDA_INVALID_VALUE: rmtError = 37;
pub const RMT_ERROR_CUDA_INVALID_CONTEXT: rmtError = 36;
pub const RMT_ERROR_CUDA_NOT_INITIALIZED: rmtError = 35;
pub const RMT_ERROR_CUDA_DEINITIALIZED: rmtError = 34;
pub const RMT_ERROR_SEND_ON_INCOMPLETE_PROFILE: rmtError = 33;
pub const RMT_ERROR_REMOTERY_NOT_CREATED: rmtError = 32;
pub const RMT_ERROR_WEBSOCKET_RECEIVE_TIMEOUT: rmtError = 31;
pub const RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER_MASK: rmtError = 30;
pub const RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER_SIZE: rmtError = 29;
pub const RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER: rmtError = 28;
pub const RMT_ERROR_WEBSOCKET_DISCONNECTED: rmtError = 27;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL: rmtError = 26;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_KEY: rmtError = 25;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_KEY: rmtError = 24;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_HOST: rmtError = 23;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_HOST: rmtError = 22;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_VERSION: rmtError = 21;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_VERSION: rmtError = 20;
pub const RMT_ERROR_WEBSOCKET_HANDSHAKE_NOT_GET: rmtError = 19;
pub const RMT_ERROR_SOCKET_RECV_FAILED: rmtError = 18;
pub const RMT_ERROR_SOCKET_RECV_TIMEOUT: rmtError = 17;
pub const RMT_ERROR_SOCKET_RECV_NO_DATA: rmtError = 16;
pub const RMT_ERROR_SOCKET_SEND_FAIL: rmtError = 15;
pub const RMT_ERROR_SOCKET_POLL_ERRORS: rmtError = 14;
pub const RMT_ERROR_SOCKET_SELECT_FAIL: rmtError = 13;
pub const RMT_ERROR_SOCKET_INVALID_POLL: rmtError = 12;
pub const RMT_ERROR_OPEN_THREAD_HANDLE_FAIL: rmtError = 11;
pub const RMT_ERROR_CREATE_THREAD_FAIL: rmtError = 10;
pub const RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL: rmtError = 9;
pub const RMT_ERROR_TLS_ALLOC_FAIL: rmtError = 8;
pub const RMT_ERROR_MALLOC_FAIL: rmtError = 7;
pub const RMT_ERROR_TIMEOUT: rmtError = 6;
pub const RMT_ERROR_RESOURCE_ACCESS_FAIL: rmtError = 5;
pub const RMT_ERROR_RESOURCE_CREATE_FAIL: rmtError = 4;
pub const RMT_ERROR_INVALID_INPUT: rmtError = 3;
pub const RMT_ERROR_UNKNOWN: rmtError = 2;
pub const RMT_ERROR_RECURSIVE_SAMPLE: rmtError = 1;
pub const RMT_ERROR_NONE: rmtError = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtProperty {
    pub initialised: rmtBool,
    pub type_0: rmtPropertyType,
    pub flags: rmtPropertyFlags,
    pub value: rmtPropertyValue,
    pub lastFrameValue: rmtPropertyValue,
    pub prevValue: rmtPropertyValue,
    pub prevValueFrame: rmtU32,
    pub name: *const libc::c_char,
    pub description: *const libc::c_char,
    pub defaultValue: rmtPropertyValue,
    pub parent: *mut rmtProperty,
    pub firstChild: *mut rmtProperty,
    pub lastChild: *mut rmtProperty,
    pub nextSibling: *mut rmtProperty,
    pub nameHash: rmtU32,
    pub uniqueID: rmtU32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union rmtPropertyValue {
    pub Bool: rmtBool,
    pub S32: rmtS32,
    pub U32: rmtU32,
    pub F32: rmtF32,
    pub S64: rmtS64,
    pub U64: rmtU64,
    pub F64: rmtF64,
}
pub type rmtF64 = libc::c_double;
pub type rmtU64 = libc::c_ulonglong;
pub type rmtS64 = libc::c_longlong;
pub type rmtF32 = libc::c_float;
pub type rmtBool = libc::c_uint;
pub type rmtPropertyFlags = __anonenum_rmtPropertyFlags_993090800;
pub type __anonenum_rmtPropertyFlags_993090800 = libc::c_uint;
pub const RMT_PropertyFlags_FrameReset: __anonenum_rmtPropertyFlags_993090800 = 1;
pub const RMT_PropertyFlags_NoFlags: __anonenum_rmtPropertyFlags_993090800 = 0;
pub type rmtPropertyType = __anonenum_rmtPropertyType_767673012;
pub type __anonenum_rmtPropertyType_767673012 = libc::c_uint;
pub const RMT_PropertyType_rmtF64: __anonenum_rmtPropertyType_767673012 = 7;
pub const RMT_PropertyType_rmtU64: __anonenum_rmtPropertyType_767673012 = 6;
pub const RMT_PropertyType_rmtS64: __anonenum_rmtPropertyType_767673012 = 5;
pub const RMT_PropertyType_rmtF32: __anonenum_rmtPropertyType_767673012 = 4;
pub const RMT_PropertyType_rmtU32: __anonenum_rmtPropertyType_767673012 = 3;
pub const RMT_PropertyType_rmtS32: __anonenum_rmtPropertyType_767673012 = 2;
pub const RMT_PropertyType_rmtBool: __anonenum_rmtPropertyType_767673012 = 1;
pub const RMT_PropertyType_rmtGroup: __anonenum_rmtPropertyType_767673012 = 0;
pub type rmtMutex = pthread_mutex_t;
pub type pthread_mutex_t = __anonunion_pthread_mutex_t_335460617;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_pthread_mutex_t_335460617 {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
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
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadProfilers {
    pub timer: *mut usTimer,
    pub mqToRmtThread: *mut rmtMessageQueue,
    pub compiledSampleFn: *mut libc::c_void,
    pub compiledSampleFnSize: rmtU32,
    pub threadProfilerTlsHandle: rmtTLS,
    pub threadProfilers: [ThreadProfiler; 256],
    pub nbThreadProfilers: rmtAtomicU32,
    pub maxNbThreadProfilers: rmtU32,
    pub threadProfilerMutex: rmtMutex,
    pub threadSampleThread: *mut rmtThread,
    pub threadGatherThread: *mut rmtThread,
}
pub type rmtThread = Thread_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Thread_t {
    pub handle: rmtThreadHandle,
    pub callback: Option::<unsafe extern "C" fn(*mut rmtThread) -> rmtError>,
    pub param: *mut libc::c_void,
    pub error: rmtError,
    pub request_exit: rmtBool,
}
pub type rmtThreadHandle = pthread_t;
pub type pthread_t = libc::c_ulong;
pub type rmtAtomicU32 = rmtU32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadProfiler {
    pub registerBackup0: rmtU64,
    pub registerBackup1: rmtU64,
    pub registerBackup2: rmtU64,
    pub nbSamplesWithoutCallback: rmtAtomicS32,
    pub processorIndex: rmtU32,
    pub lastProcessorIndex: rmtU32,
    pub threadId: rmtThreadId,
    pub threadHandle: rmtThreadHandle,
    pub threadName: [libc::c_char; 64],
    pub threadNameHash: rmtU32,
    pub sampleTrees: [*mut SampleTree; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SampleTree {
    pub allocator: *mut ObjectAllocator,
    pub root: *mut Sample,
    pub currentParent: *mut Sample,
    pub msLastTreeSendTime: rmtAtomicU32,
    pub treeBeingModified: rmtAtomicU32,
    pub sendSampleOnClose: *mut Sample,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sample {
    pub Link: ObjectLink,
    pub type_0: rmtSampleType,
    pub name_hash: rmtU32,
    pub unique_id: rmtU32,
    pub uniqueColour: [rmtU8; 3],
    pub parent: *mut Sample,
    pub first_child: *mut Sample,
    pub last_child: *mut Sample,
    pub next_sibling: *mut Sample,
    pub nb_children: rmtU32,
    pub us_start: rmtU64,
    pub us_end: rmtU64,
    pub us_length: rmtU64,
    pub us_sampled_length: rmtU64,
    pub usGpuIssueOnCpu: rmtU64,
    pub call_count: rmtU32,
    pub recurse_depth: rmtU16,
    pub max_recurse_depth: rmtU16,
}
pub type rmtU16 = libc::c_ushort;
pub type rmtU8 = libc::c_uchar;
pub type rmtSampleType = libc::c_uint;
pub const RMT_SampleType_Count: rmtSampleType = 6;
pub const RMT_SampleType_Metal: rmtSampleType = 5;
pub const RMT_SampleType_OpenGL: rmtSampleType = 4;
pub const RMT_SampleType_D3D12: rmtSampleType = 3;
pub const RMT_SampleType_D3D11: rmtSampleType = 2;
pub const RMT_SampleType_CUDA: rmtSampleType = 1;
pub const RMT_SampleType_CPU: rmtSampleType = 0;
pub type rmtThreadId = uintptr_t;
pub type uintptr_t = libc::c_ulong;
pub type rmtTLS = pthread_key_t;
pub type pthread_key_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtMessageQueue {
    pub size: rmtU32,
    pub data: *mut VirtualMirrorBuffer,
    pub read_pos: rmtAtomicU32,
    pub write_pos: rmtAtomicU32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VirtualMirrorBuffer {
    pub size: rmtU32,
    pub ptr: *mut rmtU8,
}
pub type usTimer = __anonstruct_usTimer_887188579;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_usTimer_887188579 {
    pub counter_start: LARGE_INTEGER,
    pub counter_scale: libc::c_double,
}
pub type LARGE_INTEGER = rmtU64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Message {
    pub id: MessageID,
    pub payload_size: rmtU32,
    pub threadProfiler: *mut ThreadProfiler,
    pub payload: [rmtU8; 1],
}
pub type MessageID = libc::c_uint;
pub const MsgID_Force32Bits: MessageID = 4294967295;
pub const MsgID_PropertySnapshot: MessageID = 6;
pub const MsgID_None: MessageID = 5;
pub const MsgID_ProcessorThreads: MessageID = 4;
pub const MsgID_SampleTree: MessageID = 3;
pub const MsgID_LogText: MessageID = 2;
pub const MsgID_AddToStringTable: MessageID = 1;
pub const MsgID_NotReady: MessageID = 0;
pub type FILE = _IO_FILE;
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
pub type size_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub type StringTable = __anonstruct_StringTable_515543109;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_StringTable_515543109 {
    pub text: *mut Buffer,
    pub text_map: *mut rmtHashTable,
}
pub type rmtHashTable = __anonstruct_rmtHashTable_1056137675;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_rmtHashTable_1056137675 {
    pub maxNbSlots: rmtU32,
    pub nbSlots: rmtU32,
    pub slots: *mut HashSlot,
}
pub type HashSlot = __anonstruct_HashSlot_851742752;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_HashSlot_851742752 {
    pub key: rmtU32,
    pub value: rmtU64,
}
pub type Buffer = __anonstruct_Buffer_678719378;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_Buffer_678719378 {
    pub alloc_granularity: rmtU32,
    pub bytes_allocated: rmtU32,
    pub bytes_used: rmtU32,
    pub data: *mut rmtU8,
}
pub type Server = __anonstruct_Server_646260831;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_Server_646260831 {
    pub listen_socket: *mut WebSocket,
    pub client_socket: *mut WebSocket,
    pub last_ping_time: rmtU32,
    pub port: rmtU16,
    pub reuse_open_port: rmtBool,
    pub limit_connections_to_localhost: rmtBool,
    pub bin_buf: *mut Buffer,
    pub receive_handler: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, rmtU32) -> rmtError,
    >,
    pub receive_handler_context: *mut libc::c_void,
}
pub type WebSocket = __anonstruct_WebSocket_37999051;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_WebSocket_37999051 {
    pub tcp_socket: *mut TCPSocket,
    pub mode: WebSocketMode,
    pub frame_bytes_remaining: rmtU32,
    pub mask_offset: rmtU32,
    pub data: __anonunion_data_769749382,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_data_769749382 {
    pub mask: [rmtU8; 4],
    pub mask_u32: rmtU32,
}
pub type WebSocketMode = libc::c_uint;
pub const WEBSOCKET_BINARY: WebSocketMode = 2;
pub const WEBSOCKET_TEXT: WebSocketMode = 1;
pub const WEBSOCKET_NONE: WebSocketMode = 0;
pub type TCPSocket = __anonstruct_TCPSocket_44120681;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_TCPSocket_44120681 {
    pub socket: SOCKET,
}
pub type SOCKET = libc::c_int;
pub type rmtS8 = libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Msg_SampleTree {
    pub rootSample: *mut Sample,
    pub allocator: *mut ObjectAllocator,
    pub threadName: rmtPStr,
    pub userData: rmtU32,
    pub partialTree: rmtBool,
}
pub type rmtSampleTree = Msg_SampleTree;
pub type rmtSample = Sample;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtSettings {
    pub port: rmtU16,
    pub reuse_open_port: rmtBool,
    pub limit_connections_to_localhost: rmtBool,
    pub enableThreadSampler: rmtBool,
    pub msSleepBetweenServerUpdates: rmtU32,
    pub messageQueueSizeInBytes: rmtU32,
    pub maxNbMessagesPerUpdate: rmtU32,
    pub malloc: Option::<
        unsafe extern "C" fn(*mut libc::c_void, rmtU32) -> *mut libc::c_void,
    >,
    pub realloc: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            rmtU32,
        ) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub mm_context: *mut libc::c_void,
    pub input_handler: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
    >,
    pub sampletree_handler: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut rmtSampleTree) -> (),
    >,
    pub sampletree_context: *mut libc::c_void,
    pub snapshot_callback: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut rmtProperty) -> (),
    >,
    pub snapshot_context: *mut libc::c_void,
    pub input_handler_context: *mut libc::c_void,
    pub logPath: rmtPStr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtSampleIterator {
    pub sample: *mut rmtSample,
    pub initial: *mut rmtSample,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rmtPropertyIterator {
    pub property: *mut rmtProperty,
    pub initial: *mut rmtProperty,
}
pub type wchar_t = libc::c_int;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type ssize_t = __ssize_t;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
pub type errno_t = libc::c_int;
pub type r_size_t = libc::c_uint;
pub type rmtCpuContext = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_u_919609132 {
    pub i: libc::c_uint,
    pub c: [libc::c_uchar; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_u_682901414 {
    pub d: libc::c_double,
    pub c: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_SocketStatus_11743799 {
    pub can_read: rmtBool,
    pub can_write: rmtBool,
    pub error_state: rmtError,
}
pub type SocketStatus = __anonstruct_SocketStatus_11743799;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_SHA1_345707942 {
    pub data: [rmtU8; 20],
}
pub type SHA1 = __anonstruct_SHA1_345707942;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Msg_AddToStringTable {
    pub hash: rmtU32,
    pub length: rmtU32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Processor {
    pub threadProfiler: *mut ThreadProfiler,
    pub sampleCount: rmtU32,
    pub sampleTime: rmtU64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Msg_ProcessorThreads {
    pub messageIndex: rmtU64,
    pub nbProcessors: rmtU32,
    pub processors: [Processor; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PropertySnapshot {
    pub Link: ObjectLink,
    pub type_0: rmtPropertyType,
    pub value: rmtPropertyValue,
    pub prevValue: rmtPropertyValue,
    pub prevValueFrame: rmtU32,
    pub nameHash: rmtU32,
    pub uniqueID: rmtU32,
    pub depth: rmtU8,
    pub nbChildren: rmtU32,
    pub nextSnapshot: *mut PropertySnapshot,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Msg_PropertySnapshot {
    pub rootSnapshot: *mut PropertySnapshot,
    pub nbSnapshots: rmtU32,
    pub propertyFrame: rmtU32,
}
static mut rmt_sample_hash_aggregate: rmtU32 = 0 as libc::c_int as rmtU32;
pub unsafe extern "C" fn aggregateFunction() {
    _rmt_BeginCPUSample(
        b"aggregate\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as rmtU32,
        &mut rmt_sample_hash_aggregate,
    );
    _rmt_EndCPUSample();
}
static mut rmt_sample_hash_recursive: rmtU32 = 0 as libc::c_int as rmtU32;
pub unsafe extern "C" fn recursiveFunction(mut depth: libc::c_int) {
    _rmt_BeginCPUSample(
        b"recursive\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as rmtU32,
        &mut rmt_sample_hash_recursive,
    );
    if depth < 5 as libc::c_int {
        recursiveFunction(depth + 1 as libc::c_int);
    }
    _rmt_EndCPUSample();
}
static mut rmt_sample_hash_delay: rmtU32 = 0 as libc::c_int as rmtU32;
pub unsafe extern "C" fn delay() -> libc::c_double {
    let mut i: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut j: libc::c_double = 0.;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_double = 0.;
    j = 0 as libc::c_int as libc::c_double;
    _rmt_BeginCPUSample(
        b"delay\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int as rmtU32,
        &mut rmt_sample_hash_delay,
    );
    i = 0 as libc::c_int;
    tmp = rand();
    end = tmp / 100 as libc::c_int;
    while i < end {
        tmp___0 = sin(i as libc::c_double);
        j += tmp___0;
        i += 1;
    }
    recursiveFunction(0 as libc::c_int);
    aggregateFunction();
    aggregateFunction();
    aggregateFunction();
    _rmt_EndCPUSample();
    return j;
}
pub static mut sig: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn sigintHandler(mut sig_num: libc::c_int) {
    sig = sig_num;
    printf(b"Interrupted\n\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    let mut rmt: *mut Remotery = 0 as *mut Remotery;
    let mut error: rmtError = RMT_ERROR_NONE;
    signal(
        2 as libc::c_int,
        Some(sigintHandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    error = _rmt_CreateGlobalInstance(&mut rmt);
    if 0 as libc::c_uint != error as libc::c_uint {
        printf(
            b"Error launching Remotery %d\n\0" as *const u8 as *const libc::c_char,
            error as libc::c_uint,
        );
        return -(1 as libc::c_int);
    }
    while sig == 0 as libc::c_int {
        _rmt_LogText(b"start profiling\0" as *const u8 as *const libc::c_char);
        delay();
        _rmt_LogText(b"end profiling\0" as *const u8 as *const libc::c_char);
    }
    _rmt_DestroyGlobalInstance(rmt);
    printf(b"Cleaned up and quit\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
static mut g_Settings: rmtSettings = rmtSettings {
    port: 0,
    reuse_open_port: 0,
    limit_connections_to_localhost: 0,
    enableThreadSampler: 0,
    msSleepBetweenServerUpdates: 0,
    messageQueueSizeInBytes: 0,
    maxNbMessagesPerUpdate: 0,
    malloc: None,
    realloc: None,
    free: None,
    mm_context: 0 as *const libc::c_void as *mut libc::c_void,
    input_handler: None,
    sampletree_handler: None,
    sampletree_context: 0 as *const libc::c_void as *mut libc::c_void,
    snapshot_callback: None,
    snapshot_context: 0 as *const libc::c_void as *mut libc::c_void,
    input_handler_context: 0 as *const libc::c_void as *mut libc::c_void,
    logPath: 0 as *const libc::c_char,
};
static mut g_SettingsInitialized: rmtBool = 0 as libc::c_int as rmtBool;
unsafe extern "C" fn maxU16(mut a: rmtU16, mut b: rmtU16) -> rmtU16 {
    let mut tmp: libc::c_int = 0;
    if a as libc::c_int > b as libc::c_int {
        tmp = a as libc::c_int;
    } else {
        tmp = b as libc::c_int;
    }
    return tmp as rmtU16;
}
unsafe extern "C" fn minS32(mut a: rmtS32, mut b: rmtS32) -> rmtS32 {
    let mut tmp: rmtS32 = 0;
    if a < b {
        tmp = a;
    } else {
        tmp = b;
    }
    return tmp;
}
unsafe extern "C" fn maxS32(mut a: rmtS32, mut b: rmtS32) -> rmtS32 {
    let mut tmp: rmtS32 = 0;
    if a > b {
        tmp = a;
    } else {
        tmp = b;
    }
    return tmp;
}
unsafe extern "C" fn minU32(mut a: rmtU32, mut b: rmtU32) -> rmtU32 {
    let mut tmp: rmtU32 = 0;
    if a < b {
        tmp = a;
    } else {
        tmp = b;
    }
    return tmp;
}
unsafe extern "C" fn maxS64(mut a: rmtS64, mut b: rmtS64) -> rmtS64 {
    let mut tmp: rmtS64 = 0;
    if a > b {
        tmp = a;
    } else {
        tmp = b;
    }
    return tmp;
}
unsafe extern "C" fn rmtMalloc(mut size: rmtU32) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = (Some((g_Settings.malloc).expect("non-null function pointer")))
        .expect("non-null function pointer")(g_Settings.mm_context, size);
    return tmp;
}
unsafe extern "C" fn rmtRealloc(
    mut ptr: *mut libc::c_void,
    mut size: rmtU32,
) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = (Some((g_Settings.realloc).expect("non-null function pointer")))
        .expect("non-null function pointer")(g_Settings.mm_context, ptr, size);
    return tmp;
}
unsafe extern "C" fn rmtFree(mut ptr: *mut libc::c_void) {
    (Some((g_Settings.free).expect("non-null function pointer")))
        .expect("non-null function pointer")(g_Settings.mm_context, ptr);
}
unsafe extern "C" fn rmtOpenFile(
    mut filename: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> *mut FILE {
    let mut tmp: *mut FILE = 0 as *mut FILE;
    tmp = fopen(filename, mode);
    return tmp;
}
pub unsafe extern "C" fn rmtCloseFile(mut fp: *mut FILE) {
    if fp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        fclose(fp);
    }
}
pub unsafe extern "C" fn rmtWriteFile(
    mut fp: *mut FILE,
    mut data: *const libc::c_void,
    mut size: rmtU32,
) -> rmtBool {
    let mut tmp___1: rmtBool = 0;
    let mut tmp___2: size_t = 0;
    if !(fp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"fp != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_uint,
            b"rmtWriteFile\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___2 = fwrite(data, size as size_t, 1 as libc::c_int as size_t, fp);
    if tmp___2 == size as size_t {
        tmp___1 = 1 as libc::c_int as rmtBool;
    } else {
        tmp___1 = 0 as libc::c_int as rmtBool;
    }
    return tmp___1;
}
unsafe extern "C" fn msTimer_Get() -> rmtU32 {
    let mut time___0: clock_t = 0;
    let mut tmp: clock_t = 0;
    let mut msTime: rmtU32 = 0;
    tmp = clock();
    time___0 = tmp;
    msTime = (time___0 / 1000 as libc::c_long) as rmtU32;
    return msTime;
}
unsafe extern "C" fn usTimer_Init(mut timer: *mut usTimer) {
    let mut tv: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(0 as libc::c_int, &mut tv);
    (*timer)
        .counter_start = (tv.tv_sec as rmtU64)
        .wrapping_mul(1000000 as libc::c_ulonglong)
        .wrapping_add((tv.tv_nsec as libc::c_double * 0.001f64) as rmtU64);
}
unsafe extern "C" fn usTimer_Get(mut timer: *mut usTimer) -> rmtU64 {
    let mut tv: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(0 as libc::c_int, &mut tv);
    return (tv.tv_sec as rmtU64)
        .wrapping_mul(1000000 as libc::c_ulonglong)
        .wrapping_add((tv.tv_nsec as libc::c_double * 0.001f64) as rmtU64)
        .wrapping_sub((*timer).counter_start);
}
unsafe extern "C" fn msSleep(mut time_ms: rmtU32) {
    usleep(time_ms.wrapping_mul(1000 as libc::c_uint));
}
unsafe extern "C" fn TimeDateNow() -> *mut tm {
    let mut time_now: time_t = 0;
    let mut tmp: time_t = 0;
    let mut tmp___0: *mut tm = 0 as *mut tm;
    tmp = time(0 as *mut libc::c_void as *mut time_t);
    time_now = tmp;
    tmp___0 = gmtime(&mut time_now as *mut time_t as *const time_t);
    return tmp___0;
}
unsafe extern "C" fn tlsAlloc(mut handle: *mut rmtTLS) -> rmtError {
    let mut tmp___0: libc::c_int = 0;
    if !(handle as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"handle != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_uint,
            b"tlsAlloc\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = pthread_key_create(
        handle,
        ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(0 as *mut libc::c_void),
    );
    if tmp___0 != 0 as libc::c_int {
        *handle = 4294967295 as libc::c_uint;
        return RMT_ERROR_TLS_ALLOC_FAIL;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn tlsFree(mut handle: rmtTLS) {
    if !(handle != 4294967295 as libc::c_uint) {
        __assert_fail(
            b"handle != TLS_INVALID_HANDLE\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_uint,
            b"tlsFree\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_key_delete(handle);
}
unsafe extern "C" fn tlsSet(mut handle: rmtTLS, mut value: *mut libc::c_void) {
    if !(handle != 4294967295 as libc::c_uint) {
        __assert_fail(
            b"handle != TLS_INVALID_HANDLE\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            471 as libc::c_uint,
            b"tlsSet\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_setspecific(handle, value as *const libc::c_void);
}
unsafe extern "C" fn tlsGet(mut handle: rmtTLS) -> *mut libc::c_void {
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if !(handle != 4294967295 as libc::c_uint) {
        __assert_fail(
            b"handle != TLS_INVALID_HANDLE\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            481 as libc::c_uint,
            b"tlsGet\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = pthread_getspecific(handle);
    return tmp___0;
}
static mut g_lastErrorMessageTlsHandle: rmtTLS = 4294967295 as libc::c_uint;
static mut g_errorMessageSize: libc::c_uint = 1024 as libc::c_int as rmtU32;
unsafe extern "C" fn rmtMakeError(
    mut error: rmtError,
    mut error_message: rmtPStr,
) -> rmtError {
    let mut thread_message_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error_len: rmtU32 = 0;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: size_t = 0;
    if g_lastErrorMessageTlsHandle == 4294967295 as libc::c_uint {
        tmp = tlsAlloc(&mut g_lastErrorMessageTlsHandle);
        error___0 = tmp;
        if error___0 as libc::c_uint != 0 as libc::c_uint {
            return error___0;
        }
    }
    tmp___0 = tlsGet(g_lastErrorMessageTlsHandle);
    thread_message_ptr = tmp___0 as *mut libc::c_char;
    if thread_message_ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = rmtMalloc(g_errorMessageSize);
        thread_message_ptr = tmp___1 as *mut libc::c_char;
        if thread_message_ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
        {
            return RMT_ERROR_MALLOC_FAIL;
        }
        tlsSet(g_lastErrorMessageTlsHandle, thread_message_ptr as *mut libc::c_void);
    }
    tmp___2 = strlen(error_message);
    error_len = tmp___2 as rmtU32;
    if error_len >= g_errorMessageSize {
        error_len = g_errorMessageSize.wrapping_sub(1 as libc::c_uint);
    } else {
        error_len = error_len;
    }
    memcpy(
        thread_message_ptr as *mut libc::c_void,
        error_message as *const libc::c_void,
        error_len as size_t,
    );
    *thread_message_ptr.offset(error_len as isize) = 0 as libc::c_int as libc::c_char;
    return error;
}
pub unsafe extern "C" fn rmt_GetLastErrorMessage() -> rmtPStr {
    let mut thread_message_ptr: rmtPStr = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if g_lastErrorMessageTlsHandle == 4294967295 as libc::c_uint {
        return b"No error message\0" as *const u8 as *const libc::c_char;
    }
    tmp = tlsGet(g_lastErrorMessageTlsHandle);
    thread_message_ptr = tmp as rmtPStr;
    if thread_message_ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return b"No error message\0" as *const u8 as *const libc::c_char;
    }
    return thread_message_ptr;
}
unsafe extern "C" fn mtxInit(mut mutex: *mut rmtMutex) {
    if !(mutex as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"mutex != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            571 as libc::c_uint,
            b"mtxInit\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_mutex_init(mutex, 0 as *mut libc::c_void as *const pthread_mutexattr_t);
}
unsafe extern "C" fn mtxLock(mut mutex: *mut rmtMutex) {
    if !(mutex as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"mutex != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            581 as libc::c_uint,
            b"mtxLock\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_mutex_lock(mutex);
}
unsafe extern "C" fn mtxUnlock(mut mutex: *mut rmtMutex) {
    if !(mutex as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"mutex != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            591 as libc::c_uint,
            b"mtxUnlock\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_mutex_unlock(mutex);
}
unsafe extern "C" fn mtxDelete(mut mutex: *mut rmtMutex) {
    if !(mutex as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"mutex != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            601 as libc::c_uint,
            b"mtxDelete\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_mutex_destroy(mutex);
}
unsafe extern "C" fn AtomicCompareAndSwapU32(
    mut val: *mut rmtU32,
    mut old_val: libc::c_long,
    mut new_val: libc::c_long,
) -> rmtBool {
    let mut tmp___0: rmtBool = 0;
    let mut tmp___1: bool = false;
    tmp___1 = (::std::intrinsics::atomic_cxchg(
        val,
        old_val as rmtU32,
        new_val as rmtU32,
    ))
        .1;
    if tmp___1 {
        tmp___0 = 1 as libc::c_int as rmtBool;
    } else {
        tmp___0 = 0 as libc::c_int as rmtBool;
    }
    return tmp___0;
}
unsafe extern "C" fn AtomicCompareAndSwapPointer(
    mut ptr: *mut *mut libc::c_long,
    mut old_ptr: *mut libc::c_long,
    mut new_ptr: *mut libc::c_long,
) -> rmtBool {
    let mut tmp___0: rmtBool = 0;
    let mut tmp___1: bool = false;
    tmp___1 = (::std::intrinsics::atomic_cxchg(ptr, old_ptr, new_ptr)).1;
    if tmp___1 {
        tmp___0 = 1 as libc::c_int as rmtBool;
    } else {
        tmp___0 = 0 as libc::c_int as rmtBool;
    }
    return tmp___0;
}
unsafe extern "C" fn AtomicAddS32(
    mut value: *mut rmtAtomicS32,
    mut add: rmtS32,
) -> rmtS32 {
    let mut tmp: rmtAtomicS32 = 0;
    tmp = ::std::intrinsics::atomic_xadd_seqcst(value, add);
    return tmp;
}
unsafe extern "C" fn AtomicSubS32(mut value: *mut rmtAtomicS32, mut sub: rmtS32) {
    AtomicAddS32(value, -sub);
}
unsafe extern "C" fn CompilerWriteFence() {
    asm!("", options(preserves_flags, att_syntax));
}
unsafe extern "C" fn CompilerReadFence() {
    asm!("", options(preserves_flags, att_syntax));
}
unsafe extern "C" fn LoadAcquire(mut address: *mut rmtAtomicU32) -> rmtU32 {
    let mut value: rmtU32 = 0;
    value = *address;
    CompilerReadFence();
    return value;
}
unsafe extern "C" fn LoadAcquirePointer(
    mut ptr: *mut *mut libc::c_long,
) -> *mut libc::c_long {
    let mut value: *mut libc::c_long = 0 as *mut libc::c_long;
    value = *ptr;
    CompilerReadFence();
    return value;
}
unsafe extern "C" fn StoreRelease(mut address: *mut rmtAtomicU32, mut value: rmtU32) {
    CompilerWriteFence();
    *address = value;
}
unsafe extern "C" fn StoreReleasePointer(
    mut ptr: *mut *mut libc::c_long,
    mut value: *mut libc::c_long,
) {
    CompilerWriteFence();
    *ptr = value;
}
static mut Well512_State: [rmtU32; 16] = [0; 16];
static mut Well512_Index: rmtU32 = 0;
unsafe extern "C" fn Well512_Init(mut seed: rmtU32) {
    let mut i: rmtU32 = 0;
    let mut prev: rmtU32 = 0;
    Well512_State[0 as libc::c_int as usize] = seed;
    i = 1 as libc::c_int as rmtU32;
    while i < 16 as libc::c_uint {
        prev = Well512_State[i.wrapping_sub(1 as libc::c_uint) as usize];
        Well512_State[i
            as usize] = (1812433253 as libc::c_uint)
            .wrapping_mul(prev ^ prev >> 30 as libc::c_int)
            .wrapping_add(i);
        i = i.wrapping_add(1);
    }
    Well512_Index = 0 as libc::c_int as rmtU32;
}
unsafe extern "C" fn Well512_RandomU32() -> rmtU32 {
    let mut a: rmtU32 = 0;
    let mut b: rmtU32 = 0;
    let mut c: rmtU32 = 0;
    let mut d: rmtU32 = 0;
    let mut tmp: rmtU32 = 0;
    a = Well512_State[Well512_Index as usize];
    c = Well512_State[(Well512_Index.wrapping_add(13 as libc::c_uint)
        & 15 as libc::c_uint) as usize];
    b = a ^ c ^ a << 16 as libc::c_int ^ c << 15 as libc::c_int;
    c = Well512_State[(Well512_Index.wrapping_add(9 as libc::c_uint)
        & 15 as libc::c_uint) as usize];
    c ^= c >> 11 as libc::c_int;
    tmp = b ^ c;
    Well512_State[Well512_Index as usize] = tmp;
    a = tmp;
    d = (a as libc::c_ulong
        ^ (a << 5 as libc::c_int) as libc::c_ulong & 3661901092 as libc::c_ulong)
        as rmtU32;
    Well512_Index = Well512_Index.wrapping_add(15 as libc::c_uint) & 15 as libc::c_uint;
    a = Well512_State[Well512_Index as usize];
    Well512_State[Well512_Index
        as usize] = a ^ b ^ d ^ a << 2 as libc::c_int ^ b << 18 as libc::c_int
        ^ c << 28 as libc::c_int;
    return Well512_State[Well512_Index as usize];
}
unsafe extern "C" fn Well512_RandomOpenLimit(mut limit: rmtU32) -> rmtU32 {
    let mut bucket_size: rmtU32 = 0;
    let mut bucket_limit: rmtU32 = 0;
    let mut r: rmtU32 = 0;
    bucket_size = (4294967295 as libc::c_uint).wrapping_div(limit);
    bucket_limit = bucket_size.wrapping_mul(limit);
    loop {
        r = Well512_RandomU32();
        if !(r >= bucket_limit) {
            break;
        }
    }
    return r.wrapping_div(bucket_size);
}
static mut MultiplyDeBruijnBitPosition: [rmtU8; 32] = [
    0 as libc::c_int as rmtU8,
    9 as libc::c_int as rmtU8,
    1 as libc::c_int as rmtU8,
    10 as libc::c_int as rmtU8,
    13 as libc::c_int as rmtU8,
    21 as libc::c_int as rmtU8,
    2 as libc::c_int as rmtU8,
    29 as libc::c_int as rmtU8,
    11 as libc::c_int as rmtU8,
    14 as libc::c_int as rmtU8,
    16 as libc::c_int as rmtU8,
    18 as libc::c_int as rmtU8,
    22 as libc::c_int as rmtU8,
    25 as libc::c_int as rmtU8,
    3 as libc::c_int as rmtU8,
    30 as libc::c_int as rmtU8,
    8 as libc::c_int as rmtU8,
    12 as libc::c_int as rmtU8,
    20 as libc::c_int as rmtU8,
    28 as libc::c_int as rmtU8,
    15 as libc::c_int as rmtU8,
    17 as libc::c_int as rmtU8,
    24 as libc::c_int as rmtU8,
    7 as libc::c_int as rmtU8,
    19 as libc::c_int as rmtU8,
    27 as libc::c_int as rmtU8,
    23 as libc::c_int as rmtU8,
    6 as libc::c_int as rmtU8,
    26 as libc::c_int as rmtU8,
    5 as libc::c_int as rmtU8,
    4 as libc::c_int as rmtU8,
    31 as libc::c_int as rmtU8,
];
unsafe extern "C" fn Log2i(mut x: rmtU32) -> rmtU32 {
    x |= x >> 1 as libc::c_int;
    x |= x >> 2 as libc::c_int;
    x |= x >> 4 as libc::c_int;
    x |= x >> 8 as libc::c_int;
    x |= x >> 16 as libc::c_int;
    return MultiplyDeBruijnBitPosition[(x.wrapping_mul(130329821 as libc::c_uint)
        >> 27 as libc::c_int) as usize] as rmtU32;
}
static mut XORMasks: [rmtU8; 7] = [
    (1 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int) as rmtU8,
    ((1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int)
        as rmtU8,
    ((1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int)
        as rmtU8,
    ((1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
        as rmtU8,
    ((1 as libc::c_int) << 4 as libc::c_int | (1 as libc::c_int) << 5 as libc::c_int)
        as rmtU8,
    ((1 as libc::c_int) << 5 as libc::c_int | (1 as libc::c_int) << 6 as libc::c_int)
        as rmtU8,
    ((1 as libc::c_int) << 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        | (1 as libc::c_int) << 5 as libc::c_int
        | (1 as libc::c_int) << 7 as libc::c_int) as rmtU8,
];
unsafe extern "C" fn GaloisLFSRMask(mut table_size_log2: rmtU32) -> rmtU32 {
    if !(table_size_log2 >= 2 as libc::c_uint) {
        __assert_fail(
            b"table_size_log2 >= 2\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            853 as libc::c_uint,
            b"GaloisLFSRMask\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(table_size_log2 <= 8 as libc::c_uint) {
        __assert_fail(
            b"table_size_log2 <= 8\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            854 as libc::c_uint,
            b"GaloisLFSRMask\0" as *const u8 as *const libc::c_char,
        );
    }
    return XORMasks[table_size_log2.wrapping_sub(2 as libc::c_uint) as usize] as rmtU32;
}
unsafe extern "C" fn GaloisLFSRNext(mut value: rmtU32, mut xor_mask: rmtU32) -> rmtU32 {
    let mut lsb: rmtU32 = 0;
    lsb = value & 1 as libc::c_uint;
    value >>= 1 as libc::c_int;
    if lsb != 0 as libc::c_uint {
        value ^= xor_mask;
    }
    return value;
}
static mut k_64: libc::c_uint = 65536 as libc::c_int as rmtU32;
unsafe extern "C" fn VirtualMirrorBuffer_Constructor(
    mut buffer: *mut VirtualMirrorBuffer,
    mut size: rmtU32,
    mut nb_attempts: libc::c_int,
) -> rmtError {
    let mut path: [libc::c_char; 28] = [0; 28];
    let mut file_descriptor: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    path[0 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    path[1 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    path[2 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    path[3 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    path[4 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    path[5 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    path[6 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    path[7 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    path[8 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    path[9 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    path[10 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    path[11 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    path[12 as libc::c_int as usize] = 'g' as i32 as libc::c_char;
    path[13 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    path[14 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    path[15 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    path[16 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    path[17 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    path[18 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    path[19 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    path[20 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    path[21 as libc::c_int as usize] = 'X' as i32 as libc::c_char;
    path[22 as libc::c_int as usize] = 'X' as i32 as libc::c_char;
    path[23 as libc::c_int as usize] = 'X' as i32 as libc::c_char;
    path[24 as libc::c_int as usize] = 'X' as i32 as libc::c_char;
    path[25 as libc::c_int as usize] = 'X' as i32 as libc::c_char;
    path[26 as libc::c_int as usize] = 'X' as i32 as libc::c_char;
    path[27 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    size = size
        .wrapping_add(k_64)
        .wrapping_sub(1 as libc::c_uint)
        .wrapping_div(k_64)
        .wrapping_mul(k_64);
    (*buffer).size = size;
    (*buffer).ptr = 0 as *mut libc::c_void as *mut rmtU8;
    file_descriptor = mkstemp(path.as_mut_ptr());
    if file_descriptor < 0 as libc::c_int {
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    tmp = unlink(path.as_mut_ptr() as *const libc::c_char);
    if tmp != 0 {
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    tmp___0 = ftruncate(
        file_descriptor,
        size.wrapping_mul(2 as libc::c_uint) as __off_t,
    );
    if tmp___0 != 0 {
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    tmp___1 = mmap(
        0 as *mut libc::c_void,
        size.wrapping_mul(2 as libc::c_uint) as size_t,
        0 as libc::c_int,
        34 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as __off_t,
    );
    (*buffer).ptr = tmp___1 as *mut rmtU8;
    if (*buffer).ptr as libc::c_ulong
        == -(1 as libc::c_int) as *mut libc::c_void as libc::c_ulong
    {
        (*buffer).ptr = 0 as *mut libc::c_void as *mut rmtU8;
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    tmp___2 = mmap(
        (*buffer).ptr as *mut libc::c_void,
        size as size_t,
        3 as libc::c_int,
        17 as libc::c_int,
        file_descriptor,
        0 as libc::c_int as __off_t,
    );
    if tmp___2 as libc::c_ulong != (*buffer).ptr as libc::c_ulong {
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL
    } else {
        tmp___3 = mmap(
            ((*buffer).ptr).offset(size as isize) as *mut libc::c_void,
            size as size_t,
            3 as libc::c_int,
            17 as libc::c_int,
            file_descriptor,
            0 as libc::c_int as __off_t,
        );
        if tmp___3 as libc::c_ulong
            != ((*buffer).ptr).offset(size as isize) as libc::c_ulong
        {
            return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
        }
    }
    if (*buffer).ptr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_VIRTUAL_MEMORY_BUFFER_FAIL;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn VirtualMirrorBuffer_Destructor(
    mut buffer: *mut VirtualMirrorBuffer,
) {
    if !(buffer as libc::c_ulong != 0 as *mut VirtualMirrorBuffer as libc::c_ulong) {
        __assert_fail(
            b"buffer != 0\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            1351 as libc::c_uint,
            b"VirtualMirrorBuffer_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*buffer).ptr as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        munmap(
            (*buffer).ptr as *mut libc::c_void,
            ((*buffer).size).wrapping_mul(2 as libc::c_uint) as size_t,
        );
    }
    (*buffer).ptr = 0 as *mut libc::c_void as *mut rmtU8;
}
unsafe extern "C" fn strnlen_s_safe_c(
    mut dest: *const libc::c_char,
    mut dmax: r_size_t,
) -> r_size_t {
    let mut count: r_size_t = 0;
    if dest as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int as r_size_t;
    }
    if dmax == 0 as libc::c_uint {
        return 0 as libc::c_int as r_size_t;
    }
    if dmax as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 0 as libc::c_int as r_size_t;
    }
    count = 0 as libc::c_int as r_size_t;
    while *dest != 0 {
        if dmax == 0 {
            break;
        }
        count = count.wrapping_add(1);
        dmax = dmax.wrapping_sub(1);
        dest = dest.offset(1);
    }
    return count;
}
unsafe extern "C" fn strstr_s(
    mut dest: *mut libc::c_char,
    mut dmax: r_size_t,
    mut src: *const libc::c_char,
    mut slen: r_size_t,
    mut substring: *mut *mut libc::c_char,
) -> errno_t {
    let mut len: r_size_t = 0;
    let mut dlen: r_size_t = 0;
    let mut i: libc::c_int = 0;
    if substring as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 400 as libc::c_int;
    }
    *substring = 0 as *mut libc::c_void as *mut libc::c_char;
    if dest as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 400 as libc::c_int;
    }
    if dmax == 0 as libc::c_uint {
        return 401 as libc::c_int;
    }
    if dmax as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 403 as libc::c_int;
    }
    if src as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 400 as libc::c_int;
    }
    if slen == 0 as libc::c_uint {
        return 401 as libc::c_int;
    }
    if slen as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 403 as libc::c_int;
    }
    if *src as libc::c_int == 0 as libc::c_int {
        *substring = dest;
        return 0 as libc::c_int;
    } else {
        if dest as libc::c_ulong == src as libc::c_ulong {
            *substring = dest;
            return 0 as libc::c_int;
        }
    }
    while *dest != 0 {
        if dmax == 0 {
            break;
        }
        i = 0 as libc::c_int;
        len = slen;
        dlen = dmax;
        while *src.offset(i as isize) != 0 {
            if dlen == 0 {
                break;
            }
            if *dest.offset(i as isize) as libc::c_int
                != *src.offset(i as isize) as libc::c_int
            {
                break;
            }
            i += 1;
            len = len.wrapping_sub(1);
            dlen = dlen.wrapping_sub(1);
            if *src.offset(i as isize) as libc::c_int == 0 as libc::c_int {
                *substring = dest;
                return 0 as libc::c_int;
            } else {
                if len == 0 {
                    *substring = dest;
                    return 0 as libc::c_int;
                }
            }
        }
        dest = dest.offset(1);
        dmax = dmax.wrapping_sub(1);
    }
    *substring = 0 as *mut libc::c_void as *mut libc::c_char;
    return 409 as libc::c_int;
}
unsafe extern "C" fn strncat_s_safe_c(
    mut dest: *mut libc::c_char,
    mut dmax: r_size_t,
    mut src: *const libc::c_char,
    mut slen: r_size_t,
) -> errno_t {
    let mut overlap_bumper: *const libc::c_char = 0 as *const libc::c_char;
    if dest as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 400 as libc::c_int;
    }
    if src as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 400 as libc::c_int;
    }
    if slen as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 403 as libc::c_int;
    }
    if dmax == 0 as libc::c_uint {
        return 401 as libc::c_int;
    }
    if dmax as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 403 as libc::c_int;
    }
    if (dest as libc::c_ulong) < src as libc::c_ulong {
        overlap_bumper = src;
        while *dest as libc::c_int != 0 as libc::c_int {
            if dest as libc::c_ulong == overlap_bumper as libc::c_ulong {
                return 404 as libc::c_int;
            }
            dest = dest.offset(1);
            dmax = dmax.wrapping_sub(1);
            if dmax == 0 as libc::c_uint {
                return 407 as libc::c_int;
            }
        }
        while dmax > 0 as libc::c_uint {
            if dest as libc::c_ulong == overlap_bumper as libc::c_ulong {
                return 404 as libc::c_int;
            }
            if slen == 0 as libc::c_uint {
                *dest = '\u{0}' as i32 as libc::c_char;
                return 0 as libc::c_int;
            }
            *dest = *src;
            if *dest as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            dmax = dmax.wrapping_sub(1);
            slen = slen.wrapping_sub(1);
            dest = dest.offset(1);
            src = src.offset(1);
        }
    } else {
        overlap_bumper = dest as *const libc::c_char;
        while *dest as libc::c_int != 0 as libc::c_int {
            dest = dest.offset(1);
            dmax = dmax.wrapping_sub(1);
            if dmax == 0 as libc::c_uint {
                return 407 as libc::c_int;
            }
        }
        while dmax > 0 as libc::c_uint {
            if src as libc::c_ulong == overlap_bumper as libc::c_ulong {
                return 404 as libc::c_int;
            }
            if slen == 0 as libc::c_uint {
                *dest = '\u{0}' as i32 as libc::c_char;
                return 0 as libc::c_int;
            }
            *dest = *src;
            if *dest as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            dmax = dmax.wrapping_sub(1);
            slen = slen.wrapping_sub(1);
            dest = dest.offset(1);
            src = src.offset(1);
        }
    }
    return 406 as libc::c_int;
}
pub unsafe extern "C" fn strcpy_s_safe_c(
    mut dest: *mut libc::c_char,
    mut dmax: r_size_t,
    mut src: *const libc::c_char,
) -> errno_t {
    let mut overlap_bumper: *const libc::c_char = 0 as *const libc::c_char;
    if dest as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 400 as libc::c_int;
    }
    if dmax == 0 as libc::c_uint {
        return 401 as libc::c_int;
    }
    if dmax as libc::c_ulong > (4 as libc::c_ulong) << 10 as libc::c_int {
        return 403 as libc::c_int;
    }
    if src as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        *dest = '\u{0}' as i32 as libc::c_char;
        return 400 as libc::c_int;
    }
    if dest as libc::c_ulong == src as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if (dest as libc::c_ulong) < src as libc::c_ulong {
        overlap_bumper = src;
        while dmax > 0 as libc::c_uint {
            if dest as libc::c_ulong == overlap_bumper as libc::c_ulong {
                return 404 as libc::c_int;
            }
            *dest = *src;
            if *dest as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            dmax = dmax.wrapping_sub(1);
            dest = dest.offset(1);
            src = src.offset(1);
        }
    } else {
        overlap_bumper = dest as *const libc::c_char;
        while dmax > 0 as libc::c_uint {
            if src as libc::c_ulong == overlap_bumper as libc::c_ulong {
                return 404 as libc::c_int;
            }
            *dest = *src;
            if *dest as libc::c_int == 0 as libc::c_int {
                return 0 as libc::c_int;
            }
            dmax = dmax.wrapping_sub(1);
            dest = dest.offset(1);
            src = src.offset(1);
        }
    }
    return 406 as libc::c_int;
}
static mut hex_encoding_table: *const libc::c_char = b"0123456789ABCDEF\0" as *const u8
    as *const libc::c_char;
unsafe extern "C" fn itoahex_s(
    mut dest: *mut libc::c_char,
    mut dmax: r_size_t,
    mut value: rmtS32,
) {
    let mut len: r_size_t = 0;
    let mut halfbytepos: rmtS32 = 0;
    halfbytepos = 8 as libc::c_int;
    while halfbytepos > 1 as libc::c_int {
        halfbytepos -= 1;
        if !(value >> 4 as libc::c_int * halfbytepos & 15 as libc::c_int != 0) {
            continue;
        }
        halfbytepos += 1;
        break;
    }
    len = 0 as libc::c_int as r_size_t;
    while len.wrapping_add(1 as libc::c_uint) < dmax {
        if !(halfbytepos > 0 as libc::c_int) {
            break;
        }
        halfbytepos -= 1;
        *dest
            .offset(
                len as isize,
            ) = *hex_encoding_table
            .offset(
                (value >> 4 as libc::c_int * halfbytepos & 15 as libc::c_int) as isize,
            );
        len = len.wrapping_add(1);
    }
    if len < dmax {
        *dest.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    }
}
static mut temp_dest: [libc::c_char; 12] = [
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
    0 as libc::c_int as libc::c_char,
];
unsafe extern "C" fn itoa_s(mut value: rmtS32) -> *const libc::c_char {
    let mut pos: libc::c_int = 0;
    let mut abs_value: rmtS32 = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    pos = 10 as libc::c_int;
    tmp = abs(value);
    abs_value = tmp;
    while abs_value > 0 as libc::c_int {
        tmp___0 = pos;
        pos -= 1;
        temp_dest[tmp___0
            as usize] = (48 as libc::c_int + abs_value % 10 as libc::c_int)
            as libc::c_char;
        abs_value /= 10 as libc::c_int;
    }
    if value < 0 as libc::c_int {
        tmp___1 = pos;
        pos -= 1;
        temp_dest[tmp___1 as usize] = '-' as i32 as libc::c_char;
    }
    return temp_dest.as_mut_ptr().offset(pos as isize).offset(1 as libc::c_int as isize)
        as *const libc::c_char;
}
unsafe extern "C" fn rmtGetNbProcessors() -> rmtU32 {
    return 0 as libc::c_int as rmtU32;
}
unsafe extern "C" fn rmtGetCurrentThreadId() -> rmtThreadId {
    let mut tmp: pthread_t = 0;
    tmp = pthread_self();
    return tmp;
}
unsafe extern "C" fn rmtSuspendThread(mut thread_handle: rmtThreadHandle) -> rmtBool {
    return 0 as libc::c_int as rmtBool;
}
unsafe extern "C" fn rmtResumeThread(mut thread_handle: rmtThreadHandle) {}
unsafe extern "C" fn rmtGetUserModeThreadContext(
    mut thread: rmtThreadHandle,
    mut context: *mut rmtCpuContext,
) -> rmtBool {
    return 0 as libc::c_int as rmtBool;
}
unsafe extern "C" fn rmtSetThreadContext(
    mut thread_handle: rmtThreadHandle,
    mut context: *mut rmtCpuContext,
) {}
unsafe extern "C" fn rmtOpenThreadHandle(
    mut thread_id: rmtThreadId,
    mut out_thread_handle: *mut rmtThreadHandle,
) -> rmtError {
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtCloseThreadHandle(mut thread_handle: rmtThreadHandle) {}
static mut countThreads: rmtS32 = 0 as libc::c_int;
unsafe extern "C" fn rmtGetThreadNameFallback(
    mut out_thread_name: *mut libc::c_char,
    mut thread_name_size: rmtU32,
) {
    let mut tmp: rmtS32 = 0;
    *out_thread_name
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    strncat_s_safe_c(
        out_thread_name,
        thread_name_size,
        b"Thread\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as r_size_t,
    );
    tmp = AtomicAddS32(
        &mut countThreads as *mut rmtS32 as *mut rmtAtomicS32,
        1 as libc::c_int,
    );
    itoahex_s(
        out_thread_name.offset(6 as libc::c_int as isize),
        thread_name_size.wrapping_sub(6 as libc::c_uint),
        tmp,
    );
}
unsafe extern "C" fn rmtGetThreadName(
    mut thread_id: rmtThreadId,
    mut thread_handle: rmtThreadHandle,
    mut out_thread_name: *mut libc::c_char,
    mut thread_name_size: rmtU32,
) {
    rmtGetThreadNameFallback(out_thread_name, thread_name_size);
}
unsafe extern "C" fn StartFunc(mut pArgs: *mut libc::c_void) -> *mut libc::c_void {
    let mut thread: *mut rmtThread = 0 as *mut rmtThread;
    thread = pArgs as *mut rmtThread;
    if !(thread as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"thread != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2156 as libc::c_uint,
            b"StartFunc\0" as *const u8 as *const libc::c_char,
        );
    }
    (*thread)
        .error = (Some(((*thread).callback).expect("non-null function pointer")))
        .expect("non-null function pointer")(thread);
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn rmtThread_Valid(mut thread: *mut rmtThread) -> libc::c_int {
    let mut tmp___0: pthread_t = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    if !(thread as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"thread != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2164 as libc::c_uint,
            b"rmtThread_Valid\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = pthread_self();
    tmp___1 = pthread_equal((*thread).handle, tmp___0);
    if tmp___1 != 0 {
        tmp___2 = 0 as libc::c_int;
    } else {
        tmp___2 = 1 as libc::c_int;
    }
    return tmp___2;
}
unsafe extern "C" fn rmtThread_Constructor(
    mut thread: *mut rmtThread,
    mut callback: Option::<unsafe extern "C" fn(*mut rmtThread) -> rmtError>,
    mut param: *mut libc::c_void,
) -> rmtError {
    let mut error: int32_t = 0;
    let mut tmp___0: libc::c_int = 0;
    if !(thread as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"thread != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2175 as libc::c_uint,
            b"rmtThread_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*thread).callback = callback;
    (*thread).param = param;
    (*thread).error = RMT_ERROR_NONE;
    (*thread).request_exit = 0 as libc::c_int as rmtBool;
    tmp___0 = pthread_create(
        &mut (*thread).handle as *mut rmtThreadHandle as *mut pthread_t,
        0 as *mut libc::c_void as *const pthread_attr_t,
        Some(StartFunc as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
        thread as *mut libc::c_void,
    );
    error = tmp___0;
    if error != 0 {
        (*thread).handle = pthread_self();
        return RMT_ERROR_CREATE_THREAD_FAIL;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtThread_RequestExit(mut thread: *mut rmtThread) {
    if !(thread as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"thread != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2215 as libc::c_uint,
            b"rmtThread_RequestExit\0" as *const u8 as *const libc::c_char,
        );
    }
    (*thread).request_exit = 1 as libc::c_int as rmtBool;
}
unsafe extern "C" fn rmtThread_Join(mut thread: *mut rmtThread) {
    let mut tmp___2: libc::c_int = 0;
    tmp___2 = rmtThread_Valid(thread);
    if tmp___2 == 0 {
        __assert_fail(
            b"rmtThread_Valid(thread)\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2221 as libc::c_uint,
            b"rmtThread_Join\0" as *const u8 as *const libc::c_char,
        );
    }
    pthread_join((*thread).handle, 0 as *mut libc::c_void as *mut *mut libc::c_void);
}
unsafe extern "C" fn rmtThread_Destructor(mut thread: *mut rmtThread) {
    let mut tmp___0: libc::c_int = 0;
    if !(thread as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"thread != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2232 as libc::c_uint,
            b"rmtThread_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = rmtThread_Valid(thread);
    if tmp___0 != 0 {
        rmtThread_RequestExit(thread);
        rmtThread_Join(thread);
    }
}
unsafe extern "C" fn ObjectLink_Constructor(mut link___0: *mut ObjectLink) {
    if !(link___0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"link != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2267 as libc::c_uint,
            b"ObjectLink_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*link___0).next = 0 as *mut libc::c_void as *mut ObjectLink_s;
}
unsafe extern "C" fn ObjectAllocator_Constructor(
    mut allocator: *mut ObjectAllocator,
    mut object_size: rmtU32,
    mut constructor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> rmtError>,
    mut destructor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> rmtError {
    (*allocator).object_size = object_size;
    (*allocator).constructor = constructor;
    (*allocator).destructor = destructor;
    (*allocator).nb_free = 0 as libc::c_int;
    (*allocator).nb_inuse = 0 as libc::c_int;
    (*allocator).nb_allocated = 0 as libc::c_int;
    (*allocator).first_free = 0 as *mut libc::c_void as *mut ObjectLink;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ObjectAllocator_Destructor(mut allocator: *mut ObjectAllocator) {
    let mut next: *mut ObjectLink = 0 as *mut ObjectLink;
    if !(allocator as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2309 as libc::c_uint,
            b"ObjectAllocator_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*allocator).nb_inuse == 0 as libc::c_int) {
        __assert_fail(
            b"allocator->nb_inuse == 0\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2310 as libc::c_uint,
            b"ObjectAllocator_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    while (*allocator).first_free as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        next = (*(*allocator).first_free).next as *mut ObjectLink;
        if !(::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            libc::c_ulong,
        >((*allocator).destructor) != 0 as *mut libc::c_void as libc::c_ulong)
        {
            __assert_fail(
                b"allocator->destructor != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2316 as libc::c_uint,
                b"ObjectAllocator_Destructor\0" as *const u8 as *const libc::c_char,
            );
        }
        (Some(((*allocator).destructor).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*allocator).first_free as *mut libc::c_void);
        rmtFree((*allocator).first_free as *mut libc::c_void);
        (*allocator).first_free = next;
    }
}
unsafe extern "C" fn ObjectAllocator_Push(
    mut allocator: *mut ObjectAllocator,
    mut start: *mut ObjectLink,
    mut end: *mut ObjectLink,
) {
    let mut old_link: *mut ObjectLink = 0 as *mut ObjectLink;
    let mut tmp___2: rmtBool = 0;
    if !(allocator as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2325 as libc::c_uint,
            b"ObjectAllocator_Push\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(start as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"start != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2326 as libc::c_uint,
            b"ObjectAllocator_Push\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(end as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"end != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2327 as libc::c_uint,
            b"ObjectAllocator_Push\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        old_link = (*allocator).first_free;
        (*end).next = old_link as *mut ObjectLink_s;
        tmp___2 = AtomicCompareAndSwapPointer(
            &mut (*allocator).first_free as *mut *mut ObjectLink
                as *mut *mut libc::c_long,
            old_link as *mut libc::c_long,
            start as *mut libc::c_long,
        );
        if tmp___2 == 1 as libc::c_uint {
            break;
        }
    };
}
unsafe extern "C" fn ObjectAllocator_Pop(
    mut allocator: *mut ObjectAllocator,
) -> *mut ObjectLink {
    let mut link___0: *mut ObjectLink = 0 as *mut ObjectLink;
    let mut old_link: *mut ObjectLink = 0 as *mut ObjectLink;
    let mut next_link: *mut ObjectLink = 0 as *mut ObjectLink;
    let mut tmp___0: rmtBool = 0;
    if !(allocator as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2344 as libc::c_uint,
            b"ObjectAllocator_Pop\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        old_link = (*allocator).first_free;
        if old_link as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return 0 as *mut libc::c_void as *mut ObjectLink;
        }
        next_link = (*old_link).next as *mut ObjectLink;
        tmp___0 = AtomicCompareAndSwapPointer(
            &mut (*allocator).first_free as *mut *mut ObjectLink
                as *mut *mut libc::c_long,
            old_link as *mut libc::c_long,
            next_link as *mut libc::c_long,
        );
        if !(tmp___0 == 1 as libc::c_uint) {
            continue;
        }
        link___0 = old_link;
        break;
    }
    (*link___0).next = 0 as *mut libc::c_void as *mut ObjectLink_s;
    return link___0;
}
unsafe extern "C" fn ObjectAllocator_Alloc(
    mut allocator: *mut ObjectAllocator,
    mut object: *mut *mut libc::c_void,
) -> rmtError {
    let mut tmp___1: *mut ObjectLink = 0 as *mut ObjectLink;
    let mut error: rmtError = RMT_ERROR_NONE;
    if !(allocator as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2372 as libc::c_uint,
            b"ObjectAllocator_Alloc\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(object as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"object != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2373 as libc::c_uint,
            b"ObjectAllocator_Alloc\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___1 = ObjectAllocator_Pop(allocator);
    *object = tmp___1 as *mut libc::c_void;
    if *object as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        *object = rmtMalloc((*allocator).object_size);
        if *object as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            return RMT_ERROR_MALLOC_FAIL;
        }
        if !(::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> rmtError>,
            libc::c_ulong,
        >((*allocator).constructor) != 0 as *mut libc::c_void as libc::c_ulong)
        {
            __assert_fail(
                b"allocator->constructor != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                2387 as libc::c_uint,
                b"ObjectAllocator_Alloc\0" as *const u8 as *const libc::c_char,
            );
        }
        error = (Some(((*allocator).constructor).expect("non-null function pointer")))
            .expect("non-null function pointer")(*object);
        if error as libc::c_uint != 0 as libc::c_uint {
            if !(::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
                libc::c_ulong,
            >((*allocator).destructor) != 0 as *mut libc::c_void as libc::c_ulong)
            {
                __assert_fail(
                    b"allocator->destructor != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    2392 as libc::c_uint,
                    b"ObjectAllocator_Alloc\0" as *const u8 as *const libc::c_char,
                );
            }
            (Some(((*allocator).destructor).expect("non-null function pointer")))
                .expect("non-null function pointer")(*object);
            rmtFree(*object);
            return error;
        }
        AtomicAddS32(&mut (*allocator).nb_allocated, 1 as libc::c_int);
    } else {
        AtomicSubS32(&mut (*allocator).nb_free, 1 as libc::c_int);
    }
    AtomicAddS32(&mut (*allocator).nb_inuse, 1 as libc::c_int);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ObjectAllocator_Free(
    mut allocator: *mut ObjectAllocator,
    mut object: *mut libc::c_void,
) {
    if !(allocator as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2413 as libc::c_uint,
            b"ObjectAllocator_Free\0" as *const u8 as *const libc::c_char,
        );
    }
    ObjectAllocator_Push(
        allocator,
        object as *mut ObjectLink,
        object as *mut ObjectLink,
    );
    AtomicSubS32(&mut (*allocator).nb_inuse, 1 as libc::c_int);
    AtomicAddS32(&mut (*allocator).nb_free, 1 as libc::c_int);
}
unsafe extern "C" fn ObjectAllocator_FreeRange(
    mut allocator: *mut ObjectAllocator,
    mut start: *mut libc::c_void,
    mut end: *mut libc::c_void,
    mut count: rmtU32,
) {
    if !(allocator as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"allocator != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2421 as libc::c_uint,
            b"ObjectAllocator_FreeRange\0" as *const u8 as *const libc::c_char,
        );
    }
    ObjectAllocator_Push(allocator, start as *mut ObjectLink, end as *mut ObjectLink);
    AtomicSubS32(&mut (*allocator).nb_inuse, count as rmtS32);
    AtomicAddS32(&mut (*allocator).nb_free, count as rmtS32);
}
unsafe extern "C" fn Buffer_Constructor(
    mut buffer: *mut Buffer,
    mut alloc_granularity: rmtU32,
) -> rmtError {
    if !(buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2447 as libc::c_uint,
            b"Buffer_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*buffer).alloc_granularity = alloc_granularity;
    (*buffer).bytes_allocated = 0 as libc::c_int as rmtU32;
    (*buffer).bytes_used = 0 as libc::c_int as rmtU32;
    (*buffer).data = 0 as *mut libc::c_void as *mut rmtU8;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Buffer_Destructor(mut buffer: *mut Buffer) {
    if !(buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2457 as libc::c_uint,
            b"Buffer_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*buffer).data as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        rmtFree((*buffer).data as *mut libc::c_void);
        (*buffer).data = 0 as *mut libc::c_void as *mut rmtU8;
    }
}
unsafe extern "C" fn Buffer_Grow(
    mut buffer: *mut Buffer,
    mut length: rmtU32,
) -> rmtError {
    let mut granularity: rmtU32 = 0;
    let mut allocate: rmtU32 = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    granularity = (*buffer).alloc_granularity;
    allocate = ((*buffer).bytes_allocated).wrapping_add(length);
    allocate = (allocate as libc::c_uint)
        .wrapping_add(
            granularity
                .wrapping_sub(1 as libc::c_uint)
                .wrapping_sub(
                    allocate.wrapping_sub(1 as libc::c_uint).wrapping_rem(granularity),
                ),
        ) as rmtU32 as rmtU32;
    (*buffer).bytes_allocated = allocate;
    tmp = rmtRealloc((*buffer).data as *mut libc::c_void, (*buffer).bytes_allocated);
    (*buffer).data = tmp as *mut rmtU8;
    if (*buffer).data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Buffer_Pad(
    mut buffer: *mut Buffer,
    mut length: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    if !(buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2483 as libc::c_uint,
            b"Buffer_Pad\0" as *const u8 as *const libc::c_char,
        );
    }
    if ((*buffer).bytes_used).wrapping_add(length) > (*buffer).bytes_allocated {
        tmp___0 = Buffer_Grow(buffer, length);
        error = tmp___0;
        if error as libc::c_uint != 0 as libc::c_uint {
            return error;
        }
    }
    (*buffer)
        .bytes_used = ((*buffer).bytes_used as libc::c_uint).wrapping_add(length)
        as rmtU32 as rmtU32;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Buffer_AlignedPad(
    mut buffer: *mut Buffer,
    mut start_pos: rmtU32,
) -> rmtError {
    let mut tmp: rmtError = RMT_ERROR_NONE;
    tmp = Buffer_Pad(
        buffer,
        (4 as libc::c_uint)
            .wrapping_sub(
                ((*buffer).bytes_used).wrapping_sub(start_pos) & 3 as libc::c_uint,
            ) & 3 as libc::c_uint,
    );
    return tmp;
}
unsafe extern "C" fn Buffer_Write(
    mut buffer: *mut Buffer,
    mut data: *const libc::c_void,
    mut length: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    if !(buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2504 as libc::c_uint,
            b"Buffer_Write\0" as *const u8 as *const libc::c_char,
        );
    }
    if ((*buffer).bytes_used).wrapping_add(length) > (*buffer).bytes_allocated {
        tmp___0 = Buffer_Grow(buffer, length);
        error = tmp___0;
        if error as libc::c_uint != 0 as libc::c_uint {
            return error;
        }
    }
    memcpy(
        ((*buffer).data).offset((*buffer).bytes_used as isize) as *mut libc::c_void,
        data,
        length as size_t,
    );
    (*buffer)
        .bytes_used = ((*buffer).bytes_used as libc::c_uint).wrapping_add(length)
        as rmtU32 as rmtU32;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Buffer_WriteStringZ(
    mut buffer: *mut Buffer,
    mut string: rmtPStr,
) -> rmtError {
    let mut tmp___0: r_size_t = 0;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    if !(string as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"string != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2521 as libc::c_uint,
            b"Buffer_WriteStringZ\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = strnlen_s_safe_c(string, 2048 as libc::c_int as r_size_t);
    tmp___1 = Buffer_Write(
        buffer,
        string as *mut libc::c_void as *const libc::c_void,
        tmp___0.wrapping_add(1 as libc::c_uint),
    );
    return tmp___1;
}
unsafe extern "C" fn U32ToByteArray(mut dest: *mut rmtU8, mut value: rmtU32) {
    *dest.offset(0 as libc::c_int as isize) = (value & 255 as libc::c_uint) as rmtU8;
    *dest
        .offset(
            1 as libc::c_int as isize,
        ) = (value >> 8 as libc::c_int & 255 as libc::c_uint) as rmtU8;
    *dest
        .offset(
            2 as libc::c_int as isize,
        ) = (value >> 16 as libc::c_int & 255 as libc::c_uint) as rmtU8;
    *dest.offset(3 as libc::c_int as isize) = (value >> 24 as libc::c_int) as rmtU8;
}
unsafe extern "C" fn Buffer_WriteU32(
    mut buffer: *mut Buffer,
    mut value: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    if !(buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2541 as libc::c_uint,
            b"Buffer_WriteU32\0" as *const u8 as *const libc::c_char,
        );
    }
    if ((*buffer).bytes_used as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<rmtU32>() as libc::c_ulong)
        > (*buffer).bytes_allocated as libc::c_ulong
    {
        tmp___0 = Buffer_Grow(
            buffer,
            ::std::mem::size_of::<rmtU32>() as libc::c_ulong as rmtU32,
        );
        error = tmp___0;
        if error as libc::c_uint != 0 as libc::c_uint {
            return error;
        }
    }
    U32ToByteArray(((*buffer).data).offset((*buffer).bytes_used as isize), value);
    (*buffer)
        .bytes_used = ((*buffer).bytes_used as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<rmtU32>() as libc::c_ulong) as rmtU32;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn IsLittleEndian() -> rmtBool {
    let mut u: __anonunion_u_919609132 = __anonunion_u_919609132 { i: 0 };
    let mut tmp: rmtBool = 0;
    u.i = 1 as libc::c_uint;
    if u.c[0 as libc::c_int as usize] as libc::c_int == 1 as libc::c_int {
        tmp = 1 as libc::c_int as rmtBool;
    } else {
        tmp = 0 as libc::c_int as rmtBool;
    }
    return tmp;
}
unsafe extern "C" fn Buffer_WriteF64(
    mut buffer: *mut Buffer,
    mut value: rmtF64,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut u: __anonunion_u_682901414 = __anonunion_u_682901414 { d: 0. };
    let mut dest: *mut rmtU8 = 0 as *mut rmtU8;
    let mut tmp___1: rmtBool = 0;
    if !(buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2575 as libc::c_uint,
            b"Buffer_WriteF64\0" as *const u8 as *const libc::c_char,
        );
    }
    if ((*buffer).bytes_used as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<rmtF64>() as libc::c_ulong)
        > (*buffer).bytes_allocated as libc::c_ulong
    {
        tmp___0 = Buffer_Grow(
            buffer,
            ::std::mem::size_of::<rmtF64>() as libc::c_ulong as rmtU32,
        );
        error = tmp___0;
        if error as libc::c_uint != 0 as libc::c_uint {
            return error;
        }
    }
    dest = ((*buffer).data).offset((*buffer).bytes_used as isize);
    u.d = value;
    tmp___1 = IsLittleEndian();
    if tmp___1 != 0 {
        *dest.offset(0 as libc::c_int as isize) = u.c[0 as libc::c_int as usize];
        *dest.offset(1 as libc::c_int as isize) = u.c[1 as libc::c_int as usize];
        *dest.offset(2 as libc::c_int as isize) = u.c[2 as libc::c_int as usize];
        *dest.offset(3 as libc::c_int as isize) = u.c[3 as libc::c_int as usize];
        *dest.offset(4 as libc::c_int as isize) = u.c[4 as libc::c_int as usize];
        *dest.offset(5 as libc::c_int as isize) = u.c[5 as libc::c_int as usize];
        *dest.offset(6 as libc::c_int as isize) = u.c[6 as libc::c_int as usize];
        *dest.offset(7 as libc::c_int as isize) = u.c[7 as libc::c_int as usize];
    } else {
        *dest.offset(0 as libc::c_int as isize) = u.c[7 as libc::c_int as usize];
        *dest.offset(1 as libc::c_int as isize) = u.c[6 as libc::c_int as usize];
        *dest.offset(2 as libc::c_int as isize) = u.c[5 as libc::c_int as usize];
        *dest.offset(3 as libc::c_int as isize) = u.c[4 as libc::c_int as usize];
        *dest.offset(4 as libc::c_int as isize) = u.c[3 as libc::c_int as usize];
        *dest.offset(5 as libc::c_int as isize) = u.c[2 as libc::c_int as usize];
        *dest.offset(6 as libc::c_int as isize) = u.c[1 as libc::c_int as usize];
        *dest.offset(7 as libc::c_int as isize) = u.c[0 as libc::c_int as usize];
    }
    (*buffer)
        .bytes_used = ((*buffer).bytes_used as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<rmtF64>() as libc::c_ulong) as rmtU32;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Buffer_WriteU64(
    mut buffer: *mut Buffer,
    mut value: rmtU64,
) -> rmtError {
    let mut tmp: rmtError = RMT_ERROR_NONE;
    tmp = Buffer_WriteF64(buffer, value as libc::c_double);
    return tmp;
}
unsafe extern "C" fn Buffer_WriteStringWithLength(
    mut buffer: *mut Buffer,
    mut string: rmtPStr,
) -> rmtError {
    let mut length: rmtU32 = 0;
    let mut tmp: r_size_t = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    tmp = strnlen_s_safe_c(string, 2048 as libc::c_int as r_size_t);
    length = tmp;
    tmp___0 = Buffer_WriteU32(buffer, length);
    error = tmp___0;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    tmp___1 = Buffer_Write(
        buffer,
        string as *mut libc::c_void as *const libc::c_void,
        length,
    );
    return tmp___1;
}
unsafe extern "C" fn rmtHashTable_Constructor(
    mut table: *mut rmtHashTable,
    mut max_nb_slots: rmtU32,
) -> rmtError {
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if !(table as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"table != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2669 as libc::c_uint,
            b"rmtHashTable_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*table).maxNbSlots = max_nb_slots;
    (*table).nbSlots = 0 as libc::c_int as rmtU32;
    tmp___0 = rmtMalloc(
        ((*table).maxNbSlots as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<HashSlot>() as libc::c_ulong) as rmtU32,
    );
    (*table).slots = tmp___0 as *mut HashSlot;
    if (*table).slots as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    memset(
        (*table).slots as *mut libc::c_void,
        0 as libc::c_int,
        ((*table).maxNbSlots as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<HashSlot>() as libc::c_ulong),
    );
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtHashTable_Destructor(mut table: *mut rmtHashTable) {
    if !(table as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"table != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2682 as libc::c_uint,
            b"rmtHashTable_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*table).slots as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        rmtFree((*table).slots as *mut libc::c_void);
        (*table).slots = 0 as *mut libc::c_void as *mut HashSlot;
    }
}
unsafe extern "C" fn rmtHashTable_Insert(
    mut table: *mut rmtHashTable,
    mut key: rmtU32,
    mut value: rmtU64,
) -> rmtError {
    let mut slot: *mut HashSlot = 0 as *mut HashSlot;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut index_mask: rmtU32 = 0;
    let mut index___0: rmtU32 = 0;
    slot = 0 as *mut libc::c_void as *mut HashSlot;
    error = RMT_ERROR_NONE;
    index_mask = ((*table).maxNbSlots).wrapping_sub(1 as libc::c_uint);
    index___0 = key & index_mask;
    if !(key != 0 as libc::c_uint) {
        __assert_fail(
            b"key != 0\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2702 as libc::c_uint,
            b"rmtHashTable_Insert\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(value != 0xffffffffffffffff as libc::c_ulonglong) {
        __assert_fail(
            b"value != RMT_NOT_FOUND\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2703 as libc::c_uint,
            b"rmtHashTable_Insert\0" as *const u8 as *const libc::c_char,
        );
    }
    while (*((*table).slots).offset(index___0 as isize)).key != 0 {
        if (*((*table).slots).offset(index___0 as isize)).key == key {
            (*table).nbSlots = ((*table).nbSlots).wrapping_sub(1);
            break;
        } else {
            index___0 = index___0.wrapping_add(1 as libc::c_uint) & index_mask;
        }
    }
    if !(index___0 < (*table).maxNbSlots) {
        __assert_fail(
            b"index < table->maxNbSlots\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2720 as libc::c_uint,
            b"rmtHashTable_Insert\0" as *const u8 as *const libc::c_char,
        );
    }
    slot = ((*table).slots).offset(index___0 as isize);
    (*slot).key = key;
    (*slot).value = value;
    (*table).nbSlots = ((*table).nbSlots).wrapping_add(1);
    if (*table).nbSlots
        > ((*table).maxNbSlots)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_div(3 as libc::c_uint)
    {
        error = rmtHashTable_Resize(table);
    }
    return error;
}
unsafe extern "C" fn rmtHashTable_Resize(mut table: *mut rmtHashTable) -> rmtError {
    let mut old_max_nb_slots: rmtU32 = 0;
    let mut new_slots: *mut HashSlot = 0 as *mut HashSlot;
    let mut old_slots: *mut HashSlot = 0 as *mut HashSlot;
    let mut i: rmtU32 = 0;
    let mut new_max_nb_slots: rmtU32 = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut slot: *mut HashSlot = 0 as *mut HashSlot;
    old_max_nb_slots = (*table).maxNbSlots;
    new_slots = 0 as *mut libc::c_void as *mut HashSlot;
    old_slots = (*table).slots;
    new_max_nb_slots = (*table).maxNbSlots;
    if new_max_nb_slots < 32768 as libc::c_uint {
        new_max_nb_slots = (new_max_nb_slots as libc::c_uint)
            .wrapping_mul(4 as libc::c_uint) as rmtU32 as rmtU32;
    } else {
        new_max_nb_slots = (new_max_nb_slots as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint) as rmtU32 as rmtU32;
    }
    tmp = rmtMalloc(
        (new_max_nb_slots as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<HashSlot>() as libc::c_ulong) as rmtU32,
    );
    new_slots = tmp as *mut HashSlot;
    if new_slots as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    memset(
        new_slots as *mut libc::c_void,
        0 as libc::c_int,
        (new_max_nb_slots as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<HashSlot>() as libc::c_ulong),
    );
    (*table).slots = new_slots;
    (*table).maxNbSlots = new_max_nb_slots;
    (*table).nbSlots = 0 as libc::c_int as rmtU32;
    i = 0 as libc::c_int as rmtU32;
    while i < old_max_nb_slots {
        slot = old_slots.offset(i as isize);
        if (*slot).key != 0 as libc::c_uint {
            rmtHashTable_Insert(table, (*slot).key, (*slot).value);
        }
        i = i.wrapping_add(1);
    }
    rmtFree(old_slots as *mut libc::c_void);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtHashTable_Find(
    mut table: *mut rmtHashTable,
    mut key: rmtU32,
) -> rmtU64 {
    let mut index_mask: rmtU32 = 0;
    let mut index___0: rmtU32 = 0;
    let mut slot: *mut HashSlot = 0 as *mut HashSlot;
    index_mask = ((*table).maxNbSlots).wrapping_sub(1 as libc::c_uint);
    index___0 = key & index_mask;
    while (*((*table).slots).offset(index___0 as isize)).key != 0 {
        slot = ((*table).slots).offset(index___0 as isize);
        if (*slot).key == key {
            return (*slot).value;
        }
        index___0 = index___0.wrapping_add(1 as libc::c_uint) & index_mask;
    }
    return 18446744073709551615 as libc::c_ulonglong;
}
unsafe extern "C" fn StringTable_Constructor(mut table: *mut StringTable) -> rmtError {
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: rmtError = RMT_ERROR_NONE;
    if !(table as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"table != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2821 as libc::c_uint,
            b"StringTable_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*table).text = 0 as *mut libc::c_void as *mut Buffer;
    (*table).text_map = 0 as *mut libc::c_void as *mut rmtHashTable;
    tmp___0 = rmtMalloc(::std::mem::size_of::<Buffer>() as libc::c_ulong as rmtU32);
    (*table).text = tmp___0 as *mut Buffer;
    if (*table).text as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___1 = Buffer_Constructor((*table).text, 8192 as libc::c_int as rmtU32);
    error = tmp___1;
    if error as libc::c_uint != 0 as libc::c_uint {
        if (*table).text as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            Buffer_Destructor((*table).text);
            rmtFree((*table).text as *mut libc::c_void);
            (*table).text = 0 as *mut libc::c_void as *mut Buffer;
        }
        return error;
    }
    tmp___2 = rmtMalloc(
        ::std::mem::size_of::<rmtHashTable>() as libc::c_ulong as rmtU32,
    );
    (*table).text_map = tmp___2 as *mut rmtHashTable;
    if (*table).text_map as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___3 = rmtHashTable_Constructor((*table).text_map, 1024 as libc::c_int as rmtU32);
    error___0 = tmp___3;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        if (*table).text_map as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
        {
            rmtHashTable_Destructor((*table).text_map);
            rmtFree((*table).text_map as *mut libc::c_void);
            (*table).text_map = 0 as *mut libc::c_void as *mut rmtHashTable;
        }
        return error___0;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn StringTable_Destructor(mut table: *mut StringTable) {
    if !(table as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"table != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2834 as libc::c_uint,
            b"StringTable_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*table).text_map as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        rmtHashTable_Destructor((*table).text_map);
        rmtFree((*table).text_map as *mut libc::c_void);
        (*table).text_map = 0 as *mut libc::c_void as *mut rmtHashTable;
    }
    if (*table).text as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        Buffer_Destructor((*table).text);
        rmtFree((*table).text as *mut libc::c_void);
        (*table).text = 0 as *mut libc::c_void as *mut Buffer;
    }
}
unsafe extern "C" fn StringTable_Find(
    mut table: *mut StringTable,
    mut name_hash: rmtU32,
) -> rmtPStr {
    let mut text_offset: rmtU64 = 0;
    let mut tmp: rmtU64 = 0;
    tmp = rmtHashTable_Find((*table).text_map, name_hash);
    text_offset = tmp;
    if text_offset != 0xffffffffffffffff as libc::c_ulonglong {
        return ((*(*table).text).data).offset(text_offset as isize) as rmtPStr;
    }
    return 0 as *mut libc::c_void as rmtPStr;
}
unsafe extern "C" fn StringTable_Insert(
    mut table: *mut StringTable,
    mut name_hash: rmtU32,
    mut name: rmtPStr,
) -> rmtBool {
    let mut text_offset: rmtU64 = 0;
    let mut tmp: rmtU64 = 0;
    tmp = rmtHashTable_Find((*table).text_map, name_hash);
    text_offset = tmp;
    if text_offset == 0xffffffffffffffff as libc::c_ulonglong {
        text_offset = (*(*table).text).bytes_used as rmtU64;
        Buffer_WriteStringZ((*table).text, name);
        rmtHashTable_Insert((*table).text_map, name_hash, text_offset);
        return 1 as libc::c_int as rmtBool;
    }
    return 0 as libc::c_int as rmtBool;
}
unsafe extern "C" fn InitialiseNetwork() -> rmtError {
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ShutdownNetwork() {}
unsafe extern "C" fn TCPSocket_Constructor(mut tcp_socket: *mut TCPSocket) -> rmtError {
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    if !(tcp_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2931 as libc::c_uint,
            b"TCPSocket_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*tcp_socket).socket = -(1 as libc::c_int);
    tmp___0 = InitialiseNetwork();
    return tmp___0;
}
unsafe extern "C" fn TCPSocket_Destructor(mut tcp_socket: *mut TCPSocket) {
    if !(tcp_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2938 as libc::c_uint,
            b"TCPSocket_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    TCPSocket_Close(tcp_socket);
    ShutdownNetwork();
}
unsafe extern "C" fn TCPSocket_RunServer(
    mut tcp_socket: *mut TCPSocket,
    mut port: rmtU16,
    mut reuse_open_port: rmtBool,
    mut limit_connections_to_localhost: rmtBool,
) -> rmtError {
    let mut s: SOCKET = 0;
    let mut sin___0: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut enable: libc::c_int = 0;
    let mut tmp___1: in_addr_t = 0;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: rmtError = RMT_ERROR_NONE;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: rmtError = RMT_ERROR_NONE;
    let mut tmp___7: libc::c_int = 0;
    s = -(1 as libc::c_int);
    memset(
        &mut sin___0 as *mut sockaddr_in as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong,
    );
    if !(tcp_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            2953 as libc::c_uint,
            b"TCPSocket_RunServer\0" as *const u8 as *const libc::c_char,
        );
    }
    s = socket(2 as libc::c_int, 1 as libc::c_int, 6 as libc::c_int);
    if s == -(1 as libc::c_int) {
        tmp___0 = rmtMakeError(
            RMT_ERROR_RESOURCE_CREATE_FAIL,
            b"Can't create a socket for connection to the remote viewer\0" as *const u8
                as *const libc::c_char,
        );
        return tmp___0;
    }
    if reuse_open_port != 0 {
        enable = 1 as libc::c_int;
        setsockopt(
            s,
            1 as libc::c_int,
            2 as libc::c_int,
            &mut enable as *mut libc::c_int as *const libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
        );
    }
    sin___0.sin_family = 2 as libc::c_int as sa_family_t;
    if limit_connections_to_localhost != 0 {
        tmp___1 = 2130706433 as libc::c_int as in_addr_t;
    } else {
        tmp___1 = 0 as libc::c_int as in_addr_t;
    }
    sin___0.sin_addr.s_addr = htonl(tmp___1);
    sin___0.sin_port = htons(port);
    tmp___3 = bind(
        s,
        &mut sin___0 as *mut sockaddr_in as *mut sockaddr as *const sockaddr,
        ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t,
    );
    if tmp___3 == -(1 as libc::c_int) {
        tmp___2 = rmtMakeError(
            RMT_ERROR_RESOURCE_ACCESS_FAIL,
            b"Can't bind a socket for the server\0" as *const u8 as *const libc::c_char,
        );
        return tmp___2;
    }
    (*tcp_socket).socket = s;
    tmp___5 = listen(s, 1 as libc::c_int);
    if tmp___5 == -(1 as libc::c_int) {
        tmp___4 = rmtMakeError(
            RMT_ERROR_RESOURCE_ACCESS_FAIL,
            b"Created server socket failed to enter a listen state\0" as *const u8
                as *const libc::c_char,
        );
        return tmp___4;
    }
    tmp___7 = fcntl((*tcp_socket).socket, 4 as libc::c_int, 2048 as libc::c_int);
    if tmp___7 == -(1 as libc::c_int) {
        tmp___6 = rmtMakeError(
            RMT_ERROR_RESOURCE_ACCESS_FAIL,
            b"Created server socket failed to switch to a non-blocking state\0"
                as *const u8 as *const libc::c_char,
        );
        return tmp___6;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn TCPSocket_Close(mut tcp_socket: *mut TCPSocket) {
    let mut result: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut temp_buf: [libc::c_char; 128] = [0; 128];
    let mut tmp___1: ssize_t = 0;
    if !(tcp_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3017 as libc::c_uint,
            b"TCPSocket_Close\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*tcp_socket).socket != -(1 as libc::c_int) {
        tmp___0 = shutdown((*tcp_socket).socket, 1 as libc::c_int);
        result = tmp___0;
        if result != -(1 as libc::c_int) {
            total = 0 as libc::c_int;
            while result > 0 as libc::c_int {
                tmp___1 = recv(
                    (*tcp_socket).socket,
                    temp_buf.as_mut_ptr() as *mut libc::c_void,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    0 as libc::c_int,
                );
                result = tmp___1 as libc::c_int;
                total += result;
            }
        }
        close((*tcp_socket).socket);
        (*tcp_socket).socket = -(1 as libc::c_int);
    }
}
unsafe extern "C" fn TCPSocket_PollStatus(
    mut tcp_socket: *mut TCPSocket,
) -> SocketStatus {
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut fd_read: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fd_write: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut fd_errors: fd_set = fd_set { __fds_bits: [0; 16] };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let mut __d0___0: libc::c_int = 0;
    let mut __d1___0: libc::c_int = 0;
    let mut __d0___1: libc::c_int = 0;
    let mut __d1___1: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    status.can_read = 0 as libc::c_int as rmtBool;
    status.can_write = 0 as libc::c_int as rmtBool;
    status.error_state = RMT_ERROR_NONE;
    if !(tcp_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3051 as libc::c_uint,
            b"TCPSocket_PollStatus\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*tcp_socket).socket == -(1 as libc::c_int) {
        status.error_state = RMT_ERROR_SOCKET_INVALID_POLL;
        return status;
    }
    let fresh0 = &mut __d0;
    let fresh1;
    let fresh2 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh3 = &mut __d1;
    let fresh4;
    let fresh5 = &mut *(fd_read.__fds_bits)
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
    let fresh6 = &mut __d0___0;
    let fresh7;
    let fresh8 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh9 = &mut __d1___0;
    let fresh10;
    let fresh11 = &mut *(fd_write.__fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh6,
        fresh8) => fresh7, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh9,
        fresh11) => fresh10, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh6, fresh8, fresh7);
    c2rust_asm_casts::AsmCast::cast_out(fresh9, fresh11, fresh10);
    let fresh12 = &mut __d0___1;
    let fresh13;
    let fresh14 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh15 = &mut __d1___1;
    let fresh16;
    let fresh17 = &mut *(fd_errors.__fds_bits)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh12,
        fresh14) => fresh13, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh15,
        fresh17) => fresh16, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags, att_syntax)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh12, fresh14, fresh13);
    c2rust_asm_casts::AsmCast::cast_out(fresh15, fresh17, fresh16);
    fd_read
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    fd_write
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    fd_errors
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        |= ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask;
    tv.tv_sec = 0 as libc::c_int as __time_t;
    tv.tv_usec = 0 as libc::c_int as __suseconds_t;
    tmp___0 = select(
        (*tcp_socket).socket + 1 as libc::c_int,
        &mut fd_read as *mut fd_set,
        &mut fd_write as *mut fd_set,
        &mut fd_errors as *mut fd_set,
        &mut tv as *mut timeval,
    );
    if tmp___0 == -(1 as libc::c_int) {
        status.error_state = RMT_ERROR_SOCKET_SELECT_FAIL;
        return status;
    }
    if (fd_read
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask != 0 as libc::c_long)
        as libc::c_int != 0 as libc::c_int
    {
        status.can_read = 1 as libc::c_int as rmtBool;
    } else {
        status.can_read = 0 as libc::c_int as rmtBool;
    }
    if (fd_write
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask != 0 as libc::c_long)
        as libc::c_int != 0 as libc::c_int
    {
        status.can_write = 1 as libc::c_int as rmtBool;
    } else {
        status.can_write = 0 as libc::c_int as rmtBool;
    }
    if (fd_errors
        .__fds_bits[((*tcp_socket).socket
        / (8 as libc::c_int
            * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
        as usize]
        & ((1 as libc::c_ulong)
            << (*tcp_socket).socket
                % (8 as libc::c_int
                    * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                        as libc::c_int)) as __fd_mask != 0 as libc::c_long)
        as libc::c_int != 0 as libc::c_int
    {
        status.error_state = RMT_ERROR_SOCKET_POLL_ERRORS;
    } else {
        status.error_state = RMT_ERROR_NONE;
    }
    return status;
}
unsafe extern "C" fn TCPSocket_AcceptConnection(
    mut tcp_socket: *mut TCPSocket,
    mut client_socket: *mut *mut TCPSocket,
) -> rmtError {
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut s: SOCKET = 0;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: rmtError = RMT_ERROR_NONE;
    if !(tcp_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3094 as libc::c_uint,
            b"TCPSocket_AcceptConnection\0" as *const u8 as *const libc::c_char,
        );
    }
    status = TCPSocket_PollStatus(tcp_socket);
    if status.error_state as libc::c_uint != 0 as libc::c_uint {
        return status.error_state
    } else {
        if status.can_read == 0 {
            return status.error_state;
        }
    }
    s = accept((*tcp_socket).socket, 0 as *mut sockaddr, 0 as *mut socklen_t);
    if s == -(1 as libc::c_int) {
        tmp___0 = rmtMakeError(
            RMT_ERROR_RESOURCE_CREATE_FAIL,
            b"Server failed to accept connection from client\0" as *const u8
                as *const libc::c_char,
        );
        return tmp___0;
    }
    if !(client_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"client_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3119 as libc::c_uint,
            b"TCPSocket_AcceptConnection\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___2 = rmtMalloc(::std::mem::size_of::<TCPSocket>() as libc::c_ulong as rmtU32);
    *client_socket = tmp___2 as *mut TCPSocket;
    if *client_socket as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___3 = TCPSocket_Constructor(*client_socket);
    error = tmp___3;
    if error as libc::c_uint != 0 as libc::c_uint {
        if *client_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            TCPSocket_Destructor(*client_socket);
            rmtFree(*client_socket as *mut libc::c_void);
            *client_socket = 0 as *mut libc::c_void as *mut TCPSocket;
        }
        return error;
    }
    (**client_socket).socket = s;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn TCPTryAgain() -> libc::c_int {
    let mut tmp: *mut libc::c_int = 0 as *mut libc::c_int;
    tmp = __errno_location();
    return (*tmp == 11 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn TCPSocket_Send(
    mut tcp_socket: *mut TCPSocket,
    mut data: *const libc::c_void,
    mut length: rmtU32,
    mut timeout_ms: rmtU32,
) -> rmtError {
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut cur_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_ms: rmtU32 = 0;
    let mut cur_ms: rmtU32 = 0;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut bytes_sent: libc::c_int = 0;
    let mut send_flags: libc::c_int = 0;
    let mut tmp___1: ssize_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: rmtError = RMT_ERROR_NONE;
    cur_data = 0 as *mut libc::c_void as *mut libc::c_char;
    end_data = 0 as *mut libc::c_void as *mut libc::c_char;
    start_ms = 0 as libc::c_int as rmtU32;
    cur_ms = 0 as libc::c_int as rmtU32;
    if !(tcp_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3148 as libc::c_uint,
            b"TCPSocket_Send\0" as *const u8 as *const libc::c_char,
        );
    }
    start_ms = msTimer_Get();
    status.can_write = 0 as libc::c_int as rmtBool;
    while status.can_write == 0 {
        status = TCPSocket_PollStatus(tcp_socket);
        if status.error_state as libc::c_uint != 0 as libc::c_uint {
            return status.error_state;
        }
        cur_ms = msTimer_Get();
        if cur_ms.wrapping_sub(start_ms) > timeout_ms {
            tmp___0 = rmtMakeError(
                RMT_ERROR_TIMEOUT,
                b"Timed out trying to send data\0" as *const u8 as *const libc::c_char,
            );
            return tmp___0;
        }
    }
    cur_data = data as *mut libc::c_char;
    end_data = cur_data.offset(length as isize);
    while (cur_data as libc::c_ulong) < end_data as libc::c_ulong {
        send_flags = 0 as libc::c_int;
        send_flags = 16384 as libc::c_int;
        tmp___1 = send(
            (*tcp_socket).socket,
            cur_data as *const libc::c_void,
            end_data.offset_from(cur_data) as libc::c_long as libc::c_int as size_t,
            send_flags,
        );
        bytes_sent = tmp___1 as libc::c_int;
        if !(bytes_sent == -(1 as libc::c_int)) {
            if !(bytes_sent == 0 as libc::c_int) {
                cur_data = cur_data.offset(bytes_sent as isize);
                continue;
            }
        }
        if bytes_sent != 0 as libc::c_int {
            tmp___2 = TCPTryAgain();
            if tmp___2 == 0 {
                return RMT_ERROR_SOCKET_SEND_FAIL;
            }
        }
        cur_ms = msTimer_Get();
        if cur_ms < start_ms {
            start_ms = cur_ms;
        } else if cur_ms.wrapping_sub(start_ms) > timeout_ms {
            tmp___3 = rmtMakeError(
                RMT_ERROR_TIMEOUT,
                b"Timed out trying to send data\0" as *const u8 as *const libc::c_char,
            );
            return tmp___3;
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn TCPSocket_Receive(
    mut tcp_socket: *mut TCPSocket,
    mut data: *mut libc::c_void,
    mut length: rmtU32,
    mut timeout_ms: rmtU32,
) -> rmtError {
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut cur_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_ms: rmtU32 = 0;
    let mut cur_ms: rmtU32 = 0;
    let mut bytes_received: libc::c_int = 0;
    let mut tmp___0: ssize_t = 0;
    let mut tmp___1: libc::c_int = 0;
    cur_data = 0 as *mut libc::c_void as *mut libc::c_char;
    end_data = 0 as *mut libc::c_void as *mut libc::c_char;
    start_ms = 0 as libc::c_int as rmtU32;
    cur_ms = 0 as libc::c_int as rmtU32;
    if !(tcp_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3230 as libc::c_uint,
            b"TCPSocket_Receive\0" as *const u8 as *const libc::c_char,
        );
    }
    status = TCPSocket_PollStatus(tcp_socket);
    if status.error_state as libc::c_uint != 0 as libc::c_uint {
        return status.error_state;
    }
    if status.can_read == 0 {
        return RMT_ERROR_SOCKET_RECV_NO_DATA;
    }
    cur_data = data as *mut libc::c_char;
    end_data = cur_data.offset(length as isize);
    start_ms = msTimer_Get();
    while (cur_data as libc::c_ulong) < end_data as libc::c_ulong {
        tmp___0 = recv(
            (*tcp_socket).socket,
            cur_data as *mut libc::c_void,
            end_data.offset_from(cur_data) as libc::c_long as libc::c_int as size_t,
            0 as libc::c_int,
        );
        bytes_received = tmp___0 as libc::c_int;
        if !(bytes_received == -(1 as libc::c_int)) {
            if !(bytes_received == 0 as libc::c_int) {
                cur_data = cur_data.offset(bytes_received as isize);
                continue;
            }
        }
        if bytes_received != 0 as libc::c_int {
            tmp___1 = TCPTryAgain();
            if tmp___1 == 0 {
                return RMT_ERROR_SOCKET_RECV_FAILED;
            }
        }
        cur_ms = msTimer_Get();
        if cur_ms < start_ms {
            start_ms = cur_ms;
        } else if cur_ms.wrapping_sub(start_ms) > timeout_ms {
            return RMT_ERROR_SOCKET_RECV_TIMEOUT
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rol(value: libc::c_uint, steps: libc::c_uint) -> libc::c_uint {
    return value << steps | value >> (32 as libc::c_uint).wrapping_sub(steps);
}
unsafe extern "C" fn clearWBuffert(mut buffert: *mut libc::c_uint) {
    let mut pos: libc::c_int = 0;
    pos = 16 as libc::c_int;
    loop {
        pos -= 1;
        if !(pos >= 0 as libc::c_int) {
            break;
        }
        *buffert.offset(pos as isize) = 0 as libc::c_uint;
    };
}
unsafe extern "C" fn innerHash(mut result: *mut libc::c_uint, mut w: *mut libc::c_uint) {
    let mut a: libc::c_uint = 0;
    let mut b: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    let mut d: libc::c_uint = 0;
    let mut e: libc::c_uint = 0;
    let mut round___0: libc::c_int = 0;
    let mut t: libc::c_uint = 0;
    let mut tmp: libc::c_uint = 0;
    let mut t___0: libc::c_uint = 0;
    let mut tmp___0: libc::c_uint = 0;
    let mut t___1: libc::c_uint = 0;
    let mut tmp___1: libc::c_uint = 0;
    let mut t___2: libc::c_uint = 0;
    let mut tmp___2: libc::c_uint = 0;
    let mut t___3: libc::c_uint = 0;
    let mut tmp___3: libc::c_uint = 0;
    a = *result.offset(0 as libc::c_int as isize);
    b = *result.offset(1 as libc::c_int as isize);
    c = *result.offset(2 as libc::c_int as isize);
    d = *result.offset(3 as libc::c_int as isize);
    e = *result.offset(4 as libc::c_int as isize);
    round___0 = 0 as libc::c_int;
    while round___0 < 16 as libc::c_int {
        tmp = rol(a, 5 as libc::c_int as libc::c_uint);
        t = tmp
            .wrapping_add(b & c | !b & d)
            .wrapping_add(e)
            .wrapping_add(1518500249 as libc::c_uint)
            .wrapping_add(*w.offset(round___0 as isize));
        e = d;
        d = c;
        c = rol(b, 30 as libc::c_int as libc::c_uint);
        b = a;
        a = t;
        round___0 += 1;
    }
    while round___0 < 20 as libc::c_int {
        *w
            .offset(
                round___0 as isize,
            ) = rol(
            *w.offset((round___0 - 3 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 8 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 14 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 16 as libc::c_int) as isize),
            1 as libc::c_int as libc::c_uint,
        );
        tmp___0 = rol(a, 5 as libc::c_int as libc::c_uint);
        t___0 = tmp___0
            .wrapping_add(b & c | !b & d)
            .wrapping_add(e)
            .wrapping_add(1518500249 as libc::c_uint)
            .wrapping_add(*w.offset(round___0 as isize));
        e = d;
        d = c;
        c = rol(b, 30 as libc::c_int as libc::c_uint);
        b = a;
        a = t___0;
        round___0 += 1;
    }
    while round___0 < 40 as libc::c_int {
        *w
            .offset(
                round___0 as isize,
            ) = rol(
            *w.offset((round___0 - 3 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 8 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 14 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 16 as libc::c_int) as isize),
            1 as libc::c_int as libc::c_uint,
        );
        tmp___1 = rol(a, 5 as libc::c_int as libc::c_uint);
        t___1 = tmp___1
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add(1859775393 as libc::c_uint)
            .wrapping_add(*w.offset(round___0 as isize));
        e = d;
        d = c;
        c = rol(b, 30 as libc::c_int as libc::c_uint);
        b = a;
        a = t___1;
        round___0 += 1;
    }
    while round___0 < 60 as libc::c_int {
        *w
            .offset(
                round___0 as isize,
            ) = rol(
            *w.offset((round___0 - 3 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 8 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 14 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 16 as libc::c_int) as isize),
            1 as libc::c_int as libc::c_uint,
        );
        tmp___2 = rol(a, 5 as libc::c_int as libc::c_uint);
        t___2 = tmp___2
            .wrapping_add(b & c | b & d | c & d)
            .wrapping_add(e)
            .wrapping_add(2400959708 as libc::c_uint)
            .wrapping_add(*w.offset(round___0 as isize));
        e = d;
        d = c;
        c = rol(b, 30 as libc::c_int as libc::c_uint);
        b = a;
        a = t___2;
        round___0 += 1;
    }
    while round___0 < 80 as libc::c_int {
        *w
            .offset(
                round___0 as isize,
            ) = rol(
            *w.offset((round___0 - 3 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 8 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 14 as libc::c_int) as isize)
                ^ *w.offset((round___0 - 16 as libc::c_int) as isize),
            1 as libc::c_int as libc::c_uint,
        );
        tmp___3 = rol(a, 5 as libc::c_int as libc::c_uint);
        t___3 = tmp___3
            .wrapping_add(b ^ c ^ d)
            .wrapping_add(e)
            .wrapping_add(3395469782 as libc::c_uint)
            .wrapping_add(*w.offset(round___0 as isize));
        e = d;
        d = c;
        c = rol(b, 30 as libc::c_int as libc::c_uint);
        b = a;
        a = t___3;
        round___0 += 1;
    }
    let ref mut fresh18 = *result.offset(0 as libc::c_int as isize);
    *fresh18 = (*fresh18).wrapping_add(a);
    let ref mut fresh19 = *result.offset(1 as libc::c_int as isize);
    *fresh19 = (*fresh19).wrapping_add(b);
    let ref mut fresh20 = *result.offset(2 as libc::c_int as isize);
    *fresh20 = (*fresh20).wrapping_add(c);
    let ref mut fresh21 = *result.offset(3 as libc::c_int as isize);
    *fresh21 = (*fresh21).wrapping_add(d);
    let ref mut fresh22 = *result.offset(4 as libc::c_int as isize);
    *fresh22 = (*fresh22).wrapping_add(e);
}
unsafe extern "C" fn calc(
    mut src: *const libc::c_void,
    bytelength: libc::c_int,
    mut hash: *mut libc::c_uchar,
) {
    let mut roundPos: libc::c_int = 0;
    let mut lastBlockBytes: libc::c_int = 0;
    let mut hashByte: libc::c_int = 0;
    let mut result: [libc::c_uint; 5] = [0; 5];
    let mut sarray: *const libc::c_uchar = 0 as *const libc::c_uchar;
    let mut w: [libc::c_uint; 80] = [0; 80];
    let mut endOfFullBlocks: libc::c_int = 0;
    let mut endCurrentBlock: libc::c_int = 0;
    let mut currentBlock: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    result[0 as libc::c_int as usize] = 1732584193 as libc::c_uint;
    result[1 as libc::c_int as usize] = 4023233417 as libc::c_uint;
    result[2 as libc::c_int as usize] = 2562383102 as libc::c_uint;
    result[3 as libc::c_int as usize] = 271733878 as libc::c_uint;
    result[4 as libc::c_int as usize] = 3285377520 as libc::c_uint;
    sarray = src as *const libc::c_uchar;
    endOfFullBlocks = bytelength - 64 as libc::c_int;
    currentBlock = 0 as libc::c_int;
    while currentBlock <= endOfFullBlocks {
        endCurrentBlock = currentBlock + 64 as libc::c_int;
        roundPos = 0 as libc::c_int;
        while currentBlock < endCurrentBlock {
            tmp = roundPos;
            roundPos += 1;
            w[tmp
                as usize] = *sarray.offset((currentBlock + 3 as libc::c_int) as isize)
                as libc::c_uint
                | (*sarray.offset((currentBlock + 2 as libc::c_int) as isize)
                    as libc::c_uint) << 8 as libc::c_int
                | (*sarray.offset((currentBlock + 1 as libc::c_int) as isize)
                    as libc::c_uint) << 16 as libc::c_int
                | (*sarray.offset(currentBlock as isize) as libc::c_uint)
                    << 24 as libc::c_int;
            currentBlock += 4 as libc::c_int;
        }
        innerHash(result.as_mut_ptr(), w.as_mut_ptr());
    }
    endCurrentBlock = bytelength - currentBlock;
    clearWBuffert(w.as_mut_ptr());
    lastBlockBytes = 0 as libc::c_int;
    while lastBlockBytes < endCurrentBlock {
        w[(lastBlockBytes >> 2 as libc::c_int) as usize]
            |= (*sarray.offset((lastBlockBytes + currentBlock) as isize) as libc::c_uint)
                << (3 as libc::c_int - (lastBlockBytes & 3 as libc::c_int)
                    << 3 as libc::c_int);
        lastBlockBytes += 1;
    }
    w[(lastBlockBytes >> 2 as libc::c_int) as usize]
        |= (128 as libc::c_uint)
            << (3 as libc::c_int - (lastBlockBytes & 3 as libc::c_int)
                << 3 as libc::c_int);
    if endCurrentBlock >= 56 as libc::c_int {
        innerHash(result.as_mut_ptr(), w.as_mut_ptr());
        clearWBuffert(w.as_mut_ptr());
    }
    w[15 as libc::c_int as usize] = (bytelength << 3 as libc::c_int) as libc::c_uint;
    innerHash(result.as_mut_ptr(), w.as_mut_ptr());
    hashByte = 20 as libc::c_int;
    loop {
        hashByte -= 1;
        if !(hashByte >= 0 as libc::c_int) {
            break;
        }
        *hash
            .offset(
                hashByte as isize,
            ) = (result[(hashByte >> 2 as libc::c_int) as usize]
            >> ((3 as libc::c_int - hashByte & 3 as libc::c_int) << 3 as libc::c_int)
            & 255 as libc::c_uint) as libc::c_uchar;
    };
}
unsafe extern "C" fn SHA1_Calculate(
    mut src: *const libc::c_void,
    mut length: libc::c_uint,
) -> SHA1 {
    let mut hash: SHA1 = SHA1 { data: [0; 20] };
    if !(length as libc::c_int >= 0 as libc::c_int) {
        __assert_fail(
            b"(int)length >= 0\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3476 as libc::c_uint,
            b"SHA1_Calculate\0" as *const u8 as *const libc::c_char,
        );
    }
    calc(src, length as libc::c_int, (hash.data).as_mut_ptr());
    return hash;
}
static mut b64_encoding_table: *const libc::c_char = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/\0"
    as *const u8 as *const libc::c_char;
unsafe extern "C" fn Base64_CalculateEncodedLength(mut length: rmtU32) -> rmtU32 {
    return (4 as libc::c_uint)
        .wrapping_mul(
            length.wrapping_add(2 as libc::c_uint).wrapping_div(3 as libc::c_uint),
        );
}
unsafe extern "C" fn Base64_Encode(
    mut in_bytes: *const rmtU8,
    mut length: rmtU32,
    mut out_bytes: *mut rmtU8,
) {
    let mut i: rmtU32 = 0;
    let mut encoded_length: rmtU32 = 0;
    let mut remaining_bytes: rmtU32 = 0;
    let mut optr: *mut rmtU8 = 0 as *mut rmtU8;
    let mut c0: rmtU32 = 0;
    let mut tmp: rmtU32 = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut c1: rmtU32 = 0;
    let mut tmp___1: rmtU32 = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut c2: rmtU32 = 0;
    let mut tmp___3: rmtU32 = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut triple: rmtU32 = 0;
    let mut tmp___5: *mut rmtU8 = 0 as *mut rmtU8;
    let mut tmp___6: *mut rmtU8 = 0 as *mut rmtU8;
    let mut tmp___7: *mut rmtU8 = 0 as *mut rmtU8;
    let mut tmp___8: *mut rmtU8 = 0 as *mut rmtU8;
    optr = out_bytes;
    i = 0 as libc::c_int as rmtU32;
    while i < length {
        if i < length {
            tmp = i;
            i = i.wrapping_add(1);
            tmp___0 = *in_bytes.offset(tmp as isize) as libc::c_int;
        } else {
            tmp___0 = 0 as libc::c_int;
        }
        c0 = tmp___0 as rmtU32;
        if i < length {
            tmp___1 = i;
            i = i.wrapping_add(1);
            tmp___2 = *in_bytes.offset(tmp___1 as isize) as libc::c_int;
        } else {
            tmp___2 = 0 as libc::c_int;
        }
        c1 = tmp___2 as rmtU32;
        if i < length {
            tmp___3 = i;
            i = i.wrapping_add(1);
            tmp___4 = *in_bytes.offset(tmp___3 as isize) as libc::c_int;
        } else {
            tmp___4 = 0 as libc::c_int;
        }
        c2 = tmp___4 as rmtU32;
        triple = (c0 << 16 as libc::c_int)
            .wrapping_add(c1 << 8 as libc::c_int)
            .wrapping_add(c2);
        tmp___5 = optr;
        optr = optr.offset(1);
        *tmp___5 = *b64_encoding_table
            .offset((triple >> 18 as libc::c_int & 63 as libc::c_uint) as isize)
            as rmtU8;
        tmp___6 = optr;
        optr = optr.offset(1);
        *tmp___6 = *b64_encoding_table
            .offset((triple >> 12 as libc::c_int & 63 as libc::c_uint) as isize)
            as rmtU8;
        tmp___7 = optr;
        optr = optr.offset(1);
        *tmp___7 = *b64_encoding_table
            .offset((triple >> 6 as libc::c_int & 63 as libc::c_uint) as isize) as rmtU8;
        tmp___8 = optr;
        optr = optr.offset(1);
        *tmp___8 = *b64_encoding_table.offset((triple & 63 as libc::c_uint) as isize)
            as rmtU8;
    }
    encoded_length = Base64_CalculateEncodedLength(length);
    remaining_bytes = (3 as libc::c_uint)
        .wrapping_sub(
            length.wrapping_add(2 as libc::c_uint).wrapping_rem(3 as libc::c_uint),
        )
        .wrapping_sub(1 as libc::c_uint);
    i = 0 as libc::c_int as rmtU32;
    while i < remaining_bytes {
        *out_bytes
            .offset(
                encoded_length.wrapping_sub(1 as libc::c_uint).wrapping_sub(i) as isize,
            ) = '=' as i32 as rmtU8;
        i = i.wrapping_add(1);
    }
    *out_bytes.offset(encoded_length as isize) = 0 as libc::c_int as rmtU8;
}
unsafe extern "C" fn rotl32(mut x: rmtU32, mut r: rmtS8) -> rmtU32 {
    return x << r as libc::c_int | x >> 32 as libc::c_int - r as libc::c_int;
}
unsafe extern "C" fn getblock32(mut p: *const rmtU32, mut i: libc::c_int) -> rmtU32 {
    let mut result: rmtU32 = 0;
    let mut src: *const rmtU8 = 0 as *const rmtU8;
    src = (p as *const rmtU8)
        .offset(
            (i * ::std::mem::size_of::<rmtU32>() as libc::c_ulong as libc::c_int)
                as isize,
        );
    memcpy(
        &mut result as *mut rmtU32 as *mut libc::c_void,
        src as *const libc::c_void,
        ::std::mem::size_of::<rmtU32>() as libc::c_ulong,
    );
    return result;
}
unsafe extern "C" fn fmix32(mut h: rmtU32) -> rmtU32 {
    h ^= h >> 16 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(2246822507 as libc::c_uint) as rmtU32 as rmtU32;
    h ^= h >> 13 as libc::c_int;
    h = (h as libc::c_uint).wrapping_mul(3266489909 as libc::c_uint) as rmtU32 as rmtU32;
    h ^= h >> 16 as libc::c_int;
    return h;
}
unsafe extern "C" fn MurmurHash3_x86_32(
    mut key: *const libc::c_void,
    mut len: libc::c_int,
    mut seed: rmtU32,
) -> rmtU32 {
    let mut data: *const rmtU8 = 0 as *const rmtU8;
    let mut nblocks: libc::c_int = 0;
    let mut h1: rmtU32 = 0;
    let mut c1: rmtU32 = 0;
    let mut c2: rmtU32 = 0;
    let mut i: libc::c_int = 0;
    let mut blocks: *const rmtU32 = 0 as *const rmtU32;
    let mut tail: *const rmtU8 = 0 as *const rmtU8;
    let mut k1: rmtU32 = 0;
    let mut k2: rmtU32 = 0;
    let mut tmp: rmtU32 = 0;
    data = key as *const rmtU8;
    nblocks = len / 4 as libc::c_int;
    h1 = seed;
    c1 = 3432918353 as libc::c_uint;
    c2 = 461845907 as libc::c_int as rmtU32;
    blocks = data.offset((nblocks * 4 as libc::c_int) as isize) as *const rmtU32;
    tail = data.offset((nblocks * 4 as libc::c_int) as isize);
    k1 = 0 as libc::c_int as rmtU32;
    i = -nblocks;
    while i != 0 {
        tmp = getblock32(blocks, i);
        k2 = tmp;
        k2 = (k2 as libc::c_uint).wrapping_mul(c1) as rmtU32 as rmtU32;
        k2 = rotl32(k2, 15 as libc::c_int as rmtS8);
        k2 = (k2 as libc::c_uint).wrapping_mul(c2) as rmtU32 as rmtU32;
        h1 ^= k2;
        h1 = rotl32(h1, 13 as libc::c_int as rmtS8);
        h1 = h1.wrapping_mul(5 as libc::c_uint).wrapping_add(3864292196 as libc::c_uint);
        i += 1;
    }
    let mut current_block_26: u64;
    match len & 3 as libc::c_int {
        3 => {
            k1
                ^= ((*tail.offset(2 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int) as libc::c_uint;
            current_block_26 = 13339781286175965048;
        }
        2 => {
            current_block_26 = 13339781286175965048;
        }
        1 => {
            current_block_26 = 3181618914532459458;
        }
        _ => {
            current_block_26 = 10652014663920648156;
        }
    }
    match current_block_26 {
        13339781286175965048 => {
            k1
                ^= ((*tail.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int) as libc::c_uint;
            current_block_26 = 3181618914532459458;
        }
        _ => {}
    }
    match current_block_26 {
        3181618914532459458 => {
            k1 ^= *tail.offset(0 as libc::c_int as isize) as libc::c_uint;
            k1 = (k1 as libc::c_uint).wrapping_mul(c1) as rmtU32 as rmtU32;
            k1 = rotl32(k1, 15 as libc::c_int as rmtS8);
            k1 = (k1 as libc::c_uint).wrapping_mul(c2) as rmtU32 as rmtU32;
            h1 ^= k1;
        }
        _ => {}
    }
    h1 ^= len as libc::c_uint;
    h1 = fmix32(h1);
    return h1;
}
pub unsafe extern "C" fn _rmt_HashString32(
    mut s: *const libc::c_char,
    mut len: libc::c_int,
    mut seed: rmtU32,
) -> rmtU32 {
    let mut tmp: rmtU32 = 0;
    tmp = MurmurHash3_x86_32(s as *const libc::c_void, len, seed);
    return tmp;
}
unsafe extern "C" fn GetField(
    mut buffer: *mut libc::c_char,
    mut buffer_length: r_size_t,
    mut field_name: rmtPStr,
) -> *mut libc::c_char {
    let mut field: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut field_length: r_size_t = 0;
    let mut tmp: r_size_t = 0;
    let mut tmp___0: errno_t = 0;
    let mut tmp___1: size_t = 0;
    field = 0 as *mut libc::c_void as *mut libc::c_char;
    buffer_end = buffer
        .offset(buffer_length as isize)
        .offset(-(1 as libc::c_int as isize));
    tmp = strnlen_s_safe_c(field_name, buffer_length);
    field_length = tmp;
    if field_length == 0 as libc::c_uint {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    tmp___0 = strstr_s(buffer, buffer_length, field_name, field_length, &mut field);
    if tmp___0 != 0 as libc::c_int {
        return 0 as *mut libc::c_void as *mut libc::c_char;
    }
    tmp___1 = strlen(field_name);
    field = field.offset(tmp___1 as isize);
    while *field as libc::c_int == 32 as libc::c_int {
        if field as libc::c_ulong >= buffer_end as libc::c_ulong {
            return 0 as *mut libc::c_void as *mut libc::c_char;
        }
        field = field.offset(1);
    }
    return field;
}
static mut websocket_guid: [libc::c_char; 37] = [
    '2' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'F' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'E' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '4' as i32 as libc::c_char,
    '7' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    '9' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    'D' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    '8' as i32 as libc::c_char,
    '5' as i32 as libc::c_char,
    'B' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
static mut websocket_response: [libc::c_char; 98] = [
    'H' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'T' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    '/' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '.' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    '0' as i32 as libc::c_char,
    '1' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'h' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'P' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'l' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    '\r' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'w' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    's' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '\r' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'C' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    'i' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'n' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    'U' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    'g' as i32 as libc::c_char,
    'r' as i32 as libc::c_char,
    'a' as i32 as libc::c_char,
    'd' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    '\r' as i32 as libc::c_char,
    '\n' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'W' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'b' as i32 as libc::c_char,
    'S' as i32 as libc::c_char,
    'o' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'k' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
    'A' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'c' as i32 as libc::c_char,
    'e' as i32 as libc::c_char,
    'p' as i32 as libc::c_char,
    't' as i32 as libc::c_char,
    ':' as i32 as libc::c_char,
    ' ' as i32 as libc::c_char,
    '\u{0}' as i32 as libc::c_char,
];
unsafe extern "C" fn WebSocketHandshake(
    mut tcp_socket: *mut TCPSocket,
    mut limit_host: rmtPStr,
) -> rmtError {
    let mut start_ms: rmtU32 = 0;
    let mut now_ms: rmtU32 = 0;
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut buffer_ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buffer_len: libc::c_int = 0;
    let mut buffer_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut response_buffer: [libc::c_char; 256] = [0; 256];
    let mut response_buffer_len: libc::c_int = 0;
    let mut version: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut key_end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut hash: SHA1 = SHA1 { data: [0; 20] };
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: libc::c_int = 0;
    let mut limit_host_len: r_size_t = 0;
    let mut tmp___3: r_size_t = 0;
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: errno_t = 0;
    let mut tmp___5: errno_t = 0;
    let mut tmp___6: errno_t = 0;
    let mut tmp___7: errno_t = 0;
    let mut tmp___8: r_size_t = 0;
    let mut tmp___9: errno_t = 0;
    let mut tmp___10: errno_t = 0;
    let mut tmp___11: errno_t = 0;
    let mut tmp___12: r_size_t = 0;
    let mut tmp___13: rmtError = RMT_ERROR_NONE;
    buffer_ptr = buffer.as_mut_ptr();
    buffer_len = (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_ulong) as libc::c_int;
    buffer_end = buffer.as_mut_ptr().offset(buffer_len as isize);
    response_buffer_len = (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_ulong) as libc::c_int;
    if !(tcp_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tcp_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3731 as libc::c_uint,
            b"WebSocketHandshake\0" as *const u8 as *const libc::c_char,
        );
    }
    start_ms = msTimer_Get();
    while (buffer_ptr.offset_from(buffer.as_mut_ptr()) as libc::c_long)
        < buffer_len as libc::c_long
    {
        tmp___0 = TCPSocket_Receive(
            tcp_socket,
            buffer_ptr as *mut libc::c_void,
            1 as libc::c_int as rmtU32,
            20 as libc::c_int as rmtU32,
        );
        error = tmp___0;
        if error as libc::c_uint == 18 as libc::c_uint {
            return error;
        }
        if !(error as libc::c_uint == 16 as libc::c_uint) {
            if !(error as libc::c_uint == 17 as libc::c_uint) {
                if !(error as libc::c_uint == 0 as libc::c_uint) {
                    __assert_fail(
                        b"error == RMT_ERROR_NONE\0" as *const u8 as *const libc::c_char,
                        b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                        3754 as libc::c_uint,
                        b"WebSocketHandshake\0" as *const u8 as *const libc::c_char,
                    );
                }
                if buffer_ptr.offset_from(buffer.as_mut_ptr()) as libc::c_long
                    >= 4 as libc::c_long
                {
                    if *buffer_ptr.offset(-(3 as libc::c_int as isize)) as libc::c_int
                        == 13 as libc::c_int
                    {
                        if *buffer_ptr.offset(-(2 as libc::c_int as isize))
                            as libc::c_int == 10 as libc::c_int
                        {
                            if *buffer_ptr.offset(-(1 as libc::c_int as isize))
                                as libc::c_int == 13 as libc::c_int
                            {
                                if *buffer_ptr.offset(-(0 as libc::c_int as isize))
                                    as libc::c_int == 10 as libc::c_int
                                {
                                    break;
                                }
                            }
                        }
                    }
                }
                buffer_ptr = buffer_ptr.offset(1);
                continue;
            }
        }
        now_ms = msTimer_Get();
        if now_ms.wrapping_sub(start_ms) > 1000 as libc::c_uint {
            return RMT_ERROR_SOCKET_RECV_TIMEOUT;
        }
    }
    *buffer_ptr = 0 as libc::c_int as libc::c_char;
    tmp___2 = memcmp(
        buffer.as_mut_ptr() as *const libc::c_void,
        b"GET\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        3 as libc::c_int as size_t,
    );
    if tmp___2 != 0 as libc::c_int {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_NOT_GET;
    }
    version = GetField(
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
        b"Sec-WebSocket-Version:\0" as *const u8 as *const libc::c_char,
    );
    if version as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_VERSION;
    }
    if (buffer_end.offset_from(version) as libc::c_long) < 2 as libc::c_long {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_VERSION
    } else {
        if *version.offset(0 as libc::c_int as isize) as libc::c_int != 56 as libc::c_int
        {
            if *version.offset(0 as libc::c_int as isize) as libc::c_int
                != 49 as libc::c_int
            {
                return RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_VERSION
            } else {
                if *version.offset(1 as libc::c_int as isize) as libc::c_int
                    != 51 as libc::c_int
                {
                    return RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_VERSION;
                }
            }
        }
    }
    host = GetField(
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
        b"Host:\0" as *const u8 as *const libc::c_char,
    );
    if host as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_HOST;
    }
    if limit_host as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___3 = strnlen_s_safe_c(limit_host, 128 as libc::c_int as r_size_t);
        limit_host_len = tmp___3;
        found = 0 as *mut libc::c_void as *mut libc::c_char;
        tmp___4 = strstr_s(
            host,
            buffer_end.offset_from(host) as libc::c_long as r_size_t,
            limit_host,
            limit_host_len,
            &mut found,
        );
        if tmp___4 != 0 as libc::c_int {
            return RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_HOST;
        }
    }
    key = GetField(
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
        b"Sec-WebSocket-Key:\0" as *const u8 as *const libc::c_char,
    );
    if key as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_NO_KEY;
    }
    tmp___5 = strstr_s(
        key,
        buffer_end.offset_from(key) as libc::c_long as r_size_t,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as r_size_t,
        &mut key_end,
    );
    if tmp___5 != 0 as libc::c_int {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_BAD_KEY;
    }
    *key_end = 0 as libc::c_int as libc::c_char;
    buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp___6 = strncat_s_safe_c(
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
        key as *const libc::c_char,
        key_end.offset_from(key) as libc::c_long as r_size_t,
    );
    if tmp___6 != 0 as libc::c_int {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL;
    }
    tmp___7 = strncat_s_safe_c(
        buffer.as_mut_ptr(),
        buffer_len as r_size_t,
        websocket_guid.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 37]>() as libc::c_ulong as r_size_t,
    );
    if tmp___7 != 0 as libc::c_int {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL;
    }
    tmp___8 = strnlen_s_safe_c(
        buffer.as_mut_ptr() as *const libc::c_char,
        buffer_len as r_size_t,
    );
    hash = SHA1_Calculate(buffer.as_mut_ptr() as *const libc::c_void, tmp___8);
    Base64_Encode(
        (hash.data).as_mut_ptr() as *const rmtU8,
        ::std::mem::size_of::<[rmtU8; 20]>() as libc::c_ulong as rmtU32,
        buffer.as_mut_ptr() as *mut rmtU8,
    );
    response_buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp___9 = strncat_s_safe_c(
        response_buffer.as_mut_ptr(),
        response_buffer_len as r_size_t,
        websocket_response.as_ptr(),
        ::std::mem::size_of::<[libc::c_char; 98]>() as libc::c_ulong as r_size_t,
    );
    if tmp___9 != 0 as libc::c_int {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL;
    }
    tmp___10 = strncat_s_safe_c(
        response_buffer.as_mut_ptr(),
        response_buffer_len as r_size_t,
        buffer.as_mut_ptr() as *const libc::c_char,
        buffer_len as r_size_t,
    );
    if tmp___10 != 0 as libc::c_int {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL;
    }
    tmp___11 = strncat_s_safe_c(
        response_buffer.as_mut_ptr(),
        response_buffer_len as r_size_t,
        b"\r\n\r\n\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as r_size_t,
    );
    if tmp___11 != 0 as libc::c_int {
        return RMT_ERROR_WEBSOCKET_HANDSHAKE_STRING_FAIL;
    }
    tmp___12 = strnlen_s_safe_c(
        response_buffer.as_mut_ptr() as *const libc::c_char,
        response_buffer_len as r_size_t,
    );
    tmp___13 = TCPSocket_Send(
        tcp_socket,
        response_buffer.as_mut_ptr() as *const libc::c_void,
        tmp___12,
        1000 as libc::c_int as rmtU32,
    );
    return tmp___13;
}
unsafe extern "C" fn WebSocket_Constructor(
    mut web_socket: *mut WebSocket,
    mut tcp_socket: *mut TCPSocket,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    error = RMT_ERROR_NONE;
    if !(web_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3824 as libc::c_uint,
            b"WebSocket_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*web_socket).tcp_socket = tcp_socket;
    (*web_socket).mode = WEBSOCKET_NONE;
    (*web_socket).frame_bytes_remaining = 0 as libc::c_int as rmtU32;
    (*web_socket).mask_offset = 0 as libc::c_int as rmtU32;
    (*web_socket).data.mask[0 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*web_socket).data.mask[1 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*web_socket).data.mask[2 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*web_socket).data.mask[3 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    if (*web_socket).tcp_socket as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        tmp___0 = rmtMalloc(
            ::std::mem::size_of::<TCPSocket>() as libc::c_ulong as rmtU32,
        );
        (*web_socket).tcp_socket = tmp___0 as *mut TCPSocket;
        if (*web_socket).tcp_socket as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            return RMT_ERROR_MALLOC_FAIL;
        }
        tmp___1 = TCPSocket_Constructor((*web_socket).tcp_socket);
        error___0 = tmp___1;
        if error___0 as libc::c_uint != 0 as libc::c_uint {
            if (*web_socket).tcp_socket as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong
            {
                TCPSocket_Destructor((*web_socket).tcp_socket);
                rmtFree((*web_socket).tcp_socket as *mut libc::c_void);
                (*web_socket).tcp_socket = 0 as *mut libc::c_void as *mut TCPSocket;
            }
            return error___0;
        }
    }
    return error;
}
unsafe extern "C" fn WebSocket_Destructor(mut web_socket: *mut WebSocket) {
    WebSocket_Close(web_socket);
}
unsafe extern "C" fn WebSocket_RunServer(
    mut web_socket: *mut WebSocket,
    mut port: rmtU16,
    mut reuse_open_port: rmtBool,
    mut limit_connections_to_localhost: rmtBool,
    mut mode: WebSocketMode,
) -> rmtError {
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    if !(web_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3850 as libc::c_uint,
            b"WebSocket_RunServer\0" as *const u8 as *const libc::c_char,
        );
    }
    (*web_socket).mode = mode;
    tmp___0 = TCPSocket_RunServer(
        (*web_socket).tcp_socket,
        port,
        reuse_open_port,
        limit_connections_to_localhost,
    );
    return tmp___0;
}
unsafe extern "C" fn WebSocket_Close(mut web_socket: *mut WebSocket) {
    if !(web_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3857 as libc::c_uint,
            b"WebSocket_Close\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*web_socket).tcp_socket as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        TCPSocket_Destructor((*web_socket).tcp_socket);
        rmtFree((*web_socket).tcp_socket as *mut libc::c_void);
        (*web_socket).tcp_socket = 0 as *mut libc::c_void as *mut TCPSocket;
    }
}
unsafe extern "C" fn WebSocket_PollStatus(
    mut web_socket: *mut WebSocket,
) -> SocketStatus {
    let mut tmp___0: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    if !(web_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3863 as libc::c_uint,
            b"WebSocket_PollStatus\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = TCPSocket_PollStatus((*web_socket).tcp_socket);
    return tmp___0;
}
unsafe extern "C" fn WebSocket_AcceptConnection(
    mut web_socket: *mut WebSocket,
    mut client_socket: *mut *mut WebSocket,
) -> rmtError {
    let mut tcp_socket: *mut TCPSocket = 0 as *mut TCPSocket;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___4: rmtError = RMT_ERROR_NONE;
    tcp_socket = 0 as *mut libc::c_void as *mut TCPSocket;
    if !(web_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3872 as libc::c_uint,
            b"WebSocket_AcceptConnection\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = TCPSocket_AcceptConnection((*web_socket).tcp_socket, &mut tcp_socket);
    error = tmp___0;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    if tcp_socket as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_NONE;
    }
    tmp___1 = WebSocketHandshake(tcp_socket, 0 as *mut libc::c_void as rmtPStr);
    error___0 = tmp___1;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        return error___0;
    }
    if !(client_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"client_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3882 as libc::c_uint,
            b"WebSocket_AcceptConnection\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___3 = rmtMalloc(::std::mem::size_of::<WebSocket>() as libc::c_ulong as rmtU32);
    *client_socket = tmp___3 as *mut WebSocket;
    if *client_socket as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___4 = WebSocket_Constructor(*client_socket, tcp_socket);
    error___1 = tmp___4;
    if error___1 as libc::c_uint != 0 as libc::c_uint {
        if *client_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            WebSocket_Destructor(*client_socket);
            rmtFree(*client_socket as *mut libc::c_void);
            *client_socket = 0 as *mut libc::c_void as *mut WebSocket;
        }
        return error___1;
    }
    (**client_socket).mode = (*web_socket).mode;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn WriteSize(
    mut size: rmtU32,
    mut dest: *mut rmtU8,
    mut dest_size: rmtU32,
    mut dest_offset: rmtU32,
) {
    let mut size_size: libc::c_int = 0;
    let mut i: rmtU32 = 0;
    let mut j: libc::c_int = 0;
    size_size = dest_size.wrapping_sub(dest_offset) as libc::c_int;
    i = 0 as libc::c_int as rmtU32;
    while i < dest_size {
        j = i.wrapping_sub(dest_offset) as libc::c_int;
        if j < 0 as libc::c_int {
            *dest.offset(i as isize) = 0 as libc::c_int as rmtU8;
        } else {
            *dest
                .offset(
                    i as isize,
                ) = (size >> (size_size - j - 1 as libc::c_int) * 8 as libc::c_int
                & 255 as libc::c_uint) as rmtU8;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn WebSocket_PrepareBuffer(mut buffer: *mut Buffer) {
    let mut empty_frame_header: [libc::c_char; 10] = [0; 10];
    if !(buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3908 as libc::c_uint,
            b"WebSocket_PrepareBuffer\0" as *const u8 as *const libc::c_char,
        );
    }
    (*buffer).bytes_used = 0 as libc::c_int as rmtU32;
    Buffer_Write(
        buffer,
        empty_frame_header.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong as rmtU32,
    );
}
unsafe extern "C" fn WebSocket_FrameHeaderSize(mut length: rmtU32) -> rmtU32 {
    if length <= 125 as libc::c_uint {
        return 2 as libc::c_int as rmtU32;
    }
    if length <= 65535 as libc::c_uint {
        return 4 as libc::c_int as rmtU32;
    }
    return 10 as libc::c_int as rmtU32;
}
unsafe extern "C" fn WebSocket_WriteFrameHeader(
    mut web_socket: *mut WebSocket,
    mut dest: *mut rmtU8,
    mut length: rmtU32,
) {
    let mut final_fragment: rmtU8 = 0;
    let mut frame_type: rmtU8 = 0;
    final_fragment = ((1 as libc::c_int) << 7 as libc::c_int) as rmtU8;
    frame_type = (*web_socket).mode as rmtU8;
    *dest
        .offset(
            0 as libc::c_int as isize,
        ) = (final_fragment as libc::c_int | frame_type as libc::c_int) as rmtU8;
    if length <= 125 as libc::c_uint {
        *dest.offset(1 as libc::c_int as isize) = length as rmtU8;
    } else if length <= 65535 as libc::c_uint {
        *dest.offset(1 as libc::c_int as isize) = 126 as libc::c_int as rmtU8;
        WriteSize(
            length,
            dest.offset(2 as libc::c_int as isize),
            2 as libc::c_int as rmtU32,
            0 as libc::c_int as rmtU32,
        );
    } else {
        *dest.offset(1 as libc::c_int as isize) = 127 as libc::c_int as rmtU8;
        WriteSize(
            length,
            dest.offset(2 as libc::c_int as isize),
            8 as libc::c_int as rmtU32,
            4 as libc::c_int as rmtU32,
        );
    };
}
unsafe extern "C" fn WebSocket_Send(
    mut web_socket: *mut WebSocket,
    mut data: *const libc::c_void,
    mut length: rmtU32,
    mut timeout_ms: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut payload_length: rmtU32 = 0;
    let mut frame_header_size: rmtU32 = 0;
    let mut delta: rmtU32 = 0;
    if !(web_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3956 as libc::c_uint,
            b"WebSocket_Send\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(data as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"data != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3957 as libc::c_uint,
            b"WebSocket_Send\0" as *const u8 as *const libc::c_char,
        );
    }
    status = WebSocket_PollStatus(web_socket);
    if status.error_state as libc::c_uint != 0 as libc::c_uint {
        return status.error_state;
    }
    payload_length = length.wrapping_sub(10 as libc::c_uint);
    frame_header_size = WebSocket_FrameHeaderSize(payload_length);
    delta = (10 as libc::c_uint).wrapping_sub(frame_header_size);
    data = (data as *mut rmtU8).offset(delta as isize) as *mut libc::c_void
        as *const libc::c_void;
    length = (length as libc::c_uint).wrapping_sub(delta) as rmtU32 as rmtU32;
    WebSocket_WriteFrameHeader(web_socket, data as *mut rmtU8, payload_length);
    error = TCPSocket_Send((*web_socket).tcp_socket, data, length, timeout_ms);
    return error;
}
unsafe extern "C" fn ReceiveFrameHeader(mut web_socket: *mut WebSocket) -> rmtError {
    let mut msg_header: [rmtU8; 2] = [0; 2];
    let mut msg_length: libc::c_int = 0;
    let mut size_bytes_remaining: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mask_present: rmtBool = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut size_bytes: [rmtU8; 8] = [0; 8];
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut error___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    msg_header[0 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    msg_header[1 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    if !(web_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            3985 as libc::c_uint,
            b"ReceiveFrameHeader\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = TCPSocket_Receive(
        (*web_socket).tcp_socket,
        msg_header.as_mut_ptr() as *mut libc::c_void,
        2 as libc::c_int as rmtU32,
        20 as libc::c_int as rmtU32,
    );
    error = tmp___0;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    if msg_header[0 as libc::c_int as usize] as libc::c_int == 136 as libc::c_int {
        return RMT_ERROR_WEBSOCKET_DISCONNECTED;
    }
    if msg_header[0 as libc::c_int as usize] as libc::c_int != 129 as libc::c_int {
        if msg_header[0 as libc::c_int as usize] as libc::c_int != 130 as libc::c_int {
            return RMT_ERROR_WEBSOCKET_BAD_FRAME_HEADER;
        }
    }
    msg_length = msg_header[1 as libc::c_int as usize] as libc::c_int
        & 127 as libc::c_int;
    size_bytes_remaining = 0 as libc::c_int;
    match msg_length {
        126 => {
            size_bytes_remaining = 2 as libc::c_int;
        }
        127 => {
            size_bytes_remaining = 8 as libc::c_int;
        }
        _ => {}
    }
    if size_bytes_remaining > 0 as libc::c_int {
        tmp___1 = TCPSocket_Receive(
            (*web_socket).tcp_socket,
            size_bytes.as_mut_ptr() as *mut libc::c_void,
            size_bytes_remaining as rmtU32,
            20 as libc::c_int as rmtU32,
        );
        error___0 = tmp___1;
        if error___0 as libc::c_uint != 0 as libc::c_uint {
            return error___0;
        }
        msg_length = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < size_bytes_remaining {
            msg_length
                |= (size_bytes[i as usize] as libc::c_int)
                    << (size_bytes_remaining - 1 as libc::c_int - i) * 8 as libc::c_int;
            i += 1;
        }
    }
    if msg_header[1 as libc::c_int as usize] as libc::c_int & 128 as libc::c_int
        != 0 as libc::c_int
    {
        mask_present = 1 as libc::c_int as rmtBool;
    } else {
        mask_present = 0 as libc::c_int as rmtBool;
    }
    if mask_present != 0 {
        tmp___2 = TCPSocket_Receive(
            (*web_socket).tcp_socket,
            ((*web_socket).data.mask).as_mut_ptr() as *mut libc::c_void,
            4 as libc::c_int as rmtU32,
            20 as libc::c_int as rmtU32,
        );
        error___1 = tmp___2;
        if error___1 as libc::c_uint != 0 as libc::c_uint {
            return error___1;
        }
    }
    (*web_socket).frame_bytes_remaining = msg_length as rmtU32;
    (*web_socket).mask_offset = 0 as libc::c_int as rmtU32;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn WebSocket_Receive(
    mut web_socket: *mut WebSocket,
    mut data: *mut libc::c_void,
    mut msg_len: *mut rmtU32,
    mut length: rmtU32,
    mut timeout_ms: rmtU32,
) -> rmtError {
    let mut status: SocketStatus = SocketStatus {
        can_read: 0,
        can_write: 0,
        error_state: RMT_ERROR_NONE,
    };
    let mut cur_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end_data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start_ms: rmtU32 = 0;
    let mut now_ms: rmtU32 = 0;
    let mut bytes_to_read: rmtU32 = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut i: rmtU32 = 0;
    if !(web_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"web_socket != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4045 as libc::c_uint,
            b"WebSocket_Receive\0" as *const u8 as *const libc::c_char,
        );
    }
    status = WebSocket_PollStatus(web_socket);
    if status.error_state as libc::c_uint != 0 as libc::c_uint {
        return status.error_state;
    }
    cur_data = data as *mut libc::c_char;
    end_data = cur_data.offset(length as isize);
    start_ms = msTimer_Get();
    while (cur_data as libc::c_ulong) < end_data as libc::c_ulong {
        if (*web_socket).frame_bytes_remaining == 0 as libc::c_uint {
            tmp___0 = ReceiveFrameHeader(web_socket);
            error___0 = tmp___0;
            if error___0 as libc::c_uint != 0 as libc::c_uint {
                return error___0;
            }
            if msg_len as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                *msg_len = (*web_socket).frame_bytes_remaining;
            }
        }
        if (*web_socket).frame_bytes_remaining < length {
            bytes_to_read = (*web_socket).frame_bytes_remaining;
        } else {
            bytes_to_read = length;
        }
        error = TCPSocket_Receive(
            (*web_socket).tcp_socket,
            cur_data as *mut libc::c_void,
            bytes_to_read,
            20 as libc::c_int as rmtU32,
        );
        if error as libc::c_uint == 18 as libc::c_uint {
            return error;
        }
        if !(error as libc::c_uint == 16 as libc::c_uint) {
            if !(error as libc::c_uint == 17 as libc::c_uint) {
                if (*web_socket).data.mask_u32 != 0 as libc::c_uint {
                    i = 0 as libc::c_int as rmtU32;
                    while i < bytes_to_read {
                        *(cur_data as *mut rmtU8)
                            .offset(
                                i as isize,
                            ) = (*(cur_data as *mut rmtU8).offset(i as isize)
                            as libc::c_int
                            ^ (*web_socket)
                                .data
                                .mask[((*web_socket).mask_offset & 3 as libc::c_uint)
                                as usize] as libc::c_int) as rmtU8;
                        (*web_socket)
                            .mask_offset = ((*web_socket).mask_offset).wrapping_add(1);
                        i = i.wrapping_add(1);
                    }
                }
                cur_data = cur_data.offset(bytes_to_read as isize);
                (*web_socket)
                    .frame_bytes_remaining = ((*web_socket).frame_bytes_remaining
                    as libc::c_uint)
                    .wrapping_sub(bytes_to_read) as rmtU32 as rmtU32;
                continue;
            }
        }
        now_ms = msTimer_Get();
        if now_ms.wrapping_sub(start_ms) > timeout_ms {
            return RMT_ERROR_SOCKET_RECV_TIMEOUT;
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtMessageQueue_Constructor(
    mut queue: *mut rmtMessageQueue,
    mut size: rmtU32,
) -> rmtError {
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    if !(queue as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4153 as libc::c_uint,
            b"rmtMessageQueue_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*queue).size = 0 as libc::c_int as rmtU32;
    (*queue).data = 0 as *mut libc::c_void as *mut VirtualMirrorBuffer;
    (*queue).read_pos = 0 as libc::c_int as rmtU32;
    (*queue).write_pos = 0 as libc::c_int as rmtU32;
    tmp___0 = rmtMalloc(
        ::std::mem::size_of::<VirtualMirrorBuffer>() as libc::c_ulong as rmtU32,
    );
    (*queue).data = tmp___0 as *mut VirtualMirrorBuffer;
    if (*queue).data as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___1 = VirtualMirrorBuffer_Constructor((*queue).data, size, 10 as libc::c_int);
    error = tmp___1;
    if error as libc::c_uint != 0 as libc::c_uint {
        if (*queue).data as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            VirtualMirrorBuffer_Destructor((*queue).data);
            rmtFree((*queue).data as *mut libc::c_void);
            (*queue).data = 0 as *mut libc::c_void as *mut VirtualMirrorBuffer;
        }
        return error;
    }
    (*queue).size = (*(*queue).data).size;
    memset(
        (*(*queue).data).ptr as *mut libc::c_void,
        0 as libc::c_int,
        (*queue).size as size_t,
    );
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn rmtMessageQueue_Destructor(mut queue: *mut rmtMessageQueue) {
    if !(queue as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4175 as libc::c_uint,
            b"rmtMessageQueue_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*queue).data as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        VirtualMirrorBuffer_Destructor((*queue).data);
        rmtFree((*queue).data as *mut libc::c_void);
        (*queue).data = 0 as *mut libc::c_void as *mut VirtualMirrorBuffer;
    }
}
unsafe extern "C" fn rmtMessageQueue_SizeForPayload(mut payload_size: rmtU32) -> rmtU32 {
    let mut size: rmtU32 = 0;
    size = (::std::mem::size_of::<Message>() as libc::c_ulong)
        .wrapping_add(payload_size as libc::c_ulong) as rmtU32;
    size = size.wrapping_add(3 as libc::c_uint) & 4294967292 as libc::c_uint;
    return size;
}
unsafe extern "C" fn rmtMessageQueue_AllocMessage(
    mut queue: *mut rmtMessageQueue,
    mut payload_size: rmtU32,
    mut thread_profiler: *mut ThreadProfiler,
) -> *mut Message {
    let mut msg: *mut Message = 0 as *mut Message;
    let mut write_size: rmtU32 = 0;
    let mut tmp: rmtU32 = 0;
    let mut s: rmtU32 = 0;
    let mut w: rmtU32 = 0;
    let mut tmp___1: rmtU32 = 0;
    let mut r: rmtU32 = 0;
    let mut tmp___2: rmtU32 = 0;
    let mut tmp___3: rmtBool = 0;
    tmp = rmtMessageQueue_SizeForPayload(payload_size);
    write_size = tmp;
    if !(queue as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4198 as libc::c_uint,
            b"rmtMessageQueue_AllocMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        s = (*queue).size;
        tmp___1 = LoadAcquire(&mut (*queue).write_pos);
        w = tmp___1;
        tmp___2 = LoadAcquire(&mut (*queue).read_pos);
        r = tmp___2;
        if w.wrapping_sub(r) as libc::c_int > s.wrapping_sub(write_size) as libc::c_int {
            return 0 as *mut libc::c_void as *mut Message;
        }
        msg = ((*(*queue).data).ptr)
            .offset((w & s.wrapping_sub(1 as libc::c_uint)) as isize) as *mut Message;
        tmp___3 = AtomicCompareAndSwapU32(
            &mut (*queue).write_pos,
            w as libc::c_long,
            w.wrapping_add(write_size) as libc::c_long,
        );
        if !(tmp___3 == 1 as libc::c_uint) {
            continue;
        }
        (*msg).payload_size = payload_size;
        (*msg).threadProfiler = thread_profiler;
        break;
    }
    return msg;
}
unsafe extern "C" fn rmtMessageQueue_CommitMessage(
    mut message: *mut Message,
    mut id: MessageID,
) {
    let mut tmp___3: rmtU32 = 0;
    if !(message as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"message != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4229 as libc::c_uint,
            b"rmtMessageQueue_CommitMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___3 = LoadAcquire(
        &mut (*message).id as *mut MessageID as *mut rmtU32 as *mut rmtAtomicU32,
    );
    if !(tmp___3 == 0 as libc::c_uint) {
        __assert_fail(
            b"LoadAcquire((rmtU32*)&message->id) == MsgID_NotReady\0" as *const u8
                as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4232 as libc::c_uint,
            b"rmtMessageQueue_CommitMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    StoreRelease(
        &mut (*message).id as *mut MessageID as *mut rmtU32 as *mut rmtAtomicU32,
        id as rmtU32,
    );
}
pub unsafe extern "C" fn rmtMessageQueue_PeekNextMessage(
    mut queue: *mut rmtMessageQueue,
) -> *mut Message {
    let mut ptr: *mut Message = 0 as *mut Message;
    let mut r: rmtU32 = 0;
    let mut w: rmtU32 = 0;
    let mut id: MessageID = MsgID_NotReady;
    let mut tmp___0: rmtU32 = 0;
    if !(queue as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4242 as libc::c_uint,
            b"rmtMessageQueue_PeekNextMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    w = LoadAcquire(&mut (*queue).write_pos);
    r = (*queue).read_pos;
    if w.wrapping_sub(r) == 0 as libc::c_uint {
        return 0 as *mut libc::c_void as *mut Message;
    }
    r &= ((*queue).size).wrapping_sub(1 as libc::c_uint);
    ptr = ((*(*queue).data).ptr).offset(r as isize) as *mut Message;
    tmp___0 = LoadAcquire(
        &mut (*ptr).id as *mut MessageID as *mut rmtU32 as *mut rmtAtomicU32,
    );
    id = tmp___0 as MessageID;
    if id as libc::c_uint != 0 as libc::c_uint {
        return ptr;
    }
    return 0 as *mut libc::c_void as *mut Message;
}
unsafe extern "C" fn rmtMessageQueue_ConsumeNextMessage(
    mut queue: *mut rmtMessageQueue,
    mut message: *mut Message,
) {
    let mut message_size: rmtU32 = 0;
    let mut read_pos: rmtU32 = 0;
    if !(queue as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4266 as libc::c_uint,
            b"rmtMessageQueue_ConsumeNextMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(message as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"message != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4267 as libc::c_uint,
            b"rmtMessageQueue_ConsumeNextMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    message_size = rmtMessageQueue_SizeForPayload((*message).payload_size);
    memset(message as *mut libc::c_void, 0 as libc::c_int, message_size as size_t);
    read_pos = ((*queue).read_pos).wrapping_add(message_size);
    StoreRelease(&mut (*queue).read_pos, read_pos);
}
unsafe extern "C" fn Server_CreateListenSocket(
    mut server: *mut Server,
    mut port: rmtU16,
    mut reuse_open_port: rmtBool,
    mut limit_connections_to_localhost: rmtBool,
) -> rmtError {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    tmp = rmtMalloc(::std::mem::size_of::<WebSocket>() as libc::c_ulong as rmtU32);
    (*server).listen_socket = tmp as *mut WebSocket;
    if (*server).listen_socket as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___0 = WebSocket_Constructor(
        (*server).listen_socket,
        0 as *mut libc::c_void as *mut TCPSocket,
    );
    error = tmp___0;
    if error as libc::c_uint != 0 as libc::c_uint {
        if (*server).listen_socket as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            WebSocket_Destructor((*server).listen_socket);
            rmtFree((*server).listen_socket as *mut libc::c_void);
            (*server).listen_socket = 0 as *mut libc::c_void as *mut WebSocket;
        }
        return error;
    }
    tmp___1 = WebSocket_RunServer(
        (*server).listen_socket,
        port,
        reuse_open_port,
        limit_connections_to_localhost,
        WEBSOCKET_BINARY,
    );
    error___0 = tmp___1;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        return error___0;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Server_Constructor(
    mut server: *mut Server,
    mut port: rmtU16,
    mut reuse_open_port: rmtBool,
    mut limit_connections_to_localhost: rmtBool,
) -> rmtError {
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    if !(server as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4328 as libc::c_uint,
            b"Server_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*server).listen_socket = 0 as *mut libc::c_void as *mut WebSocket;
    (*server).client_socket = 0 as *mut libc::c_void as *mut WebSocket;
    (*server).last_ping_time = 0 as libc::c_int as rmtU32;
    (*server).port = port;
    (*server).reuse_open_port = reuse_open_port;
    (*server).limit_connections_to_localhost = limit_connections_to_localhost;
    (*server).bin_buf = 0 as *mut libc::c_void as *mut Buffer;
    (*server)
        .receive_handler = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_char,
                rmtU32,
            ) -> rmtError,
        >,
    >(0 as *mut libc::c_void);
    (*server).receive_handler_context = 0 as *mut libc::c_void;
    tmp___0 = rmtMalloc(::std::mem::size_of::<Buffer>() as libc::c_ulong as rmtU32);
    (*server).bin_buf = tmp___0 as *mut Buffer;
    if (*server).bin_buf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___1 = Buffer_Constructor((*server).bin_buf, 4096 as libc::c_int as rmtU32);
    error = tmp___1;
    if error as libc::c_uint != 0 as libc::c_uint {
        if (*server).bin_buf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
        {
            Buffer_Destructor((*server).bin_buf);
            rmtFree((*server).bin_buf as *mut libc::c_void);
            (*server).bin_buf = 0 as *mut libc::c_void as *mut Buffer;
        }
        return error;
    }
    tmp___2 = Server_CreateListenSocket(
        server,
        port,
        reuse_open_port,
        limit_connections_to_localhost,
    );
    return tmp___2;
}
unsafe extern "C" fn Server_Destructor(mut server: *mut Server) {
    if !(server as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4348 as libc::c_uint,
            b"Server_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*server).client_socket as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        WebSocket_Destructor((*server).client_socket);
        rmtFree((*server).client_socket as *mut libc::c_void);
        (*server).client_socket = 0 as *mut libc::c_void as *mut WebSocket;
    }
    if (*server).listen_socket as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        WebSocket_Destructor((*server).listen_socket);
        rmtFree((*server).listen_socket as *mut libc::c_void);
        (*server).listen_socket = 0 as *mut libc::c_void as *mut WebSocket;
    }
    if (*server).bin_buf as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        Buffer_Destructor((*server).bin_buf);
        rmtFree((*server).bin_buf as *mut libc::c_void);
        (*server).bin_buf = 0 as *mut libc::c_void as *mut Buffer;
    }
}
unsafe extern "C" fn Server_IsClientConnected(mut server: *mut Server) -> rmtBool {
    let mut tmp___0: rmtBool = 0;
    if !(server as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4356 as libc::c_uint,
            b"Server_IsClientConnected\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*server).client_socket as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        tmp___0 = 1 as libc::c_int as rmtBool;
    } else {
        tmp___0 = 0 as libc::c_int as rmtBool;
    }
    return tmp___0;
}
unsafe extern "C" fn Server_DisconnectClient(mut server: *mut Server) {
    let mut client_socket: *mut WebSocket = 0 as *mut WebSocket;
    if !(server as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4364 as libc::c_uint,
            b"Server_DisconnectClient\0" as *const u8 as *const libc::c_char,
        );
    }
    client_socket = (*server).client_socket;
    (*server).client_socket = 0 as *mut libc::c_void as *mut WebSocket;
    CompilerWriteFence();
    if client_socket as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        WebSocket_Destructor(client_socket);
        rmtFree(client_socket as *mut libc::c_void);
        client_socket = 0 as *mut libc::c_void as *mut WebSocket;
    }
}
unsafe extern "C" fn Server_Send(
    mut server: *mut Server,
    mut data: *const libc::c_void,
    mut length: rmtU32,
    mut timeout: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtBool = 0;
    if !(server as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4375 as libc::c_uint,
            b"Server_Send\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___1 = Server_IsClientConnected(server);
    if tmp___1 != 0 {
        tmp___0 = WebSocket_Send((*server).client_socket, data, length, timeout);
        error = tmp___0;
        if error as libc::c_uint == 15 as libc::c_uint {
            Server_DisconnectClient(server);
        }
        return error;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Server_ReceiveMessage(
    mut server: *mut Server,
    mut message_first_byte: libc::c_char,
    mut message_length: rmtU32,
) -> rmtError {
    let mut message_data: [libc::c_char; 1024] = [0; 1024];
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    if message_length as libc::c_ulong
        >= (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_ulong)
    {
        _rmt_LogText(
            b"Ignoring console input bigger than internal receive buffer (1024 bytes)\0"
                as *const u8 as *const libc::c_char,
        );
        return RMT_ERROR_NONE;
    }
    message_data[0 as libc::c_int as usize] = message_first_byte;
    tmp = WebSocket_Receive(
        (*server).client_socket,
        message_data.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
        0 as *mut libc::c_void as *mut rmtU32,
        message_length.wrapping_sub(1 as libc::c_uint),
        100 as libc::c_int as rmtU32,
    );
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    message_data[message_length as usize] = 0 as libc::c_int as libc::c_char;
    if message_length < 4 as libc::c_uint {
        return RMT_ERROR_NONE;
    }
    if ((*server).receive_handler).is_some() {
        tmp___0 = (Some(((*server).receive_handler).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(
            (*server).receive_handler_context,
            message_data.as_mut_ptr(),
            message_length,
        );
        error___0 = tmp___0;
        if error___0 as libc::c_uint != 0 as libc::c_uint {
            return error___0;
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn bin_MessageHeader(
    mut buffer: *mut Buffer,
    mut id: *const libc::c_char,
    mut out_write_start_offset: *mut rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    *out_write_start_offset = (*buffer).bytes_used;
    tmp = Buffer_Write(
        buffer,
        id as *mut libc::c_void as *const libc::c_void,
        4 as libc::c_int as rmtU32,
    );
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    tmp___0 = Buffer_Write(
        buffer,
        b"    \0" as *const u8 as *const libc::c_char as *mut libc::c_void
            as *const libc::c_void,
        4 as libc::c_int as rmtU32,
    );
    error___0 = tmp___0;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        return error___0;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn bin_MessageFooter(
    mut buffer: *mut Buffer,
    mut write_start_offset: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    tmp = Buffer_AlignedPad(buffer, write_start_offset);
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    U32ToByteArray(
        ((*buffer).data)
            .offset(write_start_offset as isize)
            .offset(4 as libc::c_int as isize),
        ((*buffer).bytes_used).wrapping_sub(write_start_offset),
    );
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Server_Update(mut server: *mut Server) {
    let mut cur_time: rmtU32 = 0;
    let mut client_socket: *mut WebSocket = 0 as *mut WebSocket;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut message_first_byte: libc::c_char = 0;
    let mut message_length: rmtU32 = 0;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    let mut write_start_offset: rmtU32 = 0;
    if !(server as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"server != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4439 as libc::c_uint,
            b"Server_Update\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*server).listen_socket as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        Server_CreateListenSocket(
            server,
            (*server).port,
            (*server).reuse_open_port,
            (*server).limit_connections_to_localhost,
        );
    }
    let mut current_block_24: u64;
    if (*server).listen_socket as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        if (*server).client_socket as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            client_socket = 0 as *mut libc::c_void as *mut WebSocket;
            tmp___0 = WebSocket_AcceptConnection(
                (*server).listen_socket,
                &mut client_socket,
            );
            error = tmp___0;
            if error as libc::c_uint == 0 as libc::c_uint {
                (*server).client_socket = client_socket;
            } else if (*server).listen_socket as libc::c_ulong
                    != 0 as *mut libc::c_void as libc::c_ulong
                {
                WebSocket_Destructor((*server).listen_socket);
                rmtFree((*server).listen_socket as *mut libc::c_void);
                (*server).listen_socket = 0 as *mut libc::c_void as *mut WebSocket;
            }
            current_block_24 = 11913429853522160501;
        } else {
            current_block_24 = 7569723131427624693;
        }
    } else {
        current_block_24 = 7569723131427624693;
    }
    match current_block_24 {
        7569723131427624693 => {
            loop {
                tmp___1 = WebSocket_Receive(
                    (*server).client_socket,
                    &mut message_first_byte as *mut libc::c_char as *mut libc::c_void,
                    &mut message_length,
                    1 as libc::c_int as rmtU32,
                    0 as libc::c_int as rmtU32,
                );
                error___0 = tmp___1;
                if error___0 as libc::c_uint == 0 as libc::c_uint {
                    error___0 = Server_ReceiveMessage(
                        server,
                        message_first_byte,
                        message_length,
                    );
                    if error___0 as libc::c_uint != 0 as libc::c_uint {
                        Server_DisconnectClient(server);
                        break;
                    }
                } else {
                    if error___0 as libc::c_uint == 16 as libc::c_uint {
                        break;
                    }
                    if error___0 as libc::c_uint == 17 as libc::c_uint {
                        break;
                    }
                    Server_DisconnectClient(server);
                    break;
                }
            }
        }
        _ => {}
    }
    cur_time = msTimer_Get();
    if cur_time.wrapping_sub((*server).last_ping_time) > 1000 as libc::c_uint {
        bin_buf = (*server).bin_buf;
        WebSocket_PrepareBuffer(bin_buf);
        bin_MessageHeader(
            bin_buf,
            b"PING\0" as *const u8 as *const libc::c_char,
            &mut write_start_offset,
        );
        bin_MessageFooter(bin_buf, write_start_offset);
        Server_Send(
            server,
            (*bin_buf).data as *const libc::c_void,
            (*bin_buf).bytes_used,
            10 as libc::c_int as rmtU32,
        );
        (*server).last_ping_time = cur_time;
    }
}
unsafe extern "C" fn Sample_Constructor(mut sample: *mut Sample) -> rmtError {
    if !(sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4577 as libc::c_uint,
            b"Sample_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    ObjectLink_Constructor(sample as *mut ObjectLink);
    (*sample).type_0 = RMT_SampleType_CPU;
    (*sample).name_hash = 0 as libc::c_int as rmtU32;
    (*sample).unique_id = 0 as libc::c_int as rmtU32;
    (*sample).uniqueColour[0 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*sample).uniqueColour[1 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*sample).uniqueColour[2 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    (*sample).parent = 0 as *mut libc::c_void as *mut Sample;
    (*sample).first_child = 0 as *mut libc::c_void as *mut Sample;
    (*sample).last_child = 0 as *mut libc::c_void as *mut Sample;
    (*sample).next_sibling = 0 as *mut libc::c_void as *mut Sample;
    (*sample).nb_children = 0 as libc::c_int as rmtU32;
    (*sample).us_start = 0 as libc::c_int as rmtU64;
    (*sample).us_end = 0 as libc::c_int as rmtU64;
    (*sample).us_length = 0 as libc::c_int as rmtU64;
    (*sample).us_sampled_length = 0 as libc::c_int as rmtU64;
    (*sample).usGpuIssueOnCpu = 0 as libc::c_int as rmtU64;
    (*sample).call_count = 0 as libc::c_int as rmtU32;
    (*sample).recurse_depth = 0 as libc::c_int as rmtU16;
    (*sample).max_recurse_depth = 0 as libc::c_int as rmtU16;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Sample_Destructor(mut sample: *mut Sample) {}
unsafe extern "C" fn Sample_Prepare(
    mut sample: *mut Sample,
    mut name_hash: rmtU32,
    mut parent: *mut Sample,
) {
    (*sample).name_hash = name_hash;
    (*sample).unique_id = 0 as libc::c_int as rmtU32;
    (*sample).parent = parent;
    (*sample).first_child = 0 as *mut libc::c_void as *mut Sample;
    (*sample).last_child = 0 as *mut libc::c_void as *mut Sample;
    (*sample).next_sibling = 0 as *mut libc::c_void as *mut Sample;
    (*sample).nb_children = 0 as libc::c_int as rmtU32;
    (*sample).us_start = 0 as libc::c_int as rmtU64;
    (*sample).us_end = 0 as libc::c_int as rmtU64;
    (*sample).us_length = 0 as libc::c_int as rmtU64;
    (*sample).us_sampled_length = 0 as libc::c_int as rmtU64;
    (*sample).usGpuIssueOnCpu = 0 as libc::c_int as rmtU64;
    (*sample).call_count = 1 as libc::c_int as rmtU32;
    (*sample).recurse_depth = 0 as libc::c_int as rmtU16;
    (*sample).max_recurse_depth = 0 as libc::c_int as rmtU16;
}
unsafe extern "C" fn Sample_Close(mut sample: *mut Sample, mut us_end: rmtS64) {
    let mut us_length: rmtS64 = 0;
    us_length = 0 as libc::c_int as rmtS64;
    if (*sample).call_count > 1 as libc::c_uint {
        if (*sample).max_recurse_depth as libc::c_int == 0 as libc::c_int {
            us_length = maxS64(
                (us_end as rmtU64).wrapping_sub((*sample).us_end) as rmtS64,
                0 as libc::c_int as rmtS64,
            );
        } else {
            us_length = maxS64(
                (us_end as rmtU64).wrapping_sub((*sample).us_start) as rmtS64,
                0 as libc::c_int as rmtS64,
            );
        }
    } else {
        us_length = maxS64(
            (us_end as rmtU64).wrapping_sub((*sample).us_start) as rmtS64,
            0 as libc::c_int as rmtS64,
        );
    }
    (*sample)
        .us_length = ((*sample).us_length as libc::c_ulonglong)
        .wrapping_add(us_length as rmtU64) as rmtU64 as rmtU64;
    if (*sample).parent as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*(*sample).parent)
            .us_sampled_length = ((*(*sample).parent).us_sampled_length
            as libc::c_ulonglong)
            .wrapping_add(us_length as rmtU64) as rmtU64 as rmtU64;
    }
}
unsafe extern "C" fn Sample_CopyState(
    mut dst_sample: *mut Sample,
    mut src_sample: *const Sample,
) {
    (*dst_sample).type_0 = (*src_sample).type_0;
    (*dst_sample).name_hash = (*src_sample).name_hash;
    (*dst_sample).unique_id = (*src_sample).unique_id;
    (*dst_sample).nb_children = (*src_sample).nb_children;
    (*dst_sample).us_start = (*src_sample).us_start;
    (*dst_sample).us_end = (*src_sample).us_end;
    (*dst_sample).us_length = (*src_sample).us_length;
    (*dst_sample).us_sampled_length = (*src_sample).us_sampled_length;
    (*dst_sample).usGpuIssueOnCpu = (*src_sample).usGpuIssueOnCpu;
    (*dst_sample).call_count = (*src_sample).call_count;
    (*dst_sample).recurse_depth = (*src_sample).recurse_depth;
    (*dst_sample).max_recurse_depth = (*src_sample).max_recurse_depth;
    (*dst_sample).parent = 0 as *mut libc::c_void as *mut Sample;
    (*dst_sample).first_child = 0 as *mut libc::c_void as *mut Sample;
    (*dst_sample).last_child = 0 as *mut libc::c_void as *mut Sample;
    (*dst_sample).next_sibling = 0 as *mut libc::c_void as *mut Sample;
}
unsafe extern "C" fn bin_Sample(
    mut buffer: *mut Buffer,
    mut sample: *mut Sample,
    mut depth: rmtU8,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut error___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    let mut error___2: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: rmtError = RMT_ERROR_NONE;
    let mut error___3: rmtError = RMT_ERROR_NONE;
    let mut tmp___4: rmtError = RMT_ERROR_NONE;
    let mut error___4: rmtError = RMT_ERROR_NONE;
    let mut tmp___5: rmtError = RMT_ERROR_NONE;
    let mut error___5: rmtError = RMT_ERROR_NONE;
    let mut tmp___6: rmtS64 = 0;
    let mut tmp___7: rmtError = RMT_ERROR_NONE;
    let mut error___6: rmtError = RMT_ERROR_NONE;
    let mut tmp___8: rmtError = RMT_ERROR_NONE;
    let mut error___7: rmtError = RMT_ERROR_NONE;
    let mut tmp___9: rmtError = RMT_ERROR_NONE;
    let mut error___8: rmtError = RMT_ERROR_NONE;
    let mut tmp___10: rmtError = RMT_ERROR_NONE;
    let mut error___9: rmtError = RMT_ERROR_NONE;
    let mut tmp___11: rmtError = RMT_ERROR_NONE;
    if !(sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4678 as libc::c_uint,
            b"bin_Sample\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___0 = Buffer_WriteU32(buffer, (*sample).name_hash);
    error = tmp___0;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    tmp___1 = Buffer_WriteU32(buffer, (*sample).unique_id);
    error___0 = tmp___1;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        return error___0;
    }
    tmp___2 = Buffer_Write(
        buffer,
        ((*sample).uniqueColour).as_mut_ptr() as *const libc::c_void,
        3 as libc::c_int as rmtU32,
    );
    error___1 = tmp___2;
    if error___1 as libc::c_uint != 0 as libc::c_uint {
        return error___1;
    }
    tmp___3 = Buffer_Write(
        buffer,
        &mut depth as *mut rmtU8 as *const libc::c_void,
        1 as libc::c_int as rmtU32,
    );
    error___2 = tmp___3;
    if error___2 as libc::c_uint != 0 as libc::c_uint {
        return error___2;
    }
    tmp___4 = Buffer_WriteU64(buffer, (*sample).us_start);
    error___3 = tmp___4;
    if error___3 as libc::c_uint != 0 as libc::c_uint {
        return error___3;
    }
    tmp___5 = Buffer_WriteU64(buffer, (*sample).us_length);
    error___4 = tmp___5;
    if error___4 as libc::c_uint != 0 as libc::c_uint {
        return error___4;
    }
    tmp___6 = maxS64(
        ((*sample).us_length).wrapping_sub((*sample).us_sampled_length) as rmtS64,
        0 as libc::c_int as rmtS64,
    );
    tmp___7 = Buffer_WriteU64(buffer, tmp___6 as rmtU64);
    error___5 = tmp___7;
    if error___5 as libc::c_uint != 0 as libc::c_uint {
        return error___5;
    }
    tmp___8 = Buffer_WriteU64(buffer, (*sample).usGpuIssueOnCpu);
    error___6 = tmp___8;
    if error___6 as libc::c_uint != 0 as libc::c_uint {
        return error___6;
    }
    tmp___9 = Buffer_WriteU32(buffer, (*sample).call_count);
    error___7 = tmp___9;
    if error___7 as libc::c_uint != 0 as libc::c_uint {
        return error___7;
    }
    tmp___10 = Buffer_WriteU32(buffer, (*sample).max_recurse_depth as rmtU32);
    error___8 = tmp___10;
    if error___8 as libc::c_uint != 0 as libc::c_uint {
        return error___8;
    }
    tmp___11 = bin_SampleArray(
        buffer,
        sample,
        (depth as libc::c_int + 1 as libc::c_int) as rmtU8,
    );
    error___9 = tmp___11;
    if error___9 as libc::c_uint != 0 as libc::c_uint {
        return error___9;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn bin_SampleArray(
    mut buffer: *mut Buffer,
    mut parent_sample: *mut Sample,
    mut depth: rmtU8,
) -> rmtError {
    let mut sample: *mut Sample = 0 as *mut Sample;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    tmp = Buffer_WriteU32(buffer, (*parent_sample).nb_children);
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    sample = (*parent_sample).first_child;
    while sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = bin_Sample(buffer, sample, depth);
        error___0 = tmp___0;
        if error___0 as libc::c_uint != 0 as libc::c_uint {
            return error___0;
        }
        sample = (*sample).next_sibling;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn SampleTree_Constructor(
    mut tree: *mut SampleTree,
    mut sample_size: rmtU32,
    mut constructor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> rmtError>,
    mut destructor: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) -> rmtError {
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    if !(tree as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4745 as libc::c_uint,
            b"SampleTree_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*tree).allocator = 0 as *mut libc::c_void as *mut ObjectAllocator;
    (*tree).root = 0 as *mut libc::c_void as *mut Sample;
    (*tree).currentParent = 0 as *mut libc::c_void as *mut Sample;
    StoreRelease(&mut (*tree).msLastTreeSendTime, 0 as libc::c_int as rmtU32);
    StoreRelease(&mut (*tree).treeBeingModified, 0 as libc::c_int as rmtU32);
    (*tree).sendSampleOnClose = 0 as *mut libc::c_void as *mut Sample;
    tmp___0 = rmtMalloc(
        ::std::mem::size_of::<ObjectAllocator>() as libc::c_ulong as rmtU32,
    );
    (*tree).allocator = tmp___0 as *mut ObjectAllocator;
    if (*tree).allocator as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___1 = ObjectAllocator_Constructor(
        (*tree).allocator,
        sample_size,
        constructor,
        destructor,
    );
    error = tmp___1;
    if error as libc::c_uint != 0 as libc::c_uint {
        if (*tree).allocator as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
        {
            ObjectAllocator_Destructor((*tree).allocator);
            rmtFree((*tree).allocator as *mut libc::c_void);
            (*tree).allocator = 0 as *mut libc::c_void as *mut ObjectAllocator;
        }
        return error;
    }
    tmp___2 = ObjectAllocator_Alloc(
        (*tree).allocator,
        &mut (*tree).root as *mut *mut Sample as *mut *mut libc::c_void,
    );
    error___0 = tmp___2;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        return error___0;
    }
    Sample_Prepare(
        (*tree).root,
        0 as libc::c_int as rmtU32,
        0 as *mut libc::c_void as *mut Sample,
    );
    (*tree).currentParent = (*tree).root;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn SampleTree_Destructor(mut tree: *mut SampleTree) {
    if !(tree as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4767 as libc::c_uint,
            b"SampleTree_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*tree).root as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        ObjectAllocator_Free((*tree).allocator, (*tree).root as *mut libc::c_void);
        (*tree).root = 0 as *mut libc::c_void as *mut Sample;
    }
    if (*tree).allocator as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        ObjectAllocator_Destructor((*tree).allocator);
        rmtFree((*tree).allocator as *mut libc::c_void);
        (*tree).allocator = 0 as *mut libc::c_void as *mut ObjectAllocator;
    }
}
static mut random_bits: rmtU32 = 2654435769 as libc::c_uint;
unsafe extern "C" fn HashCombine(mut hash_a: rmtU32, mut hash_b: rmtU32) -> rmtU32 {
    hash_a
        ^= hash_b
            .wrapping_add(random_bits)
            .wrapping_add(hash_a << 6 as libc::c_int)
            .wrapping_add(hash_a >> 2 as libc::c_int);
    return hash_a;
}
unsafe extern "C" fn SampleTree_Push(
    mut tree: *mut SampleTree,
    mut name_hash: rmtU32,
    mut flags: rmtU32,
    mut sample: *mut *mut Sample,
) -> rmtError {
    let mut parent: *mut Sample = 0 as *mut Sample;
    let mut unique_id: rmtU32 = 0;
    let mut sibling: *mut Sample = 0 as *mut Sample;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___5: rmtError = RMT_ERROR_NONE;
    if !(tree as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4795 as libc::c_uint,
            b"SampleTree_Push\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((*tree).currentParent as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong)
    {
        __assert_fail(
            b"tree->currentParent != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4796 as libc::c_uint,
            b"SampleTree_Push\0" as *const u8 as *const libc::c_char,
        );
    }
    parent = (*tree).currentParent;
    if flags != 0 as libc::c_uint {
        if flags & 4 as libc::c_uint != 0 as libc::c_uint {
            if !((*parent).parent as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong)
            {
                __assert_fail(
                    b"parent->parent == NULL\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    4805 as libc::c_uint,
                    b"SampleTree_Push\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if flags & 1 as libc::c_uint != 0 as libc::c_uint {
            sibling = (*parent).first_child;
            while sibling as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                if (*sibling).name_hash == name_hash {
                    (*tree).currentParent = sibling;
                    (*sibling).call_count = ((*sibling).call_count).wrapping_add(1);
                    *sample = sibling;
                    return RMT_ERROR_NONE;
                }
                sibling = (*sibling).next_sibling;
            }
        }
        if flags & 2 as libc::c_uint != 0 as libc::c_uint {
            if (*parent).name_hash == name_hash {
                (*parent)
                    .recurse_depth = ((*parent).recurse_depth as libc::c_int
                    + 1 as libc::c_int) as rmtU16;
                (*parent)
                    .max_recurse_depth = maxU16(
                    (*parent).max_recurse_depth,
                    (*parent).recurse_depth,
                );
                (*parent).call_count = ((*parent).call_count).wrapping_add(1);
                *sample = parent;
                return RMT_ERROR_RECURSIVE_SAMPLE;
            }
        }
        tmp___2 = ObjectAllocator_Alloc(
            (*tree).allocator,
            sample as *mut *mut libc::c_void,
        );
        error = tmp___2;
        if error as libc::c_uint != 0 as libc::c_uint {
            return error;
        }
        Sample_Prepare(*sample, name_hash, parent);
        if flags & 8 as libc::c_uint != 0 as libc::c_uint {
            if !((*tree).currentParent as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong)
            {
                __assert_fail(
                    b"tree->currentParent != NULL\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    4841 as libc::c_uint,
                    b"SampleTree_Push\0" as *const u8 as *const libc::c_char,
                );
            }
            if !((*tree).sendSampleOnClose as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong)
            {
                __assert_fail(
                    b"tree->sendSampleOnClose == NULL\0" as *const u8
                        as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    4842 as libc::c_uint,
                    b"SampleTree_Push\0" as *const u8 as *const libc::c_char,
                );
            }
            (*tree).sendSampleOnClose = *sample;
        }
    } else {
        tmp___5 = ObjectAllocator_Alloc(
            (*tree).allocator,
            sample as *mut *mut libc::c_void,
        );
        error___0 = tmp___5;
        if error___0 as libc::c_uint != 0 as libc::c_uint {
            return error___0;
        }
        Sample_Prepare(*sample, name_hash, parent);
    }
    unique_id = (*parent).unique_id;
    unique_id = HashCombine(unique_id, (**sample).name_hash);
    unique_id = HashCombine(unique_id, (*parent).nb_children);
    (**sample).unique_id = unique_id;
    (*parent).nb_children = ((*parent).nb_children).wrapping_add(1);
    if (*parent).first_child as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
    {
        (*parent).first_child = *sample;
        (*parent).last_child = *sample;
    } else {
        if !((*parent).last_child as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong)
        {
            __assert_fail(
                b"parent->last_child != NULL\0" as *const u8 as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                4869 as libc::c_uint,
                b"SampleTree_Push\0" as *const u8 as *const libc::c_char,
            );
        }
        (*(*parent).last_child).next_sibling = *sample;
        (*parent).last_child = *sample;
    }
    (*tree).currentParent = *sample;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn SampleTree_Pop(mut tree: *mut SampleTree, mut sample: *mut Sample) {
    if !(tree as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"tree != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4882 as libc::c_uint,
            b"SampleTree_Pop\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4883 as libc::c_uint,
            b"SampleTree_Pop\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(sample as libc::c_ulong != (*tree).root as libc::c_ulong) {
        __assert_fail(
            b"sample != tree->root\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4884 as libc::c_uint,
            b"SampleTree_Pop\0" as *const u8 as *const libc::c_char,
        );
    }
    (*tree).currentParent = (*sample).parent;
}
unsafe extern "C" fn FlattenSamples(
    mut sample: *mut Sample,
    mut nb_samples: *mut rmtU32,
) -> *mut ObjectLink {
    let mut child: *mut Sample = 0 as *mut Sample;
    let mut cur_link: *mut ObjectLink = 0 as *mut ObjectLink;
    let mut last_link: *mut ObjectLink = 0 as *mut ObjectLink;
    let mut tmp___1: *mut ObjectLink = 0 as *mut ObjectLink;
    cur_link = &mut (*sample).Link;
    if !(sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4893 as libc::c_uint,
            b"FlattenSamples\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(nb_samples as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"nb_samples != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            4894 as libc::c_uint,
            b"FlattenSamples\0" as *const u8 as *const libc::c_char,
        );
    }
    *nb_samples = (*nb_samples).wrapping_add(1);
    (*sample).Link.next = (*sample).first_child as *mut ObjectLink as *mut ObjectLink_s;
    child = (*sample).first_child;
    while child as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = FlattenSamples(child, nb_samples);
        last_link = tmp___1;
        (*last_link)
            .next = (*child).next_sibling as *mut ObjectLink as *mut ObjectLink_s;
        cur_link = last_link;
        child = (*child).next_sibling;
    }
    (*sample).first_child = 0 as *mut libc::c_void as *mut Sample;
    (*sample).last_child = 0 as *mut libc::c_void as *mut Sample;
    (*sample).nb_children = 0 as libc::c_int as rmtU32;
    return cur_link;
}
unsafe extern "C" fn FreeSamples(
    mut sample: *mut Sample,
    mut allocator: *mut ObjectAllocator,
) {
    let mut nb_cleared_samples: rmtU32 = 0;
    let mut last_link: *mut ObjectLink = 0 as *mut ObjectLink;
    let mut tmp: *mut ObjectLink = 0 as *mut ObjectLink;
    nb_cleared_samples = 0 as libc::c_int as rmtU32;
    tmp = FlattenSamples(sample, &mut nb_cleared_samples);
    last_link = tmp;
    if (*sample).Link.next as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        ObjectAllocator_FreeRange(
            allocator,
            sample as *mut libc::c_void,
            last_link as *mut libc::c_void,
            nb_cleared_samples,
        );
    } else {
        ObjectAllocator_Free(allocator, sample as *mut libc::c_void);
    };
}
unsafe extern "C" fn SampleTree_CopySample(
    mut out_dst_sample: *mut *mut Sample,
    mut dst_parent_sample: *mut Sample,
    mut allocator: *mut ObjectAllocator,
    mut src_sample: *const Sample,
) -> rmtError {
    let mut src_child: *mut Sample = 0 as *mut Sample;
    let mut dst_sample: *mut Sample = 0 as *mut Sample;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut dst_child: *mut Sample = 0 as *mut Sample;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    tmp = ObjectAllocator_Alloc(
        allocator,
        &mut dst_sample as *mut *mut Sample as *mut *mut libc::c_void,
    );
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    Sample_CopyState(dst_sample, src_sample);
    if dst_parent_sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if (*dst_parent_sample).first_child as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            (*dst_parent_sample).first_child = dst_sample;
            (*dst_parent_sample).last_child = dst_sample;
        } else {
            if !((*dst_parent_sample).last_child as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong)
            {
                __assert_fail(
                    b"dst_parent_sample->last_child != NULL\0" as *const u8
                        as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    4952 as libc::c_uint,
                    b"SampleTree_CopySample\0" as *const u8 as *const libc::c_char,
                );
            }
            (*(*dst_parent_sample).last_child).next_sibling = dst_sample;
            (*dst_parent_sample).last_child = dst_sample;
        }
    }
    src_child = (*src_sample).first_child as *mut Sample;
    while src_child as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___1 = SampleTree_CopySample(
            &mut dst_child,
            dst_sample,
            allocator,
            src_child as *const Sample,
        );
        error___0 = tmp___1;
        if error___0 as libc::c_uint != 0 as libc::c_uint {
            return error___0;
        }
        src_child = (*src_child).next_sibling;
    }
    *out_dst_sample = dst_sample;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn SampleTree_Copy(
    mut dst_tree: *mut SampleTree,
    mut src_tree: *const SampleTree,
) -> rmtError {
    let mut allocator: *mut ObjectAllocator = 0 as *mut ObjectAllocator;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    allocator = (*src_tree).allocator;
    (*dst_tree).allocator = allocator;
    tmp = SampleTree_CopySample(
        &mut (*dst_tree).root,
        0 as *mut libc::c_void as *mut Sample,
        allocator,
        (*src_tree).root as *const Sample,
    );
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    (*dst_tree).currentParent = (*dst_tree).root;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn QueueSampleTree(
    mut queue: *mut rmtMessageQueue,
    mut sample: *mut Sample,
    mut allocator: *mut ObjectAllocator,
    mut thread_name: rmtPStr,
    mut user_data: rmtU32,
    mut thread_profiler: *mut ThreadProfiler,
    mut partial_tree: rmtBool,
) {
    let mut payload: *mut Msg_SampleTree = 0 as *mut Msg_SampleTree;
    let mut message: *mut Message = 0 as *mut Message;
    let mut tmp: *mut Message = 0 as *mut Message;
    tmp = rmtMessageQueue_AllocMessage(
        queue,
        ::std::mem::size_of::<Msg_SampleTree>() as libc::c_ulong as rmtU32,
        thread_profiler,
    );
    message = tmp;
    if message as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        FreeSamples(sample, allocator);
        return;
    }
    payload = ((*message).payload).as_mut_ptr() as *mut Msg_SampleTree;
    (*payload).rootSample = sample;
    (*payload).allocator = allocator;
    (*payload).threadName = thread_name;
    (*payload).userData = user_data;
    (*payload).partialTree = partial_tree;
    rmtMessageQueue_CommitMessage(message, MsgID_SampleTree);
}
unsafe extern "C" fn QueueAddToStringTable(
    mut queue: *mut rmtMessageQueue,
    mut hash: rmtU32,
    mut string: *const libc::c_char,
    mut length: size_t,
    mut thread_profiler: *mut ThreadProfiler,
) -> rmtBool {
    let mut payload: *mut Msg_AddToStringTable = 0 as *mut Msg_AddToStringTable;
    let mut nb_string_bytes: size_t = 0;
    let mut message: *mut Message = 0 as *mut Message;
    let mut tmp: *mut Message = 0 as *mut Message;
    nb_string_bytes = length.wrapping_add(1 as libc::c_ulong);
    tmp = rmtMessageQueue_AllocMessage(
        queue,
        (::std::mem::size_of::<Msg_AddToStringTable>() as libc::c_ulong)
            .wrapping_add(nb_string_bytes) as rmtU32,
        thread_profiler,
    );
    message = tmp;
    if message as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int as rmtBool;
    }
    payload = ((*message).payload).as_mut_ptr() as *mut Msg_AddToStringTable;
    (*payload).hash = hash;
    (*payload).length = length as rmtU32;
    memcpy(
        payload.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        string as *const libc::c_void,
        nb_string_bytes,
    );
    rmtMessageQueue_CommitMessage(message, MsgID_AddToStringTable);
    return 1 as libc::c_int as rmtBool;
}
unsafe extern "C" fn ThreadProfiler_Constructor(
    mut mq_to_rmt: *mut rmtMessageQueue,
    mut thread_profiler: *mut ThreadProfiler,
    mut thread_id: rmtThreadId,
) -> rmtError {
    let mut name_length: rmtU32 = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    (*thread_profiler).nbSamplesWithoutCallback = 0 as libc::c_int;
    (*thread_profiler).processorIndex = -(1 as libc::c_int) as rmtU32;
    (*thread_profiler).lastProcessorIndex = -(1 as libc::c_int) as rmtU32;
    (*thread_profiler).threadId = thread_id;
    memset(
        ((*thread_profiler).sampleTrees).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut SampleTree; 6]>() as libc::c_ulong,
    );
    tmp = rmtOpenThreadHandle(thread_id, &mut (*thread_profiler).threadHandle);
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    rmtGetThreadName(
        thread_id,
        (*thread_profiler).threadHandle,
        ((*thread_profiler).threadName).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as rmtU32,
    );
    name_length = strnlen_s_safe_c(
        ((*thread_profiler).threadName).as_mut_ptr() as *const libc::c_char,
        64 as libc::c_int as r_size_t,
    );
    (*thread_profiler)
        .threadNameHash = _rmt_HashString32(
        ((*thread_profiler).threadName).as_mut_ptr() as *const libc::c_char,
        name_length as libc::c_int,
        0 as libc::c_int as rmtU32,
    );
    QueueAddToStringTable(
        mq_to_rmt,
        (*thread_profiler).threadNameHash,
        ((*thread_profiler).threadName).as_mut_ptr() as *const libc::c_char,
        name_length as size_t,
        thread_profiler,
    );
    tmp___0 = rmtMalloc(::std::mem::size_of::<SampleTree>() as libc::c_ulong as rmtU32);
    (*thread_profiler)
        .sampleTrees[0 as libc::c_int as usize] = tmp___0 as *mut SampleTree;
    if (*thread_profiler).sampleTrees[0 as libc::c_int as usize] as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___1 = SampleTree_Constructor(
        (*thread_profiler).sampleTrees[0 as libc::c_int as usize],
        ::std::mem::size_of::<Sample>() as libc::c_ulong as rmtU32,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Sample) -> rmtError>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> rmtError>,
        >(Some(Sample_Constructor as unsafe extern "C" fn(*mut Sample) -> rmtError)),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Sample) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(Sample_Destructor as unsafe extern "C" fn(*mut Sample) -> ())),
    );
    error___0 = tmp___1;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        if (*thread_profiler).sampleTrees[0 as libc::c_int as usize] as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            SampleTree_Destructor(
                (*thread_profiler).sampleTrees[0 as libc::c_int as usize],
            );
            rmtFree(
                (*thread_profiler).sampleTrees[0 as libc::c_int as usize]
                    as *mut libc::c_void,
            );
            (*thread_profiler)
                .sampleTrees[0 as libc::c_int
                as usize] = 0 as *mut libc::c_void as *mut SampleTree;
        }
        return error___0;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ThreadProfiler_Destructor(
    mut thread_profiler: *mut ThreadProfiler,
) {
    let mut index___0: rmtU32 = 0;
    index___0 = 0 as libc::c_int as rmtU32;
    while index___0 < 6 as libc::c_uint {
        if (*thread_profiler).sampleTrees[index___0 as usize] as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            SampleTree_Destructor((*thread_profiler).sampleTrees[index___0 as usize]);
            rmtFree(
                (*thread_profiler).sampleTrees[index___0 as usize] as *mut libc::c_void,
            );
            (*thread_profiler)
                .sampleTrees[index___0
                as usize] = 0 as *mut libc::c_void as *mut SampleTree;
        }
        index___0 = index___0.wrapping_add(1);
    }
    rmtCloseThreadHandle((*thread_profiler).threadHandle);
}
unsafe extern "C" fn ThreadProfiler_Push(
    mut tree: *mut SampleTree,
    mut name_hash: rmtU32,
    mut flags: rmtU32,
    mut sample: *mut *mut Sample,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    StoreRelease(&mut (*tree).treeBeingModified, 1 as libc::c_int as rmtU32);
    error = SampleTree_Push(tree, name_hash, flags, sample);
    StoreRelease(&mut (*tree).treeBeingModified, 0 as libc::c_int as rmtU32);
    return error;
}
unsafe extern "C" fn CloseOpenSamples(
    mut sample: *mut Sample,
    mut sample_time_us: rmtU64,
    mut parents_are_last: rmtU32,
) {
    let mut child_sample: *mut Sample = 0 as *mut Sample;
    let mut is_last: rmtU32 = 0;
    let mut tmp: libc::c_int = 0;
    child_sample = (*sample).first_child;
    while child_sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if child_sample as libc::c_ulong == (*sample).last_child as libc::c_ulong {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
        is_last = parents_are_last & tmp as libc::c_uint;
        CloseOpenSamples(child_sample, sample_time_us, is_last);
        child_sample = (*child_sample).next_sibling;
    }
    if parents_are_last > 0 as libc::c_uint {
        Sample_Close(sample, sample_time_us as rmtS64);
    }
}
unsafe extern "C" fn MakePartialTreeCopy(
    mut sample_tree: *mut SampleTree,
    mut sample_time_us: rmtU64,
    mut out_sample_tree_copy: *mut SampleTree,
) -> rmtError {
    let mut sample_time_s: rmtU32 = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    sample_time_s = sample_time_us.wrapping_div(1000 as libc::c_ulonglong) as rmtU32;
    StoreRelease(&mut (*sample_tree).msLastTreeSendTime, sample_time_s);
    tmp = SampleTree_Copy(out_sample_tree_copy, sample_tree as *const SampleTree);
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    CloseOpenSamples(
        (*out_sample_tree_copy).root,
        sample_time_us,
        1 as libc::c_int as rmtU32,
    );
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ThreadProfiler_Pop(
    mut thread_profiler: *mut ThreadProfiler,
    mut queue: *mut rmtMessageQueue,
    mut sample: *mut Sample,
    mut msg_user_data: rmtU32,
) -> rmtBool {
    let mut tree: *mut SampleTree = 0 as *mut SampleTree;
    let mut root: *mut Sample = 0 as *mut Sample;
    let mut partial_tree: SampleTree = SampleTree {
        allocator: 0 as *mut ObjectAllocator,
        root: 0 as *mut Sample,
        currentParent: 0 as *mut Sample,
        msLastTreeSendTime: 0,
        treeBeingModified: 0,
        sendSampleOnClose: 0 as *mut Sample,
    };
    let mut sample___0: *mut Sample = 0 as *mut Sample;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    tree = (*thread_profiler).sampleTrees[(*sample).type_0 as usize];
    SampleTree_Pop(tree, sample);
    if (*tree).currentParent as libc::c_ulong == (*tree).root as libc::c_ulong {
        StoreRelease(&mut (*tree).treeBeingModified, 1 as libc::c_int as rmtU32);
        root = (*tree).root;
        (*root).first_child = 0 as *mut libc::c_void as *mut Sample;
        (*root).last_child = 0 as *mut libc::c_void as *mut Sample;
        (*root).nb_children = 0 as libc::c_int as rmtU32;
        StoreRelease(&mut (*tree).treeBeingModified, 0 as libc::c_int as rmtU32);
        QueueSampleTree(
            queue,
            sample,
            (*tree).allocator,
            ((*thread_profiler).threadName).as_mut_ptr() as rmtPStr,
            msg_user_data,
            thread_profiler,
            0 as libc::c_int as rmtBool,
        );
        StoreRelease(
            &mut (*tree).msLastTreeSendTime,
            ((*sample).us_end).wrapping_div(1000 as libc::c_ulonglong) as rmtU32,
        );
        return 1 as libc::c_int as rmtBool;
    }
    if (*tree).sendSampleOnClose as libc::c_ulong == sample as libc::c_ulong {
        tmp___0 = MakePartialTreeCopy(
            tree,
            ((*sample).us_start).wrapping_add((*sample).us_length),
            &mut partial_tree,
        );
        if tmp___0 as libc::c_uint == 0 as libc::c_uint {
            sample___0 = (*partial_tree.root).first_child;
            if !(sample___0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
            {
                __assert_fail(
                    b"sample != NULL\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    5241 as libc::c_uint,
                    b"ThreadProfiler_Pop\0" as *const u8 as *const libc::c_char,
                );
            }
            QueueSampleTree(
                queue,
                sample___0,
                partial_tree.allocator,
                ((*thread_profiler).threadName).as_mut_ptr() as rmtPStr,
                msg_user_data,
                thread_profiler,
                1 as libc::c_int as rmtBool,
            );
        }
        if partial_tree.root as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
        {
            FreeSamples(partial_tree.root, partial_tree.allocator);
        }
        (*tree).sendSampleOnClose = 0 as *mut libc::c_void as *mut Sample;
    }
    return 0 as libc::c_int as rmtBool;
}
unsafe extern "C" fn ThreadProfiler_GetNameHash(
    mut thread_profiler: *mut ThreadProfiler,
    mut queue: *mut rmtMessageQueue,
    mut name: rmtPStr,
    mut hash_cache: *mut rmtU32,
) -> rmtU32 {
    let mut name_len: size_t = 0;
    let mut name_hash: rmtU32 = 0;
    let mut tmp___0: r_size_t = 0;
    let mut tmp___1: rmtBool = 0;
    let mut tmp___2: r_size_t = 0;
    if hash_cache as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        name_hash = *hash_cache;
        if name_hash == 0 as libc::c_uint {
            if !(name as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
                __assert_fail(
                    b"name != NULL\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    5269 as libc::c_uint,
                    b"ThreadProfiler_GetNameHash\0" as *const u8 as *const libc::c_char,
                );
            }
            tmp___0 = strnlen_s_safe_c(name, 256 as libc::c_int as r_size_t);
            name_len = tmp___0 as size_t;
            name_hash = _rmt_HashString32(
                name,
                name_len as libc::c_int,
                0 as libc::c_int as rmtU32,
            );
            tmp___1 = QueueAddToStringTable(
                queue,
                name_hash,
                name,
                name_len,
                thread_profiler,
            );
            if tmp___1 == 1 as libc::c_uint {
                *hash_cache = name_hash;
            }
        }
        return name_hash;
    }
    tmp___2 = strnlen_s_safe_c(name, 256 as libc::c_int as r_size_t);
    name_len = tmp___2 as size_t;
    name_hash = _rmt_HashString32(
        name,
        name_len as libc::c_int,
        0 as libc::c_int as rmtU32,
    );
    QueueAddToStringTable(queue, name_hash, name, name_len, thread_profiler);
    return name_hash;
}
unsafe extern "C" fn ThreadProfilers_Constructor(
    mut thread_profilers: *mut ThreadProfilers,
    mut timer: *mut usTimer,
    mut mq_to_rmt_thread: *mut rmtMessageQueue,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    (*thread_profilers).timer = timer;
    (*thread_profilers).mqToRmtThread = mq_to_rmt_thread;
    (*thread_profilers).compiledSampleFn = 0 as *mut libc::c_void;
    (*thread_profilers).compiledSampleFnSize = 0 as libc::c_int as rmtU32;
    (*thread_profilers).threadProfilerTlsHandle = 4294967295 as libc::c_uint;
    (*thread_profilers).nbThreadProfilers = 0 as libc::c_int as rmtU32;
    (*thread_profilers)
        .maxNbThreadProfilers = (::std::mem::size_of::<[ThreadProfiler; 256]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<ThreadProfiler>() as libc::c_ulong)
        as rmtU32;
    mtxInit(&mut (*thread_profilers).threadProfilerMutex);
    (*thread_profilers).threadSampleThread = 0 as *mut libc::c_void as *mut rmtThread;
    (*thread_profilers).threadGatherThread = 0 as *mut libc::c_void as *mut rmtThread;
    tmp = tlsAlloc(&mut (*thread_profilers).threadProfilerTlsHandle);
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    if g_Settings.enableThreadSampler == 1 as libc::c_uint {
        tmp___0 = rmtMalloc(
            ::std::mem::size_of::<rmtThread>() as libc::c_ulong as rmtU32,
        );
        (*thread_profilers).threadSampleThread = tmp___0 as *mut rmtThread;
        if (*thread_profilers).threadSampleThread as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong
        {
            return RMT_ERROR_MALLOC_FAIL;
        }
        tmp___1 = rmtThread_Constructor(
            (*thread_profilers).threadSampleThread,
            Some(SampleThreadsLoop as unsafe extern "C" fn(*mut rmtThread) -> rmtError),
            thread_profilers as *mut libc::c_void,
        );
        error___0 = tmp___1;
        if error___0 as libc::c_uint != 0 as libc::c_uint {
            if (*thread_profilers).threadSampleThread as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong
            {
                rmtThread_Destructor((*thread_profilers).threadSampleThread);
                rmtFree((*thread_profilers).threadSampleThread as *mut libc::c_void);
                (*thread_profilers)
                    .threadSampleThread = 0 as *mut libc::c_void as *mut rmtThread;
            }
            return error___0;
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ThreadProfilers_Destructor(
    mut thread_profilers: *mut ThreadProfilers,
) {
    let mut thread_index: rmtU32 = 0;
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    if (*thread_profilers).threadSampleThread as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        rmtThread_Destructor((*thread_profilers).threadSampleThread);
        rmtFree((*thread_profilers).threadSampleThread as *mut libc::c_void);
        (*thread_profilers)
            .threadSampleThread = 0 as *mut libc::c_void as *mut rmtThread;
    }
    thread_index = 0 as libc::c_int as rmtU32;
    while thread_index < (*thread_profilers).nbThreadProfilers {
        thread_profiler = ((*thread_profilers).threadProfilers)
            .as_mut_ptr()
            .offset(thread_index as isize);
        ThreadProfiler_Destructor(thread_profiler);
        thread_index = thread_index.wrapping_add(1);
    }
    if (*thread_profilers).threadProfilerTlsHandle != 4294967295 as libc::c_uint {
        tlsFree((*thread_profilers).threadProfilerTlsHandle);
    }
    mtxDelete(&mut (*thread_profilers).threadProfilerMutex);
}
unsafe extern "C" fn ThreadProfilers_GetThreadProfiler(
    mut thread_profilers: *mut ThreadProfilers,
    mut thread_id: rmtThreadId,
    mut out_thread_profiler: *mut *mut ThreadProfiler,
) -> rmtError {
    let mut profiler_index: rmtU32 = 0;
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    let mut error: rmtError = RMT_ERROR_NONE;
    mtxLock(&mut (*thread_profilers).threadProfilerMutex);
    profiler_index = 0 as libc::c_int as rmtU32;
    while profiler_index < (*thread_profilers).nbThreadProfilers {
        thread_profiler = ((*thread_profilers).threadProfilers)
            .as_mut_ptr()
            .offset(profiler_index as isize);
        if (*thread_profiler).threadId == thread_id {
            *out_thread_profiler = thread_profiler;
            mtxUnlock(&mut (*thread_profilers).threadProfilerMutex);
            return RMT_ERROR_NONE;
        }
        profiler_index = profiler_index.wrapping_add(1);
    }
    thread_profiler = ((*thread_profilers).threadProfilers)
        .as_mut_ptr()
        .offset((*thread_profilers).nbThreadProfilers as isize);
    error = ThreadProfiler_Constructor(
        (*thread_profilers).mqToRmtThread,
        thread_profiler,
        thread_id,
    );
    if error as libc::c_uint != 0 as libc::c_uint {
        ThreadProfiler_Destructor(thread_profiler);
        mtxUnlock(&mut (*thread_profilers).threadProfilerMutex);
        return error;
    }
    *out_thread_profiler = thread_profiler;
    StoreRelease(
        &mut (*thread_profilers).nbThreadProfilers,
        ((*thread_profilers).nbThreadProfilers).wrapping_add(1 as libc::c_int as rmtU32),
    );
    mtxUnlock(&mut (*thread_profilers).threadProfilerMutex);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ThreadProfilers_GetCurrentThreadProfiler(
    mut thread_profilers: *mut ThreadProfilers,
    mut out_thread_profiler: *mut *mut ThreadProfiler,
) -> rmtError {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtThreadId = 0;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    tmp = tlsGet((*thread_profilers).threadProfilerTlsHandle);
    *out_thread_profiler = tmp as *mut ThreadProfiler;
    if *out_thread_profiler as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___0 = rmtGetCurrentThreadId();
        tmp___1 = ThreadProfilers_GetThreadProfiler(
            thread_profilers,
            tmp___0,
            out_thread_profiler,
        );
        error = tmp___1;
        if error as libc::c_uint != 0 as libc::c_uint {
            return error;
        }
        tlsSet(
            (*thread_profilers).threadProfilerTlsHandle,
            *out_thread_profiler as *mut libc::c_void,
        );
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn ThreadProfilers_ThreadInCallback(
    mut thread_profilers: *mut ThreadProfilers,
    mut context: *mut rmtCpuContext,
) -> rmtBool {
    return 0 as libc::c_int as rmtBool;
}
unsafe extern "C" fn GatherThreads(mut thread_profilers: *mut ThreadProfilers) {
    if !(thread_profilers as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"thread_profilers != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            5486 as libc::c_uint,
            b"GatherThreads\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn GatherThreadsLoop(mut thread: *mut rmtThread) -> rmtError {
    let mut thread_profilers: *mut ThreadProfilers = 0 as *mut ThreadProfilers;
    let mut sleep_time: rmtU32 = 0;
    thread_profilers = (*thread).param as *mut ThreadProfilers;
    sleep_time = 100 as libc::c_int as rmtU32;
    if !(thread_profilers as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"thread_profilers != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            5529 as libc::c_uint,
            b"GatherThreadsLoop\0" as *const u8 as *const libc::c_char,
        );
    }
    _rmt_SetCurrentThreadName(
        b"RemoteryGatherThreads\0" as *const u8 as *const libc::c_char,
    );
    while (*thread).request_exit == 0 as libc::c_uint {
        GatherThreads(thread_profilers);
        msSleep(sleep_time);
        sleep_time = minU32(
            sleep_time.wrapping_mul(2 as libc::c_uint),
            2000 as libc::c_int as rmtU32,
        );
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn QueueProcessorThreads(
    mut queue: *mut rmtMessageQueue,
    mut message_index: rmtU64,
    mut nb_processors: rmtU32,
    mut processors: *mut Processor,
) {
    let mut payload: *mut Msg_ProcessorThreads = 0 as *mut Msg_ProcessorThreads;
    let mut array_size: rmtU32 = 0;
    let mut message: *mut Message = 0 as *mut Message;
    let mut tmp: *mut Message = 0 as *mut Message;
    array_size = (nb_processors.wrapping_sub(1 as libc::c_uint) as libc::c_ulong)
        .wrapping_mul(::std::mem::size_of::<Processor>() as libc::c_ulong) as rmtU32;
    tmp = rmtMessageQueue_AllocMessage(
        queue,
        (::std::mem::size_of::<Msg_ProcessorThreads>() as libc::c_ulong)
            .wrapping_add(array_size as libc::c_ulong) as rmtU32,
        0 as *mut libc::c_void as *mut ThreadProfiler,
    );
    message = tmp;
    if message as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    payload = ((*message).payload).as_mut_ptr() as *mut Msg_ProcessorThreads;
    (*payload).messageIndex = message_index;
    (*payload).nbProcessors = nb_processors;
    memcpy(
        ((*payload).processors).as_mut_ptr() as *mut libc::c_void,
        processors as *const libc::c_void,
        (nb_processors as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Processor>() as libc::c_ulong),
    );
    rmtMessageQueue_CommitMessage(message, MsgID_ProcessorThreads);
}
unsafe extern "C" fn CheckForStallingSamples(
    mut stalling_sample_tree: *mut SampleTree,
    mut thread_profiler: *mut ThreadProfiler,
    mut sample_time_us: rmtU64,
) -> rmtError {
    let mut sample_tree: *mut SampleTree = 0 as *mut SampleTree;
    let mut sample_time_s: rmtU32 = 0;
    let mut tmp: rmtU32 = 0;
    let mut root_sample: *mut Sample = 0 as *mut Sample;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtU32 = 0;
    sample_time_s = sample_time_us.wrapping_div(1000 as libc::c_ulonglong) as rmtU32;
    (*stalling_sample_tree).root = 0 as *mut libc::c_void as *mut Sample;
    (*stalling_sample_tree).allocator = 0 as *mut libc::c_void as *mut ObjectAllocator;
    sample_tree = (*thread_profiler).sampleTrees[0 as libc::c_int as usize];
    tmp = LoadAcquire(&mut (*sample_tree).treeBeingModified);
    if tmp != 0 as libc::c_uint {
        return RMT_ERROR_NONE;
    }
    if sample_tree as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        root_sample = (*sample_tree).root;
        if root_sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            if (*root_sample).nb_children > 0 as libc::c_uint {
                tmp___1 = LoadAcquire(&mut (*sample_tree).msLastTreeSendTime);
                if sample_time_s.wrapping_sub(tmp___1) > 1000 as libc::c_uint {
                    tmp___0 = MakePartialTreeCopy(
                        sample_tree,
                        sample_time_us,
                        stalling_sample_tree,
                    );
                    error = tmp___0;
                    if error as libc::c_uint != 0 as libc::c_uint {
                        return error;
                    }
                }
            }
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn InitThreadSampling(
    mut thread_profilers: *mut ThreadProfilers,
) -> rmtError {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: time_t = 0;
    _rmt_SetCurrentThreadName(
        b"RemoterySampleThreads\0" as *const u8 as *const libc::c_char,
    );
    GatherThreads(thread_profilers);
    tmp = rmtMalloc(::std::mem::size_of::<rmtThread>() as libc::c_ulong as rmtU32);
    (*thread_profilers).threadGatherThread = tmp as *mut rmtThread;
    if (*thread_profilers).threadGatherThread as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___0 = rmtThread_Constructor(
        (*thread_profilers).threadGatherThread,
        Some(GatherThreadsLoop as unsafe extern "C" fn(*mut rmtThread) -> rmtError),
        thread_profilers as *mut libc::c_void,
    );
    error = tmp___0;
    if error as libc::c_uint != 0 as libc::c_uint {
        if (*thread_profilers).threadGatherThread as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            rmtThread_Destructor((*thread_profilers).threadGatherThread);
            rmtFree((*thread_profilers).threadGatherThread as *mut libc::c_void);
            (*thread_profilers)
                .threadGatherThread = 0 as *mut libc::c_void as *mut rmtThread;
        }
        return error;
    }
    tmp___1 = time(0 as *mut libc::c_void as *mut time_t);
    Well512_Init(tmp___1 as rmtU32);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn SampleThreadsLoop(mut rmt_thread: *mut rmtThread) -> rmtError {
    let mut context: rmtCpuContext = 0;
    let mut processor_message_index: rmtU32 = 0;
    let mut nb_processors: rmtU32 = 0;
    let mut processors: *mut Processor = 0 as *mut Processor;
    let mut processor_index: rmtU32 = 0;
    let mut thread_profilers: *mut ThreadProfilers = 0 as *mut ThreadProfilers;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut lfsr_seed: rmtU32 = 0;
    let mut lfsr_value: rmtU32 = 0;
    let mut nb_thread_profilers: rmtU32 = 0;
    let mut tmp___1: rmtU32 = 0;
    let mut highest_bit_set: rmtU32 = 0;
    let mut tmp___2: rmtU32 = 0;
    let mut table_size_log2: rmtU32 = 0;
    let mut xor_mask: rmtU32 = 0;
    let mut tmp___3: rmtU32 = 0;
    let mut thread_index: rmtU32 = 0;
    let mut thread_id: rmtThreadId = 0;
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    let mut thread_handle: rmtThreadHandle = 0;
    let mut sample_time_us: rmtU64 = 0;
    let mut sample_count: rmtU32 = 0;
    let mut stalling_sample_tree: SampleTree = SampleTree {
        allocator: 0 as *mut ObjectAllocator,
        root: 0 as *mut Sample,
        currentParent: 0 as *mut Sample,
        msLastTreeSendTime: 0,
        treeBeingModified: 0,
        sendSampleOnClose: 0 as *mut Sample,
    };
    let mut tmp___4: rmtBool = 0;
    let mut tmp___5: rmtS32 = 0;
    let mut tmp___7: rmtBool = 0;
    let mut tmp___8: rmtBool = 0;
    let mut tmp___10: rmtError = RMT_ERROR_NONE;
    let mut sample: *mut Sample = 0 as *mut Sample;
    let mut processor: *mut Processor = 0 as *mut Processor;
    let mut thread_profiler___0: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    let mut last_processor_index: rmtU32 = 0;
    let mut tmp___14: rmtU32 = 0;
    processor_message_index = 0 as libc::c_int as rmtU32;
    thread_profilers = (*rmt_thread).param as *mut ThreadProfilers;
    nb_processors = rmtGetNbProcessors();
    if nb_processors == 0 as libc::c_uint {
        return RMT_ERROR_UNKNOWN;
    }
    tmp = InitThreadSampling(thread_profilers);
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    tmp___0 = rmtMalloc(
        (nb_processors as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Processor>() as libc::c_ulong) as rmtU32,
    );
    processors = tmp___0 as *mut Processor;
    if processors as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    processor_index = 0 as libc::c_int as rmtU32;
    while processor_index < nb_processors {
        let ref mut fresh23 = (*processors.offset(processor_index as isize))
            .threadProfiler;
        *fresh23 = 0 as *mut libc::c_void as *mut ThreadProfiler;
        (*processors.offset(processor_index as isize))
            .sampleTime = 0 as libc::c_int as rmtU64;
        processor_index = processor_index.wrapping_add(1);
    }
    while (*rmt_thread).request_exit == 0 as libc::c_uint {
        tmp___1 = LoadAcquire(&mut (*thread_profilers).nbThreadProfilers);
        nb_thread_profilers = tmp___1;
        tmp___2 = Log2i(nb_thread_profilers);
        highest_bit_set = tmp___2;
        table_size_log2 = highest_bit_set.wrapping_add(1 as libc::c_uint);
        tmp___3 = GaloisLFSRMask(table_size_log2);
        xor_mask = tmp___3;
        lfsr_seed = Well512_RandomOpenLimit(nb_thread_profilers);
        lfsr_value = lfsr_seed;
        loop {
            lfsr_value = GaloisLFSRNext(lfsr_value, xor_mask);
            thread_index = lfsr_value.wrapping_sub(1 as libc::c_uint);
            if !(thread_index >= nb_thread_profilers) {
                thread_id = rmtGetCurrentThreadId();
                thread_profiler = ((*thread_profilers).threadProfilers)
                    .as_mut_ptr()
                    .offset(thread_index as isize);
                if !((*thread_profiler).threadId == thread_id) {
                    thread_handle = (*thread_profiler).threadHandle;
                    tmp___4 = rmtSuspendThread(thread_handle);
                    if !(tmp___4 == 0 as libc::c_uint) {
                        sample_time_us = usTimer_Get((*thread_profilers).timer);
                        tmp___5 = AtomicAddS32(
                            &mut (*thread_profiler).nbSamplesWithoutCallback,
                            1 as libc::c_int,
                        );
                        sample_count = tmp___5 as rmtU32;
                        processor_index = (*thread_profiler).processorIndex;
                        if processor_index != 4294967295 as libc::c_uint {
                            if !(processor_index < nb_processors) {
                                __assert_fail(
                                    b"processor_index < nb_processors\0" as *const u8
                                        as *const libc::c_char,
                                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                    5888 as libc::c_uint,
                                    b"SampleThreadsLoop\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            let ref mut fresh24 = (*processors
                                .offset(processor_index as isize))
                                .threadProfiler;
                            *fresh24 = thread_profiler;
                            (*processors.offset(processor_index as isize))
                                .sampleCount = sample_count;
                            (*processors.offset(processor_index as isize))
                                .sampleTime = sample_time_us;
                        }
                        if sample_count == 0 as libc::c_uint {
                            tmp___7 = rmtGetUserModeThreadContext(
                                thread_handle,
                                &mut context,
                            );
                            if tmp___7 == 1 as libc::c_uint {
                                tmp___8 = ThreadProfilers_ThreadInCallback(
                                    thread_profilers,
                                    &mut context,
                                );
                                if tmp___8 == 0 as libc::c_uint {
                                    rmtSetThreadContext(thread_handle, &mut context);
                                } else {
                                    AtomicAddS32(
                                        &mut (*thread_profiler).nbSamplesWithoutCallback,
                                        -(1 as libc::c_int),
                                    );
                                }
                            } else {
                                AtomicAddS32(
                                    &mut (*thread_profiler).nbSamplesWithoutCallback,
                                    -(1 as libc::c_int),
                                );
                            }
                        }
                        tmp___10 = CheckForStallingSamples(
                            &mut stalling_sample_tree,
                            thread_profiler,
                            sample_time_us,
                        );
                        if 0 as libc::c_uint != tmp___10 as libc::c_uint {
                            if !(stalling_sample_tree.allocator as libc::c_ulong
                                != 0 as *mut libc::c_void as libc::c_ulong)
                            {
                                __assert_fail(
                                    b"stalling_sample_tree.allocator != NULL\0" as *const u8
                                        as *const libc::c_char,
                                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                    5936 as libc::c_uint,
                                    b"SampleThreadsLoop\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            if stalling_sample_tree.root as libc::c_ulong
                                != 0 as *mut libc::c_void as libc::c_ulong
                            {
                                FreeSamples(
                                    stalling_sample_tree.root,
                                    stalling_sample_tree.allocator,
                                );
                            }
                        }
                        rmtResumeThread(thread_handle);
                        if stalling_sample_tree.root as libc::c_ulong
                            != 0 as *mut libc::c_void as libc::c_ulong
                        {
                            sample = (*stalling_sample_tree.root).first_child;
                            if !(sample as libc::c_ulong
                                != 0 as *mut libc::c_void as libc::c_ulong)
                            {
                                __assert_fail(
                                    b"sample != NULL\0" as *const u8 as *const libc::c_char,
                                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                    5951 as libc::c_uint,
                                    b"SampleThreadsLoop\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            QueueSampleTree(
                                (*thread_profilers).mqToRmtThread,
                                sample,
                                stalling_sample_tree.allocator,
                                ((*thread_profiler).threadName).as_mut_ptr() as rmtPStr,
                                0 as libc::c_int as rmtU32,
                                thread_profiler,
                                1 as libc::c_int as rmtBool,
                            );
                            (*stalling_sample_tree.root)
                                .first_child = 0 as *mut libc::c_void as *mut Sample;
                            (*stalling_sample_tree.root)
                                .last_child = 0 as *mut libc::c_void as *mut Sample;
                            (*stalling_sample_tree.root)
                                .nb_children = 0 as libc::c_int as rmtU32;
                            if !(stalling_sample_tree.allocator as libc::c_ulong
                                != 0 as *mut libc::c_void as libc::c_ulong)
                            {
                                __assert_fail(
                                    b"stalling_sample_tree.allocator != NULL\0" as *const u8
                                        as *const libc::c_char,
                                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                    5960 as libc::c_uint,
                                    b"SampleThreadsLoop\0" as *const u8 as *const libc::c_char,
                                );
                            }
                            FreeSamples(
                                stalling_sample_tree.root,
                                stalling_sample_tree.allocator,
                            );
                        }
                    }
                }
            }
            if !(lfsr_value != lfsr_seed) {
                break;
            }
        }
        processor_index = 0 as libc::c_int as rmtU32;
        while processor_index < nb_processors {
            processor = processors.offset(processor_index as isize);
            thread_profiler___0 = (*processor).threadProfiler;
            if thread_profiler___0 as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong
            {
                last_processor_index = (*thread_profiler___0).lastProcessorIndex;
                let mut current_block_99: u64;
                if last_processor_index != 4294967295 as libc::c_uint {
                    if last_processor_index != processor_index {
                        if !(last_processor_index < nb_processors) {
                            __assert_fail(
                                b"last_processor_index < nb_processors\0" as *const u8
                                    as *const libc::c_char,
                                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                                5980 as libc::c_uint,
                                b"SampleThreadsLoop\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        if (*processors.offset(last_processor_index as isize))
                            .threadProfiler as libc::c_ulong
                            == thread_profiler___0 as libc::c_ulong
                        {
                            let ref mut fresh25 = (*processors
                                .offset(last_processor_index as isize))
                                .threadProfiler;
                            *fresh25 = 0 as *mut libc::c_void as *mut ThreadProfiler;
                        }
                        current_block_99 = 18425699056680496821;
                    } else {
                        current_block_99 = 6722402881151701848;
                    }
                } else {
                    current_block_99 = 6722402881151701848;
                }
                match current_block_99 {
                    6722402881151701848 => {
                        if (*processor).sampleCount > 1 as libc::c_uint {
                            (*processor)
                                .threadProfiler = 0 as *mut libc::c_void
                                as *mut ThreadProfiler;
                        }
                    }
                    _ => {}
                }
                (*thread_profiler___0)
                    .lastProcessorIndex = (*thread_profiler___0).processorIndex;
            }
            processor_index = processor_index.wrapping_add(1);
        }
        tmp___14 = processor_message_index;
        processor_message_index = processor_message_index.wrapping_add(1);
        QueueProcessorThreads(
            (*thread_profilers).mqToRmtThread,
            tmp___14 as rmtU64,
            nb_processors,
            processors,
        );
    }
    if (*thread_profilers).threadGatherThread as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        rmtThread_Destructor((*thread_profilers).threadGatherThread);
        rmtFree((*thread_profilers).threadGatherThread as *mut libc::c_void);
        (*thread_profilers)
            .threadGatherThread = 0 as *mut libc::c_void as *mut rmtThread;
    }
    rmtFree(processors as *mut libc::c_void);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn PropertySnapshot_Constructor(
    mut snapshot: *mut PropertySnapshot,
) -> rmtError {
    if !(snapshot as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"snapshot != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6063 as libc::c_uint,
            b"PropertySnapshot_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    ObjectLink_Constructor(snapshot as *mut ObjectLink);
    (*snapshot).type_0 = RMT_PropertyType_rmtBool;
    (*snapshot).value.Bool = 0 as libc::c_int as rmtBool;
    (*snapshot).nameHash = 0 as libc::c_int as rmtU32;
    (*snapshot).uniqueID = 0 as libc::c_int as rmtU32;
    (*snapshot).nbChildren = 0 as libc::c_int as rmtU32;
    (*snapshot).depth = 0 as libc::c_int as rmtU8;
    (*snapshot).nextSnapshot = 0 as *mut libc::c_void as *mut PropertySnapshot;
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn PropertySnapshot_Destructor(mut snapshot: *mut PropertySnapshot) {}
static mut g_Remotery: *mut Remotery = 0 as *const libc::c_void as *mut libc::c_void
    as *mut Remotery;
static mut g_RemoteryCreated: rmtBool = 0 as libc::c_int as rmtBool;
unsafe extern "C" fn saturate(mut v: libc::c_double) -> libc::c_double {
    if v < 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int as libc::c_double;
    }
    if v > 1 as libc::c_int as libc::c_double {
        return 1 as libc::c_int as libc::c_double;
    }
    return v;
}
unsafe extern "C" fn PostProcessSamples(
    mut sample: *mut Sample,
    mut nb_samples: *mut rmtU32,
) {
    let mut child: *mut Sample = 0 as *mut Sample;
    let mut h: libc::c_double = 0.;
    let mut r: libc::c_double = 0.;
    let mut tmp___1: libc::c_double = 0.;
    let mut tmp___2: libc::c_double = 0.;
    let mut tmp___3: libc::c_double = 0.;
    let mut g: libc::c_double = 0.;
    let mut tmp___4: libc::c_double = 0.;
    let mut tmp___5: libc::c_double = 0.;
    let mut tmp___6: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    let mut tmp___7: libc::c_double = 0.;
    let mut tmp___8: libc::c_double = 0.;
    let mut tmp___9: libc::c_double = 0.;
    let mut k: libc::c_double = 0.;
    let mut tmp___10: rmtS32 = 0;
    let mut tmp___11: rmtS32 = 0;
    let mut tmp___12: rmtS32 = 0;
    let mut tmp___13: rmtS32 = 0;
    let mut tmp___14: rmtS32 = 0;
    let mut tmp___15: rmtS32 = 0;
    if !(sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6165 as libc::c_uint,
            b"PostProcessSamples\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(nb_samples as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"nb_samples != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6166 as libc::c_uint,
            b"PostProcessSamples\0" as *const u8 as *const libc::c_char,
        );
    }
    *nb_samples = (*nb_samples).wrapping_add(1);
    h = (*sample).name_hash as libc::c_double
        / 4294967295 as libc::c_uint as libc::c_double;
    tmp___1 = fmod(
        h * 6 as libc::c_int as libc::c_double + 0 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
    );
    tmp___2 = fabs(tmp___1 - 3 as libc::c_int as libc::c_double);
    tmp___3 = saturate(tmp___2 - 1 as libc::c_int as libc::c_double);
    r = tmp___3;
    tmp___4 = fmod(
        h * 6 as libc::c_int as libc::c_double + 4 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
    );
    tmp___5 = fabs(tmp___4 - 3 as libc::c_int as libc::c_double);
    tmp___6 = saturate(tmp___5 - 1 as libc::c_int as libc::c_double);
    g = tmp___6;
    tmp___7 = fmod(
        h * 6 as libc::c_int as libc::c_double + 2 as libc::c_int as libc::c_double,
        6 as libc::c_int as libc::c_double,
    );
    tmp___8 = fabs(tmp___7 - 3 as libc::c_int as libc::c_double);
    tmp___9 = saturate(tmp___8 - 1 as libc::c_int as libc::c_double);
    b = tmp___9;
    r = r * r
        * (3 as libc::c_int as libc::c_double - 2 as libc::c_int as libc::c_double * r);
    g = g * g
        * (3 as libc::c_int as libc::c_double - 2 as libc::c_int as libc::c_double * g);
    b = b * b
        * (3 as libc::c_int as libc::c_double - 2 as libc::c_int as libc::c_double * b);
    k = 0.4f64;
    r = r * k + (1 as libc::c_int as libc::c_double - k);
    g = g * k + (1 as libc::c_int as libc::c_double - k);
    b = b * k + (1 as libc::c_int as libc::c_double - k);
    tmp___10 = minS32(
        (r * 255 as libc::c_int as libc::c_double) as rmtS32,
        255 as libc::c_int,
    );
    tmp___11 = maxS32(tmp___10, 0 as libc::c_int);
    (*sample).uniqueColour[0 as libc::c_int as usize] = tmp___11 as rmtU8;
    tmp___12 = minS32(
        (g * 255 as libc::c_int as libc::c_double) as rmtS32,
        255 as libc::c_int,
    );
    tmp___13 = maxS32(tmp___12, 0 as libc::c_int);
    (*sample).uniqueColour[1 as libc::c_int as usize] = tmp___13 as rmtU8;
    tmp___14 = minS32(
        (b * 255 as libc::c_int as libc::c_double) as rmtS32,
        255 as libc::c_int,
    );
    tmp___15 = maxS32(tmp___14, 0 as libc::c_int);
    (*sample).uniqueColour[2 as libc::c_int as usize] = tmp___15 as rmtU8;
    child = (*sample).first_child;
    while child as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        PostProcessSamples(child, nb_samples);
        child = (*child).next_sibling;
    }
}
unsafe extern "C" fn Remotery_SendLogTextMessage(
    mut rmt: *mut Remotery,
    mut message: *mut Message,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    let mut write_start_offset: rmtU32 = 0;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut error___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    let mut error___2: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: rmtError = RMT_ERROR_NONE;
    let mut tmp___4: rmtBool = 0;
    error = RMT_ERROR_NONE;
    if !(rmt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6213 as libc::c_uint,
            b"Remotery_SendLogTextMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(message as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"message != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6214 as libc::c_uint,
            b"Remotery_SendLogTextMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    bin_buf = (*(*rmt).server).bin_buf;
    WebSocket_PrepareBuffer(bin_buf);
    tmp___1 = bin_MessageHeader(
        bin_buf,
        b"LOGM\0" as *const u8 as *const libc::c_char,
        &mut write_start_offset,
    );
    error___0 = tmp___1;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        return error___0;
    }
    tmp___2 = Buffer_Write(
        bin_buf,
        ((*message).payload).as_mut_ptr() as *const libc::c_void,
        (*message).payload_size,
    );
    error___1 = tmp___2;
    if error___1 as libc::c_uint != 0 as libc::c_uint {
        return error___1;
    }
    tmp___3 = bin_MessageFooter(bin_buf, write_start_offset);
    error___2 = tmp___3;
    if error___2 as libc::c_uint != 0 as libc::c_uint {
        return error___2;
    }
    tmp___4 = Server_IsClientConnected((*rmt).server);
    if tmp___4 == 1 as libc::c_uint {
        error = Server_Send(
            (*rmt).server,
            (*bin_buf).data as *const libc::c_void,
            (*bin_buf).bytes_used,
            20 as libc::c_int as rmtU32,
        );
    }
    if (*rmt).logfile as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        rmtWriteFile(
            (*rmt).logfile,
            ((*bin_buf).data).offset(10 as libc::c_int as isize) as *const libc::c_void,
            ((*bin_buf).bytes_used).wrapping_sub(10 as libc::c_uint),
        );
    }
    return error;
}
unsafe extern "C" fn bin_SampleName(
    mut buffer: *mut Buffer,
    mut name: *const libc::c_char,
    mut name_hash: rmtU32,
    mut name_length: rmtU32,
) -> rmtError {
    let mut write_start_offset: rmtU32 = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut error___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut error___2: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    let mut error___3: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: rmtError = RMT_ERROR_NONE;
    tmp = bin_MessageHeader(
        buffer,
        b"SSMP\0" as *const u8 as *const libc::c_char,
        &mut write_start_offset,
    );
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    tmp___0 = Buffer_WriteU32(buffer, name_hash);
    error___0 = tmp___0;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        return error___0;
    }
    tmp___1 = Buffer_WriteU32(buffer, name_length);
    error___1 = tmp___1;
    if error___1 as libc::c_uint != 0 as libc::c_uint {
        return error___1;
    }
    tmp___2 = Buffer_Write(
        buffer,
        name as *mut libc::c_void as *const libc::c_void,
        name_length,
    );
    error___2 = tmp___2;
    if error___2 as libc::c_uint != 0 as libc::c_uint {
        return error___2;
    }
    tmp___3 = bin_MessageFooter(buffer, write_start_offset);
    error___3 = tmp___3;
    if error___3 as libc::c_uint != 0 as libc::c_uint {
        return error___3;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_AddToStringTable(
    mut rmt: *mut Remotery,
    mut message: *mut Message,
) -> rmtError {
    let mut payload: *mut Msg_AddToStringTable = 0 as *mut Msg_AddToStringTable;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut name_inserted: rmtBool = 0;
    let mut tmp: rmtBool = 0;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    payload = ((*message).payload).as_mut_ptr() as *mut Msg_AddToStringTable;
    name = payload.offset(1 as libc::c_int as isize) as *const libc::c_char;
    tmp = StringTable_Insert((*rmt).string_table, (*payload).hash, name);
    name_inserted = tmp;
    if name_inserted == 1 as libc::c_uint {
        if (*rmt).logfile as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            bin_buf = (*(*rmt).server).bin_buf;
            (*bin_buf).bytes_used = 0 as libc::c_int as rmtU32;
            tmp___0 = bin_SampleName(bin_buf, name, (*payload).hash, (*payload).length);
            error = tmp___0;
            if error as libc::c_uint != 0 as libc::c_uint {
                return error;
            }
            rmtWriteFile(
                (*rmt).logfile,
                (*bin_buf).data as *const libc::c_void,
                (*bin_buf).bytes_used,
            );
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn bin_SampleTree(
    mut buffer: *mut Buffer,
    mut msg: *mut Msg_SampleTree,
) -> rmtError {
    let mut root_sample: *mut Sample = 0 as *mut Sample;
    let mut thread_name: [libc::c_char; 256] = [0; 256];
    let mut nb_samples: rmtU32 = 0;
    let mut write_start_offset: rmtU32 = 0;
    let mut tmp___2: r_size_t = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___4: rmtError = RMT_ERROR_NONE;
    let mut error___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___5: rmtError = RMT_ERROR_NONE;
    let mut error___2: rmtError = RMT_ERROR_NONE;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: rmtError = RMT_ERROR_NONE;
    let mut error___3: rmtError = RMT_ERROR_NONE;
    let mut tmp___8: rmtError = RMT_ERROR_NONE;
    let mut error___4: rmtError = RMT_ERROR_NONE;
    let mut tmp___9: rmtError = RMT_ERROR_NONE;
    let mut error___5: rmtError = RMT_ERROR_NONE;
    let mut tmp___10: rmtError = RMT_ERROR_NONE;
    nb_samples = 0 as libc::c_int as rmtU32;
    write_start_offset = 0 as libc::c_int as rmtU32;
    if !(buffer as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"buffer != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6273 as libc::c_uint,
            b"bin_SampleTree\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(msg as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"msg != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6274 as libc::c_uint,
            b"bin_SampleTree\0" as *const u8 as *const libc::c_char,
        );
    }
    root_sample = (*msg).rootSample;
    if !(root_sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"root_sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6278 as libc::c_uint,
            b"bin_SampleTree\0" as *const u8 as *const libc::c_char,
        );
    }
    thread_name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    tmp___2 = strnlen_s_safe_c((*msg).threadName, 255 as libc::c_int as r_size_t);
    strncat_s_safe_c(
        thread_name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
        (*msg).threadName,
        tmp___2,
    );
    if (*root_sample).type_0 as libc::c_uint == 1 as libc::c_uint {
        strncat_s_safe_c(
            thread_name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
            b" (CUDA)\0" as *const u8 as *const libc::c_char,
            7 as libc::c_int as r_size_t,
        );
    }
    if (*root_sample).type_0 as libc::c_uint == 2 as libc::c_uint {
        strncat_s_safe_c(
            thread_name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
            b" (D3D11)\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as r_size_t,
        );
    }
    if (*root_sample).type_0 as libc::c_uint == 3 as libc::c_uint {
        strncat_s_safe_c(
            thread_name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
            b" (D3D12)\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as r_size_t,
        );
    }
    if (*root_sample).type_0 as libc::c_uint == 4 as libc::c_uint {
        strncat_s_safe_c(
            thread_name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
            b" (OpenGL)\0" as *const u8 as *const libc::c_char,
            9 as libc::c_int as r_size_t,
        );
    }
    if (*root_sample).type_0 as libc::c_uint == 5 as libc::c_uint {
        strncat_s_safe_c(
            thread_name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as r_size_t,
            b" (Metal)\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as r_size_t,
        );
    }
    PostProcessSamples(root_sample, &mut nb_samples);
    tmp___3 = bin_MessageHeader(
        buffer,
        b"SMPL\0" as *const u8 as *const libc::c_char,
        &mut write_start_offset,
    );
    error = tmp___3;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    tmp___4 = Buffer_WriteStringWithLength(buffer, thread_name.as_mut_ptr() as rmtPStr);
    error___0 = tmp___4;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        return error___0;
    }
    tmp___5 = Buffer_WriteU32(buffer, nb_samples);
    error___1 = tmp___5;
    if error___1 as libc::c_uint != 0 as libc::c_uint {
        return error___1;
    }
    if (*msg).partialTree != 0 {
        tmp___6 = 1 as libc::c_int;
    } else {
        tmp___6 = 0 as libc::c_int;
    }
    tmp___7 = Buffer_WriteU32(buffer, tmp___6 as rmtU32);
    error___2 = tmp___7;
    if error___2 as libc::c_uint != 0 as libc::c_uint {
        return error___2;
    }
    tmp___8 = Buffer_AlignedPad(buffer, write_start_offset);
    error___3 = tmp___8;
    if error___3 as libc::c_uint != 0 as libc::c_uint {
        return error___3;
    }
    tmp___9 = bin_Sample(buffer, root_sample, 0 as libc::c_int as rmtU8);
    error___4 = tmp___9;
    if error___4 as libc::c_uint != 0 as libc::c_uint {
        return error___4;
    }
    tmp___10 = bin_MessageFooter(buffer, write_start_offset);
    error___5 = tmp___10;
    if error___5 as libc::c_uint != 0 as libc::c_uint {
        return error___5;
    }
    return RMT_ERROR_NONE;
}
static mut rmt_sample_hash_Server_Send: rmtU32 = 0 as libc::c_int as rmtU32;
unsafe extern "C" fn Remotery_SendToViewerAndLog(
    mut rmt: *mut Remotery,
    mut bin_buf: *mut Buffer,
    mut timeout: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtBool = 0;
    error = RMT_ERROR_NONE;
    tmp = Server_IsClientConnected((*rmt).server);
    if tmp == 1 as libc::c_uint {
        _rmt_BeginCPUSample(
            b"Server_Send\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as rmtU32,
            &mut rmt_sample_hash_Server_Send,
        );
        error = Server_Send(
            (*rmt).server,
            (*bin_buf).data as *const libc::c_void,
            (*bin_buf).bytes_used,
            timeout,
        );
        _rmt_EndCPUSample();
    }
    if (*rmt).logfile as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        rmtWriteFile(
            (*rmt).logfile,
            ((*bin_buf).data).offset(10 as libc::c_int as isize) as *const libc::c_void,
            ((*bin_buf).bytes_used).wrapping_sub(10 as libc::c_uint),
        );
    }
    return error;
}
static mut rmt_sample_hash_bin_SampleTree: rmtU32 = 0 as libc::c_int as rmtU32;
unsafe extern "C" fn Remotery_SendSampleTreeMessage(
    mut rmt: *mut Remotery,
    mut message: *mut Message,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut sample_tree: *mut Msg_SampleTree = 0 as *mut Msg_SampleTree;
    let mut sample: *mut Sample = 0 as *mut Sample;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    error = RMT_ERROR_NONE;
    if !(rmt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6357 as libc::c_uint,
            b"Remotery_SendSampleTreeMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(message as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"message != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6358 as libc::c_uint,
            b"Remotery_SendSampleTreeMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    sample_tree = ((*message).payload).as_mut_ptr() as *mut Msg_SampleTree;
    sample = (*sample_tree).rootSample;
    if !(sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"sample != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6363 as libc::c_uint,
            b"Remotery_SendSampleTreeMessage\0" as *const u8 as *const libc::c_char,
        );
    }
    bin_buf = (*(*rmt).server).bin_buf;
    WebSocket_PrepareBuffer(bin_buf);
    _rmt_BeginCPUSample(
        b"bin_SampleTree\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as rmtU32,
        &mut rmt_sample_hash_bin_SampleTree,
    );
    error = bin_SampleTree(bin_buf, sample_tree);
    _rmt_EndCPUSample();
    if ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut libc::c_void, *mut rmtSampleTree) -> ()>,
        libc::c_ulong,
    >(g_Settings.sampletree_handler) != 0 as *mut libc::c_void as libc::c_ulong
    {
        (Some((g_Settings.sampletree_handler).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(g_Settings.sampletree_context, sample_tree);
    }
    FreeSamples(sample, (*sample_tree).allocator);
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    tmp___2 = Remotery_SendToViewerAndLog(rmt, bin_buf, 50000 as libc::c_int as rmtU32);
    return tmp___2;
}
unsafe extern "C" fn Remotery_SendProcessorThreads(
    mut rmt: *mut Remotery,
    mut message: *mut Message,
) -> rmtError {
    let mut processor_index: rmtU32 = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut processor_threads: *mut Msg_ProcessorThreads = 0
        as *mut Msg_ProcessorThreads;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    let mut write_start_offset: rmtU32 = 0;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut error___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut error___2: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut processor: *mut Processor = 0 as *mut Processor;
    let mut error___3: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    let mut error___4: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: rmtError = RMT_ERROR_NONE;
    let mut error___5: rmtError = RMT_ERROR_NONE;
    let mut tmp___4: rmtError = RMT_ERROR_NONE;
    let mut error___6: rmtError = RMT_ERROR_NONE;
    let mut tmp___5: rmtError = RMT_ERROR_NONE;
    let mut error___7: rmtError = RMT_ERROR_NONE;
    let mut tmp___6: rmtError = RMT_ERROR_NONE;
    let mut error___8: rmtError = RMT_ERROR_NONE;
    let mut tmp___7: rmtError = RMT_ERROR_NONE;
    let mut error___9: rmtError = RMT_ERROR_NONE;
    let mut tmp___8: rmtError = RMT_ERROR_NONE;
    let mut tmp___9: rmtError = RMT_ERROR_NONE;
    error = RMT_ERROR_NONE;
    processor_threads = ((*message).payload).as_mut_ptr() as *mut Msg_ProcessorThreads;
    bin_buf = (*(*rmt).server).bin_buf;
    WebSocket_PrepareBuffer(bin_buf);
    tmp = bin_MessageHeader(
        bin_buf,
        b"PRTH\0" as *const u8 as *const libc::c_char,
        &mut write_start_offset,
    );
    error___0 = tmp;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        return error___0;
    }
    tmp___0 = Buffer_WriteU32(bin_buf, (*processor_threads).nbProcessors);
    error___1 = tmp___0;
    if error___1 as libc::c_uint != 0 as libc::c_uint {
        return error___1;
    }
    tmp___1 = Buffer_WriteU64(bin_buf, (*processor_threads).messageIndex);
    error___2 = tmp___1;
    if error___2 as libc::c_uint != 0 as libc::c_uint {
        return error___2;
    }
    processor_index = 0 as libc::c_int as rmtU32;
    while processor_index < (*processor_threads).nbProcessors {
        processor = ((*processor_threads).processors)
            .as_mut_ptr()
            .offset(processor_index as isize);
        if (*processor).threadProfiler as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            tmp___2 = Buffer_WriteU32(
                bin_buf,
                (*(*processor).threadProfiler).threadId as rmtU32,
            );
            error___3 = tmp___2;
            if error___3 as libc::c_uint != 0 as libc::c_uint {
                return error___3;
            }
            tmp___3 = Buffer_WriteU32(
                bin_buf,
                (*(*processor).threadProfiler).threadNameHash,
            );
            error___4 = tmp___3;
            if error___4 as libc::c_uint != 0 as libc::c_uint {
                return error___4;
            }
            tmp___4 = Buffer_WriteU64(bin_buf, (*processor).sampleTime);
            error___5 = tmp___4;
            if error___5 as libc::c_uint != 0 as libc::c_uint {
                return error___5;
            }
        } else {
            tmp___5 = Buffer_WriteU32(bin_buf, -(1 as libc::c_int) as rmtU32);
            error___6 = tmp___5;
            if error___6 as libc::c_uint != 0 as libc::c_uint {
                return error___6;
            }
            tmp___6 = Buffer_WriteU32(bin_buf, 0 as libc::c_int as rmtU32);
            error___7 = tmp___6;
            if error___7 as libc::c_uint != 0 as libc::c_uint {
                return error___7;
            }
            tmp___7 = Buffer_WriteU64(bin_buf, 0 as libc::c_int as rmtU64);
            error___8 = tmp___7;
            if error___8 as libc::c_uint != 0 as libc::c_uint {
                return error___8;
            }
        }
        processor_index = processor_index.wrapping_add(1);
    }
    tmp___8 = bin_MessageFooter(bin_buf, write_start_offset);
    error___9 = tmp___8;
    if error___9 as libc::c_uint != 0 as libc::c_uint {
        return error___9;
    }
    tmp___9 = Remotery_SendToViewerAndLog(rmt, bin_buf, 50 as libc::c_int as rmtU32);
    return tmp___9;
}
unsafe extern "C" fn FreePropertySnapshots(mut snapshot: *mut PropertySnapshot) {
    if snapshot as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    if (*snapshot).nextSnapshot as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        FreePropertySnapshots((*snapshot).nextSnapshot);
    }
    ObjectAllocator_Free((*g_Remotery).propertyAllocator, snapshot as *mut libc::c_void);
}
unsafe extern "C" fn Remotery_SerialisePropertySnapshots(
    mut bin_buf: *mut Buffer,
    mut msg_snapshot: *mut Msg_PropertySnapshot,
) -> rmtError {
    let mut snapshot: *mut PropertySnapshot = 0 as *mut PropertySnapshot;
    let mut empty_group: [rmtU8; 16] = [0; 16];
    let mut write_start_offset: rmtU32 = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut error___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut colour_depth: [rmtU8; 4] = [0; 4];
    let mut tmp___2: libc::c_uint = 0;
    let mut error___2: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: rmtError = RMT_ERROR_NONE;
    let mut error___3: rmtError = RMT_ERROR_NONE;
    let mut tmp___4: rmtError = RMT_ERROR_NONE;
    let mut error___4: rmtError = RMT_ERROR_NONE;
    let mut tmp___5: rmtError = RMT_ERROR_NONE;
    let mut error___5: rmtError = RMT_ERROR_NONE;
    let mut tmp___6: rmtError = RMT_ERROR_NONE;
    let mut error___6: rmtError = RMT_ERROR_NONE;
    let mut tmp___7: rmtError = RMT_ERROR_NONE;
    let mut error___7: rmtError = RMT_ERROR_NONE;
    let mut tmp___8: rmtError = RMT_ERROR_NONE;
    let mut error___8: rmtError = RMT_ERROR_NONE;
    let mut tmp___9: rmtError = RMT_ERROR_NONE;
    let mut error___9: rmtError = RMT_ERROR_NONE;
    let mut tmp___10: rmtError = RMT_ERROR_NONE;
    let mut error___10: rmtError = RMT_ERROR_NONE;
    let mut tmp___11: rmtError = RMT_ERROR_NONE;
    let mut error___11: rmtError = RMT_ERROR_NONE;
    let mut tmp___12: rmtError = RMT_ERROR_NONE;
    let mut error___12: rmtError = RMT_ERROR_NONE;
    let mut tmp___13: rmtError = RMT_ERROR_NONE;
    let mut error___13: rmtError = RMT_ERROR_NONE;
    let mut tmp___14: rmtError = RMT_ERROR_NONE;
    let mut error___14: rmtError = RMT_ERROR_NONE;
    let mut tmp___15: rmtError = RMT_ERROR_NONE;
    let mut error___15: rmtError = RMT_ERROR_NONE;
    let mut tmp___16: rmtError = RMT_ERROR_NONE;
    let mut error___16: rmtError = RMT_ERROR_NONE;
    let mut tmp___17: rmtError = RMT_ERROR_NONE;
    let mut error___17: rmtError = RMT_ERROR_NONE;
    let mut tmp___18: rmtError = RMT_ERROR_NONE;
    let mut error___18: rmtError = RMT_ERROR_NONE;
    let mut tmp___19: rmtError = RMT_ERROR_NONE;
    let mut error___19: rmtError = RMT_ERROR_NONE;
    let mut tmp___20: rmtError = RMT_ERROR_NONE;
    let mut error___20: rmtError = RMT_ERROR_NONE;
    let mut tmp___21: rmtError = RMT_ERROR_NONE;
    let mut error___21: rmtError = RMT_ERROR_NONE;
    let mut tmp___22: rmtError = RMT_ERROR_NONE;
    empty_group[0 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[1 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[2 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[3 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[4 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[5 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[6 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[7 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[8 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[9 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[10 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[11 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[12 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[13 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[14 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    empty_group[15 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
    tmp = bin_MessageHeader(
        bin_buf,
        b"PSNP\0" as *const u8 as *const libc::c_char,
        &mut write_start_offset,
    );
    error = tmp;
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    tmp___0 = Buffer_WriteU32(bin_buf, (*msg_snapshot).nbSnapshots);
    error___0 = tmp___0;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        return error___0;
    }
    tmp___1 = Buffer_WriteU32(bin_buf, (*msg_snapshot).propertyFrame);
    error___1 = tmp___1;
    if error___1 as libc::c_uint != 0 as libc::c_uint {
        return error___1;
    }
    snapshot = (*msg_snapshot).rootSnapshot;
    while snapshot as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        colour_depth[0 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
        colour_depth[1 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
        colour_depth[2 as libc::c_int as usize] = 0 as libc::c_int as rmtU8;
        tmp___2 = 3 as libc::c_uint;
        while !(tmp___2 >= 4 as libc::c_uint) {
            colour_depth[tmp___2 as usize] = 0 as libc::c_int as libc::c_uchar;
            tmp___2 = tmp___2.wrapping_add(1);
        }
        tmp___3 = Buffer_WriteU32(bin_buf, (*snapshot).nameHash);
        error___2 = tmp___3;
        if error___2 as libc::c_uint != 0 as libc::c_uint {
            return error___2;
        }
        tmp___4 = Buffer_WriteU32(bin_buf, (*snapshot).uniqueID);
        error___3 = tmp___4;
        if error___3 as libc::c_uint != 0 as libc::c_uint {
            return error___3;
        }
        colour_depth[3 as libc::c_int as usize] = (*snapshot).depth;
        tmp___5 = Buffer_Write(
            bin_buf,
            colour_depth.as_mut_ptr() as *const libc::c_void,
            4 as libc::c_int as rmtU32,
        );
        error___4 = tmp___5;
        if error___4 as libc::c_uint != 0 as libc::c_uint {
            return error___4;
        }
        tmp___6 = Buffer_WriteU32(bin_buf, (*snapshot).type_0 as rmtU32);
        error___5 = tmp___6;
        if error___5 as libc::c_uint != 0 as libc::c_uint {
            return error___5;
        }
        match (*snapshot).type_0 as libc::c_uint {
            0 => {
                tmp___7 = Buffer_Write(
                    bin_buf,
                    empty_group.as_mut_ptr() as *const libc::c_void,
                    16 as libc::c_int as rmtU32,
                );
                error___6 = tmp___7;
                if error___6 as libc::c_uint != 0 as libc::c_uint {
                    return error___6;
                }
            }
            1 => {
                tmp___8 = Buffer_WriteF64(bin_buf, (*snapshot).value.Bool as rmtF64);
                error___7 = tmp___8;
                if error___7 as libc::c_uint != 0 as libc::c_uint {
                    return error___7;
                }
                tmp___9 = Buffer_WriteF64(bin_buf, (*snapshot).prevValue.Bool as rmtF64);
                error___8 = tmp___9;
                if error___8 as libc::c_uint != 0 as libc::c_uint {
                    return error___8;
                }
            }
            2 => {
                tmp___10 = Buffer_WriteF64(bin_buf, (*snapshot).value.S32 as rmtF64);
                error___9 = tmp___10;
                if error___9 as libc::c_uint != 0 as libc::c_uint {
                    return error___9;
                }
                tmp___11 = Buffer_WriteF64(bin_buf, (*snapshot).prevValue.S32 as rmtF64);
                error___10 = tmp___11;
                if error___10 as libc::c_uint != 0 as libc::c_uint {
                    return error___10;
                }
            }
            3 => {
                tmp___12 = Buffer_WriteF64(bin_buf, (*snapshot).value.U32 as rmtF64);
                error___11 = tmp___12;
                if error___11 as libc::c_uint != 0 as libc::c_uint {
                    return error___11;
                }
                tmp___13 = Buffer_WriteF64(bin_buf, (*snapshot).prevValue.U32 as rmtF64);
                error___12 = tmp___13;
                if error___12 as libc::c_uint != 0 as libc::c_uint {
                    return error___12;
                }
            }
            4 => {
                tmp___14 = Buffer_WriteF64(bin_buf, (*snapshot).value.F32 as rmtF64);
                error___13 = tmp___14;
                if error___13 as libc::c_uint != 0 as libc::c_uint {
                    return error___13;
                }
                tmp___15 = Buffer_WriteF64(bin_buf, (*snapshot).prevValue.F32 as rmtF64);
                error___14 = tmp___15;
                if error___14 as libc::c_uint != 0 as libc::c_uint {
                    return error___14;
                }
            }
            6 | 5 => {
                tmp___16 = Buffer_WriteU64(bin_buf, (*snapshot).value.U64);
                error___15 = tmp___16;
                if error___15 as libc::c_uint != 0 as libc::c_uint {
                    return error___15;
                }
                tmp___17 = Buffer_WriteU64(bin_buf, (*snapshot).prevValue.U64);
                error___16 = tmp___17;
                if error___16 as libc::c_uint != 0 as libc::c_uint {
                    return error___16;
                }
            }
            7 => {
                tmp___18 = Buffer_WriteF64(bin_buf, (*snapshot).value.F64);
                error___17 = tmp___18;
                if error___17 as libc::c_uint != 0 as libc::c_uint {
                    return error___17;
                }
                tmp___19 = Buffer_WriteF64(bin_buf, (*snapshot).prevValue.F64);
                error___18 = tmp___19;
                if error___18 as libc::c_uint != 0 as libc::c_uint {
                    return error___18;
                }
            }
            _ => {}
        }
        tmp___20 = Buffer_WriteU32(bin_buf, (*snapshot).prevValueFrame);
        error___19 = tmp___20;
        if error___19 as libc::c_uint != 0 as libc::c_uint {
            return error___19;
        }
        tmp___21 = Buffer_WriteU32(bin_buf, (*snapshot).nbChildren);
        error___20 = tmp___21;
        if error___20 as libc::c_uint != 0 as libc::c_uint {
            return error___20;
        }
        snapshot = (*snapshot).nextSnapshot;
    }
    tmp___22 = bin_MessageFooter(bin_buf, write_start_offset);
    error___21 = tmp___22;
    if error___21 as libc::c_uint != 0 as libc::c_uint {
        return error___21;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_SendPropertySnapshot(
    mut rmt: *mut Remotery,
    mut message: *mut Message,
) -> rmtError {
    let mut msg_snapshot: *mut Msg_PropertySnapshot = 0 as *mut Msg_PropertySnapshot;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    msg_snapshot = ((*message).payload).as_mut_ptr() as *mut Msg_PropertySnapshot;
    error = RMT_ERROR_NONE;
    bin_buf = (*(*rmt).server).bin_buf;
    WebSocket_PrepareBuffer(bin_buf);
    error = Remotery_SerialisePropertySnapshots(bin_buf, msg_snapshot);
    if error as libc::c_uint == 0 as libc::c_uint {
        error = Remotery_SendToViewerAndLog(rmt, bin_buf, 50 as libc::c_int as rmtU32);
    }
    FreePropertySnapshots((*msg_snapshot).rootSnapshot);
    return error;
}
static mut rmt_sample_hash_SendSampleTreeMessage: rmtU32 = 0 as libc::c_int as rmtU32;
unsafe extern "C" fn Remotery_ConsumeMessageQueue(mut rmt: *mut Remotery) -> rmtError {
    let mut nb_messages_sent: rmtU32 = 0;
    let mut maxNbMessagesPerUpdate: rmtU32 = 0;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut message: *mut Message = 0 as *mut Message;
    let mut tmp___0: *mut Message = 0 as *mut Message;
    nb_messages_sent = 0 as libc::c_int as rmtU32;
    maxNbMessagesPerUpdate = g_Settings.maxNbMessagesPerUpdate;
    if !(rmt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6572 as libc::c_uint,
            b"Remotery_ConsumeMessageQueue\0" as *const u8 as *const libc::c_char,
        );
    }
    while nb_messages_sent < maxNbMessagesPerUpdate {
        error = RMT_ERROR_NONE;
        tmp___0 = rmtMessageQueue_PeekNextMessage((*rmt).mq_to_rmt_thread);
        message = tmp___0;
        if message as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            break;
        }
        match (*message).id as libc::c_uint {
            0 => {
                __assert_fail(
                    b"RMT_FALSE\0" as *const u8 as *const libc::c_char,
                    b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                    6587 as libc::c_uint,
                    b"Remotery_ConsumeMessageQueue\0" as *const u8 as *const libc::c_char,
                );
            }
            1 => {
                error = Remotery_AddToStringTable(rmt, message);
            }
            2 => {
                error = Remotery_SendLogTextMessage(rmt, message);
                nb_messages_sent = nb_messages_sent.wrapping_add(1);
            }
            3 => {
                _rmt_BeginCPUSample(
                    b"SendSampleTreeMessage\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as rmtU32,
                    &mut rmt_sample_hash_SendSampleTreeMessage,
                );
                error = Remotery_SendSampleTreeMessage(rmt, message);
                nb_messages_sent = nb_messages_sent.wrapping_add(1);
                _rmt_EndCPUSample();
            }
            4 => {
                Remotery_SendProcessorThreads(rmt, message);
                nb_messages_sent = nb_messages_sent.wrapping_add(1);
            }
            6 => {
                error = Remotery_SendPropertySnapshot(rmt, message);
            }
            _ => {}
        }
        rmtMessageQueue_ConsumeNextMessage((*rmt).mq_to_rmt_thread, message);
        if error as libc::c_uint != 0 as libc::c_uint {
            return error;
        }
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_FlushMessageQueue(mut rmt: *mut Remotery) {
    let mut message: *mut Message = 0 as *mut Message;
    let mut tmp___0: *mut Message = 0 as *mut Message;
    let mut sample_tree: *mut Msg_SampleTree = 0 as *mut Msg_SampleTree;
    let mut msg_snapshot: *mut Msg_PropertySnapshot = 0 as *mut Msg_PropertySnapshot;
    if !(rmt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6629 as libc::c_uint,
            b"Remotery_FlushMessageQueue\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        tmp___0 = rmtMessageQueue_PeekNextMessage((*rmt).mq_to_rmt_thread);
        message = tmp___0;
        if message as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            break;
        }
        match (*message).id as libc::c_uint {
            3 => {
                sample_tree = ((*message).payload).as_mut_ptr() as *mut Msg_SampleTree;
                FreeSamples((*sample_tree).rootSample, (*sample_tree).allocator);
            }
            6 => {
                msg_snapshot = ((*message).payload).as_mut_ptr()
                    as *mut Msg_PropertySnapshot;
                FreePropertySnapshots((*msg_snapshot).rootSnapshot);
            }
            2 | 1 | 0 | _ => {}
        }
        rmtMessageQueue_ConsumeNextMessage((*rmt).mq_to_rmt_thread, message);
    };
}
unsafe extern "C" fn Remotery_MapMessageQueue(mut rmt: *mut Remotery) {
    let mut read_pos: rmtU32 = 0;
    let mut write_pos: rmtU32 = 0;
    let mut queue: *mut rmtMessageQueue = 0 as *mut rmtMessageQueue;
    let mut tmp___0: *mut libc::c_long = 0 as *mut libc::c_long;
    let mut r: rmtU32 = 0;
    let mut message: *mut Message = 0 as *mut Message;
    let mut message_size: rmtU32 = 0;
    let mut tmp___1: rmtU32 = 0;
    if !(rmt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6672 as libc::c_uint,
            b"Remotery_MapMessageQueue\0" as *const u8 as *const libc::c_char,
        );
    }
    loop {
        tmp___0 = LoadAcquirePointer(
            &mut (*rmt).map_message_queue_data as *mut *mut libc::c_void
                as *mut *mut libc::c_long,
        );
        if !(tmp___0 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        msSleep(1 as libc::c_int as rmtU32);
    }
    queue = (*rmt).mq_to_rmt_thread;
    write_pos = LoadAcquire(&mut (*queue).write_pos);
    read_pos = (*queue).read_pos;
    while read_pos < write_pos {
        r = read_pos & ((*queue).size).wrapping_sub(1 as libc::c_uint);
        message = ((*(*queue).data).ptr).offset(r as isize) as *mut Message;
        tmp___1 = rmtMessageQueue_SizeForPayload((*message).payload_size);
        message_size = tmp___1;
        (Some(((*rmt).map_message_queue_fn).expect("non-null function pointer")))
            .expect("non-null function pointer")(rmt, message);
        read_pos = (read_pos as libc::c_uint).wrapping_add(message_size) as rmtU32
            as rmtU32;
    }
    StoreReleasePointer(
        &mut (*rmt).map_message_queue_data as *mut *mut libc::c_void
            as *mut *mut libc::c_long,
        0 as *mut libc::c_void as *mut libc::c_long,
    );
}
static mut rmt_sample_hash_Wakeup: rmtU32 = 0 as libc::c_int as rmtU32;
static mut rmt_sample_hash_ServerUpdate: rmtU32 = 0 as libc::c_int as rmtU32;
static mut rmt_sample_hash_ConsumeMessageQueue: rmtU32 = 0 as libc::c_int as rmtU32;
unsafe extern "C" fn Remotery_ThreadMain(mut thread: *mut rmtThread) -> rmtError {
    let mut rmt: *mut Remotery = 0 as *mut Remotery;
    let mut tmp___0: *mut libc::c_long = 0 as *mut libc::c_long;
    rmt = (*thread).param as *mut Remotery;
    if !(rmt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6700 as libc::c_uint,
            b"Remotery_ThreadMain\0" as *const u8 as *const libc::c_char,
        );
    }
    _rmt_SetCurrentThreadName(b"Remotery\0" as *const u8 as *const libc::c_char);
    while (*thread).request_exit == 0 as libc::c_uint {
        _rmt_BeginCPUSample(
            b"Wakeup\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as rmtU32,
            &mut rmt_sample_hash_Wakeup,
        );
        _rmt_BeginCPUSample(
            b"ServerUpdate\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as rmtU32,
            &mut rmt_sample_hash_ServerUpdate,
        );
        Server_Update((*rmt).server);
        _rmt_EndCPUSample();
        _rmt_BeginCPUSample(
            b"ConsumeMessageQueue\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int as rmtU32,
            &mut rmt_sample_hash_ConsumeMessageQueue,
        );
        Remotery_ConsumeMessageQueue(rmt);
        _rmt_EndCPUSample();
        _rmt_EndCPUSample();
        tmp___0 = LoadAcquirePointer(
            &mut (*rmt).map_message_queue_fn
                as *mut Option::<unsafe extern "C" fn(*mut Remotery, *mut Message) -> ()>
                as *mut *mut libc::c_long,
        );
        if tmp___0 as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            Remotery_MapMessageQueue(rmt);
            StoreReleasePointer(
                &mut (*rmt).map_message_queue_fn
                    as *mut Option::<
                        unsafe extern "C" fn(*mut Remotery, *mut Message) -> (),
                    > as *mut *mut libc::c_long,
                0 as *mut libc::c_void as *mut libc::c_long,
            );
        }
        msSleep(g_Settings.msSleepBetweenServerUpdates);
    }
    Remotery_FlushMessageQueue(rmt);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_ReceiveMessage(
    mut context: *mut libc::c_void,
    mut message_data: *mut libc::c_char,
    mut message_length: rmtU32,
) -> rmtError {
    let mut rmt: *mut Remotery = 0 as *mut Remotery;
    let mut message_id: rmtU32 = 0;
    let mut name: rmtPStr = 0 as *const libc::c_char;
    let mut name_hash: rmtU32 = 0;
    let mut cur: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut name_length: rmtU32 = 0;
    let mut tmp___0: r_size_t = 0;
    let mut bin_buf: *mut Buffer = 0 as *mut Buffer;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    rmt = context as *mut Remotery;
    message_id = *(message_data as *mut rmtU32);
    match message_id {
        1229868867 => {
            _rmt_LogText(
                b"Console message received...\0" as *const u8 as *const libc::c_char,
            );
            _rmt_LogText(message_data.offset(4 as libc::c_int as isize) as rmtPStr);
            if ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> (),
                >,
                libc::c_ulong,
            >(g_Settings.input_handler) != 0 as *mut libc::c_void as libc::c_ulong
            {
                (Some((g_Settings.input_handler).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    message_data.offset(4 as libc::c_int as isize)
                        as *const libc::c_char,
                    g_Settings.input_handler_context,
                );
            }
        }
        1347244871 => {
            name_hash = 0 as libc::c_int as rmtU32;
            cur = message_data.offset(4 as libc::c_int as isize) as *const libc::c_char;
            end = cur
                .offset(message_length as isize)
                .offset(-(4 as libc::c_int as isize));
            while (cur as libc::c_ulong) < end as libc::c_ulong {
                tmp = cur;
                cur = cur.offset(1);
                name_hash = name_hash
                    .wrapping_mul(10 as libc::c_uint)
                    .wrapping_add(*tmp as rmtU32)
                    .wrapping_sub(48 as libc::c_uint);
            }
            name = StringTable_Find((*rmt).string_table, name_hash);
            if name as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
                tmp___0 = strnlen_s_safe_c(name, 244 as libc::c_int as r_size_t);
                name_length = tmp___0;
                bin_buf = (*(*rmt).server).bin_buf;
                WebSocket_PrepareBuffer(bin_buf);
                bin_SampleName(bin_buf, name, name_hash, name_length);
                tmp___1 = Server_Send(
                    (*rmt).server,
                    (*bin_buf).data as *const libc::c_void,
                    (*bin_buf).bytes_used,
                    10 as libc::c_int as rmtU32,
                );
                return tmp___1;
            }
        }
        _ => {}
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_Constructor(mut rmt: *mut Remotery) -> rmtError {
    let mut root_property: *mut rmtProperty = 0 as *mut rmtProperty;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___3: rmtError = RMT_ERROR_NONE;
    let mut tmp___4: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error___1: rmtError = RMT_ERROR_NONE;
    let mut tmp___5: rmtError = RMT_ERROR_NONE;
    let mut now_tm: *mut tm = 0 as *mut tm;
    let mut tmp___6: *mut tm = 0 as *mut tm;
    let mut filename: [libc::c_char; 512] = [0; 512];
    let mut tmp___7: libc::c_uint = 0;
    let mut tmp___8: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___9: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___10: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___11: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___12: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___13: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___14: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error___2: rmtError = RMT_ERROR_NONE;
    let mut tmp___15: rmtError = RMT_ERROR_NONE;
    let mut tmp___16: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error___3: rmtError = RMT_ERROR_NONE;
    let mut tmp___17: rmtError = RMT_ERROR_NONE;
    let mut tmp___19: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error___4: rmtError = RMT_ERROR_NONE;
    let mut tmp___20: rmtError = RMT_ERROR_NONE;
    if !(rmt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6802 as libc::c_uint,
            b"Remotery_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    (*rmt).server = 0 as *mut libc::c_void as *mut Server;
    (*rmt).mq_to_rmt_thread = 0 as *mut libc::c_void as *mut rmtMessageQueue;
    (*rmt).thread = 0 as *mut libc::c_void as *mut rmtThread;
    (*rmt).string_table = 0 as *mut libc::c_void as *mut StringTable;
    (*rmt).logfile = 0 as *mut libc::c_void as *mut FILE;
    (*rmt)
        .map_message_queue_fn = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut Remotery, *mut Message) -> ()>,
    >(0 as *mut libc::c_void);
    (*rmt).map_message_queue_data = 0 as *mut libc::c_void;
    (*rmt).threadProfilers = 0 as *mut libc::c_void as *mut ThreadProfilers;
    mtxInit(&mut (*rmt).propertyMutex);
    (*rmt).propertyAllocator = 0 as *mut libc::c_void as *mut ObjectAllocator;
    (*rmt).propertyFrame = 0 as libc::c_int as rmtU32;
    root_property = &mut (*rmt).rootProperty;
    (*root_property).initialised = 1 as libc::c_int as rmtBool;
    (*root_property).type_0 = RMT_PropertyType_rmtGroup;
    (*root_property).value.Bool = 0 as libc::c_int as rmtBool;
    (*root_property).flags = RMT_PropertyFlags_NoFlags;
    (*root_property).name = b"Root Property\0" as *const u8 as *const libc::c_char;
    (*root_property).description = b"\0" as *const u8 as *const libc::c_char;
    (*root_property).defaultValue.Bool = 0 as libc::c_int as rmtBool;
    (*root_property).parent = 0 as *mut libc::c_void as *mut rmtProperty;
    (*root_property).firstChild = 0 as *mut libc::c_void as *mut rmtProperty;
    (*root_property).lastChild = 0 as *mut libc::c_void as *mut rmtProperty;
    (*root_property).nextSibling = 0 as *mut libc::c_void as *mut rmtProperty;
    (*root_property).nameHash = 0 as libc::c_int as rmtU32;
    (*root_property).uniqueID = 0 as libc::c_int as rmtU32;
    usTimer_Init(&mut (*rmt).timer);
    tmp___0 = rmtMalloc(::std::mem::size_of::<Server>() as libc::c_ulong as rmtU32);
    (*rmt).server = tmp___0 as *mut Server;
    if (*rmt).server as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___1 = Server_Constructor(
        (*rmt).server,
        g_Settings.port,
        g_Settings.reuse_open_port,
        g_Settings.limit_connections_to_localhost,
    );
    error = tmp___1;
    if error as libc::c_uint != 0 as libc::c_uint {
        if (*rmt).server as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            Server_Destructor((*rmt).server);
            rmtFree((*rmt).server as *mut libc::c_void);
            (*rmt).server = 0 as *mut libc::c_void as *mut Server;
        }
        return error;
    }
    (*(*rmt).server)
        .receive_handler = Some(
        Remotery_ReceiveMessage
            as unsafe extern "C" fn(
                *mut libc::c_void,
                *mut libc::c_char,
                rmtU32,
            ) -> rmtError,
    );
    (*(*rmt).server).receive_handler_context = rmt as *mut libc::c_void;
    tmp___2 = rmtMalloc(
        ::std::mem::size_of::<rmtMessageQueue>() as libc::c_ulong as rmtU32,
    );
    (*rmt).mq_to_rmt_thread = tmp___2 as *mut rmtMessageQueue;
    if (*rmt).mq_to_rmt_thread as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___3 = rmtMessageQueue_Constructor(
        (*rmt).mq_to_rmt_thread,
        g_Settings.messageQueueSizeInBytes,
    );
    error___0 = tmp___3;
    if error___0 as libc::c_uint != 0 as libc::c_uint {
        if (*rmt).mq_to_rmt_thread as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            rmtMessageQueue_Destructor((*rmt).mq_to_rmt_thread);
            rmtFree((*rmt).mq_to_rmt_thread as *mut libc::c_void);
            (*rmt).mq_to_rmt_thread = 0 as *mut libc::c_void as *mut rmtMessageQueue;
        }
        return error___0;
    }
    tmp___4 = rmtMalloc(::std::mem::size_of::<StringTable>() as libc::c_ulong as rmtU32);
    (*rmt).string_table = tmp___4 as *mut StringTable;
    if (*rmt).string_table as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___5 = StringTable_Constructor((*rmt).string_table);
    error___1 = tmp___5;
    if error___1 as libc::c_uint != 0 as libc::c_uint {
        if (*rmt).string_table as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            StringTable_Destructor((*rmt).string_table);
            rmtFree((*rmt).string_table as *mut libc::c_void);
            (*rmt).string_table = 0 as *mut libc::c_void as *mut StringTable;
        }
        return error___1;
    }
    if g_Settings.logPath as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___6 = TimeDateNow();
        now_tm = tmp___6;
        filename[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        tmp___7 = 1 as libc::c_uint;
        while !(tmp___7 >= 512 as libc::c_uint) {
            filename[tmp___7 as usize] = 0 as libc::c_int as libc::c_char;
            tmp___7 = tmp___7.wrapping_add(1);
        }
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            g_Settings.logPath,
            512 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"/remotery-log-\0" as *const u8 as *const libc::c_char,
            14 as libc::c_int as r_size_t,
        );
        tmp___8 = itoa_s((*now_tm).tm_year + 1900 as libc::c_int);
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            tmp___8,
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as r_size_t,
        );
        tmp___9 = itoa_s((*now_tm).tm_mon + 1 as libc::c_int);
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            tmp___9,
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as r_size_t,
        );
        tmp___10 = itoa_s((*now_tm).tm_mday);
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            tmp___10,
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as r_size_t,
        );
        tmp___11 = itoa_s((*now_tm).tm_hour);
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            tmp___11,
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as r_size_t,
        );
        tmp___12 = itoa_s((*now_tm).tm_min);
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            tmp___12,
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b"-\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as r_size_t,
        );
        tmp___13 = itoa_s((*now_tm).tm_sec);
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            tmp___13,
            11 as libc::c_int as r_size_t,
        );
        strncat_s_safe_c(
            filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as r_size_t,
            b".rbin\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as r_size_t,
        );
        (*rmt)
            .logfile = rmtOpenFile(
            filename.as_mut_ptr() as *const libc::c_char,
            b"w\0" as *const u8 as *const libc::c_char,
        );
        if (*rmt).logfile as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            rmtWriteFile(
                (*rmt).logfile,
                b"RMTBLOGF\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                8 as libc::c_int as rmtU32,
            );
        }
    }
    tmp___14 = rmtMalloc(
        ::std::mem::size_of::<ThreadProfilers>() as libc::c_ulong as rmtU32,
    );
    (*rmt).threadProfilers = tmp___14 as *mut ThreadProfilers;
    if (*rmt).threadProfilers as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong
    {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___15 = ThreadProfilers_Constructor(
        (*rmt).threadProfilers,
        &mut (*rmt).timer,
        (*rmt).mq_to_rmt_thread,
    );
    error___2 = tmp___15;
    if error___2 as libc::c_uint != 0 as libc::c_uint {
        if (*rmt).threadProfilers as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            ThreadProfilers_Destructor((*rmt).threadProfilers);
            rmtFree((*rmt).threadProfilers as *mut libc::c_void);
            (*rmt).threadProfilers = 0 as *mut libc::c_void as *mut ThreadProfilers;
        }
        return error___2;
    }
    tmp___16 = rmtMalloc(
        ::std::mem::size_of::<ObjectAllocator>() as libc::c_ulong as rmtU32,
    );
    (*rmt).propertyAllocator = tmp___16 as *mut ObjectAllocator;
    if (*rmt).propertyAllocator as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___17 = ObjectAllocator_Constructor(
        (*rmt).propertyAllocator,
        ::std::mem::size_of::<PropertySnapshot>() as libc::c_ulong as rmtU32,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut PropertySnapshot) -> rmtError>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> rmtError>,
        >(
            Some(
                PropertySnapshot_Constructor
                    as unsafe extern "C" fn(*mut PropertySnapshot) -> rmtError,
            ),
        ),
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut PropertySnapshot) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(
            Some(
                PropertySnapshot_Destructor
                    as unsafe extern "C" fn(*mut PropertySnapshot) -> (),
            ),
        ),
    );
    error___3 = tmp___17;
    if error___3 as libc::c_uint != 0 as libc::c_uint {
        if (*rmt).propertyAllocator as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
            ObjectAllocator_Destructor((*rmt).propertyAllocator);
            rmtFree((*rmt).propertyAllocator as *mut libc::c_void);
            (*rmt).propertyAllocator = 0 as *mut libc::c_void as *mut ObjectAllocator;
        }
        return error___3;
    }
    if !(g_Remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"g_Remotery == NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6922 as libc::c_uint,
            b"Remotery_Constructor\0" as *const u8 as *const libc::c_char,
        );
    }
    g_Remotery = rmt;
    g_RemoteryCreated = 1 as libc::c_int as rmtBool;
    CompilerWriteFence();
    tmp___19 = rmtMalloc(::std::mem::size_of::<rmtThread>() as libc::c_ulong as rmtU32);
    (*rmt).thread = tmp___19 as *mut rmtThread;
    if (*rmt).thread as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___20 = rmtThread_Constructor(
        (*rmt).thread,
        Some(Remotery_ThreadMain as unsafe extern "C" fn(*mut rmtThread) -> rmtError),
        rmt as *mut libc::c_void,
    );
    error___4 = tmp___20;
    if error___4 as libc::c_uint != 0 as libc::c_uint {
        if (*rmt).thread as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            rmtThread_Destructor((*rmt).thread);
            rmtFree((*rmt).thread as *mut libc::c_void);
            (*rmt).thread = 0 as *mut libc::c_void as *mut rmtThread;
        }
        return error___4;
    }
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn Remotery_Destructor(mut rmt: *mut Remotery) {
    if !(rmt as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"rmt != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            6937 as libc::c_uint,
            b"Remotery_Destructor\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*rmt).thread as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        rmtThread_Destructor((*rmt).thread);
        rmtFree((*rmt).thread as *mut libc::c_void);
        (*rmt).thread = 0 as *mut libc::c_void as *mut rmtThread;
    }
    if g_RemoteryCreated != 0 {
        g_Remotery = 0 as *mut libc::c_void as *mut Remotery;
        g_RemoteryCreated = 0 as libc::c_int as rmtBool;
    }
    if (*rmt).propertyAllocator as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        ObjectAllocator_Destructor((*rmt).propertyAllocator);
        rmtFree((*rmt).propertyAllocator as *mut libc::c_void);
        (*rmt).propertyAllocator = 0 as *mut libc::c_void as *mut ObjectAllocator;
    }
    if (*rmt).threadProfilers as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
    {
        ThreadProfilers_Destructor((*rmt).threadProfilers);
        rmtFree((*rmt).threadProfilers as *mut libc::c_void);
        (*rmt).threadProfilers = 0 as *mut libc::c_void as *mut ThreadProfilers;
    }
    rmtCloseFile((*rmt).logfile);
    if (*rmt).string_table as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        StringTable_Destructor((*rmt).string_table);
        rmtFree((*rmt).string_table as *mut libc::c_void);
        (*rmt).string_table = 0 as *mut libc::c_void as *mut StringTable;
    }
    if (*rmt).mq_to_rmt_thread as libc::c_ulong
        != 0 as *mut libc::c_void as libc::c_ulong
    {
        rmtMessageQueue_Destructor((*rmt).mq_to_rmt_thread);
        rmtFree((*rmt).mq_to_rmt_thread as *mut libc::c_void);
        (*rmt).mq_to_rmt_thread = 0 as *mut libc::c_void as *mut rmtMessageQueue;
    }
    if (*rmt).server as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        Server_Destructor((*rmt).server);
        rmtFree((*rmt).server as *mut libc::c_void);
        (*rmt).server = 0 as *mut libc::c_void as *mut Server;
    }
    if g_lastErrorMessageTlsHandle != 4294967295 as libc::c_uint {
        tlsFree(g_lastErrorMessageTlsHandle);
        g_lastErrorMessageTlsHandle = 4294967295 as libc::c_uint;
    }
    mtxDelete(&mut (*rmt).propertyMutex);
}
unsafe extern "C" fn CRTMalloc(
    mut mm_context: *mut libc::c_void,
    mut size: rmtU32,
) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = malloc(size as size_t);
    return tmp;
}
unsafe extern "C" fn CRTFree(
    mut mm_context: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
) {
    free(ptr);
}
unsafe extern "C" fn CRTRealloc(
    mut mm_context: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut size: rmtU32,
) -> *mut libc::c_void {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = realloc(ptr, size as size_t);
    return tmp;
}
pub unsafe extern "C" fn _rmt_Settings() -> *mut rmtSettings {
    if g_SettingsInitialized == 0 as libc::c_uint {
        g_Settings.port = 17815 as libc::c_int as rmtU16;
        g_Settings.reuse_open_port = 0 as libc::c_int as rmtBool;
        g_Settings.limit_connections_to_localhost = 0 as libc::c_int as rmtBool;
        g_Settings.enableThreadSampler = 1 as libc::c_int as rmtBool;
        g_Settings.msSleepBetweenServerUpdates = 4 as libc::c_int as rmtU32;
        g_Settings.messageQueueSizeInBytes = 1048576 as libc::c_int as rmtU32;
        g_Settings.maxNbMessagesPerUpdate = 1000 as libc::c_int as rmtU32;
        g_Settings
            .malloc = Some(
            CRTMalloc
                as unsafe extern "C" fn(*mut libc::c_void, rmtU32) -> *mut libc::c_void,
        );
        g_Settings
            .free = Some(
            CRTFree as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
        );
        g_Settings
            .realloc = Some(
            CRTRealloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    rmtU32,
                ) -> *mut libc::c_void,
        );
        g_Settings
            .input_handler = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> ()>,
        >(0 as *mut libc::c_void);
        g_Settings.input_handler_context = 0 as *mut libc::c_void;
        g_Settings.logPath = 0 as *mut libc::c_void as rmtPStr;
        g_Settings
            .sampletree_handler = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut rmtSampleTree) -> ()>,
        >(0 as *mut libc::c_void);
        g_Settings.sampletree_context = 0 as *mut libc::c_void;
        g_Settings
            .snapshot_callback = ::std::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut rmtProperty) -> ()>,
        >(0 as *mut libc::c_void);
        g_Settings.snapshot_context = 0 as *mut libc::c_void;
        g_SettingsInitialized = 1 as libc::c_int as rmtBool;
    }
    return &mut g_Settings;
}
pub unsafe extern "C" fn _rmt_CreateGlobalInstance(
    mut remotery: *mut *mut Remotery,
) -> rmtError {
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut tmp___2: rmtError = RMT_ERROR_NONE;
    if !(::std::mem::size_of::<MessageID>() as libc::c_ulong
        == ::std::mem::size_of::<rmtU32>() as libc::c_ulong)
    {
        __assert_fail(
            b"sizeof(MessageID) == sizeof(rmtU32)\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            7037 as libc::c_uint,
            b"_rmt_CreateGlobalInstance\0" as *const u8 as *const libc::c_char,
        );
    }
    _rmt_Settings();
    if !(remotery as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"remotery != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            7043 as libc::c_uint,
            b"_rmt_CreateGlobalInstance\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___1 = rmtMalloc(::std::mem::size_of::<Remotery>() as libc::c_ulong as rmtU32);
    *remotery = tmp___1 as *mut Remotery;
    if *remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_MALLOC_FAIL;
    }
    tmp___2 = Remotery_Constructor(*remotery);
    error = tmp___2;
    if error as libc::c_uint != 0 as libc::c_uint {
        if *remotery as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
            Remotery_Destructor(*remotery);
            rmtFree(*remotery as *mut libc::c_void);
            *remotery = 0 as *mut libc::c_void as *mut Remotery;
        }
        return error;
    }
    return RMT_ERROR_NONE;
}
pub unsafe extern "C" fn _rmt_DestroyGlobalInstance(mut remotery: *mut Remotery) {
    if !(g_RemoteryCreated == 1 as libc::c_uint) {
        __assert_fail(
            b"g_RemoteryCreated == RMT_TRUE\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            7051 as libc::c_uint,
            b"_rmt_DestroyGlobalInstance\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(g_Remotery as libc::c_ulong == remotery as libc::c_ulong) {
        __assert_fail(
            b"g_Remotery == remotery\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            7052 as libc::c_uint,
            b"_rmt_DestroyGlobalInstance\0" as *const u8 as *const libc::c_char,
        );
    }
    if remotery as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        Remotery_Destructor(remotery);
        rmtFree(remotery as *mut libc::c_void);
        remotery = 0 as *mut libc::c_void as *mut Remotery;
    }
}
pub unsafe extern "C" fn _rmt_SetGlobalInstance(mut remotery: *mut Remotery) {
    _rmt_Settings();
    g_Remotery = remotery;
}
pub unsafe extern "C" fn _rmt_GetGlobalInstance() -> *mut Remotery {
    return g_Remotery;
}
pub unsafe extern "C" fn MakeWideString(
    mut string: *const libc::c_char,
) -> *mut wchar_t {
    let mut wlen: size_t = 0;
    let mut wstr: *mut wchar_t = 0 as *mut wchar_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: size_t = 0;
    wlen = mbstowcs(
        0 as *mut libc::c_void as *mut wchar_t,
        string,
        256 as libc::c_int as size_t,
    );
    tmp = rmtMalloc(
        wlen
            .wrapping_add(1 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<wchar_t>() as libc::c_ulong) as rmtU32,
    );
    wstr = tmp as *mut wchar_t;
    if wstr as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut wchar_t;
    }
    tmp___0 = mbstowcs(wstr, string, wlen.wrapping_add(1 as libc::c_ulong));
    if tmp___0 != wlen {
        rmtFree(wstr as *mut libc::c_void);
        return 0 as *mut libc::c_void as *mut wchar_t;
    }
    return wstr;
}
unsafe extern "C" fn SetDebuggerThreadName(mut name: *const libc::c_char) {
    let mut name_clamp: [libc::c_char; 16] = [0; 16];
    name_clamp[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    strncat_s_safe_c(
        name_clamp.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as r_size_t,
        name,
        15 as libc::c_int as r_size_t,
    );
    prctl(
        15 as libc::c_int,
        name_clamp.as_mut_ptr(),
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn _rmt_SetCurrentThreadName(mut thread_name: rmtPStr) {
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    let mut tmp: rmtError = RMT_ERROR_NONE;
    let mut tmp___0: r_size_t = 0;
    if g_Remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    tmp = ThreadProfilers_GetCurrentThreadProfiler(
        (*g_Remotery).threadProfilers,
        &mut thread_profiler,
    );
    if tmp as libc::c_uint != 0 as libc::c_uint {
        return;
    }
    strcpy_s_safe_c(
        ((*thread_profiler).threadName).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as r_size_t,
        thread_name,
    );
    tmp___0 = strnlen_s_safe_c(thread_name, 64 as libc::c_int as r_size_t);
    (*thread_profiler)
        .threadNameHash = _rmt_HashString32(
        thread_name,
        tmp___0 as libc::c_int,
        0 as libc::c_int as rmtU32,
    );
    SetDebuggerThreadName(thread_name);
}
unsafe extern "C" fn QueueLine(
    mut queue: *mut rmtMessageQueue,
    mut text: *mut libc::c_uchar,
    mut size: rmtU32,
    mut thread_profiler: *mut ThreadProfiler,
) -> rmtBool {
    let mut message: *mut Message = 0 as *mut Message;
    let mut text_size: rmtU32 = 0;
    if !(queue as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
        __assert_fail(
            b"queue != NULL\0" as *const u8 as *const libc::c_char,
            b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
            7207 as libc::c_uint,
            b"QueueLine\0" as *const u8 as *const libc::c_char,
        );
    }
    text_size = size.wrapping_sub(4 as libc::c_uint);
    U32ToByteArray(text, text_size);
    message = rmtMessageQueue_AllocMessage(queue, size, thread_profiler);
    if message as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return 0 as libc::c_int as rmtBool;
    }
    memcpy(
        ((*message).payload).as_mut_ptr() as *mut libc::c_void,
        text as *const libc::c_void,
        size as size_t,
    );
    rmtMessageQueue_CommitMessage(message, MsgID_LogText);
    return 1 as libc::c_int as rmtBool;
}
pub unsafe extern "C" fn _rmt_LogText(mut text: rmtPStr) {
    let mut start_offset: libc::c_int = 0;
    let mut offset: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut line_buffer: [libc::c_uchar; 1024] = [0; 1024];
    let mut tmp: libc::c_uint = 0;
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut c: libc::c_char = 0;
    let mut tmp___1: rmtBool = 0;
    let mut tmp___2: libc::c_int = 0;
    line_buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_uchar;
    tmp = 1 as libc::c_uint;
    while !(tmp >= 1024 as libc::c_uint) {
        line_buffer[tmp as usize] = 0 as libc::c_int as libc::c_uchar;
        tmp = tmp.wrapping_add(1);
    }
    if g_Remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    tmp___0 = ThreadProfilers_GetCurrentThreadProfiler(
        (*g_Remotery).threadProfilers,
        &mut thread_profiler,
    );
    if tmp___0 as libc::c_uint != 0 as libc::c_uint {
        return;
    }
    line_buffer[0 as libc::c_int as usize] = ' ' as i32 as libc::c_uchar;
    line_buffer[1 as libc::c_int as usize] = ' ' as i32 as libc::c_uchar;
    line_buffer[2 as libc::c_int as usize] = ' ' as i32 as libc::c_uchar;
    line_buffer[3 as libc::c_int as usize] = ' ' as i32 as libc::c_uchar;
    start_offset = 4 as libc::c_int;
    offset = start_offset;
    i = 0 as libc::c_int;
    while *text.offset(i as isize) as libc::c_int != 0 as libc::c_int {
        let mut current_block_29: u64;
        c = *text.offset(i as isize);
        if offset as libc::c_ulong
            == (::std::mem::size_of::<[libc::c_uchar; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_ulong)
        {
            current_block_29 = 16324386933792234486;
        } else if c as libc::c_int == 10 as libc::c_int {
            current_block_29 = 16324386933792234486;
        } else {
            current_block_29 = 7056779235015430508;
        }
        match current_block_29 {
            16324386933792234486 => {
                tmp___1 = QueueLine(
                    (*g_Remotery).mq_to_rmt_thread,
                    line_buffer.as_mut_ptr(),
                    offset as rmtU32,
                    thread_profiler,
                );
                if tmp___1 == 0 as libc::c_uint {
                    return;
                }
                offset = start_offset;
                if c as libc::c_int == 10 as libc::c_int {
                    current_block_29 = 2593591675580731294;
                } else {
                    current_block_29 = 7056779235015430508;
                }
            }
            _ => {}
        }
        match current_block_29 {
            7056779235015430508 => {
                tmp___2 = offset;
                offset += 1;
                line_buffer[tmp___2 as usize] = c as libc::c_uchar;
            }
            _ => {}
        }
        i += 1;
    }
    if offset > start_offset {
        if !(offset
            < ::std::mem::size_of::<[libc::c_uchar; 1024]>() as libc::c_ulong
                as libc::c_int)
        {
            __assert_fail(
                b"offset < (int)sizeof(line_buffer)\0" as *const u8
                    as *const libc::c_char,
                b"lib/Remotery.c\0" as *const u8 as *const libc::c_char,
                7276 as libc::c_uint,
                b"_rmt_LogText\0" as *const u8 as *const libc::c_char,
            );
        }
        QueueLine(
            (*g_Remotery).mq_to_rmt_thread,
            line_buffer.as_mut_ptr(),
            offset as rmtU32,
            thread_profiler,
        );
    }
}
pub unsafe extern "C" fn _rmt_BeginCPUSample(
    mut name: rmtPStr,
    mut flags: rmtU32,
    mut hash_cache: *mut rmtU32,
) {
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    let mut sample: *mut Sample = 0 as *mut Sample;
    let mut name_hash: rmtU32 = 0;
    let mut tmp: rmtU32 = 0;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    let mut tmp___1: rmtError = RMT_ERROR_NONE;
    if g_Remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    tmp___1 = ThreadProfilers_GetCurrentThreadProfiler(
        (*g_Remotery).threadProfilers,
        &mut thread_profiler,
    );
    if tmp___1 as libc::c_uint == 0 as libc::c_uint {
        tmp = ThreadProfiler_GetNameHash(
            thread_profiler,
            (*g_Remotery).mq_to_rmt_thread,
            name,
            hash_cache,
        );
        name_hash = tmp;
        tmp___0 = ThreadProfiler_Push(
            (*thread_profiler).sampleTrees[0 as libc::c_int as usize],
            name_hash,
            flags,
            &mut sample,
        );
        if tmp___0 as libc::c_uint == 0 as libc::c_uint {
            if (*sample).call_count > 1 as libc::c_uint {
                (*sample).us_end = usTimer_Get(&mut (*g_Remotery).timer);
            } else {
                (*sample).us_start = usTimer_Get(&mut (*g_Remotery).timer);
            }
        }
    }
}
pub unsafe extern "C" fn _rmt_EndCPUSample() {
    let mut thread_profiler: *mut ThreadProfiler = 0 as *mut ThreadProfiler;
    let mut sample: *mut Sample = 0 as *mut Sample;
    let mut us_end: rmtU64 = 0;
    let mut tmp: rmtU64 = 0;
    let mut tmp___0: rmtError = RMT_ERROR_NONE;
    if g_Remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    tmp___0 = ThreadProfilers_GetCurrentThreadProfiler(
        (*g_Remotery).threadProfilers,
        &mut thread_profiler,
    );
    if tmp___0 as libc::c_uint == 0 as libc::c_uint {
        sample = (*(*thread_profiler).sampleTrees[0 as libc::c_int as usize])
            .currentParent;
        if (*sample).recurse_depth as libc::c_int > 0 as libc::c_int {
            (*sample)
                .recurse_depth = ((*sample).recurse_depth as libc::c_int
                - 1 as libc::c_int) as rmtU16;
        } else {
            tmp = usTimer_Get(&mut (*g_Remotery).timer);
            us_end = tmp;
            Sample_Close(sample, us_end as rmtS64);
            ThreadProfiler_Pop(
                thread_profiler,
                (*g_Remotery).mq_to_rmt_thread,
                sample,
                0 as libc::c_int as rmtU32,
            );
        }
    }
}
pub unsafe extern "C" fn _rmt_MarkFrame() -> rmtError {
    if g_Remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_REMOTERY_NOT_CREATED;
    }
    return RMT_ERROR_NONE;
}
pub unsafe extern "C" fn _rmt_IterateChildren(
    mut iterator: *mut rmtSampleIterator,
    mut sample: *mut rmtSample,
) {
    (*iterator).sample = 0 as *mut rmtSample;
    if sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*iterator).initial = (*sample).first_child;
    } else {
        (*iterator).initial = 0 as *mut rmtSample;
    };
}
pub unsafe extern "C" fn _rmt_IterateNext(mut iter: *mut rmtSampleIterator) -> rmtBool {
    let mut tmp: rmtBool = 0;
    if (*iter).initial as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*iter).sample = (*iter).initial;
        (*iter).initial = 0 as *mut rmtSample;
    } else if (*iter).sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong
        {
        (*iter).sample = (*(*iter).sample).next_sibling;
    }
    if (*iter).sample as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = 1 as libc::c_int as rmtBool;
    } else {
        tmp = 0 as libc::c_int as rmtBool;
    }
    return tmp;
}
pub unsafe extern "C" fn _rmt_SampleTreeGetThreadName(
    mut sample_tree: *mut rmtSampleTree,
) -> *const libc::c_char {
    return (*sample_tree).threadName;
}
pub unsafe extern "C" fn _rmt_SampleTreeGetRootSample(
    mut sample_tree: *mut rmtSampleTree,
) -> *mut rmtSample {
    return (*sample_tree).rootSample;
}
pub unsafe extern "C" fn _rmt_SampleGetName(
    mut sample: *mut rmtSample,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: rmtPStr = 0 as *const libc::c_char;
    tmp = StringTable_Find((*g_Remotery).string_table, (*sample).name_hash);
    name = tmp;
    if name as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return b"null\0" as *const u8 as *const libc::c_char;
    }
    return name;
}
pub unsafe extern "C" fn _rmt_SampleGetNameHash(mut sample: *mut rmtSample) -> rmtU32 {
    return (*sample).name_hash;
}
pub unsafe extern "C" fn _rmt_SampleGetCallCount(mut sample: *mut rmtSample) -> rmtU32 {
    return (*sample).call_count;
}
pub unsafe extern "C" fn _rmt_SampleGetStart(mut sample: *mut rmtSample) -> rmtU64 {
    return (*sample).us_start;
}
pub unsafe extern "C" fn _rmt_SampleGetTime(mut sample: *mut rmtSample) -> rmtU64 {
    return (*sample).us_length;
}
pub unsafe extern "C" fn _rmt_SampleGetSelfTime(mut sample: *mut rmtSample) -> rmtU64 {
    let mut tmp: rmtS64 = 0;
    tmp = maxS64(
        ((*sample).us_length).wrapping_sub((*sample).us_sampled_length) as rmtS64,
        0 as libc::c_int as rmtS64,
    );
    return tmp as rmtU64;
}
pub unsafe extern "C" fn _rmt_SampleGetType(
    mut sample: *mut rmtSample,
) -> rmtSampleType {
    return (*sample).type_0;
}
pub unsafe extern "C" fn _rmt_SampleGetColour(
    mut sample: *mut rmtSample,
    mut r: *mut rmtU8,
    mut g: *mut rmtU8,
    mut b: *mut rmtU8,
) {
    *r = (*sample).uniqueColour[0 as libc::c_int as usize];
    *g = (*sample).uniqueColour[1 as libc::c_int as usize];
    *b = (*sample).uniqueColour[2 as libc::c_int as usize];
}
pub unsafe extern "C" fn _rmt_PropertyIterateChildren(
    mut iterator: *mut rmtPropertyIterator,
    mut property: *mut rmtProperty,
) {
    (*iterator).property = 0 as *mut rmtProperty;
    if property as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*iterator).initial = (*property).firstChild;
    } else {
        (*iterator).initial = 0 as *mut rmtProperty;
    };
}
pub unsafe extern "C" fn _rmt_PropertyIterateNext(
    mut iter: *mut rmtPropertyIterator,
) -> rmtBool {
    let mut tmp: rmtBool = 0;
    if (*iter).initial as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*iter).property = (*iter).initial;
        (*iter).initial = 0 as *mut rmtProperty;
    } else if (*iter).property as libc::c_ulong
            != 0 as *mut libc::c_void as libc::c_ulong
        {
        (*iter).property = (*(*iter).property).nextSibling;
    }
    if (*iter).property as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp = 1 as libc::c_int as rmtBool;
    } else {
        tmp = 0 as libc::c_int as rmtBool;
    }
    return tmp;
}
pub unsafe extern "C" fn _rmt_PropertyGetName(
    mut property: *mut rmtProperty,
) -> *const libc::c_char {
    return (*property).name;
}
pub unsafe extern "C" fn _rmt_PropertyGetDescription(
    mut property: *mut rmtProperty,
) -> *const libc::c_char {
    return (*property).description;
}
pub unsafe extern "C" fn _rmt_PropertyGetNameHash(
    mut property: *mut rmtProperty,
) -> rmtU32 {
    return (*property).nameHash;
}
pub unsafe extern "C" fn _rmt_PropertyGetType(
    mut property: *mut rmtProperty,
) -> rmtPropertyType {
    return (*property).type_0;
}
pub unsafe extern "C" fn _rmt_PropertyGetValue(
    mut property: *mut rmtProperty,
) -> rmtPropertyValue {
    return (*property).value;
}
unsafe extern "C" fn RegisterProperty(
    mut property: *mut rmtProperty,
    mut can_lock: rmtBool,
) {
    let mut name_len: rmtU32 = 0;
    let mut parent_property: *mut rmtProperty = 0 as *mut rmtProperty;
    if (*property).initialised == 0 as libc::c_uint {
        if can_lock != 0 {
            mtxLock(&mut (*g_Remotery).propertyMutex);
        }
        if (*property).initialised == 0 as libc::c_uint {
            parent_property = (*property).parent;
            if parent_property as libc::c_ulong
                == 0 as *mut libc::c_void as libc::c_ulong
            {
                (*property).parent = &mut (*g_Remotery).rootProperty;
                parent_property = (*property).parent;
            }
            RegisterProperty(parent_property, 0 as libc::c_int as rmtBool);
            if (*parent_property).firstChild as libc::c_ulong
                != 0 as *mut libc::c_void as libc::c_ulong
            {
                (*(*parent_property).lastChild).nextSibling = property;
                (*parent_property).lastChild = property;
            } else {
                (*parent_property).firstChild = property;
                (*parent_property).lastChild = property;
            }
            name_len = strnlen_s_safe_c(
                (*property).name,
                256 as libc::c_int as r_size_t,
            );
            (*property)
                .nameHash = _rmt_HashString32(
                (*property).name,
                name_len as libc::c_int,
                0 as libc::c_int as rmtU32,
            );
            QueueAddToStringTable(
                (*g_Remotery).mq_to_rmt_thread,
                (*property).nameHash,
                (*property).name,
                name_len as size_t,
                0 as *mut libc::c_void as *mut ThreadProfiler,
            );
            (*property).uniqueID = (*parent_property).uniqueID;
            (*property)
                .uniqueID = HashCombine((*property).uniqueID, (*property).nameHash);
            (*property).initialised = 1 as libc::c_int as rmtBool;
        }
        if can_lock != 0 {
            mtxUnlock(&mut (*g_Remotery).propertyMutex);
        }
    }
}
pub unsafe extern "C" fn _rmt_PropertySetValue(mut property: *mut rmtProperty) {
    if g_Remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    RegisterProperty(property, 1 as libc::c_int as rmtBool);
}
pub unsafe extern "C" fn _rmt_PropertyAddValue(
    mut property: *mut rmtProperty,
    mut add_value: rmtPropertyValue,
) {
    if g_Remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    RegisterProperty(property, 1 as libc::c_int as rmtBool);
}
unsafe extern "C" fn TakePropertySnapshot(
    mut property: *mut rmtProperty,
    mut parent_snapshot: *mut PropertySnapshot,
    mut first_snapshot: *mut *mut PropertySnapshot,
    mut prev_snapshot: *mut *mut PropertySnapshot,
    mut depth: rmtU32,
) -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut child_property: *mut rmtProperty = 0 as *mut rmtProperty;
    let mut snapshot: *mut PropertySnapshot = 0 as *mut PropertySnapshot;
    error = ObjectAllocator_Alloc(
        (*g_Remotery).propertyAllocator,
        &mut snapshot as *mut *mut PropertySnapshot as *mut *mut libc::c_void,
    );
    if error as libc::c_uint != 0 as libc::c_uint {
        return error;
    }
    (*snapshot).type_0 = (*property).type_0;
    (*snapshot).value = (*property).value;
    (*snapshot).prevValue = (*property).prevValue;
    (*snapshot).prevValueFrame = (*property).prevValueFrame;
    (*snapshot).nameHash = (*property).nameHash;
    (*snapshot).uniqueID = (*property).uniqueID;
    (*snapshot).nbChildren = 0 as libc::c_int as rmtU32;
    (*snapshot).depth = depth as rmtU8;
    (*snapshot).nextSnapshot = 0 as *mut libc::c_void as *mut PropertySnapshot;
    if parent_snapshot as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (*parent_snapshot).nbChildren = ((*parent_snapshot).nbChildren).wrapping_add(1);
    }
    if *first_snapshot as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        *first_snapshot = snapshot;
    }
    if *prev_snapshot as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        (**prev_snapshot).nextSnapshot = snapshot;
    }
    *prev_snapshot = snapshot;
    child_property = (*property).firstChild;
    while child_property as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        error = TakePropertySnapshot(
            child_property,
            snapshot,
            first_snapshot,
            prev_snapshot,
            depth.wrapping_add(1 as libc::c_uint),
        );
        if error as libc::c_uint != 0 as libc::c_uint {
            return error;
        }
        child_property = (*child_property).nextSibling;
    }
    return RMT_ERROR_NONE;
}
pub unsafe extern "C" fn _rmt_PropertySnapshotAll() -> rmtError {
    let mut error: rmtError = RMT_ERROR_NONE;
    let mut first_snapshot: *mut PropertySnapshot = 0 as *mut PropertySnapshot;
    let mut prev_snapshot: *mut PropertySnapshot = 0 as *mut PropertySnapshot;
    let mut payload: *mut Msg_PropertySnapshot = 0 as *mut Msg_PropertySnapshot;
    let mut message: *mut Message = 0 as *mut Message;
    let mut nb_snapshot_allocs: rmtU32 = 0;
    if g_Remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return RMT_ERROR_REMOTERY_NOT_CREATED;
    }
    if (*g_Remotery).rootProperty.firstChild as libc::c_ulong
        == 0 as *mut libc::c_void as libc::c_ulong
    {
        return RMT_ERROR_NONE;
    }
    nb_snapshot_allocs = (*(*g_Remotery).propertyAllocator).nb_inuse as rmtU32;
    first_snapshot = 0 as *mut libc::c_void as *mut PropertySnapshot;
    prev_snapshot = 0 as *mut libc::c_void as *mut PropertySnapshot;
    mtxLock(&mut (*g_Remotery).propertyMutex);
    error = TakePropertySnapshot(
        &mut (*g_Remotery).rootProperty,
        0 as *mut libc::c_void as *mut PropertySnapshot,
        &mut first_snapshot,
        &mut prev_snapshot,
        0 as libc::c_int as rmtU32,
    );
    if ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut libc::c_void, *mut rmtProperty) -> ()>,
        libc::c_ulong,
    >(g_Settings.snapshot_callback) != 0 as *mut libc::c_void as libc::c_ulong
    {
        (Some((g_Settings.snapshot_callback).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(g_Settings.snapshot_context, &mut (*g_Remotery).rootProperty);
    }
    mtxUnlock(&mut (*g_Remotery).propertyMutex);
    if error as libc::c_uint != 0 as libc::c_uint {
        FreePropertySnapshots(first_snapshot);
        return error;
    }
    message = rmtMessageQueue_AllocMessage(
        (*g_Remotery).mq_to_rmt_thread,
        ::std::mem::size_of::<Msg_PropertySnapshot>() as libc::c_ulong as rmtU32,
        0 as *mut libc::c_void as *mut ThreadProfiler,
    );
    if message as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        FreePropertySnapshots(first_snapshot);
        return RMT_ERROR_UNKNOWN;
    }
    payload = ((*message).payload).as_mut_ptr() as *mut Msg_PropertySnapshot;
    (*payload).rootSnapshot = first_snapshot;
    (*payload)
        .nbSnapshots = ((*(*g_Remotery).propertyAllocator).nb_inuse as rmtU32)
        .wrapping_sub(nb_snapshot_allocs);
    (*payload).propertyFrame = (*g_Remotery).propertyFrame;
    rmtMessageQueue_CommitMessage(message, MsgID_PropertySnapshot);
    return RMT_ERROR_NONE;
}
unsafe extern "C" fn PropertyFrameReset(
    mut rmt: *mut Remotery,
    mut first_property: *mut rmtProperty,
) {
    let mut property: *mut rmtProperty = 0 as *mut rmtProperty;
    let mut changed: rmtBool = 0;
    property = first_property;
    while property as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        PropertyFrameReset(rmt, (*property).firstChild);
        changed = 0 as libc::c_int as rmtBool;
        match (*property).type_0 as libc::c_uint {
            1 => {
                changed = ((*property).lastFrameValue.Bool != (*property).value.Bool)
                    as libc::c_int as rmtBool;
            }
            4 | 3 | 2 => {
                changed = ((*property).lastFrameValue.U32 != (*property).value.U32)
                    as libc::c_int as rmtBool;
            }
            7 | 6 | 5 => {
                changed = ((*property).lastFrameValue.U64 != (*property).value.U64)
                    as libc::c_int as rmtBool;
            }
            0 | _ => {}
        }
        if changed != 0 {
            (*property).prevValue = (*property).lastFrameValue;
            (*property).prevValueFrame = (*rmt).propertyFrame;
        }
        (*property).lastFrameValue = (*property).value;
        if (*property).flags as libc::c_uint & 1 as libc::c_uint != 0 as libc::c_uint {
            (*property).value = (*property).defaultValue;
        }
        property = (*property).nextSibling;
    }
}
pub unsafe extern "C" fn _rmt_PropertyFrameResetAll() {
    if g_Remotery as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return;
    }
    mtxLock(&mut (*g_Remotery).propertyMutex);
    PropertyFrameReset(g_Remotery, (*g_Remotery).rootProperty.firstChild);
    mtxUnlock(&mut (*g_Remotery).propertyMutex);
    (*g_Remotery).propertyFrame = ((*g_Remotery).propertyFrame).wrapping_add(1);
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
