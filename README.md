# EventTreasury

**EventTreasury** - Decentralized Event Fund & Expense Management System on Stellar

## Project Description

EventTreasury is a decentralized treasury management smart contract built on the Stellar blockchain using the Soroban SDK. The application enables event organizers to transparently manage event funds, track budgets, submit expense requests, approve expenditures, and monitor treasury activities directly on-chain.

Traditional event fund management often relies on spreadsheets, messaging applications, and manual approvals, which can lead to miscommunication, lack of transparency, and inefficient financial oversight. EventTreasury addresses these challenges by providing a trustless and auditable system where all treasury operations are recorded on the blockchain.

The contract allows organizers to create events, deposit funds into an event treasury, receive expense proposals from committee members, approve spending requests, and execute payouts while maintaining a complete on-chain record of all financial activities.

## Project Vision

Our vision is to modernize financial management for events and organizations by leveraging blockchain technology.

We aim to:

* **Increase Transparency**: Ensure all event funds and expenditures are visible and verifiable on-chain.
* **Improve Accountability**: Require approvals before treasury funds can be spent.
* **Reduce Administrative Complexity**: Automate treasury workflows and expense management.
* **Build Trust Among Stakeholders**: Allow organizers, sponsors, and committee members to verify financial activities independently.
* **Create a Scalable Financial Infrastructure**: Enable future integration with payments, sponsorship systems, payroll, and decentralized governance.

EventTreasury is designed to serve student organizations, hackathons, conferences, workshops, community events, and professional event organizers.

## Key Features

### 1. Event Creation

* Create and manage multiple events.
* Assign an event administrator.
* Initialize dedicated treasury balances for each event.
* Store event information directly on-chain.

### 2. Treasury Management

* Deposit funds into an event treasury.
* Track treasury balances transparently.
* Maintain a secure and immutable financial record.
* Support multiple funding sources such as sponsors and contributors.

### 3. Expense Proposal System

* Committee members can submit spending requests.
* Include expense descriptions and requested amounts.
* Create a structured approval workflow.
* Maintain a history of submitted proposals.

### 4. Approval Workflow

* Event administrators review expense proposals.
* Approve or reject spending requests.
* Prevent unauthorized treasury withdrawals.
* Ensure financial accountability before fund allocation.

### 5. Treasury Payouts

* Execute payouts only after approval.
* Automatically deduct approved expenses from treasury balances.
* Prevent duplicate payments.
* Track completed disbursements on-chain.

### 6. Blockchain Transparency

* All treasury actions are recorded on Stellar.
* Event data remains immutable and auditable.
* Financial activities can be independently verified.
* Eliminates dependence on centralized financial tracking systems.

## Smart Contract Functions

### Event Management

#### `create_event()`

Creates a new event treasury.

Parameters:

* Event Name
* Admin Address

#### `get_events()`

Retrieves all registered events.

### Treasury Management

#### `deposit_fund()`

Adds funds to an event treasury.

Parameters:

* Event ID
* Amount

### Proposal Management

#### `create_proposal()`

Creates a new expense request.

Parameters:

* Event ID
* Requester Address
* Description
* Amount

#### `get_proposals()`

Retrieves all submitted proposals.

### Approval Management

#### `approve_proposal()`

Approves an expense proposal.

Parameters:

* Proposal ID
* Admin Address

### Payout Management

#### `payout()`

Executes an approved treasury payment.

Parameters:

* Proposal ID
* Admin Address

## Contract Details

* Network: Stellar Testnet
* Smart Contract ID: CDRXVRH35IBYSE4U2UE37N6JT7AIOAQYX6JYTX3LQZK2T5KDKJ3MYDLT

## Future Scope

### Short-Term Enhancements

1. Multi-Admin Approval System

   * Require multiple organizers to approve large expenses.
   * Support configurable approval thresholds.

2. Sponsor Contributions

   * Allow sponsors to contribute directly to event treasuries.
   * Generate transparent sponsorship records.

3. Expense Categories

   * Marketing
   * Venue
   * Logistics
   * Catering
   * Operations

4. Real Token Transfers

   * Integrate Stellar USDC and other Stellar assets.
   * Enable actual on-chain payments.

### Medium-Term Development

5. Committee Roles & Permissions

   * Treasurer
   * Event Manager
   * Finance Team
   * Committee Member

6. Budget Allocation

   * Department-level budgets.
   * Spending limits per category.

7. Financial Analytics Dashboard

   * Treasury balance monitoring.
   * Expense reporting.
   * Budget utilization tracking.

8. Notification System

   * Proposal submission alerts.
   * Approval notifications.
   * Treasury activity updates.

### Long-Term Vision

9. Decentralized Event DAO

   * Community-driven treasury governance.
   * Voting-based expense approvals.

10. Cross-Organization Treasury Management

* Manage multiple organizations within one platform.
* Shared treasury infrastructure.

11. Grant & Sponsorship Marketplace

* Connect sponsors with event organizers.
* On-chain funding and milestone tracking.

12. Payroll Integration

* Automate payments to event staff and contributors.
* Scheduled compensation distribution.

13. AI-Powered Budget Assistant

* Detect unusual spending patterns.
* Provide budget optimization recommendations.
* Forecast treasury requirements.

14. Mainnet Deployment

* Production-ready infrastructure.
* Real-world event adoption.
* Enterprise-grade security review.

## Technical Requirements

* Rust
* Soroban SDK
* Stellar Testnet
* Stellar CLI

## Getting Started

Deploy the smart contract to Stellar Soroban Testnet and interact with the following core functions:

* `create_event()`
* `get_events()`
* `deposit_fund()`
* `create_proposal()`
* `get_proposals()`
* `approve_proposal()`
* `payout()`

## Use Cases

* Student Organizations
* University Events
* Hackathons
* Community Meetups
* Conferences
* Workshops
* Non-Profit Organizations
* Professional Event Organizers

---

**EventTreasury** — Bringing Transparent and Decentralized Financial Management to Events on Stellar.
