const mongoose = require('mongoose');

const playerSchema = new mongoose.Schema({
  address: { type: String, required: true, unique: true },
  seeks: { type: Number, default: 0 },
  hides: { type: Number, default: 0 },
  rank: { type: Number, default: 201 },
  rewards: {
    seek: { type: Number, default: 0 },
    sol: { type: Number, default: 0 },
    btc: { type: Number, default: 0 },
  },
});

const Player = mongoose.model('Player', playerSchema);

const connectDB = async () => {
  try {
    await mongoose.connect(process.env.MONGO_URI, {
      useNewUrlParser: true,
      useUnifiedTopology: true,
    });
    console.log('MongoDB connected');
  } catch (error) {
    console.error('MongoDB connection error:', error);
    process.exit(1);
  }
};

const updatePlayer = async (address, data) => {
  try {
    await Player.findOneAndUpdate(
      { address },
      { $set: data },
      { upsert: true, new: true }
    );
  } catch (error) {
    console.error('Update player error:', error);
  }
};

const getLeaderboard = async () => {
  try {
    return await Player.find()
      .sort({ rank: 1 })
      .limit(200)
      .select('address seeks rank');
  } catch (error) {
    console.error('Get leaderboard error:', error);
    return [];
  }
};

module.exports = { connectDB, updatePlayer, getLeaderboard };
