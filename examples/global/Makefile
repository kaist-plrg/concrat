CFLAGS = -pthread

a.out: a.o
	$(CC) $(CFLAGS) -o $@ $^

a_cil.out: a.o b.o
	$(CC) $(CFLAGS) -o $@ $^

a_cil.out_comb.c: a.c
	CC="cilly --gcc=/usr/bin/gcc --merge --keepmerged" make a_cil.out

compile_commands.json: a.c
	intercept-build make a.out

a.rs: compile_commands.json
	make clean-rust && c2rust transpile -e compile_commands.json

clean:
	rm -f *.o *.out 

clean-rust:
	rm -rf *.rs Cargo.* target rust-toolchain
