mod server;
mod client;

fn main() {
    // Ejecuta el servidor en un hilo separado
    std::thread::spawn(|| {
        server::start_server();
    });

    // Ejecuta el cliente
    client::run_client();
}
