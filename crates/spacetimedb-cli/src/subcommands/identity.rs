use crate::{
    config::{Config, IdentityConfig},
    util::{init_default, IdentityTokenJson, InitDefaultResultType},
};
use clap::{arg, Arg, ArgAction, ArgMatches, Command};
use reqwest::StatusCode;
use serde::Deserialize;
use tabled::{object::Columns, Alignment, Modify, Style, Table, Tabled};

pub fn cli() -> Command {
    Command::new("identity")
        .args_conflicts_with_subcommands(true)
        .subcommand_required(true)
        .subcommands(get_subcommands())
        .about("Manage identities stored by the command line tool")
}

fn get_subcommands() -> Vec<Command> {
    vec![
        Command::new("ls").about("List saved identities"),
        Command::new("set-default")
            .about("Set the default identity")
            .arg(Arg::new("identity").conflicts_with("name").required(true))
            .arg(arg!(-n --name <NAME> "name").conflicts_with("identity").required(true)),
        Command::new("set-email")
            .about("Associates an identity with an email address")
            .arg(Arg::new("identity").required(true))
            .arg(Arg::new("email").required(true)),
        Command::new("init-default")
            .about("Initialize a new default identity if missing")
            .arg(
                arg!(-n --name [NAME] "Nickname for this identity")
                    .required(false)
                    .default_missing_value(""),
            )
            .arg(
                Arg::new("quiet")
                    .long("quiet")
                    .short('q')
                    .action(ArgAction::SetTrue)
                    .help("Runs command in silent mode."),
            ),
        Command::new("new")
            .about("Create a new identity")
            .arg(
                Arg::new("no-save")
                    .help("Don't save save to local config, just create a new identity")
                    .long("no-save")
                    .action(ArgAction::SetTrue)
                    .required(false),
            )
            .arg(
                arg!(-n --name [NAME] "Nickname for this identity")
                    .required(false)
                    .conflicts_with("no-save"),
            )
            .arg(
                arg!(-e --email [EMAIL] "Recovery email for this identity")
                    .required(false)
                    .conflicts_with("no-save"),
            ),
        Command::new("delete")
            .about("Delete a saved identity")
            .arg(Arg::new("identity").conflicts_with("name").required(true))
            .arg(arg!(-n --name <NAME> "name").conflicts_with("identity").required(true)),
        Command::new("add")
            .about("Add an existing identity")
            .arg(Arg::new("identity").required(true))
            .arg(Arg::new("token").required(true))
            .arg(
                arg!(-n --name [NAME] "Nickname for identity")
                    .required(false)
                    .default_missing_value(""),
            )
            .arg(
                arg!(-e --email [EMAIL] "Nickname for identity")
                    .required(false)
                    .default_missing_value(""),
            ),
        Command::new("find")
            .about("Find an identity for an email")
            .arg(Arg::new("email").required(true)),
    ]
}

pub async fn exec(config: Config, args: &ArgMatches) -> Result<(), anyhow::Error> {
    let (cmd, subcommand_args) = args.subcommand().expect("Subcommand required");
    exec_subcommand(config, cmd, subcommand_args).await
}

async fn exec_subcommand(config: Config, cmd: &str, args: &ArgMatches) -> Result<(), anyhow::Error> {
    match cmd {
        "ls" => exec_ls(config, args).await,
        "set-default" => exec_set_default(config, args).await,
        "init-default" => exec_init_default(config, args).await,
        "new" => exec_new(config, args).await,
        "delete" => exec_delete(config, args).await,
        "add" => exec_add(config, args).await,
        "set-email" => exec_email(config, args).await,
        "find" => exec_find(config, args).await,
        unknown => Err(anyhow::anyhow!("Invalid subcommand: {}", unknown)),
    }
}

