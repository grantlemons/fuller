use anyhow::{Context, Result};
use itertools::Itertools;
use std::path::PathBuf;

use fuller_config::Config;
use reqwest::Client;

const CONFIG_FILE: &str = "../config.toml";
async fn setup() -> Result<(Config, Client)> {
    let config = fuller_config::get_config(Some(PathBuf::from(CONFIG_FILE)))?;
    let auth_token = fuller_canvas_auth::connect(&config)
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        fuller_canvas_api::create_client(auth_token, &config).context("Creating Client Failed!")?;

    Ok((config, client))
}

#[tokio::test]
async fn test_self_info() -> Result<()> {
    let (config, client) = setup().await?;

    let profile = fuller_canvas_api::requests::get_self(client, &config).await;
    assert!(profile.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_todo() -> Result<()> {
    let (config, client) = setup().await?;

    let todo = fuller_canvas_api::requests::get_todo(client, &config).await;
    assert!(todo.is_ok());
    assert!(todo?
        .iter()
        .map(|todo| todo.html_url.to_owned())
        .all_unique());

    Ok(())
}

#[tokio::test]
async fn test_course_list() -> Result<()> {
    let (config, client) = setup().await?;

    let courses = fuller_canvas_api::requests::get_courses(client, &config).await;
    assert!(courses.is_ok());
    assert!(courses?.iter().map(|course| course.id).all_unique());

    Ok(())
}

#[tokio::test]
async fn test_course_assignments() -> Result<()> {
    let (config, client) = setup().await?;

    let courses = fuller_canvas_api::requests::get_courses(client.to_owned(), &config).await?;
    let handles = courses.iter().map(|course| course.id).map(|course_id| {
        tokio::spawn(fuller_canvas_api::requests::list_course_assignments(
            client.to_owned(),
            config.to_owned(),
            course_id,
        ))
    });
    for handle in handles {
        let assignments = handle.await?;
        assert!(assignments.is_ok());
        assert!(assignments?
            .iter()
            .map(|assignment| assignment.id)
            .all_unique());
    }

    Ok(())
}

#[tokio::test]
pub async fn test_modules_list() -> Result<()> {
    let (config, client) = setup().await?;

    let courses = fuller_canvas_api::requests::get_courses(client.to_owned(), &config).await?;
    let handles = courses.iter().map(|course| course.id).map(|course_id| {
        tokio::spawn(fuller_canvas_api::requests::list_course_modules(
            client.to_owned(),
            config.to_owned(),
            course_id,
        ))
    });
    for handle in handles {
        let modules = handle.await?;
        assert!(modules.is_ok());
        assert!(modules?.iter().map(|module| module.id).all_unique());
    }

    Ok(())
}

#[tokio::test]
pub async fn test_modules_items_list() -> Result<()> {
    let (config, client) = setup().await?;

    let courses = fuller_canvas_api::requests::get_courses(client.to_owned(), &config).await?;
    let handles = courses.iter().map(|course| course.id).map(|course_id| {
        tokio::spawn(fuller_canvas_api::requests::list_course_modules_with_items(
            client.to_owned(),
            config.to_owned(),
            course_id,
        ))
    });
    for handle in handles {
        let module = handle.await?;
        assert!(module.is_ok());
        assert!(module?
            .into_iter()
            .flat_map(|module| module.items)
            .flatten()
            .map(|item| item.id)
            .all_unique());
    }

    Ok(())
}

#[tokio::test]
pub async fn test_discussions_list() -> Result<()> {
    let (config, client) = setup().await?;

    let courses = fuller_canvas_api::requests::get_courses(client.to_owned(), &config).await?;
    let handles = courses.iter().map(|course| course.id).map(|course_id| {
        tokio::spawn(fuller_canvas_api::requests::list_course_discussions(
            client.to_owned(),
            config.to_owned(),
            course_id,
        ))
    });
    for handle in handles {
        let discussions = handle.await?;
        assert!(discussions.is_ok());
        assert!(discussions?
            .iter()
            .map(|discussion| discussion.id)
            .all_unique());
    }

    Ok(())
}

#[tokio::test]
pub async fn test_discussion_replies_list() -> Result<()> {
    let (config, client) = setup().await?;

    let courses = fuller_canvas_api::requests::get_courses(client.to_owned(), &config).await?;
    let client_handles = courses.iter().map(|course| course.id).map(|course_id| {
        (
            course_id,
            tokio::spawn(fuller_canvas_api::requests::list_course_discussions(
                client.to_owned(),
                config.to_owned(),
                course_id.to_owned(),
            )),
        )
    });
    for (course_id, handle) in client_handles {
        let discussions = handle.await?;
        assert!(discussions.is_ok());
        let reply_handles = discussions?
            .into_iter()
            .map(|discussion| discussion.id)
            .map(|discussion_id| {
                tokio::spawn(fuller_canvas_api::requests::list_course_discussion_replies(
                    client.to_owned(),
                    config.to_owned(),
                    course_id,
                    discussion_id,
                ))
            });
        for handle in reply_handles {
            let replies = handle.await?;
            assert!(replies.is_ok());
            assert!(replies?.iter().map(|reply| reply.id).all_unique());
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_conversations() -> Result<()> {
    let (config, client) = setup().await?;

    let conversation_overviews =
        fuller_canvas_api::requests::list_conversations(client.to_owned(), &config).await?;
    let handles = conversation_overviews
        .iter()
        .map(|conversation| conversation.id)
        .map(|conversation_id| {
            tokio::spawn(fuller_canvas_api::requests::get_conversation(
                client.to_owned(),
                config.to_owned(),
                conversation_id,
            ))
        });
    let mut conversations = Vec::new();
    for handle in handles {
        let conversation_options = handle.await?;
        assert!(conversation_options.is_ok());

        conversations.push(conversation_options?);
    }
    assert!(conversations
        .iter()
        .map(|conversation| conversation.id)
        .all_unique());

    Ok(())
}
