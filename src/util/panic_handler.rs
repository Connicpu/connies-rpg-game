use backtrace::Backtrace;
use msgbox::{self, IconType};

use std::panic;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

pub fn init() {
    panic::set_hook(Box::new(|info| display_panic(info)));
}

fn trim_path(path: &Path) -> String {
    if let Some(component) = path.components()
        .filter_map(|c| c.as_os_str().to_str())
        .filter(|c| c.starts_with("<"))
        .nth(0)
    {
        return component.into();
    }

    let mut any = false;
    let mut base = PathBuf::new();
    let mut components = path.components();
    let mut krate = OsStr::new("");
    while let Some(component) = components.next() {
        if component.as_os_str() == ".cargo" {
            components.next();
            components.next();
            base.push(".cargo");
            base.push("registry");
            base.push("src");
        } else if component.as_os_str() == "src" {
            any = true;
            break;
        } else {
            krate = component.as_os_str();
            base.push(component.as_os_str());
        }
    }

    if any {
        if let Ok(stripped) = path.strip_prefix(&base) {
            return format!(
                "{}: {}",
                krate.to_string_lossy(),
                stripped.to_string_lossy()
            );
        }
    }

    path.to_string_lossy().into_owned()
}

fn display_panic(info: &panic::PanicInfo) {
    let backtrace = Backtrace::new();

    let backtrace: String = backtrace
        .frames()
        .iter()
        .flat_map(|frame| {
            if frame.symbols().len() == 0 {
                return vec![format!("Unresolved symbol {:?}", frame.symbol_address())].into_iter();
            }

            frame
                .symbols()
                .iter()
                .filter_map(|symbol| {
                    match (
                        symbol.name(),
                        symbol.filename(),
                        symbol.lineno(),
                        symbol.addr(),
                    ) {
                        (Some(name), Some(file), Some(line), _) => Some(format!(
                            "fn {}(...)\n    in '{}' at line {}",
                            name,
                            trim_path(file),
                            line
                        )),
                        (None, Some(file), Some(line), _) => {
                            Some(format!("fn in '{}' at line {}", trim_path(file), line))
                        }
                        (Some(name), Some(file), None, _) => {
                            Some(format!("fn {}(...)\n    in '{}'", name, trim_path(file)))
                        }
                        (Some(name), None, None, _) => Some(format!("fn {}(...)", name)),
                        (_, _, _, Some(addr)) => Some(format!("Unresolved symbol {:?}", addr)),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<Vec<_>>()
        .join("\n");

    let locinfo = if let Some(loc) = info.location() {
        format!(
            "Panic occurred in '{}' at line {}",
            trim_path(Path::new(loc.file())),
            loc.line()
        )
    } else {
        "Panic ocurred at an unknown location".to_string()
    };

    let content = if let Some(s) = info.payload().downcast_ref::<&'static str>() {
        s.to_string()
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        s.clone()
    } else {
        format!("Unknown error type: {:?}", info.payload().get_type_id())
    };

    let text = format!("{}\n{}\n\n{}", locinfo, content, backtrace);

    eprintln!("{}", text);

    msgbox::create("The game has encountered a panic", &text, IconType::ERROR);
}
