
NPM = npm
NODE = node
src_dir = ./
dest_dir = ./bin/

SOURCES = \
	$(src_dir)src/index.js

all: $(dst_dir)index.js

$(dst_dir)index.js : 

npm_setup:
	$(NPM) install babel@/core babel@cli

.PHONY: npm_setup all

# vi: se ts=4 sw=4 noet:
