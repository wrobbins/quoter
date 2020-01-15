#!/bin/bash

# converts args to an array
tmp=("$@")

# array to joined string
symbols=$(IFS=,; echo "${tmp[*]}")
url=https://query1.finance.yahoo.com/v7/finance/quote

fields=regularMarketPrice,regularMarketChangePercent 

curl -s $url\?symbols\=$symbols\&fields\=$fields \
        | jq ' .quoteResponse.result[] |                
                { 
                        price: .regularMarketPrice, 
                        change: (.regularMarketChangePercent | tonumber),
                        symbol: .symbol
                }' 