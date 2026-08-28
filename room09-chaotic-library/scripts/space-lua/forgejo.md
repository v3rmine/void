# Deploy website clearnet only
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Forgejo: Deploy website (clearnet only)",
  run = function()
    if not editor.confirm("Are you sure you want to deploy the website?") then
      return
    end
    
    local result = net.proxyFetch("https://git.v3rm.in/api/v1/repos/v3rmine/void/actions/workflows/personal-website.yaml/dispatches", {
      method = "POST",
      body = {
        inputs = {
          ["only-clearnet"] = "true",
        },
        ref = "refs/heads/main"
      },
      headers = {
        ["Accept"] = "application/json",        
        ["Content-Type"] = "application/json",
        Authorization = "token "..config.get("forgejo.token", "UNSET"),
        ["User-Agent"] = "silverbullet/v3rmine",
      }
    })
    
    if result.status == 204 then
      editor.flashNotification("Successfully started website deployment", "info")
    elseif result.status > 400 then
      editor.flashNotification("Shit Happened: "..result.body.message, "error")
    else
      editor.flashNotification("Unhandled code: "..result.status, "warning")
    end
  end
}
```


# Deploy website clearnet + tor
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Forgejo: Deploy website (tor + clearnet)",
  run = function()
    if not editor.confirm("Are you sure you want to deploy the website?") then
      return
    end
    
    local result = net.proxyFetch("https://git.v3rm.in/api/v1/repos/v3rmine/void/actions/workflows/personal-website.yaml/dispatches", {
      method = "POST",
      body = {
        inputs = {
          ["only-clearnet"] = "false",
        },
        ref = "refs/heads/main"
      },
      headers = {
        ["Accept"] = "application/json",        
        ["Content-Type"] = "application/json",
        Authorization = "token "..config.get("forgejo.token", "UNSET"),
        ["User-Agent"] = "silverbullet/v3rmine",
      }
    })
    
    if result.status == 204 then
      editor.flashNotification("Successfully started website deployment", "info")
    elseif result.status > 400 then
      editor.flashNotification("Shit Happened: "..result.body.message, "error")
    else
      editor.flashNotification("Unhandled code: "..result.status, "warning")
    end
  end
}
```