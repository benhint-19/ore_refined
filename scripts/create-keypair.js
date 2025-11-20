import fs from 'fs';
import bs58 from 'bs58';

// Private key in base58
const privateKey = '5tk76qXqGEuCWp25S9Rsws76WJprwdW15RekPVqUyiFGSpJZDUd5tZQszKXTPAZEjVjVQLehzTaSgFi1Jxf8RdiR';

// Decode the private key
const secretKey = bs58.decode(privateKey);

// Solana keypair format is 64 bytes: 32 bytes secret key + 32 bytes public key
// If we only have the secret key, we need to derive the public key
// For now, we'll create a 64-byte array (secret key + zeros for public key, which will be derived)
// Actually, Solana keypair JSON format is just the 64-byte secret key array
const keypairArray = Array.from(secretKey);

// Write to keypair.json
fs.writeFileSync('keypair.json', JSON.stringify(keypairArray));
console.log('Keypair file created successfully');

