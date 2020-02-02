# mysqlpinger

A cli that pings MySQL for specified period. It is mainly intended for use when you want to check "ready" of MySQL.

## Example

```console
# wait for db started by docker-compose.
DOCKER_COMPOSE_PROJECT=xxx
DOCKER_COMPOSE_NETWORK=yyy
docker run --rm -t \
  --network=${DOCKER_COMPOSE_PROJECT}_${DOCKER_COMPOSE_NETWORK} \
  ymgyt/mysqlpinger:latest \
  --user=user --pass=secret --host=<container_name> [--forever|--max-retry=20]
```

## Usage

```console
mysqlpinger 0.1.0
ping to mysql server

USAGE:
    mysqlpinger [FLAGS] [OPTIONS] [DBNAME]

ARGS:
    <DBNAME>    database name [env: MYSQL_DB_NAME=]  [default: sys]

FLAGS:
    -s, --silent     running with no logging
    -v, --verbose    verbose
        --forever    retry without limit
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <HOST>            mysql server hostname [env: MYSQL_HOST=]  [default: 127.0.0.1]
    -p, --port <PORT>            mysql server port [env: MYSQL_PORT=]  [default: 3306]
    -u, --user <USER>            user for authentication [env: MYSQL_USER=]  [default: root]
    -P, --pass <PASS>            password for authentication [env: MYSQL_PASSWORD=]
    -m, --max-retry <COUNT>      max retry count [default: 9]
    -i, --interval <DURATION>    retry ping interval [default: 1s]

Example:
    mysqlpinger --pass=root --port=30303
```