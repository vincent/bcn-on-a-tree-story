#!/bin/bash

# Start the first process
cd ./boats_api
cargo run --bin boats_api &
  
# Start the second process
./boats_web
trunk serve --port 8081 &
  
# Wait for any process to exit
wait -n
  
# Exit with status of process that exited first
exit $?
