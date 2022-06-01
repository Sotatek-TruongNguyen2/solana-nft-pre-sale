import * as anchor from "@project-serum/anchor";
import * as spl from '@solana/spl-token';
import { Program, Wallet } from "@project-serum/anchor";
import { NftPresale } from "../target/types/nft_presale";
import { LAMPORTS_PER_SOL } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, createMint, createMintToInstruction, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token' // IGNORE THESE ERRORS IF ANY

const { SystemProgram } = anchor.web3

type Keypair = anchor.web3.Keypair;
type PublicKey = anchor.web3.PublicKey;

describe("nft-presale", () => {
  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );
  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as Wallet;
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.NftPresale as Program<NftPresale>;

  let preSaleMarket: Keypair;
  let preSaleTreasury: PublicKey;
  let tokenA: PublicKey;
  let mintAuthority: Keypair;

  beforeEach(async () => {
    mintAuthority = anchor.web3.Keypair.generate();

    const signature = await program.provider.connection.requestAirdrop(mintAuthority.publicKey, LAMPORTS_PER_SOL);
    await program.provider.connection.confirmTransaction(signature, 'finalized');

    console.log("FINISH AIRDROP");

    tokenA = await createMint(
      program.provider.connection,
      mintAuthority,
      wallet.publicKey,
      wallet.publicKey,
      9
    )
  })

  it("Initialized", async () => {
    preSaleMarket = anchor.web3.Keypair.generate();

    const [preSaleTreasuryPubkey] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("treasury"),
        preSaleMarket.publicKey.toBuffer()
      ],
      program.programId
    )

    preSaleTreasury = preSaleTreasuryPubkey;

    const currentDate = Math.floor(new Date().getTime() / 1000);

    const tx = await program.methods.initializePreSaleMarket(
      "After Earth",
      "AFE",
      new anchor.BN(`${currentDate}`),
      new anchor.BN(`${currentDate + 30000}`),
      100,
      new anchor.BN(`${10 * (10 ** 9)}`),
    ).accounts({
      acceptPaymentToken: tokenA,
      preSaleMarket: preSaleMarket.publicKey,
      treasury: preSaleTreasuryPubkey,
      authority: wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    }).signers([preSaleMarket])
    .rpc();

    console.log("Initialize tx: " + tx);
  })

  it("NFT minting!", async () => {
    const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(
      MINT_SIZE
    );

    const getMetadata = async (
      mint: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const getMasterEdition = async (
      mint: anchor.web3.PublicKey
    ): Promise<anchor.web3.PublicKey> => {
      return (
        await anchor.web3.PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
            Buffer.from("edition"),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
    const NftTokenAccount = await getAssociatedTokenAddress(
      mintKey.publicKey,
      wallet.publicKey
    );
    const buyerTokenAccount = await getAssociatedTokenAddress(
      tokenA,
      wallet.publicKey
    )

    console.log("NFT Account: ", NftTokenAccount.toBase58());

    const mint_tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mintKey.publicKey,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
        lamports,
      }),
      createInitializeMintInstruction(
        mintKey.publicKey,
        0,
        wallet.publicKey,
        wallet.publicKey
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        NftTokenAccount,
        wallet.publicKey,
        mintKey.publicKey
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        buyerTokenAccount,
        wallet.publicKey,
        tokenA
      ),
      createMintToInstruction(
        tokenA,
        buyerTokenAccount,
        mintAuthority.publicKey,
        50 * (10 ** 9),
      )
    );

    await program.provider.sendAndConfirm(mint_tx, [mintKey]);

    console.log("Mint key: ", mintKey.publicKey.toString());
    console.log("User: ", wallet.publicKey.toString());

    const metadataAddress = await getMetadata(mintKey.publicKey);
    const masterEdition = await getMasterEdition(mintKey.publicKey);

    console.log("Metadata address: ", metadataAddress.toBase58());
    console.log("MasterEdition: ", masterEdition.toBase58());

    const tx = await program.methods.mintNft(
      mintKey.publicKey,
      "https://arweave.net/y5e5DJsiwH0s_ayfMwYk-SnrZtVZzHLQDSTZ5dNRUHA",
    ).accounts({
      authority: wallet.publicKey,
      preSaleMarket: preSaleMarket.publicKey,
      buyerTokenAccount,
      treasury: preSaleTreasury,
      mintAuthority: wallet.publicKey,
      mint: mintKey.publicKey,
      tokenAccount: NftTokenAccount,
      metadata: metadataAddress,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      payer: wallet.publicKey,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      masterEdition,
    },
    ).signers([wallet]).rpc();
    console.log("Your transaction signature", tx);
  });
});
