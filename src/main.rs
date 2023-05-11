use dotenv::dotenv;

#[tokio::main]
async fn main() {
    // let url = format!(
    //     "https://eadvs-cscc-catalog-api.apps.asu.edu/catalog-microservices/api/v1/search/classes?&refine=Y&campusOrOnlineSelection=A&catalogNbr=330&honors=F&promod=F&searchType=all&subject=CSE&term=2237"
    // );


    dotenv().ok();
    let myurl = std::env::var("TURSO_DB_URL").expect("URL must be set");
    let token = std::env::var("TURSO_DB_AUTH_TOKEN").expect("AUTH TOKEN must be set");

}

