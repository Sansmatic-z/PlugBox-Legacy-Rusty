import plugbox

def test_network_plugin():
    print("\n--- Network Plugin: Request Filter ---")
    
    def firewall_rule(request):
        # Python defines the security rules
        if "admin" in request or "drop" in request:
            return False
        return True
        
    box = plugbox.NetworkBox()
    box.wire("filter", firewall_rule)
    box.seal()
    
    requests = [
        "/home",
        "/admin/login",
        "/api/data",
        "/drop_table"
    ]
    
    responses = box.process_requests(requests)
    for req, res in zip(requests, responses):
        print(f"Request: {req.ljust(15)} -> {'ALLOWED' if res else 'BLOCKED'}")
        
    assert responses == [True, False, True, False]
    print("Network plugin test passed.")

if __name__ == "__main__":
    test_network_plugin()
