#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(const_raw_ptr_to_usize_cast, const_transmute, register_tool)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn abs(_: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
     -> *mut libc::c_void;
    #[no_mangle]
    fn Q_strncpy(dst: *mut libc::c_char, src: *const libc::c_char,
                 siz: size_t) -> size_t;
    #[no_mangle]
    fn Q_pretifymem(value: libc::c_float, digitsafterdecimal: libc::c_int)
     -> *mut libc::c_char;
    #[no_mangle]
    fn Sys_Error(error: *const libc::c_char, _: ...);
    #[no_mangle]
    fn Con_Printf(szFmt: *const libc::c_char, _: ...);
}
pub type __uint32_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type uint = libc::c_uint;
pub type uint32_t = __uint32_t;
pub type byte = libc::c_uchar;
pub type poolhandle_t = uint32_t;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type mempool_t = mempool_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mempool_s {
    pub sentinel1: uint,
    pub chain: *mut memheader_s,
    pub totalsize: size_t,
    pub realsize: size_t,
    pub lastchecksize: size_t,
    pub next: *mut mempool_s,
    pub filename: *const libc::c_char,
    pub fileline: libc::c_int,
    pub idx: poolhandle_t,
    pub name: [libc::c_char; 64],
    pub sentinel2: uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct memheader_s {
    pub next: *mut memheader_s,
    pub prev: *mut memheader_s,
    pub pool: *mut mempool_s,
    pub size: size_t,
    pub filename: *const libc::c_char,
    pub fileline: uint,
    pub sentinel1: uint,
}
pub type memheader_t = memheader_s;
static mut poolchain: *mut mempool_t =
    0 as *const mempool_t as *mut mempool_t;
// should always be MEMHEADER_SENTINEL1
// chain of individual memory allocations
// total memory allocated in this pool (inside memheaders)
// total memory allocated in this pool (actual malloc total)
// updated each time the pool is displayed by memlist
// linked into global mempool list
// file name and line where Mem_AllocPool was called
// name of the pool
// should always be MEMHEADER_SENTINEL1
// next and previous memheaders in chain belonging to pool
// pool this memheader belongs to
// size of the memory after the header (excluding header and sentinel2)
// file name and line where Mem_Alloc was called
// should always be MEMHEADER_SENTINEL1
// immediately followed by data, which is followed by a MEMHEADER_SENTINEL2 byte
// critical stuff
// a1ba: due to mempool being passed with the model through reused 32-bit field
// which makes engine incompatible with 64-bit pointers I changed mempool type
// from pointer to 32-bit handle, thankfully mempool structure is private
// But! Mempools are handled through linked list so we can't index them safely
static mut lastidx: uint = 0 as libc::c_int as uint;
unsafe extern "C" fn Mem_FindPool(mut poolptr: poolhandle_t)
 -> *mut mempool_t {
    let mut pool: *mut mempool_t = 0 as *mut mempool_t;
    pool = poolchain;
    while !pool.is_null() {
        if (*pool).idx == poolptr { return pool }
        pool = (*pool).next
    }
    Sys_Error(b"%s: not allocated or double freed pool %d\x00" as *const u8 as
                  *const libc::c_char,
              (*::std::mem::transmute::<&[u8; 13],
                                        &[libc::c_char; 13]>(b"Mem_FindPool\x00")).as_ptr(),
              poolptr);
    return 0 as *mut mempool_t;
}
#[no_mangle]
pub unsafe extern "C" fn _Mem_Alloc(mut poolptr: poolhandle_t,
                                    mut size: size_t, mut clear: qboolean,
                                    mut filename: *const libc::c_char,
                                    mut fileline: libc::c_int)
 -> *mut libc::c_void {
    let mut mem: *mut memheader_t = 0 as *mut memheader_t;
    let mut pool: *mut mempool_t = 0 as *mut mempool_t;
    if size <= 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void
    }
    if poolptr == 0 {
        Sys_Error(b"Mem_Alloc: pool == NULL (alloc at %s:%i)\n\x00" as
                      *const u8 as *const libc::c_char, filename, fileline);
    }
    pool = Mem_FindPool(poolptr);
    (*pool).totalsize =
        ((*pool).totalsize as libc::c_ulong).wrapping_add(size) as size_t as
            size_t;
    // big allocations are not clumped
    (*pool).realsize =
        ((*pool).realsize as
             libc::c_ulong).wrapping_add((::std::mem::size_of::<memheader_t>()
                                              as
                                              libc::c_ulong).wrapping_add(size).wrapping_add(::std::mem::size_of::<libc::c_int>()
                                                                                                 as
                                                                                                 libc::c_ulong))
            as size_t as size_t;
    mem =
        malloc((::std::mem::size_of::<memheader_t>() as
                    libc::c_ulong).wrapping_add(size).wrapping_add(::std::mem::size_of::<libc::c_int>()
                                                                       as
                                                                       libc::c_ulong))
            as *mut memheader_t;
    if mem.is_null() {
        Sys_Error(b"Mem_Alloc: out of memory (alloc at %s:%i)\n\x00" as
                      *const u8 as *const libc::c_char, filename, fileline);
    }
    (*mem).filename = filename;
    (*mem).fileline = fileline as uint;
    (*mem).size = size;
    (*mem).pool = pool;
    (*mem).sentinel1 = 0xdeadf00d as libc::c_uint;
    // we have to use only a single byte for this sentinel, because it may not be aligned
	// and some platforms can't use unaligned accesses
    *(mem as
          *mut byte).offset(::std::mem::size_of::<memheader_t>() as
                                libc::c_ulong as
                                isize).offset((*mem).size as isize) =
        0xdf as libc::c_int as byte;
    // append to head of list
    (*mem).next = (*pool).chain;
    (*mem).prev = 0 as *mut memheader_s;
    (*pool).chain = mem;
    if !(*mem).next.is_null() { (*(*mem).next).prev = mem }
    if clear as u64 != 0 {
        memset((mem as
                    *mut byte).offset(::std::mem::size_of::<memheader_t>() as
                                          libc::c_ulong as isize) as
                   *mut libc::c_void, 0 as libc::c_int, (*mem).size);
    }
    return (mem as
                *mut byte).offset(::std::mem::size_of::<memheader_t>() as
                                      libc::c_ulong as isize) as
               *mut libc::c_void;
}
unsafe extern "C" fn Mem_CheckFilename(mut filename: *const libc::c_char)
 -> *const libc::c_char {
    static mut dummy: *const libc::c_char =
        b"<corrupted>\x00\x00" as *const u8 as *const libc::c_char;
    let mut out: *const libc::c_char = filename;
    let mut i: libc::c_int = 0;
    if if out.is_null() || *out == 0 {
           0 as libc::c_int
       } else { 1 as libc::c_int } == 0 {
        return dummy
    }
    i = 0 as libc::c_int;
    while i < 260 as libc::c_int {
        if *out as libc::c_int == '\u{0}' as i32 { return filename }
        i += 1;
        out = out.offset(1)
        // valid name
    } // make sure what we don't crash var_args
    return dummy; // make sure what we don't crash var_args
}
unsafe extern "C" fn Mem_FreeBlock(mut mem: *mut memheader_t,
                                   mut filename: *const libc::c_char,
                                   mut fileline: libc::c_int) {
    let mut pool: *mut mempool_t = 0 as *mut mempool_t;
    if (*mem).sentinel1 != 0xdeadf00d as libc::c_uint {
        (*mem).filename = Mem_CheckFilename((*mem).filename);
        Sys_Error(b"Mem_Free: trashed header sentinel 1 (alloc at %s:%i, free at %s:%i)\n\x00"
                      as *const u8 as *const libc::c_char, (*mem).filename,
                  (*mem).fileline, filename, fileline);
    }
    if *(mem as
             *mut byte).offset(::std::mem::size_of::<memheader_t>() as
                                   libc::c_ulong as
                                   isize).offset((*mem).size as isize) as
           libc::c_int != 0xdf as libc::c_int {
        (*mem).filename = Mem_CheckFilename((*mem).filename);
        Sys_Error(b"Mem_Free: trashed header sentinel 2 (alloc at %s:%i, free at %s:%i)\n\x00"
                      as *const u8 as *const libc::c_char, (*mem).filename,
                  (*mem).fileline, filename, fileline);
    }
    pool = (*mem).pool;
    // unlink memheader from doubly linked list
    if (if !(*mem).prev.is_null() {
            ((*(*mem).prev).next != mem) as libc::c_int
        } else { ((*pool).chain != mem) as libc::c_int }) != 0 ||
           !(*mem).next.is_null() && (*(*mem).next).prev != mem {
        Sys_Error(b"Mem_Free: not allocated or double freed (free at %s:%i)\n\x00"
                      as *const u8 as *const libc::c_char, filename,
                  fileline);
    }
    if !(*mem).prev.is_null() {
        (*(*mem).prev).next = (*mem).next
    } else { (*pool).chain = (*mem).next }
    if !(*mem).next.is_null() { (*(*mem).next).prev = (*mem).prev }
    // memheader has been unlinked, do the actual free now
    (*pool).totalsize =
        ((*pool).totalsize as libc::c_ulong).wrapping_sub((*mem).size) as
            size_t as size_t; // no need to reallocate
    (*pool).realsize =
        ((*pool).realsize as
             libc::c_ulong).wrapping_sub((::std::mem::size_of::<memheader_t>()
                                              as
                                              libc::c_ulong).wrapping_add((*mem).size).wrapping_add(::std::mem::size_of::<libc::c_int>()
                                                                                                        as
                                                                                                        libc::c_ulong))
            as size_t as size_t;
    free(mem as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _Mem_Free(mut data: *mut libc::c_void,
                                   mut filename: *const libc::c_char,
                                   mut fileline: libc::c_int) {
    if data.is_null() {
        Sys_Error(b"Mem_Free: data == NULL (called at %s:%i)\n\x00" as
                      *const u8 as *const libc::c_char, filename, fileline);
    }
    Mem_FreeBlock((data as
                       *mut byte).offset(-(::std::mem::size_of::<memheader_t>()
                                               as libc::c_ulong as isize)) as
                      *mut memheader_t, filename, fileline);
}
#[no_mangle]
pub unsafe extern "C" fn _Mem_Realloc(mut poolptr: poolhandle_t,
                                      mut memptr: *mut libc::c_void,
                                      mut size: size_t, mut clear: qboolean,
                                      mut filename: *const libc::c_char,
                                      mut fileline: libc::c_int)
 -> *mut libc::c_void {
    let mut memhdr: *mut memheader_t = 0 as *mut memheader_t;
    let mut nb: *mut libc::c_char = 0 as *mut libc::c_char;
    if size <= 0 as libc::c_int as libc::c_ulong { return memptr }
    if !memptr.is_null() {
        memhdr =
            (memptr as
                 *mut byte).offset(-(::std::mem::size_of::<memheader_t>() as
                                         libc::c_ulong as isize)) as
                *mut memheader_t;
        if size == (*memhdr).size { return memptr }
    }
    nb =
        _Mem_Alloc(poolptr, size, clear, filename, fileline) as
            *mut libc::c_char;
    if !memptr.is_null() {
        // first allocate?
        let mut newsize: size_t =
            if (*memhdr).size < size {
                (*memhdr).size
            } else { size }; // upper data can be trucnated!
        memcpy(nb as *mut libc::c_void, memptr, newsize);
        _Mem_Free(memptr, filename, fileline);
    }
    return nb as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn _Mem_AllocPool(mut name: *const libc::c_char,
                                        mut filename: *const libc::c_char,
                                        mut fileline: libc::c_int)
 -> poolhandle_t {
    let mut pool: *mut mempool_t = 0 as *mut mempool_t;
    pool =
        malloc(::std::mem::size_of::<mempool_t>() as libc::c_ulong) as
            *mut mempool_t;
    if pool.is_null() {
        Sys_Error(b"Mem_AllocPool: out of memory (allocpool at %s:%i)\n\x00"
                      as *const u8 as *const libc::c_char, filename,
                  fileline);
        return 0 as libc::c_int as poolhandle_t
    }
    memset(pool as *mut libc::c_void, 0 as libc::c_int,
           ::std::mem::size_of::<mempool_t>() as libc::c_ulong);
    // fill header
    (*pool).sentinel1 = 0xdeadf00d as libc::c_uint;
    (*pool).sentinel2 = 0xdeadf00d as libc::c_uint;
    (*pool).filename = filename;
    (*pool).fileline = fileline;
    (*pool).chain = 0 as *mut memheader_s;
    (*pool).totalsize = 0 as libc::c_int as size_t;
    (*pool).realsize = ::std::mem::size_of::<mempool_t>() as libc::c_ulong;
    Q_strncpy((*pool).name.as_mut_ptr(), name,
              ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong);
    (*pool).next = poolchain;
    lastidx = lastidx.wrapping_add(1);
    (*pool).idx = lastidx;
    poolchain = pool;
    return (*pool).idx;
}
#[no_mangle]
pub unsafe extern "C" fn _Mem_FreePool(mut poolptr: *mut poolhandle_t,
                                       mut filename: *const libc::c_char,
                                       mut fileline: libc::c_int) {
    let mut pool: *mut mempool_t = 0 as *mut mempool_t;
    let mut chainaddress: *mut *mut mempool_t = 0 as *mut *mut mempool_t;
    if *poolptr != 0 && { pool = Mem_FindPool(*poolptr); !pool.is_null() } {
        // unlink pool from chain
        chainaddress = &mut poolchain;
        while !(*chainaddress).is_null() && *chainaddress != pool {
            chainaddress = &mut (**chainaddress).next
        }
        if *chainaddress != pool {
            Sys_Error(b"Mem_FreePool: pool already free (freepool at %s:%i)\n\x00"
                          as *const u8 as *const libc::c_char, filename,
                      fileline);
        }
        if (*pool).sentinel1 != 0xdeadf00d as libc::c_uint {
            Sys_Error(b"Mem_FreePool: trashed pool sentinel 1 (allocpool at %s:%i, freepool at %s:%i)\n\x00"
                          as *const u8 as *const libc::c_char,
                      (*pool).filename, (*pool).fileline, filename, fileline);
        }
        if (*pool).sentinel2 != 0xdeadf00d as libc::c_uint {
            Sys_Error(b"Mem_FreePool: trashed pool sentinel 2 (allocpool at %s:%i, freepool at %s:%i)\n\x00"
                          as *const u8 as *const libc::c_char,
                      (*pool).filename, (*pool).fileline, filename, fileline);
        }
        *chainaddress = (*pool).next;
        // free memory owned by the pool
        while !(*pool).chain.is_null() {
            Mem_FreeBlock((*pool).chain, filename, fileline);
        }
        // free the pool itself
        memset(pool as *mut libc::c_void, 0xbf as libc::c_int,
               ::std::mem::size_of::<mempool_t>() as libc::c_ulong);
        free(pool as *mut libc::c_void);
        *poolptr = 0 as libc::c_int as poolhandle_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn _Mem_EmptyPool(mut poolptr: poolhandle_t,
                                        mut filename: *const libc::c_char,
                                        mut fileline: libc::c_int) {
    let mut pool: *mut mempool_t = Mem_FindPool(poolptr);
    if poolptr == 0 {
        Sys_Error(b"Mem_EmptyPool: pool == NULL (emptypool at %s:%i)\n\x00" as
                      *const u8 as *const libc::c_char, filename, fileline);
    }
    if (*pool).sentinel1 != 0xdeadf00d as libc::c_uint {
        Sys_Error(b"Mem_EmptyPool: trashed pool sentinel 1 (allocpool at %s:%i, emptypool at %s:%i)\n\x00"
                      as *const u8 as *const libc::c_char, (*pool).filename,
                  (*pool).fileline, filename, fileline);
    }
    if (*pool).sentinel2 != 0xdeadf00d as libc::c_uint {
        Sys_Error(b"Mem_EmptyPool: trashed pool sentinel 2 (allocpool at %s:%i, emptypool at %s:%i)\n\x00"
                      as *const u8 as *const libc::c_char, (*pool).filename,
                  (*pool).fileline, filename, fileline);
    }
    // free memory owned by the pool
    while !(*pool).chain.is_null() {
        Mem_FreeBlock((*pool).chain, filename, fileline);
    };
}
unsafe extern "C" fn Mem_CheckAlloc(mut pool: *mut mempool_t,
                                    mut data: *mut libc::c_void) -> qboolean {
    let mut header: *mut memheader_t = 0 as *mut memheader_t;
    let mut target: *mut memheader_t = 0 as *mut memheader_t;
    if !pool.is_null() {
        // search only one pool
        target =
            (data as
                 *mut byte).offset(-(::std::mem::size_of::<memheader_t>() as
                                         libc::c_ulong as isize)) as
                *mut memheader_t;
        header = (*pool).chain;
        while !header.is_null() {
            if header == target { return true_0 }
            header = (*header).next
        }
    } else {
        // search all pools
        pool = poolchain;
        while !pool.is_null() {
            if Mem_CheckAlloc(pool, data) as u64 != 0 { return true_0 }
            pool = (*pool).next
        }
    }
    return false_0;
}
/*
========================
Check pointer for memory
========================
*/
#[no_mangle]
pub unsafe extern "C" fn Mem_IsAllocatedExt(mut poolptr: poolhandle_t,
                                            mut data: *mut libc::c_void)
 -> qboolean {
    let mut pool: *mut mempool_t =
        0 as *mut mempool_t; // make sure what we don't crash var_args
    if poolptr != 0 {
        pool = Mem_FindPool(poolptr)
    } // make sure what we don't crash var_args
    return Mem_CheckAlloc(pool, data);
}
unsafe extern "C" fn Mem_CheckHeaderSentinels(mut data: *mut libc::c_void,
                                              mut filename:
                                                  *const libc::c_char,
                                              mut fileline: libc::c_int) {
    let mut mem: *mut memheader_t = 0 as *mut memheader_t;
    if data.is_null() {
        Sys_Error(b"Mem_CheckSentinels: data == NULL (sentinel check at %s:%i)\n\x00"
                      as *const u8 as *const libc::c_char, filename,
                  fileline);
    }
    mem =
        (data as
             *mut byte).offset(-(::std::mem::size_of::<memheader_t>() as
                                     libc::c_ulong as isize)) as
            *mut memheader_t;
    if (*mem).sentinel1 != 0xdeadf00d as libc::c_uint {
        (*mem).filename = Mem_CheckFilename((*mem).filename);
        Sys_Error(b"Mem_CheckSentinels: trashed header sentinel 1 (block allocated at %s:%i, sentinel check at %s:%i)\n\x00"
                      as *const u8 as *const libc::c_char, (*mem).filename,
                  (*mem).fileline, filename, fileline);
    }
    if *(mem as
             *mut byte).offset(::std::mem::size_of::<memheader_t>() as
                                   libc::c_ulong as
                                   isize).offset((*mem).size as isize) as
           libc::c_int != 0xdf as libc::c_int {
        (*mem).filename = Mem_CheckFilename((*mem).filename);
        Sys_Error(b"Mem_CheckSentinels: trashed header sentinel 2 (block allocated at %s:%i, sentinel check at %s:%i)\n\x00"
                      as *const u8 as *const libc::c_char, (*mem).filename,
                  (*mem).fileline, filename, fileline);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _Mem_Check(mut filename: *const libc::c_char,
                                    mut fileline: libc::c_int) {
    let mut mem: *mut memheader_t = 0 as *mut memheader_t;
    let mut pool: *mut mempool_t = 0 as *mut mempool_t;
    pool = poolchain;
    while !pool.is_null() {
        if (*pool).sentinel1 != 0xdeadf00d as libc::c_uint {
            Sys_Error(b"Mem_CheckSentinelsGlobal: trashed pool sentinel 1 (allocpool at %s:%i, sentinel check at %s:%i)\n\x00"
                          as *const u8 as *const libc::c_char,
                      (*pool).filename, (*pool).fileline, filename, fileline);
        }
        if (*pool).sentinel2 != 0xdeadf00d as libc::c_uint {
            Sys_Error(b"Mem_CheckSentinelsGlobal: trashed pool sentinel 2 (allocpool at %s:%i, sentinel check at %s:%i)\n\x00"
                          as *const u8 as *const libc::c_char,
                      (*pool).filename, (*pool).fileline, filename, fileline);
        }
        pool = (*pool).next
    }
    pool = poolchain;
    while !pool.is_null() {
        mem = (*pool).chain;
        while !mem.is_null() {
            Mem_CheckHeaderSentinels((mem as
                                          *mut byte).offset(::std::mem::size_of::<memheader_t>()
                                                                as
                                                                libc::c_ulong
                                                                as isize) as
                                         *mut libc::c_void, filename,
                                     fileline);
            mem = (*mem).next
        }
        pool = (*pool).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn Mem_PrintStats() {
    let mut count: size_t = 0 as libc::c_int as size_t;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut realsize: size_t = 0 as libc::c_int as size_t;
    let mut pool: *mut mempool_t = 0 as *mut mempool_t;
    _Mem_Check(b"../engine/common/zone.c\x00" as *const u8 as
                   *const libc::c_char, 342 as libc::c_int);
    pool = poolchain;
    while !pool.is_null() {
        count = count.wrapping_add(1);
        size =
            (size as libc::c_ulong).wrapping_add((*pool).totalsize) as size_t
                as size_t;
        realsize =
            (realsize as libc::c_ulong).wrapping_add((*pool).realsize) as
                size_t as size_t;
        pool = (*pool).next
    }
    Con_Printf(b"^3%lu^7 memory pools, totalling: ^1%s\n\x00" as *const u8 as
                   *const libc::c_char, count,
               Q_pretifymem(size as libc::c_float, 2 as libc::c_int));
    Con_Printf(b"total allocated size: ^1%s\n\x00" as *const u8 as
                   *const libc::c_char,
               Q_pretifymem(realsize as libc::c_float, 2 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn Mem_PrintList(mut minallocationsize: size_t) {
    let mut pool: *mut mempool_t = 0 as *mut mempool_t;
    let mut mem: *mut memheader_t = 0 as *mut memheader_t;
    _Mem_Check(b"../engine/common/zone.c\x00" as *const u8 as
                   *const libc::c_char, 359 as libc::c_int);
    Con_Printf(b"memory pool list:\n  ^3size                          name\n\x00"
                   as *const u8 as *const libc::c_char);
    pool = poolchain;
    while !pool.is_null() {
        let mut changed_size: libc::c_long =
            (*pool).totalsize as libc::c_long -
                (*pool).lastchecksize as libc::c_long;
        // poolnames can contain color symbols, make sure what color is reset
        if changed_size != 0 as libc::c_int as libc::c_long {
            let mut sign: libc::c_char =
                if changed_size < 0 as libc::c_int as libc::c_long {
                    '-' as i32
                } else { '+' as i32 } as libc::c_char;
            Con_Printf(b"%10s (%10s actual) %s (^7%c%s change)\n\x00" as
                           *const u8 as *const libc::c_char,
                       Q_pretifymem((*pool).totalsize as libc::c_float,
                                    2 as libc::c_int),
                       Q_pretifymem((*pool).realsize as libc::c_float,
                                    2 as libc::c_int),
                       (*pool).name.as_mut_ptr(), sign as libc::c_int,
                       Q_pretifymem(abs(changed_size as libc::c_int) as
                                        libc::c_float, 2 as libc::c_int));
        } else {
            Con_Printf(b"%5s (%5s actual) %s\n\x00" as *const u8 as
                           *const libc::c_char,
                       Q_pretifymem((*pool).totalsize as libc::c_float,
                                    2 as libc::c_int),
                       Q_pretifymem((*pool).realsize as libc::c_float,
                                    2 as libc::c_int),
                       (*pool).name.as_mut_ptr());
        }
        (*pool).lastchecksize = (*pool).totalsize;
        mem = (*pool).chain;
        while !mem.is_null() {
            if (*mem).size >= minallocationsize {
                Con_Printf(b"%10s allocated at %s:%i\n\x00" as *const u8 as
                               *const libc::c_char,
                           Q_pretifymem((*mem).size as libc::c_float,
                                        2 as libc::c_int), (*mem).filename,
                           (*mem).fileline);
            }
            mem = (*mem).next
        }
        pool = (*pool).next
    };
}
/*
========================
Memory_Init
========================
*/
#[no_mangle]
pub unsafe extern "C" fn Memory_Init() {
    poolchain = 0 as *mut mempool_t;
    // init mem chain
}
