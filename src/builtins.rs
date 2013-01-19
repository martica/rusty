macro_rules! math_function {
    ($function:ident $identity:expr) => {
        pub fn $function( args:~[Expression], _:@Environment) -> Expression {
            match args.len() {
                0 => $identity,
                1 => vec::foldl($identity, args, |x, y| {x.$function(y)}),
                _ => vec::foldl(args.head(), args.tail(), |x, y| {x.$function(y)})
            }
        }
    }
}

math_function!(add Int(0))
math_function!(sub Int(0))
math_function!(mul Int(1))
math_function!(div Int(1))

pub fn equals( args:~[Expression], _:@Environment) -> Expression {
    for args.tail().each() |&expr| {
        if expr != args.head() {
            return Int(0);
        }
    }
    return Int(1);
}

pub fn builtins() -> ~[(~str,~fn(~[Expression], @Environment) -> Expression)] {
    ~[ (~"+", add), (~"-", sub), (~"*", mul), (~"/", div),
       (~"=", equals)
    ]
}
