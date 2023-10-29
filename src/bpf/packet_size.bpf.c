#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

struct
{
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, 1024);
    __type(key, __u32);
    __type(value, __u64);
} packet_stats SEC(".maps");

char __license[] SEC("license") = "GPL";
#define TC_ACT_OK 0

SEC("tc")
int packet_size(struct __sk_buff *ctx)
{
    void *data = (void *)(long)ctx->data;
    void *data_end = (void *)(long)ctx->data_end;
    int pkt_sz = data_end - data;

    int pid = bpf_get_current_pid_tgid() >> 32;

    struct value *value = bpf_map_lookup_elem(&packet_stats, &pid);

    if (value)
    {
        __sync_fetch_and_add(&value, pkt_sz);
    }
    else
    {
        bpf_map_update_elem(&packet_stats, &pid, &pkt_sz, BPF_NOEXIST);
    }

    return TC_ACT_OK;
}
