//! It updates the application from version v1.0.0 to v2.0.0.
//!
//! NOTES for `torrust_users` table transfer:
//!
//! - In v2, the table `torrust_user` contains a field `date_registered` non existing in v1.
//!   We changed that columns to allow NULL. We also added the new column `date_imported` with
//!   the datetime when the upgrader was executed.
//!
//! NOTES for `torrust_user_profiles` table transfer:
//!
//! - In v2, the table `torrust_user_profiles` contains two new fields: `bio` and `avatar`.
//!   Empty string is used as default value.

use std::env;
use std::time::SystemTime;

use chrono::prelude::{DateTime, Utc};
use text_colorizer::*;

use crate::upgrades::from_v1_0_0_to_v2_0_0::databases::{current_db, migrate_destiny_database, new_db, reset_destiny_database};
use crate::upgrades::from_v1_0_0_to_v2_0_0::transferrers::category_transferrer::transfer_categories;
use crate::upgrades::from_v1_0_0_to_v2_0_0::transferrers::torrent_transferrer::transfer_torrents;
use crate::upgrades::from_v1_0_0_to_v2_0_0::transferrers::tracker_key_transferrer::transfer_tracker_keys;
use crate::upgrades::from_v1_0_0_to_v2_0_0::transferrers::user_transferrer::transfer_users;

const NUMBER_OF_ARGUMENTS: i64 = 3;

#[derive(Debug)]
pub struct Arguments {
    pub source_database_file: String,  // The source database in version v1.0.0 we want to migrate
    pub destiny_database_file: String, // The new migrated database in version v2.0.0
    pub upload_path: String,           // The relative dir where torrent files are stored
}

fn print_usage() {
    eprintln!(
        "{} - migrates date from version v1.0.0 to v2.0.0.

        cargo run --bin upgrade SOURCE_DB_FILE DESTINY_DB_FILE TORRENT_UPLOAD_DIR

        For example:

        cargo run --bin upgrade ./data.db ./data_v2.db ./uploads

        ",
        "Upgrader".green()
    );
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 3 {
        eprintln!(
            "{} wrong number of arguments: expected {}, got {}",
            "Error".red().bold(),
            NUMBER_OF_ARGUMENTS,
            args.len()
        );
        print_usage();
    }

    Arguments {
        source_database_file: args[0].clone(),
        destiny_database_file: args[1].clone(),
        upload_path: args[2].clone(),
    }
}

pub async fn run_upgrader() {
    let now = datetime_iso_8601();
    upgrade(&parse_args(), &now).await;
}

pub async fn upgrade(args: &Arguments, date_imported: &str) {
    // Get connection to source database (current DB in settings)
    let source_database = current_db(&args.source_database_file).await;

    // Get connection to destiny database
    let dest_database = new_db(&args.destiny_database_file).await;

    println!("Upgrading data from version v1.0.0 to v2.0.0 ...");

    migrate_destiny_database(dest_database.clone()).await;
    reset_destiny_database(dest_database.clone()).await;

    transfer_categories(source_database.clone(), dest_database.clone()).await;
    transfer_users(source_database.clone(), dest_database.clone(), date_imported).await;
    transfer_tracker_keys(source_database.clone(), dest_database.clone()).await;
    transfer_torrents(source_database.clone(), dest_database.clone(), &args.upload_path).await;
}

/// Current datetime in ISO8601 without time zone.
/// For example: 2022-11-10 10:35:15
pub fn datetime_iso_8601() -> String {
    let dt: DateTime<Utc> = SystemTime::now().into();
    format!("{}", dt.format("%Y-%m-%d %H:%M:%S"))
}
