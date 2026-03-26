import sys
import plugbox

def test_basic_wire():
    print("\n=== TEST: Basic Wire ===")

    def my_processor(payload):
        print(f"  [Python] Got: {payload}")
        return f"processed:{payload.data['raw_result']}"

    box = plugbox.RustBox.load("BasicBox")
    box.wire("process", my_processor)
    box.seal()

    results = box.run()
    print(f"  [Python] Results: {results}")
    assert "process" in results
    assert "processed:" in results["process"]
    print("  PASSED")

def test_list_wires():
    print("\n=== TEST: List Wires ===")

    box = plugbox.RustBox.load("ListBox")
    box.wire("alpha", lambda d: d.data)
    box.wire("beta", lambda d: d.key)
    box.seal()

    wires = box.list_wires()
    assert "alpha" in wires
    assert "beta" in wires
    print(f"  Wires found: {wires}")
    print("  PASSED")

if __name__ == "__main__":
    test_basic_wire()
    test_list_wires()
    print("\nAll basic tests passed.")
