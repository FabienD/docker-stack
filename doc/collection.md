# Docker Stack – Compose File Collection

<<[Home](../README.md)>>

## Overview

This repository provides a curated collection of ready-to-use Docker Compose files for local web development. The collection is organized into five categories: **web**, **data**, **logging**, **tools**, and **monitoring**. Each service is pre-configured for easy integration and can be customized to fit your needs.

To use a service, copy the corresponding `.dist` file to `.yml` (e.g., `docker-compose.yml.dist` → `docker-compose.yml`).

| Application     | Description             | Category      | Container Name | Docker Image | Documentation |
|-----------------|------------------------|---------------|----------------|--------------|---------------|
| Traefik         | Expose your apps locally via domain names | web      | stack.web.reverse      | [⤴](https://hub.docker.com/_/traefik) | [⤴](https://doc.traefik.io/traefik/) |
| MailCatcher     | Catch all outgoing emails | web      | stack.web.mailcatcher   | [⤴](https://hub.docker.com/r/schickling/mailcatcher) | [⤴](https://mailcatcher.me/) |
| PostgreSQL      | Relational database      | data     | stack.data.postgres     | [⤴](https://hub.docker.com/_/postgres/) | [⤴](https://www.postgresql.org/docs/) |
| MySQL           | Relational database      | data     | stack.data.mysql        | [⤴](https://hub.docker.com/_/mysql) | [⤴](https://dev.mysql.com/doc/) |
| Redis           | In-memory cache          | data     | stack.data.redis        | [⤴](https://hub.docker.com/_/redis) | [⤴](https://redis.io/docs/) |
| RabbitMQ        | Message broker           | data     | stack.data.rabbitmq     | [⤴](https://hub.docker.com/_/rabbitmq) | [⤴](https://www.rabbitmq.com/documentation.html) |
| Rsyslog         | Log aggregator           | logging  | stack.logging.rsyslog   | [⤴]() | [⤴]() |
| Loki            | Log aggregator           | logging  | stack.logging.loki      | [⤴](https://hub.docker.com/r/grafana/loki) | [⤴](https://grafana.com/docs/loki/latest/) |
| Promtail        | Log collector            | logging  | stack.logging.promtail  | [⤴](https://hub.docker.com/r/grafana/promtail) | [⤴](https://grafana.com/docs/loki/latest/clients/promtail/) |
| Grafana         | Dashboard & visualization| tools    | stack.tools.grafana     | [⤴](https://hub.docker.com/r/grafana/grafana) | [⤴](https://grafana.com/docs/grafana/latest/) |
| Portainer CE    | Container manager        | tools    | stack.tools.portainer   | [⤴](https://hub.docker.com/r/portainer/portainer.ce) | [⤴](https://docs.portainer.io/) |

You can add your own services or customize the existing ones. **Software versions** and some properties are configurable via the `.env` file at the root of the collection.

---

## Getting Started

### Requirements

- [Docker](https://docs.docker.com/engine/install/) must be installed.
- All containers share a common Docker network (default: `stack_dev`) for easy inter-service communication.

Create the network (if not already present):

```bash
docker network create stack_dev
```

You can change the network name by editing the `DOCKER_NETWORK` variable in the `.env` file. Start by copying `.env.dist` to `.env` and adjusting values as needed.

---

## Category Details

### 1. Web

- **Traefik**: Reverse proxy for routing requests to your apps using local domain names (e.g., `myapp.stack.local`).
- **MailCatcher**: Captures all outgoing emails for safe testing.

#### Traefik Example

To expose your app at a local domain, add these labels to your service:

```yaml
services:
  nginx:
    container_name: myapp-nginx
    image: nginx:1.20
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.myapp.rule=Host(`myapp.stack.local`)"
      - "traefik.http.routers.myapp.entrypoints=web"
      - "traefik.http.routers.myapp.service=myapp"
      - "traefik.http.services.myapp.loadbalancer.server.port=8080"
    ports:
      - "8080"
```

Your app will be accessible at `http://myapp.stack.local` (Traefik forwards to port 8080 inside the container).

#### MailCatcher

- Web interface: [http://mailcatcher.stack.local](http://mailcatcher.stack.local)
- SMTP server: port 1025 (no authentication)

---

### 2. Data

- **Redis**: Exposed on port 6379 (no auth by default)
- **RabbitMQ**: Exposed on port 5672; management UI at [http://rabbitmq.stack.local](http://rabbitmq.stack.local) (credentials in `.env`)
- **PostgreSQL**: Exposed on port 5432 (credentials in `.env`)
- **MySQL**: Exposed on port 3306 (credentials in `.env`)

---

### 3. Logging

- **Loki**: Log aggregation (API on port 3100, metrics at [http://loki.stack.local/metrics](http://loki.stack.local/metrics)). Loki collects logs from your containers and makes them available for querying in Grafana. To use Loki effectively, you need to:
  - Install the [Loki Docker logging driver](https://grafana.com/docs/loki/latest/clients/docker-driver/).
  - Configure your containers to use the Loki driver (see example below).
  - Optionally, use Promtail to collect host logs and forward them to Loki.
- **Promtail**: Collects logs from the host and sends them to Loki. Configure Promtail in `logging/docker/promtail/config.yml` (rename `config.yml.dist` and adjust paths as needed).
- **Rsyslog**: (WIP)

#### Loki Setup & Usage

1. **Install the Loki Docker logging driver** on your Docker host:

```bash
docker plugin install grafana/loki-docker-driver:latest --alias loki --grant-all-permissions
```

2. **Configure your container to use the Loki logging driver**. Example for a PHP-FPM service:

```yaml
services:
  php-fpm:
    container_name: myapp-php-fpm
    image: myprod/php:8.1-fpm
    logging:
      driver: loki
      options:
        loki-url: http://loki.stack.local/loki/api/v1/push
        loki-retries: "5"
        loki-batch-size: "400"
        loki-external-labels: job=myapp,env=dev
```

- `loki-url` should point to your Loki instance (default: `http://loki.stack.local/loki/api/v1/push`).
- You can add custom labels to help filter logs in Grafana.

3. **Explore logs in Grafana**:
   - Access Grafana at [http://grafana.stack.local](http://grafana.stack.local)
   - Add Loki as a data source (URL: `http://stack.logging.loki:3100`)
   - Use the Explore tab to query your logs (e.g., `{job="myapp"}`)

4. **Promtail (optional):**
   - Promtail can collect logs from the Docker host (e.g., `/var/log`) and forward them to Loki.
   - Edit `logging/docker/promtail/config.yml` to match your log paths and labels.

**Note:**
- Only logs sent to stdout/stderr are collected by default. For application logs, ensure your app writes to stdout or configure file-based collection with Promtail.
- For advanced Loki usage, see the [official documentation](https://grafana.com/docs/loki/latest/).

---

### 4. Monitoring

- (WIP)

---

### 5. Tools

- **Portainer**: Web UI for managing containers ([http://portainer.stack.local](http://portainer.stack.local))
- **Grafana**: Visualize data and logs ([http://grafana.stack.local](http://grafana.stack.local))

---

## Contributing

Contributions are welcome! If you spot a typo, want to improve the documentation, or add new services, feel free to open a pull request.