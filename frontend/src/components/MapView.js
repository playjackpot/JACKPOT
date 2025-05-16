import React from 'react';
import { StyleSheet } from 'react-native';
import MapView, { Marker } from 'react-native-maps';

const MapComponent = ({ hides, onSelectHide }) => {
  return (
    <MapView
      style={styles.map}
      initialRegion={{
        latitude: 37.78825,
        longitude: -122.4324,
        latitudeDelta: 0.0922,
        longitudeDelta: 0.0421,
      }}
    >
      {hides.map(hide => (
        <Marker
          key={hide.id}
          coordinate={{ latitude: hide.latitude, longitude: hide.longitude }}
          title={hide.title}
          onPress={() => onSelectHide(hide)}
        />
      ))}
    </MapView>
  );
};

const styles = StyleSheet.create({
  map: {
    flex: 1,
  },
});

export default MapComponent;