async fn exec_set_default(mut config: Config, args: &ArgMatches) -> Result<(), anyhow::Error> {
    let name = args.get_one::<String>("name");
    if let Some(name) = name {
        if let Some(identity_config) = config.get_identity_config_by_name(name) {
            config.default_identity = Some(identity_config.identity.clone());
            config.save();
            return Ok(());
        } else {
            return Err(anyhow::anyhow!("No such identity by that name."));
        }
    }

    if let Some(identity) = args.get_one::<String>("identity") {
        if let Some(identity_config) = config.get_identity_config_by_identity(identity) {
            config.default_identity = Some(identity_config.identity.clone());
            config.save();
            return Ok(());
        } else {
            return Err(anyhow::anyhow!("No such identity."));
        }
    }

    Err(anyhow::anyhow!("Either a name or an identity must be provided."))
}

// TODO(cloutiertyler): Realistically this should just be run before every
// single command, but I'm separating it out into its own command for now for
// simplicity.
async fn exec_init_default(mut config: Config, args: &ArgMatches) -> Result<(), anyhow::Error> {
    let nickname = args.get_one::<String>("name").map(|s| s.to_owned());
    let quiet = args.get_flag("quiet");

    let init_default_result = init_default(&mut config, nickname).await?;
    let identity_config = init_default_result.identity_config;
    let result_type = init_default_result.result_type;

    if !quiet {
        match result_type {
            InitDefaultResultType::Existing => {
                println!(" Existing default identity");
                println!(" IDENTITY  {}", identity_config.identity);
                println!(" NAME      {}", identity_config.nickname.unwrap_or_default());
                return Ok(());
            }
            InitDefaultResultType::SavedNew => {
                println!(" Saved new identity");
                println!(" IDENTITY  {}", identity_config.identity);
                println!(" NAME      {}", identity_config.nickname.unwrap_or_default());
            }
        }
    }

    Ok(())
}

async fn exec_delete(mut config: Config, args: &ArgMatches) -> Result<(), anyhow::Error> {
    let name = args.get_one::<String>("name");
    if let Some(name) = name {
        let index = config
            .identity_configs
            .iter()
            .position(|c| c.nickname.as_ref() == Some(name));
        if let Some(index) = index {
            let ic = config.identity_configs.remove(index);
            config.update_default_identity();
            config.save();
            println!(" Removed identity");
            println!(" IDENTITY  {}", ic.identity);
            println!(" NAME  {}", ic.nickname.unwrap_or_default());
        } else {
            println!("No such identity by that name.");
        }
        std::process::exit(0);
    }

    if let Some(identity) = args.get_one::<String>("identity") {
        let index = config.identity_configs.iter().position(|c| &c.identity == identity);
        if let Some(index) = index {
            let ic = config.identity_configs.remove(index);
            config.update_default_identity();
            config.save();
            println!(" Removed identity");
            println!(" IDENTITY  {}", ic.identity);
            println!(" NAME  {}", ic.nickname.unwrap_or_default());
        } else {
            println!("No such identity.");
        }
    }

    Ok(())
}

async fn exec_new(mut config: Config, args: &ArgMatches) -> Result<(), anyhow::Error> {
    let save = !*args.get_one::<bool>("no-save").unwrap_or(&false);
    if save {
        let nickname = args.get_one::<String>("name").unwrap_or(&"".to_string()).clone();
        if config.name_exists(&nickname) {
            println!("An identity with that name already exists.");
            std::process::exit(0);
        }
    }

    let client = reqwest::Client::new();
    let mut builder = client.post(format!("http://{}/identity", config.host));

    if let Some(identity_token) = config.get_default_identity_config() {
        builder = builder.basic_auth("token", Some(identity_token.token.clone()));
    }

    let res = builder.send().await?;
    let res = res.error_for_status()?;

    let body = res.bytes().await?;
    let body = String::from_utf8(body.to_vec())?;

    let identity_token: IdentityTokenJson = serde_json::from_str(&body)?;

    let identity = identity_token.identity.clone();

    if save {
        let nickname = args.get_one::<String>("name").cloned();
        let email = args.get_one::<String>("email").cloned();

        config.identity_configs.push(IdentityConfig {
            identity: identity_token.identity,
            token: identity_token.token,
            nickname: nickname.clone(),
            email: email.clone(),
        });
        if config.default_identity.is_none() {
            config.default_identity = Some(identity.clone());
        }
        config.save();
        println!(" Saved new identity");
        println!(" IDENTITY  {}", identity);
        println!(" NAME      {}", nickname.unwrap_or_default());
        println!(" EMAIL     {}", email.unwrap_or_default());
    } else {
        println!(" IDENTITY  {}", identity);
        println!(" TOKEN     {}", identity_token.token);
    }

    Ok(())
}

