extern mod std;
use io::{Reader,ReaderUtil};

mod environment;
use environment::Environment;
mod expression;
use expression::Expression; 
use expression::{Bool,Int,Float,Symbol,List,Proc,Error,Lambda};
use expression::Expression::new_proc;
mod parse;
use parse::parse;

fn test_env() -> @Environment {
    @Environment::new_global_environment()
}

fn test_eval( expr:&str, result:&str ) {
    let evaluated = eval(parse(expr), test_env());
    let expected = parse(result);
    if expected != evaluated.first() {
        fail fmt!("Expected: %s -> Got %s", expected.to_str(), evaluated.first().to_str())
    }
}

fn test_eval_fails( expr:&str, result:&str, reason:&str ) {
    let evaluated = eval(parse(expr), test_env());
    let expected = parse(result);
    if expected == evaluated.first() {
        fail fmt!("%s should not have evaluated to %s (%s)", expr, result, reason)
    }
}

fn test_eval_to_error( expr:&str, reason:&str ) {
    let evaluated = eval(parse(expr), test_env());
    match evaluated.first() {
        Error(_) => (),
        _ => fail fmt!("%s should have returned an error: %s", expr, reason)
    }
}

#[test]
fn test_eval_empty_list_returns_itself() {
    test_eval( ~"()", ~"()" );
}

#[test]
fn test_eval_returns_number_when_passed_number() {
    test_eval( ~"1", ~"1" );
}

#[test]
fn test_eval_returns_expression_when_passed_quote() {
    test_eval( ~"(quote (a))", ~"(a)" );
}

#[test]
fn test_eval_returns_last_expression_when_passed_begin() {
    test_eval( ~"(begin 1 2 3)", ~"3" );
}

#[test]
fn test_if_returns_third_part_when_if_is_true() {
    test_eval( ~"(if 1 2 3)", ~"2" );
}  

#[test]
fn test_if_returns_fourth_part_when_if_is_false() {
    test_eval( ~"(if 0 2 3)", ~"3" );
}  

#[test]
fn test_that_if_evaluates_the_then_branch() {
    let expression = ~"(if 1 (begin 1 2) 7)";
    test_eval_fails( expression, ~"(begin 1 2)", ~"if just returned the raw then" );
    test_eval( expression, ~"2" ); 
}

#[test]
fn test_that_if_evaluates_the_else_branch() {
    let expression = ~"(if 0 7 (begin 1 2))";
    test_eval_fails( expression, ~"(begin 1 2)", ~"if just returned the else branch" );
    test_eval( expression, ~"2");
}

#[test]
fn test_that_if_evaluates_the_test() {
    let expression = ~"(if (begin 1 0) 1 2)";
    test_eval_fails( expression, ~"1", ~"if didn't evaluate the test" );
    test_eval( expression, ~"2" );
}

#[test]
fn test_that_bare_symbol_is_interpreted_as_variable() {
    let env = test_env();
    env.define(~"monkey", Int(10));
    let expression = parse( ~"monkey" );
    let value = eval( expression, env );
    match value {
        (Int(10), _) => (),
        _ => fail fmt!("Expected 10 got %s", value.first().to_str())
    }
}

#[test]
fn test_that_undefined_symbol_is_an_error() {
    let expression = ~"monkey";
    test_eval_to_error( expression, ~"Undefined symbol wasn't an error" );
}

#[test]
fn test_that_define_can_add_a_variable() {
    let env = test_env();
    let expression = parse( ~"(define x 10)" );
    let value = eval( expression, env );
    match env.lookup(~"x") {
        Some(Int(10)) => (),
        _ => fail fmt!("Expected 10 got %s", value.first().to_str())
    }
}

#[test]
fn test_that_set_cannot_create_a_variable() {
    let env = test_env();
    let expression = parse( ~"(set! x 10)" );
    let value = eval( expression, env );
    match value.first() {
        Error(_) => (),
        _ => fail ~"set! created a variable"
    }
}

#[test]
fn test_that_set_can_change_a_variable() {
    let env = test_env();
    env.define(~"x", Int(100));
    let expression = parse( ~"(set! x 10)" );
    let value = eval( expression, env );
    match env.lookup(~"x") {
        Some(Int(10)) => (),
        _ => fail fmt!("Expected 10 got %s", value.first().to_str())
    }
}

