# gmsv_fs

filesystem extension for garry's mod servers using [cap-std](https://github.com/bytecodealliance/cap-std) for sandboxing and additional safety.

## installation

place `gmsv_fs_<os>.dll` in `garrysmod/lua/bin/`

## basic usage

```lua
require("fs")

-- read file
local content = fs.read("data/myfile.txt")

-- write file
fs.write("data/hello.txt", "Hello, World!")

-- append to file
fs.append("data/log.txt", "New line\n")

-- check if file/directory exists
if fs.isfile("data/config.json") then
  print("Config exists!")
end

if fs.isdir("data/backups") then
  print("Backup folder exists!")
end

-- scan directory
local files = fs.scan("data/")
for _, info in ipairs(files) do
  print(info.name, info.is_dir)
end

-- create directory
fs.mkdir("data/cache", true) -- recursive
fs.mkdir("data/cache") -- will error if data is not already created

-- delete file/directory
fs.rm("data/old.txt")
fs.rmdir("data/temp") -- REMOVES ENTIRE DIRECTORY WITH ALL OF ITS CONTENTS
fs.rmfile("data/file.txt")

-- move
fs.mv("old.txt", "new.txt")

-- copy
fs.cp("source.txt", "dest.txt")
fs.cp("source_dir/", "dest_dir/") -- copies directories & files inside recursively

-- path utilities
local path = "data\\folder\\file.txt"
print(fs.forward(path))  -- "data/folder/file.txt"
print(fs.backward(path)) -- "data\\folder\\file.txt"

print(fs.join("data", "folder", "file.txt")) -- "data/folder/file.txt"
print(fs.extname("file.txt")) -- ".txt"
print(fs.filename("data/file.txt")) -- "file.txt"
print(fs.dirname("data/file.txt")) -- "data"

-- sanitize filename (removes invalid characters)
print(fs.sanitize("file<name>.txt")) -- "file_name_.txt"

-- get canonical path
print(fs.canonical("."))

-- check if path is within directory
print(fs.within("data/", "data/subfolder/file.txt")) -- true

-- get file metadata
local meta = fs.metadata("data/file.txt")
print(meta.size, meta.modified, meta.is_dir)
```

## api reference

| function | description |
|----------|-------------|
| `fs.read(path)` | read entire file contents as string |
| `fs.write(path, data)` | write data to file (overwrites) |
| `fs.append(path, data)` | append data to file |
| `fs.isfile(path)` | check if path is a file |
| `fs.isdir(path)` | check if path is a directory |
| `fs.readable(path)` | check if file is writable |
| `fs.scan(path)` | list directory contents with metadata |
| `fs.mkdir(path, recursive?)` | create directory |
| `fs.rmdir(path)` | remove empty directory |
| `fs.rmfile(path)` | remove file |
| `fs.rm(path)` | remove file or directory |
| `fs.mv(from, to)` | ,ove/rename file or directory |
| `fs.cp(from, to)` | copy file or directory |
| `fs.forward(path)` | convert backslashes to forward slashes |
| `fs.backward(path)` | convert forward slashes to backslashes |
| `fs.join(...)` | join path components |
| `fs.extname(path, replace?)` | get/set file extension |
| `fs.filename(path, replace?)` | get/set filename |
| `fs.dirname(path, replace?)` | get/set parent directory name |
| `fs.sanitize(path)` | remove invalid path characters |
| `fs.canonical(path)` | get absolute canonical path |
| `fs.within(root, path)` | check if path is within root directory |
| `fs.metadata(path)` | get file metadata (size, timestamps) |

## credits

- [interstellar/fs](https://blueshank-gh.github.io/interstellar-docs/fs/)
- [cap-std](https://github.com/bytecodealliance/cap-std)
- [gmodx-rs](https://github.com/Srlion/gmodx-rs)
