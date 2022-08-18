use clap::Arg;
use clap::ArgMatches;
use std::fs;
pub fn cli() -> clap::Command<'static> {
    clap::Command::new("update")
        .about("Update a new SpacetimeDB actor.")
        .override_usage("stdb update <identity> <name> <path to project>")
        .arg(Arg::new("identity").required(true))
        .arg(Arg::new("name").required(true))
        .arg(Arg::new("path to project").required(true))
        .after_help("Run `stdb help init for more detailed information.\n`")
}

pub async fn exec(host: &str, args: &ArgMatches) -> Result<(), anyhow::Error> {
    let hex_identity = args.value_of("identity").unwrap();
    let name = args.value_of("name").unwrap();
    let path_to_project = args.value_of("path to project").unwrap();

    let path = fs::canonicalize(path_to_project).unwrap();
    let wasm_bytes = fs::read(path)?;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/database/{}/{}/update", host, hex_identity, name))
        .body(wasm_bytes)
        .send()
        .await?;

    res.error_for_status()?;

    Ok(())
}
