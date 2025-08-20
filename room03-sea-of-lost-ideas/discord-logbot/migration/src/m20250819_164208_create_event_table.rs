use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Event::Table)
                    .if_not_exists()
                    .col(pk_auto(Event::Id))
                    .col(integer(Event::Guild).not_null())
                    .col(integer(Event::GuildMember).not_null())
                    .col(enumeration(
                        Event::Type,
                        "type",
                        vec!["message", "voice", "guild", "user"],
                    ))
                    .col(enumeration(
                        Event::SubType,
                        "subtype",
                        vec![
                            "message_update",
                            "message_delete",
                            "voice_join",
                            "voice_leave",
                            "guild_join",
                            "guild_leave",
                            "user_change_nick",
                            "user_change_avatar",
                            "user_change_bio",
                            "user_change_username",
                        ],
                    ))
                    .col(
                        timestamp(Event::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(json(Event::Data).not_null())
                    .col(binary(Event::RowEncryptionKey).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(Event::Guild)
                            .to(Guild::Table, Guild::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_col(Event::GuildMember)
                            .to(GuildMember::Table, GuildMember::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(Event::TypeIndex.to_string())
                    .table(Event::Table)
                    .if_not_exists()
                    .col(Event::Type)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name(Event::SubTypeIndex.to_string())
                    .table(Event::Table)
                    .if_not_exists()
                    .col(Event::SubType)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
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
}

#[derive(DeriveIden)]
enum Event {
    Table,
    Id,
    Guild,
    GuildMember,
    Type,
    SubType,
    Data,
    CreatedAt,
    RowEncryptionKey,
    TypeIndex,
    SubTypeIndex,
}
