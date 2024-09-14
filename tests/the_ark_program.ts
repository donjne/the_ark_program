import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TheArkProgram } from "../target/types/the_ark_program";
import { assert } from "chai";

describe("the_ark_program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ArkMonolith as Program<TheArkProgram>;

  let analyticsAccount = anchor.web3.Keypair.generate();
  let escrowInfoAccount = anchor.web3.Keypair.generate();

  console.log("Analytics Account Public Key:", analyticsAccount.publicKey.toString());
  console.log("Escrow Info Account Public Key:", escrowInfoAccount.publicKey.toString());

  it("Initialize the ark", async () => {
      await program.methods
        .initializeArk()
        .accounts({
          arkAnalytics: analyticsAccount.publicKey,
          signer: provider.wallet.publicKey,
        })
        .signers([analyticsAccount])
        .rpc();
    

    const analytics = await program.account.arkAnalytics.fetch(analyticsAccount.publicKey);
    
    assert.equal(analytics.noOfGovernments.toNumber(), 0);
    assert.equal(analytics.polls.toNumber(), 0);
    assert.equal(analytics.approved.toNumber(), 0);
    assert.equal(analytics.rejected.toNumber(), 0);
    assert.equal(analytics.points.toNumber(), 0);
  });

  it("Initialize the escrow", async () => {
    await program.methods
      .initEscrow()
      .accounts({
        escrowInfo: escrowInfoAccount.publicKey,
        signer: provider.wallet.publicKey,
        // systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([escrowInfoAccount])
      .rpc();

    const escrowInfo = await program.account.escrowInfo.fetch(escrowInfoAccount.publicKey);
    
    assert.equal(escrowInfo.totalTrades.toNumber(), 0);
    assert.equal(escrowInfo.totalServices.toNumber(), 0);
    assert.equal(escrowInfo.totalFeesCollected.toNumber(), 0);
    assert.equal(escrowInfo.totalAmountTransferred.toNumber(), 0);
  });

  it("Register a trade", async () => {
    const tradeAddress = anchor.web3.Keypair.generate().publicKey;

    await program.methods
      .registerTrades(tradeAddress)
      .accounts({
        escrowInfo: escrowInfoAccount.publicKey,
      })
      .rpc();

    const escrowInfo = await program.account.escrowInfo.fetch(escrowInfoAccount.publicKey);
    
    assert.equal(escrowInfo.totalTrades.toNumber(), 1);
    assert.deepEqual(escrowInfo.trades[0], tradeAddress);
  });

  it("Register a service", async () => {
    const servicesAddress = anchor.web3.Keypair.generate().publicKey;

    await program.methods
      .registerServices(servicesAddress)
      .accounts({
        escrowInfo: escrowInfoAccount.publicKey,
      })
      .rpc();

    const escrowInfo = await program.account.escrowInfo.fetch(escrowInfoAccount.publicKey);
    
    assert.equal(escrowInfo.totalServices.toNumber(), 1);
    assert.deepEqual(escrowInfo.services[0], servicesAddress);
  });

  it("Register a government", async () => {
    const governmentAddress = anchor.web3.Keypair.generate().publicKey;

    await program.methods
      .registerGovernment(governmentAddress)
      .accounts({
        arkAnalytics: analyticsAccount.publicKey,
        governmentProgram: program.programId,
        payer: provider.wallet.publicKey,
        // stateInfo: ,
      })
      .rpc();

    const analytics = await program.account.arkAnalytics.fetch(analyticsAccount.publicKey);
    
    assert.equal(analytics.noOfGovernments.toNumber(), 1);
    assert.deepEqual(analytics.governments[0], governmentAddress);
  });

  it("Update analytics", async () => {
    await program.methods
      .updateAnalytics(true)
      .accounts({
        arkAnalytics: analyticsAccount.publicKey,
      })
      .rpc();

    const analytics = await program.account.arkAnalytics.fetch(analyticsAccount.publicKey);
    
    assert.equal(analytics.approved.toNumber(), 1);
    assert.equal(analytics.polls.toNumber(), 1);
  });
});
