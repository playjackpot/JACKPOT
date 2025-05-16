import React, { useState, useEffect } from 'react';
import { View, Button, Text } from 'react-native';
import MapComponent from '../components/MapView';
import HintDisplay from '../components/HintDisplay';
import { Connection, PublicKey } from '@solana/web3.js';

const SeekScreen = () => {
  const [hides, setHides] = useState([
    { id: '1', latitude: 37.78825, longitude: -122.4324, title: 'Treasure 1' },
  ]);
  const [hint, setHint] = useState(null);
  const [mapRegion, setMapRegion] = useState({
    latitude: 37.78825,
    longitude: -122.4324,
    latitudeDelta: 0.0922,
    longitudeDelta: 0.0421,
  });
  const connection = new Connection('https://api.devnet.solana.com');

  const handleSeek = async (hide) => {
    try {
      // Validate geolocation
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

      // Call seek instruction with hint
      const programId = new PublicKey('YourProgramIDHere');
      // Mock transaction with use_hint = true
      // Listen for HintEvent (simplified)
      setHint({
        hint_type: 'BroadArea',
        data: JSON.stringify({
          type: 'broad_area',
          center: { lat: hide.latitude, lng: hide.longitude },
          radius_m: 1000,
        }),
      });
    } catch (error) {
      console.error('Seek error:', error);
    }
  };

  return (
    <View style={{ flex: 1 }}>
      <MapComponent
        hides={hides}
        onSelectHide={handleSeek}
        region={mapRegion}
      >
        <HintDisplay hint={hint} mapRegion={mapRegion} />
      </MapComponent>
      <Button title="Refresh Hides" onPress={() => {}} />
    </View>
  );
};

export default SeekScreen;
