use std::io;
use std::fs;
use std::sync::Once;

// https://www.kernel.org/doc/Documentation/vm/hugetlbpage.txt
// 
// statistics
// cat /proc/meminfo | grep Huge

#[derive(Debug, Clone, Copy)]
pub struct HugePageInfo {
    /// in bytes.
    pub anon_pages: usize,
    /// in bytes
    pub shmem_pages: usize,
    /// HPAGE_SIZE, in bytes
    pub size: usize,
    
    pub total: usize,
    // 未实际使用的内存页数
    pub free: usize,
    // NOTE: 已经分配，但未实际使用的内存页数
    //       已使用的内存页数可以通过 total - free 计算。
    pub rsvd: usize,
    // NOTE: 允许 实际分配的巨页数量超过 `nr_hugepages` 设定。
    //       内核会在 `nr_hugepages` 页数耗尽后，使用内核的内存来分配，
    //       而内核的内存分配，并不能分配超如 1G 大小的大内存，所以这个只会在
    //       巨页大小为 兆时（如 2MB），才有效。
    pub surp: usize,
}


pub fn kernel_default_hugepage_info() -> Result<HugePageInfo, io::Error> {
    let mut size        = 0usize;
    let mut anon_pages  = 0usize;
    let mut shmem_pages = 0usize;
    
    let mut total = 0usize;
    let mut free  = 0usize;
    let mut rsvd  = 0usize;
    let mut surp  = 0usize;

    fn f1(line: &str, key: &str) -> Option<usize> {
        line.strip_prefix(key)?.trim().parse::<usize>().ok()
    }

    fn f2(line: &str, key: &str) -> Option<usize> {
        line.strip_prefix(key)?.strip_suffix("kB")?.trim().parse::<usize>().ok()
    }

    let meminfo = fs::read_to_string("/proc/meminfo")?;
    for line in meminfo.lines() {
        if let Some(num) = f2(line, "Hugepagesize:") {
            // In bytes.
            size = num * 1024;
        }
        if let Some(num) = f2(line, "AnonHugePages:") {
            // In bytes.
            anon_pages = num * 1024;
        }
        if let Some(num) = f2(line, "ShmemHugePages:") {
            // In bytes.
            shmem_pages = num * 1024;
        }

        if let Some(num) = f1(line, "HugePages_Total:") {
            total = num;
        }
        if let Some(num) = f1(line, "HugePages_Free:") {
            free = num;
        }
        if let Some(num) = f1(line, "HugePages_Rsvd:") {
            rsvd = num;
        }
        if let Some(num) = f1(line, "HugePages_Surp:") {
            surp = num;
        }
    }

    if size > 0 {
        Ok(HugePageInfo { anon_pages, shmem_pages, total, free, rsvd, surp, size })
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "meminfo read error."))
    }
}

pub fn kernel_default_hugepage_size() -> usize {
    // NOTE: 单位 Bytes.
    static mut HUGE_PAGE_SIZE: usize = 0;
    static QUERY: Once = Once::new();
    
    unsafe {
        QUERY.call_once(|| {
            match kernel_default_hugepage_info() {
                Ok(meminfo) => {
                    HUGE_PAGE_SIZE = meminfo.size;
                },
                Err(_) => { },
            }
        });
        
        HUGE_PAGE_SIZE
    }
}
