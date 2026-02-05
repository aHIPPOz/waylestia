// Build script for waylestia-widgets
// Compiles .proto files if needed

fn main() {
    // Proto compilation will be integrated later
    // For now, we use JSON-based IPC
    println!("cargo:rerun-if-changed=src/");
}
