import wallet from '../wba-wallet.json';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from '@metaplex-foundation/umi';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';
import { readFile } from 'fs/promises';
import path from 'path';

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {
    //1. Load image
    const imageFile = await readFile(
      path.join(__dirname, './images/generug2.png')
    );
    //2. Convert image to generic file.
    const umiImageFile = createGenericFile(imageFile, 'rug image', {
      tags: [{ name: 'Content-Type', value: 'image/png' }],
    });
    //3. Upload image
    const imageUri = await umi.uploader.upload([umiImageFile]).catch((err) => {
      throw new Error(err);
    });
    console.log('Your image URI: ', imageUri[0]);
    // generug https://arweave.net/DW-3_1ABpidIv39QkTJbo9DFl5E2P9hrtIwX4Q07pys
    // generug2: https://arweave.net/W3gogC1dAqi6YqYYdQFuRGZe2v2S1IMIPztuk91Udlc
  } catch (error) {
    console.log('Oops.. Something went wrong', error);
  }
})();
