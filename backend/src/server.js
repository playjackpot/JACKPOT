const express = require('express');
const geolocationRoutes = require('./api/geolocation');
const rewardsRoutes = require('./api/rewards');

const app = express();
app.use(express.json());

app.use('/api/geolocation', geolocationRoutes);
app.use('/api/rewards', rewardsRoutes);

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => {
  console.log(`Server running on port ${PORT}`);
});
