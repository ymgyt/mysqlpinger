use clap::ArgMatches;
use log::{debug, info};
use mysql::{Conn, Opts, OptsBuilder};
use parse_duration;
use std::{
    borrow::Cow,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

type BoxError = Box<dyn std::error::Error>;

pub struct MySQLPinger {
    opts: Arc<Opts>,
    interval: Duration,
    forever: bool,
    max_retry: u64,
    canceled: AtomicBool,
}

impl MySQLPinger {
    pub fn from_arg(m: &ArgMatches) -> Result<Self, BoxError> {
        let interval = parse_duration::parse(m.value_of("interval").unwrap())?;
        // we need OptsBuilder type first, then calling building methods
        let mut builder = OptsBuilder::default();
        builder
            .ip_or_hostname(m.value_of("host"))
            .tcp_port(
                m.value_of("port")
                    .unwrap()
                    .parse::<u16>()
                    .map_err(|e| format!("invalid port {}", e))?,
            )
            .user(m.value_of("user"))
            .pass(m.value_of("pass"))
            .prefer_socket(false)
            .db_name(m.value_of("dbname"))
            .tcp_connect_timeout(Some(interval));


        Ok(Self {
            opts: Arc::new(builder.into()),
            interval,
            forever: m.is_present("forever"),
            max_retry: m.value_of("max_retry").unwrap().parse()?,
            canceled: AtomicBool::new(false),
        })
    }

    pub fn stop(&self) {
        debug!("stop called");
        self.canceled.store(true, Ordering::SeqCst)
    }

    pub fn ping(&self) -> Result<(), BoxError> {
        info!(
            "ping -> addr:{host}:{port} user:{user} db:{db}",
            host = self.opts.get_ip_or_hostname().unwrap_or(""),
            port = self.opts.get_tcp_port(),
            user = self.opts.get_user().unwrap_or(""),
            db = self.opts.get_db_name().unwrap_or(""),
        );
        debug!("interval:{interval:.1}sec attempt:{attempt}",
            interval = self.interval.as_secs_f64(),
            attempt = self.max_attempt_symbol(),
        );

        let mut attempt = 1;
        let max_attempt = self.max_retry + 1;
        loop {
            if !self.forever && attempt > max_attempt {
                return Err("Max retry count exceeded".into());
            }
            if self.canceled.load(Ordering::SeqCst) {
                return Err("Canceled".into());
            }

            use mysql::DriverError;
            use mysql::Error::*;
            let opts = Arc::clone(&self.opts);
            match Conn::new(Opts::clone(&opts)) {
                Ok(mut conn) => {
                    if conn.ping() {
                        return Ok(());
                    }
                }
                Err(DriverError(DriverError::CouldNotConnect(err))) => {
                    if let Some(err) = err {
                        let (_, description, _) = err;
                        info!("{}/{} {}", attempt, self.max_attempt_symbol(), description);
                    }
                }
                Err(DriverError(DriverError::ConnectTimeout)) => {
                    info!("{}/{} {}", attempt, self.max_attempt_symbol(), "Connection timeout");
                }
                Err(err) => return Err(Box::new(err)),
            }

            thread::sleep(self.interval);
            attempt = attempt.wrapping_add(1);
        }
    }

    fn max_attempt_symbol(&self) -> Cow<'static, str> {
        if self.forever {
            "♾ ".into()
        } else {
            (self.max_retry + 1).to_string().into()
        }
    }
}
