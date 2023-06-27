use sea_orm_migration::prelude::*;

use crate::m20230408_000001_create_table_user::User;
use crate::PXID_LENGTH;

const MAX_BODY_CONTENT_LENGTH: u32 = 2048;
const MAX_TITLE_CONTENT_LENGTH: u32 = 150;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Feed::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Feed::Id)
                            .string_len(PXID_LENGTH)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Feed::Title)
                            .string_len(MAX_TITLE_CONTENT_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Feed::Body)
                            .string_len(MAX_BODY_CONTENT_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Feed::UserId)
                            .string_len(PXID_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Feed::CreatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .col(
                        ColumnDef::new(Feed::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .extra(String::from("DEFAULT NOW()::timestamp")),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_feed_user_id")
                            .from(Feed::Table, Feed::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Feed::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Feed {
    Table,
    Id,
    Title,
    Body,
    UserId,
    CreatedAt,
    UpdatedAt,
}