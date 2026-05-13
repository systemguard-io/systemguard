# ADR 003: Rule-Based Detection, Not ML

**Decision:** Statistical baselines + explicit rules. No ML in v1.0

**Why:**
- Explainability: "nginx accessed /etc/shadow" not "score 0.87"
- Performance: 0.5ms vs 50ms
- Debuggable and unit-testable
