build::
	wasm-pack build --scope wgslr

publish: build
	wasm-pack publish -a public --tag dev 

