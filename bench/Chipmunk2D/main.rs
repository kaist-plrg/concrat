use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(
        _: *mut FILE,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn abort() -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
    );
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
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
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> libc::c_int;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
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
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type va_list___0 = __gnuc_va_list;
pub type cpFloat = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpVect {
    pub x: cpFloat,
    pub y: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpBB {
    pub l: cpFloat,
    pub b: cpFloat,
    pub r: cpFloat,
    pub t: cpFloat,
}
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
pub type cpHashValue = uintptr_t;
pub type cpCollisionID = uint32_t;
pub type cpBool = libc::c_uchar;
pub type cpDataPointer = *mut libc::c_void;
pub type cpCollisionType = uintptr_t;
pub type cpGroup = uintptr_t;
pub type cpBitmask = libc::c_uint;
pub type cpTimestamp = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpTransform {
    pub a: cpFloat,
    pub b: cpFloat,
    pub c: cpFloat,
    pub d: cpFloat,
    pub tx: cpFloat,
    pub ty: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpArray {
    pub num: libc::c_int,
    pub max: libc::c_int,
    pub arr: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpHashSet {
    pub entries: libc::c_uint,
    pub size: libc::c_uint,
    pub eql: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cpBool,
    >,
    pub default_value: *mut libc::c_void,
    pub table: *mut *mut cpHashSetBin,
    pub pooledBins: *mut cpHashSetBin,
    pub allocatedBuffers: *mut cpArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpHashSetBin {
    pub elt: *mut libc::c_void,
    pub hash: cpHashValue,
    pub next: *mut cpHashSetBin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpBody {
    pub velocity_func: Option::<
        unsafe extern "C" fn(*mut cpBody, cpVect, cpFloat, cpFloat) -> (),
    >,
    pub position_func: Option::<unsafe extern "C" fn(*mut cpBody, cpFloat) -> ()>,
    pub m: cpFloat,
    pub m_inv: cpFloat,
    pub i: cpFloat,
    pub i_inv: cpFloat,
    pub cog: cpVect,
    pub p: cpVect,
    pub v: cpVect,
    pub f: cpVect,
    pub a: cpFloat,
    pub w: cpFloat,
    pub t: cpFloat,
    pub transform: cpTransform,
    pub userData: cpDataPointer,
    pub v_bias: cpVect,
    pub w_bias: cpFloat,
    pub space: *mut cpSpace,
    pub shapeList: *mut cpShape,
    pub arbiterList: *mut cpArbiter,
    pub constraintList: *mut cpConstraint,
    pub sleeping: __anonstruct_sleeping_590565998,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_sleeping_590565998 {
    pub root: *mut cpBody,
    pub next: *mut cpBody,
    pub idleTime: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpConstraint {
    pub klass: *const cpConstraintClass,
    pub space: *mut cpSpace,
    pub a: *mut cpBody,
    pub b: *mut cpBody,
    pub next_a: *mut cpConstraint,
    pub next_b: *mut cpConstraint,
    pub maxForce: cpFloat,
    pub errorBias: cpFloat,
    pub maxBias: cpFloat,
    pub collideBodies: cpBool,
    pub preSolve: Option::<unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> ()>,
    pub postSolve: Option::<unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> ()>,
    pub userData: cpDataPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpace {
    pub iterations: libc::c_int,
    pub gravity: cpVect,
    pub damping: cpFloat,
    pub idleSpeedThreshold: cpFloat,
    pub sleepTimeThreshold: cpFloat,
    pub collisionSlop: cpFloat,
    pub collisionBias: cpFloat,
    pub collisionPersistence: cpTimestamp,
    pub userData: cpDataPointer,
    pub stamp: cpTimestamp,
    pub curr_dt: cpFloat,
    pub dynamicBodies: *mut cpArray,
    pub staticBodies: *mut cpArray,
    pub rousedBodies: *mut cpArray,
    pub sleepingComponents: *mut cpArray,
    pub shapeIDCounter: cpHashValue,
    pub staticShapes: *mut cpSpatialIndex,
    pub dynamicShapes: *mut cpSpatialIndex,
    pub constraints: *mut cpArray,
    pub arbiters: *mut cpArray,
    pub contactBuffersHead: *mut cpContactBufferHeader,
    pub cachedArbiters: *mut cpHashSet,
    pub pooledArbiters: *mut cpArray,
    pub allocatedBuffers: *mut cpArray,
    pub locked: libc::c_int,
    pub usesWildcards: cpBool,
    pub collisionHandlers: *mut cpHashSet,
    pub defaultHandler: cpCollisionHandler,
    pub skipPostStep: cpBool,
    pub postStepCallbacks: *mut cpArray,
    pub staticBody: *mut cpBody,
    pub _staticBody: cpBody,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpCollisionHandler {
    pub typeA: cpCollisionType,
    pub typeB: cpCollisionType,
    pub beginFunc: Option::<
        unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> cpBool,
    >,
    pub preSolveFunc: Option::<
        unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> cpBool,
    >,
    pub postSolveFunc: Option::<
        unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> (),
    >,
    pub separateFunc: Option::<
        unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> (),
    >,
    pub userData: cpDataPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpArbiter {
    pub e: cpFloat,
    pub u: cpFloat,
    pub surface_vr: cpVect,
    pub data: cpDataPointer,
    pub a: *const cpShape,
    pub b: *const cpShape,
    pub body_a: *mut cpBody,
    pub body_b: *mut cpBody,
    pub thread_a: cpArbiterThread,
    pub thread_b: cpArbiterThread,
    pub count: libc::c_int,
    pub contacts: *mut cpContact,
    pub n: cpVect,
    pub handler: *mut cpCollisionHandler,
    pub handlerA: *mut cpCollisionHandler,
    pub handlerB: *mut cpCollisionHandler,
    pub swapped: cpBool,
    pub stamp: cpTimestamp,
    pub state: cpArbiterState,
}
pub type cpArbiterState = libc::c_uint;
pub const CP_ARBITER_STATE_INVALIDATED: cpArbiterState = 4;
pub const CP_ARBITER_STATE_CACHED: cpArbiterState = 3;
pub const CP_ARBITER_STATE_IGNORE: cpArbiterState = 2;
pub const CP_ARBITER_STATE_NORMAL: cpArbiterState = 1;
pub const CP_ARBITER_STATE_FIRST_COLLISION: cpArbiterState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpContact {
    pub r1: cpVect,
    pub r2: cpVect,
    pub nMass: cpFloat,
    pub tMass: cpFloat,
    pub bounce: cpFloat,
    pub jnAcc: cpFloat,
    pub jtAcc: cpFloat,
    pub jBias: cpFloat,
    pub bias: cpFloat,
    pub hash: cpHashValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpArbiterThread {
    pub next: *mut cpArbiter,
    pub prev: *mut cpArbiter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpShape {
    pub klass: *const cpShapeClass,
    pub space: *mut cpSpace,
    pub body: *mut cpBody,
    pub massInfo: cpShapeMassInfo,
    pub bb: cpBB,
    pub sensor: cpBool,
    pub e: cpFloat,
    pub u: cpFloat,
    pub surfaceV: cpVect,
    pub userData: cpDataPointer,
    pub type_0: cpCollisionType,
    pub filter: cpShapeFilter,
    pub next: *mut cpShape,
    pub prev: *mut cpShape,
    pub hashid: cpHashValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpShapeFilter {
    pub group: cpGroup,
    pub categories: cpBitmask,
    pub mask: cpBitmask,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpShapeMassInfo {
    pub m: cpFloat,
    pub i: cpFloat,
    pub cog: cpVect,
    pub area: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpShapeClass {
    pub type_0: cpShapeType,
    pub cacheData: Option::<unsafe extern "C" fn(*mut cpShape, cpTransform) -> cpBB>,
    pub destroy: Option::<unsafe extern "C" fn(*mut cpShape) -> ()>,
    pub pointQuery: Option::<
        unsafe extern "C" fn(*const cpShape, cpVect, *mut cpPointQueryInfo) -> (),
    >,
    pub segmentQuery: Option::<
        unsafe extern "C" fn(
            *const cpShape,
            cpVect,
            cpVect,
            cpFloat,
            *mut cpSegmentQueryInfo,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSegmentQueryInfo {
    pub shape: *const cpShape,
    pub point: cpVect,
    pub normal: cpVect,
    pub alpha: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPointQueryInfo {
    pub shape: *const cpShape,
    pub point: cpVect,
    pub distance: cpFloat,
    pub gradient: cpVect,
}
pub type cpShapeType = libc::c_uint;
pub const CP_NUM_SHAPES: cpShapeType = 3;
pub const CP_POLY_SHAPE: cpShapeType = 2;
pub const CP_SEGMENT_SHAPE: cpShapeType = 1;
pub const CP_CIRCLE_SHAPE: cpShapeType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpContactBufferHeader {
    pub stamp: cpTimestamp,
    pub next: *mut cpContactBufferHeader,
    pub numContacts: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpatialIndex {
    pub klass: *mut cpSpatialIndexClass,
    pub bbfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
    pub staticIndex: *mut cpSpatialIndex,
    pub dynamicIndex: *mut cpSpatialIndex,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpatialIndexClass {
    pub destroy: Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> ()>,
    pub count: Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> libc::c_int>,
    pub each: Option::<
        unsafe extern "C" fn(
            *mut cpSpatialIndex,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
            *mut libc::c_void,
        ) -> (),
    >,
    pub contains: Option::<
        unsafe extern "C" fn(
            *mut cpSpatialIndex,
            *mut libc::c_void,
            cpHashValue,
        ) -> cpBool,
    >,
    pub insert: Option::<
        unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
    >,
    pub remove: Option::<
        unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
    >,
    pub reindex: Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> ()>,
    pub reindexObject: Option::<
        unsafe extern "C" fn(*mut cpSpatialIndex, *mut libc::c_void, cpHashValue) -> (),
    >,
    pub reindexQuery: Option::<
        unsafe extern "C" fn(
            *mut cpSpatialIndex,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
            *mut libc::c_void,
        ) -> (),
    >,
    pub query: Option::<
        unsafe extern "C" fn(
            *mut cpSpatialIndex,
            *mut libc::c_void,
            cpBB,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
            *mut libc::c_void,
        ) -> (),
    >,
    pub segmentQuery: Option::<
        unsafe extern "C" fn(
            *mut cpSpatialIndex,
            *mut libc::c_void,
            cpVect,
            cpVect,
            cpFloat,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> cpFloat,
            >,
            *mut libc::c_void,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpConstraintClass {
    pub preStep: Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
    pub applyCachedImpulse: Option::<
        unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> (),
    >,
    pub applyImpulse: Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
    pub getImpulse: Option::<unsafe extern "C" fn(*mut cpConstraint) -> cpFloat>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpContactPointSet {
    pub count: libc::c_int,
    pub normal: cpVect,
    pub points: [__anonstruct_points_450528349; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_points_450528349 {
    pub pointA: cpVect,
    pub pointB: cpVect,
    pub distance: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpCollisionInfo {
    pub a: *const cpShape,
    pub b: *const cpShape,
    pub id: cpCollisionID,
    pub n: cpVect,
    pub count: libc::c_int,
    pub arr: *mut cpContact,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpBBTree {
    pub spatialIndex: cpSpatialIndex,
    pub velocityFunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpVect>,
    pub leaves: *mut cpHashSet,
    pub root: *mut Node,
    pub pooledNodes: *mut Node,
    pub pooledPairs: *mut Pair,
    pub allocatedBuffers: *mut cpArray,
    pub stamp: cpTimestamp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pair {
    pub a: Thread,
    pub b: Thread,
    pub id: cpCollisionID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Thread {
    pub prev: *mut Pair,
    pub leaf: *mut Node,
    pub next: *mut Pair,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub obj: *mut libc::c_void,
    pub bb: cpBB,
    pub parent: *mut Node,
    pub node: __anonunion_node_125920227,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion_node_125920227 {
    pub children: __anonstruct_children_590640126,
    pub leaf: __anonstruct_leaf_219122021,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_leaf_219122021 {
    pub stamp: cpTimestamp,
    pub pairs: *mut Pair,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_children_590640126 {
    pub a: *mut Node,
    pub b: *mut Node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MarkContext {
    pub tree: *mut cpBBTree,
    pub staticRoot: *mut Node,
    pub func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct eachContext {
    pub func: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    pub data: *mut libc::c_void,
}
pub type cpBodyType = libc::c_uint;
pub const CP_BODY_TYPE_STATIC: cpBodyType = 2;
pub const CP_BODY_TYPE_KINEMATIC: cpBodyType = 1;
pub const CP_BODY_TYPE_DYNAMIC: cpBodyType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpCircleShape {
    pub shape: cpShape,
    pub c: cpVect,
    pub tc: cpVect,
    pub r: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSegmentShape {
    pub shape: cpShape,
    pub a: cpVect,
    pub b: cpVect,
    pub n: cpVect,
    pub ta: cpVect,
    pub tb: cpVect,
    pub tn: cpVect,
    pub r: cpFloat,
    pub a_tangent: cpVect,
    pub b_tangent: cpVect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPolyShape {
    pub shape: cpShape,
    pub r: cpFloat,
    pub count: libc::c_int,
    pub planes: *mut cpSplittingPlane,
    pub _planes: [cpSplittingPlane; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSplittingPlane {
    pub v0: cpVect,
    pub n: cpVect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SupportPoint {
    pub p: cpVect,
    pub index: cpCollisionID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MinkowskiPoint {
    pub a: cpVect,
    pub b: cpVect,
    pub ab: cpVect,
    pub id: cpCollisionID,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SupportContext {
    pub shape1: *const cpShape,
    pub shape2: *const cpShape,
    pub func1: Option::<unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint>,
    pub func2: Option::<unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgePoint {
    pub p: cpVect,
    pub hash: cpHashValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Edge {
    pub a: EdgePoint,
    pub b: EdgePoint,
    pub r: cpFloat,
    pub n: cpVect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClosestPoints {
    pub a: cpVect,
    pub b: cpVect,
    pub n: cpVect,
    pub d: cpFloat,
    pub id: cpCollisionID,
}
pub type CollisionFunc = Option::<
    unsafe extern "C" fn(*const cpShape, *const cpShape, *mut cpCollisionInfo) -> (),
>;
pub type cpConstraintPreSolveFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
>;
pub type cpConstraintPostSolveFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpDampedRotarySpring {
    pub constraint: cpConstraint,
    pub restAngle: cpFloat,
    pub stiffness: cpFloat,
    pub damping: cpFloat,
    pub springTorqueFunc: Option::<
        unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat,
    >,
    pub target_wrn: cpFloat,
    pub w_coef: cpFloat,
    pub iSum: cpFloat,
    pub jAcc: cpFloat,
}
pub type cpDampedRotarySpringTorqueFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpDampedSpring {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub restLength: cpFloat,
    pub stiffness: cpFloat,
    pub damping: cpFloat,
    pub springForceFunc: Option::<
        unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat,
    >,
    pub target_vrn: cpFloat,
    pub v_coef: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub nMass: cpFloat,
    pub n: cpVect,
    pub jAcc: cpFloat,
}
pub type cpDampedSpringForceFunc = Option::<
    unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpGearJoint {
    pub constraint: cpConstraint,
    pub phase: cpFloat,
    pub ratio: cpFloat,
    pub ratio_inv: cpFloat,
    pub iSum: cpFloat,
    pub bias: cpFloat,
    pub jAcc: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpMat2x2 {
    pub a: cpFloat,
    pub b: cpFloat,
    pub c: cpFloat,
    pub d: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpGrooveJoint {
    pub constraint: cpConstraint,
    pub grv_n: cpVect,
    pub grv_a: cpVect,
    pub grv_b: cpVect,
    pub anchorB: cpVect,
    pub grv_tn: cpVect,
    pub clamp: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub k: cpMat2x2,
    pub jAcc: cpVect,
    pub bias: cpVect,
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
pub union __anonunion____missing_field_name_886645174 {
    pub __wseq: libc::c_ulonglong,
    pub __wseq32: __anonstruct___wseq32_112954846,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct___g1_start32_112954847 {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __anonunion____missing_field_name_54864270 {
    pub __g1_start: libc::c_ulonglong,
    pub __g1_start32: __anonstruct___g1_start32_112954847,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __annonCompField1: __anonunion____missing_field_name_886645174,
    pub __annonCompField2: __anonunion____missing_field_name_54864270,
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
pub struct cpHastySpace {
    pub space: cpSpace,
    pub num_threads: libc::c_ulong,
    pub num_working: libc::c_ulong,
    pub constraint_count_threshold: libc::c_ulong,
    pub mutex: pthread_mutex_t,
    pub cond_work: pthread_cond_t,
    pub cond_resume: pthread_cond_t,
    pub work: Option::<
        unsafe extern "C" fn(*mut cpSpace, libc::c_ulong, libc::c_ulong) -> (),
    >,
    pub workers: [ThreadContext; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadContext {
    pub thread: pthread_t,
    pub space: *mut cpHastySpace,
    pub thread_num: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPinJoint {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub dist: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub n: cpVect,
    pub nMass: cpFloat,
    pub jnAcc: cpFloat,
    pub bias: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPivotJoint {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub r1: cpVect,
    pub r2: cpVect,
    pub k: cpMat2x2,
    pub jAcc: cpVect,
    pub bias: cpVect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPolyline {
    pub count: libc::c_int,
    pub capacity: libc::c_int,
    pub verts: [cpVect; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPolylineSet {
    pub count: libc::c_int,
    pub capacity: libc::c_int,
    pub lines: *mut *mut cpPolyline,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Notch {
    pub i: libc::c_int,
    pub d: cpFloat,
    pub v: cpVect,
    pub n: cpVect,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpRatchetJoint {
    pub constraint: cpConstraint,
    pub angle: cpFloat,
    pub phase: cpFloat,
    pub ratchet: cpFloat,
    pub iSum: cpFloat,
    pub bias: cpFloat,
    pub jAcc: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpRotaryLimitJoint {
    pub constraint: cpConstraint,
    pub min: cpFloat,
    pub max: cpFloat,
    pub iSum: cpFloat,
    pub bias: cpFloat,
    pub jAcc: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSimpleMotor {
    pub constraint: cpConstraint,
    pub rate: cpFloat,
    pub iSum: cpFloat,
    pub jAcc: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSlideJoint {
    pub constraint: cpConstraint,
    pub anchorA: cpVect,
    pub anchorB: cpVect,
    pub min: cpFloat,
    pub max: cpFloat,
    pub r1: cpVect,
    pub r2: cpVect,
    pub n: cpVect,
    pub nMass: cpFloat,
    pub jnAcc: cpFloat,
    pub bias: cpFloat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct arbiterFilterContext {
    pub space: *mut cpSpace,
    pub body: *mut cpBody,
    pub shape: *mut cpShape,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spaceShapeContext {
    pub func: Option::<unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> ()>,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceDebugColor {
    pub r: libc::c_float,
    pub g: libc::c_float,
    pub b: libc::c_float,
    pub a: libc::c_float,
}
pub type cpSpaceDebugDrawFlags = libc::c_uint;
pub const CP_SPACE_DEBUG_DRAW_COLLISION_POINTS: cpSpaceDebugDrawFlags = 4;
pub const CP_SPACE_DEBUG_DRAW_CONSTRAINTS: cpSpaceDebugDrawFlags = 2;
pub const CP_SPACE_DEBUG_DRAW_SHAPES: cpSpaceDebugDrawFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceDebugDrawOptions {
    pub drawCircle: Option::<
        unsafe extern "C" fn(
            cpVect,
            cpFloat,
            cpFloat,
            cpSpaceDebugColor,
            cpSpaceDebugColor,
            cpDataPointer,
        ) -> (),
    >,
    pub drawSegment: Option::<
        unsafe extern "C" fn(cpVect, cpVect, cpSpaceDebugColor, cpDataPointer) -> (),
    >,
    pub drawFatSegment: Option::<
        unsafe extern "C" fn(
            cpVect,
            cpVect,
            cpFloat,
            cpSpaceDebugColor,
            cpSpaceDebugColor,
            cpDataPointer,
        ) -> (),
    >,
    pub drawPolygon: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const cpVect,
            cpFloat,
            cpSpaceDebugColor,
            cpSpaceDebugColor,
            cpDataPointer,
        ) -> (),
    >,
    pub drawDot: Option::<
        unsafe extern "C" fn(cpFloat, cpVect, cpSpaceDebugColor, cpDataPointer) -> (),
    >,
    pub flags: cpSpaceDebugDrawFlags,
    pub shapeOutlineColor: cpSpaceDebugColor,
    pub colorForShape: Option::<
        unsafe extern "C" fn(*mut cpShape, cpDataPointer) -> cpSpaceDebugColor,
    >,
    pub constraintColor: cpSpaceDebugColor,
    pub collisionPointColor: cpSpaceDebugColor,
    pub data: cpDataPointer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceHash {
    pub spatialIndex: cpSpatialIndex,
    pub numcells: libc::c_int,
    pub celldim: cpFloat,
    pub table: *mut *mut cpSpaceHashBin,
    pub handleSet: *mut cpHashSet,
    pub pooledBins: *mut cpSpaceHashBin,
    pub pooledHandles: *mut cpArray,
    pub allocatedBuffers: *mut cpArray,
    pub stamp: cpTimestamp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSpaceHashBin {
    pub handle: *mut cpHandle,
    pub next: *mut cpSpaceHashBin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpHandle {
    pub obj: *mut libc::c_void,
    pub retain: libc::c_int,
    pub stamp: cpTimestamp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct queryRehashContext {
    pub hash: *mut cpSpaceHash,
    pub func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PointQueryContext {
    pub point: cpVect,
    pub maxDistance: cpFloat,
    pub filter: cpShapeFilter,
    pub func: Option::<
        unsafe extern "C" fn(
            *mut cpShape,
            cpVect,
            cpFloat,
            cpVect,
            *mut libc::c_void,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SegmentQueryContext {
    pub start: cpVect,
    pub end: cpVect,
    pub radius: cpFloat,
    pub filter: cpShapeFilter,
    pub func: Option::<
        unsafe extern "C" fn(
            *mut cpShape,
            cpVect,
            cpVect,
            cpFloat,
            *mut libc::c_void,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BBQueryContext {
    pub bb: cpBB,
    pub filter: cpShapeFilter,
    pub func: Option::<unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ShapeQueryContext {
    pub func: Option::<
        unsafe extern "C" fn(
            *mut cpShape,
            *mut cpContactPointSet,
            *mut libc::c_void,
        ) -> (),
    >,
    pub data: *mut libc::c_void,
    pub anyCollision: cpBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpPostStepCallback {
    pub func: Option::<
        unsafe extern "C" fn(*mut cpSpace, *mut libc::c_void, *mut libc::c_void) -> (),
    >,
    pub key: *mut libc::c_void,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpContactBuffer {
    pub header: cpContactBufferHeader,
    pub contacts: [cpContact; 341],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynamicToStaticContext {
    pub bbfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
    pub staticIndex: *mut cpSpatialIndex,
    pub queryFunc: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpSweep1D {
    pub spatialIndex: cpSpatialIndex,
    pub num: libc::c_int,
    pub max: libc::c_int,
    pub table: *mut TableCell,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableCell {
    pub obj: *mut libc::c_void,
    pub bounds: Bounds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Bounds {
    pub min: cpFloat,
    pub max: cpFloat,
}
#[inline]
unsafe extern "C" fn cpfabs(mut f: cpFloat) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    if f < 0 as libc::c_int as cpFloat {
        tmp = -f;
    } else {
        tmp = f;
    }
    return tmp;
}
static mut cpvzero: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpv(x: cpFloat, y: cpFloat) -> cpVect {
    let mut v: cpVect = cpVect { x: 0., y: 0. };
    v.x = x;
    v.y = y;
    return v;
}
#[inline]
unsafe extern "C" fn cpvadd(v1: cpVect, v2: cpVect) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(v1.x + v2.x, v1.y + v2.y);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpvsub(v1: cpVect, v2: cpVect) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(v1.x - v2.x, v1.y - v2.y);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpvmult(v: cpVect, s: cpFloat) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(v.x * s, v.y * s);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpvdot(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.x + v1.y * v2.y;
}
#[inline]
unsafe extern "C" fn cpvcross(v1: cpVect, v2: cpVect) -> cpFloat {
    return v1.x * v2.y - v1.y * v2.x;
}
#[inline]
unsafe extern "C" fn cpvlengthsq(v: cpVect) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpvdot(v, v);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpvlength(v: cpVect) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: libc::c_double = 0.;
    tmp = cpvdot(v, v);
    tmp___0 = sqrt(tmp);
    return tmp___0;
}
#[inline]
unsafe extern "C" fn cpvlerp(v1: cpVect, v2: cpVect, t: cpFloat) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvmult(v2, t);
    tmp___0 = cpvmult(v1, 1.0f32 as cpFloat - t);
    tmp___1 = cpvadd(tmp___0, tmp);
    return tmp___1;
}
#[inline]
unsafe extern "C" fn cpvdist(v1: cpVect, v2: cpVect) -> cpFloat {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpFloat = 0.;
    tmp = cpvsub(v1, v2);
    tmp___0 = cpvlength(tmp);
    return tmp___0;
}
pub unsafe extern "C" fn cpMessage(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut isError: libc::c_int,
    mut isHardError: libc::c_int,
    mut message: *const libc::c_char,
    mut args: ...
) {
    let mut tmp: *const libc::c_char = 0 as *const libc::c_char;
    let mut vargs: ::std::ffi::VaListImpl;
    if isError != 0 {
        tmp = b"Aborting due to Chipmunk error: \0" as *const u8 as *const libc::c_char;
    } else {
        tmp = b"Chipmunk warning: \0" as *const u8 as *const libc::c_char;
    }
    fprintf(stderr, tmp);
    vargs = args.clone();
    vfprintf(stderr, message, vargs.as_va_list());
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"\tFailed condition: %s\n\0" as *const u8 as *const libc::c_char,
        condition,
    );
    fprintf(
        stderr,
        b"\tSource:%s:%d\n\0" as *const u8 as *const libc::c_char,
        file,
        line,
    );
}
pub static mut cpVersionString: *const libc::c_char = b"7.0.3\0" as *const u8
    as *const libc::c_char;
pub unsafe extern "C" fn cpMomentForCircle(
    mut m: cpFloat,
    mut r1: cpFloat,
    mut r2: cpFloat,
    mut offset: cpVect,
) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpvlengthsq(offset);
    return m * (0.5f32 as cpFloat * (r1 * r1 + r2 * r2) + tmp);
}
pub unsafe extern "C" fn cpAreaForCircle(mut r1: cpFloat, mut r2: cpFloat) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpfabs(r1 * r1 - r2 * r2);
    return 3.14159265358979323846264338327950288f64 * tmp;
}
pub unsafe extern "C" fn cpMomentForSegment(
    mut m: cpFloat,
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
) -> cpFloat {
    let mut offset: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut length: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    tmp = cpvlerp(a, b, 0.5f32 as cpFloat);
    offset = tmp;
    tmp___0 = cpvdist(b, a);
    length = tmp___0 + 2.0f32 as cpFloat * r;
    tmp___1 = cpvlengthsq(offset);
    return m
        * ((length * length + 4.0f32 as cpFloat * r * r) / 12.0f32 as cpFloat + tmp___1);
}
pub unsafe extern "C" fn cpAreaForSegment(
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpvdist(a, b);
    return r * (3.14159265358979323846264338327950288f64 * r + 2.0f32 as cpFloat * tmp);
}
pub unsafe extern "C" fn cpMomentForPoly(
    mut m: cpFloat,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut offset: cpVect,
    mut r: cpFloat,
) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    let mut sum1: cpFloat = 0.;
    let mut sum2: cpFloat = 0.;
    let mut i: libc::c_int = 0;
    let mut v1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut v2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut a: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut b: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut tmp___5: cpFloat = 0.;
    if count == 2 as libc::c_int {
        tmp = cpMomentForSegment(
            m,
            *verts.offset(0 as libc::c_int as isize),
            *verts.offset(1 as libc::c_int as isize),
            0.0f32 as cpFloat,
        );
        return tmp;
    }
    sum1 = 0.0f32 as cpFloat;
    sum2 = 0.0f32 as cpFloat;
    i = 0 as libc::c_int;
    while i < count {
        tmp___0 = cpvadd(*verts.offset(i as isize), offset);
        v1 = tmp___0;
        tmp___1 = cpvadd(
            *verts.offset(((i + 1 as libc::c_int) % count) as isize),
            offset,
        );
        v2 = tmp___1;
        tmp___2 = cpvcross(v2, v1);
        a = tmp___2;
        tmp___3 = cpvdot(v1, v1);
        tmp___4 = cpvdot(v1, v2);
        tmp___5 = cpvdot(v2, v2);
        b = tmp___3 + tmp___4 + tmp___5;
        sum1 += a * b;
        sum2 += a;
        i += 1;
    }
    return m * sum1 / (6.0f32 as cpFloat * sum2);
}
pub unsafe extern "C" fn cpAreaForPoly(
    count: libc::c_int,
    mut verts: *const cpVect,
    mut r: cpFloat,
) -> cpFloat {
    let mut area: cpFloat = 0.;
    let mut perimeter: cpFloat = 0.;
    let mut i: libc::c_int = 0;
    let mut v1: cpVect = cpVect { x: 0., y: 0. };
    let mut v2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    area = 0.0f32 as cpFloat;
    perimeter = 0.0f32 as cpFloat;
    i = 0 as libc::c_int;
    while i < count {
        v1 = *verts.offset(i as isize);
        v2 = *verts.offset(((i + 1 as libc::c_int) % count) as isize);
        tmp = cpvcross(v1, v2);
        area += tmp;
        tmp___0 = cpvdist(v1, v2);
        perimeter += tmp___0;
        i += 1;
    }
    tmp___1 = cpfabs(r);
    return r * (3.14159265358979323846264338327950288f64 * tmp___1 + perimeter)
        + area / 2.0f32 as cpFloat;
}
pub unsafe extern "C" fn cpCentroidForPoly(
    count: libc::c_int,
    mut verts: *const cpVect,
) -> cpVect {
    let mut sum: cpFloat = 0.;
    let mut vsum: cpVect = cpVect { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut v1: cpVect = cpVect { x: 0., y: 0. };
    let mut v2: cpVect = cpVect { x: 0., y: 0. };
    let mut cross: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    sum = 0.0f32 as cpFloat;
    vsum = cpvzero;
    i = 0 as libc::c_int;
    while i < count {
        v1 = *verts.offset(i as isize);
        v2 = *verts.offset(((i + 1 as libc::c_int) % count) as isize);
        tmp = cpvcross(v1, v2);
        cross = tmp;
        sum += cross;
        tmp___0 = cpvadd(v1, v2);
        tmp___1 = cpvmult(tmp___0, cross);
        vsum = cpvadd(vsum, tmp___1);
        i += 1;
    }
    tmp___2 = cpvmult(vsum, 1.0f32 as cpFloat / (3.0f32 as cpFloat * sum));
    return tmp___2;
}
pub unsafe extern "C" fn cpMomentForBox(
    mut m: cpFloat,
    mut width: cpFloat,
    mut height: cpFloat,
) -> cpFloat {
    return m * (width * width + height * height) / 12.0f32 as cpFloat;
}
pub unsafe extern "C" fn cpMomentForBox2(mut m: cpFloat, mut box_0: cpBB) -> cpFloat {
    let mut width: cpFloat = 0.;
    let mut height: cpFloat = 0.;
    let mut offset: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    width = box_0.r - box_0.l;
    height = box_0.t - box_0.b;
    tmp = cpv(box_0.l + box_0.r, box_0.b + box_0.t);
    tmp___0 = cpvmult(tmp, 0.5f32 as cpFloat);
    offset = tmp___0;
    tmp___1 = cpMomentForBox(m, width, height);
    tmp___2 = cpvlengthsq(offset);
    return tmp___1 + m * tmp___2;
}
pub unsafe extern "C" fn cpLoopIndexes(
    mut verts: *const cpVect,
    mut count: libc::c_int,
    mut start: *mut libc::c_int,
    mut end: *mut libc::c_int,
) {
    let mut tmp: libc::c_int = 0;
    let mut min: cpVect = cpVect { x: 0., y: 0. };
    let mut max: cpVect = cpVect { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut v: cpVect = cpVect { x: 0., y: 0. };
    tmp = 0 as libc::c_int;
    *end = tmp;
    *start = tmp;
    min = *verts.offset(0 as libc::c_int as isize);
    max = min;
    i = 1 as libc::c_int;
    while i < count {
        v = *verts.offset(i as isize);
        if v.x < min.x {
            min = v;
            *start = i;
        } else {
            let mut current_block_22: u64;
            if v.x == min.x {
                if v.y < min.y {
                    min = v;
                    *start = i;
                    current_block_22 = 14401909646449704462;
                } else {
                    current_block_22 = 17143856208604645301;
                }
            } else {
                current_block_22 = 17143856208604645301;
            }
            match current_block_22 {
                17143856208604645301 => {
                    if v.x > max.x {
                        max = v;
                        *end = i;
                    } else if v.x == max.x {
                        if v.y > max.y {
                            max = v;
                            *end = i;
                        }
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn QHullPartition(
    mut verts: *mut cpVect,
    mut count: libc::c_int,
    mut a: cpVect,
    mut b: cpVect,
    mut tol: cpFloat,
) -> libc::c_int {
    let mut max: cpFloat = 0.;
    let mut pivot: libc::c_int = 0;
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut valueTol: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut head: libc::c_int = 0;
    let mut tail: libc::c_int = 0;
    let mut value: cpFloat = 0.;
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpFloat = 0.;
    let mut __TMP__: cpVect = cpVect { x: 0., y: 0. };
    let mut __TMP_____0: cpVect = cpVect { x: 0., y: 0. };
    if count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    max = 0 as libc::c_int as cpFloat;
    pivot = 0 as libc::c_int;
    tmp = cpvsub(b, a);
    delta = tmp;
    tmp___0 = cpvlength(delta);
    valueTol = tol * tmp___0;
    head = 0 as libc::c_int;
    tail = count - 1 as libc::c_int;
    while head <= tail {
        tmp___1 = cpvsub(*verts.offset(head as isize), a);
        tmp___2 = cpvcross(tmp___1, delta);
        value = tmp___2;
        if value > valueTol {
            if value > max {
                max = value;
                pivot = head;
            }
            head += 1;
        } else {
            __TMP__ = *verts.offset(head as isize);
            *verts.offset(head as isize) = *verts.offset(tail as isize);
            *verts.offset(tail as isize) = __TMP__;
            tail -= 1;
        }
    }
    if pivot != 0 as libc::c_int {
        __TMP_____0 = *verts.offset(0 as libc::c_int as isize);
        *verts.offset(0 as libc::c_int as isize) = *verts.offset(pivot as isize);
        *verts.offset(pivot as isize) = __TMP_____0;
    }
    return head;
}
unsafe extern "C" fn QHullReduce(
    mut tol: cpFloat,
    mut verts: *mut cpVect,
    mut count: libc::c_int,
    mut a: cpVect,
    mut pivot: cpVect,
    mut b: cpVect,
    mut result: *mut cpVect,
) -> libc::c_int {
    let mut left_count: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut index___0: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut right_count: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    if count < 0 as libc::c_int {
        return 0 as libc::c_int
    } else if count == 0 as libc::c_int {
        *result.offset(0 as libc::c_int as isize) = pivot;
        return 1 as libc::c_int;
    } else {
        tmp = QHullPartition(verts, count, a, pivot, tol);
        left_count = tmp;
        tmp___0 = QHullReduce(
            tol,
            verts.offset(1 as libc::c_int as isize),
            left_count - 1 as libc::c_int,
            a,
            *verts.offset(0 as libc::c_int as isize),
            pivot,
            result,
        );
        index___0 = tmp___0;
        tmp___1 = index___0;
        index___0 += 1;
        *result.offset(tmp___1 as isize) = pivot;
        tmp___2 = QHullPartition(
            verts.offset(left_count as isize),
            count - left_count,
            pivot,
            b,
            tol,
        );
        right_count = tmp___2;
        tmp___3 = QHullReduce(
            tol,
            verts.offset(left_count as isize).offset(1 as libc::c_int as isize),
            right_count - 1 as libc::c_int,
            pivot,
            *verts.offset(left_count as isize),
            b,
            result.offset(index___0 as isize),
        );
        return index___0 + tmp___3;
    };
}
pub unsafe extern "C" fn cpConvexHull(
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut result: *mut cpVect,
    mut first: *mut libc::c_int,
    mut tol: cpFloat,
) -> libc::c_int {
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut __TMP__: cpVect = cpVect { x: 0., y: 0. };
    let mut __TMP_____0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut a: cpVect = cpVect { x: 0., y: 0. };
    let mut b: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: libc::c_int = 0;
    if verts as libc::c_ulong != result as libc::c_ulong {
        memcpy(
            result as *mut libc::c_void,
            verts as *const libc::c_void,
            (count as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
        );
    }
    cpLoopIndexes(verts, count, &mut start, &mut end);
    if start == end {
        if !first.is_null() {
            *first = 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    __TMP__ = *result.offset(0 as libc::c_int as isize);
    *result.offset(0 as libc::c_int as isize) = *result.offset(start as isize);
    *result.offset(start as isize) = __TMP__;
    __TMP_____0 = *result.offset(1 as libc::c_int as isize);
    if end == 0 as libc::c_int {
        tmp = start;
    } else {
        tmp = end;
    }
    *result.offset(1 as libc::c_int as isize) = *result.offset(tmp as isize);
    if end == 0 as libc::c_int {
        tmp___0 = start;
    } else {
        tmp___0 = end;
    }
    *result.offset(tmp___0 as isize) = __TMP_____0;
    a = *result.offset(0 as libc::c_int as isize);
    b = *result.offset(1 as libc::c_int as isize);
    if !first.is_null() {
        *first = start;
    }
    tmp___1 = QHullReduce(
        tol,
        result.offset(2 as libc::c_int as isize),
        count - 2 as libc::c_int,
        a,
        b,
        a,
        result.offset(1 as libc::c_int as isize),
    );
    return tmp___1 + 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn cpfmax(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    if a > b {
        tmp = a;
    } else {
        tmp = b;
    }
    return tmp;
}
#[inline]
unsafe extern "C" fn cpfmin(mut a: cpFloat, mut b: cpFloat) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    if a < b {
        tmp = a;
    } else {
        tmp = b;
    }
    return tmp;
}
#[inline]
unsafe extern "C" fn cpfclamp(
    mut f: cpFloat,
    mut min: cpFloat,
    mut max: cpFloat,
) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    tmp = cpfmax(f, min);
    tmp___0 = cpfmin(tmp, max);
    return tmp___0;
}
static mut cpvzero___0: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpvneg(v: cpVect) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(-v.x, -v.y);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpvperp(v: cpVect) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(-v.y, v.x);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpvrotate(v1: cpVect, v2: cpVect) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(v1.x * v2.x - v1.y * v2.y, v1.x * v2.y + v1.y * v2.x);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpArbiterThreadForBody(
    mut arb: *mut cpArbiter,
    mut body: *mut cpBody,
) -> *mut cpArbiterThread {
    let mut tmp: *mut cpArbiterThread = 0 as *mut cpArbiterThread;
    if (*arb).body_a as libc::c_ulong == body as libc::c_ulong {
        tmp = &mut (*arb).thread_a;
    } else {
        tmp = &mut (*arb).thread_b;
    }
    return tmp;
}
#[inline]
unsafe extern "C" fn relative_velocity(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
) -> cpVect {
    let mut v1_sum: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut v2_sum: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvperp(r1);
    tmp___0 = cpvmult(tmp, (*a).w);
    tmp___1 = cpvadd((*a).v, tmp___0);
    v1_sum = tmp___1;
    tmp___2 = cpvperp(r2);
    tmp___3 = cpvmult(tmp___2, (*b).w);
    tmp___4 = cpvadd((*b).v, tmp___3);
    v2_sum = tmp___4;
    tmp___5 = cpvsub(v2_sum, v1_sum);
    return tmp___5;
}
#[inline]
unsafe extern "C" fn normal_relative_velocity(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
    mut n: cpVect,
) -> cpFloat {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpFloat = 0.;
    tmp = relative_velocity(a, b, r1, r2);
    tmp___0 = cpvdot(tmp, n);
    return tmp___0;
}
#[inline]
unsafe extern "C" fn apply_impulse(mut body: *mut cpBody, mut j: cpVect, mut r: cpVect) {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpFloat = 0.;
    tmp = cpvmult(j, (*body).m_inv);
    (*body).v = cpvadd((*body).v, tmp);
    tmp___0 = cpvcross(r, j);
    (*body).w += (*body).i_inv * tmp___0;
}
#[inline]
unsafe extern "C" fn apply_impulses(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
    mut j: cpVect,
) {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvneg(j);
    apply_impulse(a, tmp, r1);
    apply_impulse(b, j, r2);
}
#[inline]
unsafe extern "C" fn apply_bias_impulse(
    mut body: *mut cpBody,
    mut j: cpVect,
    mut r: cpVect,
) {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpFloat = 0.;
    tmp = cpvmult(j, (*body).m_inv);
    (*body).v_bias = cpvadd((*body).v_bias, tmp);
    tmp___0 = cpvcross(r, j);
    (*body).w_bias += (*body).i_inv * tmp___0;
}
#[inline]
unsafe extern "C" fn apply_bias_impulses(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
    mut j: cpVect,
) {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvneg(j);
    apply_bias_impulse(a, tmp, r1);
    apply_bias_impulse(b, j, r2);
}
#[inline]
unsafe extern "C" fn k_scalar_body(
    mut body: *mut cpBody,
    mut r: cpVect,
    mut n: cpVect,
) -> cpFloat {
    let mut rcn: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    tmp = cpvcross(r, n);
    rcn = tmp;
    return (*body).m_inv + (*body).i_inv * rcn * rcn;
}
#[inline]
unsafe extern "C" fn k_scalar(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
    mut n: cpVect,
) -> cpFloat {
    let mut value: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    tmp = k_scalar_body(a, r1, n);
    tmp___0 = k_scalar_body(b, r2, n);
    value = tmp + tmp___0;
    return value;
}
#[inline]
unsafe extern "C" fn unthreadHelper(mut arb: *mut cpArbiter, mut body: *mut cpBody) {
    let mut thread: *mut cpArbiterThread = 0 as *mut cpArbiterThread;
    let mut tmp: *mut cpArbiterThread = 0 as *mut cpArbiterThread;
    let mut prev: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut next: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut tmp___0: *mut cpArbiterThread = 0 as *mut cpArbiterThread;
    let mut tmp___1: *mut cpArbiterThread = 0 as *mut cpArbiterThread;
    tmp = cpArbiterThreadForBody(arb, body);
    thread = tmp;
    prev = (*thread).prev;
    next = (*thread).next;
    if !prev.is_null() {
        tmp___0 = cpArbiterThreadForBody(prev, body);
        (*tmp___0).next = next;
    } else if (*body).arbiterList as libc::c_ulong == arb as libc::c_ulong {
        (*body).arbiterList = next;
    }
    if !next.is_null() {
        tmp___1 = cpArbiterThreadForBody(next, body);
        (*tmp___1).prev = prev;
    }
    (*thread).prev = 0 as *mut libc::c_void as *mut cpArbiter;
    (*thread).next = 0 as *mut libc::c_void as *mut cpArbiter;
}
pub unsafe extern "C" fn cpArbiterUnthread(mut arb: *mut cpArbiter) {
    unthreadHelper(arb, (*arb).body_a);
    unthreadHelper(arb, (*arb).body_b);
}
pub unsafe extern "C" fn cpArbiterIsFirstContact(mut arb: *const cpArbiter) -> cpBool {
    return ((*arb).state as libc::c_uint == 0 as libc::c_uint) as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpArbiterIsRemoval(mut arb: *const cpArbiter) -> cpBool {
    return ((*arb).state as libc::c_uint == 4 as libc::c_uint) as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpArbiterGetCount(mut arb: *const cpArbiter) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if ((*arb).state as libc::c_uint) < 3 as libc::c_uint {
        tmp = (*arb).count;
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp;
}
pub unsafe extern "C" fn cpArbiterGetNormal(mut arb: *const cpArbiter) -> cpVect {
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    if (*arb).swapped != 0 {
        tmp = -1.0f32 as libc::c_double;
    } else {
        tmp = 1.0f64;
    }
    tmp___0 = cpvmult((*arb).n, tmp);
    return tmp___0;
}
pub unsafe extern "C" fn cpArbiterGetPointA(
    mut arb: *const cpArbiter,
    mut i: libc::c_int,
) -> cpVect {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    if 0 as libc::c_int <= i {
        tmp = cpArbiterGetCount(arb);
        if !(i < tmp) {
            cpMessage(
                b"0 <= i && i < cpArbiterGetCount(arb)\0" as *const u8
                    as *const libc::c_char,
                b"../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Index error: The specified contact index is invalid for this arbiter\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
    } else {
        cpMessage(
            b"0 <= i && i < cpArbiterGetCount(arb)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Index error: The specified contact index is invalid for this arbiter\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    tmp___0 = cpvadd((*(*arb).body_a).p, (*((*arb).contacts).offset(i as isize)).r1);
    return tmp___0;
}
pub unsafe extern "C" fn cpArbiterGetPointB(
    mut arb: *const cpArbiter,
    mut i: libc::c_int,
) -> cpVect {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    if 0 as libc::c_int <= i {
        tmp = cpArbiterGetCount(arb);
        if !(i < tmp) {
            cpMessage(
                b"0 <= i && i < cpArbiterGetCount(arb)\0" as *const u8
                    as *const libc::c_char,
                b"../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
                86 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Index error: The specified contact index is invalid for this arbiter\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
    } else {
        cpMessage(
            b"0 <= i && i < cpArbiterGetCount(arb)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Index error: The specified contact index is invalid for this arbiter\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    tmp___0 = cpvadd((*(*arb).body_b).p, (*((*arb).contacts).offset(i as isize)).r2);
    return tmp___0;
}
pub unsafe extern "C" fn cpArbiterGetDepth(
    mut arb: *const cpArbiter,
    mut i: libc::c_int,
) -> cpFloat {
    let mut tmp: libc::c_int = 0;
    let mut con: *mut cpContact = 0 as *mut cpContact;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpFloat = 0.;
    if 0 as libc::c_int <= i {
        tmp = cpArbiterGetCount(arb);
        if !(i < tmp) {
            cpMessage(
                b"0 <= i && i < cpArbiterGetCount(arb)\0" as *const u8
                    as *const libc::c_char,
                b"../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
                93 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Index error: The specified contact index is invalid for this arbiter\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
    } else {
        cpMessage(
            b"0 <= i && i < cpArbiterGetCount(arb)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
            93 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Index error: The specified contact index is invalid for this arbiter\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    con = ((*arb).contacts).offset(i as isize);
    tmp___0 = cpvsub((*(*arb).body_b).p, (*(*arb).body_a).p);
    tmp___1 = cpvsub((*con).r2, (*con).r1);
    tmp___2 = cpvadd(tmp___1, tmp___0);
    tmp___3 = cpvdot(tmp___2, (*arb).n);
    return tmp___3;
}
pub unsafe extern "C" fn cpArbiterGetContactPointSet(
    mut arb: *const cpArbiter,
) -> cpContactPointSet {
    let mut set: cpContactPointSet = cpContactPointSet {
        count: 0,
        normal: cpVect { x: 0., y: 0. },
        points: [__anonstruct_points_450528349 {
            pointA: cpVect { x: 0., y: 0. },
            pointB: cpVect { x: 0., y: 0. },
            distance: 0.,
        }; 2],
    };
    let mut swapped: cpBool = 0;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut p1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut p2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    set.count = cpArbiterGetCount(arb);
    swapped = (*arb).swapped;
    n = (*arb).n;
    if swapped != 0 {
        tmp = cpvneg(n);
        set.normal = tmp;
    } else {
        set.normal = n;
    }
    i = 0 as libc::c_int;
    while i < set.count {
        tmp___0 = cpvadd((*(*arb).body_a).p, (*((*arb).contacts).offset(i as isize)).r1);
        p1 = tmp___0;
        tmp___1 = cpvadd((*(*arb).body_b).p, (*((*arb).contacts).offset(i as isize)).r2);
        p2 = tmp___1;
        if swapped != 0 {
            set.points[i as usize].pointA = p2;
        } else {
            set.points[i as usize].pointA = p1;
        }
        if swapped != 0 {
            set.points[i as usize].pointB = p1;
        } else {
            set.points[i as usize].pointB = p2;
        }
        tmp___2 = cpvsub(p2, p1);
        set.points[i as usize].distance = cpvdot(tmp___2, n);
        i += 1;
    }
    return set;
}
pub unsafe extern "C" fn cpArbiterSetContactPointSet(
    mut arb: *mut cpArbiter,
    mut set: *mut cpContactPointSet,
) {
    let mut count: libc::c_int = 0;
    let mut swapped: cpBool = 0;
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut p1: cpVect = cpVect { x: 0., y: 0. };
    let mut p2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    count = (*set).count;
    if !(count == (*arb).count) {
        cpMessage(
            b"count == arb->count\0" as *const u8 as *const libc::c_char,
            b"../src/cpArbiter.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"The number of contact points cannot be changed.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    swapped = (*arb).swapped;
    if swapped != 0 {
        tmp = cpvneg((*set).normal);
        (*arb).n = tmp;
    } else {
        (*arb).n = (*set).normal;
    }
    i = 0 as libc::c_int;
    while i < count {
        p1 = (*set).points[i as usize].pointA;
        p2 = (*set).points[i as usize].pointB;
        if swapped != 0 {
            tmp___0 = p2;
        } else {
            tmp___0 = p1;
        }
        (*((*arb).contacts).offset(i as isize)).r1 = cpvsub(tmp___0, (*(*arb).body_a).p);
        if swapped != 0 {
            tmp___1 = p1;
        } else {
            tmp___1 = p2;
        }
        (*((*arb).contacts).offset(i as isize)).r2 = cpvsub(tmp___1, (*(*arb).body_b).p);
        i += 1;
    }
}
pub unsafe extern "C" fn cpArbiterTotalImpulse(mut arb: *const cpArbiter) -> cpVect {
    let mut contacts: *mut cpContact = 0 as *mut cpContact;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut sum: cpVect = cpVect { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut con: *mut cpContact = 0 as *mut cpContact;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    contacts = (*arb).contacts;
    n = (*arb).n;
    sum = cpvzero___0;
    i = 0 as libc::c_int;
    tmp = cpArbiterGetCount(arb);
    count = tmp;
    while i < count {
        con = contacts.offset(i as isize);
        tmp___0 = cpv((*con).jnAcc, (*con).jtAcc);
        tmp___1 = cpvrotate(n, tmp___0);
        sum = cpvadd(sum, tmp___1);
        i += 1;
    }
    if (*arb).swapped != 0 {
        tmp___3 = sum;
    } else {
        tmp___2 = cpvneg(sum);
        tmp___3 = tmp___2;
    }
    return tmp___3;
}
pub unsafe extern "C" fn cpArbiterTotalKE(mut arb: *const cpArbiter) -> cpFloat {
    let mut eCoef: cpFloat = 0.;
    let mut sum: cpFloat = 0.;
    let mut contacts: *mut cpContact = 0 as *mut cpContact;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut con: *mut cpContact = 0 as *mut cpContact;
    let mut jnAcc: cpFloat = 0.;
    let mut jtAcc: cpFloat = 0.;
    eCoef = (1 as libc::c_int as cpFloat - (*arb).e)
        / (1 as libc::c_int as cpFloat + (*arb).e);
    sum = 0.0f64;
    contacts = (*arb).contacts;
    i = 0 as libc::c_int;
    tmp = cpArbiterGetCount(arb);
    count = tmp;
    while i < count {
        con = contacts.offset(i as isize);
        jnAcc = (*con).jnAcc;
        jtAcc = (*con).jtAcc;
        sum += eCoef * jnAcc * jnAcc / (*con).nMass + jtAcc * jtAcc / (*con).tMass;
        i += 1;
    }
    return sum;
}
pub unsafe extern "C" fn cpArbiterIgnore(mut arb: *mut cpArbiter) -> cpBool {
    (*arb).state = CP_ARBITER_STATE_IGNORE;
    return 0 as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpArbiterGetRestitution(mut arb: *const cpArbiter) -> cpFloat {
    return (*arb).e;
}
pub unsafe extern "C" fn cpArbiterSetRestitution(
    mut arb: *mut cpArbiter,
    mut restitution: cpFloat,
) {
    (*arb).e = restitution;
}
pub unsafe extern "C" fn cpArbiterGetFriction(mut arb: *const cpArbiter) -> cpFloat {
    return (*arb).u;
}
pub unsafe extern "C" fn cpArbiterSetFriction(
    mut arb: *mut cpArbiter,
    mut friction: cpFloat,
) {
    (*arb).u = friction;
}
pub unsafe extern "C" fn cpArbiterGetSurfaceVelocity(mut arb: *mut cpArbiter) -> cpVect {
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    if (*arb).swapped != 0 {
        tmp = -1.0f32 as libc::c_double;
    } else {
        tmp = 1.0f64;
    }
    tmp___0 = cpvmult((*arb).surface_vr, tmp);
    return tmp___0;
}
pub unsafe extern "C" fn cpArbiterSetSurfaceVelocity(
    mut arb: *mut cpArbiter,
    mut vr: cpVect,
) {
    let mut tmp: libc::c_double = 0.;
    if (*arb).swapped != 0 {
        tmp = -1.0f32 as libc::c_double;
    } else {
        tmp = 1.0f64;
    }
    (*arb).surface_vr = cpvmult(vr, tmp);
}
pub unsafe extern "C" fn cpArbiterGetUserData(
    mut arb: *const cpArbiter,
) -> cpDataPointer {
    return (*arb).data;
}
pub unsafe extern "C" fn cpArbiterSetUserData(
    mut arb: *mut cpArbiter,
    mut userData: cpDataPointer,
) {
    (*arb).data = userData;
}
pub unsafe extern "C" fn cpArbiterGetShapes(
    mut arb: *const cpArbiter,
    mut a: *mut *mut cpShape,
    mut b: *mut *mut cpShape,
) {
    if (*arb).swapped != 0 {
        *a = (*arb).b as *mut cpShape;
        *b = (*arb).a as *mut cpShape;
    } else {
        *a = (*arb).a as *mut cpShape;
        *b = (*arb).b as *mut cpShape;
    };
}
pub unsafe extern "C" fn cpArbiterGetBodies(
    mut arb: *const cpArbiter,
    mut a: *mut *mut cpBody,
    mut b: *mut *mut cpBody,
) {
    let mut shape_a: *mut cpShape = 0 as *mut cpShape;
    let mut shape_b: *mut cpShape = 0 as *mut cpShape;
    cpArbiterGetShapes(arb, &mut shape_a, &mut shape_b);
    *a = (*shape_a).body;
    *b = (*shape_b).body;
}
pub unsafe extern "C" fn cpArbiterCallWildcardBeginA(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) -> cpBool {
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    let mut tmp: cpBool = 0;
    handler = (*arb).handlerA;
    tmp = (Some(((*handler).beginFunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(arb, space, (*handler).userData);
    return tmp;
}
pub unsafe extern "C" fn cpArbiterCallWildcardBeginB(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) -> cpBool {
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    let mut retval: cpBool = 0;
    let mut tmp: cpBool = 0;
    handler = (*arb).handlerB;
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    tmp = (Some(((*handler).beginFunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(arb, space, (*handler).userData);
    retval = tmp;
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    return retval;
}
pub unsafe extern "C" fn cpArbiterCallWildcardPreSolveA(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) -> cpBool {
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    let mut tmp: cpBool = 0;
    handler = (*arb).handlerA;
    tmp = (Some(((*handler).preSolveFunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(arb, space, (*handler).userData);
    return tmp;
}
pub unsafe extern "C" fn cpArbiterCallWildcardPreSolveB(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) -> cpBool {
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    let mut retval: cpBool = 0;
    let mut tmp: cpBool = 0;
    handler = (*arb).handlerB;
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    tmp = (Some(((*handler).preSolveFunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(arb, space, (*handler).userData);
    retval = tmp;
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    return retval;
}
pub unsafe extern "C" fn cpArbiterCallWildcardPostSolveA(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) {
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    handler = (*arb).handlerA;
    (Some(((*handler).postSolveFunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(arb, space, (*handler).userData);
}
pub unsafe extern "C" fn cpArbiterCallWildcardPostSolveB(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) {
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    handler = (*arb).handlerB;
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    (Some(((*handler).postSolveFunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(arb, space, (*handler).userData);
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpArbiterCallWildcardSeparateA(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) {
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    handler = (*arb).handlerA;
    (Some(((*handler).separateFunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(arb, space, (*handler).userData);
}
pub unsafe extern "C" fn cpArbiterCallWildcardSeparateB(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) {
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    handler = (*arb).handlerB;
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
    (Some(((*handler).separateFunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(arb, space, (*handler).userData);
    (*arb).swapped = ((*arb).swapped == 0) as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpArbiterInit(
    mut arb: *mut cpArbiter,
    mut a: *mut cpShape,
    mut b: *mut cpShape,
) -> *mut cpArbiter {
    (*arb).handler = 0 as *mut libc::c_void as *mut cpCollisionHandler;
    (*arb).swapped = 0 as libc::c_int as cpBool;
    (*arb).handler = 0 as *mut libc::c_void as *mut cpCollisionHandler;
    (*arb).handlerA = 0 as *mut libc::c_void as *mut cpCollisionHandler;
    (*arb).handlerB = 0 as *mut libc::c_void as *mut cpCollisionHandler;
    (*arb).e = 0.0f32 as cpFloat;
    (*arb).u = 0.0f32 as cpFloat;
    (*arb).surface_vr = cpvzero___0;
    (*arb).count = 0 as libc::c_int;
    (*arb).contacts = 0 as *mut libc::c_void as *mut cpContact;
    (*arb).a = a as *const cpShape;
    (*arb).body_a = (*a).body;
    (*arb).b = b as *const cpShape;
    (*arb).body_b = (*b).body;
    (*arb).thread_a.next = 0 as *mut libc::c_void as *mut cpArbiter;
    (*arb).thread_b.next = 0 as *mut libc::c_void as *mut cpArbiter;
    (*arb).thread_a.prev = 0 as *mut libc::c_void as *mut cpArbiter;
    (*arb).thread_b.prev = 0 as *mut libc::c_void as *mut cpArbiter;
    (*arb).stamp = 0 as libc::c_int as cpTimestamp;
    (*arb).state = CP_ARBITER_STATE_FIRST_COLLISION;
    (*arb).data = 0 as *mut libc::c_void;
    return arb;
}
#[inline]
unsafe extern "C" fn cpSpaceLookupHandler(
    mut space: *mut cpSpace,
    mut a: cpCollisionType,
    mut b: cpCollisionType,
    mut defaultValue: *mut cpCollisionHandler,
) -> *mut cpCollisionHandler {
    let mut types: [cpCollisionType; 2] = [0; 2];
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    let mut tmp___0: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    types[0 as libc::c_int as usize] = a;
    types[1 as libc::c_int as usize] = b;
    tmp = cpHashSetFind(
        (*space).collisionHandlers,
        a.wrapping_mul(3344921057 as libc::c_ulong)
            ^ b.wrapping_mul(3344921057 as libc::c_ulong),
        types.as_mut_ptr() as *const libc::c_void,
    );
    handler = tmp as *mut cpCollisionHandler;
    if !handler.is_null() {
        tmp___0 = handler;
    } else {
        tmp___0 = defaultValue;
    }
    return tmp___0;
}
pub unsafe extern "C" fn cpArbiterUpdate(
    mut arb: *mut cpArbiter,
    mut info: *mut cpCollisionInfo,
    mut space: *mut cpSpace,
) {
    let mut a: *const cpShape = 0 as *const cpShape;
    let mut b: *const cpShape = 0 as *const cpShape;
    let mut i: libc::c_int = 0;
    let mut con: *mut cpContact = 0 as *mut cpContact;
    let mut tmp: cpFloat = 0.;
    let mut j: libc::c_int = 0;
    let mut old: *mut cpContact = 0 as *mut cpContact;
    let mut surface_vr: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut typeA: cpCollisionType = 0;
    let mut typeB: cpCollisionType = 0;
    let mut defaultHandler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    let mut tmp___3: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    let mut swapped: cpBool = 0;
    let mut tmp___4: cpBool = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: cpCollisionType = 0;
    let mut tmp___7: cpCollisionType = 0;
    a = (*info).a;
    b = (*info).b;
    (*arb).a = a;
    (*arb).body_a = (*a).body;
    (*arb).b = b;
    (*arb).body_b = (*b).body;
    i = 0 as libc::c_int;
    while i < (*info).count {
        con = ((*info).arr).offset(i as isize);
        (*con).r1 = cpvsub((*con).r1, (*(*a).body).p);
        (*con).r2 = cpvsub((*con).r2, (*(*b).body).p);
        tmp = 0.0f32 as cpFloat;
        (*con).jtAcc = tmp;
        (*con).jnAcc = tmp;
        j = 0 as libc::c_int;
        while j < (*arb).count {
            old = ((*arb).contacts).offset(j as isize);
            if (*con).hash == (*old).hash {
                (*con).jnAcc = (*old).jnAcc;
                (*con).jtAcc = (*old).jtAcc;
            }
            j += 1;
        }
        i += 1;
    }
    (*arb).contacts = (*info).arr;
    (*arb).count = (*info).count;
    (*arb).n = (*info).n;
    (*arb).e = (*a).e * (*b).e;
    (*arb).u = (*a).u * (*b).u;
    tmp___0 = cpvsub((*b).surfaceV, (*a).surfaceV);
    surface_vr = tmp___0;
    tmp___1 = cpvdot(surface_vr, (*info).n);
    tmp___2 = cpvmult((*info).n, tmp___1);
    (*arb).surface_vr = cpvsub(surface_vr, tmp___2);
    typeA = (*(*info).a).type_0;
    typeB = (*(*info).b).type_0;
    defaultHandler = &mut (*space).defaultHandler;
    tmp___3 = cpSpaceLookupHandler(space, typeA, typeB, defaultHandler);
    (*arb).handler = tmp___3;
    handler = tmp___3;
    if typeA != (*handler).typeA {
        if (*handler).typeA != 0xffffffffffffffff as libc::c_ulong {
            tmp___5 = 1 as libc::c_int;
        } else {
            tmp___5 = 0 as libc::c_int;
        }
    } else {
        tmp___5 = 0 as libc::c_int;
    }
    tmp___4 = tmp___5 as cpBool;
    (*arb).swapped = tmp___4;
    swapped = tmp___4;
    let mut current_block_64: u64;
    if handler as libc::c_ulong != defaultHandler as libc::c_ulong {
        current_block_64 = 11188772430193394432;
    } else if (*space).usesWildcards != 0 {
        current_block_64 = 11188772430193394432;
    } else {
        current_block_64 = 16415152177862271243;
    }
    match current_block_64 {
        11188772430193394432 => {
            if swapped != 0 {
                tmp___6 = typeB;
            } else {
                tmp___6 = typeA;
            }
            (*arb)
                .handlerA = cpSpaceLookupHandler(
                space,
                tmp___6,
                !(0 as libc::c_int as cpCollisionType),
                &mut cpCollisionHandlerDoNothing,
            );
            if swapped != 0 {
                tmp___7 = typeA;
            } else {
                tmp___7 = typeB;
            }
            (*arb)
                .handlerB = cpSpaceLookupHandler(
                space,
                tmp___7,
                !(0 as libc::c_int as cpCollisionType),
                &mut cpCollisionHandlerDoNothing,
            );
        }
        _ => {}
    }
    if (*arb).state as libc::c_uint == 3 as libc::c_uint {
        (*arb).state = CP_ARBITER_STATE_FIRST_COLLISION;
    }
}
pub unsafe extern "C" fn cpArbiterPreStep(
    mut arb: *mut cpArbiter,
    mut dt: cpFloat,
    mut slop: cpFloat,
    mut bias: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut body_delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut con: *mut cpContact = 0 as *mut cpContact;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpFloat = 0.;
    let mut dist: cpFloat = 0.;
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpFloat = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut tmp___7: cpFloat = 0.;
    a = (*arb).body_a;
    b = (*arb).body_b;
    n = (*arb).n;
    tmp = cpvsub((*b).p, (*a).p);
    body_delta = tmp;
    i = 0 as libc::c_int;
    while i < (*arb).count {
        con = ((*arb).contacts).offset(i as isize);
        tmp___0 = k_scalar(a, b, (*con).r1, (*con).r2, n);
        (*con).nMass = 1.0f32 as cpFloat / tmp___0;
        tmp___1 = cpvperp(n);
        tmp___2 = k_scalar(a, b, (*con).r1, (*con).r2, tmp___1);
        (*con).tMass = 1.0f32 as cpFloat / tmp___2;
        tmp___3 = cpvsub((*con).r2, (*con).r1);
        tmp___4 = cpvadd(tmp___3, body_delta);
        tmp___5 = cpvdot(tmp___4, n);
        dist = tmp___5;
        tmp___6 = cpfmin(0.0f32 as cpFloat, dist + slop);
        (*con).bias = -bias * tmp___6 / dt;
        (*con).jBias = 0.0f32 as cpFloat;
        tmp___7 = normal_relative_velocity(a, b, (*con).r1, (*con).r2, n);
        (*con).bounce = tmp___7 * (*arb).e;
        i += 1;
    }
}
pub unsafe extern "C" fn cpArbiterApplyCachedImpulse(
    mut arb: *mut cpArbiter,
    mut dt_coef: cpFloat,
) {
    let mut tmp: cpBool = 0;
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut con: *mut cpContact = 0 as *mut cpContact;
    let mut j: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpArbiterIsFirstContact(arb as *const cpArbiter);
    if tmp != 0 {
        return;
    }
    a = (*arb).body_a;
    b = (*arb).body_b;
    n = (*arb).n;
    i = 0 as libc::c_int;
    while i < (*arb).count {
        con = ((*arb).contacts).offset(i as isize);
        tmp___0 = cpv((*con).jnAcc, (*con).jtAcc);
        tmp___1 = cpvrotate(n, tmp___0);
        j = tmp___1;
        tmp___2 = cpvmult(j, dt_coef);
        apply_impulses(a, b, (*con).r1, (*con).r2, tmp___2);
        i += 1;
    }
}
pub unsafe extern "C" fn cpArbiterApplyImpulse(mut arb: *mut cpArbiter) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut surface_vr: cpVect = cpVect { x: 0., y: 0. };
    let mut friction: cpFloat = 0.;
    let mut i: libc::c_int = 0;
    let mut con: *mut cpContact = 0 as *mut cpContact;
    let mut nMass: cpFloat = 0.;
    let mut r1: cpVect = cpVect { x: 0., y: 0. };
    let mut r2: cpVect = cpVect { x: 0., y: 0. };
    let mut vb1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut vb2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut vr: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut vbn: cpFloat = 0.;
    let mut tmp___7: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___8: cpFloat = 0.;
    let mut vrn: cpFloat = 0.;
    let mut tmp___9: cpFloat = 0.;
    let mut vrt: cpFloat = 0.;
    let mut tmp___10: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___11: cpFloat = 0.;
    let mut jbn: cpFloat = 0.;
    let mut jbnOld: cpFloat = 0.;
    let mut jn___0: cpFloat = 0.;
    let mut jnOld: cpFloat = 0.;
    let mut jtMax: cpFloat = 0.;
    let mut jt: cpFloat = 0.;
    let mut jtOld: cpFloat = 0.;
    let mut tmp___12: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___13: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___14: cpVect = cpVect { x: 0., y: 0. };
    a = (*arb).body_a;
    b = (*arb).body_b;
    n = (*arb).n;
    surface_vr = (*arb).surface_vr;
    friction = (*arb).u;
    i = 0 as libc::c_int;
    while i < (*arb).count {
        con = ((*arb).contacts).offset(i as isize);
        nMass = (*con).nMass;
        r1 = (*con).r1;
        r2 = (*con).r2;
        tmp = cpvperp(r1);
        tmp___0 = cpvmult(tmp, (*a).w_bias);
        tmp___1 = cpvadd((*a).v_bias, tmp___0);
        vb1 = tmp___1;
        tmp___2 = cpvperp(r2);
        tmp___3 = cpvmult(tmp___2, (*b).w_bias);
        tmp___4 = cpvadd((*b).v_bias, tmp___3);
        vb2 = tmp___4;
        tmp___5 = relative_velocity(a, b, r1, r2);
        tmp___6 = cpvadd(tmp___5, surface_vr);
        vr = tmp___6;
        tmp___7 = cpvsub(vb2, vb1);
        tmp___8 = cpvdot(tmp___7, n);
        vbn = tmp___8;
        tmp___9 = cpvdot(vr, n);
        vrn = tmp___9;
        tmp___10 = cpvperp(n);
        tmp___11 = cpvdot(vr, tmp___10);
        vrt = tmp___11;
        jbn = ((*con).bias - vbn) * nMass;
        jbnOld = (*con).jBias;
        (*con).jBias = cpfmax(jbnOld + jbn, 0.0f32 as cpFloat);
        jn___0 = -((*con).bounce + vrn) * nMass;
        jnOld = (*con).jnAcc;
        (*con).jnAcc = cpfmax(jnOld + jn___0, 0.0f32 as cpFloat);
        jtMax = friction * (*con).jnAcc;
        jt = -vrt * (*con).tMass;
        jtOld = (*con).jtAcc;
        (*con).jtAcc = cpfclamp(jtOld + jt, -jtMax, jtMax);
        tmp___12 = cpvmult(n, (*con).jBias - jbnOld);
        apply_bias_impulses(a, b, r1, r2, tmp___12);
        tmp___13 = cpv((*con).jnAcc - jnOld, (*con).jtAcc - jtOld);
        tmp___14 = cpvrotate(n, tmp___13);
        apply_impulses(a, b, r1, r2, tmp___14);
        i += 1;
    }
}
pub unsafe extern "C" fn cpArrayNew(mut size: libc::c_int) -> *mut cpArray {
    let mut arr: *mut cpArray = 0 as *mut cpArray;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpArray>() as libc::c_ulong,
    );
    arr = tmp as *mut cpArray;
    (*arr).num = 0 as libc::c_int;
    if size != 0 {
        (*arr).max = size;
    } else {
        (*arr).max = 4 as libc::c_int;
    }
    tmp___0 = calloc(
        (*arr).max as size_t,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    );
    (*arr).arr = tmp___0 as *mut *mut libc::c_void;
    return arr;
}
pub unsafe extern "C" fn cpArrayFree(mut arr: *mut cpArray) {
    if !arr.is_null() {
        free((*arr).arr as *mut libc::c_void);
        (*arr).arr = 0 as *mut libc::c_void as *mut *mut libc::c_void;
        free(arr as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpArrayPush(
    mut arr: *mut cpArray,
    mut object: *mut libc::c_void,
) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*arr).num == (*arr).max {
        (*arr)
            .max = 3 as libc::c_int * ((*arr).max + 1 as libc::c_int) / 2 as libc::c_int;
        tmp = realloc(
            (*arr).arr as *mut libc::c_void,
            ((*arr).max as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
        (*arr).arr = tmp as *mut *mut libc::c_void;
    }
    let ref mut fresh0 = *((*arr).arr).offset((*arr).num as isize);
    *fresh0 = object;
    (*arr).num += 1;
}
pub unsafe extern "C" fn cpArrayPop(mut arr: *mut cpArray) -> *mut libc::c_void {
    let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
    (*arr).num -= 1;
    value = *((*arr).arr).offset((*arr).num as isize);
    let ref mut fresh1 = *((*arr).arr).offset((*arr).num as isize);
    *fresh1 = 0 as *mut libc::c_void;
    return value;
}
pub unsafe extern "C" fn cpArrayDeleteObj(
    mut arr: *mut cpArray,
    mut obj: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*arr).num {
        if *((*arr).arr).offset(i as isize) as libc::c_ulong == obj as libc::c_ulong {
            (*arr).num -= 1;
            let ref mut fresh2 = *((*arr).arr).offset(i as isize);
            *fresh2 = *((*arr).arr).offset((*arr).num as isize);
            let ref mut fresh3 = *((*arr).arr).offset((*arr).num as isize);
            *fresh3 = 0 as *mut libc::c_void;
            return;
        }
        i += 1;
    }
}
pub unsafe extern "C" fn cpArrayFreeEach(
    mut arr: *mut cpArray,
    mut freeFunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*arr).num {
        (Some(freeFunc.expect("non-null function pointer")))
            .expect("non-null function pointer")(*((*arr).arr).offset(i as isize));
        i += 1;
    }
}
pub unsafe extern "C" fn cpArrayContains(
    mut arr: *mut cpArray,
    mut ptr: *mut libc::c_void,
) -> cpBool {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*arr).num {
        if *((*arr).arr).offset(i as isize) as libc::c_ulong == ptr as libc::c_ulong {
            return 1 as libc::c_int as cpBool;
        }
        i += 1;
    }
    return 0 as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn cpBBNew(l: cpFloat, b: cpFloat, r: cpFloat, t: cpFloat) -> cpBB {
    let mut bb: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    bb.l = l;
    bb.b = b;
    bb.r = r;
    bb.t = t;
    return bb;
}
#[inline]
unsafe extern "C" fn cpBBIntersects(a: cpBB, b: cpBB) -> cpBool {
    let mut tmp: libc::c_int = 0;
    if a.l <= b.r {
        if b.l <= a.r {
            if a.b <= b.t {
                if b.b <= a.t {
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
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp as cpBool;
}
#[inline]
unsafe extern "C" fn cpBBContainsBB(bb: cpBB, other: cpBB) -> cpBool {
    let mut tmp: libc::c_int = 0;
    if bb.l <= other.l {
        if bb.r >= other.r {
            if bb.b <= other.b {
                if bb.t >= other.t {
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
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp as cpBool;
}
#[inline]
unsafe extern "C" fn cpBBMerge(a: cpBB, b: cpBB) -> cpBB {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = cpfmax(a.t, b.t);
    tmp___0 = cpfmax(a.r, b.r);
    tmp___1 = cpfmin(a.b, b.b);
    tmp___2 = cpfmin(a.l, b.l);
    tmp___3 = cpBBNew(tmp___2, tmp___1, tmp___0, tmp);
    return tmp___3;
}
#[inline]
unsafe extern "C" fn cpBBArea(mut bb: cpBB) -> cpFloat {
    return (bb.r - bb.l) * (bb.t - bb.b);
}
#[inline]
unsafe extern "C" fn cpBBMergedArea(mut a: cpBB, mut b: cpBB) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    tmp = cpfmax(a.r, b.r);
    tmp___0 = cpfmin(a.l, b.l);
    tmp___1 = cpfmax(a.t, b.t);
    tmp___2 = cpfmin(a.b, b.b);
    return (tmp - tmp___0) * (tmp___1 - tmp___2);
}
#[inline]
unsafe extern "C" fn cpBBSegmentQuery(
    mut bb: cpBB,
    mut a: cpVect,
    mut b: cpVect,
) -> cpFloat {
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmin: cpFloat = 0.;
    let mut tmp___0: libc::c_float = 0.;
    let mut tmax: cpFloat = 0.;
    let mut tmp___1: libc::c_float = 0.;
    let mut tmp___2: libc::c_float = 0.;
    let mut t1: cpFloat = 0.;
    let mut t2: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut tmp___5: libc::c_float = 0.;
    let mut t1___0: cpFloat = 0.;
    let mut t2___0: cpFloat = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut tmp___7: cpFloat = 0.;
    let mut tmp___8: cpFloat = 0.;
    let mut tmp___9: libc::c_float = 0.;
    tmp = cpvsub(b, a);
    delta = tmp;
    tmp___0 = ::std::f32::INFINITY;
    tmin = -tmp___0 as cpFloat;
    tmp___1 = ::std::f32::INFINITY;
    tmax = tmp___1 as cpFloat;
    if delta.x == 0.0f32 as cpFloat {
        if a.x < bb.l {
            tmp___2 = ::std::f32::INFINITY;
            return tmp___2 as cpFloat;
        } else {
            if bb.r < a.x {
                tmp___2 = ::std::f32::INFINITY;
                return tmp___2 as cpFloat;
            }
        }
    } else {
        t1 = (bb.l - a.x) / delta.x;
        t2 = (bb.r - a.x) / delta.x;
        tmp___3 = cpfmin(t1, t2);
        tmin = cpfmax(tmin, tmp___3);
        tmp___4 = cpfmax(t1, t2);
        tmax = cpfmin(tmax, tmp___4);
    }
    if delta.y == 0.0f32 as cpFloat {
        if a.y < bb.b {
            tmp___5 = ::std::f32::INFINITY;
            return tmp___5 as cpFloat;
        } else {
            if bb.t < a.y {
                tmp___5 = ::std::f32::INFINITY;
                return tmp___5 as cpFloat;
            }
        }
    } else {
        t1___0 = (bb.b - a.y) / delta.y;
        t2___0 = (bb.t - a.y) / delta.y;
        tmp___6 = cpfmin(t1___0, t2___0);
        tmin = cpfmax(tmin, tmp___6);
        tmp___7 = cpfmax(t1___0, t2___0);
        tmax = cpfmin(tmax, tmp___7);
    }
    if tmin <= tmax {
        if 0.0f32 as cpFloat <= tmax {
            if tmin <= 1.0f32 as cpFloat {
                tmp___8 = cpfmax(tmin, 0.0f32 as cpFloat);
                return tmp___8;
            } else {
                tmp___9 = ::std::f32::INFINITY;
                return tmp___9 as cpFloat;
            }
        } else {
            tmp___9 = ::std::f32::INFINITY;
            return tmp___9 as cpFloat;
        }
    } else {
        tmp___9 = ::std::f32::INFINITY;
        return tmp___9 as cpFloat;
    };
}
#[inline]
unsafe extern "C" fn GetBB(mut tree: *mut cpBBTree, mut obj: *mut libc::c_void) -> cpBB {
    let mut bb: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut velocityFunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpVect> = None;
    let mut coef: cpFloat = 0.;
    let mut x: cpFloat = 0.;
    let mut y: cpFloat = 0.;
    let mut v: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut tmp___5: cpFloat = 0.;
    let mut tmp___6: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = (Some(((*tree).spatialIndex.bbfunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(obj);
    bb = tmp;
    velocityFunc = (*tree).velocityFunc;
    if velocityFunc.is_some() {
        coef = 0.1f32 as cpFloat;
        x = (bb.r - bb.l) * coef;
        y = (bb.t - bb.b) * coef;
        tmp___0 = (Some(velocityFunc.expect("non-null function pointer")))
            .expect("non-null function pointer")(obj);
        tmp___1 = cpvmult(tmp___0, 0.1f32 as cpFloat);
        v = tmp___1;
        tmp___2 = cpfmax(y, v.y);
        tmp___3 = cpfmax(x, v.x);
        tmp___4 = cpfmin(-y, v.y);
        tmp___5 = cpfmin(-x, v.x);
        tmp___6 = cpBBNew(
            bb.l + tmp___5,
            bb.b + tmp___4,
            bb.r + tmp___3,
            bb.t + tmp___2,
        );
        return tmp___6;
    } else {
        return bb
    };
}
#[inline]
unsafe extern "C" fn GetTree(mut index: *mut cpSpatialIndex) -> *mut cpBBTree {
    let mut tmp___0: *mut cpBBTree = 0 as *mut cpBBTree;
    let mut tmp___1: *mut cpSpatialIndexClass = 0 as *mut cpSpatialIndexClass;
    if !index.is_null() {
        tmp___1 = Klass();
        if (*index).klass as libc::c_ulong == tmp___1 as libc::c_ulong {
            tmp___0 = index as *mut cpBBTree;
        } else {
            tmp___0 = 0 as *mut libc::c_void as *mut cpBBTree;
        }
    } else {
        tmp___0 = 0 as *mut libc::c_void as *mut cpBBTree;
    }
    return tmp___0;
}
#[inline]
unsafe extern "C" fn GetRootIfTree(mut index: *mut cpSpatialIndex) -> *mut Node {
    let mut tmp___0: *mut Node = 0 as *mut Node;
    let mut tmp___1: *mut cpSpatialIndexClass = 0 as *mut cpSpatialIndexClass;
    if !index.is_null() {
        tmp___1 = Klass();
        if (*index).klass as libc::c_ulong == tmp___1 as libc::c_ulong {
            tmp___0 = (*(index as *mut cpBBTree)).root;
        } else {
            tmp___0 = 0 as *mut libc::c_void as *mut Node;
        }
    } else {
        tmp___0 = 0 as *mut libc::c_void as *mut Node;
    }
    return tmp___0;
}
#[inline]
unsafe extern "C" fn GetMasterTree(mut tree: *mut cpBBTree) -> *mut cpBBTree {
    let mut dynamicTree: *mut cpBBTree = 0 as *mut cpBBTree;
    let mut tmp: *mut cpBBTree = 0 as *mut cpBBTree;
    let mut tmp___0: *mut cpBBTree = 0 as *mut cpBBTree;
    tmp = GetTree((*tree).spatialIndex.dynamicIndex);
    dynamicTree = tmp;
    if !dynamicTree.is_null() {
        tmp___0 = dynamicTree;
    } else {
        tmp___0 = tree;
    }
    return tmp___0;
}
#[inline]
unsafe extern "C" fn IncrementStamp(mut tree: *mut cpBBTree) {
    let mut dynamicTree: *mut cpBBTree = 0 as *mut cpBBTree;
    let mut tmp: *mut cpBBTree = 0 as *mut cpBBTree;
    tmp = GetTree((*tree).spatialIndex.dynamicIndex);
    dynamicTree = tmp;
    if !dynamicTree.is_null() {
        (*dynamicTree).stamp = ((*dynamicTree).stamp).wrapping_add(1);
    } else {
        (*tree).stamp = ((*tree).stamp).wrapping_add(1);
    };
}
unsafe extern "C" fn PairRecycle(mut tree: *mut cpBBTree, mut pair: *mut Pair) {
    tree = GetMasterTree(tree);
    (*pair).a.next = (*tree).pooledPairs;
    (*tree).pooledPairs = pair;
}
unsafe extern "C" fn PairFromPool(mut tree: *mut cpBBTree) -> *mut Pair {
    let mut pair: *mut Pair = 0 as *mut Pair;
    let mut count: libc::c_int = 0;
    let mut buffer: *mut Pair = 0 as *mut Pair;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    tree = GetMasterTree(tree);
    pair = (*tree).pooledPairs;
    if !pair.is_null() {
        (*tree).pooledPairs = (*pair).a.next;
        return pair;
    } else {
        count = (32768 as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Pair>() as libc::c_ulong) as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../src/cpBBTree.c\0" as *const u8 as *const libc::c_char,
                157 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size is too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        tmp = calloc(1 as libc::c_int as size_t, 32768 as libc::c_int as size_t);
        buffer = tmp as *mut Pair;
        cpArrayPush((*tree).allocatedBuffers, buffer as *mut libc::c_void);
        i = 1 as libc::c_int;
        while i < count {
            PairRecycle(tree, buffer.offset(i as isize));
            i += 1;
        }
        return buffer;
    };
}
#[inline]
unsafe extern "C" fn ThreadUnlink(mut thread: Thread) {
    let mut next: *mut Pair = 0 as *mut Pair;
    let mut prev: *mut Pair = 0 as *mut Pair;
    next = thread.next;
    prev = thread.prev;
    if !next.is_null() {
        if (*next).a.leaf as libc::c_ulong == thread.leaf as libc::c_ulong {
            (*next).a.prev = prev;
        } else {
            (*next).b.prev = prev;
        }
    }
    if !prev.is_null() {
        if (*prev).a.leaf as libc::c_ulong == thread.leaf as libc::c_ulong {
            (*prev).a.next = next;
        } else {
            (*prev).b.next = next;
        }
    } else {
        (*thread.leaf).node.leaf.pairs = next;
    };
}
unsafe extern "C" fn PairsClear(mut leaf: *mut Node, mut tree: *mut cpBBTree) {
    let mut pair: *mut Pair = 0 as *mut Pair;
    let mut next: *mut Pair = 0 as *mut Pair;
    let mut next___0: *mut Pair = 0 as *mut Pair;
    pair = (*leaf).node.leaf.pairs;
    (*leaf).node.leaf.pairs = 0 as *mut libc::c_void as *mut Pair;
    while !pair.is_null() {
        if (*pair).a.leaf as libc::c_ulong == leaf as libc::c_ulong {
            next = (*pair).a.next;
            ThreadUnlink((*pair).b);
            PairRecycle(tree, pair);
            pair = next;
        } else {
            next___0 = (*pair).b.next;
            ThreadUnlink((*pair).a);
            PairRecycle(tree, pair);
            pair = next___0;
        }
    }
}
unsafe extern "C" fn PairInsert(
    mut a: *mut Node,
    mut b: *mut Node,
    mut tree: *mut cpBBTree,
) {
    let mut nextA: *mut Pair = 0 as *mut Pair;
    let mut nextB: *mut Pair = 0 as *mut Pair;
    let mut pair: *mut Pair = 0 as *mut Pair;
    let mut tmp: *mut Pair = 0 as *mut Pair;
    let mut temp: Pair = Pair {
        a: Thread {
            prev: 0 as *mut Pair,
            leaf: 0 as *mut Node,
            next: 0 as *mut Pair,
        },
        b: Thread {
            prev: 0 as *mut Pair,
            leaf: 0 as *mut Node,
            next: 0 as *mut Pair,
        },
        id: 0,
    };
    let mut tmp___0: *mut Pair = 0 as *mut Pair;
    nextA = (*a).node.leaf.pairs;
    nextB = (*b).node.leaf.pairs;
    tmp = PairFromPool(tree);
    pair = tmp;
    temp.a.prev = 0 as *mut libc::c_void as *mut Pair;
    temp.a.leaf = a;
    temp.a.next = nextA;
    temp.b.prev = 0 as *mut libc::c_void as *mut Pair;
    temp.b.leaf = b;
    temp.b.next = nextB;
    temp.id = 0 as libc::c_int as cpCollisionID;
    tmp___0 = pair;
    (*b).node.leaf.pairs = tmp___0;
    (*a).node.leaf.pairs = tmp___0;
    *pair = temp;
    if !nextA.is_null() {
        if (*nextA).a.leaf as libc::c_ulong == a as libc::c_ulong {
            (*nextA).a.prev = pair;
        } else {
            (*nextA).b.prev = pair;
        }
    }
    if !nextB.is_null() {
        if (*nextB).a.leaf as libc::c_ulong == b as libc::c_ulong {
            (*nextB).a.prev = pair;
        } else {
            (*nextB).b.prev = pair;
        }
    }
}
unsafe extern "C" fn NodeRecycle(mut tree: *mut cpBBTree, mut node: *mut Node) {
    (*node).parent = (*tree).pooledNodes;
    (*tree).pooledNodes = node;
}
unsafe extern "C" fn NodeFromPool(mut tree: *mut cpBBTree) -> *mut Node {
    let mut node: *mut Node = 0 as *mut Node;
    let mut count: libc::c_int = 0;
    let mut buffer: *mut Node = 0 as *mut Node;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    node = (*tree).pooledNodes;
    if !node.is_null() {
        (*tree).pooledNodes = (*node).parent;
        return node;
    } else {
        count = (32768 as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<Node>() as libc::c_ulong) as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../src/cpBBTree.c\0" as *const u8 as *const libc::c_char,
                246 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size is too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        tmp = calloc(1 as libc::c_int as size_t, 32768 as libc::c_int as size_t);
        buffer = tmp as *mut Node;
        cpArrayPush((*tree).allocatedBuffers, buffer as *mut libc::c_void);
        i = 1 as libc::c_int;
        while i < count {
            NodeRecycle(tree, buffer.offset(i as isize));
            i += 1;
        }
        return buffer;
    };
}
#[inline]
unsafe extern "C" fn NodeSetA(mut node: *mut Node, mut value: *mut Node) {
    (*node).node.children.a = value;
    (*value).parent = node;
}
#[inline]
unsafe extern "C" fn NodeSetB(mut node: *mut Node, mut value: *mut Node) {
    (*node).node.children.b = value;
    (*value).parent = node;
}
unsafe extern "C" fn NodeNew(
    mut tree: *mut cpBBTree,
    mut a: *mut Node,
    mut b: *mut Node,
) -> *mut Node {
    let mut node: *mut Node = 0 as *mut Node;
    let mut tmp: *mut Node = 0 as *mut Node;
    tmp = NodeFromPool(tree);
    node = tmp;
    (*node).obj = 0 as *mut libc::c_void;
    (*node).bb = cpBBMerge((*a).bb, (*b).bb);
    (*node).parent = 0 as *mut libc::c_void as *mut Node;
    NodeSetA(node, a);
    NodeSetB(node, b);
    return node;
}
#[inline]
unsafe extern "C" fn NodeIsLeaf(mut node: *mut Node) -> cpBool {
    return ((*node).obj as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn NodeOther(mut node: *mut Node, mut child: *mut Node) -> *mut Node {
    let mut tmp: *mut Node = 0 as *mut Node;
    if (*node).node.children.a as libc::c_ulong == child as libc::c_ulong {
        tmp = (*node).node.children.b;
    } else {
        tmp = (*node).node.children.a;
    }
    return tmp;
}
#[inline]
unsafe extern "C" fn NodeReplaceChild(
    mut parent: *mut Node,
    mut child: *mut Node,
    mut value: *mut Node,
    mut tree: *mut cpBBTree,
) {
    let mut node: *mut Node = 0 as *mut Node;
    if (*parent).node.children.a as libc::c_ulong == child as libc::c_ulong {
        NodeRecycle(tree, (*parent).node.children.a);
        NodeSetA(parent, value);
    } else {
        NodeRecycle(tree, (*parent).node.children.b);
        NodeSetB(parent, value);
    }
    node = parent;
    while !node.is_null() {
        (*node)
            .bb = cpBBMerge(
            (*(*node).node.children.a).bb,
            (*(*node).node.children.b).bb,
        );
        node = (*node).parent;
    }
}
#[inline]
unsafe extern "C" fn cpBBProximity(mut a: cpBB, mut b: cpBB) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    tmp = cpfabs(a.l + a.r - b.l - b.r);
    tmp___0 = cpfabs(a.b + a.t - b.b - b.t);
    return tmp + tmp___0;
}
unsafe extern "C" fn SubtreeInsert(
    mut subtree: *mut Node,
    mut leaf: *mut Node,
    mut tree: *mut cpBBTree,
) -> *mut Node {
    let mut tmp: *mut Node = 0 as *mut Node;
    let mut cost_a: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut cost_b: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut tmp___4: *mut Node = 0 as *mut Node;
    let mut tmp___5: *mut Node = 0 as *mut Node;
    let mut tmp___6: cpBool = 0;
    if subtree as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return leaf
    } else {
        tmp___6 = NodeIsLeaf(subtree);
        if tmp___6 != 0 {
            tmp = NodeNew(tree, leaf, subtree);
            return tmp;
        } else {
            tmp___0 = cpBBArea((*(*subtree).node.children.b).bb);
            tmp___1 = cpBBMergedArea((*(*subtree).node.children.a).bb, (*leaf).bb);
            cost_a = tmp___0 + tmp___1;
            tmp___2 = cpBBArea((*(*subtree).node.children.a).bb);
            tmp___3 = cpBBMergedArea((*(*subtree).node.children.b).bb, (*leaf).bb);
            cost_b = tmp___2 + tmp___3;
            if cost_a == cost_b {
                cost_a = cpBBProximity((*(*subtree).node.children.a).bb, (*leaf).bb);
                cost_b = cpBBProximity((*(*subtree).node.children.b).bb, (*leaf).bb);
            }
            if cost_b < cost_a {
                tmp___4 = SubtreeInsert((*subtree).node.children.b, leaf, tree);
                NodeSetB(subtree, tmp___4);
            } else {
                tmp___5 = SubtreeInsert((*subtree).node.children.a, leaf, tree);
                NodeSetA(subtree, tmp___5);
            }
            (*subtree).bb = cpBBMerge((*subtree).bb, (*leaf).bb);
            return subtree;
        }
    };
}
unsafe extern "C" fn SubtreeQuery(
    mut subtree: *mut Node,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    let mut tmp: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    tmp___0 = cpBBIntersects((*subtree).bb, bb);
    if tmp___0 != 0 {
        tmp = NodeIsLeaf(subtree);
        if tmp != 0 {
            (Some(func.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(obj, (*subtree).obj, 0 as libc::c_int as cpCollisionID, data);
        } else {
            SubtreeQuery((*subtree).node.children.a, obj, bb, func, data);
            SubtreeQuery((*subtree).node.children.b, obj, bb, func, data);
        }
    }
}
unsafe extern "C" fn SubtreeSegmentQuery(
    mut subtree: *mut Node,
    mut obj: *mut libc::c_void,
    mut a: cpVect,
    mut b: cpVect,
    mut t_exit: cpFloat,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> cpFloat,
    >,
    mut data: *mut libc::c_void,
) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    let mut t_a: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut t_b: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut tmp___5: cpFloat = 0.;
    let mut tmp___6: cpBool = 0;
    tmp___6 = NodeIsLeaf(subtree);
    if tmp___6 != 0 {
        tmp = (Some(func.expect("non-null function pointer")))
            .expect("non-null function pointer")(obj, (*subtree).obj, data);
        return tmp;
    } else {
        tmp___0 = cpBBSegmentQuery((*(*subtree).node.children.a).bb, a, b);
        t_a = tmp___0;
        tmp___1 = cpBBSegmentQuery((*(*subtree).node.children.b).bb, a, b);
        t_b = tmp___1;
        if t_a < t_b {
            if t_a < t_exit {
                tmp___2 = SubtreeSegmentQuery(
                    (*subtree).node.children.a,
                    obj,
                    a,
                    b,
                    t_exit,
                    func,
                    data,
                );
                t_exit = cpfmin(t_exit, tmp___2);
            }
            if t_b < t_exit {
                tmp___3 = SubtreeSegmentQuery(
                    (*subtree).node.children.b,
                    obj,
                    a,
                    b,
                    t_exit,
                    func,
                    data,
                );
                t_exit = cpfmin(t_exit, tmp___3);
            }
        } else {
            if t_b < t_exit {
                tmp___4 = SubtreeSegmentQuery(
                    (*subtree).node.children.b,
                    obj,
                    a,
                    b,
                    t_exit,
                    func,
                    data,
                );
                t_exit = cpfmin(t_exit, tmp___4);
            }
            if t_a < t_exit {
                tmp___5 = SubtreeSegmentQuery(
                    (*subtree).node.children.a,
                    obj,
                    a,
                    b,
                    t_exit,
                    func,
                    data,
                );
                t_exit = cpfmin(t_exit, tmp___5);
            }
        }
        return t_exit;
    };
}
unsafe extern "C" fn SubtreeRecycle(mut tree: *mut cpBBTree, mut node: *mut Node) {
    let mut tmp: cpBool = 0;
    tmp = NodeIsLeaf(node);
    if tmp == 0 {
        SubtreeRecycle(tree, (*node).node.children.a);
        SubtreeRecycle(tree, (*node).node.children.b);
        NodeRecycle(tree, node);
    }
}
#[inline]
unsafe extern "C" fn SubtreeRemove(
    mut subtree: *mut Node,
    mut leaf: *mut Node,
    mut tree: *mut cpBBTree,
) -> *mut Node {
    let mut parent: *mut Node = 0 as *mut Node;
    let mut other: *mut Node = 0 as *mut Node;
    let mut tmp: *mut Node = 0 as *mut Node;
    let mut tmp___0: *mut Node = 0 as *mut Node;
    if leaf as libc::c_ulong == subtree as libc::c_ulong {
        return 0 as *mut libc::c_void as *mut Node
    } else {
        parent = (*leaf).parent;
        if parent as libc::c_ulong == subtree as libc::c_ulong {
            tmp = NodeOther(subtree, leaf);
            other = tmp;
            (*other).parent = (*subtree).parent;
            NodeRecycle(tree, subtree);
            return other;
        } else {
            tmp___0 = NodeOther(parent, leaf);
            NodeReplaceChild((*parent).parent, parent, tmp___0, tree);
            return subtree;
        }
    };
}
unsafe extern "C" fn MarkLeafQuery(
    mut subtree: *mut Node,
    mut leaf: *mut Node,
    mut left: cpBool,
    mut context: *mut MarkContext,
) {
    let mut tmp: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    tmp___0 = cpBBIntersects((*leaf).bb, (*subtree).bb);
    if tmp___0 != 0 {
        tmp = NodeIsLeaf(subtree);
        if tmp != 0 {
            if left != 0 {
                PairInsert(leaf, subtree, (*context).tree);
            } else {
                if (*subtree).node.leaf.stamp < (*leaf).node.leaf.stamp {
                    PairInsert(subtree, leaf, (*context).tree);
                }
                (Some(((*context).func).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*leaf).obj,
                    (*subtree).obj,
                    0 as libc::c_int as cpCollisionID,
                    (*context).data,
                );
            }
        } else {
            MarkLeafQuery((*subtree).node.children.a, leaf, left, context);
            MarkLeafQuery((*subtree).node.children.b, leaf, left, context);
        }
    }
}
unsafe extern "C" fn MarkLeaf(mut leaf: *mut Node, mut context: *mut MarkContext) {
    let mut tree: *mut cpBBTree = 0 as *mut cpBBTree;
    let mut staticRoot: *mut Node = 0 as *mut Node;
    let mut node: *mut Node = 0 as *mut Node;
    let mut pair: *mut Pair = 0 as *mut Pair;
    let mut tmp: *mut cpBBTree = 0 as *mut cpBBTree;
    tree = (*context).tree;
    tmp = GetMasterTree(tree);
    if (*leaf).node.leaf.stamp == (*tmp).stamp {
        staticRoot = (*context).staticRoot;
        if !staticRoot.is_null() {
            MarkLeafQuery(staticRoot, leaf, 0 as libc::c_int as cpBool, context);
        }
        node = leaf;
        while !((*node).parent).is_null() {
            if node as libc::c_ulong
                == (*(*node).parent).node.children.a as libc::c_ulong
            {
                MarkLeafQuery(
                    (*(*node).parent).node.children.b,
                    leaf,
                    1 as libc::c_int as cpBool,
                    context,
                );
            } else {
                MarkLeafQuery(
                    (*(*node).parent).node.children.a,
                    leaf,
                    0 as libc::c_int as cpBool,
                    context,
                );
            }
            node = (*node).parent;
        }
    } else {
        pair = (*leaf).node.leaf.pairs;
        while !pair.is_null() {
            if leaf as libc::c_ulong == (*pair).b.leaf as libc::c_ulong {
                (*pair)
                    .id = (Some(((*context).func).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )((*(*pair).a.leaf).obj, (*leaf).obj, (*pair).id, (*context).data);
                pair = (*pair).b.next;
            } else {
                pair = (*pair).a.next;
            }
        }
    };
}
unsafe extern "C" fn MarkSubtree(mut subtree: *mut Node, mut context: *mut MarkContext) {
    let mut tmp: cpBool = 0;
    tmp = NodeIsLeaf(subtree);
    if tmp != 0 {
        MarkLeaf(subtree, context);
    } else {
        MarkSubtree((*subtree).node.children.a, context);
        MarkSubtree((*subtree).node.children.b, context);
    };
}
unsafe extern "C" fn LeafNew(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
) -> *mut Node {
    let mut node: *mut Node = 0 as *mut Node;
    let mut tmp: *mut Node = 0 as *mut Node;
    tmp = NodeFromPool(tree);
    node = tmp;
    (*node).obj = obj;
    (*node).bb = GetBB(tree, obj);
    (*node).parent = 0 as *mut libc::c_void as *mut Node;
    (*node).node.leaf.stamp = 0 as libc::c_int as cpTimestamp;
    (*node).node.leaf.pairs = 0 as *mut libc::c_void as *mut Pair;
    return node;
}
unsafe extern "C" fn LeafUpdate(mut leaf: *mut Node, mut tree: *mut cpBBTree) -> cpBool {
    let mut root: *mut Node = 0 as *mut Node;
    let mut bb: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp___0: *mut cpBBTree = 0 as *mut cpBBTree;
    let mut tmp___1: cpBool = 0;
    root = (*tree).root;
    tmp = (Some(((*tree).spatialIndex.bbfunc).expect("non-null function pointer")))
        .expect("non-null function pointer")((*leaf).obj);
    bb = tmp;
    tmp___1 = cpBBContainsBB((*leaf).bb, bb);
    if tmp___1 != 0 {
        return 0 as libc::c_int as cpBool
    } else {
        (*leaf).bb = GetBB(tree, (*leaf).obj);
        root = SubtreeRemove(root, leaf, tree);
        (*tree).root = SubtreeInsert(root, leaf, tree);
        PairsClear(leaf, tree);
        tmp___0 = GetMasterTree(tree);
        (*leaf).node.leaf.stamp = (*tmp___0).stamp;
        return 1 as libc::c_int as cpBool;
    };
}
unsafe extern "C" fn VoidQueryFunc(
    mut obj1: *mut libc::c_void,
    mut obj2: *mut libc::c_void,
    mut id: cpCollisionID,
    mut data: *mut libc::c_void,
) -> cpCollisionID {
    return id;
}
unsafe extern "C" fn LeafAddPairs(mut leaf: *mut Node, mut tree: *mut cpBBTree) {
    let mut dynamicIndex: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    let mut dynamicRoot: *mut Node = 0 as *mut Node;
    let mut tmp: *mut Node = 0 as *mut Node;
    let mut dynamicTree: *mut cpBBTree = 0 as *mut cpBBTree;
    let mut tmp___0: *mut cpBBTree = 0 as *mut cpBBTree;
    let mut context: MarkContext = MarkContext {
        tree: 0 as *mut cpBBTree,
        staticRoot: 0 as *mut Node,
        func: None,
        data: 0 as *mut libc::c_void,
    };
    let mut staticRoot: *mut Node = 0 as *mut Node;
    let mut tmp___1: *mut Node = 0 as *mut Node;
    let mut context___0: MarkContext = MarkContext {
        tree: 0 as *mut cpBBTree,
        staticRoot: 0 as *mut Node,
        func: None,
        data: 0 as *mut libc::c_void,
    };
    dynamicIndex = (*tree).spatialIndex.dynamicIndex;
    if !dynamicIndex.is_null() {
        tmp = GetRootIfTree(dynamicIndex);
        dynamicRoot = tmp;
        if !dynamicRoot.is_null() {
            tmp___0 = GetTree(dynamicIndex);
            dynamicTree = tmp___0;
            context.tree = dynamicTree;
            context.staticRoot = 0 as *mut libc::c_void as *mut Node;
            context
                .func = ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<
                    unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut libc::c_void,
                        cpCollisionID,
                        *mut libc::c_void,
                    ) -> cpCollisionID,
                >,
            >(0 as *mut libc::c_void);
            context.data = 0 as *mut libc::c_void;
            MarkLeafQuery(dynamicRoot, leaf, 1 as libc::c_int as cpBool, &mut context);
        }
    } else {
        tmp___1 = GetRootIfTree((*tree).spatialIndex.staticIndex);
        staticRoot = tmp___1;
        context___0.tree = tree;
        context___0.staticRoot = staticRoot;
        context___0
            .func = Some(
            VoidQueryFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
        );
        context___0.data = 0 as *mut libc::c_void;
        MarkLeaf(leaf, &mut context___0);
    };
}
pub unsafe extern "C" fn cpBBTreeAlloc() -> *mut cpBBTree {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpBBTree>() as libc::c_ulong,
    );
    return tmp as *mut cpBBTree;
}
unsafe extern "C" fn leafSetEql(
    mut obj: *mut libc::c_void,
    mut node: *mut Node,
) -> libc::c_int {
    return (obj as libc::c_ulong == (*node).obj as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn leafSetTrans(
    mut obj: *mut libc::c_void,
    mut tree: *mut cpBBTree,
) -> *mut libc::c_void {
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp___0: *mut Node = 0 as *mut Node;
    tmp = (Some(((*tree).spatialIndex.bbfunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(obj);
    tmp___0 = LeafNew(tree, obj, tmp);
    return tmp___0 as *mut libc::c_void;
}
pub unsafe extern "C" fn cpBBTreeInit(
    mut tree: *mut cpBBTree,
    mut bbfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    let mut tmp: *mut cpSpatialIndexClass = 0 as *mut cpSpatialIndexClass;
    tmp = Klass();
    cpSpatialIndexInit(tree as *mut cpSpatialIndex, tmp, bbfunc, staticIndex);
    (*tree)
        .velocityFunc = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpVect>,
    >(0 as *mut libc::c_void);
    (*tree)
        .leaves = cpHashSetNew(
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut Node) -> libc::c_int>,
            Option::<
                unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cpBool,
            >,
        >(
            Some(
                leafSetEql
                    as unsafe extern "C" fn(*mut libc::c_void, *mut Node) -> libc::c_int,
            ),
        ),
    );
    (*tree).root = 0 as *mut libc::c_void as *mut Node;
    (*tree).pooledNodes = 0 as *mut libc::c_void as *mut Node;
    (*tree).allocatedBuffers = cpArrayNew(0 as libc::c_int);
    (*tree).stamp = 0 as libc::c_int as cpTimestamp;
    return tree as *mut cpSpatialIndex;
}
pub unsafe extern "C" fn cpBBTreeSetVelocityFunc(
    mut index: *mut cpSpatialIndex,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpVect>,
) {
    let mut tmp: *mut cpSpatialIndexClass = 0 as *mut cpSpatialIndexClass;
    tmp = Klass();
    if (*index).klass as libc::c_ulong != tmp as libc::c_ulong {
        return;
    }
    let ref mut fresh4 = (*(index as *mut cpBBTree)).velocityFunc;
    *fresh4 = func;
}
pub unsafe extern "C" fn cpBBTreeNew(
    mut bbfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    let mut tmp: *mut cpBBTree = 0 as *mut cpBBTree;
    let mut tmp___0: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    tmp = cpBBTreeAlloc();
    tmp___0 = cpBBTreeInit(tmp, bbfunc, staticIndex);
    return tmp___0;
}
unsafe extern "C" fn cpBBTreeDestroy(mut tree: *mut cpBBTree) {
    cpHashSetFree((*tree).leaves);
    if !((*tree).allocatedBuffers).is_null() {
        cpArrayFreeEach(
            (*tree).allocatedBuffers,
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
    cpArrayFree((*tree).allocatedBuffers);
}
unsafe extern "C" fn cpBBTreeInsert(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut leaf: *mut Node = 0 as *mut Node;
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    let mut root: *mut Node = 0 as *mut Node;
    let mut tmp___0: *mut cpBBTree = 0 as *mut cpBBTree;
    tmp = cpHashSetInsert(
        (*tree).leaves,
        hashid,
        obj as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cpBBTree,
                ) -> *mut libc::c_void,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
        >(
            Some(
                leafSetTrans
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cpBBTree,
                    ) -> *mut libc::c_void,
            ),
        ),
        tree as *mut libc::c_void,
    );
    leaf = tmp as *mut Node;
    root = (*tree).root;
    (*tree).root = SubtreeInsert(root, leaf, tree);
    tmp___0 = GetMasterTree(tree);
    (*leaf).node.leaf.stamp = (*tmp___0).stamp;
    LeafAddPairs(leaf, tree);
    IncrementStamp(tree);
}
unsafe extern "C" fn cpBBTreeRemove(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut leaf: *mut Node = 0 as *mut Node;
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    tmp = cpHashSetRemove((*tree).leaves, hashid, obj as *const libc::c_void);
    leaf = tmp as *mut Node;
    (*tree).root = SubtreeRemove((*tree).root, leaf, tree);
    PairsClear(leaf, tree);
    NodeRecycle(tree, leaf);
}
unsafe extern "C" fn cpBBTreeContains(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) -> cpBool {
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    tmp = cpHashSetFind((*tree).leaves, hashid, obj as *const libc::c_void);
    return (tmp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as cpBool;
}
unsafe extern "C" fn LeafUpdateWrap(mut leaf: *mut Node, mut tree: *mut cpBBTree) {
    LeafUpdate(leaf, tree);
}
unsafe extern "C" fn cpBBTreeReindexQuery(
    mut tree: *mut cpBBTree,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    let mut staticIndex: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    let mut staticRoot: *mut Node = 0 as *mut Node;
    let mut tmp___0: *mut Node = 0 as *mut Node;
    let mut tmp___1: *mut cpSpatialIndexClass = 0 as *mut cpSpatialIndexClass;
    let mut context: MarkContext = MarkContext {
        tree: 0 as *mut cpBBTree,
        staticRoot: 0 as *mut Node,
        func: None,
        data: 0 as *mut libc::c_void,
    };
    if ((*tree).root).is_null() {
        return;
    }
    cpHashSetEach(
        (*tree).leaves,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Node, *mut cpBBTree) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(Some(LeafUpdateWrap as unsafe extern "C" fn(*mut Node, *mut cpBBTree) -> ())),
        tree as *mut libc::c_void,
    );
    staticIndex = (*tree).spatialIndex.staticIndex;
    if !staticIndex.is_null() {
        tmp___1 = Klass();
        if (*staticIndex).klass as libc::c_ulong == tmp___1 as libc::c_ulong {
            tmp___0 = (*(staticIndex as *mut cpBBTree)).root;
        } else {
            tmp___0 = 0 as *mut libc::c_void as *mut Node;
        }
    } else {
        tmp___0 = 0 as *mut libc::c_void as *mut Node;
    }
    staticRoot = tmp___0;
    context.tree = tree;
    context.staticRoot = staticRoot;
    context.func = func;
    context.data = data;
    MarkSubtree((*tree).root, &mut context);
    if !staticIndex.is_null() {
        if staticRoot.is_null() {
            cpSpatialIndexCollideStatic(
                tree as *mut cpSpatialIndex,
                staticIndex,
                func,
                data,
            );
        }
    }
    IncrementStamp(tree);
}
unsafe extern "C" fn cpBBTreeReindex(mut tree: *mut cpBBTree) {
    cpBBTreeReindexQuery(
        tree,
        Some(
            VoidQueryFunc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
        ),
        0 as *mut libc::c_void,
    );
}
unsafe extern "C" fn cpBBTreeReindexObject(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut leaf: *mut Node = 0 as *mut Node;
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    let mut tmp___0: cpBool = 0;
    tmp = cpHashSetFind((*tree).leaves, hashid, obj as *const libc::c_void);
    leaf = tmp as *mut Node;
    if !leaf.is_null() {
        tmp___0 = LeafUpdate(leaf, tree);
        if tmp___0 != 0 {
            LeafAddPairs(leaf, tree);
        }
        IncrementStamp(tree);
    }
}
unsafe extern "C" fn cpBBTreeSegmentQuery(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut a: cpVect,
    mut b: cpVect,
    mut t_exit: cpFloat,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> cpFloat,
    >,
    mut data: *mut libc::c_void,
) {
    let mut root: *mut Node = 0 as *mut Node;
    root = (*tree).root;
    if !root.is_null() {
        SubtreeSegmentQuery(root, obj, a, b, t_exit, func, data);
    }
}
unsafe extern "C" fn cpBBTreeQuery(
    mut tree: *mut cpBBTree,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    if !((*tree).root).is_null() {
        SubtreeQuery((*tree).root, obj, bb, func, data);
    }
}
unsafe extern "C" fn cpBBTreeCount(mut tree: *mut cpBBTree) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = cpHashSetCount((*tree).leaves);
    return tmp;
}
unsafe extern "C" fn each_helper(mut node: *mut Node, mut context: *mut eachContext) {
    (Some(((*context).func).expect("non-null function pointer")))
        .expect("non-null function pointer")((*node).obj, (*context).data);
}
unsafe extern "C" fn cpBBTreeEach(
    mut tree: *mut cpBBTree,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    let mut context: eachContext = eachContext {
        func: None,
        data: 0 as *mut libc::c_void,
    };
    context.func = func;
    context.data = data;
    cpHashSetEach(
        (*tree).leaves,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Node, *mut eachContext) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(Some(each_helper as unsafe extern "C" fn(*mut Node, *mut eachContext) -> ())),
        &mut context as *mut eachContext as *mut libc::c_void,
    );
}
static mut klass: cpSpatialIndexClass = unsafe {
    {
        let mut init = cpSpatialIndexClass {
            destroy: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpBBTree) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> ()>,
            >(Some(cpBBTreeDestroy as unsafe extern "C" fn(*mut cpBBTree) -> ())),
            count: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpBBTree) -> libc::c_int>,
                Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> libc::c_int>,
            >(Some(cpBBTreeCount as unsafe extern "C" fn(*mut cpBBTree) -> libc::c_int)),
            each: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpBBTreeEach
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                ) -> (),
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            contains: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> cpBool,
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> cpBool,
                >,
            >(
                Some(
                    cpBBTreeContains
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> cpBool,
                ),
            ),
            insert: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
            >(
                Some(
                    cpBBTreeInsert
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            remove: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
            >(
                Some(
                    cpBBTreeRemove
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindex: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpBBTree) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> ()>,
            >(Some(cpBBTreeReindex as unsafe extern "C" fn(*mut cpBBTree) -> ())),
            reindexObject: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
            >(
                Some(
                    cpBBTreeReindexObject
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindexQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpBBTreeReindexQuery
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                    cpCollisionID,
                                    *mut libc::c_void,
                                ) -> cpCollisionID,
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            query: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpBB,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpBB,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpBBTreeQuery
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpBB,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                    cpCollisionID,
                                    *mut libc::c_void,
                                ) -> cpCollisionID,
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpBBTree,
                        *mut libc::c_void,
                        cpVect,
                        cpVect,
                        cpFloat,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> cpFloat,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpVect,
                        cpVect,
                        cpFloat,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> cpFloat,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpBBTreeSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpBBTree,
                            *mut libc::c_void,
                            cpVect,
                            cpVect,
                            cpFloat,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                ) -> cpFloat,
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
#[inline]
unsafe extern "C" fn Klass() -> *mut cpSpatialIndexClass {
    return &mut klass;
}
unsafe extern "C" fn cpfcompare(
    mut a: *const cpFloat,
    mut b: *const cpFloat,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if *a < *b {
        tmp___0 = -(1 as libc::c_int);
    } else {
        if *b < *a {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
        tmp___0 = tmp;
    }
    return tmp___0;
}
unsafe extern "C" fn fillNodeArray(
    mut node: *mut Node,
    mut cursor: *mut *mut *mut Node,
) {
    **cursor = node;
    *cursor = (*cursor).offset(1);
}
unsafe extern "C" fn partitionNodes(
    mut tree: *mut cpBBTree,
    mut nodes: *mut *mut Node,
    mut count: libc::c_int,
) -> *mut Node {
    let mut tmp: *mut Node = 0 as *mut Node;
    let mut bb: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut i: libc::c_int = 0;
    let mut splitWidth: cpBool = 0;
    let mut bounds: *mut cpFloat = 0 as *mut cpFloat;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i___0: libc::c_int = 0;
    let mut i___1: libc::c_int = 0;
    let mut split: cpFloat = 0.;
    let mut a: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut b: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut right: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut node: *mut Node = 0 as *mut Node;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut node___0: *mut Node = 0 as *mut Node;
    let mut i___2: libc::c_int = 0;
    let mut tmp___3: *mut Node = 0 as *mut Node;
    let mut tmp___4: *mut Node = 0 as *mut Node;
    let mut tmp___5: *mut Node = 0 as *mut Node;
    if count == 1 as libc::c_int {
        return *nodes.offset(0 as libc::c_int as isize)
    } else {
        if count == 2 as libc::c_int {
            tmp = NodeNew(
                tree,
                *nodes.offset(0 as libc::c_int as isize),
                *nodes.offset(1 as libc::c_int as isize),
            );
            return tmp;
        }
    }
    bb = (**nodes.offset(0 as libc::c_int as isize)).bb;
    i = 1 as libc::c_int;
    while i < count {
        bb = cpBBMerge(bb, (**nodes.offset(i as isize)).bb);
        i += 1;
    }
    splitWidth = (bb.r - bb.l > bb.t - bb.b) as libc::c_int as cpBool;
    tmp___0 = calloc(
        (count * 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<cpFloat>() as libc::c_ulong,
    );
    bounds = tmp___0 as *mut cpFloat;
    if splitWidth != 0 {
        i___0 = 0 as libc::c_int;
        while i___0 < count {
            *bounds
                .offset(
                    (2 as libc::c_int * i___0) as isize,
                ) = (**nodes.offset(i___0 as isize)).bb.l;
            *bounds
                .offset(
                    (2 as libc::c_int * i___0 + 1 as libc::c_int) as isize,
                ) = (**nodes.offset(i___0 as isize)).bb.r;
            i___0 += 1;
        }
    } else {
        i___1 = 0 as libc::c_int;
        while i___1 < count {
            *bounds
                .offset(
                    (2 as libc::c_int * i___1) as isize,
                ) = (**nodes.offset(i___1 as isize)).bb.b;
            *bounds
                .offset(
                    (2 as libc::c_int * i___1 + 1 as libc::c_int) as isize,
                ) = (**nodes.offset(i___1 as isize)).bb.t;
            i___1 += 1;
        }
    }
    qsort(
        bounds as *mut libc::c_void,
        (count * 2 as libc::c_int) as size_t,
        ::std::mem::size_of::<cpFloat>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const cpFloat, *const cpFloat) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
            >,
        >(
            Some(
                cpfcompare
                    as unsafe extern "C" fn(
                        *const cpFloat,
                        *const cpFloat,
                    ) -> libc::c_int,
            ),
        ),
    );
    split = (*bounds.offset((count - 1 as libc::c_int) as isize)
        + *bounds.offset(count as isize)) * 0.5f32 as cpFloat;
    free(bounds as *mut libc::c_void);
    a = bb;
    b = bb;
    if splitWidth != 0 {
        b.l = split;
        a.r = b.l;
    } else {
        b.b = split;
        a.t = b.b;
    }
    right = count;
    left = 0 as libc::c_int;
    while left < right {
        node = *nodes.offset(left as isize);
        tmp___1 = cpBBMergedArea((*node).bb, b);
        tmp___2 = cpBBMergedArea((*node).bb, a);
        if tmp___1 < tmp___2 {
            right -= 1;
            let ref mut fresh5 = *nodes.offset(left as isize);
            *fresh5 = *nodes.offset(right as isize);
            let ref mut fresh6 = *nodes.offset(right as isize);
            *fresh6 = node;
        } else {
            left += 1;
        }
    }
    if right == count {
        node___0 = 0 as *mut libc::c_void as *mut Node;
        i___2 = 0 as libc::c_int;
        while i___2 < count {
            node___0 = SubtreeInsert(node___0, *nodes.offset(i___2 as isize), tree);
            i___2 += 1;
        }
        return node___0;
    }
    tmp___3 = partitionNodes(tree, nodes.offset(right as isize), count - right);
    tmp___4 = partitionNodes(tree, nodes, right);
    tmp___5 = NodeNew(tree, tmp___4, tmp___3);
    return tmp___5;
}
pub unsafe extern "C" fn cpBBTreeOptimize(mut index: *mut cpSpatialIndex) {
    let mut tree: *mut cpBBTree = 0 as *mut cpBBTree;
    let mut root: *mut Node = 0 as *mut Node;
    let mut count: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut nodes: *mut *mut Node = 0 as *mut *mut Node;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut cursor: *mut *mut Node = 0 as *mut *mut Node;
    if (*index).klass as libc::c_ulong
        != &mut klass as *mut cpSpatialIndexClass as libc::c_ulong
    {
        return;
    }
    tree = index as *mut cpBBTree;
    root = (*tree).root;
    if root.is_null() {
        return;
    }
    tmp = cpBBTreeCount(tree);
    count = tmp;
    tmp___0 = calloc(
        count as size_t,
        ::std::mem::size_of::<*mut Node>() as libc::c_ulong,
    );
    nodes = tmp___0 as *mut *mut Node;
    cursor = nodes;
    cpHashSetEach(
        (*tree).leaves,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut Node, *mut *mut *mut Node) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                fillNodeArray
                    as unsafe extern "C" fn(*mut Node, *mut *mut *mut Node) -> (),
            ),
        ),
        &mut cursor as *mut *mut *mut Node as *mut libc::c_void,
    );
    SubtreeRecycle(tree, root);
    (*tree).root = partitionNodes(tree, nodes, count);
    free(nodes as *mut libc::c_void);
}
static mut cpvzero___1: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpvforangle(a: cpFloat) -> cpVect {
    let mut tmp: libc::c_double = 0.;
    let mut tmp___0: libc::c_double = 0.;
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    tmp = sin(a);
    tmp___0 = cos(a);
    tmp___1 = cpv(tmp___0, tmp);
    return tmp___1;
}
#[inline]
unsafe extern "C" fn cpvdistsq(v1: cpVect, v2: cpVect) -> cpFloat {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpFloat = 0.;
    tmp = cpvsub(v1, v2);
    tmp___0 = cpvlengthsq(tmp);
    return tmp___0;
}
#[inline]
unsafe extern "C" fn cpTransformNewTranspose(
    mut a: cpFloat,
    mut c: cpFloat,
    mut tx: cpFloat,
    mut b: cpFloat,
    mut d: cpFloat,
    mut ty: cpFloat,
) -> cpTransform {
    let mut t: cpTransform = cpTransform {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        tx: 0.,
        ty: 0.,
    };
    t.a = a;
    t.b = b;
    t.c = c;
    t.d = d;
    t.tx = tx;
    t.ty = ty;
    return t;
}
#[inline]
unsafe extern "C" fn cpTransformPoint(mut t: cpTransform, mut p: cpVect) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(t.a * p.x + t.c * p.y + t.tx, t.b * p.x + t.d * p.y + t.ty);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpTransformVect(mut t: cpTransform, mut v: cpVect) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(t.a * v.x + t.c * v.y, t.b * v.x + t.d * v.y);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpTransformRigidInverse(mut t: cpTransform) -> cpTransform {
    let mut tmp: cpTransform = cpTransform {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        tx: 0.,
        ty: 0.,
    };
    tmp = cpTransformNewTranspose(
        t.d,
        -t.c,
        t.c * t.ty - t.tx * t.d,
        -t.b,
        t.a,
        t.tx * t.b - t.a * t.ty,
    );
    return tmp;
}
#[inline]
unsafe extern "C" fn cpSpatialIndexInsert(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    (Some(((*(*index).klass).insert).expect("non-null function pointer")))
        .expect("non-null function pointer")(index, obj, hashid);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexRemove(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    (Some(((*(*index).klass).remove).expect("non-null function pointer")))
        .expect("non-null function pointer")(index, obj, hashid);
}
#[inline]
unsafe extern "C" fn cpSpaceArrayForBodyType(
    mut space: *mut cpSpace,
    mut type_0: cpBodyType,
) -> *mut cpArray {
    let mut tmp: *mut cpArray = 0 as *mut cpArray;
    if type_0 as libc::c_uint == 2 as libc::c_uint {
        tmp = (*space).staticBodies;
    } else {
        tmp = (*space).dynamicBodies;
    }
    return tmp;
}
#[inline]
unsafe extern "C" fn cpConstraintNext(
    mut node: *mut cpConstraint,
    mut body: *mut cpBody,
) -> *mut cpConstraint {
    let mut tmp: *mut cpConstraint = 0 as *mut cpConstraint;
    if (*node).a as libc::c_ulong == body as libc::c_ulong {
        tmp = (*node).next_a;
    } else {
        tmp = (*node).next_b;
    }
    return tmp;
}
#[inline]
unsafe extern "C" fn cpArbiterNext(
    mut node: *mut cpArbiter,
    mut body: *mut cpBody,
) -> *mut cpArbiter {
    let mut tmp: *mut cpArbiter = 0 as *mut cpArbiter;
    if (*node).body_a as libc::c_ulong == body as libc::c_ulong {
        tmp = (*node).thread_a.next;
    } else {
        tmp = (*node).thread_b.next;
    }
    return tmp;
}
pub unsafe extern "C" fn cpBodyAlloc() -> *mut cpBody {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpBody>() as libc::c_ulong,
    );
    return tmp as *mut cpBody;
}
pub unsafe extern "C" fn cpBodyInit(
    mut body: *mut cpBody,
    mut mass: cpFloat,
    mut moment: cpFloat,
) -> *mut cpBody {
    (*body).space = 0 as *mut libc::c_void as *mut cpSpace;
    (*body).shapeList = 0 as *mut libc::c_void as *mut cpShape;
    (*body).arbiterList = 0 as *mut libc::c_void as *mut cpArbiter;
    (*body).constraintList = 0 as *mut libc::c_void as *mut cpConstraint;
    (*body)
        .velocity_func = Some(
        cpBodyUpdateVelocity
            as unsafe extern "C" fn(*mut cpBody, cpVect, cpFloat, cpFloat) -> (),
    );
    (*body)
        .position_func = Some(
        cpBodyUpdatePosition as unsafe extern "C" fn(*mut cpBody, cpFloat) -> (),
    );
    (*body).sleeping.root = 0 as *mut libc::c_void as *mut cpBody;
    (*body).sleeping.next = 0 as *mut libc::c_void as *mut cpBody;
    (*body).sleeping.idleTime = 0.0f32 as cpFloat;
    (*body).p = cpvzero___1;
    (*body).v = cpvzero___1;
    (*body).f = cpvzero___1;
    (*body).w = 0.0f32 as cpFloat;
    (*body).t = 0.0f32 as cpFloat;
    (*body).v_bias = cpvzero___1;
    (*body).w_bias = 0.0f32 as cpFloat;
    (*body).userData = 0 as *mut libc::c_void;
    cpBodySetMass(body, mass);
    cpBodySetMoment(body, moment);
    cpBodySetAngle(body, 0.0f32 as cpFloat);
    return body;
}
pub unsafe extern "C" fn cpBodyNew(
    mut mass: cpFloat,
    mut moment: cpFloat,
) -> *mut cpBody {
    let mut tmp: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___0: *mut cpBody = 0 as *mut cpBody;
    tmp = cpBodyAlloc();
    tmp___0 = cpBodyInit(tmp, mass, moment);
    return tmp___0;
}
pub unsafe extern "C" fn cpBodyNewKinematic() -> *mut cpBody {
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: *mut cpBody = 0 as *mut cpBody;
    tmp = cpBodyNew(0.0f32 as cpFloat, 0.0f32 as cpFloat);
    body = tmp;
    cpBodySetType(body, CP_BODY_TYPE_KINEMATIC);
    return body;
}
pub unsafe extern "C" fn cpBodyNewStatic() -> *mut cpBody {
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: *mut cpBody = 0 as *mut cpBody;
    tmp = cpBodyNew(0.0f32 as cpFloat, 0.0f32 as cpFloat);
    body = tmp;
    cpBodySetType(body, CP_BODY_TYPE_STATIC);
    return body;
}
pub unsafe extern "C" fn cpBodyDestroy(mut body: *mut cpBody) {}
pub unsafe extern "C" fn cpBodyFree(mut body: *mut cpBody) {
    if !body.is_null() {
        cpBodyDestroy(body);
        free(body as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpBodyIsSleeping(mut body: *const cpBody) -> cpBool {
    return ((*body).sleeping.root as libc::c_ulong != 0 as *mut cpBody as libc::c_ulong)
        as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpBodyGetType(mut body: *mut cpBody) -> cpBodyType {
    let mut tmp: libc::c_float = 0.;
    let mut tmp___0: libc::c_float = 0.;
    tmp___0 = ::std::f32::INFINITY;
    if (*body).sleeping.idleTime == tmp___0 as cpFloat {
        return CP_BODY_TYPE_STATIC
    } else {
        tmp = ::std::f32::INFINITY;
        if (*body).m == tmp as cpFloat {
            return CP_BODY_TYPE_KINEMATIC
        } else {
            return CP_BODY_TYPE_DYNAMIC
        }
    };
}
pub unsafe extern "C" fn cpBodySetType(mut body: *mut cpBody, mut type_0: cpBodyType) {
    let mut oldType: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___0: libc::c_float = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: libc::c_float = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut tmp___5: libc::c_float = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut space: *mut cpSpace = 0 as *mut cpSpace;
    let mut tmp___7: *mut cpSpace = 0 as *mut cpSpace;
    let mut fromArray: *mut cpArray = 0 as *mut cpArray;
    let mut tmp___8: *mut cpArray = 0 as *mut cpArray;
    let mut toArray: *mut cpArray = 0 as *mut cpArray;
    let mut tmp___9: *mut cpArray = 0 as *mut cpArray;
    let mut fromIndex: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    let mut tmp___10: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    let mut toIndex: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    let mut tmp___11: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    tmp = cpBodyGetType(body);
    oldType = tmp;
    if oldType as libc::c_uint == type_0 as libc::c_uint {
        return;
    }
    if type_0 as libc::c_uint == 2 as libc::c_uint {
        tmp___0 = ::std::f32::INFINITY;
        (*body).sleeping.idleTime = tmp___0 as cpFloat;
    } else {
        (*body).sleeping.idleTime = 0.0f32 as cpFloat;
    }
    if type_0 as libc::c_uint == 0 as libc::c_uint {
        tmp___1 = 0.0f32 as cpFloat;
        (*body).i = tmp___1;
        (*body).m = tmp___1;
        tmp___3 = ::std::f32::INFINITY;
        tmp___2 = tmp___3 as cpFloat;
        (*body).i_inv = tmp___2;
        (*body).m_inv = tmp___2;
        cpBodyAccumulateMassFromShapes(body);
    } else {
        tmp___5 = ::std::f32::INFINITY;
        tmp___4 = tmp___5 as cpFloat;
        (*body).i = tmp___4;
        (*body).m = tmp___4;
        tmp___6 = 0.0f32 as cpFloat;
        (*body).i_inv = tmp___6;
        (*body).m_inv = tmp___6;
        (*body).v = cpvzero___1;
        (*body).w = 0.0f32 as cpFloat;
    }
    tmp___7 = cpBodyGetSpace(body as *const cpBody);
    space = tmp___7;
    if space as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if (*space).locked != 0 {
            cpMessage(
                b"!space->locked\0" as *const u8 as *const libc::c_char,
                b"../src/cpBody.c\0" as *const u8 as *const libc::c_char,
                174 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
        if !(oldType as libc::c_uint == 2 as libc::c_uint) {
            cpBodyActivate(body);
        }
        tmp___8 = cpSpaceArrayForBodyType(space, oldType);
        fromArray = tmp___8;
        tmp___9 = cpSpaceArrayForBodyType(space, type_0);
        toArray = tmp___9;
        if fromArray as libc::c_ulong != toArray as libc::c_ulong {
            cpArrayDeleteObj(fromArray, body as *mut libc::c_void);
            cpArrayPush(toArray, body as *mut libc::c_void);
        }
        if oldType as libc::c_uint == 2 as libc::c_uint {
            tmp___10 = (*space).staticShapes;
        } else {
            tmp___10 = (*space).dynamicShapes;
        }
        fromIndex = tmp___10;
        if type_0 as libc::c_uint == 2 as libc::c_uint {
            tmp___11 = (*space).staticShapes;
        } else {
            tmp___11 = (*space).dynamicShapes;
        }
        toIndex = tmp___11;
        if fromIndex as libc::c_ulong != toIndex as libc::c_ulong {
            shape = (*body).shapeList;
            while !shape.is_null() {
                cpSpatialIndexRemove(
                    fromIndex,
                    shape as *mut libc::c_void,
                    (*shape).hashid,
                );
                cpSpatialIndexInsert(
                    toIndex,
                    shape as *mut libc::c_void,
                    (*shape).hashid,
                );
                shape = (*shape).next;
            }
        }
    }
}
pub unsafe extern "C" fn cpBodyAccumulateMassFromShapes(mut body: *mut cpBody) {
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___0: cpFloat = 0.;
    let mut pos: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    let mut info: *mut cpShapeMassInfo = 0 as *mut cpShapeMassInfo;
    let mut m: cpFloat = 0.;
    let mut msum: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    if body as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        return
    } else {
        tmp = cpBodyGetType(body);
        if tmp as libc::c_uint != 0 as libc::c_uint {
            return;
        }
    }
    tmp___0 = 0.0f32 as cpFloat;
    (*body).i = tmp___0;
    (*body).m = tmp___0;
    (*body).cog = cpvzero___1;
    tmp___1 = cpBodyGetPosition(body as *const cpBody);
    pos = tmp___1;
    shape = (*body).shapeList;
    while !shape.is_null() {
        info = &mut (*shape).massInfo;
        m = (*info).m;
        if m > 0.0f32 as cpFloat {
            msum = (*body).m + m;
            tmp___2 = cpvdistsq((*body).cog, (*info).cog);
            (*body).i += m * (*info).i + tmp___2 * (m * (*body).m) / msum;
            (*body).cog = cpvlerp((*body).cog, (*info).cog, m / msum);
            (*body).m = msum;
        }
        shape = (*shape).next;
    }
    (*body).m_inv = 1.0f32 as cpFloat / (*body).m;
    (*body).i_inv = 1.0f32 as cpFloat / (*body).i;
    cpBodySetPosition(body, pos);
}
pub unsafe extern "C" fn cpBodyGetSpace(mut body: *const cpBody) -> *mut cpSpace {
    return (*body).space;
}
pub unsafe extern "C" fn cpBodyGetMass(mut body: *const cpBody) -> cpFloat {
    return (*body).m;
}
pub unsafe extern "C" fn cpBodySetMass(mut body: *mut cpBody, mut mass: cpFloat) {
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___0: libc::c_float = 0.;
    let mut tmp___1: libc::c_float = 0.;
    tmp = cpBodyGetType(body);
    if !(tmp as libc::c_uint == 0 as libc::c_uint) {
        cpMessage(
            b"cpBodyGetType(body) == CP_BODY_TYPE_DYNAMIC\0" as *const u8
                as *const libc::c_char,
            b"../src/cpBody.c\0" as *const u8 as *const libc::c_char,
            256 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You cannot set the mass of kinematic or static bodies.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if 0.0f32 as cpFloat <= mass {
        tmp___0 = ::std::f32::INFINITY;
        if !(mass < tmp___0 as cpFloat) {
            cpMessage(
                b"0.0f <= mass && mass < INFINITY\0" as *const u8 as *const libc::c_char,
                b"../src/cpBody.c\0" as *const u8 as *const libc::c_char,
                257 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Mass must be positive and finite.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
    } else {
        cpMessage(
            b"0.0f <= mass && mass < INFINITY\0" as *const u8 as *const libc::c_char,
            b"../src/cpBody.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Mass must be positive and finite.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate(body);
    (*body).m = mass;
    if mass == 0.0f32 as cpFloat {
        tmp___1 = ::std::f32::INFINITY;
        (*body).m_inv = tmp___1 as cpFloat;
    } else {
        (*body).m_inv = 1.0f32 as cpFloat / mass;
    };
}
pub unsafe extern "C" fn cpBodyGetMoment(mut body: *const cpBody) -> cpFloat {
    return (*body).i;
}
pub unsafe extern "C" fn cpBodySetMoment(mut body: *mut cpBody, mut moment: cpFloat) {
    let mut tmp: libc::c_float = 0.;
    if !(moment >= 0.0f32 as cpFloat) {
        cpMessage(
            b"moment >= 0.0f\0" as *const u8 as *const libc::c_char,
            b"../src/cpBody.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Moment of Inertia must be positive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate(body);
    (*body).i = moment;
    if moment == 0.0f32 as cpFloat {
        tmp = ::std::f32::INFINITY;
        (*body).i_inv = tmp as cpFloat;
    } else {
        (*body).i_inv = 1.0f32 as cpFloat / moment;
    };
}
pub unsafe extern "C" fn cpBodyGetRotation(mut body: *const cpBody) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv((*body).transform.a, (*body).transform.b);
    return tmp;
}
pub unsafe extern "C" fn cpBodyAddShape(mut body: *mut cpBody, mut shape: *mut cpShape) {
    let mut next: *mut cpShape = 0 as *mut cpShape;
    next = (*body).shapeList;
    if !next.is_null() {
        (*next).prev = shape;
    }
    (*shape).next = next;
    (*body).shapeList = shape;
    if (*shape).massInfo.m > 0.0f32 as cpFloat {
        cpBodyAccumulateMassFromShapes(body);
    }
}
pub unsafe extern "C" fn cpBodyRemoveShape(
    mut body: *mut cpBody,
    mut shape: *mut cpShape,
) {
    let mut prev: *mut cpShape = 0 as *mut cpShape;
    let mut next: *mut cpShape = 0 as *mut cpShape;
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    prev = (*shape).prev;
    next = (*shape).next;
    if !prev.is_null() {
        (*prev).next = next;
    } else {
        (*body).shapeList = next;
    }
    if !next.is_null() {
        (*next).prev = prev;
    }
    (*shape).prev = 0 as *mut libc::c_void as *mut cpShape;
    (*shape).next = 0 as *mut libc::c_void as *mut cpShape;
    tmp = cpBodyGetType(body);
    if tmp as libc::c_uint == 0 as libc::c_uint {
        if (*shape).massInfo.m > 0.0f32 as cpFloat {
            cpBodyAccumulateMassFromShapes(body);
        }
    }
}
unsafe extern "C" fn filterConstraints(
    mut node: *mut cpConstraint,
    mut body: *mut cpBody,
    mut filter: *mut cpConstraint,
) -> *mut cpConstraint {
    let mut tmp: *mut cpConstraint = 0 as *mut cpConstraint;
    if node as libc::c_ulong == filter as libc::c_ulong {
        tmp = cpConstraintNext(node, body);
        return tmp;
    } else {
        if (*node).a as libc::c_ulong == body as libc::c_ulong {
            (*node).next_a = filterConstraints((*node).next_a, body, filter);
        } else {
            (*node).next_b = filterConstraints((*node).next_b, body, filter);
        }
    }
    return node;
}
pub unsafe extern "C" fn cpBodyRemoveConstraint(
    mut body: *mut cpBody,
    mut constraint: *mut cpConstraint,
) {
    (*body).constraintList = filterConstraints((*body).constraintList, body, constraint);
}
unsafe extern "C" fn SetTransform(mut body: *mut cpBody, mut p: cpVect, mut a: cpFloat) {
    let mut rot: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut c: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvforangle(a);
    rot = tmp;
    c = (*body).cog;
    (*body)
        .transform = cpTransformNewTranspose(
        rot.x,
        -rot.y,
        p.x - (c.x * rot.x - c.y * rot.y),
        rot.y,
        rot.x,
        p.y - (c.x * rot.y + c.y * rot.x),
    );
}
#[inline]
unsafe extern "C" fn SetAngle(mut body: *mut cpBody, mut a: cpFloat) -> cpFloat {
    (*body).a = a;
    return a;
}
pub unsafe extern "C" fn cpBodyGetPosition(mut body: *const cpBody) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpTransformPoint((*body).transform, cpvzero___1);
    return tmp;
}
pub unsafe extern "C" fn cpBodySetPosition(mut body: *mut cpBody, mut position: cpVect) {
    let mut p: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    cpBodyActivate(body);
    tmp___0 = cpTransformVect((*body).transform, (*body).cog);
    tmp = cpvadd(tmp___0, position);
    (*body).p = tmp;
    p = tmp;
    SetTransform(body, p, (*body).a);
}
pub unsafe extern "C" fn cpBodyGetCenterOfGravity(mut body: *const cpBody) -> cpVect {
    return (*body).cog;
}
pub unsafe extern "C" fn cpBodySetCenterOfGravity(
    mut body: *mut cpBody,
    mut cog: cpVect,
) {
    cpBodyActivate(body);
    (*body).cog = cog;
}
pub unsafe extern "C" fn cpBodyGetVelocity(mut body: *const cpBody) -> cpVect {
    return (*body).v;
}
pub unsafe extern "C" fn cpBodySetVelocity(mut body: *mut cpBody, mut velocity: cpVect) {
    cpBodyActivate(body);
    (*body).v = velocity;
}
pub unsafe extern "C" fn cpBodyGetForce(mut body: *const cpBody) -> cpVect {
    return (*body).f;
}
pub unsafe extern "C" fn cpBodySetForce(mut body: *mut cpBody, mut force: cpVect) {
    cpBodyActivate(body);
    (*body).f = force;
}
pub unsafe extern "C" fn cpBodyGetAngle(mut body: *const cpBody) -> cpFloat {
    return (*body).a;
}
pub unsafe extern "C" fn cpBodySetAngle(mut body: *mut cpBody, mut angle: cpFloat) {
    cpBodyActivate(body);
    SetAngle(body, angle);
    SetTransform(body, (*body).p, angle);
}
pub unsafe extern "C" fn cpBodyGetAngularVelocity(mut body: *const cpBody) -> cpFloat {
    return (*body).w;
}
pub unsafe extern "C" fn cpBodySetAngularVelocity(
    mut body: *mut cpBody,
    mut angularVelocity: cpFloat,
) {
    cpBodyActivate(body);
    (*body).w = angularVelocity;
}
pub unsafe extern "C" fn cpBodyGetTorque(mut body: *const cpBody) -> cpFloat {
    return (*body).t;
}
pub unsafe extern "C" fn cpBodySetTorque(mut body: *mut cpBody, mut torque: cpFloat) {
    cpBodyActivate(body);
    (*body).t = torque;
}
pub unsafe extern "C" fn cpBodyGetUserData(mut body: *const cpBody) -> cpDataPointer {
    return (*body).userData;
}
pub unsafe extern "C" fn cpBodySetUserData(
    mut body: *mut cpBody,
    mut userData: cpDataPointer,
) {
    (*body).userData = userData;
}
pub unsafe extern "C" fn cpBodySetVelocityUpdateFunc(
    mut body: *mut cpBody,
    mut velocityFunc: Option::<
        unsafe extern "C" fn(*mut cpBody, cpVect, cpFloat, cpFloat) -> (),
    >,
) {
    (*body).velocity_func = velocityFunc;
}
pub unsafe extern "C" fn cpBodySetPositionUpdateFunc(
    mut body: *mut cpBody,
    mut positionFunc: Option::<unsafe extern "C" fn(*mut cpBody, cpFloat) -> ()>,
) {
    (*body).position_func = positionFunc;
}
pub unsafe extern "C" fn cpBodyUpdateVelocity(
    mut body: *mut cpBody,
    mut gravity: cpVect,
    mut damping: cpFloat,
    mut dt: cpFloat,
) {
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpBodyGetType(body);
    if tmp as libc::c_uint == 1 as libc::c_uint {
        return;
    }
    tmp___0 = cpvmult((*body).f, (*body).m_inv);
    tmp___1 = cpvadd(gravity, tmp___0);
    tmp___2 = cpvmult(tmp___1, dt);
    tmp___3 = cpvmult((*body).v, damping);
    (*body).v = cpvadd(tmp___3, tmp___2);
    (*body).w = (*body).w * damping + (*body).t * (*body).i_inv * dt;
    (*body).f = cpvzero___1;
    (*body).t = 0.0f32 as cpFloat;
}
pub unsafe extern "C" fn cpBodyUpdatePosition(mut body: *mut cpBody, mut dt: cpFloat) {
    let mut p: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut a: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    tmp___0 = cpvadd((*body).v, (*body).v_bias);
    tmp___1 = cpvmult(tmp___0, dt);
    tmp = cpvadd((*body).p, tmp___1);
    (*body).p = tmp;
    p = tmp;
    tmp___2 = SetAngle(body, (*body).a + ((*body).w + (*body).w_bias) * dt);
    a = tmp___2;
    SetTransform(body, p, a);
    (*body).v_bias = cpvzero___1;
    (*body).w_bias = 0.0f32 as cpFloat;
}
pub unsafe extern "C" fn cpBodyLocalToWorld(
    mut body: *const cpBody,
    point: cpVect,
) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpTransformPoint((*body).transform, point);
    return tmp;
}
pub unsafe extern "C" fn cpBodyWorldToLocal(
    mut body: *const cpBody,
    point: cpVect,
) -> cpVect {
    let mut tmp: cpTransform = cpTransform {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
        tx: 0.,
        ty: 0.,
    };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpTransformRigidInverse((*body).transform);
    tmp___0 = cpTransformPoint(tmp, point);
    return tmp___0;
}
pub unsafe extern "C" fn cpBodyApplyForceAtWorldPoint(
    mut body: *mut cpBody,
    mut force: cpVect,
    mut point: cpVect,
) {
    let mut r: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpFloat = 0.;
    cpBodyActivate(body);
    (*body).f = cpvadd((*body).f, force);
    tmp = cpTransformPoint((*body).transform, (*body).cog);
    tmp___0 = cpvsub(point, tmp);
    r = tmp___0;
    tmp___1 = cpvcross(r, force);
    (*body).t += tmp___1;
}
pub unsafe extern "C" fn cpBodyApplyForceAtLocalPoint(
    mut body: *mut cpBody,
    mut force: cpVect,
    mut point: cpVect,
) {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpTransformPoint((*body).transform, point);
    tmp___0 = cpTransformVect((*body).transform, force);
    cpBodyApplyForceAtWorldPoint(body, tmp___0, tmp);
}
pub unsafe extern "C" fn cpBodyApplyImpulseAtWorldPoint(
    mut body: *mut cpBody,
    mut impulse: cpVect,
    mut point: cpVect,
) {
    let mut r: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    cpBodyActivate(body);
    tmp = cpTransformPoint((*body).transform, (*body).cog);
    tmp___0 = cpvsub(point, tmp);
    r = tmp___0;
    apply_impulse(body, impulse, r);
}
pub unsafe extern "C" fn cpBodyApplyImpulseAtLocalPoint(
    mut body: *mut cpBody,
    mut impulse: cpVect,
    mut point: cpVect,
) {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpTransformPoint((*body).transform, point);
    tmp___0 = cpTransformVect((*body).transform, impulse);
    cpBodyApplyImpulseAtWorldPoint(body, tmp___0, tmp);
}
pub unsafe extern "C" fn cpBodyGetVelocityAtLocalPoint(
    mut body: *const cpBody,
    mut point: cpVect,
) -> cpVect {
    let mut r: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvsub(point, (*body).cog);
    tmp___0 = cpTransformVect((*body).transform, tmp);
    r = tmp___0;
    tmp___1 = cpvperp(r);
    tmp___2 = cpvmult(tmp___1, (*body).w);
    tmp___3 = cpvadd((*body).v, tmp___2);
    return tmp___3;
}
pub unsafe extern "C" fn cpBodyGetVelocityAtWorldPoint(
    mut body: *const cpBody,
    mut point: cpVect,
) -> cpVect {
    let mut r: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpTransformPoint((*body).transform, (*body).cog);
    tmp___0 = cpvsub(point, tmp);
    r = tmp___0;
    tmp___1 = cpvperp(r);
    tmp___2 = cpvmult(tmp___1, (*body).w);
    tmp___3 = cpvadd((*body).v, tmp___2);
    return tmp___3;
}
pub unsafe extern "C" fn cpBodyKineticEnergy(mut body: *const cpBody) -> cpFloat {
    let mut vsq: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    let mut wsq: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    tmp = cpvdot((*body).v, (*body).v);
    vsq = tmp;
    wsq = (*body).w * (*body).w;
    if vsq != 0. {
        tmp___0 = vsq * (*body).m;
    } else {
        tmp___0 = 0.0f32 as cpFloat;
    }
    if wsq != 0. {
        tmp___1 = wsq * (*body).i;
    } else {
        tmp___1 = 0.0f32 as cpFloat;
    }
    return tmp___0 + tmp___1;
}
pub unsafe extern "C" fn cpBodyEachShape(
    mut body: *mut cpBody,
    mut func: Option::<
        unsafe extern "C" fn(*mut cpBody, *mut cpShape, *mut libc::c_void) -> (),
    >,
    mut data: *mut libc::c_void,
) {
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    let mut next: *mut cpShape = 0 as *mut cpShape;
    shape = (*body).shapeList;
    while !shape.is_null() {
        next = (*shape).next;
        (Some(func.expect("non-null function pointer")))
            .expect("non-null function pointer")(body, shape, data);
        shape = next;
    }
}
pub unsafe extern "C" fn cpBodyEachConstraint(
    mut body: *mut cpBody,
    mut func: Option::<
        unsafe extern "C" fn(*mut cpBody, *mut cpConstraint, *mut libc::c_void) -> (),
    >,
    mut data: *mut libc::c_void,
) {
    let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut next: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut tmp: *mut cpConstraint = 0 as *mut cpConstraint;
    constraint = (*body).constraintList;
    while !constraint.is_null() {
        tmp = cpConstraintNext(constraint, body);
        next = tmp;
        (Some(func.expect("non-null function pointer")))
            .expect("non-null function pointer")(body, constraint, data);
        constraint = next;
    }
}
pub unsafe extern "C" fn cpBodyEachArbiter(
    mut body: *mut cpBody,
    mut func: Option::<
        unsafe extern "C" fn(*mut cpBody, *mut cpArbiter, *mut libc::c_void) -> (),
    >,
    mut data: *mut libc::c_void,
) {
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut next: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut tmp: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut swapped: cpBool = 0;
    arb = (*body).arbiterList;
    while !arb.is_null() {
        tmp = cpArbiterNext(arb, body);
        next = tmp;
        swapped = (*arb).swapped;
        (*arb)
            .swapped = (body as libc::c_ulong == (*arb).body_b as libc::c_ulong)
            as libc::c_int as cpBool;
        (Some(func.expect("non-null function pointer")))
            .expect("non-null function pointer")(body, arb, data);
        (*arb).swapped = swapped;
        arb = next;
    }
}
#[inline]
unsafe extern "C" fn cpfclamp01(mut f: cpFloat) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    tmp = cpfmin(f, 1.0f32 as cpFloat);
    tmp___0 = cpfmax(0.0f32 as cpFloat, tmp);
    return tmp___0;
}
static mut cpvzero___2: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpveql(v1: cpVect, v2: cpVect) -> cpBool {
    let mut tmp: libc::c_int = 0;
    if v1.x == v2.x {
        if v1.y == v2.y {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp as cpBool;
}
#[inline]
unsafe extern "C" fn cpvrperp(v: cpVect) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(v.y, -v.x);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpvnormalize(v: cpVect) -> cpVect {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvlength(v);
    tmp___0 = cpvmult(v, 1.0f32 as cpFloat / (tmp + 2.2250738585072014e-308f64));
    return tmp___0;
}
#[inline]
unsafe extern "C" fn cpBBCenter(mut bb: cpBB) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(bb.r, bb.t);
    tmp___0 = cpv(bb.l, bb.b);
    tmp___1 = cpvlerp(tmp___0, tmp, 0.5f32 as cpFloat);
    return tmp___1;
}
#[inline]
unsafe extern "C" fn cpCollisionInfoPushContact(
    mut info: *mut cpCollisionInfo,
    mut p1: cpVect,
    mut p2: cpVect,
    mut hash: cpHashValue,
) {
    let mut con: *mut cpContact = 0 as *mut cpContact;
    con = ((*info).arr).offset((*info).count as isize);
    (*con).r1 = p1;
    (*con).r2 = p2;
    (*con).hash = hash;
    (*info).count += 1;
}
#[inline]
unsafe extern "C" fn PolySupportPointIndex(
    count: libc::c_int,
    mut planes: *const cpSplittingPlane,
    n: cpVect,
) -> libc::c_int {
    let mut max: cpFloat = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut index___0: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut v: cpVect = cpVect { x: 0., y: 0. };
    let mut d: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    tmp = ::std::f32::INFINITY;
    max = -tmp as cpFloat;
    index___0 = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        v = (*planes.offset(i as isize)).v0;
        tmp___0 = cpvdot(v, n);
        d = tmp___0;
        if d > max {
            max = d;
            index___0 = i;
        }
        i += 1;
    }
    return index___0;
}
#[inline]
unsafe extern "C" fn SupportPointNew(
    mut p: cpVect,
    mut index___0: cpCollisionID,
) -> SupportPoint {
    let mut point: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    point.p = p;
    point.index = index___0;
    return point;
}
#[inline]
unsafe extern "C" fn CircleSupportPoint(
    mut circle: *const cpCircleShape,
    n: cpVect,
) -> SupportPoint {
    let mut tmp: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    tmp = SupportPointNew((*circle).tc, 0 as libc::c_int as cpCollisionID);
    return tmp;
}
#[inline]
unsafe extern "C" fn SegmentSupportPoint(
    mut seg___0: *const cpSegmentShape,
    n: cpVect,
) -> SupportPoint {
    let mut tmp: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut tmp___0: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    tmp___1 = cpvdot((*seg___0).ta, n);
    tmp___2 = cpvdot((*seg___0).tb, n);
    if tmp___1 > tmp___2 {
        tmp = SupportPointNew((*seg___0).ta, 0 as libc::c_int as cpCollisionID);
        return tmp;
    } else {
        tmp___0 = SupportPointNew((*seg___0).tb, 1 as libc::c_int as cpCollisionID);
        return tmp___0;
    };
}
#[inline]
unsafe extern "C" fn PolySupportPoint(
    mut poly: *const cpPolyShape,
    n: cpVect,
) -> SupportPoint {
    let mut planes: *const cpSplittingPlane = 0 as *const cpSplittingPlane;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    planes = (*poly).planes as *const cpSplittingPlane;
    tmp = PolySupportPointIndex((*poly).count, planes, n);
    i = tmp;
    tmp___0 = SupportPointNew((*planes.offset(i as isize)).v0, i as cpCollisionID);
    return tmp___0;
}
#[inline]
unsafe extern "C" fn MinkowskiPointNew(
    a: SupportPoint,
    b: SupportPoint,
) -> MinkowskiPoint {
    let mut point: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvsub(b.p, a.p);
    point.a = a.p;
    point.b = b.p;
    point.ab = tmp;
    point
        .id = (a.index & 255 as libc::c_uint) << 8 as libc::c_int
        | b.index & 255 as libc::c_uint;
    return point;
}
#[inline]
unsafe extern "C" fn Support(
    mut ctx: *const SupportContext,
    n: cpVect,
) -> MinkowskiPoint {
    let mut a: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut b: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut tmp___1: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut tmp___2: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    tmp = cpvneg(n);
    tmp___0 = (Some(((*ctx).func1).expect("non-null function pointer")))
        .expect("non-null function pointer")((*ctx).shape1, tmp);
    a = tmp___0;
    tmp___1 = (Some(((*ctx).func2).expect("non-null function pointer")))
        .expect("non-null function pointer")((*ctx).shape2, n);
    b = tmp___1;
    tmp___2 = MinkowskiPointNew(a, b);
    return tmp___2;
}
unsafe extern "C" fn SupportEdgeForPoly(
    mut poly: *const cpPolyShape,
    n: cpVect,
) -> Edge {
    let mut count: libc::c_int = 0;
    let mut i1: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i0: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut planes: *const cpSplittingPlane = 0 as *const cpSplittingPlane;
    let mut hashid: cpHashValue = 0;
    let mut edge: Edge = Edge {
        a: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        b: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        r: 0.,
        n: cpVect { x: 0., y: 0. },
    };
    let mut edge___0: Edge = Edge {
        a: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        b: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        r: 0.,
        n: cpVect { x: 0., y: 0. },
    };
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    count = (*poly).count;
    tmp = PolySupportPointIndex(
        (*poly).count,
        (*poly).planes as *const cpSplittingPlane,
        n,
    );
    i1 = tmp;
    i0 = (i1 - 1 as libc::c_int + count) % count;
    i2 = (i1 + 1 as libc::c_int) % count;
    planes = (*poly).planes as *const cpSplittingPlane;
    hashid = (*poly).shape.hashid;
    tmp___0 = cpvdot(n, (*planes.offset(i1 as isize)).n);
    tmp___1 = cpvdot(n, (*planes.offset(i2 as isize)).n);
    if tmp___0 > tmp___1 {
        edge.a.p = (*planes.offset(i0 as isize)).v0;
        edge
            .a
            .hash = hashid.wrapping_mul(3344921057 as libc::c_ulong)
            ^ (i0 as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong);
        edge.b.p = (*planes.offset(i1 as isize)).v0;
        edge
            .b
            .hash = hashid.wrapping_mul(3344921057 as libc::c_ulong)
            ^ (i1 as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong);
        edge.r = (*poly).r;
        edge.n = (*planes.offset(i1 as isize)).n;
        return edge;
    } else {
        edge___0.a.p = (*planes.offset(i1 as isize)).v0;
        edge___0
            .a
            .hash = hashid.wrapping_mul(3344921057 as libc::c_ulong)
            ^ (i1 as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong);
        edge___0.b.p = (*planes.offset(i2 as isize)).v0;
        edge___0
            .b
            .hash = hashid.wrapping_mul(3344921057 as libc::c_ulong)
            ^ (i2 as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong);
        edge___0.r = (*poly).r;
        edge___0.n = (*planes.offset(i2 as isize)).n;
        return edge___0;
    };
}
unsafe extern "C" fn SupportEdgeForSegment(
    mut seg___0: *const cpSegmentShape,
    n: cpVect,
) -> Edge {
    let mut hashid: cpHashValue = 0;
    let mut edge: Edge = Edge {
        a: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        b: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        r: 0.,
        n: cpVect { x: 0., y: 0. },
    };
    let mut edge___0: Edge = Edge {
        a: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        b: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        r: 0.,
        n: cpVect { x: 0., y: 0. },
    };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpFloat = 0.;
    hashid = (*seg___0).shape.hashid;
    tmp___0 = cpvdot((*seg___0).tn, n);
    if tmp___0 > 0.0f64 {
        edge.a.p = (*seg___0).ta;
        edge.a.hash = hashid.wrapping_mul(3344921057 as libc::c_ulong);
        edge.b.p = (*seg___0).tb;
        edge
            .b
            .hash = hashid.wrapping_mul(3344921057 as libc::c_ulong)
            ^ 3344921057 as libc::c_ulong;
        edge.r = (*seg___0).r;
        edge.n = (*seg___0).tn;
        return edge;
    } else {
        tmp = cpvneg((*seg___0).tn);
        edge___0.a.p = (*seg___0).tb;
        edge___0
            .a
            .hash = hashid.wrapping_mul(3344921057 as libc::c_ulong)
            ^ 3344921057 as libc::c_ulong;
        edge___0.b.p = (*seg___0).ta;
        edge___0.b.hash = hashid.wrapping_mul(3344921057 as libc::c_ulong);
        edge___0.r = (*seg___0).r;
        edge___0.n = tmp;
        return edge___0;
    };
}
#[inline]
unsafe extern "C" fn ClosestT(a: cpVect, b: cpVect) -> cpFloat {
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    tmp = cpvsub(b, a);
    delta = tmp;
    tmp___0 = cpvadd(a, b);
    tmp___1 = cpvdot(delta, tmp___0);
    tmp___2 = cpvlengthsq(delta);
    tmp___3 = cpfclamp(
        tmp___1 / (tmp___2 + 2.2250738585072014e-308f64),
        -1.0f32 as cpFloat,
        1.0f32 as cpFloat,
    );
    return -tmp___3;
}
#[inline]
unsafe extern "C" fn LerpT(a: cpVect, b: cpVect, t: cpFloat) -> cpVect {
    let mut ht: cpFloat = 0.;
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    ht = 0.5f32 as cpFloat * t;
    tmp = cpvmult(b, 0.5f32 as cpFloat + ht);
    tmp___0 = cpvmult(a, 0.5f32 as cpFloat - ht);
    tmp___1 = cpvadd(tmp___0, tmp);
    return tmp___1;
}
#[inline]
unsafe extern "C" fn ClosestPointsNew(
    v0: MinkowskiPoint,
    v1: MinkowskiPoint,
) -> ClosestPoints {
    let mut t: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    let mut p: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut pa: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut pb: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut id: cpCollisionID = 0;
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut d: cpFloat = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut points: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut d2: cpFloat = 0.;
    let mut tmp___7: cpFloat = 0.;
    let mut n2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    let mut points___0: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    tmp = ClosestT(v0.ab, v1.ab);
    t = tmp;
    tmp___0 = LerpT(v0.ab, v1.ab, t);
    p = tmp___0;
    tmp___1 = LerpT(v0.a, v1.a, t);
    pa = tmp___1;
    tmp___2 = LerpT(v0.b, v1.b, t);
    pb = tmp___2;
    id = (v0.id & 65535 as libc::c_uint) << 16 as libc::c_int
        | v1.id & 65535 as libc::c_uint;
    tmp___3 = cpvsub(v1.ab, v0.ab);
    delta = tmp___3;
    tmp___4 = cpvrperp(delta);
    tmp___5 = cpvnormalize(tmp___4);
    n = tmp___5;
    tmp___6 = cpvdot(n, p);
    d = tmp___6;
    let mut current_block_31: u64;
    if !(d <= 0.0f32 as cpFloat) {
        if (-1.0f32 as cpFloat) < t {
            if t < 1.0f32 as cpFloat {
                current_block_31 = 18177466129122142005;
            } else {
                current_block_31 = 7593991042637275866;
            }
        } else {
            current_block_31 = 7593991042637275866;
        }
        match current_block_31 {
            18177466129122142005 => {}
            _ => {
                tmp___7 = cpvlength(p);
                d2 = tmp___7;
                tmp___8 = cpvmult(
                    p,
                    1.0f32 as cpFloat / (d2 + 2.2250738585072014e-308f64),
                );
                n2 = tmp___8;
                points___0.a = pa;
                points___0.b = pb;
                points___0.n = n2;
                points___0.d = d2;
                points___0.id = id;
                return points___0;
            }
        }
    }
    points.a = pa;
    points.b = pb;
    points.n = n;
    points.d = d;
    points.id = id;
    return points;
}
#[inline]
unsafe extern "C" fn ClosestDist(v0: cpVect, v1: cpVect) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpFloat = 0.;
    tmp = ClosestT(v0, v1);
    tmp___0 = LerpT(v0, v1, tmp);
    tmp___1 = cpvlengthsq(tmp___0);
    return tmp___1;
}
unsafe extern "C" fn EPARecurse(
    mut ctx: *const SupportContext,
    count: libc::c_int,
    mut hull: *const MinkowskiPoint,
    iteration: libc::c_int,
) -> ClosestPoints {
    let mut mini: libc::c_int = 0;
    let mut minDist: cpFloat = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut d: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut v0: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut v1: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut p: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut duplicate: cpBool = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut hull2: *mut MinkowskiPoint = 0 as *mut MinkowskiPoint;
    let mut tmp___5: *mut _ = 0 as *mut _;
    let mut count2: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut index___0: libc::c_int = 0;
    let mut h0: cpVect = cpVect { x: 0., y: 0. };
    let mut h1: cpVect = cpVect { x: 0., y: 0. };
    let mut h2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut tmp___7: cpBool = 0;
    let mut tmp___8: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp___9: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp___10: cpBool = 0;
    mini = 0 as libc::c_int;
    tmp = ::std::f32::INFINITY;
    minDist = tmp as cpFloat;
    j = 0 as libc::c_int;
    i = count - 1 as libc::c_int;
    while j < count {
        tmp___0 = ClosestDist(
            (*hull.offset(i as isize)).ab,
            (*hull.offset(j as isize)).ab,
        );
        d = tmp___0;
        if d < minDist {
            minDist = d;
            mini = i;
        }
        i = j;
        j += 1;
    }
    v0 = *hull.offset(mini as isize);
    v1 = *hull.offset(((mini + 1 as libc::c_int) % count) as isize);
    tmp___1 = cpvsub(v1.ab, v0.ab);
    tmp___2 = cpvperp(tmp___1);
    tmp___3 = Support(ctx, tmp___2);
    p = tmp___3;
    if p.id == v0.id {
        tmp___4 = 1 as libc::c_int;
    } else if p.id == v1.id {
        tmp___4 = 1 as libc::c_int;
    } else {
        tmp___4 = 0 as libc::c_int;
    }
    duplicate = tmp___4 as cpBool;
    if duplicate == 0 {
        tmp___10 = cpCheckPointGreater(v0.ab, v1.ab, p.ab);
        if tmp___10 != 0 {
            if iteration < 30 as libc::c_int {
                let mut fresh7 = ::std::vec::from_elem(
                    0,
                    ((count + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<MinkowskiPoint>() as libc::c_ulong,
                        ) as usize,
                );
                tmp___5 = fresh7.as_mut_ptr();
                hull2 = tmp___5 as *mut MinkowskiPoint;
                count2 = 1 as libc::c_int;
                *hull2.offset(0 as libc::c_int as isize) = p;
                i___0 = 0 as libc::c_int;
                while i___0 < count {
                    index___0 = (mini + 1 as libc::c_int + i___0) % count;
                    h0 = (*hull2.offset((count2 - 1 as libc::c_int) as isize)).ab;
                    h1 = (*hull.offset(index___0 as isize)).ab;
                    if (i___0 + 1 as libc::c_int) < count {
                        tmp___6 = *hull
                            .offset(((index___0 + 1 as libc::c_int) % count) as isize);
                    } else {
                        tmp___6 = p;
                    }
                    h2 = tmp___6.ab;
                    tmp___7 = cpCheckPointGreater(h0, h2, h1);
                    if tmp___7 != 0 {
                        *hull2
                            .offset(count2 as isize) = *hull.offset(index___0 as isize);
                        count2 += 1;
                    }
                    i___0 += 1;
                }
                tmp___8 = EPARecurse(
                    ctx,
                    count2,
                    hull2 as *const MinkowskiPoint,
                    iteration + 1 as libc::c_int,
                );
                return tmp___8;
            } else {
                tmp___9 = ClosestPointsNew(v0, v1);
                return tmp___9;
            }
        } else {
            tmp___9 = ClosestPointsNew(v0, v1);
            return tmp___9;
        }
    } else {
        tmp___9 = ClosestPointsNew(v0, v1);
        return tmp___9;
    };
}
unsafe extern "C" fn EPA(
    mut ctx: *const SupportContext,
    v0: MinkowskiPoint,
    v1: MinkowskiPoint,
    v2: MinkowskiPoint,
) -> ClosestPoints {
    let mut hull: [MinkowskiPoint; 3] = [MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    }; 3];
    let mut tmp: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    hull[0 as libc::c_int as usize] = v0;
    hull[1 as libc::c_int as usize] = v1;
    hull[2 as libc::c_int as usize] = v2;
    tmp = EPARecurse(
        ctx,
        3 as libc::c_int,
        hull.as_mut_ptr() as *const MinkowskiPoint,
        1 as libc::c_int,
    );
    return tmp;
}
#[inline]
unsafe extern "C" fn GJKRecurse(
    mut ctx: *const SupportContext,
    v0: MinkowskiPoint,
    v1: MinkowskiPoint,
    iteration: libc::c_int,
) -> ClosestPoints {
    let mut tmp: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp___0: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut t: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut p: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut tmp___7: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut tmp___8: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp___9: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp___10: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp___11: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp___12: cpFloat = 0.;
    let mut tmp___13: cpFloat = 0.;
    let mut tmp___14: cpBool = 0;
    let mut tmp___15: cpBool = 0;
    let mut tmp___16: cpBool = 0;
    let mut tmp___17: cpBool = 0;
    if iteration > 30 as libc::c_int {
        tmp = ClosestPointsNew(v0, v1);
        return tmp;
    }
    tmp___17 = cpCheckPointGreater(v1.ab, v0.ab, cpvzero___2);
    if tmp___17 != 0 {
        tmp___0 = GJKRecurse(ctx, v1, v0, iteration);
        return tmp___0;
    } else {
        tmp___1 = ClosestT(v0.ab, v1.ab);
        t = tmp___1;
        if (-1.0f32 as cpFloat) < t {
            if t < 1.0f32 as cpFloat {
                tmp___2 = cpvsub(v1.ab, v0.ab);
                tmp___3 = cpvperp(tmp___2);
                tmp___6 = tmp___3;
            } else {
                tmp___4 = LerpT(v0.ab, v1.ab, t);
                tmp___5 = cpvneg(tmp___4);
                tmp___6 = tmp___5;
            }
        } else {
            tmp___4 = LerpT(v0.ab, v1.ab, t);
            tmp___5 = cpvneg(tmp___4);
            tmp___6 = tmp___5;
        }
        n = tmp___6;
        tmp___7 = Support(ctx, n);
        p = tmp___7;
        tmp___15 = cpCheckPointGreater(p.ab, v0.ab, cpvzero___2);
        if tmp___15 != 0 {
            tmp___16 = cpCheckPointGreater(v1.ab, p.ab, cpvzero___2);
            if tmp___16 != 0 {
                tmp___8 = EPA(ctx, v0, p, v1);
                return tmp___8;
            }
        }
        tmp___14 = cpCheckAxis(v0.ab, v1.ab, p.ab, n);
        if tmp___14 != 0 {
            tmp___9 = ClosestPointsNew(v0, v1);
            return tmp___9;
        } else {
            tmp___12 = ClosestDist(v0.ab, p.ab);
            tmp___13 = ClosestDist(p.ab, v1.ab);
            if tmp___12 < tmp___13 {
                tmp___10 = GJKRecurse(ctx, v0, p, iteration + 1 as libc::c_int);
                return tmp___10;
            } else {
                tmp___11 = GJKRecurse(ctx, p, v1, iteration + 1 as libc::c_int);
                return tmp___11;
            }
        }
    };
}
unsafe extern "C" fn ShapePoint(
    mut shape: *const cpShape,
    i: libc::c_int,
) -> SupportPoint {
    let mut tmp: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut seg___0: *mut cpSegmentShape = 0 as *mut cpSegmentShape;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut poly: *mut cpPolyShape = 0 as *mut cpPolyShape;
    let mut index___0: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut tmp___4: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    match (*(*shape).klass).type_0 as libc::c_uint {
        0 => {
            tmp = SupportPointNew(
                (*(shape as *mut cpCircleShape)).tc,
                0 as libc::c_int as cpCollisionID,
            );
            return tmp;
        }
        1 => {
            seg___0 = shape as *mut cpSegmentShape;
            if i == 0 as libc::c_int {
                tmp___0 = (*seg___0).ta;
            } else {
                tmp___0 = (*seg___0).tb;
            }
            tmp___1 = SupportPointNew(tmp___0, i as cpCollisionID);
            return tmp___1;
        }
        2 => {
            poly = shape as *mut cpPolyShape;
            if i < (*poly).count {
                tmp___2 = i;
            } else {
                tmp___2 = 0 as libc::c_int;
            }
            index___0 = tmp___2;
            tmp___3 = SupportPointNew(
                (*((*poly).planes).offset(index___0 as isize)).v0,
                index___0 as cpCollisionID,
            );
            return tmp___3;
        }
        _ => {
            tmp___4 = SupportPointNew(cpvzero___2, 0 as libc::c_int as cpCollisionID);
            return tmp___4;
        }
    };
}
unsafe extern "C" fn GJK(
    mut ctx: *const SupportContext,
    mut id: *mut cpCollisionID,
) -> ClosestPoints {
    let mut v0: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut v1: MinkowskiPoint = MinkowskiPoint {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        ab: cpVect { x: 0., y: 0. },
        id: 0,
    };
    let mut tmp: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut tmp___0: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut tmp___1: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut tmp___2: SupportPoint = SupportPoint {
        p: cpVect { x: 0., y: 0. },
        index: 0,
    };
    let mut axis: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpVect = cpVect { x: 0., y: 0. };
    let mut points: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp___8: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    if *id != 0 {
        tmp = ShapePoint(
            (*ctx).shape2,
            (*id >> 16 as libc::c_int & 255 as libc::c_uint) as libc::c_int,
        );
        tmp___0 = ShapePoint(
            (*ctx).shape1,
            (*id >> 24 as libc::c_int & 255 as libc::c_uint) as libc::c_int,
        );
        v0 = MinkowskiPointNew(tmp___0, tmp);
        tmp___1 = ShapePoint((*ctx).shape2, (*id & 255 as libc::c_uint) as libc::c_int);
        tmp___2 = ShapePoint(
            (*ctx).shape1,
            (*id >> 8 as libc::c_int & 255 as libc::c_uint) as libc::c_int,
        );
        v1 = MinkowskiPointNew(tmp___2, tmp___1);
    } else {
        tmp___3 = cpBBCenter((*(*ctx).shape2).bb);
        tmp___4 = cpBBCenter((*(*ctx).shape1).bb);
        tmp___5 = cpvsub(tmp___4, tmp___3);
        tmp___6 = cpvperp(tmp___5);
        axis = tmp___6;
        v0 = Support(ctx, axis);
        tmp___7 = cpvneg(axis);
        v1 = Support(ctx, tmp___7);
    }
    tmp___8 = GJKRecurse(ctx, v0, v1, 1 as libc::c_int);
    points = tmp___8;
    *id = points.id;
    return points;
}
#[inline]
unsafe extern "C" fn ContactPoints(
    e1: Edge,
    e2: Edge,
    points: ClosestPoints,
    mut info: *mut cpCollisionInfo,
) {
    let mut mindist: cpFloat = 0.;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut d_e1_a: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut d_e1_b: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut d_e2_a: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut d_e2_b: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut e1_denom: cpFloat = 0.;
    let mut e2_denom: cpFloat = 0.;
    let mut p1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpFloat = 0.;
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpVect = cpVect { x: 0., y: 0. };
    let mut p2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___8: cpFloat = 0.;
    let mut tmp___9: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___10: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___11: cpVect = cpVect { x: 0., y: 0. };
    let mut dist: cpFloat = 0.;
    let mut tmp___12: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___13: cpFloat = 0.;
    let mut hash_1a2b: cpHashValue = 0;
    let mut p1___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___14: cpFloat = 0.;
    let mut tmp___15: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___16: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___17: cpVect = cpVect { x: 0., y: 0. };
    let mut p2___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___18: cpFloat = 0.;
    let mut tmp___19: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___20: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___21: cpVect = cpVect { x: 0., y: 0. };
    let mut dist___0: cpFloat = 0.;
    let mut tmp___22: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___23: cpFloat = 0.;
    let mut hash_1b2a: cpHashValue = 0;
    mindist = e1.r + e2.r;
    if points.d <= mindist {
        tmp = points.n;
        (*info).n = tmp;
        n = tmp;
        tmp___0 = cpvcross(e1.a.p, n);
        d_e1_a = tmp___0;
        tmp___1 = cpvcross(e1.b.p, n);
        d_e1_b = tmp___1;
        tmp___2 = cpvcross(e2.a.p, n);
        d_e2_a = tmp___2;
        tmp___3 = cpvcross(e2.b.p, n);
        d_e2_b = tmp___3;
        e1_denom = 1.0f32 as cpFloat / (d_e1_b - d_e1_a + 2.2250738585072014e-308f64);
        e2_denom = 1.0f32 as cpFloat / (d_e2_b - d_e2_a + 2.2250738585072014e-308f64);
        tmp___4 = cpfclamp01((d_e2_b - d_e1_a) * e1_denom);
        tmp___5 = cpvlerp(e1.a.p, e1.b.p, tmp___4);
        tmp___6 = cpvmult(n, e1.r);
        tmp___7 = cpvadd(tmp___6, tmp___5);
        p1 = tmp___7;
        tmp___8 = cpfclamp01((d_e1_a - d_e2_a) * e2_denom);
        tmp___9 = cpvlerp(e2.a.p, e2.b.p, tmp___8);
        tmp___10 = cpvmult(n, -e2.r);
        tmp___11 = cpvadd(tmp___10, tmp___9);
        p2 = tmp___11;
        tmp___12 = cpvsub(p2, p1);
        tmp___13 = cpvdot(tmp___12, n);
        dist = tmp___13;
        if dist <= 0.0f32 as cpFloat {
            hash_1a2b = (e1.a.hash).wrapping_mul(3344921057 as libc::c_ulong)
                ^ (e2.b.hash).wrapping_mul(3344921057 as libc::c_ulong);
            cpCollisionInfoPushContact(info, p1, p2, hash_1a2b);
        }
        tmp___14 = cpfclamp01((d_e2_a - d_e1_a) * e1_denom);
        tmp___15 = cpvlerp(e1.a.p, e1.b.p, tmp___14);
        tmp___16 = cpvmult(n, e1.r);
        tmp___17 = cpvadd(tmp___16, tmp___15);
        p1___0 = tmp___17;
        tmp___18 = cpfclamp01((d_e1_b - d_e2_a) * e2_denom);
        tmp___19 = cpvlerp(e2.a.p, e2.b.p, tmp___18);
        tmp___20 = cpvmult(n, -e2.r);
        tmp___21 = cpvadd(tmp___20, tmp___19);
        p2___0 = tmp___21;
        tmp___22 = cpvsub(p2___0, p1___0);
        tmp___23 = cpvdot(tmp___22, n);
        dist___0 = tmp___23;
        if dist___0 <= 0.0f32 as cpFloat {
            hash_1b2a = (e1.b.hash).wrapping_mul(3344921057 as libc::c_ulong)
                ^ (e2.a.hash).wrapping_mul(3344921057 as libc::c_ulong);
            cpCollisionInfoPushContact(info, p1___0, p2___0, hash_1b2a);
        }
    }
}
unsafe extern "C" fn CircleToCircle(
    mut c1: *const cpCircleShape,
    mut c2: *const cpCircleShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut mindist: cpFloat = 0.;
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut distsq: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut dist: cpFloat = 0.;
    let mut tmp___1: libc::c_double = 0.;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    mindist = (*c1).r + (*c2).r;
    tmp = cpvsub((*c2).tc, (*c1).tc);
    delta = tmp;
    tmp___0 = cpvlengthsq(delta);
    distsq = tmp___0;
    if distsq < mindist * mindist {
        tmp___1 = sqrt(distsq);
        dist = tmp___1;
        if dist != 0. {
            tmp___3 = cpvmult(delta, 1.0f32 as cpFloat / dist);
            tmp___2 = tmp___3;
        } else {
            tmp___4 = cpv(1.0f32 as cpFloat, 0.0f32 as cpFloat);
            tmp___2 = tmp___4;
        }
        (*info).n = tmp___2;
        n = tmp___2;
        tmp___5 = cpvmult(n, -(*c2).r);
        tmp___6 = cpvadd((*c2).tc, tmp___5);
        tmp___7 = cpvmult(n, (*c1).r);
        tmp___8 = cpvadd((*c1).tc, tmp___7);
        cpCollisionInfoPushContact(
            info,
            tmp___8,
            tmp___6,
            0 as libc::c_int as cpHashValue,
        );
    }
}
unsafe extern "C" fn CircleToSegment(
    mut circle: *const cpCircleShape,
    mut segment: *const cpSegmentShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut seg_a: cpVect = cpVect { x: 0., y: 0. };
    let mut seg_b: cpVect = cpVect { x: 0., y: 0. };
    let mut center: cpVect = cpVect { x: 0., y: 0. };
    let mut seg_delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut closest_t: cpFloat = 0.;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut closest: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut mindist: cpFloat = 0.;
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut distsq: cpFloat = 0.;
    let mut tmp___7: cpFloat = 0.;
    let mut dist: cpFloat = 0.;
    let mut tmp___8: libc::c_double = 0.;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___9: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___10: cpVect = cpVect { x: 0., y: 0. };
    let mut rot: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___11: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___12: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___13: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___14: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___15: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___16: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___17: cpFloat = 0.;
    let mut tmp___18: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___19: cpFloat = 0.;
    seg_a = (*segment).ta;
    seg_b = (*segment).tb;
    center = (*circle).tc;
    tmp = cpvsub(seg_b, seg_a);
    seg_delta = tmp;
    tmp___0 = cpvsub(center, seg_a);
    tmp___1 = cpvdot(seg_delta, tmp___0);
    tmp___2 = cpvlengthsq(seg_delta);
    tmp___3 = cpfclamp01(tmp___1 / tmp___2);
    closest_t = tmp___3;
    tmp___4 = cpvmult(seg_delta, closest_t);
    tmp___5 = cpvadd(seg_a, tmp___4);
    closest = tmp___5;
    mindist = (*circle).r + (*segment).r;
    tmp___6 = cpvsub(closest, center);
    delta = tmp___6;
    tmp___7 = cpvlengthsq(delta);
    distsq = tmp___7;
    if distsq < mindist * mindist {
        tmp___8 = sqrt(distsq);
        dist = tmp___8;
        if dist != 0. {
            tmp___10 = cpvmult(delta, 1.0f32 as cpFloat / dist);
            tmp___9 = tmp___10;
        } else {
            tmp___9 = (*segment).tn;
        }
        (*info).n = tmp___9;
        n = tmp___9;
        tmp___11 = cpBodyGetRotation((*segment).shape.body as *const cpBody);
        rot = tmp___11;
        let mut current_block_48: u64;
        if closest_t != 0.0f32 as cpFloat {
            current_block_48 = 16398409263262297310;
        } else {
            tmp___16 = cpvrotate((*segment).a_tangent, rot);
            tmp___17 = cpvdot(n, tmp___16);
            if tmp___17 >= 0.0f64 {
                current_block_48 = 16398409263262297310;
            } else {
                current_block_48 = 13619784596304402172;
            }
        }
        match current_block_48 {
            16398409263262297310 => {
                if closest_t != 1.0f32 as cpFloat {
                    tmp___12 = cpvmult(n, -(*segment).r);
                    tmp___13 = cpvadd(closest, tmp___12);
                    tmp___14 = cpvmult(n, (*circle).r);
                    tmp___15 = cpvadd(center, tmp___14);
                    cpCollisionInfoPushContact(
                        info,
                        tmp___15,
                        tmp___13,
                        0 as libc::c_int as cpHashValue,
                    );
                } else {
                    tmp___18 = cpvrotate((*segment).b_tangent, rot);
                    tmp___19 = cpvdot(n, tmp___18);
                    if tmp___19 >= 0.0f64 {
                        tmp___12 = cpvmult(n, -(*segment).r);
                        tmp___13 = cpvadd(closest, tmp___12);
                        tmp___14 = cpvmult(n, (*circle).r);
                        tmp___15 = cpvadd(center, tmp___14);
                        cpCollisionInfoPushContact(
                            info,
                            tmp___15,
                            tmp___13,
                            0 as libc::c_int as cpHashValue,
                        );
                    }
                }
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn SegmentToSegment(
    mut seg1: *const cpSegmentShape,
    mut seg2: *const cpSegmentShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut context: SupportContext = SupportContext {
        shape1: 0 as *const cpShape,
        shape2: 0 as *const cpShape,
        func1: None,
        func2: None,
    };
    let mut points: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut rot1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut rot2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: Edge = Edge {
        a: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        b: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        r: 0.,
        n: cpVect { x: 0., y: 0. },
    };
    let mut tmp___4: Edge = Edge {
        a: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        b: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        r: 0.,
        n: cpVect { x: 0., y: 0. },
    };
    let mut tmp___5: cpBool = 0;
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpFloat = 0.;
    let mut tmp___8: cpBool = 0;
    let mut tmp___9: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___10: cpFloat = 0.;
    let mut tmp___11: cpBool = 0;
    let mut tmp___12: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___13: cpFloat = 0.;
    let mut tmp___14: cpBool = 0;
    let mut tmp___15: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___16: cpFloat = 0.;
    context.shape1 = seg1 as *mut cpShape as *const cpShape;
    context.shape2 = seg2 as *mut cpShape as *const cpShape;
    context
        .func1 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*const cpSegmentShape, cpVect) -> SupportPoint>,
        Option::<unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint>,
    >(
        Some(
            SegmentSupportPoint
                as unsafe extern "C" fn(*const cpSegmentShape, cpVect) -> SupportPoint,
        ),
    );
    context
        .func2 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*const cpSegmentShape, cpVect) -> SupportPoint>,
        Option::<unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint>,
    >(
        Some(
            SegmentSupportPoint
                as unsafe extern "C" fn(*const cpSegmentShape, cpVect) -> SupportPoint,
        ),
    );
    tmp = GJK(
        &mut context as *mut SupportContext as *const SupportContext,
        &mut (*info).id,
    );
    points = tmp;
    n = points.n;
    tmp___0 = cpBodyGetRotation((*seg1).shape.body as *const cpBody);
    rot1 = tmp___0;
    tmp___1 = cpBodyGetRotation((*seg2).shape.body as *const cpBody);
    rot2 = tmp___1;
    if points.d <= (*seg1).r + (*seg2).r {
        tmp___5 = cpveql(points.a, (*seg1).ta);
        let mut current_block_37: u64;
        if tmp___5 != 0 {
            tmp___6 = cpvrotate((*seg1).a_tangent, rot1);
            tmp___7 = cpvdot(n, tmp___6);
            if tmp___7 <= 0.0f64 {
                current_block_37 = 13318077168829773021;
            } else {
                current_block_37 = 14447253356787937536;
            }
        } else {
            current_block_37 = 13318077168829773021;
        }
        match current_block_37 {
            13318077168829773021 => {
                tmp___8 = cpveql(points.a, (*seg1).tb);
                let mut current_block_36: u64;
                if tmp___8 != 0 {
                    tmp___9 = cpvrotate((*seg1).b_tangent, rot1);
                    tmp___10 = cpvdot(n, tmp___9);
                    if tmp___10 <= 0.0f64 {
                        current_block_36 = 14706570030715500319;
                    } else {
                        current_block_36 = 10758786907990354186;
                    }
                } else {
                    current_block_36 = 14706570030715500319;
                }
                match current_block_36 {
                    14706570030715500319 => {
                        tmp___11 = cpveql(points.b, (*seg2).ta);
                        let mut current_block_35: u64;
                        if tmp___11 != 0 {
                            tmp___12 = cpvrotate((*seg2).a_tangent, rot2);
                            tmp___13 = cpvdot(n, tmp___12);
                            if tmp___13 >= 0.0f64 {
                                current_block_35 = 9447123683320331402;
                            } else {
                                current_block_35 = 7226443171521532240;
                            }
                        } else {
                            current_block_35 = 9447123683320331402;
                        }
                        match current_block_35 {
                            9447123683320331402 => {
                                tmp___14 = cpveql(points.b, (*seg2).tb);
                                if tmp___14 != 0 {
                                    tmp___15 = cpvrotate((*seg2).b_tangent, rot2);
                                    tmp___16 = cpvdot(n, tmp___15);
                                    if tmp___16 >= 0.0f64 {
                                        tmp___2 = cpvneg(n);
                                        tmp___3 = SupportEdgeForSegment(seg2, tmp___2);
                                        tmp___4 = SupportEdgeForSegment(seg1, n);
                                        ContactPoints(tmp___4, tmp___3, points, info);
                                    }
                                } else {
                                    tmp___2 = cpvneg(n);
                                    tmp___3 = SupportEdgeForSegment(seg2, tmp___2);
                                    tmp___4 = SupportEdgeForSegment(seg1, n);
                                    ContactPoints(tmp___4, tmp___3, points, info);
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
    }
}
unsafe extern "C" fn PolyToPoly(
    mut poly1: *const cpPolyShape,
    mut poly2: *const cpPolyShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut context: SupportContext = SupportContext {
        shape1: 0 as *const cpShape,
        shape2: 0 as *const cpShape,
        func1: None,
        func2: None,
    };
    let mut points: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: Edge = Edge {
        a: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        b: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        r: 0.,
        n: cpVect { x: 0., y: 0. },
    };
    let mut tmp___2: Edge = Edge {
        a: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        b: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        r: 0.,
        n: cpVect { x: 0., y: 0. },
    };
    context.shape1 = poly1 as *mut cpShape as *const cpShape;
    context.shape2 = poly2 as *mut cpShape as *const cpShape;
    context
        .func1 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint>,
        Option::<unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint>,
    >(
        Some(
            PolySupportPoint
                as unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint,
        ),
    );
    context
        .func2 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint>,
        Option::<unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint>,
    >(
        Some(
            PolySupportPoint
                as unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint,
        ),
    );
    tmp = GJK(
        &mut context as *mut SupportContext as *const SupportContext,
        &mut (*info).id,
    );
    points = tmp;
    if points.d - (*poly1).r - (*poly2).r <= 0.0f64 {
        tmp___0 = cpvneg(points.n);
        tmp___1 = SupportEdgeForPoly(poly2, tmp___0);
        tmp___2 = SupportEdgeForPoly(poly1, points.n);
        ContactPoints(tmp___2, tmp___1, points, info);
    }
}
unsafe extern "C" fn SegmentToPoly(
    mut seg___0: *const cpSegmentShape,
    mut poly: *const cpPolyShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut context: SupportContext = SupportContext {
        shape1: 0 as *const cpShape,
        shape2: 0 as *const cpShape,
        func1: None,
        func2: None,
    };
    let mut points: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut rot: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: Edge = Edge {
        a: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        b: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        r: 0.,
        n: cpVect { x: 0., y: 0. },
    };
    let mut tmp___3: Edge = Edge {
        a: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        b: EdgePoint {
            p: cpVect { x: 0., y: 0. },
            hash: 0,
        },
        r: 0.,
        n: cpVect { x: 0., y: 0. },
    };
    let mut tmp___4: cpBool = 0;
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpFloat = 0.;
    let mut tmp___7: cpBool = 0;
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___9: cpFloat = 0.;
    context.shape1 = seg___0 as *mut cpShape as *const cpShape;
    context.shape2 = poly as *mut cpShape as *const cpShape;
    context
        .func1 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*const cpSegmentShape, cpVect) -> SupportPoint>,
        Option::<unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint>,
    >(
        Some(
            SegmentSupportPoint
                as unsafe extern "C" fn(*const cpSegmentShape, cpVect) -> SupportPoint,
        ),
    );
    context
        .func2 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint>,
        Option::<unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint>,
    >(
        Some(
            PolySupportPoint
                as unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint,
        ),
    );
    tmp = GJK(
        &mut context as *mut SupportContext as *const SupportContext,
        &mut (*info).id,
    );
    points = tmp;
    n = points.n;
    tmp___0 = cpBodyGetRotation((*seg___0).shape.body as *const cpBody);
    rot = tmp___0;
    if points.d - (*seg___0).r - (*poly).r <= 0.0f64 {
        tmp___4 = cpveql(points.a, (*seg___0).ta);
        let mut current_block_27: u64;
        if tmp___4 != 0 {
            tmp___5 = cpvrotate((*seg___0).a_tangent, rot);
            tmp___6 = cpvdot(n, tmp___5);
            if tmp___6 <= 0.0f64 {
                current_block_27 = 2816505574879542467;
            } else {
                current_block_27 = 14359455889292382949;
            }
        } else {
            current_block_27 = 2816505574879542467;
        }
        match current_block_27 {
            2816505574879542467 => {
                tmp___7 = cpveql(points.a, (*seg___0).tb);
                if tmp___7 != 0 {
                    tmp___8 = cpvrotate((*seg___0).b_tangent, rot);
                    tmp___9 = cpvdot(n, tmp___8);
                    if tmp___9 <= 0.0f64 {
                        tmp___1 = cpvneg(n);
                        tmp___2 = SupportEdgeForPoly(poly, tmp___1);
                        tmp___3 = SupportEdgeForSegment(seg___0, n);
                        ContactPoints(tmp___3, tmp___2, points, info);
                    }
                } else {
                    tmp___1 = cpvneg(n);
                    tmp___2 = SupportEdgeForPoly(poly, tmp___1);
                    tmp___3 = SupportEdgeForSegment(seg___0, n);
                    ContactPoints(tmp___3, tmp___2, points, info);
                }
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn CircleToPoly(
    mut circle: *const cpCircleShape,
    mut poly: *const cpPolyShape,
    mut info: *mut cpCollisionInfo,
) {
    let mut context: SupportContext = SupportContext {
        shape1: 0 as *const cpShape,
        shape2: 0 as *const cpShape,
        func1: None,
        func2: None,
    };
    let mut points: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut tmp: ClosestPoints = ClosestPoints {
        a: cpVect { x: 0., y: 0. },
        b: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
        d: 0.,
        id: 0,
    };
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    context.shape1 = circle as *mut cpShape as *const cpShape;
    context.shape2 = poly as *mut cpShape as *const cpShape;
    context
        .func1 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*const cpCircleShape, cpVect) -> SupportPoint>,
        Option::<unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint>,
    >(
        Some(
            CircleSupportPoint
                as unsafe extern "C" fn(*const cpCircleShape, cpVect) -> SupportPoint,
        ),
    );
    context
        .func2 = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint>,
        Option::<unsafe extern "C" fn(*const cpShape, cpVect) -> SupportPoint>,
    >(
        Some(
            PolySupportPoint
                as unsafe extern "C" fn(*const cpPolyShape, cpVect) -> SupportPoint,
        ),
    );
    tmp = GJK(
        &mut context as *mut SupportContext as *const SupportContext,
        &mut (*info).id,
    );
    points = tmp;
    if points.d <= (*circle).r + (*poly).r {
        tmp___0 = points.n;
        (*info).n = tmp___0;
        n = tmp___0;
        tmp___1 = cpvmult(n, -(*poly).r);
        tmp___2 = cpvadd(points.b, tmp___1);
        tmp___3 = cpvmult(n, (*circle).r);
        tmp___4 = cpvadd(points.a, tmp___3);
        cpCollisionInfoPushContact(
            info,
            tmp___4,
            tmp___2,
            0 as libc::c_int as cpHashValue,
        );
    }
}
unsafe extern "C" fn CollisionError(
    mut circle: *const cpShape,
    mut poly: *const cpShape,
    mut info: *mut cpCollisionInfo,
) {
    cpMessage(
        b"cpFalse\0" as *const u8 as *const libc::c_char,
        b"../src/cpCollision.c\0" as *const u8 as *const libc::c_char,
        684 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        b"Internal Error: Shape types are not sorted.\0" as *const u8
            as *const libc::c_char,
    );
    abort();
}
static mut BuiltinCollisionFuncs: [CollisionFunc; 9] = unsafe {
    [
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpCircleShape,
                    *const cpCircleShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            Option::<
                unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
        >(
            Some(
                CircleToCircle
                    as unsafe extern "C" fn(
                        *const cpCircleShape,
                        *const cpCircleShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            CollisionFunc,
        >(
            Some(
                CollisionError
                    as unsafe extern "C" fn(
                        *const cpShape,
                        *const cpShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            CollisionFunc,
        >(
            Some(
                CollisionError
                    as unsafe extern "C" fn(
                        *const cpShape,
                        *const cpShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpCircleShape,
                    *const cpSegmentShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            Option::<
                unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
        >(
            Some(
                CircleToSegment
                    as unsafe extern "C" fn(
                        *const cpCircleShape,
                        *const cpSegmentShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpSegmentShape,
                    *const cpSegmentShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            Option::<
                unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
        >(
            Some(
                SegmentToSegment
                    as unsafe extern "C" fn(
                        *const cpSegmentShape,
                        *const cpSegmentShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            CollisionFunc,
        >(
            Some(
                CollisionError
                    as unsafe extern "C" fn(
                        *const cpShape,
                        *const cpShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpCircleShape,
                    *const cpPolyShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            Option::<
                unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
        >(
            Some(
                CircleToPoly
                    as unsafe extern "C" fn(
                        *const cpCircleShape,
                        *const cpPolyShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpSegmentShape,
                    *const cpPolyShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            Option::<
                unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
        >(
            Some(
                SegmentToPoly
                    as unsafe extern "C" fn(
                        *const cpSegmentShape,
                        *const cpPolyShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const cpPolyShape,
                    *const cpPolyShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
            Option::<
                unsafe extern "C" fn(
                    *const cpShape,
                    *const cpShape,
                    *mut cpCollisionInfo,
                ) -> (),
            >,
        >(
            Some(
                PolyToPoly
                    as unsafe extern "C" fn(
                        *const cpPolyShape,
                        *const cpPolyShape,
                        *mut cpCollisionInfo,
                    ) -> (),
            ),
        ),
    ]
};
static mut CollisionFuncs: *const CollisionFunc = unsafe {
    BuiltinCollisionFuncs.as_ptr()
};
pub unsafe extern "C" fn cpCollide(
    mut a: *const cpShape,
    mut b: *const cpShape,
    mut id: cpCollisionID,
    mut contacts: *mut cpContact,
) -> cpCollisionInfo {
    let mut info: cpCollisionInfo = cpCollisionInfo {
        a: 0 as *const cpShape,
        b: 0 as *const cpShape,
        id: 0,
        n: cpVect { x: 0., y: 0. },
        count: 0,
        arr: 0 as *mut cpContact,
    };
    info.a = a;
    info.b = b;
    info.id = id;
    info.n = cpvzero___2;
    info.count = 0 as libc::c_int;
    info.arr = contacts;
    if (*(*a).klass).type_0 as libc::c_uint > (*(*b).klass).type_0 as libc::c_uint {
        info.a = b;
        info.b = a;
    }
    (Some(
        (*CollisionFuncs
            .offset(
                ((*(*info.a).klass).type_0 as libc::c_uint)
                    .wrapping_add(
                        ((*(*info.b).klass).type_0 as libc::c_uint)
                            .wrapping_mul(3 as libc::c_uint),
                    ) as isize,
            ))
            .expect("non-null function pointer"),
    ))
        .expect("non-null function pointer")(info.a, info.b, &mut info);
    return info;
}
#[inline]
unsafe extern "C" fn cpConstraintActivateBodies(mut constraint: *mut cpConstraint) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    a = (*constraint).a;
    cpBodyActivate(a);
    b = (*constraint).b;
    cpBodyActivate(b);
}
pub unsafe extern "C" fn cpConstraintDestroy(mut constraint: *mut cpConstraint) {}
pub unsafe extern "C" fn cpConstraintFree(mut constraint: *mut cpConstraint) {
    if !constraint.is_null() {
        cpConstraintDestroy(constraint);
        free(constraint as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpConstraintInit(
    mut constraint: *mut cpConstraint,
    mut klass___12: *const cpConstraintClass,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
) {
    let mut tmp: libc::c_float = 0.;
    let mut tmp___0: libc::c_float = 0.;
    (*constraint).klass = klass___12;
    (*constraint).a = a;
    (*constraint).b = b;
    (*constraint).space = 0 as *mut libc::c_void as *mut cpSpace;
    (*constraint).next_a = 0 as *mut libc::c_void as *mut cpConstraint;
    (*constraint).next_b = 0 as *mut libc::c_void as *mut cpConstraint;
    tmp = ::std::f32::INFINITY;
    (*constraint).maxForce = tmp as cpFloat;
    (*constraint)
        .errorBias = pow((1.0f32 - 0.1f32) as libc::c_double, 60.0f32 as libc::c_double);
    tmp___0 = ::std::f32::INFINITY;
    (*constraint).maxBias = tmp___0 as cpFloat;
    (*constraint).collideBodies = 1 as libc::c_int as cpBool;
    (*constraint)
        .preSolve = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> ()>,
    >(0 as *mut libc::c_void);
    (*constraint)
        .postSolve = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> ()>,
    >(0 as *mut libc::c_void);
}
pub unsafe extern "C" fn cpConstraintGetSpace(
    mut constraint: *const cpConstraint,
) -> *mut cpSpace {
    return (*constraint).space;
}
pub unsafe extern "C" fn cpConstraintGetBodyA(
    mut constraint: *const cpConstraint,
) -> *mut cpBody {
    return (*constraint).a;
}
pub unsafe extern "C" fn cpConstraintGetBodyB(
    mut constraint: *const cpConstraint,
) -> *mut cpBody {
    return (*constraint).b;
}
pub unsafe extern "C" fn cpConstraintGetMaxForce(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    return (*constraint).maxForce;
}
pub unsafe extern "C" fn cpConstraintSetMaxForce(
    mut constraint: *mut cpConstraint,
    mut maxForce: cpFloat,
) {
    if !(maxForce >= 0.0f32 as cpFloat) {
        cpMessage(
            b"maxForce >= 0.0f\0" as *const u8 as *const libc::c_char,
            b"../src/cpConstraint.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"maxForce must be positive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*constraint).maxForce = maxForce;
}
pub unsafe extern "C" fn cpConstraintGetErrorBias(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    return (*constraint).errorBias;
}
pub unsafe extern "C" fn cpConstraintSetErrorBias(
    mut constraint: *mut cpConstraint,
    mut errorBias: cpFloat,
) {
    if !(errorBias >= 0.0f32 as cpFloat) {
        cpMessage(
            b"errorBias >= 0.0f\0" as *const u8 as *const libc::c_char,
            b"../src/cpConstraint.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"errorBias must be positive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*constraint).errorBias = errorBias;
}
pub unsafe extern "C" fn cpConstraintGetMaxBias(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    return (*constraint).maxBias;
}
pub unsafe extern "C" fn cpConstraintSetMaxBias(
    mut constraint: *mut cpConstraint,
    mut maxBias: cpFloat,
) {
    if !(maxBias >= 0.0f32 as cpFloat) {
        cpMessage(
            b"maxBias >= 0.0f\0" as *const u8 as *const libc::c_char,
            b"../src/cpConstraint.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"maxBias must be positive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*constraint).maxBias = maxBias;
}
pub unsafe extern "C" fn cpConstraintGetCollideBodies(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return (*constraint).collideBodies;
}
pub unsafe extern "C" fn cpConstraintSetCollideBodies(
    mut constraint: *mut cpConstraint,
    mut collideBodies: cpBool,
) {
    cpConstraintActivateBodies(constraint);
    (*constraint).collideBodies = collideBodies;
}
pub unsafe extern "C" fn cpConstraintGetPreSolveFunc(
    mut constraint: *const cpConstraint,
) -> cpConstraintPreSolveFunc {
    return (*constraint).preSolve;
}
pub unsafe extern "C" fn cpConstraintSetPreSolveFunc(
    mut constraint: *mut cpConstraint,
    mut preSolveFunc: Option::<
        unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
    >,
) {
    (*constraint).preSolve = preSolveFunc;
}
pub unsafe extern "C" fn cpConstraintGetPostSolveFunc(
    mut constraint: *const cpConstraint,
) -> cpConstraintPostSolveFunc {
    return (*constraint).postSolve;
}
pub unsafe extern "C" fn cpConstraintSetPostSolveFunc(
    mut constraint: *mut cpConstraint,
    mut postSolveFunc: Option::<
        unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
    >,
) {
    (*constraint).postSolve = postSolveFunc;
}
pub unsafe extern "C" fn cpConstraintGetUserData(
    mut constraint: *const cpConstraint,
) -> cpDataPointer {
    return (*constraint).userData;
}
pub unsafe extern "C" fn cpConstraintSetUserData(
    mut constraint: *mut cpConstraint,
    mut userData: cpDataPointer,
) {
    (*constraint).userData = userData;
}
pub unsafe extern "C" fn cpConstraintGetImpulse(
    mut constraint: *mut cpConstraint,
) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = (Some(((*(*constraint).klass).getImpulse).expect("non-null function pointer")))
        .expect("non-null function pointer")(constraint);
    return tmp;
}
unsafe extern "C" fn defaultSpringTorque(
    mut spring: *mut cpDampedRotarySpring,
    mut relativeAngle: cpFloat,
) -> cpFloat {
    return (relativeAngle - (*spring).restAngle) * (*spring).stiffness;
}
unsafe extern "C" fn preStep(mut spring: *mut cpDampedRotarySpring, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut moment: cpFloat = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut j_spring: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    a = (*spring).constraint.a;
    b = (*spring).constraint.b;
    moment = (*a).i_inv + (*b).i_inv;
    (*spring).iSum = 1.0f32 as cpFloat / moment;
    tmp = exp(-(*spring).damping * dt * moment);
    (*spring).w_coef = 1.0f32 as libc::c_double - tmp;
    (*spring).target_wrn = 0.0f32 as cpFloat;
    tmp___0 = (Some(((*spring).springTorqueFunc).expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(spring as *mut cpConstraint, (*a).a - (*b).a);
    j_spring = tmp___0 * dt;
    (*spring).jAcc = j_spring;
    (*a).w -= j_spring * (*a).i_inv;
    (*b).w += j_spring * (*b).i_inv;
}
unsafe extern "C" fn applyCachedImpulse(
    mut spring: *mut cpDampedRotarySpring,
    mut dt_coef: cpFloat,
) {}
unsafe extern "C" fn applyImpulse(
    mut spring: *mut cpDampedRotarySpring,
    mut dt: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut wrn: cpFloat = 0.;
    let mut w_damp: cpFloat = 0.;
    let mut j_damp: cpFloat = 0.;
    a = (*spring).constraint.a;
    b = (*spring).constraint.b;
    wrn = (*a).w - (*b).w;
    w_damp = ((*spring).target_wrn - wrn) * (*spring).w_coef;
    (*spring).target_wrn = wrn + w_damp;
    j_damp = w_damp * (*spring).iSum;
    (*spring).jAcc += j_damp;
    (*a).w += j_damp * (*a).i_inv;
    (*b).w -= j_damp * (*b).i_inv;
}
unsafe extern "C" fn getImpulse(mut spring: *mut cpDampedRotarySpring) -> cpFloat {
    return (*spring).jAcc;
}
static mut klass___0: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedRotarySpring, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    preStep
                        as unsafe extern "C" fn(*mut cpDampedRotarySpring, cpFloat) -> (),
                ),
            ),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedRotarySpring, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyCachedImpulse
                        as unsafe extern "C" fn(*mut cpDampedRotarySpring, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedRotarySpring, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyImpulse
                        as unsafe extern "C" fn(*mut cpDampedRotarySpring, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedRotarySpring) -> cpFloat>,
                Option::<unsafe extern "C" fn(*mut cpConstraint) -> cpFloat>,
            >(
                Some(
                    getImpulse
                        as unsafe extern "C" fn(*mut cpDampedRotarySpring) -> cpFloat,
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpDampedRotarySpringAlloc() -> *mut cpDampedRotarySpring {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpDampedRotarySpring>() as libc::c_ulong,
    );
    return tmp as *mut cpDampedRotarySpring;
}
pub unsafe extern "C" fn cpDampedRotarySpringInit(
    mut spring: *mut cpDampedRotarySpring,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut restAngle: cpFloat,
    mut stiffness: cpFloat,
    mut damping: cpFloat,
) -> *mut cpDampedRotarySpring {
    cpConstraintInit(spring as *mut cpConstraint, &klass___0, a, b);
    (*spring).restAngle = restAngle;
    (*spring).stiffness = stiffness;
    (*spring).damping = damping;
    (*spring)
        .springTorqueFunc = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut cpDampedRotarySpring, cpFloat) -> cpFloat>,
        Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat>,
    >(
        Some(
            defaultSpringTorque
                as unsafe extern "C" fn(*mut cpDampedRotarySpring, cpFloat) -> cpFloat,
        ),
    );
    (*spring).jAcc = 0.0f32 as cpFloat;
    return spring;
}
pub unsafe extern "C" fn cpDampedRotarySpringNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut restAngle: cpFloat,
    mut stiffness: cpFloat,
    mut damping: cpFloat,
) -> *mut cpConstraint {
    let mut tmp: *mut cpDampedRotarySpring = 0 as *mut cpDampedRotarySpring;
    let mut tmp___0: *mut cpDampedRotarySpring = 0 as *mut cpDampedRotarySpring;
    tmp = cpDampedRotarySpringAlloc();
    tmp___0 = cpDampedRotarySpringInit(tmp, a, b, restAngle, stiffness, damping);
    return tmp___0 as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsDampedRotarySpring(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass as libc::c_ulong
        == &klass___0 as *const cpConstraintClass as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpDampedRotarySpringGetRestAngle(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedRotarySpring(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedRotarySpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedRotarySpring.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped rotary spring.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedRotarySpring)).restAngle;
}
pub unsafe extern "C" fn cpDampedRotarySpringSetRestAngle(
    mut constraint: *mut cpConstraint,
    mut restAngle: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedRotarySpring(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedRotarySpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedRotarySpring.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped rotary spring.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedRotarySpring)).restAngle = restAngle;
}
pub unsafe extern "C" fn cpDampedRotarySpringGetStiffness(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedRotarySpring(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedRotarySpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedRotarySpring.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped rotary spring.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedRotarySpring)).stiffness;
}
pub unsafe extern "C" fn cpDampedRotarySpringSetStiffness(
    mut constraint: *mut cpConstraint,
    mut stiffness: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedRotarySpring(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedRotarySpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedRotarySpring.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped rotary spring.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedRotarySpring)).stiffness = stiffness;
}
pub unsafe extern "C" fn cpDampedRotarySpringGetDamping(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedRotarySpring(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedRotarySpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedRotarySpring.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped rotary spring.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedRotarySpring)).damping;
}
pub unsafe extern "C" fn cpDampedRotarySpringSetDamping(
    mut constraint: *mut cpConstraint,
    mut damping: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedRotarySpring(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedRotarySpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedRotarySpring.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped rotary spring.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedRotarySpring)).damping = damping;
}
pub unsafe extern "C" fn cpDampedRotarySpringGetSpringTorqueFunc(
    mut constraint: *const cpConstraint,
) -> cpDampedRotarySpringTorqueFunc {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedRotarySpring(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedRotarySpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedRotarySpring.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped rotary spring.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedRotarySpring)).springTorqueFunc;
}
pub unsafe extern "C" fn cpDampedRotarySpringSetSpringTorqueFunc(
    mut constraint: *mut cpConstraint,
    mut springTorqueFunc: Option::<
        unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat,
    >,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedRotarySpring(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedRotarySpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedRotarySpring.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped rotary spring.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    let ref mut fresh8 = (*(constraint as *mut cpDampedRotarySpring)).springTorqueFunc;
    *fresh8 = springTorqueFunc;
}
unsafe extern "C" fn defaultSpringForce(
    mut spring: *mut cpDampedSpring,
    mut dist: cpFloat,
) -> cpFloat {
    return ((*spring).restLength - dist) * (*spring).stiffness;
}
unsafe extern "C" fn preStep___0(mut spring: *mut cpDampedSpring, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut dist: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut tmp___5: libc::c_float = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut k: cpFloat = 0.;
    let mut tmp___7: cpFloat = 0.;
    let mut tmp___8: libc::c_double = 0.;
    let mut f_spring: cpFloat = 0.;
    let mut tmp___9: cpFloat = 0.;
    let mut j_spring: cpFloat = 0.;
    let mut tmp___10: cpFloat = 0.;
    let mut tmp___11: cpVect = cpVect { x: 0., y: 0. };
    a = (*spring).constraint.a;
    b = (*spring).constraint.b;
    tmp = cpvsub((*spring).anchorA, (*a).cog);
    (*spring).r1 = cpTransformVect((*a).transform, tmp);
    tmp___0 = cpvsub((*spring).anchorB, (*b).cog);
    (*spring).r2 = cpTransformVect((*b).transform, tmp___0);
    tmp___1 = cpvadd((*a).p, (*spring).r1);
    tmp___2 = cpvadd((*b).p, (*spring).r2);
    tmp___3 = cpvsub(tmp___2, tmp___1);
    delta = tmp___3;
    tmp___4 = cpvlength(delta);
    dist = tmp___4;
    if dist != 0. {
        tmp___6 = dist;
    } else {
        tmp___5 = ::std::f32::INFINITY;
        tmp___6 = tmp___5 as cpFloat;
    }
    (*spring).n = cpvmult(delta, 1.0f32 as cpFloat / tmp___6);
    tmp___7 = k_scalar(a, b, (*spring).r1, (*spring).r2, (*spring).n);
    k = tmp___7;
    (*spring).nMass = 1.0f32 as cpFloat / k;
    (*spring).target_vrn = 0.0f32 as cpFloat;
    tmp___8 = exp(-(*spring).damping * dt * k);
    (*spring).v_coef = 1.0f32 as libc::c_double - tmp___8;
    tmp___9 = (Some(((*spring).springForceFunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(spring as *mut cpConstraint, dist);
    f_spring = tmp___9;
    tmp___10 = f_spring * dt;
    (*spring).jAcc = tmp___10;
    j_spring = tmp___10;
    tmp___11 = cpvmult((*spring).n, j_spring);
    apply_impulses(a, b, (*spring).r1, (*spring).r2, tmp___11);
}
unsafe extern "C" fn applyCachedImpulse___0(
    mut spring: *mut cpDampedSpring,
    mut dt_coef: cpFloat,
) {}
unsafe extern "C" fn applyImpulse___0(mut spring: *mut cpDampedSpring, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut r1: cpVect = cpVect { x: 0., y: 0. };
    let mut r2: cpVect = cpVect { x: 0., y: 0. };
    let mut vrn: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    let mut v_damp: cpFloat = 0.;
    let mut j_damp: cpFloat = 0.;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    a = (*spring).constraint.a;
    b = (*spring).constraint.b;
    n = (*spring).n;
    r1 = (*spring).r1;
    r2 = (*spring).r2;
    tmp = normal_relative_velocity(a, b, r1, r2, n);
    vrn = tmp;
    v_damp = ((*spring).target_vrn - vrn) * (*spring).v_coef;
    (*spring).target_vrn = vrn + v_damp;
    j_damp = v_damp * (*spring).nMass;
    (*spring).jAcc += j_damp;
    tmp___0 = cpvmult((*spring).n, j_damp);
    apply_impulses(a, b, (*spring).r1, (*spring).r2, tmp___0);
}
unsafe extern "C" fn getImpulse___0(mut spring: *mut cpDampedSpring) -> cpFloat {
    return (*spring).jAcc;
}
static mut klass___1: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    preStep___0
                        as unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> (),
                ),
            ),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyCachedImpulse___0
                        as unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyImpulse___0
                        as unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpDampedSpring) -> cpFloat>,
                Option::<unsafe extern "C" fn(*mut cpConstraint) -> cpFloat>,
            >(
                Some(
                    getImpulse___0
                        as unsafe extern "C" fn(*mut cpDampedSpring) -> cpFloat,
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpDampedSpringAlloc() -> *mut cpDampedSpring {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpDampedSpring>() as libc::c_ulong,
    );
    return tmp as *mut cpDampedSpring;
}
pub unsafe extern "C" fn cpDampedSpringInit(
    mut spring: *mut cpDampedSpring,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
    mut restLength: cpFloat,
    mut stiffness: cpFloat,
    mut damping: cpFloat,
) -> *mut cpDampedSpring {
    cpConstraintInit(spring as *mut cpConstraint, &klass___1, a, b);
    (*spring).anchorA = anchorA;
    (*spring).anchorB = anchorB;
    (*spring).restLength = restLength;
    (*spring).stiffness = stiffness;
    (*spring).damping = damping;
    (*spring)
        .springForceFunc = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> cpFloat>,
        Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat>,
    >(
        Some(
            defaultSpringForce
                as unsafe extern "C" fn(*mut cpDampedSpring, cpFloat) -> cpFloat,
        ),
    );
    (*spring).jAcc = 0.0f32 as cpFloat;
    return spring;
}
pub unsafe extern "C" fn cpDampedSpringNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
    mut restLength: cpFloat,
    mut stiffness: cpFloat,
    mut damping: cpFloat,
) -> *mut cpConstraint {
    let mut tmp: *mut cpDampedSpring = 0 as *mut cpDampedSpring;
    let mut tmp___0: *mut cpDampedSpring = 0 as *mut cpDampedSpring;
    tmp = cpDampedSpringAlloc();
    tmp___0 = cpDampedSpringInit(
        tmp,
        a,
        b,
        anchorA,
        anchorB,
        restLength,
        stiffness,
        damping,
    );
    return tmp___0 as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsDampedSpring(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass as libc::c_ulong
        == &klass___1 as *const cpConstraintClass as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpDampedSpringGetAnchorA(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).anchorA;
}
pub unsafe extern "C" fn cpDampedSpringSetAnchorA(
    mut constraint: *mut cpConstraint,
    mut anchorA: cpVect,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedSpring)).anchorA = anchorA;
}
pub unsafe extern "C" fn cpDampedSpringGetAnchorB(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).anchorB;
}
pub unsafe extern "C" fn cpDampedSpringSetAnchorB(
    mut constraint: *mut cpConstraint,
    mut anchorB: cpVect,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedSpring)).anchorB = anchorB;
}
pub unsafe extern "C" fn cpDampedSpringGetRestLength(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            161 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).restLength;
}
pub unsafe extern "C" fn cpDampedSpringSetRestLength(
    mut constraint: *mut cpConstraint,
    mut restLength: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedSpring)).restLength = restLength;
}
pub unsafe extern "C" fn cpDampedSpringGetStiffness(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).stiffness;
}
pub unsafe extern "C" fn cpDampedSpringSetStiffness(
    mut constraint: *mut cpConstraint,
    mut stiffness: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedSpring)).stiffness = stiffness;
}
pub unsafe extern "C" fn cpDampedSpringGetDamping(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            191 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).damping;
}
pub unsafe extern "C" fn cpDampedSpringSetDamping(
    mut constraint: *mut cpConstraint,
    mut damping: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpDampedSpring)).damping = damping;
}
pub unsafe extern "C" fn cpDampedSpringGetSpringForceFunc(
    mut constraint: *const cpConstraint,
) -> cpDampedSpringForceFunc {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpDampedSpring)).springForceFunc;
}
pub unsafe extern "C" fn cpDampedSpringSetSpringForceFunc(
    mut constraint: *mut cpConstraint,
    mut springForceFunc: Option::<
        unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> cpFloat,
    >,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsDampedSpring(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsDampedSpring(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpDampedSpring.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a damped spring.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    let ref mut fresh9 = (*(constraint as *mut cpDampedSpring)).springForceFunc;
    *fresh9 = springForceFunc;
}
#[inline]
unsafe extern "C" fn bias_coef(mut errorBias: cpFloat, mut dt: cpFloat) -> cpFloat {
    let mut tmp: libc::c_double = 0.;
    tmp = pow(errorBias, dt);
    return 1.0f32 as libc::c_double - tmp;
}
unsafe extern "C" fn preStep___1(mut joint: *mut cpGearJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut maxBias: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    (*joint)
        .iSum = 1.0f32 as cpFloat
        / ((*a).i_inv * (*joint).ratio_inv + (*joint).ratio * (*b).i_inv);
    maxBias = (*joint).constraint.maxBias;
    tmp = bias_coef((*joint).constraint.errorBias, dt);
    (*joint)
        .bias = cpfclamp(
        -tmp * ((*b).a * (*joint).ratio - (*a).a - (*joint).phase) / dt,
        -maxBias,
        maxBias,
    );
}
unsafe extern "C" fn applyCachedImpulse___1(
    mut joint: *mut cpGearJoint,
    mut dt_coef: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut j: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    j = (*joint).jAcc * dt_coef;
    (*a).w -= j * (*a).i_inv * (*joint).ratio_inv;
    (*b).w += j * (*b).i_inv;
}
unsafe extern "C" fn applyImpulse___1(mut joint: *mut cpGearJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut wr: cpFloat = 0.;
    let mut jMax: cpFloat = 0.;
    let mut j: cpFloat = 0.;
    let mut jOld: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    wr = (*b).w * (*joint).ratio - (*a).w;
    jMax = (*joint).constraint.maxForce * dt;
    j = ((*joint).bias - wr) * (*joint).iSum;
    jOld = (*joint).jAcc;
    (*joint).jAcc = cpfclamp(jOld + j, -jMax, jMax);
    j = (*joint).jAcc - jOld;
    (*a).w -= j * (*a).i_inv * (*joint).ratio_inv;
    (*b).w += j * (*b).i_inv;
}
unsafe extern "C" fn getImpulse___1(mut joint: *mut cpGearJoint) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpfabs((*joint).jAcc);
    return tmp;
}
static mut klass___2: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpGearJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    preStep___1 as unsafe extern "C" fn(*mut cpGearJoint, cpFloat) -> (),
                ),
            ),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpGearJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyCachedImpulse___1
                        as unsafe extern "C" fn(*mut cpGearJoint, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpGearJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyImpulse___1
                        as unsafe extern "C" fn(*mut cpGearJoint, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpGearJoint) -> cpFloat>,
                Option::<unsafe extern "C" fn(*mut cpConstraint) -> cpFloat>,
            >(Some(getImpulse___1 as unsafe extern "C" fn(*mut cpGearJoint) -> cpFloat)),
        };
        init
    }
};
pub unsafe extern "C" fn cpGearJointAlloc() -> *mut cpGearJoint {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpGearJoint>() as libc::c_ulong,
    );
    return tmp as *mut cpGearJoint;
}
pub unsafe extern "C" fn cpGearJointInit(
    mut joint: *mut cpGearJoint,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut phase: cpFloat,
    mut ratio: cpFloat,
) -> *mut cpGearJoint {
    cpConstraintInit(joint as *mut cpConstraint, &klass___2, a, b);
    (*joint).phase = phase;
    (*joint).ratio = ratio;
    (*joint).ratio_inv = 1.0f32 as cpFloat / ratio;
    (*joint).jAcc = 0.0f32 as cpFloat;
    return joint;
}
pub unsafe extern "C" fn cpGearJointNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut phase: cpFloat,
    mut ratio: cpFloat,
) -> *mut cpConstraint {
    let mut tmp: *mut cpGearJoint = 0 as *mut cpGearJoint;
    let mut tmp___0: *mut cpGearJoint = 0 as *mut cpGearJoint;
    tmp = cpGearJointAlloc();
    tmp___0 = cpGearJointInit(tmp, a, b, phase, ratio);
    return tmp___0 as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsGearJoint(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass as libc::c_ulong
        == &klass___2 as *const cpConstraintClass as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpGearJointGetPhase(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsGearJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsGearJoint(constraint)\0" as *const u8 as *const libc::c_char,
            b"../src/cpGearJoint.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a ratchet joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpGearJoint)).phase;
}
pub unsafe extern "C" fn cpGearJointSetPhase(
    mut constraint: *mut cpConstraint,
    mut phase: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsGearJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsGearJoint(constraint)\0" as *const u8 as *const libc::c_char,
            b"../src/cpGearJoint.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a ratchet joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpGearJoint)).phase = phase;
}
pub unsafe extern "C" fn cpGearJointGetRatio(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsGearJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsGearJoint(constraint)\0" as *const u8 as *const libc::c_char,
            b"../src/cpGearJoint.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a ratchet joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpGearJoint)).ratio;
}
pub unsafe extern "C" fn cpGearJointSetRatio(
    mut constraint: *mut cpConstraint,
    mut ratio: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsGearJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsGearJoint(constraint)\0" as *const u8 as *const libc::c_char,
            b"../src/cpGearJoint.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a ratchet joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpGearJoint)).ratio = ratio;
    (*(constraint as *mut cpGearJoint)).ratio_inv = 1.0f32 as cpFloat / ratio;
}
static mut cpvzero___3: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpvproject(v1: cpVect, v2: cpVect) -> cpVect {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvdot(v1, v2);
    tmp___0 = cpvdot(v2, v2);
    tmp___1 = cpvmult(v2, tmp / tmp___0);
    return tmp___1;
}
#[inline]
unsafe extern "C" fn cpvclamp(v: cpVect, len: cpFloat) -> cpVect {
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpFloat = 0.;
    tmp___3 = cpvdot(v, v);
    if tmp___3 > len * len {
        tmp___0 = cpvnormalize(v);
        tmp___1 = cpvmult(tmp___0, len);
        tmp___2 = tmp___1;
    } else {
        tmp___2 = v;
    }
    return tmp___2;
}
#[inline]
unsafe extern "C" fn cpMat2x2New(
    mut a: cpFloat,
    mut b: cpFloat,
    mut c: cpFloat,
    mut d: cpFloat,
) -> cpMat2x2 {
    let mut m: cpMat2x2 = cpMat2x2 {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
    };
    m.a = a;
    m.b = b;
    m.c = c;
    m.d = d;
    return m;
}
#[inline]
unsafe extern "C" fn cpMat2x2Transform(mut m: cpMat2x2, mut v: cpVect) -> cpVect {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpv(v.x * m.a + v.y * m.b, v.x * m.c + v.y * m.d);
    return tmp;
}
#[inline]
unsafe extern "C" fn k_tensor(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut r1: cpVect,
    mut r2: cpVect,
) -> cpMat2x2 {
    let mut m_sum: cpFloat = 0.;
    let mut k11: cpFloat = 0.;
    let mut k12: cpFloat = 0.;
    let mut k21: cpFloat = 0.;
    let mut k22: cpFloat = 0.;
    let mut a_i_inv: cpFloat = 0.;
    let mut r1xsq: cpFloat = 0.;
    let mut r1ysq: cpFloat = 0.;
    let mut r1nxy: cpFloat = 0.;
    let mut b_i_inv: cpFloat = 0.;
    let mut r2xsq: cpFloat = 0.;
    let mut r2ysq: cpFloat = 0.;
    let mut r2nxy: cpFloat = 0.;
    let mut det: cpFloat = 0.;
    let mut det_inv: cpFloat = 0.;
    let mut tmp: cpMat2x2 = cpMat2x2 {
        a: 0.,
        b: 0.,
        c: 0.,
        d: 0.,
    };
    m_sum = (*a).m_inv + (*b).m_inv;
    k11 = m_sum;
    k12 = 0.0f32 as cpFloat;
    k21 = 0.0f32 as cpFloat;
    k22 = m_sum;
    a_i_inv = (*a).i_inv;
    r1xsq = r1.x * r1.x * a_i_inv;
    r1ysq = r1.y * r1.y * a_i_inv;
    r1nxy = -r1.x * r1.y * a_i_inv;
    k11 += r1ysq;
    k12 += r1nxy;
    k21 += r1nxy;
    k22 += r1xsq;
    b_i_inv = (*b).i_inv;
    r2xsq = r2.x * r2.x * b_i_inv;
    r2ysq = r2.y * r2.y * b_i_inv;
    r2nxy = -r2.x * r2.y * b_i_inv;
    k11 += r2ysq;
    k12 += r2nxy;
    k21 += r2nxy;
    k22 += r2xsq;
    det = k11 * k22 - k12 * k21;
    det_inv = 1.0f32 as cpFloat / det;
    tmp = cpMat2x2New(k22 * det_inv, -k12 * det_inv, -k21 * det_inv, k11 * det_inv);
    return tmp;
}
unsafe extern "C" fn preStep___2(mut joint: *mut cpGrooveJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut ta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tb: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut d: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut td: cpFloat = 0.;
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpFloat = 0.;
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___9: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___10: cpFloat = 0.;
    let mut tmp___11: cpFloat = 0.;
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___12: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___13: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___14: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___15: cpFloat = 0.;
    let mut tmp___16: cpVect = cpVect { x: 0., y: 0. };
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    tmp = cpTransformPoint((*a).transform, (*joint).grv_a);
    ta = tmp;
    tmp___0 = cpTransformPoint((*a).transform, (*joint).grv_b);
    tb = tmp___0;
    tmp___1 = cpTransformVect((*a).transform, (*joint).grv_n);
    n = tmp___1;
    tmp___2 = cpvdot(ta, n);
    d = tmp___2;
    (*joint).grv_tn = n;
    tmp___3 = cpvsub((*joint).anchorB, (*b).cog);
    (*joint).r2 = cpTransformVect((*b).transform, tmp___3);
    tmp___4 = cpvadd((*b).p, (*joint).r2);
    tmp___5 = cpvcross(tmp___4, n);
    td = tmp___5;
    tmp___11 = cpvcross(ta, n);
    if td <= tmp___11 {
        (*joint).clamp = 1.0f32 as cpFloat;
        (*joint).r1 = cpvsub(ta, (*a).p);
    } else {
        tmp___10 = cpvcross(tb, n);
        if td >= tmp___10 {
            (*joint).clamp = -1.0f32 as cpFloat;
            (*joint).r1 = cpvsub(tb, (*a).p);
        } else {
            (*joint).clamp = 0.0f32 as cpFloat;
            tmp___6 = cpvmult(n, d);
            tmp___7 = cpvperp(n);
            tmp___8 = cpvmult(tmp___7, -td);
            tmp___9 = cpvadd(tmp___8, tmp___6);
            (*joint).r1 = cpvsub(tmp___9, (*a).p);
        }
    }
    (*joint).k = k_tensor(a, b, (*joint).r1, (*joint).r2);
    tmp___12 = cpvadd((*a).p, (*joint).r1);
    tmp___13 = cpvadd((*b).p, (*joint).r2);
    tmp___14 = cpvsub(tmp___13, tmp___12);
    delta = tmp___14;
    tmp___15 = bias_coef((*joint).constraint.errorBias, dt);
    tmp___16 = cpvmult(delta, -tmp___15 / dt);
    (*joint).bias = cpvclamp(tmp___16, (*joint).constraint.maxBias);
}
unsafe extern "C" fn applyCachedImpulse___2(
    mut joint: *mut cpGrooveJoint,
    mut dt_coef: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    tmp = cpvmult((*joint).jAcc, dt_coef);
    apply_impulses(a, b, (*joint).r1, (*joint).r2, tmp);
}
#[inline]
unsafe extern "C" fn grooveConstrain(
    mut joint: *mut cpGrooveJoint,
    mut j: cpVect,
    mut dt: cpFloat,
) -> cpVect {
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut jClamp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    n = (*joint).grv_tn;
    tmp___2 = cpvcross(j, n);
    if (*joint).clamp * tmp___2 > 0.0f32 as cpFloat {
        tmp___1 = j;
    } else {
        tmp___0 = cpvproject(j, n);
        tmp___1 = tmp___0;
    }
    jClamp = tmp___1;
    tmp___3 = cpvclamp(jClamp, (*joint).constraint.maxForce * dt);
    return tmp___3;
}
unsafe extern "C" fn applyImpulse___2(mut joint: *mut cpGrooveJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut r1: cpVect = cpVect { x: 0., y: 0. };
    let mut r2: cpVect = cpVect { x: 0., y: 0. };
    let mut vr: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut j: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut jOld: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    r1 = (*joint).r1;
    r2 = (*joint).r2;
    tmp = relative_velocity(a, b, r1, r2);
    vr = tmp;
    tmp___0 = cpvsub((*joint).bias, vr);
    tmp___1 = cpMat2x2Transform((*joint).k, tmp___0);
    j = tmp___1;
    jOld = (*joint).jAcc;
    tmp___2 = cpvadd(jOld, j);
    (*joint).jAcc = grooveConstrain(joint, tmp___2, dt);
    j = cpvsub((*joint).jAcc, jOld);
    apply_impulses(a, b, (*joint).r1, (*joint).r2, j);
}
unsafe extern "C" fn getImpulse___2(mut joint: *mut cpGrooveJoint) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpvlength((*joint).jAcc);
    return tmp;
}
static mut klass___3: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpGrooveJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    preStep___2
                        as unsafe extern "C" fn(*mut cpGrooveJoint, cpFloat) -> (),
                ),
            ),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpGrooveJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyCachedImpulse___2
                        as unsafe extern "C" fn(*mut cpGrooveJoint, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpGrooveJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyImpulse___2
                        as unsafe extern "C" fn(*mut cpGrooveJoint, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpGrooveJoint) -> cpFloat>,
                Option::<unsafe extern "C" fn(*mut cpConstraint) -> cpFloat>,
            >(
                Some(
                    getImpulse___2 as unsafe extern "C" fn(*mut cpGrooveJoint) -> cpFloat,
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpGrooveJointAlloc() -> *mut cpGrooveJoint {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpGrooveJoint>() as libc::c_ulong,
    );
    return tmp as *mut cpGrooveJoint;
}
pub unsafe extern "C" fn cpGrooveJointInit(
    mut joint: *mut cpGrooveJoint,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut groove_a: cpVect,
    mut groove_b: cpVect,
    mut anchorB: cpVect,
) -> *mut cpGrooveJoint {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    cpConstraintInit(joint as *mut cpConstraint, &klass___3, a, b);
    (*joint).grv_a = groove_a;
    (*joint).grv_b = groove_b;
    tmp = cpvsub(groove_b, groove_a);
    tmp___0 = cpvnormalize(tmp);
    (*joint).grv_n = cpvperp(tmp___0);
    (*joint).anchorB = anchorB;
    (*joint).jAcc = cpvzero___3;
    return joint;
}
pub unsafe extern "C" fn cpGrooveJointNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut groove_a: cpVect,
    mut groove_b: cpVect,
    mut anchorB: cpVect,
) -> *mut cpConstraint {
    let mut tmp: *mut cpGrooveJoint = 0 as *mut cpGrooveJoint;
    let mut tmp___0: *mut cpGrooveJoint = 0 as *mut cpGrooveJoint;
    tmp = cpGrooveJointAlloc();
    tmp___0 = cpGrooveJointInit(tmp, a, b, groove_a, groove_b, anchorB);
    return tmp___0 as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsGrooveJoint(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass as libc::c_ulong
        == &klass___3 as *const cpConstraintClass as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpGrooveJointGetGrooveA(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsGrooveJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsGrooveJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpGrooveJoint.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a groove joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpGrooveJoint)).grv_a;
}
pub unsafe extern "C" fn cpGrooveJointSetGrooveA(
    mut constraint: *mut cpConstraint,
    mut value: cpVect,
) {
    let mut tmp: cpBool = 0;
    let mut g: *mut cpGrooveJoint = 0 as *mut cpGrooveJoint;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpConstraintIsGrooveJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsGrooveJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpGrooveJoint.c\0" as *const u8 as *const libc::c_char,
            156 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a groove joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    g = constraint as *mut cpGrooveJoint;
    (*g).grv_a = value;
    tmp___0 = cpvsub((*g).grv_b, value);
    tmp___1 = cpvnormalize(tmp___0);
    (*g).grv_n = cpvperp(tmp___1);
    cpConstraintActivateBodies(constraint);
}
pub unsafe extern "C" fn cpGrooveJointGetGrooveB(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsGrooveJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsGrooveJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpGrooveJoint.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a groove joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpGrooveJoint)).grv_b;
}
pub unsafe extern "C" fn cpGrooveJointSetGrooveB(
    mut constraint: *mut cpConstraint,
    mut value: cpVect,
) {
    let mut tmp: cpBool = 0;
    let mut g: *mut cpGrooveJoint = 0 as *mut cpGrooveJoint;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpConstraintIsGrooveJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsGrooveJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpGrooveJoint.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a groove joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    g = constraint as *mut cpGrooveJoint;
    (*g).grv_b = value;
    tmp___0 = cpvsub(value, (*g).grv_a);
    tmp___1 = cpvnormalize(tmp___0);
    (*g).grv_n = cpvperp(tmp___1);
    cpConstraintActivateBodies(constraint);
}
pub unsafe extern "C" fn cpGrooveJointGetAnchorB(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsGrooveJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsGrooveJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpGrooveJoint.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a groove joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpGrooveJoint)).anchorB;
}
pub unsafe extern "C" fn cpGrooveJointSetAnchorB(
    mut constraint: *mut cpConstraint,
    mut anchorB: cpVect,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsGrooveJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsGrooveJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpGrooveJoint.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a groove joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpGrooveJoint)).anchorB = anchorB;
}
static mut primes: [libc::c_int; 30] = [
    5 as libc::c_int,
    13 as libc::c_int,
    23 as libc::c_int,
    47 as libc::c_int,
    97 as libc::c_int,
    193 as libc::c_int,
    389 as libc::c_int,
    769 as libc::c_int,
    1543 as libc::c_int,
    3079 as libc::c_int,
    6151 as libc::c_int,
    12289 as libc::c_int,
    24593 as libc::c_int,
    49157 as libc::c_int,
    98317 as libc::c_int,
    196613 as libc::c_int,
    393241 as libc::c_int,
    786433 as libc::c_int,
    1572869 as libc::c_int,
    3145739 as libc::c_int,
    6291469 as libc::c_int,
    12582917 as libc::c_int,
    25165843 as libc::c_int,
    50331653 as libc::c_int,
    100663319 as libc::c_int,
    201326611 as libc::c_int,
    402653189 as libc::c_int,
    805306457 as libc::c_int,
    1610612741 as libc::c_int,
    0 as libc::c_int,
];
#[inline]
unsafe extern "C" fn next_prime(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while n > primes[i as usize] {
        i += 1;
        if primes[i as usize] == 0 {
            cpMessage(
                b"primes[i]\0" as *const u8 as *const libc::c_char,
                b"../src/prime.h\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Tried to resize a hash table to a size greater than 1610612741 O_o\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
    }
    return primes[i as usize];
}
pub unsafe extern "C" fn cpHashSetFree(mut set: *mut cpHashSet) {
    if !set.is_null() {
        free((*set).table as *mut libc::c_void);
        cpArrayFreeEach(
            (*set).allocatedBuffers,
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        cpArrayFree((*set).allocatedBuffers);
        free(set as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpHashSetNew(
    mut size: libc::c_int,
    mut eqlFunc: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cpBool,
    >,
) -> *mut cpHashSet {
    let mut set: *mut cpHashSet = 0 as *mut cpHashSet;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpHashSet>() as libc::c_ulong,
    );
    set = tmp as *mut cpHashSet;
    tmp___0 = next_prime(size);
    (*set).size = tmp___0 as libc::c_uint;
    (*set).entries = 0 as libc::c_uint;
    (*set).eql = eqlFunc;
    (*set).default_value = 0 as *mut libc::c_void;
    tmp___1 = calloc(
        (*set).size as size_t,
        ::std::mem::size_of::<*mut cpHashSetBin>() as libc::c_ulong,
    );
    (*set).table = tmp___1 as *mut *mut cpHashSetBin;
    (*set).pooledBins = 0 as *mut libc::c_void as *mut cpHashSetBin;
    (*set).allocatedBuffers = cpArrayNew(0 as libc::c_int);
    return set;
}
pub unsafe extern "C" fn cpHashSetSetDefaultValue(
    mut set: *mut cpHashSet,
    mut default_value: *mut libc::c_void,
) {
    (*set).default_value = default_value;
}
unsafe extern "C" fn setIsFull(mut set: *mut cpHashSet) -> libc::c_int {
    return ((*set).entries >= (*set).size) as libc::c_int;
}
unsafe extern "C" fn cpHashSetResize(mut set: *mut cpHashSet) {
    let mut newSize: libc::c_uint = 0;
    let mut tmp: libc::c_int = 0;
    let mut newTable: *mut *mut cpHashSetBin = 0 as *mut *mut cpHashSetBin;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_uint = 0;
    let mut bin: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    let mut next: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    let mut idx: cpHashValue = 0;
    tmp = next_prime(((*set).size).wrapping_add(1 as libc::c_uint) as libc::c_int);
    newSize = tmp as libc::c_uint;
    tmp___0 = calloc(
        newSize as size_t,
        ::std::mem::size_of::<*mut cpHashSetBin>() as libc::c_ulong,
    );
    newTable = tmp___0 as *mut *mut cpHashSetBin;
    i = 0 as libc::c_uint;
    while i < (*set).size {
        bin = *((*set).table).offset(i as isize);
        while !bin.is_null() {
            next = (*bin).next;
            idx = ((*bin).hash).wrapping_rem(newSize as libc::c_ulong);
            (*bin).next = *newTable.offset(idx as isize);
            let ref mut fresh10 = *newTable.offset(idx as isize);
            *fresh10 = bin;
            bin = next;
        }
        i = i.wrapping_add(1);
    }
    free((*set).table as *mut libc::c_void);
    (*set).table = newTable;
    (*set).size = newSize;
}
#[inline]
unsafe extern "C" fn recycleBin(mut set: *mut cpHashSet, mut bin: *mut cpHashSetBin) {
    (*bin).next = (*set).pooledBins;
    (*set).pooledBins = bin;
    (*bin).elt = 0 as *mut libc::c_void;
}
unsafe extern "C" fn getUnusedBin(mut set: *mut cpHashSet) -> *mut cpHashSetBin {
    let mut bin: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    let mut count: libc::c_int = 0;
    let mut buffer: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    bin = (*set).pooledBins;
    if !bin.is_null() {
        (*set).pooledBins = (*bin).next;
        return bin;
    } else {
        count = (32768 as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cpHashSetBin>() as libc::c_ulong)
            as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../src/cpHashSet.c\0" as *const u8 as *const libc::c_char,
                135 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size is too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        tmp = calloc(1 as libc::c_int as size_t, 32768 as libc::c_int as size_t);
        buffer = tmp as *mut cpHashSetBin;
        cpArrayPush((*set).allocatedBuffers, buffer as *mut libc::c_void);
        i = 1 as libc::c_int;
        while i < count {
            recycleBin(set, buffer.offset(i as isize));
            i += 1;
        }
        return buffer;
    };
}
pub unsafe extern "C" fn cpHashSetCount(mut set: *mut cpHashSet) -> libc::c_int {
    return (*set).entries as libc::c_int;
}
pub unsafe extern "C" fn cpHashSetInsert(
    mut set: *mut cpHashSet,
    mut hash: cpHashValue,
    mut ptr: *const libc::c_void,
    mut trans: Option::<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    mut data: *mut libc::c_void,
) -> *const libc::c_void {
    let mut idx: cpHashValue = 0;
    let mut bin: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    let mut tmp: cpBool = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: libc::c_int = 0;
    idx = hash.wrapping_rem((*set).size as libc::c_ulong);
    bin = *((*set).table).offset(idx as isize);
    while !bin.is_null() {
        tmp = (Some(((*set).eql).expect("non-null function pointer")))
            .expect("non-null function pointer")(ptr, (*bin).elt as *const libc::c_void);
        if tmp != 0 {
            break;
        }
        bin = (*bin).next;
    }
    if bin.is_null() {
        bin = getUnusedBin(set);
        (*bin).hash = hash;
        if trans.is_some() {
            tmp___0 = (Some(trans.expect("non-null function pointer")))
                .expect("non-null function pointer")(ptr, data);
            (*bin).elt = tmp___0;
        } else {
            (*bin).elt = data;
        }
        (*bin).next = *((*set).table).offset(idx as isize);
        let ref mut fresh11 = *((*set).table).offset(idx as isize);
        *fresh11 = bin;
        (*set).entries = ((*set).entries).wrapping_add(1);
        tmp___1 = setIsFull(set);
        if tmp___1 != 0 {
            cpHashSetResize(set);
        }
    }
    return (*bin).elt as *const libc::c_void;
}
pub unsafe extern "C" fn cpHashSetRemove(
    mut set: *mut cpHashSet,
    mut hash: cpHashValue,
    mut ptr: *const libc::c_void,
) -> *const libc::c_void {
    let mut idx: cpHashValue = 0;
    let mut prev_ptr: *mut *mut cpHashSetBin = 0 as *mut *mut cpHashSetBin;
    let mut bin: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    let mut tmp: cpBool = 0;
    let mut elt: *const libc::c_void = 0 as *const libc::c_void;
    idx = hash.wrapping_rem((*set).size as libc::c_ulong);
    prev_ptr = ((*set).table).offset(idx as isize);
    bin = *((*set).table).offset(idx as isize);
    while !bin.is_null() {
        tmp = (Some(((*set).eql).expect("non-null function pointer")))
            .expect("non-null function pointer")(ptr, (*bin).elt as *const libc::c_void);
        if tmp != 0 {
            break;
        }
        prev_ptr = &mut (*bin).next;
        bin = (*bin).next;
    }
    if !bin.is_null() {
        *prev_ptr = (*bin).next;
        (*set).entries = ((*set).entries).wrapping_sub(1);
        elt = (*bin).elt as *const libc::c_void;
        recycleBin(set, bin);
        return elt;
    }
    return 0 as *mut libc::c_void as *const libc::c_void;
}
pub unsafe extern "C" fn cpHashSetFind(
    mut set: *mut cpHashSet,
    mut hash: cpHashValue,
    mut ptr: *const libc::c_void,
) -> *const libc::c_void {
    let mut idx: cpHashValue = 0;
    let mut bin: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    let mut tmp: cpBool = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    idx = hash.wrapping_rem((*set).size as libc::c_ulong);
    bin = *((*set).table).offset(idx as isize);
    while !bin.is_null() {
        tmp = (Some(((*set).eql).expect("non-null function pointer")))
            .expect("non-null function pointer")(ptr, (*bin).elt as *const libc::c_void);
        if tmp != 0 {
            break;
        }
        bin = (*bin).next;
    }
    if !bin.is_null() {
        tmp___0 = (*bin).elt;
    } else {
        tmp___0 = (*set).default_value;
    }
    return tmp___0 as *const libc::c_void;
}
pub unsafe extern "C" fn cpHashSetEach(
    mut set: *mut cpHashSet,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    let mut i: libc::c_uint = 0;
    let mut bin: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    let mut next: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    i = 0 as libc::c_uint;
    while i < (*set).size {
        bin = *((*set).table).offset(i as isize);
        while !bin.is_null() {
            next = (*bin).next;
            (Some(func.expect("non-null function pointer")))
                .expect("non-null function pointer")((*bin).elt, data);
            bin = next;
        }
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn cpHashSetFilter(
    mut set: *mut cpHashSet,
    mut func: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> cpBool,
    >,
    mut data: *mut libc::c_void,
) {
    let mut i: libc::c_uint = 0;
    let mut prev_ptr: *mut *mut cpHashSetBin = 0 as *mut *mut cpHashSetBin;
    let mut bin: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    let mut next: *mut cpHashSetBin = 0 as *mut cpHashSetBin;
    let mut tmp: cpBool = 0;
    i = 0 as libc::c_uint;
    while i < (*set).size {
        prev_ptr = ((*set).table).offset(i as isize);
        bin = *((*set).table).offset(i as isize);
        while !bin.is_null() {
            next = (*bin).next;
            tmp = (Some(func.expect("non-null function pointer")))
                .expect("non-null function pointer")((*bin).elt, data);
            if tmp != 0 {
                prev_ptr = &mut (*bin).next;
            } else {
                *prev_ptr = next;
                (*set).entries = ((*set).entries).wrapping_sub(1);
                recycleBin(set, bin);
            }
            bin = next;
        }
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn cpSpatialIndexEach(
    mut index: *mut cpSpatialIndex,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    (Some(((*(*index).klass).each).expect("non-null function pointer")))
        .expect("non-null function pointer")(index, func, data);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexReindexQuery(
    mut index: *mut cpSpatialIndex,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    (Some(((*(*index).klass).reindexQuery).expect("non-null function pointer")))
        .expect("non-null function pointer")(index, func, data);
}
unsafe extern "C" fn WorkerThreadLoop(
    mut context: *mut ThreadContext,
) -> *mut libc::c_void {
    let mut hasty: *mut cpHastySpace = 0 as *mut cpHastySpace;
    let mut thread: libc::c_ulong = 0;
    let mut num_threads: libc::c_ulong = 0;
    let mut func: Option::<
        unsafe extern "C" fn(*mut cpSpace, libc::c_ulong, libc::c_ulong) -> (),
    > = None;
    hasty = (*context).space;
    thread = (*context).thread_num;
    num_threads = (*hasty).num_threads;
    loop {
        pthread_mutex_lock(&mut (*hasty).mutex);
        (*hasty).num_working = ((*hasty).num_working).wrapping_sub(1);
        if (*hasty).num_working == 0 as libc::c_ulong {
            pthread_cond_signal(&mut (*hasty).cond_resume);
        }
        pthread_cond_wait(
            &mut (*hasty).cond_work as *mut pthread_cond_t,
            &mut (*hasty).mutex as *mut pthread_mutex_t,
        );
        pthread_mutex_unlock(&mut (*hasty).mutex);
        func = (*hasty).work;
        if !func.is_some() {
            break;
        }
        (Some(((*hasty).work).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(&mut (*hasty).space, thread, num_threads);
    }
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn RunWorkers(
    mut hasty: *mut cpHastySpace,
    mut func: Option::<
        unsafe extern "C" fn(*mut cpSpace, libc::c_ulong, libc::c_ulong) -> (),
    >,
) {
    (*hasty).num_working = ((*hasty).num_threads).wrapping_sub(1 as libc::c_ulong);
    (*hasty).work = func;
    if (*hasty).num_working > 0 as libc::c_ulong {
        pthread_mutex_lock(&mut (*hasty).mutex);
        pthread_cond_broadcast(&mut (*hasty).cond_work);
        pthread_mutex_unlock(&mut (*hasty).mutex);
        (Some(func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(hasty as *mut cpSpace, 0 as libc::c_ulong, (*hasty).num_threads);
        pthread_mutex_lock(&mut (*hasty).mutex);
        if (*hasty).num_working > 0 as libc::c_ulong {
            pthread_cond_wait(
                &mut (*hasty).cond_resume as *mut pthread_cond_t,
                &mut (*hasty).mutex as *mut pthread_mutex_t,
            );
        }
        pthread_mutex_unlock(&mut (*hasty).mutex);
    } else {
        (Some(func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(hasty as *mut cpSpace, 0 as libc::c_ulong, (*hasty).num_threads);
    }
    (*hasty)
        .work = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut cpSpace, libc::c_ulong, libc::c_ulong) -> ()>,
    >(0 as *mut libc::c_void);
}
unsafe extern "C" fn Solver(
    mut space: *mut cpSpace,
    mut worker: libc::c_ulong,
    mut worker_count: libc::c_ulong,
) {
    let mut constraints: *mut cpArray = 0 as *mut cpArray;
    let mut arbiters: *mut cpArray = 0 as *mut cpArray;
    let mut dt: cpFloat = 0.;
    let mut iterations: libc::c_ulong = 0;
    let mut i: libc::c_ulong = 0;
    let mut j: libc::c_int = 0;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut j___0: libc::c_int = 0;
    let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
    constraints = (*space).constraints;
    arbiters = (*space).arbiters;
    dt = (*space).curr_dt;
    iterations = ((*space).iterations as libc::c_ulong)
        .wrapping_add(worker_count)
        .wrapping_sub(1 as libc::c_ulong)
        .wrapping_div(worker_count);
    i = 0 as libc::c_ulong;
    while i < iterations {
        j = 0 as libc::c_int;
        while j < (*arbiters).num {
            arb = *((*arbiters).arr).offset(j as isize) as *mut cpArbiter;
            cpArbiterApplyImpulse(arb);
            j += 1;
        }
        j___0 = 0 as libc::c_int;
        while j___0 < (*constraints).num {
            constraint = *((*constraints).arr).offset(j___0 as isize)
                as *mut cpConstraint;
            (Some(
                ((*(*constraint).klass).applyImpulse).expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(constraint, dt);
            j___0 += 1;
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn HaltThreads(mut hasty: *mut cpHastySpace) {
    let mut mutex: *mut pthread_mutex_t = 0 as *mut pthread_mutex_t;
    let mut i: libc::c_ulong = 0;
    mutex = &mut (*hasty).mutex;
    pthread_mutex_lock(mutex);
    (*hasty)
        .work = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(*mut cpSpace, libc::c_ulong, libc::c_ulong) -> ()>,
    >(0 as *mut libc::c_void);
    pthread_cond_broadcast(&mut (*hasty).cond_work);
    pthread_mutex_unlock(mutex);
    i = 0 as libc::c_ulong;
    while i < ((*hasty).num_threads).wrapping_sub(1 as libc::c_ulong) {
        pthread_join(
            (*((*hasty).workers).as_mut_ptr().offset(i as isize)).thread,
            0 as *mut libc::c_void as *mut *mut libc::c_void,
        );
        i = i.wrapping_add(1);
    }
}
pub unsafe extern "C" fn cpHastySpaceSetThreads(
    mut space: *mut cpSpace,
    mut threads: libc::c_ulong,
) {
    let mut hasty: *mut cpHastySpace = 0 as *mut cpHastySpace;
    let mut i: libc::c_ulong = 0;
    hasty = space as *mut cpHastySpace;
    HaltThreads(hasty);
    if threads == 0 as libc::c_ulong {
        threads = 1 as libc::c_ulong;
    }
    if threads < 2 as libc::c_ulong {
        (*hasty).num_threads = threads;
    } else {
        (*hasty).num_threads = 2 as libc::c_ulong;
    }
    (*hasty).num_working = ((*hasty).num_threads).wrapping_sub(1 as libc::c_ulong);
    if (*hasty).num_working > 0 as libc::c_ulong {
        pthread_mutex_lock(&mut (*hasty).mutex);
        i = 0 as libc::c_ulong;
        while i < ((*hasty).num_threads).wrapping_sub(1 as libc::c_ulong) {
            let ref mut fresh12 = (*((*hasty).workers).as_mut_ptr().offset(i as isize))
                .space;
            *fresh12 = hasty;
            (*((*hasty).workers).as_mut_ptr().offset(i as isize))
                .thread_num = i.wrapping_add(1 as libc::c_ulong);
            pthread_create(
                &mut (*((*hasty).workers).as_mut_ptr().offset(i as isize)).thread
                    as *mut pthread_t,
                0 as *mut libc::c_void as *const pthread_attr_t,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(*mut ThreadContext) -> *mut libc::c_void,
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                    >,
                >(
                    Some(
                        WorkerThreadLoop
                            as unsafe extern "C" fn(
                                *mut ThreadContext,
                            ) -> *mut libc::c_void,
                    ),
                ),
                &mut *((*hasty).workers).as_mut_ptr().offset(i as isize)
                    as *mut ThreadContext as *mut libc::c_void,
            );
            i = i.wrapping_add(1);
        }
        pthread_cond_wait(
            &mut (*hasty).cond_resume as *mut pthread_cond_t,
            &mut (*hasty).mutex as *mut pthread_mutex_t,
        );
        pthread_mutex_unlock(&mut (*hasty).mutex);
    }
}
pub unsafe extern "C" fn cpHastySpaceGetThreads(
    mut space: *mut cpSpace,
) -> libc::c_ulong {
    return (*(space as *mut cpHastySpace)).num_threads;
}
pub unsafe extern "C" fn cpHastySpaceNew() -> *mut cpSpace {
    let mut hasty: *mut cpHastySpace = 0 as *mut cpHastySpace;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpHastySpace>() as libc::c_ulong,
    );
    hasty = tmp as *mut cpHastySpace;
    cpSpaceInit(hasty as *mut cpSpace);
    pthread_mutex_init(
        &mut (*hasty).mutex,
        0 as *mut libc::c_void as *const pthread_mutexattr_t,
    );
    pthread_cond_init(
        &mut (*hasty).cond_work as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    pthread_cond_init(
        &mut (*hasty).cond_resume as *mut pthread_cond_t,
        0 as *mut libc::c_void as *const pthread_condattr_t,
    );
    (*hasty).constraint_count_threshold = 50 as libc::c_ulong;
    (*hasty).num_threads = 1 as libc::c_ulong;
    cpHastySpaceSetThreads(hasty as *mut cpSpace, 1 as libc::c_ulong);
    return hasty as *mut cpSpace;
}
pub unsafe extern "C" fn cpHastySpaceFree(mut space: *mut cpSpace) {
    let mut hasty: *mut cpHastySpace = 0 as *mut cpHastySpace;
    hasty = space as *mut cpHastySpace;
    HaltThreads(hasty);
    pthread_mutex_destroy(&mut (*hasty).mutex);
    pthread_cond_destroy(&mut (*hasty).cond_work);
    pthread_cond_destroy(&mut (*hasty).cond_resume);
    cpSpaceFree(space);
}
pub unsafe extern "C" fn cpHastySpaceStep(mut space: *mut cpSpace, mut dt: cpFloat) {
    let mut prev_dt: cpFloat = 0.;
    let mut bodies: *mut cpArray = 0 as *mut cpArray;
    let mut constraints: *mut cpArray = 0 as *mut cpArray;
    let mut arbiters: *mut cpArray = 0 as *mut cpArray;
    let mut i: libc::c_int = 0;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut tmp: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    let mut i___0: libc::c_int = 0;
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut slop: cpFloat = 0.;
    let mut biasCoef: cpFloat = 0.;
    let mut tmp___1: libc::c_double = 0.;
    let mut i___1: libc::c_int = 0;
    let mut i___2: libc::c_int = 0;
    let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut preSolve: Option::<
        unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
    > = None;
    let mut damping: cpFloat = 0.;
    let mut tmp___2: libc::c_double = 0.;
    let mut gravity: cpVect = cpVect { x: 0., y: 0. };
    let mut i___3: libc::c_int = 0;
    let mut body___0: *mut cpBody = 0 as *mut cpBody;
    let mut dt_coef: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut i___4: libc::c_int = 0;
    let mut i___5: libc::c_int = 0;
    let mut constraint___0: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut hasty: *mut cpHastySpace = 0 as *mut cpHastySpace;
    let mut i___6: libc::c_int = 0;
    let mut constraint___1: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut postSolve: Option::<
        unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
    > = None;
    let mut i___7: libc::c_int = 0;
    let mut arb___0: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    if dt == 0.0f32 as cpFloat {
        return;
    }
    (*space).stamp = ((*space).stamp).wrapping_add(1);
    prev_dt = (*space).curr_dt;
    (*space).curr_dt = dt;
    bodies = (*space).dynamicBodies;
    constraints = (*space).constraints;
    arbiters = (*space).arbiters;
    i = 0 as libc::c_int;
    while i < (*arbiters).num {
        arb = *((*arbiters).arr).offset(i as isize) as *mut cpArbiter;
        (*arb).state = CP_ARBITER_STATE_NORMAL;
        tmp = cpBodyIsSleeping((*arb).body_a as *const cpBody);
        if tmp == 0 {
            tmp___0 = cpBodyIsSleeping((*arb).body_b as *const cpBody);
            if tmp___0 == 0 {
                cpArbiterUnthread(arb);
            }
        }
        i += 1;
    }
    (*arbiters).num = 0 as libc::c_int;
    cpSpaceLock(space);
    i___0 = 0 as libc::c_int;
    while i___0 < (*bodies).num {
        body = *((*bodies).arr).offset(i___0 as isize) as *mut cpBody;
        (Some(((*body).position_func).expect("non-null function pointer")))
            .expect("non-null function pointer")(body, dt);
        i___0 += 1;
    }
    cpSpacePushFreshContactBuffer(space);
    cpSpatialIndexEach(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                cpShapeUpdateFunc
                    as unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> (),
            ),
        ),
        0 as *mut libc::c_void,
    );
    cpSpatialIndexReindexQuery(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpShape,
                    *mut cpShape,
                    cpCollisionID,
                    *mut cpSpace,
                ) -> cpCollisionID,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
        >(
            Some(
                cpSpaceCollideShapes
                    as unsafe extern "C" fn(
                        *mut cpShape,
                        *mut cpShape,
                        cpCollisionID,
                        *mut cpSpace,
                    ) -> cpCollisionID,
            ),
        ),
        space as *mut libc::c_void,
    );
    cpSpaceUnlock(space, 0 as libc::c_int as cpBool);
    cpSpaceProcessComponents(space, dt);
    cpSpaceLock(space);
    cpHashSetFilter(
        (*space).cachedArbiters,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace) -> cpBool>,
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> cpBool,
            >,
        >(
            Some(
                cpSpaceArbiterSetFilter
                    as unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace) -> cpBool,
            ),
        ),
        space as *mut libc::c_void,
    );
    slop = (*space).collisionSlop;
    tmp___1 = pow((*space).collisionBias, dt);
    biasCoef = 1.0f32 as libc::c_double - tmp___1;
    i___1 = 0 as libc::c_int;
    while i___1 < (*arbiters).num {
        cpArbiterPreStep(
            *((*arbiters).arr).offset(i___1 as isize) as *mut cpArbiter,
            dt,
            slop,
            biasCoef,
        );
        i___1 += 1;
    }
    i___2 = 0 as libc::c_int;
    while i___2 < (*constraints).num {
        constraint = *((*constraints).arr).offset(i___2 as isize) as *mut cpConstraint;
        preSolve = (*constraint).preSolve;
        if preSolve.is_some() {
            (Some(preSolve.expect("non-null function pointer")))
                .expect("non-null function pointer")(constraint, space);
        }
        (Some(((*(*constraint).klass).preStep).expect("non-null function pointer")))
            .expect("non-null function pointer")(constraint, dt);
        i___2 += 1;
    }
    tmp___2 = pow((*space).damping, dt);
    damping = tmp___2;
    gravity = (*space).gravity;
    i___3 = 0 as libc::c_int;
    while i___3 < (*bodies).num {
        body___0 = *((*bodies).arr).offset(i___3 as isize) as *mut cpBody;
        (Some(((*body___0).velocity_func).expect("non-null function pointer")))
            .expect("non-null function pointer")(body___0, gravity, damping, dt);
        i___3 += 1;
    }
    if prev_dt == 0.0f32 as cpFloat {
        tmp___3 = 0.0f32 as cpFloat;
    } else {
        tmp___3 = dt / prev_dt;
    }
    dt_coef = tmp___3;
    i___4 = 0 as libc::c_int;
    while i___4 < (*arbiters).num {
        cpArbiterApplyCachedImpulse(
            *((*arbiters).arr).offset(i___4 as isize) as *mut cpArbiter,
            dt_coef,
        );
        i___4 += 1;
    }
    i___5 = 0 as libc::c_int;
    while i___5 < (*constraints).num {
        constraint___0 = *((*constraints).arr).offset(i___5 as isize)
            as *mut cpConstraint;
        (Some(
            ((*(*constraint___0).klass).applyCachedImpulse)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(constraint___0, dt_coef);
        i___5 += 1;
    }
    hasty = space as *mut cpHastySpace;
    if ((*arbiters).num + (*constraints).num) as libc::c_ulong
        > (*hasty).constraint_count_threshold
    {
        RunWorkers(
            hasty,
            Some(
                Solver
                    as unsafe extern "C" fn(
                        *mut cpSpace,
                        libc::c_ulong,
                        libc::c_ulong,
                    ) -> (),
            ),
        );
    } else {
        Solver(space, 0 as libc::c_ulong, 1 as libc::c_ulong);
    }
    i___6 = 0 as libc::c_int;
    while i___6 < (*constraints).num {
        constraint___1 = *((*constraints).arr).offset(i___6 as isize)
            as *mut cpConstraint;
        postSolve = (*constraint___1).postSolve;
        if postSolve.is_some() {
            (Some(postSolve.expect("non-null function pointer")))
                .expect("non-null function pointer")(constraint___1, space);
        }
        i___6 += 1;
    }
    i___7 = 0 as libc::c_int;
    while i___7 < (*arbiters).num {
        arb___0 = *((*arbiters).arr).offset(i___7 as isize) as *mut cpArbiter;
        handler = (*arb___0).handler;
        (Some(((*handler).postSolveFunc).expect("non-null function pointer")))
            .expect("non-null function pointer")(arb___0, space, (*handler).userData);
        i___7 += 1;
    }
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
#[inline]
unsafe extern "C" fn cpflerp(
    mut f1: cpFloat,
    mut f2: cpFloat,
    mut t: cpFloat,
) -> cpFloat {
    return f1 * (1.0f32 as cpFloat - t) + f2 * t;
}
unsafe extern "C" fn cpMarchCells(
    mut bb: cpBB,
    mut x_samples: libc::c_ulong,
    mut y_samples: libc::c_ulong,
    mut t: cpFloat,
    mut segment: Option::<unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> ()>,
    mut segment_data: *mut libc::c_void,
    mut sample: Option::<unsafe extern "C" fn(cpVect, *mut libc::c_void) -> cpFloat>,
    mut sample_data: *mut libc::c_void,
    mut cell: Option::<
        unsafe extern "C" fn(
            cpFloat,
            cpFloat,
            cpFloat,
            cpFloat,
            cpFloat,
            cpFloat,
            cpFloat,
            cpFloat,
            cpFloat,
            Option::<unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> ()>,
            *mut libc::c_void,
        ) -> (),
    >,
) {
    let mut x_denom: cpFloat = 0.;
    let mut y_denom: cpFloat = 0.;
    let mut buffer: *mut cpFloat = 0 as *mut cpFloat;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_ulong = 0;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut j: libc::c_ulong = 0;
    let mut y0___0: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut y1___0: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut a: cpFloat = 0.;
    let mut b: cpFloat = 0.;
    let mut c: cpFloat = 0.;
    let mut d: cpFloat = 0.;
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpFloat = 0.;
    let mut i___0: libc::c_ulong = 0;
    let mut x0: cpFloat = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut x1: cpFloat = 0.;
    let mut tmp___7: cpFloat = 0.;
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    x_denom = 1.0f64 / x_samples.wrapping_sub(1 as libc::c_ulong) as cpFloat;
    y_denom = 1.0f64 / y_samples.wrapping_sub(1 as libc::c_ulong) as cpFloat;
    tmp = calloc(x_samples, ::std::mem::size_of::<cpFloat>() as libc::c_ulong);
    buffer = tmp as *mut cpFloat;
    i = 0 as libc::c_ulong;
    while i < x_samples {
        tmp___0 = cpflerp(bb.l, bb.r, i as cpFloat * x_denom);
        tmp___1 = cpv(tmp___0, bb.b);
        *buffer
            .offset(
                i as isize,
            ) = (Some(sample.expect("non-null function pointer")))
            .expect("non-null function pointer")(tmp___1, sample_data);
        i = i.wrapping_add(1);
    }
    j = 0 as libc::c_ulong;
    while j < y_samples.wrapping_sub(1 as libc::c_ulong) {
        tmp___2 = cpflerp(bb.b, bb.t, j as cpFloat * y_denom);
        y0___0 = tmp___2;
        tmp___3 = cpflerp(
            bb.b,
            bb.t,
            j.wrapping_add(1 as libc::c_ulong) as cpFloat * y_denom,
        );
        y1___0 = tmp___3;
        b = *buffer.offset(0 as libc::c_int as isize);
        tmp___4 = cpv(bb.l, y1___0);
        tmp___5 = (Some(sample.expect("non-null function pointer")))
            .expect("non-null function pointer")(tmp___4, sample_data);
        d = tmp___5;
        *buffer.offset(0 as libc::c_int as isize) = d;
        i___0 = 0 as libc::c_ulong;
        while i___0 < x_samples.wrapping_sub(1 as libc::c_ulong) {
            tmp___6 = cpflerp(bb.l, bb.r, i___0 as cpFloat * x_denom);
            x0 = tmp___6;
            tmp___7 = cpflerp(
                bb.l,
                bb.r,
                i___0.wrapping_add(1 as libc::c_ulong) as cpFloat * x_denom,
            );
            x1 = tmp___7;
            a = b;
            b = *buffer.offset(i___0.wrapping_add(1 as libc::c_ulong) as isize);
            c = d;
            tmp___8 = cpv(x1, y1___0);
            d = (Some(sample.expect("non-null function pointer")))
                .expect("non-null function pointer")(tmp___8, sample_data);
            *buffer.offset(i___0.wrapping_add(1 as libc::c_ulong) as isize) = d;
            (Some(cell.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(t, a, b, c, d, x0, x1, y0___0, y1___0, segment, segment_data);
            i___0 = i___0.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    free(buffer as *mut libc::c_void);
}
#[inline]
unsafe extern "C" fn seg(
    mut v0: cpVect,
    mut v1: cpVect,
    mut f: Option::<unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    let mut tmp: cpBool = 0;
    tmp = cpveql(v0, v1);
    if tmp == 0 {
        (Some(f.expect("non-null function pointer")))
            .expect("non-null function pointer")(v1, v0, data);
    }
}
#[inline]
unsafe extern "C" fn midlerp(
    mut x0: cpFloat,
    mut x1: cpFloat,
    mut s0: cpFloat,
    mut s1: cpFloat,
    mut t: cpFloat,
) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpflerp(x0, x1, (t - s0) / (s1 - s0));
    return tmp;
}
unsafe extern "C" fn cpMarchCellSoft(
    mut t: cpFloat,
    mut a: cpFloat,
    mut b: cpFloat,
    mut c: cpFloat,
    mut d: cpFloat,
    mut x0: cpFloat,
    mut x1: cpFloat,
    mut y0___0: cpFloat,
    mut y1___0: cpFloat,
    mut segment: Option::<unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> ()>,
    mut segment_data: *mut libc::c_void,
) {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpFloat = 0.;
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpFloat = 0.;
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpFloat = 0.;
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___9: cpFloat = 0.;
    let mut tmp___10: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___11: cpFloat = 0.;
    let mut tmp___12: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___13: cpFloat = 0.;
    let mut tmp___14: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___15: cpFloat = 0.;
    let mut tmp___16: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___17: cpFloat = 0.;
    let mut tmp___18: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___19: cpFloat = 0.;
    let mut tmp___20: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___21: cpFloat = 0.;
    let mut tmp___22: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___23: cpFloat = 0.;
    let mut tmp___24: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___25: cpFloat = 0.;
    let mut tmp___26: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___27: cpFloat = 0.;
    let mut tmp___28: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___29: cpFloat = 0.;
    let mut tmp___30: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___31: cpFloat = 0.;
    let mut tmp___32: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___33: cpFloat = 0.;
    let mut tmp___34: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___35: cpFloat = 0.;
    let mut tmp___36: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___37: cpFloat = 0.;
    let mut tmp___38: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___39: cpFloat = 0.;
    let mut tmp___40: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___41: cpFloat = 0.;
    let mut tmp___42: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___43: cpFloat = 0.;
    let mut tmp___44: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___45: cpFloat = 0.;
    let mut tmp___46: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___47: cpFloat = 0.;
    let mut tmp___48: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___49: cpFloat = 0.;
    let mut tmp___50: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___51: cpFloat = 0.;
    let mut tmp___52: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___53: cpFloat = 0.;
    let mut tmp___54: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___55: cpFloat = 0.;
    let mut tmp___56: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___57: cpFloat = 0.;
    let mut tmp___58: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___59: cpFloat = 0.;
    let mut tmp___60: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___61: cpFloat = 0.;
    let mut tmp___62: cpVect = cpVect { x: 0., y: 0. };
    match (a > t) as libc::c_int | ((b > t) as libc::c_int) << 1 as libc::c_int
        | ((c > t) as libc::c_int) << 2 as libc::c_int
        | ((d > t) as libc::c_int) << 3 as libc::c_int
    {
        1 => {
            tmp = midlerp(x0, x1, a, b, t);
            tmp___0 = cpv(tmp, y0___0);
            tmp___1 = midlerp(y0___0, y1___0, a, c, t);
            tmp___2 = cpv(x0, tmp___1);
            seg(tmp___2, tmp___0, segment, segment_data);
        }
        2 => {
            tmp___3 = midlerp(y0___0, y1___0, b, d, t);
            tmp___4 = cpv(x1, tmp___3);
            tmp___5 = midlerp(x0, x1, a, b, t);
            tmp___6 = cpv(tmp___5, y0___0);
            seg(tmp___6, tmp___4, segment, segment_data);
        }
        3 => {
            tmp___7 = midlerp(y0___0, y1___0, b, d, t);
            tmp___8 = cpv(x1, tmp___7);
            tmp___9 = midlerp(y0___0, y1___0, a, c, t);
            tmp___10 = cpv(x0, tmp___9);
            seg(tmp___10, tmp___8, segment, segment_data);
        }
        4 => {
            tmp___11 = midlerp(y0___0, y1___0, a, c, t);
            tmp___12 = cpv(x0, tmp___11);
            tmp___13 = midlerp(x0, x1, c, d, t);
            tmp___14 = cpv(tmp___13, y1___0);
            seg(tmp___14, tmp___12, segment, segment_data);
        }
        5 => {
            tmp___15 = midlerp(x0, x1, a, b, t);
            tmp___16 = cpv(tmp___15, y0___0);
            tmp___17 = midlerp(x0, x1, c, d, t);
            tmp___18 = cpv(tmp___17, y1___0);
            seg(tmp___18, tmp___16, segment, segment_data);
        }
        6 => {
            tmp___19 = midlerp(y0___0, y1___0, b, d, t);
            tmp___20 = cpv(x1, tmp___19);
            tmp___21 = midlerp(x0, x1, a, b, t);
            tmp___22 = cpv(tmp___21, y0___0);
            seg(tmp___22, tmp___20, segment, segment_data);
            tmp___23 = midlerp(y0___0, y1___0, a, c, t);
            tmp___24 = cpv(x0, tmp___23);
            tmp___25 = midlerp(x0, x1, c, d, t);
            tmp___26 = cpv(tmp___25, y1___0);
            seg(tmp___26, tmp___24, segment, segment_data);
        }
        7 => {
            tmp___27 = midlerp(y0___0, y1___0, b, d, t);
            tmp___28 = cpv(x1, tmp___27);
            tmp___29 = midlerp(x0, x1, c, d, t);
            tmp___30 = cpv(tmp___29, y1___0);
            seg(tmp___30, tmp___28, segment, segment_data);
        }
        8 => {
            tmp___31 = midlerp(x0, x1, c, d, t);
            tmp___32 = cpv(tmp___31, y1___0);
            tmp___33 = midlerp(y0___0, y1___0, b, d, t);
            tmp___34 = cpv(x1, tmp___33);
            seg(tmp___34, tmp___32, segment, segment_data);
        }
        9 => {
            tmp___35 = midlerp(x0, x1, a, b, t);
            tmp___36 = cpv(tmp___35, y0___0);
            tmp___37 = midlerp(y0___0, y1___0, a, c, t);
            tmp___38 = cpv(x0, tmp___37);
            seg(tmp___38, tmp___36, segment, segment_data);
            tmp___39 = midlerp(x0, x1, c, d, t);
            tmp___40 = cpv(tmp___39, y1___0);
            tmp___41 = midlerp(y0___0, y1___0, b, d, t);
            tmp___42 = cpv(x1, tmp___41);
            seg(tmp___42, tmp___40, segment, segment_data);
        }
        10 => {
            tmp___43 = midlerp(x0, x1, c, d, t);
            tmp___44 = cpv(tmp___43, y1___0);
            tmp___45 = midlerp(x0, x1, a, b, t);
            tmp___46 = cpv(tmp___45, y0___0);
            seg(tmp___46, tmp___44, segment, segment_data);
        }
        11 => {
            tmp___47 = midlerp(x0, x1, c, d, t);
            tmp___48 = cpv(tmp___47, y1___0);
            tmp___49 = midlerp(y0___0, y1___0, a, c, t);
            tmp___50 = cpv(x0, tmp___49);
            seg(tmp___50, tmp___48, segment, segment_data);
        }
        12 => {
            tmp___51 = midlerp(y0___0, y1___0, a, c, t);
            tmp___52 = cpv(x0, tmp___51);
            tmp___53 = midlerp(y0___0, y1___0, b, d, t);
            tmp___54 = cpv(x1, tmp___53);
            seg(tmp___54, tmp___52, segment, segment_data);
        }
        13 => {
            tmp___55 = midlerp(x0, x1, a, b, t);
            tmp___56 = cpv(tmp___55, y0___0);
            tmp___57 = midlerp(y0___0, y1___0, b, d, t);
            tmp___58 = cpv(x1, tmp___57);
            seg(tmp___58, tmp___56, segment, segment_data);
        }
        14 => {
            tmp___59 = midlerp(y0___0, y1___0, a, c, t);
            tmp___60 = cpv(x0, tmp___59);
            tmp___61 = midlerp(x0, x1, a, b, t);
            tmp___62 = cpv(tmp___61, y0___0);
            seg(tmp___62, tmp___60, segment, segment_data);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn cpMarchSoft(
    mut bb: cpBB,
    mut x_samples: libc::c_ulong,
    mut y_samples: libc::c_ulong,
    mut t: cpFloat,
    mut segment: Option::<unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> ()>,
    mut segment_data: *mut libc::c_void,
    mut sample: Option::<unsafe extern "C" fn(cpVect, *mut libc::c_void) -> cpFloat>,
    mut sample_data: *mut libc::c_void,
) {
    cpMarchCells(
        bb,
        x_samples,
        y_samples,
        t,
        segment,
        segment_data,
        sample,
        sample_data,
        Some(
            cpMarchCellSoft
                as unsafe extern "C" fn(
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    Option::<
                        unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> (),
                    >,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
}
#[inline]
unsafe extern "C" fn segs(
    mut a: cpVect,
    mut b: cpVect,
    mut c: cpVect,
    mut f: Option::<unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    seg(b, c, f, data);
    seg(a, b, f, data);
}
unsafe extern "C" fn cpMarchCellHard(
    mut t: cpFloat,
    mut a: cpFloat,
    mut b: cpFloat,
    mut c: cpFloat,
    mut d: cpFloat,
    mut x0: cpFloat,
    mut x1: cpFloat,
    mut y0___0: cpFloat,
    mut y1___0: cpFloat,
    mut segment: Option::<unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> ()>,
    mut segment_data: *mut libc::c_void,
) {
    let mut xm: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    let mut ym: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___9: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___10: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___11: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___12: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___13: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___14: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___15: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___16: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___17: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___18: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___19: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___20: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___21: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___22: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___23: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___24: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___25: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___26: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___27: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___28: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___29: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___30: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___31: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___32: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___33: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___34: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___35: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___36: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___37: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___38: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___39: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___40: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___41: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___42: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___43: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___44: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpflerp(x0, x1, 0.5f32 as cpFloat);
    xm = tmp;
    tmp___0 = cpflerp(y0___0, y1___0, 0.5f32 as cpFloat);
    ym = tmp___0;
    match (a > t) as libc::c_int | ((b > t) as libc::c_int) << 1 as libc::c_int
        | ((c > t) as libc::c_int) << 2 as libc::c_int
        | ((d > t) as libc::c_int) << 3 as libc::c_int
    {
        1 => {
            tmp___1 = cpv(xm, y0___0);
            tmp___2 = cpv(xm, ym);
            tmp___3 = cpv(x0, ym);
            segs(tmp___3, tmp___2, tmp___1, segment, segment_data);
        }
        2 => {
            tmp___4 = cpv(x1, ym);
            tmp___5 = cpv(xm, ym);
            tmp___6 = cpv(xm, y0___0);
            segs(tmp___6, tmp___5, tmp___4, segment, segment_data);
        }
        3 => {
            tmp___7 = cpv(x1, ym);
            tmp___8 = cpv(x0, ym);
            seg(tmp___8, tmp___7, segment, segment_data);
        }
        4 => {
            tmp___9 = cpv(x0, ym);
            tmp___10 = cpv(xm, ym);
            tmp___11 = cpv(xm, y1___0);
            segs(tmp___11, tmp___10, tmp___9, segment, segment_data);
        }
        5 => {
            tmp___12 = cpv(xm, y0___0);
            tmp___13 = cpv(xm, y1___0);
            seg(tmp___13, tmp___12, segment, segment_data);
        }
        6 => {
            tmp___14 = cpv(x0, ym);
            tmp___15 = cpv(xm, ym);
            tmp___16 = cpv(xm, y0___0);
            segs(tmp___16, tmp___15, tmp___14, segment, segment_data);
            tmp___17 = cpv(x1, ym);
            tmp___18 = cpv(xm, ym);
            tmp___19 = cpv(xm, y1___0);
            segs(tmp___19, tmp___18, tmp___17, segment, segment_data);
        }
        7 => {
            tmp___20 = cpv(x1, ym);
            tmp___21 = cpv(xm, ym);
            tmp___22 = cpv(xm, y1___0);
            segs(tmp___22, tmp___21, tmp___20, segment, segment_data);
        }
        8 => {
            tmp___23 = cpv(xm, y1___0);
            tmp___24 = cpv(xm, ym);
            tmp___25 = cpv(x1, ym);
            segs(tmp___25, tmp___24, tmp___23, segment, segment_data);
        }
        9 => {
            tmp___26 = cpv(xm, y0___0);
            tmp___27 = cpv(xm, ym);
            tmp___28 = cpv(x1, ym);
            segs(tmp___28, tmp___27, tmp___26, segment, segment_data);
            tmp___29 = cpv(xm, y1___0);
            tmp___30 = cpv(xm, ym);
            tmp___31 = cpv(x0, ym);
            segs(tmp___31, tmp___30, tmp___29, segment, segment_data);
        }
        10 => {
            tmp___32 = cpv(xm, y1___0);
            tmp___33 = cpv(xm, y0___0);
            seg(tmp___33, tmp___32, segment, segment_data);
        }
        11 => {
            tmp___34 = cpv(xm, y1___0);
            tmp___35 = cpv(xm, ym);
            tmp___36 = cpv(x0, ym);
            segs(tmp___36, tmp___35, tmp___34, segment, segment_data);
        }
        12 => {
            tmp___37 = cpv(x0, ym);
            tmp___38 = cpv(x1, ym);
            seg(tmp___38, tmp___37, segment, segment_data);
        }
        13 => {
            tmp___39 = cpv(xm, y0___0);
            tmp___40 = cpv(xm, ym);
            tmp___41 = cpv(x1, ym);
            segs(tmp___41, tmp___40, tmp___39, segment, segment_data);
        }
        14 => {
            tmp___42 = cpv(x0, ym);
            tmp___43 = cpv(xm, ym);
            tmp___44 = cpv(xm, y0___0);
            segs(tmp___44, tmp___43, tmp___42, segment, segment_data);
        }
        _ => {}
    };
}
pub unsafe extern "C" fn cpMarchHard(
    mut bb: cpBB,
    mut x_samples: libc::c_ulong,
    mut y_samples: libc::c_ulong,
    mut t: cpFloat,
    mut segment: Option::<unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> ()>,
    mut segment_data: *mut libc::c_void,
    mut sample: Option::<unsafe extern "C" fn(cpVect, *mut libc::c_void) -> cpFloat>,
    mut sample_data: *mut libc::c_void,
) {
    cpMarchCells(
        bb,
        x_samples,
        y_samples,
        t,
        segment,
        segment_data,
        sample,
        sample_data,
        Some(
            cpMarchCellHard
                as unsafe extern "C" fn(
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    cpFloat,
                    Option::<
                        unsafe extern "C" fn(cpVect, cpVect, *mut libc::c_void) -> (),
                    >,
                    *mut libc::c_void,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn preStep___3(mut joint: *mut cpPinJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut dist: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut tmp___5: libc::c_float = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut tmp___7: cpFloat = 0.;
    let mut maxBias: cpFloat = 0.;
    let mut tmp___8: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    tmp = cpvsub((*joint).anchorA, (*a).cog);
    (*joint).r1 = cpTransformVect((*a).transform, tmp);
    tmp___0 = cpvsub((*joint).anchorB, (*b).cog);
    (*joint).r2 = cpTransformVect((*b).transform, tmp___0);
    tmp___1 = cpvadd((*a).p, (*joint).r1);
    tmp___2 = cpvadd((*b).p, (*joint).r2);
    tmp___3 = cpvsub(tmp___2, tmp___1);
    delta = tmp___3;
    tmp___4 = cpvlength(delta);
    dist = tmp___4;
    if dist != 0. {
        tmp___6 = dist;
    } else {
        tmp___5 = ::std::f32::INFINITY;
        tmp___6 = tmp___5 as cpFloat;
    }
    (*joint).n = cpvmult(delta, 1.0f32 as cpFloat / tmp___6);
    tmp___7 = k_scalar(a, b, (*joint).r1, (*joint).r2, (*joint).n);
    (*joint).nMass = 1.0f32 as cpFloat / tmp___7;
    maxBias = (*joint).constraint.maxBias;
    tmp___8 = bias_coef((*joint).constraint.errorBias, dt);
    (*joint).bias = cpfclamp(-tmp___8 * (dist - (*joint).dist) / dt, -maxBias, maxBias);
}
unsafe extern "C" fn applyCachedImpulse___3(
    mut joint: *mut cpPinJoint,
    mut dt_coef: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut j: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    tmp = cpvmult((*joint).n, (*joint).jnAcc * dt_coef);
    j = tmp;
    apply_impulses(a, b, (*joint).r1, (*joint).r2, j);
}
unsafe extern "C" fn applyImpulse___3(mut joint: *mut cpPinJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut vrn: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    let mut jnMax: cpFloat = 0.;
    let mut jn___0: cpFloat = 0.;
    let mut jnOld: cpFloat = 0.;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    n = (*joint).n;
    tmp = normal_relative_velocity(a, b, (*joint).r1, (*joint).r2, n);
    vrn = tmp;
    jnMax = (*joint).constraint.maxForce * dt;
    jn___0 = ((*joint).bias - vrn) * (*joint).nMass;
    jnOld = (*joint).jnAcc;
    (*joint).jnAcc = cpfclamp(jnOld + jn___0, -jnMax, jnMax);
    jn___0 = (*joint).jnAcc - jnOld;
    tmp___0 = cpvmult(n, jn___0);
    apply_impulses(a, b, (*joint).r1, (*joint).r2, tmp___0);
}
unsafe extern "C" fn getImpulse___3(mut joint: *mut cpPinJoint) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpfabs((*joint).jnAcc);
    return tmp;
}
static mut klass___4: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPinJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(Some(preStep___3 as unsafe extern "C" fn(*mut cpPinJoint, cpFloat) -> ())),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPinJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyCachedImpulse___3
                        as unsafe extern "C" fn(*mut cpPinJoint, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPinJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyImpulse___3
                        as unsafe extern "C" fn(*mut cpPinJoint, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPinJoint) -> cpFloat>,
                Option::<unsafe extern "C" fn(*mut cpConstraint) -> cpFloat>,
            >(Some(getImpulse___3 as unsafe extern "C" fn(*mut cpPinJoint) -> cpFloat)),
        };
        init
    }
};
pub unsafe extern "C" fn cpPinJointAlloc() -> *mut cpPinJoint {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpPinJoint>() as libc::c_ulong,
    );
    return tmp as *mut cpPinJoint;
}
pub unsafe extern "C" fn cpPinJointInit(
    mut joint: *mut cpPinJoint,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
) -> *mut cpPinJoint {
    let mut p1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut p2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    cpConstraintInit(joint as *mut cpConstraint, &klass___4, a, b);
    (*joint).anchorA = anchorA;
    (*joint).anchorB = anchorB;
    if !a.is_null() {
        tmp = cpTransformPoint((*a).transform, anchorA);
        tmp___0 = tmp;
    } else {
        tmp___0 = anchorA;
    }
    p1 = tmp___0;
    if !b.is_null() {
        tmp___1 = cpTransformPoint((*b).transform, anchorB);
        tmp___2 = tmp___1;
    } else {
        tmp___2 = anchorB;
    }
    p2 = tmp___2;
    tmp___3 = cpvsub(p2, p1);
    (*joint).dist = cpvlength(tmp___3);
    (*joint).jnAcc = 0.0f32 as cpFloat;
    return joint;
}
pub unsafe extern "C" fn cpPinJointNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
) -> *mut cpConstraint {
    let mut tmp: *mut cpPinJoint = 0 as *mut cpPinJoint;
    let mut tmp___0: *mut cpPinJoint = 0 as *mut cpPinJoint;
    tmp = cpPinJointAlloc();
    tmp___0 = cpPinJointInit(tmp, a, b, anchorA, anchorB);
    return tmp___0 as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsPinJoint(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass as libc::c_ulong
        == &klass___4 as *const cpConstraintClass as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpPinJointGetAnchorA(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsPinJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsPinJoint(constraint)\0" as *const u8 as *const libc::c_char,
            b"../src/cpPinJoint.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pin joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpPinJoint)).anchorA;
}
pub unsafe extern "C" fn cpPinJointSetAnchorA(
    mut constraint: *mut cpConstraint,
    mut anchorA: cpVect,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsPinJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsPinJoint(constraint)\0" as *const u8 as *const libc::c_char,
            b"../src/cpPinJoint.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pin joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpPinJoint)).anchorA = anchorA;
}
pub unsafe extern "C" fn cpPinJointGetAnchorB(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsPinJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsPinJoint(constraint)\0" as *const u8 as *const libc::c_char,
            b"../src/cpPinJoint.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pin joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpPinJoint)).anchorB;
}
pub unsafe extern "C" fn cpPinJointSetAnchorB(
    mut constraint: *mut cpConstraint,
    mut anchorB: cpVect,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsPinJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsPinJoint(constraint)\0" as *const u8 as *const libc::c_char,
            b"../src/cpPinJoint.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pin joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpPinJoint)).anchorB = anchorB;
}
pub unsafe extern "C" fn cpPinJointGetDist(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsPinJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsPinJoint(constraint)\0" as *const u8 as *const libc::c_char,
            b"../src/cpPinJoint.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pin joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpPinJoint)).dist;
}
pub unsafe extern "C" fn cpPinJointSetDist(
    mut constraint: *mut cpConstraint,
    mut dist: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsPinJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsPinJoint(constraint)\0" as *const u8 as *const libc::c_char,
            b"../src/cpPinJoint.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pin joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpPinJoint)).dist = dist;
}
static mut cpvzero___4: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
unsafe extern "C" fn preStep___4(mut joint: *mut cpPivotJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpFloat = 0.;
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    tmp = cpvsub((*joint).anchorA, (*a).cog);
    (*joint).r1 = cpTransformVect((*a).transform, tmp);
    tmp___0 = cpvsub((*joint).anchorB, (*b).cog);
    (*joint).r2 = cpTransformVect((*b).transform, tmp___0);
    (*joint).k = k_tensor(a, b, (*joint).r1, (*joint).r2);
    tmp___1 = cpvadd((*a).p, (*joint).r1);
    tmp___2 = cpvadd((*b).p, (*joint).r2);
    tmp___3 = cpvsub(tmp___2, tmp___1);
    delta = tmp___3;
    tmp___4 = bias_coef((*joint).constraint.errorBias, dt);
    tmp___5 = cpvmult(delta, -tmp___4 / dt);
    (*joint).bias = cpvclamp(tmp___5, (*joint).constraint.maxBias);
}
unsafe extern "C" fn applyCachedImpulse___4(
    mut joint: *mut cpPivotJoint,
    mut dt_coef: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    tmp = cpvmult((*joint).jAcc, dt_coef);
    apply_impulses(a, b, (*joint).r1, (*joint).r2, tmp);
}
unsafe extern "C" fn applyImpulse___4(mut joint: *mut cpPivotJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut r1: cpVect = cpVect { x: 0., y: 0. };
    let mut r2: cpVect = cpVect { x: 0., y: 0. };
    let mut vr: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut j: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut jOld: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    r1 = (*joint).r1;
    r2 = (*joint).r2;
    tmp = relative_velocity(a, b, r1, r2);
    vr = tmp;
    tmp___0 = cpvsub((*joint).bias, vr);
    tmp___1 = cpMat2x2Transform((*joint).k, tmp___0);
    j = tmp___1;
    jOld = (*joint).jAcc;
    tmp___2 = cpvadd((*joint).jAcc, j);
    (*joint).jAcc = cpvclamp(tmp___2, (*joint).constraint.maxForce * dt);
    j = cpvsub((*joint).jAcc, jOld);
    apply_impulses(a, b, (*joint).r1, (*joint).r2, j);
}
unsafe extern "C" fn getImpulse___4(mut joint: *mut cpConstraint) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpvlength((*(joint as *mut cpPivotJoint)).jAcc);
    return tmp;
}
static mut klass___5: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    preStep___4 as unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> (),
                ),
            ),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyCachedImpulse___4
                        as unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyImpulse___4
                        as unsafe extern "C" fn(*mut cpPivotJoint, cpFloat) -> (),
                ),
            ),
            getImpulse: Some(
                getImpulse___4 as unsafe extern "C" fn(*mut cpConstraint) -> cpFloat,
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpPivotJointAlloc() -> *mut cpPivotJoint {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpPivotJoint>() as libc::c_ulong,
    );
    return tmp as *mut cpPivotJoint;
}
pub unsafe extern "C" fn cpPivotJointInit(
    mut joint: *mut cpPivotJoint,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
) -> *mut cpPivotJoint {
    cpConstraintInit(joint as *mut cpConstraint, &klass___5, a, b);
    (*joint).anchorA = anchorA;
    (*joint).anchorB = anchorB;
    (*joint).jAcc = cpvzero___4;
    return joint;
}
pub unsafe extern "C" fn cpPivotJointNew2(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
) -> *mut cpConstraint {
    let mut tmp: *mut cpPivotJoint = 0 as *mut cpPivotJoint;
    let mut tmp___0: *mut cpPivotJoint = 0 as *mut cpPivotJoint;
    tmp = cpPivotJointAlloc();
    tmp___0 = cpPivotJointInit(tmp, a, b, anchorA, anchorB);
    return tmp___0 as *mut cpConstraint;
}
pub unsafe extern "C" fn cpPivotJointNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut pivot: cpVect,
) -> *mut cpConstraint {
    let mut anchorA: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut anchorB: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: *mut cpConstraint = 0 as *mut cpConstraint;
    if !a.is_null() {
        tmp = cpBodyWorldToLocal(a as *const cpBody, pivot);
        tmp___0 = tmp;
    } else {
        tmp___0 = pivot;
    }
    anchorA = tmp___0;
    if !b.is_null() {
        tmp___1 = cpBodyWorldToLocal(b as *const cpBody, pivot);
        tmp___2 = tmp___1;
    } else {
        tmp___2 = pivot;
    }
    anchorB = tmp___2;
    tmp___3 = cpPivotJointNew2(a, b, anchorA, anchorB);
    return tmp___3;
}
pub unsafe extern "C" fn cpConstraintIsPivotJoint(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass as libc::c_ulong
        == &klass___5 as *const cpConstraintClass as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpPivotJointGetAnchorA(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsPivotJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsPivotJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpPivotJoint.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pivot joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpPivotJoint)).anchorA;
}
pub unsafe extern "C" fn cpPivotJointSetAnchorA(
    mut constraint: *mut cpConstraint,
    mut anchorA: cpVect,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsPivotJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsPivotJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpPivotJoint.c\0" as *const u8 as *const libc::c_char,
            134 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pivot joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpPivotJoint)).anchorA = anchorA;
}
pub unsafe extern "C" fn cpPivotJointGetAnchorB(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsPivotJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsPivotJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpPivotJoint.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pivot joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpPivotJoint)).anchorB;
}
pub unsafe extern "C" fn cpPivotJointSetAnchorB(
    mut constraint: *mut cpConstraint,
    mut anchorB: cpVect,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsPivotJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsPivotJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpPivotJoint.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a pivot joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpPivotJoint)).anchorB = anchorB;
}
static mut cpvzero___5: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpClosetPointOnSegment(p: cpVect, a: cpVect, b: cpVect) -> cpVect {
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut t: cpFloat = 0.;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvsub(a, b);
    delta = tmp;
    tmp___0 = cpvsub(p, b);
    tmp___1 = cpvdot(delta, tmp___0);
    tmp___2 = cpvlengthsq(delta);
    tmp___3 = cpfclamp01(tmp___1 / tmp___2);
    t = tmp___3;
    tmp___4 = cpvmult(delta, t);
    tmp___5 = cpvadd(b, tmp___4);
    return tmp___5;
}
#[inline]
unsafe extern "C" fn CircleSegmentQuery(
    mut shape: *mut cpShape,
    mut center: cpVect,
    mut r1: cpFloat,
    mut a: cpVect,
    mut b: cpVect,
    mut r2: cpFloat,
    mut info: *mut cpSegmentQueryInfo,
) {
    let mut da: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut db: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut rsum: cpFloat = 0.;
    let mut qa: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut qb: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut tmp___5: cpFloat = 0.;
    let mut det: cpFloat = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut t: cpFloat = 0.;
    let mut tmp___7: libc::c_double = 0.;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___9: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___10: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___11: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvsub(a, center);
    da = tmp;
    tmp___0 = cpvsub(b, center);
    db = tmp___0;
    rsum = r1 + r2;
    tmp___1 = cpvdot(da, da);
    tmp___2 = cpvdot(da, db);
    tmp___3 = cpvdot(db, db);
    qa = tmp___1 - 2.0f32 as cpFloat * tmp___2 + tmp___3;
    tmp___4 = cpvdot(da, db);
    tmp___5 = cpvdot(da, da);
    qb = tmp___4 - tmp___5;
    tmp___6 = cpvdot(da, da);
    det = qb * qb - qa * (tmp___6 - rsum * rsum);
    if det >= 0.0f32 as cpFloat {
        tmp___7 = sqrt(det);
        t = (-qb - tmp___7) / qa;
        if 0.0f32 as cpFloat <= t {
            if t <= 1.0f32 as cpFloat {
                tmp___8 = cpvlerp(da, db, t);
                tmp___9 = cpvnormalize(tmp___8);
                n = tmp___9;
                (*info).shape = shape as *const cpShape;
                tmp___10 = cpvmult(n, r2);
                tmp___11 = cpvlerp(a, b, t);
                (*info).point = cpvsub(tmp___11, tmp___10);
                (*info).normal = n;
                (*info).alpha = t;
            }
        }
    }
}
pub unsafe extern "C" fn cpPolyShapeAlloc() -> *mut cpPolyShape {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpPolyShape>() as libc::c_ulong,
    );
    return tmp as *mut cpPolyShape;
}
unsafe extern "C" fn cpPolyShapeDestroy(mut poly: *mut cpPolyShape) {
    if (*poly).count > 6 as libc::c_int {
        free((*poly).planes as *mut libc::c_void);
    }
}
unsafe extern "C" fn cpPolyShapeCacheData(
    mut poly: *mut cpPolyShape,
    mut transform: cpTransform,
) -> cpBB {
    let mut count: libc::c_int = 0;
    let mut dst: *mut cpSplittingPlane = 0 as *mut cpSplittingPlane;
    let mut src: *mut cpSplittingPlane = 0 as *mut cpSplittingPlane;
    let mut l: cpFloat = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut r: cpFloat = 0.;
    let mut tmp___0: libc::c_float = 0.;
    let mut b: cpFloat = 0.;
    let mut tmp___1: libc::c_float = 0.;
    let mut t: cpFloat = 0.;
    let mut tmp___2: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut v: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut radius: cpFloat = 0.;
    let mut tmp___5: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    count = (*poly).count;
    dst = (*poly).planes;
    src = dst.offset(count as isize);
    tmp = ::std::f32::INFINITY;
    l = tmp as cpFloat;
    tmp___0 = ::std::f32::INFINITY;
    r = -(tmp___0 as cpFloat);
    tmp___1 = ::std::f32::INFINITY;
    b = tmp___1 as cpFloat;
    tmp___2 = ::std::f32::INFINITY;
    t = -(tmp___2 as cpFloat);
    i = 0 as libc::c_int;
    while i < count {
        tmp___3 = cpTransformPoint(transform, (*src.offset(i as isize)).v0);
        v = tmp___3;
        tmp___4 = cpTransformVect(transform, (*src.offset(i as isize)).n);
        n = tmp___4;
        (*dst.offset(i as isize)).v0 = v;
        (*dst.offset(i as isize)).n = n;
        l = cpfmin(l, v.x);
        r = cpfmax(r, v.x);
        b = cpfmin(b, v.y);
        t = cpfmax(t, v.y);
        i += 1;
    }
    radius = (*poly).r;
    tmp___5 = cpBBNew(l - radius, b - radius, r + radius, t + radius);
    (*poly).shape.bb = tmp___5;
    return tmp___5;
}
unsafe extern "C" fn cpPolyShapePointQuery(
    mut poly: *mut cpPolyShape,
    mut p: cpVect,
    mut info: *mut cpPointQueryInfo,
) {
    let mut count: libc::c_int = 0;
    let mut planes: *mut cpSplittingPlane = 0 as *mut cpSplittingPlane;
    let mut r: cpFloat = 0.;
    let mut v0: cpVect = cpVect { x: 0., y: 0. };
    let mut minDist: cpFloat = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut closestPoint: cpVect = cpVect { x: 0., y: 0. };
    let mut closestNormal: cpVect = cpVect { x: 0., y: 0. };
    let mut outside: cpBool = 0;
    let mut i: libc::c_int = 0;
    let mut v1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: libc::c_int = 0;
    let mut closest: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut dist: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut dist___0: cpFloat = 0.;
    let mut tmp___5: cpFloat = 0.;
    let mut g: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    count = (*poly).count;
    planes = (*poly).planes;
    r = (*poly).r;
    v0 = (*planes.offset((count - 1 as libc::c_int) as isize)).v0;
    tmp = ::std::f32::INFINITY;
    minDist = tmp as cpFloat;
    closestPoint = cpvzero___5;
    closestNormal = cpvzero___5;
    outside = 0 as libc::c_int as cpBool;
    i = 0 as libc::c_int;
    while i < count {
        v1 = (*planes.offset(i as isize)).v0;
        if outside != 0 {
            tmp___2 = 1 as libc::c_int;
        } else {
            tmp___0 = cpvsub(p, v1);
            tmp___1 = cpvdot((*planes.offset(i as isize)).n, tmp___0);
            if tmp___1 > 0.0f32 as cpFloat {
                tmp___2 = 1 as libc::c_int;
            } else {
                tmp___2 = 0 as libc::c_int;
            }
        }
        outside = tmp___2 as cpBool;
        tmp___3 = cpClosetPointOnSegment(p, v0, v1);
        closest = tmp___3;
        tmp___4 = cpvdist(p, closest);
        dist = tmp___4;
        if dist < minDist {
            minDist = dist;
            closestPoint = closest;
            closestNormal = (*planes.offset(i as isize)).n;
        }
        v0 = v1;
        i += 1;
    }
    if outside != 0 {
        tmp___5 = minDist;
    } else {
        tmp___5 = -minDist;
    }
    dist___0 = tmp___5;
    tmp___6 = cpvsub(p, closestPoint);
    tmp___7 = cpvmult(tmp___6, 1.0f32 as cpFloat / dist___0);
    g = tmp___7;
    (*info).shape = poly as *mut cpShape as *const cpShape;
    tmp___8 = cpvmult(g, r);
    (*info).point = cpvadd(closestPoint, tmp___8);
    (*info).distance = dist___0 - r;
    if minDist > 1e-5f64 {
        (*info).gradient = g;
    } else {
        (*info).gradient = closestNormal;
    };
}
unsafe extern "C" fn cpPolyShapeSegmentQuery(
    mut poly: *mut cpPolyShape,
    mut a: cpVect,
    mut b: cpVect,
    mut r2: cpFloat,
    mut info: *mut cpSegmentQueryInfo,
) {
    let mut planes: *mut cpSplittingPlane = 0 as *mut cpSplittingPlane;
    let mut count: libc::c_int = 0;
    let mut r: cpFloat = 0.;
    let mut rsum: cpFloat = 0.;
    let mut i: libc::c_int = 0;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut an: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    let mut d: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut bn: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut t: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut point: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut dt: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut dtMin: cpFloat = 0.;
    let mut tmp___5: cpFloat = 0.;
    let mut dtMax: cpFloat = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut tmp___7: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    let mut i___0: libc::c_int = 0;
    let mut circle_info: cpSegmentQueryInfo = cpSegmentQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        normal: cpVect { x: 0., y: 0. },
        alpha: 0.,
    };
    planes = (*poly).planes;
    count = (*poly).count;
    r = (*poly).r;
    rsum = r + r2;
    i = 0 as libc::c_int;
    while i < count {
        n = (*planes.offset(i as isize)).n;
        tmp = cpvdot(a, n);
        an = tmp;
        tmp___0 = cpvdot((*planes.offset(i as isize)).v0, n);
        d = an - tmp___0 - rsum;
        if !(d < 0.0f32 as cpFloat) {
            tmp___1 = cpvdot(b, n);
            bn = tmp___1;
            tmp___2 = cpfmax(an - bn, 2.2250738585072014e-308f64);
            t = d / tmp___2;
            if !(t < 0.0f32 as cpFloat) {
                if !((1.0f32 as cpFloat) < t) {
                    tmp___3 = cpvlerp(a, b, t);
                    point = tmp___3;
                    tmp___4 = cpvcross(n, point);
                    dt = tmp___4;
                    tmp___5 = cpvcross(
                        n,
                        (*planes
                            .offset(((i - 1 as libc::c_int + count) % count) as isize))
                            .v0,
                    );
                    dtMin = tmp___5;
                    tmp___6 = cpvcross(n, (*planes.offset(i as isize)).v0);
                    dtMax = tmp___6;
                    if dtMin <= dt {
                        if dt <= dtMax {
                            (*info).shape = poly as *mut cpShape as *const cpShape;
                            tmp___7 = cpvmult(n, r2);
                            tmp___8 = cpvlerp(a, b, t);
                            (*info).point = cpvsub(tmp___8, tmp___7);
                            (*info).normal = n;
                            (*info).alpha = t;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    if rsum > 0.0f32 as cpFloat {
        i___0 = 0 as libc::c_int;
        while i___0 < count {
            circle_info.shape = 0 as *mut libc::c_void as *const cpShape;
            circle_info.point = b;
            circle_info.normal = cpvzero___5;
            circle_info.alpha = 1.0f32 as cpFloat;
            CircleSegmentQuery(
                &mut (*poly).shape,
                (*planes.offset(i___0 as isize)).v0,
                r,
                a,
                b,
                r2,
                &mut circle_info,
            );
            if circle_info.alpha < (*info).alpha {
                *info = circle_info;
            }
            i___0 += 1;
        }
    }
}
unsafe extern "C" fn SetVerts(
    mut poly: *mut cpPolyShape,
    mut count: libc::c_int,
    mut verts: *const cpVect,
) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut a: cpVect = cpVect { x: 0., y: 0. };
    let mut b: cpVect = cpVect { x: 0., y: 0. };
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    (*poly).count = count;
    if count <= 6 as libc::c_int {
        (*poly).planes = ((*poly)._planes).as_mut_ptr();
    } else {
        tmp = calloc(
            (2 as libc::c_int * count) as size_t,
            ::std::mem::size_of::<cpSplittingPlane>() as libc::c_ulong,
        );
        (*poly).planes = tmp as *mut cpSplittingPlane;
    }
    i = 0 as libc::c_int;
    while i < count {
        a = *verts.offset(((i - 1 as libc::c_int + count) % count) as isize);
        b = *verts.offset(i as isize);
        tmp___0 = cpvsub(b, a);
        tmp___1 = cpvrperp(tmp___0);
        tmp___2 = cpvnormalize(tmp___1);
        n = tmp___2;
        (*((*poly).planes).offset((i + count) as isize)).v0 = b;
        (*((*poly).planes).offset((i + count) as isize)).n = n;
        i += 1;
    }
}
unsafe extern "C" fn cpPolyShapeMassInfo(
    mut mass: cpFloat,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut radius: cpFloat,
) -> cpShapeMassInfo {
    let mut centroid: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut info: cpShapeMassInfo = cpShapeMassInfo {
        m: 0.,
        i: 0.,
        cog: cpVect { x: 0., y: 0. },
        area: 0.,
    };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    tmp = cpCentroidForPoly(count, verts);
    centroid = tmp;
    tmp___0 = cpvneg(centroid);
    tmp___1 = cpMomentForPoly(1.0f32 as cpFloat, count, verts, tmp___0, radius);
    tmp___2 = cpAreaForPoly(count, verts, radius);
    info.m = mass;
    info.i = tmp___1;
    info.cog = centroid;
    info.area = tmp___2;
    return info;
}
static mut polyClass: cpShapeClass = unsafe {
    {
        let mut init = cpShapeClass {
            type_0: CP_POLY_SHAPE,
            cacheData: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPolyShape, cpTransform) -> cpBB>,
                Option::<unsafe extern "C" fn(*mut cpShape, cpTransform) -> cpBB>,
            >(
                Some(
                    cpPolyShapeCacheData
                        as unsafe extern "C" fn(*mut cpPolyShape, cpTransform) -> cpBB,
                ),
            ),
            destroy: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpPolyShape) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpShape) -> ()>,
            >(Some(cpPolyShapeDestroy as unsafe extern "C" fn(*mut cpPolyShape) -> ())),
            pointQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpPolyShape,
                        cpVect,
                        *mut cpPointQueryInfo,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const cpShape,
                        cpVect,
                        *mut cpPointQueryInfo,
                    ) -> (),
                >,
            >(
                Some(
                    cpPolyShapePointQuery
                        as unsafe extern "C" fn(
                            *mut cpPolyShape,
                            cpVect,
                            *mut cpPointQueryInfo,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpPolyShape,
                        cpVect,
                        cpVect,
                        cpFloat,
                        *mut cpSegmentQueryInfo,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const cpShape,
                        cpVect,
                        cpVect,
                        cpFloat,
                        *mut cpSegmentQueryInfo,
                    ) -> (),
                >,
            >(
                Some(
                    cpPolyShapeSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpPolyShape,
                            cpVect,
                            cpVect,
                            cpFloat,
                            *mut cpSegmentQueryInfo,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpPolyShapeInit(
    mut poly: *mut cpPolyShape,
    mut body: *mut cpBody,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut transform: cpTransform,
    mut radius: cpFloat,
) -> *mut cpPolyShape {
    let mut hullVerts: *mut cpVect = 0 as *mut cpVect;
    let mut tmp: *mut _ = 0 as *mut _;
    let mut i: libc::c_int = 0;
    let mut hullCount: libc::c_uint = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut cpPolyShape = 0 as *mut cpPolyShape;
    let mut fresh13 = ::std::vec::from_elem(
        0,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
    );
    tmp = fresh13.as_mut_ptr();
    hullVerts = tmp as *mut cpVect;
    i = 0 as libc::c_int;
    while i < count {
        *hullVerts
            .offset(i as isize) = cpTransformPoint(transform, *verts.offset(i as isize));
        i += 1;
    }
    tmp___0 = cpConvexHull(
        count,
        hullVerts as *const cpVect,
        hullVerts,
        0 as *mut libc::c_void as *mut libc::c_int,
        0.0f64,
    );
    hullCount = tmp___0 as libc::c_uint;
    tmp___1 = cpPolyShapeInitRaw(
        poly,
        body,
        hullCount as libc::c_int,
        hullVerts as *const cpVect,
        radius,
    );
    return tmp___1;
}
pub unsafe extern "C" fn cpPolyShapeInitRaw(
    mut poly: *mut cpPolyShape,
    mut body: *mut cpBody,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut radius: cpFloat,
) -> *mut cpPolyShape {
    let mut tmp: cpShapeMassInfo = cpShapeMassInfo {
        m: 0.,
        i: 0.,
        cog: cpVect { x: 0., y: 0. },
        area: 0.,
    };
    tmp = cpPolyShapeMassInfo(0.0f32 as cpFloat, count, verts, radius);
    cpShapeInit(poly as *mut cpShape, &polyClass, body, tmp);
    SetVerts(poly, count, verts);
    (*poly).r = radius;
    return poly;
}
pub unsafe extern "C" fn cpPolyShapeNew(
    mut body: *mut cpBody,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut transform: cpTransform,
    mut radius: cpFloat,
) -> *mut cpShape {
    let mut tmp: *mut cpPolyShape = 0 as *mut cpPolyShape;
    let mut tmp___0: *mut cpPolyShape = 0 as *mut cpPolyShape;
    tmp = cpPolyShapeAlloc();
    tmp___0 = cpPolyShapeInit(tmp, body, count, verts, transform, radius);
    return tmp___0 as *mut cpShape;
}
pub unsafe extern "C" fn cpPolyShapeNewRaw(
    mut body: *mut cpBody,
    mut count: libc::c_int,
    mut verts: *const cpVect,
    mut radius: cpFloat,
) -> *mut cpShape {
    let mut tmp: *mut cpPolyShape = 0 as *mut cpPolyShape;
    let mut tmp___0: *mut cpPolyShape = 0 as *mut cpPolyShape;
    tmp = cpPolyShapeAlloc();
    tmp___0 = cpPolyShapeInitRaw(tmp, body, count, verts, radius);
    return tmp___0 as *mut cpShape;
}
pub unsafe extern "C" fn cpBoxShapeInit(
    mut poly: *mut cpPolyShape,
    mut body: *mut cpBody,
    mut width: cpFloat,
    mut height: cpFloat,
    mut radius: cpFloat,
) -> *mut cpPolyShape {
    let mut hw: cpFloat = 0.;
    let mut hh: cpFloat = 0.;
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp___0: *mut cpPolyShape = 0 as *mut cpPolyShape;
    hw = width / 2.0f32 as cpFloat;
    hh = height / 2.0f32 as cpFloat;
    tmp = cpBBNew(-hw, -hh, hw, hh);
    tmp___0 = cpBoxShapeInit2(poly, body, tmp, radius);
    return tmp___0;
}
pub unsafe extern "C" fn cpBoxShapeInit2(
    mut poly: *mut cpPolyShape,
    mut body: *mut cpBody,
    mut box_0: cpBB,
    mut radius: cpFloat,
) -> *mut cpPolyShape {
    let mut verts: [cpVect; 4] = [cpVect { x: 0., y: 0. }; 4];
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: *mut cpPolyShape = 0 as *mut cpPolyShape;
    tmp = cpv(box_0.r, box_0.b);
    tmp___0 = cpv(box_0.r, box_0.t);
    tmp___1 = cpv(box_0.l, box_0.t);
    tmp___2 = cpv(box_0.l, box_0.b);
    verts[0 as libc::c_int as usize] = tmp;
    verts[1 as libc::c_int as usize] = tmp___0;
    verts[2 as libc::c_int as usize] = tmp___1;
    verts[3 as libc::c_int as usize] = tmp___2;
    tmp___3 = cpPolyShapeInitRaw(
        poly,
        body,
        4 as libc::c_int,
        verts.as_mut_ptr() as *const cpVect,
        radius,
    );
    return tmp___3;
}
pub unsafe extern "C" fn cpBoxShapeNew(
    mut body: *mut cpBody,
    mut width: cpFloat,
    mut height: cpFloat,
    mut radius: cpFloat,
) -> *mut cpShape {
    let mut tmp: *mut cpPolyShape = 0 as *mut cpPolyShape;
    let mut tmp___0: *mut cpPolyShape = 0 as *mut cpPolyShape;
    tmp = cpPolyShapeAlloc();
    tmp___0 = cpBoxShapeInit(tmp, body, width, height, radius);
    return tmp___0 as *mut cpShape;
}
pub unsafe extern "C" fn cpBoxShapeNew2(
    mut body: *mut cpBody,
    mut box_0: cpBB,
    mut radius: cpFloat,
) -> *mut cpShape {
    let mut tmp: *mut cpPolyShape = 0 as *mut cpPolyShape;
    let mut tmp___0: *mut cpPolyShape = 0 as *mut cpPolyShape;
    tmp = cpPolyShapeAlloc();
    tmp___0 = cpBoxShapeInit2(tmp, body, box_0, radius);
    return tmp___0 as *mut cpShape;
}
pub unsafe extern "C" fn cpPolyShapeGetCount(mut shape: *const cpShape) -> libc::c_int {
    if !((*shape).klass as libc::c_ulong
        == &polyClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &polyClass\0" as *const u8 as *const libc::c_char,
            b"../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            262 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a poly shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpPolyShape)).count;
}
pub unsafe extern "C" fn cpPolyShapeGetVert(
    mut shape: *const cpShape,
    mut i: libc::c_int,
) -> cpVect {
    let mut count: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    if !((*shape).klass as libc::c_ulong
        == &polyClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &polyClass\0" as *const u8 as *const libc::c_char,
            b"../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            269 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a poly shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    tmp = cpPolyShapeGetCount(shape);
    count = tmp;
    if 0 as libc::c_int <= i {
        if !(i < count) {
            cpMessage(
                b"0 <= i && i < count\0" as *const u8 as *const libc::c_char,
                b"../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
                272 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Index out of range.\0" as *const u8 as *const libc::c_char,
            );
            abort();
        }
    } else {
        cpMessage(
            b"0 <= i && i < count\0" as *const u8 as *const libc::c_char,
            b"../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            272 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Index out of range.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*((*(shape as *mut cpPolyShape)).planes).offset((i + count) as isize)).v0;
}
pub unsafe extern "C" fn cpPolyShapeGetRadius(mut shape: *const cpShape) -> cpFloat {
    if !((*shape).klass as libc::c_ulong
        == &polyClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &polyClass\0" as *const u8 as *const libc::c_char,
            b"../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            280 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a poly shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpPolyShape)).r;
}
pub unsafe extern "C" fn cpPolyShapeSetVerts(
    mut shape: *mut cpShape,
    mut count: libc::c_int,
    mut verts: *mut cpVect,
    mut transform: cpTransform,
) {
    let mut hullVerts: *mut cpVect = 0 as *mut cpVect;
    let mut tmp: *mut _ = 0 as *mut _;
    let mut i: libc::c_int = 0;
    let mut hullCount: libc::c_uint = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut fresh14 = ::std::vec::from_elem(
        0,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
    );
    tmp = fresh14.as_mut_ptr();
    hullVerts = tmp as *mut cpVect;
    i = 0 as libc::c_int;
    while i < count {
        *hullVerts
            .offset(i as isize) = cpTransformPoint(transform, *verts.offset(i as isize));
        i += 1;
    }
    tmp___0 = cpConvexHull(
        count,
        hullVerts as *const cpVect,
        hullVerts,
        0 as *mut libc::c_void as *mut libc::c_int,
        0.0f64,
    );
    hullCount = tmp___0 as libc::c_uint;
    cpPolyShapeSetVertsRaw(shape, hullCount as libc::c_int, hullVerts);
}
pub unsafe extern "C" fn cpPolyShapeSetVertsRaw(
    mut shape: *mut cpShape,
    mut count: libc::c_int,
    mut verts: *mut cpVect,
) {
    let mut poly: *mut cpPolyShape = 0 as *mut cpPolyShape;
    let mut mass: cpFloat = 0.;
    if !((*shape).klass as libc::c_ulong
        == &polyClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &polyClass\0" as *const u8 as *const libc::c_char,
            b"../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            301 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a poly shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    poly = shape as *mut cpPolyShape;
    cpPolyShapeDestroy(poly);
    SetVerts(poly, count, verts as *const cpVect);
    mass = (*shape).massInfo.m;
    (*shape)
        .massInfo = cpPolyShapeMassInfo(
        (*shape).massInfo.m,
        count,
        verts as *const cpVect,
        (*poly).r,
    );
    if mass > 0.0f32 as cpFloat {
        cpBodyAccumulateMassFromShapes((*shape).body);
    }
}
pub unsafe extern "C" fn cpPolyShapeSetRadius(
    mut shape: *mut cpShape,
    mut radius: cpFloat,
) {
    let mut poly: *mut cpPolyShape = 0 as *mut cpPolyShape;
    if !((*shape).klass as libc::c_ulong
        == &polyClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &polyClass\0" as *const u8 as *const libc::c_char,
            b"../src/cpPolyShape.c\0" as *const u8 as *const libc::c_char,
            315 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a poly shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    poly = shape as *mut cpPolyShape;
    (*poly).r = radius;
}
#[inline]
unsafe extern "C" fn cpvnear(v1: cpVect, v2: cpVect, dist: cpFloat) -> cpBool {
    let mut tmp: cpFloat = 0.;
    tmp = cpvdistsq(v1, v2);
    return (tmp < dist * dist) as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn Next(mut i: libc::c_int, mut count: libc::c_int) -> libc::c_int {
    return (i + 1 as libc::c_int) % count;
}
unsafe extern "C" fn cpPolylineSizeForCapacity(
    mut capacity: libc::c_int,
) -> libc::c_int {
    return (::std::mem::size_of::<cpPolyline>() as libc::c_ulong)
        .wrapping_add(
            (capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
        ) as libc::c_int;
}
unsafe extern "C" fn cpPolylineMake(mut capacity: libc::c_int) -> *mut cpPolyline {
    let mut line: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    if capacity > 16 as libc::c_int {
        capacity = capacity;
    } else {
        capacity = 16 as libc::c_int;
    }
    tmp = cpPolylineSizeForCapacity(capacity);
    tmp___0 = calloc(1 as libc::c_int as size_t, tmp as size_t);
    line = tmp___0 as *mut cpPolyline;
    (*line).count = 0 as libc::c_int;
    (*line).capacity = capacity;
    return line;
}
unsafe extern "C" fn cpPolylineMake2(
    mut capacity: libc::c_int,
    mut a: cpVect,
    mut b: cpVect,
) -> *mut cpPolyline {
    let mut line: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut tmp: *mut cpPolyline = 0 as *mut cpPolyline;
    tmp = cpPolylineMake(capacity);
    line = tmp;
    (*line).count = 2 as libc::c_int;
    *((*line).verts).as_mut_ptr().offset(0 as libc::c_int as isize) = a;
    *((*line).verts).as_mut_ptr().offset(1 as libc::c_int as isize) = b;
    return line;
}
unsafe extern "C" fn cpPolylineShrink(mut line: *mut cpPolyline) -> *mut cpPolyline {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    (*line).capacity = (*line).count;
    tmp = cpPolylineSizeForCapacity((*line).count);
    tmp___0 = realloc(line as *mut libc::c_void, tmp as size_t);
    return tmp___0 as *mut cpPolyline;
}
pub unsafe extern "C" fn cpPolylineFree(mut line: *mut cpPolyline) {
    free(line as *mut libc::c_void);
}
unsafe extern "C" fn cpPolylineGrow(
    mut line: *mut cpPolyline,
    mut count: libc::c_int,
) -> *mut cpPolyline {
    let mut capacity: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    (*line).count += count;
    capacity = (*line).capacity;
    while (*line).count > capacity {
        capacity *= 2 as libc::c_int;
    }
    if (*line).capacity < capacity {
        (*line).capacity = capacity;
        tmp = cpPolylineSizeForCapacity(capacity);
        tmp___0 = realloc(line as *mut libc::c_void, tmp as size_t);
        line = tmp___0 as *mut cpPolyline;
    }
    return line;
}
unsafe extern "C" fn cpPolylinePush(
    mut line: *mut cpPolyline,
    mut v: cpVect,
) -> *mut cpPolyline {
    let mut count: libc::c_int = 0;
    count = (*line).count;
    line = cpPolylineGrow(line, 1 as libc::c_int);
    *((*line).verts).as_mut_ptr().offset(count as isize) = v;
    return line;
}
unsafe extern "C" fn cpPolylineEnqueue(
    mut line: *mut cpPolyline,
    mut v: cpVect,
) -> *mut cpPolyline {
    let mut count: libc::c_int = 0;
    count = (*line).count;
    line = cpPolylineGrow(line, 1 as libc::c_int);
    memmove(
        ((*line).verts).as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut libc::c_void,
        ((*line).verts).as_mut_ptr() as *const libc::c_void,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
    );
    *((*line).verts).as_mut_ptr().offset(0 as libc::c_int as isize) = v;
    return line;
}
pub unsafe extern "C" fn cpPolylineIsClosed(mut line: *mut cpPolyline) -> cpBool {
    let mut tmp: cpBool = 0;
    let mut tmp___0: libc::c_int = 0;
    if (*line).count > 1 as libc::c_int {
        tmp = cpveql(
            *((*line).verts).as_mut_ptr().offset(0 as libc::c_int as isize),
            *((*line).verts)
                .as_mut_ptr()
                .offset(((*line).count - 1 as libc::c_int) as isize),
        );
        if tmp != 0 {
            tmp___0 = 1 as libc::c_int;
        } else {
            tmp___0 = 0 as libc::c_int;
        }
    } else {
        tmp___0 = 0 as libc::c_int;
    }
    return tmp___0 as cpBool;
}
unsafe extern "C" fn cpPolylineIsShort(
    mut points: *mut cpVect,
    mut count: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut min: cpFloat,
) -> cpBool {
    let mut length: cpFloat = 0.;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: cpFloat = 0.;
    length = 0.0f32 as cpFloat;
    i = start;
    while i != end {
        tmp = Next(i, count);
        tmp___0 = cpvdist(*points.offset(i as isize), *points.offset(tmp as isize));
        length += tmp___0;
        if length > min {
            return 0 as libc::c_int as cpBool;
        }
        i = Next(i, count);
    }
    return 1 as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn Sharpness(mut a: cpVect, mut b: cpVect, mut c: cpVect) -> cpFloat {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpFloat = 0.;
    tmp = cpvsub(c, b);
    tmp___0 = cpvnormalize(tmp);
    tmp___1 = cpvsub(a, b);
    tmp___2 = cpvnormalize(tmp___1);
    tmp___3 = cpvdot(tmp___2, tmp___0);
    return tmp___3;
}
pub unsafe extern "C" fn cpPolylineSimplifyVertexes(
    mut line: *mut cpPolyline,
    mut tol: cpFloat,
) -> *mut cpPolyline {
    let mut reduced: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut tmp: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut minSharp: cpFloat = 0.;
    let mut tmp___0: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut vert: cpVect = cpVect { x: 0., y: 0. };
    let mut sharp: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpBool = 0;
    let mut tmp___3: cpFloat = 0.;
    tmp = cpPolylineMake2(
        0 as libc::c_int,
        *((*line).verts).as_mut_ptr().offset(0 as libc::c_int as isize),
        *((*line).verts).as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    reduced = tmp;
    tmp___0 = cos(tol);
    minSharp = -tmp___0;
    i = 2 as libc::c_int;
    while i < (*line).count {
        vert = *((*line).verts).as_mut_ptr().offset(i as isize);
        tmp___1 = Sharpness(
            *((*reduced).verts)
                .as_mut_ptr()
                .offset(((*reduced).count - 2 as libc::c_int) as isize),
            *((*reduced).verts)
                .as_mut_ptr()
                .offset(((*reduced).count - 1 as libc::c_int) as isize),
            vert,
        );
        sharp = tmp___1;
        if sharp <= minSharp {
            *((*reduced).verts)
                .as_mut_ptr()
                .offset(((*reduced).count - 1 as libc::c_int) as isize) = vert;
        } else {
            reduced = cpPolylinePush(reduced, vert);
        }
        i += 1;
    }
    tmp___2 = cpPolylineIsClosed(line);
    if tmp___2 != 0 {
        tmp___3 = Sharpness(
            *((*reduced).verts)
                .as_mut_ptr()
                .offset(((*reduced).count - 2 as libc::c_int) as isize),
            *((*reduced).verts).as_mut_ptr().offset(0 as libc::c_int as isize),
            *((*reduced).verts).as_mut_ptr().offset(1 as libc::c_int as isize),
        );
        if tmp___3 < minSharp {
            *((*reduced).verts)
                .as_mut_ptr()
                .offset(
                    0 as libc::c_int as isize,
                ) = *((*reduced).verts)
                .as_mut_ptr()
                .offset(((*reduced).count - 2 as libc::c_int) as isize);
            (*reduced).count -= 1;
        }
    }
    return reduced;
}
unsafe extern "C" fn DouglasPeucker(
    mut verts: *mut cpVect,
    mut reduced: *mut cpPolyline,
    mut length: libc::c_int,
    mut start: libc::c_int,
    mut end: libc::c_int,
    mut min: cpFloat,
    mut tol: cpFloat,
) -> *mut cpPolyline {
    let mut a: cpVect = cpVect { x: 0., y: 0. };
    let mut b: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    let mut max: cpFloat = 0.;
    let mut maxi: libc::c_int = 0;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut d: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut i: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut dist: cpFloat = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut tmp___7: libc::c_double = 0.;
    if (end - start + length) % length < 2 as libc::c_int {
        return reduced;
    }
    a = *verts.offset(start as isize);
    b = *verts.offset(end as isize);
    tmp = cpvnear(a, b, min);
    if tmp != 0 {
        tmp___0 = cpPolylineIsShort(verts, length, start, end, min);
        if tmp___0 != 0 {
            return reduced;
        }
    }
    max = 0.0f64;
    maxi = start;
    tmp___1 = cpvsub(b, a);
    tmp___2 = cpvperp(tmp___1);
    tmp___3 = cpvnormalize(tmp___2);
    n = tmp___3;
    tmp___4 = cpvdot(n, a);
    d = tmp___4;
    tmp___5 = Next(start, length);
    i = tmp___5;
    while i != end {
        tmp___6 = cpvdot(n, *verts.offset(i as isize));
        tmp___7 = fabs(tmp___6 - d);
        dist = tmp___7;
        if dist > max {
            max = dist;
            maxi = i;
        }
        i = Next(i, length);
    }
    if max > tol {
        reduced = DouglasPeucker(verts, reduced, length, start, maxi, min, tol);
        reduced = cpPolylinePush(reduced, *verts.offset(maxi as isize));
        reduced = DouglasPeucker(verts, reduced, length, maxi, end, min, tol);
    }
    return reduced;
}
pub unsafe extern "C" fn cpPolylineSimplifyCurves(
    mut line: *mut cpPolyline,
    mut tol: cpFloat,
) -> *mut cpPolyline {
    let mut reduced: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut tmp: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut min: cpFloat = 0.;
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut tmp___0: cpBool = 0;
    let mut tmp___1: *mut cpPolyline = 0 as *mut cpPolyline;
    tmp = cpPolylineMake((*line).count);
    reduced = tmp;
    min = tol / 2.0f32 as cpFloat;
    tmp___0 = cpPolylineIsClosed(line);
    if tmp___0 != 0 {
        cpLoopIndexes(
            ((*line).verts).as_mut_ptr() as *const cpVect,
            (*line).count - 1 as libc::c_int,
            &mut start,
            &mut end,
        );
        reduced = cpPolylinePush(
            reduced,
            *((*line).verts).as_mut_ptr().offset(start as isize),
        );
        reduced = DouglasPeucker(
            ((*line).verts).as_mut_ptr(),
            reduced,
            (*line).count - 1 as libc::c_int,
            start,
            end,
            min,
            tol,
        );
        reduced = cpPolylinePush(
            reduced,
            *((*line).verts).as_mut_ptr().offset(end as isize),
        );
        reduced = DouglasPeucker(
            ((*line).verts).as_mut_ptr(),
            reduced,
            (*line).count - 1 as libc::c_int,
            end,
            start,
            min,
            tol,
        );
        reduced = cpPolylinePush(
            reduced,
            *((*line).verts).as_mut_ptr().offset(start as isize),
        );
    } else {
        reduced = cpPolylinePush(
            reduced,
            *((*line).verts).as_mut_ptr().offset(0 as libc::c_int as isize),
        );
        reduced = DouglasPeucker(
            ((*line).verts).as_mut_ptr(),
            reduced,
            (*line).count,
            0 as libc::c_int,
            (*line).count - 1 as libc::c_int,
            min,
            tol,
        );
        reduced = cpPolylinePush(
            reduced,
            *((*line).verts)
                .as_mut_ptr()
                .offset(((*line).count - 1 as libc::c_int) as isize),
        );
    }
    tmp___1 = cpPolylineShrink(reduced);
    return tmp___1;
}
pub unsafe extern "C" fn cpPolylineSetAlloc() -> *mut cpPolylineSet {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpPolylineSet>() as libc::c_ulong,
    );
    return tmp as *mut cpPolylineSet;
}
pub unsafe extern "C" fn cpPolylineSetInit(
    mut set: *mut cpPolylineSet,
) -> *mut cpPolylineSet {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    (*set).count = 0 as libc::c_int;
    (*set).capacity = 8 as libc::c_int;
    tmp = calloc(
        (*set).capacity as size_t,
        ::std::mem::size_of::<cpPolyline>() as libc::c_ulong,
    );
    (*set).lines = tmp as *mut *mut cpPolyline;
    return set;
}
pub unsafe extern "C" fn cpPolylineSetNew() -> *mut cpPolylineSet {
    let mut tmp: *mut cpPolylineSet = 0 as *mut cpPolylineSet;
    let mut tmp___0: *mut cpPolylineSet = 0 as *mut cpPolylineSet;
    tmp = cpPolylineSetAlloc();
    tmp___0 = cpPolylineSetInit(tmp);
    return tmp___0;
}
pub unsafe extern "C" fn cpPolylineSetDestroy(
    mut set: *mut cpPolylineSet,
    mut freePolylines: cpBool,
) {
    let mut i: libc::c_int = 0;
    if freePolylines != 0 {
        i = 0 as libc::c_int;
        while i < (*set).count {
            cpPolylineFree(*((*set).lines).offset(i as isize));
            i += 1;
        }
    }
    free((*set).lines as *mut libc::c_void);
}
pub unsafe extern "C" fn cpPolylineSetFree(
    mut set: *mut cpPolylineSet,
    mut freePolylines: cpBool,
) {
    if !set.is_null() {
        cpPolylineSetDestroy(set, freePolylines);
        free(set as *mut libc::c_void);
    }
}
unsafe extern "C" fn cpPolylineSetFindEnds(
    mut set: *mut cpPolylineSet,
    mut v: cpVect,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut lines: *mut *mut cpPolyline = 0 as *mut *mut cpPolyline;
    let mut i: libc::c_int = 0;
    let mut line: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut tmp: cpBool = 0;
    count = (*set).count;
    lines = (*set).lines;
    i = 0 as libc::c_int;
    while i < count {
        line = *lines.offset(i as isize);
        tmp = cpveql(
            *((*line).verts)
                .as_mut_ptr()
                .offset(((*line).count - 1 as libc::c_int) as isize),
            v,
        );
        if tmp != 0 {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cpPolylineSetFindStarts(
    mut set: *mut cpPolylineSet,
    mut v: cpVect,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    let mut lines: *mut *mut cpPolyline = 0 as *mut *mut cpPolyline;
    let mut i: libc::c_int = 0;
    let mut tmp: cpBool = 0;
    count = (*set).count;
    lines = (*set).lines;
    i = 0 as libc::c_int;
    while i < count {
        tmp = cpveql(
            *((**lines.offset(i as isize)).verts)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize),
            v,
        );
        if tmp != 0 {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn cpPolylineSetPush(
    mut set: *mut cpPolylineSet,
    mut line: *mut cpPolyline,
) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    (*set).count += 1;
    if (*set).count > (*set).capacity {
        (*set).capacity *= 2 as libc::c_int;
        tmp = realloc(
            (*set).lines as *mut libc::c_void,
            ((*set).capacity as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<cpPolyline>() as libc::c_ulong),
        );
        (*set).lines = tmp as *mut *mut cpPolyline;
    }
    let ref mut fresh15 = *((*set).lines)
        .offset(((*set).count - 1 as libc::c_int) as isize);
    *fresh15 = line;
}
unsafe extern "C" fn cpPolylineSetAdd(
    mut set: *mut cpPolylineSet,
    mut v0: cpVect,
    mut v1: cpVect,
) {
    let mut tmp: *mut cpPolyline = 0 as *mut cpPolyline;
    tmp = cpPolylineMake2(16 as libc::c_int, v0, v1);
    cpPolylineSetPush(set, tmp);
}
unsafe extern "C" fn cpPolylineSetJoin(
    mut set: *mut cpPolylineSet,
    mut before: libc::c_int,
    mut after: libc::c_int,
) {
    let mut lbefore: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut lafter: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut count: libc::c_int = 0;
    lbefore = *((*set).lines).offset(before as isize);
    lafter = *((*set).lines).offset(after as isize);
    count = (*lbefore).count;
    lbefore = cpPolylineGrow(lbefore, (*lafter).count);
    memmove(
        ((*lbefore).verts).as_mut_ptr().offset(count as isize) as *mut libc::c_void,
        ((*lafter).verts).as_mut_ptr() as *const libc::c_void,
        ((*lafter).count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
    );
    let ref mut fresh16 = *((*set).lines).offset(before as isize);
    *fresh16 = lbefore;
    (*set).count -= 1;
    cpPolylineFree(*((*set).lines).offset(after as isize));
    let ref mut fresh17 = *((*set).lines).offset(after as isize);
    *fresh17 = *((*set).lines).offset((*set).count as isize);
}
pub unsafe extern "C" fn cpPolylineSetCollectSegment(
    mut v0: cpVect,
    mut v1: cpVect,
    mut lines: *mut cpPolylineSet,
) {
    let mut before: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut after: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    tmp = cpPolylineSetFindEnds(lines, v0);
    before = tmp;
    tmp___0 = cpPolylineSetFindStarts(lines, v1);
    after = tmp___0;
    let mut current_block_17: u64;
    if before >= 0 as libc::c_int {
        if after >= 0 as libc::c_int {
            if before == after {
                let ref mut fresh18 = *((*lines).lines).offset(before as isize);
                *fresh18 = cpPolylinePush(*((*lines).lines).offset(before as isize), v1);
            } else {
                cpPolylineSetJoin(lines, before, after);
            }
            current_block_17 = 6057473163062296781;
        } else {
            current_block_17 = 12465756146256798921;
        }
    } else {
        current_block_17 = 12465756146256798921;
    }
    match current_block_17 {
        12465756146256798921 => {
            if before >= 0 as libc::c_int {
                let ref mut fresh19 = *((*lines).lines).offset(before as isize);
                *fresh19 = cpPolylinePush(*((*lines).lines).offset(before as isize), v1);
            } else if after >= 0 as libc::c_int {
                let ref mut fresh20 = *((*lines).lines).offset(after as isize);
                *fresh20 = cpPolylineEnqueue(
                    *((*lines).lines).offset(after as isize),
                    v0,
                );
            } else {
                cpPolylineSetAdd(lines, v0, v1);
            }
        }
        _ => {}
    };
}
pub unsafe extern "C" fn cpPolylineToConvexHull(
    mut line: *mut cpPolyline,
    mut tol: cpFloat,
) -> *mut cpPolyline {
    let mut hull: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut tmp: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut tmp___0: *mut cpPolyline = 0 as *mut cpPolyline;
    tmp = cpPolylineMake((*line).count + 1 as libc::c_int);
    hull = tmp;
    (*hull)
        .count = cpConvexHull(
        (*line).count,
        ((*line).verts).as_mut_ptr() as *const cpVect,
        ((*hull).verts).as_mut_ptr(),
        0 as *mut libc::c_void as *mut libc::c_int,
        tol,
    );
    hull = cpPolylinePush(
        hull,
        *((*hull).verts).as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    tmp___0 = cpPolylineShrink(hull);
    return tmp___0;
}
unsafe extern "C" fn FindSteiner(
    mut count: libc::c_int,
    mut verts: *mut cpVect,
    mut notch: Notch,
) -> cpFloat {
    let mut min: cpFloat = 0.;
    let mut tmp: libc::c_float = 0.;
    let mut feature: cpFloat = 0.;
    let mut i: libc::c_int = 0;
    let mut index___0: libc::c_int = 0;
    let mut seg_a: cpVect = cpVect { x: 0., y: 0. };
    let mut seg_b: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: libc::c_int = 0;
    let mut thing_a: cpFloat = 0.;
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpFloat = 0.;
    let mut thing_b: cpFloat = 0.;
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpFloat = 0.;
    let mut t: cpFloat = 0.;
    let mut dist: cpFloat = 0.;
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpFloat = 0.;
    tmp = ::std::f32::INFINITY;
    min = tmp as cpFloat;
    feature = -1.0f64;
    i = 1 as libc::c_int;
    while i < count - 1 as libc::c_int {
        index___0 = (notch.i + i) % count;
        seg_a = *verts.offset(index___0 as isize);
        tmp___0 = Next(index___0, count);
        seg_b = *verts.offset(tmp___0 as isize);
        tmp___1 = cpvsub(seg_a, notch.v);
        tmp___2 = cpvcross(notch.n, tmp___1);
        thing_a = tmp___2;
        tmp___3 = cpvsub(seg_b, notch.v);
        tmp___4 = cpvcross(notch.n, tmp___3);
        thing_b = tmp___4;
        if thing_a * thing_b <= 0.0f64 {
            t = thing_a / (thing_a - thing_b);
            tmp___5 = cpvlerp(seg_a, seg_b, t);
            tmp___6 = cpvsub(tmp___5, notch.v);
            tmp___7 = cpvdot(notch.n, tmp___6);
            dist = tmp___7;
            if dist >= 0.0f64 {
                if dist <= min {
                    min = dist;
                    feature = index___0 as cpFloat + t;
                }
            }
        }
        i += 1;
    }
    return feature;
}
unsafe extern "C" fn DeepestNotch(
    mut count: libc::c_int,
    mut verts: *mut cpVect,
    mut hullCount: libc::c_int,
    mut hullVerts: *mut cpVect,
    mut first: libc::c_int,
    mut tol: cpFloat,
) -> Notch {
    let mut notch: Notch = Notch {
        i: 0,
        d: 0.,
        v: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
    };
    let mut j: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut a: cpVect = cpVect { x: 0., y: 0. };
    let mut b: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: libc::c_int = 0;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut d: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut v: cpVect = cpVect { x: 0., y: 0. };
    let mut depth: cpFloat = 0.;
    let mut tmp___5: cpFloat = 0.;
    let mut tmp___6: cpBool = 0;
    notch.i = 0 as libc::c_int;
    notch.d = 0.0f64;
    notch.v.x = 0.0f64;
    notch.v.y = 0.0f64;
    notch.n.x = 0.0f64;
    notch.n.y = 0.0f64;
    tmp = Next(first, count);
    j = tmp;
    i = 0 as libc::c_int;
    while i < hullCount {
        a = *hullVerts.offset(i as isize);
        tmp___0 = Next(i, hullCount);
        b = *hullVerts.offset(tmp___0 as isize);
        tmp___1 = cpvsub(a, b);
        tmp___2 = cpvrperp(tmp___1);
        tmp___3 = cpvnormalize(tmp___2);
        n = tmp___3;
        tmp___4 = cpvdot(n, a);
        d = tmp___4;
        v = *verts.offset(j as isize);
        loop {
            tmp___6 = cpveql(v, b);
            if tmp___6 != 0 {
                break;
            }
            tmp___5 = cpvdot(n, v);
            depth = tmp___5 - d;
            if depth > notch.d {
                notch.d = depth;
                notch.i = j;
                notch.v = v;
                notch.n = n;
            }
            j = Next(j, count);
            v = *verts.offset(j as isize);
        }
        j = Next(j, count);
        i += 1;
    }
    return notch;
}
#[inline]
unsafe extern "C" fn IMAX(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    if a > b {
        tmp = a;
    } else {
        tmp = b;
    }
    return tmp;
}
unsafe extern "C" fn ApproximateConcaveDecomposition(
    mut verts: *mut cpVect,
    mut count: libc::c_int,
    mut tol: cpFloat,
    mut set: *mut cpPolylineSet,
) {
    let mut first: libc::c_int = 0;
    let mut hullVerts: *mut cpVect = 0 as *mut cpVect;
    let mut tmp: *mut _ = 0 as *mut _;
    let mut hullCount: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut notch: Notch = Notch {
        i: 0,
        d: 0.,
        v: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
    };
    let mut tmp___1: Notch = Notch {
        i: 0,
        d: 0.,
        v: cpVect { x: 0., y: 0. },
        n: cpVect { x: 0., y: 0. },
    };
    let mut steiner_it: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut steiner_i: libc::c_int = 0;
    let mut steiner: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut sub1_count: libc::c_int = 0;
    let mut sub2_count: libc::c_int = 0;
    let mut scratch: *mut cpVect = 0 as *mut cpVect;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: *mut _ = 0 as *mut _;
    let mut i: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut hull: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut tmp___7: *mut cpPolyline = 0 as *mut cpPolyline;
    let mut fresh21 = ::std::vec::from_elem(
        0,
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong) as usize,
    );
    tmp = fresh21.as_mut_ptr();
    hullVerts = tmp as *mut cpVect;
    tmp___0 = cpConvexHull(count, verts as *const cpVect, hullVerts, &mut first, 0.0f64);
    hullCount = tmp___0;
    if hullCount != count {
        tmp___1 = DeepestNotch(count, verts, hullCount, hullVerts, first, tol);
        notch = tmp___1;
        if notch.d > tol {
            tmp___2 = FindSteiner(count, verts, notch);
            steiner_it = tmp___2;
            if steiner_it >= 0.0f64 {
                steiner_i = steiner_it as libc::c_int;
                tmp___3 = Next(steiner_i, count);
                tmp___4 = cpvlerp(
                    *verts.offset(steiner_i as isize),
                    *verts.offset(tmp___3 as isize),
                    steiner_it - steiner_i as cpFloat,
                );
                steiner = tmp___4;
                sub1_count = (steiner_i - notch.i + count) % count + 1 as libc::c_int;
                sub2_count = count - (steiner_i - notch.i + count) % count;
                tmp___5 = IMAX(sub1_count, sub2_count);
                let mut fresh22 = ::std::vec::from_elem(
                    0,
                    ((tmp___5 + 1 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong)
                        as usize,
                );
                tmp___6 = fresh22.as_mut_ptr();
                scratch = tmp___6 as *mut cpVect;
                i = 0 as libc::c_int;
                while i < sub1_count {
                    *scratch
                        .offset(
                            i as isize,
                        ) = *verts.offset(((notch.i + i) % count) as isize);
                    i += 1;
                }
                *scratch.offset(sub1_count as isize) = steiner;
                ApproximateConcaveDecomposition(
                    scratch,
                    sub1_count + 1 as libc::c_int,
                    tol,
                    set,
                );
                i___0 = 0 as libc::c_int;
                while i___0 < sub2_count {
                    *scratch
                        .offset(
                            i___0 as isize,
                        ) = *verts
                        .offset(
                            ((steiner_i + 1 as libc::c_int + i___0) % count) as isize,
                        );
                    i___0 += 1;
                }
                *scratch.offset(sub2_count as isize) = steiner;
                ApproximateConcaveDecomposition(
                    scratch,
                    sub2_count + 1 as libc::c_int,
                    tol,
                    set,
                );
                return;
            }
        }
    }
    tmp___7 = cpPolylineMake(hullCount + 1 as libc::c_int);
    hull = tmp___7;
    memcpy(
        ((*hull).verts).as_mut_ptr() as *mut libc::c_void,
        hullVerts as *const libc::c_void,
        (hullCount as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong),
    );
    *((*hull).verts)
        .as_mut_ptr()
        .offset(hullCount as isize) = *hullVerts.offset(0 as libc::c_int as isize);
    (*hull).count = hullCount + 1 as libc::c_int;
    cpPolylineSetPush(set, hull);
}
pub unsafe extern "C" fn cpPolylineConvexDecomposition(
    mut line: *mut cpPolyline,
    mut tol: cpFloat,
) -> *mut cpPolylineSet {
    let mut set: *mut cpPolylineSet = 0 as *mut cpPolylineSet;
    let mut tmp: *mut cpPolylineSet = 0 as *mut cpPolylineSet;
    tmp = cpPolylineSetNew();
    set = tmp;
    ApproximateConcaveDecomposition(
        ((*line).verts).as_mut_ptr(),
        (*line).count - 1 as libc::c_int,
        tol,
        set,
    );
    return set;
}
unsafe extern "C" fn preStep___5(mut joint: *mut cpRatchetJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut angle: cpFloat = 0.;
    let mut phase: cpFloat = 0.;
    let mut ratchet: cpFloat = 0.;
    let mut delta: cpFloat = 0.;
    let mut diff: cpFloat = 0.;
    let mut pdist: cpFloat = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut maxBias: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    angle = (*joint).angle;
    phase = (*joint).phase;
    ratchet = (*joint).ratchet;
    delta = (*b).a - (*a).a;
    diff = angle - delta;
    pdist = 0.0f32 as cpFloat;
    if diff * ratchet > 0.0f32 as cpFloat {
        pdist = diff;
    } else {
        tmp = floor((delta - phase) / ratchet);
        (*joint).angle = tmp * ratchet + phase;
    }
    (*joint).iSum = 1.0f32 as cpFloat / ((*a).i_inv + (*b).i_inv);
    maxBias = (*joint).constraint.maxBias;
    tmp___0 = bias_coef((*joint).constraint.errorBias, dt);
    (*joint).bias = cpfclamp(-tmp___0 * pdist / dt, -maxBias, maxBias);
    if (*joint).bias == 0. {
        (*joint).jAcc = 0.0f32 as cpFloat;
    }
}
unsafe extern "C" fn applyCachedImpulse___5(
    mut joint: *mut cpRatchetJoint,
    mut dt_coef: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut j: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    j = (*joint).jAcc * dt_coef;
    (*a).w -= j * (*a).i_inv;
    (*b).w += j * (*b).i_inv;
}
unsafe extern "C" fn applyImpulse___5(mut joint: *mut cpRatchetJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut wr: cpFloat = 0.;
    let mut ratchet: cpFloat = 0.;
    let mut jMax: cpFloat = 0.;
    let mut j: cpFloat = 0.;
    let mut jOld: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    if (*joint).bias == 0. {
        return;
    }
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    wr = (*b).w - (*a).w;
    ratchet = (*joint).ratchet;
    jMax = (*joint).constraint.maxForce * dt;
    j = -((*joint).bias + wr) * (*joint).iSum;
    jOld = (*joint).jAcc;
    tmp = cpfabs(ratchet);
    tmp___0 = cpfclamp((jOld + j) * ratchet, 0.0f32 as cpFloat, jMax * tmp);
    (*joint).jAcc = tmp___0 / ratchet;
    j = (*joint).jAcc - jOld;
    (*a).w -= j * (*a).i_inv;
    (*b).w += j * (*b).i_inv;
}
unsafe extern "C" fn getImpulse___5(mut joint: *mut cpRatchetJoint) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpfabs((*joint).jAcc);
    return tmp;
}
static mut klass___6: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRatchetJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    preStep___5
                        as unsafe extern "C" fn(*mut cpRatchetJoint, cpFloat) -> (),
                ),
            ),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRatchetJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyCachedImpulse___5
                        as unsafe extern "C" fn(*mut cpRatchetJoint, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRatchetJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyImpulse___5
                        as unsafe extern "C" fn(*mut cpRatchetJoint, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRatchetJoint) -> cpFloat>,
                Option::<unsafe extern "C" fn(*mut cpConstraint) -> cpFloat>,
            >(
                Some(
                    getImpulse___5
                        as unsafe extern "C" fn(*mut cpRatchetJoint) -> cpFloat,
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpRatchetJointAlloc() -> *mut cpRatchetJoint {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpRatchetJoint>() as libc::c_ulong,
    );
    return tmp as *mut cpRatchetJoint;
}
pub unsafe extern "C" fn cpRatchetJointInit(
    mut joint: *mut cpRatchetJoint,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut phase: cpFloat,
    mut ratchet: cpFloat,
) -> *mut cpRatchetJoint {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    cpConstraintInit(joint as *mut cpConstraint, &klass___6, a, b);
    (*joint).angle = 0.0f32 as cpFloat;
    (*joint).phase = phase;
    (*joint).ratchet = ratchet;
    if !b.is_null() {
        tmp = (*b).a;
    } else {
        tmp = 0.0f32 as cpFloat;
    }
    if !a.is_null() {
        tmp___0 = (*a).a;
    } else {
        tmp___0 = 0.0f32 as cpFloat;
    }
    (*joint).angle = tmp - tmp___0;
    return joint;
}
pub unsafe extern "C" fn cpRatchetJointNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut phase: cpFloat,
    mut ratchet: cpFloat,
) -> *mut cpConstraint {
    let mut tmp: *mut cpRatchetJoint = 0 as *mut cpRatchetJoint;
    let mut tmp___0: *mut cpRatchetJoint = 0 as *mut cpRatchetJoint;
    tmp = cpRatchetJointAlloc();
    tmp___0 = cpRatchetJointInit(tmp, a, b, phase, ratchet);
    return tmp___0 as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsRatchetJoint(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass as libc::c_ulong
        == &klass___6 as *const cpConstraintClass as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpRatchetJointGetAngle(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsRatchetJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsRatchetJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpRatchetJoint.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a ratchet joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpRatchetJoint)).angle;
}
pub unsafe extern "C" fn cpRatchetJointSetAngle(
    mut constraint: *mut cpConstraint,
    mut angle: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsRatchetJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsRatchetJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpRatchetJoint.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a ratchet joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpRatchetJoint)).angle = angle;
}
pub unsafe extern "C" fn cpRatchetJointGetPhase(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsRatchetJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsRatchetJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpRatchetJoint.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a ratchet joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpRatchetJoint)).phase;
}
pub unsafe extern "C" fn cpRatchetJointSetPhase(
    mut constraint: *mut cpConstraint,
    mut phase: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsRatchetJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsRatchetJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpRatchetJoint.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a ratchet joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpRatchetJoint)).phase = phase;
}
pub unsafe extern "C" fn cpRatchetJointGetRatchet(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsRatchetJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsRatchetJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpRatchetJoint.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a ratchet joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpRatchetJoint)).ratchet;
}
pub unsafe extern "C" fn cpRatchetJointSetRatchet(
    mut constraint: *mut cpConstraint,
    mut ratchet: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsRatchetJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsRatchetJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpRatchetJoint.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a ratchet joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpRatchetJoint)).ratchet = ratchet;
}
pub unsafe extern "C" fn cpCheckPointGreater(a: cpVect, b: cpVect, c: cpVect) -> cpBool {
    return ((b.y - a.y) * (a.x + b.x - 2 as libc::c_int as cpFloat * c.x)
        > (b.x - a.x) * (a.y + b.y - 2 as libc::c_int as cpFloat * c.y)) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpCheckAxis(
    mut v0: cpVect,
    mut v1: cpVect,
    mut p: cpVect,
    mut n: cpVect,
) -> cpBool {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    tmp = cpvdot(p, n);
    tmp___0 = cpvdot(v1, n);
    tmp___1 = cpvdot(v0, n);
    tmp___2 = cpfmax(tmp___1, tmp___0);
    return (tmp <= tmp___2) as libc::c_int as cpBool;
}
unsafe extern "C" fn preStep___6(mut joint: *mut cpRotaryLimitJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut dist: cpFloat = 0.;
    let mut pdist: cpFloat = 0.;
    let mut maxBias: cpFloat = 0.;
    let mut tmp: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    dist = (*b).a - (*a).a;
    pdist = 0.0f32 as cpFloat;
    if dist > (*joint).max {
        pdist = (*joint).max - dist;
    } else if dist < (*joint).min {
        pdist = (*joint).min - dist;
    }
    (*joint).iSum = 1.0f32 as cpFloat / ((*a).i_inv + (*b).i_inv);
    maxBias = (*joint).constraint.maxBias;
    tmp = bias_coef((*joint).constraint.errorBias, dt);
    (*joint).bias = cpfclamp(-tmp * pdist / dt, -maxBias, maxBias);
    if (*joint).bias == 0. {
        (*joint).jAcc = 0.0f32 as cpFloat;
    }
}
unsafe extern "C" fn applyCachedImpulse___6(
    mut joint: *mut cpRotaryLimitJoint,
    mut dt_coef: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut j: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    j = (*joint).jAcc * dt_coef;
    (*a).w -= j * (*a).i_inv;
    (*b).w += j * (*b).i_inv;
}
unsafe extern "C" fn applyImpulse___6(
    mut joint: *mut cpRotaryLimitJoint,
    mut dt: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut wr: cpFloat = 0.;
    let mut jMax: cpFloat = 0.;
    let mut j: cpFloat = 0.;
    let mut jOld: cpFloat = 0.;
    if (*joint).bias == 0. {
        return;
    }
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    wr = (*b).w - (*a).w;
    jMax = (*joint).constraint.maxForce * dt;
    j = -((*joint).bias + wr) * (*joint).iSum;
    jOld = (*joint).jAcc;
    if (*joint).bias < 0.0f32 as cpFloat {
        (*joint).jAcc = cpfclamp(jOld + j, 0.0f32 as cpFloat, jMax);
    } else {
        (*joint).jAcc = cpfclamp(jOld + j, -jMax, 0.0f32 as cpFloat);
    }
    j = (*joint).jAcc - jOld;
    (*a).w -= j * (*a).i_inv;
    (*b).w += j * (*b).i_inv;
}
unsafe extern "C" fn getImpulse___6(mut joint: *mut cpRotaryLimitJoint) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpfabs((*joint).jAcc);
    return tmp;
}
static mut klass___7: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    preStep___6
                        as unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> (),
                ),
            ),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyCachedImpulse___6
                        as unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyImpulse___6
                        as unsafe extern "C" fn(*mut cpRotaryLimitJoint, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpRotaryLimitJoint) -> cpFloat>,
                Option::<unsafe extern "C" fn(*mut cpConstraint) -> cpFloat>,
            >(
                Some(
                    getImpulse___6
                        as unsafe extern "C" fn(*mut cpRotaryLimitJoint) -> cpFloat,
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpRotaryLimitJointAlloc() -> *mut cpRotaryLimitJoint {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpRotaryLimitJoint>() as libc::c_ulong,
    );
    return tmp as *mut cpRotaryLimitJoint;
}
pub unsafe extern "C" fn cpRotaryLimitJointInit(
    mut joint: *mut cpRotaryLimitJoint,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut min: cpFloat,
    mut max: cpFloat,
) -> *mut cpRotaryLimitJoint {
    cpConstraintInit(joint as *mut cpConstraint, &klass___7, a, b);
    (*joint).min = min;
    (*joint).max = max;
    (*joint).jAcc = 0.0f32 as cpFloat;
    return joint;
}
pub unsafe extern "C" fn cpRotaryLimitJointNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut min: cpFloat,
    mut max: cpFloat,
) -> *mut cpConstraint {
    let mut tmp: *mut cpRotaryLimitJoint = 0 as *mut cpRotaryLimitJoint;
    let mut tmp___0: *mut cpRotaryLimitJoint = 0 as *mut cpRotaryLimitJoint;
    tmp = cpRotaryLimitJointAlloc();
    tmp___0 = cpRotaryLimitJointInit(tmp, a, b, min, max);
    return tmp___0 as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsRotaryLimitJoint(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass as libc::c_ulong
        == &klass___7 as *const cpConstraintClass as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpRotaryLimitJointGetMin(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsRotaryLimitJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsRotaryLimitJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpRotaryLimitJoint.c\0" as *const u8 as *const libc::c_char,
            135 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a rotary limit joint.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpRotaryLimitJoint)).min;
}
pub unsafe extern "C" fn cpRotaryLimitJointSetMin(
    mut constraint: *mut cpConstraint,
    mut min: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsRotaryLimitJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsRotaryLimitJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpRotaryLimitJoint.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a rotary limit joint.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpRotaryLimitJoint)).min = min;
}
pub unsafe extern "C" fn cpRotaryLimitJointGetMax(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsRotaryLimitJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsRotaryLimitJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpRotaryLimitJoint.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a rotary limit joint.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpRotaryLimitJoint)).max;
}
pub unsafe extern "C" fn cpRotaryLimitJointSetMax(
    mut constraint: *mut cpConstraint,
    mut max: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsRotaryLimitJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsRotaryLimitJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpRotaryLimitJoint.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a rotary limit joint.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpRotaryLimitJoint)).max = max;
}
static mut cpvzero___6: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpBBNewForExtents(c: cpVect, hw: cpFloat, hh: cpFloat) -> cpBB {
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = cpBBNew(c.x - hw, c.y - hh, c.x + hw, c.y + hh);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpBBNewForCircle(p: cpVect, r: cpFloat) -> cpBB {
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = cpBBNewForExtents(p, r, r);
    return tmp;
}
#[inline]
unsafe extern "C" fn cpShapeActive(mut shape: *mut cpShape) -> cpBool {
    let mut tmp: libc::c_int = 0;
    if !((*shape).prev).is_null() {
        tmp = 1 as libc::c_int;
    } else if !((*shape).body).is_null() {
        if (*(*shape).body).shapeList as libc::c_ulong == shape as libc::c_ulong {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp as cpBool;
}
pub unsafe extern "C" fn cpShapeInit(
    mut shape: *mut cpShape,
    mut klass___12: *const cpShapeClass,
    mut body: *mut cpBody,
    mut massInfo: cpShapeMassInfo,
) -> *mut cpShape {
    (*shape).klass = klass___12;
    (*shape).body = body;
    (*shape).massInfo = massInfo;
    (*shape).sensor = 0 as libc::c_int as cpBool;
    (*shape).e = 0.0f32 as cpFloat;
    (*shape).u = 0.0f32 as cpFloat;
    (*shape).surfaceV = cpvzero___6;
    (*shape).type_0 = 0 as libc::c_int as cpCollisionType;
    (*shape).filter.group = 0 as libc::c_int as cpGroup;
    (*shape).filter.categories = !(0 as libc::c_int as cpBitmask);
    (*shape).filter.mask = !(0 as libc::c_int as cpBitmask);
    (*shape).userData = 0 as *mut libc::c_void;
    (*shape).space = 0 as *mut libc::c_void as *mut cpSpace;
    (*shape).next = 0 as *mut libc::c_void as *mut cpShape;
    (*shape).prev = 0 as *mut libc::c_void as *mut cpShape;
    return shape;
}
pub unsafe extern "C" fn cpShapeDestroy(mut shape: *mut cpShape) {
    if !((*shape).klass).is_null() {
        if ((*(*shape).klass).destroy).is_some() {
            (Some(((*(*shape).klass).destroy).expect("non-null function pointer")))
                .expect("non-null function pointer")(shape);
        }
    }
}
pub unsafe extern "C" fn cpShapeFree(mut shape: *mut cpShape) {
    if !shape.is_null() {
        cpShapeDestroy(shape);
        free(shape as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpShapeGetSpace(mut shape: *const cpShape) -> *mut cpSpace {
    return (*shape).space;
}
pub unsafe extern "C" fn cpShapeGetBody(mut shape: *const cpShape) -> *mut cpBody {
    return (*shape).body;
}
pub unsafe extern "C" fn cpShapeSetBody(mut shape: *mut cpShape, mut body: *mut cpBody) {
    let mut tmp: cpBool = 0;
    tmp = cpShapeActive(shape);
    if tmp != 0 {
        cpMessage(
            b"!cpShapeActive(shape)\0" as *const u8 as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            90 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You cannot change the body on an active shape. You must remove the shape from the space before changing the body.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    (*shape).body = body;
}
pub unsafe extern "C" fn cpShapeGetMass(mut shape: *mut cpShape) -> cpFloat {
    return (*shape).massInfo.m;
}
pub unsafe extern "C" fn cpShapeSetMass(mut shape: *mut cpShape, mut mass: cpFloat) {
    let mut body: *mut cpBody = 0 as *mut cpBody;
    body = (*shape).body;
    cpBodyActivate(body);
    (*shape).massInfo.m = mass;
    cpBodyAccumulateMassFromShapes(body);
}
pub unsafe extern "C" fn cpShapeGetDensity(mut shape: *mut cpShape) -> cpFloat {
    return (*shape).massInfo.m / (*shape).massInfo.area;
}
pub unsafe extern "C" fn cpShapeSetDensity(
    mut shape: *mut cpShape,
    mut density: cpFloat,
) {
    cpShapeSetMass(shape, density * (*shape).massInfo.area);
}
pub unsafe extern "C" fn cpShapeGetMoment(mut shape: *mut cpShape) -> cpFloat {
    return (*shape).massInfo.m * (*shape).massInfo.i;
}
pub unsafe extern "C" fn cpShapeGetArea(mut shape: *mut cpShape) -> cpFloat {
    return (*shape).massInfo.area;
}
pub unsafe extern "C" fn cpShapeGetCenterOfGravity(mut shape: *mut cpShape) -> cpVect {
    return (*shape).massInfo.cog;
}
pub unsafe extern "C" fn cpShapeGetBB(mut shape: *const cpShape) -> cpBB {
    return (*shape).bb;
}
pub unsafe extern "C" fn cpShapeGetSensor(mut shape: *const cpShape) -> cpBool {
    return (*shape).sensor;
}
pub unsafe extern "C" fn cpShapeSetSensor(mut shape: *mut cpShape, mut sensor: cpBool) {
    cpBodyActivate((*shape).body);
    (*shape).sensor = sensor;
}
pub unsafe extern "C" fn cpShapeGetElasticity(mut shape: *const cpShape) -> cpFloat {
    return (*shape).e;
}
pub unsafe extern "C" fn cpShapeSetElasticity(
    mut shape: *mut cpShape,
    mut elasticity: cpFloat,
) {
    if !(elasticity >= 0.0f32 as cpFloat) {
        cpMessage(
            b"elasticity >= 0.0f\0" as *const u8 as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Elasticity must be positive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate((*shape).body);
    (*shape).e = elasticity;
}
pub unsafe extern "C" fn cpShapeGetFriction(mut shape: *const cpShape) -> cpFloat {
    return (*shape).u;
}
pub unsafe extern "C" fn cpShapeSetFriction(
    mut shape: *mut cpShape,
    mut friction: cpFloat,
) {
    if !(friction >= 0.0f32 as cpFloat) {
        cpMessage(
            b"friction >= 0.0f\0" as *const u8 as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Friction must be postive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate((*shape).body);
    (*shape).u = friction;
}
pub unsafe extern "C" fn cpShapeGetSurfaceVelocity(mut shape: *const cpShape) -> cpVect {
    return (*shape).surfaceV;
}
pub unsafe extern "C" fn cpShapeSetSurfaceVelocity(
    mut shape: *mut cpShape,
    mut surfaceVelocity: cpVect,
) {
    cpBodyActivate((*shape).body);
    (*shape).surfaceV = surfaceVelocity;
}
pub unsafe extern "C" fn cpShapeGetUserData(mut shape: *const cpShape) -> cpDataPointer {
    return (*shape).userData;
}
pub unsafe extern "C" fn cpShapeSetUserData(
    mut shape: *mut cpShape,
    mut userData: cpDataPointer,
) {
    (*shape).userData = userData;
}
pub unsafe extern "C" fn cpShapeGetCollisionType(
    mut shape: *const cpShape,
) -> cpCollisionType {
    return (*shape).type_0;
}
pub unsafe extern "C" fn cpShapeSetCollisionType(
    mut shape: *mut cpShape,
    mut collisionType: cpCollisionType,
) {
    cpBodyActivate((*shape).body);
    (*shape).type_0 = collisionType;
}
pub unsafe extern "C" fn cpShapeGetFilter(mut shape: *const cpShape) -> cpShapeFilter {
    return (*shape).filter;
}
pub unsafe extern "C" fn cpShapeSetFilter(
    mut shape: *mut cpShape,
    mut filter: cpShapeFilter,
) {
    cpBodyActivate((*shape).body);
    (*shape).filter = filter;
}
pub unsafe extern "C" fn cpShapeCacheBB(mut shape: *mut cpShape) -> cpBB {
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = cpShapeUpdate(shape, (*(*shape).body).transform);
    return tmp;
}
pub unsafe extern "C" fn cpShapeUpdate(
    mut shape: *mut cpShape,
    mut transform: cpTransform,
) -> cpBB {
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = (Some(((*(*shape).klass).cacheData).expect("non-null function pointer")))
        .expect("non-null function pointer")(shape, transform);
    (*shape).bb = tmp;
    return tmp;
}
pub unsafe extern "C" fn cpShapePointQuery(
    mut shape: *const cpShape,
    mut p: cpVect,
    mut info: *mut cpPointQueryInfo,
) -> cpFloat {
    let mut blank: cpPointQueryInfo = cpPointQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        distance: 0.,
        gradient: cpVect { x: 0., y: 0. },
    };
    let mut tmp: libc::c_float = 0.;
    tmp = ::std::f32::INFINITY;
    blank.shape = 0 as *mut libc::c_void as *const cpShape;
    blank.point = cpvzero___6;
    blank.distance = tmp as cpFloat;
    blank.gradient = cpvzero___6;
    if !info.is_null() {
        *info = blank;
    } else {
        info = &mut blank;
    }
    (Some(((*(*shape).klass).pointQuery).expect("non-null function pointer")))
        .expect("non-null function pointer")(shape, p, info);
    return (*info).distance;
}
pub unsafe extern "C" fn cpShapeSegmentQuery(
    mut shape: *const cpShape,
    mut a: cpVect,
    mut b: cpVect,
    mut radius: cpFloat,
    mut info: *mut cpSegmentQueryInfo,
) -> cpBool {
    let mut blank: cpSegmentQueryInfo = cpSegmentQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        normal: cpVect { x: 0., y: 0. },
        alpha: 0.,
    };
    let mut nearest: cpPointQueryInfo = cpPointQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        distance: 0.,
        gradient: cpVect { x: 0., y: 0. },
    };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    blank.shape = 0 as *mut libc::c_void as *const cpShape;
    blank.point = b;
    blank.normal = cpvzero___6;
    blank.alpha = 1.0f32 as cpFloat;
    if !info.is_null() {
        *info = blank;
    } else {
        info = &mut blank;
    }
    (Some(((*(*shape).klass).pointQuery).expect("non-null function pointer")))
        .expect("non-null function pointer")(shape, a, &mut nearest);
    if nearest.distance <= radius {
        (*info).shape = shape;
        (*info).alpha = 0.0f64;
        tmp = cpvsub(a, nearest.point);
        (*info).normal = cpvnormalize(tmp);
    } else {
        (Some(((*(*shape).klass).segmentQuery).expect("non-null function pointer")))
            .expect("non-null function pointer")(shape, a, b, radius, info);
    }
    return ((*info).shape as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpShapesCollide(
    mut a: *const cpShape,
    mut b: *const cpShape,
) -> cpContactPointSet {
    let mut contacts: [cpContact; 2] = [cpContact {
        r1: cpVect { x: 0., y: 0. },
        r2: cpVect { x: 0., y: 0. },
        nMass: 0.,
        tMass: 0.,
        bounce: 0.,
        jnAcc: 0.,
        jtAcc: 0.,
        jBias: 0.,
        bias: 0.,
        hash: 0,
    }; 2];
    let mut info: cpCollisionInfo = cpCollisionInfo {
        a: 0 as *const cpShape,
        b: 0 as *const cpShape,
        id: 0,
        n: cpVect { x: 0., y: 0. },
        count: 0,
        arr: 0 as *mut cpContact,
    };
    let mut tmp: cpCollisionInfo = cpCollisionInfo {
        a: 0 as *const cpShape,
        b: 0 as *const cpShape,
        id: 0,
        n: cpVect { x: 0., y: 0. },
        count: 0,
        arr: 0 as *mut cpContact,
    };
    let mut set: cpContactPointSet = cpContactPointSet {
        count: 0,
        normal: cpVect { x: 0., y: 0. },
        points: [__anonstruct_points_450528349 {
            pointA: cpVect { x: 0., y: 0. },
            pointB: cpVect { x: 0., y: 0. },
            distance: 0.,
        }; 2],
    };
    let mut swapped: cpBool = 0;
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut i: libc::c_int = 0;
    let mut p1: cpVect = cpVect { x: 0., y: 0. };
    let mut p2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpCollide(a, b, 0 as libc::c_int as cpCollisionID, contacts.as_mut_ptr());
    info = tmp;
    set.count = info.count;
    swapped = (a as libc::c_ulong != info.a as libc::c_ulong) as libc::c_int as cpBool;
    if swapped != 0 {
        tmp___0 = cpvneg(info.n);
        set.normal = tmp___0;
    } else {
        set.normal = info.n;
    }
    i = 0 as libc::c_int;
    while i < info.count {
        p1 = contacts[i as usize].r1;
        p2 = contacts[i as usize].r2;
        if swapped != 0 {
            set.points[i as usize].pointA = p2;
        } else {
            set.points[i as usize].pointA = p1;
        }
        if swapped != 0 {
            set.points[i as usize].pointB = p1;
        } else {
            set.points[i as usize].pointB = p2;
        }
        tmp___1 = cpvsub(p2, p1);
        set.points[i as usize].distance = cpvdot(tmp___1, set.normal);
        i += 1;
    }
    return set;
}
pub unsafe extern "C" fn cpCircleShapeAlloc() -> *mut cpCircleShape {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpCircleShape>() as libc::c_ulong,
    );
    return tmp as *mut cpCircleShape;
}
unsafe extern "C" fn cpCircleShapeCacheData(
    mut circle: *mut cpCircleShape,
    mut transform: cpTransform,
) -> cpBB {
    let mut c: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = cpTransformPoint(transform, (*circle).c);
    (*circle).tc = tmp;
    c = tmp;
    tmp___0 = cpBBNewForCircle(c, (*circle).r);
    return tmp___0;
}
unsafe extern "C" fn cpCircleShapePointQuery(
    mut circle: *mut cpCircleShape,
    mut p: cpVect,
    mut info: *mut cpPointQueryInfo,
) {
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut d: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut r: cpFloat = 0.;
    let mut r_over_d: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpvsub(p, (*circle).tc);
    delta = tmp;
    tmp___0 = cpvlength(delta);
    d = tmp___0;
    r = (*circle).r;
    (*info).shape = circle as *mut cpShape as *const cpShape;
    if d > 0.0f32 as cpFloat {
        tmp___1 = r / d;
    } else {
        tmp___1 = r;
    }
    r_over_d = tmp___1;
    tmp___2 = cpvmult(delta, r_over_d);
    (*info).point = cpvadd((*circle).tc, tmp___2);
    (*info).distance = d - r;
    if d > 1e-5f64 {
        tmp___3 = cpvmult(delta, 1.0f32 as cpFloat / d);
        (*info).gradient = tmp___3;
    } else {
        tmp___4 = cpv(0.0f32 as cpFloat, 1.0f32 as cpFloat);
        (*info).gradient = tmp___4;
    };
}
unsafe extern "C" fn cpCircleShapeSegmentQuery(
    mut circle: *mut cpCircleShape,
    mut a: cpVect,
    mut b: cpVect,
    mut radius: cpFloat,
    mut info: *mut cpSegmentQueryInfo,
) {
    CircleSegmentQuery(
        circle as *mut cpShape,
        (*circle).tc,
        (*circle).r,
        a,
        b,
        radius,
        info,
    );
}
unsafe extern "C" fn cpCircleShapeMassInfo(
    mut mass: cpFloat,
    mut radius: cpFloat,
    mut center: cpVect,
) -> cpShapeMassInfo {
    let mut info: cpShapeMassInfo = cpShapeMassInfo {
        m: 0.,
        i: 0.,
        cog: cpVect { x: 0., y: 0. },
        area: 0.,
    };
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    tmp = cpMomentForCircle(1.0f32 as cpFloat, 0.0f32 as cpFloat, radius, cpvzero___6);
    tmp___0 = cpAreaForCircle(0.0f32 as cpFloat, radius);
    info.m = mass;
    info.i = tmp;
    info.cog = center;
    info.area = tmp___0;
    return info;
}
static mut cpCircleShapeClass: cpShapeClass = unsafe {
    {
        let mut init = cpShapeClass {
            type_0: CP_CIRCLE_SHAPE,
            cacheData: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpCircleShape, cpTransform) -> cpBB>,
                Option::<unsafe extern "C" fn(*mut cpShape, cpTransform) -> cpBB>,
            >(
                Some(
                    cpCircleShapeCacheData
                        as unsafe extern "C" fn(*mut cpCircleShape, cpTransform) -> cpBB,
                ),
            ),
            destroy: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut cpShape) -> ()>,
            >(0 as *const libc::c_void as *mut libc::c_void),
            pointQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpCircleShape,
                        cpVect,
                        *mut cpPointQueryInfo,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const cpShape,
                        cpVect,
                        *mut cpPointQueryInfo,
                    ) -> (),
                >,
            >(
                Some(
                    cpCircleShapePointQuery
                        as unsafe extern "C" fn(
                            *mut cpCircleShape,
                            cpVect,
                            *mut cpPointQueryInfo,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpCircleShape,
                        cpVect,
                        cpVect,
                        cpFloat,
                        *mut cpSegmentQueryInfo,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const cpShape,
                        cpVect,
                        cpVect,
                        cpFloat,
                        *mut cpSegmentQueryInfo,
                    ) -> (),
                >,
            >(
                Some(
                    cpCircleShapeSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpCircleShape,
                            cpVect,
                            cpVect,
                            cpFloat,
                            *mut cpSegmentQueryInfo,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpCircleShapeInit(
    mut circle: *mut cpCircleShape,
    mut body: *mut cpBody,
    mut radius: cpFloat,
    mut offset: cpVect,
) -> *mut cpCircleShape {
    let mut tmp: cpShapeMassInfo = cpShapeMassInfo {
        m: 0.,
        i: 0.,
        cog: cpVect { x: 0., y: 0. },
        area: 0.,
    };
    (*circle).c = offset;
    (*circle).r = radius;
    tmp = cpCircleShapeMassInfo(0.0f32 as cpFloat, radius, offset);
    cpShapeInit(circle as *mut cpShape, &cpCircleShapeClass, body, tmp);
    return circle;
}
pub unsafe extern "C" fn cpCircleShapeNew(
    mut body: *mut cpBody,
    mut radius: cpFloat,
    mut offset: cpVect,
) -> *mut cpShape {
    let mut tmp: *mut cpCircleShape = 0 as *mut cpCircleShape;
    let mut tmp___0: *mut cpCircleShape = 0 as *mut cpCircleShape;
    tmp = cpCircleShapeAlloc();
    tmp___0 = cpCircleShapeInit(tmp, body, radius, offset);
    return tmp___0 as *mut cpShape;
}
pub unsafe extern "C" fn cpCircleShapeGetOffset(mut shape: *const cpShape) -> cpVect {
    if !((*shape).klass as libc::c_ulong
        == &cpCircleShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpCircleShapeClass\0" as *const u8 as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            360 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a circle shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpCircleShape)).c;
}
pub unsafe extern "C" fn cpCircleShapeGetRadius(mut shape: *const cpShape) -> cpFloat {
    if !((*shape).klass as libc::c_ulong
        == &cpCircleShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpCircleShapeClass\0" as *const u8 as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            367 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a circle shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpCircleShape)).r;
}
pub unsafe extern "C" fn cpSegmentShapeAlloc() -> *mut cpSegmentShape {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpSegmentShape>() as libc::c_ulong,
    );
    return tmp as *mut cpSegmentShape;
}
unsafe extern "C" fn cpSegmentShapeCacheData(
    mut seg___0: *mut cpSegmentShape,
    mut transform: cpTransform,
) -> cpBB {
    let mut l: cpFloat = 0.;
    let mut r: cpFloat = 0.;
    let mut b: cpFloat = 0.;
    let mut t: cpFloat = 0.;
    let mut rad: cpFloat = 0.;
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    (*seg___0).ta = cpTransformPoint(transform, (*seg___0).a);
    (*seg___0).tb = cpTransformPoint(transform, (*seg___0).b);
    (*seg___0).tn = cpTransformVect(transform, (*seg___0).n);
    if (*seg___0).ta.x < (*seg___0).tb.x {
        l = (*seg___0).ta.x;
        r = (*seg___0).tb.x;
    } else {
        l = (*seg___0).tb.x;
        r = (*seg___0).ta.x;
    }
    if (*seg___0).ta.y < (*seg___0).tb.y {
        b = (*seg___0).ta.y;
        t = (*seg___0).tb.y;
    } else {
        b = (*seg___0).tb.y;
        t = (*seg___0).ta.y;
    }
    rad = (*seg___0).r;
    tmp = cpBBNew(l - rad, b - rad, r + rad, t + rad);
    return tmp;
}
unsafe extern "C" fn cpSegmentShapePointQuery(
    mut seg___0: *mut cpSegmentShape,
    mut p: cpVect,
    mut info: *mut cpPointQueryInfo,
) {
    let mut closest: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut d: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut r: cpFloat = 0.;
    let mut g: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpClosetPointOnSegment(p, (*seg___0).ta, (*seg___0).tb);
    closest = tmp;
    tmp___0 = cpvsub(p, closest);
    delta = tmp___0;
    tmp___1 = cpvlength(delta);
    d = tmp___1;
    r = (*seg___0).r;
    tmp___2 = cpvmult(delta, 1.0f32 as cpFloat / d);
    g = tmp___2;
    (*info).shape = seg___0 as *mut cpShape as *const cpShape;
    if d != 0. {
        tmp___3 = cpvmult(g, r);
        tmp___4 = cpvadd(closest, tmp___3);
        (*info).point = tmp___4;
    } else {
        (*info).point = closest;
    }
    (*info).distance = d - r;
    if d > 1e-5f64 {
        (*info).gradient = g;
    } else {
        (*info).gradient = (*seg___0).n;
    };
}
unsafe extern "C" fn cpSegmentShapeSegmentQuery(
    mut seg___0: *mut cpSegmentShape,
    mut a: cpVect,
    mut b: cpVect,
    mut r2: cpFloat,
    mut info: *mut cpSegmentQueryInfo,
) {
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut d: cpFloat = 0.;
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpFloat = 0.;
    let mut r: cpFloat = 0.;
    let mut flipped_n: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut seg_offset: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut seg_a: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut seg_b: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpVect = cpVect { x: 0., y: 0. };
    let mut d_offset: cpFloat = 0.;
    let mut tmp___8: cpFloat = 0.;
    let mut ad: cpFloat = 0.;
    let mut bd: cpFloat = 0.;
    let mut tmp___9: cpFloat = 0.;
    let mut t: cpFloat = 0.;
    let mut tmp___10: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___11: cpVect = cpVect { x: 0., y: 0. };
    let mut info1: cpSegmentQueryInfo = cpSegmentQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        normal: cpVect { x: 0., y: 0. },
        alpha: 0.,
    };
    let mut info2: cpSegmentQueryInfo = cpSegmentQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        normal: cpVect { x: 0., y: 0. },
        alpha: 0.,
    };
    let mut tmp___12: cpFloat = 0.;
    let mut tmp___13: cpFloat = 0.;
    n = (*seg___0).tn;
    tmp = cpvsub((*seg___0).ta, a);
    tmp___0 = cpvdot(tmp, n);
    d = tmp___0;
    r = (*seg___0).r + r2;
    if d > 0.0f32 as cpFloat {
        tmp___1 = cpvneg(n);
        tmp___2 = tmp___1;
    } else {
        tmp___2 = n;
    }
    flipped_n = tmp___2;
    tmp___3 = cpvmult(flipped_n, r);
    tmp___4 = cpvsub(tmp___3, a);
    seg_offset = tmp___4;
    tmp___5 = cpvadd((*seg___0).ta, seg_offset);
    seg_a = tmp___5;
    tmp___6 = cpvadd((*seg___0).tb, seg_offset);
    seg_b = tmp___6;
    tmp___7 = cpvsub(b, a);
    delta = tmp___7;
    tmp___12 = cpvcross(delta, seg_a);
    tmp___13 = cpvcross(delta, seg_b);
    if tmp___12 * tmp___13 <= 0.0f32 as cpFloat {
        if d > 0.0f32 as cpFloat {
            tmp___8 = -r;
        } else {
            tmp___8 = r;
        }
        d_offset = d + tmp___8;
        ad = -d_offset;
        tmp___9 = cpvdot(delta, n);
        bd = tmp___9 - d_offset;
        if ad * bd < 0.0f32 as cpFloat {
            t = ad / (ad - bd);
            (*info).shape = seg___0 as *mut cpShape as *const cpShape;
            tmp___10 = cpvmult(flipped_n, r2);
            tmp___11 = cpvlerp(a, b, t);
            (*info).point = cpvsub(tmp___11, tmp___10);
            (*info).normal = flipped_n;
            (*info).alpha = t;
        }
    } else if r != 0.0f32 as cpFloat {
        info1.shape = 0 as *mut libc::c_void as *const cpShape;
        info1.point = b;
        info1.normal = cpvzero___6;
        info1.alpha = 1.0f32 as cpFloat;
        info2.shape = 0 as *mut libc::c_void as *const cpShape;
        info2.point = b;
        info2.normal = cpvzero___6;
        info2.alpha = 1.0f32 as cpFloat;
        CircleSegmentQuery(
            seg___0 as *mut cpShape,
            (*seg___0).ta,
            (*seg___0).r,
            a,
            b,
            r2,
            &mut info1,
        );
        CircleSegmentQuery(
            seg___0 as *mut cpShape,
            (*seg___0).tb,
            (*seg___0).r,
            a,
            b,
            r2,
            &mut info2,
        );
        if info1.alpha < info2.alpha {
            *info = info1;
        } else {
            *info = info2;
        }
    }
}
unsafe extern "C" fn cpSegmentShapeMassInfo(
    mut mass: cpFloat,
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
) -> cpShapeMassInfo {
    let mut info: cpShapeMassInfo = cpShapeMassInfo {
        m: 0.,
        i: 0.,
        cog: cpVect { x: 0., y: 0. },
        area: 0.,
    };
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpFloat = 0.;
    tmp = cpvdist(a, b);
    tmp___0 = cpMomentForBox(
        1.0f32 as cpFloat,
        tmp + 2.0f32 as cpFloat * r,
        2.0f32 as cpFloat * r,
    );
    tmp___1 = cpvlerp(a, b, 0.5f32 as cpFloat);
    tmp___2 = cpAreaForSegment(a, b, r);
    info.m = mass;
    info.i = tmp___0;
    info.cog = tmp___1;
    info.area = tmp___2;
    return info;
}
static mut cpSegmentShapeClass: cpShapeClass = unsafe {
    {
        let mut init = cpShapeClass {
            type_0: CP_SEGMENT_SHAPE,
            cacheData: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSegmentShape, cpTransform) -> cpBB>,
                Option::<unsafe extern "C" fn(*mut cpShape, cpTransform) -> cpBB>,
            >(
                Some(
                    cpSegmentShapeCacheData
                        as unsafe extern "C" fn(*mut cpSegmentShape, cpTransform) -> cpBB,
                ),
            ),
            destroy: ::std::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut cpShape) -> ()>,
            >(0 as *const libc::c_void as *mut libc::c_void),
            pointQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSegmentShape,
                        cpVect,
                        *mut cpPointQueryInfo,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const cpShape,
                        cpVect,
                        *mut cpPointQueryInfo,
                    ) -> (),
                >,
            >(
                Some(
                    cpSegmentShapePointQuery
                        as unsafe extern "C" fn(
                            *mut cpSegmentShape,
                            cpVect,
                            *mut cpPointQueryInfo,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSegmentShape,
                        cpVect,
                        cpVect,
                        cpFloat,
                        *mut cpSegmentQueryInfo,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const cpShape,
                        cpVect,
                        cpVect,
                        cpFloat,
                        *mut cpSegmentQueryInfo,
                    ) -> (),
                >,
            >(
                Some(
                    cpSegmentShapeSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpSegmentShape,
                            cpVect,
                            cpVect,
                            cpFloat,
                            *mut cpSegmentQueryInfo,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpSegmentShapeInit(
    mut seg___0: *mut cpSegmentShape,
    mut body: *mut cpBody,
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
) -> *mut cpSegmentShape {
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpShapeMassInfo = cpShapeMassInfo {
        m: 0.,
        i: 0.,
        cog: cpVect { x: 0., y: 0. },
        area: 0.,
    };
    (*seg___0).a = a;
    (*seg___0).b = b;
    tmp = cpvsub(b, a);
    tmp___0 = cpvnormalize(tmp);
    (*seg___0).n = cpvrperp(tmp___0);
    (*seg___0).r = r;
    (*seg___0).a_tangent = cpvzero___6;
    (*seg___0).b_tangent = cpvzero___6;
    tmp___1 = cpSegmentShapeMassInfo(0.0f32 as cpFloat, a, b, r);
    cpShapeInit(seg___0 as *mut cpShape, &cpSegmentShapeClass, body, tmp___1);
    return seg___0;
}
pub unsafe extern "C" fn cpSegmentShapeNew(
    mut body: *mut cpBody,
    mut a: cpVect,
    mut b: cpVect,
    mut r: cpFloat,
) -> *mut cpShape {
    let mut tmp: *mut cpSegmentShape = 0 as *mut cpSegmentShape;
    let mut tmp___0: *mut cpSegmentShape = 0 as *mut cpSegmentShape;
    tmp = cpSegmentShapeAlloc();
    tmp___0 = cpSegmentShapeInit(tmp, body, a, b, r);
    return tmp___0 as *mut cpShape;
}
pub unsafe extern "C" fn cpSegmentShapeGetA(mut shape: *const cpShape) -> cpVect {
    if !((*shape).klass as libc::c_ulong
        == &cpSegmentShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            513 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpSegmentShape)).a;
}
pub unsafe extern "C" fn cpSegmentShapeGetB(mut shape: *const cpShape) -> cpVect {
    if !((*shape).klass as libc::c_ulong
        == &cpSegmentShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            520 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpSegmentShape)).b;
}
pub unsafe extern "C" fn cpSegmentShapeGetNormal(mut shape: *const cpShape) -> cpVect {
    if !((*shape).klass as libc::c_ulong
        == &cpSegmentShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            527 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpSegmentShape)).n;
}
pub unsafe extern "C" fn cpSegmentShapeGetRadius(mut shape: *const cpShape) -> cpFloat {
    if !((*shape).klass as libc::c_ulong
        == &cpSegmentShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            534 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(shape as *mut cpSegmentShape)).r;
}
pub unsafe extern "C" fn cpSegmentShapeSetNeighbors(
    mut shape: *mut cpShape,
    mut prev: cpVect,
    mut next: cpVect,
) {
    let mut seg___0: *mut cpSegmentShape = 0 as *mut cpSegmentShape;
    if !((*shape).klass as libc::c_ulong
        == &cpSegmentShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            541 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    seg___0 = shape as *mut cpSegmentShape;
    (*seg___0).a_tangent = cpvsub(prev, (*seg___0).a);
    (*seg___0).b_tangent = cpvsub(next, (*seg___0).b);
}
pub unsafe extern "C" fn cpCircleShapeSetRadius(
    mut shape: *mut cpShape,
    mut radius: cpFloat,
) {
    let mut circle: *mut cpCircleShape = 0 as *mut cpCircleShape;
    let mut mass: cpFloat = 0.;
    if !((*shape).klass as libc::c_ulong
        == &cpCircleShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpCircleShapeClass\0" as *const u8 as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            555 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a circle shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    circle = shape as *mut cpCircleShape;
    (*circle).r = radius;
    mass = (*shape).massInfo.m;
    (*shape).massInfo = cpCircleShapeMassInfo(mass, (*circle).r, (*circle).c);
    if mass > 0.0f32 as cpFloat {
        cpBodyAccumulateMassFromShapes((*shape).body);
    }
}
pub unsafe extern "C" fn cpCircleShapeSetOffset(
    mut shape: *mut cpShape,
    mut offset: cpVect,
) {
    let mut circle: *mut cpCircleShape = 0 as *mut cpCircleShape;
    let mut mass: cpFloat = 0.;
    if !((*shape).klass as libc::c_ulong
        == &cpCircleShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpCircleShapeClass\0" as *const u8 as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            568 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a circle shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    circle = shape as *mut cpCircleShape;
    (*circle).c = offset;
    mass = (*shape).massInfo.m;
    (*shape)
        .massInfo = cpCircleShapeMassInfo((*shape).massInfo.m, (*circle).r, (*circle).c);
    if mass > 0.0f32 as cpFloat {
        cpBodyAccumulateMassFromShapes((*shape).body);
    }
}
pub unsafe extern "C" fn cpSegmentShapeSetEndpoints(
    mut shape: *mut cpShape,
    mut a: cpVect,
    mut b: cpVect,
) {
    let mut seg___0: *mut cpSegmentShape = 0 as *mut cpSegmentShape;
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut mass: cpFloat = 0.;
    if !((*shape).klass as libc::c_ulong
        == &cpSegmentShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            581 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    seg___0 = shape as *mut cpSegmentShape;
    (*seg___0).a = a;
    (*seg___0).b = b;
    tmp = cpvsub(b, a);
    tmp___0 = cpvnormalize(tmp);
    (*seg___0).n = cpvperp(tmp___0);
    mass = (*shape).massInfo.m;
    (*shape)
        .massInfo = cpSegmentShapeMassInfo(
        (*shape).massInfo.m,
        (*seg___0).a,
        (*seg___0).b,
        (*seg___0).r,
    );
    if mass > 0.0f32 as cpFloat {
        cpBodyAccumulateMassFromShapes((*shape).body);
    }
}
pub unsafe extern "C" fn cpSegmentShapeSetRadius(
    mut shape: *mut cpShape,
    mut radius: cpFloat,
) {
    let mut seg___0: *mut cpSegmentShape = 0 as *mut cpSegmentShape;
    let mut mass: cpFloat = 0.;
    if !((*shape).klass as libc::c_ulong
        == &cpSegmentShapeClass as *const cpShapeClass as libc::c_ulong)
    {
        cpMessage(
            b"shape->klass == &cpSegmentShapeClass\0" as *const u8
                as *const libc::c_char,
            b"../src/cpShape.c\0" as *const u8 as *const libc::c_char,
            596 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Shape is not a segment shape.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    seg___0 = shape as *mut cpSegmentShape;
    (*seg___0).r = radius;
    mass = (*shape).massInfo.m;
    (*shape)
        .massInfo = cpSegmentShapeMassInfo(
        (*shape).massInfo.m,
        (*seg___0).a,
        (*seg___0).b,
        (*seg___0).r,
    );
    if mass > 0.0f32 as cpFloat {
        cpBodyAccumulateMassFromShapes((*shape).body);
    }
}
unsafe extern "C" fn preStep___7(mut joint: *mut cpSimpleMotor, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    (*joint).iSum = 1.0f32 as cpFloat / ((*a).i_inv + (*b).i_inv);
}
unsafe extern "C" fn applyCachedImpulse___7(
    mut joint: *mut cpSimpleMotor,
    mut dt_coef: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut j: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    j = (*joint).jAcc * dt_coef;
    (*a).w -= j * (*a).i_inv;
    (*b).w += j * (*b).i_inv;
}
unsafe extern "C" fn applyImpulse___7(mut joint: *mut cpSimpleMotor, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut wr: cpFloat = 0.;
    let mut jMax: cpFloat = 0.;
    let mut j: cpFloat = 0.;
    let mut jOld: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    wr = (*b).w - (*a).w + (*joint).rate;
    jMax = (*joint).constraint.maxForce * dt;
    j = -wr * (*joint).iSum;
    jOld = (*joint).jAcc;
    (*joint).jAcc = cpfclamp(jOld + j, -jMax, jMax);
    j = (*joint).jAcc - jOld;
    (*a).w -= j * (*a).i_inv;
    (*b).w += j * (*b).i_inv;
}
unsafe extern "C" fn getImpulse___7(mut joint: *mut cpSimpleMotor) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpfabs((*joint).jAcc);
    return tmp;
}
static mut klass___8: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSimpleMotor, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    preStep___7
                        as unsafe extern "C" fn(*mut cpSimpleMotor, cpFloat) -> (),
                ),
            ),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSimpleMotor, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyCachedImpulse___7
                        as unsafe extern "C" fn(*mut cpSimpleMotor, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSimpleMotor, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyImpulse___7
                        as unsafe extern "C" fn(*mut cpSimpleMotor, cpFloat) -> (),
                ),
            ),
            getImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSimpleMotor) -> cpFloat>,
                Option::<unsafe extern "C" fn(*mut cpConstraint) -> cpFloat>,
            >(
                Some(
                    getImpulse___7 as unsafe extern "C" fn(*mut cpSimpleMotor) -> cpFloat,
                ),
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpSimpleMotorAlloc() -> *mut cpSimpleMotor {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpSimpleMotor>() as libc::c_ulong,
    );
    return tmp as *mut cpSimpleMotor;
}
pub unsafe extern "C" fn cpSimpleMotorInit(
    mut joint: *mut cpSimpleMotor,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut rate: cpFloat,
) -> *mut cpSimpleMotor {
    cpConstraintInit(joint as *mut cpConstraint, &klass___8, a, b);
    (*joint).rate = rate;
    (*joint).jAcc = 0.0f32 as cpFloat;
    return joint;
}
pub unsafe extern "C" fn cpSimpleMotorNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut rate: cpFloat,
) -> *mut cpConstraint {
    let mut tmp: *mut cpSimpleMotor = 0 as *mut cpSimpleMotor;
    let mut tmp___0: *mut cpSimpleMotor = 0 as *mut cpSimpleMotor;
    tmp = cpSimpleMotorAlloc();
    tmp___0 = cpSimpleMotorInit(tmp, a, b, rate);
    return tmp___0 as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsSimpleMotor(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass as libc::c_ulong
        == &klass___8 as *const cpConstraintClass as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpSimpleMotorGetRate(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsSimpleMotor(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsSimpleMotor(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSimpleMotor.c\0" as *const u8 as *const libc::c_char,
            113 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a SimpleMotor.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpSimpleMotor)).rate;
}
pub unsafe extern "C" fn cpSimpleMotorSetRate(
    mut constraint: *mut cpConstraint,
    mut rate: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsSimpleMotor(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsSimpleMotor(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSimpleMotor.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a SimpleMotor.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpSimpleMotor)).rate = rate;
}
static mut cpvzero___7: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
unsafe extern "C" fn preStep___8(mut joint: *mut cpSlideJoint, mut dt: cpFloat) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut dist: cpFloat = 0.;
    let mut tmp___4: cpFloat = 0.;
    let mut pdist: cpFloat = 0.;
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpFloat = 0.;
    let mut maxBias: cpFloat = 0.;
    let mut tmp___7: cpFloat = 0.;
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    tmp = cpvsub((*joint).anchorA, (*a).cog);
    (*joint).r1 = cpTransformVect((*a).transform, tmp);
    tmp___0 = cpvsub((*joint).anchorB, (*b).cog);
    (*joint).r2 = cpTransformVect((*b).transform, tmp___0);
    tmp___1 = cpvadd((*a).p, (*joint).r1);
    tmp___2 = cpvadd((*b).p, (*joint).r2);
    tmp___3 = cpvsub(tmp___2, tmp___1);
    delta = tmp___3;
    tmp___4 = cpvlength(delta);
    dist = tmp___4;
    pdist = 0.0f32 as cpFloat;
    if dist > (*joint).max {
        pdist = dist - (*joint).max;
        (*joint).n = cpvnormalize(delta);
    } else if dist < (*joint).min {
        pdist = (*joint).min - dist;
        tmp___5 = cpvnormalize(delta);
        (*joint).n = cpvneg(tmp___5);
    } else {
        (*joint).n = cpvzero___7;
        (*joint).jnAcc = 0.0f32 as cpFloat;
    }
    tmp___6 = k_scalar(a, b, (*joint).r1, (*joint).r2, (*joint).n);
    (*joint).nMass = 1.0f32 as cpFloat / tmp___6;
    maxBias = (*joint).constraint.maxBias;
    tmp___7 = bias_coef((*joint).constraint.errorBias, dt);
    (*joint).bias = cpfclamp(-tmp___7 * pdist / dt, -maxBias, maxBias);
}
unsafe extern "C" fn applyCachedImpulse___8(
    mut joint: *mut cpSlideJoint,
    mut dt_coef: cpFloat,
) {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut j: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    tmp = cpvmult((*joint).n, (*joint).jnAcc * dt_coef);
    j = tmp;
    apply_impulses(a, b, (*joint).r1, (*joint).r2, j);
}
unsafe extern "C" fn applyImpulse___8(mut joint: *mut cpSlideJoint, mut dt: cpFloat) {
    let mut tmp: cpBool = 0;
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut r1: cpVect = cpVect { x: 0., y: 0. };
    let mut r2: cpVect = cpVect { x: 0., y: 0. };
    let mut vr: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut vrn: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut jn___0: cpFloat = 0.;
    let mut jnOld: cpFloat = 0.;
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    tmp = cpveql((*joint).n, cpvzero___7);
    if tmp != 0 {
        return;
    }
    a = (*joint).constraint.a;
    b = (*joint).constraint.b;
    n = (*joint).n;
    r1 = (*joint).r1;
    r2 = (*joint).r2;
    tmp___0 = relative_velocity(a, b, r1, r2);
    vr = tmp___0;
    tmp___1 = cpvdot(vr, n);
    vrn = tmp___1;
    jn___0 = ((*joint).bias - vrn) * (*joint).nMass;
    jnOld = (*joint).jnAcc;
    (*joint)
        .jnAcc = cpfclamp(
        jnOld + jn___0,
        -(*joint).constraint.maxForce * dt,
        0.0f32 as cpFloat,
    );
    jn___0 = (*joint).jnAcc - jnOld;
    tmp___2 = cpvmult(n, jn___0);
    apply_impulses(a, b, (*joint).r1, (*joint).r2, tmp___2);
}
unsafe extern "C" fn getImpulse___8(mut joint: *mut cpConstraint) -> cpFloat {
    let mut tmp: cpFloat = 0.;
    tmp = cpfabs((*(joint as *mut cpSlideJoint)).jnAcc);
    return tmp;
}
static mut klass___9: cpConstraintClass = unsafe {
    {
        let mut init = cpConstraintClass {
            preStep: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSlideJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    preStep___8 as unsafe extern "C" fn(*mut cpSlideJoint, cpFloat) -> (),
                ),
            ),
            applyCachedImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSlideJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyCachedImpulse___8
                        as unsafe extern "C" fn(*mut cpSlideJoint, cpFloat) -> (),
                ),
            ),
            applyImpulse: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSlideJoint, cpFloat) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpConstraint, cpFloat) -> ()>,
            >(
                Some(
                    applyImpulse___8
                        as unsafe extern "C" fn(*mut cpSlideJoint, cpFloat) -> (),
                ),
            ),
            getImpulse: Some(
                getImpulse___8 as unsafe extern "C" fn(*mut cpConstraint) -> cpFloat,
            ),
        };
        init
    }
};
pub unsafe extern "C" fn cpSlideJointAlloc() -> *mut cpSlideJoint {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpSlideJoint>() as libc::c_ulong,
    );
    return tmp as *mut cpSlideJoint;
}
pub unsafe extern "C" fn cpSlideJointInit(
    mut joint: *mut cpSlideJoint,
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
    mut min: cpFloat,
    mut max: cpFloat,
) -> *mut cpSlideJoint {
    cpConstraintInit(joint as *mut cpConstraint, &klass___9, a, b);
    (*joint).anchorA = anchorA;
    (*joint).anchorB = anchorB;
    (*joint).min = min;
    (*joint).max = max;
    (*joint).jnAcc = 0.0f32 as cpFloat;
    return joint;
}
pub unsafe extern "C" fn cpSlideJointNew(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
    mut anchorA: cpVect,
    mut anchorB: cpVect,
    mut min: cpFloat,
    mut max: cpFloat,
) -> *mut cpConstraint {
    let mut tmp: *mut cpSlideJoint = 0 as *mut cpSlideJoint;
    let mut tmp___0: *mut cpSlideJoint = 0 as *mut cpSlideJoint;
    tmp = cpSlideJointAlloc();
    tmp___0 = cpSlideJointInit(tmp, a, b, anchorA, anchorB, min, max);
    return tmp___0 as *mut cpConstraint;
}
pub unsafe extern "C" fn cpConstraintIsSlideJoint(
    mut constraint: *const cpConstraint,
) -> cpBool {
    return ((*constraint).klass as libc::c_ulong
        == &klass___9 as *const cpConstraintClass as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpSlideJointGetAnchorA(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsSlideJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsSlideJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSlideJoint.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a slide joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpSlideJoint)).anchorA;
}
pub unsafe extern "C" fn cpSlideJointSetAnchorA(
    mut constraint: *mut cpConstraint,
    mut anchorA: cpVect,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsSlideJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsSlideJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSlideJoint.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a slide joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpSlideJoint)).anchorA = anchorA;
}
pub unsafe extern "C" fn cpSlideJointGetAnchorB(
    mut constraint: *const cpConstraint,
) -> cpVect {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsSlideJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsSlideJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSlideJoint.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a slide joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpSlideJoint)).anchorB;
}
pub unsafe extern "C" fn cpSlideJointSetAnchorB(
    mut constraint: *mut cpConstraint,
    mut anchorB: cpVect,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsSlideJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsSlideJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSlideJoint.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a slide joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpSlideJoint)).anchorB = anchorB;
}
pub unsafe extern "C" fn cpSlideJointGetMin(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsSlideJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsSlideJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSlideJoint.c\0" as *const u8 as *const libc::c_char,
            170 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a slide joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpSlideJoint)).min;
}
pub unsafe extern "C" fn cpSlideJointSetMin(
    mut constraint: *mut cpConstraint,
    mut min: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsSlideJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsSlideJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSlideJoint.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a slide joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpSlideJoint)).min = min;
}
pub unsafe extern "C" fn cpSlideJointGetMax(
    mut constraint: *const cpConstraint,
) -> cpFloat {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsSlideJoint(constraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsSlideJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSlideJoint.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a slide joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    return (*(constraint as *mut cpSlideJoint)).max;
}
pub unsafe extern "C" fn cpSlideJointSetMax(
    mut constraint: *mut cpConstraint,
    mut max: cpFloat,
) {
    let mut tmp: cpBool = 0;
    tmp = cpConstraintIsSlideJoint(constraint as *const cpConstraint);
    if tmp == 0 {
        cpMessage(
            b"cpConstraintIsSlideJoint(constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSlideJoint.c\0" as *const u8 as *const libc::c_char,
            192 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is not a slide joint.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpConstraintActivateBodies(constraint);
    (*(constraint as *mut cpSlideJoint)).max = max;
}
static mut cpvzero___8: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpSpatialIndexReindex(mut index___0: *mut cpSpatialIndex) {
    (Some(((*(*index___0).klass).reindex).expect("non-null function pointer")))
        .expect("non-null function pointer")(index___0);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexReindexObject(
    mut index___0: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    (Some(((*(*index___0).klass).reindexObject).expect("non-null function pointer")))
        .expect("non-null function pointer")(index___0, obj, hashid);
}
unsafe extern "C" fn arbiterSetEql(
    mut shapes: *mut *mut cpShape,
    mut arb: *mut cpArbiter,
) -> cpBool {
    let mut a: *mut cpShape = 0 as *mut cpShape;
    let mut b: *mut cpShape = 0 as *mut cpShape;
    let mut tmp: libc::c_int = 0;
    a = *shapes.offset(0 as libc::c_int as isize);
    b = *shapes.offset(1 as libc::c_int as isize);
    let mut current_block_12: u64;
    if a as libc::c_ulong == (*arb).a as libc::c_ulong {
        if b as libc::c_ulong == (*arb).b as libc::c_ulong {
            tmp = 1 as libc::c_int;
            current_block_12 = 10048703153582371463;
        } else {
            current_block_12 = 16837027526642580192;
        }
    } else {
        current_block_12 = 16837027526642580192;
    }
    match current_block_12 {
        16837027526642580192 => {
            if b as libc::c_ulong == (*arb).a as libc::c_ulong {
                if a as libc::c_ulong == (*arb).b as libc::c_ulong {
                    tmp = 1 as libc::c_int;
                } else {
                    tmp = 0 as libc::c_int;
                }
            } else {
                tmp = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    return tmp as cpBool;
}
unsafe extern "C" fn handlerSetEql(
    mut check: *mut cpCollisionHandler,
    mut pair: *mut cpCollisionHandler,
) -> cpBool {
    let mut tmp: libc::c_int = 0;
    let mut current_block_10: u64;
    if (*check).typeA == (*pair).typeA {
        if (*check).typeB == (*pair).typeB {
            tmp = 1 as libc::c_int;
            current_block_10 = 1856101646708284338;
        } else {
            current_block_10 = 9796624928983172044;
        }
    } else {
        current_block_10 = 9796624928983172044;
    }
    match current_block_10 {
        9796624928983172044 => {
            if (*check).typeB == (*pair).typeA {
                if (*check).typeA == (*pair).typeB {
                    tmp = 1 as libc::c_int;
                } else {
                    tmp = 0 as libc::c_int;
                }
            } else {
                tmp = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    return tmp as cpBool;
}
unsafe extern "C" fn handlerSetTrans(
    mut handler: *mut cpCollisionHandler,
    mut unused: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut copy: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpCollisionHandler>() as libc::c_ulong,
    );
    copy = tmp as *mut cpCollisionHandler;
    memcpy(
        copy as *mut libc::c_void,
        handler as *const libc::c_void,
        ::std::mem::size_of::<cpCollisionHandler>() as libc::c_ulong,
    );
    return copy as *mut libc::c_void;
}
unsafe extern "C" fn DefaultBegin(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) -> cpBool {
    let mut retA: cpBool = 0;
    let mut tmp: cpBool = 0;
    let mut retB: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = cpArbiterCallWildcardBeginA(arb, space);
    retA = tmp;
    tmp___0 = cpArbiterCallWildcardBeginB(arb, space);
    retB = tmp___0;
    if retA != 0 {
        if retB != 0 {
            tmp___1 = 1 as libc::c_int;
        } else {
            tmp___1 = 0 as libc::c_int;
        }
    } else {
        tmp___1 = 0 as libc::c_int;
    }
    return tmp___1 as cpBool;
}
unsafe extern "C" fn DefaultPreSolve(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) -> cpBool {
    let mut retA: cpBool = 0;
    let mut tmp: cpBool = 0;
    let mut retB: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    let mut tmp___1: libc::c_int = 0;
    tmp = cpArbiterCallWildcardPreSolveA(arb, space);
    retA = tmp;
    tmp___0 = cpArbiterCallWildcardPreSolveB(arb, space);
    retB = tmp___0;
    if retA != 0 {
        if retB != 0 {
            tmp___1 = 1 as libc::c_int;
        } else {
            tmp___1 = 0 as libc::c_int;
        }
    } else {
        tmp___1 = 0 as libc::c_int;
    }
    return tmp___1 as cpBool;
}
unsafe extern "C" fn DefaultPostSolve(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) {
    cpArbiterCallWildcardPostSolveA(arb, space);
    cpArbiterCallWildcardPostSolveB(arb, space);
}
unsafe extern "C" fn DefaultSeparate(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) {
    cpArbiterCallWildcardSeparateA(arb, space);
    cpArbiterCallWildcardSeparateB(arb, space);
}
static mut cpCollisionHandlerDefault: cpCollisionHandler = {
    let mut init = cpCollisionHandler {
        typeA: !(0 as libc::c_int as cpCollisionType),
        typeB: !(0 as libc::c_int as cpCollisionType),
        beginFunc: Some(
            DefaultBegin
                as unsafe extern "C" fn(
                    *mut cpArbiter,
                    *mut cpSpace,
                    cpDataPointer,
                ) -> cpBool,
        ),
        preSolveFunc: Some(
            DefaultPreSolve
                as unsafe extern "C" fn(
                    *mut cpArbiter,
                    *mut cpSpace,
                    cpDataPointer,
                ) -> cpBool,
        ),
        postSolveFunc: Some(
            DefaultPostSolve
                as unsafe extern "C" fn(
                    *mut cpArbiter,
                    *mut cpSpace,
                    cpDataPointer,
                ) -> (),
        ),
        separateFunc: Some(
            DefaultSeparate
                as unsafe extern "C" fn(
                    *mut cpArbiter,
                    *mut cpSpace,
                    cpDataPointer,
                ) -> (),
        ),
        userData: 0 as *const libc::c_void as *mut libc::c_void,
    };
    init
};
unsafe extern "C" fn AlwaysCollide(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) -> cpBool {
    return 1 as libc::c_int as cpBool;
}
unsafe extern "C" fn DoNothing(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
    mut data: cpDataPointer,
) {}
pub static mut cpCollisionHandlerDoNothing: cpCollisionHandler = {
    let mut init = cpCollisionHandler {
        typeA: !(0 as libc::c_int as cpCollisionType),
        typeB: !(0 as libc::c_int as cpCollisionType),
        beginFunc: Some(
            AlwaysCollide
                as unsafe extern "C" fn(
                    *mut cpArbiter,
                    *mut cpSpace,
                    cpDataPointer,
                ) -> cpBool,
        ),
        preSolveFunc: Some(
            AlwaysCollide
                as unsafe extern "C" fn(
                    *mut cpArbiter,
                    *mut cpSpace,
                    cpDataPointer,
                ) -> cpBool,
        ),
        postSolveFunc: Some(
            DoNothing
                as unsafe extern "C" fn(
                    *mut cpArbiter,
                    *mut cpSpace,
                    cpDataPointer,
                ) -> (),
        ),
        separateFunc: Some(
            DoNothing
                as unsafe extern "C" fn(
                    *mut cpArbiter,
                    *mut cpSpace,
                    cpDataPointer,
                ) -> (),
        ),
        userData: 0 as *const libc::c_void as *mut libc::c_void,
    };
    init
};
unsafe extern "C" fn ShapeVelocityFunc(mut shape: *mut cpShape) -> cpVect {
    return (*(*shape).body).v;
}
unsafe extern "C" fn FreeWrap(
    mut ptr: *mut libc::c_void,
    mut unused: *mut libc::c_void,
) {
    free(ptr);
}
pub unsafe extern "C" fn cpSpaceAlloc() -> *mut cpSpace {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpSpace>() as libc::c_ulong,
    );
    return tmp as *mut cpSpace;
}
pub unsafe extern "C" fn cpSpaceInit(mut space: *mut cpSpace) -> *mut cpSpace {
    let mut tmp: libc::c_float = 0.;
    let mut staticBody: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___0: *mut cpBody = 0 as *mut cpBody;
    (*space).iterations = 10 as libc::c_int;
    (*space).gravity = cpvzero___8;
    (*space).damping = 1.0f32 as cpFloat;
    (*space).collisionSlop = 0.1f32 as cpFloat;
    (*space)
        .collisionBias = pow(
        (1.0f32 - 0.1f32) as libc::c_double,
        60.0f32 as libc::c_double,
    );
    (*space).collisionPersistence = 3 as libc::c_int as cpTimestamp;
    (*space).locked = 0 as libc::c_int;
    (*space).stamp = 0 as libc::c_int as cpTimestamp;
    (*space).shapeIDCounter = 0 as libc::c_int as cpHashValue;
    (*space)
        .staticShapes = cpBBTreeNew(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*const cpShape) -> cpBB>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
        >(Some(cpShapeGetBB as unsafe extern "C" fn(*const cpShape) -> cpBB)),
        0 as *mut libc::c_void as *mut cpSpatialIndex,
    );
    (*space)
        .dynamicShapes = cpBBTreeNew(
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*const cpShape) -> cpBB>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
        >(Some(cpShapeGetBB as unsafe extern "C" fn(*const cpShape) -> cpBB)),
        (*space).staticShapes,
    );
    cpBBTreeSetVelocityFunc(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape) -> cpVect>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpVect>,
        >(Some(ShapeVelocityFunc as unsafe extern "C" fn(*mut cpShape) -> cpVect)),
    );
    (*space).allocatedBuffers = cpArrayNew(0 as libc::c_int);
    (*space).dynamicBodies = cpArrayNew(0 as libc::c_int);
    (*space).staticBodies = cpArrayNew(0 as libc::c_int);
    (*space).sleepingComponents = cpArrayNew(0 as libc::c_int);
    (*space).rousedBodies = cpArrayNew(0 as libc::c_int);
    tmp = ::std::f32::INFINITY;
    (*space).sleepTimeThreshold = tmp as cpFloat;
    (*space).idleSpeedThreshold = 0.0f32 as cpFloat;
    (*space).arbiters = cpArrayNew(0 as libc::c_int);
    (*space).pooledArbiters = cpArrayNew(0 as libc::c_int);
    (*space).contactBuffersHead = 0 as *mut libc::c_void as *mut cpContactBufferHeader;
    (*space)
        .cachedArbiters = cpHashSetNew(
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut *mut cpShape, *mut cpArbiter) -> cpBool>,
            Option::<
                unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cpBool,
            >,
        >(
            Some(
                arbiterSetEql
                    as unsafe extern "C" fn(*mut *mut cpShape, *mut cpArbiter) -> cpBool,
            ),
        ),
    );
    (*space).constraints = cpArrayNew(0 as libc::c_int);
    (*space).usesWildcards = 0 as libc::c_int as cpBool;
    memcpy(
        &mut (*space).defaultHandler as *mut cpCollisionHandler as *mut libc::c_void,
        &mut cpCollisionHandlerDoNothing as *mut cpCollisionHandler
            as *const libc::c_void,
        ::std::mem::size_of::<cpCollisionHandler>() as libc::c_ulong,
    );
    (*space)
        .collisionHandlers = cpHashSetNew(
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpCollisionHandler,
                    *mut cpCollisionHandler,
                ) -> cpBool,
            >,
            Option::<
                unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cpBool,
            >,
        >(
            Some(
                handlerSetEql
                    as unsafe extern "C" fn(
                        *mut cpCollisionHandler,
                        *mut cpCollisionHandler,
                    ) -> cpBool,
            ),
        ),
    );
    (*space).postStepCallbacks = cpArrayNew(0 as libc::c_int);
    (*space).skipPostStep = 0 as libc::c_int as cpBool;
    tmp___0 = cpBodyInit(
        &mut (*space)._staticBody,
        0.0f32 as cpFloat,
        0.0f32 as cpFloat,
    );
    staticBody = tmp___0;
    cpBodySetType(staticBody, CP_BODY_TYPE_STATIC);
    cpSpaceSetStaticBody(space, staticBody);
    return space;
}
pub unsafe extern "C" fn cpSpaceNew() -> *mut cpSpace {
    let mut tmp: *mut cpSpace = 0 as *mut cpSpace;
    let mut tmp___0: *mut cpSpace = 0 as *mut cpSpace;
    tmp = cpSpaceAlloc();
    tmp___0 = cpSpaceInit(tmp);
    return tmp___0;
}
unsafe extern "C" fn cpBodyActivateWrap(
    mut body: *mut cpBody,
    mut unused: *mut libc::c_void,
) {
    cpBodyActivate(body);
}
pub unsafe extern "C" fn cpSpaceDestroy(mut space: *mut cpSpace) {
    cpSpaceEachBody(
        space,
        Some(
            cpBodyActivateWrap
                as unsafe extern "C" fn(*mut cpBody, *mut libc::c_void) -> (),
        ),
        0 as *mut libc::c_void,
    );
    cpSpatialIndexFree((*space).staticShapes);
    cpSpatialIndexFree((*space).dynamicShapes);
    cpArrayFree((*space).dynamicBodies);
    cpArrayFree((*space).staticBodies);
    cpArrayFree((*space).sleepingComponents);
    cpArrayFree((*space).rousedBodies);
    cpArrayFree((*space).constraints);
    cpHashSetFree((*space).cachedArbiters);
    cpArrayFree((*space).arbiters);
    cpArrayFree((*space).pooledArbiters);
    if !((*space).allocatedBuffers).is_null() {
        cpArrayFreeEach(
            (*space).allocatedBuffers,
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        cpArrayFree((*space).allocatedBuffers);
    }
    if !((*space).postStepCallbacks).is_null() {
        cpArrayFreeEach(
            (*space).postStepCallbacks,
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        cpArrayFree((*space).postStepCallbacks);
    }
    if !((*space).collisionHandlers).is_null() {
        cpHashSetEach(
            (*space).collisionHandlers,
            Some(
                FreeWrap
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
            0 as *mut libc::c_void,
        );
    }
    cpHashSetFree((*space).collisionHandlers);
}
pub unsafe extern "C" fn cpSpaceFree(mut space: *mut cpSpace) {
    if !space.is_null() {
        cpSpaceDestroy(space);
        free(space as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpSpaceGetIterations(mut space: *const cpSpace) -> libc::c_int {
    return (*space).iterations;
}
pub unsafe extern "C" fn cpSpaceSetIterations(
    mut space: *mut cpSpace,
    mut iterations: libc::c_int,
) {
    if !(iterations > 0 as libc::c_int) {
        cpMessage(
            b"iterations > 0\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            243 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Iterations must be positive and non-zero.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    (*space).iterations = iterations;
}
pub unsafe extern "C" fn cpSpaceGetGravity(mut space: *const cpSpace) -> cpVect {
    return (*space).gravity;
}
pub unsafe extern "C" fn cpSpaceSetGravity(
    mut space: *mut cpSpace,
    mut gravity: cpVect,
) {
    let mut components: *mut cpArray = 0 as *mut cpArray;
    let mut i: libc::c_int = 0;
    (*space).gravity = gravity;
    components = (*space).sleepingComponents;
    i = 0 as libc::c_int;
    while i < (*components).num {
        cpBodyActivate(*((*components).arr).offset(i as isize) as *mut cpBody);
        i += 1;
    }
}
pub unsafe extern "C" fn cpSpaceGetDamping(mut space: *const cpSpace) -> cpFloat {
    return (*space).damping;
}
pub unsafe extern "C" fn cpSpaceSetDamping(
    mut space: *mut cpSpace,
    mut damping: cpFloat,
) {
    if !(damping >= 0.0f64) {
        cpMessage(
            b"damping >= 0.0\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            274 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Damping must be positive.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    (*space).damping = damping;
}
pub unsafe extern "C" fn cpSpaceGetIdleSpeedThreshold(
    mut space: *const cpSpace,
) -> cpFloat {
    return (*space).idleSpeedThreshold;
}
pub unsafe extern "C" fn cpSpaceSetIdleSpeedThreshold(
    mut space: *mut cpSpace,
    mut idleSpeedThreshold: cpFloat,
) {
    (*space).idleSpeedThreshold = idleSpeedThreshold;
}
pub unsafe extern "C" fn cpSpaceGetSleepTimeThreshold(
    mut space: *const cpSpace,
) -> cpFloat {
    return (*space).sleepTimeThreshold;
}
pub unsafe extern "C" fn cpSpaceSetSleepTimeThreshold(
    mut space: *mut cpSpace,
    mut sleepTimeThreshold: cpFloat,
) {
    (*space).sleepTimeThreshold = sleepTimeThreshold;
}
pub unsafe extern "C" fn cpSpaceGetCollisionSlop(mut space: *const cpSpace) -> cpFloat {
    return (*space).collisionSlop;
}
pub unsafe extern "C" fn cpSpaceSetCollisionSlop(
    mut space: *mut cpSpace,
    mut collisionSlop: cpFloat,
) {
    (*space).collisionSlop = collisionSlop;
}
pub unsafe extern "C" fn cpSpaceGetCollisionBias(mut space: *const cpSpace) -> cpFloat {
    return (*space).collisionBias;
}
pub unsafe extern "C" fn cpSpaceSetCollisionBias(
    mut space: *mut cpSpace,
    mut collisionBias: cpFloat,
) {
    (*space).collisionBias = collisionBias;
}
pub unsafe extern "C" fn cpSpaceGetCollisionPersistence(
    mut space: *const cpSpace,
) -> cpTimestamp {
    return (*space).collisionPersistence;
}
pub unsafe extern "C" fn cpSpaceSetCollisionPersistence(
    mut space: *mut cpSpace,
    mut collisionPersistence: cpTimestamp,
) {
    (*space).collisionPersistence = collisionPersistence;
}
pub unsafe extern "C" fn cpSpaceGetUserData(mut space: *const cpSpace) -> cpDataPointer {
    return (*space).userData;
}
pub unsafe extern "C" fn cpSpaceSetUserData(
    mut space: *mut cpSpace,
    mut userData: cpDataPointer,
) {
    (*space).userData = userData;
}
pub unsafe extern "C" fn cpSpaceGetStaticBody(mut space: *const cpSpace) -> *mut cpBody {
    return (*space).staticBody;
}
pub unsafe extern "C" fn cpSpaceGetCurrentTimeStep(
    mut space: *const cpSpace,
) -> cpFloat {
    return (*space).curr_dt;
}
pub unsafe extern "C" fn cpSpaceSetStaticBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) {
    if (*space).staticBody as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if !((*(*space).staticBody).shapeList as libc::c_ulong
            == 0 as *mut libc::c_void as libc::c_ulong)
        {
            cpMessage(
                b"space->staticBody->shapeList == NULL\0" as *const u8
                    as *const libc::c_char,
                b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
                366 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Changing the designated static body while the old one still had shapes attached.\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
        (*(*space).staticBody).space = 0 as *mut libc::c_void as *mut cpSpace;
    }
    (*space).staticBody = body;
    (*body).space = space;
}
pub unsafe extern "C" fn cpSpaceIsLocked(mut space: *mut cpSpace) -> cpBool {
    return ((*space).locked > 0 as libc::c_int) as libc::c_int as cpBool;
}
unsafe extern "C" fn cpSpaceUseWildcardDefaultHandler(mut space: *mut cpSpace) {
    if (*space).usesWildcards == 0 {
        (*space).usesWildcards = 1 as libc::c_int as cpBool;
        memcpy(
            &mut (*space).defaultHandler as *mut cpCollisionHandler as *mut libc::c_void,
            &mut cpCollisionHandlerDefault as *mut cpCollisionHandler
                as *const libc::c_void,
            ::std::mem::size_of::<cpCollisionHandler>() as libc::c_ulong,
        );
    }
}
pub unsafe extern "C" fn cpSpaceAddDefaultCollisionHandler(
    mut space: *mut cpSpace,
) -> *mut cpCollisionHandler {
    cpSpaceUseWildcardDefaultHandler(space);
    return &mut (*space).defaultHandler;
}
pub unsafe extern "C" fn cpSpaceAddCollisionHandler(
    mut space: *mut cpSpace,
    mut a: cpCollisionType,
    mut b: cpCollisionType,
) -> *mut cpCollisionHandler {
    let mut hash: cpHashValue = 0;
    let mut handler: cpCollisionHandler = cpCollisionHandler {
        typeA: 0,
        typeB: 0,
        beginFunc: None,
        preSolveFunc: None,
        postSolveFunc: None,
        separateFunc: None,
        userData: 0 as *mut libc::c_void,
    };
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    hash = a.wrapping_mul(3344921057 as libc::c_ulong)
        ^ b.wrapping_mul(3344921057 as libc::c_ulong);
    handler
        .beginFunc = Some(
        DefaultBegin
            as unsafe extern "C" fn(
                *mut cpArbiter,
                *mut cpSpace,
                cpDataPointer,
            ) -> cpBool,
    );
    handler
        .preSolveFunc = Some(
        DefaultPreSolve
            as unsafe extern "C" fn(
                *mut cpArbiter,
                *mut cpSpace,
                cpDataPointer,
            ) -> cpBool,
    );
    handler
        .postSolveFunc = Some(
        DefaultPostSolve
            as unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> (),
    );
    handler
        .separateFunc = Some(
        DefaultSeparate
            as unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> (),
    );
    handler.userData = 0 as *mut libc::c_void;
    tmp = cpHashSetInsert(
        (*space).collisionHandlers,
        hash,
        &mut handler as *mut cpCollisionHandler as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpCollisionHandler,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
        >(
            Some(
                handlerSetTrans
                    as unsafe extern "C" fn(
                        *mut cpCollisionHandler,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
        ),
        0 as *mut libc::c_void,
    );
    return tmp as *mut cpCollisionHandler;
}
pub unsafe extern "C" fn cpSpaceAddWildcardHandler(
    mut space: *mut cpSpace,
    mut type_0: cpCollisionType,
) -> *mut cpCollisionHandler {
    let mut hash: cpHashValue = 0;
    let mut handler: cpCollisionHandler = cpCollisionHandler {
        typeA: 0,
        typeB: 0,
        beginFunc: None,
        preSolveFunc: None,
        postSolveFunc: None,
        separateFunc: None,
        userData: 0 as *mut libc::c_void,
    };
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    cpSpaceUseWildcardDefaultHandler(space);
    hash = (type_0.wrapping_mul(3344921057 as libc::c_ulong) as libc::c_ulonglong
        ^ 18446744070364630559 as libc::c_ulonglong) as cpHashValue;
    handler
        .beginFunc = Some(
        AlwaysCollide
            as unsafe extern "C" fn(
                *mut cpArbiter,
                *mut cpSpace,
                cpDataPointer,
            ) -> cpBool,
    );
    handler
        .preSolveFunc = Some(
        AlwaysCollide
            as unsafe extern "C" fn(
                *mut cpArbiter,
                *mut cpSpace,
                cpDataPointer,
            ) -> cpBool,
    );
    handler
        .postSolveFunc = Some(
        DoNothing
            as unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> (),
    );
    handler
        .separateFunc = Some(
        DoNothing
            as unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace, cpDataPointer) -> (),
    );
    handler.userData = 0 as *mut libc::c_void;
    tmp = cpHashSetInsert(
        (*space).collisionHandlers,
        hash,
        &mut handler as *mut cpCollisionHandler as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpCollisionHandler,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
        >(
            Some(
                handlerSetTrans
                    as unsafe extern "C" fn(
                        *mut cpCollisionHandler,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
        ),
        0 as *mut libc::c_void,
    );
    return tmp as *mut cpCollisionHandler;
}
pub unsafe extern "C" fn cpSpaceAddShape(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
) -> *mut cpShape {
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut isStatic: cpBool = 0;
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___0: cpHashValue = 0;
    let mut tmp___1: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    if !((*shape).space as libc::c_ulong != space as libc::c_ulong) {
        cpMessage(
            b"shape->space != space\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            420 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this shape to this space. You must not add it a second time.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*shape).space).is_null() {
        cpMessage(
            b"!shape->space\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this shape to another space. You cannot add it to a second.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if ((*shape).body).is_null() {
        cpMessage(
            b"shape->body\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            422 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"The shape's body is not defined.\0" as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*(*shape).body).space as libc::c_ulong == space as libc::c_ulong) {
        cpMessage(
            b"shape->body->space == space\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            423 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"The shape's body must be added to the space before the shape.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            424 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    body = (*shape).body;
    tmp = cpBodyGetType(body);
    isStatic = (tmp as libc::c_uint == 2 as libc::c_uint) as libc::c_int as cpBool;
    if isStatic == 0 {
        cpBodyActivate(body);
    }
    cpBodyAddShape(body, shape);
    tmp___0 = (*space).shapeIDCounter;
    (*space).shapeIDCounter = ((*space).shapeIDCounter).wrapping_add(1);
    (*shape).hashid = tmp___0;
    cpShapeUpdate(shape, (*body).transform);
    if isStatic != 0 {
        tmp___1 = (*space).staticShapes;
    } else {
        tmp___1 = (*space).dynamicShapes;
    }
    cpSpatialIndexInsert(tmp___1, shape as *mut libc::c_void, (*shape).hashid);
    (*shape).space = space;
    return shape;
}
pub unsafe extern "C" fn cpSpaceAddBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) -> *mut cpBody {
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___0: *mut cpArray = 0 as *mut cpArray;
    if !((*body).space as libc::c_ulong != space as libc::c_ulong) {
        cpMessage(
            b"body->space != space\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            443 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this body to this space. You must not add it a second time.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*body).space).is_null() {
        cpMessage(
            b"!body->space\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            444 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this body to another space. You cannot add it to a second.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            445 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    tmp = cpBodyGetType(body);
    tmp___0 = cpSpaceArrayForBodyType(space, tmp);
    cpArrayPush(tmp___0, body as *mut libc::c_void);
    (*body).space = space;
    return body;
}
pub unsafe extern "C" fn cpSpaceAddConstraint(
    mut space: *mut cpSpace,
    mut constraint: *mut cpConstraint,
) -> *mut cpConstraint {
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    if !((*constraint).space as libc::c_ulong != space as libc::c_ulong) {
        cpMessage(
            b"constraint->space != space\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            456 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this constraint to this space. You must not add it a second time.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !((*constraint).space).is_null() {
        cpMessage(
            b"!constraint->space\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            457 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You have already added this constraint to another space. You cannot add it to a second.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            458 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    a = (*constraint).a;
    b = (*constraint).b;
    if a as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        if !(b as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            cpMessage(
                b"a != NULL && b != NULL\0" as *const u8 as *const libc::c_char,
                b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
                461 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Constraint is attached to a NULL body.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
    } else {
        cpMessage(
            b"a != NULL && b != NULL\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            461 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Constraint is attached to a NULL body.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate(a);
    cpBodyActivate(b);
    cpArrayPush((*space).constraints, constraint as *mut libc::c_void);
    (*constraint).next_a = (*a).constraintList;
    (*a).constraintList = constraint;
    (*constraint).next_b = (*b).constraintList;
    (*b).constraintList = constraint;
    (*constraint).space = space;
    return constraint;
}
unsafe extern "C" fn cachedArbitersFilter(
    mut arb: *mut cpArbiter,
    mut context: *mut arbiterFilterContext,
) -> cpBool {
    let mut current_block: u64;
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    shape = (*context).shape;
    body = (*context).body;
    if body as libc::c_ulong == (*arb).body_a as libc::c_ulong {
        if shape as libc::c_ulong == (*arb).a as libc::c_ulong {
            current_block = 13064796872413869620;
        } else if shape as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            current_block = 13064796872413869620;
        } else {
            current_block = 11013350028856247212;
        }
    } else {
        current_block = 11013350028856247212;
    }
    match current_block {
        11013350028856247212 => {
            if body as libc::c_ulong == (*arb).body_b as libc::c_ulong {
                if shape as libc::c_ulong == (*arb).b as libc::c_ulong {
                    current_block = 13064796872413869620;
                } else if shape as libc::c_ulong
                        == 0 as *mut libc::c_void as libc::c_ulong
                    {
                    current_block = 13064796872413869620;
                } else {
                    current_block = 26972500619410423;
                }
            } else {
                current_block = 26972500619410423;
            }
            match current_block {
                13064796872413869620 => {}
                _ => return 1 as libc::c_int as cpBool,
            }
        }
        _ => {}
    }
    if !shape.is_null() {
        if (*arb).state as libc::c_uint != 3 as libc::c_uint {
            (*arb).state = CP_ARBITER_STATE_INVALIDATED;
            handler = (*arb).handler;
            (Some(((*handler).separateFunc).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(arb, (*context).space, (*handler).userData);
        }
    }
    cpArbiterUnthread(arb);
    cpArrayDeleteObj((*(*context).space).arbiters, arb as *mut libc::c_void);
    cpArrayPush((*(*context).space).pooledArbiters, arb as *mut libc::c_void);
    return 0 as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpSpaceFilterArbiters(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
    mut filter: *mut cpShape,
) {
    let mut context: arbiterFilterContext = arbiterFilterContext {
        space: 0 as *mut cpSpace,
        body: 0 as *mut cpBody,
        shape: 0 as *mut cpShape,
    };
    cpSpaceLock(space);
    context.space = space;
    context.body = body;
    context.shape = filter;
    cpHashSetFilter(
        (*space).cachedArbiters,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut cpArbiter, *mut arbiterFilterContext) -> cpBool,
            >,
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> cpBool,
            >,
        >(
            Some(
                cachedArbitersFilter
                    as unsafe extern "C" fn(
                        *mut cpArbiter,
                        *mut arbiterFilterContext,
                    ) -> cpBool,
            ),
        ),
        &mut context as *mut arbiterFilterContext as *mut libc::c_void,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
pub unsafe extern "C" fn cpSpaceRemoveShape(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
) {
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: cpBool = 0;
    let mut isStatic: cpBool = 0;
    let mut tmp___0: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___1: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    body = (*shape).body;
    tmp = cpSpaceContainsShape(space, shape);
    if tmp == 0 {
        cpMessage(
            b"cpSpaceContainsShape(space, shape)\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            526 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Cannot remove a shape that was not added to the space. (Removed twice maybe?)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            527 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    tmp___0 = cpBodyGetType(body);
    isStatic = (tmp___0 as libc::c_uint == 2 as libc::c_uint) as libc::c_int as cpBool;
    if isStatic != 0 {
        cpBodyActivateStatic(body, shape);
    } else {
        cpBodyActivate(body);
    }
    cpBodyRemoveShape(body, shape);
    cpSpaceFilterArbiters(space, body, shape);
    if isStatic != 0 {
        tmp___1 = (*space).staticShapes;
    } else {
        tmp___1 = (*space).dynamicShapes;
    }
    cpSpatialIndexRemove(tmp___1, shape as *mut libc::c_void, (*shape).hashid);
    (*shape).space = 0 as *mut libc::c_void as *mut cpSpace;
    (*shape).hashid = 0 as libc::c_int as cpHashValue;
}
pub unsafe extern "C" fn cpSpaceRemoveBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) {
    let mut tmp: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___0: cpBool = 0;
    let mut tmp___1: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___2: *mut cpArray = 0 as *mut cpArray;
    tmp = cpSpaceGetStaticBody(space as *const cpSpace);
    if !(body as libc::c_ulong != tmp as libc::c_ulong) {
        cpMessage(
            b"body != cpSpaceGetStaticBody(space)\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            546 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Cannot remove the designated static body for the space.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    tmp___0 = cpSpaceContainsBody(space, body);
    if tmp___0 == 0 {
        cpMessage(
            b"cpSpaceContainsBody(space, body)\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            547 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Cannot remove a body that was not added to the space. (Removed twice maybe?)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            550 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate(body);
    tmp___1 = cpBodyGetType(body);
    tmp___2 = cpSpaceArrayForBodyType(space, tmp___1);
    cpArrayDeleteObj(tmp___2, body as *mut libc::c_void);
    (*body).space = 0 as *mut libc::c_void as *mut cpSpace;
}
pub unsafe extern "C" fn cpSpaceRemoveConstraint(
    mut space: *mut cpSpace,
    mut constraint: *mut cpConstraint,
) {
    let mut tmp: cpBool = 0;
    tmp = cpSpaceContainsConstraint(space, constraint);
    if tmp == 0 {
        cpMessage(
            b"cpSpaceContainsConstraint(space, constraint)\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            561 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Cannot remove a constraint that was not added to the space. (Removed twice maybe?)\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            562 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"This operation cannot be done safely during a call to cpSpaceStep() or during a query. Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpBodyActivate((*constraint).a);
    cpBodyActivate((*constraint).b);
    cpArrayDeleteObj((*space).constraints, constraint as *mut libc::c_void);
    cpBodyRemoveConstraint((*constraint).a, constraint);
    cpBodyRemoveConstraint((*constraint).b, constraint);
    (*constraint).space = 0 as *mut libc::c_void as *mut cpSpace;
}
pub unsafe extern "C" fn cpSpaceContainsShape(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
) -> cpBool {
    return ((*shape).space as libc::c_ulong == space as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpSpaceContainsBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) -> cpBool {
    return ((*body).space as libc::c_ulong == space as libc::c_ulong) as libc::c_int
        as cpBool;
}
pub unsafe extern "C" fn cpSpaceContainsConstraint(
    mut space: *mut cpSpace,
    mut constraint: *mut cpConstraint,
) -> cpBool {
    return ((*constraint).space as libc::c_ulong == space as libc::c_ulong)
        as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpSpaceEachBody(
    mut space: *mut cpSpace,
    mut func: Option::<unsafe extern "C" fn(*mut cpBody, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    let mut bodies: *mut cpArray = 0 as *mut cpArray;
    let mut i: libc::c_int = 0;
    let mut otherBodies: *mut cpArray = 0 as *mut cpArray;
    let mut i___0: libc::c_int = 0;
    let mut components: *mut cpArray = 0 as *mut cpArray;
    let mut i___1: libc::c_int = 0;
    let mut root: *mut cpBody = 0 as *mut cpBody;
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut next: *mut cpBody = 0 as *mut cpBody;
    cpSpaceLock(space);
    bodies = (*space).dynamicBodies;
    i = 0 as libc::c_int;
    while i < (*bodies).num {
        (Some(func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(*((*bodies).arr).offset(i as isize) as *mut cpBody, data);
        i += 1;
    }
    otherBodies = (*space).staticBodies;
    i___0 = 0 as libc::c_int;
    while i___0 < (*otherBodies).num {
        (Some(func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(*((*otherBodies).arr).offset(i___0 as isize) as *mut cpBody, data);
        i___0 += 1;
    }
    components = (*space).sleepingComponents;
    i___1 = 0 as libc::c_int;
    while i___1 < (*components).num {
        root = *((*components).arr).offset(i___1 as isize) as *mut cpBody;
        body = root;
        while !body.is_null() {
            next = (*body).sleeping.next;
            (Some(func.expect("non-null function pointer")))
                .expect("non-null function pointer")(body, data);
            body = next;
        }
        i___1 += 1;
    }
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
unsafe extern "C" fn spaceEachShapeIterator(
    mut shape: *mut cpShape,
    mut context: *mut spaceShapeContext,
) {
    (Some(((*context).func).expect("non-null function pointer")))
        .expect("non-null function pointer")(shape, (*context).data);
}
pub unsafe extern "C" fn cpSpaceEachShape(
    mut space: *mut cpSpace,
    mut func: Option::<unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    let mut context: spaceShapeContext = spaceShapeContext {
        func: None,
        data: 0 as *mut libc::c_void,
    };
    cpSpaceLock(space);
    context.func = func;
    context.data = data;
    cpSpatialIndexEach(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut spaceShapeContext) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                spaceEachShapeIterator
                    as unsafe extern "C" fn(*mut cpShape, *mut spaceShapeContext) -> (),
            ),
        ),
        &mut context as *mut spaceShapeContext as *mut libc::c_void,
    );
    cpSpatialIndexEach(
        (*space).staticShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut spaceShapeContext) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                spaceEachShapeIterator
                    as unsafe extern "C" fn(*mut cpShape, *mut spaceShapeContext) -> (),
            ),
        ),
        &mut context as *mut spaceShapeContext as *mut libc::c_void,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
pub unsafe extern "C" fn cpSpaceEachConstraint(
    mut space: *mut cpSpace,
    mut func: Option::<unsafe extern "C" fn(*mut cpConstraint, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    let mut constraints: *mut cpArray = 0 as *mut cpArray;
    let mut i: libc::c_int = 0;
    cpSpaceLock(space);
    constraints = (*space).constraints;
    i = 0 as libc::c_int;
    while i < (*constraints).num {
        (Some(func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(*((*constraints).arr).offset(i as isize) as *mut cpConstraint, data);
        i += 1;
    }
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
pub unsafe extern "C" fn cpSpaceReindexStatic(mut space: *mut cpSpace) {
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            656 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You cannot manually reindex objects while the space is locked. Wait until the current query or step is complete.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpSpatialIndexEach(
        (*space).staticShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                cpShapeUpdateFunc
                    as unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> (),
            ),
        ),
        0 as *mut libc::c_void,
    );
    cpSpatialIndexReindex((*space).staticShapes);
}
pub unsafe extern "C" fn cpSpaceReindexShape(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
) {
    if (*space).locked != 0 {
        cpMessage(
            b"!space->locked\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpace.c\0" as *const u8 as *const libc::c_char,
            665 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"You cannot manually reindex objects while the space is locked. Wait until the current query or step is complete.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpShapeCacheBB(shape);
    cpSpatialIndexReindexObject(
        (*space).dynamicShapes,
        shape as *mut libc::c_void,
        (*shape).hashid,
    );
    cpSpatialIndexReindexObject(
        (*space).staticShapes,
        shape as *mut libc::c_void,
        (*shape).hashid,
    );
}
pub unsafe extern "C" fn cpSpaceReindexShapesForBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) {
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    shape = (*body).shapeList;
    while !shape.is_null() {
        cpSpaceReindexShape(space, shape);
        shape = (*shape).next;
    }
}
unsafe extern "C" fn copyShapes(
    mut shape: *mut cpShape,
    mut index___0: *mut cpSpatialIndex,
) {
    cpSpatialIndexInsert(index___0, shape as *mut libc::c_void, (*shape).hashid);
}
pub unsafe extern "C" fn cpSpaceUseSpatialHash(
    mut space: *mut cpSpace,
    mut dim: cpFloat,
    mut count: libc::c_int,
) {
    let mut staticShapes: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    let mut tmp: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    let mut dynamicShapes: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    let mut tmp___0: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    tmp = cpSpaceHashNew(
        dim,
        count,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*const cpShape) -> cpBB>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
        >(Some(cpShapeGetBB as unsafe extern "C" fn(*const cpShape) -> cpBB)),
        0 as *mut libc::c_void as *mut cpSpatialIndex,
    );
    staticShapes = tmp;
    tmp___0 = cpSpaceHashNew(
        dim,
        count,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*const cpShape) -> cpBB>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
        >(Some(cpShapeGetBB as unsafe extern "C" fn(*const cpShape) -> cpBB)),
        staticShapes,
    );
    dynamicShapes = tmp___0;
    cpSpatialIndexEach(
        (*space).staticShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut cpSpatialIndex) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                copyShapes
                    as unsafe extern "C" fn(*mut cpShape, *mut cpSpatialIndex) -> (),
            ),
        ),
        staticShapes as *mut libc::c_void,
    );
    cpSpatialIndexEach(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut cpSpatialIndex) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                copyShapes
                    as unsafe extern "C" fn(*mut cpShape, *mut cpSpatialIndex) -> (),
            ),
        ),
        dynamicShapes as *mut libc::c_void,
    );
    cpSpatialIndexFree((*space).staticShapes);
    cpSpatialIndexFree((*space).dynamicShapes);
    (*space).staticShapes = staticShapes;
    (*space).dynamicShapes = dynamicShapes;
}
#[inline]
unsafe extern "C" fn cpSpaceUncacheArbiter(
    mut space: *mut cpSpace,
    mut arb: *mut cpArbiter,
) {
    let mut a: *const cpShape = 0 as *const cpShape;
    let mut b: *const cpShape = 0 as *const cpShape;
    let mut shape_pair: [*const cpShape; 2] = [0 as *const cpShape; 2];
    let mut arbHashID: cpHashValue = 0;
    a = (*arb).a;
    b = (*arb).b;
    shape_pair[0 as libc::c_int as usize] = a;
    shape_pair[1 as libc::c_int as usize] = b;
    arbHashID = (a as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong)
        ^ (b as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong);
    cpHashSetRemove(
        (*space).cachedArbiters,
        arbHashID,
        shape_pair.as_mut_ptr() as *const libc::c_void,
    );
    cpArrayDeleteObj((*space).arbiters, arb as *mut libc::c_void);
}
pub unsafe extern "C" fn cpSpaceActivateBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) {
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___0: cpBool = 0;
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut bodyA: *mut cpBody = 0 as *mut cpBody;
    let mut numContacts: libc::c_int = 0;
    let mut contacts: *mut cpContact = 0 as *mut cpContact;
    let mut a: *const cpShape = 0 as *const cpShape;
    let mut b: *const cpShape = 0 as *const cpShape;
    let mut shape_pair: [*const cpShape; 2] = [0 as *const cpShape; 2];
    let mut arbHashID: cpHashValue = 0;
    let mut tmp___1: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut bodyA___0: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___2: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    tmp = cpBodyGetType(body);
    if !(tmp as libc::c_uint == 0 as libc::c_uint) {
        cpMessage(
            b"cpBodyGetType(body) == CP_BODY_TYPE_DYNAMIC\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Internal error: Attempting to activate a non-dynamic body.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked != 0 {
        tmp___0 = cpArrayContains((*space).rousedBodies, body as *mut libc::c_void);
        if tmp___0 == 0 {
            cpArrayPush((*space).rousedBodies, body as *mut libc::c_void);
        }
    } else {
        cpArrayPush((*space).dynamicBodies, body as *mut libc::c_void);
        shape = (*body).shapeList;
        while !shape.is_null() {
            cpSpatialIndexRemove(
                (*space).staticShapes,
                shape as *mut libc::c_void,
                (*shape).hashid,
            );
            cpSpatialIndexInsert(
                (*space).dynamicShapes,
                shape as *mut libc::c_void,
                (*shape).hashid,
            );
            shape = (*shape).next;
        }
        arb = (*body).arbiterList;
        while !arb.is_null() {
            bodyA = (*arb).body_a;
            let mut current_block_33: u64;
            if body as libc::c_ulong == bodyA as libc::c_ulong {
                current_block_33 = 6355164878261996678;
            } else {
                tmp___1 = cpBodyGetType(bodyA);
                if tmp___1 as libc::c_uint == 2 as libc::c_uint {
                    current_block_33 = 6355164878261996678;
                } else {
                    current_block_33 = 14136749492126903395;
                }
            }
            match current_block_33 {
                6355164878261996678 => {
                    numContacts = (*arb).count;
                    contacts = (*arb).contacts;
                    (*arb).contacts = cpContactBufferGetArray(space);
                    memcpy(
                        (*arb).contacts as *mut libc::c_void,
                        contacts as *const libc::c_void,
                        (numContacts as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<cpContact>() as libc::c_ulong,
                            ),
                    );
                    cpSpacePushContacts(space, numContacts);
                    a = (*arb).a;
                    b = (*arb).b;
                    shape_pair[0 as libc::c_int as usize] = a;
                    shape_pair[1 as libc::c_int as usize] = b;
                    arbHashID = (a as cpHashValue)
                        .wrapping_mul(3344921057 as libc::c_ulong)
                        ^ (b as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong);
                    cpHashSetInsert(
                        (*space).cachedArbiters,
                        arbHashID,
                        shape_pair.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::transmute::<
                            *mut libc::c_void,
                            Option::<
                                unsafe extern "C" fn(
                                    *const libc::c_void,
                                    *mut libc::c_void,
                                ) -> *mut libc::c_void,
                            >,
                        >(0 as *mut libc::c_void),
                        arb as *mut libc::c_void,
                    );
                    (*arb).stamp = (*space).stamp;
                    cpArrayPush((*space).arbiters, arb as *mut libc::c_void);
                    free(contacts as *mut libc::c_void);
                }
                _ => {}
            }
            arb = cpArbiterNext(arb, body);
        }
        constraint = (*body).constraintList;
        while !constraint.is_null() {
            bodyA___0 = (*constraint).a;
            if body as libc::c_ulong == bodyA___0 as libc::c_ulong {
                cpArrayPush((*space).constraints, constraint as *mut libc::c_void);
            } else {
                tmp___2 = cpBodyGetType(bodyA___0);
                if tmp___2 as libc::c_uint == 2 as libc::c_uint {
                    cpArrayPush((*space).constraints, constraint as *mut libc::c_void);
                }
            }
            constraint = cpConstraintNext(constraint, body);
        }
    };
}
unsafe extern "C" fn cpSpaceDeactivateBody(
    mut space: *mut cpSpace,
    mut body: *mut cpBody,
) {
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut bodyA: *mut cpBody = 0 as *mut cpBody;
    let mut bytes: size_t = 0;
    let mut contacts: *mut cpContact = 0 as *mut cpContact;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut bodyA___0: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___2: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    tmp = cpBodyGetType(body);
    if !(tmp as libc::c_uint == 0 as libc::c_uint) {
        cpMessage(
            b"cpBodyGetType(body) == CP_BODY_TYPE_DYNAMIC\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Internal error: Attempting to deactivate a non-dynamic body.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    cpArrayDeleteObj((*space).dynamicBodies, body as *mut libc::c_void);
    shape = (*body).shapeList;
    while !shape.is_null() {
        cpSpatialIndexRemove(
            (*space).dynamicShapes,
            shape as *mut libc::c_void,
            (*shape).hashid,
        );
        cpSpatialIndexInsert(
            (*space).staticShapes,
            shape as *mut libc::c_void,
            (*shape).hashid,
        );
        shape = (*shape).next;
    }
    arb = (*body).arbiterList;
    while !arb.is_null() {
        bodyA = (*arb).body_a;
        let mut current_block_20: u64;
        if body as libc::c_ulong == bodyA as libc::c_ulong {
            current_block_20 = 17791669900386623267;
        } else {
            tmp___1 = cpBodyGetType(bodyA);
            if tmp___1 as libc::c_uint == 2 as libc::c_uint {
                current_block_20 = 17791669900386623267;
            } else {
                current_block_20 = 14576567515993809846;
            }
        }
        match current_block_20 {
            17791669900386623267 => {
                cpSpaceUncacheArbiter(space, arb);
                bytes = ((*arb).count as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<cpContact>() as libc::c_ulong);
                tmp___0 = calloc(1 as libc::c_int as size_t, bytes);
                contacts = tmp___0 as *mut cpContact;
                memcpy(
                    contacts as *mut libc::c_void,
                    (*arb).contacts as *const libc::c_void,
                    bytes,
                );
                (*arb).contacts = contacts;
            }
            _ => {}
        }
        arb = cpArbiterNext(arb, body);
    }
    constraint = (*body).constraintList;
    while !constraint.is_null() {
        bodyA___0 = (*constraint).a;
        if body as libc::c_ulong == bodyA___0 as libc::c_ulong {
            cpArrayDeleteObj((*space).constraints, constraint as *mut libc::c_void);
        } else {
            tmp___2 = cpBodyGetType(bodyA___0);
            if tmp___2 as libc::c_uint == 2 as libc::c_uint {
                cpArrayDeleteObj((*space).constraints, constraint as *mut libc::c_void);
            }
        }
        constraint = cpConstraintNext(constraint, body);
    }
}
#[inline]
unsafe extern "C" fn ComponentRoot(mut body: *mut cpBody) -> *mut cpBody {
    let mut tmp: *mut cpBody = 0 as *mut cpBody;
    if !body.is_null() {
        tmp = (*body).sleeping.root;
    } else {
        tmp = 0 as *mut libc::c_void as *mut cpBody;
    }
    return tmp;
}
pub unsafe extern "C" fn cpBodyActivate(mut body: *mut cpBody) {
    let mut root: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: *mut cpBody = 0 as *mut cpBody;
    let mut space: *mut cpSpace = 0 as *mut cpSpace;
    let mut body___0: *mut cpBody = 0 as *mut cpBody;
    let mut next: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___0: cpBool = 0;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut other: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___1: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___2: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___3: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    if body as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        tmp___3 = cpBodyGetType(body);
        if tmp___3 as libc::c_uint == 0 as libc::c_uint {
            (*body).sleeping.idleTime = 0.0f32 as cpFloat;
            tmp = ComponentRoot(body);
            root = tmp;
            if !root.is_null() {
                tmp___0 = cpBodyIsSleeping(root as *const cpBody);
                if tmp___0 != 0 {
                    space = (*root).space;
                    body___0 = root;
                    while !body___0.is_null() {
                        next = (*body___0).sleeping.next;
                        (*body___0).sleeping.idleTime = 0.0f32 as cpFloat;
                        (*body___0)
                            .sleeping
                            .root = 0 as *mut libc::c_void as *mut cpBody;
                        (*body___0)
                            .sleeping
                            .next = 0 as *mut libc::c_void as *mut cpBody;
                        cpSpaceActivateBody(space, body___0);
                        body___0 = next;
                    }
                    cpArrayDeleteObj(
                        (*space).sleepingComponents,
                        root as *mut libc::c_void,
                    );
                }
            }
            arb = (*body).arbiterList;
            while !arb.is_null() {
                if (*arb).body_a as libc::c_ulong == body as libc::c_ulong {
                    tmp___1 = (*arb).body_b;
                } else {
                    tmp___1 = (*arb).body_a;
                }
                other = tmp___1;
                tmp___2 = cpBodyGetType(other);
                if tmp___2 as libc::c_uint != 2 as libc::c_uint {
                    (*other).sleeping.idleTime = 0.0f32 as cpFloat;
                }
                arb = cpArbiterNext(arb, body);
            }
        }
    }
}
pub unsafe extern "C" fn cpBodyActivateStatic(
    mut body: *mut cpBody,
    mut filter: *mut cpShape,
) {
    let mut current_block: u64;
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut tmp___0: *mut cpBody = 0 as *mut cpBody;
    tmp = cpBodyGetType(body);
    if !(tmp as libc::c_uint == 2 as libc::c_uint) {
        cpMessage(
            b"cpBodyGetType(body) == CP_BODY_TYPE_STATIC\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"cpBodyActivateStatic() called on a non-static body.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    arb = (*body).arbiterList;
    while !arb.is_null() {
        if filter.is_null() {
            current_block = 7729562826072400187;
        } else if filter as libc::c_ulong == (*arb).a as libc::c_ulong {
            current_block = 7729562826072400187;
        } else if filter as libc::c_ulong == (*arb).b as libc::c_ulong {
            current_block = 7729562826072400187;
        } else {
            current_block = 9606288038608642794;
        }
        match current_block {
            7729562826072400187 => {
                if (*arb).body_a as libc::c_ulong == body as libc::c_ulong {
                    tmp___0 = (*arb).body_b;
                } else {
                    tmp___0 = (*arb).body_a;
                }
                cpBodyActivate(tmp___0);
            }
            _ => {}
        }
        arb = cpArbiterNext(arb, body);
    }
}
#[inline]
unsafe extern "C" fn cpBodyPushArbiter(mut body: *mut cpBody, mut arb: *mut cpArbiter) {
    let mut next: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut tmp: *mut cpArbiterThread = 0 as *mut cpArbiterThread;
    let mut tmp___0: *mut cpArbiterThread = 0 as *mut cpArbiterThread;
    next = (*body).arbiterList;
    tmp = cpArbiterThreadForBody(arb, body);
    (*tmp).next = next;
    if !next.is_null() {
        tmp___0 = cpArbiterThreadForBody(next, body);
        (*tmp___0).prev = arb;
    }
    (*body).arbiterList = arb;
}
#[inline]
unsafe extern "C" fn ComponentAdd(mut root: *mut cpBody, mut body: *mut cpBody) {
    (*body).sleeping.root = root;
    if body as libc::c_ulong != root as libc::c_ulong {
        (*body).sleeping.next = (*root).sleeping.next;
        (*root).sleeping.next = body;
    }
}
#[inline]
unsafe extern "C" fn FloodFillComponent(mut root: *mut cpBody, mut body: *mut cpBody) {
    let mut other_root: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: *mut cpBody = 0 as *mut cpBody;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut tmp___0: *mut cpBody = 0 as *mut cpBody;
    let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut tmp___1: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___2: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    tmp___2 = cpBodyGetType(body);
    if tmp___2 as libc::c_uint == 0 as libc::c_uint {
        tmp = ComponentRoot(body);
        other_root = tmp;
        if other_root as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
            ComponentAdd(root, body);
            arb = (*body).arbiterList;
            while !arb.is_null() {
                if body as libc::c_ulong == (*arb).body_a as libc::c_ulong {
                    tmp___0 = (*arb).body_b;
                } else {
                    tmp___0 = (*arb).body_a;
                }
                FloodFillComponent(root, tmp___0);
                arb = cpArbiterNext(arb, body);
            }
            constraint = (*body).constraintList;
            while !constraint.is_null() {
                if body as libc::c_ulong == (*constraint).a as libc::c_ulong {
                    tmp___1 = (*constraint).b;
                } else {
                    tmp___1 = (*constraint).a;
                }
                FloodFillComponent(root, tmp___1);
                constraint = cpConstraintNext(constraint, body);
            }
        }
    }
}
#[inline]
unsafe extern "C" fn ComponentActive(
    mut root: *mut cpBody,
    mut threshold: cpFloat,
) -> cpBool {
    let mut body: *mut cpBody = 0 as *mut cpBody;
    body = root;
    while !body.is_null() {
        if (*body).sleeping.idleTime < threshold {
            return 1 as libc::c_int as cpBool;
        }
        body = (*body).sleeping.next;
    }
    return 0 as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpSpaceProcessComponents(
    mut space: *mut cpSpace,
    mut dt: cpFloat,
) {
    let mut sleep: cpBool = 0;
    let mut tmp: libc::c_float = 0.;
    let mut bodies: *mut cpArray = 0 as *mut cpArray;
    let mut dv: cpFloat = 0.;
    let mut dvsq: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut i: libc::c_int = 0;
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___2: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut keThreshold: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut tmp___5: cpFloat = 0.;
    let mut arbiters: *mut cpArray = 0 as *mut cpArray;
    let mut i___0: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___6: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___7: cpBool = 0;
    let mut tmp___8: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___9: cpBool = 0;
    let mut constraints: *mut cpArray = 0 as *mut cpArray;
    let mut i___1: libc::c_int = 0;
    let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut a___0: *mut cpBody = 0 as *mut cpBody;
    let mut b___0: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___10: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___11: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut i___2: libc::c_int = 0;
    let mut body___0: *mut cpBody = 0 as *mut cpBody;
    let mut other: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___12: cpBool = 0;
    let mut tmp___13: *mut cpBody = 0 as *mut cpBody;
    tmp = ::std::f32::INFINITY;
    sleep = ((*space).sleepTimeThreshold != tmp as cpFloat) as libc::c_int as cpBool;
    bodies = (*space).dynamicBodies;
    if sleep != 0 {
        dv = (*space).idleSpeedThreshold;
        if dv != 0. {
            tmp___1 = dv * dv;
        } else {
            tmp___0 = cpvlengthsq((*space).gravity);
            tmp___1 = tmp___0 * dt * dt;
        }
        dvsq = tmp___1;
        i = 0 as libc::c_int;
        while i < (*bodies).num {
            body = *((*bodies).arr).offset(i as isize) as *mut cpBody;
            tmp___2 = cpBodyGetType(body);
            if !(tmp___2 as libc::c_uint != 0 as libc::c_uint) {
                if dvsq != 0. {
                    tmp___3 = (*body).m * dvsq;
                } else {
                    tmp___3 = 0.0f32 as cpFloat;
                }
                keThreshold = tmp___3;
                tmp___5 = cpBodyKineticEnergy(body as *const cpBody);
                if tmp___5 > keThreshold {
                    (*body).sleeping.idleTime = 0.0f32 as cpFloat;
                } else {
                    (*body).sleeping.idleTime += dt;
                }
            }
            i += 1;
        }
    }
    arbiters = (*space).arbiters;
    i___0 = 0 as libc::c_int;
    count = (*arbiters).num;
    while i___0 < count {
        arb = *((*arbiters).arr).offset(i___0 as isize) as *mut cpArbiter;
        a = (*arb).body_a;
        b = (*arb).body_b;
        if sleep != 0 {
            tmp___6 = cpBodyGetType(b);
            if tmp___6 as libc::c_uint == 1 as libc::c_uint {
                cpBodyActivate(a);
            } else {
                tmp___7 = cpBodyIsSleeping(a as *const cpBody);
                if tmp___7 != 0 {
                    cpBodyActivate(a);
                }
            }
            tmp___8 = cpBodyGetType(a);
            if tmp___8 as libc::c_uint == 1 as libc::c_uint {
                cpBodyActivate(b);
            } else {
                tmp___9 = cpBodyIsSleeping(b as *const cpBody);
                if tmp___9 != 0 {
                    cpBodyActivate(b);
                }
            }
        }
        cpBodyPushArbiter(a, arb);
        cpBodyPushArbiter(b, arb);
        i___0 += 1;
    }
    if sleep != 0 {
        constraints = (*space).constraints;
        i___1 = 0 as libc::c_int;
        while i___1 < (*constraints).num {
            constraint = *((*constraints).arr).offset(i___1 as isize)
                as *mut cpConstraint;
            a___0 = (*constraint).a;
            b___0 = (*constraint).b;
            tmp___10 = cpBodyGetType(b___0);
            if tmp___10 as libc::c_uint == 1 as libc::c_uint {
                cpBodyActivate(a___0);
            }
            tmp___11 = cpBodyGetType(a___0);
            if tmp___11 as libc::c_uint == 1 as libc::c_uint {
                cpBodyActivate(b___0);
            }
            i___1 += 1;
        }
        i___2 = 0 as libc::c_int;
        while i___2 < (*bodies).num {
            let mut current_block_92: u64;
            body___0 = *((*bodies).arr).offset(i___2 as isize) as *mut cpBody;
            tmp___13 = ComponentRoot(body___0);
            if tmp___13 as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
                FloodFillComponent(body___0, body___0);
                tmp___12 = ComponentActive(body___0, (*space).sleepTimeThreshold);
                if tmp___12 == 0 {
                    cpArrayPush(
                        (*space).sleepingComponents,
                        body___0 as *mut libc::c_void,
                    );
                    other = body___0;
                    while !other.is_null() {
                        cpSpaceDeactivateBody(space, other);
                        other = (*other).sleeping.next;
                    }
                    current_block_92 = 1209030638129645089;
                } else {
                    current_block_92 = 5207889489643863322;
                }
            } else {
                current_block_92 = 5207889489643863322;
            }
            match current_block_92 {
                5207889489643863322 => {
                    i___2 += 1;
                    (*body___0).sleeping.root = 0 as *mut libc::c_void as *mut cpBody;
                    (*body___0).sleeping.next = 0 as *mut libc::c_void as *mut cpBody;
                }
                _ => {}
            }
        }
    }
}
pub unsafe extern "C" fn cpBodySleep(mut body: *mut cpBody) {
    cpBodySleepWithGroup(body, 0 as *mut libc::c_void as *mut cpBody);
}
pub unsafe extern "C" fn cpBodySleepWithGroup(
    mut body: *mut cpBody,
    mut group: *mut cpBody,
) {
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut space: *mut cpSpace = 0 as *mut cpSpace;
    let mut tmp___0: cpBool = 0;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: libc::c_float = 0.;
    let mut tmp___3: cpBool = 0;
    let mut tmp___4: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___5: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___6: cpBool = 0;
    let mut shape: *mut cpShape = 0 as *mut cpShape;
    let mut root: *mut cpBody = 0 as *mut cpBody;
    let mut tmp___7: *mut cpBody = 0 as *mut cpBody;
    tmp = cpBodyGetType(body);
    if !(tmp as libc::c_uint == 0 as libc::c_uint) {
        cpMessage(
            b"cpBodyGetType(body) == CP_BODY_TYPE_DYNAMIC\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Non-dynamic bodies cannot be put to sleep.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    space = (*body).space;
    tmp___0 = cpSpaceIsLocked(space);
    if tmp___0 != 0 {
        cpMessage(
            b"!cpSpaceIsLocked(space)\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            320 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Bodies cannot be put to sleep during a query or a call to cpSpaceStep(). Put these calls into a post-step callback.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    tmp___1 = cpSpaceGetSleepTimeThreshold(space as *const cpSpace);
    tmp___2 = ::std::f32::INFINITY;
    if !(tmp___1 < tmp___2 as cpFloat) {
        cpMessage(
            b"cpSpaceGetSleepTimeThreshold(space) < INFINITY\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
            321 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Sleeping is not enabled on the space. You cannot sleep a body without setting a sleep time threshold on the space.\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    if !(group as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong) {
        tmp___3 = cpBodyIsSleeping(group as *const cpBody);
        if tmp___3 == 0 {
            cpMessage(
                b"group == NULL || cpBodyIsSleeping(group)\0" as *const u8
                    as *const libc::c_char,
                b"../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
                322 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Cannot use a non-sleeping body as a group identifier.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
    }
    tmp___6 = cpBodyIsSleeping(body as *const cpBody);
    if tmp___6 != 0 {
        tmp___4 = ComponentRoot(body);
        tmp___5 = ComponentRoot(group);
        if !(tmp___4 as libc::c_ulong == tmp___5 as libc::c_ulong) {
            cpMessage(
                b"ComponentRoot(body) == ComponentRoot(group)\0" as *const u8
                    as *const libc::c_char,
                b"../src/cpSpaceComponent.c\0" as *const u8 as *const libc::c_char,
                325 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"The body is already sleeping and it's group cannot be reassigned.\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
        return;
    }
    shape = (*body).shapeList;
    while !shape.is_null() {
        cpShapeCacheBB(shape);
        shape = (*shape).next;
    }
    cpSpaceDeactivateBody(space, body);
    if !group.is_null() {
        tmp___7 = ComponentRoot(group);
        root = tmp___7;
        (*body).sleeping.root = root;
        (*body).sleeping.next = (*root).sleeping.next;
        (*body).sleeping.idleTime = 0.0f32 as cpFloat;
        (*root).sleeping.next = body;
    } else {
        (*body).sleeping.root = body;
        (*body).sleeping.next = 0 as *mut libc::c_void as *mut cpBody;
        (*body).sleeping.idleTime = 0.0f32 as cpFloat;
        cpArrayPush((*space).sleepingComponents, body as *mut libc::c_void);
    }
    cpArrayDeleteObj((*space).dynamicBodies, body as *mut libc::c_void);
}
unsafe extern "C" fn cpSpaceDebugDrawShape(
    mut shape: *mut cpShape,
    mut options: *mut cpSpaceDebugDrawOptions,
) {
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut data: cpDataPointer = 0 as *mut libc::c_void;
    let mut outline_color: cpSpaceDebugColor = cpSpaceDebugColor {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    let mut fill_color: cpSpaceDebugColor = cpSpaceDebugColor {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    let mut tmp: cpSpaceDebugColor = cpSpaceDebugColor {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    let mut circle: *mut cpCircleShape = 0 as *mut cpCircleShape;
    let mut seg___0: *mut cpSegmentShape = 0 as *mut cpSegmentShape;
    let mut poly: *mut cpPolyShape = 0 as *mut cpPolyShape;
    let mut count: libc::c_int = 0;
    let mut planes: *mut cpSplittingPlane = 0 as *mut cpSplittingPlane;
    let mut verts: *mut cpVect = 0 as *mut cpVect;
    let mut tmp___0: *mut _ = 0 as *mut _;
    let mut i: libc::c_int = 0;
    body = (*shape).body;
    data = (*options).data;
    outline_color = (*options).shapeOutlineColor;
    tmp = (Some(((*options).colorForShape).expect("non-null function pointer")))
        .expect("non-null function pointer")(shape, data);
    fill_color = tmp;
    match (*(*shape).klass).type_0 as libc::c_uint {
        0 => {
            circle = shape as *mut cpCircleShape;
            (Some(((*options).drawCircle).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )((*circle).tc, (*body).a, (*circle).r, outline_color, fill_color, data);
        }
        1 => {
            seg___0 = shape as *mut cpSegmentShape;
            (Some(((*options).drawFatSegment).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                (*seg___0).ta,
                (*seg___0).tb,
                (*seg___0).r,
                outline_color,
                fill_color,
                data,
            );
        }
        2 => {
            poly = shape as *mut cpPolyShape;
            count = (*poly).count;
            planes = (*poly).planes;
            let mut fresh23 = ::std::vec::from_elem(
                0,
                (count as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<cpVect>() as libc::c_ulong)
                    as usize,
            );
            tmp___0 = fresh23.as_mut_ptr();
            verts = tmp___0 as *mut cpVect;
            i = 0 as libc::c_int;
            while i < count {
                *verts.offset(i as isize) = (*planes.offset(i as isize)).v0;
                i += 1;
            }
            (Some(((*options).drawPolygon).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                count,
                verts as *const cpVect,
                (*poly).r,
                outline_color,
                fill_color,
                data,
            );
        }
        _ => {}
    };
}
static mut spring_verts: [cpVect; 15] = [
    {
        let mut init = cpVect {
            x: 0.00f32 as cpFloat,
            y: 0.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.20f32 as cpFloat,
            y: 0.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.25f32 as cpFloat,
            y: 3.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.30f32 as cpFloat,
            y: -6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.35f32 as cpFloat,
            y: 6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.40f32 as cpFloat,
            y: -6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.45f32 as cpFloat,
            y: 6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.50f32 as cpFloat,
            y: -6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.55f32 as cpFloat,
            y: 6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.60f32 as cpFloat,
            y: -6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.65f32 as cpFloat,
            y: 6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.70f32 as cpFloat,
            y: -3.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.75f32 as cpFloat,
            y: 6.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 0.80f32 as cpFloat,
            y: 0.0f32 as cpFloat,
        };
        init
    },
    {
        let mut init = cpVect {
            x: 1.00f32 as cpFloat,
            y: 0.0f32 as cpFloat,
        };
        init
    },
];
static mut spring_count: libc::c_int = 0;
unsafe extern "C" fn cpSpaceDebugDrawConstraint(
    mut constraint: *mut cpConstraint,
    mut options: *mut cpSpaceDebugDrawOptions,
) {
    let mut data: cpDataPointer = 0 as *mut libc::c_void;
    let mut color: cpSpaceDebugColor = cpSpaceDebugColor {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    let mut body_a: *mut cpBody = 0 as *mut cpBody;
    let mut body_b: *mut cpBody = 0 as *mut cpBody;
    let mut joint: *mut cpPinJoint = 0 as *mut cpPinJoint;
    let mut a: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut b: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut joint___0: *mut cpSlideJoint = 0 as *mut cpSlideJoint;
    let mut a___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut b___0: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut joint___1: *mut cpPivotJoint = 0 as *mut cpPivotJoint;
    let mut a___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut b___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    let mut joint___2: *mut cpGrooveJoint = 0 as *mut cpGrooveJoint;
    let mut a___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___5: cpVect = cpVect { x: 0., y: 0. };
    let mut b___2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___6: cpVect = cpVect { x: 0., y: 0. };
    let mut c: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___7: cpVect = cpVect { x: 0., y: 0. };
    let mut spring: *mut cpDampedSpring = 0 as *mut cpDampedSpring;
    let mut a___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___8: cpVect = cpVect { x: 0., y: 0. };
    let mut b___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___9: cpVect = cpVect { x: 0., y: 0. };
    let mut delta: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___10: cpVect = cpVect { x: 0., y: 0. };
    let mut cos___0: cpFloat = 0.;
    let mut sin___0: cpFloat = 0.;
    let mut s: cpFloat = 0.;
    let mut tmp___11: cpFloat = 0.;
    let mut r1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___12: cpVect = cpVect { x: 0., y: 0. };
    let mut r2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___13: cpVect = cpVect { x: 0., y: 0. };
    let mut verts: *mut cpVect = 0 as *mut cpVect;
    let mut tmp___14: *mut _ = 0 as *mut _;
    let mut i: libc::c_int = 0;
    let mut v: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___15: cpFloat = 0.;
    let mut tmp___16: cpFloat = 0.;
    let mut i___0: libc::c_int = 0;
    let mut tmp___17: cpBool = 0;
    let mut tmp___18: cpBool = 0;
    let mut tmp___19: cpBool = 0;
    let mut tmp___20: cpBool = 0;
    let mut tmp___21: cpBool = 0;
    data = (*options).data;
    color = (*options).constraintColor;
    body_a = (*constraint).a;
    body_b = (*constraint).b;
    tmp___21 = cpConstraintIsPinJoint(constraint as *const cpConstraint);
    if tmp___21 != 0 {
        joint = constraint as *mut cpPinJoint;
        tmp = cpTransformPoint((*body_a).transform, (*joint).anchorA);
        a = tmp;
        tmp___0 = cpTransformPoint((*body_b).transform, (*joint).anchorB);
        b = tmp___0;
        (Some(((*options).drawDot).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(5 as libc::c_int as cpFloat, a, color, data);
        (Some(((*options).drawDot).expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(5 as libc::c_int as cpFloat, b, color, data);
        (Some(((*options).drawSegment).expect("non-null function pointer")))
            .expect("non-null function pointer")(a, b, color, data);
    } else {
        tmp___20 = cpConstraintIsSlideJoint(constraint as *const cpConstraint);
        if tmp___20 != 0 {
            joint___0 = constraint as *mut cpSlideJoint;
            tmp___1 = cpTransformPoint((*body_a).transform, (*joint___0).anchorA);
            a___0 = tmp___1;
            tmp___2 = cpTransformPoint((*body_b).transform, (*joint___0).anchorB);
            b___0 = tmp___2;
            (Some(((*options).drawDot).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(5 as libc::c_int as cpFloat, a___0, color, data);
            (Some(((*options).drawDot).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(5 as libc::c_int as cpFloat, b___0, color, data);
            (Some(((*options).drawSegment).expect("non-null function pointer")))
                .expect("non-null function pointer")(a___0, b___0, color, data);
        } else {
            tmp___19 = cpConstraintIsPivotJoint(constraint as *const cpConstraint);
            if tmp___19 != 0 {
                joint___1 = constraint as *mut cpPivotJoint;
                tmp___3 = cpTransformPoint((*body_a).transform, (*joint___1).anchorA);
                a___1 = tmp___3;
                tmp___4 = cpTransformPoint((*body_b).transform, (*joint___1).anchorB);
                b___1 = tmp___4;
                (Some(((*options).drawDot).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(5 as libc::c_int as cpFloat, a___1, color, data);
                (Some(((*options).drawDot).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(5 as libc::c_int as cpFloat, b___1, color, data);
            } else {
                tmp___18 = cpConstraintIsGrooveJoint(constraint as *const cpConstraint);
                if tmp___18 != 0 {
                    joint___2 = constraint as *mut cpGrooveJoint;
                    tmp___5 = cpTransformPoint((*body_a).transform, (*joint___2).grv_a);
                    a___2 = tmp___5;
                    tmp___6 = cpTransformPoint((*body_a).transform, (*joint___2).grv_b);
                    b___2 = tmp___6;
                    tmp___7 = cpTransformPoint(
                        (*body_b).transform,
                        (*joint___2).anchorB,
                    );
                    c = tmp___7;
                    (Some(((*options).drawDot).expect("non-null function pointer")))
                        .expect(
                            "non-null function pointer",
                        )(5 as libc::c_int as cpFloat, c, color, data);
                    (Some(((*options).drawSegment).expect("non-null function pointer")))
                        .expect("non-null function pointer")(a___2, b___2, color, data);
                } else {
                    tmp___17 = cpConstraintIsDampedSpring(
                        constraint as *const cpConstraint,
                    );
                    if tmp___17 != 0 {
                        spring = constraint as *mut cpDampedSpring;
                        tmp___8 = cpTransformPoint(
                            (*body_a).transform,
                            (*spring).anchorA,
                        );
                        a___3 = tmp___8;
                        tmp___9 = cpTransformPoint(
                            (*body_b).transform,
                            (*spring).anchorB,
                        );
                        b___3 = tmp___9;
                        (Some(((*options).drawDot).expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(5 as libc::c_int as cpFloat, a___3, color, data);
                        (Some(((*options).drawDot).expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(5 as libc::c_int as cpFloat, b___3, color, data);
                        tmp___10 = cpvsub(b___3, a___3);
                        delta = tmp___10;
                        cos___0 = delta.x;
                        sin___0 = delta.y;
                        tmp___11 = cpvlength(delta);
                        s = 1.0f32 as cpFloat / tmp___11;
                        tmp___12 = cpv(cos___0, -sin___0 * s);
                        r1 = tmp___12;
                        tmp___13 = cpv(sin___0, cos___0 * s);
                        r2 = tmp___13;
                        let mut fresh24 = ::std::vec::from_elem(
                            0,
                            (spring_count as libc::c_ulong)
                                .wrapping_mul(
                                    ::std::mem::size_of::<cpVect>() as libc::c_ulong,
                                ) as usize,
                        );
                        tmp___14 = fresh24.as_mut_ptr();
                        verts = tmp___14 as *mut cpVect;
                        i = 0 as libc::c_int;
                        while i < spring_count {
                            v = spring_verts[i as usize];
                            tmp___15 = cpvdot(v, r2);
                            tmp___16 = cpvdot(v, r1);
                            *verts
                                .offset(
                                    i as isize,
                                ) = cpv(tmp___16 + a___3.x, tmp___15 + a___3.y);
                            i += 1;
                        }
                        i___0 = 0 as libc::c_int;
                        while i___0 < spring_count - 1 as libc::c_int {
                            (Some(
                                ((*options).drawSegment).expect("non-null function pointer"),
                            ))
                                .expect(
                                    "non-null function pointer",
                                )(
                                *verts.offset(i___0 as isize),
                                *verts.offset((i___0 + 1 as libc::c_int) as isize),
                                color,
                                data,
                            );
                            i___0 += 1;
                        }
                    }
                }
            }
        }
    };
}
pub unsafe extern "C" fn cpSpaceDebugDraw(
    mut space: *mut cpSpace,
    mut options: *mut cpSpaceDebugDrawOptions,
) {
    let mut arbiters: *mut cpArray = 0 as *mut cpArray;
    let mut color: cpSpaceDebugColor = cpSpaceDebugColor {
        r: 0.,
        g: 0.,
        b: 0.,
        a: 0.,
    };
    let mut draw_seg: Option::<
        unsafe extern "C" fn(cpVect, cpVect, cpSpaceDebugColor, cpDataPointer) -> (),
    > = None;
    let mut data: cpDataPointer = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut n: cpVect = cpVect { x: 0., y: 0. };
    let mut j: libc::c_int = 0;
    let mut p1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp: cpVect = cpVect { x: 0., y: 0. };
    let mut p2: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___0: cpVect = cpVect { x: 0., y: 0. };
    let mut d: cpFloat = 0.;
    let mut a: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___1: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___2: cpVect = cpVect { x: 0., y: 0. };
    let mut b: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___3: cpVect = cpVect { x: 0., y: 0. };
    let mut tmp___4: cpVect = cpVect { x: 0., y: 0. };
    if (*options).flags as libc::c_uint & 1 as libc::c_uint != 0 {
        cpSpaceEachShape(
            space,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpShape,
                        *mut cpSpaceDebugDrawOptions,
                    ) -> (),
                >,
                Option::<unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> ()>,
            >(
                Some(
                    cpSpaceDebugDrawShape
                        as unsafe extern "C" fn(
                            *mut cpShape,
                            *mut cpSpaceDebugDrawOptions,
                        ) -> (),
                ),
            ),
            options as *mut libc::c_void,
        );
    }
    if (*options).flags as libc::c_uint & 2 as libc::c_uint != 0 {
        cpSpaceEachConstraint(
            space,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpConstraint,
                        *mut cpSpaceDebugDrawOptions,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(*mut cpConstraint, *mut libc::c_void) -> (),
                >,
            >(
                Some(
                    cpSpaceDebugDrawConstraint
                        as unsafe extern "C" fn(
                            *mut cpConstraint,
                            *mut cpSpaceDebugDrawOptions,
                        ) -> (),
                ),
            ),
            options as *mut libc::c_void,
        );
    }
    if (*options).flags as libc::c_uint & 4 as libc::c_uint != 0 {
        arbiters = (*space).arbiters;
        color = (*options).collisionPointColor;
        draw_seg = (*options).drawSegment;
        data = (*options).data;
        i = 0 as libc::c_int;
        while i < (*arbiters).num {
            arb = *((*arbiters).arr).offset(i as isize) as *mut cpArbiter;
            n = (*arb).n;
            j = 0 as libc::c_int;
            while j < (*arb).count {
                tmp = cpvadd(
                    (*(*arb).body_a).p,
                    (*((*arb).contacts).offset(j as isize)).r1,
                );
                p1 = tmp;
                tmp___0 = cpvadd(
                    (*(*arb).body_b).p,
                    (*((*arb).contacts).offset(j as isize)).r2,
                );
                p2 = tmp___0;
                d = 2.0f32 as cpFloat;
                tmp___1 = cpvmult(n, -d);
                tmp___2 = cpvadd(p1, tmp___1);
                a = tmp___2;
                tmp___3 = cpvmult(n, d);
                tmp___4 = cpvadd(p2, tmp___3);
                b = tmp___4;
                (Some(draw_seg.expect("non-null function pointer")))
                    .expect("non-null function pointer")(a, b, color, data);
                j += 1;
            }
            i += 1;
        }
    }
}
static mut primes___0: [libc::c_int; 30] = [
    5 as libc::c_int,
    13 as libc::c_int,
    23 as libc::c_int,
    47 as libc::c_int,
    97 as libc::c_int,
    193 as libc::c_int,
    389 as libc::c_int,
    769 as libc::c_int,
    1543 as libc::c_int,
    3079 as libc::c_int,
    6151 as libc::c_int,
    12289 as libc::c_int,
    24593 as libc::c_int,
    49157 as libc::c_int,
    98317 as libc::c_int,
    196613 as libc::c_int,
    393241 as libc::c_int,
    786433 as libc::c_int,
    1572869 as libc::c_int,
    3145739 as libc::c_int,
    6291469 as libc::c_int,
    12582917 as libc::c_int,
    25165843 as libc::c_int,
    50331653 as libc::c_int,
    100663319 as libc::c_int,
    201326611 as libc::c_int,
    402653189 as libc::c_int,
    805306457 as libc::c_int,
    1610612741 as libc::c_int,
    0 as libc::c_int,
];
#[inline]
unsafe extern "C" fn next_prime___0(mut n: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while n > primes___0[i as usize] {
        i += 1;
        if primes___0[i as usize] == 0 {
            cpMessage(
                b"primes[i]\0" as *const u8 as *const libc::c_char,
                b"../src/prime.h\0" as *const u8 as *const libc::c_char,
                64 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Tried to resize a hash table to a size greater than 1610612741 O_o\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
    }
    return primes___0[i as usize];
}
unsafe extern "C" fn cpHandleInit(
    mut hand: *mut cpHandle,
    mut obj: *mut libc::c_void,
) -> *mut cpHandle {
    (*hand).obj = obj;
    (*hand).retain = 0 as libc::c_int;
    (*hand).stamp = 0 as libc::c_int as cpTimestamp;
    return hand;
}
#[inline]
unsafe extern "C" fn cpHandleRetain(mut hand: *mut cpHandle) {
    (*hand).retain += 1;
}
#[inline]
unsafe extern "C" fn cpHandleRelease(
    mut hand: *mut cpHandle,
    mut pooledHandles: *mut cpArray,
) {
    (*hand).retain -= 1;
    if (*hand).retain == 0 as libc::c_int {
        cpArrayPush(pooledHandles, hand as *mut libc::c_void);
    }
}
unsafe extern "C" fn handleSetEql(
    mut obj: *mut libc::c_void,
    mut hand: *mut cpHandle,
) -> libc::c_int {
    return (obj as libc::c_ulong == (*hand).obj as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn handleSetTrans(
    mut obj: *mut libc::c_void,
    mut hash: *mut cpSpaceHash,
) -> *mut libc::c_void {
    let mut count: libc::c_int = 0;
    let mut buffer: *mut cpHandle = 0 as *mut cpHandle;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut hand: *mut cpHandle = 0 as *mut cpHandle;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut cpHandle = 0 as *mut cpHandle;
    if (*(*hash).pooledHandles).num == 0 as libc::c_int {
        count = (32768 as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cpHandle>() as libc::c_ulong)
            as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../src/cpSpaceHash.c\0" as *const u8 as *const libc::c_char,
                80 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size is too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        tmp = calloc(1 as libc::c_int as size_t, 32768 as libc::c_int as size_t);
        buffer = tmp as *mut cpHandle;
        cpArrayPush((*hash).allocatedBuffers, buffer as *mut libc::c_void);
        i = 0 as libc::c_int;
        while i < count {
            cpArrayPush(
                (*hash).pooledHandles,
                buffer.offset(i as isize) as *mut libc::c_void,
            );
            i += 1;
        }
    }
    tmp___0 = cpArrayPop((*hash).pooledHandles);
    tmp___1 = cpHandleInit(tmp___0 as *mut cpHandle, obj);
    hand = tmp___1;
    cpHandleRetain(hand);
    return hand as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn recycleBin___0(
    mut hash: *mut cpSpaceHash,
    mut bin: *mut cpSpaceHashBin,
) {
    (*bin).next = (*hash).pooledBins;
    (*hash).pooledBins = bin;
}
#[inline]
unsafe extern "C" fn clearTableCell(mut hash: *mut cpSpaceHash, mut idx: libc::c_int) {
    let mut bin: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    let mut next: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    bin = *((*hash).table).offset(idx as isize);
    while !bin.is_null() {
        next = (*bin).next;
        cpHandleRelease((*bin).handle, (*hash).pooledHandles);
        recycleBin___0(hash, bin);
        bin = next;
    }
    let ref mut fresh25 = *((*hash).table).offset(idx as isize);
    *fresh25 = 0 as *mut libc::c_void as *mut cpSpaceHashBin;
}
unsafe extern "C" fn clearTable(mut hash: *mut cpSpaceHash) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*hash).numcells {
        clearTableCell(hash, i);
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn getEmptyBin(mut hash: *mut cpSpaceHash) -> *mut cpSpaceHashBin {
    let mut bin: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    let mut count: libc::c_int = 0;
    let mut buffer: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    bin = (*hash).pooledBins;
    if !bin.is_null() {
        (*hash).pooledBins = (*bin).next;
        return bin;
    } else {
        count = (32768 as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cpSpaceHashBin>() as libc::c_ulong)
            as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../src/cpSpaceHash.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size is too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        tmp = calloc(1 as libc::c_int as size_t, 32768 as libc::c_int as size_t);
        buffer = tmp as *mut cpSpaceHashBin;
        cpArrayPush((*hash).allocatedBuffers, buffer as *mut libc::c_void);
        i = 1 as libc::c_int;
        while i < count {
            recycleBin___0(hash, buffer.offset(i as isize));
            i += 1;
        }
        return buffer;
    };
}
pub unsafe extern "C" fn cpSpaceHashAlloc() -> *mut cpSpaceHash {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpSpaceHash>() as libc::c_ulong,
    );
    return tmp as *mut cpSpaceHash;
}
unsafe extern "C" fn cpSpaceHashAllocTable(
    mut hash: *mut cpSpaceHash,
    mut numcells: libc::c_int,
) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    free((*hash).table as *mut libc::c_void);
    (*hash).numcells = numcells;
    tmp = calloc(
        numcells as size_t,
        ::std::mem::size_of::<*mut cpSpaceHashBin>() as libc::c_ulong,
    );
    (*hash).table = tmp as *mut *mut cpSpaceHashBin;
}
pub unsafe extern "C" fn cpSpaceHashInit(
    mut hash: *mut cpSpaceHash,
    mut celldim: cpFloat,
    mut numcells: libc::c_int,
    mut bbfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    let mut tmp: *mut cpSpatialIndexClass = 0 as *mut cpSpatialIndexClass;
    let mut tmp___0: libc::c_int = 0;
    tmp = Klass___0();
    cpSpatialIndexInit(hash as *mut cpSpatialIndex, tmp, bbfunc, staticIndex);
    tmp___0 = next_prime___0(numcells);
    cpSpaceHashAllocTable(hash, tmp___0);
    (*hash).celldim = celldim;
    (*hash)
        .handleSet = cpHashSetNew(
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut cpHandle) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> cpBool,
            >,
        >(
            Some(
                handleSetEql
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cpHandle,
                    ) -> libc::c_int,
            ),
        ),
    );
    (*hash).pooledHandles = cpArrayNew(0 as libc::c_int);
    (*hash).pooledBins = 0 as *mut libc::c_void as *mut cpSpaceHashBin;
    (*hash).allocatedBuffers = cpArrayNew(0 as libc::c_int);
    (*hash).stamp = 1 as libc::c_int as cpTimestamp;
    return hash as *mut cpSpatialIndex;
}
pub unsafe extern "C" fn cpSpaceHashNew(
    mut celldim: cpFloat,
    mut cells: libc::c_int,
    mut bbfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    let mut tmp: *mut cpSpaceHash = 0 as *mut cpSpaceHash;
    let mut tmp___0: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    tmp = cpSpaceHashAlloc();
    tmp___0 = cpSpaceHashInit(tmp, celldim, cells, bbfunc, staticIndex);
    return tmp___0;
}
unsafe extern "C" fn cpSpaceHashDestroy(mut hash: *mut cpSpaceHash) {
    if !((*hash).table).is_null() {
        clearTable(hash);
    }
    free((*hash).table as *mut libc::c_void);
    cpHashSetFree((*hash).handleSet);
    cpArrayFreeEach(
        (*hash).allocatedBuffers,
        Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    cpArrayFree((*hash).allocatedBuffers);
    cpArrayFree((*hash).pooledHandles);
}
#[inline]
unsafe extern "C" fn containsHandle(
    mut bin: *mut cpSpaceHashBin,
    mut hand: *mut cpHandle,
) -> cpBool {
    while !bin.is_null() {
        if (*bin).handle as libc::c_ulong == hand as libc::c_ulong {
            return 1 as libc::c_int as cpBool;
        }
        bin = (*bin).next;
    }
    return 0 as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn hash_func(
    mut x: cpHashValue,
    mut y: cpHashValue,
    mut n: cpHashValue,
) -> cpHashValue {
    return (x.wrapping_mul(1640531513 as libc::c_ulong)
        ^ y.wrapping_mul(2654435789 as libc::c_ulong))
        .wrapping_rem(n);
}
#[inline]
unsafe extern "C" fn floor_int(mut f: cpFloat) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    i = f as libc::c_int;
    if f < 0.0f32 as cpFloat {
        if f != i as cpFloat {
            tmp = i - 1 as libc::c_int;
        } else {
            tmp = i;
        }
    } else {
        tmp = i;
    }
    return tmp;
}
#[inline]
unsafe extern "C" fn hashHandle(
    mut hash: *mut cpSpaceHash,
    mut hand: *mut cpHandle,
    mut bb: cpBB,
) {
    let mut dim: cpFloat = 0.;
    let mut l: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut idx: cpHashValue = 0;
    let mut tmp___3: cpHashValue = 0;
    let mut bin: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    let mut tmp___4: cpBool = 0;
    let mut newBin: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    let mut tmp___5: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    dim = (*hash).celldim;
    tmp = floor_int(bb.l / dim);
    l = tmp;
    tmp___0 = floor_int(bb.r / dim);
    r = tmp___0;
    tmp___1 = floor_int(bb.b / dim);
    b = tmp___1;
    tmp___2 = floor_int(bb.t / dim);
    t = tmp___2;
    n = (*hash).numcells;
    i = l;
    while i <= r {
        j = b;
        while j <= t {
            tmp___3 = hash_func(i as cpHashValue, j as cpHashValue, n as cpHashValue);
            idx = tmp___3;
            bin = *((*hash).table).offset(idx as isize);
            tmp___4 = containsHandle(bin, hand);
            if !(tmp___4 != 0) {
                cpHandleRetain(hand);
                tmp___5 = getEmptyBin(hash);
                newBin = tmp___5;
                (*newBin).handle = hand;
                (*newBin).next = bin;
                let ref mut fresh26 = *((*hash).table).offset(idx as isize);
                *fresh26 = newBin;
            }
            j += 1;
        }
        i += 1;
    }
}
unsafe extern "C" fn cpSpaceHashInsert(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut hand: *mut cpHandle = 0 as *mut cpHandle;
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    let mut tmp___0: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = cpHashSetInsert(
        (*hash).handleSet,
        hashid,
        obj as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut cpSpaceHash,
                ) -> *mut libc::c_void,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
        >(
            Some(
                handleSetTrans
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut cpSpaceHash,
                    ) -> *mut libc::c_void,
            ),
        ),
        hash as *mut libc::c_void,
    );
    hand = tmp as *mut cpHandle;
    tmp___0 = (Some(((*hash).spatialIndex.bbfunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(obj);
    hashHandle(hash, hand, tmp___0);
}
unsafe extern "C" fn cpSpaceHashRehashObject(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut hand: *mut cpHandle = 0 as *mut cpHandle;
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    tmp = cpHashSetRemove((*hash).handleSet, hashid, obj as *const libc::c_void);
    hand = tmp as *mut cpHandle;
    if !hand.is_null() {
        (*hand).obj = 0 as *mut libc::c_void;
        cpHandleRelease(hand, (*hash).pooledHandles);
        cpSpaceHashInsert(hash, obj, hashid);
    }
}
unsafe extern "C" fn rehash_helper(mut hand: *mut cpHandle, mut hash: *mut cpSpaceHash) {
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = (Some(((*hash).spatialIndex.bbfunc).expect("non-null function pointer")))
        .expect("non-null function pointer")((*hand).obj);
    hashHandle(hash, hand, tmp);
}
unsafe extern "C" fn cpSpaceHashRehash(mut hash: *mut cpSpaceHash) {
    clearTable(hash);
    cpHashSetEach(
        (*hash).handleSet,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpHandle, *mut cpSpaceHash) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                rehash_helper
                    as unsafe extern "C" fn(*mut cpHandle, *mut cpSpaceHash) -> (),
            ),
        ),
        hash as *mut libc::c_void,
    );
}
unsafe extern "C" fn cpSpaceHashRemove(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut hand: *mut cpHandle = 0 as *mut cpHandle;
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    tmp = cpHashSetRemove((*hash).handleSet, hashid, obj as *const libc::c_void);
    hand = tmp as *mut cpHandle;
    if !hand.is_null() {
        (*hand).obj = 0 as *mut libc::c_void;
        cpHandleRelease(hand, (*hash).pooledHandles);
    }
}
unsafe extern "C" fn eachHelper(mut hand: *mut cpHandle, mut context: *mut eachContext) {
    (Some(((*context).func).expect("non-null function pointer")))
        .expect("non-null function pointer")((*hand).obj, (*context).data);
}
unsafe extern "C" fn cpSpaceHashEach(
    mut hash: *mut cpSpaceHash,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    let mut context: eachContext = eachContext {
        func: None,
        data: 0 as *mut libc::c_void,
    };
    context.func = func;
    context.data = data;
    cpHashSetEach(
        (*hash).handleSet,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpHandle, *mut eachContext) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                eachHelper as unsafe extern "C" fn(*mut cpHandle, *mut eachContext) -> (),
            ),
        ),
        &mut context as *mut eachContext as *mut libc::c_void,
    );
}
unsafe extern "C" fn remove_orphaned_handles(
    mut hash: *mut cpSpaceHash,
    mut bin_ptr: *mut *mut cpSpaceHashBin,
) {
    let mut bin: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    let mut hand: *mut cpHandle = 0 as *mut cpHandle;
    let mut next: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    bin = *bin_ptr;
    while !bin.is_null() {
        hand = (*bin).handle;
        next = (*bin).next;
        if ((*hand).obj).is_null() {
            *bin_ptr = (*bin).next;
            recycleBin___0(hash, bin);
            cpHandleRelease(hand, (*hash).pooledHandles);
        } else {
            bin_ptr = &mut (*bin).next;
        }
        bin = next;
    }
}
#[inline]
unsafe extern "C" fn query_helper(
    mut hash: *mut cpSpaceHash,
    mut bin_ptr: *mut *mut cpSpaceHashBin,
    mut obj: *mut libc::c_void,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    let mut bin: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    let mut hand: *mut cpHandle = 0 as *mut cpHandle;
    let mut other: *mut libc::c_void = 0 as *mut libc::c_void;
    '_restart: loop {
        bin = *bin_ptr;
        loop {
            if bin.is_null() {
                break '_restart;
            }
            hand = (*bin).handle;
            other = (*hand).obj;
            if !((*hand).stamp == (*hash).stamp) {
                if !(obj as libc::c_ulong == other as libc::c_ulong) {
                    if !other.is_null() {
                        (Some(func.expect("non-null function pointer")))
                            .expect(
                                "non-null function pointer",
                            )(obj, other, 0 as libc::c_int as cpCollisionID, data);
                        (*hand).stamp = (*hash).stamp;
                    } else {
                        remove_orphaned_handles(hash, bin_ptr);
                        break;
                    }
                }
            }
            bin = (*bin).next;
        }
    };
}
unsafe extern "C" fn cpSpaceHashQuery(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    let mut dim: cpFloat = 0.;
    let mut l: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut table: *mut *mut cpSpaceHashBin = 0 as *mut *mut cpSpaceHashBin;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut tmp___3: cpHashValue = 0;
    dim = (*hash).celldim;
    tmp = floor_int(bb.l / dim);
    l = tmp;
    tmp___0 = floor_int(bb.r / dim);
    r = tmp___0;
    tmp___1 = floor_int(bb.b / dim);
    b = tmp___1;
    tmp___2 = floor_int(bb.t / dim);
    t = tmp___2;
    n = (*hash).numcells;
    table = (*hash).table;
    i = l;
    while i <= r {
        j = b;
        while j <= t {
            tmp___3 = hash_func(i as cpHashValue, j as cpHashValue, n as cpHashValue);
            query_helper(hash, table.offset(tmp___3 as isize), obj, func, data);
            j += 1;
        }
        i += 1;
    }
    (*hash).stamp = ((*hash).stamp).wrapping_add(1);
}
unsafe extern "C" fn queryRehash_helper(
    mut hand: *mut cpHandle,
    mut context: *mut queryRehashContext,
) {
    let mut hash: *mut cpSpaceHash = 0 as *mut cpSpaceHash;
    let mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    > = None;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dim: cpFloat = 0.;
    let mut n: libc::c_int = 0;
    let mut obj: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bb: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut l: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut table: *mut *mut cpSpaceHashBin = 0 as *mut *mut cpSpaceHashBin;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut idx: cpHashValue = 0;
    let mut tmp___4: cpHashValue = 0;
    let mut bin: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    let mut tmp___5: cpBool = 0;
    let mut newBin: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    let mut tmp___6: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    hash = (*context).hash;
    func = (*context).func;
    data = (*context).data;
    dim = (*hash).celldim;
    n = (*hash).numcells;
    obj = (*hand).obj;
    tmp = (Some(((*hash).spatialIndex.bbfunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(obj);
    bb = tmp;
    tmp___0 = floor_int(bb.l / dim);
    l = tmp___0;
    tmp___1 = floor_int(bb.r / dim);
    r = tmp___1;
    tmp___2 = floor_int(bb.b / dim);
    b = tmp___2;
    tmp___3 = floor_int(bb.t / dim);
    t = tmp___3;
    table = (*hash).table;
    i = l;
    while i <= r {
        j = b;
        while j <= t {
            tmp___4 = hash_func(i as cpHashValue, j as cpHashValue, n as cpHashValue);
            idx = tmp___4;
            bin = *table.offset(idx as isize);
            tmp___5 = containsHandle(bin, hand);
            if !(tmp___5 != 0) {
                cpHandleRetain(hand);
                query_helper(hash, &mut bin, obj, func, data);
                tmp___6 = getEmptyBin(hash);
                newBin = tmp___6;
                (*newBin).handle = hand;
                (*newBin).next = bin;
                let ref mut fresh27 = *table.offset(idx as isize);
                *fresh27 = newBin;
            }
            j += 1;
        }
        i += 1;
    }
    (*hash).stamp = ((*hash).stamp).wrapping_add(1);
}
unsafe extern "C" fn cpSpaceHashReindexQuery(
    mut hash: *mut cpSpaceHash,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    let mut context: queryRehashContext = queryRehashContext {
        hash: 0 as *mut cpSpaceHash,
        func: None,
        data: 0 as *mut libc::c_void,
    };
    clearTable(hash);
    context.hash = hash;
    context.func = func;
    context.data = data;
    cpHashSetEach(
        (*hash).handleSet,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpHandle, *mut queryRehashContext) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                queryRehash_helper
                    as unsafe extern "C" fn(*mut cpHandle, *mut queryRehashContext) -> (),
            ),
        ),
        &mut context as *mut queryRehashContext as *mut libc::c_void,
    );
    cpSpatialIndexCollideStatic(
        hash as *mut cpSpatialIndex,
        (*hash).spatialIndex.staticIndex,
        func,
        data,
    );
}
#[inline]
unsafe extern "C" fn segmentQuery_helper(
    mut hash: *mut cpSpaceHash,
    mut bin_ptr: *mut *mut cpSpaceHashBin,
    mut obj: *mut libc::c_void,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> cpFloat,
    >,
    mut data: *mut libc::c_void,
) -> cpFloat {
    let mut t: cpFloat = 0.;
    let mut bin: *mut cpSpaceHashBin = 0 as *mut cpSpaceHashBin;
    let mut hand: *mut cpHandle = 0 as *mut cpHandle;
    let mut other: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp: cpFloat = 0.;
    t = 1.0f32 as cpFloat;
    '_restart: loop {
        bin = *bin_ptr;
        loop {
            if bin.is_null() {
                break '_restart;
            }
            hand = (*bin).handle;
            other = (*hand).obj;
            if !((*hand).stamp == (*hash).stamp) {
                if !other.is_null() {
                    tmp = (Some(func.expect("non-null function pointer")))
                        .expect("non-null function pointer")(obj, other, data);
                    t = cpfmin(t, tmp);
                    (*hand).stamp = (*hash).stamp;
                } else {
                    remove_orphaned_handles(hash, bin_ptr);
                    break;
                }
            }
            bin = (*bin).next;
        }
    }
    return t;
}
unsafe extern "C" fn cpSpaceHashSegmentQuery(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut a: cpVect,
    mut b: cpVect,
    mut t_exit: cpFloat,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> cpFloat,
    >,
    mut data: *mut libc::c_void,
) {
    let mut cell_x: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut cell_y: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut t: cpFloat = 0.;
    let mut x_inc: libc::c_int = 0;
    let mut y_inc: libc::c_int = 0;
    let mut temp_v: cpFloat = 0.;
    let mut temp_h: cpFloat = 0.;
    let mut tmp___1: libc::c_double = 0.;
    let mut tmp___2: libc::c_double = 0.;
    let mut tmp___3: libc::c_double = 0.;
    let mut tmp___4: libc::c_double = 0.;
    let mut dx: cpFloat = 0.;
    let mut tmp___5: cpFloat = 0.;
    let mut dy: cpFloat = 0.;
    let mut tmp___6: cpFloat = 0.;
    let mut dt_dx: cpFloat = 0.;
    let mut tmp___7: libc::c_float = 0.;
    let mut tmp___8: cpFloat = 0.;
    let mut dt_dy: cpFloat = 0.;
    let mut tmp___9: libc::c_float = 0.;
    let mut tmp___10: cpFloat = 0.;
    let mut next_h: cpFloat = 0.;
    let mut tmp___11: cpFloat = 0.;
    let mut next_v: cpFloat = 0.;
    let mut tmp___12: cpFloat = 0.;
    let mut n: libc::c_int = 0;
    let mut table: *mut *mut cpSpaceHashBin = 0 as *mut *mut cpSpaceHashBin;
    let mut idx: cpHashValue = 0;
    let mut tmp___13: cpHashValue = 0;
    let mut tmp___14: cpFloat = 0.;
    a = cpvmult(a, 1.0f32 as cpFloat / (*hash).celldim);
    b = cpvmult(b, 1.0f32 as cpFloat / (*hash).celldim);
    tmp = floor_int(a.x);
    cell_x = tmp;
    tmp___0 = floor_int(a.y);
    cell_y = tmp___0;
    t = 0 as libc::c_int as cpFloat;
    if b.x > a.x {
        x_inc = 1 as libc::c_int;
        tmp___1 = floor(a.x + 1.0f32 as cpFloat);
        temp_h = tmp___1 - a.x;
    } else {
        x_inc = -(1 as libc::c_int);
        tmp___2 = floor(a.x);
        temp_h = a.x - tmp___2;
    }
    if b.y > a.y {
        y_inc = 1 as libc::c_int;
        tmp___3 = floor(a.y + 1.0f32 as cpFloat);
        temp_v = tmp___3 - a.y;
    } else {
        y_inc = -(1 as libc::c_int);
        tmp___4 = floor(a.y);
        temp_v = a.y - tmp___4;
    }
    tmp___5 = cpfabs(b.x - a.x);
    dx = tmp___5;
    tmp___6 = cpfabs(b.y - a.y);
    dy = tmp___6;
    if dx != 0. {
        tmp___8 = 1.0f32 as cpFloat / dx;
    } else {
        tmp___7 = ::std::f32::INFINITY;
        tmp___8 = tmp___7 as cpFloat;
    }
    dt_dx = tmp___8;
    if dy != 0. {
        tmp___10 = 1.0f32 as cpFloat / dy;
    } else {
        tmp___9 = ::std::f32::INFINITY;
        tmp___10 = tmp___9 as cpFloat;
    }
    dt_dy = tmp___10;
    if temp_h != 0. {
        tmp___11 = temp_h * dt_dx;
    } else {
        tmp___11 = dt_dx;
    }
    next_h = tmp___11;
    if temp_v != 0. {
        tmp___12 = temp_v * dt_dy;
    } else {
        tmp___12 = dt_dy;
    }
    next_v = tmp___12;
    n = (*hash).numcells;
    table = (*hash).table;
    while t < t_exit {
        tmp___13 = hash_func(
            cell_x as cpHashValue,
            cell_y as cpHashValue,
            n as cpHashValue,
        );
        idx = tmp___13;
        tmp___14 = segmentQuery_helper(
            hash,
            table.offset(idx as isize),
            obj,
            func,
            data,
        );
        t_exit = cpfmin(t_exit, tmp___14);
        if next_v < next_h {
            cell_y += y_inc;
            t = next_v;
            next_v += dt_dy;
        } else {
            cell_x += x_inc;
            t = next_h;
            next_h += dt_dx;
        }
    }
    (*hash).stamp = ((*hash).stamp).wrapping_add(1);
}
pub unsafe extern "C" fn cpSpaceHashResize(
    mut hash: *mut cpSpaceHash,
    mut celldim: cpFloat,
    mut numcells: libc::c_int,
) {
    let mut tmp: *mut cpSpatialIndexClass = 0 as *mut cpSpatialIndexClass;
    let mut tmp___0: libc::c_int = 0;
    tmp = Klass___0();
    if (*hash).spatialIndex.klass as libc::c_ulong != tmp as libc::c_ulong {
        return;
    }
    clearTable(hash);
    (*hash).celldim = celldim;
    tmp___0 = next_prime___0(numcells);
    cpSpaceHashAllocTable(hash, tmp___0);
}
unsafe extern "C" fn cpSpaceHashCount(mut hash: *mut cpSpaceHash) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = cpHashSetCount((*hash).handleSet);
    return tmp;
}
unsafe extern "C" fn cpSpaceHashContains(
    mut hash: *mut cpSpaceHash,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) -> libc::c_int {
    let mut tmp: *const libc::c_void = 0 as *const libc::c_void;
    tmp = cpHashSetFind((*hash).handleSet, hashid, obj as *const libc::c_void);
    return (tmp as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        as libc::c_int;
}
static mut klass___10: cpSpatialIndexClass = unsafe {
    {
        let mut init = cpSpatialIndexClass {
            destroy: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSpaceHash) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> ()>,
            >(Some(cpSpaceHashDestroy as unsafe extern "C" fn(*mut cpSpaceHash) -> ())),
            count: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSpaceHash) -> libc::c_int>,
                Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> libc::c_int>,
            >(
                Some(
                    cpSpaceHashCount
                        as unsafe extern "C" fn(*mut cpSpaceHash) -> libc::c_int,
                ),
            ),
            each: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpSpaceHashEach
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                ) -> (),
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            contains: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> libc::c_int,
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> cpBool,
                >,
            >(
                Some(
                    cpSpaceHashContains
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> libc::c_int,
                ),
            ),
            insert: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
            >(
                Some(
                    cpSpaceHashInsert
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            remove: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
            >(
                Some(
                    cpSpaceHashRemove
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindex: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSpaceHash) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> ()>,
            >(Some(cpSpaceHashRehash as unsafe extern "C" fn(*mut cpSpaceHash) -> ())),
            reindexObject: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
            >(
                Some(
                    cpSpaceHashRehashObject
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindexQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpSpaceHashReindexQuery
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                    cpCollisionID,
                                    *mut libc::c_void,
                                ) -> cpCollisionID,
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            query: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpBB,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpBB,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpSpaceHashQuery
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpBB,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                    cpCollisionID,
                                    *mut libc::c_void,
                                ) -> cpCollisionID,
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpaceHash,
                        *mut libc::c_void,
                        cpVect,
                        cpVect,
                        cpFloat,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> cpFloat,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpVect,
                        cpVect,
                        cpFloat,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> cpFloat,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpSpaceHashSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpSpaceHash,
                            *mut libc::c_void,
                            cpVect,
                            cpVect,
                            cpFloat,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                ) -> cpFloat,
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
#[inline]
unsafe extern "C" fn Klass___0() -> *mut cpSpatialIndexClass {
    return &mut klass___10;
}
static mut cpvzero___9: cpVect = {
    let mut init = cpVect {
        x: 0.0f32 as cpFloat,
        y: 0.0f32 as cpFloat,
    };
    init
};
#[inline]
unsafe extern "C" fn cpSpatialIndexQuery(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    (Some(((*(*index).klass).query).expect("non-null function pointer")))
        .expect("non-null function pointer")(index, obj, bb, func, data);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexSegmentQuery(
    mut index: *mut cpSpatialIndex,
    mut obj: *mut libc::c_void,
    mut a: cpVect,
    mut b: cpVect,
    mut t_exit: cpFloat,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> cpFloat,
    >,
    mut data: *mut libc::c_void,
) {
    (Some(((*(*index).klass).segmentQuery).expect("non-null function pointer")))
        .expect("non-null function pointer")(index, obj, a, b, t_exit, func, data);
}
#[inline]
unsafe extern "C" fn cpShapeFilterReject(
    mut a: cpShapeFilter,
    mut b: cpShapeFilter,
) -> cpBool {
    let mut tmp: libc::c_int = 0;
    let mut current_block_9: u64;
    if a.group != 0 as libc::c_ulong {
        if a.group == b.group {
            tmp = 1 as libc::c_int;
            current_block_9 = 12209867499936983673;
        } else {
            current_block_9 = 17453451912164602733;
        }
    } else {
        current_block_9 = 17453451912164602733;
    }
    match current_block_9 {
        17453451912164602733 => {
            if a.categories & b.mask == 0 as libc::c_uint {
                tmp = 1 as libc::c_int;
            } else if b.categories & a.mask == 0 as libc::c_uint {
                tmp = 1 as libc::c_int;
            } else {
                tmp = 0 as libc::c_int;
            }
        }
        _ => {}
    }
    return tmp as cpBool;
}
unsafe extern "C" fn NearestPointQuery(
    mut context: *mut PointQueryContext,
    mut shape: *mut cpShape,
    mut id: cpCollisionID,
    mut data: *mut libc::c_void,
) -> cpCollisionID {
    let mut info: cpPointQueryInfo = cpPointQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        distance: 0.,
        gradient: cpVect { x: 0., y: 0. },
    };
    let mut tmp: cpBool = 0;
    tmp = cpShapeFilterReject((*shape).filter, (*context).filter);
    if tmp == 0 {
        cpShapePointQuery(shape as *const cpShape, (*context).point, &mut info);
        if !(info.shape).is_null() {
            if info.distance < (*context).maxDistance {
                (Some(((*context).func).expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(shape, info.point, info.distance, info.gradient, data);
            }
        }
    }
    return id;
}
pub unsafe extern "C" fn cpSpacePointQuery(
    mut space: *mut cpSpace,
    mut point: cpVect,
    mut maxDistance: cpFloat,
    mut filter: cpShapeFilter,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut cpShape,
            cpVect,
            cpFloat,
            cpVect,
            *mut libc::c_void,
        ) -> (),
    >,
    mut data: *mut libc::c_void,
) {
    let mut context: PointQueryContext = PointQueryContext {
        point: cpVect { x: 0., y: 0. },
        maxDistance: 0.,
        filter: cpShapeFilter {
            group: 0,
            categories: 0,
            mask: 0,
        },
        func: None,
    };
    let mut bb: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    context.point = point;
    context.maxDistance = maxDistance;
    context.filter = filter;
    context.func = func;
    tmp = cpfmax(maxDistance, 0.0f32 as cpFloat);
    tmp___0 = cpBBNewForCircle(point, tmp);
    bb = tmp___0;
    cpSpaceLock(space);
    cpSpatialIndexQuery(
        (*space).dynamicShapes,
        &mut context as *mut PointQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut PointQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
        >(
            Some(
                NearestPointQuery
                    as unsafe extern "C" fn(
                        *mut PointQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut libc::c_void,
                    ) -> cpCollisionID,
            ),
        ),
        data,
    );
    cpSpatialIndexQuery(
        (*space).staticShapes,
        &mut context as *mut PointQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut PointQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
        >(
            Some(
                NearestPointQuery
                    as unsafe extern "C" fn(
                        *mut PointQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut libc::c_void,
                    ) -> cpCollisionID,
            ),
        ),
        data,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
unsafe extern "C" fn NearestPointQueryNearest(
    mut context: *mut PointQueryContext,
    mut shape: *mut cpShape,
    mut id: cpCollisionID,
    mut out: *mut cpPointQueryInfo,
) -> cpCollisionID {
    let mut info: cpPointQueryInfo = cpPointQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        distance: 0.,
        gradient: cpVect { x: 0., y: 0. },
    };
    let mut tmp: cpBool = 0;
    tmp = cpShapeFilterReject((*shape).filter, (*context).filter);
    if tmp == 0 {
        if (*shape).sensor == 0 {
            cpShapePointQuery(shape as *const cpShape, (*context).point, &mut info);
            if info.distance < (*out).distance {
                *out = info;
            }
        }
    }
    return id;
}
pub unsafe extern "C" fn cpSpacePointQueryNearest(
    mut space: *mut cpSpace,
    mut point: cpVect,
    mut maxDistance: cpFloat,
    mut filter: cpShapeFilter,
    mut out: *mut cpPointQueryInfo,
) -> *mut cpShape {
    let mut info: cpPointQueryInfo = cpPointQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        distance: 0.,
        gradient: cpVect { x: 0., y: 0. },
    };
    let mut context: PointQueryContext = PointQueryContext {
        point: cpVect { x: 0., y: 0. },
        maxDistance: 0.,
        filter: cpShapeFilter {
            group: 0,
            categories: 0,
            mask: 0,
        },
        func: None,
    };
    let mut bb: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    info.shape = 0 as *mut libc::c_void as *const cpShape;
    info.point = cpvzero___9;
    info.distance = maxDistance;
    info.gradient = cpvzero___9;
    if !out.is_null() {
        *out = info;
    } else {
        out = &mut info;
    }
    context.point = point;
    context.maxDistance = maxDistance;
    context.filter = filter;
    context
        .func = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                *mut cpShape,
                cpVect,
                cpFloat,
                cpVect,
                *mut libc::c_void,
            ) -> (),
        >,
    >(0 as *mut libc::c_void);
    tmp = cpfmax(maxDistance, 0.0f32 as cpFloat);
    tmp___0 = cpBBNewForCircle(point, tmp);
    bb = tmp___0;
    cpSpatialIndexQuery(
        (*space).dynamicShapes,
        &mut context as *mut PointQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut PointQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut cpPointQueryInfo,
                ) -> cpCollisionID,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
        >(
            Some(
                NearestPointQueryNearest
                    as unsafe extern "C" fn(
                        *mut PointQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut cpPointQueryInfo,
                    ) -> cpCollisionID,
            ),
        ),
        out as *mut libc::c_void,
    );
    cpSpatialIndexQuery(
        (*space).staticShapes,
        &mut context as *mut PointQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut PointQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut cpPointQueryInfo,
                ) -> cpCollisionID,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
        >(
            Some(
                NearestPointQueryNearest
                    as unsafe extern "C" fn(
                        *mut PointQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut cpPointQueryInfo,
                    ) -> cpCollisionID,
            ),
        ),
        out as *mut libc::c_void,
    );
    return (*out).shape as *mut cpShape;
}
unsafe extern "C" fn SegmentQuery(
    mut context: *mut SegmentQueryContext,
    mut shape: *mut cpShape,
    mut data: *mut libc::c_void,
) -> cpFloat {
    let mut info: cpSegmentQueryInfo = cpSegmentQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        normal: cpVect { x: 0., y: 0. },
        alpha: 0.,
    };
    let mut tmp: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    tmp = cpShapeFilterReject((*shape).filter, (*context).filter);
    if tmp == 0 {
        tmp___0 = cpShapeSegmentQuery(
            shape as *const cpShape,
            (*context).start,
            (*context).end,
            (*context).radius,
            &mut info,
        );
        if tmp___0 != 0 {
            (Some(((*context).func).expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(shape, info.point, info.normal, info.alpha, data);
        }
    }
    return 1.0f32 as cpFloat;
}
pub unsafe extern "C" fn cpSpaceSegmentQuery(
    mut space: *mut cpSpace,
    mut start: cpVect,
    mut end: cpVect,
    mut radius: cpFloat,
    mut filter: cpShapeFilter,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut cpShape,
            cpVect,
            cpVect,
            cpFloat,
            *mut libc::c_void,
        ) -> (),
    >,
    mut data: *mut libc::c_void,
) {
    let mut context: SegmentQueryContext = SegmentQueryContext {
        start: cpVect { x: 0., y: 0. },
        end: cpVect { x: 0., y: 0. },
        radius: 0.,
        filter: cpShapeFilter {
            group: 0,
            categories: 0,
            mask: 0,
        },
        func: None,
    };
    context.start = start;
    context.end = end;
    context.radius = radius;
    context.filter = filter;
    context.func = func;
    cpSpaceLock(space);
    cpSpatialIndexSegmentQuery(
        (*space).staticShapes,
        &mut context as *mut SegmentQueryContext as *mut libc::c_void,
        start,
        end,
        1.0f32 as cpFloat,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut SegmentQueryContext,
                    *mut cpShape,
                    *mut libc::c_void,
                ) -> cpFloat,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> cpFloat,
            >,
        >(
            Some(
                SegmentQuery
                    as unsafe extern "C" fn(
                        *mut SegmentQueryContext,
                        *mut cpShape,
                        *mut libc::c_void,
                    ) -> cpFloat,
            ),
        ),
        data,
    );
    cpSpatialIndexSegmentQuery(
        (*space).dynamicShapes,
        &mut context as *mut SegmentQueryContext as *mut libc::c_void,
        start,
        end,
        1.0f32 as cpFloat,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut SegmentQueryContext,
                    *mut cpShape,
                    *mut libc::c_void,
                ) -> cpFloat,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> cpFloat,
            >,
        >(
            Some(
                SegmentQuery
                    as unsafe extern "C" fn(
                        *mut SegmentQueryContext,
                        *mut cpShape,
                        *mut libc::c_void,
                    ) -> cpFloat,
            ),
        ),
        data,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
unsafe extern "C" fn SegmentQueryFirst(
    mut context: *mut SegmentQueryContext,
    mut shape: *mut cpShape,
    mut out: *mut cpSegmentQueryInfo,
) -> cpFloat {
    let mut info: cpSegmentQueryInfo = cpSegmentQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        normal: cpVect { x: 0., y: 0. },
        alpha: 0.,
    };
    let mut tmp: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    tmp = cpShapeFilterReject((*shape).filter, (*context).filter);
    if tmp == 0 {
        if (*shape).sensor == 0 {
            tmp___0 = cpShapeSegmentQuery(
                shape as *const cpShape,
                (*context).start,
                (*context).end,
                (*context).radius,
                &mut info,
            );
            if tmp___0 != 0 {
                if info.alpha < (*out).alpha {
                    *out = info;
                }
            }
        }
    }
    return (*out).alpha;
}
pub unsafe extern "C" fn cpSpaceSegmentQueryFirst(
    mut space: *mut cpSpace,
    mut start: cpVect,
    mut end: cpVect,
    mut radius: cpFloat,
    mut filter: cpShapeFilter,
    mut out: *mut cpSegmentQueryInfo,
) -> *mut cpShape {
    let mut info: cpSegmentQueryInfo = cpSegmentQueryInfo {
        shape: 0 as *const cpShape,
        point: cpVect { x: 0., y: 0. },
        normal: cpVect { x: 0., y: 0. },
        alpha: 0.,
    };
    let mut context: SegmentQueryContext = SegmentQueryContext {
        start: cpVect { x: 0., y: 0. },
        end: cpVect { x: 0., y: 0. },
        radius: 0.,
        filter: cpShapeFilter {
            group: 0,
            categories: 0,
            mask: 0,
        },
        func: None,
    };
    info.shape = 0 as *mut libc::c_void as *const cpShape;
    info.point = end;
    info.normal = cpvzero___9;
    info.alpha = 1.0f32 as cpFloat;
    if !out.is_null() {
        *out = info;
    } else {
        out = &mut info;
    }
    context.start = start;
    context.end = end;
    context.radius = radius;
    context.filter = filter;
    context
        .func = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                *mut cpShape,
                cpVect,
                cpVect,
                cpFloat,
                *mut libc::c_void,
            ) -> (),
        >,
    >(0 as *mut libc::c_void);
    cpSpatialIndexSegmentQuery(
        (*space).staticShapes,
        &mut context as *mut SegmentQueryContext as *mut libc::c_void,
        start,
        end,
        1.0f32 as cpFloat,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut SegmentQueryContext,
                    *mut cpShape,
                    *mut cpSegmentQueryInfo,
                ) -> cpFloat,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> cpFloat,
            >,
        >(
            Some(
                SegmentQueryFirst
                    as unsafe extern "C" fn(
                        *mut SegmentQueryContext,
                        *mut cpShape,
                        *mut cpSegmentQueryInfo,
                    ) -> cpFloat,
            ),
        ),
        out as *mut libc::c_void,
    );
    cpSpatialIndexSegmentQuery(
        (*space).dynamicShapes,
        &mut context as *mut SegmentQueryContext as *mut libc::c_void,
        start,
        end,
        (*out).alpha,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut SegmentQueryContext,
                    *mut cpShape,
                    *mut cpSegmentQueryInfo,
                ) -> cpFloat,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> cpFloat,
            >,
        >(
            Some(
                SegmentQueryFirst
                    as unsafe extern "C" fn(
                        *mut SegmentQueryContext,
                        *mut cpShape,
                        *mut cpSegmentQueryInfo,
                    ) -> cpFloat,
            ),
        ),
        out as *mut libc::c_void,
    );
    return (*out).shape as *mut cpShape;
}
unsafe extern "C" fn BBQuery(
    mut context: *mut BBQueryContext,
    mut shape: *mut cpShape,
    mut id: cpCollisionID,
    mut data: *mut libc::c_void,
) -> cpCollisionID {
    let mut tmp: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    tmp = cpShapeFilterReject((*shape).filter, (*context).filter);
    if tmp == 0 {
        tmp___0 = cpBBIntersects((*context).bb, (*shape).bb);
        if tmp___0 != 0 {
            (Some(((*context).func).expect("non-null function pointer")))
                .expect("non-null function pointer")(shape, data);
        }
    }
    return id;
}
pub unsafe extern "C" fn cpSpaceBBQuery(
    mut space: *mut cpSpace,
    mut bb: cpBB,
    mut filter: cpShapeFilter,
    mut func: Option::<unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    let mut context: BBQueryContext = BBQueryContext {
        bb: cpBB { l: 0., b: 0., r: 0., t: 0. },
        filter: cpShapeFilter {
            group: 0,
            categories: 0,
            mask: 0,
        },
        func: None,
    };
    context.bb = bb;
    context.filter = filter;
    context.func = func;
    cpSpaceLock(space);
    cpSpatialIndexQuery(
        (*space).dynamicShapes,
        &mut context as *mut BBQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut BBQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
        >(
            Some(
                BBQuery
                    as unsafe extern "C" fn(
                        *mut BBQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut libc::c_void,
                    ) -> cpCollisionID,
            ),
        ),
        data,
    );
    cpSpatialIndexQuery(
        (*space).staticShapes,
        &mut context as *mut BBQueryContext as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut BBQueryContext,
                    *mut cpShape,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
        >(
            Some(
                BBQuery
                    as unsafe extern "C" fn(
                        *mut BBQueryContext,
                        *mut cpShape,
                        cpCollisionID,
                        *mut libc::c_void,
                    ) -> cpCollisionID,
            ),
        ),
        data,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
unsafe extern "C" fn ShapeQuery(
    mut a: *mut cpShape,
    mut b: *mut cpShape,
    mut id: cpCollisionID,
    mut context: *mut ShapeQueryContext,
) -> cpCollisionID {
    let mut tmp: cpBool = 0;
    let mut set: cpContactPointSet = cpContactPointSet {
        count: 0,
        normal: cpVect { x: 0., y: 0. },
        points: [__anonstruct_points_450528349 {
            pointA: cpVect { x: 0., y: 0. },
            pointB: cpVect { x: 0., y: 0. },
            distance: 0.,
        }; 2],
    };
    let mut tmp___0: cpContactPointSet = cpContactPointSet {
        count: 0,
        normal: cpVect { x: 0., y: 0. },
        points: [__anonstruct_points_450528349 {
            pointA: cpVect { x: 0., y: 0. },
            pointB: cpVect { x: 0., y: 0. },
            distance: 0.,
        }; 2],
    };
    let mut tmp___1: libc::c_int = 0;
    tmp = cpShapeFilterReject((*a).filter, (*b).filter);
    if tmp != 0 {
        return id
    } else {
        if a as libc::c_ulong == b as libc::c_ulong {
            return id;
        }
    }
    tmp___0 = cpShapesCollide(a as *const cpShape, b as *const cpShape);
    set = tmp___0;
    if set.count != 0 {
        if ((*context).func).is_some() {
            (Some(((*context).func).expect("non-null function pointer")))
                .expect("non-null function pointer")(b, &mut set, (*context).data);
        }
        if (*a).sensor != 0 {
            tmp___1 = 0 as libc::c_int;
        } else if (*b).sensor != 0 {
            tmp___1 = 0 as libc::c_int;
        } else {
            tmp___1 = 1 as libc::c_int;
        }
        (*context).anyCollision = tmp___1 as cpBool;
    }
    return id;
}
pub unsafe extern "C" fn cpSpaceShapeQuery(
    mut space: *mut cpSpace,
    mut shape: *mut cpShape,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut cpShape,
            *mut cpContactPointSet,
            *mut libc::c_void,
        ) -> (),
    >,
    mut data: *mut libc::c_void,
) -> cpBool {
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut bb: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp___0: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut context: ShapeQueryContext = ShapeQueryContext {
        func: None,
        data: 0 as *mut libc::c_void,
        anyCollision: 0,
    };
    body = (*shape).body;
    if !body.is_null() {
        tmp = cpShapeUpdate(shape, (*body).transform);
        tmp___0 = tmp;
    } else {
        tmp___0 = (*shape).bb;
    }
    bb = tmp___0;
    context.func = func;
    context.data = data;
    context.anyCollision = 0 as libc::c_int as cpBool;
    cpSpaceLock(space);
    cpSpatialIndexQuery(
        (*space).dynamicShapes,
        shape as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpShape,
                    *mut cpShape,
                    cpCollisionID,
                    *mut ShapeQueryContext,
                ) -> cpCollisionID,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
        >(
            Some(
                ShapeQuery
                    as unsafe extern "C" fn(
                        *mut cpShape,
                        *mut cpShape,
                        cpCollisionID,
                        *mut ShapeQueryContext,
                    ) -> cpCollisionID,
            ),
        ),
        &mut context as *mut ShapeQueryContext as *mut libc::c_void,
    );
    cpSpatialIndexQuery(
        (*space).staticShapes,
        shape as *mut libc::c_void,
        bb,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpShape,
                    *mut cpShape,
                    cpCollisionID,
                    *mut ShapeQueryContext,
                ) -> cpCollisionID,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
        >(
            Some(
                ShapeQuery
                    as unsafe extern "C" fn(
                        *mut cpShape,
                        *mut cpShape,
                        cpCollisionID,
                        *mut ShapeQueryContext,
                    ) -> cpCollisionID,
            ),
        ),
        &mut context as *mut ShapeQueryContext as *mut libc::c_void,
    );
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
    return context.anyCollision;
}
pub unsafe extern "C" fn cpSpaceGetPostStepCallback(
    mut space: *mut cpSpace,
    mut key: *mut libc::c_void,
) -> *mut cpPostStepCallback {
    let mut arr: *mut cpArray = 0 as *mut cpArray;
    let mut i: libc::c_int = 0;
    let mut callback: *mut cpPostStepCallback = 0 as *mut cpPostStepCallback;
    arr = (*space).postStepCallbacks;
    i = 0 as libc::c_int;
    while i < (*arr).num {
        callback = *((*arr).arr).offset(i as isize) as *mut cpPostStepCallback;
        if !callback.is_null() {
            if (*callback).key as libc::c_ulong == key as libc::c_ulong {
                return callback;
            }
        }
        i += 1;
    }
    return 0 as *mut libc::c_void as *mut cpPostStepCallback;
}
unsafe extern "C" fn PostStepDoNothing(
    mut space: *mut cpSpace,
    mut obj: *mut libc::c_void,
    mut data: *mut libc::c_void,
) {}
pub unsafe extern "C" fn cpSpaceAddPostStepCallback(
    mut space: *mut cpSpace,
    mut func: Option::<
        unsafe extern "C" fn(*mut cpSpace, *mut libc::c_void, *mut libc::c_void) -> (),
    >,
    mut key: *mut libc::c_void,
    mut data: *mut libc::c_void,
) -> cpBool {
    let mut callback: *mut cpPostStepCallback = 0 as *mut cpPostStepCallback;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___0: *mut cpPostStepCallback = 0 as *mut cpPostStepCallback;
    tmp___0 = cpSpaceGetPostStepCallback(space, key);
    if !tmp___0.is_null() {
        return 0 as libc::c_int as cpBool
    } else {
        tmp = calloc(
            1 as libc::c_int as size_t,
            ::std::mem::size_of::<cpPostStepCallback>() as libc::c_ulong,
        );
        callback = tmp as *mut cpPostStepCallback;
        if func.is_some() {
            (*callback).func = func;
        } else {
            (*callback)
                .func = Some(
                PostStepDoNothing
                    as unsafe extern "C" fn(
                        *mut cpSpace,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            );
        }
        (*callback).key = key;
        (*callback).data = data;
        cpArrayPush((*space).postStepCallbacks, callback as *mut libc::c_void);
        return 1 as libc::c_int as cpBool;
    };
}
pub unsafe extern "C" fn cpSpaceLock(mut space: *mut cpSpace) {
    (*space).locked += 1;
}
pub unsafe extern "C" fn cpSpaceUnlock(
    mut space: *mut cpSpace,
    mut runPostStep: cpBool,
) {
    let mut waking: *mut cpArray = 0 as *mut cpArray;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut arr: *mut cpArray = 0 as *mut cpArray;
    let mut i___0: libc::c_int = 0;
    let mut callback: *mut cpPostStepCallback = 0 as *mut cpPostStepCallback;
    let mut func: Option::<
        unsafe extern "C" fn(*mut cpSpace, *mut libc::c_void, *mut libc::c_void) -> (),
    > = None;
    (*space).locked -= 1;
    if !((*space).locked >= 0 as libc::c_int) {
        cpMessage(
            b"space->locked >= 0\0" as *const u8 as *const libc::c_char,
            b"../src/cpSpaceStep.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Internal Error: Space lock underflow.\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    if (*space).locked == 0 as libc::c_int {
        waking = (*space).rousedBodies;
        i = 0 as libc::c_int;
        count = (*waking).num;
        while i < count {
            cpSpaceActivateBody(
                space,
                *((*waking).arr).offset(i as isize) as *mut cpBody,
            );
            let ref mut fresh28 = *((*waking).arr).offset(i as isize);
            *fresh28 = 0 as *mut libc::c_void;
            i += 1;
        }
        (*waking).num = 0 as libc::c_int;
        if (*space).locked == 0 as libc::c_int {
            if runPostStep != 0 {
                if (*space).skipPostStep == 0 {
                    (*space).skipPostStep = 1 as libc::c_int as cpBool;
                    arr = (*space).postStepCallbacks;
                    i___0 = 0 as libc::c_int;
                    while i___0 < (*arr).num {
                        callback = *((*arr).arr).offset(i___0 as isize)
                            as *mut cpPostStepCallback;
                        func = (*callback).func;
                        (*callback)
                            .func = ::std::mem::transmute::<
                            *mut libc::c_void,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut cpSpace,
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                ) -> (),
                            >,
                        >(0 as *mut libc::c_void);
                        if func.is_some() {
                            (Some(func.expect("non-null function pointer")))
                                .expect(
                                    "non-null function pointer",
                                )(space, (*callback).key, (*callback).data);
                        }
                        let ref mut fresh29 = *((*arr).arr).offset(i___0 as isize);
                        *fresh29 = 0 as *mut libc::c_void;
                        free(callback as *mut libc::c_void);
                        i___0 += 1;
                    }
                    (*arr).num = 0 as libc::c_int;
                    (*space).skipPostStep = 0 as libc::c_int as cpBool;
                }
            }
        }
    }
}
unsafe extern "C" fn cpSpaceAllocContactBuffer(
    mut space: *mut cpSpace,
) -> *mut cpContactBufferHeader {
    let mut buffer: *mut cpContactBuffer = 0 as *mut cpContactBuffer;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpContactBuffer>() as libc::c_ulong,
    );
    buffer = tmp as *mut cpContactBuffer;
    cpArrayPush((*space).allocatedBuffers, buffer as *mut libc::c_void);
    return buffer as *mut cpContactBufferHeader;
}
unsafe extern "C" fn cpContactBufferHeaderInit(
    mut header: *mut cpContactBufferHeader,
    mut stamp: cpTimestamp,
    mut splice: *mut cpContactBufferHeader,
) -> *mut cpContactBufferHeader {
    (*header).stamp = stamp;
    if !splice.is_null() {
        (*header).next = (*splice).next;
    } else {
        (*header).next = header;
    }
    (*header).numContacts = 0 as libc::c_uint;
    return header;
}
pub unsafe extern "C" fn cpSpacePushFreshContactBuffer(mut space: *mut cpSpace) {
    let mut stamp: cpTimestamp = 0;
    let mut head: *mut cpContactBufferHeader = 0 as *mut cpContactBufferHeader;
    let mut tmp: *mut cpContactBufferHeader = 0 as *mut cpContactBufferHeader;
    let mut tail: *mut cpContactBufferHeader = 0 as *mut cpContactBufferHeader;
    let mut buffer: *mut cpContactBufferHeader = 0 as *mut cpContactBufferHeader;
    let mut tmp___0: *mut cpContactBufferHeader = 0 as *mut cpContactBufferHeader;
    let mut tmp___1: *mut cpContactBufferHeader = 0 as *mut cpContactBufferHeader;
    let mut tmp___2: *mut cpContactBufferHeader = 0 as *mut cpContactBufferHeader;
    stamp = (*space).stamp;
    head = (*space).contactBuffersHead;
    if head.is_null() {
        tmp = cpSpaceAllocContactBuffer(space);
        (*space)
            .contactBuffersHead = cpContactBufferHeaderInit(
            tmp,
            stamp,
            0 as *mut libc::c_void as *mut cpContactBufferHeader,
        );
    } else if stamp.wrapping_sub((*(*head).next).stamp) > (*space).collisionPersistence {
        tail = (*head).next;
        (*space).contactBuffersHead = cpContactBufferHeaderInit(tail, stamp, tail);
    } else {
        tmp___0 = cpSpaceAllocContactBuffer(space);
        tmp___1 = cpContactBufferHeaderInit(tmp___0, stamp, head);
        buffer = tmp___1;
        tmp___2 = buffer;
        (*head).next = tmp___2;
        (*space).contactBuffersHead = tmp___2;
    };
}
pub unsafe extern "C" fn cpContactBufferGetArray(
    mut space: *mut cpSpace,
) -> *mut cpContact {
    let mut head: *mut cpContactBufferHeader = 0 as *mut cpContactBufferHeader;
    if ((*(*space).contactBuffersHead).numContacts).wrapping_add(2 as libc::c_uint)
        as libc::c_ulong
        > (32768 as libc::c_ulong)
            .wrapping_sub(
                ::std::mem::size_of::<cpContactBufferHeader>() as libc::c_ulong,
            )
            .wrapping_div(::std::mem::size_of::<cpContact>() as libc::c_ulong)
    {
        cpSpacePushFreshContactBuffer(space);
    }
    head = (*space).contactBuffersHead;
    return ((*(head as *mut cpContactBuffer)).contacts)
        .as_mut_ptr()
        .offset((*head).numContacts as isize);
}
pub unsafe extern "C" fn cpSpacePushContacts(
    mut space: *mut cpSpace,
    mut count: libc::c_int,
) {
    if !(count <= 2 as libc::c_int) {
        cpMessage(
            b"count <= CP_MAX_CONTACTS_PER_ARBITER\0" as *const u8
                as *const libc::c_char,
            b"../src/cpSpaceStep.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            1 as libc::c_int,
            1 as libc::c_int,
            b"Internal Error: Contact buffer overflow!\0" as *const u8
                as *const libc::c_char,
        );
        abort();
    }
    (*(*space).contactBuffersHead)
        .numContacts = ((*(*space).contactBuffersHead).numContacts)
        .wrapping_add(count as libc::c_uint);
}
unsafe extern "C" fn cpSpacePopContacts(
    mut space: *mut cpSpace,
    mut count: libc::c_int,
) {
    (*(*space).contactBuffersHead)
        .numContacts = ((*(*space).contactBuffersHead).numContacts)
        .wrapping_sub(count as libc::c_uint);
}
unsafe extern "C" fn cpSpaceArbiterSetTrans(
    mut shapes: *mut *mut cpShape,
    mut space: *mut cpSpace,
) -> *mut libc::c_void {
    let mut count: libc::c_int = 0;
    let mut buffer: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut cpArbiter = 0 as *mut cpArbiter;
    if (*(*space).pooledArbiters).num == 0 as libc::c_int {
        count = (32768 as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<cpArbiter>() as libc::c_ulong)
            as libc::c_int;
        if count == 0 {
            cpMessage(
                b"count\0" as *const u8 as *const libc::c_char,
                b"../src/cpSpaceStep.c\0" as *const u8 as *const libc::c_char,
                193 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"Internal Error: Buffer size too small.\0" as *const u8
                    as *const libc::c_char,
            );
            abort();
        }
        tmp = calloc(1 as libc::c_int as size_t, 32768 as libc::c_int as size_t);
        buffer = tmp as *mut cpArbiter;
        cpArrayPush((*space).allocatedBuffers, buffer as *mut libc::c_void);
        i = 0 as libc::c_int;
        while i < count {
            cpArrayPush(
                (*space).pooledArbiters,
                buffer.offset(i as isize) as *mut libc::c_void,
            );
            i += 1;
        }
    }
    tmp___0 = cpArrayPop((*space).pooledArbiters);
    tmp___1 = cpArbiterInit(
        tmp___0 as *mut cpArbiter,
        *shapes.offset(0 as libc::c_int as isize),
        *shapes.offset(1 as libc::c_int as isize),
    );
    return tmp___1 as *mut libc::c_void;
}
#[inline]
unsafe extern "C" fn QueryRejectConstraint(
    mut a: *mut cpBody,
    mut b: *mut cpBody,
) -> cpBool {
    let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
    constraint = (*a).constraintList;
    while !constraint.is_null() {
        if (*constraint).collideBodies == 0 {
            if (*constraint).a as libc::c_ulong == a as libc::c_ulong {
                if (*constraint).b as libc::c_ulong == b as libc::c_ulong {
                    return 1 as libc::c_int as cpBool;
                }
            }
            if (*constraint).a as libc::c_ulong == b as libc::c_ulong {
                if (*constraint).b as libc::c_ulong == a as libc::c_ulong {
                    return 1 as libc::c_int as cpBool;
                }
            }
        }
        constraint = cpConstraintNext(constraint, a);
    }
    return 0 as libc::c_int as cpBool;
}
#[inline]
unsafe extern "C" fn QueryReject(mut a: *mut cpShape, mut b: *mut cpShape) -> cpBool {
    let mut tmp: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    let mut tmp___1: cpBool = 0;
    let mut tmp___2: libc::c_int = 0;
    tmp = cpBBIntersects((*a).bb, (*b).bb);
    if tmp != 0 {
        if (*a).body as libc::c_ulong == (*b).body as libc::c_ulong {
            tmp___2 = 1 as libc::c_int;
        } else {
            tmp___0 = cpShapeFilterReject((*a).filter, (*b).filter);
            if tmp___0 != 0 {
                tmp___2 = 1 as libc::c_int;
            } else {
                tmp___1 = QueryRejectConstraint((*a).body, (*b).body);
                if tmp___1 != 0 {
                    tmp___2 = 1 as libc::c_int;
                } else {
                    tmp___2 = 0 as libc::c_int;
                }
            }
        }
    } else {
        tmp___2 = 1 as libc::c_int;
    }
    return tmp___2 as cpBool;
}
pub unsafe extern "C" fn cpSpaceCollideShapes(
    mut a: *mut cpShape,
    mut b: *mut cpShape,
    mut id: cpCollisionID,
    mut space: *mut cpSpace,
) -> cpCollisionID {
    let mut current_block: u64;
    let mut tmp: cpBool = 0;
    let mut info: cpCollisionInfo = cpCollisionInfo {
        a: 0 as *const cpShape,
        b: 0 as *const cpShape,
        id: 0,
        n: cpVect { x: 0., y: 0. },
        count: 0,
        arr: 0 as *mut cpContact,
    };
    let mut tmp___0: *mut cpContact = 0 as *mut cpContact;
    let mut tmp___1: cpCollisionInfo = cpCollisionInfo {
        a: 0 as *const cpShape,
        b: 0 as *const cpShape,
        id: 0,
        n: cpVect { x: 0., y: 0. },
        count: 0,
        arr: 0 as *mut cpContact,
    };
    let mut shape_pair: [*const cpShape; 2] = [0 as *const cpShape; 2];
    let mut arbHashID: cpHashValue = 0;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut tmp___2: *const libc::c_void = 0 as *const libc::c_void;
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    let mut tmp___3: cpBool = 0;
    let mut tmp___4: cpBool = 0;
    let mut tmp___5: libc::c_float = 0.;
    let mut tmp___6: libc::c_float = 0.;
    tmp = QueryReject(a, b);
    if tmp != 0 {
        return id;
    }
    tmp___0 = cpContactBufferGetArray(space);
    tmp___1 = cpCollide(a as *const cpShape, b as *const cpShape, id, tmp___0);
    info = tmp___1;
    if info.count == 0 as libc::c_int {
        return info.id;
    }
    cpSpacePushContacts(space, info.count);
    shape_pair[0 as libc::c_int as usize] = info.a;
    shape_pair[1 as libc::c_int as usize] = info.b;
    arbHashID = (info.a as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong)
        ^ (info.b as cpHashValue).wrapping_mul(3344921057 as libc::c_ulong);
    tmp___2 = cpHashSetInsert(
        (*space).cachedArbiters,
        arbHashID,
        shape_pair.as_mut_ptr() as *const libc::c_void,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut *mut cpShape,
                    *mut cpSpace,
                ) -> *mut libc::c_void,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
        >(
            Some(
                cpSpaceArbiterSetTrans
                    as unsafe extern "C" fn(
                        *mut *mut cpShape,
                        *mut cpSpace,
                    ) -> *mut libc::c_void,
            ),
        ),
        space as *mut libc::c_void,
    );
    arb = tmp___2 as *mut cpArbiter;
    cpArbiterUpdate(arb, &mut info, space);
    handler = (*arb).handler;
    if (*arb).state as libc::c_uint == 0 as libc::c_uint {
        tmp___3 = (Some(((*handler).beginFunc).expect("non-null function pointer")))
            .expect("non-null function pointer")(arb, space, (*handler).userData);
        if tmp___3 == 0 {
            cpArbiterIgnore(arb);
        }
    }
    if (*arb).state as libc::c_uint != 2 as libc::c_uint {
        tmp___4 = (Some(((*handler).preSolveFunc).expect("non-null function pointer")))
            .expect("non-null function pointer")(arb, space, (*handler).userData);
        if tmp___4 != 0 {
            if (*arb).state as libc::c_uint != 2 as libc::c_uint {
                if (*a).sensor != 0 {
                    current_block = 1081730029988001839;
                } else if (*b).sensor != 0 {
                    current_block = 1081730029988001839;
                } else {
                    tmp___5 = ::std::f32::INFINITY;
                    if (*(*a).body).m == tmp___5 as cpFloat {
                        tmp___6 = ::std::f32::INFINITY;
                        if (*(*b).body).m == tmp___6 as cpFloat {
                            current_block = 1081730029988001839;
                        } else {
                            cpArrayPush((*space).arbiters, arb as *mut libc::c_void);
                            current_block = 1847472278776910194;
                        }
                    } else {
                        cpArrayPush((*space).arbiters, arb as *mut libc::c_void);
                        current_block = 1847472278776910194;
                    }
                }
            } else {
                current_block = 1081730029988001839;
            }
        } else {
            current_block = 1081730029988001839;
        }
    } else {
        current_block = 1081730029988001839;
    }
    match current_block {
        1081730029988001839 => {
            cpSpacePopContacts(space, info.count);
            (*arb).contacts = 0 as *mut libc::c_void as *mut cpContact;
            (*arb).count = 0 as libc::c_int;
            if (*arb).state as libc::c_uint != 2 as libc::c_uint {
                (*arb).state = CP_ARBITER_STATE_NORMAL;
            }
        }
        _ => {}
    }
    (*arb).stamp = (*space).stamp;
    return info.id;
}
pub unsafe extern "C" fn cpSpaceArbiterSetFilter(
    mut arb: *mut cpArbiter,
    mut space: *mut cpSpace,
) -> cpBool {
    let mut ticks: cpTimestamp = 0;
    let mut a: *mut cpBody = 0 as *mut cpBody;
    let mut b: *mut cpBody = 0 as *mut cpBody;
    let mut tmp: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___0: cpBool = 0;
    let mut tmp___1: cpBodyType = CP_BODY_TYPE_DYNAMIC;
    let mut tmp___2: cpBool = 0;
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    ticks = ((*space).stamp).wrapping_sub((*arb).stamp);
    a = (*arb).body_a;
    b = (*arb).body_b;
    tmp = cpBodyGetType(a);
    let mut current_block_13: u64;
    if tmp as libc::c_uint == 2 as libc::c_uint {
        current_block_13 = 10765626082782879053;
    } else {
        tmp___0 = cpBodyIsSleeping(a as *const cpBody);
        if tmp___0 != 0 {
            current_block_13 = 10765626082782879053;
        } else {
            current_block_13 = 8831408221741692167;
        }
    }
    match current_block_13 {
        10765626082782879053 => {
            tmp___1 = cpBodyGetType(b);
            if tmp___1 as libc::c_uint == 2 as libc::c_uint {
                return 1 as libc::c_int as cpBool
            } else {
                tmp___2 = cpBodyIsSleeping(b as *const cpBody);
                if tmp___2 != 0 {
                    return 1 as libc::c_int as cpBool;
                }
            }
        }
        _ => {}
    }
    if ticks >= 1 as libc::c_uint {
        if (*arb).state as libc::c_uint != 3 as libc::c_uint {
            (*arb).state = CP_ARBITER_STATE_CACHED;
            handler = (*arb).handler;
            (Some(((*handler).separateFunc).expect("non-null function pointer")))
                .expect("non-null function pointer")(arb, space, (*handler).userData);
        }
    }
    if ticks >= (*space).collisionPersistence {
        (*arb).contacts = 0 as *mut libc::c_void as *mut cpContact;
        (*arb).count = 0 as libc::c_int;
        cpArrayPush((*space).pooledArbiters, arb as *mut libc::c_void);
        return 0 as libc::c_int as cpBool;
    }
    return 1 as libc::c_int as cpBool;
}
pub unsafe extern "C" fn cpShapeUpdateFunc(
    mut shape: *mut cpShape,
    mut unused: *mut libc::c_void,
) {
    cpShapeCacheBB(shape);
}
pub unsafe extern "C" fn cpSpaceStep(mut space: *mut cpSpace, mut dt: cpFloat) {
    let mut prev_dt: cpFloat = 0.;
    let mut bodies: *mut cpArray = 0 as *mut cpArray;
    let mut constraints: *mut cpArray = 0 as *mut cpArray;
    let mut arbiters: *mut cpArray = 0 as *mut cpArray;
    let mut i: libc::c_int = 0;
    let mut arb: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut tmp: cpBool = 0;
    let mut tmp___0: cpBool = 0;
    let mut i___0: libc::c_int = 0;
    let mut body: *mut cpBody = 0 as *mut cpBody;
    let mut slop: cpFloat = 0.;
    let mut biasCoef: cpFloat = 0.;
    let mut tmp___1: libc::c_double = 0.;
    let mut i___1: libc::c_int = 0;
    let mut i___2: libc::c_int = 0;
    let mut constraint: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut preSolve: Option::<
        unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
    > = None;
    let mut damping: cpFloat = 0.;
    let mut tmp___2: libc::c_double = 0.;
    let mut gravity: cpVect = cpVect { x: 0., y: 0. };
    let mut i___3: libc::c_int = 0;
    let mut body___0: *mut cpBody = 0 as *mut cpBody;
    let mut dt_coef: cpFloat = 0.;
    let mut tmp___3: cpFloat = 0.;
    let mut i___4: libc::c_int = 0;
    let mut i___5: libc::c_int = 0;
    let mut constraint___0: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut i___6: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut j___0: libc::c_int = 0;
    let mut constraint___1: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut i___7: libc::c_int = 0;
    let mut constraint___2: *mut cpConstraint = 0 as *mut cpConstraint;
    let mut postSolve: Option::<
        unsafe extern "C" fn(*mut cpConstraint, *mut cpSpace) -> (),
    > = None;
    let mut i___8: libc::c_int = 0;
    let mut arb___0: *mut cpArbiter = 0 as *mut cpArbiter;
    let mut handler: *mut cpCollisionHandler = 0 as *mut cpCollisionHandler;
    if dt == 0.0f32 as cpFloat {
        return;
    }
    (*space).stamp = ((*space).stamp).wrapping_add(1);
    prev_dt = (*space).curr_dt;
    (*space).curr_dt = dt;
    bodies = (*space).dynamicBodies;
    constraints = (*space).constraints;
    arbiters = (*space).arbiters;
    i = 0 as libc::c_int;
    while i < (*arbiters).num {
        arb = *((*arbiters).arr).offset(i as isize) as *mut cpArbiter;
        (*arb).state = CP_ARBITER_STATE_NORMAL;
        tmp = cpBodyIsSleeping((*arb).body_a as *const cpBody);
        if tmp == 0 {
            tmp___0 = cpBodyIsSleeping((*arb).body_b as *const cpBody);
            if tmp___0 == 0 {
                cpArbiterUnthread(arb);
            }
        }
        i += 1;
    }
    (*arbiters).num = 0 as libc::c_int;
    cpSpaceLock(space);
    i___0 = 0 as libc::c_int;
    while i___0 < (*bodies).num {
        body = *((*bodies).arr).offset(i___0 as isize) as *mut cpBody;
        (Some(((*body).position_func).expect("non-null function pointer")))
            .expect("non-null function pointer")(body, dt);
        i___0 += 1;
    }
    cpSpacePushFreshContactBuffer(space);
    cpSpatialIndexEach(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
        >(
            Some(
                cpShapeUpdateFunc
                    as unsafe extern "C" fn(*mut cpShape, *mut libc::c_void) -> (),
            ),
        ),
        0 as *mut libc::c_void,
    );
    cpSpatialIndexReindexQuery(
        (*space).dynamicShapes,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut cpShape,
                    *mut cpShape,
                    cpCollisionID,
                    *mut cpSpace,
                ) -> cpCollisionID,
            >,
            Option::<
                unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    cpCollisionID,
                    *mut libc::c_void,
                ) -> cpCollisionID,
            >,
        >(
            Some(
                cpSpaceCollideShapes
                    as unsafe extern "C" fn(
                        *mut cpShape,
                        *mut cpShape,
                        cpCollisionID,
                        *mut cpSpace,
                    ) -> cpCollisionID,
            ),
        ),
        space as *mut libc::c_void,
    );
    cpSpaceUnlock(space, 0 as libc::c_int as cpBool);
    cpSpaceProcessComponents(space, dt);
    cpSpaceLock(space);
    cpHashSetFilter(
        (*space).cachedArbiters,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace) -> cpBool>,
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> cpBool,
            >,
        >(
            Some(
                cpSpaceArbiterSetFilter
                    as unsafe extern "C" fn(*mut cpArbiter, *mut cpSpace) -> cpBool,
            ),
        ),
        space as *mut libc::c_void,
    );
    slop = (*space).collisionSlop;
    tmp___1 = pow((*space).collisionBias, dt);
    biasCoef = 1.0f32 as libc::c_double - tmp___1;
    i___1 = 0 as libc::c_int;
    while i___1 < (*arbiters).num {
        cpArbiterPreStep(
            *((*arbiters).arr).offset(i___1 as isize) as *mut cpArbiter,
            dt,
            slop,
            biasCoef,
        );
        i___1 += 1;
    }
    i___2 = 0 as libc::c_int;
    while i___2 < (*constraints).num {
        constraint = *((*constraints).arr).offset(i___2 as isize) as *mut cpConstraint;
        preSolve = (*constraint).preSolve;
        if preSolve.is_some() {
            (Some(preSolve.expect("non-null function pointer")))
                .expect("non-null function pointer")(constraint, space);
        }
        (Some(((*(*constraint).klass).preStep).expect("non-null function pointer")))
            .expect("non-null function pointer")(constraint, dt);
        i___2 += 1;
    }
    tmp___2 = pow((*space).damping, dt);
    damping = tmp___2;
    gravity = (*space).gravity;
    i___3 = 0 as libc::c_int;
    while i___3 < (*bodies).num {
        body___0 = *((*bodies).arr).offset(i___3 as isize) as *mut cpBody;
        (Some(((*body___0).velocity_func).expect("non-null function pointer")))
            .expect("non-null function pointer")(body___0, gravity, damping, dt);
        i___3 += 1;
    }
    if prev_dt == 0.0f32 as cpFloat {
        tmp___3 = 0.0f32 as cpFloat;
    } else {
        tmp___3 = dt / prev_dt;
    }
    dt_coef = tmp___3;
    i___4 = 0 as libc::c_int;
    while i___4 < (*arbiters).num {
        cpArbiterApplyCachedImpulse(
            *((*arbiters).arr).offset(i___4 as isize) as *mut cpArbiter,
            dt_coef,
        );
        i___4 += 1;
    }
    i___5 = 0 as libc::c_int;
    while i___5 < (*constraints).num {
        constraint___0 = *((*constraints).arr).offset(i___5 as isize)
            as *mut cpConstraint;
        (Some(
            ((*(*constraint___0).klass).applyCachedImpulse)
                .expect("non-null function pointer"),
        ))
            .expect("non-null function pointer")(constraint___0, dt_coef);
        i___5 += 1;
    }
    i___6 = 0 as libc::c_int;
    while i___6 < (*space).iterations {
        j = 0 as libc::c_int;
        while j < (*arbiters).num {
            cpArbiterApplyImpulse(
                *((*arbiters).arr).offset(j as isize) as *mut cpArbiter,
            );
            j += 1;
        }
        j___0 = 0 as libc::c_int;
        while j___0 < (*constraints).num {
            constraint___1 = *((*constraints).arr).offset(j___0 as isize)
                as *mut cpConstraint;
            (Some(
                ((*(*constraint___1).klass).applyImpulse)
                    .expect("non-null function pointer"),
            ))
                .expect("non-null function pointer")(constraint___1, dt);
            j___0 += 1;
        }
        i___6 += 1;
    }
    i___7 = 0 as libc::c_int;
    while i___7 < (*constraints).num {
        constraint___2 = *((*constraints).arr).offset(i___7 as isize)
            as *mut cpConstraint;
        postSolve = (*constraint___2).postSolve;
        if postSolve.is_some() {
            (Some(postSolve.expect("non-null function pointer")))
                .expect("non-null function pointer")(constraint___2, space);
        }
        i___7 += 1;
    }
    i___8 = 0 as libc::c_int;
    while i___8 < (*arbiters).num {
        arb___0 = *((*arbiters).arr).offset(i___8 as isize) as *mut cpArbiter;
        handler = (*arb___0).handler;
        (Some(((*handler).postSolveFunc).expect("non-null function pointer")))
            .expect("non-null function pointer")(arb___0, space, (*handler).userData);
        i___8 += 1;
    }
    cpSpaceUnlock(space, 1 as libc::c_int as cpBool);
}
#[inline]
unsafe extern "C" fn cpSpatialIndexDestroy(mut index: *mut cpSpatialIndex) {
    if !((*index).klass).is_null() {
        (Some(((*(*index).klass).destroy).expect("non-null function pointer")))
            .expect("non-null function pointer")(index);
    }
}
#[inline]
unsafe extern "C" fn cpSpatialIndexCount(mut index: *mut cpSpatialIndex) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    tmp = (Some(((*(*index).klass).count).expect("non-null function pointer")))
        .expect("non-null function pointer")(index);
    return tmp;
}
pub unsafe extern "C" fn cpSpatialIndexFree(mut index: *mut cpSpatialIndex) {
    if !index.is_null() {
        cpSpatialIndexDestroy(index);
        free(index as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn cpSpatialIndexInit(
    mut index: *mut cpSpatialIndex,
    mut klass___12: *mut cpSpatialIndexClass,
    mut bbfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    (*index).klass = klass___12;
    (*index).bbfunc = bbfunc;
    (*index).staticIndex = staticIndex;
    if !staticIndex.is_null() {
        if !((*staticIndex).dynamicIndex).is_null() {
            cpMessage(
                b"!staticIndex->dynamicIndex\0" as *const u8 as *const libc::c_char,
                b"../src/cpSpatialIndex.c\0" as *const u8 as *const libc::c_char,
                41 as libc::c_int,
                1 as libc::c_int,
                1 as libc::c_int,
                b"This static index is already associated with a dynamic index.\0"
                    as *const u8 as *const libc::c_char,
            );
            abort();
        }
        (*staticIndex).dynamicIndex = index;
    }
    return index;
}
unsafe extern "C" fn dynamicToStaticIter(
    mut obj: *mut libc::c_void,
    mut context: *mut dynamicToStaticContext,
) {
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = (Some(((*context).bbfunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(obj);
    cpSpatialIndexQuery(
        (*context).staticIndex,
        obj,
        tmp,
        (*context).queryFunc,
        (*context).data,
    );
}
pub unsafe extern "C" fn cpSpatialIndexCollideStatic(
    mut dynamicIndex: *mut cpSpatialIndex,
    mut staticIndex: *mut cpSpatialIndex,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    let mut context: dynamicToStaticContext = dynamicToStaticContext {
        bbfunc: None,
        staticIndex: 0 as *mut cpSpatialIndex,
        queryFunc: None,
        data: 0 as *mut libc::c_void,
    };
    let mut tmp: libc::c_int = 0;
    if !staticIndex.is_null() {
        tmp = cpSpatialIndexCount(staticIndex);
        if tmp > 0 as libc::c_int {
            context.bbfunc = (*dynamicIndex).bbfunc;
            context.staticIndex = staticIndex;
            context.queryFunc = func;
            context.data = data;
            cpSpatialIndexEach(
                dynamicIndex,
                ::std::mem::transmute::<
                    Option::<
                        unsafe extern "C" fn(
                            *mut libc::c_void,
                            *mut dynamicToStaticContext,
                        ) -> (),
                    >,
                    Option::<
                        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
                    >,
                >(
                    Some(
                        dynamicToStaticIter
                            as unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut dynamicToStaticContext,
                            ) -> (),
                    ),
                ),
                &mut context as *mut dynamicToStaticContext as *mut libc::c_void,
            );
        }
    }
}
#[inline]
unsafe extern "C" fn cpBBExpand(bb: cpBB, v: cpVect) -> cpBB {
    let mut tmp: cpFloat = 0.;
    let mut tmp___0: cpFloat = 0.;
    let mut tmp___1: cpFloat = 0.;
    let mut tmp___2: cpFloat = 0.;
    let mut tmp___3: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    tmp = cpfmax(bb.t, v.y);
    tmp___0 = cpfmax(bb.r, v.x);
    tmp___1 = cpfmin(bb.b, v.y);
    tmp___2 = cpfmin(bb.l, v.x);
    tmp___3 = cpBBNew(tmp___2, tmp___1, tmp___0, tmp);
    return tmp___3;
}
#[inline]
unsafe extern "C" fn BoundsOverlap(mut a: Bounds, mut b: Bounds) -> cpBool {
    let mut tmp: libc::c_int = 0;
    if a.min <= b.max {
        if b.min <= a.max {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
    } else {
        tmp = 0 as libc::c_int;
    }
    return tmp as cpBool;
}
#[inline]
unsafe extern "C" fn BBToBounds(mut sweep: *mut cpSweep1D, mut bb: cpBB) -> Bounds {
    let mut bounds: Bounds = Bounds { min: 0., max: 0. };
    bounds.min = bb.l;
    bounds.max = bb.r;
    return bounds;
}
#[inline]
unsafe extern "C" fn MakeTableCell(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
) -> TableCell {
    let mut cell: TableCell = TableCell {
        obj: 0 as *mut libc::c_void,
        bounds: Bounds { min: 0., max: 0. },
    };
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp___0: Bounds = Bounds { min: 0., max: 0. };
    tmp = (Some(((*sweep).spatialIndex.bbfunc).expect("non-null function pointer")))
        .expect("non-null function pointer")(obj);
    tmp___0 = BBToBounds(sweep, tmp);
    cell.obj = obj;
    cell.bounds = tmp___0;
    return cell;
}
pub unsafe extern "C" fn cpSweep1DAlloc() -> *mut cpSweep1D {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    tmp = calloc(
        1 as libc::c_int as size_t,
        ::std::mem::size_of::<cpSweep1D>() as libc::c_ulong,
    );
    return tmp as *mut cpSweep1D;
}
unsafe extern "C" fn ResizeTable(mut sweep: *mut cpSweep1D, mut size: libc::c_int) {
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    (*sweep).max = size;
    tmp = realloc(
        (*sweep).table as *mut libc::c_void,
        (size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TableCell>() as libc::c_ulong),
    );
    (*sweep).table = tmp as *mut TableCell;
}
pub unsafe extern "C" fn cpSweep1DInit(
    mut sweep: *mut cpSweep1D,
    mut bbfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    let mut tmp: *mut cpSpatialIndexClass = 0 as *mut cpSpatialIndexClass;
    tmp = Klass___1();
    cpSpatialIndexInit(sweep as *mut cpSpatialIndex, tmp, bbfunc, staticIndex);
    (*sweep).num = 0 as libc::c_int;
    ResizeTable(sweep, 32 as libc::c_int);
    return sweep as *mut cpSpatialIndex;
}
pub unsafe extern "C" fn cpSweep1DNew(
    mut bbfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> cpBB>,
    mut staticIndex: *mut cpSpatialIndex,
) -> *mut cpSpatialIndex {
    let mut tmp: *mut cpSweep1D = 0 as *mut cpSweep1D;
    let mut tmp___0: *mut cpSpatialIndex = 0 as *mut cpSpatialIndex;
    tmp = cpSweep1DAlloc();
    tmp___0 = cpSweep1DInit(tmp, bbfunc, staticIndex);
    return tmp___0;
}
unsafe extern "C" fn cpSweep1DDestroy(mut sweep: *mut cpSweep1D) {
    free((*sweep).table as *mut libc::c_void);
    (*sweep).table = 0 as *mut libc::c_void as *mut TableCell;
}
unsafe extern "C" fn cpSweep1DCount(mut sweep: *mut cpSweep1D) -> libc::c_int {
    return (*sweep).num;
}
unsafe extern "C" fn cpSweep1DEach(
    mut sweep: *mut cpSweep1D,
    mut func: Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
    mut data: *mut libc::c_void,
) {
    let mut table: *mut TableCell = 0 as *mut TableCell;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    table = (*sweep).table;
    i = 0 as libc::c_int;
    count = (*sweep).num;
    while i < count {
        (Some(func.expect("non-null function pointer")))
            .expect("non-null function pointer")((*table.offset(i as isize)).obj, data);
        i += 1;
    }
}
unsafe extern "C" fn cpSweep1DContains(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) -> libc::c_int {
    let mut table: *mut TableCell = 0 as *mut TableCell;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    table = (*sweep).table;
    i = 0 as libc::c_int;
    count = (*sweep).num;
    while i < count {
        if (*table.offset(i as isize)).obj as libc::c_ulong == obj as libc::c_ulong {
            return 1 as libc::c_int;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cpSweep1DInsert(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    if (*sweep).num == (*sweep).max {
        ResizeTable(sweep, (*sweep).max * 2 as libc::c_int);
    }
    *((*sweep).table).offset((*sweep).num as isize) = MakeTableCell(sweep, obj);
    (*sweep).num += 1;
}
unsafe extern "C" fn cpSweep1DRemove(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {
    let mut table: *mut TableCell = 0 as *mut TableCell;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    table = (*sweep).table;
    i = 0 as libc::c_int;
    count = (*sweep).num;
    while i < count {
        if (*table.offset(i as isize)).obj as libc::c_ulong == obj as libc::c_ulong {
            (*sweep).num -= 1;
            num = (*sweep).num;
            *table.offset(i as isize) = *table.offset(num as isize);
            let ref mut fresh30 = (*table.offset(num as isize)).obj;
            *fresh30 = 0 as *mut libc::c_void;
            return;
        }
        i += 1;
    }
}
unsafe extern "C" fn cpSweep1DReindexObject(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut hashid: cpHashValue,
) {}
unsafe extern "C" fn cpSweep1DReindex(mut sweep: *mut cpSweep1D) {}
unsafe extern "C" fn cpSweep1DQuery(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut bb: cpBB,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    let mut bounds: Bounds = Bounds { min: 0., max: 0. };
    let mut tmp: Bounds = Bounds { min: 0., max: 0. };
    let mut table: *mut TableCell = 0 as *mut TableCell;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut cell: TableCell = TableCell {
        obj: 0 as *mut libc::c_void,
        bounds: Bounds { min: 0., max: 0. },
    };
    let mut tmp___0: cpBool = 0;
    tmp = BBToBounds(sweep, bb);
    bounds = tmp;
    table = (*sweep).table;
    i = 0 as libc::c_int;
    count = (*sweep).num;
    while i < count {
        cell = *table.offset(i as isize);
        tmp___0 = BoundsOverlap(bounds, cell.bounds);
        if tmp___0 != 0 {
            if obj as libc::c_ulong != cell.obj as libc::c_ulong {
                (Some(func.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(obj, cell.obj, 0 as libc::c_int as cpCollisionID, data);
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn cpSweep1DSegmentQuery(
    mut sweep: *mut cpSweep1D,
    mut obj: *mut libc::c_void,
    mut a: cpVect,
    mut b: cpVect,
    mut t_exit: cpFloat,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> cpFloat,
    >,
    mut data: *mut libc::c_void,
) {
    let mut bb: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut tmp___0: cpBB = cpBB { l: 0., b: 0., r: 0., t: 0. };
    let mut bounds: Bounds = Bounds { min: 0., max: 0. };
    let mut tmp___1: Bounds = Bounds { min: 0., max: 0. };
    let mut table: *mut TableCell = 0 as *mut TableCell;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut cell: TableCell = TableCell {
        obj: 0 as *mut libc::c_void,
        bounds: Bounds { min: 0., max: 0. },
    };
    let mut tmp___2: cpBool = 0;
    tmp = cpBBNew(a.x, a.y, a.x, a.y);
    tmp___0 = cpBBExpand(tmp, b);
    bb = tmp___0;
    tmp___1 = BBToBounds(sweep, bb);
    bounds = tmp___1;
    table = (*sweep).table;
    i = 0 as libc::c_int;
    count = (*sweep).num;
    while i < count {
        cell = *table.offset(i as isize);
        tmp___2 = BoundsOverlap(bounds, cell.bounds);
        if tmp___2 != 0 {
            (Some(func.expect("non-null function pointer")))
                .expect("non-null function pointer")(obj, cell.obj, data);
        }
        i += 1;
    }
}
unsafe extern "C" fn TableSort(
    mut a: *mut TableCell,
    mut b: *mut TableCell,
) -> libc::c_int {
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    if (*a).bounds.min < (*b).bounds.min {
        tmp___0 = -(1 as libc::c_int);
    } else {
        if (*a).bounds.min > (*b).bounds.min {
            tmp = 1 as libc::c_int;
        } else {
            tmp = 0 as libc::c_int;
        }
        tmp___0 = tmp;
    }
    return tmp___0;
}
unsafe extern "C" fn cpSweep1DReindexQuery(
    mut sweep: *mut cpSweep1D,
    mut func: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            cpCollisionID,
            *mut libc::c_void,
        ) -> cpCollisionID,
    >,
    mut data: *mut libc::c_void,
) {
    let mut table: *mut TableCell = 0 as *mut TableCell;
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut cell: TableCell = TableCell {
        obj: 0 as *mut libc::c_void,
        bounds: Bounds { min: 0., max: 0. },
    };
    let mut max: cpFloat = 0.;
    let mut j: libc::c_int = 0;
    table = (*sweep).table;
    count = (*sweep).num;
    i = 0 as libc::c_int;
    while i < count {
        *table
            .offset(i as isize) = MakeTableCell(sweep, (*table.offset(i as isize)).obj);
        i += 1;
    }
    qsort(
        table as *mut libc::c_void,
        count as size_t,
        ::std::mem::size_of::<TableCell>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut TableCell, *mut TableCell) -> libc::c_int,
            >,
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
            >,
        >(
            Some(
                TableSort
                    as unsafe extern "C" fn(
                        *mut TableCell,
                        *mut TableCell,
                    ) -> libc::c_int,
            ),
        ),
    );
    i___0 = 0 as libc::c_int;
    while i___0 < count {
        cell = *table.offset(i___0 as isize);
        max = cell.bounds.max;
        j = i___0 + 1 as libc::c_int;
        while (*table.offset(j as isize)).bounds.min < max {
            if !(j < count) {
                break;
            }
            (Some(func.expect("non-null function pointer")))
                .expect(
                    "non-null function pointer",
                )(
                cell.obj,
                (*table.offset(j as isize)).obj,
                0 as libc::c_int as cpCollisionID,
                data,
            );
            j += 1;
        }
        i___0 += 1;
    }
    cpSpatialIndexCollideStatic(
        sweep as *mut cpSpatialIndex,
        (*sweep).spatialIndex.staticIndex,
        func,
        data,
    );
}
static mut klass___11: cpSpatialIndexClass = unsafe {
    {
        let mut init = cpSpatialIndexClass {
            destroy: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSweep1D) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> ()>,
            >(Some(cpSweep1DDestroy as unsafe extern "C" fn(*mut cpSweep1D) -> ())),
            count: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSweep1D) -> libc::c_int>,
                Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> libc::c_int>,
            >(
                Some(
                    cpSweep1DCount as unsafe extern "C" fn(*mut cpSweep1D) -> libc::c_int,
                ),
            ),
            each: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpSweep1DEach
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                ) -> (),
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            contains: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> libc::c_int,
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> cpBool,
                >,
            >(
                Some(
                    cpSweep1DContains
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> libc::c_int,
                ),
            ),
            insert: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
            >(
                Some(
                    cpSweep1DInsert
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            remove: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
            >(
                Some(
                    cpSweep1DRemove
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindex: ::std::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut cpSweep1D) -> ()>,
                Option::<unsafe extern "C" fn(*mut cpSpatialIndex) -> ()>,
            >(Some(cpSweep1DReindex as unsafe extern "C" fn(*mut cpSweep1D) -> ())),
            reindexObject: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpHashValue,
                    ) -> (),
                >,
            >(
                Some(
                    cpSweep1DReindexObject
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            *mut libc::c_void,
                            cpHashValue,
                        ) -> (),
                ),
            ),
            reindexQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpSweep1DReindexQuery
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                    cpCollisionID,
                                    *mut libc::c_void,
                                ) -> cpCollisionID,
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            query: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpBB,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpBB,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                cpCollisionID,
                                *mut libc::c_void,
                            ) -> cpCollisionID,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpSweep1DQuery
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            *mut libc::c_void,
                            cpBB,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                    cpCollisionID,
                                    *mut libc::c_void,
                                ) -> cpCollisionID,
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
            segmentQuery: ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSweep1D,
                        *mut libc::c_void,
                        cpVect,
                        cpVect,
                        cpFloat,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> cpFloat,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
                Option::<
                    unsafe extern "C" fn(
                        *mut cpSpatialIndex,
                        *mut libc::c_void,
                        cpVect,
                        cpVect,
                        cpFloat,
                        Option::<
                            unsafe extern "C" fn(
                                *mut libc::c_void,
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> cpFloat,
                        >,
                        *mut libc::c_void,
                    ) -> (),
                >,
            >(
                Some(
                    cpSweep1DSegmentQuery
                        as unsafe extern "C" fn(
                            *mut cpSweep1D,
                            *mut libc::c_void,
                            cpVect,
                            cpVect,
                            cpFloat,
                            Option::<
                                unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                    *mut libc::c_void,
                                ) -> cpFloat,
                            >,
                            *mut libc::c_void,
                        ) -> (),
                ),
            ),
        };
        init
    }
};
#[inline]
unsafe extern "C" fn Klass___1() -> *mut cpSpatialIndexClass {
    return &mut klass___11;
}
unsafe extern "C" fn run_static_initializers() {
    spring_count = (::std::mem::size_of::<[cpVect; 15]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cpVect>() as libc::c_ulong) as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
