mod server;
mod client;

fn main() {
    // Ejecuta el servidor en un hilo separado
    std::thread::spawn(|| {
        server::start_server();
    });

    // Ejecuta el cliente en el hilo principal
    client::run_client();

    //mostrar el SeatingStructure


}
