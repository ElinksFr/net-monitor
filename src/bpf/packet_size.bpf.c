// #include <linux/bpf.h>
#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

typedef struct
{
    u32 received;
    u32 send;
} track;

struct
{
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, 5096);
    __type(key, u32);
    __type(value, track);
} packet_stats SEC(".maps");

char __license[] SEC("license") = "GPL";
#define TC_ACT_OK 0
#define UDP_ACT_OK 0

void increment_received_packet_counter(u32 pid, int size_of_new_packets)
{
    track *value = bpf_map_lookup_elem(&packet_stats, &pid);

    if (value)
    {
        value->received += size_of_new_packets;
    }
    else
    {
        track tracked = {size_of_new_packets, 0};
        bpf_map_update_elem(&packet_stats, &pid, &tracked, BPF_ANY);
    }
}

void increment_send_packet_counter(u32 pid, int size_of_new_packets)
{
    track *value = bpf_map_lookup_elem(&packet_stats, &pid);

    if (value)
    {
        value->send += size_of_new_packets;
    }
    else
    {
        track tracked = {0, size_of_new_packets};
        bpf_map_update_elem(&packet_stats, &pid, &tracked, BPF_ANY);
    }
}

SEC("kretprobe/tcp_recvmsg")
int BPF_KRETPROBE(tcp_received_packet_size, int ret)
{
    u32 pid = bpf_get_current_pid_tgid() >> 32;
    increment_received_packet_counter(pid, ret);

    return TC_ACT_OK;
}

SEC("kretprobe/udp_recvmsg")
int BPF_KRETPROBE(udp_received_packet_size, int ret)
{
    u32 pid = bpf_get_current_pid_tgid() >> 32;
    increment_received_packet_counter(pid, ret);

    return UDP_ACT_OK;
}

SEC("kretprobe/tcp_sendmsg")
int BPF_KRETPROBE(tcp_send_packet_size, int ret)
{
    u32 pid = bpf_get_current_pid_tgid() >> 32;
    increment_send_packet_counter(pid, ret);

    return TC_ACT_OK;
}

SEC("kretprobe/udp_sendmsg")
int BPF_KRETPROBE(udp_send_packet_size, int ret)
{
    u32 pid = bpf_get_current_pid_tgid() >> 32;
    increment_send_packet_counter(pid, ret);

    return UDP_ACT_OK;
}
