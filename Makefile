unit-tests:
	@cargo test --lib -- --show-output

integration-tests:
	@cargo test --test init_db -- --show-output
	@cargo test --test integration_tests -- --show-output
