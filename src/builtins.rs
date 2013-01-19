macro_rules! math_function {
    ($function:ident) => {
        pub fn $function( args:~[Expression], _:@Environment) -> Expression {
            vec::foldl(args.head(), args.tail(), |x, y| {x.$function(y)})
        }
    }
}

math_function!(add)
math_function!(sub)
math_function!(mul)
math_function!(div)
        
pub fn equals( args:~[Expression], _:@Environment) -> Expression {
    for args.tail().each() |&expr| {
        if expr != args.head() {
            return Int(0);
        }
    }
    return Int(1);
}
