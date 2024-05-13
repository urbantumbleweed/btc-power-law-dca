use warp::Filter;
use env_logger;
use log;

#[tokio::main]
async fn main() {
        // Initialize logger
        env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();
    
    // Create a filter for logging
    let log = warp::log::custom(|info| {
        eprintln!(
            "Logged Method: {:?}, Path: {:?}, Status: {:?}, User Agent: {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.user_agent(),
        );
    });

    // Create your routes here
    let routes = warp::any()
        .map(|| "Hell, world!")
        .with(log);  // Apply the logging filter

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod tests {
    use warp::Filter;
    use warp::test::request;

    #[tokio::test]
    async fn test_hello_world() {
        let api = warp::get().map(|| "Hello, world!");

        let resp = request()
            .method("GET")
            .path("/")
            .reply(&api)
            .await;

        assert_eq!(resp.status(), 200, "Should return 200 OK.");
        assert_eq!(resp.body(), "Hello, world!", "Should return 'Hello, world!'.");
    }
}