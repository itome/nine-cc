test:
	cargo test
	./test.sh

clean:
	rm -f *~ tmp*

.PHONY: test clean
