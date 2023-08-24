#!/bin/bash

# sudo apt  install jq

value=$(jq '.azle_hello_world.local' config/backend_canister_ids.json)

echo $value





nextPublic=NEXT_PUBLIC_CANISTER_ID_AZLE_HELLO_WORLD=
canisterId=$value
variable="NEXT_PUBLIC_CANISTER_ID_AZLE_HELLO_WORLD="$canisterId
# echo " String 1: $variable"

icHost="NEXT_PUBLIC_IC_HOST="http://127.0.0.1:8080/"

echo $variable > .env
echo $icHost >> .env

