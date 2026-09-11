```space-lua
virtualPage.define {
  pattern = "oauth:mastodon:(.+)",
  run = function(code)
    editor.copyToClipboard(code)
    editor.flashNotification("Code copied you can close the page", "info")
    return "# Mastodon Oauth Page\nCode:`"..code.."`"
  end
}
```

```space-lua
function get_mastodon_settings()
  local frontmatter = editor.getCurrentPageMeta()
  local domain = config.get("mastodon.domain")
  local id = nil
  if frontmatter and frontmatter.extra then
    if frontmatter.extra.comments then
      domain = frontmatter.extra.comments.host
      id = frontmatter.extra.comments.id
    end
  end

  return {
    base_url = "https://"..domain,
    domain = domain,
    id = id,
    config = config.get({ "mastodon", domain })
  }
end

function upload_file_to_mastodon(file, base_url, access_token, alt)
  local file_name = file.meta.name:gsub(".*/([^/]+)", "%1")

  local form_border = "------------------------border"
  local parts = {
    "--"..form_border.."\r\n",
    "Content-Disposition: form-data; name=\"file\"; filename=\""..file_name.."\"\r\n",
    "Content-Type: "..file.meta.contentType.."\r\n\r\n",
    file.data,
    "\r\n"
  }

  print(file_name, alt)
  if alt then
    table.insert(parts, "--"..form_border.."\r\n")
    table.insert(parts, "Content-Disposition: form-data; name=\"description\"\r\n\r\n")
    table.insert(parts, alt)
    table.insert(parts, "\r\n")
  end
  
  table.insert(parts, "--"..form_border.."--\r\n")
  
  local form = js.new(js.window.Blob, parts)
  
  local result = net.proxyFetch(base_url.."/api/v2/media", {
    method = "POST",
    headers = {
      ["Content-Type"] = "multipart/form-data; boundary="..form_border,
      ["Authorization"] = "Bearer "..access_token,
    },
    body = form.bytes()
  })

  return result
end
```

# Mastodon Register app
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Mastodon: Register app",
  run = function()
    local mastodon_settings = get_mastodon_settings()
    local base_url = mastodon_settings.base_url
    local mastodon_domain = mastodon_settings.domain

    if config.has({ "mastodon", mastodon_domain, "client_key" }) then
      editor.flashNotification("App already registered", "warning")
      return
    end
    
    local result = net.proxyFetch(base_url.."/api/v1/apps", {
      method = "POST",
      headers = {
        ["Accept"] = "application/json",
        ["Content-Type"] = "application/json",
      },
      body = {
        client_name = "Silverbullet",
        redirect_uris = "https://notes-oauth.astriiid.fr/oauth:mastodon",
        scopes = "write:statuses write:media"
      }
    })
    
    if result.status == 200 then
      editor.flashNotification("Successfully registered app in mastodon you can paste it in CONFIG to persist it", "info")
      config.set("mastodon", {
        [mastodon_domain] = {
          client_key = result.body.client_id,
          client_secret = result.body.client_secret
        }
      })
      editor.copyToClipboard("[\""..mastodon_domain.."\"] = { client_key = \""..result.body.client_id.."\", client_secret = \""..result.body.client_secret.."\" },")
      editor.navigate("CONFIG")
    elseif result.status > 400 then
      editor.flashNotification("Shit Happened: "..result.body.error_description, "error")
    else
      editor.flashNotification("Unhandled code: "..result.status, "warning")
    end
  end
}
```

# Mastodon Authenticate app
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Mastodon: Auth",
  run = function()
    local mastodon_settings = get_mastodon_settings()
    local base_url = mastodon_settings.base_url
    local mastodon_domain = mastodon_settings.domain
    local mastodon_config = mastodon_settings.config
    if not mastodon_config then
      editor.flashNotification("App not registered please register it with 'Mastodon: Register app'", "error")
      return
    end
    
    js.window.open(base_url.."/oauth/authorize?client_id="..mastodon_config.client_key.."&scope=write:statuses+write:media&redirect_uri=https://notes-oauth.astriiid.fr/oauth:mastodon&response_type=code", nil, "popup")
    local authorization_code = editor.prompt("Authorization Code?")
    
    local result = net.proxyFetch(base_url.."/oauth/token", {
      method = "POST",
      headers = {
        ["Accept"] = "application/json",
        ["Content-Type"] = "application/json",
      },
      body = {
        grant_type = "authorization_code",
        client_id = mastodon_config.client_key,
        client_secret = mastodon_config.client_secret,
        redirect_uri = "https://notes-oauth.astriiid.fr/oauth:mastodon",
        code = authorization_code,
      }
    })
    
    if result.status == 200 then
      editor.flashNotification("Successfully logged in mastodon you can paste it in CONFIG to persist it", "info")
      config.set({ "mastodon", mastodon_domain, "access_token" }, result.body.access_token)
      editor.copyToClipboard("access_token = \""..result.body.access_token.."\"")
      editor.navigate("CONFIG")
    elseif result.status > 400 then
      editor.flashNotification("Shit Happened: "..result.body.error_description, "error")
    else
      editor.flashNotification("Unhandled code: "..result.status, "warning")
    end
  end
}
```


