#!/usr/bin/env bash

# navigate to directory
SCRIPTPATH="$( cd "$(dirname "$0")" ; pwd -P )"
cd $SCRIPTPATH

cd ..
export $(cat .env | xargs)

cd ..
cd frontend
npm install
npm run build
cd ../rust-server/networking/actix-server
cargo clean
cargo run