#!/bin/bash
set -e

# Generate keypair.json from PRIVATE_KEY environment variable
if [ -z "$PRIVATE_KEY" ]; then
    echo "Error: PRIVATE_KEY environment variable is not set"
    exit 1
fi

# Create keypair.json from base58 private key
node scripts/render-keypair.js

# Verify keypair file exists
if [ ! -f "keypair.json" ]; then
    echo "Error: Failed to create keypair.json"
    exit 1
fi

# Set default values if not provided
RPC_URL=${RPC_URL:-"https://solana.drpc.org"}
PER_ROUND_DEPLOY_AMOUNT=${PER_ROUND_DEPLOY_AMOUNT:-"0.0001"}
REMAINING_SLOTS=${REMAINING_SLOTS:-"15"}
ORE_REFINED_RATE=${ORE_REFINED_RATE:-"1.3"}

echo "Starting ORE Refined miner..."
echo "RPC: $RPC_URL"
echo "Deploy Amount: $PER_ROUND_DEPLOY_AMOUNT SOL"
echo "Remaining Slots: $REMAINING_SLOTS"
echo "ORE Refined Rate: $ORE_REFINED_RATE"

# Run the compiled binary
./target/release/ore-refined \
    --rpc "$RPC_URL" \
    --keypair keypair.json \
    --per-round-deploy-amount "$PER_ROUND_DEPLOY_AMOUNT" \
    --remaining-slots "$REMAINING_SLOTS" \
    --ore-refined-rate "$ORE_REFINED_RATE"