async fn exec_add(mut config: Config, args: &ArgMatches) -> Result<(), anyhow::Error> {
    let identity: String = args.get_one::<String>("identity").unwrap().clone();
    let token: String = args.get_one::<String>("token").unwrap().clone();

    //optional
    let nickname = args.get_one::<String>("name").cloned();
    let email = args.get_one::<String>("email").cloned();

    config.identity_configs.push(IdentityConfig {
        identity,
        token,
        nickname: nickname.clone(),
        email,
    });

    config.save();

    println!(" New Identity Added");
    println!(" NAME      {}", nickname.unwrap_or_default());

    Ok(())
}

#[derive(Tabled)]
#[tabled(rename_all = "UPPERCASE")]
struct LsRow {
    default: String,
    identity: String,
    name: String,
    email: String,
}

async fn exec_ls(config: Config, _args: &ArgMatches) -> Result<(), anyhow::Error> {
    let mut rows: Vec<LsRow> = Vec::new();
    for identity_token in config.identity_configs {
        let default_str = if config.default_identity.is_some()
            && config.default_identity.as_ref().unwrap() == &identity_token.identity
        {
            "***"
        } else {
            ""
        };
        rows.push(LsRow {
            default: default_str.to_string(),
            identity: identity_token.identity,
            name: identity_token.nickname.unwrap_or_default(),
            email: identity_token.email.unwrap_or_default(),
        });
    }
    let table = Table::new(&rows)
        .with(Style::empty())
        .with(Modify::new(Columns::first()).with(Alignment::right()));
    println!("{}", table);
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
struct GetIdentityResponse {
    identities: Vec<GetIdentityResponseEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct GetIdentityResponseEntry {
    identity: String,
    email: String,
}

async fn exec_find(config: Config, args: &ArgMatches) -> Result<(), anyhow::Error> {
    let email = args.get_one::<String>("email").unwrap().clone();

    let client = reqwest::Client::new();
    let builder = client.get(format!("http://{}/identity?email={}", config.host, email));

    let res = builder.send().await?;

    if res.status() == StatusCode::OK {
        let response: GetIdentityResponse = serde_json::from_slice(&res.bytes().await?[..])?;
        if response.identities.len() == 0 {
            return Err(anyhow::anyhow!("Could not find identity for: {}", email));
        }

        for identity in response.identities {
            println!("Identity");
            println!(" IDENTITY  {}", identity.identity);
            println!(" EMAIL     {}", identity.email);
        }
        Ok(())
    } else if res.status() == StatusCode::NOT_FOUND {
        Err(anyhow::anyhow!("Could not find identity for: {}", email))
    } else {
        Err(anyhow::anyhow!("Error occurred in lookup: {}", res.status()))
    }
}

async fn exec_email(mut config: Config, args: &ArgMatches) -> Result<(), anyhow::Error> {
    let email = args.get_one::<String>("email").unwrap().clone();
    let identity = args.get_one::<String>("identity").unwrap().clone();

    let client = reqwest::Client::new();
    let mut builder = client.post(format!(
        "http://{}/identity/{}/set-email?email={}",
        config.host, identity, email
    ));

    if let Some(identity_token) = config.get_identity_config_by_identity(&identity) {
        builder = builder.basic_auth("token", Some(identity_token.token.clone()));
    } else {
        println!("Missing identity credentials for identity.");
        std::process::exit(0);
    }

    let res = builder.send().await?;
    res.error_for_status()?;

    let ic = config.get_identity_config_by_identity_mut(&identity).unwrap();
    ic.email = Some(email.clone());
    config.save();

    println!(" Associated email with identity");
    println!(" IDENTITY  {}", identity);
    println!(" EMAIL     {}", email);

    Ok(())
}
