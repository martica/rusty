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
            Int(arg) => {
                if integral {
                    int_acc = int_func(int_acc, arg)
                } else {
                    float_acc = float_func(float_acc, arg as f64)
                }
            }
            Float(arg) => {
                if integral {
                    integral = false;
                    float_acc = float_func(int_acc as f64, arg as f64)
                } else {
                    float_acc = float_func(float_acc, arg as f64)
                }
            }
            _ => fail
        }
    }

    if integral {
        Int(int_acc)
    } else {
        Float(float_acc as float)
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
