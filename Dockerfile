FROM postgres:16

RUN apt-get update \
    && apt-get install -y \
    postgresql-server-dev-16 \
    gcc \
    make \
    git \
    cron \
    && rm -rf /var/lib/apt/lists/*

RUN git clone https://github.com/citusdata/pg_cron.git /pg_cron \
    && cd /pg_cron \
    && make && make install

RUN echo "shared_preload_libraries = 'pg_cron'" >> /usr/share/postgresql/postgresql.conf.sample

COPY ./migrations /docker-entrypoint-initdb.d/

RUN chmod -R 777 /docker-entrypoint-initdb.d/
