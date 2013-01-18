fn fold_list( args:~[Expression], int_func:fn(int, int) -> int,
              float_func:fn(f64, f64) -> f64 ) -> Expression {
    let mut integral:bool = true;
    let mut int_acc:int = 0;
    let mut float_acc:f64 = 0.0;
    match args.head() {
        Int(first) => int_acc = first,
        Float(first) => { float_acc = first as f64; integral = false }
        _ => fail
    };

    for args.tail().each() |&expr| {
        match expr {
            Int(arg) if integral => int_acc = int_func(int_acc, arg),
            Float(arg) if integral => {
                integral = false;
                float_acc = float_func(int_acc as f64, arg as f64);
            }
            _ => float_acc = float_func(float_acc, expr.to_float() as f64)
        }
    }

    match integral {
        true => Int(int_acc),
        false => Float(float_acc as float)
    }
}

pub fn sum( addends:~[Expression],  _:@Environment) -> Expression {
    fold_list( addends, int::add, float::add )
}

pub fn multiply( factors:~[Expression], _:@Environment) -> Expression {
    fold_list( factors, int::mul, float::mul )
}

pub fn subtract( args:~[Expression], _:@Environment) -> Expression {
    fold_list( args, int::sub, float::sub )
}

pub fn divide( args:~[Expression], _:@Environment) -> Expression {
    fold_list( args, int::div, float::div )
}

pub fn equals( args:~[Expression], _:@Environment) -> Expression {
    for args.tail().each() |&expr| {
        if expr != args.head() {
            return Int(0);
        }
    }
    return Int(1);
}
