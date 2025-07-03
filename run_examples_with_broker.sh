#!/bin/bash

echo "Starting ngd broker..."
cargo run -p ngd -- -vv --save-key -l 14400 > ngd.log 2>&1 &
NGD_PID=$!

echo "Waiting for broker to start..."
sleep 5

PEER_ID=$(grep "PeerId of node:" ngd.log | sed 's/.*PeerId of node: \([A-Za-z0-9_-]*\).*/\1/' | head -1)

if [ -z "$PEER_ID" ]; then
    echo "Failed to extract peer ID from broker logs"
    kill $NGD_PID
    exit 1
fi

echo "Extracted peer ID: $PEER_ID"

echo "Running in_memory example..."
cargo run -p nextgraph --example in_memory -- --peer-id "$PEER_ID"

echo "Running persistent example..."
cargo run -p nextgraph --example persistent -- --peer-id "$PEER_ID"

echo "Running open example..."
cargo run -p nextgraph --example open -- --peer-id "$PEER_ID"

echo "Running sparql_update example..."
cargo run -p nextgraph --example sparql_update

kill $NGD_PID
echo "Broker stopped"
