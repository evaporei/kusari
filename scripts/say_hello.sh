#!/bin/bash

curl --data '{"method":"say_hello","id":1,"jsonrpc":"2.0"}' -H "Content-Type: application/json" -X POST http://localhost:3030
