# Insert UUID
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Editor: Insert UUID",
  run = function()
    editor.insertAtCursor(js.tolua(js.window.crypto.randomUUID()))
  end
}
```

# Insert Today
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Editor: Insert Today",
  run = function()
    editor.insertAtCursor(os.date("%Y-%m-%d"))
  end
}
```

# Toggle plain text
${widgets.commandButton("System: Reload")}

```space-lua
if editor.isMobile() then
  editor.invokeCommand("Editor: Toggle Markdown Syntax Rendering")
end

actionButton.define {
  icon = "eye",
  run = function()
    editor.invokeCommand("Editor: Toggle Markdown Syntax Rendering")
  end
}
```
