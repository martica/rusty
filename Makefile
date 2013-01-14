dir_guard=@mkdir -p $(@D)
ENV=RUST_LOG=rustc=1,::rt::backtrace

all: test run

clean:
	rm -rf bin

run: bin/rusty
	bin/rusty
	
test: bin/test-rusty
	bin/test-rusty

bin/test-rusty: rusty.rs
	$(dir_guard)
	$(ENV) rustc --test rusty.rs -o bin/test-rusty

bin/rusty: rusty.rs
	$(dir_guard)
	$(ENV) rustc rusty.rs -o bin/rusty
