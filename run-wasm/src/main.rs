fn main() {
    cargo_run_wasm::run_wasm_with_css(
        "body { margin: 0px; } canvas { width: 100%; height: 100%; background-color:black;}",
    )
}
