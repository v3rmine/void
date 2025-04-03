use clap::{ArgEnum, Parser};
use derive_more::Display;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// The folder containing the migrations
    #[clap(
        long,
        global = true,
        value_name = "FOLDER",
        default_value = "./migrations"
    )]
    pub migration_folder: String,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub enum Command {
    /// Apply the next not runned migration
    Up {
        /// The database url default from the env variable DATABASE_URL
        #[clap(short, long, value_name = "URL", env = "DATABASE_URL", required = true)]
        database_url: String,
    },
    /// Rollback the last runned migration
    Down {
        /// The database url default from the env variable DATABASE_URL
        #[clap(short, long, value_name = "URL", env = "DATABASE_URL", required = true)]
        database_url: String,
    },
    /// Apply all not runned migrations
    Apply {
        /// The database url default from the env variable DATABASE_URL
        #[clap(short, long, value_name = "URL", env = "DATABASE_URL", required = true)]
        database_url: String,
    },
    /// Rollback all runned migrations
    Rollback {
        /// The database url default from the env variable DATABASE_URL
        #[clap(short, long, value_name = "URL", env = "DATABASE_URL", required = true)]
        database_url: String,
    },
    /// Rollback and reapply all the migrations
    Refresh {
        /// The database url default from the env variable DATABASE_URL
        #[clap(short, long, value_name = "URL", env = "DATABASE_URL", required = true)]
        database_url: String,
    },
    /// Print the migrations statuses
    Status {
        /// The database url default from the env variable DATABASE_URL
        #[clap(short, long, value_name = "URL", env = "DATABASE_URL", required = true)]
        database_url: String,
    },
    /// Create a new migration
    Create {
        #[clap(short, long)]
        name: String,
        #[clap(long, value_enum, default_value = "sql")]
        migration_type: MigrationType,
    },
    /// Validate the migrations
    Validate,
}

#[derive(Debug, Clone, ArgEnum, Display)]
pub enum MigrationType {
    #[display(fmt = "shell")]
    Shell,
    #[display(fmt = "sql")]
    Sql,
}
