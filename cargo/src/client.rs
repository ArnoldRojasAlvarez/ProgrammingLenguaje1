use rand::Rng;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

fn should_reserve() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.5)  // Genera un booleano con 50% de probabilidad
}

fn handle_request(stream: &mut TcpStream, request: &str) -> io::Result<String> {
    println!("Sending request: {}", request);
    stream.write_all(request.as_bytes())?;

    let mut buffer = [0; 512];
    let size = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..size]).to_string();
    println!("Server response: {}", response);

    // Simula la aceptación automática de las reservas encontradas
    if request.starts_with("find") {
        if response.contains("No suitable seats found.") {
            println!("No seats found. Skipping reservation.");
        } else {
            if should_reserve() {
                println!("Seats found. Automatically reserving...");

                // Simula un retraso antes de realizar la reserva
                sleep(Duration::from_secs(2));

                // Obtener el tipo de categoría de la respuesta
                let category_parent = response.split_whitespace().last().unwrap_or("");
                let category = category_parent
                    .split(',')
                    .next()
                    .unwrap_or("")
                    .trim_matches(|c| c == '(' || c == ')');

                // Dividir la cadena en partes y eliminar el último elemento
                let parts: Vec<&str> = response
                    .split_whitespace()
                    .collect();

                // Crear solicitudes de reserva excluyendo el último elemento
                let reserve_requests: Vec<String> = parts
                    .iter()
                    .filter_map(|&seat| {
                        let trimmed_seat = seat.trim_matches(|c| c == '(' || c == ')');
                        if let Some((zona, seat_number)) = trimmed_seat.split_once(',') {
                            Some(format!("reserve {} {} {}", category, zona.trim(), seat_number.trim()))
                        } else {
                            None
                        }
                    })
                    .collect();

                println!("Reserve requests: {:?}", reserve_requests);
                let mut response_server: Vec<String> = Vec::new();
                for reserve in reserve_requests {
                    println!("Sending reserve request: {}", reserve);
                    stream.write_all(reserve.as_bytes())?;

                    // Lee la respuesta de la reserva
                    let mut reserve_buffer = [0; 512];
                    let reserve_size = stream.read(&mut reserve_buffer)?;
                    let reserve_response = String::from_utf8_lossy(&reserve_buffer[..reserve_size]).to_string();
                    response_server.push(reserve_response.clone()); // Clona la respuesta antes de moverla
                    println!("Server response to reservation: {}", reserve_response);
                }

                // Mandar al servidor `response_server`
                for response in response_server {
                    println!("Lista Reservada a comprar: {:?}", response);
                    stream.write_all(response.as_bytes())?;

                    let mut reserve_buffer = [0; 512];
                    let reserve_size = stream.read(&mut reserve_buffer)?;
                    let reserve_response = String::from_utf8_lossy(&reserve_buffer[..reserve_size]).to_string();
                    println!("Server response to reservation: {}", reserve_response);
                }
            } else {
                println!("Skipping reservation.");
            }
        }
    }

    Ok(response)
}

pub fn run_client() {
    loop {
        match TcpStream::connect("127.0.0.1:7878") {
            Ok(mut stream) => {
                // Datos quemados para simular múltiples solicitudes
                let requests = vec![
                    "find VIP 4",    // Solicita 4 asientos en la categoría VIP
                    "find General 2", // Solicita 2 asientos en la categoría General
                    "find VIP 1",    // Solicita 1 asiento en la categoría VIP
                    "find VIP 5",    // Solicita 5 asientos en la categoría VIP
                    "find General 3", // Solicita 3 asientos en la categoría General
                    "find General 2", // Solicita 2 asientos en la categoría General
                ];

                for request in requests {
                    if let Err(e) = handle_request(&mut stream, request) {
                        eprintln!("Error handling request: {}", e);
                    }
                    // Agrega un retraso para simular la concurrencia
                    sleep(Duration::from_secs(2));
                }

                println!("Client finished requests. Exiting...");
                break;
            }
            Err(e) => {
                eprintln!("Failed to connect to server: {}", e);
                println!("Retrying in 5 seconds...");
                sleep(Duration::from_secs(5));
            }
        }
    }
}
