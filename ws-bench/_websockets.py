#!/usr/bin/env python

import asyncio
import sys
from websockets.server import serve

async def echo(websocket):
    async for message in websocket:
        await websocket.send(message)

async def main():
    async with serve(echo, "localhost", sys.argv[1], max_size=16777216):
        await asyncio.Future()

asyncio.run(main())