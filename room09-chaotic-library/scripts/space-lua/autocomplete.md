# Complete medias
```space-lua
event.listen {
  name = "editor:complete",
  run = function(e)
    local linePrefix = e.data.linePrefix or ""
    local pos = e.data.pos or 0

    local match_prefix = "/media/"
    local match = linePrefix:match(match_prefix.."([a-zA-Z0-9_]*)$")
    if not match then
      return
    end
  
    local media = query[[ 
      from p = index.documents()
      where p.name:startsWith("void/room09-chaotic-library/static/media/")
    ]]
    local opts = {}
  
    for _, p in ipairs(media) do
      local label = p.ref:gsub("^void/room09%-chaotic%-library/static/media/", "")
      local completionText = p.ref:gsub("^void/room09%-chaotic%-library/static", "")
  
      table.insert(opts, {
        label = label,
        detail = p.ref,
        apply = function()
          editor.dispatch({
            changes = {
              from = pos - match:len() - match_prefix:len(),
              to = pos,
              insert = completionText
            }
          })
        end
      })
    end

    return {
      from = pos - string.len(match),
      options = opts
    }
  end
}
```

# Complete @ links
```space-lua
event.listen {
  name = "editor:complete",
  run = function(e)
    local linePrefix = e.data.linePrefix or ""
    local pos = e.data.pos or 0

    local match_prefix = "@/"
    local match = linePrefix:match(match_prefix.."([a-zA-Z0-9_]*)$")
    if not match then
      return
    end
  
    local media = query[[ 
      from p = index.pages()
      where p.name:startsWith("void/room09-chaotic-library/content/")
    ]]
    local opts = {}
  
    for _, p in ipairs(media) do
      local label = p.ref:gsub("^void/room09%-chaotic%-library/content/", "")
      label = label..".md"
      local completionText = p.ref:gsub("^void/room09%-chaotic%-library/content/", "")
      completionText = completionText..".md"
  
      table.insert(opts, {
        label = label,
        detail = p.ref,
        apply = function()
          editor.dispatch({
            changes = {
              from = pos - match:len(),
              to = pos,
              insert = completionText
            }
          })
        end
      })
    end

    return {
      from = pos - string.len(match),
      options = opts
    }
  end
}
```


# Complete customoji
```space-lua
event.listen {
  name = "editor:complete",
  run = function(e)
    local linePrefix = e.data.linePrefix or ""
    local pos = e.data.pos or 0

    local match = linePrefix:match("customoji%(name=\"([a-zA-Z0-9_]*)$")
    if not match then
      return
    end
  
    local media = query[[ 
      from p = index.documents()
      where p.name:match("^void/room09%-chaotic%-library/static/media/emojis/[^/]+$") or p.name:match("^void/room09%-chaotic%-library/static/media/emojis/personas/[^/]+/[^/]+$")
    ]]
    local opts = {}
  
    for _, p in ipairs(media) do
      local label = p.ref:gsub("^void/room09%-chaotic%-library/static/media/emojis/", "")
      label = label:gsub("%.[^.]+$", "")
      local completionText = p.ref:gsub("^void/room09%-chaotic%-library/static/media/emojis/", "")
      completionText = completionText:gsub("%.[^.]+$", "")
  
      table.insert(opts, {
        label = label,
        detail = p.ref,
        apply = function()
          editor.dispatch({
            changes = {
              from = pos - match:len(),
              to = pos,
              insert = completionText
            }
          })
        end
      })
    end

    return {
      from = pos - string.len(match),
      options = opts
    }
  end
}
```

# Complete say
## who
```space-lua
event.listen {
  name = "editor:complete",
  run = function(e)
    local linePrefix = e.data.linePrefix or ""
    local pos = e.data.pos or 0

    local match = linePrefix:match("say%([^)]*who=\"([a-zA-Z0-9_]*)$")
    if not match then
      return
    end
  
    local media = query[[ 
      from p = index.documents()
      where p.name:startsWith("void/room09-chaotic-library/static/media/emojis/personas/")
      select ""..string.gsub(p.name:gsub("^void/room09%-chaotic%-library/static/media/emojis/personas/", ""), "^([^/]+)/.*$", "%1")
    ]]
    local opts = {}
  
    for _, p in ipairs(media) do
      local label = p
      local completionText = p
  
      table.insert(opts, {
        label = label,
        detail = p,
        apply = function()
          editor.dispatch({
            changes = {
              from = pos - match:len(),
              to = pos,
              insert = completionText
            }
          })
        end
      })
    end

    return {
      from = pos - string.len(match),
      options = opts
    }
  end
}
```

## what
```space-lua
event.listen {
  name = "editor:complete",
  run = function(e)
    local linePrefix = e.data.linePrefix or ""
    local pos = e.data.pos or 0

    local match = linePrefix:match("say%([^)]*what=\"([a-zA-Z0-9_]*)$")
    if not match then
      return
    end
  
    local media = query[[ 
      from p = index.documents()
      where p.name:match("^void/room09%-chaotic%-library/static/media/emojis/personas/[^/]+/[^/]+$")
    ]]
    local opts = {}
  
    for _, p in ipairs(media) do
      local label = p.ref:gsub("^void/room09%-chaotic%-library/static/media/emojis/personas/[^/]+/", "")
      label = label:gsub("%.[^.]+$", "")
      local completionText = p.ref:gsub("^void/room09%-chaotic%-library/static/media/emojis/personas/[^/]+/", "")
      completionText = completionText:gsub("%.[^.]+$", "")
  
      table.insert(opts, {
        label = label,
        detail = p.ref,
        apply = function()
          editor.dispatch({
            changes = {
              from = pos - match:len(),
              to = pos,
              insert = completionText
            }
          })
        end
      })
    end

    return {
      from = pos - string.len(match),
      options = opts
    }
  end
}
```

# Complete autolink
```space-lua
local toml = js.import("https://esm.sh/toml")
local autolink_path = "void/room09-chaotic-library/content/autolink.toml"
local raw_autolink_content = space.readFile(autolink_path)
-- Convert bytes to array
local autolink_content_arr = {}
for i, val in ipairs(raw_autolink_content) do
  autolink_content_arr[i] = val
end
local autolink_content = string.char(table.unpack(autolink_content_arr))
local autolink = js.tolua(toml.parse(autolink_content))

event.listen {
  name = "editor:complete",
  run = function(e)
    local linePrefix = e.data.linePrefix or ""
    local pos = e.data.pos or 0

    local match = linePrefix:match("al%(k=\"([a-zA-Z0-9_]*)$")
    if not match then
      return
    end

    local keys = {}
    for url, a_keys in pairs(autolink) do
      for _, key in pairs(a_keys) do
        keys[key] = url
      end
    end
    
    local opts = {}
    
    for key, url in pairs(keys) do
      local label = key
      local completionText = key
      js.log(key)

      table.insert(opts, {
        label = label,
        detail = url,
        apply = function()
          editor.dispatch({
            changes = {
              from = pos - match:len(),
              to = pos,
              insert = completionText
            }
          })
        end
      })
    end

    return {
      from = pos - string.len(match),
      options = opts
    }
  end
}
```
