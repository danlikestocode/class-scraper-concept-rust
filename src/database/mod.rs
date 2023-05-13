/*
TODO: test that watchlist error works by deleting watchlist and trying to grab it
*/

#[allow(warnings, unused)]
mod prisma;

use prisma_client_rust::QueryError;
use prisma::PrismaClient;
use prisma::class;

async fn get_prisma_client() -> Result<PrismaClient, ()> {
    let client = PrismaClient::_builder().build().await;

    if client.is_err() {
        log::error!("Failed to get prisma client");
        return Err(());
    }

    return Ok(client.unwrap());
}


pub async fn get_class_by_code(code: String) -> Result<class::Data, ()> {
    log::debug!("Getting class by code");

    let client = get_prisma_client().await.unwrap();

    let post = client
        .class()
        .find_unique(class::code::equals(code))
        .exec()
        .await;

    if post.is_err() {
        log::error!("Failed to get class by code");
        return Err(());
    }

    if post.as_ref().unwrap().is_none() {
        log::error!("Class by code does not exist");
        return Err(());
    }

    log::debug!("Found class by code");
    return Ok(post.unwrap().unwrap());
}

pub async fn delete_class_by_code(code: String) 
        -> Result<i64, ()> {
    log::debug!("Deleting class by code");

    let client = PrismaClient::_builder().build().await.unwrap();

    let deleted_class = client
        .class()
        .delete_many(vec![
            class::code::equals(code)
        ])
        .exec()
        .await;

    if deleted_class.is_err() {
        log::error!("Failed to delete class by code");
        return Err(());
    }

    log::debug!("Deleted class by code");
    return Ok(deleted_class.unwrap());
}

pub async fn get_classes_on_watchlist() -> Result<Vec<class::Data>, ()> {
    log::debug!("Getting classes on watchlist");

    let client = get_prisma_client().await.unwrap();

    // Grab all classes that are not open from DB
    let classes: Result<Vec<class::Data>, QueryError> = client
        .class()
        .find_many(vec![
            class::has_open_seats::equals(false)
        ])
        .exec()
        .await;

    if classes.is_err() {
        log::error!("Failed to get classes on watchlist");
        return Err(());
    }

    let amount = &classes.as_ref().unwrap().len();

    if amount == &0 {
        log::error!("No classes on watchlist");
        return Err(());
    }

    log::debug!("Found {} classes on watchlist", amount);
    return Ok(classes.unwrap());
}

