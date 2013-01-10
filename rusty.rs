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
