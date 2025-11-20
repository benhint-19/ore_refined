# Render Deployment Guide

## Quick Start

### Option 1: Using render.yaml (Recommended)

1. **Push your code to GitHub/GitLab/Bitbucket**

2. **Connect to Render:**
   - Go to [Render Dashboard](https://dashboard.render.com)
   - Click "New +" → "Blueprint"
   - Connect your repository
   - Render will automatically detect `render.yaml`

3. **Set Environment Variables:**
   - In the Render dashboard, go to your service
   - Navigate to "Environment" tab
   - Add the following variables:

```
RPC_URL=https://api.mainnet-beta.solana.com
PRIVATE_KEY=5tk76qXqGEuCWp25S9Rsws76WJprwdW15RekPVqUyiFGSpJZDUd5tZQszKXTPAZEjVjVQLehzTaSgFi1Jxf8RdiR
```

**Note:** The following are already set in `render.yaml` but can be overridden:
- `PER_ROUND_DEPLOY_AMOUNT=0.0001`
- `REMAINING_SLOTS=15`
- `ORE_REFINED_RATE=1.3`
- `PUBLIC_KEY=8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b`

4. **Deploy:**
   - Render will automatically build and deploy
   - Monitor logs in the Render dashboard

### Option 2: Manual Setup

1. **Create a new Worker service:**
   - Go to Render Dashboard
   - Click "New +" → "Background Worker"
   - Connect your repository

2. **Configure the service:**
   - **Name:** `ore-refined-miner`
   - **Environment:** `Rust`
   - **Build Command:** `cargo build --release`
   - **Start Command:** `./scripts/start.sh`
   - **Plan:** Choose based on your needs (Free tier available for testing)

3. **Set Environment Variables:**
   - `RPC_URL` - Your Solana RPC endpoint (recommended: Helius)
   - `PRIVATE_KEY` - Your base58 private key
   - `PER_ROUND_DEPLOY_AMOUNT=0.0001`
   - `REMAINING_SLOTS=15`
   - `ORE_REFINED_RATE=1.3`
   - `PUBLIC_KEY=8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b`

4. **Deploy:**
   - Click "Create Background Worker"
   - Render will build and start the service

## Configuration

### Current Settings

- **Deploy Amount:** 0.0001 SOL per round
- **Remaining Slots:** 15 (deploy in last 15 slots of each round)
- **ORE Refined Rate:** 1.3 (accept up to 1.3 ORE per unclaimed ORE)
- **Public Key:** 8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b

### RPC Providers

Recommended RPC providers:
- **Helius** - https://www.helius.dev (recommended)
- **QuickNode** - https://www.quicknode.com
- **Solana Mainnet** - https://api.mainnet-beta.solana.com (public, rate-limited)

## Monitoring

- **Logs:** View real-time logs in Render dashboard
- **Metrics:** Monitor CPU, memory, and network usage
- **Alerts:** Set up alerts for service failures

## Troubleshooting

### Build Fails

- Ensure Rust toolchain is available (Render auto-detects Rust projects)
- Check that all dependencies in `Cargo.toml` are accessible
- Review build logs for specific errors

### Service Crashes

- Check logs for error messages
- Verify `PRIVATE_KEY` is set correctly
- Ensure `RPC_URL` is accessible
- Check wallet has sufficient SOL balance

### Keypair Issues

- Verify `PRIVATE_KEY` is in base58 format
- Ensure the keypair file is generated correctly (check startup logs)
- The script `scripts/render-keypair.js` generates `keypair.json` at startup

## Cost Considerations

- **Free Tier:** Limited hours per month, suitable for testing
- **Starter Plan:** $7/month - Good for continuous operation
- **Professional Plan:** $25/month - Better performance and reliability

## Security Notes

- Never commit `keypair.json` to git (already in `.gitignore`)
- Use Render's environment variables for sensitive data
- Consider using Render's secret management features
- Regularly rotate keys if exposed

## Files

- `render.yaml` - Render service configuration
- `scripts/start.sh` - Startup script that generates keypair and runs miner
- `scripts/render-keypair.js` - Script to generate keypair from env var
- `.gitignore` - Excludes keypair.json and other sensitive files

## Support

For issues:
1. Check Render service logs
2. Verify environment variables are set correctly
3. Ensure wallet has sufficient SOL balance
4. Check RPC endpoint is accessible and not rate-limited