#[test]
fn test_that_set_returns_the_value_not_the_key() {
    let env = test_env();
    env.define(~"x", Int(100));
    let expression = parse( ~"(set! x 10)" );
    let value = eval( expression, env );
    match value {
        (Int(10), _) => (),
        (Symbol(~"x"), _) => fail ~"set! returned the key, not the value",
        _ => fail fmt!("Expected 10 got %s", value.first().to_str())
    }
}

#[test]
fn test_that_begin_can_handle_one_argument() {
    let env = test_env();
    let expression = parse( ~"(begin 10)" );
    let value = eval( expression, env );
    match value {
        (Int(10), _) => (),
        _ => fail fmt!("Expected 10 got %s", value.first().to_str())
    }
}

#[test]
fn test_that_begin_evaluates_all_arguments() {
    let env = test_env();
    let expression = parse( ~"(begin (define x 10) x)" );
    let value = eval( expression, env );
    match env.lookup(~"x") {
        Some(Int(10)) => (),
        _ => fail fmt!("Expected 10 got %s", value.first().to_str())
    }
    match value {
        (Int(10), _) => (),
        _ => fail fmt!("Expected 10 got %s", value.first().to_str())
    }
}

#[test]
fn test_that_other_symbols_are_evaluated_as_procs() {
    let env = test_env();
    let expression = parse( ~"(+ 1 2)" );
    let value = eval( expression, env );
    match value {
        (Int(3), _) => (),
        _ => fail fmt!("(+ 1 2) became %s", value.first().to_str())
    }
}

#[test]
fn test_that_proc_params_are_evaluated() {
    let env=test_env();
    let expression = parse( ~"(+ (+ 1 2) 3)" );
    let value = eval (expression, env);
    match value {
        (Int(6), _) => (),
        _ => fail fmt!("(+ (+ 1 2) 3) became %s", value.first().to_str())
    }
}

#[test]
fn test_that_lambda_evaluates_to_a_lambda() {
    let env=test_env();
    let expression = parse( ~"(lambda (x) (* x x))" );
    let value = eval(expression, env);
    match value {
       (Lambda(_,_,_), _) => (),
        _ => fail ~"lambda doesn't turn into a Proc"
    }
}

#[test]
fn test_that_lambda_without_variables_evals() {
    let env=test_env();
    let expression = parse( ~"( (lambda () (+ 1 1))  )" );
    let value = eval(expression, env);
    match value {
        (Int(2), _) => (),
        _ => fail fmt!("lambda evaluated to %s", value.first().to_str())
    }
}

#[test]
fn test_that_lambda_with_a_variable_evals() {
    let env=test_env();
    let expression = parse( ~"( (lambda (x) (+ x 1)) 1  )" );
    let value = eval(expression, env);
    match value {
        (Int(2), _) => (),
        _ => fail fmt!("lambda evaluated to %s", value.first().to_str())
    }
}

