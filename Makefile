postgres-up:
	if [ ! -d "_local/pg_data" ]; then mkdir _local/pg_data; fi
	docker compose -f _local/docker-compose.yml up -d 

postgres-down:
	docker stop postgres-local
	docker stop adminer-local