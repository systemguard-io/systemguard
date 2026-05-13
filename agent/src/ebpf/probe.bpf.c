// SystemGuard eBPF probe
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
struct event { __u64 ts; __u32 pid; __u32 uid; char comm[16]; __u32 syscall; char path[256]; };
struct { __uint(type, BPF_MAP_TYPE_RINGBUF); __uint(max_entries, 1<<24); } events SEC(".maps");
SEC("tracepoint/syscalls/sys_enter_openat")
int trace_openat(void *ctx) { struct event *e = bpf_ringbuf_reserve(&events, sizeof(*e),0); if(!e) return 0; e->ts=bpf_ktime_get_ns(); e->pid=bpf_get_current_pid_tgid()>>32; e->uid=bpf_get_current_uid_gid(); bpf_get_current_comm(&e->comm,16); e->syscall=257; bpf_ringbuf_submit(e,0); return 0; }
char LICENSE[] SEC("license") = "GPL";
