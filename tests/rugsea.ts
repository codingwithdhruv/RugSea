import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Rugsea } from "../target/types/rugsea";
import { Keypair, PublicKey, SYSVAR_RENT_PUBKEY } from "@solana/web3.js";
import {
  createMint, mintTo, getAccount, getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";

describe("rugsea", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Rugsea as Program<Rugsea>;

  it("Setup NFT", async () => {
    const seller = Keypair.generate();
    await provider.connection.requestAirdrop(seller.publicKey, 1e9);

    const nftMint = await createMint(
      provider.connection,
      provider.wallet.payer,
      seller.publicKey,
      null,
      0
    );
    const sellerNftAccount = getAssociatedTokenAddressSync(nftMint, seller.publicKey);
    const createSellerAtaIx = createAssociatedTokenAccountInstruction(
      provider.wallet.payer.publicKey,
      sellerNftAccount,
      seller.publicKey,
      nftMint,
    )
    const sellerAtaTx = new anchor.web3.Transaction().add(createSellerAtaIx);
    await provider.sendAndConfirm(sellerAtaTx);

    await mintTo(
      provider.connection,
      provider.wallet.payer,
      nftMint,
      sellerNftAccount,
      seller,
      1
    )

    const [listingPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("listing"), seller.publicKey.toBuffer(), nftMint.toBuffer()],
      program.programId
    )

    const [escrowAuthority] = PublicKey.findProgramAddressSync(
      [Buffer.from("escrow")], program.programId
    );

    const escrowAccount = getAssociatedTokenAddressSync(
      nftMint, escrowAuthority, true, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID
    )

    const escrowInfo = await provider.connection.getAccountInfo(escrowAccount);
    if (!escrowInfo) {
      const createEscrowIx = createAssociatedTokenAccountInstruction(
        provider.wallet.payer.publicKey,
        escrowAccount,
        escrowAuthority,
        nftMint
      );
      const escrowTx = new anchor.web3.Transaction().add(createEscrowIx);
      await provider.sendAndConfirm(escrowTx);
    }

    const price = new anchor.BN(1000000000);

    const tx = await program.methods
    .createListing(price)
    .accounts({
      seller: seller.publicKey,
      listing: listingPda,
      nftMint,
      sellerNftAccount,
      escrow: escrowAccount,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY
    })
    .signers([seller])
    .rpc();

    console.log("Listing TX:", tx);



  });
});
