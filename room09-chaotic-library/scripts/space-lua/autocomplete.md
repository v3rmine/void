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
      where p.name:match("^void/room09%-chaotic%-library/static/media/emojis/[^/]+$")
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

    