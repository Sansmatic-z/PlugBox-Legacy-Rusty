import plugbox

def test_structured_data():
    print("\n=== TEST: Structured Data Across Wire ===")

    received = {}

    def capture_wire(payload):
        received["key"]      = payload.key
        received["data"]     = payload.data
        received["metadata"] = payload.metadata
        return f"captured:{payload.key}"

    box = plugbox.RustBox.load("DataBox")
    box.wire("capture", capture_wire)
    box.seal()
    box.run()

    assert received["key"] == "core_result"
    assert received["data"]["raw_result"] > 0
    assert received["data"]["is_even"] in (True, False)
    assert received["metadata"]["box_name"] == "DataBox"
    print(f"  Data received: {received}")
    print("  PASSED")

def test_multiple_wires():
    print("\n=== TEST: Multiple Wires All Called ===")

    call_log = []

    def wire_a(d): call_log.append("A"); return "A_done"
    def wire_b(d): call_log.append("B"); return "B_done"
    def wire_c(d): call_log.append("C"); return "C_done"

    box = plugbox.RustBox.load("MultiBox")
    box.wire("wire_a", wire_a)
    box.wire("wire_b", wire_b)
    box.wire("wire_c", wire_c)
    box.seal()

    results = box.run()

    assert len(call_log) == 3, f"Expected 3 calls, got {len(call_log)}"
    assert "A" in call_log
    assert "B" in call_log
    assert "C" in call_log
    print(f"  All wires called: {call_log}")
    print(f"  All results: {results}")
    print("  PASSED")

def test_direct_wire_call():
    print("\n=== TEST: Direct Wire Call ===")

    def transform(payload):
        return payload.data["text"].upper()

    box = plugbox.RustBox.load("DirectBox")
    box.wire("transform", transform)
    box.seal()

    custom_data = plugbox.WireData("my_key", {"text": "hello_world"}, {"type": "test"})
    result = box.call_wire("transform", custom_data)
    assert result == "HELLO_WORLD"
    print(f"  Direct call result: {result}")
    print("  PASSED")

if __name__ == "__main__":
    test_structured_data()
    test_multiple_wires()
    test_direct_wire_call()
    print("\nAll data tests passed.")
