import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ideathon } from "../target/types/ideathon";
import { expect } from "chai";

describe("ideathon", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Ideathon as Program<Ideathon>;

  const eventId = new anchor.BN(123);
  const price = new anchor.BN(100000000);
  const reward = new anchor.BN(50);

  const [eventPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("event"), eventId.toArrayLike(Buffer, "le", 8)],
    program.programId
  );

  const [ticketPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("ticket"),
      provider.wallet.publicKey.toBuffer(),
      eventId.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [statsPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user_stats"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("1. Organizador cria o evento!", async () => {
    await program.methods
      .initializeEvent(eventId, price, reward)
      .accounts({
        eventAccount: eventPda,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    const account = await program.account.eventState.fetch(eventPda);
    expect(account.isActive).to.be.true;
  });

  it("2. Fã compra o ingresso!", async () => {
    await program.methods
      .buyTicket(eventId)
      .accounts({
        eventAccount: eventPda,
        ticketAccount: ticketPda,
        userStats: statsPda,
        buyer: provider.wallet.publicKey,
        organizer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    const ticket = await program.account.ticketAccount.fetch(ticketPda);
    expect(ticket.isUsed).to.be.false;
  });

  it("3. Check-in!", async () => {
    await program.methods
      .checkIn(eventId)
      .accounts({
        eventAccount: eventPda,
        ticketAccount: ticketPda,
        userStats: statsPda,
        organizer: provider.wallet.publicKey,
        ticketOwner: provider.wallet.publicKey,
      } as any)
      .rpc();

    const stats = await program.account.userStats.fetch(statsPda);
    const ticket = await program.account.ticketAccount.fetch(ticketPda);

    expect(ticket.isUsed).to.be.true;
    expect(stats.attendanceCount.toNumber()).to.equal(1);
  });
});