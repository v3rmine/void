# Linkding insert links
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Linkding: Insert links",
  run = function()
    local dateFns = js.import("https://esm.sh/date-fns")
    local dateFnsTz = js.import("https://esm.sh/@date-fns/tz")
    local nLastDays = editor.prompt("Since how much days?", "7")
    local previousSunday = dateFns.subDays(os.date("%Y-%m-%d"), tonumber(nLastDays))
    local date = dateFns.formatISO(previousSunday, {
      ["in"] = dateFnsTz.tz("Utc")
    })
    print(date)
    local bookmarks = net.proxyFetch("https://links.astriiid.fr/api/bookmarks/?q=%23note-hebdo&added_since="..date, {
      headers = {
        Authorization = "Token "..config.get("linkding.token", "UNSET")
      }
    })

    local markdown = ""
    for bookmark in bookmarks.body.results.reverse() do
      print(bookmark)
      markdown = markdown .. "- ["..bookmark.title.."]("..bookmark.url..") : " .. bookmark.description .. "\n"
    end

    editor.insertAtCursor(markdown)
  end
}
```



