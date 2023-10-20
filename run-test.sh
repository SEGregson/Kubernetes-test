#!/bin/bash
## currently not working

# run server
cd server
cargo run

# wait for server to start
sleep 3

cd ../client
cargo run