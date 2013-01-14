pub enum Expression {
    Int(int),
    Float(float),
    Symbol(~str),
    List(~[Expression])
} 

pub impl Expression {
    fn is_truthy(&self) -> bool {
        match *self {
            Int( number ) => 0 != number,
            Float( number ) => 0.0 != number,
            _ => true
        }
    }
}

pub fn stringify( expression:Expression ) -> ~str {
    match expression {
        Int(number) => { fmt!("%d", number) }
        Float(number) => { 
            if number == (number as int) as float {
                fmt!("%.1f", number)
            } else {
                fmt!("%f", number)
            }
        }
        Symbol(string) => { string }
        List(expressions) => {
            let strings = expressions.map( | &expr | {stringify(expr)} );
            ~"(" + strings.foldl(~"", |&x, &y| { x + ~" " + y } ).trim() + ~")"
        }
    }
}

#[test]
fn test_is_truthy_returns_true_for_non_zero_numbers() {
    assert is_truthy( Int(1) );
    assert is_truthy( Int(-1) );
    assert is_truthy( Float(1.0) );
    assert is_truthy( Float(-1.0) );
}

#[test]
fn test_is_truthy_returns_false_for_zero_numbers() {
    assert !is_truthy( Int(0) );
    assert !is_truthy( Float(0.0) );
}

pub fn is_truthy( expression:Expression ) -> bool {
    match expression {
        Int( number ) => 0 != number,
        Float( number ) => 0.0 != number,
        _ => true
    }
}
