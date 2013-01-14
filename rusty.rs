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
    let value = eval( Int(1), @Environment::new_global_environment() );
    match value {
        Int(1) => (),
        _ => fail
    }
}

#[test]
fn test_eval_returns_expression_when_passed_quote() {
    let value = eval( List( ~[ Symbol(~"quote"), List( ~[ Symbol(~"a") ] ) ] ), 
    @Environment::new_global_environment() );
    io::println( stringify_expression( value ) );
    match value {
        List( [ Symbol( ~"a" ) ] ) => (),
        _ => fail
    }
}

#[test]
fn test_eval_returns_last_expression_when_passed_begin() {
    let expression = parse( ~"(begin 1 2 3)" );
    let value = eval( expression, @Environment::new_global_environment() );
    match value {
        Int(3) => (),
        _ => fail
    }
}

#[test]
fn test_if_returns_third_part_when_if_is_true() {
    let expression = parse( ~"(if 1 2 3)" );
    let value = eval( expression, @Environment::new_global_environment() );
    match value {
        Int(2) => (),
        _ => fail
    }
}  

#[test]
fn test_if_returns_fourth_part_when_if_is_false() {
    let expression = parse( ~"(if 0 2 3)" );
    let value = eval( expression, @Environment::new_global_environment() );
    match value {
        Int(3) => (),
        _ => fail
    }
}  

#[test]
fn test_that_if_evaluates_the_then_branch() {
    let expression = parse( ~"(if 1 (begin 1 2) 7)" );
    let value = eval( expression, @Environment::new_global_environment() );
    match value {
        Int(2) => (),
        List( [Symbol(~"begin"), Int(1), Int(2)]) => fail ~"If just returned the then branch",
        _ => fail fmt!("If returned something unusual (%s)", stringify_expression(value))
    }
}

#[test]
fn test_that_if_evaluates_the_else_branch() {
    let expression = parse( ~"(if 0 7 (begin 1 2))" );
    let value = eval( expression, @Environment::new_global_environment() );
    match value {
        Int(2) => (),
        List( [Symbol(~"begin"), Int(1), Int(2)]) => fail ~"If just returned the else branch",
        _ => fail fmt!("If returned something unusual (%s)", stringify_expression(value))
    }
}

#[test]
fn test_that_if_evaluates_the_test() {
    let expression = parse( ~"(if (begin 1 0) 1 2)" );
    let value = eval( expression, @Environment::new_global_environment() );
    match value {
        Int(1) => fail ~"If didn't evaluate the test",
        Int(2) => (),
        _ => fail fmt!("If returned something unusual (%s)", stringify_expression(value))
    }
}

#[test]
fn test_that_bare_symbol_is_interpreted_as_variable() {
    let env = @Environment::new_global_environment();
    env.define(~"monkey", Int(10));
    let expression = parse( ~"monkey" );
    let value = eval( expression, env );
    match value {
        Int(10) => (),
        _ => fail fmt!("Expected 10 got %s", stringify_expression(value))
    }
}

#[test]
#[should_fail]
fn test_that_undefined_symbol_is_an_error() {
    let env = @Environment::new_global_environment();
    let expression = parse( ~"monkey" );
    eval( expression, env );
}

#[test]
fn test_that_define_can_add_a_variable() {
    let env = @Environment::new_global_environment();
    let expression = parse( ~"(define x 10)" );
    let value = eval( expression, env );
    match env.lookup(~"x") {
        Some(Int(10)) => (),
        _ => fail fmt!("Expected 10 got %s", stringify_expression(value))
    }
}

