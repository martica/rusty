use send_map::linear::LinearMap;
mod builtins;

pub struct Environment {
    mappings:@[@mut LinearMap<~str,Expression>]
}

pub impl Environment {
    fn lookup( &self, key:~str ) -> Option<Expression> {
        for self.mappings.each() |&mapping| {
            match mapping.find(&key) {
                None => (),
                value => return value
            }
        }

        None
    }

    fn define( &self, key:~str, value:Expression ) {
        let map = self.mappings.head();
        map.insert(key, value);
    }

    fn reset( &self, key:~str, value:Expression ) {
        for self.mappings.each() |&mapping| {
            match mapping.find(&key) {
                None => (),
                Some(_) => {
                     mapping.insert(copy key, copy value);
                     return;
                }
            }
        }

        fail ~"Attempt to use set! with an undefined variable"
    }

    static fn new_global_environment() -> Environment {
        let mapping:LinearMap<~str,Expression> = LinearMap();
        let env = Environment {mappings:@[@mut mapping]};
        for builtins::builtins().each() |&(name, function)| {
            env.define(name, new_proc(function));
        }
        env
        //Environment {mappings:@[@mut mapping]}
    }

    static fn new(enclosure:Environment) -> Environment {
        let mapping:LinearMap<~str,Expression> = LinearMap();
        Environment {mappings:@[@mut mapping] + enclosure.mappings}
    }
}

#[test]
fn test_environments_with_shared_mapping_sets_share_changes() {
    let global_env:Environment = Environment::new_global_environment();
    global_env.define( ~"monkey", Int(1) );
    let sub_env1 = Environment::new(global_env);
    let sub_env2 = Environment::new(global_env);
    sub_env1.reset( ~"monkey", Int(2) );
    match sub_env2.lookup( ~"monkey" ) {
        Some(Int(1)) => fail ~"My monkeys are out of sync",
        Some(Int(2)) => (),
        None =>  fail ~"Monkey not found.",
        _ => fail ~"Monkey of unexpected value"
    }
}

#[test]
fn test_changes_to_new_scope_are_not_reflected_in_original_env() {
    let env:Environment = Environment::new_global_environment();
    env.define( ~"monkey", Int(1) );
    let sub_env = Environment::new(env);
    sub_env.define( ~"monkey", Int(2) );
    match env.lookup( ~"monkey" ) {
        Some(Int(1)) => (),
        Some(Int(2)) => fail ~"My monkey was changed!",
        Some(_) => fail ~"This is not my monkey",
        _ => fail ~"Where is my monkey?"
    }
}

#[test]
fn test_changes_to_nested_scope_are_reflected_in_original_env() {
    let env:Environment = Environment::new_global_environment();
    env.define( ~"monkey", Int(1) );
    let sub_env = Environment::new(env);
    sub_env.reset( ~"monkey", Int(2) );
    match env.lookup( ~"monkey" ) {
        Some(Int(2)) => (),
        Some(Int(1)) => fail ~"My monkey was cloned?",
        Some(_) => fail ~"This is not my monkey",
        _ => fail ~"Where is my monkey?"
    }
}

#[test]
fn test_environment_check_nested_scopes() {
    let env:Environment = Environment::new_global_environment();
    env.define( ~"monkey", Int(1) );
    let sub_env = Environment::new(env);
    match sub_env.lookup( ~"monkey" ) {
        Some(Int(1)) => (),
        Some(_) => fail ~"This is not my monkey",
        _ => fail ~"Where is my monkey?"
    }
}

#[test]
fn test_new_environment_allow_reset_of_defined_variable() {
    let env:Environment = Environment::new_global_environment();
    env.define( ~"monkey", Int(1) );
    env.reset( ~"monkey", Int(2) );
    match env.lookup( ~"monkey" ) {
        Some(Int(1)) => fail ~"Monkey didn't change",
        Some(Int(2)) => (),
        Some(_) => fail ~"Monkey changed unrecognizably",
        None => fail ~"Monkey got lost?"
    }
}

#[test]
#[should_fail]
fn test_new_environment_fails_to_reset_value() {
    let env:Environment = Environment::new_global_environment();
    env.reset( ~"monkey", Int(2) );
}

#[test]
fn test_new_environment_accepts_new_value() {
    let env:Environment = Environment::new_global_environment();
    env.define( ~"monkey", Int(1) );
    match env.lookup( ~"monkey" ) {
        Some(Int(1)) => (),
        Some(_) => fail ~"Monkey mutated?",
        None => fail ~"Monkey got lost?"
    }
}

#[test]
fn test_new_environment_is_empty() {
    let env:Environment = Environment::new_global_environment();
    match env.lookup( ~"monkey" ) {
        Some(_) => fail ~"Monkey already present?",
        None => ()
    }
}
