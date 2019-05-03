#!/bin/bash

set -o errexit

phrase=${1:-""}

is_uppercase() {
	[[ $1 =~ [[:alpha:]] && $1 =~ ^([A-Z1-9!?,%^*@#$\(\?]|[[:space:]])+$ ]]
}

is_whitespace() {
	[[ $1 =~ ^[[:space:]]*$ ]]
}

if [[ $phrase =~ \?[[:space:]]*$ ]]; then
	if is_uppercase "$phrase"; then
		echo "Calm down, I know what I'm doing!"
	else 
		echo "Sure."
	fi				
else 
	if is_uppercase "$phrase"; then 
		echo "Whoa, chill out!"
	elif is_whitespace "$phrase"; then
		echo "Fine. Be that way!"
	else
		echo "Whatever."
	fi			
fi	