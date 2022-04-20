#!/usr/bin/env bash

set -e

curl --location --request POST http://localhost:3030 \
  --header 'Content-Type: application/json' \
  --data-raw '{
  	"jsonrpc":"2.0",
  	"method":"ku_clientVersion",
  	"params":[],
  	"id":1
  }'

# kusari/v0.1.0-stable-cfd35ce/macos-aarch64/rust1.58.0
