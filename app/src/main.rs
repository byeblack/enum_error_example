use entity::post::{self, Status};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ColumnTrait, EntityTrait, Iterable, QueryFilter, Set};
use tokio::fs::File;

#[tokio::main]
async fn main() {
    File::create("data.sqlite").await.unwrap();

    let connection = migration::sea_orm::Database::connect("sqlite://data.sqlite")
        .await
        .unwrap();
    Migrator::up(&connection, None).await.unwrap();

    let connection = sea_orm::Database::connect("sqlite://data.sqlite")
        .await
        .unwrap();

    for (index, item) in Status::iter().enumerate() {
        let model = post::ActiveModel {
            id: Set(index as i32),
            title: Set("title".to_string()),
            text: Set(item.clone()),
        };
        post::Entity::insert(model).exec(&connection).await.unwrap();
    }

    let posts = post::Entity::find().all(&connection).await.unwrap();
    for post in posts {
        println!("{:?}", post);
    }

    for item in Status::iter() {
        let post = post::Entity::find()
            .filter(post::Column::Text.eq(item.clone()))
            .one(&connection)
            .await
            .unwrap();

        println!("{item:?} - {:?}", post);
    }
}
