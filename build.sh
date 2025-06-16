#!/bin/sh

set -ex

PI_HOST=pi@192.168.0.23
PROJECT_DIR=/home/pi/heisenberg
FILES="heisenberg.service config.json build/erlang-shipment/"

gleam export erlang-shipment
rsync -r -vv $FILES $PI_HOST:$PROJECT_DIR

if [ "$1" = "--release" ]; then
    echo "Starting systemd service..."
    ssh $PI_HOST << EOF
        sudo systemctl link $PROJECT_DIR/heisenberg.service
        sudo systemctl enable heisenberg
        sudo systemctl restart heisenberg
EOF
else
    echo "Running in dev mode..."
    # Run the program directly for testing
    ssh -t $PI_HOST "
        # sudo systemctl stop heisenberg;
        cd ./heisenberg;
        ./entrypoint.sh run"
fi
