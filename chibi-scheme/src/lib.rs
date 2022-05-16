pub mod serde;
pub mod sexp;

extern crate serde as lib_serde;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_and_execute() {
        let mut ctx = sexp::Context::default();
        ctx.standard_env().unwrap();
        let res = ctx.eval_string("(+ 5 5)").unwrap();
        assert_eq!(res, sexp::SExp::Integer(10.into()));
    }

    #[test]
    fn hello_world() {
        let mut ctx = sexp::Context::default();
        ctx.standard_env().unwrap();
        ctx.standard_ports().unwrap();
        ctx.eval_string("(display \"hello from scheme\")").unwrap();
        let res = ctx.eval_string("(newline)").unwrap();
        assert_eq!(res, sexp::SExp::Null(sexp::NULL));
    }
}
