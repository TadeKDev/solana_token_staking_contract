import * as anchor from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { IDL } from "../target/types/anchor_escrow";
import { PublicKey, SystemProgram, Transaction, Connection, Commitment, clusterApiUrl } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount, transfer } from "@solana/spl-token";
import { assert } from "chai";

function wait(milliseconds) {
  return new Promise((resolve) => {
    setTimeout(resolve, milliseconds);
  });
}

describe("anchor-escrow", () => {
  // Use Mainnet-fork for testing
  const commitment: Commitment = "confirmed";
  const connection = new Connection(clusterApiUrl("mainnet-beta"), {
    commitment,
    // wsEndpoint: "wss://api.devnet.solana.com/",
  });
  const options = anchor.AnchorProvider.defaultOptions();
  const wallet = NodeWallet.local();
  const provider = new anchor.AnchorProvider(connection, wallet, options);

  anchor.setProvider(provider);

  // CAUTTION: if you are intended to use the program that is deployed by yourself,
  // please make sure that the programIDs are consistent
  const programId = new PublicKey("2gSyVrvohTuae4WQZcrVUdV5vhfxWmbGkPjYJjiZx6rX");
  const program = new anchor.Program(IDL, programId, provider);

  // Determined Seeds
  const adminSeed = "admin";
  const stateSeed = "state";

  const adminKey = PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode(stateSeed)), Buffer.from(anchor.utils.bytes.utf8.encode(adminSeed))],
    program.programId
  )[0];

  it("init admin address", async () => {
    await program.methods
      .initAdmin()
      .accounts({
        admin1: wallet.publicKey.toString(),
        admin2: new PublicKey("BddjKVEuSUbmAv7cyXKyzBUQDUHshwihWmkoqwXmpwvi"),
        resolver: new PublicKey("4b2mrvjxPjwzASUXYDNVhuy8bbp5jVZC2TJms1veYRJf"),
        adminState: adminKey.toString(),
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([wallet.payer])
      .rpc();

    await wait(500);
    const fetchedAdminState: any = await program.account.adminState.fetch(adminKey);
    console.log("bump", Number(fetchedAdminState.bump));
    assert.ok(fetchedAdminState.admin1.toString() === wallet.publicKey.toString());
    assert.ok(fetchedAdminState.admin2.toString() === "BddjKVEuSUbmAv7cyXKyzBUQDUHshwihWmkoqwXmpwvi");
    assert.ok(fetchedAdminState.resolver.toString() === "4b2mrvjxPjwzASUXYDNVhuy8bbp5jVZC2TJms1veYRJf");
  });

  // it("set fee", async () => {
  //   const adminfee = 5;
  //   const resolverfee = 1;
  //   await program.methods
  //     .setFee(new anchor.BN(adminfee), new anchor.BN(resolverfee))
  //     .accounts({
  //       admin1: wallet.publicKey.toString(),
  //       adminState: adminKey.toString(),
  //     })
  //     .signers([wallet.payer])
  //     .rpc();

  //   await wait(500);
  //   const fetchedAdminState: any = await program.account.adminState.fetch(adminKey);
  //   assert.ok(fetchedAdminState.adminFee.toNumber() === adminfee);
  //   assert.ok(fetchedAdminState.resolverFee.toNumber() === resolverfee);
  // });
});
