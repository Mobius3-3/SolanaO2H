import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Storage } from "../target/types/storage";
import { assert } from "chai";

describe("storage", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Storage as Program<Storage>;

  const key = "unique-key";
  const name = "Alice";
  const address = "123 Blockchain Ave";

  let userPda: anchor.web3.PublicKey;
  let bump: number;

  it("creates a user", async () => {
    // Derive PDA
    [userPda, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(key)],
      program.programId
    );

    // Send the transaction
    await program.methods
      .createUser(key, name, address)
      .accounts({
        userAccount: userPda,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([]) // no manual signers needed since provider wallet signs
      .rpc();

    // Fetch the account
    const account = await program.account.user.fetch(userPda);
    const decodedName = Buffer.from(account.name).toString("utf-8").replace(/\0/g, "");

    console.log("Decoded name:", decodedName);
    console.log("Address:", account.address);

    // Assertions
    assert.equal(decodedName, name);
    assert.equal(account.address, address);
  });
});
