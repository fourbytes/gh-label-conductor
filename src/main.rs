#![warn(clippy::pedantic)]
#[macro_use]
extern crate tracing;

use std::{collections::HashMap, env, fs::File, path::PathBuf, str::FromStr};

use itertools::Itertools;
use miette::{IntoDiagnostic, DiagnosticResult, Diagnostic};
use clap::Clap;
use serde::{Serialize, Deserialize};
use octorust::{Client, auth::Credentials, issues::Issues, types::{IssuesCreateLabelRequest, IssuesUpdateLabelRequest}};
use tracing_subscriber::util::SubscriberInitExt;

fn setup_logging() -> DiagnosticResult<()> {
    Ok(tracing_subscriber::fmt()
        .without_time()
        .pretty()
        .finish()
        .try_init()
        .into_diagnostic("tracing_subscriber::init")?)
}

#[derive(Debug)]
struct RepositoryPath {
    owner: String,
    name: String
}

#[derive(Debug, thiserror::Error)] 
enum RepositoryPathParseError {
    #[error("invalid repository path")]
    Invalid
}

impl FromStr for RepositoryPath {
    type Err = RepositoryPathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (owner, name) = s.split('/').map(String::from).collect_tuple().ok_or(RepositoryPathParseError::Invalid)?;
        Ok(Self { owner, name })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    prefix: String,
    color: String,
    labels: HashMap<String, String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    categories: Vec<Category>
}

#[derive(Clap)]
struct Apply {
    repository: RepositoryPath
}

#[derive(Clap)]
enum SubCommand {
    Apply(Apply)
}

#[derive(Clap)]
struct Command {
    #[clap(long, default_value = "config.yaml")]
    config_path: PathBuf,

    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Debug, thiserror::Error, Diagnostic)]
enum ClientError {
    #[error("Encountered GitHub client error: {0:?}")]
    #[diagnostic(code(octorust::client))]
    Other(#[from] anyhow::Error)
}

fn setup_client() -> Result<Client, ClientError> {
    trace!("setting up GitHub client");
    Ok(Client::new("gh-label-conductor", Credentials::Token(env::var("GH_TOKEN").expect("Missing GH_TOKEN env var.")))?)
}

#[instrument(skip(client, repository))]
async fn apply_label(client: Client, repository: &RepositoryPath, name: String, description: String, color: String) -> DiagnosticResult<()> {
    let issues = Issues { client };
    match issues.create_label(&repository.owner, &repository.name, &IssuesCreateLabelRequest {
        name: name.clone(),
        color: color.clone(),
        description: description.clone()
    }).await {
        Ok(_) => (),
        Err(error) => {
            debug!(?error, "failed to create label, updating instead");
            issues.update_label(&repository.owner, &repository.name, &name, &IssuesUpdateLabelRequest {
                new_name: name.clone(),
                color: color.clone(),
                description: description.clone(),
            }).await.map_err(ClientError::from)?;
        }
    }
    info!("applied label");
    Ok(())
}

#[instrument(skip(client, repository, labels))]
async fn apply_category(client: Client, repository: &RepositoryPath, Category { prefix, color, labels }: Category) -> DiagnosticResult<()> {
    for (name, description) in labels {
        let name = format!("{}-{}", prefix, name);
        apply_label(client.clone(), repository, name.clone(), description.clone(), color.clone()).await?;
    }
    info!("applied category");
    Ok(())
}

#[tokio::main]
async fn main() -> DiagnosticResult<()> {
    setup_logging()?;

    let cmd = Command::parse();
    let config: Config = serde_yaml::from_reader(File::open(cmd.config_path).into_diagnostic("config::open")?).into_diagnostic("config::parse")?;
    debug!(?config, "loaded config");

    match cmd.subcmd {
        SubCommand::Apply(Apply { repository }) => {
            let span = info_span!("apply", ?repository);
            let _handle = span.enter();
            let client = setup_client()?;
            for category in config.categories {
                apply_category(client.clone(), &repository, category).await?;
            }
        }
    }
    Ok(())
}

