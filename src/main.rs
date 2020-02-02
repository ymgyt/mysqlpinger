use env_logger::fmt::Color;
use log::{error, info, Level, LevelFilter};
use mysqlpinger::MySQLPinger;
use std::{io::Write, sync::Arc, time::Instant};

fn main() {
    let start = Instant::now();
    let version = env!("CARGO_PKG_VERSION");
    let arg = mysqlpinger::arg_parser::new(version).get_matches();

    init_logger(arg.is_present("silent"), arg.occurrences_of("verbose"));

    let pinger = match MySQLPinger::from_arg(&arg) {
        Ok(pinger) => Arc::new(pinger),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let pinger2 = Arc::clone(&pinger);
    ctrlc::set_handler(move || {
        // when user put Ctrl+C
        // NG: ^CINFO ...
        // OK: ^C
        //     INFO ...
        println!("");
        info!("Handle Ctrl+C [SIGINT] stopping pinger...");
        pinger2.stop();

        // force quit
        std::thread::sleep(std::time::Duration::from_secs(1));
        std::process::exit(2);
    })
    .expect("Error setting Ctrl-C handler");

    match pinger.ping() {
        Ok(_) => info!("OK (elapsed {:.3}sec)", start.elapsed().as_secs_f64()),
        Err(err) => {
            error!("{} (elapsed {:.3}sec)", err, start.elapsed().as_secs_f64());
            std::process::exit(1);
        }
    };
}

fn init_logger(silent: bool, verbose: u64) {
    let mut builder = env_logger::Builder::new();

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
