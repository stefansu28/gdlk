#!/bin/bash

set -ex

# When using this script, make sure you start the container with --init
# so that it dies properly
pushd ../wasm
wasm-pack build
popd
npm install
npx nodemon -x "npm run relay" -w src -e "ts tsx" -i "**/__generated/*" &
npm start | cat # pipe into cat so console does not get reset
