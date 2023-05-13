/*
* TODO: Send email/text when class is available
*/

use crate::database;
use dotenv::dotenv;
use reqwest::{Client, header::AUTHORIZATION};
use std::{thread, time};


pub async fn start_daemon() {
    dotenv().ok();
    let sleep_time = std::env::var("SLEEP_TIME")
        .expect("SLEEP_TIME must be set")
        .parse::<u64>()
        .expect("SLEEP_TIME must be a valid integer");

    let api_search_url = std::env::var("API_SEARCH_URL")
        .expect("API_SEARCH_URL must be set");

    log::debug!("Found sleep_time: {} seconds", sleep_time);
    log::debug!("Found api_search_url: {}", api_search_url);

    let client = Client::new();

    // loop forever and sleep for sleep_time env variable
    loop {
        log::info!("I'm awake! Checking class watchlist...");
        let classes = database::get_classes_on_watchlist().await;

        if classes.is_err() {
            log::error!("Error getting classes on watchlist, will retry in 60 seconds...");
            thread::sleep(time::Duration::from_secs(sleep_time));
            continue;
        }

        let classes = classes.unwrap();

        for class in classes {
            let class_code = class.code;
            let class_availability = check_class_availability(&class_code, &api_search_url, &client).await;

            if class_availability.is_err() {
                log::error!("Error checking class availability for: {}", class_code);
                continue;
            }

            let class_availability = class_availability.unwrap();

            if class_availability > 0 {
                log::info!("Class {} has {} seats available", class_code, class_availability);
                log::info!("Removing class {} from watchlist", class_code);
                database::delete_class_by_code(class_code).await;
            }
        }

        log::info!("Finished checking class watchlist, going to sleep for {} seconds...", sleep_time);
        thread::sleep(time::Duration::from_secs(sleep_time));
    }
}

async fn check_class_availability(code: &str, api_url: &str, client: &Client) 
    -> Result<i32, ()> {
    log::info!("Checking class availability for: {}", code);

    let search_url = 
        format!("{}classes?refine=Y&advanced=true&campusOrOnlineSelection=A&classNbr={}&honors=F&promod=F&searchType=all&term=2237", 
            api_url, 
            code
        );

    let response = client.get(search_url.as_str())
        .header(AUTHORIZATION, "Bearer null")
        .send()
        .await;

    if response.is_err() {
        log::error!("Error checking class availability for: {}", code);
        return Err(());
    }

    let response = response.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            let json = response.json::<serde_json::Value>().await.unwrap();

            if &json["total"]["value"] == 0 {
                log::info!("No results for class: {}", code);
                return Ok(0);
            }

            let max_seats = &json["classes"][0]["reservedSeatsInfo"][0]["ENRL_CAP"];
            let taken_seats = &json["classes"][0]["reservedSeatsInfo"][0]["ENRL_TOT"];
            let available_seats = max_seats.as_u64().unwrap() as i32 - taken_seats.as_u64().unwrap() as i32;



            log::info!("Pulled data for class: {}", code);
            return Ok(available_seats);
        }

        reqwest::StatusCode::UNAUTHORIZED => {
            log::info!("UNAUTHORIZED to grab seat information for: {}", code);
            return Ok(0);
        }

        other => {
            log::error!("Unexpected error: {:?}", other);
            return Err(());
        }
    }
}


