import csv
import time
import random
import os
import plugbox

# Generate mock CSV if not exists
CSV_FILE = "mock_data.csv"
NUM_ROWS = 100_000

if not os.path.exists(CSV_FILE):
    print(f"Generating {NUM_ROWS} rows of test data...")
    with open(CSV_FILE, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["id", "name", "value", "category"])
        for i in range(NUM_ROWS):
            writer.writerow([str(i), f"User{i}", str(random.randint(0, 100)), random.choice(["A", "B", "C"])])

def pure_python_benchmark():
    print("\n--- Pure Python Processing ---")
    start = time.perf_counter()
    passed = 0
    with open(CSV_FILE, "r", newline="") as f:
        reader = csv.reader(f)
        next(reader) # skip header
        for row in reader:
            if int(row[2]) > 50:
                passed += 1
    duration = time.perf_counter() - start
    print(f"Rows passed: {passed}")
    print(f"Time: {duration:.4f}s")
    return duration

def plugbox_benchmark():
    print("\n--- PlugBox Processing ---")
    start = time.perf_counter()
    
    def my_filter(row):
        # row[2] is value
        return int(row[2]) > 50

    box = plugbox.CsvBox()
    box.wire("filter", my_filter)
    box.seal()
    passed = box.process(CSV_FILE)
    
    duration = time.perf_counter() - start
    print(f"Rows passed: {passed}")
    print(f"Time: {duration:.4f}s")
    return duration

if __name__ == "__main__":
    py_time = pure_python_benchmark()
    pb_time = plugbox_benchmark()
    
    speedup = py_time / pb_time
    print(f"\nResult: PlugBox is {speedup:.1f}x faster.")
