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

@app.get("/neo")
async def get_neo(start_date: datetime.datetime, end_date: datetime.datetime, response: Response):
    """ 
    Endpoint to get Near-Earth Objects (NEOs) data from NASA API for the given date range.
    Parameters:
    - start_date: The start date for the NEO data.
    - end_date: The end date for the NEO data.
    - response: The response object to manipulate the HTTP response.
    """
    start_date_formatted = start_date.date()
    params = {
        "start_date": start_date.date(),
        "end_date": end_date.date(),
        "api_key": NASA
    }
    web_resp = requests.get("https://api.nasa.gov/neo/rest/v1/feed", params=params)

    response.status_code = web_resp.status_code
    response.body = web_resp.content

    # response.headers = web_resp.headers
    return response


@app.get("/")
async def root():
    """ 
    Default root endpoint that returns a simple hello world message 
    """
    return {"message": "Hello World"}


@app.get("/hello/{name}")
async def say_hello(name: str):
    """ 
    Endpoint to return a personalized greeting.
    Parameters:
    - name: The name to include in the greeting message.
    """
    return {"message": f"Hello {name}"}

