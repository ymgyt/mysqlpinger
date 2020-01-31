use clap::{App, Arg};
use mysql::{OptsBuilder, Conn};

fn main() {
    let version = env!("CARGO_PKG_VERSION");

    let app =  App::new("mysqlpinger")
        .about("ping to mysql server")
        .version(version)
        .arg(
           Arg::with_name("host")
               .long("host")
               .short("h")
               .takes_value(true)
               .default_value("127.0.0.1")
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .short("p")
                .takes_value(true)
                .default_value("3306")
        )
        .arg(
            Arg::with_name("user")
                .long("user")
                .short("u")
                .takes_value(true)
                .default_value("root")
        )
        .arg(
           Arg::with_name("pass")
               .long("pass")
               .alias("password")
               .short("-P")
               .takes_value(true)
               .env("MYSQL_PASSWORD")
        )
        .arg(
            Arg::with_name("dbname")
                .index(1)
                .required(true)
        );

    let m = app.get_matches();

    // we need OptsBuilder type first, then calling building methods
    let mut opts = OptsBuilder::default();
    opts
        .ip_or_hostname(m.value_of("host"))
        .tcp_port(m.value_of("port").unwrap().parse::<u16>().unwrap())
        .user(m.value_of("user"))
        .pass(m.value_of("pass"))
        .prefer_socket(false)
        .db_name(m.value_of("dbname"));

    let mut conn = Conn::new(opts).unwrap();
    if conn.ping() {
        println!("ping success!");
    } else {
        println!("ping failed");
    }
}

