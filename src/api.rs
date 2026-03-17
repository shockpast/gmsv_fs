use std::{io::{Read, Write}, path::{Path, PathBuf}, time::SystemTime};

use gmodx::lua;
use cap_std::{ambient_authority, fs::{Dir, OpenOptions}};

//
fn io_err(e: std::io::Error) -> lua::Error {
    lua::Error::Runtime(e.to_string().into())
}

fn copy_dir(root: &Dir, from: &String, to: &String) -> lua::Result<()> {
    root.create_dir_all(to).map_err(io_err)?;

    let src_dir = root.open_dir(from).map_err(io_err)?;
    for entry in src_dir.entries().map_err(io_err)? {
        let entry = entry.map_err(io_err)?;
        let name = entry.file_name().to_string_lossy().to_string();
        let meta = entry.metadata().map_err(io_err)?;

        let src_path = format!("{}/{}", from, name);
        let dst_path = format!("{}/{}", to, name);

        if meta.is_dir() {
            copy_dir(root, &src_path, &dst_path)?;
        } else {
            root.copy(&src_path, root, &dst_path).map_err(io_err)?;
        }
    }

    Ok(())
}

//
fn fs_read(_: &lua::State, path: lua::String) -> lua::Result<lua::String> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    let mut file = root.open_with(&path.to_string(), OpenOptions::new().create(true)).map_err(io_err)?;

    let mut buf = String::new();

    file.read_to_string(&mut buf).map_err(io_err)?;

    Ok(buf.into())
}

fn fs_write(_: &lua::State, path: lua::String, data: lua::String) -> lua::Result<()> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    let mut file = root.open_with(&path.to_string(), OpenOptions::new().create(true)).map_err(io_err)?;

    file.write_all(&data).map_err(io_err)?;

    Ok(())
}

fn fs_append(_: &lua::State, path: lua::String, data: lua::String) -> lua::Result<()> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    let mut file = root.open_with(&path.to_string(), OpenOptions::new().create(true).append(true)).map_err(io_err)?;

    file.write_all(&data).map_err(io_err)?;

    Ok(())
}

fn fs_isfile(_: &lua::State, path: lua::String) -> lua::Result<bool> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    Ok(root.is_file(&path.to_string()))
}

fn fs_isdir(_: &lua::State, path: lua::String) -> lua::Result<bool> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    Ok(root.is_dir(&path.to_string()))
}

fn fs_readable(_: &lua::State, path: lua::String) -> lua::Result<bool> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    let meta = root.metadata(&path.to_string()).map_err(io_err)?;

    Ok(!meta.permissions().readonly())
}

fn fs_scan(state: &lua::State, path: lua::String) -> lua::Result<lua::Table> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    let dir = root.open_dir(&path.to_string()).map_err(io_err)?;
    let entries = dir.entries().map_err(io_err)?;

    let table = state.create_table();
    let mut i = 1;

    for entry in entries {
        let entry = entry.map_err(io_err)?;
        let meta = entry.metadata().map_err(io_err)?;

        let created = meta.created().ok()
            .map(|s| s.into_std())
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let modified = meta.modified().ok()
            .map(|s| s.into_std())
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let accessed = meta.accessed().ok()
            .map(|s| s.into_std())
            .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let t = state.create_table();
        t.raw_set(state, "name", entry.file_name().to_string_lossy().to_string());
        t.raw_set(state, "is_dir", meta.is_dir());
        t.raw_set(state, "is_file", meta.is_file());
        t.raw_set(state, "creation_time", created);
        t.raw_set(state, "modified", modified);
        t.raw_set(state, "accessed", accessed);

        table.raw_set(state, i, t);
        i += 1;
    }

    Ok(table)
}

fn fs_mkdir(_: &lua::State, path: lua::String, recursive: Option<bool>) -> lua::Result<()> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    let path = &path.to_string();

    if recursive.unwrap_or(false) {
        root.create_dir_all(path).map_err(io_err)?;
    } else {
        root.create_dir(path).map_err(io_err)?;
    }

    Ok(())
}

fn fs_rmdir(_: &lua::State, path: lua::String) -> lua::Result<()> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    let path = &path.to_string();

    root.remove_dir_all(path).map_err(io_err)?;

    Ok(())
}

fn fs_rmfile(_: &lua::State, path: lua::String) -> lua::Result<()> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    let path = &path.to_string();

    root.remove_file(path).map_err(io_err)?;

    Ok(())
}

fn fs_rm(_: &lua::State, path: lua::String) -> lua::Result<()> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;

    let path = &path.to_string();
    let meta = root.metadata(path).map_err(io_err)?;

    if meta.is_dir() {
        root.remove_dir_all(path).map_err(io_err)?;
    } else {
        root.remove_file(path).map_err(io_err)?;
    }

    Ok(())
}

fn fs_mv(_: &lua::State, from: lua::String, to: lua::String) -> lua::Result<bool> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    root.rename(&from.to_string(), &root, &to.to_string()).map_err(io_err)?;

    Ok(true)
}

fn fs_cp(_: &lua::State, from: lua::String, to: lua::String) -> lua::Result<bool> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;

    let from = &from.to_string();
    let to = &to.to_string();

    let meta = root.metadata(from).map_err(io_err)?;
    if meta.is_dir() {
        copy_dir(&root, from, to)?;
    } else {
        root.copy(from, &root, to).map_err(io_err)?;
    }

    Ok(true)
}

