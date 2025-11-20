# Render Quick Start

## 1. Push to Git
```bash
git add .
git commit -m "Add Render deployment config"
git push
```

## 2. Deploy on Render

### Using Blueprint (Easiest)
1. Go to https://dashboard.render.com
2. Click "New +" → "Blueprint"
3. Connect your repository
4. Render auto-detects `render.yaml`
5. Set environment variables:
   - `RPC_URL` (your Solana RPC)
   - `PRIVATE_KEY=5tk76qXqGEuCWp25S9Rsws76WJprwdW15RekPVqUyiFGSpJZDUd5tZQszKXTPAZEjVjVQLehzTaSgFi1Jxf8RdiR`
6. Click "Apply"

### Manual Setup
1. Go to https://dashboard.render.com
2. Click "New +" → "Background Worker"
3. Connect repository
4. Settings:
   - Build: `cargo build --release`
   - Start: `./scripts/start.sh`
5. Add environment variables (see above)
6. Deploy

## 3. Monitor
- View logs: Dashboard → Service → Logs
- Check status: Dashboard → Service → Metrics

## Environment Variables

**Required:**
- `RPC_URL` - Solana RPC endpoint
- `PRIVATE_KEY` - Your base58 private key

**Optional (defaults in render.yaml):**
- `PER_ROUND_DEPLOY_AMOUNT=0.0001`
- `REMAINING_SLOTS=15`
- `ORE_REFINED_RATE=1.3`

## Your Configuration
- Deploy: **0.0001 SOL** per round
- Public Key: **8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b**

Done! Your miner will start automatically.

