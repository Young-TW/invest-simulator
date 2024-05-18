import requests

# CoinGecko API URL
url = 'https://api.coingecko.com/api/v3/simple/price'
btc_params = {
    'ids': 'bitcoin',
    'vs_currencies': 'usd'
}

eth_params = {
    'ids': 'ethereum',
    'vs_currencies': 'usd'
}

sol_parmas = {
    'ids': 'solana',
    'vs_currencies': 'usd'
}


response = requests.get(url, params=btc_params)
data = response.json()

# 打印比特幣價格
print('Bitcoin price (USD):', data['bitcoin']['usd'])
# print('Ethereum price (USD):', data['ethereum']['usd'])
# print('Solana price (USD):', data['solana']['usd'])
