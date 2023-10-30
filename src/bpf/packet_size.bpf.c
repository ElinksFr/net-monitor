// #include <linux/bpf.h>
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

struct
{
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 5096);
    __type(key, __u32);
    __type(value, int);
} packet_stats SEC(".maps");

char __license[] SEC("license") = "GPL";
#define TC_ACT_OK 0

SEC("kretprobe/tcp_recvmsg")
int BPF_KRETPROBE(packet_size, int ret)
{
    int pkt_sz = ret;

    __u32 pid = bpf_get_current_pid_tgid() >> 32;

    int *value = bpf_map_lookup_elem(&packet_stats, &pid);

    if (value)
    {
        *value += pkt_sz;
        int total_size = (*value) + pkt_sz;
        bpf_map_update_elem(&packet_stats, &pid, &total_size, BPF_EXIST);
    }
    else
    {
        bpf_map_update_elem(&packet_stats, &pid, &pkt_sz, BPF_ANY);
    }

    return TC_ACT_OK;
}
