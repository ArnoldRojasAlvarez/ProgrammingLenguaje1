use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

pub fn run_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            // Datos quemados para simular múltiples solicitudes
            let requests = vec![
                "find VIP 6",    // Solicita 3 asientos en la categoría VIP
                //"find General 2", // Solicita 2 asientos en la categoría General
                //"find VIP 2",    // Solicita 2 asientos en la categoría VIP
                //"find VIP 4",    // Solicita 4 asientos en la categoría VIP
            ];

            for request in requests {
                println!("Sending request: {}", request);
                if let Err(e) = stream.write(request.as_bytes()) {
                    eprintln!("Failed to write to server: {}", e);
                    break;
                }

                let mut buffer = [0; 512];
                match stream.read(&mut buffer) {
                    Ok(size) => {
                        let response = String::from_utf8_lossy(&buffer[..size]);
                        println!("Server response: {}", response);
                        // Simula la aceptación automática de las reservas encontradas
                        if request.starts_with("find") {
                            if response.contains("No free seats found") {
                                println!("No seats found. Skipping reservation.");
                            } else {
                                //consultar si desea o no reservar
                                println!("Do you want to reserve the seats? (y/n)");
                                let mut input = "y".to_string();
                                if input.trim() != "y" {
                                    println!("Skipping reservation.");
                                    continue;
                                } else {
                                    println!("Reserving seats...");
                                    println!("Seats found. Automatically reserving...");
                                    println!("Reserving seats: {}", response);
                                    //sleep
                                    std::thread::sleep(std::time::Duration::from_secs(10));
                                    // Parseamos la respuesta del servidor para reservar automáticamente los asientos recomendados
                                    let reserve_request: Vec<String> = response
                                        .lines()
                                        .map(|line| {
                                            let parts: Vec<&str> = line.split(", ").collect();
                                            let zone = parts.get(0).and_then(|s| s.split("Zone: ").nth(1)).unwrap_or("");
                                            let row = parts.get(1).and_then(|s| s.split("Row: ").nth(1)).unwrap_or("");
                                            let seat = parts.get(2).and_then(|s| s.split("Seat: ").nth(1)).unwrap_or("");
                                            format!("reserve {} {} {} {}", zone, row, seat, seat) // Ejemplo: "reserve VIP ZonaA 1 1"
                                        })
                                        .collect(); // Ahora es Vec<String>

                                    for reserve in reserve_request {
                                        if let Err(e) = stream.write(reserve.as_bytes()) {
                                            eprintln!("Failed to write to server: {}", e);
                                            break;
                                        }

                                        // Lee la respuesta de la reserva
                                        let mut reserve_buffer = [0; 512];
                                        match stream.read(&mut reserve_buffer) {
                                            Ok(reserve_size) => {
                                                let reserve_response = String::from_utf8_lossy(&reserve_buffer[..reserve_size]);
                                                println!("Server response to reservation: {}", reserve_response);
                                            }
                                            Err(e) => {
                                                eprintln!("Failed to read from server: {}", e);
                                                break;
                                            }
                                        }
                                    }
                                }


                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from server: {}", e);
                        break;
                    }
                }

                // Agrega un retraso para simular la concurrencia
                sleep(Duration::from_secs(2));
            }
        }
        Err(e) => eprintln!("Failed to connect to server: {}", e),
    }
}
