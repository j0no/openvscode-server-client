#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, WindowBuilder, WindowUrl};


fn main() {
  fn custom_menu(name: &str) -> CustomMenuItem {
    let c = CustomMenuItem::new(name.to_string(), name);
    return c;
  }
  let menu = Menu::new()
  .add_submenu(Submenu::new(
    "opencode-server-client",
    Menu::new()
      // .add_native_item(MenuItem::About("Mr Tagger".to_string()))
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Services)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Hide)
      .add_native_item(MenuItem::HideOthers)
      .add_native_item(MenuItem::ShowAll)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Quit),
  ))
  .add_submenu(Submenu::new(
    "File",
    Menu::new()
      .add_item(custom_menu("Open...").accelerator("cmdOrControl+O"))
      .add_native_item(MenuItem::Separator)
      .add_item(custom_menu("Close").accelerator("cmdOrControl+W"))
      .add_item(custom_menu("Save").accelerator("cmdOrControl+S"))
      .add_item(custom_menu("Save As...").accelerator("shift+cmdOrControl+S")),
  ))
  .add_submenu(Submenu::new("Edit", {
    let mut menu = Menu::new();
    menu = menu.add_native_item(MenuItem::Undo);
    menu = menu.add_native_item(MenuItem::Redo);
    menu = menu.add_native_item(MenuItem::Separator);
    menu = menu.add_native_item(MenuItem::Cut);
    menu = menu.add_native_item(MenuItem::Copy);
    menu = menu.add_native_item(MenuItem::Paste);
    #[cfg(not(target_os = "macos"))]
    {
      menu = menu.add_native_item(MenuItem::Separator);
    }
    menu = menu.add_native_item(MenuItem::SelectAll);
    menu
  }))
  .add_submenu(Submenu::new("View", Menu::new()))
  .add_submenu(Submenu::new(
    "Window",
    Menu::new()
      .add_native_item(MenuItem::Minimize)
      .add_native_item(MenuItem::Zoom),
  ))
  .add_submenu(Submenu::new(
    "Help",
    Menu::new().add_item(custom_menu("Learn More")),
  ))
  .add_native_item(MenuItem::Copy);

  tauri::Builder::default()
    .menu(menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
