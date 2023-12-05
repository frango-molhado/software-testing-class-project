use std::env;

use anyhow::Result;
use common::database::Database;
use common::io_handler::{DefaultIOHandler, IOHandler};
use dotenv::dotenv;

use receptionist::service_manager::ServiceManager;

fn main() -> Result<()> {
    dotenv().ok();

    let pacient_queue_file_path = env::var("PACIENT_QUEUE_FILE_PATH")?;
    let dentist_queue_file_path = env::var("DENTIST_QUEUE_FILE_PATH")?;

    let io_handler = IOHandler::default();
    io_handler.set_remove_file_on_exit_handler(
        dentist_queue_file_path.clone(),
        Some("\n\nPrograma encerrado.".to_string()),
    )?;

    let pacient_accounts = Database::new(env::var("PACIENT_ACCOUNTS_DATABASE")?);
    let service_sheets_history = Database::new(env::var("SERVICE_SHEETS_HISTORY_DATABASE")?);
    let appointment_schedule = Database::new(env::var("APPOINTMENT_SCHEDULE_DATABASE")?);
    let payment_records = Database::new(env::var("PAYMENT_RECORDS_DATABASE")?);

    let mut manager = ServiceManager::new(
        io_handler,
        pacient_queue_file_path,
        dentist_queue_file_path,
        pacient_accounts,
        service_sheets_history,
        appointment_schedule,
        payment_records,
    );
    manager.start();
}
