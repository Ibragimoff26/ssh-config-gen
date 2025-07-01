use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

use clap::{Arg, ArgAction, ArgMatches, Command};
use handlebars::Handlebars;
use serde_json::json;

fn create_command() -> Command {
    Command::new("ssh-config-add")
        .about("Generate SSH configuration")
        .arg(Arg::new("port").short('p').help("SSH port"))
        .arg(
            Arg::new("identity-file")
                .short('I')
                .help("Path to identity file"),
        )
        .arg(
            Arg::new("compression")
                .short('C')
                .help("Enable compression")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("destination")
                .short('D')
                .help("Destination")
                .value_name("[user@]hostname"),
        )
        .arg(
            Arg::new("host")
                .help("One or more host aliases")
                .required(true)
                .num_args(1..),
        )
}

fn parse_destination(destination: &str) -> Result<(&str, &str), Box<IoError>> {
    match destination.split_once('@') {
        Some((u, h)) => Ok((u, h)),
        None => {
            return Err(Box::new(IoError::new(
                ErrorKind::InvalidInput,
                format!("Expected user@hostname, got: {destination}"),
            )));
        }
    }
}

fn build_config(matches: ArgMatches) -> Result<serde_json::Value, Box<IoError>> {
    let (user, hostname) = if let Some(dest) = matches.get_one::<String>("destination") {
        let (u, h) = parse_destination(&dest)?;
        (Some(u), Some(h))
    } else {
        (None, None)
    };

    let hosts: Vec<&str> = matches
        .get_many::<String>("host")
        .expect("Host is required")
        .map(String::as_str)
        .collect();

    Ok(json!({
        "host": hosts,
        "user": user,
        "hostname": hostname,
        "port": matches.get_one::<String>("port"),
        "identity_file": matches.get_one::<String>("identity-file"),
        "compression": matches.get_flag("compression"),
    }))
}

fn render_template(config: serde_json::Value) -> Result<String, Box<dyn Error>> {
    let template = include_str!("ssh_config.txt");

    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("config", template)?;

    Ok(handlebars.render("config", &config)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = create_command().get_matches();

    let config = build_config(matches)?;
    let rendered = render_template(config)?;

    println!("{}", rendered);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_destination() {
        let (u, h) = parse_destination("test@192.168.0.1").unwrap();
        assert_eq!(u, "test");
        assert_eq!(h, "192.168.0.1");
    }

    #[test]
    fn test_parse_destination_error() {
        let err: Box<IoError> = parse_destination("test192.168.0.1").unwrap_err();
        assert!(err.to_string().contains("Expected user@hostname"));
    }

    #[test]
    fn test_render() {
        let config = json!({
            "host": Vec::from(["test"]),
            "user": "user",
            "hostname": "hostname",
            "port": "22",
            "compression": true,
            "identity_file": "~/.ssh/id_rsa"
        });

        assert_eq!(
            render_template(config).unwrap(),
            "Host test
    HostName hostname
    User user
    Port 22
    IdentityFile ~/.ssh/id_rsa
    Compression yes"
        );
    }
}