# Mastodon new post for blog
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Mastodon: New post for blog",
  run = function()
    local mastodon_settings = get_mastodon_settings()
    local base_url = mastodon_settings.base_url
    local mastodon_config = mastodon_settings.config
    if not mastodon_config then
      editor.flashNotification("App not registered please register it with 'Mastodon: Register app'", "error")
      return
    end
    
    local comments_id = mastodon_config.id
    if comments_id and string.len(comments_id) > 0 then
      editor.flashNotification("There's already a post linked to this note", "warning")
      return
    end
    
    if not mastodon_config.access_token then
      editor.flashNotification("Not authenticated with Mastodon please use 'Mastodon: Auth'", "error")
      return
    end

    local status = index.extractFrontmatter(editor.getText(), { 
      removeFrontMatterSection = true 
    }).text
    status = status:gsub(" *{{ *image%([^)]+%) *}} *", "")
    
    local visibility = editor.filterBox("Status visibility:", {
      { name = "Public", value = "public" },
      { name = "Non listé", value = "unlisted" },
      { name = "Followers seulement", value = "private" },
      { name = "Privé", value = "direct" }
    }).value
    if not visibility then
      visibility = "unlisted"
    end

    local lang = editor.filterBox("Language:", {
      { name = "Français", value = "fr" },
      { name = "Anglais", value = "en" }
    }).value

    local in_reply_to_id = nil
    if editor.getCurrentPage().startsWith("void/room09-chaotic-library/content/thoughts/") then
      local thoughts = query[[
        from p = index.pages()
        where 
          p.name.startsWith("void/room09-chaotic-library/content/thoughts/") 
          and p.extra 
          and p.extra.comments 
          and p.extra.comments.id
          and string.len(p.extra.comments.id) > 0
      ]]
      if thoughts[1] then
        local options = {}
        for thought in thoughts do
          table.insert(options, {
            name = thought.title,
            description = thought.name,
            value = thought.extra.comments.id
          })
        end
        in_reply_to_id = editor.filterBox("In reply to:", options).value
      end
    end

    if not editor.confirm("Are you sure you want to post:"..status) then
      return
    end

    local images = {}
    for image in index.extractFrontmatter(editor.getText(), { removeFrontMatterSection = true }).text:gmatch("{{ ?image%([^)]+%) ?}}") do 
      local match_image = string.matchRegex(image, 'url="(?<url>.*?)"(?:, *alt="(?<alt>.*?)")?')
    
      if not match_image then
        goto continue
      end
      
      local image_path = match_image.groups.url
      local image_alt = match_image.groups.alt
      table.insert(images, { path = image_path, alt = image_alt})
      
      ::continue::
    end
    
    media_ids = {}
    for image in images do
      local file = space.readFileWithMeta("void/room09-chaotic-library/static"..image.path)
      
      local result = upload_file_to_mastodon(file, base_url, mastodon_config.access_token, image.alt)
      print(result)
      
      if result.status == 200 then
          table.insert(media_ids, result.body.id) 
        elseif result.status > 400 then
          editor.flashNotification("Shit Happened while uploading image: "..result.body.error, "error")
        else
          editor.flashNotification("Unhandled code while uploading image: "..result.status, "warning")
        end
    end

    print({
      status = status,
      visibility = visibility,
      language = lang,
      media_ids = media_ids,
      in_reply_to_id = in_reply_to_id,
    })
    
    local result = net.proxyFetch(base_url.."/api/v1/statuses", {
      method = "POST",
      headers = {
        ["Accept"] = "application/json",
        ["Content-Type"] = "application/json",
        ["Authorization"] = "Bearer "..mastodon_config.access_token,
      },
      body = {
        status = status,
        visibility = visibility,
        language = lang,
        media_ids = media_ids,
        in_reply_to_id = in_reply_to_id,
      }
    })
    print(result)
    
    if result.status == 200 then
      editor.flashNotification("Post successfully published", "info")
      local patched_text = editor.getText():gsub("(comments.*id: *)''", "%1'"..result.body.id.."'")
      editor.setText(patched_text)
    elseif result.status > 400 then
      editor.flashNotification("Shit Happened: "..result.body.error_description, "error")
    else
      editor.flashNotification("Unhandled code: "..result.status, "warning")
    end
  end
}
```

# Mastodon delete post for blog
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Mastodon: Delete post for blog",
  run = function()
    local mastodon_settings = get_mastodon_settings()
    local base_url = mastodon_settings.base_url
    local mastodon_config = mastodon_settings.config
    if not mastodon_config then
      editor.flashNotification("App not registered please register it with 'Mastodon: Register app'", "error")
      return
    end
    
    local comments_id = mastodon_config.id
    if not comments_id or string.len(comments_id) == 0 then
      editor.flashNotification("There's no post linked to this note", "warning")
      return
    end
    
    if not mastodon_config.access_token then
      editor.flashNotification("Not authenticated with Mastodon please use 'Mastodon: Auth'", "error")
      return
    end
    
    local result = net.proxyFetch(base_url.."/api/v1/statuses/"..comments_id.."?delete_media=true", {
      method = "DELETE",
      headers = {
        ["Accept"] = "application/json",
        ["Content-Type"] = "application/json",
        ["Authorization"] = "Bearer "..mastodon_config.access_token,
      },
      body = {}
    })
    print(result)
    
    if result.status == 200 then
      editor.flashNotification("Post successfully deleted", "info")
      local patched_text = editor.getText():gsub("(comments.*id: *)'[^']*'", "%1''")
      editor.setText(patched_text)
    elseif result.status > 400 then
      editor.flashNotification("Shit Happened: "..result.body.error, "error")
    else
      editor.flashNotification("Unhandled code: "..result.status, "warning")
    end
  end
}
```

