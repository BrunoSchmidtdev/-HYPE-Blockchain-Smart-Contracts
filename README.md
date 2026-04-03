# ⚡ HYPE — Blockchain & Smart Contracts

> *Você esteve lá. A blockchain prova isso.*

---

## O que é a HYPE?

A **HYPE** é um protocolo de ingressos e presença construído na **Solana**, onde cada ingresso comprado, cada check-in feito e cada conquista acumulada fica registrado de forma **imutável e descentralizada** na blockchain.

Chega de certificado velho que ninguém lembra. Chega de ingresso falso. Chega de prêmio que demora meses para cair. Na HYPE, tudo acontece on-chain — transparente, instantâneo e para sempre.

---

## O Problema

Hoje, quem vai a hackathons e ideathons enfrenta:

- 🎟️ **Ingresso fácil de falsificar** — sem garantia real de autenticidade
- 👻 **Participante some pós-evento** — sem histórico, sem reconhecimento
- ⏳ **Prêmios demoram semanas ou meses** para serem pagos
- 📊 **Organizadores no escuro** — não sabem quem pagou e não foi, quem realmente apareceu
- 🏆 **Fidelidade ignorada** — quem vai em 10 eventos é tratado igual a quem vai no primeiro

---

## A Solução — Smart Contracts na Solana

Três instruções. Tudo on-chain. Zero intermediários.

### `initialize_event` — Criar Evento
O organizador registra o evento na blockchain com preço, recompensa e ID único. A partir desse momento, o evento existe de forma imutável.

```rust
pub fn initialize_event(ctx, event_id: u64, price: u64, reward: u64)
```

### `buy_ticket` — Comprar Ingresso
O participante paga em SOL e recebe um ingresso digital único na blockchain. Sem falsificação possível.

**Fidelidade automática:** quem foi em 3 ou mais eventos anteriores recebe **10% de desconto** — sem cupom, sem cadastro, sem burocracia. A blockchain já sabe quem você é.

```rust
pub fn buy_ticket(ctx, event_id: u64)
```

### `check_in` — Confirmar Presença
O organizador valida a entrada. O check-in é registrado na blockchain — o participante nunca mais "some" pós-evento.

```rust
pub fn check_in(ctx, event_id: u64)
```

---

## Por que Solana?

- ⚡ **Transações em milissegundos** — check-in instantâneo
- 💸 **Taxa de ~$0,00025 por transação** — acessível para qualquer evento
- 🔒 **Imutabilidade garantida** — ninguém apaga o que aconteceu
- 🌐 **Descentralizado** — não depende de nenhuma empresa ou servidor

---

## Arquitetura dos Contratos

```
programs/
└── ideathon/
    └── src/
        └── lib.rs          # Smart contract principal
```

**Accounts (PDAs):**

| Account | Seeds | Descrição |
|---|---|---|
| `EventState` | `["event", event_id]` | Estado do evento |
| `TicketAccount` | `["ticket", buyer, event_id]` | Ingresso único por participante |
| `UserStats` | `["user_stats", user]` | Histórico de presença do usuário |

**Estrutura de dados:**

```rust
EventState {
    organizer: Pubkey,
    price: u64,
    reward_amount: u64,
    is_active: bool,
    event_id: u64,
    total_checked_in: u64,
}

TicketAccount {
    owner: Pubkey,
    event_id: u64,
    is_used: bool,
}

UserStats {
    attendance_count: u64,  // acumula para sempre
}
```

---

## Deploy

**Rede:** Solana Devnet

**Program ID:**
```
EFr4g6v9FbL1MHexWPAjmoEtaoDsrYKuztF47EhpZSbu
```

**Verificar no Explorer:**
[explorer.solana.com — HYPE Program](https://explorer.solana.com/address/EFr4g6v9FbL1MHexWPAjmoEtaoDsrYKuztF47EhpZSbu?cluster=devnet)

---

## Como rodar localmente

**Pré-requisitos:**
- Rust 1.85.0
- Solana CLI 1.18.26
- Anchor CLI 0.32.1
- Node.js + Yarn

**Instalação:**
```bash
yarn install
anchor build
anchor deploy
```

**Testes:**
```bash
anchor test
```

---

## Stack Técnica

| Tecnologia | Versão | Uso |
|---|---|---|
| Rust | 1.85.0 | Linguagem dos smart contracts |
| Anchor | 0.32.1 | Framework para Solana |
| Solana | 1.18.26 | Blockchain |
| TypeScript | 5.7.3 | Testes e integração |

---

## O que vem a seguir

- [ ] **Token $HYPE** — moeda nativa do protocolo para pagamento de ingressos
- [ ] **Pagamento multi-moeda** — aceitar SOL, USDC e outros tokens SPL
- [ ] **NFT de presença** — cada check-in gera um NFT colecionável
- [ ] **Prêmio automático** — ao vencer um hackathon, o prêmio cai na carteira instantaneamente via SC

---

*Construído na Solana Ideathon 2026 — porque sua presença merece ser eterna.* ⚡
