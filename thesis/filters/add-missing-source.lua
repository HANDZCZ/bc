-- adds source to every element if it's missing

figure_source = nil
table_source = nil
listings_source = nil

-- check if the `figure-source` variable is set
function Meta(meta)
  if meta["figure-source"] then
      figure_source = meta["figure-source"];
  end
  if meta["table-source"] then
      table_source = meta["table-source"];
  end
  if meta["listings-source"] then
      listings_source = meta["listings-source"];
  end
  return meta
end

-- add source to figures
function Figure(element)
    -- if source is not set then don't modify the figure
    if figure_source == nil then
        return element
    end

    -- check if figure has a caption
    if element.caption and element.caption.long and element.caption.long[1] then
        -- check if figure already contains source
        for _, val in ipairs(element.caption.long[1].content) do
            -- if it contains source return original element
            if val.citations then
               return element
            end
        end

        -- add source to the end
        table.insert(element.caption.long[1].content, pandoc.Space())
        for _, val in ipairs(figure_source) do
            table.insert(element.caption.long[1].content, val)
        end
    end

    -- return modified element
    return element
end

-- add source to tables
function Table(element)
    -- if source is not set then don't modify the table
    if table_source == nil then
        return element
    end

    -- check if table has a caption
    if element.caption and element.caption.long and element.caption.long[1] then
        -- check if table already contains source
        for _, val in ipairs(element.caption.long[1].content) do
            -- if it contains source return original element
            if val.citations then
               return element
            end
        end

        -- add source to the end
        table.insert(element.caption.long[1].content, pandoc.Space())
        for _, val in ipairs(table_source) do
            table.insert(element.caption.long[1].content, val)
        end
    end

    -- return modified element
    return element
end

function Div(element)
    -- if source is not set then don't modify the div
    if listings_source == nil then
        return element
    end

    -- check if div is code listing
    if element.content and element.content[1] and element.content[1].tag == "RawBlock" and string.find(element.content[1].text, "codelisting") then
        -- check if div is has caption
        if element.content[2] and element.content[2].tag == "Plain" and element.content[2].content[2] and element.content[2].content[2].tag == "Span" then
            -- check if table already contains source
            for _, val in ipairs(element.content[2].content[2].content) do
                -- if it contains source return original element
                if val.citations then
                    return element
                end
            end

            -- add source to the end
            table.insert(element.content[2].content[2].content, pandoc.Space())
            for _, val in ipairs(listings_source) do
                table.insert(element.content[2].content[2].content, val)
            end
        end
    end

    -- return modified element
    return element
end

return {
  {Meta = Meta},
  {Figure = Figure},
  {Table = Table},
  {Div = Div}
}
