# Rust Class Scraper
This project is a proof of concept for an application that check 
student class availability. It is *NOT* ready for production
and has some serious drawbacks. This was mainly implemented to test
how well a single rust application can handle making concurrent
GET requests to an API endpoint.  
  
While this project was decently successful a rewrite to GO &
exposing a public API to check classes seems viable.

### Technologies Used
* Rust
* Reqwest
* Tokio
* Prisma
* Planetscale
