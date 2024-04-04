use std::io::Read;

pub struct Caller {}

pub fn test() {
    let cookies: &str = "Authorization=Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ1c2VyMSIsImV4cCI6MTcxOTYwOTUxOH0.L6lttRKQBsLihnDS1EevzhpEh2m9VC2uVHUmIqwenmI; Path=/; HttpOnly; Expires=Fri, 28 Jun 2024 21:18:38 GMT;";

    // Create a Reqwest client with a cookie store
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    // Make the request with cookies
    let mut response = client
        .get("http://127.0.0.1:8080/todo")
        .header(reqwest::header::COOKIE, cookies)
        .send()
        .unwrap();

    // Read the response body into a string
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    // Deserialize the JSON string to serde_json::Value for pretty printing
    let json_value: serde_json::Value = serde_json::from_str(&body).unwrap();

    // Serialize the serde_json::Value to a pretty JSON string
    let pretty_json = serde_json::to_string_pretty(&json_value).unwrap();

    // Print the prettified JSON
    println!("{}", pretty_json);
}
