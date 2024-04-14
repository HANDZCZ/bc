chars = { ".", ",", "!", "?" }

-- get note for link if needed
function get_note(el)
    local text = el.content
    local url = el.target

    -- if the text is the same as url then return nil
    if #text == 1 and text[1].tag == "Str" and text[1].text == url then
        return nil
    end

    local note = pandoc.Note(pandoc.Link(url, url))

    -- return footnote
    return note
end

function Inlines(inlines)
    for i = #inlines - 1, 1, -1 do
        local next_ = inlines[i + 1]
        local curr = inlines[i]

        if curr.tag == "Link" then
            local note = get_note(curr)
            if note ~= nil then
                local middle = nil

                if next_.tag == "Str" and has_value(chars, next_.text) then
                    middle = table.remove(inlines, i + 1)
                end

                local new_inline = { curr, note }
                if middle ~= nil then
                    table.insert(new_inline, 2, middle)
                end
                inlines[i] = pandoc.Span(new_inline)
            end
        end
    end

    return inlines
end

function has_value(table, value)
    for _, val in ipairs(table) do
        if val == value then
            return true
        end
    end
    return false
end

return {
  {Inlines = Inlines}
}
