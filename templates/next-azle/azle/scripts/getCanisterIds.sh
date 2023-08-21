#!/bin/bash


# azle/.dfx/local/canister_ids.json

if [[ $# -lt 1 ]]; then
    echo "Number of arguments supplied not correct. Call this script: \
    ./deploy.sh {env} \
    env should be one of the networks configured in dfx.json."
    exit 1
fi


ENV=$1


if [[ $ENV == "local" ]]; then

cp .dfx/local/canister_ids.json ./canister_ids.json

fi


if [[ $ENV == "ic" ]]; then

cp .dfx/ic/canister_ids.json ./canister_ids.json

fi