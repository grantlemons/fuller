pub mod types {
    mod assignment;
    mod course;
    mod submission;
    mod todo;

    pub use assignment::*;
    pub use course::*;
    pub use submission::*;
    pub use todo::*;
}
pub mod requests {
    const BASE_URL: &str = "";

    async fn get_generic<'a, T: serde::de::DeserializeOwned>(
        client: reqwest::Client,
        path: &str,
        query: Option<&[(&str, &str)]>,
    ) -> reqwest::Result<T> {
        let request = client.get(BASE_URL.to_owned() + path);
        match query {
            Some(q) => request.query(q).send().await?.json::<T>().await,
            None => request.send().await?.json::<T>().await,
        }
    }

    mod assignment;
    mod todo;

    pub use assignment::*;
    pub use todo::*;
}
