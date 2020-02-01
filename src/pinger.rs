use clap::ArgMatches;
use log::info;
use mysql::{Conn, Opts, OptsBuilder};
use parse_duration;
use std::{borrow::Cow, thread, time::Duration};

type BoxError = Box<dyn std::error::Error>;

pub struct MySQLPinger {
    opts: Opts,
    interval: Duration,
    forever: bool,
    max_retry: u64,
}

impl MySQLPinger {
    pub fn from_arg(m: &ArgMatches) -> Result<Self, BoxError> {
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
            .db_name(m.value_of("dbname"));

        let interval = parse_duration::parse(m.value_of("interval").unwrap())?;

        Ok(Self {
            opts: builder.into(),
            interval,
            forever: m.is_present("forever"),
            max_retry: m.value_of("max_retry").unwrap().parse()?,
        })
    }

    pub fn ping(&mut self) -> Result<(), BoxError> {
        info!(
            "ping -> addr:{host}:{port} user:{user} db:{db}",
            host = self.opts.get_ip_or_hostname().unwrap_or(""),
            port = self.opts.get_tcp_port(),
            user = self.opts.get_user().unwrap_or(""),
            db = self.opts.get_db_name().unwrap_or(""),
        );

        let mut attempt = 1;
        let max_attempt = self.max_retry + 1;
        loop {
            if !self.forever && attempt > max_attempt {
                return Err("Max retry count exceeded".into());
            }

            use mysql::DriverError;
            use mysql::Error::*;
            match Conn::new(self.opts.clone()) {
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
