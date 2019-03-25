#!/bin/bash

URL="https://forecast.weather.gov/MapClick.php?lat=35.9082&lon=-79.0459&unit=0&lg=english&FcstType=text&TextType=1"

# curl options
# --silent - prevents display of download progress
#
# pandoc options
# --wrap=none - avoids automatic line wrapping which helps for matching against complete paragraphs 
#
# egrep options
# --ignore-case
