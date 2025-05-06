from fastapi import FastAPI
from app.api.v1.endpoints import pool

app = FastAPI()

app.include_router(pool.router, prefix="/v1")