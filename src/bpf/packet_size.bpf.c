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
#define UDP_ACT_OK 0

void increate_received_packet_counter(__u32 pid, int size_of_new_packets)
{
    int *value = bpf_map_lookup_elem(&packet_stats, &pid);

    if (value)
    {
        int total_size = (*value) + size_of_new_packets;
        bpf_map_update_elem(&packet_stats, &pid, &total_size, BPF_EXIST);
    }
    else
    {
        bpf_map_update_elem(&packet_stats, &pid, &size_of_new_packets, BPF_ANY);
    }
}

SEC("kretprobe/tcp_recvmsg")
int BPF_KRETPROBE(tcp_received_packet_size, int ret)
{
    __u32 pid = bpf_get_current_pid_tgid() >> 32;
    increate_received_packet_counter(pid, ret);

    return TC_ACT_OK;
}

SEC("kretprobe/udp_recvmsg")
int BPF_KRETPROBE(udp_received_packet_size, int ret)
{
    __u32 pid = bpf_get_current_pid_tgid() >> 32;
    increate_received_packet_counter(pid, ret);

    return UDP_ACT_OK;
}
