dir_guard=@mkdir -p $(@D)
ENV=RUST_LOG=rustc=1,::rt::backtrace
OPTIONS=-A non-implicitly-copyable-typarams

all: test run

clean:
	rm -rf bin

run: bin/rusty
	bin/rusty
	
test: bin/test-rusty
	bin/test-rusty

bin/test-rusty: rusty.rs environment.rs expression.rs parse.rs
	$(dir_guard)
	$(ENV) rustc $(OPTIONS) --test rusty.rs -o bin/test-rusty

bin/rusty: rusty.rs environment.rs expression.rs parse.rs
	$(dir_guard)
	$(ENV) rustc $(OPTIONS) rusty.rs -o bin/rusty
