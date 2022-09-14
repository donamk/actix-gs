use actix_gs::configuration::get_configuration;
use actix_gs::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    application_task
        .await
        .unwrap()
        .expect("failed to start server.");

    Ok(())
}
