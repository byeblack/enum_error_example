use entity::post::{self, Status};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use tokio::fs::File;

#[tokio::main]
async fn main() {
    File::create("data.sqlite").await.unwrap();

    let connection = sea_orm::Database::connect("sqlite://data.sqlite")
        .await
        .unwrap();
    Migrator::up(&connection, None).await.unwrap();

    post::Entity::delete_by_id(1)
        .exec(&connection)
        .await
        .unwrap();

    post::ActiveModel {
        id: Set(1),
        title: Set("Hello".to_string()),
        text: Set(Status::Open),
    }
    .insert(&connection)
    .await
    .unwrap();
}
