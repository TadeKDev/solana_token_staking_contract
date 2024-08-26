import * as anchor from "@project-serum/anchor";
import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { IDL } from "../target/types/anchor_escrow";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Connection, Commitment } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, getAccount, ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from '@solana/spl-token';

function wait(milliseconds) {
  return new Promise((resolve) => {
    setTimeout(resolve, milliseconds);
  });
}

describe("anchor-escrow", () => {
  const commitment: Commitment = "confirmed";
  const connection = new Connection("https://api.devnet.solana.com", {
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
  const stakeToken = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");

  // it("init admin address", async () => {
  //   const adminState = PublicKey.findProgramAddressSync(
  //     [Buffer.from("state"),Buffer.from("admin")],
  //     program.programId
  //   )[0];
  //   const vault = PublicKey.findProgramAddressSync(
  //     [Buffer.from("vault"),stakeToken.toBuffer()],
  //     program.programId
  //   )[0];
    
  //   await program.methods
  //     .initAdmin()
  //     .accounts({
  //       admin: wallet.publicKey.toString(),
  //       adminState,
  //       stakeToken,
  //       vault, 
  //       systemProgram: SystemProgram.programId,
  //       rent: SYSVAR_RENT_PUBKEY,
  //       tokenProgram: TOKEN_PROGRAM_ID
  //     })
  //     .signers([wallet.payer])
  //     .rpc();

  //   await wait(500);
  //   const fetchedAdminState: any = await program.account.adminState.fetch(adminState);
  //   console.log(fetchedAdminState);
  // });

  // it("update admin address", async () => {
  //   const adminState = PublicKey.findProgramAddressSync(
  //     [Buffer.from("state"),Buffer.from("admin")],
  //     program.programId
  //   )[0];
    
  //   await program.methods
  //     .updateAdminInfo(new anchor.BN(5), new anchor.BN(50), new anchor.BN(60 * 5))
  //     .accounts({
  //       admin: wallet.publicKey.toString(),
  //       adminState
  //     })
  //     .signers([wallet.payer])
  //     .rpc();

  //   await wait(500);
  //   const fetchedAdminState: any = await program.account.adminState.fetch(adminState);
  //   console.log(fetchedAdminState);
  // });

  // it("add liquidity", async () => {
  //   const adminState = PublicKey.findProgramAddressSync(
  //     [Buffer.from("state"),Buffer.from("admin")],
  //     program.programId
  //   )[0];

  //   const vault = PublicKey.findProgramAddressSync(
  //     [Buffer.from("vault"),stakeToken.toBuffer()],
  //     program.programId
  //   )[0];

  //   const adminDepositTokenAccount = await getAssociatedTokenAddress(
  //     stakeToken,
  //     wallet.publicKey,  
  //     true,
  //     TOKEN_PROGRAM_ID,
  //     ASSOCIATED_TOKEN_PROGRAM_ID,
  //   );
  //   await program.methods
  //     .addLiquidity(new anchor.BN(10000000))
  //     .accounts({
  //       admin: wallet.publicKey.toString(),
  //       stakeToken,
  //       adminState,
  //       vault,
  //       adminDepositTokenAccount,
  //       tokenProgram: TOKEN_PROGRAM_ID
  //     })
  //     .signers([wallet.payer])
  //     .rpc();

  //   await wait(500);
  //   const fetchedAdminState: any = await program.account.adminState.fetch(adminState);
  //   console.log(fetchedAdminState);
  // });

  // it("stake function", async () => {

  //   // Derive PDAs: escrowStateKey, vaultKey, vaultAuthorityKey
  //   const userState = PublicKey.findProgramAddressSync(
  //     [Buffer.from("state"),Buffer.from("user"), wallet.publicKey.toBuffer()],
  //     program.programId
  //   )[0];
  //   const adminState = PublicKey.findProgramAddressSync(
  //     [Buffer.from("state"),Buffer.from("admin")],
  //     program.programId
  //   )[0];
  //   const vault = PublicKey.findProgramAddressSync(
  //     [Buffer.from("vault"),stakeToken.toBuffer()],
  //     program.programId
  //   )[0];
  //   const stakerDepositTokenAccount = await getAssociatedTokenAddress(
  //     stakeToken,
  //     wallet.publicKey,  
  //     true,
  //     TOKEN_PROGRAM_ID,
  //     ASSOCIATED_TOKEN_PROGRAM_ID,
  //   );

  //   await program.methods
  //     .stake(new anchor.BN(10000000))
  //     .accounts({
  //       staker: wallet.publicKey.toString(),
  //       adminState,
  //       stakeToken,
  //       vault,
  //       stakerDepositTokenAccount,
  //       userState,
  //       systemProgram: SystemProgram.programId,
  //       rent: SYSVAR_RENT_PUBKEY,
  //       tokenProgram: TOKEN_PROGRAM_ID
  //     })
  //     .signers([wallet.payer])
  //     .rpc();

  //   await wait(500);
  //   const fetchedUserState: any = await program.account.userState.fetch(userState);
  //   console.log(fetchedUserState);
  // });

  // it("get reward function", async () => {

  //     // Derive PDAs: escrowStateKey, vaultKey, vaultAuthorityKey
  //     const userState = PublicKey.findProgramAddressSync(
  //       [Buffer.from("state"),Buffer.from("user"), wallet.publicKey.toBuffer()],
  //       program.programId
  //     )[0];
  //     const adminState = PublicKey.findProgramAddressSync(
  //       [Buffer.from("state"),Buffer.from("admin")],
  //       program.programId
  //     )[0];
  //     const vault = PublicKey.findProgramAddressSync(
  //       [Buffer.from("vault"),stakeToken.toBuffer()],
  //       program.programId
  //     )[0];
  //     const stakerDepositTokenAccount = await getAssociatedTokenAddress(
  //       stakeToken,
  //       wallet.publicKey,  
  //       true,
  //       TOKEN_PROGRAM_ID,
  //       ASSOCIATED_TOKEN_PROGRAM_ID,
  //     );
  //     const adminDepositTokenAccount = await getAssociatedTokenAddress(
  //       stakeToken,
  //       wallet.publicKey,  
  //       true,
  //       TOKEN_PROGRAM_ID,
  //       ASSOCIATED_TOKEN_PROGRAM_ID,
  //     );

  //     await program.methods
  //       .getReward()
  //       .accounts({
  //         staker: wallet.publicKey.toString(),
  //         admin: wallet.publicKey.toString(),
  //         adminState,
  //         stakeToken,
  //         vault,
  //         stakerDepositTokenAccount,
  //         adminDepositTokenAccount,
  //         userState,
  //         tokenProgram: TOKEN_PROGRAM_ID
  //       })
  //       .signers([wallet.payer])
  //       .rpc();

  //     await wait(500);
  //     const fetchedUserState: any = await program.account.userState.fetch(userState);
  //     console.log(fetchedUserState);
  //   });

  it("unstake function", async () => {

    // // Derive PDAs: escrowStateKey, vaultKey, vaultAuthorityKey
    const userState = PublicKey.findProgramAddressSync(
      [Buffer.from("state"),Buffer.from("user"), wallet.publicKey.toBuffer()],
      program.programId
    )[0];
    const adminState = PublicKey.findProgramAddressSync(
      [Buffer.from("state"),Buffer.from("admin")],
      program.programId
    )[0];
    const vault = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"),stakeToken.toBuffer()],
      program.programId
    )[0];
    const stakerDepositTokenAccount = await getAssociatedTokenAddress(
      stakeToken,
      wallet.publicKey,  
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
    );
    const adminDepositTokenAccount = await getAssociatedTokenAddress(
      stakeToken,
      wallet.publicKey,  
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID,
    );
    await program.methods
      .unstake()
      .accounts({
        staker: wallet.publicKey.toString(),
        admin: wallet.publicKey.toString(),
        adminState,
        stakeToken,
        vault,
        stakerDepositTokenAccount,
        adminDepositTokenAccount,
        userState,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([wallet.payer])
      .rpc();

    await wait(500);
    const fetchedUserState: any = await program.account.userState.fetch(userState);
    console.log(fetchedUserState);
  });
});