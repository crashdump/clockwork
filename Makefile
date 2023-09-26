prog := clockwork
debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=debug
else
  release :=--release
  target :=release
  extension :=
endif

all: clean build install

clean:
	$(MAKE) -C www clean
	rm -rf target/*

test:
	$(MAKE) -C www test
	cargo test

build:
	$(MAKE) -C www build
	cargo build $(release)

install:
	cp target/$(target)/$(prog) ~/bin/$(prog)

help:
	@echo "usage: make $(prog) [debug=1]"