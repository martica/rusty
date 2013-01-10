dir_guard=@mkdir -p $(@D)

all: test run

clean:
	rm -rf bin

run: bin/rusty
	bin/rusty
	
test: bin/test-rusty
	bin/test-rusty

bin/test-rusty: rusty.rs
	$(dir_guard)
	rustc --test rusty.rs -o bin/test-rusty

bin/rusty: rusty.rs
	$(dir_guard)
	rustc rusty.rs -o bin/rusty
