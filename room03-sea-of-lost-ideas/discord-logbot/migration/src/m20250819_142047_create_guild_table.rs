use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(pk_auto(Guild::Id))
                    .col(string(Guild::GuildId).not_null().unique_key())
                    .col(binary(Guild::PublicKey).not_null())
                    .col(binary(Guild::PrivateKey).not_null())
                    .col(binary(Guild::Nonce).not_null())
                    .col(
                        timestamp(Guild::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(Guild::GuildIdIndex.to_string())
                    .table(Guild::Table)
                    .if_not_exists()
                    .col(Guild::GuildId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Guild::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
    GuildId,
    CreatedAt,
    PublicKey,
    PrivateKey,
    Nonce,
    GuildIdIndex,
}
