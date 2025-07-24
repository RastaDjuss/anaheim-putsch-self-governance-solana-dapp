// FILE: read-keypair.js
const fs = require('fs');
const anchor = require('@coral-xyz/anchor');

const secretKey = JSON.parse(fs.readFileSync('anchor/target/deploy/anaheim-keypair.json', 'utf-8'));
const keypair = anchor.web3.Keypair.fromSecretKey(new Uint8Array(secretKey));

console.log('Program ID:', keypair.publicKey.toBase58());
