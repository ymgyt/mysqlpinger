use clap::{App, AppSettings, Arg};
use colored::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref EXAMPLE: String = format!(
        r#"{}
    # Basic
    mysqlpinger --pass=root --port=30303 <db_name>

    # Docker
    docker run --rm -t --network=<network> ymgyt/mysqlpinger:latest \
       --user=user --pass=secret --host=<container_name> [--forever|--max-retry=20]

    # Slave status check
    mysqlpinger --pass=root --port=30304 --check-slave <db_name>
    "#,
        "Example:".yellow(),
    );
}

pub fn new(version: &str) -> App {
    App::new("mysqlpinger")
        .about("Ping to mysql server")
        .version(version)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("host")
                .long("host")
                .short('h')
                .help("MySQL server hostname")
                .takes_value(true)
                .default_value("127.0.0.1")
                .value_name("HOST")
                .env("MYSQL_HOST"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short('p')
                .help("MySQL server port")
                .takes_value(true)
                .default_value("3306")
                .value_name("PORT")
                .env("MYSQL_PORT"),
        )
        .arg(
            Arg::with_name("user")
                .long("user")
                .short('u')
                .help("User for authentication")
                .takes_value(true)
                .default_value("root")
                .value_name("USER")
                .env("MYSQL_USER"),
        )
        .arg(
            Arg::with_name("pass")
                .long("pass")
                .alias("password")
                .short('P')
                .help("Password for authentication")
                .takes_value(true)
                .env("MYSQL_PASSWORD")
                .value_name("PASS"),
        )
        .arg(
            Arg::with_name("silent")
                .long("silent")
                .short('s')
                .help("Running with no logging"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short('v')
                .help("Verbose")
                .multiple_occurrences(true)
                .conflicts_with("silent"),
        )
        .arg(
            Arg::with_name("max_retry")
                .long("max-retry")
                .short('m')
                .help("Max retry count")
                .takes_value(true)
                .default_value("9")
                .value_name("COUNT"),
        )
        .arg(
            Arg::with_name("forever")
                .long("forever")
                .help("Retry without limit")
                .conflicts_with("max_retry"),
        )
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .short('i')
                .help("Retry ping interval")
                .takes_value(true)
                .default_value("1s")
                .value_name("DURATION"),
        )
        .arg(
            Arg::with_name("check_slave_status")
                .help("check slave threads status")
                .long("check-slave")
                .alias("check-slave-status"),
        )
        .arg(
            Arg::with_name("dbname")
                .help("Database name")
                .index(1)
                .default_value("sys")
                .value_name("DBNAME")
                .env("MYSQL_DB_NAME"),
        )
        .after_help((*EXAMPLE).as_str())
}
