# JACKPOT
A geolocation-based play-to-earn (P2E) game on Solana, blending real-world exploration with blockchain rewards. Players hide and seek virtual treasures (PrizePals NFTs and treasure boxes) to earn $SEEK, SOL, and BTC.

## Overview
- **Blockchain**: Solana
- **Token**: $SEEK (SPL Token, deflationary)
- **NFTs**: PrizePals (100,000, Metaplex)
- **Rewards**: $SEEK, SOL, and BTC via BTC Rewards Wallet and BTC Drop Wallet
- **Frontend**: React Native mobile app
- **Backend**: Node.js with geolocation APIs

## Setup
### Contracts
1. Install Rust and Anchor: `cargo install --git https://github.com/project-serum/anchor anchor-cli`.
2. Navigate to `contracts/jackpot_program/`.
3. Build: `anchor build`.
4. Deploy to devnet: `anchor deploy`.

### Frontend
1. Navigate to `frontend/`.
2. Install dependencies: `npm install`.
3. Run: `npx react-native run-android` or `run-ios`.

### Backend
1. Navigate to `backend/`.
2. Install dependencies: `npm install`.
3. Start server: `node src/server.js`.

## License
MIT
