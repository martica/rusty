all: test run

clean:
	rm -rf bin/*

run: bin/rusty
	bin/rusty
	
test: bin/test-rusty
	bin/test-rusty

bin/test-rusty: rusty.rs
	rustc --test rusty.rs -o bin/test-rusty

bin/rusty: rusty.rs
	rustc rusty.rs -o bin/rusty
