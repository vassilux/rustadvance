use rustls::server::ServerConfig;
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::{certs, rsa_private_keys};
use std::fs::File;
use std::io::{self, BufReader};

fn load_cert_file(path: &str) -> io::Result<Vec<Certificate>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let certs = certs(&mut reader)?.into_iter().map(Certificate).collect();
    Ok(certs)
}

fn load_private_keys(path: &str) -> io::Result<PrivateKey> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let keys = rsa_private_keys(&mut reader)?
        .into_iter()
        .map(PrivateKey)
        .collect::<Vec<_>>();
    if keys.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No valid private keys found.",
        ));
    }
    Ok(keys[0].clone())
}

pub fn load_rustls_config() -> Result<ServerConfig, Box<dyn std::error::Error + Send + Sync>> {
    let cert_chain = load_cert_file("certs/cert.pem")?;
    let private_key = load_private_keys("certs/key.pem")?;

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)?;

    Ok(config)
}
