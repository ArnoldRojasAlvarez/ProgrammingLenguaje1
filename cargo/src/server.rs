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
pub struct Vip {
    pub seats: Vec<Seat>,
}

#[derive(Clone, Debug)]
pub struct General {
    pub seats: Vec<Seat>,
}


#[derive(Clone, Debug)]
pub struct Zone {
    pub name: String,
    pub Vip: Vip,
    pub General: General,
}

#[derive(Clone, Debug)]
pub struct Category {
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
                zones: vec![
                    Zone {
                        name: "ZonaA".to_string(),
                        Vip: Vip {
                            seats: vec![
                                Seat {
                                    status: SeatStatus::Purchased,
                                    vision_percentage: 100,
                                    number: 1,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 100,
                                    number: 2,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 100,
                                    number: 3,
                                },
                                Seat {
                                    status: SeatStatus::Purchased,
                                    vision_percentage: 100,
                                    number: 4,
                                },
                                Seat {
                                    status: SeatStatus::Reserved,
                                    vision_percentage: 100,
                                    number: 5,
                                },
                            ],
                        },
                        General: General {
                            seats: vec![
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 80,
                                    number: 1,
                                },
                                Seat {
                                    status: SeatStatus::Reserved,
                                    vision_percentage: 80,
                                    number: 2,
                                },
                                Seat {
                                    status: SeatStatus::Reserved,
                                    vision_percentage: 80,
                                    number: 3,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 80,
                                    number: 4,
                                },
                                Seat {
                                    status: SeatStatus::Reserved,
                                    vision_percentage: 80,
                                    number: 5,
                                },
                            ],
                        },
                    },
                    Zone {
                        name: "ZonaB".to_string(),
                        Vip: Vip {
                            seats: vec![
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 95,
                                    number: 1,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 95,
                                    number: 2,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 95,
                                    number: 3,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 95,
                                    number: 4,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 95,
                                    number: 5,
                                },
                            ],
                        },
                        General: General {
                            seats: vec![
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 75,
                                    number: 1,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 75,
                                    number: 2,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 75,
                                    number: 3,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 75,
                                    number: 4,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 75,
                                    number: 5,
                                },
                            ],
                        },
                    },
                    Zone {
                        name: "ZonaC".to_string(),
                        Vip: Vip {
                            seats: vec![
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 90,
                                    number: 1,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 90,
                                    number: 2,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 90,
                                    number: 3,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 90,
                                    number: 4,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 90,
                                    number: 5,
                                },
                            ],
                        },
                        General: General {
                            seats: vec![
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 90,
                                    number: 1,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 90,
                                    number: 2,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 90,
                                    number: 3,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 90,
                                    number: 4,
                                },
                                Seat {
                                    status: SeatStatus::Free,
                                    vision_percentage: 90,
                                    number: 5,
                                },
                            ],
                        },
                    },
                ],
            },
        ];
        SeatingStructure { categories }
    }

    pub fn find_free_seats(&self, typ: &str, seat_count: u32) -> Vec<(String, u32)> {
        // Verificar si seat_count es mayor que 5
        if seat_count > 5 {
            eprintln!("Error: Cannot exceed a seat count greater than 5.");
            return vec![];
        }

        let mut best_seat_combination: Vec<(String, u32)> = Vec::new();
        let mut best_vision_percentage = 0;
        let mut best_distance = usize::MAX;

        let categories = &self.categories;
        let mut found_seats = false;  // Bandera para verificar si se encontraron asientos

        for category in categories {
            for zone in &category.zones {
                let num_zone = match zone.name.as_str() {
                    "ZonaA" => 0,
                    "ZonaB" => 1,
                    "ZonaC" => 2,
                    _ => {
                        eprintln!("Invalid zone: {}", zone.name);
                        continue;
                    }
                };

                let seats = match typ {
                    "VIP" => &zone.Vip.seats,
                    "General" => &zone.General.seats,
                    _ => {
                        eprintln!("Invalid category: {}", typ);
                        return vec![];
                    }
                };

                let mut free_seats = 0;
                let mut vision_percentage = 0;
                let mut distance = 0;
                let mut seat_combination: Vec<(String, u32)> = Vec::new();

                for seat in seats {
                    if free_seats == seat_count {
                        // Compara y guarda la mejor combinación de asientos encontrados
                        if best_distance > distance {
                            best_distance = distance;
                            if best_vision_percentage < vision_percentage {
                                best_vision_percentage = vision_percentage;
                                best_seat_combination = seat_combination.clone();
                            } else if best_vision_percentage == vision_percentage {
                                if best_distance > distance {
                                    best_seat_combination = seat_combination.clone();
                                }
                            }
                        }
                        found_seats = true;  // Asientos encontrados en la zona
                    }

                    if seat.status == SeatStatus::Free {
                        free_seats += 1;
                        vision_percentage += seat.vision_percentage as u32;
                        if free_seats > 1 {
                            distance += 1;
                        }
                        seat_combination.push((zone.name.clone(), seat.number));
                    } else {
                        if free_seats > 1 {
                            distance += 1;
                        }
                    }
                }

                // Manejo de zonas adicionales si no se ha encontrado suficientes asientos
                while free_seats < seat_count {
                    let next_zone = (num_zone + 1) % category.zones.len();
                    let seats = match typ {
                        "VIP" => &category.zones[next_zone].Vip.seats,
                        "General" => &category.zones[next_zone].General.seats,
                        _ => {
                            eprintln!("Invalid category: {}", typ);
                            return vec![];
                        }
                    };

                    for seat in seats {
                        if free_seats == seat_count {
                            if best_distance > distance {
                                best_distance = distance;
                                if best_vision_percentage < vision_percentage {
                                    best_vision_percentage = vision_percentage;
                                    best_seat_combination = seat_combination.clone();
                                } else if best_vision_percentage == vision_percentage {
                                    if best_distance > distance {
                                        best_seat_combination = seat_combination.clone();
                                    }
                                }
                            }
                            found_seats = true;  // Asientos encontrados en la siguiente zona
                        }

                        if seat.status == SeatStatus::Free {
                            free_seats += 1;
                            vision_percentage += seat.vision_percentage as u32;
                            if free_seats > 1 {
                                distance += 1;
                            }
                            seat_combination.push((category.zones[next_zone].name.clone(), seat.number));
                        } else {
                            if free_seats > 1 {
                                distance += 1;
                            }
                        }
                    }
                }
            }
        }

        // Si no se encontraron asientos, retorna un mensaje de error
        if !found_seats {
            eprintln!("No suitable seats found.");
            return vec![];
        }

        // Agregar la categoría al final de la mejor combinación
        best_seat_combination.push((typ.to_string(), seat_count));
        best_seat_combination
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
                //println!("Received request: {}", request);
                //println!("{:?}", request);
                let mut parts = request.split_whitespace();
                let command = parts.next().unwrap_or("");
                let typ = parts.next().unwrap_or("");
                let seat_count = parts.next().unwrap_or("").parse::<u32>().unwrap_or(0);

                let response = match command {
                    "find" => {
                        let seating_structure = seating_structure.lock().unwrap();
                        let seat_combination = seating_structure.find_free_seats(typ, seat_count);
                        if seat_combination.is_empty() {
                            "No suitable seats found.".to_string()
                        } else {
                            let mut response = seat_combination
                                .iter()
                                .map(|(zone, number)| format!("({},{})", zone, number))
                                .collect::<Vec<String>>()
                                .join(" ");
                            response
                        }
                    }
                    "reserve" => {
                        // reserve zona numero
                        //buscar la zona y el numero de asiento y cambiar el estado a reservado
                        // estructura de dato que llega del cliente: reserve VIP ZonaA 2
                        let mut parts = request.split_whitespace();
                        let _ = parts.next();
                        let typ = parts.next().unwrap_or("");
                        let zonesend = parts.next().unwrap_or("");
                        let seat_number = parts.next().unwrap_or("").parse::<u32>().unwrap_or(0);
                        let mut seating_structure = seating_structure.lock().unwrap();
                        let categories = &mut seating_structure.categories;
                        for category in categories {
                            for zone in &mut category.zones {
                                let seats = match typ {
                                    "VIP" => &mut zone.Vip.seats,
                                    "General" => &mut zone.General.seats,
                                    _ => {
                                        eprintln!("Invalid category: {}", typ);
                                        return;
                                    }
                                };
                                for seat in seats {
                                    if zone.name == zonesend && seat.number == seat_number {
                                        seat.status = SeatStatus::Reserved;
                                    }
                                }
                            }
                        }
                        //Mandar de regreso la misma información que llego
                        format!("purchase {} {} {}", typ, zonesend, seat_number)

                    }
                    "purchase" => {
                        // purchase zona numero
                        //buscar la zona y el numero de asiento y cambiar el estado a comprado
                        // estructura de dato que llega del cliente: purchase VIP ZonaA 2
                        let mut parts = request.split_whitespace();
                        let _ = parts.next();
                        let typ = parts.next().unwrap_or("");
                        let zonesend = parts.next().unwrap_or("");
                        let seat_number = parts.next().unwrap_or("").parse::<u32>().unwrap_or(0);
                        let mut seating_structure = seating_structure.lock().unwrap();
                        let categories = &mut seating_structure.categories;
                        for category in categories {
                            for zone in &mut category.zones {
                                let seats = match typ {
                                    "VIP" => &mut zone.Vip.seats,
                                    "General" => &mut zone.General.seats,
                                    _ => {
                                        eprintln!("Invalid category: {}", typ);
                                        return;
                                    }
                                };
                                for seat in seats {
                                    if zone.name == zonesend && seat.number == seat_number {
                                        seat.status = SeatStatus::Purchased;
                                    }
                                }
                            }
                        }
                        "Purchase completed".to_string()
                    }
                    _ => "Invalid command.".to_string(),
                };

                if let Err(e) = stream.write(response.as_bytes()) {
                    eprintln!("Failed to write to stream: {}", e);
                    break;
                }
            }
            _ => {}
        }
    }
}


