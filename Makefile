dev:
	docker-compose up -d

dev-down:
	docker-compose down

new-migrate:
	sqlx migrate add -r $(name)

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

server:
	cargo watch -q -c -w src/ -x run

install:
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add async-trait
	cargo add serde -F derive
	cargo add chrono -F serde
	cargo add futures-util
	cargo add env_logger
	cargo add dotenv
	cargo add uuid -F "serde v4"
	cargo add sqlx -F "tls-native-tls runtime-async-std postgres chrono uuid"
	cargo add jsonwebtoken
	cargo add argon2
	cargo add validator -F derive
