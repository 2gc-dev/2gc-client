use keyring::Entry;
use std::collections::HashMap;
use tokio::io;

/// Название приложения в хранилище учётных данных
const APP_PREFIX: &str = "2GC";

/// Создаёт безопасную запись в системном хранилище
fn save_password(section: &str, key: &str, value: &str) -> Result<(), keyring::Error> {
    let service = format!("{}-{}", APP_PREFIX, section);
    let entry = Entry::new(&service, key)?;
    entry.set_password(value)
}

/// Загружает пароль
fn load_password(section: &str, key: &str) -> Option<String> {
    let service = format!("{}-{}", APP_PREFIX, section);
    let entry = Entry::new(&service, key).ok()?;
    entry.get_password().ok()
}

/// Удаляет пароль
fn delete_password(section: &str, key: &str) -> Option<()> {
    let service = format!("{}-{}", APP_PREFIX, section);
    let entry = Entry::new(&service, key).ok()?;
    entry.delete_password().ok()
}

/// Асинхронное чтение значения
pub async fn read_value_from_ini(section: &str, key: &str) -> Option<String> {
    load_password(section, key)
}

/// Асинхронная запись значения
pub async fn write_key_value_to_ini(section: &str, key: &str, value: &str) -> io::Result<()> {
    save_password(section, key, value)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Keyring error: {}", e)))
}

/// Асинхронное удаление значения
pub async fn delete_key_from_ini(section: &str, key: &str) -> io::Result<()> {
    delete_password(section, key);
    Ok(())
}

/// Заглушка для чтения всех значений — в `keyring` нельзя это сделать без индекс-файла
pub async fn read_all_values_from_section(_section: &str) -> HashMap<String, String> {
    HashMap::new()
}

/// Заглушка на удаление целой секции (можно сделать через индекс, если нужно)
pub async fn delete_section_from_ini(_section: &str) -> io::Result<()> {
    Ok(())
}

/// Очистка не используется при keyring
pub async fn clear_file() -> io::Result<()> {
    Ok(())
}


