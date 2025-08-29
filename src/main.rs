mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config("repoxy.toml")?;
    println!("{:#?}", config);

    println!("Proxy listening on {}: {}", config.server.host, config.server.port);
    for route in config.routes {
        println!("Route: {}", route.path);
        println!("  Backends: {:?}", route.backends);
        if let Some(ref method) = route.method {
            println!("  Methods: {:?}", method);
        }
        if let Some(ref timeout) = route.timeout {
            println!("  Timeout: {}ms", timeout);
        }
        if let Some(ref retries) = route.retries {
            println!("  Retries: {}", retries);
        }
        if let Some(ref add_headers) = route.add_headers {
            println!("  Add Headers: {:?}", add_headers);
        }
    }

    Ok(())
}
