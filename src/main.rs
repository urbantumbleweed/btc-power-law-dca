fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use warp::Filter;
    use warp::test::request;

    #[tokio::test]
    async fn test_hello_world() {
        let api = warp::get().map(|| "Hell, world!");

        let resp = request()
            .method("GET")
            .path("/")
            .reply(&api)
            .await;

        assert_eq!(resp.status(), 200, "Should return 200 OK.");
        assert_eq!(resp.body(), "Hello, world!", "Should return 'Hello, world!'.");
    }
}