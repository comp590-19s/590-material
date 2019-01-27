SRC = $(wildcard *.md)
DST = $(addprefix /vagrant/,$(SRC:.md=.pdf))

.PHONY: all

all: $(DST)

/vagrant/%.pdf: ./%.md
	pandoc -o $@ $<
