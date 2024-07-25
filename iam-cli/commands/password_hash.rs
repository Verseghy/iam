use clap::{Arg, ArgMatches, Command};

pub fn command() -> Command {
    Command::new("password-hash")
        .about("Creates a hash from a password")
        .arg(
            Arg::new("password")
                .long("password")
                .short('p')
                .required(true)
                .help("The password to be hashed"),
        )
}

pub fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    let password = matches.get_one::<String>("password").unwrap();

    let hashed = iam_common::password::hash(password).unwrap();

    println!("{hashed}");

    Ok(())
}
