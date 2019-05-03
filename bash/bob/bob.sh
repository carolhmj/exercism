#!/bin/bash

set -o errexit

phrase=${1:-""}

is_uppercase() {
	[[ $1 =~ [[:alpha:]] && $1 =~ ^([A-Z1-9!?,%^*@#$\(\?]|[[:space:]])+$ ]]
}

is_whitespace() {
	[[ $1 =~ ^[[:space:]]*$ ]]
}

is_question() {
	[[ $1 =~ \?[[:space:]]*$ ]]
}

if is_question "$phrase" && is_uppercase "$phrase"; then
	echo "Calm down, I know what I'm doing!"
elif is_question "$phrase"; then
	echo "Sure."
elif is_uppercase "$phrase"; then
	echo "Whoa, chill out!"
elif is_whitespace "$phrase"; then
	echo "Fine. Be that way!"		 
else 
	echo "Whatever."
fi