# Zola inline shortcodes
${widgets.commandButton("System: Reload")}

```space-lua
local static_dir = "void/room09-chaotic-library/static"

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

function parse_inline_shortcode(body)
  local render = dom.i { body }

  -- Parse shortcode {{ url(link="") }}
  local match_url = string.matchRegex(body, 'url\\(link="(?<url>.*?)"\\)')
  if match_url != nil then
    local url = match_url.groups.url
    render = dom.a {
      href = url,
      target = "_blank",
      url
    }
  end

  -- Parse shortcode {{ customoji(name="") }}
  local match_customoji = string.matchRegex(body, 'customoji\\(name="(?<name>.*?)"[^)]*\\)')
  if match_customoji != nil then
    local name = match_customoji.groups.name

    local file = static_dir.."/media/emojis/"..name..".gif"
    if not space.fileExists(file) then
      file = static_dir.."/media/emojis/"..name..".png"
    end
    if not space.fileExists(file) then
      file = static_dir.."/media/emojis/personas/"..name..".gif"
    end

    if space.fileExists(file) then
      render = dom.img {
        class = "sb-blog-customoji-inline",
        src = "/.fs/"..file
      }
    else
      print("Emoji not found: " .. file)
    end
  end

  -- Parse shortcode {{ image(url="") }}
  local match_image = string.matchRegex(body, 'image\\(url="(?<url>.*?)"[^)]*\\)')
  if match_image != nil then
    local url = match_image.groups.url

    local file = static_dir..url
    if space.fileExists(file) then
      render = dom.img {
        class = "sb-blog-image",
        src = "/.fs/"..file
      }
    else
      print("Image not found: " .. file)
    end
  end
  
  -- Parse shortcode {{ al(k="") }}
  local match_al = string.matchRegex(body, 'al\\(k="(?<key>.*?)"[^)]*\\)')
  if match_al != nil then
    local key = match_al.groups.key

    for url, keys in pairs(autolink) do
      if table.includes(keys, key) then
        render = dom.a {
          href = url,
          target = "_blank",
          key
        }
        break;
      end
    end
  end
  
  return {
    render = render,
  }
end

syntax.define {
  name = "ZolaShortcodeInline",
  startMarker = "\\{\\{\\s*",
  endMarker = "\\s*\\}\\}",
  mode = "inline",
  startMarkerClass = "sb-zola-shortcode-marker",
  --bodyClass = "",
  endMarkerClass = "sb-zola-shortcode-marker",
  --renderClass = "",
  renderWidget = function(body, pageName)
    local parsed = parse_inline_shortcode(body)
    return widget.html(parsed.render)
  end,
  renderHtml = function(body, pageName)
    local parsed = parse_inline_shortcode(body)
    return parsed.render
  end
}
```

```space-style
.sb-zola-shortcode-marker {
  color: grey;
}
#sb-main .cm-editor .sb-lua-directive-inline:has(.sb-blog-customoji-inline),
#sb-main .cm-editor .sb-lua-directive-inline:has(.sb-blog-image){
  border: none;
  padding: 0;
}
/* Customoji */
.sb-blog-customoji-inline {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  height: 30px;
}
#sb-main .cm-editor .sb-lua-directive-inline:has(.sb-blog-customoji-inline) {
  display: inline-block;
  width: 30px;
  height: 1em;
  min-height: unset;
}
/* Image */
.sb-blog-image {
  max-height: 500px;
  max-width: 100%;
}
```

# Zola block shortcodes
${widgets.commandButton("System: Reload")}

```lua
syntax.define {
  name = "ZolaShortcodeBlock",
  startMarker = "{%\\s*(?!end\\s*%)[^%]+%}",
  endMarker = "(?:(?!{%)[\\s\\S])*(?:{%\\s*(?!end\\s*%)[^%]+%}(?:(?!{%)[\\s\\S])*(?:{%\\s*(?!end\\s*%)[^%]+%}(?:(?!{%)[\\s\\S])*(?:{%\\s*(?!end\\s*%)[^%]+%}(?:(?!{%)[\\s\\S])*{%\\s*end\\s*%}(?:(?!{%)[\\s\\S])*)*{%\\s*end\\s*%}(?:(?!{%)[\\s\\S])*)*{%\\s*end\\s*%}(?:(?!{%)[\\s\\S])*)*({%\\s*end\\s*%})",
  mode = "block",
  startMarkerClass = "sb-zola-shortcode-marker",
  --bodyClass = "",
  endMarkerClass = "sb-zola-shortcode-marker",
  --renderClass = "",
  renderWidget = function(body, pageName)
    return widget.html(dom.i { body })
  end,
  renderHtml = function(body, pageName)
    return dom.i { body }
  end
}
```

{% say(who="astrid", what="Sweat", rev=1) %}
{% xxx %}
Bon ça allait pas super fort non plus.  
Toujours pas de bonne piste de taff et le changement de prénom toujours au point mort... Mais après une bonne pause lecture et **loin de l'ordi** ça a l'air d'aller un peu mieux
{% end %}
{% end %}

## Workaround block shortcodes
${widgets.commandButton("System: Reload")}

```space-lua
local static_dir = "void/room09-chaotic-library/static"

function parse_block_shortcode(body)
  local render = dom.i { body }

  -- Parse end
  local match_end = string.matchRegex(body, '^end$')
  if match_end != nil then
    render = dom.span {
      class = "sb-zola-shortcode-marker",
      "--- end ---"
    }
  end

  -- Parse wild only
  local match_end = string.matchRegex(body, '^wild_only\\(\\)$')
  if match_end != nil then
    render = dom.span {
      class = "sb-zola-shortcode-marker",
      "--- wild only ---"
    }
  end

  -- Parse say
  local match_say = string.matchRegex(body, 'say\\(who="(?<who>.*?)"(?:,\\s*what="(?<what>.*?)")?[^)]*\\)')
  if match_say != nil then
    local who = match_say.groups.who
    local what = match_say.groups.what
    if what == nil then
      what = "default"
    end

    local file = static_dir.."/media/emojis/personas/"..who.."/"..what..".gif"
    if space.fileExists(file) then
      render = dom.img {
        class = "sb-blog-say",
        src = "/.fs/"..file
      }
    else
      print("Persona "..who.."'s reaction not found: "..what)
    end
  end
  
  return {
    render = render,
  }
end

syntax.define {
  name = "ZolaShortcodeWorkaroundBlock",
  startMarker = "\\{%\\s*",
  endMarker = "\\s*%\\}",
  mode = "inline",
  startMarkerClass = "sb-zola-shortcode-marker",
  --bodyClass = "",
  endMarkerClass = "sb-zola-shortcode-marker",
  --renderClass = "",
  renderWidget = function(body, pageName)
    local parsed = parse_block_shortcode(body)
    return widget.html(parsed.render)
  end,
  renderHtml = function(body, pageName)
    local parsed = parse_block_shortcode(body)
    return parsed.render
  end
}
```

```space-style
#sb-main .cm-editor .sb-lua-directive-inline:has(.sb-blog-say) {
  border: none;
  padding: 0;
}
/* Say */
.sb-blog-say {
  height: 100%;
}
#sb-main .cm-editor .sb-lua-directive-inline:has(.sb-blog-say) {
  display: inline-block;
  width: 60px;
  height: 60px;
  min-height: unset;
  border: 1px solid gray;
  background-color: lightgray;
  border-radius: 9999rem;
  overflow: hidden;
}
```