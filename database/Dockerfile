FROM postgres:16.1

# Copy sql scripts
COPY ./types.sql /docker-entrypoint-initdb.d/01-types.sql
COPY ./tables.sql /docker-entrypoint-initdb.d/02-tables.sql
COPY ./inserts.sql /docker-entrypoint-initdb.d/03-inserts.sql
COPY ./views.sql /docker-entrypoint-initdb.d/04-views.sql
COPY ./triggers.sql /docker-entrypoint-initdb.d/05-triggers.sql
COPY ./procedures.sql /docker-entrypoint-initdb.d/06-procedures.sql
