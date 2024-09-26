use serde::Deserialize;

use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

pub use tracing::{debug, error, info, trace, warn};

#[derive(Deserialize, Debug, Clone)]
pub struct LogConfig {
    pub log_level: LogLevel,
    pub log_format: LogFormat,
    pub filtering_directive: Option<String>,
}

impl LogConfig {
    pub fn default() -> Self{
        Self {
            log_level: LogLevel::Warn,
            log_format: LogFormat::Console,
            filtering_directive: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    // Debug,
    // Info,
    Warn,
    Error,
    // Off,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    Console,
    Json,
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            // LogLevel::Debug => Self::DEBUG,
            // LogLevel::Info => Self::INFO,
            LogLevel::Warn => Self::WARN,
            LogLevel::Error => Self::ERROR,
            // LogLevel::Off => Self::OFF,
        }
    }
}

pub struct Guards {
    _log_guard: LogGuard,
}

pub(super) struct LogGuard {
    _log_guard: WorkerGuard,
}

pub fn setup(log_config: &LogConfig) -> Guards {
    let log_guard = setup_logging_pipeline(log_config);

    Guards {
        _log_guard: log_guard,
    }
}

pub(super) fn setup_logging_pipeline(
    log_config: &LogConfig,
) -> LogGuard {
    let subscriber = tracing_subscriber::registry();

    let console_filter = get_envfilter(
        LogLevel::Warn,
    );

    let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());

    match log_config.log_format {
        LogFormat::Console => {
            let logging_layer = fmt::layer()
                .with_timer(fmt::time::time())
                .pretty()
                .with_writer(non_blocking)
                .with_filter(console_filter);

            subscriber.with(logging_layer).init();
        }
        LogFormat::Json => {
            let logging_layer = fmt::layer()
                .json()
                .with_timer(fmt::time::time())
                .with_writer(non_blocking)
                .with_filter(console_filter);

            subscriber.with(logging_layer).init();
        }
    }

    LogGuard { _log_guard: guard }
}

fn get_envfilter<'a>(
    default_log_level: impl Into<LevelFilter> + Copy,
) -> EnvFilter {
    #[allow(clippy::expect_used)]
    EnvFilter::default()
        .add_directive(default_log_level.into().into())
}
