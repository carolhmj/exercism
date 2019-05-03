#!/bin/bash

set -o errexit
set -o nounset

#usage
if [[ "${#@}" -ne 2 ]]; then
	echo "Usage: hamming.sh <strand1> <strand2>"
	exit 1
fi

#disallows inputs of unequal length
if [[ "${#1}" -ne "${#2}" ]]; then
	echo "left and right strands must be of equal length"
	exit 1
fi

#iterates over both strands, adding one to each diff found
diff=0
for (( i = 0; i < "${#1}"; i++ )); do
	if [[ "${1:i:1}" != "${2:i:1}" ]]; then
		diff=$(( diff+1 ))
	fi
done

echo "$diff"