# Mastodon new note post
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Mastodon: New note post",
  run = function()
    local mastodon_settings = get_mastodon_settings()
    local base_url = mastodon_settings.base_url
    local mastodon_config = mastodon_settings.config
    if not mastodon_config then
      editor.flashNotification("App not registered please register it with 'Mastodon: Register app'", "error")
      return
    end
    
    local frontmatter = editor.getCurrentPageMeta()
    local comments_id = mastodon_config.id
    if comments_id and string.len(comments_id) > 0 then
      editor.flashNotification("There's already a post linked to this note", "warning")
      return
    end
    
    if not mastodon_config.access_token then
      editor.flashNotification("Not authenticated with Mastodon please use 'Mastodon: Auth'", "error")
      return
    end

    local post_url = "https://astriiid.fr/notes/"..editor.getCurrentPage():gsub("^.*/([^/]+)$", "%1").."/"
    local status = frontmatter.title .." : "..frontmatter.description.."\n\n"..post_url.."\n\n".."#noteshebdo"

    if not editor.confirm("Are you sure you want to post:"..status) then
      return
    end
    
    local result = net.proxyFetch(base_url.."/api/v1/statuses", {
      method = "POST",
      headers = {
        ["Accept"] = "application/json",
        ["Content-Type"] = "application/json",
        ["Authorization"] = "Bearer "..mastodon_config.access_token,
      },
      body = {
        status = status,
        visibility = "public",
        language = "fr",
      }
    })
    print(result)
    
    if result.status == 200 then
      editor.flashNotification("Post successfully published", "info")
      local patched_text = editor.getText():gsub("(comments.*id: *)''", "%1'"..result.body.id.."'")
      editor.setText(patched_text)
    elseif result.status > 400 then
      editor.flashNotification("Shit Happened: "..result.body.error_description, "error")
    else
      editor.flashNotification("Unhandled code: "..result.status, "warning")
    end
  end
}
```

# Mastodon new post
${widgets.commandButton("System: Reload")}

```space-lua
command.define {
  name = "Mastodon: New free post",
  run = function()
    local mastodon_settings = get_mastodon_settings()
    local base_url = mastodon_settings.base_url
    local mastodon_config = mastodon_settings.config
    if not mastodon_config then
      editor.flashNotification("App not registered please register it with 'Mastodon: Register app'", "error")
      return
    end
    
    if not mastodon_config.access_token then
      editor.flashNotification("Not authenticated with Mastodon please use 'Mastodon: Auth'", "error")
      return
    end

    local status = editor.prompt("What do you wanna say?")
    
    local visibility = editor.filterBox("Status visibility:", {
      { name = "Public", value = "public" },
      { name = "Non listé", value = "unlisted" },
      { name = "Followers seulement", value = "private" },
      { name = "Privé", value = "direct" }
    })
    if not visibility then
      visibility = "unlisted"
    end

    local lang = editor.filterBox("Language:", {
      { name = "Français", value = "fr" },
      { name = "Anglais", value = "en" }
    })
    
    local result = net.proxyFetch(base_url.."/api/v1/statuses", {
      method = "POST",
      headers = {
        ["Accept"] = "application/json",
        ["Content-Type"] = "application/json",
        ["Authorization"] = "Bearer "..mastodon_config.access_token,
      },
      body = {
        status = status,
        visibility = visibility,
        language = lang,
      }
    })
    print(result)
    
    if result.status == 200 then
      editor.flashNotification("Post successfully published", "info")
    elseif result.status > 400 then
      editor.flashNotification("Shit Happened: "..result.body.error_description, "error")
    else
      editor.flashNotification("Unhandled code: "..result.status, "warning")
    end
  end
}
```


