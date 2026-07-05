use std::path::{Path, PathBuf};

pub struct FileEntry {
    pub relative_path: String,
    pub absolute_path: PathBuf,
    pub size: u64,
}

pub fn scan_project(root: &Path) -> Result<Vec<FileEntry>, Box<dyn std::error::Error>> {
    let mut entries = Vec::new();

    let mut builder = ignore::WalkBuilder::new(root);
    builder
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .hidden(true)
        .ignore(true)
        .require_git(false)
        .sort_by_file_name(|a, b| a.cmp(b));

    for result in builder.build() {
        let entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };

        if !entry.file_type().map_or(false, |ft| ft.is_file()) {
            continue;
        }

        let path = entry.path();
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);

        let rel_path = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        if size > 500_000 {
            continue;
        }

        entries.push(FileEntry {
            relative_path: rel_path,
            absolute_path: path.to_path_buf(),
            size,
        });
    }

    Ok(entries)
}

pub fn read_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(path)?)
}

#[allow(dead_code)]
pub fn read_file_lines(
    path: &Path,
    start: usize,
    end: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();
    let start = start.min(lines.len());
    let end = end.min(lines.len());
    Ok(lines[start..end].join("\n"))
}

pub fn search_codebase(
    root: &Path,
    pattern: &str,
) -> Result<Vec<(String, usize, String)>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    let entries = scan_project(root)?;

    for entry in &entries {
        let content = match std::fs::read_to_string(&entry.absolute_path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for (i, line) in content.lines().enumerate() {
            if line.contains(pattern) {
                if results.len() < 50 {
                    results.push((
                        entry.relative_path.clone(),
                        i + 1,
                        line.trim().to_string(),
                    ));
                }
            }
        }
    }

    Ok(results)
}

#[allow(dead_code)]
pub fn compute_file_hash(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    use std::hash::Hasher;
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.write(&buf[..n]);
    }
    Ok(format!("{:016x}", hasher.finish()))
}
