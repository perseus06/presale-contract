import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Presale } from "../target/types/presale";

import { TOKEN_PROGRAM_ID, createAccount, createInitializeMintInstruction, MINT_SIZE, getMinimumBalanceForRentExemptMint, createMint, createAssociatedTokenAccount, getAssociatedTokenAddress, ASSOCIATED_TOKEN_PROGRAM_ID, mintTo, mintToChecked, getAccount, getMint, getAssociatedTokenAddressSync, createAssociatedTokenAccountInstruction, createMintToCheckedInstruction } from "@solana/spl-token";
import * as bs58 from "bs58";
import { SystemProgram, Keypair, PublicKey, Transaction, SYSVAR_RENT_PUBKEY, SYSVAR_CLOCK_PUBKEY, Connection, clusterApiUrl, sendAndConfirmTransaction } from "@solana/web3.js";
import assert from "assert";

describe("presale", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Presale as Program<Presale>;

  let presale, vault, tokenVault: PublicKey;
  let presaleBump, vaultBump, tokenVaultBump: Number;
  let tokenMint = new PublicKey("9aXG6LBZWhSapmjQDfuZzBp67Nd3TDbdrJKi1yDZVDCp");

  // Bmed1qoe6u8VxmJ5p6SW77fb7LiSqWmQdTtKTz5dyh62
  let owner = Keypair.fromSecretKey(bs58.decode(""));

  let user = Keypair.fromSecretKey(bs58.decode(""));


  it("Get PDA", async() => {
    [presale, presaleBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("PRESALE_SEED")
      ],
      program.programId
    );

    [vault, vaultBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("VAULT_SEED")
      ],
      program.programId
    );

    [tokenVault, tokenVaultBump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("TOKEN_VAULT_SEED"),
        tokenMint.toBuffer()
      ],
      program.programId
    );
    // const presaleData = await program.account.presale.fetch(presale);
    // const tokenAmount = presaleData.tokenAmount;
    // console.log(presaleData);
  });
  it("Is initialized!", async () => {
    try {
      let amount = 10000000000000;
      let tokenPrice = 100000;

      const tokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        owner.publicKey
      );
      // Add your test here.
      const tx = await program.rpc.initialize(
        new anchor.BN(amount),
        new anchor.BN(tokenPrice), {
          accounts: {
            owner: owner.publicKey,
            presale,
            vault,
            tokenMint,
            tokenAccount,
            tokenVaultAccount:tokenVault,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [owner]
        }
      );
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.log(error);
    }
  });
  it("active the contract", async() => {
    try {
      const tx = await program.rpc.toggleStatus(
        {
          accounts: {
            owner: owner.publicKey,
            presale
          },
          signers: [owner]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });
  
  it("token sale for presale", async() => {
    try {
      let amount = 100000000; // 0.1 sol in private sale
      const userTokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        user.publicKey
      );
      const staked_period = 3;

      const [userInfo, userInfoBump] = await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("USER_INFO_SEED"),
          user.publicKey.toBuffer()

        ],
        program.programId
      )

      const tx = await program.rpc.tokenSale(
        new anchor.BN(amount),
        staked_period, {
          accounts: {
            user: user.publicKey,
            userInfo,
            presale,
            vault,
            tokenMint,
            tokenAccount:userTokenAccount,
            tokenVaultAccount: tokenVault,
            tokenProgram:TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [user]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });
  it("claim 3 months token for presale", async() => {
    try {
      const userTokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        user.publicKey
      );
      const staked_period = 3;

      const [userInfo, userInfoBump] = await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("USER_INFO_SEED"),
          user.publicKey.toBuffer()

        ],
        program.programId
      )

      const tx = await program.rpc.claimStakedToken(
        staked_period, {
          accounts: {
            user: user.publicKey,
            userInfo,
            presale,
            vault,
            tokenMint,
            tokenAccount:userTokenAccount,
            tokenVaultAccount: tokenVault,
            tokenProgram:TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [user]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });


  it("convert sale into public", async() => {
    try {
      const tx = await program.rpc.updateSaleType(
        {
          accounts: {
            owner: owner.publicKey,
            presale
          },
          signers: [owner]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });
  it("update the rate", async() => {
    try {
      let rate = 5000;
      const tx = await program.rpc.updateRate(
        new anchor.BN(rate),
        {
          accounts: {
            owner: owner.publicKey,
            presale
          },
          signers: [owner]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });
  it("token sale for public", async() => {
    try {
      let amount = 10000000000; // 100 token in public sale
      const userTokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        user.publicKey
      );

      const [userInfo, userInfoBump] = await anchor.web3.PublicKey.findProgramAddress(
        [
          Buffer.from("USER_INFO_SEED"),
          user.publicKey.toBuffer()

        ],
        program.programId
      );
      const stakingPeriod = 0;

      const tx = await program.rpc.tokenSale(
        new anchor.BN(amount), 
        stakingPeriod,{
          accounts: {
            user: user.publicKey,
            presale,
            userInfo,
            vault,
            tokenMint,
            tokenAccount:userTokenAccount,
            tokenVaultAccount: tokenVault,
            tokenProgram:TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [user]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });
  it("withdraw token", async() => {
    try {
      const presaleData = await program.account.presale.fetch(presale);
      const tokenAmount = presaleData.tokenAmount;
      const tokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        owner.publicKey
      );

      const tx = await program.rpc.withdrawToken(
        tokenAmount, {
          accounts: {
            owner: owner.publicKey,
            presale,
            tokenMint,
            tokenAccount,
            tokenVaultAccount:tokenVault,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId
          },
          signers: [owner]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });
  it("withdraw sol", async() => {
    try {
      const presaleData = await program.account.presale.fetch(presale);
      const solAmount = presaleData.solAmount;
      const tokenAccount = await getAssociatedTokenAddress(
        tokenMint,
        owner.publicKey
      );

      const tx = await program.rpc.withdraw(
        solAmount, {
          accounts: {
            owner: owner.publicKey,
            presale,
            vault,
            systemProgram: SystemProgram.programId
          },
          signers: [owner]
        }
      );
      console.log("tx->", tx);
    } catch (error) {
      console.log(error);
    }
  });
});
