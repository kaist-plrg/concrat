use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn snoopy_entrypoint_test_cli_init(
        filename: *const libc::c_char,
        argv: *const *mut libc::c_char,
        configFilePath: *mut libc::c_char,
    );
    fn snoopy_entrypoint_test_cli_exit();
    fn snoopy_error_handler(errorMsg: *const libc::c_char);
    fn snoopy_datasourceregistry_getCount() -> libc::c_int;
    fn snoopy_datasourceregistry_doesNameExist(
        datasourceName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasourceregistry_getName(datasourceId: libc::c_int) -> *mut libc::c_char;
    fn snoopy_datasourceregistry_callById(
        datasourceId: libc::c_int,
        result: *mut libc::c_char,
        datasourceArg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_datasourceregistry_callByName(
        datasourceName: *const libc::c_char,
        result: *mut libc::c_char,
        datasourceArg: *const libc::c_char,
    ) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn snoopy_configuration_get() -> *mut snoopy_configuration_t;
    fn snoopy_filtering_check_chain(chain: *const libc::c_char) -> libc::c_int;
    fn snoopy_action_log_message_dispatch(
        logMessage: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_message_generateFromFormat(
        logMessage: *mut libc::c_char,
        logMessageBufSize: size_t,
        logMessageFormat: *mut libc::c_char,
    );
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn snoopy_outputregistry_getCount() -> libc::c_int;
    fn snoopy_outputregistry_doesNameExist(
        outputName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_outputregistry_getName(outputId: libc::c_int) -> *mut libc::c_char;
    fn snoopy_outputregistry_callById(
        outputId: libc::c_int,
        logMessage: *const libc::c_char,
        outputArg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_outputregistry_callByName(
        outputName: *const libc::c_char,
        logMessage: *const libc::c_char,
        outputArg: *const libc::c_char,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn snoopy_action_log_syscall_exec();
    fn snoopy_outputregistry_doesIdExist(outputId: libc::c_int) -> libc::c_int;
    fn snoopy_outputregistry_getIdFromName(
        outputName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_outputregistry_dispatch(logMessage: *const libc::c_char) -> libc::c_int;
    fn snoopy_util_syslog_convertFacilityToInt(
        facilityStr: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_util_syslog_convertFacilityToStr(
        facilityInt: libc::c_int,
    ) -> *const libc::c_char;
    fn snoopy_util_syslog_convertLevelToInt(
        levelStr: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_util_syslog_convertLevelToStr(
        levelInt: libc::c_int,
    ) -> *const libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlerror() -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn snoopy_ini_parse_string(
        string: *const libc::c_char,
        handler: Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_char,
                *const libc::c_char,
                *const libc::c_char,
            ) -> libc::c_int,
        >,
        user: *mut libc::c_void,
    ) -> libc::c_int;
    fn snoopy_filterregistry_getCount() -> libc::c_int;
    fn snoopy_filterregistry_doesNameExist(
        filterName: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_filterregistry_getName(filterId: libc::c_int) -> *mut libc::c_char;
    fn snoopy_filterregistry_callById(
        filterId: libc::c_int,
        filterArg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_filterregistry_callByName(
        filterName: *const libc::c_char,
        filterArg: *const libc::c_char,
    ) -> libc::c_int;
    fn snoopy_filterregistry_doesIdExist(filterId: libc::c_int) -> libc::c_int;
    fn snoopy_filterregistry_getIdFromName(
        filterName: *const libc::c_char,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn snoopy_entrypoint_test_cli_threads_init();
    fn snoopy_configuration_preinit_disableConfigFileParsing();
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
    fn pthread_self() -> pthread_t;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn snoopy_tsrm_get_threadCount() -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn execv(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn fork() -> __pid_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn snoopy_util_systemd_convertCgroupEntryToUnitName(
        cgroupEntry: *const libc::c_char,
    ) -> *mut libc::c_char;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub struct __anonstruct_snoopy_configuration_t_620930686 {
    pub initialized: libc::c_int,
    pub configfile_enabled: libc::c_int,
    pub configfile_path: *mut libc::c_char,
    pub configfile_found: libc::c_int,
    pub configfile_parsed: libc::c_int,
    pub error_logging_enabled: libc::c_int,
    pub message_format: *mut libc::c_char,
    pub message_format_malloced: libc::c_int,
    pub filtering_enabled: libc::c_int,
    pub filter_chain: *mut libc::c_char,
    pub filter_chain_malloced: libc::c_int,
    pub output: *mut libc::c_char,
    pub output_malloced: libc::c_int,
    pub output_arg: *mut libc::c_char,
    pub output_arg_malloced: libc::c_int,
    pub syslog_facility: libc::c_int,
    pub syslog_level: libc::c_int,
    pub syslog_ident_format_malloced: libc::c_int,
    pub syslog_ident_format: *mut libc::c_char,
}
pub type snoopy_configuration_t = __anonstruct_snoopy_configuration_t_620930686;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __anonstruct_tData_t_1064464920 {
    pub seqNr: libc::c_int,
}
pub type tData_t = __anonstruct_tData_t_1064464920;
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
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
pub unsafe extern "C" fn snoopyTestCli_action_run_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `run`\n\nUsage:\n    snoopy run SUBSYSTEM [ARGS]\n\nAvailable subsystems:\n    configfile,cf      Run a configfile (.ini) parser\n    datasource,ds      Run a data source\n    filter,f           Run a filter\n    filterchain,fc     Run a filter chain (as if it would be configured in snoopy.ini)\n    messageformat,mf   Run the message formatter\n    output,o           Run an output\n\n    everything         Runs every subsystem, as much as possible (for Valgrind)\n\n    help,h             Show this help\n    SUBSYSTEM help     Show SUBSYSTEM's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_run(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
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
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_showHelp();
        fatalError(b"No subsystem specified.\0" as *const u8 as *const libc::c_char);
    }
    tmp___0 = strcmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"configfile\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp___0 {
        tmp = snoopyTestCli_action_run_configfile(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp;
    } else {
        tmp___1 = strcmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"cf\0" as *const u8 as *const libc::c_char,
        );
        if 0 as libc::c_int == tmp___1 {
            tmp = snoopyTestCli_action_run_configfile(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp;
        }
    }
    tmp___3 = strcmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"datasource\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp___3 {
        tmp___2 = snoopyTestCli_action_run_datasource(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___2;
    } else {
        tmp___4 = strcmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"ds\0" as *const u8 as *const libc::c_char,
        );
        if 0 as libc::c_int == tmp___4 {
            tmp___2 = snoopyTestCli_action_run_datasource(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___2;
        }
    }
    tmp___6 = strcmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"filter\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp___6 {
        tmp___5 = snoopyTestCli_action_run_filter(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___5;
    } else {
        tmp___7 = strcmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"f\0" as *const u8 as *const libc::c_char,
        );
        if 0 as libc::c_int == tmp___7 {
            tmp___5 = snoopyTestCli_action_run_filter(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___5;
        }
    }
    tmp___9 = strcmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"filterchain\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp___9 {
        tmp___8 = snoopyTestCli_action_run_filterchain(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___8;
    } else {
        tmp___10 = strcmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"fc\0" as *const u8 as *const libc::c_char,
        );
        if 0 as libc::c_int == tmp___10 {
            tmp___8 = snoopyTestCli_action_run_filterchain(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___8;
        }
    }
    tmp___12 = strcmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"messageformat\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp___12 {
        tmp___11 = snoopyTestCli_action_run_messageformat(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___11;
    } else {
        tmp___13 = strcmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"mf\0" as *const u8 as *const libc::c_char,
        );
        if 0 as libc::c_int == tmp___13 {
            tmp___11 = snoopyTestCli_action_run_messageformat(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___11;
        }
    }
    tmp___15 = strcmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"output\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp___15 {
        tmp___14 = snoopyTestCli_action_run_output(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___14;
    } else {
        tmp___16 = strcmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"o\0" as *const u8 as *const libc::c_char,
        );
        if 0 as libc::c_int == tmp___16 {
            tmp___14 = snoopyTestCli_action_run_output(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___14;
        }
    }
    tmp___18 = strcmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"everything\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp___18 {
        tmp___17 = snoopyTestCli_action_run_everything();
        return tmp___17;
    }
    tmp___19 = strcmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"help\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp___19 {
        snoopyTestCli_action_run_showHelp();
        return 0 as libc::c_int;
    } else {
        tmp___20 = strcmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"h\0" as *const u8 as *const libc::c_char,
        );
        if 0 as libc::c_int == tmp___20 {
            snoopyTestCli_action_run_showHelp();
            return 0 as libc::c_int;
        }
    }
    snoopyTestCli_action_run_showHelp();
    fatalErrorValue(
        b"Unknown subsystem\0" as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
    );
    return 127 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_datasource_showList() {
    let mut dCount: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    printf(b"Available datasources:\n\0" as *const u8 as *const libc::c_char);
    tmp = snoopy_datasourceregistry_getCount();
    dCount = tmp;
    i = 0 as libc::c_int;
    while i < dCount {
        tmp___0 = snoopy_datasourceregistry_getName(i);
        printf(b"    %s\n\0" as *const u8 as *const libc::c_char, tmp___0);
        i += 1;
    }
}
pub unsafe extern "C" fn snoopyTestCli_action_run_datasource_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `datasource`\n\nUsage:\n    snoopy-test run datasource DATASOURCE [ARGS]\n    snoopy-test run datasource --all    # Runs all datasources\n    snoopy-test run datasource --help   # Shows this help message\n    snoopy-test run datasource --list   # Lists all available datasources\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
    snoopyTestCli_action_run_datasource_showList();
}
pub unsafe extern "C" fn snoopyTestCli_action_run_datasource(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut datasourceName: *const libc::c_char = 0 as *const libc::c_char;
    let mut datasourceArg: *const libc::c_char = 0 as *const libc::c_char;
    let mut datasourceResult: [libc::c_char; 2048] = [0; 2048];
    let mut retVal: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_datasource_showHelp();
        fatalError(
            b"Missing argument: `datasource name` or `--list`\0" as *const u8
                as *const libc::c_char,
        );
    }
    arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    tmp = strcmp(arg1, b"--all\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_run_datasource_all();
        return 0 as libc::c_int;
    }
    tmp___0 = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___0 {
        snoopyTestCli_action_run_datasource_showHelp();
        return 0 as libc::c_int;
    }
    tmp___1 = strcmp(arg1, b"--list\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___1 {
        snoopyTestCli_action_run_datasource_showList();
        return 0 as libc::c_int;
    }
    datasourceName = arg1;
    tmp___2 = snoopy_datasourceregistry_doesNameExist(datasourceName);
    if 0 as libc::c_int == tmp___2 {
        snoopyTestCli_action_run_datasource_showHelp();
        fatalError(
            b"Invalid datasource name given\0" as *const u8 as *const libc::c_char,
        );
    }
    if argc >= 2 as libc::c_int {
        datasourceArg = *argv.offset(1 as libc::c_int as isize) as *const libc::c_char;
    } else {
        datasourceArg = b"\0" as *const u8 as *const libc::c_char;
    }
    retVal = snoopy_datasourceregistry_callByName(
        datasourceName,
        datasourceResult.as_mut_ptr(),
        datasourceArg,
    );
    if retVal < 0 as libc::c_int {
        fatalErrorValue(
            b"Datasource failed\0" as *const u8 as *const libc::c_char,
            datasourceResult.as_mut_ptr() as *const libc::c_char,
        );
    }
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, datasourceResult.as_mut_ptr());
    snoopy_entrypoint_test_cli_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_datasource_all() {
    let mut itemName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itemArgs: *const libc::c_char = 0 as *const libc::c_char;
    let mut itemResult: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itemResultSize: libc::c_int = 0;
    let mut dCount: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    itemName = 0 as *mut libc::c_void as *mut libc::c_char;
    itemArgs = 0 as *mut libc::c_void as *const libc::c_char;
    itemResult = 0 as *mut libc::c_void as *mut libc::c_char;
    tmp = malloc(2049 as libc::c_int as size_t);
    itemResult = tmp as *mut libc::c_char;
    dCount = snoopy_datasourceregistry_getCount();
    i = 0 as libc::c_int;
    while i < dCount {
        itemName = snoopy_datasourceregistry_getName(i);
        printf(b"Datasource %15s: \0" as *const u8 as *const libc::c_char, itemName);
        tmp___1 = strcmp(
            itemName as *const libc::c_char,
            b"env\0" as *const u8 as *const libc::c_char,
        );
        if tmp___1 == 0 as libc::c_int {
            itemArgs = b"HOME\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___0 = strcmp(
                itemName as *const libc::c_char,
                b"snoopy_literal\0" as *const u8 as *const libc::c_char,
            );
            if tmp___0 == 0 as libc::c_int {
                itemArgs = b"somestring\0" as *const u8 as *const libc::c_char;
            } else {
                itemArgs = b"\0" as *const u8 as *const libc::c_char;
            }
        }
        itemResultSize = snoopy_datasourceregistry_callById(i, itemResult, itemArgs);
        if itemResultSize > 2048 as libc::c_int {
            snoopy_error_handler(
                b"Maximum data source message size exceeded\0" as *const u8
                    as *const libc::c_char,
            );
        }
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, itemResult);
        i += 1;
    }
    free(itemResult as *mut libc::c_void);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_everything_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Run everything (for Valgrind)\n\nUsage:\n    snoopy-test run everything\n\nResult:\n    Runs as many subsystems as possible, to cover as much code as possible.\n    Useful for Valgrind analysis.\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_everything() -> libc::c_int {
    let mut logMessage: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    logMessage = 0 as *mut libc::c_void as *mut libc::c_char;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    CFG = snoopy_configuration_get();
    tmp = malloc(16383 as libc::c_int as size_t);
    logMessage = tmp as *mut libc::c_char;
    *logMessage.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    printf(
        b"-----[ Filters ]---------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopyTestCli_action_run_filter_all();
    printf(
        b"-----[ Filtering ]-------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopy_filtering_check_chain(
        b"exclude_uid:10,11,12;only_uid=0,1,2,3\0" as *const u8 as *const libc::c_char,
    );
    printf(b"Done.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"-----[ Datasources ]-----------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopyTestCli_action_run_datasource_all();
    printf(
        b"-----[ Outputs ]---------------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopyTestCli_action_run_output_all();
    printf(
        b"-----[ Message formatting ]----------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopy_message_generateFromFormat(
        logMessage,
        16383 as libc::c_int as size_t,
        (*CFG).message_format,
    );
    printf(b"Message: %s\n\0" as *const u8 as *const libc::c_char, logMessage);
    printf(
        b"-----[ Dispatching ]-----------------------------------\n\0" as *const u8
            as *const libc::c_char,
    );
    snoopy_action_log_message_dispatch(logMessage as *const libc::c_char);
    printf(b"Done.\n\0" as *const u8 as *const libc::c_char);
    printf(b"\nAll done.\n\0" as *const u8 as *const libc::c_char);
    free(logMessage as *mut libc::c_void);
    snoopy_entrypoint_test_cli_exit();
    fclose(stdin);
    fclose(stdout);
    fclose(stderr);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_messageformat_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `message formatter`\n\nUsage:\n    snoopy-test run messageformat \"FORMAT SPECIFICATION\"\n\nResult:\n    Prints a log message formatted according to the given format specification.\n    Process data is taken from self.\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_messageformat(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut messageFormat: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_messageformat_showHelp();
        fatalError(
            b"Missing argument: message format\0" as *const u8 as *const libc::c_char,
        );
    }
    messageFormat = *argv.offset(0 as libc::c_int as isize);
    tmp = malloc(16383 as libc::c_int as size_t);
    message = tmp as *mut libc::c_char;
    *message.offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    snoopy_message_generateFromFormat(
        message,
        16383 as libc::c_int as size_t,
        messageFormat,
    );
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, message);
    free(message as *mut libc::c_void);
    snoopy_entrypoint_test_cli_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_output_showList() {
    let mut oCount: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    printf(b"Available outputs:\n\0" as *const u8 as *const libc::c_char);
    tmp = snoopy_outputregistry_getCount();
    oCount = tmp;
    i = 0 as libc::c_int;
    while i < oCount {
        tmp___0 = snoopy_outputregistry_getName(i);
        printf(b"    %s\n\0" as *const u8 as *const libc::c_char, tmp___0);
        i += 1;
    }
}
pub unsafe extern "C" fn snoopyTestCli_action_run_output_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `output`\n\nUsage:\n    snoopy-test run output \"LOG MESSAGE\" OUTPUT [OUTPUT_ARGS]\n    snoopy-test run output --all\n    snoopy-test run output --list\n    snoopy-test run output --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
    snoopyTestCli_action_run_output_showList();
}
pub unsafe extern "C" fn snoopyTestCli_action_run_output(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut message: *const libc::c_char = 0 as *const libc::c_char;
    let mut outputName: *const libc::c_char = 0 as *const libc::c_char;
    let mut outputArg: *const libc::c_char = 0 as *const libc::c_char;
    let mut retVal: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_output_showHelp();
        fatalError(
            b"Missing argument: log message, or --all or --list\0" as *const u8
                as *const libc::c_char,
        );
    }
    arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    tmp = strcmp(arg1, b"--all\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_run_output_all();
        return 0 as libc::c_int;
    }
    tmp___0 = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___0 {
        snoopyTestCli_action_run_output_showHelp();
        return 0 as libc::c_int;
    }
    tmp___1 = strcmp(arg1, b"--list\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___1 {
        snoopyTestCli_action_run_output_showList();
        return 0 as libc::c_int;
    }
    message = arg1;
    if argc < 2 as libc::c_int {
        snoopyTestCli_action_run_output_showHelp();
        fatalError(
            b"Missing argument: output name\0" as *const u8 as *const libc::c_char,
        );
    }
    outputName = *argv.offset(1 as libc::c_int as isize) as *const libc::c_char;
    if argc > 2 as libc::c_int {
        outputArg = *argv.offset(2 as libc::c_int as isize) as *const libc::c_char;
    } else {
        outputArg = b"\0" as *const u8 as *const libc::c_char;
    }
    tmp___2 = snoopy_outputregistry_doesNameExist(outputName);
    if 0 as libc::c_int == tmp___2 {
        snoopyTestCli_action_run_output_showHelp();
        fatalErrorValue(
            b"Invalid output name given\0" as *const u8 as *const libc::c_char,
            outputName,
        );
    }
    retVal = snoopy_outputregistry_callByName(outputName, message, outputArg);
    if retVal < 0 as libc::c_int {
        fatalError(b"Output failure\0" as *const u8 as *const libc::c_char);
    }
    snoopy_entrypoint_test_cli_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_output_all() {
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itemName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itemArgs: *const libc::c_char = 0 as *const libc::c_char;
    let mut itemResult: libc::c_int = 0;
    let mut iCount: libc::c_int = 0;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    message = 0 as *mut libc::c_void as *mut libc::c_char;
    itemName = 0 as *mut libc::c_void as *mut libc::c_char;
    itemArgs = 0 as *mut libc::c_void as *const libc::c_char;
    tmp = malloc(16383 as libc::c_int as size_t);
    message = tmp as *mut libc::c_char;
    snprintf(
        message,
        16383 as libc::c_int as size_t,
        b"Snoopy output debugging\0" as *const u8 as *const libc::c_char,
    );
    iCount = snoopy_outputregistry_getCount();
    i = 0 as libc::c_int;
    while i < iCount {
        itemName = snoopy_outputregistry_getName(i);
        printf(b"Output %19s: \0" as *const u8 as *const libc::c_char, itemName);
        tmp___1 = strcmp(
            itemName as *const libc::c_char,
            b"file\0" as *const u8 as *const libc::c_char,
        );
        if tmp___1 == 0 as libc::c_int {
            itemArgs = b"./fileoutput.out\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___0 = strcmp(
                itemName as *const libc::c_char,
                b"socket\0" as *const u8 as *const libc::c_char,
            );
            if tmp___0 == 0 as libc::c_int {
                itemArgs = b"/dev/log\0" as *const u8 as *const libc::c_char;
            } else {
                itemArgs = b"\0" as *const u8 as *const libc::c_char;
            }
        }
        itemResult = snoopy_outputregistry_callById(
            i,
            message as *const libc::c_char,
            itemArgs,
        );
        printf(
            b"%d chars transmitted. (output arg:%s)\n\0" as *const u8
                as *const libc::c_char,
            itemResult,
            itemArgs,
        );
        tmp___2 = strcmp(
            itemName as *const libc::c_char,
            b"file\0" as *const u8 as *const libc::c_char,
        );
        if tmp___2 == 0 as libc::c_int {
            unlink(itemArgs);
        }
        i += 1;
    }
    free(message as *mut libc::c_void);
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `stress`\n\nUsage:\n    snoopy stress SUBSYSTEM [ARGS]\n\nAvailable subsystems:\n    threads,t          Stress internal threading implementation\n    threadsexec,te     Stress threading implementation by including execution of an external command\n\n    help,h             Show this help\n    SUBSYSTEM help     Show SUBSYSTEM's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_stress(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut subsystem: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_stress_showHelp();
        fatalError(b"No subsystem specified.\0" as *const u8 as *const libc::c_char);
    }
    subsystem = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    tmp = strcmp(subsystem, b"help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_stress_showHelp();
        return 0 as libc::c_int;
    } else {
        tmp___0 = strcmp(subsystem, b"h\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___0 {
            snoopyTestCli_action_stress_showHelp();
            return 0 as libc::c_int;
        }
    }
    tmp___2 = strcmp(subsystem, b"threads\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___2 {
        tmp___1 = snoopyTestCli_action_stress_threads(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___1;
    } else {
        tmp___3 = strcmp(subsystem, b"t\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___3 {
            tmp___1 = snoopyTestCli_action_stress_threads(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___1;
        }
    }
    tmp___5 = strcmp(subsystem, b"threadsexec\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___5 {
        tmp___4 = snoopyTestCli_action_stress_threadsexec(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___4;
    } else {
        tmp___6 = strcmp(subsystem, b"te\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___6 {
            tmp___4 = snoopyTestCli_action_stress_threadsexec(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___4;
        }
    }
    snoopyTestCli_action_stress_showHelp();
    fatalErrorValue(
        b"Unknown subsystem\0" as *const u8 as *const libc::c_char,
        subsystem,
    );
    return 127 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit`\n\nUsage:\n    snoopy-test unit UNIT [ARGS]\n\nAvailable units:\n    action,a           Run a unit test on action/*.c\n    error,e            Run a unit test on src/error.c\n    ext-ini,ei         Run a unit test on lib/inih/src/ini.c\n    filterregistry,fr  Run a unit test on src/filterregistry.c\n    outputregistry,or  Run a unit test on src/outputregistry.c\n    util,u             Run a unit test on src/util/*.c\n\n    --help,-h          Show this help\n    UNIT --help        Show UNIT's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut unit: *const libc::c_char = 0 as *const libc::c_char;
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
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_unit_showHelp();
        fatalError(b"No unit specified.\0" as *const u8 as *const libc::c_char);
    }
    unit = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    tmp___0 = strcmp(unit, b"action\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___0 {
        tmp = snoopyTestCli_action_unit_action(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp;
    } else {
        tmp___1 = strcmp(unit, b"a\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___1 {
            tmp = snoopyTestCli_action_unit_action(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp;
        }
    }
    tmp___3 = strcmp(unit, b"error\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___3 {
        tmp___2 = snoopyTestCli_action_unit_error(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___2;
    } else {
        tmp___4 = strcmp(unit, b"e\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___4 {
            tmp___2 = snoopyTestCli_action_unit_error(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___2;
        }
    }
    tmp___6 = strcmp(unit, b"ext-ini\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___6 {
        tmp___5 = snoopyTestCli_action_unit_ext_ini(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___5;
    } else {
        tmp___7 = strcmp(unit, b"ei\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___7 {
            tmp___5 = snoopyTestCli_action_unit_ext_ini(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___5;
        }
    }
    tmp___9 = strcmp(unit, b"filterregistry\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___9 {
        tmp___8 = snoopyTestCli_action_unit_filterregistry(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___8;
    } else {
        tmp___10 = strcmp(unit, b"fr\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___10 {
            tmp___8 = snoopyTestCli_action_unit_filterregistry(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___8;
        }
    }
    tmp___12 = strcmp(unit, b"outputregistry\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___12 {
        tmp___11 = snoopyTestCli_action_unit_outputregistry(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___11;
    } else {
        tmp___13 = strcmp(unit, b"or\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___13 {
            tmp___11 = snoopyTestCli_action_unit_outputregistry(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___11;
        }
    }
    tmp___15 = strcmp(unit, b"util\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___15 {
        tmp___14 = snoopyTestCli_action_unit_util(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___14;
    } else {
        tmp___16 = strcmp(unit, b"u\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___16 {
            tmp___14 = snoopyTestCli_action_unit_util(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___14;
        }
    }
    tmp___17 = strcmp(unit, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___17 {
        snoopyTestCli_action_unit_showHelp();
        return 0 as libc::c_int;
    } else {
        tmp___18 = strcmp(unit, b"-h\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___18 {
            snoopyTestCli_action_unit_showHelp();
            return 0 as libc::c_int;
        }
    }
    snoopyTestCli_action_unit_showHelp();
    fatalErrorValue(b"Unknown unit\0" as *const u8 as *const libc::c_char, unit);
    return 127 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_action_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `action`\n\nUsage:\n    snoopy-test unit action SUBUNIT [ARGS]\n\nAvailable subunits:\n    log-message-dispatch,lmd   Run a unit test on src/action/log-message-dispatch.c\n    log-syscall-exec,lse       Run a unit test on src/action/log-syscall-exec.c\n\n    --help,-h          Show this help\n    UNIT --help        Show UNIT's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_action(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut subunit: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_unit_action_showHelp();
        fatalError(b"No subunit specified.\0" as *const u8 as *const libc::c_char);
    }
    subunit = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    tmp___0 = strcmp(
        subunit,
        b"log-message-dispatch\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp___0 {
        tmp = snoopyTestCli_action_unit_action_log_message_dispatch(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp;
    } else {
        tmp___1 = strcmp(subunit, b"lmd\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___1 {
            tmp = snoopyTestCli_action_unit_action_log_message_dispatch(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp;
        }
    }
    tmp___3 = strcmp(subunit, b"log-syscall-exec\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___3 {
        tmp___2 = snoopyTestCli_action_unit_action_log_syscall_exec(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___2;
    } else {
        tmp___4 = strcmp(subunit, b"lse\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___4 {
            tmp___2 = snoopyTestCli_action_unit_action_log_syscall_exec(
                argc - 1 as libc::c_int,
                argv.offset(1 as libc::c_int as isize),
            );
            return tmp___2;
        }
    }
    tmp___5 = strcmp(subunit, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___5 {
        snoopyTestCli_action_unit_action_showHelp();
        return 0 as libc::c_int;
    } else {
        tmp___6 = strcmp(subunit, b"-h\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___6 {
            snoopyTestCli_action_unit_action_showHelp();
            return 0 as libc::c_int;
        }
    }
    snoopyTestCli_action_unit_action_showHelp();
    fatalErrorValue(b"Unknown subunit\0" as *const u8 as *const libc::c_char, subunit);
    return 127 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_action_log_syscall_exec_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `action` :: Subunit 'log-syscall-exec'\n\nDescription:\n    Mocks src/action/log-syscall-exec.c implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit action log-syscall-exec\n    snoopy-test unit action log-syscall-exec --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_action_log_syscall_exec(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    tmp = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_unit_action_log_syscall_exec_showHelp();
        return 0 as libc::c_int;
    }
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    CFG = snoopy_configuration_get();
    (*CFG)
        .message_format = b"Test error message, sent out via stdout output.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*CFG).filter_chain = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*CFG).output = b"stdout\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    snoopy_action_log_syscall_exec();
    snoopy_entrypoint_test_cli_exit();
    printSuccess(
        b"Mocking src/action/log-syscall-exec.c complete.\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_action_log_message_dispatch_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `action` :: Subunit 'log-message-dispatch'\n\nDescription:\n    Mocks src/action/log-message-dispatch.c implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit action log-message-dispatch\n    snoopy-test unit action log-message-dispatch --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_action_log_message_dispatch(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    tmp = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_unit_action_log_message_dispatch_showHelp();
        return 0 as libc::c_int;
    }
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    CFG = snoopy_configuration_get();
    (*CFG).output = b"stdout\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    snoopy_action_log_message_dispatch(
        b"Test error message, sent out via stdout output.\0" as *const u8
            as *const libc::c_char,
    );
    snoopy_entrypoint_test_cli_exit();
    printSuccess(
        b"Mocking src/action/log-message-dispatch.c complete.\0" as *const u8
            as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_error_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `error`\n\nDescription:\n    Mocks src/error.c implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit error\n    snoopy-test unit error --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_error(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    tmp = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_unit_error_showHelp();
        return 0 as libc::c_int;
    }
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    CFG = snoopy_configuration_get();
    (*CFG).output = b"stderr\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    snoopy_error_handler(
        b"Test error message, sent out via stderr output.\0" as *const u8
            as *const libc::c_char,
    );
    snoopy_entrypoint_test_cli_exit();
    printSuccess(b"Mocking src/error.c complete.\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_outputregistry_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `outputregistry`\n\nDescription:\n    Mocks outputregistry implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit outputregistry\n    snoopy-test unit outputregistry --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_outputregistry(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut outputName: *const libc::c_char = 0 as *const libc::c_char;
    let mut outputCount: libc::c_int = 0;
    let mut outputIdPreset: libc::c_int = 0;
    let mut outputIdReceived: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    tmp = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_unit_outputregistry_showHelp();
        return 0 as libc::c_int;
    }
    outputName = 0 as *mut libc::c_void as *const libc::c_char;
    outputCount = 0 as libc::c_int;
    outputIdPreset = 0 as libc::c_int;
    outputIdReceived = 0 as libc::c_int;
    outputCount = snoopy_outputregistry_getCount();
    if outputCount < 1 as libc::c_int {
        fatalError(
            b"No outputs available, output count is 0\0" as *const u8
                as *const libc::c_char,
        );
    }
    printf(
        b"[DEBUG] Number of outputs available: %d\n\0" as *const u8
            as *const libc::c_char,
        outputCount,
    );
    tmp___0 = snoopy_outputregistry_doesIdExist(outputIdPreset);
    if tmp___0 != 1 as libc::c_int {
        fatalError(
            b"Output with a preset ID (0) does not exist\0" as *const u8
                as *const libc::c_char,
        );
    }
    tmp___1 = snoopy_outputregistry_getName(outputIdPreset);
    outputName = tmp___1 as *const libc::c_char;
    outputIdReceived = snoopy_outputregistry_getIdFromName(outputName);
    if outputIdPreset != outputIdReceived {
        fatalError(
            b"Output ID returned when searching by name differs from the initially used ID to find that same output\0"
                as *const u8 as *const libc::c_char,
        );
    }
    outputName = b"noop\0" as *const u8 as *const libc::c_char;
    outputIdReceived = snoopy_outputregistry_getIdFromName(outputName);
    snoopy_outputregistry_callById(
        outputIdReceived,
        0 as *mut libc::c_void as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    tmp___2 = snoopy_outputregistry_callById(
        -(1 as libc::c_int),
        0 as *mut libc::c_void as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if -(1 as libc::c_int) != tmp___2 {
        fatalError(
            b"Output ID -1 unexpectedly exists\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___3 = snoopy_outputregistry_callByName(
        b"fakeOutputNameThatShouldNeverExist\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if -(1 as libc::c_int) != tmp___3 {
        fatalError(
            b"Output with an unexpected name actually exists\0" as *const u8
                as *const libc::c_char,
        );
    }
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    CFG = snoopy_configuration_get();
    (*CFG).output = b"noop\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    snoopy_outputregistry_dispatch(0 as *mut libc::c_void as *const libc::c_char);
    snoopy_entrypoint_test_cli_exit();
    printSuccess(
        b"Mocking src/outputregistry.c complete.\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `util`\n\nUsage:\n    snoopy-test unit util SUBUNIT [ARGS]\n\nAvailable subunits:\n    syslog             Run a unit test on src/util/syslog.c\n    systemd            Run a unit test on src/util/systemd.c\n\n    --help,-h          Show this help\n    UNIT --help        Show UNIT's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut subunit: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_unit_util_showHelp();
        fatalError(b"No subunit specified.\0" as *const u8 as *const libc::c_char);
    }
    subunit = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    tmp___0 = strcmp(subunit, b"syslog\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___0 {
        tmp = snoopyTestCli_action_unit_util_syslog(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp;
    }
    tmp___2 = strcmp(subunit, b"systemd\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___2 {
        tmp___1 = snoopyTestCli_action_unit_util_systemd(
            argc - 1 as libc::c_int,
            argv.offset(1 as libc::c_int as isize),
        );
        return tmp___1;
    }
    tmp___3 = strcmp(subunit, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___3 {
        snoopyTestCli_action_unit_util_showHelp();
        return 0 as libc::c_int;
    } else {
        tmp___4 = strcmp(subunit, b"-h\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___4 {
            snoopyTestCli_action_unit_util_showHelp();
            return 0 as libc::c_int;
        }
    }
    snoopyTestCli_action_unit_util_showHelp();
    fatalErrorValue(b"Unknown subunit\0" as *const u8 as *const libc::c_char, subunit);
    return 127 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util_syslog_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `misc` :: Subunit `syslog`\n\nDescription:\n    Mocks src/unit/syslog.c implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit util syslog\n    snoopy-test unit util syslog --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util_syslog(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
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
    let mut tmp___23: libc::c_int = 0;
    let mut tmp___24: libc::c_int = 0;
    let mut tmp___25: libc::c_int = 0;
    let mut tmp___26: libc::c_int = 0;
    let mut tmp___27: libc::c_int = 0;
    let mut tmp___28: libc::c_int = 0;
    let mut tmp___29: libc::c_int = 0;
    let mut tmp___30: libc::c_int = 0;
    let mut tmp___31: libc::c_int = 0;
    let mut tmp___32: libc::c_int = 0;
    let mut tmp___33: libc::c_int = 0;
    let mut tmp___34: libc::c_int = 0;
    let mut tmp___35: libc::c_int = 0;
    let mut tmp___36: libc::c_int = 0;
    let mut tmp___37: libc::c_int = 0;
    let mut tmp___38: libc::c_int = 0;
    let mut tmp___39: libc::c_int = 0;
    let mut tmp___40: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___41: libc::c_int = 0;
    let mut tmp___42: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___43: libc::c_int = 0;
    let mut tmp___44: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___45: libc::c_int = 0;
    let mut tmp___46: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___47: libc::c_int = 0;
    let mut tmp___48: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___49: libc::c_int = 0;
    let mut tmp___50: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___51: libc::c_int = 0;
    let mut tmp___52: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___53: libc::c_int = 0;
    let mut tmp___54: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___55: libc::c_int = 0;
    let mut tmp___56: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___57: libc::c_int = 0;
    let mut tmp___58: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___59: libc::c_int = 0;
    let mut tmp___60: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___61: libc::c_int = 0;
    let mut tmp___62: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___63: libc::c_int = 0;
    let mut tmp___64: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___65: libc::c_int = 0;
    let mut tmp___66: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___67: libc::c_int = 0;
    let mut tmp___68: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___69: libc::c_int = 0;
    let mut tmp___70: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___71: libc::c_int = 0;
    let mut tmp___72: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___73: libc::c_int = 0;
    let mut tmp___74: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___75: libc::c_int = 0;
    let mut tmp___76: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___77: libc::c_int = 0;
    let mut tmp___78: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___79: libc::c_int = 0;
    let mut tmp___80: libc::c_int = 0;
    let mut tmp___81: libc::c_int = 0;
    let mut tmp___82: libc::c_int = 0;
    let mut tmp___83: libc::c_int = 0;
    let mut tmp___84: libc::c_int = 0;
    let mut tmp___85: libc::c_int = 0;
    let mut tmp___86: libc::c_int = 0;
    let mut tmp___87: libc::c_int = 0;
    let mut tmp___88: libc::c_int = 0;
    let mut tmp___89: libc::c_int = 0;
    let mut tmp___90: libc::c_int = 0;
    let mut tmp___91: libc::c_int = 0;
    let mut tmp___92: libc::c_int = 0;
    let mut tmp___93: libc::c_int = 0;
    let mut tmp___94: libc::c_int = 0;
    let mut tmp___95: libc::c_int = 0;
    let mut tmp___96: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___97: libc::c_int = 0;
    let mut tmp___98: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___99: libc::c_int = 0;
    let mut tmp___100: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___101: libc::c_int = 0;
    let mut tmp___102: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___103: libc::c_int = 0;
    let mut tmp___104: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___105: libc::c_int = 0;
    let mut tmp___106: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___107: libc::c_int = 0;
    let mut tmp___108: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___109: libc::c_int = 0;
    let mut tmp___110: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___111: libc::c_int = 0;
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    tmp = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_unit_util_syslog_showHelp();
        return 0 as libc::c_int;
    }
    tmp___0 = snoopy_util_syslog_convertFacilityToInt(
        b"AUTH\0" as *const u8 as *const libc::c_char,
    );
    if tmp___0 != (4 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"AUTH\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___1 = snoopy_util_syslog_convertFacilityToInt(
        b"AUTHPRIV\0" as *const u8 as *const libc::c_char,
    );
    if tmp___1 != (10 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"AUTHPRIV\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___2 = snoopy_util_syslog_convertFacilityToInt(
        b"CRON\0" as *const u8 as *const libc::c_char,
    );
    if tmp___2 != (9 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"CRON\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___3 = snoopy_util_syslog_convertFacilityToInt(
        b"DAEMON\0" as *const u8 as *const libc::c_char,
    );
    if tmp___3 != (3 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"DAEMON\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___4 = snoopy_util_syslog_convertFacilityToInt(
        b"FTP\0" as *const u8 as *const libc::c_char,
    );
    if tmp___4 != (11 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"FTP\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___5 = snoopy_util_syslog_convertFacilityToInt(
        b"KERN\0" as *const u8 as *const libc::c_char,
    );
    if tmp___5 != 0 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"KERN\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___6 = snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL0\0" as *const u8 as *const libc::c_char,
    );
    if tmp___6 != (16 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL0\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___7 = snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL1\0" as *const u8 as *const libc::c_char,
    );
    if tmp___7 != (17 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL1\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___8 = snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL2\0" as *const u8 as *const libc::c_char,
    );
    if tmp___8 != (18 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL2\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___9 = snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL3\0" as *const u8 as *const libc::c_char,
    );
    if tmp___9 != (19 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL3\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___10 = snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL4\0" as *const u8 as *const libc::c_char,
    );
    if tmp___10 != (20 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL4\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___11 = snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL5\0" as *const u8 as *const libc::c_char,
    );
    if tmp___11 != (21 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL5\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___12 = snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL6\0" as *const u8 as *const libc::c_char,
    );
    if tmp___12 != (22 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL6\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___13 = snoopy_util_syslog_convertFacilityToInt(
        b"LOCAL7\0" as *const u8 as *const libc::c_char,
    );
    if tmp___13 != (23 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOCAL7\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___14 = snoopy_util_syslog_convertFacilityToInt(
        b"LPR\0" as *const u8 as *const libc::c_char,
    );
    if tmp___14 != (6 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LPR\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___15 = snoopy_util_syslog_convertFacilityToInt(
        b"MAIL\0" as *const u8 as *const libc::c_char,
    );
    if tmp___15 != (2 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"MAIL\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___16 = snoopy_util_syslog_convertFacilityToInt(
        b"NEWS\0" as *const u8 as *const libc::c_char,
    );
    if tmp___16 != (7 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"NEWS\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___17 = snoopy_util_syslog_convertFacilityToInt(
        b"SYSLOG\0" as *const u8 as *const libc::c_char,
    );
    if tmp___17 != (5 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"SYSLOG\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___18 = snoopy_util_syslog_convertFacilityToInt(
        b"USER\0" as *const u8 as *const libc::c_char,
    );
    if tmp___18 != (1 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"USER\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___19 = snoopy_util_syslog_convertFacilityToInt(
        b"UUCP\0" as *const u8 as *const libc::c_char,
    );
    if tmp___19 != (8 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"UUCP\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___20 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_AUTH\0" as *const u8 as *const libc::c_char,
    );
    if tmp___20 != (4 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_AUTH\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___21 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_AUTHPRIV\0" as *const u8 as *const libc::c_char,
    );
    if tmp___21 != (10 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_AUTHPRIV\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___22 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_CRON\0" as *const u8 as *const libc::c_char,
    );
    if tmp___22 != (9 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_CRON\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___23 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_DAEMON\0" as *const u8 as *const libc::c_char,
    );
    if tmp___23 != (3 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_DAEMON\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___24 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_FTP\0" as *const u8 as *const libc::c_char,
    );
    if tmp___24 != (11 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_FTP\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___25 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_KERN\0" as *const u8 as *const libc::c_char,
    );
    if tmp___25 != 0 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_KERN\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___26 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL0\0" as *const u8 as *const libc::c_char,
    );
    if tmp___26 != (16 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL0\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___27 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL1\0" as *const u8 as *const libc::c_char,
    );
    if tmp___27 != (17 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL1\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___28 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL2\0" as *const u8 as *const libc::c_char,
    );
    if tmp___28 != (18 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL2\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___29 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL3\0" as *const u8 as *const libc::c_char,
    );
    if tmp___29 != (19 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL3\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___30 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL4\0" as *const u8 as *const libc::c_char,
    );
    if tmp___30 != (20 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL4\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___31 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL5\0" as *const u8 as *const libc::c_char,
    );
    if tmp___31 != (21 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL5\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___32 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL6\0" as *const u8 as *const libc::c_char,
    );
    if tmp___32 != (22 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL6\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___33 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LOCAL7\0" as *const u8 as *const libc::c_char,
    );
    if tmp___33 != (23 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LOCAL7\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___34 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_LPR\0" as *const u8 as *const libc::c_char,
    );
    if tmp___34 != (6 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_LPR\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___35 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_MAIL\0" as *const u8 as *const libc::c_char,
    );
    if tmp___35 != (2 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_MAIL\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___36 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_NEWS\0" as *const u8 as *const libc::c_char,
    );
    if tmp___36 != (7 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_NEWS\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___37 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_SYSLOG\0" as *const u8 as *const libc::c_char,
    );
    if tmp___37 != (5 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_SYSLOG\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___38 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_USER\0" as *const u8 as *const libc::c_char,
    );
    if tmp___38 != (1 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_USER\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___39 = snoopy_util_syslog_convertFacilityToInt(
        b"LOG_UUCP\0" as *const u8 as *const libc::c_char,
    );
    if tmp___39 != (8 as libc::c_int) << 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog facility str to int\0" as *const u8
                as *const libc::c_char,
            b"LOG_UUCP\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___40 = snoopy_util_syslog_convertFacilityToStr(
        (4 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___41 = strcmp(tmp___40, b"AUTH\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___41 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"AUTH\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___42 = snoopy_util_syslog_convertFacilityToStr(
        (10 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___43 = strcmp(tmp___42, b"AUTHPRIV\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___43 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"AUTHPRIV\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___44 = snoopy_util_syslog_convertFacilityToStr(
        (9 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___45 = strcmp(tmp___44, b"CRON\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___45 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"CRON\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___46 = snoopy_util_syslog_convertFacilityToStr(
        (3 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___47 = strcmp(tmp___46, b"DAEMON\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___47 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"DAEMON\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___48 = snoopy_util_syslog_convertFacilityToStr(
        (11 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___49 = strcmp(tmp___48, b"FTP\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___49 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"FTP\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___50 = snoopy_util_syslog_convertFacilityToStr(0 as libc::c_int);
    tmp___51 = strcmp(tmp___50, b"KERN\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___51 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"KERN\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___52 = snoopy_util_syslog_convertFacilityToStr(
        (16 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___53 = strcmp(tmp___52, b"LOCAL0\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___53 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL0\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___54 = snoopy_util_syslog_convertFacilityToStr(
        (17 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___55 = strcmp(tmp___54, b"LOCAL1\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___55 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL1\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___56 = snoopy_util_syslog_convertFacilityToStr(
        (18 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___57 = strcmp(tmp___56, b"LOCAL2\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___57 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL2\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___58 = snoopy_util_syslog_convertFacilityToStr(
        (19 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___59 = strcmp(tmp___58, b"LOCAL3\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___59 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL3\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___60 = snoopy_util_syslog_convertFacilityToStr(
        (20 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___61 = strcmp(tmp___60, b"LOCAL4\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___61 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL4\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___62 = snoopy_util_syslog_convertFacilityToStr(
        (21 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___63 = strcmp(tmp___62, b"LOCAL5\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___63 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL5\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___64 = snoopy_util_syslog_convertFacilityToStr(
        (22 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___65 = strcmp(tmp___64, b"LOCAL6\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___65 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL6\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___66 = snoopy_util_syslog_convertFacilityToStr(
        (23 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___67 = strcmp(tmp___66, b"LOCAL7\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___67 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LOCAL7\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___68 = snoopy_util_syslog_convertFacilityToStr(
        (6 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___69 = strcmp(tmp___68, b"LPR\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___69 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"LPR\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___70 = snoopy_util_syslog_convertFacilityToStr(
        (2 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___71 = strcmp(tmp___70, b"MAIL\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___71 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"MAIL\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___72 = snoopy_util_syslog_convertFacilityToStr(
        (7 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___73 = strcmp(tmp___72, b"NEWS\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___73 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"NEWS\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___74 = snoopy_util_syslog_convertFacilityToStr(
        (5 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___75 = strcmp(tmp___74, b"SYSLOG\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___75 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"SYSLOG\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___76 = snoopy_util_syslog_convertFacilityToStr(
        (1 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___77 = strcmp(tmp___76, b"USER\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___77 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"USER\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___78 = snoopy_util_syslog_convertFacilityToStr(
        (8 as libc::c_int) << 3 as libc::c_int,
    );
    tmp___79 = strcmp(tmp___78, b"UUCP\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___79 {
        fatalErrorValue(
            b"Error converting syslog facility int to str\0" as *const u8
                as *const libc::c_char,
            b"UUCP\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___80 = snoopy_util_syslog_convertLevelToInt(
        b"EMERG\0" as *const u8 as *const libc::c_char,
    );
    if tmp___80 != 0 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"EMERG\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___81 = snoopy_util_syslog_convertLevelToInt(
        b"ALERT\0" as *const u8 as *const libc::c_char,
    );
    if tmp___81 != 1 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"ALERT\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___82 = snoopy_util_syslog_convertLevelToInt(
        b"CRIT\0" as *const u8 as *const libc::c_char,
    );
    if tmp___82 != 2 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"CRIT\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___83 = snoopy_util_syslog_convertLevelToInt(
        b"ERR\0" as *const u8 as *const libc::c_char,
    );
    if tmp___83 != 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"ERR\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___84 = snoopy_util_syslog_convertLevelToInt(
        b"WARNING\0" as *const u8 as *const libc::c_char,
    );
    if tmp___84 != 4 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"WARNING\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___85 = snoopy_util_syslog_convertLevelToInt(
        b"NOTICE\0" as *const u8 as *const libc::c_char,
    );
    if tmp___85 != 5 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"NOTICE\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___86 = snoopy_util_syslog_convertLevelToInt(
        b"INFO\0" as *const u8 as *const libc::c_char,
    );
    if tmp___86 != 6 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"INFO\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___87 = snoopy_util_syslog_convertLevelToInt(
        b"DEBUG\0" as *const u8 as *const libc::c_char,
    );
    if tmp___87 != 7 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"DEBUG\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___88 = snoopy_util_syslog_convertLevelToInt(
        b"LOG_EMERG\0" as *const u8 as *const libc::c_char,
    );
    if tmp___88 != 0 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"EMERG\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___89 = snoopy_util_syslog_convertLevelToInt(
        b"LOG_ALERT\0" as *const u8 as *const libc::c_char,
    );
    if tmp___89 != 1 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"ALERT\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___90 = snoopy_util_syslog_convertLevelToInt(
        b"LOG_CRIT\0" as *const u8 as *const libc::c_char,
    );
    if tmp___90 != 2 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"CRIT\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___91 = snoopy_util_syslog_convertLevelToInt(
        b"LOG_ERR\0" as *const u8 as *const libc::c_char,
    );
    if tmp___91 != 3 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"ERR\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___92 = snoopy_util_syslog_convertLevelToInt(
        b"LOG_WARNING\0" as *const u8 as *const libc::c_char,
    );
    if tmp___92 != 4 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"WARNING\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___93 = snoopy_util_syslog_convertLevelToInt(
        b"LOG_NOTICE\0" as *const u8 as *const libc::c_char,
    );
    if tmp___93 != 5 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"NOTICE\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___94 = snoopy_util_syslog_convertLevelToInt(
        b"LOG_INFO\0" as *const u8 as *const libc::c_char,
    );
    if tmp___94 != 6 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"INFO\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___95 = snoopy_util_syslog_convertLevelToInt(
        b"LOG_DEBUG\0" as *const u8 as *const libc::c_char,
    );
    if tmp___95 != 7 as libc::c_int {
        fatalErrorValue(
            b"Error converting syslog level str to int\0" as *const u8
                as *const libc::c_char,
            b"DEBUG\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___96 = snoopy_util_syslog_convertLevelToStr(0 as libc::c_int);
    tmp___97 = strcmp(tmp___96, b"EMERG\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___97 {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"EMERG\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___98 = snoopy_util_syslog_convertLevelToStr(1 as libc::c_int);
    tmp___99 = strcmp(tmp___98, b"ALERT\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___99 {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"ALERT\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___100 = snoopy_util_syslog_convertLevelToStr(2 as libc::c_int);
    tmp___101 = strcmp(tmp___100, b"CRIT\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___101 {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"CRIT\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___102 = snoopy_util_syslog_convertLevelToStr(3 as libc::c_int);
    tmp___103 = strcmp(tmp___102, b"ERR\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___103 {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"ERR\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___104 = snoopy_util_syslog_convertLevelToStr(4 as libc::c_int);
    tmp___105 = strcmp(tmp___104, b"WARNING\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___105 {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"WARNING\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___106 = snoopy_util_syslog_convertLevelToStr(5 as libc::c_int);
    tmp___107 = strcmp(tmp___106, b"NOTICE\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___107 {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"NOTICE\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___108 = snoopy_util_syslog_convertLevelToStr(6 as libc::c_int);
    tmp___109 = strcmp(tmp___108, b"INFO\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___109 {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"INFO\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___110 = snoopy_util_syslog_convertLevelToStr(7 as libc::c_int);
    tmp___111 = strcmp(tmp___110, b"DEBUG\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___111 {
        fatalErrorValue(
            b"Error converting syslog level int to str\0" as *const u8
                as *const libc::c_char,
            b"DEBUG\0" as *const u8 as *const libc::c_char,
        );
    }
    printSuccess(b"Mocking src/misc.c complete.\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub static mut g_argc: libc::c_int = 0;
pub static mut g_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
pub unsafe extern "C" fn snoopyTestCli_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility\n\nUsage:\n    snoopy ACTION [SUBACTION] [ARGS]\n\nAvailable actions:\n    run            Run Snoopy's subsystem\n    stress         Run stress tests\n    unit           Run unit tests\n\n    --help,-h      Show this help\n    ACTION --help  Show ACTION's help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut action: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    let mut tmp___4: libc::c_int = 0;
    let mut tmp___5: libc::c_int = 0;
    let mut tmp___6: libc::c_int = 0;
    g_argc = argc;
    g_argv = argv;
    if argc < 2 as libc::c_int {
        snoopyTestCli_showHelp();
        fatalError(b"No action specified.\0" as *const u8 as *const libc::c_char);
    }
    action = *argv.offset(1 as libc::c_int as isize) as *const libc::c_char;
    tmp = strcmp(action, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_showHelp();
        return 0 as libc::c_int;
    } else {
        tmp___0 = strcmp(action, b"-h\0" as *const u8 as *const libc::c_char);
        if 0 as libc::c_int == tmp___0 {
            snoopyTestCli_showHelp();
            return 0 as libc::c_int;
        }
    }
    tmp___2 = strcmp(action, b"run\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___2 {
        tmp___1 = snoopyTestCli_action_run(
            argc - 2 as libc::c_int,
            argv.offset(2 as libc::c_int as isize),
        );
        return tmp___1;
    }
    tmp___4 = strcmp(action, b"stress\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___4 {
        tmp___3 = snoopyTestCli_action_stress(
            argc - 2 as libc::c_int,
            argv.offset(2 as libc::c_int as isize),
        );
        return tmp___3;
    }
    tmp___6 = strcmp(action, b"unit\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___6 {
        tmp___5 = snoopyTestCli_action_unit(
            argc - 2 as libc::c_int,
            argv.offset(2 as libc::c_int as isize),
        );
        return tmp___5;
    }
    snoopyTestCli_showHelp();
    fatalErrorValue(b"Unknown action\0" as *const u8 as *const libc::c_char, action);
    return 0 as libc::c_int;
}
pub static mut g_etcLdSoPreloadPath: *const libc::c_char = 0 as *const libc::c_char;
pub static mut g_libsnoopySoPath: *const libc::c_char = 0 as *const libc::c_char;
pub static mut g_libsnoopySoHandle: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
pub unsafe extern "C" fn printMessage(message: *const libc::c_char) {
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printDiag(message: *const libc::c_char) {
    printf(b"[DIAG] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printDiagValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    printf(b"[DIAG] %s: %s\n\0" as *const u8 as *const libc::c_char, message, value);
}
pub unsafe extern "C" fn printInfo(message: *const libc::c_char) {
    printf(b"[INFO] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printInfoValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    printf(b"[INFO] %s: %s\n\0" as *const u8 as *const libc::c_char, message, value);
}
pub unsafe extern "C" fn printNotice(message: *const libc::c_char) {
    printf(b"[NOTICE] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printNoticeValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    printf(b"[NOTICE] %s: %s\n\0" as *const u8 as *const libc::c_char, message, value);
}
pub unsafe extern "C" fn printSuccess(message: *const libc::c_char) {
    printf(b"[SUCCESS] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printWarning(message: *const libc::c_char) {
    fprintf(stderr, b"[WARNING] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printWarningValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    fprintf(
        stderr,
        b"[WARNING] %s: %s\n\0" as *const u8 as *const libc::c_char,
        message,
        value,
    );
}
pub unsafe extern "C" fn printError(message: *const libc::c_char) {
    fprintf(stderr, b"[ERROR] %s\n\0" as *const u8 as *const libc::c_char, message);
}
pub unsafe extern "C" fn printErrorValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    fprintf(
        stderr,
        b"[ERROR] %s: %s\n\0" as *const u8 as *const libc::c_char,
        message,
        value,
    );
}
pub unsafe extern "C" fn fatalError(message: *const libc::c_char) {
    printError(message);
    exit(127 as libc::c_int);
}
pub unsafe extern "C" fn fatalErrorValue(
    message: *const libc::c_char,
    value: *const libc::c_char,
) {
    printErrorValue(message, value);
    exit(127 as libc::c_int);
}
pub unsafe extern "C" fn libsnoopySo_getFilePath() -> *mut libc::c_char {
    let mut filePath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: libc::c_int = 0;
    filePath = libsnoopySo_getFilePathNoCheck();
    tmp = access(filePath as *const libc::c_char, 4 as libc::c_int);
    if tmp != 0 as libc::c_int {
        printDiagValue(
            b"libsnoopy.so path\0" as *const u8 as *const libc::c_char,
            filePath as *const libc::c_char,
        );
        fatalError(
            b"Unable to access libsnoopy.so file\0" as *const u8 as *const libc::c_char,
        );
    }
    return filePath;
}
pub unsafe extern "C" fn libsnoopySo_getFilePathNoCheck() -> *mut libc::c_char {
    let mut filePath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut envValue: *mut libc::c_char = 0 as *mut libc::c_char;
    envValue = getenv(
        b"SNOOPY_TEST_LIBSNOOPY_SO_PATH\0" as *const u8 as *const libc::c_char,
    );
    if envValue as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        printWarningValue(
            b"Using non-default path to libsnoopy.so\0" as *const u8
                as *const libc::c_char,
            envValue as *const libc::c_char,
        );
        filePath = envValue;
    } else {
        filePath = b"/usr/local/lib/libsnoopy.so\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    return filePath;
}
pub unsafe extern "C" fn libsnoopySo_load() {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    tmp = libsnoopySo_getFilePath();
    g_libsnoopySoPath = tmp as *const libc::c_char;
    g_libsnoopySoHandle = dlopen(g_libsnoopySoPath, 1 as libc::c_int);
    if g_libsnoopySoHandle.is_null() {
        tmp___0 = dlerror();
        printErrorValue(
            b"Dynamic linker error message\0" as *const u8 as *const libc::c_char,
            tmp___0 as *const libc::c_char,
        );
        fatalError(
            b"Unable to load shared library\0" as *const u8 as *const libc::c_char,
        );
    }
    dlerror();
}
pub unsafe extern "C" fn libsnoopySo_dlsym(
    functionName: *const libc::c_char,
) -> *mut libc::c_void {
    let mut functionPtr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut error: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    functionPtr = dlsym(g_libsnoopySoHandle, functionName);
    tmp = dlerror();
    error = tmp as *const libc::c_char;
    if error as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        printDiagValue(
            b"libsnoopy.so path\0" as *const u8 as *const libc::c_char,
            g_libsnoopySoPath,
        );
        fatalError(error);
    }
    return functionPtr;
}
pub unsafe extern "C" fn etcLdSoPreload_getFilePath() -> *const libc::c_char {
    let mut envValue: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    envValue = 0 as *mut libc::c_void as *const libc::c_char;
    tmp = getenv(
        b"SNOOPY_TEST_LD_SO_PRELOAD_PATH\0" as *const u8 as *const libc::c_char,
    );
    envValue = tmp as *const libc::c_char;
    if envValue as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        printWarningValue(
            b"Using non-standard path to ld.so.preload\0" as *const u8
                as *const libc::c_char,
            envValue,
        );
        return envValue;
    } else {
        return b"/etc/ld.so.preload\0" as *const u8 as *const libc::c_char
    };
}
pub unsafe extern "C" fn etcLdSoPreload_readFile() -> *mut libc::c_char {
    let mut filePath: *const libc::c_char = 0 as *const libc::c_char;
    let mut fileContentLen: libc::c_int = 0;
    let mut fileContentBuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fileHandle: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut FILE = 0 as *mut FILE;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___3: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___4: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_long = 0;
    let mut tmp___8: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___9: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___10: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut tmp___11: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___12: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut freadRes: libc::c_long = 0;
    let mut tmp___13: size_t = 0;
    fileContentBuf = 0 as *mut libc::c_char;
    filePath = etcLdSoPreload_getFilePath();
    tmp = fopen(filePath, b"r\0" as *const u8 as *const libc::c_char);
    fileHandle = tmp;
    if fileHandle as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        tmp___3 = __errno_location();
        if *tmp___3 == 2 as libc::c_int {
            tmp___0 = malloc(1 as libc::c_int as size_t);
            fileContentBuf = tmp___0 as *mut libc::c_char;
            *fileContentBuf
                .offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
            return fileContentBuf;
        } else {
            printDiagValue(
                b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
                filePath,
            );
            tmp___1 = __errno_location();
            tmp___2 = strerror(*tmp___1);
            printDiagValue(
                b"Error message\0" as *const u8 as *const libc::c_char,
                tmp___2 as *const libc::c_char,
            );
            fatalError(
                b"Unable to read ld.so.preload file.\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    tmp___6 = fseek(fileHandle, 0 as libc::c_long, 2 as libc::c_int);
    if tmp___6 != 0 as libc::c_int {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        printDiagValue(
            b"function called\0" as *const u8 as *const libc::c_char,
            b"fseek(fh, 0, SEEK_END)\0" as *const u8 as *const libc::c_char,
        );
        tmp___4 = __errno_location();
        tmp___5 = strerror(*tmp___4);
        printDiagValue(
            b"Error message\0" as *const u8 as *const libc::c_char,
            tmp___5 as *const libc::c_char,
        );
        fatalError(
            b"Unable to determine the size of the ld.so.preload file.\0" as *const u8
                as *const libc::c_char,
        );
    }
    tmp___7 = ftell(fileHandle);
    fileContentLen = tmp___7 as libc::c_int;
    if fileContentLen == -(1 as libc::c_int) {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        printDiagValue(
            b"function called\0" as *const u8 as *const libc::c_char,
            b"ftell(fh)\0" as *const u8 as *const libc::c_char,
        );
        tmp___8 = __errno_location();
        tmp___9 = strerror(*tmp___8);
        printDiagValue(
            b"Error message\0" as *const u8 as *const libc::c_char,
            tmp___9 as *const libc::c_char,
        );
        fatalError(
            b"Unable to determine the size of the ld.so.preload file.\0" as *const u8
                as *const libc::c_char,
        );
    }
    fseek(fileHandle, 0 as libc::c_long, 0 as libc::c_int);
    tmp___10 = malloc((fileContentLen + 1 as libc::c_int) as size_t);
    fileContentBuf = tmp___10 as *mut libc::c_char;
    if fileContentBuf as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        tmp___11 = __errno_location();
        tmp___12 = strerror(*tmp___11);
        printDiagValue(
            b"Error message\0" as *const u8 as *const libc::c_char,
            tmp___12 as *const libc::c_char,
        );
        fatalError(
            b"Unable to malloc() for reading the file content.\0" as *const u8
                as *const libc::c_char,
        );
    }
    if fileContentLen == 0 as libc::c_int {
        *fileContentBuf
            .offset(0 as libc::c_int as isize) = '\u{0}' as i32 as libc::c_char;
    } else {
        tmp___13 = fread(
            fileContentBuf as *mut libc::c_void,
            1 as libc::c_int as size_t,
            fileContentLen as size_t,
            fileHandle,
        );
        freadRes = tmp___13 as libc::c_long;
        if freadRes < fileContentLen as libc::c_long {
            *fileContentBuf.offset(freadRes as isize) = '\u{0}' as i32 as libc::c_char;
            printDiagValue(
                b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
                filePath,
            );
            fatalError(
                b"Unable to read the whole content of the file.\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            *fileContentBuf
                .offset(fileContentLen as isize) = '\u{0}' as i32 as libc::c_char;
        }
    }
    fclose(fileHandle);
    g_etcLdSoPreloadPath = filePath;
    return fileContentBuf;
}
pub unsafe extern "C" fn etcLdSoPreload_writeFile(mut newContent: *mut libc::c_char) {
    let mut filePath: *const libc::c_char = 0 as *const libc::c_char;
    let mut fileHandle: *mut FILE = 0 as *mut FILE;
    let mut tmp: *mut FILE = 0 as *mut FILE;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___4: libc::c_int = 0;
    filePath = etcLdSoPreload_getFilePath();
    tmp = fopen(filePath, b"w+\0" as *const u8 as *const libc::c_char);
    fileHandle = tmp;
    if fileHandle as libc::c_ulong == 0 as *mut libc::c_void as libc::c_ulong {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        printDiagValue(
            b"Error message\0" as *const u8 as *const libc::c_char,
            tmp___1 as *const libc::c_char,
        );
        fatalError(
            b"Unable to open file for writing (missing sudo, maybe?).\0" as *const u8
                as *const libc::c_char,
        );
    }
    tmp___4 = fprintf(
        fileHandle,
        b"%s\0" as *const u8 as *const libc::c_char,
        newContent,
    );
    if tmp___4 < 0 as libc::c_int {
        printDiagValue(
            b"ld.so.preload path\0" as *const u8 as *const libc::c_char,
            filePath,
        );
        tmp___2 = __errno_location();
        tmp___3 = strerror(*tmp___2);
        printDiagValue(
            b"Error message\0" as *const u8 as *const libc::c_char,
            tmp___3 as *const libc::c_char,
        );
        fatalError(b"Unable to write to file.\0" as *const u8 as *const libc::c_char);
    }
    fclose(fileHandle);
}
pub unsafe extern "C" fn etcLdSoPreload_findEntry(
    mut content: *const libc::c_char,
    mut entry: *const libc::c_char,
) -> *const libc::c_char {
    let mut contentPos: *const libc::c_char = 0 as *const libc::c_char;
    let mut entryPos: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: size_t = 0;
    let mut tmp___1: size_t = 0;
    let mut tmp___2: size_t = 0;
    let mut tmp___3: size_t = 0;
    let mut tmp___4: size_t = 0;
    let mut tmp___5: *mut libc::c_char = 0 as *mut libc::c_char;
    contentPos = 0 as *mut libc::c_void as *const libc::c_char;
    entryPos = 0 as *mut libc::c_void as *const libc::c_char;
    contentPos = content;
    loop {
        tmp___5 = strstr(contentPos, entry);
        entryPos = tmp___5 as *const libc::c_char;
        if !(entryPos as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong) {
            break;
        }
        let mut current_block_28: u64;
        if entryPos as libc::c_ulong == content as libc::c_ulong {
            current_block_28 = 5318666750832191330;
        } else if entryPos as libc::c_ulong > content as libc::c_ulong {
            if *entryPos.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == 10 as libc::c_int
            {
                current_block_28 = 5318666750832191330;
            } else {
                current_block_28 = 17788412896529399552;
            }
        } else {
            current_block_28 = 17788412896529399552;
        }
        match current_block_28 {
            5318666750832191330 => {
                tmp = strlen(entry);
                if *entryPos.offset(tmp as isize) as libc::c_int == 0 as libc::c_int {
                    return entryPos
                } else {
                    tmp___0 = strlen(entry);
                    if *entryPos.offset(tmp___0 as isize) as libc::c_int
                        == 10 as libc::c_int
                    {
                        return entryPos
                    } else {
                        tmp___1 = strlen(entry);
                        if *entryPos.offset(tmp___1 as isize) as libc::c_int
                            == 35 as libc::c_int
                        {
                            return entryPos
                        } else {
                            tmp___2 = strlen(entry);
                            if *entryPos.offset(tmp___2 as isize) as libc::c_int
                                == 32 as libc::c_int
                            {
                                return entryPos
                            } else {
                                tmp___3 = strlen(entry);
                                if *entryPos.offset(tmp___3 as isize) as libc::c_int
                                    == 9 as libc::c_int
                                {
                                    return entryPos;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        tmp___4 = strlen(entry);
        contentPos = entryPos.offset(tmp___4 as isize);
    }
    return 0 as *mut libc::c_void as *const libc::c_char;
}
pub unsafe extern "C" fn etcLdSoPreload_findNonCommentLineContainingString(
    mut content: *const libc::c_char,
    mut searchString: *const libc::c_char,
) -> *const libc::c_char {
    let mut contentPos: *const libc::c_char = 0 as *const libc::c_char;
    let mut foundStringPos: *const libc::c_char = 0 as *const libc::c_char;
    let mut lineStartPtr: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: size_t = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    contentPos = 0 as *mut libc::c_void as *const libc::c_char;
    foundStringPos = 0 as *mut libc::c_void as *const libc::c_char;
    lineStartPtr = 0 as *mut libc::c_void as *const libc::c_char;
    contentPos = content;
    loop {
        tmp___0 = strstr(contentPos, searchString);
        foundStringPos = tmp___0 as *const libc::c_char;
        if !(foundStringPos as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong)
        {
            break;
        }
        lineStartPtr = foundStringPos;
        while lineStartPtr as libc::c_ulong > contentPos as libc::c_ulong {
            if !(*lineStartPtr as libc::c_int != 10 as libc::c_int) {
                break;
            }
            lineStartPtr = lineStartPtr.offset(-1);
        }
        if *lineStartPtr as libc::c_int == 10 as libc::c_int {
            lineStartPtr = lineStartPtr.offset(1);
        }
        if *lineStartPtr as libc::c_int != 35 as libc::c_int {
            return lineStartPtr;
        }
        tmp = strlen(searchString);
        contentPos = foundStringPos.offset(tmp as isize);
    }
    return 0 as *mut libc::c_void as *const libc::c_char;
}
pub unsafe extern "C" fn displayHelp() {
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Usage: \n\0" as *const u8 as *const libc::c_char);
    printf(
        b"    snoopy-test-configfile PATH-TO-INI CONFIG-VARIABLE-TO-DISPLAY\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    printf(b"Available configfile variables:\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"    (check etc/snoopy.ini for list of supported configuration variables)\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\n\0" as *const u8 as *const libc::c_char);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_configfile_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `configfile`\n\nUsage:\n    snoopy-test run configfile INI_FILE KEY\n\nResult:\n    Prints value of the requested configuration KEY from the given INI_FILE.\n\nSupported configuration keys (check etc/snoopy.ini for more information):\n    message_format\n    filter_chain\n    output\n    syslog_facility\n    syslog_ident\n    syslog_level\nNOTICE: These keys MUST be placed in a section named [snoopy].\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_configfile(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut iniFilePath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut showConfigVar: *const libc::c_char = 0 as *const libc::c_char;
    let mut CFG: *mut snoopy_configuration_t = 0 as *mut snoopy_configuration_t;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___4: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___5: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: libc::c_int = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: libc::c_int = 0;
    let mut tmp___10: libc::c_int = 0;
    let mut tmp___11: libc::c_int = 0;
    let mut tmp___12: libc::c_int = 0;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_configfile_showHelp();
        fatalError(
            b"Missing argument: path to INI config file\0" as *const u8
                as *const libc::c_char,
        );
    }
    tmp = strcmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"--help\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_run_configfile_showHelp();
        return 0 as libc::c_int;
    }
    iniFilePath = *argv.offset(0 as libc::c_int as isize);
    if argc < 2 as libc::c_int {
        snoopyTestCli_action_run_configfile_showHelp();
        fatalError(
            b"Missing argument: configuration variable to display\0" as *const u8
                as *const libc::c_char,
        );
    }
    showConfigVar = *argv.offset(1 as libc::c_int as isize) as *const libc::c_char;
    tmp___2 = access(iniFilePath as *const libc::c_char, 4 as libc::c_int);
    if -(1 as libc::c_int) == tmp___2 {
        snoopyTestCli_action_run_configfile_showHelp();
        printErrorValue(
            b"INI file path\0" as *const u8 as *const libc::c_char,
            iniFilePath as *const libc::c_char,
        );
        tmp___0 = __errno_location();
        tmp___1 = strerror(*tmp___0);
        fatalErrorValue(
            b"Unable to open/read given INI file\0" as *const u8 as *const libc::c_char,
            tmp___1 as *const libc::c_char,
        );
    }
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        iniFilePath,
    );
    CFG = snoopy_configuration_get();
    tmp___12 = strcmp(
        showConfigVar,
        b"message_format\0" as *const u8 as *const libc::c_char,
    );
    if 0 as libc::c_int == tmp___12 {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*CFG).message_format);
    } else {
        tmp___11 = strcmp(
            showConfigVar,
            b"filter_chain\0" as *const u8 as *const libc::c_char,
        );
        if 0 as libc::c_int == tmp___11 {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, (*CFG).filter_chain);
        } else {
            tmp___10 = strcmp(
                showConfigVar,
                b"output\0" as *const u8 as *const libc::c_char,
            );
            if 0 as libc::c_int == tmp___10 {
                printf(b"%s\0" as *const u8 as *const libc::c_char, (*CFG).output);
                if 0 as libc::c_int
                    != *((*CFG).output_arg).offset(0 as libc::c_int as isize)
                        as libc::c_int
                {
                    printf(
                        b":%s\0" as *const u8 as *const libc::c_char,
                        (*CFG).output_arg,
                    );
                }
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            } else {
                tmp___9 = strcmp(
                    showConfigVar,
                    b"syslog_facility\0" as *const u8 as *const libc::c_char,
                );
                if 0 as libc::c_int == tmp___9 {
                    tmp___3 = snoopy_util_syslog_convertFacilityToStr(
                        (*CFG).syslog_facility,
                    );
                    printf(b"%s\n\0" as *const u8 as *const libc::c_char, tmp___3);
                } else {
                    tmp___8 = strcmp(
                        showConfigVar,
                        b"syslog_ident\0" as *const u8 as *const libc::c_char,
                    );
                    if 0 as libc::c_int == tmp___8 {
                        printf(
                            b"%s\n\0" as *const u8 as *const libc::c_char,
                            (*CFG).syslog_ident_format,
                        );
                    } else {
                        tmp___7 = strcmp(
                            showConfigVar,
                            b"syslog_level\0" as *const u8 as *const libc::c_char,
                        );
                        if 0 as libc::c_int == tmp___7 {
                            tmp___4 = snoopy_util_syslog_convertLevelToStr(
                                (*CFG).syslog_level,
                            );
                            printf(
                                b"%s\n\0" as *const u8 as *const libc::c_char,
                                tmp___4,
                            );
                        } else {
                            tmp___6 = strcmp(
                                showConfigVar,
                                b"error_logging\0" as *const u8 as *const libc::c_char,
                            );
                            if 0 as libc::c_int == tmp___6 {
                                if (*CFG).error_logging_enabled == 1 as libc::c_int {
                                    tmp___5 = b"y\0" as *const u8 as *const libc::c_char;
                                } else {
                                    tmp___5 = b"n\0" as *const u8 as *const libc::c_char;
                                }
                                printf(
                                    b"%s\n\0" as *const u8 as *const libc::c_char,
                                    tmp___5,
                                );
                            } else {
                                fatalErrorValue(
                                    b"Unknown setting given\0" as *const u8
                                        as *const libc::c_char,
                                    showConfigVar,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    snoopy_entrypoint_test_cli_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_ext_ini_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `ext-ini`\n\nDescription:\n    Mocks external ini implementation code paths (mainly for the code coverage of parts not used by Snoopy).\n\nUsage:\n    snoopy-test unit ext-ini\n    snoopy-test unit ext-ini --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_ext_ini_parserCallback(
    mut userPtr: *mut libc::c_void,
    mut section: *const libc::c_char,
    mut name: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    let mut errorMessage: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    errorMessage = userPtr as *mut *const libc::c_char;
    if *errorMessage as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        return 1 as libc::c_int;
    }
    tmp = strcmp(section, b"sectionName\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp {
        *errorMessage = b"Unexpected section\0" as *const u8 as *const libc::c_char;
        return 1 as libc::c_int;
    }
    tmp___0 = strcmp(name, b"testKey\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___0 {
        *errorMessage = b"Unexpected key\0" as *const u8 as *const libc::c_char;
        return 1 as libc::c_int;
    }
    tmp___1 = strcmp(value, b"testVal\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int != tmp___1 {
        *errorMessage = b"Unexpected value\0" as *const u8 as *const libc::c_char;
        return 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_ext_ini(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut iniContent: *const libc::c_char = 0 as *const libc::c_char;
    let mut errorMessage: *const libc::c_char = 0 as *const libc::c_char;
    let mut parserStatus: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    tmp = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_unit_ext_ini_showHelp();
        return 0 as libc::c_int;
    }
    iniContent = b"[sectionName]\ntestKey=testVal\n\0" as *const u8
        as *const libc::c_char;
    errorMessage = 0 as *mut libc::c_void as *const libc::c_char;
    parserStatus = 0 as libc::c_int;
    tmp___0 = snoopy_ini_parse_string(
        iniContent,
        Some(
            snoopyTestCli_action_unit_ext_ini_parserCallback
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *const libc::c_char,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
        ),
        &mut errorMessage as *mut *const libc::c_char as *mut libc::c_void,
    );
    parserStatus = tmp___0;
    if parserStatus < 0 as libc::c_int {
        fatalError(b"INI parsing failed\0" as *const u8 as *const libc::c_char);
    }
    if errorMessage as libc::c_ulong != 0 as *mut libc::c_void as libc::c_ulong {
        fatalErrorValue(
            b"INI parsing failure\0" as *const u8 as *const libc::c_char,
            errorMessage,
        );
    }
    printSuccess(
        b"Mocking lib/inih/src/ini.c complete.\0" as *const u8 as *const libc::c_char,
    );
    snoopy_entrypoint_test_cli_exit();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filter_showList() {
    let mut fCount: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp___0: *mut libc::c_char = 0 as *mut libc::c_char;
    printf(b"Available filters:\n\0" as *const u8 as *const libc::c_char);
    tmp = snoopy_filterregistry_getCount();
    fCount = tmp;
    i = 0 as libc::c_int;
    while i < fCount {
        tmp___0 = snoopy_filterregistry_getName(i);
        printf(b"    %s\n\0" as *const u8 as *const libc::c_char, tmp___0);
        i += 1;
    }
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filter_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `filter`\n\nUsage:\n    snoopy-test run filter FILTER [FILTER_ARGS]\n    snoopy-test run filter --all\n    snoopy-test run filter --list\n    snoopy-test run filter --help\n\nResult:\n    Prints the result of a called filter as a \"PASS\" or a \"DROP\" to stdout.\n    Sets the exit status to 0 or PASS or 1 for DROP.\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
    snoopyTestCli_action_run_filter_showList();
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filter(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterName: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterArg: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterResult: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_filter_showHelp();
        fatalError(
            b"Missing argument: filter name, or --all, or --list, or --help\0"
                as *const u8 as *const libc::c_char,
        );
    }
    arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    tmp = strcmp(arg1, b"--all\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_run_filter_all();
        return 0 as libc::c_int;
    }
    tmp___0 = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___0 {
        snoopyTestCli_action_run_filter_showHelp();
        return 0 as libc::c_int;
    }
    tmp___1 = strcmp(arg1, b"--list\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp___1 {
        snoopyTestCli_action_run_filter_showList();
        return 0 as libc::c_int;
    }
    filterName = arg1;
    if argc > 1 as libc::c_int {
        filterArg = *argv.offset(1 as libc::c_int as isize) as *const libc::c_char;
    } else {
        filterArg = b"\0" as *const u8 as *const libc::c_char;
    }
    tmp___2 = snoopy_filterregistry_doesNameExist(filterName);
    if 0 as libc::c_int == tmp___2 {
        snoopyTestCli_action_run_filter_showHelp();
        fatalErrorValue(
            b"Invalid filter name given\0" as *const u8 as *const libc::c_char,
            filterName,
        );
    }
    filterResult = snoopy_filterregistry_callByName(filterName, filterArg);
    snoopy_entrypoint_test_cli_exit();
    if 1 as libc::c_int == filterResult {
        printf(b"PASS\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    } else {
        printf(b"DROP\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filter_all() {
    let mut itemName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut itemArgs: *const libc::c_char = 0 as *const libc::c_char;
    let mut itemResult: libc::c_int = 0;
    let mut fCount: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    itemName = 0 as *mut libc::c_void as *mut libc::c_char;
    itemArgs = 0 as *mut libc::c_void as *const libc::c_char;
    fCount = snoopy_filterregistry_getCount();
    i = 0 as libc::c_int;
    while i < fCount {
        itemName = snoopy_filterregistry_getName(i);
        printf(b"Filter %19s: \0" as *const u8 as *const libc::c_char, itemName);
        tmp___2 = strcmp(
            itemName as *const libc::c_char,
            b"exclude_spawns_of\0" as *const u8 as *const libc::c_char,
        );
        if tmp___2 == 0 as libc::c_int {
            itemArgs = b"asdf,bsdf\0" as *const u8 as *const libc::c_char;
        } else {
            tmp___1 = strcmp(
                itemName as *const libc::c_char,
                b"exclude_uid\0" as *const u8 as *const libc::c_char,
            );
            if tmp___1 == 0 as libc::c_int {
                itemArgs = b"0\0" as *const u8 as *const libc::c_char;
            } else {
                tmp___0 = strcmp(
                    itemName as *const libc::c_char,
                    b"only_root\0" as *const u8 as *const libc::c_char,
                );
                if tmp___0 == 0 as libc::c_int {
                    itemArgs = b"\0" as *const u8 as *const libc::c_char;
                } else {
                    tmp = strcmp(
                        itemName as *const libc::c_char,
                        b"only_uid\0" as *const u8 as *const libc::c_char,
                    );
                    if tmp == 0 as libc::c_int {
                        itemArgs = b"0\0" as *const u8 as *const libc::c_char;
                    } else {
                        itemArgs = b"\0" as *const u8 as *const libc::c_char;
                    }
                }
            }
        }
        itemResult = snoopy_filterregistry_callById(i, itemArgs);
        if 1 as libc::c_int == itemResult {
            printf(b"PASS\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"DROP\0" as *const u8 as *const libc::c_char);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filterchain_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `run` :: Subsystem `filterchain`\n\nUsage:\n    snoopy-test run filterchain \"FILTER_CHAIN\"\n    snoopy-test run filterchain --help\n\nDescription:\n    Runs MESSAGE through a specified FILTER_CHAIN, with filters acting on the data taken from the current process.\n    Filter chain specification format is described in the comments of snoopy.ini.\n\nResult:\n    Prints the result of a filter chain as a \"PASS\" or a \"DROP\" to stdout.\n    Sets the exit status to 0 or PASS or 1 for DROP.\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_run_filterchain(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterChain: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterResult: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    snoopy_entrypoint_test_cli_init(
        *g_argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        g_argv as *const *mut libc::c_char,
        0 as *mut libc::c_void as *mut libc::c_char,
    );
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_filterchain_showHelp();
        fatalError(
            b"Missing argument: filter chain specification, or --help\0" as *const u8
                as *const libc::c_char,
        );
    }
    arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    tmp = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_run_filterchain_showHelp();
        return 0 as libc::c_int;
    }
    filterChain = arg1;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_run_filterchain_showHelp();
        fatalError(
            b"Missing argument: filter chain specification\0" as *const u8
                as *const libc::c_char,
        );
    }
    filterResult = snoopy_filtering_check_chain(filterChain);
    snoopy_entrypoint_test_cli_exit();
    if 1 as libc::c_int == filterResult {
        printf(b"PASS\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    } else {
        printf(b"DROP\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    };
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_filterregistry_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `filterregistry`\n\nDescription:\n    Mocks filterregistry implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit filterregistry\n    snoopy-test unit filterregistry --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_filterregistry(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut filterName: *const libc::c_char = 0 as *const libc::c_char;
    let mut filterCount: libc::c_int = 0;
    let mut filterIdPreset: libc::c_int = 0;
    let mut filterIdReceived: libc::c_int = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    tmp = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_unit_filterregistry_showHelp();
        return 0 as libc::c_int;
    }
    filterName = 0 as *mut libc::c_void as *const libc::c_char;
    filterCount = 0 as libc::c_int;
    filterIdPreset = 0 as libc::c_int;
    filterIdReceived = 0 as libc::c_int;
    filterCount = snoopy_filterregistry_getCount();
    if filterCount < 1 as libc::c_int {
        fatalError(
            b"No filters available, filter count is 0\0" as *const u8
                as *const libc::c_char,
        );
    }
    printf(
        b"[DEBUG] Number of filters available: %d\n\0" as *const u8
            as *const libc::c_char,
        filterCount,
    );
    tmp___0 = snoopy_filterregistry_doesIdExist(filterIdPreset);
    if tmp___0 != 1 as libc::c_int {
        fatalError(
            b"Filter with a preset ID (0) does not exist\0" as *const u8
                as *const libc::c_char,
        );
    }
    tmp___1 = snoopy_filterregistry_getName(filterIdPreset);
    filterName = tmp___1 as *const libc::c_char;
    filterIdReceived = snoopy_filterregistry_getIdFromName(filterName);
    if filterIdPreset != filterIdReceived {
        fatalError(
            b"Filter ID returned when searching by name differs from the initially used ID to find that same filter\0"
                as *const u8 as *const libc::c_char,
        );
    }
    filterName = b"noop\0" as *const u8 as *const libc::c_char;
    filterIdReceived = snoopy_filterregistry_getIdFromName(filterName);
    snoopy_filterregistry_callById(
        filterIdReceived,
        b"\0" as *const u8 as *const libc::c_char,
    );
    tmp___2 = snoopy_filterregistry_callById(
        -(1 as libc::c_int),
        b"\0" as *const u8 as *const libc::c_char,
    );
    if -(1 as libc::c_int) != tmp___2 {
        fatalError(
            b"Filter ID -1 unexpectedly exists\0" as *const u8 as *const libc::c_char,
        );
    }
    tmp___3 = snoopy_filterregistry_callByName(
        b"fakeFilterNameThatShouldNeverExist\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if -(1 as libc::c_int) != tmp___3 {
        fatalError(
            b"Filter with an unexpected name actually exists\0" as *const u8
                as *const libc::c_char,
        );
    }
    printSuccess(
        b"Mocking src/filterregistry.c complete.\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
pub static mut snoopyTestCli_action_stress_threads_tRepo: [pthread_t; 10000] = [0; 10000];
pub static mut threadCountMutex: pthread_mutex_t = __anonunion_pthread_mutex_t_335460617 {
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
pub static mut threadCountCreated: libc::c_int = 0 as libc::c_int;
pub static mut threadCountAliveNow: libc::c_int = 0 as libc::c_int;
pub static mut threadCountAliveMax: libc::c_int = 0 as libc::c_int;
pub static mut verbose: libc::c_int = 0;
pub unsafe extern "C" fn snoopyTestCli_action_stress_threads_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `stress` :: Subsystem `threads`\n\nUsage:\n    snoopy-test stress threads THREAD_COUNT [-v]\n\nDescription:\n    Stresses Snoopy's threading implementation by creating and destroying THREAD_COUNT\n    threads as fast as possible.\n\nArguments:\n    THREAD_COUNT       Number of threads to create and destroy\n    -v                 Verbose debugging output\n\nOutput:\n    Various threading-related messages and some statistics at the end.\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_threads(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut threadsToCreate: libc::c_int = 0;
    let mut maxConcurrentThreadsObserved: libc::c_int = 0;
    let mut retVal: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tArgs: *mut tData_t = 0 as *mut tData_t;
    let mut tmp___0: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ts_sleep: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut tmp___1: libc::c_int = 0;
    let mut i___0: libc::c_int = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: libc::c_int = 0;
    maxConcurrentThreadsObserved = 0 as libc::c_int;
    retVal = 0 as libc::c_int;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_stress_threads_showHelp();
        fatalError(
            b"Missing argument: number of threads to create\0" as *const u8
                as *const libc::c_char,
        );
    }
    threadsToCreate = atoi(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
    );
    if threadsToCreate < 1 as libc::c_int {
        snoopyTestCli_action_stress_threads_showHelp();
        fatalError(
            b"Invalid number of threads to create (min 1, max THREAD_COUNT_MAX)\0"
                as *const u8 as *const libc::c_char,
        );
    } else if threadsToCreate > 10000 as libc::c_int {
        snoopyTestCli_action_stress_threads_showHelp();
        fatalError(
            b"Invalid number of threads to create (min 1, max THREAD_COUNT_MAX)\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if argc >= 2 as libc::c_int {
        tmp = strcmp(
            *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
            b"-v\0" as *const u8 as *const libc::c_char,
        );
        if 0 as libc::c_int == tmp {
            verbose = 1 as libc::c_int;
        } else {
            verbose = 0 as libc::c_int;
        }
    } else {
        verbose = 0 as libc::c_int;
    }
    snoopy_configuration_preinit_disableConfigFileParsing();
    printf(b"M: Starting threads... \0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < threadsToCreate {
        tmp___0 = malloc(::std::mem::size_of::<tData_t>() as libc::c_ulong);
        tArgs = tmp___0 as *mut tData_t;
        (*tArgs).seqNr = i;
        if verbose != 0 {
            printf(
                b" M: Starting thread #%d:\n\0" as *const u8 as *const libc::c_char,
                i + 1 as libc::c_int,
            );
        }
        retVal = pthread_create(
            &mut *snoopyTestCli_action_stress_threads_tRepo
                .as_mut_ptr()
                .offset(i as isize) as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                snoopyTestCli_action_stress_threads_threadMain
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            tArgs as *mut libc::c_void,
        );
        i += 1;
    }
    printf(b"done.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"M: Threads alive right after thread creation was completed: %d\n\0"
            as *const u8 as *const libc::c_char,
        threadCountAliveNow,
    );
    if verbose != 0 {
        ts_sleep.tv_sec = 0 as libc::c_int as __time_t;
        ts_sleep.tv_nsec = 200000000 as libc::c_int as __syscall_slong_t;
        nanosleep(
            &mut ts_sleep as *mut timespec as *const timespec,
            0 as *mut libc::c_void as *mut timespec,
        );
        maxConcurrentThreadsObserved = snoopy_tsrm_get_threadCount();
        printf(
            b"M: Threads after first sleep: %d\n\0" as *const u8 as *const libc::c_char,
            maxConcurrentThreadsObserved,
        );
        nanosleep(
            &mut ts_sleep as *mut timespec as *const timespec,
            0 as *mut libc::c_void as *mut timespec,
        );
        tmp___1 = snoopy_tsrm_get_threadCount();
        printf(
            b"M: Threads after all threads are supposedly finished: %d\n\0" as *const u8
                as *const libc::c_char,
            tmp___1,
        );
    }
    printf(
        b"M: Waiting for all threads to finish... \0" as *const u8 as *const libc::c_char,
    );
    i___0 = 0 as libc::c_int;
    while i___0 < threadsToCreate {
        pthread_join(
            snoopyTestCli_action_stress_threads_tRepo[i___0 as usize],
            0 as *mut libc::c_void as *mut *mut libc::c_void,
        );
        if verbose != 0 {
            printf(
                b"M: Thread joined: #%d\n\0" as *const u8 as *const libc::c_char,
                i___0 + 1 as libc::c_int,
            );
            printf(
                b" M: Thread #%d joined.\n\0" as *const u8 as *const libc::c_char,
                i___0 + 1 as libc::c_int,
            );
        }
        i___0 += 1;
    }
    printf(b"done.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"M: Number of threads created:        %d\n\0" as *const u8
            as *const libc::c_char,
        threadCountCreated,
    );
    printf(
        b"M: Max threads alive simultaneously: %d\n\0" as *const u8
            as *const libc::c_char,
        threadCountAliveMax,
    );
    if verbose != 0 {
        tmp___2 = snoopy_tsrm_get_threadCount();
        printf(
            b"M: Threads after all threads, except main, have finished: %d\n\0"
                as *const u8 as *const libc::c_char,
            tmp___2,
        );
    }
    if verbose != 0 {
        tmp___3 = snoopy_tsrm_get_threadCount();
        printf(
            b"M: Threads after all threads have finished: %d\n\0" as *const u8
                as *const libc::c_char,
            tmp___3,
        );
    }
    if verbose != 0 {
        printf(
            b"SUCCESS! Expected Snoopy threads count reached: %d\n\0" as *const u8
                as *const libc::c_char,
            maxConcurrentThreadsObserved,
        );
    }
    return retVal;
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_threads_threadMain(
    mut args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tArgs: *mut tData_t = 0 as *mut tData_t;
    let mut seqNr: libc::c_int = 0;
    let mut seqNrPub: libc::c_int = 0;
    let mut dsCount: libc::c_int = 0;
    let mut dsId: libc::c_int = 0;
    let mut dsName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dsArg: *const libc::c_char = 0 as *const libc::c_char;
    let mut dsResult: [libc::c_char; 2048] = [0; 2048];
    let mut retVal: libc::c_int = 0;
    let mut tmp: pthread_t = 0;
    let mut tmp___0: libc::c_int = 0;
    let mut tmp___1: pthread_t = 0;
    let mut tmp___2: libc::c_int = 0;
    let mut tmp___3: pthread_t = 0;
    let mut tmp___4: pthread_t = 0;
    let mut tmp___5: pthread_t = 0;
    let mut tmp___6: libc::c_int = 0;
    let mut tmp___7: pthread_t = 0;
    let mut tmp___8: libc::c_int = 0;
    let mut tmp___9: pthread_t = 0;
    let mut tmp___10: pthread_t = 0;
    tArgs = args as *mut tData_t;
    seqNr = (*tArgs).seqNr;
    seqNrPub = seqNr + 1 as libc::c_int;
    dsArg = b"\0" as *const u8 as *const libc::c_char;
    pthread_mutex_lock(&mut threadCountMutex);
    threadCountCreated += 1;
    threadCountAliveNow += 1;
    if threadCountAliveNow > threadCountAliveMax {
        threadCountAliveMax = threadCountAliveNow;
    }
    pthread_mutex_unlock(&mut threadCountMutex);
    if verbose != 0 {
        tmp = pthread_self();
        printf(
            b"    t%d %llu : Hello from thread #%d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            tmp as libc::c_ulonglong,
            seqNrPub,
        );
    }
    if verbose != 0 {
        tmp___0 = snoopy_tsrm_get_threadCount();
        tmp___1 = pthread_self();
        printf(
            b"    t%d %llu : Threads before snoopy_init():    %d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            tmp___1 as libc::c_ulonglong,
            tmp___0,
        );
    }
    snoopy_entrypoint_test_cli_threads_init();
    if verbose != 0 {
        tmp___2 = snoopy_tsrm_get_threadCount();
        tmp___3 = pthread_self();
        printf(
            b"    t%d %llu : Threads after  snoopy_init():    %d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            tmp___3 as libc::c_ulonglong,
            tmp___2,
        );
    }
    dsCount = snoopy_datasourceregistry_getCount();
    dsId = snoopyTestCli_action_stress_threads_randomNumberInclusive(
        0 as libc::c_int,
        dsCount - 1 as libc::c_int,
    );
    dsName = snoopy_datasourceregistry_getName(dsId);
    retVal = snoopy_datasourceregistry_callById(dsId, dsResult.as_mut_ptr(), dsArg);
    if 0 as libc::c_int > retVal {
        tmp___4 = pthread_self();
        printf(
            b"    t%d %llu : Datasource %s returned negative result: %d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            tmp___4 as libc::c_ulonglong,
            dsName,
            retVal,
        );
    } else {
        tmp___5 = pthread_self();
        printf(
            b"    t%d %llu : DS result: %30s = %s\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            tmp___5 as libc::c_ulonglong,
            dsName,
            dsResult.as_mut_ptr(),
        );
    }
    if verbose != 0 {
        tmp___6 = snoopy_tsrm_get_threadCount();
        tmp___7 = pthread_self();
        printf(
            b"    t%d %llu : Threads before snoopy_cleanup(): %d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            tmp___7 as libc::c_ulonglong,
            tmp___6,
        );
    }
    snoopy_entrypoint_test_cli_exit();
    if verbose != 0 {
        tmp___8 = snoopy_tsrm_get_threadCount();
        tmp___9 = pthread_self();
        printf(
            b"    t%d %llu : Threads after  snoopy_cleanup(): %d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            tmp___9 as libc::c_ulonglong,
            tmp___8,
        );
    }
    if verbose != 0 {
        tmp___10 = pthread_self();
        printf(
            b"    t%d %llu : Thread exiting: #%d\n\0" as *const u8
                as *const libc::c_char,
            seqNrPub,
            tmp___10 as libc::c_ulonglong,
            seqNrPub,
        );
    }
    pthread_mutex_lock(&mut threadCountMutex);
    threadCountAliveNow -= 1;
    pthread_mutex_unlock(&mut threadCountMutex);
    free(tArgs as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_threads_randomNumberInclusive(
    mut nMin: libc::c_int,
    mut nMax: libc::c_int,
) -> libc::c_int {
    let mut randomNrRaw: libc::c_int = 0;
    let mut randomNr: libc::c_int = 0;
    let mut bytesRead: ssize_t = 0;
    let mut buffer: [libc::c_uchar; 4] = [0; 4];
    let mut fd: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut i: libc::c_uint = 0;
    randomNrRaw = 0 as libc::c_int;
    bytesRead = 0 as libc::c_int as ssize_t;
    tmp = open(b"/dev/urandom\0" as *const u8 as *const libc::c_char, 0 as libc::c_int);
    fd = tmp;
    if -(1 as libc::c_int) == fd {
        printf(
            b"ERROR: Unable to open /dev/urandom.\n\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    bytesRead = read(
        fd,
        buffer.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
    );
    close(fd);
    if bytesRead as libc::c_ulong
        != ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
    {
        printf(
            b"ERROR: Unable to read %lu bytes from /dev/urandom, only got %li bytes.\n\0"
                as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            bytesRead,
        );
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_uint;
    while (i as libc::c_ulong) < ::std::mem::size_of::<libc::c_int>() as libc::c_ulong {
        randomNrRaw
            += (buffer[i as usize] as libc::c_int) << i.wrapping_mul(8 as libc::c_uint);
        i = i.wrapping_add(1);
    }
    if randomNrRaw < 0 as libc::c_int {
        randomNrRaw = -randomNrRaw;
    }
    randomNr = randomNrRaw % (nMax - nMin + 1 as libc::c_int) + nMin;
    return randomNr;
}
pub static mut snoopyTestCli_action_stress_threadsexec_runCmdAndArgv: *mut *mut libc::c_char = 0
    as *const *mut libc::c_char as *mut *mut libc::c_char;
pub static mut snoopyTestCli_action_stress_threadsexec_tRepo: [pthread_t; 10000] = [0; 10000];
pub unsafe extern "C" fn snoopyTestCli_action_stress_threadsexec_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `stress` :: Subsystem `threadsexec`\n\nUsage:\n    snoopy-test stress threadsexec THREAD_COUNT CMD [CMD_ARGS]\n\nDescription:\n    Stresses Snoopy's threading implementation by creating and destroying THREAD_COUNT\n    threads as fast as possible and executing CMD from those threads.\n\nArguments:\n    THREAD_COUNT       Number of threads to create and destroy\n    CMD                External command to execute from each newly created thread\n    [CMD_ARGS]         Optional argument(s) for the external command\n\nOutput:\n    Various threading-related messages are shown, followed by a word \"SUCCESS!\".\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_threadsexec(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut threadsToCreate: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut retVal: libc::c_int = 0;
    let mut tArgs: *mut tData_t = 0 as *mut tData_t;
    let mut tmp: *mut libc::c_void = 0 as *mut libc::c_void;
    retVal = 0 as libc::c_int;
    if argc < 1 as libc::c_int {
        snoopyTestCli_action_stress_threadsexec_showHelp();
        fatalError(
            b"Missing argument: number of threads to run\0" as *const u8
                as *const libc::c_char,
        );
    }
    threadsToCreate = atoi(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
    );
    if threadsToCreate < 1 as libc::c_int {
        snoopyTestCli_action_stress_threadsexec_showHelp();
        fatalErrorValue(
            b"Invalid number of threads to create (min 1, max THREAD_COUNT_MAX)\0"
                as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        );
    } else if threadsToCreate > 10000 as libc::c_int {
        snoopyTestCli_action_stress_threadsexec_showHelp();
        fatalErrorValue(
            b"Invalid number of threads to create (min 1, max THREAD_COUNT_MAX)\0"
                as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        );
    }
    if argc < 2 as libc::c_int {
        snoopyTestCli_action_stress_threadsexec_showHelp();
        fatalError(
            b"Missing argument: external command to run\0" as *const u8
                as *const libc::c_char,
        );
    }
    snoopyTestCli_action_stress_threadsexec_runCmdAndArgv = argv
        .offset(1 as libc::c_int as isize);
    printf(b"M: Starting threads:\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < threadsToCreate {
        tmp = malloc(::std::mem::size_of::<tData_t>() as libc::c_ulong);
        tArgs = tmp as *mut tData_t;
        (*tArgs).seqNr = i;
        printf(
            b" M: Starting thread #%d:\n\0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        retVal = pthread_create(
            &mut *snoopyTestCli_action_stress_threadsexec_tRepo
                .as_mut_ptr()
                .offset(i as isize) as *mut pthread_t,
            0 as *mut libc::c_void as *const pthread_attr_t,
            Some(
                snoopyTestCli_action_stress_threadsexec_threadMain
                    as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
            ),
            tArgs as *mut libc::c_void,
        );
        i += 1;
    }
    printf(b"M: All threads started\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"M: Waiting for all threads to finish:\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < threadsToCreate {
        pthread_join(
            snoopyTestCli_action_stress_threadsexec_tRepo[i as usize],
            0 as *mut libc::c_void as *mut *mut libc::c_void,
        );
        printf(
            b" M: Thread #%d joined.\n\0" as *const u8 as *const libc::c_char,
            i + 1 as libc::c_int,
        );
        i += 1;
    }
    printf(b"M: All threads have finished.\n\0" as *const u8 as *const libc::c_char);
    printf(b"SUCCESS!\n\0" as *const u8 as *const libc::c_char);
    return retVal;
}
pub unsafe extern "C" fn snoopyTestCli_action_stress_threadsexec_threadMain(
    mut args: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut tArgs: *mut tData_t = 0 as *mut tData_t;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argv: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pid: pid_t = 0;
    let mut tmp: __pid_t = 0;
    let mut status: *mut libc::c_int = 0 as *mut libc::c_int;
    tArgs = args as *mut tData_t;
    printf(
        b"  t%d : Hello from thread #%d.\n\0" as *const u8 as *const libc::c_char,
        (*tArgs).seqNr + 1 as libc::c_int,
        (*tArgs).seqNr + 1 as libc::c_int,
    );
    tmp = fork();
    pid = tmp;
    if pid > 0 as libc::c_int {
        printf(
            b"  t%dp: Hello from parent proc\n\0" as *const u8 as *const libc::c_char,
            (*tArgs).seqNr + 1 as libc::c_int,
        );
        status = 0 as *mut libc::c_int;
        waitpid(pid, status, 0 as libc::c_int);
        printf(
            b"  t%dp: Child proc has finished\n\0" as *const u8 as *const libc::c_char,
            (*tArgs).seqNr + 1 as libc::c_int,
        );
    } else if pid == 0 as libc::c_int {
        printf(
            b"  t%dc: Hello from child proc\n\0" as *const u8 as *const libc::c_char,
            (*tArgs).seqNr + 1 as libc::c_int,
        );
        cmd = *snoopyTestCli_action_stress_threadsexec_runCmdAndArgv
            .offset(0 as libc::c_int as isize);
        argv = snoopyTestCli_action_stress_threadsexec_runCmdAndArgv
            .offset(0 as libc::c_int as isize);
        printf(
            b"  t%dc: running cmd %s %s\n\0" as *const u8 as *const libc::c_char,
            (*tArgs).seqNr + 1 as libc::c_int,
            cmd,
            *argv.offset(0 as libc::c_int as isize),
        );
        execv(cmd as *const libc::c_char, argv as *const *mut libc::c_char);
    } else {
        printf(
            b"  t%d : Fork failed!\n\0" as *const u8 as *const libc::c_char,
            (*tArgs).seqNr + 1 as libc::c_int,
        );
    }
    free(tArgs as *mut libc::c_void);
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util_systemd_showHelp() {
    let mut helpContent: *mut libc::c_char = 0 as *mut libc::c_char;
    helpContent = b"Snoopy TEST SUITE CLI utility :: Action `unit` :: Unit `util` :: Subunit 'systemd'\n\nDescription:\n    Mocks src/action/systemd.c implementation code paths (mainly for the coverage of code parts/paths not covered by the test suite).\n\nUsage:\n    snoopy-test unit util systemd\n    snoopy-test unit util systemd --help\n\n\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    printf(b"%s\0" as *const u8 as *const libc::c_char, helpContent);
}
pub unsafe extern "C" fn snoopyTestCli_action_unit_util_systemd(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut arg1: *const libc::c_char = 0 as *const libc::c_char;
    let mut tmp: libc::c_int = 0;
    let mut cgroupEntry1: [libc::c_char; 27] = [0; 27];
    let mut expectedUnitName1: *const libc::c_char = 0 as *const libc::c_char;
    let mut cgroupEntry2: [libc::c_char; 42] = [0; 42];
    let mut expectedUnitName2: *const libc::c_char = 0 as *const libc::c_char;
    let mut cgroupEntry3: [libc::c_char; 59] = [0; 59];
    let mut expectedUnitName3: *const libc::c_char = 0 as *const libc::c_char;
    let mut cgroupEntry4: [libc::c_char; 63] = [0; 63];
    let mut expectedUnitName4: *const libc::c_char = 0 as *const libc::c_char;
    let mut cgroupEntry5: [libc::c_char; 17] = [0; 17];
    let mut expectedUnitName5: *const libc::c_char = 0 as *const libc::c_char;
    let mut cgroupEntry6: [libc::c_char; 13] = [0; 13];
    let mut expectedUnitName6: *const libc::c_char = 0 as *const libc::c_char;
    let mut cgroupEntry7: [libc::c_char; 93] = [0; 93];
    let mut expectedUnitName7: *const libc::c_char = 0 as *const libc::c_char;
    if argc > 0 as libc::c_int {
        arg1 = *argv.offset(0 as libc::c_int as isize) as *const libc::c_char;
    } else {
        arg1 = b"\0" as *const u8 as *const libc::c_char;
    }
    tmp = strcmp(arg1, b"--help\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int == tmp {
        snoopyTestCli_action_unit_util_systemd_showHelp();
        return 0 as libc::c_int;
    }
    cgroupEntry1[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry1[1 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry1[2 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    cgroupEntry1[3 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    cgroupEntry1[4 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry1[5 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry1[6 as libc::c_int as usize] = '=' as i32 as libc::c_char;
    cgroupEntry1[7 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry1[8 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    cgroupEntry1[9 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry1[10 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    cgroupEntry1[11 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry1[12 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry1[13 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry1[14 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry1[15 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry1[16 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry1[17 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    cgroupEntry1[18 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry1[19 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    cgroupEntry1[20 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    cgroupEntry1[21 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry1[22 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry1[23 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    cgroupEntry1[24 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    cgroupEntry1[25 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry1[26 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    expectedUnitName1 = b"init\0" as *const u8 as *const libc::c_char;
    mockCgroupEntryConversion(
        cgroupEntry1.as_mut_ptr() as *const libc::c_char,
        expectedUnitName1,
    );
    cgroupEntry2[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry2[1 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry2[2 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    cgroupEntry2[3 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    cgroupEntry2[4 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry2[5 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry2[6 as libc::c_int as usize] = '=' as i32 as libc::c_char;
    cgroupEntry2[7 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry2[8 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    cgroupEntry2[9 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry2[10 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    cgroupEntry2[11 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry2[12 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry2[13 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry2[14 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry2[15 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry2[16 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry2[17 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    cgroupEntry2[18 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry2[19 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    cgroupEntry2[20 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry2[21 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry2[22 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    cgroupEntry2[23 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry2[24 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    cgroupEntry2[25 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry2[26 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry2[27 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry2[28 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry2[29 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry2[30 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    cgroupEntry2[31 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    cgroupEntry2[32 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry2[33 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    cgroupEntry2[34 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry2[35 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry2[36 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    cgroupEntry2[37 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    cgroupEntry2[38 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry2[39 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry2[40 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry2[41 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    expectedUnitName2 = b"dbus\0" as *const u8 as *const libc::c_char;
    mockCgroupEntryConversion(
        cgroupEntry2.as_mut_ptr() as *const libc::c_char,
        expectedUnitName2,
    );
    cgroupEntry3[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry3[1 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry3[2 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    cgroupEntry3[3 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    cgroupEntry3[4 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry3[5 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry3[6 as libc::c_int as usize] = '=' as i32 as libc::c_char;
    cgroupEntry3[7 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry3[8 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    cgroupEntry3[9 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry3[10 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    cgroupEntry3[11 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry3[12 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry3[13 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry3[14 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry3[15 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry3[16 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    cgroupEntry3[17 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry3[18 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry3[19 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    cgroupEntry3[20 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    cgroupEntry3[21 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry3[22 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    cgroupEntry3[23 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry3[24 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry3[25 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry3[26 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry3[27 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    cgroupEntry3[28 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry3[29 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry3[30 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    cgroupEntry3[31 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    cgroupEntry3[32 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    cgroupEntry3[33 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    cgroupEntry3[34 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry3[35 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    cgroupEntry3[36 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry3[37 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry3[38 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry3[39 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry3[40 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry3[41 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry3[42 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry3[43 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry3[44 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry3[45 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    cgroupEntry3[46 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    cgroupEntry3[47 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    cgroupEntry3[48 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    cgroupEntry3[49 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry3[50 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    cgroupEntry3[51 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    cgroupEntry3[52 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    cgroupEntry3[53 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry3[54 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry3[55 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    cgroupEntry3[56 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    cgroupEntry3[57 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry3[58 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    expectedUnitName3 = b"root\0" as *const u8 as *const libc::c_char;
    mockCgroupEntryConversion(
        cgroupEntry3.as_mut_ptr() as *const libc::c_char,
        expectedUnitName3,
    );
    cgroupEntry4[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry4[1 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry4[2 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    cgroupEntry4[3 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    cgroupEntry4[4 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry4[5 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry4[6 as libc::c_int as usize] = '=' as i32 as libc::c_char;
    cgroupEntry4[7 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry4[8 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    cgroupEntry4[9 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry4[10 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    cgroupEntry4[11 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry4[12 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry4[13 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry4[14 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry4[15 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry4[16 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    cgroupEntry4[17 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry4[18 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry4[19 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    cgroupEntry4[20 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    cgroupEntry4[21 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry4[22 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    cgroupEntry4[23 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry4[24 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry4[25 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry4[26 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry4[27 as libc::c_int as usize] = 'u' as i32 as libc::c_char;
    cgroupEntry4[28 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry4[29 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry4[30 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    cgroupEntry4[31 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    cgroupEntry4[32 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    cgroupEntry4[33 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    cgroupEntry4[34 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry4[35 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    cgroupEntry4[36 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    cgroupEntry4[37 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    cgroupEntry4[38 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry4[39 as libc::c_int as usize] = 'l' as i32 as libc::c_char;
    cgroupEntry4[40 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry4[41 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry4[42 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry4[43 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry4[44 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry4[45 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry4[46 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry4[47 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry4[48 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry4[49 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    cgroupEntry4[50 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    cgroupEntry4[51 as libc::c_int as usize] = '-' as i32 as libc::c_char;
    cgroupEntry4[52 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    cgroupEntry4[53 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry4[54 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    cgroupEntry4[55 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    cgroupEntry4[56 as libc::c_int as usize] = '.' as i32 as libc::c_char;
    cgroupEntry4[57 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry4[58 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry4[59 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    cgroupEntry4[60 as libc::c_int as usize] = 'p' as i32 as libc::c_char;
    cgroupEntry4[61 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry4[62 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    expectedUnitName4 = b"user-57123\0" as *const u8 as *const libc::c_char;
    mockCgroupEntryConversion(
        cgroupEntry4.as_mut_ptr() as *const libc::c_char,
        expectedUnitName4,
    );
    cgroupEntry5[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry5[1 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry5[2 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    cgroupEntry5[3 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    cgroupEntry5[4 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry5[5 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry5[6 as libc::c_int as usize] = '=' as i32 as libc::c_char;
    cgroupEntry5[7 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry5[8 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    cgroupEntry5[9 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry5[10 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    cgroupEntry5[11 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry5[12 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry5[13 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry5[14 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry5[15 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry5[16 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    expectedUnitName5 = b"-\0" as *const u8 as *const libc::c_char;
    mockCgroupEntryConversion(
        cgroupEntry5.as_mut_ptr() as *const libc::c_char,
        expectedUnitName5,
    );
    cgroupEntry6[0 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    cgroupEntry6[1 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry6[2 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry6[3 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry6[4 as libc::c_int as usize] = 'w' as i32 as libc::c_char;
    cgroupEntry6[5 as libc::c_int as usize] = 'h' as i32 as libc::c_char;
    cgroupEntry6[6 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    cgroupEntry6[7 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    cgroupEntry6[8 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry6[9 as libc::c_int as usize] = 'v' as i32 as libc::c_char;
    cgroupEntry6[10 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry6[11 as libc::c_int as usize] = 'r' as i32 as libc::c_char;
    cgroupEntry6[12 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    expectedUnitName6 = 0 as *mut libc::c_void as *const libc::c_char;
    mockCgroupEntryConversion(
        cgroupEntry6.as_mut_ptr() as *const libc::c_char,
        expectedUnitName6,
    );
    cgroupEntry7[0 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry7[1 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry7[2 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    cgroupEntry7[3 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    cgroupEntry7[4 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry7[5 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry7[6 as libc::c_int as usize] = '=' as i32 as libc::c_char;
    cgroupEntry7[7 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry7[8 as libc::c_int as usize] = 'y' as i32 as libc::c_char;
    cgroupEntry7[9 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry7[10 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    cgroupEntry7[11 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry7[12 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    cgroupEntry7[13 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry7[14 as libc::c_int as usize] = ':' as i32 as libc::c_char;
    cgroupEntry7[15 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry7[16 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    cgroupEntry7[17 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry7[18 as libc::c_int as usize] = 't' as i32 as libc::c_char;
    cgroupEntry7[19 as libc::c_int as usize] = 'i' as i32 as libc::c_char;
    cgroupEntry7[20 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    cgroupEntry7[21 as libc::c_int as usize] = 'n' as i32 as libc::c_char;
    cgroupEntry7[22 as libc::c_int as usize] = 's' as i32 as libc::c_char;
    cgroupEntry7[23 as libc::c_int as usize] = '_' as i32 as libc::c_char;
    cgroupEntry7[24 as libc::c_int as usize] = 'j' as i32 as libc::c_char;
    cgroupEntry7[25 as libc::c_int as usize] = 'o' as i32 as libc::c_char;
    cgroupEntry7[26 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    cgroupEntry7[27 as libc::c_int as usize] = '/' as i32 as libc::c_char;
    cgroupEntry7[28 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    cgroupEntry7[29 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    cgroupEntry7[30 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry7[31 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    cgroupEntry7[32 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    cgroupEntry7[33 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    cgroupEntry7[34 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry7[35 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    cgroupEntry7[36 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry7[37 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    cgroupEntry7[38 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    cgroupEntry7[39 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    cgroupEntry7[40 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry7[41 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry7[42 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry7[43 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry7[44 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    cgroupEntry7[45 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    cgroupEntry7[46 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    cgroupEntry7[47 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    cgroupEntry7[48 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    cgroupEntry7[49 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    cgroupEntry7[50 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    cgroupEntry7[51 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    cgroupEntry7[52 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry7[53 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    cgroupEntry7[54 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry7[55 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry7[56 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    cgroupEntry7[57 as libc::c_int as usize] = '5' as i32 as libc::c_char;
    cgroupEntry7[58 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry7[59 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    cgroupEntry7[60 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry7[61 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    cgroupEntry7[62 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry7[63 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    cgroupEntry7[64 as libc::c_int as usize] = '8' as i32 as libc::c_char;
    cgroupEntry7[65 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry7[66 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    cgroupEntry7[67 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    cgroupEntry7[68 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    cgroupEntry7[69 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry7[70 as libc::c_int as usize] = '0' as i32 as libc::c_char;
    cgroupEntry7[71 as libc::c_int as usize] = '4' as i32 as libc::c_char;
    cgroupEntry7[72 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    cgroupEntry7[73 as libc::c_int as usize] = '9' as i32 as libc::c_char;
    cgroupEntry7[74 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    cgroupEntry7[75 as libc::c_int as usize] = '1' as i32 as libc::c_char;
    cgroupEntry7[76 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    cgroupEntry7[77 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    cgroupEntry7[78 as libc::c_int as usize] = '2' as i32 as libc::c_char;
    cgroupEntry7[79 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    cgroupEntry7[80 as libc::c_int as usize] = 'd' as i32 as libc::c_char;
    cgroupEntry7[81 as libc::c_int as usize] = '8' as i32 as libc::c_char;
    cgroupEntry7[82 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    cgroupEntry7[83 as libc::c_int as usize] = '3' as i32 as libc::c_char;
    cgroupEntry7[84 as libc::c_int as usize] = '7' as i32 as libc::c_char;
    cgroupEntry7[85 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry7[86 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    cgroupEntry7[87 as libc::c_int as usize] = '6' as i32 as libc::c_char;
    cgroupEntry7[88 as libc::c_int as usize] = 'f' as i32 as libc::c_char;
    cgroupEntry7[89 as libc::c_int as usize] = 'e' as i32 as libc::c_char;
    cgroupEntry7[90 as libc::c_int as usize] = 'a' as i32 as libc::c_char;
    cgroupEntry7[91 as libc::c_int as usize] = 'c' as i32 as libc::c_char;
    cgroupEntry7[92 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
    expectedUnitName7 = 0 as *mut libc::c_void as *const libc::c_char;
    mockCgroupEntryConversion(
        cgroupEntry7.as_mut_ptr() as *const libc::c_char,
        expectedUnitName7,
    );
    printSuccess(
        b"Mocking src/unit/systemd.c complete.\0" as *const u8 as *const libc::c_char,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn mockCgroupEntryConversion(
    cgroupEntry: *const libc::c_char,
    expectedUnitName: *const libc::c_char,
) {
    let mut unitName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp___0: libc::c_int = 0;
    tmp = snoopy_util_systemd_convertCgroupEntryToUnitName(cgroupEntry);
    unitName = tmp;
    if unitName.is_null() {
        if expectedUnitName.is_null() {
            return;
        }
    }
    if unitName.is_null() {
        if !expectedUnitName.is_null() {
            printDiagValue(
                b"cgroupEntry\0" as *const u8 as *const libc::c_char,
                cgroupEntry,
            );
            printDiagValue(
                b"Expected unit name\0" as *const u8 as *const libc::c_char,
                expectedUnitName,
            );
            fatalError(
                b"Unable to convert cgroup entry to Systemd unit name\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
    }
    if !unitName.is_null() {
        if expectedUnitName.is_null() {
            printDiagValue(
                b"cgroupEntry\0" as *const u8 as *const libc::c_char,
                cgroupEntry,
            );
            printDiagValue(
                b"Converted unit name\0" as *const u8 as *const libc::c_char,
                unitName as *const libc::c_char,
            );
            fatalError(
                b"Cgroup entry not converted to NULL\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
    }
    tmp___0 = strcmp(expectedUnitName, unitName as *const libc::c_char);
    if 0 as libc::c_int != tmp___0 {
        printDiagValue(
            b"cgroupEntry\0" as *const u8 as *const libc::c_char,
            cgroupEntry,
        );
        printDiagValue(
            b"Expected  unit name\0" as *const u8 as *const libc::c_char,
            expectedUnitName,
        );
        printDiagValue(
            b"Converted unit name\0" as *const u8 as *const libc::c_char,
            unitName as *const libc::c_char,
        );
        fatalError(
            b"Unexpected unit name conversion\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !unitName.is_null() {
        free(unitName as *mut libc::c_void);
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
