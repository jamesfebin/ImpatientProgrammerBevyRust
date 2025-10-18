#!/bin/bash

echo "🚀 Starting game with performance monitoring..."
echo "📊 Performance logs will appear every second"
echo "⏹️  Press Ctrl+C to stop"
echo ""

# Run the game and capture output
cargo run 2>&1 | grep -E "(Performance:|📊|ERROR|WARN|INFO)"
