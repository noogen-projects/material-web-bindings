use env_logger::Env;
use tiny_file_server::FileServer;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    FileServer::http("127.0.0.1:9080")
        .expect("Server should be created")
        .run("target/web")
        .expect("Server should start");
}
