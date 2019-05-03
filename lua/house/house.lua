local house = {}

function pair(obj, action)
	return {obj = obj, action = action}
end

parts = {
	pair("house that Jack built", nil),
	pair("malt", "lay in"),
	pair("rat", "ate"),
	pair("cat", "killed"),
	pair("dog", "worried"),
	pair("cow with the crumpled horn", "tossed"),
	pair("maiden all forlorn", "milked"),
	pair("man all tattered and torn", "kissed"),
	pair("priest all shaven and shorn", "married"),
	pair("rooster that crowed in the morn", "woke"),
	pair("farmer sowing his corn", "kept"),
	pair("horse and the hound and the horn", "belonged to")
}

function house.verse(pos)
	-- recites each verse of the song
	local comp = "This is the "..parts[pos].obj
	if pos > 1 then
		for i=pos,2,-1 do
			comp = comp.."\nthat "..parts[i].action.." the "..parts[i-1].obj
		end
	end
	comp = comp.."."
	return comp
end

function house.recite()
	-- recites the entire song
	local song = ""
	for i=1,#parts do
		song = song..house.verse(i)
		if i ~= #parts then
			song = song.."\n"
		end	
	end
	return song
end

return house