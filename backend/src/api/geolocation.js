const express = require('express');
const haversine = require('haversine-distance');

const router = express.Router();

router.post('/validate', (req, res) => {
  const { playerCoords, hideCoords } = req.body;
  const distance = haversine(
    { latitude: playerCoords.lat, longitude: playerCoords.lng },
    { latitude: hideCoords.lat, longitude: hideCoords.lng }
  );
  const valid = distance <= 100; // 100m radius
  res.json({ valid });
});

module.exports = router;
