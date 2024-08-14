import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { NftStaking } from '../target/types/nft_staking';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
  createNft,
  findMasterEditionPda,
  findMetadataPda,
  mplTokenMetadata,
  verifyCollection,
  verifySizedCollectionItem,
} from '@metaplex-foundation/mpl-token-metadata';
import {
  KeypairSigner,
  PublicKey,
  createSignerFromKeypair,
  generateSigner,
  keypairIdentity,
  percentAmount,
} from '@metaplex-foundation/umi';
import NodeWallet from '@coral-xyz/anchor/dist/cjs/nodewallet';

describe('nft-staking', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NftStaking as Program<NftStaking>;

  const umi = createUmi(provider.connection);

  const payer = provider.wallet as NodeWallet;

  let nftMint: KeypairSigner;
  let collectionMint: KeypairSigner;

  let stakeAccount = anchor.web3.PublicKey;

  const creatorWallet = umi.eddsa.createKeypairFromSecretKey(
    new Uint8Array(payer.payer.secretKey)
  );
  const creator = createSignerFromKeypair(umi, creatorWallet);

  umi.use(keypairIdentity(creator));
  umi.use(mplTokenMetadata());

  const config = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('config')],
    program.programId
  )[0];
  const rewardsMint = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('rewards')],
    program.programId
  )[0];

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.methods.init_config().rpc();
    console.log('Your transaction signature', tx);
  });
});
