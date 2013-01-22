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
 *
 * Coming:
 * equal?, eq?
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
            match args.len() {
                0 => $identity,
                1 => vec::foldl($identity, args, |x, y| {x.$function(y)}),
                _ => vec::foldl(args.head(), args.tail(), |x, y| {x.$function(y)})
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

pub fn not( args:~[Expression], _:@Environment) -> Expression {
    if args.len() > 1 {
        fail fmt!("Built-in function 'not' takes only one argument. It was called with %u '%s'", args.len(), List(args).to_str());
    }

    return Bool(!args[0].to_bool());
}

pub fn car( args:~[Expression], _:@Environment) -> Expression {
    if args.len() != 1 {
        fail fmt!("Built-in function 'car' takes only one argument. It was called with %u '%s'", args.len(), List(args).to_str());
    }

    match copy args[0] {
        List(list) => list.head(),
        _ => fail fmt!("Built-in function 'car' requires a list argument. It was called with %s", args[0].to_str())
    }
}

pub fn cdr( args:~[Expression], _:@Environment) -> Expression {
    if args.len() != 1 {
        fail fmt!("Built-in function 'cdr' takes only one argument. It was called with %u '%s'", args.len(), List(args).to_str());
    }

    match copy args[0] {
        List(list) => List(list.tail()),
        _ => fail fmt!("Built-in function 'cdr' requires a list argument. It was called with %s", args[0].to_str())
    }
}

pub fn cons( args:~[Expression], _:@Environment) -> Expression {
    if args.len() != 2 {
        fail fmt!("Built-in function 'cons' takes two arguments. It was called with %u '%s'", args.len(), List(args).to_str());
    }

    match copy args[1] {
        List(list) => List( ~[args[0]] + list ),
        x => List( ~[args[0]] + ~[x] )
    }
}

pub fn append( args:~[Expression], _:@Environment) -> Expression {
    if args.len() != 2 {
        fail fmt!("Built-in function 'append' takes two arguments. It was called with %u '%s'", args.len(), List(args).to_str());
    }

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
    if args.len() != 1 {
        fail fmt!("Built-in function 'length' takes only one argument. It was called with %u '%s'", args.len(), List(args).to_str());
    }
    
    match copy args[0] {
        List(list) => Int(list.len() as int),
        _ => fail fmt!("Built-in function 'append' requires a list argument. It was called with %s", List(args).to_str())
    }
}

pub fn equal_( args:~[Expression], _:@Environment ) -> Expression {
    if args.len() != 2 {
        fail fmt!("Built-in function 'equal?' takes two arguments. It was called with %u '%s'", args.len(), List(args).to_str());
    }

    Bool(args[0] == args[1])
}


pub fn symbol_( args:~[Expression], _:@Environment) -> Expression {
    if args.len() != 1 {
        fail fmt!("Built-in function 'symbol?' takes only one argument. It was called with %u '%s'", args.len(), List(args).to_str());
    }

    match args[0] {
        Symbol(_) => Bool(true),
        _ => Bool(false)
    }
}

pub fn list_( args:~[Expression], _:@Environment) -> Expression {
    if args.len() != 1 {
        fail fmt!("Built-in function 'list?' takes only one argument. It was called with %u '%s'", args.len(), List(args).to_str());
    }

    match args[0] {
        List(_) => Bool(true),
        _ => Bool(false)
    }
}

pub fn null_( args:~[Expression], _:@Environment) -> Expression {
    if args.len() != 1 {
        fail fmt!("Built-in function 'null?' takes only one argument. It was called with %u '%s'", args.len(), List(args).to_str());
    }

    match copy args[0] {
        List(list) => Bool(list.len() == 0),
        _ => Bool(false)
    }
}

pub fn list( args:~[Expression], _:@Environment) -> Expression {
    List(args)
}

pub fn builtins() -> ~[(~str,~fn(~[Expression], @Environment) -> Expression)] {
    ~[ (~"+", add), (~"-", sub), (~"*", mul), (~"/", div),
       (~"=", equals), (~"not", not),
       (~"car", car), (~"cdr", cdr),
       (~"cons", cons), (~"append", append),
       (~"list", list), (~"length", length),
       (~"equal?", equal_),
       (~"symbol?", symbol_),
       (~"list?", list_),
       (~"null?", null_)
    ]
}
