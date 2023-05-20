use std::error::Error;
use std::fs;
use std::path::PathBuf;

use clap::Parser;

use jet::junos_20_4::jnx::jet::{
    authentication::{authentication_client::AuthenticationClient, LoginRequest},
    common::StatusCode,
    management::{
        management_client::ManagementClient, op_command_get_request::Command, OpCommandGetRequest,
        OpCommandOutputFormat,
    },
};

use rpassword::prompt_password;

use simple_logger::SimpleLogger;

use tonic::transport::{Certificate, Channel, ClientTlsConfig, Identity};

/// JET demo application.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Junos operational mode command to execute
    command: String,
    /// JET server hostname.
    #[arg(short = 'H', long)]
    host: String,
    /// JET service TCP port.
    #[arg(short = 'P', long, default_value_t = 32767)]
    port: u16,
    /// Authentication username.
    #[arg(short = 'u', long, default_value = "support")]
    username: String,
    /// Client certificate path.
    #[arg(long, requires("client_key_path"))]
    client_cert_path: Option<PathBuf>,
    /// Client key path.
    #[arg(long, requires("client_cert_path"))]
    client_key_path: Option<PathBuf>,
    /// CA certificate path.
    #[arg(long, default_value = "./examples/pki/ca.crt")]
    ca_cert_path: PathBuf,
    /// Application client ID.
    #[arg(long, default_value = "jet-demo-rs")]
    client_id: String,
    /// Override the domain name against which the server's TLS certificate is verified.
    #[arg(long)]
    server_tls_domain_name: Option<String>,

    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    SimpleLogger::new()
        .with_level(args.verbosity.log_level_filter())
        .init()?;
    log::debug!("logger initialized");

    log::debug!("setting up TLS config");
    let tls_config = {
        let mut config = ClientTlsConfig::new().ca_certificate(Certificate::from_pem(
            fs::read_to_string(args.ca_cert_path)?,
        ));
        if let Some(domain_name) = args.server_tls_domain_name {
            log::info!("overriding server TLS domain name: {domain_name}");
            config = config.domain_name(domain_name);
        }
        match (args.client_cert_path, args.client_key_path) {
            (Some(cert_path), Some(key_path)) => {
                log::info!("enabling mutual TLS authentication");
                config.identity(Identity::from_pem(
                    fs::read_to_string(cert_path)?,
                    fs::read_to_string(key_path)?,
                ))
            }
            (None, None) => config,
            _ => unreachable!("cli argument parsing rules should prevent this being reached"),
        }
    };

    log::info!("connecting gRPC channel");
    let mut channel = Channel::from_shared(format!("https://{}:{}", args.host, args.port))?
        .tls_config(tls_config)?
        .connect()
        .await?;

    log::info!("attempting to authenticate to JET server");
    let login_resp = AuthenticationClient::new(&mut channel)
        .login(LoginRequest {
            client_id: args.client_id,
            username: args.username,
            password: prompt_password("Password: ")?,
        })
        .await?
        .into_inner();
    if let Some(status) = login_resp.status {
        match status.code() {
            StatusCode::Success => log::info!("authentication successful"),
            StatusCode::Failure => {
                return Err(format!("authentication failed: {}", status.message).into());
            }
        };
    } else {
        return Err(format!("no status in login response message: {:?}", login_resp).into());
    };

    let mut op_command_resp_stream = ManagementClient::new(&mut channel)
        .op_command_get(OpCommandGetRequest {
            out_format: OpCommandOutputFormat::OpCommandOutputCli.into(),
            command: Some(Command::CliCommand(args.command)),
        })
        .await?
        .into_inner();
    while let Some(op_command_resp) = op_command_resp_stream.message().await? {
        if let Some(status) = op_command_resp.status {
            match status.code() {
                StatusCode::Success => println!("{}", op_command_resp.data),
                StatusCode::Failure => {
                    return Err(format!("op command failed: {}", status.message).into());
                }
            }
        } else {
            return Err(format!(
                "no status in op_command response message: {:?}",
                op_command_resp
            )
            .into());
        }
    }

    Ok(())
}
