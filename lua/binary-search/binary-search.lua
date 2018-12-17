local function recurse(array, searched, low, high)
	if low > high then
		return -1
	end
		
	local mid = math.ceil((low + high) / 2)
	
	if array[mid] == searched then
		return mid
	elseif array[mid] > searched then
			return recurse(array, searched, low, mid-1)
	else -- array[mid] < searched 
			return recurse(array, searched, mid+1, high)
	end			
end

local function find(array, searched)
	return recurse(array, searched, 1, #array)
end

return find