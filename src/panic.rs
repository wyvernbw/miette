use std::{error::Error, fmt::Display};

use backtrace::Backtrace;

use crate::{Context, Diagnostic, Result};

/// Tells miette to render panics using its rendering engine.
pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(move |info| {
        let mut message = "Something went wrong".to_string();
        let payload = info.payload();
        if let Some(msg) = payload.downcast_ref::<&str>() {
            message = msg.to_string();
        }
        if let Some(msg) = payload.downcast_ref::<String>() {
            message.clone_from(msg);
        }
        let mut report: Result<()> = Err(Panic(message).into());
        if let Some(loc) = info.location() {
            report = report
                .with_context(|| format!("at {}:{}:{}", loc.file(), loc.line(), loc.column()));
        }
        if let Err(err) = report.with_context(|| "Main thread panicked.".to_string()) {
            eprintln!("Error: {:?}", err);
        }
    }));
}

#[derive(Debug)]
struct Panic(String);

impl Display for Panic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = &self.0;
        write!(f, "{msg}")
    }
}

impl Error for Panic {}

impl Diagnostic for Panic {
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(
            "set the `RUST_BACKTRACE=1` environment variable to display a backtrace.",
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn panic() {
        let panic = Panic("ruh roh raggy".to_owned());

        assert_eq!(panic.to_string(), "ruh roh raggy");
        assert!(panic.source().is_none());
        assert!(panic.code().is_none());
        assert!(panic.severity().is_none());
        assert_eq!(
            panic.help().map(|h| h.to_string()),
            Some(
                "set the `RUST_BACKTRACE=1` environment variable to display a backtrace."
                    .to_owned()
            )
        );
        assert!(panic.url().is_none());
        assert!(panic.source_code().is_none());
        assert!(panic.labels().is_none());
        assert!(panic.related().is_none());
        assert!(panic.diagnostic_source().is_none());
    }
}
