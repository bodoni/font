all: tests-unit

tests: tests-unit tests-draw tests-scan tests-sign

tests-draw:
	cargo run --bin draw --features drawing --quiet -- \
		--font ../font/tests/fixtures/AdobeBlank-Regular.ttf \
		--character "a" > assets/AdobeBlank-Regular.svg
	cargo run --bin draw --features drawing --quiet -- \
		--font ../font/tests/fixtures/Numans-Regular.ttf \
		--character "a" > assets/Numans-Regular.svg
	cargo run --bin draw --features drawing --quiet -- \
		--font ../font/tests/fixtures/OpenSans-Italic.ttf \
		--character "&" > assets/OpenSans-Italic.svg
	cargo run --bin draw --features drawing --quiet -- \
		--font ../font/tests/fixtures/SourceSerifPro-Regular.otf \
		--character "ö" > assets/SourceSerifPro-Regular.svg
	cargo run --bin draw --features drawing --quiet -- \
		--font ../font/tests/fixtures/VesperLibre-Regular.ttf \
		--character "å" > assets/VesperLibre-Regular.svg
	[ "$$(git diff assets | wc -l | xargs)" = 0 ] || exit 1

tests-scan:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	RUST_BACKTRACE=full cargo run --bin scan --features scanning -- \
		--path tests/fixtures \
		--ignore bungeecolor \
		--ignore bungeespice \
		--ignore gruppo \
		--ignore iceland \
		--ignore kaushanscript \
		--ignore ubuntu \
		--workers 4

tests-sign:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	rm -rf target/signatures && mkdir -p target/signatures
	RUST_BACKTRACE=full cargo run --bin sign --features drawing,scanning -- \
		--input tests/fixtures \
		--output target/signatures \
		--ignore bungeecolor \
		--ignore bungeespice \
		--ignore gruppo \
		--ignore iceland \
		--ignore kaushanscript \
		--ignore ubuntu \
		--workers 4

tests-unit:
	cargo test

.PHONY: tests tests-draw tests-scan tests-sign tests-unit
