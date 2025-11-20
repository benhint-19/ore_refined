# Vercel Deployment Guide

## Prerequisites

1. Vercel account
2. Vercel CLI installed: `npm i -g vercel`

## Environment Variables

Set these in your Vercel project settings (Dashboard > Project > Settings > Environment Variables):

```
RPC_URL=https://api.mainnet-beta.solana.com
PER_ROUND_DEPLOY_AMOUNT=0.0001
REMAINING_SLOTS=15
ORE_REFINED_RATE=1.3
PRIVATE_KEY=5tk76qXqGEuCWp25S9Rsws76WJprwdW15RekPVqUyiFGSpJZDUd5tZQszKXTPAZEjVjVQLehzTaSgFi1Jxf8RdiR
PUBLIC_KEY=8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b
```

## Important Notes

**Vercel Limitations:**
- Vercel serverless functions have execution time limits (10s free, 60s pro)
- This Rust application runs in a continuous loop, which is incompatible with serverless functions
- The current setup provides the infrastructure, but the actual mining logic needs to run elsewhere

## Deployment Options

### Option 1: Use Vercel for API/Configuration Only
- Deploy the API endpoint to Vercel
- Run the actual Rust binary on a service that supports long-running processes (e.g., Railway, Render, AWS EC2, DigitalOcean)

### Option 2: Implement Mining Logic in Node.js
- Rewrite the mining logic in TypeScript/JavaScript
- Deploy as Vercel serverless functions
- Use Vercel Cron to trigger periodic mining operations

### Option 3: Hybrid Approach
- Keep configuration and API on Vercel
- Deploy Rust binary to a separate service
- Have Vercel API trigger the external service

## Deployment Steps

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Create keypair file:**
   ```bash
   npm run create-keypair
   ```

3. **Deploy to Vercel:**
   ```bash
   vercel
   ```

4. **Set environment variables in Vercel dashboard**

5. **Configure Cron Jobs** (if using Vercel Pro):
   - Go to Project Settings > Cron Jobs
   - The cron job is configured in `vercel.json` to run every minute

## Current Configuration

- **Deploy Amount:** 0.0001 SOL per round
- **Remaining Slots:** 15 (default)
- **ORE Refined Rate:** 1.3 (default)
- **Public Key:** 8BEuH1JdD5wTVJfBh2y6htgjiBUyuZ1keDLdi1FpU39b

## Testing

After deployment, test the endpoint:
```bash
curl https://your-app.vercel.app/api/mining
```

