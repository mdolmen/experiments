# Blockchain analysis experiments

Get blockchain data.

## Features

The blockchain data comes from the Dexscreener API.

### Get largest pool

Endpoint: `/v1/largest-pool`.

Returns the largest pool by liquidity (usd) for given chain and address.

Exemple:
```bash
blockchain-analysis git:(master) ✗ curl -X POST http://127.0.0.1:8000/v1/largest-pool \
  -H "Content-Type: application/json" \
  -d '{"chain": "solana", "address":"FQgtfugBdpFN7PZ6NdPrZpVLDBrPGxXesi4gVu3vErhY"}'
```

https://api.dexscreener.com/token-pairs/v1/{chainId}/{tokenAddress}

### Get aggregated liquidity

Endpoint: `/v1/liquidity`.

Return the aggregated liquidity of all the pools, in usd, for given chain and
address.

Exemple:
```bash
curl -X POST http://127.0.0.1:8000/v1/liquidity \
  -H "Content-Type: application/json" \
  -d '{"chain": "solana", "address":"FQgtfugBdpFN7PZ6NdPrZpVLDBrPGxXesi4gVu3vErhY"}'
```

### Get number of pools

Endpoint: `/v1/number-of-pools`.

Return the number of pools for given chain and address.

Exemple:
```bash
curl -X POST http://127.0.0.1:8000/v1/number-of-pools \
  -H "Content-Type: application/json" \
  -d '{"chain": "solana", "address":"FQgtfugBdpFN7PZ6NdPrZpVLDBrPGxXesi4gVu3vErhY"}'
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
