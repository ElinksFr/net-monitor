# Net Monitor

Reproduce some of the functionality of nethogs with rust and eBPF

## Required dependecies

Ubuntu
```console
apt install clang gcc-multilib libelf1 libelf-dev zlib1g-dev
```


## vmlinux.h

To regenerate: `bpftool btf dump file /sys/kernel/btf/vmlinux format c > src/bpf/vmlinux.h`