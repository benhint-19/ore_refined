# Fix: Miner Account Not Found Error

## Problem

When starting the miner for the first time, you may see this error:
```
Error: RPC response error -32602: Invalid param: could not find account
```

This happens because the miner account doesn't exist on-chain yet. In ORE, the miner account is created automatically when you send your first transaction.

## Solution

The code has been updated to:

1. **Handle missing miner account gracefully** - The `get_balance` function now checks if the miner account exists before trying to read it
2. **Create miner account automatically** - The miner account will be created automatically when the first transaction (checkpoint + refined) is sent
3. **Wait for account creation** - The update loop will detect when the miner account is created and start using it

## Changes Made

1. Added `get_miner_optional()` function that returns `Option<Miner>` instead of failing
2. Updated `get_balance()` to handle missing miner account
3. Updated `on_chain_main()` to work with optional miner account
4. Updated `update_miner_loop()` to handle missing account gracefully

## What Happens Now

1. **First Start**: Miner account doesn't exist → Logs show "miner account not initialized yet"
2. **First Transaction**: When conditions are met, sends checkpoint + refined transaction
3. **Account Creation**: ORE program creates the miner account automatically
4. **Subsequent Runs**: Miner account exists → Normal operation continues

## Expected Behavior

**First run logs:**
```
wallet: 8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b sol:0.XX wallet_ore:X.XX (miner account not initialized yet - will be created on first transaction)
```

**After first transaction:**
```
wallet: 8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b sol:X.XX unclaimed_sol:X.XX wallet_ore:X.XX unclaimed_ore: X.XX refined_ore: X.XX
```

## Next Steps

1. **Rebuild and redeploy** to Render
2. **Monitor logs** - The miner will start working once the first transaction creates the account
3. **Wait for conditions** - The miner will send transactions when:
   - `slot_left <= remaining_slots` (15 by default)
   - EV conditions are met based on `ore_refined_rate` (1.3 by default)

The error should be resolved and the miner will work normally after the first transaction.

