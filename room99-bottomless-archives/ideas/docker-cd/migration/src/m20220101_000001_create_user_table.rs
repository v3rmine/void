use entities::users;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(users::Entity)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(users::Column::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(users::Column::Name)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(users::Column::Password).text().not_null())
                    .clone(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(users::Entity).if_exists().clone())
            .await
    }
}
