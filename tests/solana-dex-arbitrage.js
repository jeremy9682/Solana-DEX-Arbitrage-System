const anchor = require("@project-serum/anchor");

describe("solana-dex-arbitrage", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaDexArbitrage;

  it("Executes arbitrage", async () => {
    console.log("Test setup complete.");
  });
});