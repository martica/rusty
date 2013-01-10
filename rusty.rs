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

enum Expression {
    Int(int),
    Float(float),
    Symbol(~str),
    List(~[Expression])
}

fn print_expression( expression:Expression ) {
    fn print_expr( expression:Expression ) {
        match expression {
            Int(number) => { io::print( fmt!("%d", number) ) }
            Float(number) => { io::print( fmt!("%f", number) ) }
            Symbol(string) => { io::print( string ) }
            List(expressions) => {
                io::print("(");
                for expressions.init().each |&expression| {
                    print_expr( expression );
                    io::print(", ");
                }
                print_expr( expressions.last() );
                io::print(")");
            }
        }
    }
    print_expr(expression);
    io::println(~"");
}

fn main() {
    let blah:Expression = List(~[Int(1), List(~[Float(1.0), Symbol(~"xyz")])]);
    print_expression(blah);
    io::println( "(begin 1 2)" )
}
