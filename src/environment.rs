use send_map::linear::LinearMap;
mod builtins;

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
    let enclosure:@Environment = @Environment::new_global_environment();
    enclosure.define( ~"monkey", Int(1) );
    let env:Environment = Environment::new(enclosure);
    match env.lookup( ~"monkey" ) {
        Some(Int(1)) => (),
        _ => fail ~"monkey wasn't found in env or enclosure... he's escaped?"
    }
}

pub struct Environment {
    enclosure:Option<@Environment>,
    mut mappings:LinearMap<~str,Expression>
}

pub impl Environment {
    fn define(&self, key:~str, value:Expression) {
        self.mappings.insert(key, value);
    }

    fn check_enclosure(&self, key:~str) -> Option<Expression> {
        match copy self.enclosure {
            Some(environment) => environment.lookup(key),
            _ => None
        }
    }

    fn lookup(&self, key:~str) -> Option<Expression> {
        let local_definition = self.mappings.find(&key);
        match local_definition {
            None => self.check_enclosure(key),
            _ => local_definition
        }
    }

    static fn new_global_environment() -> Environment {
        let mappings = LinearMap();
        let env = Environment {enclosure:None, mappings:mappings};
        for builtins::builtins().each() |&(name, function)| {
            env.define(name, new_proc(function));
        }
        env
    }

    static fn new(enclosure:@Environment) -> Environment {
        let mappings = LinearMap();
        Environment {enclosure:Some(enclosure), mappings:mappings}
    }

}

pub fn env_with_new_global(env:@Environment, global:@Environment) -> @Environment {
    match env.enclosure {
        Some(parent) => {
            let enclosure = match parent.enclosure {
                Some(grandparent) => env_with_new_global(parent, global),
                _ => global
            };
            @Environment {enclosure:Some(enclosure), mappings:copy env.mappings}
        }
        _ => @Environment::new(global)
    }
}

pub fn topmost_env(env:@Environment) -> @Environment {
    match env.enclosure {
        Some(parent) => topmost_env(parent),
        _ => env
    }
}
