use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Keypair, Signer},
    },
    Client, Cluster,
};
use clap::{Parser, Subcommand};

use elementerra_sdk::{derive, programs::elementerra};

use std::rc::Rc;

/// Elementerra: Ele CLI
#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    #[clap(flatten)]
    provider_config: ProviderConfig,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Show(Show),
}

#[derive(Subcommand)]
enum Show {
    /// Print the player (given wallet address).
    Player { authority: Pubkey },
    /// Print the current season.
    Season,
}

#[derive(Default, Parser)]
struct ProviderConfig {
    /// RPC URL for the Solana cluster.
    #[clap(long = "provider.cluster", env = "PROVIDER_CLUSTER")]
    pub cluster: Option<Cluster>,
    /// Wallet keypair to use.
    #[clap(long = "provider.wallet", env = "PROVIDER_WALLET")]
    wallet: Option<String>,
}

fn default_keypair() -> Keypair {
    read_keypair_file(&*shellexpand::tilde("~/.config/solana/id.json"))
        .expect("Requires a keypair file")
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let payer = match cli.provider_config.wallet {
        Some(wallet) => read_keypair_file(wallet).expect("Requires a keypair file"),
        None => default_keypair(),
    };

    let url = match cli.provider_config.cluster {
        Some(cluster) => cluster,
        None => Cluster::Devnet,
    };

    let client = Client::new_with_options(
        url,
        Rc::new(Keypair::from_bytes(&payer.to_bytes())?),
        CommitmentConfig::confirmed(),
    );

    let program = client.program(elementerra::ID)?;

    match cli.command {
        Commands::Show(show) => match show {
            Show::Player { authority } => {
                let player_accounts = derive::player_accounts(&program, &authority)?;
                dbg!(player_accounts);
            }
            Show::Season => {
                let season_accounts = derive::current_season_accounts(&program)?;
                dbg!(season_accounts);
            }
        },
    }

    Ok(())
}
