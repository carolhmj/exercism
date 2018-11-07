local diff = {}

function diff.square_of_sum(n)
	local acc = 0
	for i=1,n do
		acc = acc + i
	end
	return acc^2
end

function diff.sum_of_squares(n)
	local acc = 0
	for i=1,n do
		acc = acc + i^2
	end
	return acc
end	

function diff.difference_of_squares(n)
	return diff.square_of_sum(n) - diff.sum_of_squares(n)
end

return diff