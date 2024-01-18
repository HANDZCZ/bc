-- remve brackets from citations
function Cite(element)
    found = false

    -- check if any citation contains SuppressAuthor
    for _, val in ipairs(element.citations) do
        if val.mode == "SuppressAuthor" then
            found = true
            break
        end
    end

    -- if citations contain SuppressAuthor and there are more then one citation return error and don't modify anything
    if found and #element.citations ~= 1 then
        print("Found citations mixed with citation that is set to SuppressAuthor or multiple SuppressAuthor citations -> skipping modification!")
        print("  └─» To fix this move SuppressAuthor citation/s to it's own citation block.")
        return element
    end

    -- if citation with SuppressAuthor is found then remove brackets from it
    if found then
        table.remove(element.content, 3)
        table.remove(element.content, 1)
    end

    -- return modified or original element
    return element
end

return {
  {Cite = Cite}
}
