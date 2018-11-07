local hamming = {}

function hamming.compute(left, right)
	if left:len() ~= right:len() then
		return -1
	end

	local dist = 0	
	for	i = 1, left:len() do
		if left:sub(i,i) ~= right:sub(i,i) then
			dist = dist + 1
		end
	end
	return dist	
end

return hamming