import React, { useState } from 'react';
import { View, Button, Text } from 'react-native';
import MapComponent from '../components/MapView';
import { Connection, PublicKey, Transaction } from '@solana/web3.js';

const SeekScreen = () => {
  const [hides, setHides] = useState([
    { id: '1', latitude: 37.78825, longitude: -122.4324, title: 'Treasure 1' },
  ]);
  const connection = new Connection('https://api.devnet.solana.com');

  const handleSeek = async (hide) => {
    try {
      // Call backend to validate geolocation
      const response = await fetch('http://localhost:3000/api/geolocation/validate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          playerCoords: { lat: 37.78825, lng: -122.4324 }, // Mock GPS
          hideCoords: { lat: hide.latitude, lng: hide.longitude },
        }),
      });
      const { valid } = await response.json();
      if (!valid) {
        alert('Too far from treasure!');
        return;
      }

      // Create transaction to call seek instruction
      const programId = new PublicKey('YourProgramIDHere');
      // Add transaction logic (simplified)
      alert('Seek transaction sent!');
    } catch (error) {
      console.error('Seek error:', error);
    }
  };

  return (
    <View style={{ flex: 1 }}>
      <MapComponent hides={hides} onSelectHide={handleSeek} />
      <Button title="Refresh Hides" onPress={() => {}} />
    </View>
  );
};

export default SeekScreen;
