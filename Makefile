run:
	cd ./engine && cargo watch -q -c -w src/ -x 'run -q'

test-watch:
	cd ./engine/engine_tester && cargo watch -q -c -w src/ -x 'run -q'

test:
	cd ./engine/engine_tester && cargo run