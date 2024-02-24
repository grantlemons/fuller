use anyhow::Context;
use itertools::Itertools;

#[tokio::test]
async fn test_self_info() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        canvas_api::create_test_client(auth_token.secret()).context("Creating Client Failed!")?;

    let profile = canvas_api::requests::get_self(client).await;
    assert!(profile.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_todo() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        canvas_api::create_test_client(auth_token.secret()).context("Creating Client Failed!")?;

    let todo = canvas_api::requests::get_todo(client).await;
    assert!(todo.is_ok());
    assert!(todo?.iter().map(|todo| todo.html_url.clone()).all_unique());

    Ok(())
}

#[tokio::test]
async fn test_course_list() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        canvas_api::create_test_client(auth_token.secret()).context("Creating Client Failed!")?;

    let courses = canvas_api::requests::get_courses(client).await;
    assert!(courses.is_ok());
    assert!(courses?.iter().map(|course| course.id).all_unique());

    Ok(())
}

#[tokio::test]
async fn test_course_assignments() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        canvas_api::create_test_client(auth_token.secret()).context("Creating Client Failed!")?;

    let courses = canvas_api::requests::get_courses(client.clone()).await?;
    let handles = courses.iter().map(|course| course.id).map(|course_id| {
        tokio::spawn(canvas_api::requests::list_course_assignments(
            client.clone(),
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
pub async fn test_modules_list() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        canvas_api::create_test_client(auth_token.secret()).context("Creating Client Failed!")?;

    let courses = canvas_api::requests::get_courses(client.clone()).await?;
    let handles = courses.iter().map(|course| course.id).map(|course_id| {
        tokio::spawn(canvas_api::requests::list_course_modules(
            client.clone(),
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
pub async fn test_modules_items_list() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        canvas_api::create_test_client(auth_token.secret()).context("Creating Client Failed!")?;

    let courses = canvas_api::requests::get_courses(client.clone()).await?;
    let handles = courses.iter().map(|course| course.id).map(|course_id| {
        tokio::spawn(canvas_api::requests::list_course_modules_with_items(
            client.clone(),
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
pub async fn test_discussions_list() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        canvas_api::create_test_client(auth_token.secret()).context("Creating Client Failed!")?;

    let courses = canvas_api::requests::get_courses(client.clone()).await?;
    let handles = courses.iter().map(|course| course.id).map(|course_id| {
        tokio::spawn(canvas_api::requests::list_course_discussions(
            client.clone(),
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
pub async fn test_discussion_replies_list() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        canvas_api::create_test_client(auth_token.secret()).context("Creating Client Failed!")?;

    let courses = canvas_api::requests::get_courses(client.clone()).await?;
    let client_handles = courses.iter().map(|course| course.id).map(|course_id| {
        (
            course_id,
            tokio::spawn(canvas_api::requests::list_course_discussions(
                client.clone(),
                course_id,
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
                tokio::spawn(canvas_api::requests::list_course_discussion_replies(
                    client.clone(),
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
