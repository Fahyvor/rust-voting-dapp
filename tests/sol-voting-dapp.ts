import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolVotingDapp } from "../target/types/sol_voting_dapp";

describe("sol-voting-dapp", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolVotingDapp as Program<SolVotingDapp>;

  const poll = anchor.web3.Keypair.generate();
  const question = "What is your favourite programming language?";
  const options = ["Rust", "JavaScript", "Python", "Solidity"];

  it("should initialize and create a poll", async () => {
    await program.methods
      .createPoll(question, options)
      .accounts({
        poll: poll.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([poll, provider.wallet.payer])
      .rpc();

    console.log("Poll created:", poll.publicKey.toBase58());
  });

  it("should vote on the poll", async () => {
    await program.methods
      .vote(poll.publicKey, new anchor.BN(1))
      .accounts({
        poll: poll.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([provider.wallet.payer])
      .rpc();

    console.log("Vote cast on poll:", poll.publicKey.toBase58());
  });

  it("should close the poll", async () => {
    await program.methods
      .closePoll(poll.publicKey)
      .accounts({
        poll: poll.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([provider.wallet.payer])
      .rpc();

    console.log("Poll closed:", poll.publicKey.toBase58());
  });
});

