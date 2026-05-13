C_SOURCES := $(wildcard examples/*.c)
CLANG_ASM := $(C_SOURCES:.c=.clang.s)
CRUST_ASM := $(C_SOURCES:.c=.crust.s)
CLANG_BIN := $(C_SOURCES:.c=.clang)
CRUST_BIN := $(C_SOURCES:.c=.crust)

CRUST := target/debug/crust
RUST_SOURCES := $(shell find src -name '*.rs') Cargo.toml

.PHONY: all clean

all: $(CLANG_ASM) $(CRUST_ASM) $(CLANG_BIN) $(CRUST_BIN)

$(CRUST): $(RUST_SOURCES)
	cargo build

examples/%.clang.s: examples/%.c
	cc -S -o $@ $<

examples/%.crust.s: examples/%.c $(CRUST)
	$(CRUST) $< > $@

examples/%.clang: examples/%.clang.s
	cc -o $@ $<

examples/%.crust: examples/%.crust.s
	cc -o $@ $<

clean:
	rm -f $(CLANG_ASM) $(CRUST_ASM) $(CLANG_BIN) $(CRUST_BIN)
