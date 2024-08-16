import wallet from '../wba-wallet.json';
import {
  createProgrammableNft,
  mplTokenMetadata,
} from '@metaplex-foundation/mpl-token-metadata';
import {
  createGenericFile,
  generateSigner,
  percentAmount,
  signerIdentity,
  createSignerFromKeypair,
} from '@metaplex-foundation/umi';
import { irysUploader } from '@metaplex-foundation/umi-uploader-irys';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { base58 } from '@metaplex-foundation/umi/serializers';
import { readFile } from 'fs/promises';
import path from 'path';

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));
umi.use(mplTokenMetadata());

const name = 'N8HughsWBA';
const symbol = 'N8';

const createNft = async () => {
  // 1. load image
  const imageFile = await readFile(
    path.join(__dirname, './images/nate-wba.jpeg')
  );
  // 2. Convert image to generic file.
  const umiImageFile = createGenericFile(imageFile, 'nate image', {
    tags: [{ name: 'Content-Type', value: 'image/png' }],
  });
  // 3. To get the uri we want we can call index [0] in the array.
  const imageUri = await umi.uploader.upload([umiImageFile]).catch((err) => {
    throw new Error(err);
  });

  const image = imageUri[0];

  // 4. add metada
  const metadata = {
    name: name,
    symbol: symbol,
    description: 'This is an NFT of WBA cofounder Nate Hughs',
    image,
    // external_url: 'https://example.com',
    attributes: [{ trait_type: 'Rarity', value: 'Unique' }],
    properties: {
      files: [
        {
          uri: image,
          type: 'image/png',
        },
      ],
      category: 'image',
    },
  };

  // 5. Call upon umi's uploadJson function to upload our metadata to Arweave via Irys.
  const metadataUri = await umi.uploader.uploadJson(metadata).catch((err) => {
    throw new Error(err);
  });

  // 6. We generate a signer for the NFT
  const nftSigner = generateSigner(umi);

  // 7. Decide on a ruleset for the NFT.
  // Metaplex ruleset - publicKey("eBJLFYPxJmMGKuFwpDWkzxZeUrad92kZRC5BJLpzyT9")
  // Compatability ruleset - publicKey("AdH2Utn6Fus15ZhtenW4hZBQnvtLgM1YCW2MfVp7pYS5")
  const ruleset = null; // or set a publicKey from above

  // decide if you are using createNft() or createProgramableNft()
  // you can enforce royalties in pNFTs
  // 8. create NFT
  const tx = await createProgrammableNft(umi, {
    mint: nftSigner,
    sellerFeeBasisPoints: percentAmount(5.5),
    name: name,
    uri: metadataUri,
    ruleSet: ruleset,
  }).sendAndConfirm(umi);

  // 9. finally we can deserialize the signature that we can check on chain.
  const signature = base58.deserialize(tx.signature);

  console.log(
    `Successfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`
  );

  console.log(
    'check out the nft here: ',
    `https://explorer.solana.com/address/${nftSigner.publicKey}?cluster=devnet`
  );
};

createNft();
