use clap_verbosity_flag::{InfoLevel, Verbosity};
use colog::format::CologStyle;
use log::Level;
use owo_colors::OwoColorize;

pub struct DemiurgeLog;

impl DemiurgeLog {
    pub fn init(verbosity: Verbosity<InfoLevel>) {
        colog::default_builder()
            .filter_level(verbosity.into())
            .format(colog::formatter(CustomPrefixToken))
            .init();
    }
}

pub struct CustomPrefixToken;

impl CologStyle for CustomPrefixToken {
    fn prefix_token(&self, level: &Level) -> String {
        format!(
            "[{}]{}",
            self.level_color(level, self.level_token(level)),
            " ->".blue().bold()
        )
    }

    fn level_token(&self, level: &Level) -> &str {
        match *level {
            Level::Error => "ERROR",
            Level::Warn => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
            Level::Trace => "TRACE",
        }
    }
}
