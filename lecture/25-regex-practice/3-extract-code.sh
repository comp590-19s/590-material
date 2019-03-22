#!/bin/bash

# Use a pattern with a capturing group to extract only the course code
# from the URL of each department.

sed -E 's/.*\[(.*)\].*/\1/g' ./data/dept-lines.md >./data/dept-codes
