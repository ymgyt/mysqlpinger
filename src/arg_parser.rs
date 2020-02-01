use clap::{App, AppSettings, Arg};
use colored::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref EXAMPLE: String = format!(
        "{}\n    {}",
        "Example:".yellow(),
        "mysqlpinger --pass=root --port=30303 ",
    );
}

pub fn new(version: &str) -> App {
    App::new("mysqlpinger")
        .about("ping to mysql server")
        .version(version)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("host")
                .long("host")
                .short('h')
                .help("mysql server hostname")
                .takes_value(true)
                .default_value("127.0.0.1")
                .value_name("HOST")
                .env("MYSQL_HOST"),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short('p')
                .help("mysql server port")
                .takes_value(true)
                .default_value("3306")
                .value_name("PORT")
                .env("MYSQL_PORT"),
        )
        .arg(
            Arg::with_name("user")
                .long("user")
                .short('u')
                .help("user for authentication")
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
                .help("password for authentication")
                .takes_value(true)
                .env("MYSQL_PASSWORD")
                .value_name("PASS"),
        )
        .arg(
            Arg::with_name("silent")
                .long("silent")
                .short('s')
                .help("running with no logging"),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short('v')
                .help("verbose")
                .multiple_occurrences(true)
                .conflicts_with("silent"),
        )
        .arg(
            Arg::with_name("max_retry")
                .long("max-retry")
                .short('m')
                .help("max retry count")
                .takes_value(true)
                .default_value("9")
                .value_name("COUNT"),
        )
        .arg(
            Arg::with_name("forever")
                .long("forever")
                .help("retry without limit")
                .conflicts_with("max_retry"),
        )
        .arg(
            Arg::with_name("interval")
                .long("interval")
                .short('i')
                .help("retry ping interval")
                .takes_value(true)
                .default_value("1s")
                .value_name("DURATION"),
        )
        .arg(
            Arg::with_name("dbname")
                .help("database name")
                .index(1)
                .default_value("sys")
                .value_name("DBNAME")
                .env("MYSQL_DB_NAME"),
        )
        .after_help((*EXAMPLE).as_str())
}
