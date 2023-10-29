#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

struct
{
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 5096);
    __type(key, __u32);
    __type(value, __u64);
} packet_stats SEC(".maps");

char __license[] SEC("license") = "GPL";
#define TC_ACT_OK 0

SEC("sk_msg")
int packet_size(struct sk_msg_md *msg)
{
    void *data = (void *)(long)msg->data;
    void *data_end = (void *)(long)msg->data_end;
    __u64 pkt_sz = data_end - data;

    __u32 pid = bpf_get_current_pid_tgid() >> 32;

    __u64 *value = bpf_map_lookup_elem(&packet_stats, &pid);

    if (value)
    {
        *value += pkt_sz;
        __u64 total_size = (*value) + pkt_sz;
        bpf_map_update_elem(&packet_stats, &pid, &total_size, BPF_EXIST);
    }
    else
    {
        bpf_map_update_elem(&packet_stats, &pid, &pkt_sz, BPF_ANY);
    }

    return TC_ACT_OK;
}
