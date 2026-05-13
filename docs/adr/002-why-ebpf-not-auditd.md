# ADR 002: eBPF vs auditd

| Method | CPU | Latency | Loss |
| auditd | 18-25% | 2-5ms | 3-5% |
| eBPF | 1.2-1.8% | 0.1ms | 0% |

**Decision:** eBPF primary, auditd fallback for kernel <5.4
