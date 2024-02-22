pub mod types {
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
