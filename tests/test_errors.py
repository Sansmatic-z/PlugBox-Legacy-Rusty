import plugbox

def test_seal_empty_box():
    print("\n=== TEST: Cannot seal empty box ===")
    box = plugbox.RustBox.load("EmptyBox")
    try:
        box.seal()
        assert False, "Should have raised"
    except RuntimeError as e:
        print(f"  Correctly blocked: {e}")
    print("  PASSED")

def test_wire_after_seal():
    print("\n=== TEST: Cannot wire after seal ===")
    box = plugbox.RustBox.load("SealedBox")
    box.wire("x", lambda d: d.data)
    box.seal()
    try:
        box.wire("y", lambda d: d.data)
        assert False, "Should have raised"
    except RuntimeError as e:
        print(f"  Correctly blocked: {e}")
    print("  PASSED")

def test_run_before_seal():
    print("\n=== TEST: Cannot run before seal ===")
    box = plugbox.RustBox.load("UnsealedBox")
    box.wire("x", lambda d: d.data)
    try:
        box.run()
        assert False, "Should have raised"
    except RuntimeError as e:
        print(f"  Correctly blocked: {e}")
    print("  PASSED")

def test_unknown_wire():
    print("\n=== TEST: Unknown wire call raises KeyError ===")
    box = plugbox.RustBox.load("KeyBox")
    box.wire("real_wire", lambda d: d.data)
    box.seal()
    try:
        custom = plugbox.WireData("k", {}, {})
        box.call_wire("fake_wire", custom)
        assert False, "Should have raised"
    except KeyError as e:
        print(f"  Correctly blocked: {e}")
    print("  PASSED")

def test_not_callable_wire():
    print("\n=== TEST: Cannot wire a non-callable ===")
    box = plugbox.RustBox.load("NonCallableBox")
    try:
        box.wire("bad_wire", "I am a string, not a function")
        assert False, "Should have raised"
    except TypeError as e:
        print(f"  Correctly blocked: {e}")
    print("  PASSED")

if __name__ == "__main__":
    test_seal_empty_box()
    test_wire_after_seal()
    test_run_before_seal()
    test_unknown_wire()
    test_not_callable_wire()
    print("\nAll error tests passed.")

