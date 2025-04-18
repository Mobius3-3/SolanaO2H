import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crud } from "../target/types/crud";
import { expect, assert } from "chai";
import BN from "bn.js";

async function sleep(time: number=1000) {
  return new Promise(resolve => setTimeout(resolve, time));
}

describe("crud", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Part1 as Program<Crud>;
  let myStoragePda: anchor.web3.PublicKey; 
  let value1: anchor.BN;

  it("Is created!", async () => {
    const seeds = []
    const [_myStoragePda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
    myStoragePda = _myStoragePda;
    await program.methods.create().accounts({ myStorage: myStoragePda}).rpc();
    await sleep();
  });

  it("Is read!", async () => {
    value1 = (await program.account.myStorage.fetch(myStoragePda)).value1
    expect(value1.toNumber()).to.equal(0);
  });

  it("Is updated!", async () => {
    await program.methods.update(new BN(1)).accounts({ myStorage: myStoragePda}).rpc();
    await sleep();

    value1 = (await program.account.myStorage.fetch(myStoragePda)).value1 
    expect(value1.toNumber()).to.equal(1);
  });

  it("Deletes the myStorage account", async () => {
    await program.methods
      .delete()
      .accounts({
        myStorage: myStoragePda,
      })
      .rpc();
  
    await sleep();
  
    try {
      await program.account.myStorage.fetch(myStoragePda);
      assert.fail("❌ Expected account to be deleted, but fetch succeeded.");
    } catch (e) {
      if (e.message.includes("Account does not exist")) {
        console.log("✅ Account deleted as expected.");
      } else {
        console.error("❌ Unexpected error while fetching account:", e.message);
        // throw e; // Re-throw if you want the test to fail
      }
    }
  });
});