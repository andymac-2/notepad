.PHONY: serve build serve_pe_book

serve:
	bundle exec jekyll serve

build:
	cd _pe_book_src && mdbook build
	rm -r pe_book
	mv _pe_book_src/book pe_book

serve_pe_book:
	cd _pe_book_src && mdbook serve

