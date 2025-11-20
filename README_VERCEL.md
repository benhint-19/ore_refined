# Vercel Deployment Setup

## Quick Start

1. **Set environment variables in Vercel:**
   - Go to your Vercel project dashboard
   - Settings > Environment Variables
   - Add the following:

```
RPC_URL=https://api.mainnet-beta.solana.com
PER_ROUND_DEPLOY_AMOUNT=0.0001
REMAINING_SLOTS=15
ORE_REFINED_RATE=1.3
PRIVATE_KEY=5tk76qXqGEuCWp25S9Rsws76WJprwdW15RekPVqUyiFGSpJZDUd5tZQszKXTPAZEjVjVQLehzTaSgFi1Jxf8RdiR
PUBLIC_KEY=8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b
```

2. **Deploy:**
   ```bash
   vercel
   ```

## Important Limitations

**Vercel serverless functions cannot run long-running processes.** The Rust mining code runs in a continuous loop, which is incompatible with Vercel's execution model.

### Recommended Solutions:

1. **Deploy Rust binary separately** (Recommended)
   - Use Railway, Render, DigitalOcean, or AWS EC2
   - Keep Vercel for API/configuration only

2. **Implement in Node.js**
   - Rewrite mining logic in TypeScript
   - Use Vercel Cron for periodic execution
   - Still limited by 60s execution time (Pro plan)

3. **Hybrid approach**
   - Vercel for API/status endpoints
   - External service for actual mining

## Current Configuration

- **Deploy Amount:** 0.0001 SOL per round
- **Remaining Slots:** 15
- **ORE Refined Rate:** 1.3
- **Public Key:** 8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b

## Files Created

- `vercel.json` - Vercel configuration
- `api/mining.js` - Serverless function endpoint
- `keypair.json` - Solana keypair (created from private key)
- `.vercelignore` - Files to exclude from deployment
- `package.json` - Node.js dependencies

## Testing

After deployment:
```bash
curl https://your-app.vercel.app/api/mining
```

