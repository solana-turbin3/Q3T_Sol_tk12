import { Commitment, Connection, Keypair, PublicKey } from '@solana/web3.js';
import wallet from '../wba-wallet.json';
import { getOrCreateAssociatedTokenAccount, transfer } from '@solana/spl-token';

// We're going to import our keypair from the wallet file
const fromKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const toKeypair = Keypair.generate();
console.log('toKeypair.publicKey', toKeypair.publicKey.toBase58()); // CcNe4jbdkqGJ9axynDZ2ymdh6ua3r1XTyP1B2ZgnKZj8

//Create a Solana devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection('https://api.devnet.solana.com', commitment);

// Mint address
const mintAddress = new PublicKey(
  '64kivSq7dzejdRmEZvVGLFVdAow8FvTcCHSKrpx3McBu'
);

// Recipient address -> already created above
// const toAddress = new PublicKey('<publicKey>');

(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const fromAta = await getOrCreateAssociatedTokenAccount(
      connection,
      fromKeypair,
      mintAddress,
      fromKeypair.publicKey
    );
    // Get the token account of the toWallet address, and if it does not exist, create it
    const toAta = await getOrCreateAssociatedTokenAccount(
      connection,
      fromKeypair,
      mintAddress,
      toKeypair.publicKey
    );
    // Transfer the new token to the "toTokenAccount" we just created
    const txSignature = await transfer(
      connection,
      fromKeypair,
      fromAta.address,
      toAta.address,
      fromKeypair,
      1e6
    );
    // this is the address that will be shown on block explorer - not the user's wallet publicKey
    console.log('toAta.address', toAta.address.toBase58());
    console.log(
      `Successful tx at: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`
    );
    // https://explorer.solana.com/tx/zf1EQkkzkGky2uvAvmcN49JZSzjr9VgM1CLySmXpJtemoQtVQAZZZN4Nk9qfAd18MUkmTxz5xU8rYddV6BaRDXx?cluster=devnet
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
