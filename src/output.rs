use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikiMeta {
    pub file_hashes: HashMap<String, String>,
}

pub fn load_wiki_meta(codewiki_dir: &Path) -> WikiMeta {
    let meta_path = codewiki_dir.join(".codewiki.json");
    match std::fs::read_to_string(&meta_path) {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(meta) => meta,
            Err(_) => WikiMeta {
                file_hashes: HashMap::new(),
            },
        },
        Err(_) => WikiMeta {
            file_hashes: HashMap::new(),
        },
    }
}

pub fn save_wiki_meta(codewiki_dir: &Path, meta: &WikiMeta) {
    let meta_path = codewiki_dir.join(".codewiki.json");
    if let Ok(json) = serde_json::to_string_pretty(meta) {
        let _ = std::fs::write(meta_path, json);
    }
}

pub fn write_doc(codewiki_dir: &Path, relative_path: &str, content: &str) -> PathBuf {
    let full_path = codewiki_dir.join(relative_path);
    if let Some(parent) = full_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(&full_path, content);
    full_path
}

pub fn append_agents_reference(project_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let agents_path = project_dir.join("AGENTS.md");
    let reference = "\n\n<!-- codewiki:start -->\n## codewiki Documentation\n\nThis repository has codewiki-generated documentation in the `codewiki/` directory.\nWhen you need context about the codebase, reference the files in `codewiki/`:\n- `codewiki/index.md` — Project overview\n- `codewiki/architecture.md` — Architecture and design\n\nYou can also use the codewiki CLI to update documentation:\n```bash\ncodewiki --update\n```\n<!-- codewiki:end -->\n";

    let existing = if agents_path.exists() {
        std::fs::read_to_string(&agents_path).unwrap_or_default()
    } else {
        String::new()
    };

    let start_marker = "<!-- codewiki:start -->";
    let end_marker = "<!-- codewiki:end -->";

    let new_content = if let (Some(start), Some(end)) =
        (existing.find(start_marker), existing.find(end_marker))
    {
        format!(
            "{}{}{}",
            &existing[..start],
            reference.trim(),
            &existing[end + end_marker.len()..]
        )
    } else {
        format!(
            "{}<!-- codewiki:start -->\n## codewiki Documentation\n\nThis repository has codewiki-generated documentation in the `codewiki/` directory.\nWhen you need context about the codebase, reference the files in `codewiki/`:\n- `codewiki/index.md` — Project overview\n- `codewiki/architecture.md` — Architecture and design\n\nYou can also use the codewiki CLI to update documentation:\n```bash\ncodewiki --update\n```\n<!-- codewiki:end -->\n",
            existing
        )
    };

    std::fs::write(&agents_path, new_content)?;
    Ok(())
}
