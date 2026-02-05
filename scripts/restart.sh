#!/bin/bash

# Restart Waylestia services

echo "Restarting Waylestia services..."

systemctl --user restart waylestia-widgets
echo "✓ waylestia-widgets restarted"

systemctl --user restart waylestia-core
echo "✓ waylestia-core restarted"

echo ""
echo "Waylestia services have been restarted."
