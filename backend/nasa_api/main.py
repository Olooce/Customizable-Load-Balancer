import datetime

import requests
from fastapi import FastAPI, Response

app = FastAPI()

import codecs

""" 
Decodes the given API key string using ROT13 encoding 
"""
NASA = codecs.decode("9PNJiWSUGzpFtreNtHWzyT8w0ute06KVcUDRKpt6", "rot13")


@app.head("/heartbeat")
async def heartbeat():
    """ 
    Endpoint to check if the service is alive. Responds with a status message 
    """
    return {"status": "alive"}



