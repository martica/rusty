/*
 * Built-in method definitions for scheme
 * 
 * +, -, *, / (works on lists of numbers)
 * =, >, <, >=, <= (works on list of numbers)
 * not (works on single argument)
 * list (works on any arguments)
 * list?, null?, symbol? (works on single argument)
 * cons, car, cdr, append
 * equal?
 * eq?
 *
 */

macro_rules! math_function {
    ($function:ident $identity:expr) => {
        pub fn $function( args:~[Expression], _:@Environment) -> Expression {
            match args {
                [] => $identity,
                [head] => vec::foldl($identity, ~[head], |x, y| {x.$function(y)}),
                [head, ..tail] => vec::foldl(head, tail, |x, y| {x.$function(y)})
            }
        }
    }
}

math_function!(add Int(0))
math_function!(sub Int(0))
math_function!(mul Int(1))
math_function!(div Int(1))

macro_rules! return_first_error {
    () => {
        for args.each() |&arg| {
            if arg.is_error() {
                return arg;
            }
        };
    }
}

fn sometimes_ess( number:uint ) -> ~str {
    if number == 1 {
        ~""
    } else {
        ~"s"
    }
}

macro_rules! assert_mininum_number_of_args {
    ($function:expr $minimum:expr) => {
        if args.len() < $minimum {
            return Error(fmt!("Built-in function '%s' takes at least %u argument%s. It was called with %u '%s'", $function, $minimum, sometimes_ess($minimum), args.len(), List(args).to_str()));
        };
    }
}

macro_rules! ensure_arguments_are_numbers {
    () => {
        for args.each() |&arg| {
            if !arg.is_number() {
                return Error( fmt!("%s was given where a number was expected", arg.to_str()) );
            }
        };
    }
}

macro_rules! comparison_function {
    ($function:ident $name:expr) => {
        pub fn $function( args:~[Expression], _:@Environment ) -> Expression {
            assert_mininum_number_of_args!($name 2)
            return_first_error!()
            ensure_arguments_are_numbers!()

            let comparisons = vec::map2( args.init(), args.tail(),
                                         |a, b| {a.$function(b)});
            Bool(vec::foldl(true, comparisons, |x, &y| {x && y}))
        }
    }
}

comparison_function!(lt ~"<")
comparison_function!(le ~"<=")
comparison_function!(gt ~">")
comparison_function!(ge ~">=")

pub fn equals( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()

    for args.tail().each() |&expr| {
        if expr != args.head() {
            return Bool(false);
        }
    }
    return Bool(true);
}

macro_rules! assert_arg_count_range {
    ($function:expr $minimum:expr $maximum:expr) => {
        if args.len() > $maximum || args.len() < $minimum {
            let head = fmt!("Built-in function '%s' takes", $function);
            let middle = if $minimum == $maximum {
                fmt!("only %u argument%s.", $minimum, sometimes_ess($minimum))
            } else {
                fmt!("betwen %u and %u arguments.", $minimum, $maximum)
            };
            let tail = fmt!("It was called with %u '%s'.", args.len(), List(args).to_str());

            return Error( fmt!("%s %s %s", head, middle, tail) );
        }
    }
}

pub fn not( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"not" 1 1 )

    return Bool(!args[0].to_bool());
}

pub fn car( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"car" 1 1 )

    match copy args[0] {
        List(list) => list.head(),
        _ => Error( fmt!("Built-in function 'car' requires a list argument. It was called with %s", args[0].to_str()) )
    }
}

pub fn cdr( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"cdr" 1 1 )

    match copy args[0] {
        List(list) => List(list.tail()),
        _ => Error( fmt!("Built-in function 'cdr' requires a list argument. It was called with %s", args[0].to_str()) )
    }
}

pub fn cons( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"cons" 2 2 )

    match copy args[1] {
        List(list) => List( ~[args[0]] + list ),
        x => List( ~[args[0]] + ~[x] )
    }
}

pub fn append( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"append" 2 2 )

    match copy args[0] {
        List(list1) => {
            match copy args[1] {
                List(list) => List( list1 + list ),
                x => List( list1 + ~[x] )
            }
        }
        _ => Error( fmt!("Built-in function 'append' requires a list as the first arguments. It was called with %s", List(args).to_str()) )
    }
}

pub fn length( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"length" 1 1 )
    
    match copy args[0] {
        List(list) => Int(list.len() as int),
        _ => Error( fmt!("Built-in function 'length' requires a list argument. It was called with %s", List(args).to_str()) )
    }
}

