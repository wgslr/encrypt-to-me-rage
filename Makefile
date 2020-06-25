publish:
	wasm-pack build --scope wgslr
	wasm-pack publish -a public --tag dev 

