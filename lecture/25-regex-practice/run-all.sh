#!/bin/bash

# Match all of the numbered script files
# Then run them in order.

ls | egrep '^[0-9].*\.sh$' | xargs -I % bash %
