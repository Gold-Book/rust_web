use actix_web::{HttpServer, App, middleware};
use actix_web::middleware::Logger;
use listenfd::ListenFd;
use example::apis::{endpoint_config, example_config};

fn main() {
    env_logger::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(
        || App::new()
            .wrap(Logger::default())
            .wrap(middleware::NormalizePath)
            .wrap(example::middlewares::requests::authentication_infrastructure::AuthenticationInfrastructure)
            .configure(endpoint_config)
            .configure(example_config)
    );

    server = if let Some(tcp_listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(tcp_listener).unwrap()
    } else {
        server.bind("0.0.0.0:3000").unwrap()
    };

    server.run().unwrap();
}
