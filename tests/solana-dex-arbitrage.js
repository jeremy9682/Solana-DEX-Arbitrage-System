const anchor = require("@project-serum/anchor");

describe("solana-dex-arbitrage", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolanaDexArbitrage;

  it("Executes arbitrage on devnet", async () => {
    const tx = await program.rpc.executeArbitrage(
      new anchor.BN(1000000),  // amount_in
      new anchor.BN(50000),    // min_profit
      new anchor.BN(50),       // max_slippage_bps
      {
        accounts: {
          user: provider.wallet.publicKey,
          userTokenA: /* 用户代币 A 账户 */,
          userTokenB: /* 用户代币 B 账户 */,
          orcaPool: /* Orca 池地址 */,
          orcaProgram: "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP",
          meteoraPool: /* Meteora 池地址 */,
          meteoraProgram: "MERLuDFBMmsHnsBPZw2sDQZHvJsm3fk3dngFXkQV4i",
          serumMarket: /* Serum 市场地址 */,
          serumProgram: "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin",
          tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      }
    );
    console.log("Transaction signature:", tx);
  });
});