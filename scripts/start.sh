#!/bin/bash

# Start Waylestia services

echo "Starting Waylestia services..."

systemctl --user start waylestia-core
echo "✓ waylestia-core started"

sleep 2

systemctl --user start waylestia-widgets
echo "✓ waylestia-widgets started"

echo ""
echo "Waylestia services are now running."
echo "Check status with: systemctl --user status waylestia-core"
