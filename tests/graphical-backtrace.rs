#![cfg(feature = "fancy")]
#![cfg(feature = "backtrace")]

use miette::{LabeledSpan, Report};

#[test]
fn test_report_backtrace() {
    fn operation() -> Result<(), Report> {
        fn inner_operation() -> Result<(), Report> {
            let rand = 42;
            if rand > 60 {
                Ok(())
            } else {
                Err(miette::miette!(
                    labels = vec![LabeledSpan::at_offset(0, "here")],
                    "oops failed"
                )
                .with_source_code("2+2"))
            }
        }
        inner_operation()?;
        Ok(())
    }

    let report = operation().unwrap_err();
    let result = format!("{report:?}");

    println!("{result}");
}
