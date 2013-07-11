bin:
	mkdir -p bin

mongo-demo: bin
	rustc -L ./lib --out-dir ./bin src/mongo-demo/main.rs

clean:
	rm -rf *.dSYM/*
	rmdir -p *.dSYM
	rm -f *.o
