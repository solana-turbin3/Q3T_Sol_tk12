import { Keypair } from '@solana/web3.js';
import bs58 from 'bs58';

function deriveKeypairFromSecretKey(secretKeyBase58: string) {
  try {
    // Decode the secret key from Base58
    const secretKey = bs58.decode(secretKeyBase58);

    // Create a Keypair from the secret key
    const keypair = Keypair.fromSecretKey(secretKey);

    // Extract the public key and secret key as Uint8Array
    const publicKey = keypair.publicKey.toBytes();
    const secretKeyArray = keypair.secretKey;

    console.log('Public Key (Uint8Array):', publicKey);
    console.log('Secret Key (Uint8Array):', secretKeyArray);

    return { publicKey, secretKey: secretKeyArray };
  } catch (err) {
    console.error('Error deriving keys:', err);
  }
}

// Example secret key in Base58 (replace with your actual secret key)
const secretKeyBase58 = 'secret key';

const keys = deriveKeypairFromSecretKey(secretKeyBase58);
if (keys) {
  console.log('Derived Public Key (Base58):', bs58.encode(keys.publicKey));
  console.log('Derived Secret Key (Base58):', bs58.encode(keys.secretKey));
}
