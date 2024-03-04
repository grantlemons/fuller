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
