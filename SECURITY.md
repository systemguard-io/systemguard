# Security Policy

## Supported Versions
| Version | Supported |
| 0.1.x | :white_check_mark: |
| < 0.1 | :x: |

## Reporting a Vulnerability
**Do NOT open a public issue.**
Email: security@systemguard.io
- Description, steps to reproduce, impact

We respond within 48h. Critical fixes within 7 days.

## Security Architecture
- Agent runs with CAP_BPF only
- eBPF verified by kernel
- Collector uses prepared statements
- mTLS planned for v0.2
- Audit log append-only, HMAC-signed

## Known Limitations (v0.1)
- No at-rest encryption
- Requires root/CAP_BPF
- Cannot detect kernel rootkits
