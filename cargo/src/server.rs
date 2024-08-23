use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, PartialEq, Debug)]
pub enum SeatStatus {
    Free,
    Reserved,
    Purchased,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Seat {
    pub status: SeatStatus,
}

#[derive(Clone, Debug)]
pub struct Row {
    pub number: u32,
    pub seats: Vec<Seat>,
}

#[derive(Clone, Debug)]
pub struct Zone {
    pub name: String,
    pub rows: Vec<Row>,
}

#[derive(Clone, Debug)]
pub struct Category {
    pub name: String,
    pub zones: Vec<Zone>,
}

pub struct SeatingStructure {
    pub categories: Vec<Category>,
}

impl SeatingStructure {
    pub fn new() -> Self {
        // Crear la estructura de asientos con datos más detallados
        let categories = vec![
            Category {
                name: "VIP".to_string(),
                zones: vec![
                    Zone {
                        name: "ZonaA".to_string(),
                        rows: (1..=5).map(|row_number| {
                            Row {
                                number: row_number,
                                seats: (1..=10).map(|_| Seat { status: SeatStatus::Free }).collect(),
                            }
                        }).collect(),
                    },
                    Zone {
                        name: "ZonaB".to_string(),
                        rows: (1..=5).map(|row_number| {
                            Row {
                                number: row_number,
                                seats: (1..=10).map(|_| Seat { status: SeatStatus::Free }).collect(),
                            }
                        }).collect(),
                    },
                ],
            },
            Category {
                name: "General".to_string(),
                zones: vec![
                    Zone {
                        name: "ZonaC".to_string(),
                        rows: (1..=10).map(|row_number| {
                            Row {
                                number: row_number,
                                seats: (1..=20).map(|_| Seat { status: SeatStatus::Free }).collect(),
                            }
                        }).collect(),
                    },
                    Zone {
                        name: "ZonaD".to_string(),
                        rows: (1..=10).map(|row_number| {
                            Row {
                                number: row_number,
                                seats: (1..=20).map(|_| Seat { status: SeatStatus::Free }).collect(),
                            }
                        }).collect(),
                    },
                ],
            },
        ];

        SeatingStructure { categories }
    }

    pub fn find_free_seats(&self, category_name: &str, seat_count: u32) -> Vec<(String, u32, u32)> {
        let mut seats = Vec::new();
        for category in &self.categories {
            if category.name == category_name {
                for zone in &category.zones {
                    for row in &zone.rows {
                        let free_seat_indices: Vec<u32> = row.seats.iter()
                            .enumerate()
                            .filter(|(_, s)| s.status == SeatStatus::Free)
                            .map(|(i, _)| i as u32 + 1)
                            .collect();

                        if free_seat_indices.len() >= seat_count as usize {
                            let start_seat = free_seat_indices[0];
                            let end_seat = (start_seat + seat_count - 1).min(*free_seat_indices.last().unwrap());
                            let seats_in_row: Vec<(String, u32, u32)> = (start_seat..=end_seat)
                                .map(|i| (zone.name.clone(), row.number, i))
                                .collect();
                            seats.extend(seats_in_row);
                            return seats;
                        }
                    }
                }
            }
        }
        seats
    }

    pub fn reserve_seats(&mut self, category_name: &str, seats: Vec<(String, u32, u32)>) -> bool {
        let mut reserved = false;
        for category in &mut self.categories {
            if category.name == category_name {
                for zone in &mut category.zones {
                    for row in &mut zone.rows {
                        let mut seats_to_reserve = Vec::new();
                        for (zone_name, row_num, seat_num) in &seats {
                            if *zone_name == zone.name && *row_num == row.number {
                                if let Some(index) = row.seats.iter().position(|s| s.status == SeatStatus::Free) {
                                    if *seat_num == index as u32 + 1 {
                                        seats_to_reserve.push(index);
                                    }
                                }
                            }
                        }
                        for index in seats_to_reserve {
                            row.seats[index].status = SeatStatus::Reserved;
                            reserved = true;
                        }
                    }
                }
            }
        }
        reserved
    }

    pub fn purchase_seats(&mut self, category_name: &str, seats: Vec<(String, u32, u32)>) -> bool {
        let mut purchased = false;
        for category in &mut self.categories {
            if category.name == category_name {
                for zone in &mut category.zones {
                    for row in &mut zone.rows {
                        let mut seats_to_purchase = Vec::new();
                        for (zone_name, row_num, seat_num) in &seats {
                            if *zone_name == zone.name && *row_num == row.number {
                                if let Some(index) = row.seats.iter().position(|s| s.status == SeatStatus::Reserved) {
                                    if *seat_num == index as u32 + 1 {
                                        seats_to_purchase.push(index);
                                    }
                                }
                            }
                        }
                        for index in seats_to_purchase {
                            row.seats[index].status = SeatStatus::Purchased;
                            purchased = true;
                        }
                    }
                }
            }
        }
        purchased
    }
}

pub fn start_server() {
    let seating_structure = Arc::new(Mutex::new(SeatingStructure::new()));

    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to address");
    println!("Server is listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let seating_structure = Arc::clone(&seating_structure);
                thread::spawn(move || {
                    handle_client(stream, seating_structure);
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream, seating_structure: Arc<Mutex<SeatingStructure>>) {
    let mut buffer = [0; 512];

    // Bucle para mantener la conexión abierta mientras el cliente envíe solicitudes
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    // Si no hay más datos, significa que el cliente ha cerrado la conexión
                    break;
                }

                let request = String::from_utf8_lossy(&buffer[..size]);
                let parts: Vec<&str> = request.trim().split_whitespace().collect();
                if parts.len() > 1 {
                    let command = parts[0];
                    let category_name = parts[1];
                    let seat_count: u32 = parts.get(2).and_then(|&s| s.parse().ok()).unwrap_or(0);

                    let mut seating = seating_structure.lock().unwrap();

                    let response = match command {
                        "find" => {
                            let seats = seating.find_free_seats(category_name, seat_count);
                            if seats.is_empty() {
                                "No free seats found".to_string()
                            } else {
                                seats.iter()
                                    .map(|(zone, row, seat)| format!("Zone: {}, Row: {}, Seat: {}", zone, row, seat))
                                    .collect::<Vec<_>>()
                                    .join("\n")
                            }
                        }
                        "reserve" => {
                            let seats = (2..parts.len())
                                .map(|i| (parts[i].to_string(), 1, i as u32 + 1)) // Simplificado para el ejemplo
                                .collect();
                            if seating.reserve_seats(category_name, seats) {
                                "Seats reserved successfully".to_string()
                            } else {
                                "Failed to reserve seats".to_string()
                            }
                        }
                        "purchase" => {
                            let seats = (2..parts.len())
                                .map(|i| (parts[i].to_string(), 1, i as u32 + 1)) // Simplificado para el ejemplo
                                .collect();
                            if seating.purchase_seats(category_name, seats) {
                                "Seats purchased successfully".to_string()
                            } else {
                                "Failed to purchase seats".to_string()
                            }
                        }
                        _ => "Unknown command".to_string(),
                    };

                    if let Err(e) = stream.write(response.as_bytes()) {
                        eprintln!("Failed to write to stream: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
}


