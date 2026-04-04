# ⚡ HYPE Protocol — Integração Frontend

## Informações do Contrato

| | |
|---|---|
| **Program ID** | `EFr4g6v9FbL1MHexWPAjmoEtaoDsrYKuztF47EhpZSbu` |
| **Mint $HYPE** | `4V43aWiupprp3QVrifUDPu2HNRatBzPY5RX4A76Fygx9` |
| **Rede** | Solana Devnet |
| **RPC URL** | `https://api.devnet.solana.com` |

---

## Dependências
```bash
npm install @coral-xyz/anchor @solana/web3.js @solana/spl-token @solana/wallet-adapter-react @solana/wallet-adapter-phantom
```

---

## Setup da Conexão
```typescript
import * as anchor from "@coral-xyz/anchor";
import { Connection, PublicKey, clusterApiUrl } from "@solana/web3.js";

const PROGRAM_ID = new PublicKey("EFr4g6v9FbL1MHexWPAjmoEtaoDsrYKuztF47EhpZSbu");
const HYPE_MINT = new PublicKey("4V43aWiupprp3QVrifUDPu2HNRatBzPY5RX4A76Fygx9");
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
```

---

## Seeds — Derivar os PDAs
```typescript
import { BN } from "@coral-xyz/anchor";

const eventId = new BN(1); // ID do evento

// PDA do Evento
const [eventPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("event"), eventId.toArrayLike(Buffer, "le", 8)],
  PROGRAM_ID
);

// PDA do Ingresso (por comprador)
const [ticketPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("ticket"),
    walletPublicKey.toBuffer(),
    eventId.toArrayLike(Buffer, "le", 8),
  ],
  PROGRAM_ID
);

// PDA do Histórico do Usuário
const [statsPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("user_stats"), walletPublicKey.toBuffer()],
  PROGRAM_ID
);
```

---

## Funções do Contrato

### 1. Criar Evento (organizador)
```typescript
await program.methods
  .initializeEvent(eventId, price, reward)
  .accounts({
    eventAccount: eventPda,
    authority: wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### 2. Comprar Ingresso (participante)
```typescript
await program.methods
  .buyTicket(eventId)
  .accounts({
    eventAccount: eventPda,
    ticketAccount: ticketPda,
    userStats: statsPda,
    buyer: wallet.publicKey,
    organizer: organizerPublicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();
```

### 3. Check-in (organizador valida)
```typescript
await program.methods
  .checkIn(eventId)
  .accounts({
    eventAccount: eventPda,
    ticketAccount: ticketPda,
    userStats: statsPda,
    organizer: wallet.publicKey,
    ticketOwner: participantPublicKey,
  })
  .rpc();
```

---

## Estrutura dos Dados On-chain
```typescript
// EventState
{
  organizer: PublicKey,
  price: BN,           // em lamports
  reward_amount: BN,
  is_active: boolean,
  event_id: BN,
  total_checked_in: BN,
  winner: PublicKey | null,
}

// TicketAccount
{
  owner: PublicKey,
  event_id: BN,
  is_used: boolean,
}

// UserStats
{
  attendance_count: BN, // acumula entre eventos — 3+ = 10% desconto
}
```

---

## IDL

O arquivo `ideathon.json` está na pasta `target/idl/` do repositório do Bruno.

---

## Explorer

- **Program:** https://explorer.solana.com/address/EFr4g6v9FbL1MHexWPAjmoEtaoDsrYKuztF47EhpZSbu?cluster=devnet
- **Token $HYPE:** https://explorer.solana.com/address/4V43aWiupprp3QVrifUDPu2HNRatBzPY5RX4A76Fygx9?cluster=devnet
