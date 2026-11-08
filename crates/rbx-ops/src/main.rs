use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

use rbx_core::GlobalFlags;

#[derive(Parser)]
#[command(
    name = "rbx-ops",
    version,
    about = "Operate a live Roblox experience",
    long_about = "Operate a live Roblox experience through Open Cloud: its servers, its players, \
                  its data, the messages and secrets its servers read, and its ad campaigns. \
                  Every write is something somebody typed, never a file reconciled. \
                  Per-command help: `rbx-ops <command> --help`."
)]
struct Cli {
    #[command(flatten)]
    global: GlobalFlags,

    #[command(subcommand)]
    command: Command,
}

// Order chosen to read top-to-bottom: what is running, who is playing, what
// they have stored, what reaches the servers, then the one command that spends
// money. Not alphabetical, and clap renders subcommands in declaration order,
// so this list *is* the help output.
#[derive(Subcommand)]
enum Command {
    /// Servers currently up, and how the stopped ones ended
    Servers(rbx_servers::ServersCli),
    /// Query the experience's own analytics metrics
    Analytics(rbx_analytics::AnalyticsCli),
    /// Inspect and change player restrictions
    Ban(rbx_ban::BanCli),
    /// List a group's roles and move members between them
    ///
    /// Beside `ban` because both act on people rather than on servers or
    /// data. Roles themselves are read-only: Open Cloud can list them and
    /// assign them, and nothing else.
    Group(rbx_group::GroupCli),
    /// Forecast and launch a rolling server restart
    Restart(rbx_restart::RestartCli),
    /// Read and overwrite a data store entry
    Data(rbx_data::DataCli),
    /// Read and write memory store sorted map items
    Memorystore(rbx_memorystore::MemoryStoreCli),
    /// Push a MessagingService message to every running server
    ///
    /// Named `message`, not `publish`: "publish" means a deploy to most people
    /// who reach for a Roblox command line, and somebody looking for how to
    /// publish a place would find this first and get far enough in to be
    /// confused.
    Message(rbx_message::MessageCli),
    /// Secrets HttpService:GetSecret reads, written encrypted
    ///
    /// Write-only by design: Roblox never sends a stored value back, so there
    /// is nothing here that can print one.
    Secret(rbx_secret::SecretCli),
    /// Launch and steer ad campaigns
    ///
    /// Last because it is the odd one: it spends real money and reads no
    /// results back. `/ads-management/v1` has no reporting endpoint, so
    /// campaign performance is read in Ads Manager.
    Ads(rbx_ads::AdsCli),
    /// Turn a username into a user id, or an id into a username
    ///
    /// `--id` and `--name` print the bare value, one per line, so it composes:
    /// `rbx-ops data get --datastore PlayerData "User_$(rbx-ops user builderman --id)"`.
    ///
    /// Hidden from the command list because it is reached for inside another
    /// command line, not looked up in `--help`.
    #[command(hide = true)]
    User(rbx_user::UserCli),
}

#[tokio::main]
async fn main() -> ExitCode {
    match dispatch().await {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        }
    }
}

async fn dispatch() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Servers(c) => rbx_servers::run(c, &cli.global).await,
        Command::Analytics(c) => rbx_analytics::run(c, &cli.global).await,
        Command::Ban(c) => rbx_ban::run(c, &cli.global).await,
        Command::Group(c) => rbx_group::run(c, &cli.global).await,
        Command::Restart(c) => rbx_restart::run(c, &cli.global).await,
        Command::Data(c) => rbx_data::run(c, &cli.global).await,
        Command::Memorystore(c) => rbx_memorystore::run(c, &cli.global).await,
        Command::Message(c) => rbx_message::run(c, &cli.global).await,
        Command::Secret(c) => rbx_secret::run(c, &cli.global).await,
        Command::Ads(c) => rbx_ads::run(c, &cli.global).await,
        Command::User(c) => rbx_user::run(c, &cli.global).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    /// clap checks a command tree only when it is built, and panics on a
    /// clash at that moment: on a user's machine, at the first run. Eleven
    /// crates contribute arguments under one set of global flags, which is
    /// exactly where two of them end up answering to the same name.
    #[test]
    fn the_command_tree_is_well_formed() {
        Cli::command().debug_assert();
    }

    /// `user` runs and completes, but stays out of the list.
    #[test]
    fn user_is_hidden_and_still_parses() {
        let help = Cli::command().render_help().to_string();
        assert!(!help.contains("\n  user"), "{help}");
        assert!(Cli::try_parse_from(["rbx-ops", "user", "builderman", "--id"]).is_ok());
    }
}
