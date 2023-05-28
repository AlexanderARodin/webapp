binname="webapp"


help:
	@echo 'there is no help.. yet'

edit:
	@nvim ./src/root_app/domik_view.rs
edit_main:
	@nvim ./src/main.rs
edit_domik:
	@nvim ./src/root_app.rs

savetogit:
	@git add . && git commit -m 'saving' && git push

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
	@cargo test

size:
	@ls -lAh ./target/release/$(binname)

clean:
	@cargo clean
