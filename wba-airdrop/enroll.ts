import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { Program, Wallet, AnchorProvider } from '@coral-xyz/anchor';
import { IDL, WbaPrereq } from './programs/wba_prereq';
import wallet from './wba-wallet.json';

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection('https://api.devnet.solana.com');

const github = Buffer.from('timknapp12', 'utf8');

const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: 'confirmed',
});

const program: Program<WbaPrereq> = new Program(IDL, provider);
console.log('keypair.publicKey', keypair.publicKey);

const enrollment_seeds = [Buffer.from('prereq'), keypair.publicKey.toBuffer()];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
  enrollment_seeds,
  program.programId
);
(async () => {
  try {
    const txhash = await program.methods
      .complete(github)
      .accounts({
        signer: keypair.publicKey,
        prereq: enrollment_key,
        system_program: PublicKey.default,
      })
      .signers([keypair])
      .rpc();
    console.log(`Success! Check out your TX here:
        https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();

// success: https://explorer.solana.com/tx/52by7aK16oxngCpHTz7jHXLJxGscwVN2FcKscwZ4cCNn9YsvddfxyaBXCKaEbMDM2R8e4ts8PogdyrN5RT9mMAQp?cluster=devnet
