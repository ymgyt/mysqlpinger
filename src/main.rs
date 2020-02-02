use log::{error, info};
use mysqlpinger::MySQLPinger;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let version = env!("CARGO_PKG_VERSION");
    let arg = mysqlpinger::arg_parser::new(version).get_matches();

    init_signal_handler();
    init_logger(arg.is_present("silent"), arg.occurrences_of("verbose"));

    let mut pinger = match MySQLPinger::from_arg(&arg) {
        Ok(pinger) => pinger,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    match pinger.ping() {
        Ok(_) => info!("OK (elapsed {:.3}sec)", start.elapsed().as_secs_f64()),
        Err(err) => {
            error!("{} (elapsed {:.3}sec)", err, start.elapsed().as_secs_f64());
            std::process::exit(1);
        }
    }
}

fn init_signal_handler() {
    ctrlc::set_handler(move || {
        // when user put Ctrl+C
        // NG: ^CINFO ...
        // OK: ^C
        //     INFO ...
        println!("");
        info!("Handle Ctrl+C[SIGINT]");
        std::process::exit(2);
    }).expect("Error setting Ctrl-C handler");
}

fn init_logger(silent: bool, verbose: u64) {
    let mut builder = env_logger::Builder::new();

    use env_logger::fmt::Color;
    use log::{Level, LevelFilter};
    use std::io::Write;
    builder.format(|buf, record| {
        let level_color = match record.level() {
            Level::Trace => Color::White,
            Level::Debug => Color::Blue,
            Level::Info => Color::Green,
            Level::Warn => Color::Yellow,
            Level::Error => Color::Red,
        };
        let mut level_style = buf.style();
        level_style.set_color(level_color);

        writeln!(
            buf,
            "{level} {args}",
            level = level_style.value(record.level()),
            args = record.args(),
        )
    });

    builder.filter(
        None,
        match (silent, verbose) {
            (true, _) => LevelFilter::Error,
            (_, 0) => LevelFilter::Info,
            (_, 1) => LevelFilter::Debug,
            (_, _) => LevelFilter::Trace,
        },
    );
    builder.write_style(env_logger::WriteStyle::Auto);

    builder.init();
}
