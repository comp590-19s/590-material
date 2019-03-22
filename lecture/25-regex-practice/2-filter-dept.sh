#!/bin/bash

# TODO: Find a pattern in the first argument that matches
# only the lines that begin with a dash and have a department's
# courses URL in them.

# Once you have this working, redirect the output to ./data/dept-lines.md
egrep 'TODO' ./data/depts.md >./data/dept-lines.md
