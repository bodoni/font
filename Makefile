all: tests-unit

tests: tests-unit tests-draw-selected tests-sign-selected tests-scan tests-sign

tests-unit:
	cargo test

tests-draw-selected:
	cargo run --bin draw --features drawing --quiet -- \
		--font tests/fixtures/selected-fonts/AdobeBlank-Regular.ttf \
		--character "a" > assets/draw/AdobeBlank-Regular.svg
	cargo run --bin draw --features drawing --quiet -- \
		--font tests/fixtures/selected-fonts/CrimsonText-Regular.ttf \
		--character "Q" > assets/draw/CrimsonText-Regular.svg
	cargo run --bin draw --features drawing --quiet -- \
		--font tests/fixtures/selected-fonts/Numans-Regular.ttf \
		--character "a" > assets/draw/Numans-Regular.svg
	cargo run --bin draw --features drawing --quiet -- \
		--font tests/fixtures/selected-fonts/OpenSans-Italic.ttf \
		--character "&" > assets/draw/OpenSans-Italic.svg
	cargo run --bin draw --features drawing --quiet -- \
		--font tests/fixtures/selected-fonts/SourceSerifPro-Regular.otf \
		--character "ö" > assets/draw/SourceSerifPro-Regular.svg
	cargo run --bin draw --features drawing --quiet -- \
		--font tests/fixtures/selected-fonts/VesperLibre-Regular.ttf \
		--character "å" > assets/draw/VesperLibre-Regular.svg
	[ "$$(git diff assets/draw | wc -l | xargs)" = 0 ] || exit 1

tests-sign-selected:
	RUST_BACKTRACE=full cargo run --bin sign --features drawing,scanning -- \
		--input tests/fixtures/selected-fonts \
		--output assets/sign \
		--characters anop
	[ "$$(git diff assets/sign | wc -l | xargs)" = 0 ] || exit 1

tests-scan:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	RUST_BACKTRACE=full cargo run --bin scan --features scanning -- \
		--path tests/fixtures \
		--ignore google-fonts/ofl/bungeecolor \
		--ignore google-fonts/ofl/bungeespice \
		--ignore google-fonts/ofl/gruppo \
		--ignore google-fonts/ofl/iceland \
		--ignore google-fonts/ofl/kaushanscript \
		--ignore google-fonts/ufl/ubuntu \
		--ignore web-platform-tests/css/WOFF2/support/SFNT-CFF-Fallback \
		--ignore web-platform-tests/css/WOFF2/support/SFNT-CFF-Reference \
		--ignore web-platform-tests/css/css-fonts/support/fonts/FontWithFancyFeatures \
		--ignore web-platform-tests/css/css-fonts/support/fonts/FontWithFeatures2 \
		--ignore web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup1 \
		--ignore web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup3 \
		--workers 4

tests-sign:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	RUST_BACKTRACE=full cargo run --bin sign --features drawing,scanning -- \
		--input tests/fixtures \
		--output assets/sign \
		--characters anop \
		--ignore google-fonts/ofl/bungeecolor \
		--ignore google-fonts/ofl/bungeespice \
		--ignore google-fonts/ofl/gruppo \
		--ignore google-fonts/ofl/iceland \
		--ignore google-fonts/ofl/kaushanscript \
		--ignore google-fonts/ufl/ubuntu \
		--ignore web-platform-tests/css/WOFF2/support/SFNT-CFF-Fallback \
		--ignore web-platform-tests/css/WOFF2/support/SFNT-CFF-Reference \
		--ignore web-platform-tests/css/css-fonts/support/fonts/FontWithFancyFeatures \
		--ignore web-platform-tests/css/css-fonts/support/fonts/FontWithFeatures2 \
		--ignore web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup1 \
		--ignore web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup3 \
		--ignore web-platform-tests/css/css-writing-modes/support/tcu-font \
		--ignore web-platform-tests/fonts/adobe-fonts/CSSFWOrientationTest \
		--ignore web-platform-tests/fonts/adobe-fonts/CSSHWOrientationTest \
		--workers 4

.PHONY: tests
.PHONY: tests-unit
.PHONY: tests-draw-selected tests-sign-selected
.PHONY: tests-scan tests-sign
