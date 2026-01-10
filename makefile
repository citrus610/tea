EXE ?= tea

ifeq ($(OS),Windows_NT)
	SUFFIX := .exe
	OSNAME := win
else
	SUFFIX :=
	OSNAME := linux
endif

main: clean
	cargo rustc --release -- -C target-cpu=native --emit link=$(EXE)$(SUFFIX)

clean:
	cargo clean
	rm -rf $(EXE)$(SUFFIX)
	rm -rf *.pdb