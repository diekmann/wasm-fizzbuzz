all:
	$(MAKE) -C wat
	$(MAKE) -C rust
	$(MAKE) -C doom

gh-pages: all
	mkdir -p ./public
	cp -r ./wat ./public/
	cp -r ./rust ./public/
	# doom is a bit bigger, so we only copy the final artifacts.
	mkdir -p ./public/doom
	cp -a ./doom/index.html ./public/doom/
	cp -a ./doom/main.js ./public/doom/
	cp -a ./doom/doom.wasm ./public/doom/

clean:
	rm -rf ./public

