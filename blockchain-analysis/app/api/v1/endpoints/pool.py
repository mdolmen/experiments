from fastapi import APIRouter

router = APIRouter()

@router.get("/")
async def index():
    return {"message": "ping ok"}

@router.get("/largest-pool")
async def get_largest_pool():
    return {"message": "TODO: get info about the largest pool"}

@router.get("liquidity")
async def get_liquidity():
    return {"message": "TODO: get liquidity of all pools"}

@router.get("/number-of-pools")
async def get_number_of_pools():
    return {"message": "TODO: get the number of pools"}