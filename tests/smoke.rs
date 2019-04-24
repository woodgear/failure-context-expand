use failure::Error;
use std::net::Ipv4Addr;

#[failure_context_expand::fce]
fn a() -> Result<(), Error> {
    b()?;
    Ok(())
}

#[failure_context_expand::fce]
fn b() -> Result<(), Error> {
    c()?;
    Ok(())
}

#[failure_context_expand::fce]
fn c() -> Result<Ipv4Addr, Error> {
    let res = "127.0.0.x".parse()?;
    Ok(res)
}

pub trait FailureExt {
    fn pretty_log(&self) -> String;
}

impl FailureExt for failure::Error {
    fn pretty_log(&self) -> String {
        let mut err_chain = self.iter_chain();
        let first_err = err_chain.next();
        if let Some(first_err) = first_err {
            let mut err_msg = format!("{}", first_err);
            for c in err_chain {
                err_msg = format!("{}\ncause by: {}", err_msg, c);
            }
            return err_msg;
        }
        return "without any cause".to_string();
    }
}

#[test]
fn works() {
    let e = a().err().unwrap();
    assert_eq!(
        e.pretty_log(),
        r#"call a() err
cause by: call b() err
cause by: call c() err
cause by: invalid IP address syntax"#
    )
}
