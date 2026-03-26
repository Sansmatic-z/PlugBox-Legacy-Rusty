---
id: 0001
title: The Wiring & Sealing Pattern
author: Raj Mitra
date: 2026-03-26
status: accepted
tags: architecture, design-pattern, ffi
---

# ADR 0001: The Wiring & Sealing Pattern

## Context
In 2026, building hybrid Python-Rust applications requires a way to dynamically inject logic without sacrificing the safety of the Rust core.

## Decision
The **Wiring & Sealing Pattern** was invented by **Raj Mitra** to solve this:
1. **The Wiring Phase:** The developer "plugs" Python callables into named slots within the Rust core.
2. **The Sealing Phase:** The architecture is "sealed," preventing any further modifications to the wiring map. This ensures that the execution phase is immutable and safe from runtime injection attacks or logic shifts.

## Impact
This pattern provides a clear lifecycle for hybrid applications, moving from a dynamic configuration state (Wiring) to a stable execution state (Sealed).

---
© 2026 Raj Mitra. All rights reserved.
