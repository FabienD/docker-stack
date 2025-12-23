# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Pre-configured Docker Compose files for local web development, organized by category.

## Setup

```bash
# Create shared network (required)
docker network create stack_dev

# Copy and customize environment
cp .env.dist .env

# Copy compose files to activate services
cp web/docker-compose.yml.dist web/docker-compose.yml
```

## Directory Structure

```
collection/
├── .env.dist          # Environment variables (versions, credentials)
├── web/               # Reverse proxy (Traefik), mail catcher
├── data/              # Databases: PostgreSQL, MySQL, Redis, RabbitMQ
├── logging/           # Loki, Promtail, Rsyslog
│   └── docker/        # Config files for Loki and Promtail
├── tools/             # Portainer, Grafana
└── monitoring/        # (WIP)
```

## Conventions

### Container Naming
All containers follow: `stack.<category>.<service>`
- `stack.web.reverse_proxy`
- `stack.data.redis`
- `stack.data.postgresql`
- `stack.logging.loki`
- `stack.tools.grafana`

### Network
All services share the `stack_dev` external network (configurable via `DOCKER_NETWORK` in `.env`).

### Domain Names
Services exposed via Traefik use `<service>.${DOMAIN}` pattern:
- `dashboard.stack.local` - Traefik dashboard
- `mailcatcher.stack.local` - Mail UI
- `rabbitmq.stack.local` - RabbitMQ management
- `grafana.stack.local` - Grafana
- `portainer.stack.local` - Portainer

### Ports
Direct access ports (without reverse proxy):
- Redis: 6379
- RabbitMQ: 5672
- PostgreSQL: 5432
- MySQL: 3306
- MailCatcher SMTP: 1025

## Environment Variables

Key variables in `.env.dist`:
- `DOMAIN` - Base domain for Traefik routing (default: `stack.local`)
- `DOCKER_NETWORK` - Shared network name (default: `stack_dev`)
- `*_VERSION` - Docker image versions for each service
- Database credentials: `POSTGRES_*`, `MYSQL_*`, `RABBITMQ_*`

## Traefik Labels

To expose your app via Traefik, add these labels:
```yaml
labels:
  - "traefik.enable=true"
  - "traefik.http.routers.myapp.rule=Host(`myapp.${DOMAIN}`)"
  - "traefik.http.routers.myapp.entrypoints=web"
  - "traefik.http.services.myapp.loadbalancer.server.port=8080"
```

## Loki Logging

To send container logs to Loki:
```yaml
logging:
  driver: loki
  options:
    loki-url: http://loki.stack.local/loki/api/v1/push
    loki-external-labels: job=myapp,env=dev
```

Requires the Loki Docker driver plugin:
```bash
docker plugin install grafana/loki-docker-driver:latest --alias loki --grant-all-permissions
```
