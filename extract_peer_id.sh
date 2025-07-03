#!/bin/bash

if [ $# -eq 0 ]; then
    grep "PeerId of node:" | sed 's/.*PeerId of node: \([A-Za-z0-9_-]*\).*/\1/' | head -1
else
    grep "PeerId of node:" "$1" | sed 's/.*PeerId of node: \([A-Za-z0-9_-]*\).*/\1/' | head -1
fi
