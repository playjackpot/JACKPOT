const express = require('express');
const { Connection, PublicKey } = require('@solana/web3.js');

const router = express.Router();
const connection = new Connection('https://api.devnet.solana.com');

// Mock MongoDB or in-memory store (replace with real DB)
let leaderboard = [];

router.get('/', async (req, res) => {
  try {
    // Fetch top 200 players (mock data; integrate with Solana events)
    // In practice, listen for RankUpdateEvent and store in DB
    const players = leaderboard.sort((a, b) => a.rank - b.rank).slice(0, 200);
    res.json(players);
  } catch (error) {
    res.status(500).json({ error: 'Failed to fetch leaderboard' });
  }
});

// Mock event listener for RankUpdateEvent
function updateLeaderboard(event) {
  const { player, new_rank, seeks } = event; // Assume event parsing
  const existing = leaderboard.find(p => p.address === player);
  if (existing) {
    existing.rank = new_rank;
    existing.seeks = seeks;
  } else {
    leaderboard.push({
      address: player,
      rank: new_rank,
      seeks: seeks || 0,
    });
  }
}

module.exports = router;
