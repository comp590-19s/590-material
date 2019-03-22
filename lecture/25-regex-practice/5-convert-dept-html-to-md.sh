#!/bin/bash

# Run pandoc on all of the deptartment listing html files
# and store the results as markdown files in the same dir.

cat ./data/dept-codes | xargs -I % pandoc -o./data/depts/%.md ./data/depts/%.html
