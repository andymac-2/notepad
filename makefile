.PHONY: serve build serve_pe_book

serve:
	bundle exec jekyll serve

build:
	cd _pe_book_src && mdbook build
	rm -rf pe_book
	mv _pe_book_src/book pe_book
	cd _ksp_book_src && mdbook build
	rm -rf ksp_book
	mv _ksp_book_src/book ksp_book

serve_pe_book:
	cd _pe_book_src && mdbook serve

