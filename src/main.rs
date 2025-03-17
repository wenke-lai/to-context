use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

struct ExcludePattern {
    pattern: &'static str,
    is_wildcard: bool,
}

fn get_exclude_patterns() -> Vec<ExcludePattern> {
    vec![
        ExcludePattern {
            pattern: ".git",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: "node_modules",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: "package-lock.json",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: "target",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: ".idea",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: ".vscode",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: "dist",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: "build",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: ".DS_Store",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: "Thumbs.db",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: "uv.lock",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: ".venv",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: ".next",
            is_wildcard: false,
        },
        ExcludePattern {
            pattern: "*.pyc",
            is_wildcard: true,
        },
        ExcludePattern {
            pattern: "*.class",
            is_wildcard: true,
        },
        ExcludePattern {
            pattern: "*.o",
            is_wildcard: true,
        },
        ExcludePattern {
            pattern: "*.so",
            is_wildcard: true,
        },
        ExcludePattern {
            pattern: "*.dll",
            is_wildcard: true,
        },
        ExcludePattern {
            pattern: "*.exe",
            is_wildcard: true,
        },
        ExcludePattern {
            pattern: "*.jar",
            is_wildcard: true,
        },
        ExcludePattern {
            pattern: "*.zip",
            is_wildcard: true,
        },
        ExcludePattern {
            pattern: "*.tar.gz",
            is_wildcard: true,
        },
        ExcludePattern {
            pattern: "*.log",
            is_wildcard: true,
        },
    ]
}

fn should_exclude(path: &Path, patterns: &[ExcludePattern]) -> bool {
    if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
        for pattern in patterns {
            if pattern.is_wildcard {
                if let Some(wildcard_pos) = pattern.pattern.find('*') {
                    let suffix = &pattern.pattern[wildcard_pos + 1..];
                    if suffix.is_empty() || filename.ends_with(suffix) {
                        return true;
                    }
                }
            } else {
                if filename == pattern.pattern {
                    return true;
                }
            }
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("用法: {} <資料夾路徑>", args[0]);
        std::process::exit(1);
    }

    let dir_path = &args[1];
    let path = Path::new(dir_path);

    if !path.is_dir() {
        eprintln!("錯誤: '{}' 不是一個資料夾", dir_path);
        std::process::exit(1);
    }

    let exclude_patterns = get_exclude_patterns();

    let folder_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");

    let output_filename = format!("{}.context.txt", folder_name);
    let mut output_file = File::create(&output_filename)?;

    collect_files_content(path, path, &mut output_file, &exclude_patterns)?;

    println!("成功創建檔案: {}", output_filename);

    Ok(())
}

fn collect_files_content(
    base_dir: &Path,
    current_dir: &Path,
    output: &mut File,
    exclude_patterns: &[ExcludePattern],
) -> io::Result<()> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if should_exclude(&path, exclude_patterns) {
            println!("排除路徑: {}", path.display());
            continue;
        }

        if path.is_file() {
            let relative_path = path
                .strip_prefix(base_dir)
                .unwrap_or(&path)
                .to_string_lossy();

            writeln!(output, "# {}", relative_path)?;

            match fs::read_to_string(&path) {
                Ok(content) => {
                    writeln!(output, "{}", content)?;
                    writeln!(output)?;
                }
                Err(e) => {
                    writeln!(output, "[無法讀取檔案: {}]", e)?;
                    writeln!(output)?;
                }
            }
        } else if path.is_dir() {
            collect_files_content(base_dir, &path, output, exclude_patterns)?;
        }
    }

    Ok(())
}

