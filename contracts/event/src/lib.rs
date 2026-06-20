#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, String,
    Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Event {
    pub id: u64,
    pub name: String,
    pub admin: Address,
    pub balance: u128,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct ExpenseProposal {
    pub id: u64,
    pub event_id: u64,
    pub requester: Address,
    pub description: String,
    pub amount: u128,
    pub approved: bool,
    pub paid: bool,
}

const EVENTS: Symbol = symbol_short!("EVENTS");
const PROPOSALS: Symbol = symbol_short!("PROPOSAL");

#[contract]
pub struct EventTreasuryContract;

#[contractimpl]
impl EventTreasuryContract {

    // =========================
    // EVENT MANAGEMENT
    // =========================

    pub fn create_event(
        env: Env,
        name: String,
        admin: Address,
    ) -> String {

        admin.require_auth();

        let mut events: Vec<Event> = env
            .storage()
            .instance()
            .get(&EVENTS)
            .unwrap_or(Vec::new(&env));

        let event = Event {
            id: env.prng().gen::<u64>(),
            name,
            admin,
            balance: 0,
        };

        events.push_back(event);

        env.storage().instance().set(&EVENTS, &events);

        String::from_str(&env, "Event berhasil dibuat")
    }

    pub fn get_events(env: Env) -> Vec<Event> {
        env.storage()
            .instance()
            .get(&EVENTS)
            .unwrap_or(Vec::new(&env))
    }

    // =========================
    // TREASURY
    // =========================

    pub fn deposit_fund(
        env: Env,
        event_id: u64,
        amount: u128,
    ) -> String {

        let mut events: Vec<Event> = env
            .storage()
            .instance()
            .get(&EVENTS)
            .unwrap_or(Vec::new(&env));

        for i in 0..events.len() {

            let mut event = events.get(i).unwrap();

            if event.id == event_id {

                event.balance += amount;

                events.set(i, event);

                env.storage().instance().set(&EVENTS, &events);

                return String::from_str(
                    &env,
                    "Dana berhasil ditambahkan",
                );
            }
        }

        String::from_str(&env, "Event tidak ditemukan")
    }

    // =========================
    // PROPOSAL
    // =========================

    pub fn create_proposal(
        env: Env,
        event_id: u64,
        requester: Address,
        description: String,
        amount: u128,
    ) -> String {

        requester.require_auth();

        let mut proposals: Vec<ExpenseProposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap_or(Vec::new(&env));

        let proposal = ExpenseProposal {
            id: env.prng().gen::<u64>(),
            event_id,
            requester,
            description,
            amount,
            approved: false,
            paid: false,
        };

        proposals.push_back(proposal);

        env.storage()
            .instance()
            .set(&PROPOSALS, &proposals);

        String::from_str(
            &env,
            "Proposal berhasil dibuat",
        )
    }

    pub fn get_proposals(
        env: Env,
    ) -> Vec<ExpenseProposal> {

        env.storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap_or(Vec::new(&env))
    }

    // =========================
    // APPROVAL
    // =========================

    pub fn approve_proposal(
        env: Env,
        proposal_id: u64,
        admin: Address,
    ) -> String {

        admin.require_auth();

        let events: Vec<Event> = env
            .storage()
            .instance()
            .get(&EVENTS)
            .unwrap_or(Vec::new(&env));

        let mut proposals: Vec<ExpenseProposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap_or(Vec::new(&env));

        for i in 0..proposals.len() {

            let mut proposal =
                proposals.get(i).unwrap();

            if proposal.id == proposal_id {

                let mut is_admin = false;

                for j in 0..events.len() {

                    let event =
                        events.get(j).unwrap();

                    if event.id
                        == proposal.event_id
                        && event.admin == admin
                    {
                        is_admin = true;
                        break;
                    }
                }

                if !is_admin {
                    return String::from_str(
                        &env,
                        "Bukan admin event",
                    );
                }

                proposal.approved = true;

                proposals.set(i, proposal);

                env.storage()
                    .instance()
                    .set(&PROPOSALS, &proposals);

                return String::from_str(
                    &env,
                    "Proposal disetujui",
                );
            }
        }

        String::from_str(
            &env,
            "Proposal tidak ditemukan",
        )
    }

    // =========================
    // PAYOUT
    // =========================

    pub fn payout(
        env: Env,
        proposal_id: u64,
        admin: Address,
    ) -> String {

        admin.require_auth();

        let mut events: Vec<Event> = env
            .storage()
            .instance()
            .get(&EVENTS)
            .unwrap_or(Vec::new(&env));

        let mut proposals: Vec<ExpenseProposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap_or(Vec::new(&env));

        for p in 0..proposals.len() {

            let mut proposal =
                proposals.get(p).unwrap();

            if proposal.id == proposal_id {

                if !proposal.approved {

                    return String::from_str(
                        &env,
                        "Proposal belum disetujui",
                    );
                }

                if proposal.paid {

                    return String::from_str(
                        &env,
                        "Proposal sudah dibayar",
                    );
                }

                for e in 0..events.len() {

                    let mut event =
                        events.get(e).unwrap();

                    if event.id
                        == proposal.event_id
                    {

                        if event.admin != admin {

                            return String::from_str(
                                &env,
                                "Bukan admin",
                            );
                        }

                        if event.balance
                            < proposal.amount
                        {

                            return String::from_str(
                                &env,
                                "Saldo tidak cukup",
                            );
                        }

                        event.balance -=
                            proposal.amount;

                        proposal.paid = true;

                        events.set(e, event);
                        proposals.set(p, proposal);

                        env.storage()
                            .instance()
                            .set(&EVENTS, &events);

                        env.storage()
                            .instance()
                            .set(
                                &PROPOSALS,
                                &proposals,
                            );

                        return String::from_str(
                            &env,
                            "Payout berhasil",
                        );
                    }
                }
            }
        }

        String::from_str(
            &env,
            "Proposal tidak ditemukan",
        )
    }
}