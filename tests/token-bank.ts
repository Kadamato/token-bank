import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenBank } from "../target/types/token_bank";
import { BN } from "bn.js";
import { PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("token-bank", async () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const wallet = provider.wallet;

  const program = anchor.workspace.TokenBank as Program<TokenBank>;

  const saleKeyPair = anchor.web3.Keypair.generate();
  const userKeyPair = anchor.web3.Keypair.generate();
  const directorKeyPair = anchor.web3.Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(
        require("fs").readFileSync(
          "/home/kadamato/.config/solana/id.json",
          "utf-8"
        )
      )
    )
  );
  const ROLE = {
    DIRECTOR: 1,
    SALE: 2,
    USER: 0,
  };

  const bank = {
    name: "SUPERTEAM BANK",
    interest: new BN(55),
  };

  const [bankAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bank")],
    program.programId
  );

  const [directorAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), wallet.publicKey.toBuffer()],
    program.programId
  );

  const [saleAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), saleKeyPair.publicKey.toBuffer()],
    program.programId
  );

  const [userAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), userKeyPair.publicKey.toBuffer()],
    program.programId
  );

  const [bankSolAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bank_sol")],
    program.programId
  );

  const amount = new BN(1000000000);
  const withdrawAmount = new BN(333333333);

  before(async () => {
    await Promise.all([
      airdropAccount(provider, userKeyPair.publicKey),
      airdropAccount(provider, saleKeyPair.publicKey),
      airdropAccount(provider, wallet.publicKey),
    ]);
  });

  //   initialize bank
  it("initialize bank!", async () => {
    const tx = await program.methods
      .initBank(bank.name, bank.interest)
      .accountsPartial({
        bank: bankAccount,
        owner: wallet.publicKey,
      })
      .rpc({ commitment: "confirmed" });

    console.log("Bank initialized with tx:", tx);
  });

  //  Initialize roles
  it("initialize director!", async () => {
    const tx = await program.methods
      .initUser()
      .accountsPartial({
        user: directorAccount,
        owner: wallet.publicKey,
      })
      .rpc({
        commitment: "confirmed",
      });

    console.log("Director initialized with tx:", tx);

    const account = await program.account.user.fetch(directorAccount);

    console.log("Director account:", account);
  });

  it("initialize sale!", async () => {
    const tx = await program.methods
      .initUser()
      .accountsPartial({
        user: saleAccount,
        owner: saleKeyPair.publicKey,
      })
      .signers([saleKeyPair])
      .rpc({
        commitment: "confirmed",
      });

    console.log("Sale account initialized with tx:", tx);

    const account = await program.account.user.fetch(saleAccount);

    console.log("Sale account:", account);
  });

  it("initialize user!", async () => {
    const tx = await program.methods
      .initUser()
      .accountsPartial({
        user: userAccount,
        owner: userKeyPair.publicKey,
      })
      .signers([userKeyPair])
      .rpc({
        commitment: "confirmed",
      });

    console.log("user account initialized with tx:", tx);

    const account = await program.account.user.fetch(userAccount);

    console.log("Sale account:", account);
  });

  // user deposit
  it(" user deposit bank!", async () => {
    //  before
    const accountBefore = await program.account.user.fetch(userAccount);
    console.log(
      "User deposited before:",
      accountBefore.depositAmountTotal.toNumber() / LAMPORTS_PER_SOL,
      "SOL"
    );

    const amount = new BN(2222222);
    const tx = await program.methods
      .deposit(amount)
      .accounts({
        bank: bankAccount,
        owner: userKeyPair.publicKey,
        bankSol: bankSolAccount,
        user: userAccount,
      })
      .signers([userKeyPair])
      .rpc({ commitment: "confirmed" });

    console.log(`Deposit: ${amount}`, tx);

    //  after
    const accountAfter = await program.account.user.fetch(userAccount);
    console.log(
      "User deposited after:",
      accountAfter.depositAmountTotal.toNumber() / LAMPORTS_PER_SOL,
      "SOL"
    );
  });

  it(" user withdraw", async () => {
    //  withdrawn before
    const accountBefore = await program.account.user.fetch(userAccount);
    console.log(
      "User withdrawn before:",
      accountBefore.withdrawTotal.toNumber() / LAMPORTS_PER_SOL,
      "SOL"
    );

    //  deposit before
    const accountDepositBefore = await program.account.user.fetch(userAccount);
    console.log(
      "Deposit before:",
      accountDepositBefore.depositAmountTotal.toNumber() / LAMPORTS_PER_SOL,
      "SOL"
    );

    const tx = await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        bank: bankAccount,
        owner: userKeyPair.publicKey,
        user: userAccount,
        bankSol: bankSolAccount,
      })
      .signers([userKeyPair])
      .rpc({ commitment: "confirmed" });

    console.log("Cannot withdraw because not enough days");

    // after
    const accountAfter = await program.account.user.fetch(userAccount);
    console.log(
      "User withdrawn after:",
      accountAfter.withdrawTotal.toNumber() / LAMPORTS_PER_SOL,
      "SOL"
    );
  });

  //  sale withdraw
  it("sale withdraw", async () => {
    //  before
    const accountBefore = await program.account.user.fetch(saleAccount);
    console.log(
      "Sale withdrawn before:",
      accountBefore.withdrawTotal.toNumber() / LAMPORTS_PER_SOL,
      "SOL"
    );

    const tx = await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        bank: bankAccount,
        owner: saleKeyPair.publicKey,
        user: saleAccount,
        bankSol: bankSolAccount,
      })
      .signers([saleKeyPair])
      .rpc({ commitment: "confirmed" });

    const account = await program.account.user.fetch(saleAccount);
    const bank = await program.account.bank.fetch(bankAccount);

    console.log(
      "Sale withdrawn after:",
      account.withdrawTotal.toNumber() / LAMPORTS_PER_SOL
    );
    console.log(
      "Bank account:",
      bank.totalBalance.toNumber() / LAMPORTS_PER_SOL
    );
  });

  //  director withdraw
  // it("director  withdraw all", async () => {
  //   //  withdrawn before
  //   const directorBefore = await program.account.user.fetch(directorAccount);
  //   console.log(
  //     "Director withdrawn before:",
  //     directorBefore.withdrawTotal.toNumber() / LAMPORTS_PER_SOL,
  //     "SOL"
  //   );

  //   const tx = await program.methods
  //     .withdrawAll()
  //     .accountsPartial({
  //       bank: bankAccount,
  //       bankSol: bankSolAccount,
  //       owner: wallet.publicKey,
  //       user: directorAccount,
  //     })
  //     .rpc({ commitment: "confirmed" });

  //   console.log("tx", tx);
  //   // after
  //   const directorAfter = await program.account.user.fetch(directorAccount);
  //   console.log(
  //     "Director withdrawn after:",
  //     directorAfter.withdrawTotal.toNumber() / LAMPORTS_PER_SOL,
  //     "SOL"
  //   );

  //   const bank = await program.account.bank.fetch(bankAccount);

  //   console.log(
  //     "Bank balance after withdrawn by director",
  //     bank.totalBalance.toNumber() / LAMPORTS_PER_SOL,
  //     "SOL"
  //   );
  // });

  // update bank
  it("update bank", async () => {
    const new_bank = {
      name: "SUPERTEAM_BANK_NEW",
      interest: new BN(48),
    };
    const tx = await program.methods
      .updateBank(new_bank.name, new_bank.interest)
      .accountsPartial({
        bank: bankAccount,
        user: directorAccount,
        owner: wallet.publicKey,
      })
      .rpc({
        commitment: "confirmed",
      });

    console.log("tx", tx);

    const bank = await program.account.bank.fetch(bankAccount);
    console.log(
      "Bank updated:",
      bank.name,
      "===>",
      bank.interestRate.toNumber(),
      "SOL"
    );
  });

  //  grant permission
  it("grant permission", async () => {
    try {
      console.log("Director account:", directorKeyPair.publicKey.toBase58());
      const permission = 2; // add sale
      const tx = await program.methods
        .grantPermission(permission)
        .accountsPartial({
          bank: bankAccount,
          user: saleAccount,
          owner: directorKeyPair.publicKey,
        })
        .signers([directorKeyPair])
        .rpc({
          commitment: "confirmed",
        });

      const account = await program.account.user.fetch(userAccount);

      console.log("Permission granted:", account.role);
    } catch (error) {
      console.log(error);
    }
  });

  it("sale withdraw after grant permission", async () => {
    //  before
    const accountBefore = await program.account.user.fetch(saleAccount);
    console.log(
      "Sale withdrawn before:",
      accountBefore.withdrawTotal.toNumber() / LAMPORTS_PER_SOL,
      "SOL"
    );

    const tx = await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        bank: bankAccount,
        owner: saleKeyPair.publicKey,
        user: saleAccount,
        bankSol: bankSolAccount,
      })
      .signers([saleKeyPair])
      .rpc({ commitment: "confirmed" });

    const account = await program.account.user.fetch(saleAccount);
    const bank = await program.account.bank.fetch(bankAccount);

    console.log(
      "Sale withdrawn after:",
      account.withdrawTotal.toNumber() / LAMPORTS_PER_SOL
    );
    console.log(
      "Bank account:",
      bank.totalBalance.toNumber() / LAMPORTS_PER_SOL
    );
  });
});

const airdropAccount = async (provider, accountPublicKey: PublicKey) => {
  const signature = await provider.connection.requestAirdrop(
    new PublicKey(accountPublicKey),
    2 * LAMPORTS_PER_SOL
  );
  await provider.connection.confirmTransaction(signature);
  console.log(`Airdrop successful for account: ${accountPublicKey}`);
};
