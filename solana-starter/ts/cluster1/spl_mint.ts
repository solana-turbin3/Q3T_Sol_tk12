import { Keypair, PublicKey, Connection, Commitment } from '@solana/web3.js';
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from '../wba-wallet.json';

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = 'confirmed';
const connection = new Connection('https://api.devnet.solana.com', commitment);

const token_decimals = 1_000_000;

// Mint address
const mintAddress = new PublicKey(
  '64kivSq7dzejdRmEZvVGLFVdAow8FvTcCHSKrpx3McBu'
);

(async () => {
  try {
    // Create an ATA
    const ata = await getOrCreateAssociatedTokenAccount(
      connection,
      keypair,
      mintAddress,
      keypair.publicKey
    );
    console.log(`Your ata is: ${ata.address.toBase58()}`);
    // 9bs51cnNE9t9HFirzaDi6AS5aCrpWVxXaDHVoD131SBf

    // Mint to ATA
    const mintTx = await mintTo(
      connection,
      keypair,
      mintAddress,
      ata.address,
      keypair,
      1000 * token_decimals
    );
    console.log(
      `Success! Check out your tx here: https://explorer.solana.com/tx/${mintTx}?cluster=devnet`
    );
    // mintTx = '5fnJEwaABLrt37PmGzy5KqRsoED3YUjbejR3WjYrN3pXHX2irKBbRLz4UKBKo9UxHEJ4Smfc8jvunq6hkoVdEeA4'
    // https://explorer.solana.com/tx/5fnJEwaABLrt37PmGzy5KqRsoED3YUjbejR3WjYrN3pXHX2irKBbRLz4UKBKo9UxHEJ4Smfc8jvunq6hkoVdEeA4?cluster=devnet
  } catch (error) {
    console.log(`Oops, something went wrong: ${error}`);
  }
})();
