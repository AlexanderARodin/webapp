binname="webapp"


help:
	@echo 'there is no help.. yet'

edit:
	@nvim .
editview:
	@nvim ./src/domik_view.rs
editapp:
	@nvim ./src/main_app.rs
editmain:
	@nvim ./src/main.rs

savetogit:
	@git add . && git commit -m 'saving' && git push
pull:
	@git pull

release:
	@cargo rustc --release -- -C prefer-dynamic
run: release size
	@cargo run --release

trunk-release:
	@trunk build --release

serve:
	@trunk serve


all: release trunk-release savetogit


test:
	@cargo test -- --show-output

size:
	@ls -lAh ./target/release/$(binname)

clean:
	@cargo clean
	@trunk clean
