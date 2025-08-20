use migration::MigratorTrait;
use poise::serenity_prelude as serenity;
use sea_orm::{
    ColumnTrait, ConnectOptions, ConnectionTrait, Database, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use tracing::level_filters::LevelFilter;

mod commands;
mod entities;
mod event_handler;

struct Data {
    pub database: DatabaseConnection,
}
impl Data {
    pub async fn get_guild(&self, guild_id: serenity::GuildId) -> Option<entities::guild::Model> {
        entities::guild::Entity::find()
            .filter(entities::guild::Column::GuildId.eq(guild_id.to_string()))
            .one(&self.database)
            .await
            .ok()
            .flatten()
    }
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .parse_lossy(
                    std::env::var("RUST_LOG")
                        .unwrap_or_else(|_| "discord_logbot=trace".to_string()),
                ),
        )
        .init();

    let database_options = ConnectOptions::new(
        std::env::var("DATABASE_URL").unwrap_or_else(|_| format!("sqlite://db.sqlite?mode=rwc")),
    );
    let database = Database::connect(database_options)
        .await
        .expect("Failed to connect to database");
    migration::Migrator::up(&database, None)
        .await
        .expect("Failed to apply new migrations");

    // Optimize sequelize
    database
        .execute_unprepared("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")
        .await
        .expect("Failed to optimize sqlite");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::stats(),
                commands::query(),
                commands::setup(),
                commands::set_admin_role(),
                commands::change_key(),
                commands::purge_data(),
                commands::dump_data(),
                commands::help(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler::event_handler(ctx, event, framework, data))
            },
            pre_command: |ctx| {
                Box::pin(async move {
                    // Store the DB GuildId in the context
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { database })
            })
        })
        .build();

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::GUILD_PRESENCES
        | serenity::GatewayIntents::GUILD_VOICE_STATES
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
