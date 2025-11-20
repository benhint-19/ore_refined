# Free Solana RPC Endpoints

## Recommended Free RPCs

### 1. **Solana Foundation Public** (Most Reliable)
```
https://api.mainnet-beta.solana.com
```
- Official Solana endpoint
- Free, no API key required
- Rate limited (may throttle under heavy load)
- Good for development and light usage

### 2. **Pocket Network** (Good Alternative)
```
https://solana.api.pocket.network
```
- Free public endpoint
- No API key required
- Decentralized network
- Better reliability than public endpoint

### 3. **dRPC** (Free Tier Available)
```
https://solana.drpc.org/
```
- Free public endpoint
- No account required for basic usage
- Good performance

### 4. **QuickNode Free Tier** (Best for Production)
- Sign up at https://www.quicknode.com
- Free tier: 100M compute units/month
- More reliable than public endpoints
- Requires account creation

### 5. **Helius Free Tier** (Recommended for Mining)
- Sign up at https://www.helius.dev
- Free tier available
- Optimized for Solana applications
- Better rate limits than public endpoints

## For Your Render Deployment

**Quick Setup (No Signup):**
```
RPC_URL=https://api.mainnet-beta.solana.com
```

**Better Performance (Requires Signup):**
1. Sign up for Helius free tier: https://www.helius.dev
2. Get your RPC URL from dashboard
3. Use that URL in Render environment variables

## Rate Limits

Public endpoints (`api.mainnet-beta.solana.com`) have rate limits:
- ~40 requests per 10 seconds per IP
- May throttle during high network activity
- For continuous mining, consider a paid tier

## Recommendation

For **testing/development**: Use `https://api.mainnet-beta.solana.com`

For **production mining**: Sign up for Helius or QuickNode free tier for better reliability and higher rate limits.

