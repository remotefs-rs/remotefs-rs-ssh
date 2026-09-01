# Run all unit, integration, binary, and documentation tests.
[group('test')]
test args="":
  cargo test --lib {{ args }}
  cargo test --doc {{ args }}

# Generate an LCOV coverage report.
[group('test')]
coverage output="lcov.info" args="":
  cargo llvm-cov --lib --lcov --output-path {{ output }} {{ args }}

# Run benchmarks for one backend feature.
[group('test')]
bench feature="libssh2":
  cargo bench --bench {{ feature }} --features {{ feature }}
