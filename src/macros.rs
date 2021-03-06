macro_rules! patter {
    ($code:expr) => {{
        let tokens = parse::lex($code);
        parse::parse(&tokens)
    }};
}

macro_rules! patter_std {
    ($code:expr) => {
        patter!($code).eval(&mut crate::STD_CXT.clone())
    };
}

macro_rules! ident {
    ($ident:expr) => {{
        let tokens = parse::lex($ident);
        crate::IDENTS.intern(parse::parse_ident(&tokens))
    }};
}

macro_rules! get {
    ($ident:expr, $cxt:expr) => {
        $cxt.clone()
            .lookup(ident!($ident))
            .expect(&format!("Ident not found: {:?}", $ident))
    };
}

macro_rules! primitive {
    ($name:expr, $ptn:expr, $impl:expr, $evals_to:expr, $cxt:ident) => {
        &Bindings::of(
            ident!($name),
            &SExpr::Fun(crate::Fun {
                body: Box::new(SExpr::Operation {
                    eval: |#[allow(unused_mut)] mut $cxt: &mut Context| {
                        #[allow(unused_imports)]
                        use crate::SExpr::*;
                        Ok($impl)
                    },
                    evals_to:
                        |#[allow(unused_variables)] $cxt: &dyn Fn(
                            Interned<'static, Ident>,
                        )
                            -> Option<
                            SExpr,
                        >| {
                            #[allow(unused_imports)]
                            use crate::SExpr::*;
                            $evals_to
                        },
                }),
                args_ptn: Box::new(
                    patter!($ptn).eval(&mut Context::basic()).unwrap(),
                ),
                closure: Box::new(Context::basic().collapse()),
            }),
        )
    };
}

macro_rules! interpreter_err {
    ($err:ident) => {
        crate::error::InterpreterError {
            info: crate::error::InterpreterErrorInfo::$err,
            callstack: Vec::new(),
        }
    };
    ($err:ident, $info:expr) => {
        crate::error::InterpreterError {
            info: crate::error::InterpreterErrorInfo::$err($info),
            callstack: Vec::new(),
        }
    };
    ($err:ident, $info_1:expr, $info_2:expr) => {
        crate::error::InterpreterError {
            info: crate::error::InterpreterErrorInfo::$err($info_1, $info_2),
            callstack: Vec::new(),
        }
    };
}

macro_rules! throw_interpreter_err {
    ($err:ident) => {
        Err(interpreter_err!($err))?
    };
    ($err:ident, $info:expr) => {
        Err(interpreter_err!($err, $info))?
    };
    ($err:ident, $info_1:expr, $info_2:expr) => {
        Err(interpreter_err!($err, $info_1, $info_2))?
    };
}

macro_rules! patter_sr {
    ($fun:expr, $args:expr, $cxt:expr) => {{
        $fun.call($args.as_list().unwrap(), $cxt)
    }};
    ($fun:expr, $args:expr) => {
        patter_sr!($fun, $args, &mut Context::empty())
    };
}

macro_rules! number {
    ($n:expr) => {
        crate::SExpr::Number(crate::number::Number::from($n))
    };
}
