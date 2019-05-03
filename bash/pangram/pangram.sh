#!/usr/bin/env bash

set -o errexit
set -o nounset
#case insensitive pattern matching
shopt -s nocasematch

sentence=$1

#declares and populates our dictionary, we add zeroes for 
#each char we didn't see yet  
declare -A dict
for c in {a..z}; do
  dict[$c]=0
done

#go over each lettern and tries to pattern match it 
for c in {a..z}; do
  if [[ $sentence =~ $c ]]; then
    dict[$c]=1
  else
  	echo "false"
  	exit 0  
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