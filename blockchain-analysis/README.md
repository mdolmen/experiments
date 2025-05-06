# Blockchain analysis experiments

Get blockchain data.

This app runs at: https://blockchain-analysis-418744621127.us-central1.run.app

## Features

The blockchain data comes from the Dexscreener API.

### Get largest pool

Endpoint: `/v1/largest-pool`.

Returns the largest pool by liquidity (usd) for given chain and address.

Example:
```bash
blockchain-analysis git:(master) ✗ curl -X POST http://127.0.0.1:8000/v1/largest-pool \
  -H "Content-Type: application/json" \
  -d '{"chain": "solana", "address":"FQgtfugBdpFN7PZ6NdPrZpVLDBrPGxXesi4gVu3vErhY"}' | jq
```

https://api.dexscreener.com/token-pairs/v1/{chainId}/{tokenAddress}

### Get aggregated liquidity

Endpoint: `/v1/liquidity`.

Return the aggregated liquidity of all the pools, in usd, for given chain and
address.

Example:
```bash
curl -X POST http://127.0.0.1:8000/v1/liquidity \
  -H "Content-Type: application/json" \
  -d '{"chain": "solana", "address":"FQgtfugBdpFN7PZ6NdPrZpVLDBrPGxXesi4gVu3vErhY"}' | jq
```

### Get number of pools

Endpoint: `/v1/number-of-pools`.

Return the number of pools for given chain and address.

Example:
```bash
curl -X POST http://127.0.0.1:8000/v1/number-of-pools \
  -H "Content-Type: application/json" \
  -d '{"chain": "solana", "address":"FQgtfugBdpFN7PZ6NdPrZpVLDBrPGxXesi4gVu3vErhY"}' | jq
```

## How to use

**Setup environment**

```bash
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
```

**Run tests**

```bash
pytest -s app/tests/tests_pool.py
```

**Start app**

```bash
uvicorn app.main:app --reload

# http://localhost:8000/[...]
```

## Deploy on GCP

```bash
# Assumes gcloud is install and project is created/configured
# Builds an amd64 image and pushes it to GCP
./deploy.sh
```

**Example**
```bash
curl -X POST https://blockchain-analysis-418744621127.us-central1.run.app/v1/number-of-pools \
  -H "Content-Type: application/json" \
  -d '{"chain": "solana", "address":"FQgtfugBdpFN7PZ6NdPrZpVLDBrPGxXesi4gVu3vErhY"}' | jq
```

## Notes

All endpoints are defined in the same file: `pool.py`.

It's possible to authorize other chains by adding them in
`pool.py:SUPPORTED_CHAINS` list.

Unit test are available for all of the three endpoints.

A docker image is created for deployment on GCP.

A basic logging mechanism is implemented to store logs in a text file.
