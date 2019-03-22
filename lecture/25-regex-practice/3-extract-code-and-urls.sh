#!/bin/bash

sed -E "s/.*\[.*\((.{4})\)\]\((.*)\).*/\1 \2 \3.html/g" ./data/dept-lines.md
