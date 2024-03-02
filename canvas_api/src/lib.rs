pub mod types {
    pub mod requests {
        pub(crate) trait RequestType: std::fmt::Debug + serde::ser::Serialize {}
        impl<T: RequestType> RequestType for Vec<T> {}

        mod submission;

        pub use submission::*;
    }

    pub mod responses {
        pub(crate) trait ResponseType:
            std::fmt::Debug + std::cmp::PartialEq + serde::de::DeserializeOwned
        {
        }
        impl<T: ResponseType> ResponseType for Vec<T> {}

        mod assignment;
        mod course;
        mod discussion;
        mod module;
        mod profile;
        mod submission;
        mod todo;

        pub use assignment::*;
        pub use course::*;
        pub use discussion::*;
        pub use module::*;
        pub use profile::*;
        pub use submission::*;
        pub use todo::*;
    }

    pub use requests::*;
    pub use responses::*;
}

pub mod requests;
pub use requests::create_client;
