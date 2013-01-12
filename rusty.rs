extern mod std;

#[test]
fn test_tokenize() {
    assert tokenize( ~"(1 2 3)" ) == ~[~"(", ~"1", ~"2", ~"3", ~")"];
}

#[test]
fn test_tokenize_empty() {
    assert tokenize( ~"" ) == ~[];
}

#[test]
fn test_tokenize_no_spaces() {
    assert tokenize( ~"one" ) == ~[~"one"];
}

#[test]
fn test_pad_parentheses() {
    assert pad_parentheses( ~"(1 2 3)" ) == ~" ( 1 2 3 ) "
}

#[test]
fn test_pad_parentheses_empty() {
    assert pad_parentheses( ~"" ) == ~""
}

#[test]
fn test_pad_parentheses_one() {
    assert pad_parentheses( ~"(" ) == ~" ( ";
    assert pad_parentheses( ~")" ) == ~" ) ";
}

fn pad_parentheses( input:&str ) -> ~str {
    str::replace(str::replace(input, ")", " ) "), "(", " ( ")
}

fn tokenize( input:&str ) -> ~[~str] {
    str::words(pad_parentheses(input))
}

#[test]
fn test_that_atom_can_read_a_symbol() {
    match atom(~"hello") {
        Symbol(~"hello") => (),
            _ => fail
    }
}

#[test]
fn test_that_atom_can_read_an_int() {
    match atom(~"10") {
        Int(10) => (),
            _ => fail
    }
}

#[test]
fn test_that_atom_can_read_a_float() {
    match atom(~"10.1") {
        Float(10.1) => (),
            _ => fail
    }
}

fn atom( input:~str ) -> Expression {
    match int::from_str(input) {
        Some(number) => Int(number),
        None => match float::from_str(input) {
            Some(number) => Float(number),
            None => Symbol(input)
        }
    }
}

#[test]
fn test_that_read_can_read_an_atom() {
    let atom = read( ~[~"12"] );
    match atom {
        Int(12) => (),
            _ => fail
    }
}

#[test]
fn test_that_read_can_read_a_list() {
    let list = read( ~[ ~"(", ~"1", ~")" ] );
    match list {
        List([Int(1)]) => (),
            _ => fail ~"not a list"
    }
}

#[test]
fn test_that_read_can_read_a_nested_list() {
    let list = read( ~[ ~"(", ~"1", ~"(", ~"2", ~")", ~"3", ~")" ] );
    match list {
        List([Int(1), List([Int(2)]), Int(3)]) => (),
        _ => fail
    }
}

fn parse( program:&str ) -> Expression {
    read( tokenize( program ) )
}

fn read( tokens:~[~str] ) -> Expression {
    fn subexpression( tokens:~[~str] ) -> (Expression, ~[~str]) {
        let mut remainder = copy tokens;
        let token = remainder.remove(0);
        match token {
            ~"(" => {
                let mut accumulator:~[Expression] = ~[];
                while remainder.len() > 0 && remainder[0] != ~")" {
                    let (expr, new_remainder) = subexpression( remainder );
                    accumulator.push(expr);
                    remainder = new_remainder
                }
                // remove the final close paren, this will fail if the parens
                // aren't closed properly due to an assert in remove
                remainder.remove(0);
                (List(accumulator), remainder)
            }
            ~")" => fail,
                _ => (atom(token), remainder)
        }
    }

    let (expression, _remainder) = subexpression( tokens );
    expression
}

enum Expression {
    Int(int),
    Float(float),
    Symbol(~str),
    List(~[Expression])
} 

fn stringify_expression( expression:Expression ) -> ~str {
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
            let strings = expressions.map( | &expr | {stringify_expression(expr)} );
            ~"(" + strings.foldl(~"", |&x, &y| { x + ~" " + y } ).trim() + ~")"
        }
    }
}

#[test]
fn test_eval_returns_number_when_passed_number() {
    let value = eval( Int(1) );
    match value {
        Int(1) => (),
        _ => fail
    }
}

#[test]
fn test_eval_returns_expression_when_passed_quote() {
    let value = eval( List( ~[ Symbol(~"quote"), List( ~[ Symbol(~"a") ] ) ] ));
    io::println( stringify_expression( value ) );
    match value {
        List( [ Symbol( ~"a" ) ] ) => (),
        _ => fail
    }
}

#[test]
fn test_eval_returns_last_expression_when_passed_begin() {
    let expression = parse( ~"(begin 1 2 3)" );
    let value = eval( expression );
    match value {
        Int(3) => (),
        _ => fail
    }
}

#[test]
fn test_if_returns_third_part_when_if_is_true() {
    let expression = parse( ~"(if 1 2 3)" );
    let value = eval( expression );
    match value {
        Int(2) => (),
        _ => fail
    }
}  

#[test]
fn test_if_returns_fourth_part_when_if_is_false() {
    let expression = parse( ~"(if 0 2 3)" );
    let value = eval( expression );
    match value {
        Int(3) => (),
        _ => fail
    }
}  

fn eval( expression:Expression ) -> Expression {
    match copy expression {
        List( expressions ) => {
            match expressions[0] {
                Symbol(~"quote") => {
                    match expressions {
                        [Symbol(~"quote"), expr] => expr,
                        _ => fail
                    }
                }
                Symbol(~"begin") => eval( copy expressions[ expressions.len() - 1] ),
                Symbol(~"if") => {
                    match expressions {
                        [Symbol(~"if"), test, true_expr, false_expr] => {
                            if is_truthy( test ) { true_expr } else { false_expr }
                        }
                        _ => {fail}
                    }
                }
                _ => expression
            }
        }
        _ => {
            expression
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

fn is_truthy( expression:Expression ) -> bool {
    match expression {
        Int( number ) => 0 != number,
        Float( number ) => 0.0 != number,
        _ => true
    }
}

fn main() {
    io::println(stringify_expression( parse( "(1 2 3 (1 2 3))" ) ));
    io::println(stringify_expression( parse( "((1 2) 3 (1 2 3))" ) ));
    let blah:Expression = List(~[Int(1), List(~[Float(1.0), Symbol(~"xyz")])]);
    io::println(stringify_expression(blah));
    io::println( "(begin 1 2)" )
}
