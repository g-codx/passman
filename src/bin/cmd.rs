use dialoguer::{Input, Password, Select};
use passman::entry::Entry;
use secrecy::SecretString;
use std::path::Path;
use uuid::Uuid;

//cargo run --bin passman_cmd --release --features cli
fn main() {
    let (master, salt) = if Path::new(passman::FILE).exists() {
        let master = Password::new()
            .with_prompt("Enter master password")
            .interact()
            .expect("Failed to read master password");

        let salt = passman::storage::try_get_salt().expect("Failed to read salt");

        (master, salt)
    } else {
        let master = Password::new()
            .with_prompt("Create master password")
            .with_confirmation("Confirm master password", "Passwords don't match")
            .interact()
            .expect("Failed to read master password");

        let salt = passman::storage::generate_salt();
        (master, salt)
    };

    let key = passman::crypto::derive_key(SecretString::new(master.into_boxed_str()), &salt)
        .expect("Failed to derive key");
    let mut entries = passman::storage::load_entries(&key).expect("Failed to read entries");

    loop {
        let choices = &[
            "Add password",
            "List passwords",
            "Remove password",
            "Generate password",
            "Exit",
        ];

        let selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(choices)
            .interact()
            .expect("Failed to read selection");

        match selection {
            0 => {
                add_password(&mut entries, &key, &salt);
            }
            1 => {
                for e in &entries {
                    println!("{}", e);
                }
            }
            2 => {
                remove_password(&mut entries, &key, &salt);
            }
            3 => {
                generate_password();
            }
            4 => {
                println!("Exit!");
                break;
            }
            _ => println!("Invalid selection"),
        }
    }
}

fn generate_password() {
    let choices = &[
        "12 characters",
        "16 characters",
        "20 characters",
        "24 characters",
    ];
    let selection = Select::new()
        .with_prompt("Choose password length")
        .items(choices)
        .default(1)
        .interact()
        .expect("Failed to read selection");

    let length = match selection {
        0 => 12,
        1 => 16,
        2 => 20,
        3 => 24,
        _ => 16,
    };

    let password = passman::crypto::generate_password(length);

    println!("Generated password: {}", password);
}

fn remove_password(entries: &mut Vec<Entry>, key: &[u8; 32], salt: &[u8; 16]) {
    let uuid = Input::<String>::new()
        .with_prompt("Uuid")
        .interact()
        .expect("Failed to read uuid name");

    entries.retain(|e| e.uuid != uuid);
    passman::storage::save_entries(entries, key, salt).expect("Failed to save entries");

    println!("Password removed successfully!");
}

fn add_password(entries: &mut Vec<Entry>, key: &[u8; 32], salt: &[u8; 16]) {
    let service = Input::<String>::new()
        .with_prompt("Service name")
        .interact()
        .expect("Failed to read service name");

    let username = Input::<String>::new()
        .with_prompt("Username")
        .interact()
        .expect("Failed to read username");

    let password = Password::new()
        .with_prompt("Password")
        .interact()
        .expect("Failed to read password");

    let notes = Input::<String>::new()
        .with_prompt("Notes (optional)")
        .allow_empty(true)
        .interact()
        .expect("Failed to read notes");

    let entry = Entry {
        uuid: Uuid::new_v4().to_string(),
        service,
        username,
        password,
        notes: if notes.is_empty() { None } else { Some(notes) },
    };

    entries.push(entry);
    passman::storage::save_entries(entries, key, salt).expect("Failed to save entries");

    println!("Password added successfully!");
}
