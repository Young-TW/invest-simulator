import requests
import json

# CoinGecko API URL
url = 'https://api.coingecko.com/api/v3/simple/price'

with open('data/coin.json', 'r') as f:
    coins = json.load(f)

response = requests.get(url, params={
    'ids': ','.join(coins['BTC']['full_name']),
    'vs_currencies': 'usd'
})

print(response.json())
