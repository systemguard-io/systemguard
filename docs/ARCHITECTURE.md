# SystemGuard Architecture

## Overview

SystemGuard consists of three main components:

### 1. Agent (Lightweight Rust Binary)
- Runs on each monitored Linux host
- Loads eBPF programs into kernel
- Collects syscall events
- Applies local rules
- Sends events to collector

### 2. Collector (Central Server)
- Receives events from all agents
- Performs anomaly detection
- Stores events in PostgreSQL
- Triggers alerts
- Exposes REST API

### 3. Dashboard (React Frontend)
- Real-time event visualization
- Alert management
- Security analytics
- Report generation

## Data Flow
See implementation details in code.
