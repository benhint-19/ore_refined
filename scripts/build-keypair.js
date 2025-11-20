import fs from 'fs';
import bs58 from 'bs58';

// This script generates keypair.json from the PRIVATE_KEY environment variable
// Run this during Vercel build: vercel-build script in package.json

const PRIVATE_KEY = process.env.PRIVATE_KEY || '5tk76qXqGEuCWp25S9Rsws76WJprwdW15RekPVqUyiFGSpJZDUd5tZQszKXTPAZEjVjVQLehzTaSgFi1Jxf8RdiR';

if (!PRIVATE_KEY) {
  console.error('PRIVATE_KEY environment variable is not set');
  process.exit(1);
}

try {
  const secretKey = bs58.decode(PRIVATE_KEY);
  const keypairArray = Array.from(secretKey);
  
  if (keypairArray.length !== 64) {
    console.warn(`Warning: Keypair length is ${keypairArray.length}, expected 64`);
  }
  
  fs.writeFileSync('keypair.json', JSON.stringify(keypairArray));
  console.log('✓ Keypair file created successfully');
} catch (error) {
  console.error('Error creating keypair file:', error.message);
  process.exit(1);
}

