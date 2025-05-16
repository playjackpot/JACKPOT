const express = require('express');
const { Connection } = require('@solana/web3.js');

const router = express.Router();
const connection = new Connection('https://api.devnet.solana.com');

router.get('/calculate', async (req, res) => {
  try {
    // Mock game state (replace with actual on-chain query)
    const seekPool = 150_000_000_000_000; // 150M $SEEK
    const solPool = 20_000_000_000_000; // 20,000 SOL

    const seekReward = seekPool >= 200_000_000_000_000
      ? 25_000_000_000
      : seekPool >= 150_000_000_000_000
      ? 15_000_000_000
      : 5_000_000_000;

    const solReward = solPool > 20_000_000_000_000
      ? 0.01
      : solPool > 10_000_000_000_000
      ? 0.005
      : 0.0025;

    res.json({ seekReward, solReward });
  } catch (error) {
    res.status(500).json({ error: 'Failed to calculate rewards' });
  }
});

module.exports = router;
