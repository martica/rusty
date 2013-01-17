fn fold_list( args:~[Expression], function:fn(int, int) -> int ) -> Expression {
    let mut acc:int = match args.head() {
        Int(first) => first,
        _ => fail
    };
    for args.tail().each() |&expr| {
        match expr {
            Int(factor) => acc = function(acc, factor),
            _ => fail
        }
    }
    Int(acc)
}

pub fn sum( addends:~[Expression],  _:@Environment) -> Expression {
    fold_list( addends, int::add )
}

pub fn multiply( factors:~[Expression], _:@Environment) -> Expression {
    fold_list( factors, int::mul )
}

pub fn subtract( args:~[Expression], _:@Environment) -> Expression {
    fold_list( args, int::sub )
}
