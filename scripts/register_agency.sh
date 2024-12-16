#!/bin/bash

get_arg() {
  read -p "$1: " arg
  if [ -z "$arg" ]; then
    exit 255
  else
    echo "$arg"
  fi
}

NAME=$(get_arg "Name")
OWNER=$(get_arg "Owner principal")
AGENT=$(get_arg "Agent name")
ADDRESS=$(get_arg "Address")
ZIP_CODE=$(get_arg "Zip code")
CITY=$(get_arg "City")
REGION=$(get_arg "Region")
COUNTRY=$(get_arg "Country")
CONTINENT=$(get_arg "Continent")
LAT=$(get_arg "Latitude")
LNG=$(get_arg "Longitude")
EMAIL=$(get_arg "Email")
MOBILE=$(get_arg "Mobile")
WEBSITE=$(get_arg "Website")
VAT=$(get_arg "VAT")
LOGO=$(get_arg "Logo")

dfx canister call --ic \
    deferred_minter \
    admin_register_agency \
    "( \
        principal \"$OWNER\", \
        record { \
            address = \"$ADDRESS\"; \
            agent = \"$AGENT\"; \
            city = \"$CITY\"; \
            continent = variant { $CONTINENT }; \
            country = \"$COUNTRY\"; \
            email = \"$EMAIL\"; \
            lat = opt \"$LAT\"; \
            lng = opt \"$LNG\"; \
            logo = opt \"$LOGO\"; \
            mobile = \"$MOBILE\"; \
            name = \"$NAME\"; \
            owner = principal \"$OWNER\"; \
            region = \"$REGION\"; \
            vat = \"$VAT\"; \
            website = \"$WEBSITE\"; \
            zip_code = \"$ZIP_CODE\"; \
        } \
    )"
