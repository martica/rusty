pub enum Expression {
    Int(int),
    Float(float),
    Symbol(~str),
    List(~[Expression])
} 

pub impl Expression {
    fn to_bool(&self) -> bool {
        match *self {
            Int( number ) => 0 != number,
            Float( number ) => 0.0 != number,
            _ => true
        }
    }

    fn to_str(&self) -> ~str {
        match copy *self {
            Int(number) => { fmt!("%d", number) }
            Float(number) => { 
                if number == (number as int) as float {
                    fmt!("%.1f", number)
                } else {
                    fmt!("%f", number)
                }
            }
            Symbol(string) => { copy string }
            List(expressions) => {
                let strings = expressions.map( | &expr | {expr.to_str()} );
                ~"(" + strings.foldl(~"", |&x, &y| { x + ~" " + y } ).trim() + ~")"
            }
        }
    }
}

#[test]
fn test_to_bool_returns_true_for_non_zero_numbers() {
    assert Int(1).to_bool();
    assert Int(-1).to_bool();
    assert Float(1.0).to_bool();
    assert Float(-1.0).to_bool();
}

#[test]
fn test_to_bool_returns_false_for_zero_numbers() {
    assert !Int(0).to_bool();
    assert !Float(0.0).to_bool();
}
