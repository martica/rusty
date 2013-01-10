extern mod std;

#[test]
fn test_tokenize() {
    assert tokenize( ~"(1 2 3)" ) == ~[~"(", ~"1", ~"2", ~"3", ~")"];
}

#[test]
fn test_addspaces() {
    assert addspaces( ~"(1 2 3)" ) == ~" ( 1 2 3 ) "
}

fn addspaces( input:&str ) -> ~str {
    str::replace(str::replace(input, ")", " ) "), "(", " ( ")
}

fn tokenize( input:&str ) -> ~[~str] {
    str::split_str_nonempty(addspaces(input), " ")
}

enum Expression {
    Value(int),
    Symbol(~str),
    List(~[Expression])
    }

fn main() {
    let Blah:Expression = List(~[Value(1), List(~[Value(1), Symbol(~"xyz")])]);
    io::println( "(begin 1 2)" )
}
