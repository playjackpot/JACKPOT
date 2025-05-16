import React from 'react';
import { StyleSheet } from 'react-native';
import MapView, { Circle, Marker } from 'react-native-maps';

const HintDisplay = ({ hint, mapRegion }) => {
  if (!hint) return null;

  const parsedHint = JSON.parse(hint.data);

  switch (hint.hint_type) {
    case 'BroadArea':
    case 'SmallArea':
      return (
        <Circle
          center={{
            latitude: parsedHint.center.lat,
            longitude: parsedHint.center.lng,
          }}
          radius={parsedHint.radius_m}
          strokeColor="#FF0000"
          fillColor="rgba(255, 0, 0, 0.2)"
        />
      );
    case 'Direction':
      return (
        <Marker
          coordinate={{
            latitude: mapRegion.latitude,
            longitude: mapRegion.longitude,
          }}
          title={`Hint: ${parsedHint.direction}, ~${parsedHint.distance_m}m`}
          pinColor="blue"
        />
      );
    case 'Precise':
      return (
        <Marker
          coordinate={{
            latitude: parsedHint.center.lat,
            longitude: parsedHint.center.lng,
          }}
          title="Precise Hint"
          pinColor="gold"
        />
      );
    default:
      return null;
  }
};

export default HintDisplay;
