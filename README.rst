hugepage
===================


.. contents::


关闭透明巨页
------------------

.. code:: bash
    
    echo "never" > /sys/kernel/mm/transparent_hugepage/enabled

    cat /sys/kernel/mm/transparent_hugepage/enabled
    # 如果返回 如下输出则说明透明巨页的特性被关闭了
    # always madvise [never]


也可以通过修改 `/boot/grub2/grub.cfg` 里面的内核启动参数来禁止（加上内核启动参数 `transparent_hugepage=never`），这里不再赘述。


相关阅读: `Disable Transparent Hugepages <https://blog.nelhage.com/post/transparent-hugepages/>`_


动态调整巨页数量
-----------------

.. code:: bash
    
    # 预留巨页数量
    echo 70 > /proc/sys/vm/nr_hugepages
    
    # 查看巨页相关的内存信息
    cat /proc/meminfo | grep Huge


需要注意的是，如果系统运行很久之后，可能会导致无法保留足够多的巨页数量的情况，所以，最好在开机的时候执行该操作。


修改内核的启动参数
----------------------

打开 `/boot/grub/grub.cfg` 文件，找到 `menuentry` 区块里面的 `linux` 行，并在末尾增加 `default_hugepagesz=1G hugepagesz=1G` 参数。


.. code:: text

    menuentry 'Debian ....' {
        linux   /boot/vmlinuz-4.15.0-135-generic root=UUID=01a8 ro  default_hugepagesz=1G hugepagesz=1G transparent_hugepage=never
    }


相关的 `内核启动参数 <https://www.kernel.org/doc/Documentation/admin-guide/kernel-parameters.txt>`_ :

.. code:: text

    hugepages=      [HW,X86-32,IA-64] HugeTLB pages to allocate at boot.
    hugepagesz=     [HW,IA-64,PPC,X86-64] The size of the HugeTLB pages.
                    On x86-64 and powerpc, this option can be specified
                    multiple times interleaved with hugepages= to reserve
                    huge pages of different sizes. Valid pages sizes on
                    x86-64 are 2M (when the CPU supports "pse") and 1G
                    (when the CPU supports the "pdpe1gb" cpuinfo flag).

    default_hugepagesz=
                    [same as hugepagesz=] The size of the default
                    HugeTLB page size. This is the size represented by
                    the legacy /proc/ hugepages APIs, used for SHM, and
                    default size when mounting hugetlbfs filesystems.
                    Defaults to the default architecture's huge page size
                    if not specified.
    transparent_hugepage=
                    [KNL]
                    Format: [always|madvise|never]
                    Can be used to control the default behavior of the system
                    with respect to transparent hugepages.
                    See Documentation/vm/transhuge.txt for more details.
    nohugeiomap     [KNL,x86] Disable kernel huge I/O mappings.

