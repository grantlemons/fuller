use anyhow::Context;

#[tokio::test]
async fn test_self_info() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        canvas_api::create_client(auth_token.secret()).context("Creating Client Failed!")?;

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
        canvas_api::create_client(auth_token.secret()).context("Creating Client Failed!")?;

    let todo = canvas_api::requests::get_todo(client).await;
    assert!(todo.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_course_list() -> anyhow::Result<()> {
    let auth_token = canvas_auth::connect()
        .await
        .context("Fetching Auth Token Failed!")?;
    let client =
        canvas_api::create_client(auth_token.secret()).context("Creating Client Failed!")?;

    let courses = canvas_api::requests::get_courses(client).await;
    assert!(courses.is_ok());

    Ok(())
}
