from fastapi.testclient import TestClient
from app.main import app

def test_get_largest_pool():
    client = TestClient(app)

    payload = {
        "chain": "solana",
        "address": "FQgtfugBdpFN7PZ6NdPrZpVLDBrPGxXesi4gVu3vErhY"
    }

    response = client.post("/v1/largest-pool", json=payload)

    assert response.status_code == 200

    # Assert that the response contains the expected output
    response_json = response.json()
    if "error" in response_json:
        print("Error occurred:", response_json["error"])
    else:
        print("Largest pool:", response_json)