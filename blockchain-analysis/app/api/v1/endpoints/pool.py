import requests

from fastapi import APIRouter
from pydantic import BaseModel

SUPPORTED_CHAINS = ['solana']

router = APIRouter()

class PoolParams(BaseModel):
    chain: str
    address: str

def get_chain_data(chain, address):
    if chain not in SUPPORTED_CHAINS:
        return {"error": f"Unsupported chain provided: {chain}"}

    # Get data from Dexscreener's API
    url = f"https://api.dexscreener.com/token-pairs/v1/{chain}/{address}"
    response = requests.get(url)
    if response.status_code != 200:
        return {"error": "Failed to fetch data from DexScreener"}

    return response.json()

@router.get("/")
async def index():
    return {"message": "ping ok"}

@router.post("/largest-pool")
async def get_largest_pool(params: PoolParams):
    pairs = get_chain_data(params.chain, params.address)
    if "error" in pairs:
        return pairs

    # Find largest pool
    largest_pool = max(pairs, key=lambda pool: pool.get("liquidity", {}).get("usd", 0))
    return {"message": largest_pool}

@router.post("/liquidity")
async def get_liquidity(params: PoolParams):
    pairs = get_chain_data(params.chain, params.address)
    if "error" in pairs:
        return pairs

    liquidity = sum(pool.get("liquidity", {}).get("usd", 0) for pool in pairs)
    return {"message": liquidity}

@router.post("/number-of-pools")
async def get_number_of_pools():
    return {"message": "TODO: get the number of pools"}