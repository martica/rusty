pub enum Expression {
    Bool(bool),
    Int(int),
    Float(float),
    Symbol(~str),
    List(~[Expression]),
    Proc(~fn(~[Expression]) -> Expression, (uint,uint)),
    Lambda(@Expression,~[Expression],@Environment),
    Error(~str)
} 

macro_rules! operator_overload {
    ($operator_name:ident $function_name:ident) => (
        pub impl Expression: ops::$operator_name<Expression,Expression> {
            pure fn $function_name(&self, other:&Expression) -> Expression {
                match (copy *self, copy *other) {
                    (Int(x), Int(y)) => Int(x.$function_name(&y)),
                    (Error(x), _) => Error(copy x),
                    (_, Error(x)) => Error(copy x),
                    _ => {
                        if !self.is_number() {
                            Error(fmt!("%s was given where a number was expected", self.to_str()))
                        } else if !other.is_number() {
                            Error(fmt!("%s was given where a number was expected", other.to_str()))
                        } else {
                            Float(self.to_float().$function_name(&other.to_float()))
                        }
                    }
                }
            }
        }
    )
}

operator_overload!(Add add)
operator_overload!(Sub sub)
operator_overload!(Mul mul)
operator_overload!(Div div)

pub impl Expression: cmp::Ord {
    pure fn lt(&self, other: &Expression) -> bool {
        match (*self, *other) {
            (Int(x), Int(y)) => x.lt(&y),
            _ => self.to_float().lt(&other.to_float())
        }
    }
    pure fn le(&self, other: &Expression) -> bool {
        match (*self, *other) {
            (Int(x), Int(y)) => x.le(&y),
            _ => self.to_float().le(&other.to_float())
        }
    }
    pure fn gt(&self, other: &Expression) -> bool {
        match (*self, *other) {
            (Int(x), Int(y)) => x.gt(&y),
            _ => self.to_float().gt(&other.to_float())
        }
    }
    pure fn ge(&self, other: &Expression) -> bool {
        match (*self, *other) {
            (Int(x), Int(y)) => x.ge(&y),
            _ => self.to_float().ge(&other.to_float())
        }
    }
}

pub impl Expression {
    static fn new_proc( function:~fn(~[Expression]) -> Expression) -> Expression {
        let ptr:(uint,uint) = unsafe {
            cast::reinterpret_cast(&function)
        };
        Proc( function, ptr )
    }

    pure fn is_error(&self) -> bool {
        match *self {
            Error(_) => true,
            _ => false
        }
    }

    pure fn is_number(&self) -> bool {
        match *self {
            Int(_) | Float(_) => true,
            _ => false
        }
    }

    pure fn to_float(&self) -> float {
        match *self {
            Int( number ) => number as float,
            Float( number ) => number,
            _ => float::NaN
        }
    }

    fn to_bool(&self) -> bool {
        match *self {
            Bool( value ) => value, 
            Int( number ) => 0 != number,
            Float( number ) => 0.0 != number,
            _ => true
        }
    }

    pure fn to_str(&self) -> ~str {
        match copy *self {
            Bool(value) => if value { ~"#t" } else { ~"#f" },
            Int(number) => { fmt!("%d", number) }
            Float(number) => { 
                if number == (number as int) as float {
                    fmt!("%.1f", number)
                } else {
                    fmt!("%f", number)
                }
            }
            Symbol(string) => { copy string }
            Error(string) => { fmt!("Error: %s", string) }
            List(expressions) => {
                let strings = expressions.map( | &expr | {expr.to_str()} );
                ~"(" + strings.foldl(~"", |&x, &y| { x + ~" " + y } ).trim() + ~")"
            }
            Proc(_,x) => { fmt!("procedure: %s", x.to_str()) }
            Lambda(_,_,_) => {~"Lambda"}
        }
    }
}

impl Expression : cmp::Eq {
    pure fn eq(&self, other:&Expression) -> bool {
        match copy *self {
            Bool(x) => match *other { Bool(y) => x ==y, _ => false },
            Int(x) => match *other { Int(y) => x == y, _ => false },
            Float(x) => match *other { Float(y) => x == y, _ => false },
            Symbol(x) => match copy *other { Symbol(y) => x == y, _ => false },
            List(x) => match copy *other { List(y) => x == y, _ => false },
            Proc(_,x) => match copy *other { Proc(_,y) => x == y, _=> false },
            Lambda(a,b,c) => { 
                match copy *other { 
                    Lambda(x,y,z) => unsafe {
                        ptr::ref_eq(a,x) &&
                        b == y &&
                        ptr::ref_eq(c,z)
                    },
                    _ => false
                }
            }
            Error(_) => false
        }
    }

    pure fn ne(&self, other:&Expression) -> bool {
        return !(*self).eq(other);
    }
}

#[test]
fn test_that_adding_non_numbers_gives_an_error() {
    assert (Int(1) + Symbol(~"a")).is_error()
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

#[test]
fn test_that_integers_are_comparable() {
    assert Int(1) == Int(1);
    assert Int(1) != Int(2);
}

#[test]
fn test_that_floats_are_comparable() {
    assert Float(1.0) == Float(1.0);
    assert Float(1.0) != Float(2.0);
}

#[test]
fn test_that_symbols_are_comparable() {
    assert Symbol(~"1") == Symbol(~"1");
    assert Symbol(~"1") != Symbol(~"2");
}

#[test]
fn test_that_vectors_are_comparable() {
    assert List(~[Int(1)]) == List(~[Int(1)]);
    assert List(~[Int(1)]) != List(~[Int(2)]);
}
