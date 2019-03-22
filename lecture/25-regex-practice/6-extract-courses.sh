#!/bin/bash

# TODO: Open up one of the department markdown files
# in ./data/depts and find a way to match only lines
# with courses listed on them. Express this pattern in
# egrep. Warning you must escape *'s.

cat ./data/depts/*.md | egrep 'TODO'
