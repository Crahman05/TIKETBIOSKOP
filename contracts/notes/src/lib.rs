#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec,
};

// ================== STRUCT ==================
#[contracttype]
#[derive(Clone, Debug)]
pub struct Ticket {
    pub id: u64,
    pub movie_title: String,
    pub customer_name: String,
    pub seat: String,
    pub show_time: String,
    pub price: u64,
    pub status: Symbol, // BOOKED / USED / CANCELLED
}

// ================== STORAGE ==================
const TICKET_DATA: Symbol = symbol_short!("MOVIE");
const TICKET_COUNT: Symbol = symbol_short!("COUNT");

// ================== CONTRACT ==================
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    // 🔢 Generate ID
    fn generate_id(env: &Env) -> u64 {
        let mut count: u64 = env.storage()
            .instance()
            .get(&TICKET_COUNT)
            .unwrap_or(0);

        count += 1;
        env.storage().instance().set(&TICKET_COUNT, &count);
        count
    }

    // ================== CREATE ==================
    pub fn create_ticket(
        env: Env,
        movie_title: String,
        customer_name: String,
        seat: String,
        show_time: String,
        price: u64,
    ) -> String {

        if movie_title.len() == 0 || customer_name.len() == 0 {
            return String::from_str(&env, "Data tidak boleh kosong");
        }

        let mut tickets: Vec<Ticket> = env.storage()
            .instance()
            .get(&TICKET_DATA)
            .unwrap_or(Vec::new(&env));

        // 🚫 VALIDASI KURSI (NO DOUBLE BOOKING)
        for i in 0..tickets.len() {
            let t = tickets.get(i).unwrap();
            if t.seat == seat && t.show_time == show_time && t.status == symbol_short!("BOOKED") {
                return String::from_str(&env, "Kursi sudah dipesan!");
            }
        }

        let ticket = Ticket {
            id: Self::generate_id(&env),
            movie_title,
            customer_name,
            seat,
            show_time,
            price,
            status: symbol_short!("BOOKED"),
        };

        tickets.push_back(ticket);
        env.storage().instance().set(&TICKET_DATA, &tickets);

        String::from_str(&env, "Tiket berhasil dibuat")
    }

    // ================== READ ==================
    pub fn get_tickets(env: Env) -> Vec<Ticket> {
        env.storage()
            .instance()
            .get(&TICKET_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // ================== UPDATE ==================
    pub fn update_ticket(
        env: Env,
        id: u64,
        movie_title: String,
        customer_name: String,
        seat: String,
        show_time: String,
        price: u64,
    ) -> String {

        let mut tickets: Vec<Ticket> = env.storage()
            .instance()
            .get(&TICKET_DATA)
            .unwrap_or(Vec::new(&env));

        // 🚫 VALIDASI KURSI SAAT UPDATE
        for i in 0..tickets.len() {
            let t = tickets.get(i).unwrap();
            if t.seat == seat && t.show_time == show_time && t.id != id {
                return String::from_str(&env, "Kursi sudah dipakai!");
            }
        }

        for i in 0..tickets.len() {
            let mut ticket = tickets.get(i).unwrap();

            if ticket.id == id {
                ticket.movie_title = movie_title;
                ticket.customer_name = customer_name;
                ticket.seat = seat;
                ticket.show_time = show_time;
                ticket.price = price;

                tickets.set(i, ticket);
                env.storage().instance().set(&TICKET_DATA, &tickets);

                return String::from_str(&env, "Tiket berhasil diupdate");
            }
        }

        String::from_str(&env, "Tiket tidak ditemukan")
    }

    // ================== UPDATE STATUS ==================
    pub fn update_status(env: Env, id: u64, status: Symbol) -> String {

        let mut tickets: Vec<Ticket> = env.storage()
            .instance()
            .get(&TICKET_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tickets.len() {
            let mut ticket = tickets.get(i).unwrap();

            if ticket.id == id {
                ticket.status = status;

                tickets.set(i, ticket);
                env.storage().instance().set(&TICKET_DATA, &tickets);

                return String::from_str(&env, "Status tiket diupdate");
            }
        }

        String::from_str(&env, "Tiket tidak ditemukan")
    }

    // ================== DELETE ==================
    pub fn delete_ticket(env: Env, id: u64) -> String {

        let mut tickets: Vec<Ticket> = env.storage()
            .instance()
            .get(&TICKET_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..tickets.len() {
            if tickets.get(i).unwrap().id == id {
                tickets.remove(i);

                env.storage().instance().set(&TICKET_DATA, &tickets);

                return String::from_str(&env, "Tiket berhasil dihapus");
            }
        }

        String::from_str(&env, "Tiket tidak ditemukan")
    }
}

mod test;