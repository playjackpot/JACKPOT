import React, { useState } from 'react';
import { Button, Text } from 'react-native';
import { Connection, PublicKey } from '@solana/web3.js';

const WalletConnect = () => {
  const [publicKey, setPublicKey] = useState(null);
  const connection = new Connection('https://api.devnet.solana.com');

  const connectWallet = async () => {
    try {
      const provider = window.solana;
      if (provider && provider.isPhantom) {
        await provider.connect();
        setPublicKey(new PublicKey(provider.publicKey.toString()));
      } else {
        alert('Please install Phantom wallet');
      }
    } catch (error) {
      console.error('Wallet connection error:', error);
    }
  };

  return (
    <>
      <Button title="Connect Wallet" onPress={connectWallet} />
      {publicKey && <Text>Connected: {publicKey.toString()}</Text>}
    </>
  );
};

export default WalletConnect;
