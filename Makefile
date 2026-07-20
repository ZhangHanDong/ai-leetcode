.PHONY: book serve clean

book:
	mdbook build book

serve:
	mdbook serve book --open

clean:
	mdbook clean book