fn eval( expression:Expression, environment:@Environment ) -> (Expression, @Environment ) {
    fn quote(expressions:~[Expression]) -> Expression {
        match expressions {
            [_, expr] => expr,
            _ => Error( ~"Syntax Error: quote must take a single argument" )
        }
    }

    fn begin(expressions:~[Expression], environment:@Environment) -> Expression {
        for expressions.tail().init().each() |&expression| {
            eval( expression, environment );
        }
        eval( expressions.last(), environment ).first()
    }

    fn if_(expressions:~[Expression], environment:@Environment) -> Expression {
        match expressions {
            [_, test, true_expr, false_expr] => {
                let condition = eval(test, environment).first();
                eval(if condition.to_bool() {
                    true_expr
                } else {
                    false_expr
                }, environment).first()
            }
            _ => Error( ~"Syntax Error: if must take three arguments" )
        }
    }

    fn reset_variable(expressions:~[Expression], environment:@Environment, function:~str) -> Expression {
        match expressions {
            [_, symbol, value] => {
                match copy symbol {
                    Symbol( key ) => {
                        environment.reset(key, eval(value, environment).first());
                        symbol
                    }
                    _ => Error( fmt!("Syntax Error: %s takes a symbol as its first argument", function) )
                }
            }
            _ => Error( fmt!("Syntax Error: %s must take two arguments", function) )
        }
    }

    fn define_variable(expressions:~[Expression], environment:@Environment, function:~str) -> Expression {
        match expressions {
            [_, symbol, value] => {
                match copy symbol {
                    Symbol( key ) => {
                        environment.define(key, eval(value, environment).first());
                        symbol
                    }
                    _ => Error( fmt!("Syntax Error: %s takes a symbol as its first argument", function) )
                }
            }
            _ => Error( fmt!("Syntax Error: %s must take two arguments", function) )
        }
    }

    fn define(expressions:~[Expression], environment:@Environment) -> Expression {
        define_variable(expressions, environment, ~"define")
    }

    fn set_bang(expressions:~[Expression], environment:@Environment) -> Expression {
        match copy expressions {
            [_, symbol, _] => {
                match copy symbol {
                    Symbol( key ) => {
                        match environment.lookup( copy key )  {
                            None => return Error( ~"Syntax Error: set! cannot create a variable" ),
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
                    None => Error( ~"System Error: Reset variable failed to set the new value" )
                }
            }
            _ => Error( ~"System Error: Reset variable didn't return a symbol" )
        }
    }

    fn proc(expressions:~[Expression], environment:@Environment) -> Expression {
        let exprs = expressions.map(|&expr| eval(expr, environment).first());
        match exprs.head() {
            Proc( procedure, _ ) => procedure( exprs.tail(), environment ),
            Lambda( expr, variables, env ) => {
                let local_env = @Environment::new( *env );
                for vec::zip(copy variables, exprs.tail()).each |param| {
                    match param.first() {
                        Symbol(key) => local_env.define(key, param.second()),
                        _ => return Error( ~"Variable names must be symbols" )
                    }
                }
                eval( *expr, local_env).first()
            }
            _ => Error( fmt!("\"%s\" is not a procedure", exprs.head().to_str()) )
        }
    }

    fn lambda(expressions:~[Expression], env:@Environment) -> Expression {
        match copy expressions {
            [_, List(param_names), expression] => {
                Lambda(@expression, param_names, env)
            }
//            new_proc( |param_values, _| { 
//                    let local_env = @Environment::new( copy *env );
//                    for vec::zip(copy param_names, param_values).each |param| {
//                        match param.first() {
//                            Symbol(key) => local_env.define( key, param.second() ),
//                            _ => return Error( ~"lambda params list must be list of symbols" )
//                        }
//                    }
//                    eval(copy expression, local_env).first()
//                } ),
            _ => Error( fmt!("Syntax Error: lambda requires 2 arguments, got \"%u\"", expressions.len()-1 ) )
        }
    }

    (match copy expression {
        List( expressions ) => {
            if expressions.len() == 0 {
                expression
            } else {
                match expressions[0] {
                    Symbol(~"quote") => quote(expressions),
                    Symbol(~"begin") => begin(expressions, environment),
                    Symbol(~"if") => if_(expressions, environment),
                    Symbol(~"define") => define(expressions, environment),
                    Symbol(~"set!") => set_bang(expressions, environment),
                    Symbol(~"lambda") => lambda(expressions, environment),
                    _ => proc(expressions, environment) 
                }
            }
        }
        Symbol( symbol ) => {
            match environment.lookup( copy symbol ) {
                Some( value ) => value,
                None => Error( fmt!("Undefined symbol %s",symbol) )
            }
        }
        _ => {
            expression
        }
    }, environment)
}

fn main() {
    fn evaluate( expr:~str, env:Environment ) -> Option<Environment> {
        let (result, new_env) = eval( parse(expr), @env );
        io::println( fmt!("%s -> %s", expr, result.to_str() ));
        Some(*new_env)
    }

    let mut env = Environment::new_global_environment();
    loop {
        io::print("rusty> ");
        let in = io::stdin().read_line();
        let result = evaluate( in, copy env );
        match result {
            Some(new_env) => env = new_env,
            None => ()
        }
    }
}
