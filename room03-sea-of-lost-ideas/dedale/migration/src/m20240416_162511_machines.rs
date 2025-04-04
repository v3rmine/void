use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Machines::Table)
                    .col(pk_auto(Machines::Id))
                    .col(string_uniq(Machines::Name))
                    .col(integer(Machines::AppId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-machines-apps")
                            .from(Machines::Table, Machines::AppId)
                            .to(Apps::Table, Apps::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Machines::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Machines {
    Table,
    Id,
    Name,
    AppId,
}

#[derive(DeriveIden)]
enum Apps {
    Table,
    Id,
}
