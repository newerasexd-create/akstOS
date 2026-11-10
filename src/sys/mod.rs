pub mod mount;
pub mod power;

use std::process::Command;

/// Запуск процесса с ожиданием
pub fn run_command(cmd: &str, args: &[&str]) -> Result<(), String> {
    match Command::new(cmd).args(args).status() {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(format!("Команда завершилась с кодом: {:?}", status.code()))
            }
        }
        Err(e) => Err(format!("Не удалось выполнить {cmd}: {e}")),
    }
}
