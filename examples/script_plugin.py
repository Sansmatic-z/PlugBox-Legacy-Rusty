import plugbox

def test_game_plugin():
    print("\n--- Script Plugin: Game Engine ---")
    
    def python_logic(state):
        # Python adds a custom multiplier to the physics state
        return state + 5
        
    box = plugbox.GameBox()
    box.wire("update", python_logic)
    box.seal()
    
    final_state = box.run_loop(100)
    print(f"Final game state after 100 ticks: {final_state}")
    # Engine adds 1, Python adds 5 each tick. 100 * 6 = 600.
    assert final_state == 600
    print("Script plugin test passed.")

if __name__ == "__main__":
    test_game_plugin()
