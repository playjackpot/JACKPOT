#!/bin/bash
cd contracts/jackpot_program
anchor build
anchor deploy --provider.cluster devnet
