# Zola inline shortcodes
${widgets.commandButton("System: Reload")}

```space-lua
local static_dir = "void/room09-chaotic-library/static"

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
  startMarker = "\\{%.*?%\\}",
  endMarker = "\\{%\\s*end\\s*%\\}",
  mode = "block",
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

```
No start before
(?<!start(\s|\S)*)
Start before but ending
(?<=start(?:\s|\S)*?end(?:[^e]*|(?!end)\s)*)

Start after
No start after
(?!(?:\s|\S)*start)
Start after but ending
(?=(?:\s|\S)*start(?:\s|\S)*end)

Temp:
start1#start2#end2#end1#end3

(?:(?<!(?<ns>start[0-9]\S*))|(?<=(?<nse>(?:(?<!(?<nse_ns>start[0-9]\S*)))start[0-9]\S*end[0-9]\S*)))end[0-9]
```