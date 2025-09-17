use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(GuildMember::Table)
                    .if_not_exists()
                    .col(pk_auto(GuildMember::Id))
                    .col(integer(GuildMember::Guild).not_null())
                    .col(binary(GuildMember::UserId).not_null())
                    .col(binary(GuildMember::GuildUserBlindId).not_null())
                    .col(binary(GuildMember::RowEncryptionKey).not_null())
                    .col(binary(GuildMember::RowEncryptionNonce).not_null())
                    .col(timestamp_null(GuildMember::LastActiveAt))
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(GuildMember::Guild)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(GuildMember::GuildUserBlindIdIndex.to_string())
                    .table(GuildMember::Table)
                    .if_not_exists()
                    .col(GuildMember::GuildUserBlindId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GuildMember::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Guild {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum GuildMember {
    Table,
    Id,
    Guild,
    UserId,
    GuildUserBlindId,
    RowEncryptionKey,
    RowEncryptionNonce,
    LastActiveAt,
    GuildUserBlindIdIndex,
}
