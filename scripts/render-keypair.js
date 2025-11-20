import fs from 'fs';
import bs58 from 'bs58';

// Generate keypair.json from PRIVATE_KEY environment variable for Render
const PRIVATE_KEY = process.env.PRIVATE_KEY;

if (!PRIVATE_KEY) {
  console.error('Error: PRIVATE_KEY environment variable is not set');
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

