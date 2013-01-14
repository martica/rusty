extern mod std;

mod environment;
use environment::Environment;
mod expression;
use expression::Expression; 
use expression::{Int,Float,Symbol,List};
mod parse;
use parse::parse;

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
        _ => fail fmt!("If returned something unusual (%s)", expression::stringify(value))
    }
}

#[test]
fn test_that_if_evaluates_the_else_branch() {
    let expression = parse( ~"(if 0 7 (begin 1 2))" );
    let value = eval( expression, @Environment::new_global_environment() );
    match value {
        Int(2) => (),
        List( [Symbol(~"begin"), Int(1), Int(2)]) => fail ~"If just returned the else branch",
        _ => fail fmt!("If returned something unusual (%s)", expression::stringify(value))
    }
}

#[test]
fn test_that_if_evaluates_the_test() {
    let expression = parse( ~"(if (begin 1 0) 1 2)" );
    let value = eval( expression, @Environment::new_global_environment() );
    match value {
        Int(1) => fail ~"If didn't evaluate the test",
        Int(2) => (),
        _ => fail fmt!("If returned something unusual (%s)", expression::stringify(value))
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
        _ => fail fmt!("Expected 10 got %s", expression::stringify(value))
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
        _ => fail fmt!("Expected 10 got %s", expression::stringify(value))
    }
}

#[test]
#[should_fail]
fn test_that_set_cannot_create_a_variable() {
    let env = @Environment::new_global_environment();
    let expression = parse( ~"(set! x 10)" );
    eval( expression, env );
}

#[test]
fn test_that_set_can_change_a_variable() {
    let env = @Environment::new_global_environment();
    env.define(~"x", Int(100));
    let expression = parse( ~"(set! x 10)" );
    let value = eval( expression, env );
    match env.lookup(~"x") {
        Some(Int(10)) => (),
        _ => fail fmt!("Expected 10 got %s", expression::stringify(value))
    }
}

#[test]
fn test_that_set_returns_the_value_not_the_key() {
    let env = @Environment::new_global_environment();
    env.define(~"x", Int(100));
    let expression = parse( ~"(set! x 10)" );
    let value = eval( expression, env );
    match value {
        Int(10) => (),
        Symbol(~"x") => fail ~"set! returned the key, not the value",
        _ => fail fmt!("Expected 10 got %s", expression::stringify(value))
    }
}

#[test]
fn test_that_begin_can_handle_one_argument() {
    let env = @Environment::new_global_environment();
    let expression = parse( ~"(begin 10)" );
    let value = eval( expression, env );
    match value {
        Int(10) => (),
        _ => fail fmt!("Expected 10 got %s", expression::stringify(value))
    }
}

#[test]
fn test_that_begin_evaluates_all_arguments() {
    let env = @Environment::new_global_environment();
    let expression = parse( ~"(begin (define x 10) x)" );
    let value = eval( expression, env );
    match env.lookup(~"x") {
        Some(Int(10)) => (),
        _ => fail fmt!("Expected 10 got %s", expression::stringify(value))
    }
    match value {
        Int(10) => (),
        _ => fail fmt!("Expected 10 got %s", expression::stringify(value))
    }
}

fn eval( expression:Expression, environment:@Environment ) -> Expression {
    fn quote(expressions:~[Expression]) -> Expression {
        match expressions {
            [_, expr] => expr,
            _ => fail ~"Syntax Error: quote must take a single argument"
        }
    }

    fn begin(expressions:~[Expression], environment:@Environment) -> Expression {
        for expressions.tail().init().each() |&expression| {
            eval( expression, environment );
        }
        eval( expressions.last(), environment )
    }

    fn if_(expressions:~[Expression], environment:@Environment) -> Expression {
        match expressions {
            [_, test, true_expr, false_expr] => {
                eval(if expression::is_truthy( eval(test, environment) ) { true_expr } else { false_expr }, environment)
            }
            _ => fail ~"Syntax Error: if must take three arguments"
        }
    }

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

    fn set_bang(expressions:~[Expression], environment:@Environment) -> Expression {
        match copy expressions {
            [_, symbol, _] => {
                match copy symbol {
                    Symbol( key ) => {
                        match environment.lookup( copy key )  {
                            None => fail ~"Syntax Error: set! cannot create a variable",
                            _ => ()
                        }
                    }
                    _ => ()
                }
            }
            _ => ()
        }
        let symbol = reset_variable(expressions, environment, ~"set!");
        match copy symbol {
            Symbol( key ) => {
                match environment.lookup( copy key ) {
                    Some( value ) => value,
                    None => fail
                }
            }
            _ => fail
        }
    }

    match copy expression {
        List( expressions ) => {
            match expressions[0] {
                Symbol(~"quote") => quote(expressions),
                Symbol(~"begin") => begin(expressions, environment),
                Symbol(~"if") => if_(expressions, environment),
                Symbol(~"define") => define(expressions, environment),
                Symbol(~"set!") => set_bang(expressions, environment),
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

fn main() {
    io::println(expression::stringify( parse( "(1 2 3 (1 2 3))" ) ));
    io::println(expression::stringify( parse( "((1 2) 3 (1 2 3))" ) ));
    let blah:Expression = List(~[Int(1), List(~[Float(1.0), Symbol(~"xyz")])]);
    io::println(expression::stringify(blah));
    io::println( "(begin 1 2)" )
}
