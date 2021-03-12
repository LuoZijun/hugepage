#![feature(test, allocator_api, nonnull_slice_from_raw_parts)]

#[cfg(test)]
extern crate test;
extern crate libc;


mod meminfo;
pub use meminfo::*;

use core::ptr::NonNull;
use core::alloc::Layout;
use core::alloc::Allocator;
use core::alloc::AllocError;


pub fn is_valid_size(size: usize) -> bool {
    let hugepage_size = kernel_default_hugepage_size();

    if size == 0 {
        return false;
    }

    if hugepage_size == 0 {
        // NOTE: 避免 mod 0，同时这意味着 获取 HPAGE_SIZE 失败了。
        return false;
    }

    size % hugepage_size == 0
}

// https://github.com/torvalds/linux/blob/master/tools/testing/selftests/vm/map_hugetlb.c
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    let size = layout.size();

    if !is_valid_size(size) {
        return core::ptr::null_mut();
    }
    
    let addr  = core::ptr::null_mut();
    let prot  = libc::PROT_READ | libc::PROT_WRITE;
    let flags = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_HUGETLB;

    let raw_ptr: *mut libc::c_void = libc::mmap(addr, size, prot, flags, -1, 0);

    if raw_ptr == libc::MAP_FAILED {
        core::ptr::null_mut()
    } else {
        raw_ptr as *mut u8
    }
}

pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    let size = layout.size();
    
    if !is_valid_size(size) {
        return ();
    }
    
    let ptr  = ptr as *mut libc::c_void;
    
    // NOTE: RET == 0 表示内存释放成功，但是我们这里并不处理释放失败的情况。
    let ret = libc::munmap(ptr, size);
    assert_eq!(ret, 0);
}


#[derive(Debug, Default, Copy, Clone)]
pub struct HugePage;


unsafe impl Allocator for HugePage {
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let size = layout.size();

        let raw_ptr = unsafe { alloc(layout) };
        let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;

        Ok(NonNull::slice_from_raw_parts(ptr, size))
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        if layout.size() != 0 {
            // SAFETY: `layout` is non-zero in size,
            // other conditions must be upheld by the caller
            dealloc(ptr.as_ptr(), layout)
        }
    }
}
