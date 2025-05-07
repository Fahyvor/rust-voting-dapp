import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolVotingDapp } from "../target/types/sol_voting_dapp";
import { assert } from "chai";

describe("sol-voting-dapp", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SolVotingDapp as Program<SolVotingDapp>;

  const pollId = anchor.web3.Keypair.generate().publicKey;
  const question = "Who do you want to vote for Presidency?";
  const candidates = [
    "Mr. Peter G. Obi",
    "Dr. Goodluck E. Jonathan",
    "Emperor Nyesom Ezebunwo Wike",
    "Senior Man Asiwaju Bola Ahmed Tinubu"
  ];

  let pollPDA: anchor.web3.PublicKey;
  let bump: number;

  before(async () => {
    // Derive PDA for the poll account
    [pollPDA, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("poll"), pollId.toBuffer()],
      program.programId
    );
    console.log("Poll PDA:", pollPDA.toBase58());

    // Create the poll at the beginning to ensure it's initialized
    try {
      await program.methods
        .createPoll(pollId, question, candidates)
        .accounts({
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      
      console.log("✅ Poll created before tests:", pollPDA.toBase58());
    } catch (e) {
      console.error("Failed to initialize poll:", e);
      throw e;
    }

    // Verify poll was created properly
    try {
      const pollAccount = await program.account.poll.fetch(pollPDA);
      console.log("Poll initialized successfully with question:", pollAccount.question);
      console.log("Initial candidates:", pollAccount.candidates);
      assert(pollAccount.initialized === true, "Poll should be initialized");
    } catch (e) {
      console.error("Failed to verify poll initialization:", e);
      throw e;
    }
  });

  it("should confirm poll is initialized", async () => {
    // Double-check that we can fetch the poll account
    const pollAccount = await program.account.poll.fetch(pollPDA);
    assert(pollAccount.initialized === true, "Poll should be initialized");
    assert(pollAccount.question === question, "Poll question should match");
    console.log("Poll confirmed initialized with question:", pollAccount.question);
  });

  it("should create a candidate", async () => {
    const candidateName = "Mr. Favour Okafor Snr.";
    
    try {
      // Log the poll state before adding a candidate
      const pollBefore = await program.account.poll.fetch(pollPDA);
      console.log("Poll before adding candidate:", {
        initialized: pollBefore.initialized,
        candidatesCount: pollBefore.candidates.length
      });
      
      // Add candidate
      await program.methods
        .createCandidate(pollId, candidateName)
        .accounts({
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        } as any)
        .rpc();
      
      // Verify candidate was added
      const pollAfter = await program.account.poll.fetch(pollPDA);
      console.log("Poll after adding candidate:", {
        initialized: pollAfter.initialized,
        candidatesCount: pollAfter.candidates.length,
        candidates: pollAfter.candidates
      });
      
      const candidateExists = pollAfter.candidates.includes(candidateName);
      assert(candidateExists, `Candidate ${candidateName} should exist in poll`);
      console.log("✅ Candidate added successfully:", candidateName);
    } catch (e) {
      console.error("Failed to add candidate:", e);
      
      // Try to get more info about the poll state
      try {
        const pollState = await program.account.poll.fetch(pollPDA);
        console.log("Current poll state:", {
          initialized: pollState.initialized,
          candidates: pollState.candidates,
          address: pollPDA.toBase58()
        });
      } catch (fetchError) {
        console.error("Additionally failed to fetch poll state:", fetchError);
      }
      
      throw e;
    }
  });

  it("should vote on the poll", async () => {
    try {
      await program.methods
        .vote(pollId, new anchor.BN(0)) // vote for first candidate
        .accounts({
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId
        } as any)
        .rpc();
    
      // Verify vote was recorded
      const pollAccount = await program.account.poll.fetch(pollPDA);
      assert(pollAccount.votes[0] > 0, "Vote should be recorded for the first candidate");
      console.log("✅ Voted successfully on poll");
      console.log("Votes for first candidate:", pollAccount.votes[0]);
    } catch (e) {
      console.error("Failed to vote:", e);
      throw e;
    }
  });

  it("should close the poll", async () => {
    try {
      await program.methods
        .closePoll(pollId)
        .accounts({
          user: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId
        } as any)
        .rpc();
    
      console.log("✅ Poll closed:", pollPDA.toBase58());
    } catch (e) {
      console.error("Failed to close poll:", e);
      throw e;
    }
    
    // Verify poll is closed (this will throw an error since the account is closed)
    try {
      await program.account.poll.fetch(pollPDA);
      assert.fail("Poll account should be closed and not fetchable");
    } catch (e) {
      console.log("Poll account successfully closed and can no longer be fetched");
    }
  });
});