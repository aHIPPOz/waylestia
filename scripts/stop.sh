#!/bin/bash

# Stop Waylestia services

echo "Stopping Waylestia services..."

systemctl --user stop waylestia-widgets
echo "✓ waylestia-widgets stopped"

systemctl --user stop waylestia-core
echo "✓ waylestia-core stopped"

echo ""
echo "Waylestia services have been stopped."
