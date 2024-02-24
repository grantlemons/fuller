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

    let courses = canvas_api::requests::get_courses(client.clone()).await;
    assert!(courses.is_ok());
    let courses = courses.context("Unable to fetch courses list")?;

    {
        let client = &client;
        let list_tasks = courses
            .iter()
            .map(|course| course.id)
            .map(|course_id| {
                tokio::spawn(canvas_api::requests::list_course_assignments(
                    client.clone(),
                    course_id,
                ))
            })
            .collect_vec();
        for task in list_tasks {
            let assignments = task.await.unwrap();
            assert!(assignments.is_ok());
            assert!(assignments?.iter().map(|course| course.id).all_unique());
        }
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

    let courses = canvas_api::requests::get_courses(client.clone()).await;
    assert!(courses.is_ok());
    let courses = courses.context("Unable to fetch courses list")?;

    {
        let client = &client;
        let list_tasks = courses
            .iter()
            .map(|course| course.id)
            .map(|course_id| {
                tokio::spawn(canvas_api::requests::list_course_modules(
                    client.clone(),
                    course_id,
                ))
            })
            .collect_vec();
        for task in list_tasks {
            let modules = task.await.unwrap();
            assert!(modules.is_ok());
            assert!(modules?.iter().map(|module| module.id).all_unique());
        }
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

    let courses = canvas_api::requests::get_courses(client.clone()).await;
    assert!(courses.is_ok());
    let courses = courses.context("Unable to fetch courses list")?;

    {
        let client = &client;
        let list_modules_tasks = courses
            .iter()
            .map(|course| course.id)
            .map(|course_id| {
                tokio::spawn(canvas_api::requests::list_course_modules_with_items(
                    client.clone(),
                    course_id,
                ))
            })
            .collect_vec();
        for task in list_modules_tasks {
            let modules = task.await.unwrap();
            assert!(modules.is_ok());
            let items = modules?
                .into_iter()
                .flat_map(|module| module.items)
                .flatten();
            assert!(items.map(|item| item.id).all_unique());
        }
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

    let courses = canvas_api::requests::get_courses(client.clone()).await;
    assert!(courses.is_ok());
    let courses = courses.context("Unable to fetch courses list")?;

    {
        let client = &client;
        let list_tasks = courses
            .iter()
            .map(|course| course.id)
            .map(|course_id| {
                tokio::spawn(canvas_api::requests::list_course_discussions(
                    client.clone(),
                    course_id,
                ))
            })
            .collect_vec();
        for task in list_tasks {
            let discussions = task.await.unwrap();
            assert!(discussions.is_ok());
            assert!(discussions?
                .iter()
                .map(|discussion| discussion.id)
                .all_unique());
        }
    }

    Ok(())
}
