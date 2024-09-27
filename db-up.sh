docker container run -d --name rust-gazer-db \
-p 3306:3306 -v './schema/schema.sql:/docker-entrypoint-initdb.d/init.sql' \
-e "MARIADB_ROOT_PASSWORD=password" \
-e "MARIADB_DATABASE=mariadb" \
-e "MARIADB_USER=user" \
-e "MARIADB_PASSWORD=password" \
mariadb:11.5
