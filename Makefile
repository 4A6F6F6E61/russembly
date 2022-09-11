c = wasm-pack
russembly = /mnt/c/Users/zocke/IdeaProjects/russembly
next = /mnt/c/Users/zocke/OneDrive/Documents/next

compile_to_wasm: src/lib.rs
	$(c) build --target web
	rm -rf "$(next)/lib/wasm_bindgen/*"
	cp --recursive pkg/* "$(next)/lib/wasm_bindgen"
	rm -f "$(next)/lib/wasm_bindgen/.gitignore"