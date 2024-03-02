use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SubmissionRequest {
    comment: Option<String>,
    submission_type: SubmissionRequestType,
    /// OnlineTextEntry only
    body: Option<String>,
    /// OnlineUrl
    url: Option<String>,
    /// OnlineUpload
    file_ids: Option<Vec<u64>>,
    /// MediaRecording
    /// Not Supported!
    media_comment_id: Option<String>,
    /// MediaRecording
    /// audio or video
    /// Not Supported!
    media_comment_type: Option<String>,
    /// StudentAnnotation
    /// Not Supported!
    annotatable_attachment_id: Option<u64>,
}

impl crate::types::RequestType for SubmissionRequest {}

impl SubmissionRequest {
    pub fn new(submission_type: SubmissionRequestType, comment: Option<String>) -> Self {
        let mut res = Self {
            comment,
            submission_type: submission_type.to_owned(),
            body: None,
            url: None,
            file_ids: None,
            media_comment_id: None,
            media_comment_type: None,
            annotatable_attachment_id: None,
        };

        match submission_type {
            SubmissionRequestType::OnlineTextEntry(text) => res.body = Some(text),
            SubmissionRequestType::OnlineUrl(url) => res.url = Some(url),
            SubmissionRequestType::OnlineUpload(ids) => res.file_ids = Some(ids),
            _ => {}
        };

        res
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum SubmissionRequestType {
    /// html body
    OnlineTextEntry(String),
    /// http or https only
    OnlineUrl(String),
    /// list of uploaded file ids
    OnlineUpload(Vec<u64>),
    /// Not Supported!
    MediaRecording,
    /// Not Supported!
    StudentAnnotation,
    /// Not Supported!
    OnlineQuiz,
}
