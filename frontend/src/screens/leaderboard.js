import React, { useEffect, useState } from 'react';
import { View, Text, FlatList, StyleSheet } from 'react-native';

const Leaderboard = () => {
  const [players, setPlayers] = useState([]);

  useEffect(() => {
    // Fetch leaderboard from backend
    fetch('http://localhost:3000/api/leaderboard')
      .then(response => response.json())
      .then(data => setPlayers(data))
      .catch(error => console.error('Leaderboard fetch error:', error));
  }, []);

  const renderPlayer = ({ item }) => (
    <View style={styles.player}>
      <Text style={styles.rank}>{item.rank}</Text>
      <Text style={styles.address}>{item.address.slice(0, 8)}...</Text>
      <Text style={styles.seeks}>{item.seeks} seeks</Text>
    </View>
  );

  return (
    <View style={styles.container}>
      <Text style={styles.title}>Leaderboard</Text>
      <FlatList
        data={players}
        renderItem={renderPlayer}
        keyExtractor={item => item.address}
      />
    </View>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, padding: 20 },
  title: { fontSize: 24, fontWeight: 'bold', marginBottom: 20 },
  player: { flexDirection: 'row', padding: 10, borderBottomWidth: 1 },
  rank: { width: 50, fontWeight: 'bold' },
  address: { flex: 1 },
  seeks: { width: 100, textAlign: 'right' },
});

export default Leaderboard;