fn eval( expression:Expression, environment:@Environment ) -> Expression {
    fn quote(expressions:~[Expression]) -> Expression {
        match expressions {
            [_, expr] => expr,
            _ => fail ~"Syntax Error: quote must take a single argument"
        }
    }

    fn if_(expressions:~[Expression], environment:@Environment) -> Expression {
        match expressions {
            [_, test, true_expr, false_expr] => {
                eval(if is_truthy( eval(test, environment) ) { true_expr } else { false_expr }, environment)
            }
            _ => fail ~"Syntax Error: if must take three arguments"
        }
    }

    fn begin(expressions:~[Expression], environment:@Environment) -> Expression {
        eval( copy expressions[ expressions.len() - 1], environment )
    fn reset_variable(expressions:~[Expression], environment:@Environment, function:~str) -> Expression {
        match expressions {
            [_, symbol, value] => {
                match copy symbol {
                    Symbol( key ) => {
                        environment.define(key, eval(value, environment));
                        symbol
                    }
                    _ => fail fmt!("Syntax Error: %s takes a symbol as its first argument", function)
                }
            }
            _ => fail fmt!("Syntax Error: %s must take two arguments", function)
        }
    }

    fn define(expressions:~[Expression], environment:@Environment) -> Expression {
        reset_variable(expressions, environment, ~"define")
    }

    }

    match copy expression {
        List( expressions ) => {
            match expressions[0] {
                Symbol(~"quote") => quote(expressions),
                Symbol(~"begin") => begin(expressions, environment),
                Symbol(~"if") => if_(expressions, environment),
                Symbol(~"define") => define(expressions, environment),
                _ => expression
            }
        }
        Symbol( symbol ) => {
            match environment.lookup( copy symbol ) {
                Some( value ) => value,
                None => fail fmt!("Undefined symbol %s",symbol)
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

#[test]
fn test_environment_accepts_new_value_and_returns_it() {
    let env:Environment = Environment::new_global_environment();
    env.define( ~"monkey", Int(1) );
    env.define( ~"turkey", Int(2) );
    match env.lookup( ~"monkey" ) {
        Some(Int(1)) => (),
        _ => fail ~"Monkey wasn't an Int(1)"
    }
    match env.lookup( ~"turkey" ) {
        Some(Int(2)) => (),
        _ => fail ~"Turkey wasn't an Int(2)"
    }
}  

#[test]
fn test_environment_allows_values_to_change() {
    let env:Environment = Environment::new_global_environment();
    env.define( ~"monkey", Int(1) );
    match env.lookup( ~"monkey" ) {
        Some(Int(1)) => (),
        _ => fail ~"Monkey wasn't an Int(1) before mutation"
    }
    env.define( ~"monkey", Int(2) );
    match env.lookup( ~"monkey" ) {
        Some(Int(2)) => (),
        _ => fail ~"Monkey wasn't an Int(2) after mutation"
    }
}

#[test]
fn test_environment_checks_enclosing_environment() {
    let enclosure:~Environment = ~Environment::new_global_environment();
    enclosure.define( ~"monkey", Int(1) );
    let env:Environment = Environment::new(enclosure);
    match env.lookup( ~"monkey" ) {
        Some(Int(1)) => (),
        _ => fail ~"monkey wasn't found in env or enclosure... he's escaped?"
    }
}

struct Environment {
    enclosure:Option<~Environment>,
    mappings:std::treemap::TreeMap<~str,Expression>
}

impl Environment {
    fn define(&self, key:~str, value:Expression) {
        std::treemap::insert(self.mappings, key, value);
    }

    fn check_enclosure(&self, key:~str) -> Option<Expression> {
        match copy self.enclosure {
            Some(environment) => environment.lookup(key),
            _ => None
        }
    }

    fn lookup(&self, key:~str) -> Option<Expression> {
        let local_definition = std::treemap::find(self.mappings, copy key);
        match local_definition {
            None => self.check_enclosure(key),
            _ => local_definition
        }
    }

    static fn new_global_environment() -> Environment {
        let mappings = std::treemap::TreeMap();
        Environment {enclosure:None, mappings:mappings} 
    }

    static fn new(enclosure:~Environment) -> Environment {
        let mappings = std::treemap::TreeMap();
        Environment {enclosure:Some(enclosure), mappings:mappings}
    }
}

fn main() {
    io::println(stringify_expression( parse( "(1 2 3 (1 2 3))" ) ));
    io::println(stringify_expression( parse( "((1 2) 3 (1 2 3))" ) ));
    let blah:Expression = List(~[Int(1), List(~[Float(1.0), Symbol(~"xyz")])]);
    io::println(stringify_expression(blah));
    io::println( "(begin 1 2)" )
}
