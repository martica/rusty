
pub enum Expression {
    Int(int),
    Float(float),
    Symbol(~str),
    List(~[Expression])
} 

pub fn stringify( expression:Expression ) -> ~str {
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
            let strings = expressions.map( | &expr | {stringify(expr)} );
            ~"(" + strings.foldl(~"", |&x, &y| { x + ~" " + y } ).trim() + ~")"
        }
    }
}
