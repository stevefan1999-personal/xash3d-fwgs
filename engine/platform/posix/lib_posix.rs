#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    #[no_mangle]
    fn Con_Reportf(szFmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn _Mem_Free(data: *mut libc::c_void, filename: *const libc::c_char,
                 fileline: libc::c_int);
    #[no_mangle]
    fn dladdr(__address: *const libc::c_void, __info: *mut Dl_info)
     -> libc::c_int;
    #[no_mangle]
    fn dlerror() -> *mut libc::c_char;
    #[no_mangle]
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char)
     -> *mut libc::c_void;
    #[no_mangle]
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int)
     -> *mut libc::c_void;
    #[no_mangle]
    fn va(format: *const libc::c_char, _: ...) -> *mut libc::c_char;
    #[no_mangle]
    fn FS_FindLibrary(dllname: *const libc::c_char, directpath: qboolean)
     -> *mut dll_user_t;
    #[no_mangle]
    fn COM_ResetLibraryError();
    #[no_mangle]
    fn COM_PushLibraryError(error: *const libc::c_char);
    #[no_mangle]
    fn COM_GetPlatformNeutralName(in_name: *const libc::c_char)
     -> *const libc::c_char;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dl_info {
    pub dli_fname: *const libc::c_char,
    pub dli_fbase: *mut libc::c_void,
    pub dli_sname: *const libc::c_char,
    pub dli_saddr: *mut libc::c_void,
}
pub type uintptr_t = libc::c_ulong;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type dword = libc::c_uint;
pub type string = [libc::c_char; 256];
pub type word = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dll_user_s {
    pub hInstance: *mut libc::c_void,
    pub custom_loader: qboolean,
    pub encrypted: qboolean,
    pub dllName: [libc::c_char; 32],
    pub fullPath: string,
    pub shortPath: string,
    pub ordinals: *mut word,
    pub funcs: *mut dword,
    pub names: [*mut libc::c_char; 4096],
    pub num_ordinals: libc::c_int,
    pub funcBase: uintptr_t,
}
pub type dll_user_t = dll_user_s;
/*
lib_posix.c - dynamic library code for POSIX systems
Copyright (C) 2018 Flying With Gauss

This program is free software: you can redistribute it and/sor modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/
// wine-based dll loader
// XASH_NO_LIBDL
#[no_mangle]
pub unsafe extern "C" fn COM_CheckLibraryDirectDependency(mut name:
                                                              *const libc::c_char,
                                                          mut depname:
                                                              *const libc::c_char,
                                                          mut directpath:
                                                              qboolean)
 -> qboolean {
    // TODO: implement
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn COM_LoadLibrary(mut dllname: *const libc::c_char,
                                         mut build_ordinals_table:
                                             libc::c_int,
                                         mut directpath: qboolean)
 -> *mut libc::c_void {
    let mut hInst: *mut dll_user_t = 0 as *mut dll_user_t;
    let mut pHandle: *mut libc::c_void = 0 as *mut libc::c_void;
    COM_ResetLibraryError();
    // platforms where gameinfo mechanism is impossible
    // platforms where gameinfo mechanism is working goes here
	// and use FS_FindLibrary
    hInst = FS_FindLibrary(dllname, directpath);
    if hInst.is_null() {
        // HACKHACK: direct load dll
        // try to find by linker(LD_LIBRARY_PATH, DYLD_LIBRARY_PATH, LD_32_LIBRARY_PATH and so on...)
        if pHandle.is_null() {
            pHandle = dlopen(dllname, 0x1 as libc::c_int);
            if !pHandle.is_null() { return pHandle }
            COM_PushLibraryError(va(b"Failed to find library %s\x00" as
                                        *const u8 as *const libc::c_char,
                                    dllname));
            COM_PushLibraryError(dlerror());
            return 0 as *mut libc::c_void
        }
    }
    if (*hInst).custom_loader as u64 != 0 {
        COM_PushLibraryError(va(b"Custom library loader is not available. Extract library %s and fix gameinfo.txt!\x00"
                                    as *const u8 as *const libc::c_char,
                                (*hInst).fullPath.as_mut_ptr()));
        _Mem_Free(hInst as *mut libc::c_void,
                  b"../engine/platform/posix/lib_posix.c\x00" as *const u8 as
                      *const libc::c_char, 119 as libc::c_int);
        return 0 as *mut libc::c_void
    }
    (*hInst).hInstance =
        dlopen((*hInst).fullPath.as_mut_ptr(), 0x1 as libc::c_int);
    if (*hInst).hInstance.is_null() {
        COM_PushLibraryError(dlerror());
        _Mem_Free(hInst as *mut libc::c_void,
                  b"../engine/platform/posix/lib_posix.c\x00" as *const u8 as
                      *const libc::c_char, 146 as libc::c_int);
        return 0 as *mut libc::c_void
    }
    pHandle = (*hInst).hInstance;
    _Mem_Free(hInst as *mut libc::c_void,
              b"../engine/platform/posix/lib_posix.c\x00" as *const u8 as
                  *const libc::c_char, 153 as libc::c_int);
    return pHandle;
}
#[no_mangle]
pub unsafe extern "C" fn COM_FreeLibrary(mut hInstance: *mut libc::c_void) {
    dlclose(hInstance);
}
#[no_mangle]
pub unsafe extern "C" fn COM_GetProcAddress(mut hInstance: *mut libc::c_void,
                                            mut name: *const libc::c_char)
 -> *mut libc::c_void {
    return dlsym(hInstance, name);
}
#[no_mangle]
pub unsafe extern "C" fn COM_FunctionFromName(mut hInstance:
                                                  *mut libc::c_void,
                                              mut pName: *const libc::c_char)
 -> *mut libc::c_void {
    let mut function: *mut libc::c_void = 0 as *mut libc::c_void;
    function = COM_GetProcAddress(hInstance, pName);
    if function.is_null() {
        Con_Reportf(b"^1Error:^7 FunctionFromName: Can\'t get symbol %s: %s\n\x00"
                        as *const u8 as *const libc::c_char, pName,
                    dlerror());
    }
    return function;
}
#[no_mangle]
pub unsafe extern "C" fn COM_NameForFunction(mut hInstance: *mut libc::c_void,
                                             mut function: *mut libc::c_void)
 -> *const libc::c_char {
    // NOTE: dladdr() is a glibc extension
    let mut info: Dl_info =
        {
            let mut init =
                Dl_info{dli_fname: 0 as *const libc::c_char,
                        dli_fbase: 0 as *mut libc::c_void,
                        dli_sname: 0 as *const libc::c_char,
                        dli_saddr: 0 as *mut libc::c_void,};
            init
        };
    dladdr(function, &mut info);
    if !info.dli_sname.is_null() {
        return COM_GetPlatformNeutralName(info.dli_sname)
    }
    return 0 as *const libc::c_char;
}
// _WIN32
