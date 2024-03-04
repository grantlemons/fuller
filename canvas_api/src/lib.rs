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

        pub trait Viewable {
            fn view(&self, config: &canvas_cli_config::Config) -> String;
        }

        mod assignment;
        mod conversation;
        mod course;
        mod discussion;
        mod module;
        mod profile;
        mod submission;
        mod todo;

        pub use assignment::*;
        pub use conversation::*;
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

#[allow(unused)]
fn datetime_format(config: &canvas_cli_config::Config) -> String {
    format!("{} {}", config.formatting.date, config.formatting.time)
}

pub mod file_upload;
pub mod requests;

pub use file_upload::*;
pub use requests::create_client;
