import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MessageProgram } from "../target/types/message_program";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("message-program", () => {
  // Configure the provider (localnet or devnet depending on Anchor.toml)
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MessageProgram as Program<MessageProgram>;

  // Create a new account to store the message
  const messageAccount = Keypair.generate();

  it("Initialize message account", async () => {
    const initialMessage = "Hello, Solana!";

    await program.methods
      .initialize(initialMessage)
      .accounts({
        messageAccount: messageAccount.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([messageAccount]) // new data account must sign
      .rpc();

    const account = await program.account.messageAccount.fetch(
      messageAccount.publicKey
    );

    console.log("Stored Message:", account.message);
    expect(account.message).to.equal(initialMessage);
    expect(account.authority.toBase58()).to.equal(
      provider.wallet.publicKey.toBase58()
    );
  });

  it("Updates the message", async () => {
    const updatedMessage = "Updated message from test!";

    await program.methods
      .updateMessage(updatedMessage)
      .accounts({
        messageAccount: messageAccount.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc(); // wallet auto signs

    const updatedAccount = await program.account.messageAccount.fetch(
      messageAccount.publicKey
    );

    console.log("Updated Message:", updatedAccount.message);
    expect(updatedAccount.message).to.equal(updatedMessage);
  });
});
