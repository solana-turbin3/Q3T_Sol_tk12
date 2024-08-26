import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { NftMarketplace } from '../target/types/nft_marketplace';

describe('nft-marketplace', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorMarketplace as Program<NftMarketplace>;

  it('Is initialized!', async () => {
    const name = 'Tims market';
    const fee = 1;
    // Add your test here.
    const tx = await program.methods.initialize(name, fee).rpc();
    console.log('Your transaction signature', tx);
  });
});
