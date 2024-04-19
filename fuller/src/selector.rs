use crate::error::Error;
use fuller_canvas_api::Client;
use fuller_canvas_api::{requests::*, types::*};
use fuller_config::Config;
use tracing::info;

pub async fn prompt_selector<T: std::fmt::Display + std::fmt::Debug>(
    options: Vec<T>,
) -> Result<T, crate::Error> {
    let selection = inquire::Select::new("", options)
        .with_vim_mode(true)
        .with_page_size(15)
        .prompt()?;
    info!("User made selection: {}", selection);

    Ok(selection)
}

pub async fn prompt_multiselector<T: std::fmt::Display + std::fmt::Debug>(
    options: Vec<T>,
) -> Result<Vec<T>, crate::Error> {
    let selection = inquire::MultiSelect::new("", options)
        .with_vim_mode(true)
        .with_page_size(15)
        .prompt()?;
    info!("User made selections: {:?}", selection);

    Ok(selection)
}

pub fn text_entry(message: &str) -> Result<String, inquire::InquireError> {
    inquire::Editor::new(message).prompt()
}

pub async fn select_conversation(
    request_client: Client,
    config: &Config,
    conversation_id: Option<u64>,
) -> Result<Conversation, Error> {
    if let Some(course_id) = conversation_id {
        Ok(get_conversation(request_client, config, course_id).await?)
    } else {
        let conversation_id =
            prompt_selector(list_conversations(request_client.to_owned(), config).await?)
                .await?
                .id;
        Ok(get_conversation(request_client, config, conversation_id).await?)
    }
}

pub async fn select_course(
    request_client: Client,
    config: &Config,
    course_id: Option<u64>,
) -> Result<Course, Error> {
    if let Some(course_id) = course_id {
        Ok(get_course(request_client, config, course_id).await?)
    } else {
        prompt_selector(
            get_courses(request_client, config)
                .await?
                .into_iter()
                .filter(|c| !config.ignore.courses.contains(&(c.id as i64)))
                .collect(),
        )
        .await
    }
}

pub async fn select_courses(
    request_client: Client,
    config: &Config,
    course_ids: Option<Vec<u64>>,
) -> Result<Vec<Course>, Error> {
    let courses = get_courses(request_client, config)
        .await?
        .into_iter()
        .filter(|c| !config.ignore.courses.contains(&(c.id as i64)));

    if let Some(course_ids) = course_ids {
        Ok(courses.filter(|c| course_ids.contains(&c.id)).collect())
    } else {
        prompt_multiselector(courses.collect()).await
    }
}

pub async fn select_assignment(
    request_client: Client,
    config: &Config,
    course_id: Option<u64>,
    assignment_id: Option<u64>,
) -> Result<Assignment, Error> {
    let course_id = select_course(request_client.clone(), config, course_id)
        .await?
        .id;

    if let Some(assignment_id) = assignment_id {
        Ok(get_assignment(request_client, config, course_id, assignment_id).await?)
    } else {
        prompt_selector(
            list_course_assignments(request_client, config, course_id)
                .await?
                .into_iter()
                .filter(|a| !config.ignore.assignments.contains(&(a.id as i64)))
                .collect(),
        )
        .await
    }
}

pub async fn select_todo(request_client: Client, config: &Config) -> Result<Todo, Error> {
    let mut wrapper_vec =
        TodoDisplayWrapper::wrap_vec(get_todo(request_client, config).await?, config);
    wrapper_vec.sort_by_key(|a| a.due_date.to_owned());
    let wrapped_choice = prompt_selector(wrapper_vec).await?;

    Ok(wrapped_choice.unwrap())
}

pub async fn select_course_todo(
    request_client: Client,
    config: &Config,
    course_id: Option<u64>,
) -> Result<Todo, Error> {
    let course_id = select_course(request_client.clone(), config, course_id)
        .await?
        .id;

    let mut wrapper_vec = TodoDisplayWrapper::wrap_vec(
        get_course_todo(request_client, config, course_id).await?,
        config,
    );
    wrapper_vec.sort_by_key(|a| a.due_date.to_owned());
    let wrapped_choice = prompt_selector(wrapper_vec).await?;

    Ok(wrapped_choice.unwrap())
}

#[derive(Debug)]
struct TodoDisplayWrapper {
    todo: Todo,
    due_date: String,
}

impl std::fmt::Display for TodoDisplayWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.due_date, self.todo)
    }
}

impl TodoDisplayWrapper {
    pub fn wrap(todo: Todo, config: &Config) -> Self {
        let rendered_due_date = if let Some(assignment) = &todo.assignment {
            if let Some(due_at) = assignment.due_at {
                format!(
                    "({})  ",
                    chrono::DateTime::<chrono::Local>::from(due_at)
                        .format(&fuller_canvas_api::datetime_format(&config))
                        .to_string()
                )
            } else {
                String::default()
            }
        } else {
            String::default()
        };

        Self {
            todo,
            due_date: rendered_due_date,
        }
    }

    pub fn wrap_vec(todo: Vec<Todo>, config: &Config) -> Vec<Self> {
        todo.into_iter().map(|t| Self::wrap(t, config)).collect()
    }

    pub fn unwrap(self) -> Todo {
        self.todo
    }
}
