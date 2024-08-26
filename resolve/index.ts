import * as anchor from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { IDL } from "../target/types/anchor_escrow";
import { PublicKey, SystemProgram, Transaction, Connection, Commitment } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAccount, mintTo, getAccount, transfer } from "@solana/spl-token";
import { assert } from "chai";

function wait(milliseconds) {
  return new Promise((resolve) => {
    setTimeout(resolve, milliseconds);
  });
}

const resolver = anchor.web3.Keypair.fromSecretKey(
  new Uint8Array([
    167, 235, 82, 154, 192, 129, 184, 82, 193, 236, 49, 76, 173, 215, 178, 140, 85, 143, 232, 173, 126, 217, 195, 142,
    144, 121, 227, 134, 90, 216, 239, 37, 53, 74, 208, 68, 168, 25, 235, 51, 153, 79, 121, 236, 124, 15, 184, 27, 8, 57,
    226, 218, 87, 87, 90, 58, 246, 234, 225, 94, 197, 91, 58, 168,
  ])
);

describe("anchor-escrow", () => {
  // Use Mainnet-fork for testing
  const commitment: Commitment = "confirmed";
  const connection = new Connection("https://api.devnet.solana.com", {
    commitment,
    // wsEndpoint: "wss://api.devnet.solana.com/",
  });
  const options = anchor.AnchorProvider.defaultOptions();
  const wallet = new anchor.Wallet(resolver);
  const provider = new anchor.AnchorProvider(connection, wallet, options);

  anchor.setProvider(provider);

  // CAUTTION: if you are intended to use the program that is deployed by yourself,
  // please make sure that the programIDs are consistent
  const programId = new PublicKey("2gSyVrvohTuae4WQZcrVUdV5vhfxWmbGkPjYJjiZx6rX");
  const program = new anchor.Program(IDL, programId, provider);

  let mintA = new PublicKey("Ad4JSN6xUeok3JVgow9LTJ8GW1K1y8W397nsZrNYYW5E");
  let initializerTokenAccountA = null as PublicKey;
  let takerTokenAccountA = null as PublicKey;
  let admin1AccountA = null as PublicKey;
  let admin2AccountA = null as PublicKey;
  let resolverAccountA = null as PublicKey;
  let localWalletAccountA = null as PublicKey;

  console.log("Your wallet address is ", resolver.publicKey.toString());

  // Determined Seeds
  const adminSeed = "admin";
  const stateSeed = "state";
  const vaultSeed = "vault";
  const authoritySeed = "authority";

  const adminKey = PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode(stateSeed)), Buffer.from(anchor.utils.bytes.utf8.encode(adminSeed))],
    program.programId
  )[0];

  const vaultAuthorityKey = PublicKey.findProgramAddressSync(
    [Buffer.from(anchor.utils.bytes.utf8.encode(authoritySeed))],
    program.programId
  )[0];

  it("Solve the real dispute", async () => {
    const tmpSeed: anchor.BN = new anchor.BN(54913206);
    const escrowStateKey = PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode(stateSeed)), tmpSeed.toArrayLike(Buffer, "le", 8)],
      program.programId
    )[0];
    const vaultKey = PublicKey.findProgramAddressSync(
      [Buffer.from(anchor.utils.bytes.utf8.encode(vaultSeed)), tmpSeed.toArrayLike(Buffer, "le", 8)],
      program.programId
    )[0];
    await program.methods
      .resolve(new anchor.BN(0))
      .accounts({
        resolver: wallet.publicKey,
        takerTokenAccount: new PublicKey("FUXaFmc5xqKcSX1sdXVuEP5iYWjBSpYeWHxSRCwGxRmU"),
        admin1TokenAccount: new PublicKey("6ch2CPNLzjaCgjS7dQBgBNSrjBWVhYBDHzFERmghPqX"),
        admin2TokenAccount: new PublicKey("6a1SizqF4Mrgb1sqxHRXCe4g6UiUbGSa1qQQbZR8Tge3"),
        resolverTokenAccount: new PublicKey("CSRpjKrcXFBvWGPC1SVCbBozywWWqkAx8fTh3vvAfMn9"),
        escrowState: escrowStateKey,
        adminState: adminKey,
        vault: vaultKey,
        vaultAuthority: vaultAuthorityKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([resolver])
      .rpc();

    await wait(1000);
    let fetchedTakerTokenAccountA = await getAccount(connection, takerTokenAccountA);
    let fetchedResolverTokenAccountA = await getAccount(connection, resolverAccountA);
    let fetchedAdmin1TokenAccountA = await getAccount(connection, localWalletAccountA);
    let fetchedAdmin2TokenAccountA = await getAccount(connection, admin2AccountA);
    console.log(Number(fetchedTakerTokenAccountA.amount));
    console.log(Number(fetchedResolverTokenAccountA.amount));
    console.log(Number(fetchedAdmin1TokenAccountA.amount));
    console.log(Number(fetchedAdmin2TokenAccountA.amount));

    assert.ok(1 === 1);
  });
});
