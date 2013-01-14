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

bin/test-rusty: src/*.rs
	$(dir_guard)
	$(ENV) rustc $(OPTIONS) --test src/rusty.rs -o bin/test-rusty

bin/rusty: src/*.rs
	$(dir_guard)
	$(ENV) rustc $(OPTIONS) src/rusty.rs -o bin/rusty
