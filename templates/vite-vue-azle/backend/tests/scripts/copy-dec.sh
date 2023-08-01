#!/bin/bash

# Generate declarations files using dfx
dfx generate backend

# Create a directory
mkdir -p backend/tests/declarations

# Copy files to the new directory
cp -r src/declarations/ backend/tests/declarations/

# Change declarations from JS to TS
pushd backend/tests/declarations
    rm **/index.js
    rm **/*.did
    for f in **/*.js; do
        mv -- "$f" "${f%.js}.ts"
    done
popd