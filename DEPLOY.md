# Deployment Instructions - Render

## Prerequisites

1. Render account (sign up at https://render.com)
2. GitHub/GitLab/Bitbucket repository with your code
3. Solana RPC endpoint (recommended: Helius)

## Step-by-Step Deployment

### 1. Prepare Your Repository

Ensure these files are committed:
- `render.yaml` - Render configuration
- `scripts/start.sh` - Startup script
- `scripts/render-keypair.js` - Keypair generator
- `Cargo.toml` - Rust dependencies
- All source files in `src/`

### 2. Connect to Render

**Option A: Using Blueprint (render.yaml) - Recommended**

1. Go to https://dashboard.render.com
2. Click "New +" → "Blueprint"
3. Connect your Git repository
4. Render will auto-detect `render.yaml` and create the service

**Option B: Manual Setup**

1. Go to https://dashboard.render.com
2. Click "New +" → "Background Worker"
3. Connect your Git repository
4. Configure:
   - **Name:** `ore-refined-miner`
   - **Environment:** `Rust`
   - **Region:** Choose closest to you
   - **Branch:** `main` (or your default branch)
   - **Root Directory:** (leave empty)
   - **Build Command:** `cargo build --release`
   - **Start Command:** `./scripts/start.sh`

### 3. Set Environment Variables

In Render dashboard → Your Service → Environment:

**Required:**
```
RPC_URL=https://api.mainnet-beta.solana.com
PRIVATE_KEY=5tk76qXqGEuCWp25S9Rsws76WJprwdW15RekPVqUyiFGSpJZDUd5tZQszKXTPAZEjVjVQLehzTaSgFi1Jxf8RdiR
```

**Optional (defaults already in render.yaml):**
```
PER_ROUND_DEPLOY_AMOUNT=0.0001
REMAINING_SLOTS=15
ORE_REFINED_RATE=1.3
PUBLIC_KEY=8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b
```

### 4. Deploy

- If using Blueprint: Click "Apply" to deploy
- If manual: Click "Create Background Worker"

### 5. Monitor

- View logs in real-time: Dashboard → Your Service → Logs
- Check service status: Dashboard → Your Service → Metrics
- Verify mining activity in logs

## Configuration Details

### Your Settings

- **Deploy Amount:** 0.0001 SOL per round
- **Remaining Slots:** 15 (deploys in last 15 slots)
- **ORE Refined Rate:** 1.3
- **Public Key:** 8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b

### RPC Endpoints

**Free/Public:**
- `https://api.mainnet-beta.solana.com` (rate-limited)

**Recommended (Paid):**
- Helius: https://www.helius.dev
- QuickNode: https://www.quicknode.com
- Alchemy: https://www.alchemy.com

## Troubleshooting

### Service Won't Start

1. **Check logs** for error messages
2. **Verify environment variables** are set correctly
3. **Ensure PRIVATE_KEY** is in base58 format
4. **Check RPC_URL** is accessible

### Build Fails

1. **Check Rust version** - Render uses latest stable
2. **Verify dependencies** - All crates in Cargo.toml must be accessible
3. **Review build logs** for specific errors

### Keypair Issues

- The startup script generates `keypair.json` automatically
- Verify `PRIVATE_KEY` environment variable is set
- Check logs for "Keypair file created successfully"

### Insufficient Balance

- Ensure wallet has SOL for transactions
- Check balance in logs: `wallet: ... sol:X.XX`

## Cost

- **Free Tier:** 750 hours/month (suitable for testing)
- **Starter:** $7/month (recommended for continuous operation)
- **Professional:** $25/month (better performance)

## Security

- ✅ `keypair.json` is in `.gitignore` (never committed)
- ✅ Private key stored as environment variable
- ✅ Keypair generated at runtime, not stored in repo
- ⚠️ Never share your `PRIVATE_KEY`
- ⚠️ Use Render's environment variable encryption

## Files Created

- `render.yaml` - Service configuration
- `scripts/start.sh` - Startup script
- `scripts/render-keypair.js` - Keypair generator
- `README_RENDER.md` - Detailed documentation

## Next Steps

1. Deploy to Render
2. Monitor logs for successful startup
3. Verify mining activity
4. Set up alerts for service failures (optional)

## Support

- Render Docs: https://render.com/docs
- Render Status: https://status.render.com
- Check service logs for runtime errors

