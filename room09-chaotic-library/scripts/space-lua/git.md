# Fetch void updates
```space-lua
command.define {
  name = "Git: Fetch void",
  run = function()
    local result = shell.run('sh', {"-c", 'cd void && git pull'})
    print("Output:", result.stdout)
    print("Error:", result.stderr)
    print("Exit code:", result.code)
    if result.code == 0 then
      editor.flashNotification "Successfully fetched repo"
    else
      editor.flashNotification("Error fetching repo: " .. result.stderr)
    end
  end
}
```

# Push void updates
```space-lua
command.define {
  name = "Git: Push void",
  run = function()
    local result = shell.run('sh', {"-c", 'cd void && git push'})
    print("Output:", result.stdout)
    print("Error:", result.stderr)
    print("Exit code:", result.code)
    if result.code == 0 then
      editor.flashNotification "Successfully pushed repo"
    else
      editor.flashNotification("Error pushing repo: " .. result.stderr)
    end
  end
}
```

# Void status
```space-lua
command.define {
  name = "Git: Status void",
  run = function()
    local result = shell.run('sh', {"-c", 'cd void && git status'})
    print("Output:", result.stdout)
    print("Error:", result.stderr)
    print("Exit code:", result.code)
    if result.code == 0 then
      editor.alert(result.stdout)
    else
      editor.flashNotification("Error getting repo status: " .. result.stderr)
    end
  end
}
```

# Commit void updates
```space-lua
command.define {
  name = "Git: Commit void",
  run = function()
    local commit_message = editor.prompt("Enter a commit desc:")
    if commit_message == nil then
      return
    end
    
    local result = shell.run('sh', {"-c", 'cd void && git add . && git commit -m "feat(room09-chaotic-library): silverbullet: ' .. commit_message .. '"'})
    print("Output:", result.stdout)
    print("Error:", result.stderr)
    print("Exit code:", result.code)
    if result.code == 0 then
      editor.flashNotification "Successfully commited repo"
    else
      editor.flashNotification("Error commiting repo: " .. result.stderr)
    end
  end
}
```