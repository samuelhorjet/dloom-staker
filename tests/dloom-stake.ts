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

describe("dloom-staker (Hybrid + Security + Rent Recovery)", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DloomStaker as Program<DloomStaker>;

  // Helper to load keypair (Admin)
  const loadKeypairFromFile = (path: string) => {
    return anchor.web3.Keypair.fromSecretKey(
      Buffer.from(JSON.parse(fs.readFileSync(path, "utf-8")))
    );
  };

  // --- STATE VARIABLES ---
  let lpMint: anchor.web3.PublicKey;
  let rewardMint: anchor.web3.PublicKey;
  let farmPda: anchor.web3.PublicKey;
  let lpVaultPda: anchor.web3.PublicKey;

  // --- USERS ---
  const admin = loadKeypairFromFile("./target/test-wallets/user.json");
  const stakerFlexible = anchor.web3.Keypair.generate();
  const stakerLocked = anchor.web3.Keypair.generate();

  // PDAs
  let stakerFlexiblePda: anchor.web3.PublicKey;
  let stakerLockedPda: anchor.web3.PublicKey;

  // --- CONSTANTS ---
  const REWARD_RATE = new anchor.BN(100);
  const LOCKUP_DURATION = new anchor.BN(5); // 5 Seconds
  const LOCKUP_MULTIPLIER = 20000; // 2.0x

  before(async () => {
    console.log("Setting up test environment...");

    // 1. Fund Stakers from Admin
    const tx = new anchor.web3.Transaction().add(
      anchor.web3.SystemProgram.transfer({
        fromPubkey: admin.publicKey,
        toPubkey: stakerFlexible.publicKey,
        lamports: 0.01 * anchor.web3.LAMPORTS_PER_SOL,
      }),
      anchor.web3.SystemProgram.transfer({
        fromPubkey: admin.publicKey,
        toPubkey: stakerLocked.publicKey,
        lamports: 0.01 * anchor.web3.LAMPORTS_PER_SOL,
      })
    );
    await provider.sendAndConfirm(tx, [admin]);

    // 2. Create Mints
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

    // 3. Derive PDAs
    [farmPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("farm"), lpMint.toBuffer(), rewardMint.toBuffer()],
      program.programId
    );

    [lpVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("lp_vault"), farmPda.toBuffer()],
      program.programId
    );

    console.log("Farm PDA:", farmPda.toBase58());
  });

  // ==========================================================================
  // 1. ADMIN & SETUP TESTS
  // ==========================================================================

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

    const farm = await program.account.farm.fetch(farmPda);
    assert.isFalse(farm.isPaused);
    assert.isFalse(farm.isEmergencyMode);
  });

  it("Adds a Lockup Tier", async () => {
    await program.methods
      .addLockupTier(LOCKUP_DURATION, LOCKUP_MULTIPLIER)
      .accounts({ farm: farmPda, authority: admin.publicKey })
      .signers([admin])
      .rpc();

    const farm = await program.account.farm.fetch(farmPda);
    assert.equal(farm.lockupTiers.length, 2); // Default + New
  });

  it("Funds the Farm", async () => {
    const amount = new anchor.BN(1_000_000_000);
    const adminRewardToken = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,
      rewardMint,
      admin.publicKey
    );
    await mintTo(
      provider.connection,
      admin,
      rewardMint,
      adminRewardToken.address,
      admin,
      1_000_000_000
    );

    const farm = await program.account.farm.fetch(farmPda);

    await program.methods
      .fundFarm(amount)
      .accounts({
        farm: farmPda,
        rewardVault: farm.rewardVault,
        funder: admin.publicKey,
        funderTokenAccount: adminRewardToken.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();

    const vault = await getAccount(provider.connection, farm.rewardVault);
    assert.equal(vault.amount.toString(), amount.toString());
  });

  it("Sets Reward Rate", async () => {
    await program.methods
      .setRewardRate(REWARD_RATE)
      .accounts({ farm: farmPda, authority: admin.publicKey })
      .signers([admin])
      .rpc();
  });

  // ==========================================================================
  // 2. BUCKET (FLEXIBLE) TESTS
  // ==========================================================================

  it("Initializes Staker (Flexible)", async () => {
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

  it("Stakes Flexible (Bucket)", async () => {
    const amount = new anchor.BN(100_000);
    const userLp = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      stakerFlexible,
      lpMint,
      stakerFlexible.publicKey
    );
    await mintTo(
      provider.connection,
      admin,
      lpMint,
      userLp.address,
      admin,
      100_000
    );

    await program.methods
      .stakeFlexible(amount)
      .accounts({
        owner: stakerFlexible.publicKey,
        farm: farmPda,
        staker: stakerFlexiblePda,
        lpVault: lpVaultPda,
        userLpTokenAccount: userLp.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([stakerFlexible])
      .rpc();

    const staker = await program.account.staker.fetch(stakerFlexiblePda);
    assert.equal(staker.flexibleBalance.toString(), "100000");
    assert.equal(staker.positions.length, 0);
    // Weight should be 1:1
    assert.equal(staker.totalActiveWeight.toString(), "100000");
  });

  it("Unstakes Flexible (Bucket)", async () => {
    const amount = new anchor.BN(50_000); // Partial unstake
    const userLp = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      stakerFlexible,
      lpMint,
      stakerFlexible.publicKey
    );

    await program.methods
      .unstakeFlexible(amount)
      .accounts({
        owner: stakerFlexible.publicKey,
        farm: farmPda,
        staker: stakerFlexiblePda,
        lpVault: lpVaultPda,
        userLpTokenAccount: userLp.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([stakerFlexible])
      .rpc();

    const staker = await program.account.staker.fetch(stakerFlexiblePda);
    assert.equal(staker.flexibleBalance.toString(), "50000");
  });

  // ==========================================================================
  // 3. POSITION (LOCKED) TESTS
  // ==========================================================================

  it("Initializes Staker (Locked)", async () => {
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
  });

  it("Stakes Locked (Position)", async () => {
    const amount = new anchor.BN(100_000);
    const userLp = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      stakerLocked,
      lpMint,
      stakerLocked.publicKey
    );
    await mintTo(
      provider.connection,
      admin,
      lpMint,
      userLp.address,
      admin,
      100_000
    );

    await program.methods
      .stakeLocked(amount, LOCKUP_DURATION)
      .accounts({
        owner: stakerLocked.publicKey,
        farm: farmPda,
        staker: stakerLockedPda,
        lpVault: lpVaultPda,
        userLpTokenAccount: userLp.address,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([stakerLocked])
      .rpc();

    const staker = await program.account.staker.fetch(stakerLockedPda);
    assert.equal(staker.positions.length, 1);
    assert.equal(staker.positions[0].amount.toString(), "100000");
    assert.equal(staker.positions[0].id.toString(), "1");

    // Weight Check: 100,000 * 2.0 = 200,000
    assert.equal(staker.totalActiveWeight.toString(), "200000");
  });

  it("Fails to Unstake Locked Position Early", async () => {
    const userLp = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      stakerLocked,
      lpMint,
      stakerLocked.publicKey
    );

    try {
      await program.methods
        .unstakeLocked(new anchor.BN(1)) // ID = 1
        .accounts({
          owner: stakerLocked.publicKey,
          farm: farmPda,
          staker: stakerLockedPda,
          lpVault: lpVaultPda,
          userLpTokenAccount: userLp.address,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([stakerLocked])
        .rpc();
      assert.fail("Should have failed with StakeLocked");
    } catch (e) {
      assert.include(e.message, "StakeLocked");
    }
  });

  it("Unstakes Locked Position After Time", async () => {
    console.log("Waiting for lockup...");
    await new Promise((resolve) => setTimeout(resolve, 6000)); // Wait 6s

    const userLp = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      stakerLocked,
      lpMint,
      stakerLocked.publicKey
    );

    await program.methods
      .unstakeLocked(new anchor.BN(1)) // ID = 1
      .accounts({
        owner: stakerLocked.publicKey,
        farm: farmPda,
        staker: stakerLockedPda,
        lpVault: lpVaultPda,
        userLpTokenAccount: userLp.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([stakerLocked])
      .rpc();

    const staker = await program.account.staker.fetch(stakerLockedPda);
    assert.equal(staker.positions.length, 0);
    assert.equal(staker.totalActiveWeight.toString(), "0");
  });

  // ==========================================================================
  // 4. REWARD TESTS
  // ==========================================================================

  it("Claims Rewards", async () => {
    // Flexible user still has 50,000 staked. They should have earned rewards.
    const farm = await program.account.farm.fetch(farmPda);
    const userReward = await anchor.utils.token.associatedAddress({
      mint: rewardMint,
      owner: stakerFlexible.publicKey,
    });

    await program.methods
      .claimRewards()
      .accounts({
        owner: stakerFlexible.publicKey,
        farm: farmPda,
        staker: stakerFlexiblePda,
        rewardMint: rewardMint,
        rewardVault: farm.rewardVault,
        userRewardTokenAccount: userReward,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([stakerFlexible])
      .rpc();

    const account = await getAccount(provider.connection, userReward);
    console.log("Rewards Claimed:", account.amount.toString());
    assert.isAbove(Number(account.amount), 0);
  });

  it("Compounding fails (Different Mints)", async () => {
    try {
      const farm = await program.account.farm.fetch(farmPda);
      await program.methods
        .compound()
        .accounts({
          owner: stakerFlexible.publicKey,
          farm: farmPda,
          staker: stakerFlexiblePda,
          rewardVault: farm.rewardVault,
          lpVault: lpVaultPda,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([stakerFlexible])
        .rpc();
      assert.fail();
    } catch (e) {
      assert.include(e.message, "CompoundingNotSupported");
    }
  });

  // ==========================================================================
  // 5. ADMIN SECURITY TESTS
  // ==========================================================================

  it("Pauses the Farm", async () => {
    await program.methods
      .togglePause()
      .accounts({ farm: farmPda, authority: admin.publicKey })
      .signers([admin])
      .rpc();

    const farm = await program.account.farm.fetch(farmPda);
    assert.isTrue(farm.isPaused);
  });

  it("Fails to Stake while Paused", async () => {
    try {
      await program.methods
        .stakeFlexible(new anchor.BN(100))
        .accounts({
          owner: stakerFlexible.publicKey,
          farm: farmPda,
          staker: stakerFlexiblePda,
          lpVault: lpVaultPda,
          userLpTokenAccount: (
            await getOrCreateAssociatedTokenAccount(
              provider.connection,
              stakerFlexible,
              lpMint,
              stakerFlexible.publicKey
            )
          ).address,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([stakerFlexible])
        .rpc();
      assert.fail();
    } catch (e) {
      assert.include(e.message, "FarmPaused");
    }
  });

  it("Unpauses the Farm", async () => {
    await program.methods
      .togglePause()
      .accounts({ farm: farmPda, authority: admin.publicKey })
      .signers([admin])
      .rpc();
    const farm = await program.account.farm.fetch(farmPda);
    assert.isFalse(farm.isPaused);
  });

  it("Admin Emergency Withdraws Rewards (Success)", async () => {
    const farm = await program.account.farm.fetch(farmPda);
    const adminReward = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,
      rewardMint,
      admin.publicKey
    );
    const preBalance = (
      await getAccount(provider.connection, adminReward.address)
    ).amount;

    await program.methods
      .emergencyWithdraw(new anchor.BN(100))
      .accounts({
        farm: farmPda,
        authority: admin.publicKey,
        vault: farm.rewardVault, // Withdraw from Reward Vault
        adminTokenAccount: adminReward.address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([admin])
      .rpc();

    const postBalance = (
      await getAccount(provider.connection, adminReward.address)
    ).amount;
    assert.isAbove(Number(postBalance), Number(preBalance));
  });

  it("Admin Emergency Withdraws LP Tokens (Fail - Anti Rug)", async () => {
    const adminLp = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,
      lpMint,
      admin.publicKey
    );

    try {
      await program.methods
        .emergencyWithdraw(new anchor.BN(100))
        .accounts({
          farm: farmPda,
          authority: admin.publicKey,
          vault: lpVaultPda, // TRYING TO WITHDRAW FROM LP VAULT
          adminTokenAccount: adminLp.address,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([admin])
        .rpc();
      assert.fail("Should have failed Anti-Rug check");
    } catch (e) {
      assert.include(e.message, "AdminCannotWithdrawLP");
    }
  });

  it("Enables Emergency Mode and Users Escape", async () => {
    // 1. Stake some funds first to have something to rescue
    await program.methods
      .stakeFlexible(new anchor.BN(1000))
      .accounts({
        owner: stakerFlexible.publicKey,
        farm: farmPda,
        staker: stakerFlexiblePda,
        lpVault: lpVaultPda,
        userLpTokenAccount: (
          await getOrCreateAssociatedTokenAccount(
            provider.connection,
            stakerFlexible,
            lpMint,
            stakerFlexible.publicKey
          )
        ).address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([stakerFlexible])
      .rpc();

    // 2. Enable Emergency Mode
    await program.methods
      .setEmergencyMode(true)
      .accounts({ farm: farmPda, authority: admin.publicKey })
      .signers([admin])
      .rpc();

    const farm = await program.account.farm.fetch(farmPda);
    assert.isTrue(farm.isEmergencyMode);

    // 3. User Calls Emergency Unstake (Escape Hatch)
    // This should return ALL funds (approx 50,000 remaining + 1000 new)
    await program.methods
      .emergencyUnstake()
      .accounts({
        owner: stakerFlexible.publicKey,
        farm: farmPda,
        staker: stakerFlexiblePda,
        lpVault: lpVaultPda,
        userLpTokenAccount: (
          await getOrCreateAssociatedTokenAccount(
            provider.connection,
            stakerFlexible,
            lpMint,
            stakerFlexible.publicKey
          )
        ).address,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([stakerFlexible])
      .rpc();

    const staker = await program.account.staker.fetch(stakerFlexiblePda);
    assert.equal(staker.flexibleBalance.toString(), "0");
    assert.equal(staker.totalActiveWeight.toString(), "0");
  });

  // ==========================================================================
  // 6. CLOSE ACCOUNT TEST (RENT RECOVERY)
  // ==========================================================================

  it("Closes the Staker Account (Rent Recovery)", async () => {
    // The Flexible Staker just did 'emergencyUnstake', so balance is 0 and rewards are 0.
    // They are eligible to close the account.

    // 1. Check SOL Balance Before
    const preLamports = await provider.connection.getBalance(
      stakerFlexible.publicKey
    );

    // 2. Close Account
    await program.methods
      .closeStaker()
      .accounts({
        owner: stakerFlexible.publicKey,
        farm: farmPda,
        staker: stakerFlexiblePda,
      })
      .signers([stakerFlexible])
      .rpc();

    // 3. Verify Account is gone (Fetching should fail)
    try {
      await program.account.staker.fetch(stakerFlexiblePda);
      assert.fail("Account should not exist anymore");
    } catch (e) {
      assert.include(e.message, "Account does not exist");
    }

    // 4. Verify SOL Balance Increased (Rent Returned)
    const postLamports = await provider.connection.getBalance(
      stakerFlexible.publicKey
    );
    console.log(
      `SOL Refunded: ${
        (postLamports - preLamports) / anchor.web3.LAMPORTS_PER_SOL
      } SOL`
    );
    assert.isAbove(postLamports, preLamports);
  });
});
