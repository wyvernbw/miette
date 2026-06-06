#![cfg(feature = "fancy")]

use miette::Report;

#[test]
fn test_report_backtrace() {
    fn operation() -> Result<(), Report> {
        fn inner_operation() -> Result<(), Report> {
            let rand = 42;
            if rand > 60 {
                Ok(())
            } else {
                Err(miette::miette!("oops failed"))
            }
        }
        inner_operation()?;
        Ok(())
    }

    let report = operation().unwrap_err();
    let result = format!("{report:?}");

    println!("{result}");
}
