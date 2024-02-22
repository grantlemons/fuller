pub mod types {
    pub(crate) trait ResponseType: std::fmt::Debug + serde::de::DeserializeOwned {}
    impl<T: ResponseType> ResponseType for Vec<T> {}

    mod assignment;
    mod course;
    mod profile;
    mod submission;
    mod todo;

    pub use assignment::*;
    pub use course::*;
    pub use profile::*;
    pub use submission::*;
    pub use todo::*;
}

pub mod requests;
pub use requests::create_client;
