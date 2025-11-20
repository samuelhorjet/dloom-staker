import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DloomStaker } from "../target/types/dloom_staker";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAccount,
} from "@solana/spl-token";
import { assert } from "chai";
import * as fs from "fs";

describe("dloom-staker", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DloomStaker as Program<DloomStaker>;

  const loadKeypairFromFile = (path: string) => {
    return anchor.web3.Keypair.fromSecretKey(
      Buffer.from(JSON.parse(fs.readFileSync(path, "utf-8")))
    );
  };

  let lpMint: anchor.web3.PublicKey;
  let rewardMint: anchor.web3.PublicKey;
  let farmPda: anchor.web3.PublicKey;
  let lpVaultPda: anchor.web3.PublicKey;

  const admin = loadKeypairFromFile("./target/test-wallets/user.json");
  const stakerFlexible = anchor.web3.Keypair.generate();
  const stakerLocked = anchor.web3.Keypair.generate();

  let stakerFlexiblePda: anchor.web3.PublicKey;
  let stakerLockedPda: anchor.web3.PublicKey;

  const REWARD_RATE = new anchor.BN(100);
  const LOCKUP_DURATION = new anchor.BN(5);
  const LOCKUP_MULTIPLIER = 20000;

  before(async () => {
    const transaction = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: admin.publicKey,
        toPubkey: stakerFlexible.publicKey,
        lamports: 0.1 * anchor.web3.LAMPORTS_PER_SOL,
      }),
      anchor.web3.SystemProgram.transfer({
        fromPubkey: admin.publicKey,
        toPubkey: stakerLocked.publicKey,
        lamports: 0.1 * anchor.web3.LAMPORTS_PER_SOL,
      })
    );
    await provider.sendAndConfirm(transaction, [admin]);

    lpMint = await createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      6
    );
    rewardMint = await createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      6
    );

    [farmPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("farm"), lpMint.toBuffer(), rewardMint.toBuffer()],
      program.programId
    );

    [lpVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp_vault"), farmPda.toBuffer()],
      program.programId
    );
  });

  it("Creates a Farm", async () => {
    const rewardVault = await anchor.utils.token.associatedAddress({
      mint: rewardMint,
      owner: farmPda,
    });

    await program.methods
      .createFarm()
      .accounts({
        authority: admin.publicKey,
        farm: farmPda,
        lpMint: lpMint,
        lpVault: lpVaultPda,
        rewardMint: rewardMint,
        rewardVault: rewardVault,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([admin])
      .rpc();
  });

  it("Adds a Lockup Tier", async () => {
    await program.methods
      .addLockupTier(LOCKUP_DURATION, LOCKUP_MULTIPLIER)
      .accounts({ farm: farmPda, authority: admin.publicKey })
      .signers([admin])
      .rpc();
  });

  it("Funds the Farm", async () => {
    const amountToFund = new anchor.BN(1_000_000_000);
    const adminRewardAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,
      rewardMint,
      admin.publicKey
    );
    await mintTo(
      provider.connection,
      admin,
      rewardMint,
      adminRewardAccount.address,
      admin,
      1_000_000_000
    );
    const farmAccount = await program.account.farm.fetch(farmPda);

    await program.methods
      .fundFarm(amountToFund)
      .accounts({
        farm: farmPda,
        rewardVault: farmAccount.rewardVault,
        funder: admin.publicKey,
        funderTokenAccount: adminRewardAccount.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();
  });

  it("Sets Reward Rate", async () => {
    await program.methods
      .setRewardRate(REWARD_RATE)
      .accounts({ farm: farmPda, authority: admin.publicKey })
      .signers([admin])
      .rpc();
  });

  it("Initializes Staker (Flexible User)", async () => {
    [stakerFlexiblePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("staker"),
        stakerFlexible.publicKey.toBuffer(),
        farmPda.toBuffer(),
      ],
      program.programId
    );
    await program.methods
      .initializeStaker()
      .accounts({
        owner: stakerFlexible.publicKey,
        farm: farmPda,
        staker: stakerFlexiblePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([stakerFlexible])
      .rpc();
  });

  it("Stakes LP Tokens (Flexible)", async () => {
    const stakeAmount = new anchor.BN(100_000);
    const userLpAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      stakerFlexible,
      lpMint,
      stakerFlexible.publicKey
    );
    await mintTo(
      provider.connection,
      admin,
      lpMint,
      userLpAccount.address,
      admin,
      100_000
    );

    await program.methods
      .stake(stakeAmount, new anchor.BN(0))
      .accounts({
        owner: stakerFlexible.publicKey,
        farm: farmPda,
        staker: stakerFlexiblePda,
        lpVault: lpVaultPda,
        userLpTokenAccount: userLpAccount.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([stakerFlexible])
      .rpc();
  });

  it("Accrues Rewards and Claims", async () => {
    await new Promise((resolve) => setTimeout(resolve, 2000));
    const farmAccount = await program.account.farm.fetch(farmPda);
    const userRewardAccount = await anchor.utils.token.associatedAddress({
      mint: rewardMint,
      owner: stakerFlexible.publicKey,
    });

    await program.methods
      .claimRewards()
      .accounts({
        owner: stakerFlexible.publicKey,
        farm: farmPda,
        staker: stakerFlexiblePda,
        rewardVault: farmAccount.rewardVault,
        userRewardTokenAccount: userRewardAccount,
        rewardMint: rewardMint,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        lpVault: lpVaultPda,
      })
      .signers([stakerFlexible])
      .rpc();
  });

  it("Initializes and Stakes Locked User", async () => {
    [stakerLockedPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("staker"),
        stakerLocked.publicKey.toBuffer(),
        farmPda.toBuffer(),
      ],
      program.programId
    );

    await program.methods
      .initializeStaker()
      .accounts({
        owner: stakerLocked.publicKey,
        farm: farmPda,
        staker: stakerLockedPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([stakerLocked])
      .rpc();

    const stakeAmount = new anchor.BN(100_000);
    const userLpAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      stakerLocked,
      lpMint,
      stakerLocked.publicKey
    );
    await mintTo(
      provider.connection,
      admin,
      lpMint,
      userLpAccount.address,
      admin,
      100_000
    );

    await program.methods
      .stake(stakeAmount, LOCKUP_DURATION)
      .accounts({
        owner: stakerLocked.publicKey,
        farm: farmPda,
        staker: stakerLockedPda,
        lpVault: lpVaultPda,
        userLpTokenAccount: userLpAccount.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([stakerLocked])
      .rpc();

    const stakerAccount = await program.account.staker.fetch(stakerLockedPda);

    // --- DEBUG LOG ---
    console.log("LOCKED USER BALANCE:", stakerAccount.balance.toString());

    assert.equal(stakerAccount.balance.toString(), "100000");
    assert.equal(stakerAccount.rewardMultiplier, LOCKUP_MULTIPLIER);
    assert.isAbove(stakerAccount.lockupEndTimestamp.toNumber(), 0);
  });

  it("Fails to Unstake while Locked", async () => {
    try {
      const userLpAccount = await anchor.utils.token.associatedAddress({
        mint: lpMint,
        owner: stakerLocked.publicKey,
      });
      await program.methods
        .unstake(new anchor.BN(100_000))
        .accounts({
          owner: stakerLocked.publicKey,
          farm: farmPda,
          staker: stakerLockedPda,
          lpVault: lpVaultPda,
          userLpTokenAccount: userLpAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([stakerLocked])
        .rpc();
      assert.fail("Should have failed with StakeLocked");
    } catch (e) {
      assert.include(e.message, "StakeLocked");
    }
  });

  it("Unstakes after Lockup expires", async () => {
    console.log(
      `Waiting ${
        LOCKUP_DURATION.toNumber() + 2
      } seconds for lockup to expire...`
    );
    await new Promise((resolve) =>
      setTimeout(resolve, (LOCKUP_DURATION.toNumber() + 2) * 1000)
    );

    const userLpAccount = await anchor.utils.token.associatedAddress({
      mint: lpMint,
      owner: stakerLocked.publicKey,
    });

    await program.methods
      .unstake(new anchor.BN(100_000))
      .accounts({
        owner: stakerLocked.publicKey,
        farm: farmPda,
        staker: stakerLockedPda,
        lpVault: lpVaultPda,
        userLpTokenAccount: userLpAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([stakerLocked])
      .rpc();

    const stakerAccount = await program.account.staker.fetch(stakerLockedPda);
    assert.ok(stakerAccount.balance.eq(new anchor.BN(0)));
  });

  it("Compounding fails when LP != Reward", async () => {
    try {
      const farmAccount = await program.account.farm.fetch(farmPda);
      await program.methods
        .compound()
        .accounts({
          owner: stakerFlexible.publicKey,
          farm: farmPda,
          staker: stakerFlexiblePda,
          rewardVault: farmAccount.rewardVault,
          lpVault: lpVaultPda,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([stakerFlexible])
        .rpc();
      assert.fail("Should have failed with CompoundingNotSupported");
    } catch (e) {
      assert.include(e.message, "CompoundingNotSupported");
    }
  });
});
