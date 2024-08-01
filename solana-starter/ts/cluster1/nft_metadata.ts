import wallet from '../wba-wallet.json';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from '@metaplex-foundation/umi';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {
    // Follow this JSON structure
    // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

    const image =
      'https://arweave.net/DW-3_1ABpidIv39QkTJbo9DFl5E2P9hrtIwX4Q07pys';
    const metadata = {
      name: 'WBA - Tim',
      symbol: 'TK12R',
      description: 'Rugging NFT',
      image,
      attributes: [{ trait_type: 'Rarity', value: 'Unique' }],
      properties: {
        files: [
          {
            type: 'image/png',
            uri: image,
          },
        ],
      },
      creators: [],
    };
    const myUri = image; // This is your image URI
    console.log('Your image URI: ', myUri);
    // https://arweave.net/DW-3_1ABpidIv39QkTJbo9DFl5E2P9hrtIwX4Q07pys
  } catch (error) {
    console.log('Oops.. Something went wrong', error);
  }
})();
