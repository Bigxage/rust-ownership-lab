use serde::Deserialize; //import the tool to read JSON

//1. THE DATA MODEL
//the API sends back JSON like: { "bitcoin": { "usd": 45000 }}
//we need a struct that looks EXACTLY like that data so rust can catch it
#[derive(Deserialize, Debug)]
struct PriceResponse {
    bitcoin: Currency,
}

#[derive(Deserialize, Debug)]
struct Currency {
    usd: f64,
}

//2. the async main
//normal 'fn main' can't wait. we use 'tokio::main' to give it superpowers.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Requesting Bitcoin Price...");

    //3. the url
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";

    //4. the request (the magic happens here)
    // .get(url) -> prepares the call.
    // .send() -> hits the button.
    // .await -> "pause here until the internet replies."
   // 1. Create a "Client" that can hold settings
    let client = reqwest::Client::new();

    // 2. Build the request with a "User-Agent" header (The ID Card)
    let response = client
        .get(url)
        .header("User-Agent", "MyRustProject/1.0") // <--- This is the key!
        .send()
        .await?;

    //5. check if it worked
    if response.status().is_success() {
        //6. parse the JSON
        //convert the text response into our 'priceresponse' struct
        // .await is needed again because reading the body takes time.
        let body: PriceResponse = response.json().await?;

        println!("-------------------------");
        println!("💰 Bitcoin Price: ${}", body.bitcoin.usd);
        println!("-------------------------")
    }  else {
      println!("Server Error: {}", response.status());
    }
    Ok(())
}