pub fn equal_( args:~[Expression], _:@Environment ) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"equal?" 2 2 )
    
    Bool(args[0] == args[1])
}


pub fn symbol_( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"symbol?" 1 1 )
    
    match args[0] {
        Symbol(_) => Bool(true),
        _ => Bool(false)
    }
}

pub fn list_( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"list?" 1 1 )
    
    match args[0] {
        List(_) => Bool(true),
        _ => Bool(false)
    }
}

pub fn null_( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"null?" 1 1 )
    
    match copy args[0] {
        List(list) => Bool(list.len() == 0),
        _ => Bool(false)
    }
}

pub fn list( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    List(args)
}

pub fn eq_( args:~[Expression], _:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"eq?" 2 2 )

    Bool( args[0] == args[1] )
}

pub fn eqv_( args:~[Expression], _env:@Environment) -> Expression {
    return_first_error!()
    assert_arg_count_range!( ~"eqv?" 2 2 )

    eq_( args, _env )
}

#[test]
fn test_eqv_() {
    fn eqv__( args:~[Expression] ) -> Expression {
        eqv_( args, @Environment::new_global_environment() )
    }

    assert( Bool(true) == eqv__( ~[ Bool(true), Bool(true) ] ) );
    assert( Bool(true) == eqv__( ~[ Bool(false), Bool(false) ] ) );
    assert( Bool(false) == eqv__( ~[ Bool(true), Bool(false) ] ) );
    assert( Bool(true) == eqv__( ~[ Symbol(~"a"), Symbol(~"a") ] ) );
    assert( Bool(false) == eqv__( ~[ Symbol(~"b"), Symbol(~"a") ] ) );
    assert( Bool(true) == eqv__( ~[ Int(1), Int(1) ] ) );
    assert( Bool(false) == eqv__( ~[ Int(2), Int(1) ] ) );
    assert( Bool(true) == eqv__( ~[ Float(1.0), Float(1.0) ] ) );
    assert( Bool(false) == eqv__( ~[ Float(2.0), Float(1.0) ] ) );
    assert( Bool(true) == eqv__( ~[ List( ~[] ), List( ~[] ) ] ) );
    assert( Bool(false) == eqv__( ~[ List( ~[Int(1)] ), List( ~[] ) ] ) );

    let env = @Environment::new_global_environment();
    let proc = eval( parse("(lambda (x) (* x x))"), env ).first();
    let proc2 = eval( parse("(lambda (x) (* x x))"), env ).first();
    assert( Bool(true) == eqv__( ~[ proc, proc ] ) );
    assert( Bool(false) == eqv__( ~[ proc, proc2 ] ) );
}

#[test]
fn test_math() {
    test_eval( ~"(+ 4 2)", ~"6" );
    test_eval( ~"(- 4 2)", ~"2" );
    test_eval( ~"(/ 4 2)", ~"2" );
    test_eval( ~"(* 4 2)", ~"8" );
    test_eval( ~"(= 4 2)", ~"#f" );
    test_eval( ~"(= 4 4)", ~"#t" );
}

#[test]
fn test_list() {
    test_eval( ~"(list? (quote ()))", ~"#t" );
    test_eval( ~"(car (quote (1 2 3)))", ~"1" );
    test_eval( ~"(cdr (quote (1 2 3)))", ~"(2 3)" );
    test_eval( ~"(cons 1 (quote (2 3)))", ~"(1 2 3)" );
    test_eval( ~"(list 1)", ~"(1)" );
    test_eval( ~"(list)", ~"()" );
    test_eval( ~"(list 1 2 3)", ~"(1 2 3)" );
    test_eval( ~"(length (list))", ~"0" );
    test_eval( ~"(length (list 1))", ~"1" );
    test_eval( ~"(length (list 1 2))", ~"2" );
}

pub fn builtins() -> ~[(~str,~fn(~[Expression], @Environment) -> Expression)] {
    ~[ (~"+", add), (~"-", sub), (~"*", mul), (~"/", div),
       (~"<", lt), (~"<=", le), (~">", gt), (~">=", ge),
       (~"=", equals), (~"not", not),
       (~"car", car), (~"cdr", cdr),
       (~"cons", cons), (~"append", append),
       (~"list", list), (~"length", length),
       (~"eq?", eq_), (~"eqv?", eqv_),
       (~"equal?", equal_),
       (~"symbol?", symbol_),
       (~"list?", list_),
       (~"null?", null_)
    ]
}
