use crate::core::entry::Entry;
use crate::core::{FILE, crypto, error};
use aes_gcm::aead::OsRng;
use aes_gcm::aead::rand_core::RngCore;
use std::fs;
use std::path::Path;

const SALT_LEN: usize = 16;

/// Загружает записи из зашифрованного файла хранилища.
///
/// # Аргументы
///
/// * `key` - ключ шифрования в виде байтового среза для дешифровки данных
///
/// # Возвращаемое значение
///
/// * `Ok(Vec<Entry>)` - вектор записей при успешной загрузке
/// * `Err(error::Error)` - ошибка если возникли проблемы с чтением, дешифровкой или парсингом
pub fn load_entries(key: &[u8]) -> error::Result<Vec<Entry>> {
    if !Path::new(FILE).exists() {
        return Ok(vec![]);
    }
    let data = fs::read(FILE)
        .map_err(|e| error::Error::Storage(format!("Failed to read storage; {:?}", e)))?;
    let encrypted = &data[SALT_LEN..];
    let decrypted = crypto::decrypt_aes256gcm(key, encrypted)?;
    serde_json::from_slice(&decrypted)
        .map_err(|e| error::Error::Storage(format!("Failed to read storage; {:?}", e)))
}

/// Сохраняет список записей в зашифрованном файле.
///
/// # Аргументы
///
/// * `entries` - Список записей для сохранения
/// * `key` - Ключ шифрования для алгоритма AES-256-GCM
/// * `salt` - Соль, которая добавляется к зашифрованным данным
///
/// # Возвращаемое значение
///
/// Возвращает `Ok(())` при успешном сохранении или `Err(error::Error)` при ошибке.
pub fn save_entries(entries: &[Entry], key: &[u8], salt: &[u8]) -> error::Result<()> {
    let plaintext = serde_json::to_vec(entries)
        .map_err(|e| error::Error::Storage(format!("Failed to save storage; {:?}", e)))?;
    let mut ciphertext = crypto::encrypt_aes256gcm(key, &plaintext)?;
    let mut result = salt.to_vec();
    result.append(&mut ciphertext);
    fs::write(FILE, result)
        .map_err(|e| error::Error::Storage(format!("Failed to save storage; {:?}", e)))
}

/// Генерирует криптографически безопасную соль для шифрования.
///
/// # Возвращаемое значение
///
/// Возвращает массив из 16 байт, заполненный криптографически случайными значениями.
///
/// # Использование
///
/// Соль используется для:
/// - Добавления энтропии к процессу шифрования
/// - Защиты от атак по словарю и радужных таблиц
/// - Обеспечения уникальности зашифрованных данных даже при одинаковом ключе
///
/// # Безопасность
///
/// - Использует криптографически безопасный генератор случайных чисел (`OsRng`)
/// - Размер соли в 16 байт (128 бит) обеспечивает достаточную энтропию
/// - Каждое значение соли уникально с высокой вероятностью
///
/// # Примечания
///
/// - Соль не является секретной и может храниться вместе с зашифрованными данными
/// - При каждом вызове генерируется новая случайная соль
/// - Для верификации данных должна использоваться та же соль, что и при шифровании
pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Читает соль из файла хранилища.
///
/// # Возвращаемое значение
///
/// * `Ok([u8; 16])` - массив из 16 байт, содержащий соль
/// * `Err(error::Error)` - ошибка если файл не существует или невозможно прочитать соль
///
/// # Безопасность
///
/// - Соль читается в неизменном виде из файла
/// - Не производится проверка валидности данных соли
/// - Функция может паниковать если файл содержит меньше 16 байт (из-за `copy_from_slice`)
pub fn try_get_salt() -> error::Result<[u8; 16]> {
    let mut salt = [0u8; 16];
    let data = fs::read(FILE)
        .map_err(|e| error::Error::Storage(format!("Failed to read salt; {:?}", e)))?;
    salt.copy_from_slice(&data[..16]);

    Ok(salt)
}
