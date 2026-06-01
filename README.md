Axum Next.js
====================================================================================================

Axum x Next.js demo


Requirements
----------------------------------------------------------------------------------------------------

- Rust 1.95
- Node.js 22
- Docker


Technology Stacks
----------------------------------------------------------------------------------------------------

- API server: Rust, Axum, Jsonwebtoken, Sqlx, Config
- Frontend: TypeScript, Next.js, oidc-client-ts, Tailwind CSS


How to Run
----------------------------------------------------------------------------------------------------

```bash
docker compose down && docker compose up --build  
```


How to Run for Dev
----------------------------------------------------------------------------------------------------

### Preparation

```bash
docker compose down && docker compose up postgres dbgate keycloak keycloak-setup
```

### API (Rust - Axum)

```bash
RUSTFLAGS=-Awarnings \
RUST_LOG=debug \
APP__SERVER__PORT=8080 \
APP__DATASOURCE__URL="postgresql://postgres:postgres@localhost:5432/postgres?options=-c search_path=eight" \
APP__SECURITY__JWT__ISSUER_URI=http://localhost:8000/realms/master \
cargo run
```

### UI (TypeScript - Next.js)

```bash
(cd ui_authjs; \
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/postgres?schema=authjs" \
export KEYCLOAK_ISSUER="http://localhost:8000/realms/master" \
export KEYCLOAK_CLIENT_ID="axum-nextjs" \
export KEYCLOAK_CLIENT_SECRET="Secret-1234567890-Secret" \
export NEXTAUTH_URL="http://localhost:3000" \
export NEXTAUTH_SECRET="replace-with-a-strong-random-string" \
export AUTH_TRUST_HOST=true \
export NEXT_PUBLIC_API_BASE_URI="http://localhost:8080" \
npm install; \
npm exec prisma migrate dev -- --name init; \
npm run dev)
```
