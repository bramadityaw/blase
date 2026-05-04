use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    config::Config,
    db::def::{ComponentName, LayoutName},
};

pub(crate) fn component_paths(name: &ComponentName, config: &Config) -> (Utf8PathBuf, Utf8PathBuf) {
    let work_path = &config.workspace_folder();
    let path = name.path();
    let class_path = component_class_path(path.clone(), work_path);
    let resources_path = component_resources_path(path.clone(), work_path);
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

pub(crate) fn layout_paths(name: LayoutName, config: &Config) -> (Utf8PathBuf, Utf8PathBuf) {
    let work_path = &config.workspace_folder();
    let class_path = layout_class_path(&name, work_path);
    let resources_path = layout_resources_path(&name, work_path);
    (class_path, resources_path)
}

fn layout_class_path(name: &LayoutName, work_path: &Utf8Path) -> Utf8PathBuf {
    let layout_class_name = name.class_name();
    work_path
        .join(component_class_dir())
        .join(layout_class_name + ".php")
}

fn layout_resources_path(name: &LayoutName, work_path: &Utf8Path) -> Utf8PathBuf {
    let path = match name {
        LayoutName::Default => work_path
            .join(component_views_dir())
            .join("layout.blade.php"),
        LayoutName::Name(name) => {
            let template_path = format!("{}.blade.php", name);
            let path = Utf8PathBuf::from(views_dir())
                .join("layouts")
                .join(template_path);
            work_path.join(path)
        }
    };
    path
}

pub fn component_views_dir() -> String {
    views_dir() + std::path::MAIN_SEPARATOR_STR + "components"
}

pub fn views_dir() -> String {
    "resources/views".to_string()
}

pub fn component_class_dir() -> String {
    "app/View/Components".to_string()
}
