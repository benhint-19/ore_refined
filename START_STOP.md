# How to Start and Stop the Miner on Render

## Starting the Miner

### First Time Deployment

1. **Deploy via Blueprint:**
   - Go to https://dashboard.render.com
   - Click "New +" → "Blueprint"
   - Connect your repository
   - Set environment variables (PRIVATE_KEY, RPC_URL if needed)
   - Click "Apply"
   - Service starts automatically

2. **Manual Deployment:**
   - Go to https://dashboard.render.com
   - Click "New +" → "Background Worker"
   - Configure and deploy
   - Service starts automatically

### Restarting/Restarting After Stop

**Method 1: Via Dashboard (Easiest)**
1. Go to Render Dashboard
2. Click on your service: `ore-refined-miner`
3. Click **"Manual Deploy"** → **"Deploy latest commit"**
   - OR click **"Restart"** button (if service is running)

**Method 2: Suspend/Resume**
1. Dashboard → Your Service
2. Click **"Suspend"** to stop
3. Click **"Resume"** to start again

**Method 3: Via Render CLI** (if installed)
```bash
render services:restart <service-id>
```

## Stopping the Miner

### Suspend Service (Recommended)
1. Go to Render Dashboard
2. Click on your service: `ore-refined-miner`
3. Click **"Suspend"** button
4. Service stops immediately
5. No data loss, can resume anytime

### Manual Deploy (Stop by deploying empty commit)
- Not recommended - use Suspend instead

### Delete Service (Permanent)
1. Dashboard → Your Service → Settings
2. Scroll to bottom
3. Click **"Delete Service"**
4. ⚠️ This permanently removes the service

## Checking Status

### View Service Status
- **Dashboard** → Your Service
- Status indicators:
  - 🟢 **Live** - Running
  - 🟡 **Suspended** - Stopped
  - 🔴 **Build Failed** - Error

### View Logs
1. Dashboard → Your Service
2. Click **"Logs"** tab
3. See real-time output:
   - `Starting ORE Refined miner...`
   - `round_id: X slot_left: Y`
   - Transaction confirmations

### Check if Miner is Active
Look for these log messages:
- `Starting ORE Refined miner...`
- `round_id: ... slot_left: ...`
- `Transaction sent: ...`
- `wallet: ... sol:X.XX`

## Quick Reference

| Action | Steps |
|--------|-------|
| **Start** | Dashboard → Service → Resume / Restart |
| **Stop** | Dashboard → Service → Suspend |
| **Restart** | Dashboard → Service → Restart |
| **View Logs** | Dashboard → Service → Logs |
| **Check Status** | Dashboard → Service (status badge) |

## Troubleshooting

### Service Won't Start
1. Check **Logs** for errors
2. Verify **Environment Variables** are set
3. Check **Build** succeeded (green checkmark)
4. Ensure service is not **Suspended**

### Service Keeps Stopping
1. Check **Logs** for crash errors
2. Verify `PRIVATE_KEY` is correct
3. Check `RPC_URL` is accessible
4. Ensure wallet has SOL balance
5. Check **Metrics** for resource limits

### Can't See Logs
- Wait a few seconds for logs to appear
- Refresh the page
- Check service is not suspended

## Cost Control

**Free Tier:**
- 750 hours/month
- Service stops when hours exhausted
- Resets monthly

**To Save Hours:**
- Suspend when not mining
- Resume when ready to mine again

## Best Practices

1. **Monitor regularly** - Check logs daily
2. **Suspend when not needed** - Save free tier hours
3. **Set up alerts** - Get notified of failures
4. **Keep environment variables secure** - Never share PRIVATE_KEY

