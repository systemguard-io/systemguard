# ADR 001: Why Rust for SystemGuard
**Date:** 2025-05-13 | **Status:** Accepted

## Context
Agent runs as root on every host. Need memory safety, zero-cost, static binary, eBPF support.

## Decision
**Choose Rust** for agent and collector.

## Rationale
1. No GC pauses (Go loses events during syscall storms)
2. eBPF ecosystem mature (aya-rs, libbpf-rs)
3. Memory safety eliminates RCE class
4. 5MB static binary vs Go 18MB

## Consequences
- No unwrap() in production
- Strict clippy lints
