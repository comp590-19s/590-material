#!/bin/bash

# Use pandoc to convert depts index.html to markdown
pandoc -o ./data/depts.md ./data/depts.html
