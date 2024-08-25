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
    pub vision_percentage: u8,
    pub number: u32,
}

#[derive(Clone, Debug)]
pub struct Row {
    pub number: u32,
    pub seats: Vec<Seat>,
}
//cantidad de asientos libres en una fila
impl Row {
    pub fn free_seats(&self) -> u32 {
        self.seats.iter().filter(|s| s.status == SeatStatus::Free).count() as u32
    }
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

#[derive(Debug)]
pub struct SeatingStructure {
    pub categories: Vec<Category>,
}

impl SeatingStructure {
    pub fn new() -> Self {
        // Crear la estructura de asientos con datos más detallados y porcentajes de visión variados
        let categories = vec![
            Category {
                name: "VIP".to_string(),
                zones: vec![
                    Zone {
                        name: "ZonaA".to_string(),
                        rows: (1..=5).map(|row_number| {
                            Row {
                                number: row_number,
                                seats: (1..=5).map(|i| Seat {
                                    number: i,
                                    status: SeatStatus::Free,
                                    vision_percentage : if i == 1 { 100 } else { 50 }
                                }).collect(),
                            }
                        }).collect(),
                    },
                    Zone {
                        name: "ZonaB".to_string(),
                        rows: (1..=5).map(|row_number| {
                            Row {
                                number: row_number,
                                seats: (1..=5).map(|i| Seat {
                                    number: i,
                                    status: SeatStatus::Free,
                                    vision_percentage: if i % 3 == 0 { 100 } else { 50 }, // Ejemplo de asignación de visión
                                }).collect(),
                            }
                        }).collect(),
                    },
                    Zone {
                        name: "ZonaC".to_string(),
                        rows: (1..=5).map(|row_number| {
                            Row {
                                number: row_number,
                                seats: (1..=5).map(|i| Seat {
                                    number: i,
                                    status: SeatStatus::Reserved,
                                    vision_percentage: if i % 4 == 0 { 100 } else { 25 }, // Ejemplo de asignación de visión
                                }).collect(),
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
                        rows: (1..=5).map(|row_number| {
                            Row {
                                number: row_number,
                                seats: (1..=5).map(|i| Seat {
                                    number: i,
                                    status: SeatStatus::Free,
                                    vision_percentage: if i % 4 == 0 { 100 } else { 50 }, // Ejemplo de asignación de visión
                                }).collect(),
                            }
                        }).collect(),
                    },
                    Zone {
                        name: "ZonaD".to_string(),
                        rows: (1..=5).map(|row_number| {
                            Row {
                                number: row_number,
                                seats: (1..=5).map(|i| Seat {
                                    number: i,
                                    status: SeatStatus::Free,
                                    vision_percentage: if i % 5 == 0 { 100 } else { 25 }, // Ejemplo de asignación de visión
                                }).collect(),
                            }
                        }).collect(),
                    },
                    Zone {
                        name: "ZonaE".to_string(),
                        rows: (1..=5).map(|row_number| {
                            Row {
                                number: row_number,
                                seats: (1..=5).map(|i| Seat {
                                    number: i,
                                    status: SeatStatus::Free,
                                    vision_percentage: if i % 6 == 0 { 100 } else { 75 }, // Ejemplo de asignación de visión
                                }).collect(),
                            }
                        }).collect(),
                    },
                ],
            },
        ];

        SeatingStructure { categories }
    }

    pub fn find_free_seats(&self, category_name: &str, seat_count: u32) -> Vec<String> {
        let mut result = vec![];
        let mut allOptions = vec![];
        let category = self.categories.iter().find(|c| c.name == category_name);
        if let Some(category) = category {
            for zone in &category.zones {
                let mut seats = vec![];
                for row in &zone.rows {
                    let mut free_seats = 0;
                    let mut free_seats_str = vec![];
                    let mut zone_name = zone.name.clone();
                    let mut row_number = row.number.to_string();
                    //hacer una libreria con key value para guardar los asientos

                    &seats.push("Zone: ".to_string() + &zone_name);
                    &seats.join(", ");
                    &seats.push("Row: ".to_string() + &row_number);
                    &seats.join(", ");
                    for seat in &row.seats {
                        if seat.status == SeatStatus::Free {
                            free_seats += 1;
                            free_seats_str.push(format!("Seat: {}", seat.number));
                        }
                        if free_seats >= seat_count {
                            &seats.push(free_seats_str.join(", "));
                            allOptions.push(seats.join(", "));
                            break;
                        }
                    }
                }
            }
        }

        if allOptions.is_empty() {
            allOptions.push("No free seats found".to_string());
        }
        for r in &allOptions {
            //imprimir la zona y fila de los asientos encontrados
            println!("{}", r);
        }
        result
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
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                let request = String::from_utf8_lossy(&buffer[..size]);
                println!("Received request: {}", request);
                println!("{:?}", request);
                let response = if request.starts_with("find") {
                    let parts: Vec<&str> = request.split_whitespace().collect();
                    if parts.len() == 3 {
                        let category = parts[1];
                        let seat_count = parts[2].parse::<u32>().unwrap_or(0);
                        let mut seating_structure = seating_structure.lock().unwrap();
                        seating_structure.find_free_seats(category, seat_count);
                        println!("{} {}", category, seat_count);
                        "Seats found".to_string()

                    } else {
                        "Invalid request. Expected: find <category> <seat_count>".to_string()
                    }
                } else if request.starts_with("reserve") {
                    let parts: Vec<&str> = request.split_whitespace().collect();
                    if parts.len() == 5 {
                        let zone = parts[1];
                        let row = parts[2];
                        let seat = parts[3];
                        let seat_count = parts[4].parse::<u32>().unwrap_or(0);
                        //cambiar el estado de los asientos a reservado
                        for i in 0..seat_count {
                            let mut seating_structure = seating_structure.lock().unwrap();
                            let category = seating_structure.categories.iter_mut().find(|c| c.name == zone);
                            if let Some(category) = category {
                                let zone = category.zones.iter_mut().find(|z| z.name == row);
                                if let Some(zone) = zone {
                                    let row = zone.rows.iter_mut().find(|r| r.number == seat.parse::<u32>().unwrap_or(0));
                                    if let Some(row) = row {
                                        let seat = row.seats.iter_mut().find(|s| s.status == SeatStatus::Free);
                                        if let Some(seat) = seat {
                                            seat.status = SeatStatus::Reserved;
                                        }
                                    }
                                }
                            }
                        }
                        println!("Reserving seats: {} {} {} {}", zone, row, seat, seat_count);
                        "Seats reserved".to_string()
                    } else {
                        "Invalid request. Expected: reserve <zone> <row> <seat> <seat_count>".to_string()
                    }
                } else {
                    "Invalid request".to_string()
                };

                if let Err(e) = stream.write(response.as_bytes()) {
                    eprintln!("Failed to write to stream: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
}


