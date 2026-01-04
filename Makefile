.PHONY: fuzz pbt test clean

# Property-Based Testing (proptest)
pbt:
	cargo test --test test_proptest

# Fuzzing (cargo-fuzz)
fuzz:
	cd fuzz && cargo +nightly fuzz run fuzz_target_1

# Run all tests including PBT
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
	cd fuzz && cargo clean
