use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

pub fn run_client() {
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            // Datos quemados para simular múltiples solicitudes
            let requests = vec![
                "find VIP 3",    // Solicita 3 asientos en la categoría VIP
                "find General 2", // Solicita 2 asientos en la categoría General
                "reserve VIP ZonaA 1 3", // Reserva asientos específicos en VIP
                "purchase VIP ZonaA 1 3", // Compra asientos específicos en VIP
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
                                println!("Seats found. Automatically reserving...");
                                let reserve_request = "reserve VIP Zona A 1 1 2 3"; // Ejemplo de reserva quemada
                                if let Err(e) = stream.write(reserve_request.as_bytes()) {
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
