# ⚠️ PlugBox (LEGACY PROTOTYPE)
**By Raj Mitra**

> **NOTE:** This project is for ARCHIVAL purposes only. 
> It was built as a "V1" prototype to explore the Python-Rust bridge. 
> Due to high FFI overhead identified during benchmarking, this architecture has been succeeded by **[WireBox](https://github.com/Sansmatic-z/WireBox-Rusty)**.

## Why Legacy?
The "PlugBox" architecture crosses the FFI boundary for every single data iteration. For large datasets, this results in significant performance loss. **WireBox** solves this using "Data Trapping" and "Native Batching."

---

# Original PlugBox README

## Concept
Rust owns performance. Python owns logic. You wire them together.

## Build
pip install maturin
maturin develop --release

## Test
python3 tests/test_basic.py
python3 tests/test_data.py
python3 tests/test_errors.py

## Usage
import plugbox

def my_logic(payload):
    return payload.data["text"].upper()

box = plugbox.RustBox.load("MyBox")
box.wire("process", my_logic)
box.seal()
results = box.run()
print(results)

---

## Phase 3 Benchmark & Architecture Review

As part of our commitment to proving speed early, we ran a 100,000-row CSV processing benchmark. 

**Target:** 5x speedup over Pure Python.

**Real Numbers:**
* Pure Python (`csv.reader`): ~0.087s
* PlugBox (`CsvBox` plugin): ~0.078s 
* **Result:** ~1.1x speedup.

### Honest Kill Condition Triggered
*`Benchmark shows less than 3x speedup → architecture problem`*

**The Problem:** Crossing the FFI boundary (GIL lock + PyObject mapping) 100,000 times destroys Rust's performance advantage. Because Python's `csv` module is written in C, the per-row function call overhead in PlugBox makes it almost identical in speed to "pure" Python. 

**The Pivot:** For tight-loop plugins (like data iteration), PlugBox must move away from per-iteration FFI calls. Future iterations will pivot to either:
1. **Batch Wires:** Rust passes chunks of 10,000 rows to Python as vector arrays for batch evaluation.
2. **Declarative Wires:** Python wires pass AST/DSL rules to Rust, which Rust evaluates natively without waking the GIL.
