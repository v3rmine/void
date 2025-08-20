pub use sea_orm_migration::prelude::*;

mod m20250819_142047_create_guild_table;
mod m20250819_162047_create_guild_member_table;
mod m20250819_164208_create_event_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250819_142047_create_guild_table::Migration),
            Box::new(m20250819_162047_create_guild_member_table::Migration),
            Box::new(m20250819_164208_create_event_table::Migration),
        ]
    }
}
