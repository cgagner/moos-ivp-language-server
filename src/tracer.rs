use std::default;
use std::fmt::Display;

use clap::ValueEnum;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter, Registry};

#[derive(Default, Debug, Clone, ValueEnum)]
pub enum Level {
    error,
    warn,
    #[default]
    info,
    debug,
    trace,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Tracer {}

impl Tracer {
    pub fn init(level: Level) -> anyhow::Result<()> {
        let mils_level = format!("moos_ivp_language_server={}", level);
        let parser_level = format!("moos_parser={}", level);
        let level_filter = match level {
            Level::error => LevelFilter::ERROR,
            Level::warn => LevelFilter::WARN,
            Level::info => LevelFilter::INFO,
            Level::debug => LevelFilter::DEBUG,
            Level::trace => LevelFilter::TRACE,
        };
        let filter = EnvFilter::builder()
            .with_default_directive(level_filter.into())
            .from_env()?
            .add_directive(mils_level.parse()?)
            .add_directive(parser_level.parse()?);
        // TODO: This should eventually allow logging to a file if an
        // environment variable is set.
        let writer = BoxMakeWriter::new(std::io::stderr);
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_writer(writer)
            .with_ansi(false)
            .with_filter(filter);

        Registry::default().with(fmt_layer).try_init()?;

        Ok(())
    }
}
