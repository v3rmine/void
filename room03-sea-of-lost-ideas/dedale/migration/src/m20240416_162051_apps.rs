use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Apps::Table)
                    .col(pk_auto(Apps::Id))
                    .col(string_uniq(Apps::Name))
                    .col(integer(Apps::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-apps-users")
                            .from(Apps::Table, Apps::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Apps::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Apps {
    Table,
    Id,
    Name,
    UserId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
