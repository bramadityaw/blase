use camino::{Utf8Path, Utf8PathBuf};

use crate::config::Config;

pub(crate) fn component_paths(name: &str, config: &Config) -> (Utf8PathBuf, Utf8PathBuf) {
    let work_path = &config.workspace_folder();
    let name = name.replace('.', std::path::MAIN_SEPARATOR_STR);
    let class_path = component_class_path(name.clone(), work_path);
    let resources_path = component_resources_path(name.clone(), work_path);
    (class_path, resources_path)
}

fn component_resources_path(path: String, work_path: &Utf8Path) -> Utf8PathBuf {
    work_path
        .join(component_views_dir())
        .join(path + ".blade.php")
}

fn component_class_path(path: String, work_path: &Utf8Path) -> Utf8PathBuf {
    let class_path = path
        .split(std::path::MAIN_SEPARATOR_STR)
        .map(|p| convert_case::ccase!(pascal, p))
        .collect::<Vec<_>>()
        .join(std::path::MAIN_SEPARATOR_STR);
    work_path
        .join(component_class_dir())
        .join(class_path + ".php")
}

pub(crate) fn layout_paths(name: &str, config: &Config) -> (Utf8PathBuf, Utf8PathBuf) {
    let work_path = &config.workspace_folder();
    let class_path = layout_class_path(name, work_path);
    let resources_path = layout_resources_path(name, work_path);
    (class_path, resources_path)
}

fn layout_class_path(name: &str, work_path: &Utf8Path) -> Utf8PathBuf {
    let name = format!("{}-layout", name);
    let layout_class_name = convert_case::ccase!(pascal, name);
    work_path
        .join(component_class_dir())
        .join(layout_class_name + ".php")
}

fn layout_resources_path(name: &str, work_path: &Utf8Path) -> Utf8PathBuf {
    let path = if name.is_empty() {
        work_path
            .join(component_views_dir())
            .join("layout.blade.php")
    } else {
        let layout_name = name.strip_suffix('-').unwrap();
        let path = Utf8PathBuf::from(views_dir())
            .join("layouts")
            .join(layout_name.to_string() + ".blade.php");
        work_path.join(path)
    };
    path
}

fn component_views_dir() -> String {
    views_dir() + std::path::MAIN_SEPARATOR_STR + "components"
}
fn views_dir() -> String {
    "resources/views".to_string()
}
fn component_class_dir() -> String {
    "app/View/Components".to_string()
}
