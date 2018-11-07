#!/usr/bin/env bash

set -o errexit
set -o nounset

sentence=$1

#declares and populates our dictionary, we add zeroes for 
#each char we didn't see yet  
declare -A dict
for c in {a..z}; do
	dict[$c]=0
done

for (( i = 0; i < "${#sentence}"; i++ )); do
	char=$( echo "${sentence:i:1}" | tr '[:upper:]' '[:lower:]' )
	if [[ "${dict[$char]+_}" ]]; then
		dict[$char]=1
	fi
done

all_letters=1

for c in {a..z}; do
	if [ "${dict[$c]}" -eq 0 ]; then
		all_letters=0
	fi	
done

if [ "$all_letters" -eq 1 ]; then
	echo "true"
else 
	echo "false"
fi		