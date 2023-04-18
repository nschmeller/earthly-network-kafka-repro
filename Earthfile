VERSION 0.6
FROM earthly/dind:ubuntu

test:
    RUN export DEBIAN_FRONTEND=noninteractive \
        && apt-get update \
        && apt-get install -y curl gcc g++ make wget unzip

    RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

    RUN wget -q https://github.com/redpanda-data/redpanda/releases/latest/download/rpk-linux-amd64.zip -O rpk-linux-amd64.zip

    RUN mkdir -p /root/.local/bin
    RUN unzip rpk-linux-amd64.zip -d /usr/local/bin/
    RUN rpk version

    COPY Cargo.toml Cargo.toml
    COPY Cargo.lock Cargo.lock
    COPY docker-compose.yml docker-compose.yml
    COPY src src

    WITH DOCKER --pull mysql:5.7 --compose docker-compose.yml
        RUN rpk topic create "recents" --brokers "127.0.0.1:9092" \
            && docker run \
                --publish 3306:3306 \
                --volume $(pwd)/mysql-init:/docker-entrypoint-initdb.d \
                --env MYSQL_ROOT_PASSWORD=froyo \
                --name mysql \
                --detach \
                mysql:5.7 \
            && /root/.cargo/bin/cargo run \
            && rpk topic consume "recents" --num 1 --brokers "127.0.0.1:9092"
    END
