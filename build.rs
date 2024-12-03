const COMMANDS: &[&str] = &[
    "vm_open",
    "vm_close",
    "vm_exec",
    "vm_eval_module",
    "vm_typings",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
