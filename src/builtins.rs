/*
 * Built-in method definitions for scheme
 * 
 * Done:
 * +, -, *, / (works on lists of numbers)
 * =, >, <, >=, <= (works on list of numbers)
 * not (works on single argument)
 * list (works on any arguments)
 * list?, null?, symbol? (works on single argument)
 * cons, car, cdr, append
 * equal?
 *
 * Coming:
 * eq?
 *
 *    {'+':op.add, '-':op.sub, '*':op.mul, '/':op.div, 'not':op.not_,
       '>':op.gt, '<':op.lt, '>=':op.ge, '<=':op.le, '=':op.eq, 
       'equal?':op.eq, 'eq?':op.is_, 'length':len, 'cons':lambda x,y:[x]+y,
       'car':lambda x:x[0],'cdr':lambda x:x[1:], 'append':op.add,  
       'list':lambda *x:list(x), 'list?': lambda x:isa(x,list), 
       'null?':lambda x:x==[], 'symbol?':lambda x: isa(x, Symbol)})
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

pub fn equals( args:~[Expression], _:@Environment) -> Expression {
    for args.tail().each() |&expr| {
        if expr != args.head() {
            return Bool(false);
        }
    }
    return Bool(true);
}

pub fn assert_number_of_args( function:~str, min:uint, max:uint, args:~[Expression] ) {
    if args.len() > max || args.len() < min {
        if max == min {
            fail fmt!("Built-in function '%s' takes only %u argument%s. It was called with %u '%s'", function, max, if max==1 {~""} else {~"s"}, args.len(), List(args).to_str());
        } else {
            fail fmt!("Built-in function '%s' takes between %u and %u arguments. It was called with %u '%s'", function, min, max, args.len(), List(args).to_str());
        } 
    }
}

pub fn not( args:~[Expression], _:@Environment) -> Expression {
    assert_number_of_args( ~"not", 1, 1, copy args );

    return Bool(!args[0].to_bool());
}

pub fn car( args:~[Expression], _:@Environment) -> Expression {
    assert_number_of_args( ~"car", 1, 1, copy args );

    match copy args[0] {
        List(list) => list.head(),
        _ => fail fmt!("Built-in function 'car' requires a list argument. It was called with %s", args[0].to_str())
    }
}

pub fn cdr( args:~[Expression], _:@Environment) -> Expression {
    assert_number_of_args( ~"cdr", 1, 1, copy args );

    match copy args[0] {
        List(list) => List(list.tail()),
        _ => fail fmt!("Built-in function 'cdr' requires a list argument. It was called with %s", args[0].to_str())
    }
}

pub fn cons( args:~[Expression], _:@Environment) -> Expression {
    assert_number_of_args( ~"cons", 2, 2, copy args );

    match copy args[1] {
        List(list) => List( ~[args[0]] + list ),
        x => List( ~[args[0]] + ~[x] )
    }
}

pub fn append( args:~[Expression], _:@Environment) -> Expression {
    assert_number_of_args( ~"append", 2, 2, copy args );

    match copy args[0] {
        List(list1) => {
            match copy args[1] {
                List(list) => List( list1 + list ),
                x => List( list1 + ~[x] )
            }
        }
        _ => fail fmt!("Built-in function 'append' requires a list as the first arguments. It was called with %s", List(args).to_str())
    }
}

pub fn length( args:~[Expression], _:@Environment) -> Expression {
    assert_number_of_args( ~"length", 1, 1, copy args );
    
    match copy args[0] {
        List(list) => Int(list.len() as int),
        _ => fail fmt!("Built-in function 'append' requires a list argument. It was called with %s", List(args).to_str())
    }
}

pub fn equal_( args:~[Expression], _:@Environment ) -> Expression {
    assert_number_of_args( ~"equal?", 2, 2, copy args );
    
    Bool(args[0] == args[1])
}


pub fn symbol_( args:~[Expression], _:@Environment) -> Expression {
    assert_number_of_args( ~"symbol?", 1, 1, copy args );
    
    match args[0] {
        Symbol(_) => Bool(true),
        _ => Bool(false)
    }
}

pub fn list_( args:~[Expression], _:@Environment) -> Expression {
    assert_number_of_args( ~"list?", 1, 1, copy args );
    
    match args[0] {
        List(_) => Bool(true),
        _ => Bool(false)
    }
}

pub fn null_( args:~[Expression], _:@Environment) -> Expression {
    assert_number_of_args( ~"null?", 1, 1, copy args );
    
    match copy args[0] {
        List(list) => Bool(list.len() == 0),
        _ => Bool(false)
    }
}

pub fn list( args:~[Expression], _:@Environment) -> Expression {
    List(args)
}

pub fn eq_( args:~[Expression], _:@Environment) -> Expression {
    assert_number_of_args( ~"eq?", 2, 2, copy args );

    Bool( args[0] == args[1] )
}

pub fn eqv_( args:~[Expression], _env:@Environment) -> Expression {
    assert_number_of_args( ~"eq?", 2, 2, copy args );

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

    let proc = eval( parse("(lambda (x) (* x x))"), @Environment::new_global_environment() ).first();
    let proc2 = eval( parse("(lambda (x) (* x x))"), @Environment::new_global_environment() ).first();
    assert( Bool(true) == eqv__( ~[ proc, proc ] ) );
    assert( Bool(false) == eqv__( ~[ proc, proc2 ] ) );
}

pub fn builtins() -> ~[(~str,~fn(~[Expression], @Environment) -> Expression)] {
    ~[ (~"+", add), (~"-", sub), (~"*", mul), (~"/", div),
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