fn fs_forward(_: &lua::State, path: lua::String) -> lua::Result<String> {
    Ok(path.to_string().replace('\\', "/"))
}

fn fs_backward(_: &lua::State, path: lua::String) -> lua::Result<String> {
    Ok(path.to_string().replace('/', "\\"))
}

fn fs_join(_: &lua::State, a: lua::String, b: lua::String) -> lua::Result<String> {
    let path = Path::new(&a.to_string()).join(b.to_string());
    Ok(path.to_string_lossy().to_string())
}

fn fs_extname(_: &lua::State, path: lua::String, replace: Option<lua::String>) -> lua::Result<String> {
    let p = PathBuf::from(path.to_string());

    match replace {
        Some(ext) => {
            Ok(p.with_extension(ext.to_string()).to_string_lossy().to_string())
        },
        None => {
            Ok(p.extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default())
        },
    }
}

fn fs_filename(_: &lua::State, path: lua::String, replace: Option<lua::String>) -> lua::Result<String> {
    let p = PathBuf::from(path.to_string());

    match replace {
        Some(name) => {
            Ok(p.with_file_name(name.to_string()).to_string_lossy().to_string())
        },
        None => {
            Ok(p.file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default())
        },
    }
}

fn fs_dirname(_: &lua::State, path: lua::String, replace: Option<lua::String>) -> lua::Result<String> {
    let p = PathBuf::from(path.to_string());
    let parent = p.parent().unwrap_or(Path::new(""));

    match replace {
        Some(name) => {
            Ok(parent.with_file_name(name.to_string()).to_string_lossy().to_string())
        },
        None => {
            Ok(parent.to_string_lossy().to_string())
        },
    }
}

fn fs_sanitize(_: &lua::State, path: lua::String) -> lua::Result<String> {
    let sanitized = path.to_string().chars().map(|c| {
        match c {
            '<' | '>' | ':' | '"' | '|' | '?' | '*' | '\0' => '_',
            c => c,
        }
    }).collect();

    Ok(sanitized)
}

fn fs_canonical(_: &lua::State, path: lua::String) -> lua::Result<String> {
    let root = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    let canonical = root.canonicalize(path.to_string()).map_err(io_err)?;

    Ok(canonical.to_string_lossy().to_string())
}

fn fs_within(_: &lua::State, root: lua::String, path: lua::String) -> lua::Result<bool> {
    let dir = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;

    let root = dir.canonicalize(root.to_string()).map_err(io_err)?;
    let path = dir.canonicalize(path.to_string()).map_err(io_err)?;

    Ok(path.starts_with(&root))
}

//
fn fs_metadata(state: &lua::State, path: lua::String) -> lua::Result<lua::Table> {
    let path = &path.to_string();

    let dir = Dir::open_ambient_dir(".", ambient_authority()).map_err(io_err)?;
    let file = dir.open(path).map_err(io_err)?;
    let meta = file.metadata().map_err(io_err)?;

    let created = meta.created().ok()
        .map(|s| s.into_std())
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let modified = meta.modified().ok()
        .map(|s| s.into_std())
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let accessed = meta.accessed().ok()
        .map(|s| s.into_std())
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let table = state.create_table();
    table.raw_set(state, "is_dir", meta.is_dir());
    table.raw_set(state, "is_file", meta.is_file());
    table.raw_set(state, "size", meta.len());
    table.raw_set(state, "creation_time", created);
    table.raw_set(state, "modified", modified);
    table.raw_set(state, "accessed", accessed);

    Ok(table)
}

//
pub fn on_gmod_open(state: &lua::State, table: &lua::Table) {
    table.raw_set(state, c"read", state.create_function(fs_read));
    table.raw_set(state, c"write", state.create_function(fs_write));
    table.raw_set(state, c"append", state.create_function(fs_append));
    table.raw_set(state, c"isfile", state.create_function(fs_isfile));
    table.raw_set(state, c"isdir", state.create_function(fs_isdir));
    table.raw_set(state, c"readable", state.create_function(fs_readable));
    table.raw_set(state, c"scan", state.create_function(fs_scan));
    table.raw_set(state, c"mkdir", state.create_function(fs_mkdir));
    table.raw_set(state, c"rmdir", state.create_function(fs_rmdir));
    table.raw_set(state, c"rmfile", state.create_function(fs_rmfile));
    table.raw_set(state, c"rm", state.create_function(fs_rm));
    table.raw_set(state, c"mv", state.create_function(fs_mv));
    table.raw_set(state, c"cp", state.create_function(fs_cp));
    table.raw_set(state, c"forward", state.create_function(fs_forward));
    table.raw_set(state, c"backward", state.create_function(fs_backward));
    table.raw_set(state, c"join", state.create_function(fs_join));
    table.raw_set(state, c"extname", state.create_function(fs_extname));
    table.raw_set(state, c"filename", state.create_function(fs_filename));
    table.raw_set(state, c"dirname", state.create_function(fs_dirname));
    table.raw_set(state, c"sanitize", state.create_function(fs_sanitize));
    table.raw_set(state, c"canonical", state.create_function(fs_canonical));
    table.raw_set(state, c"within", state.create_function(fs_within));
    table.raw_set(state, c"metadata", state.create_function(fs_metadata));
}