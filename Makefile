all:
	$(MAKE) -C wat
	$(MAKE) -C rust

gh-pages: all
	mkdir -p ./public
	cp -r ./wat ./public/
	cp -r ./rust ./public/

clean:
	rm -rf ./public

