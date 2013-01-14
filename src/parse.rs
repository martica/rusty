
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
fn test_that_a_plus_sign_becomes_a_symbol() {
    match atom(~"+") {
        Symbol(~"+") => (),
        _ => fail fmt!("+ became: %s", atom(~"+").to_str())
    }
}

#[test]
fn test_that_a_minus_sign_becomes_a_symbol() {
    match atom(~"-") {
        Symbol(~"-") => (),
        _ => fail fmt!("- became: %s", atom(~"-").to_str())
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
    match input {
        ~"+" => Symbol( ~"+" ),
        ~"-" => Symbol( ~"-" ),
        _ => match int::from_str(input) {
            Some(number) => Int(number),
            None => match float::from_str(input) {
                Some(number) => Float(number),
                None => Symbol(input)
            }
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

pub fn parse( program:&str ) -> Expression {
    read( tokenize( program ) )
}
