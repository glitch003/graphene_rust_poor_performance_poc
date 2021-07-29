#!/bin/bash

INITIAL_PORT=7470
NODENUM=3

CMDS=()

for (( c=0; c<$NODENUM; c++ ))
do
	PORT=$(($INITIAL_PORT + $c ))
  CMDS+=("sudo ROCKET_PORT=$PORT graphene-sgx poc")
done

concurrently "${CMDS[@]}"

# concurrently "ROCKET_PORT=7470 cargo run" "ROCKET_PORT=7471 cargo run" "ROCKET_PORT=7472 cargo run" "ROCKET_PORT=7473 cargo run" "ROCKET_PORT=7474 cargo run" "ROCKET_PORT=7475 cargo run" "ROCKET_PORT=7476 cargo run" "ROCKET_PORT=7477 cargo run" "ROCKET_PORT=7478 cargo run" "ROCKET_PORT=7479 cargo run"