# Project Documentation
Generated at Thu Dec  5 03:52:28 PM CST 2024

## Project Structure
```
.
├── bin
│   ├── clean
│   ├── run
│   ├── setup
│   ├── start
│   └── test
├── codebase.md
├── .codewatchrc
├── docker-compose.yml
├── Dockerfile
├── .git
│   ├── branches
│   ├── COMMIT_EDITMSG
│   ├── config
│   ├── description
│   ├── FETCH_HEAD
│   ├── HEAD
│   ├── hooks
│   │   ├── applypatch-msg.sample
│   │   ├── commit-msg.sample
│   │   ├── fsmonitor-watchman.sample
│   │   ├── post-update.sample
│   │   ├── pre-applypatch.sample
│   │   ├── pre-commit.sample
│   │   ├── pre-merge-commit.sample
│   │   ├── prepare-commit-msg.sample
│   │   ├── pre-push.sample
│   │   ├── pre-rebase.sample
│   │   ├── pre-receive.sample
│   │   ├── push-to-checkout.sample
│   │   ├── sendemail-validate.sample
│   │   └── update.sample
│   ├── index
│   ├── info
│   │   └── exclude
│   ├── logs
│   │   ├── HEAD
│   │   └── refs
│   │       ├── heads
│   │       │   ├── firt
│   │       │   └── main
│   │       └── remotes
│   │           └── origin
│   │               ├── firt
│   │               ├── HEAD
│   │               └── main
│   ├── objects
│   │   ├── 25
│   │   │   └── 277ae6ff8d4a46b15b7b467567a2bb36682392
│   │   ├── 3f
│   │   │   └── 68e24440e7b97ea2c6d30c5a970c05ea171370
│   │   ├── 4a
│   │   │   └── 58ae127924b14098d88a735573bd569b7c69a8
│   │   ├── 4e
│   │   │   └── f9d566b212da8c311fdd5cd2aca22d9a1b0eb9
│   │   ├── 57
│   │   │   └── c1c41ee16aeb683b79b112c1ffde9ab09a8a3d
│   │   ├── 62
│   │   │   └── 64321189c8d2a88cab693d8c392784cabfa472
│   │   ├── 65
│   │   │   └── 46fc15d2050a603fc4e40318f23e6eb58a2981
│   │   ├── 6e
│   │   │   └── a4864a873dc2ee7101c122bd4478e19ff341c0
│   │   ├── 70
│   │   │   └── 7b8d199074c6e6528101bad5f9b2ed5e18e02c
│   │   ├── 71
│   │   │   └── 8a2184ce4fd731fa1c3b47bb07a437b136b765
│   │   ├── 76
│   │   │   └── 04d6324013b56deacaf2d652344e66503b9f50
│   │   ├── 77
│   │   │   ├── bf5ec7c8ca1338e13db3d190172afc1f6dae61
│   │   │   └── f6055aaa7646d2934540874df5438938be9abd
│   │   ├── 90
│   │   │   ├── 2c7c1da72327cd8596a53b2e51a69e38ba0606
│   │   │   └── 8aba44d4ef9d3ec956a170bb6ea5202aa40b6f
│   │   ├── a3
│   │   │   └── 321bdacbdd384fa6664848338df5765782ad5c
│   │   ├── a4
│   │   │   └── 64002cb604ae3935dd88eedc5ce3abd1659018
│   │   ├── a8
│   │   │   └── 9e98a84aad6e5df1458a8c9d9b3bf85fd1b53e
│   │   ├── bb
│   │   │   └── d8a86cd796fb2f5321e71c4f7742b7bf7f8eab
│   │   ├── c4
│   │   │   └── 6afffac45f826e60986f398b94011f9c0b1104
│   │   ├── cd
│   │   │   └── a6289f72e02da9301cb3b474f255ae9e5d63a7
│   │   ├── d5
│   │   │   └── 65c9449ae0f548c8676a2d653637ad5f2183eb
│   │   ├── ee
│   │   │   └── c29e3b819a6ee4d4c2d1b7b0e126da3a684039
│   │   ├── fd
│   │   │   └── 4631aa2b70a77c96578aaca47cfc52d441c599
│   │   ├── info
│   │   └── pack
│   │       ├── pack-bc54e344270c73e14c588caecd8b0dde32b78f70.idx
│   │       ├── pack-bc54e344270c73e14c588caecd8b0dde32b78f70.pack
│   │       └── pack-bc54e344270c73e14c588caecd8b0dde32b78f70.rev
│   ├── packed-refs
│   └── refs
│       ├── heads
│       │   ├── firt
│       │   └── main
│       ├── remotes
│       │   └── origin
│       │       ├── firt
│       │       ├── HEAD
│       │       └── main
│       └── tags
├── .gitattributes
├── .gitignore
├── Makefile
├── manage.sh
├── migrations
│   └── 20231218_initial_schema.sql
├── nginx.conf
├── quiz-app-backend
│   ├── Cargo.toml
│   ├── data
│   │   └── dmv_quiz.json
│   ├── migrations
│   │   ├── 20231212000000_initial_schema.sql
│   │   ├── 20240114000002_create_quizzes.sql
│   │   ├── 20240114000003_create_questions.sql
│   │   ├── 20240114000004_create_quiz_attempts.sql
│   │   ├── 20240118000000_init.sql
│   │   ├── 20240119000003_consolidated_schema.sql
│   │   ├── 20240120000001_add_indexes.sql
│   │   ├── 20241205211425_add_missing_columns.sql
│   │   ├── 20241205212745_create_users.sql
│   │   ├── 20241205213943_create_users.sql
│   │   ├── 20241205214032_create_users.sql
│   │   └── 20241205214206_initial_schema.sql
│   ├── prepare.sh
│   ├── README.md
│   ├── run_migrations.sh
│   ├── run_seeder.sh
│   ├── .sqlx
│   │   ├── query-0d09e64f07395abb21b675f86d1aafa8b4f0257540671dfdeb818f3be16ce1af.json
│   │   ├── query-0f70efe7df28772aa35271d6374308e6d3890314f827a7fdca1d0a8db5d38abb.json
│   │   ├── query-11833e12158e492e6e2e9a08a1af0b9f3b86b6b2836689782e045bdaa27fbe88.json
│   │   ├── query-1ba15c82ac34806fd816b38ef7825071074735233000573f53fc02b73fe101e4.json
│   │   ├── query-29940ee7030fc97ca831eeb481465e3fa4f2795da7a4a6810bf38b5461c237d6.json
│   │   ├── query-2b8b222307e37e64f4459a3a7da050fcb6ce97b5e5b2aa0bf2cf278806060768.json
│   │   ├── query-2f14c2482b4b5aa1a6c521f80f64abde774c589dd2627ce506694e2598e5034b.json
│   │   ├── query-36fea1dec8a1e9747ff80ac30143d71651fea5f7dd069e016316db2a8e83e213.json
│   │   ├── query-3ea969b6219f7e476744e6b8ba255bdb6b91fcd0b685a08a069b583d27081068.json
│   │   ├── query-497b14484b8b452c6e1572caa8ef2edcdc72ae5217cea0a34e39df00bd5d729d.json
│   │   ├── query-50224d6cf0874a4266b1e10a9003efd1f8c947b7a391dbbdcfdc1ed2ab906a0f.json
│   │   ├── query-50293c2e54af11d4c2a553e29b671cef087a159c6ee7182d8ca929ecb748f3b7.json
│   │   ├── query-50e6a96bd8c86372ec28b3b027b19eff60287f30b83208ed384424ebd5c42547.json
│   │   ├── query-534db52238e91ae4f1ede878416be57370a87e3a3b75972e3f1d3c52e2d06f17.json
│   │   ├── query-599f5b932c679f9f1cbf579417c6faba7e4c50ed1ae568e11873add887a0ef2f.json
│   │   ├── query-64de2c496b8f41ee11c4e1a24d82aa48a16c477e8019572002508d483c104674.json
│   │   ├── query-65745a95c3a49feb371990e7d94ffc061eb83f56885051c4da62c48beba0d073.json
│   │   ├── query-6b993687c07ac4d2c70c5b647a5de49ab562837a9a4022f9ec43ce3bcbe969eb.json
│   │   ├── query-6defad0d098b72bf141adc4c32237244ce333e5342d23cc1faa7d26560fd09bf.json
│   │   ├── query-7230d87f38bedb0077a50b36db619bdd8b285e0db5096fac4785082473ee8624.json
│   │   ├── query-73eeb0bcecabf428fc2beb9d8fad8cfb17500cc0d9d2e819d28e028e8c18bca7.json
│   │   ├── query-7e44776ea84d48bfa1935fe6af6ccc55d7f6a75762173ecec69fb1235ce14764.json
│   │   ├── query-7f83025af9f13d21f915f11cb7c7d0208b4f7d995909f8844634f8cbb3d4094c.json
│   │   ├── query-8211b1e6230de5ef4815732a72f7eaba03b7056e906d62189787a59b079ca5a3.json
│   │   ├── query-843923b9a0257cf80f1dff554e7dc8fdfc05f489328e8376513124dfb42996e3.json
│   │   ├── query-848369ecc1e7151ce42db64034b0bfd8f8f5d985ebe74eda2c16336910e75474.json
│   │   ├── query-86e942ccd55a1bd67872ce0f5f62e00f5c1293bc6c3a02ef87bf94f9cf8e1dae.json
│   │   ├── query-88f26472e41c0381a8945804164c12fdc502c55c9bb4f90d64fd38d953e0d5f5.json
│   │   ├── query-89e80f6b110c8d91a257c9acd4baa8f2f59f6333660676beb2878aa5a3485d2e.json
│   │   ├── query-8c3bf613ba82c5502a2274b380df2734098a8226195c95cc4e5e732a2f6840e4.json
│   │   ├── query-900b2478a91719e6f8cce2c5e6f9b8e8881cf2e64ee057c250041666609925e7.json
│   │   ├── query-93b9f4480d88b5cff07cd02a2ecc61d52da96df76584050a4c18471608a5ebd2.json
│   │   ├── query-9e4cda5ac3ad3466ff662e805472ff3f45111acafccdf15674c358ebca3c5e18.json
│   │   ├── query-a18646bcf72345796696e886c2384c9083d11c22fb94e5de313ddf60d3af1a27.json
│   │   ├── query-b0da9756e6415a350702db787bdcbacd02bce66f6683c4fbbc63641150783f39.json
│   │   ├── query-b1a30dffd394f708c0eecff11a6a4faee48cf37415f7cff3d80652ee080ceafa.json
│   │   ├── query-b9ffe941de864fb7646a559926c6e4b29dce07aaf3baef99d90f5fc11ed76414.json
│   │   ├── query-bae9fd94e339fa2a572b318c14e73c44ff9cbf96ff122b5d9171fb26ae94d042.json
│   │   ├── query-c0b6c30b0d1915bb4af51b3dc5e92b3bad8785034614cd3e8e32a888b8bb2a99.json
│   │   ├── query-c30f235fc9307f5390a1fd329e2d03e488940d69c9d4776c4ec29bed89005060.json
│   │   ├── query-c7cd86cac654893d191f345f430afff3521e15a89747ecec461433f462e46fe1.json
│   │   ├── query-cac2ae5e455fc75839a825cc1f3bcfb328dfd31ee97b36242cb7b56ab11fb63f.json
│   │   ├── query-cac36ef9c603ac102c3229828db75408b920dd585cb4545dc944e1b8d6993458.json
│   │   ├── query-cc3f77f9ec4a4992a7637877ab95ae59d518fed2d549756ea740c6f70c46ee06.json
│   │   ├── query-d2de24ced45300a690d6f8032465cc53c744c5ff7aa8ac7639b410d36a501b48.json
│   │   ├── query-dbbdb2569e8501c31bd351902dcc3a705032e280f750edcdee3f3d0e6decf8a6.json
│   │   ├── query-df0ab617a11a3cfcce5b29131de7eab6a2892b6b779cd0824d0fd8b30e56ab84.json
│   │   ├── query-e418129db0d2537a34ec3df0392cf571fde0d32e264bf51b7cfa6123a173b7f6.json
│   │   ├── query-e4dc2d8bd471f022fac32e6c0a28efc01b138f8986afd09b38c5a9b8e67ee14e.json
│   │   ├── query-eabd18a30423ca1444f0d894ec57fa5a8edbfed54ff6bb9ac73425bf66f9e810.json
│   │   ├── query-ec10aa4d7f3b122b4a856c9ab64cb3eb5b61b9bfc7c7f656830dbc6cfb6a0b25.json
│   │   ├── query-ecfb71b68d8227b4569e931dbd8af4f6589978b071a5f29e4bc92e428662caa3.json
│   │   ├── query-f2cc0aa7eb542176a72e0262a3bacc48f6004abb8f1cc29565dd7a759a220ada.json
│   │   ├── query-f4f8f8c2668ec23ba1f4a315d74087521496603e8b1bc10475a864001e795593.json
│   │   ├── query-f7de4aa65cfd46f3a0f0ec64cc5c5d266fa985eec03f30268abc593d04ef28af.json
│   │   ├── query-fb3d32f870990b73aeee9cfc8657ce746ba14d1695f86c22523884642a399cb9.json
│   │   └── query-fd64104d130b93dd5fc9414b8710ad5183b647eaaff90decbce15e10d83c7538.json
│   ├── src
│   │   ├── auth.rs
│   │   ├── bin
│   │   │   └── seed.rs
│   │   ├── config.rs
│   │   ├── data
│   │   │   └── mn_dmv_questions.json
│   │   ├── db.rs
│   │   ├── error.rs
│   │   ├── handlers
│   │   │   ├── answer.rs
│   │   │   ├── auth.rs
│   │   │   ├── mod.rs
│   │   │   ├── question.rs
│   │   │   ├── quiz_attempt.rs
│   │   │   ├── quiz.rs
│   │   │   └── user.rs
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── middleware
│   │   │   ├── cache.rs
│   │   │   ├── .gitignore
│   │   │   ├── mod.rs
│   │   │   ├── # Optional eslint cache
│   │   │   ├── serve() {.sh
│   │   │   ├── serve.sh
│   │   │   ├── test_context.rs
│   │   │   ├── Untitled-1.sh
│   │   │   └── use sqlx::PgPool;
│   │   ├── models.rs
│   │   ├── quiz-app.code-workspace
│   │   ├── seeder.rs
│   │   ├── test_helpers.rs
│   │   ├── tests
│   │   │   └── test_helpers.rs
│   │   └── utils.rs
│   └── tests
│       ├── api_tests.rs
│       ├── auth_test.rs
│       ├── db_test.rs
│       ├── handlers_test.rs
│       ├── integration_test.rs
│       ├── mod.rs
│       ├── question_test.rs
│       ├── test_helpers.rs
│       ├── tests
│       │   └── test_utils.rs
│       └── test_utils.rs
├── quiz-app-database
│   ├── schema.sql
│   └── seed_dmv_questions.sql
├── quiz-app-frontend
│   ├── babel.config.js
│   ├── cypress
│   │   ├── e2e
│   │   │   ├── quiz.cy.js
│   │   │   └── quiz.cy.ts
│   │   ├── fixtures
│   │   │   └── example.json
│   │   ├── screenshots
│   │   │   └── quiz.cy.js
│   │   │       └── Quiz App -- should display the home page -- before each hook (failed).png
│   │   ├── support
│   │   │   ├── commands.js
│   │   │   ├── commands.ts
│   │   │   ├── component-index.html
│   │   │   ├── component.js
│   │   │   ├── e2e.js
│   │   │   └── e2e.ts
│   │   └── tsconfig.json
│   ├── cypress.config.ts
│   ├── docker-compose.yml
│   ├── Dockerfile
│   ├── package.json
│   ├── public
│   │   └── index.html
│   ├── src
│   │   ├── App.tsx
│   │   ├── components
│   │   │   ├── common
│   │   │   │   ├── ErrorMessage.tsx
│   │   │   │   ├── LoadingSpinner.tsx
│   │   │   │   └── QuestionDisplay.tsx
│   │   │   ├── Home.tsx
│   │   │   ├── Navbar.tsx
│   │   │   ├── QuestionForm.tsx
│   │   │   ├── QuizCard.tsx
│   │   │   ├── QuizCreate.tsx
│   │   │   ├── QuizList.tsx
│   │   │   ├── QuizProgress.tsx
│   │   │   ├── QuizResults.tsx
│   │   │   ├── QuizStats.tsx
│   │   │   └── QuizTake.tsx
│   │   ├── hooks
│   │   │   ├── useDataFetching.ts
│   │   │   └── useFormValidation.ts
│   │   ├── index.tsx
│   │   ├── pages
│   │   │   ├── Dashboard.tsx
│   │   │   ├── QuizDetails.tsx
│   │   │   ├── QuizResults.tsx
│   │   │   └── TakeQuiz.tsx
│   │   ├── services
│   │   │   └── api.ts
│   │   ├── setupTests.ts
│   │   ├── tests
│   │   │   ├── components
│   │   │   │   ├── QuizCard.test.tsx
│   │   │   │   ├── QuizProgress.test.tsx
│   │   │   │   └── QuizResults.test.tsx
│   │   │   └── pages
│   │   │       ├── Dashboard.test.tsx
│   │   │       └── TakeQuiz.test.tsx
│   │   └── types
│   │       ├── index.ts
│   │       └── quiz.ts
│   └── tsconfig.json
├── quizmo
│   ├── cicd.json
│   └── .puppetryrc
├── README.md
├── scripts
│   ├── activate
│   └── commands.sh
├── src
│   ├── lib.rs
│   └── test_helpers.rs
├── .tasks.json
└── wait-for-it.sh

76 directories, 249 files
```

## Configuration Files

### Dockerfile
```dockerfile
# Use a multi-stage build to optimize the final image size

# Stage 1: Build frontend
FROM node:18.17.1-slim AS frontend-builder
WORKDIR /app/frontend
COPY quiz-app-frontend/package*.json ./
RUN npm install
COPY quiz-app-frontend/ .
ENV REACT_APP_API_BASE_URL=http://localhost:8080/api
RUN npm run build

# Stage 2: Build the backend
FROM rust:1.83.0 AS backend-builder
WORKDIR /app/quiz-app-backend

# Create the cargo user before changing ownership
RUN useradd -m -u 1000 -U cargo && \
    apt-get update && \
    apt-get install -y pkg-config libssl-dev netcat-openbsd && \
    cargo install sqlx-cli --no-default-features --features postgres --locked && \
    mkdir -p /usr/local/cargo/registry && \
    chmod -R 755 /usr/local/cargo/registry && \
    chown -R cargo:cargo /usr/local/cargo/registry && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Create cargo user and set permissions
RUN chown -R cargo:cargo /app && \
    mkdir -p /usr/local/cargo/registry/cache && \
    mkdir -p /usr/local/cargo/registry/index && \
    chown -R cargo:cargo /usr/local/cargo/registry/cache && \
    chown -R cargo:cargo /usr/local/cargo/registry/index

USER cargo

# Copy backend dependency files and fetch dependencies
COPY --chown=cargo:cargo quiz-app-backend/Cargo.toml quiz-app-backend/Cargo.lock ./
RUN cargo fetch

# Copy backend files and SQLx data
COPY --chown=cargo:cargo quiz-app-backend/ .
COPY --chown=cargo:cargo migrations /app/migrations
COPY --chown=cargo:cargo quiz-app-backend/.sqlx /app/.sqlx

# Build with prepared queries
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Stage 3: Final image
FROM debian:bullseye-slim
WORKDIR /app

# Install nginx for serving frontend
RUN apt-get update && \
    apt-get install -y nginx libssl-dev ca-certificates netcat-openbsd && \
    rm -rf /var/lib/apt/lists/*

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Copy frontend build
COPY --from=frontend-builder /app/frontend/build /app/frontend

# Copy backend binary
COPY --from=backend-builder /app/quiz-app-backend/target/release/quiz-app-backend /app/backend/

# Copy wait script
COPY wait-for-it.sh /app/backend/wait-for-it.sh
RUN useradd -m -u 1000 -U app && \
    chmod +x /app/backend/wait-for-it.sh && chown app:app /app/backend/wait-for-it.sh

# Create non-root user
RUN chown -R app:app /app

USER app

# Set environment variables
ENV RUST_LOG=info
ENV SKIP_DB_INIT=

EXPOSE 80 8080

# Start both nginx and backend
CMD ["sh", "-c", "./backend/wait-for-it.sh db:5432 -- ./backend/quiz-app-backend & nginx -g 'daemon off;'"]

# Stage 4: Run Endpoint Tests
FROM backend-builder AS endpoint-tester
WORKDIR /app/quiz-app-backend

# Set the CARGO_HOME environment variable to the cargo user's home directory
ENV CARGO_HOME=/home/cargo/.cargo

# Ensure the CARGO_HOME directory exists and is owned by the cargo user
RUN mkdir -p $CARGO_HOME && \
    chmod -R 755 $CARGO_HOME

USER cargo

# Install SQLx CLI for running migrations during tests
RUN cargo install sqlx-cli --no-default-features --features postgres --locked

# Copy necessary files for testing
COPY --from=backend-builder /app/quiz-app-backend/ ./ 
COPY --from=backend-builder /app/migrations ./migrations

# Set environment variables for testing
ENV DATABASE_URL=postgres://remixonwin:600181@db/quiz_app_test
ENV SQLX_OFFLINE=false

# Execute endpoint tests
CMD ["cargo", "test", "--", "endpoint_tests"]```

### docker-compose.yml
```yml
version: '3.8'
services:
  db:
    image: postgres:14
    ports:
      - "${DB_PORT:-5433}:5432"
    environment:
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      POSTGRES_DB: ${POSTGRES_DB}
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${POSTGRES_USER} -d ${POSTGRES_DB}"]
      interval: 5s
      timeout: 5s
      retries: 20
      start_period: 10s
    networks:
      - quiz-network
    dns:
      - "8.8.8.8"
      - "8.8.4.4"
    restart: unless-stopped

  app:
    build: 
      context: .
      dockerfile: Dockerfile
      args:
        CACHE_DATE: ${DATE:-$(date +%Y-%m-%d)}
    image: quiz-app-backend:1.0.0
    ports:
      - "${APP_PORT:-80}:80"
      - "${API_PORT:-8080}:8080"
    environment:
      DATABASE_URL: postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@db:5432/${POSTGRES_DB}
      SERVER_HOST: ${SERVER_HOST}
      SERVER_PORT: ${SERVER_PORT}
      RUST_LOG: ${RUST_LOG}
      JWT_SECRET: ${JWT_SECRET}
      REACT_APP_API_BASE_URL: ${REACT_APP_API_BASE_URL}
      SKIP_DB_INIT: true
    entrypoint: ["/bin/sh", "/app/backend/wait-for-it.sh", "db:5432", "--timeout=60", "--", "./backend/quiz-app-backend"]
    volumes:
      - ./quiz-app-frontend:/app/frontend
      - ./quiz-app-backend:/app/backend
      - cargo-cache:/usr/local/cargo/registry
      - frontend-node-modules:/app/quiz-app-frontend/node_modules
    depends_on:
      db:
        condition: service_healthy
    networks:
      - quiz-network
    dns:
      - "8.8.8.8"
      - "8.8.4.4"
    restart: unless-stopped

  endpoint-tester:
    build:
      context: .
      target: endpoint-tester
    image: quiz-app-endpoint-tests
    depends_on:
      - db
    environment:
      DATABASE_URL: postgres://postgres:postgres@db:5432/quiz_app_test
      SQLX_OFFLINE: false
    dns:
      - "8.8.8.8"
      - "8.8.4.4"
    restart: unless-stopped

volumes:
  cargo-cache:
  postgres_data:
  frontend-node-modules:

networks:
  quiz-network:
    driver: bridge```

### .env.example
```example
# Database Configuration
POSTGRES_USER=remixonwin
POSTGRES_PASSWORD=600181
POSTGRES_DB=quiz_app
DB_HOST=localhost
DB_PORT=5433

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
WORKERS=4
RUST_LOG=info

# JWT Configuration
JWT_SECRET=your_jwt_secret_key_here

# Frontend Configuration
REACT_APP_API_BASE_URL=http://localhost:8080/api```

## Source Code

### quiz-app-backend/tests/auth_test.rs
```rs
#[cfg(test)]
mod auth_tests {
    use quiz_app_backend::auth;
    use dotenv::dotenv;
    use std::env;

    fn setup() {
        dotenv().ok();
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    }

    #[test]
    fn test_valid_token() {
        setup();
        let user_id = 1;
        let token = auth::generate_token(user_id, "user").unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_invalid_user_id() {
        setup();
        let result = auth::generate_token(-1, "user");
        assert!(result.is_err());
    }
}
```

### quiz-app-backend/tests/integration_test.rs
```rs
use actix_web::{
    test,
    web,
    App,
    http::StatusCode,
};
use quiz_app_backend::{
    config::get_config,
    auth::generate_token,
    models::{
        User,
        CreateUser,
        LoginCredentials,
        CreateQuiz,
    },
};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[actix_web::test]
    async fn test_health_check() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;
        
        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_user_registration() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;

        let mut rng = StdRng::seed_from_u64(42);
        let username = format!("testuser_{}", rng.gen::<u32>());

        let create_user_req = test::TestRequest::post()
            .uri("/api/auth/register")
            // ...existing code...
            .to_request();

        let resp = test::call_service(&app, create_user_req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    #[actix_web::test]
    async fn test_user_login() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;
        
        let mut rng = StdRng::seed_from_u64(42);
        let username = format!("testuser_{}", rng.gen::<u32>());
        let password = "testpass123".to_string();
        
        let register_req = test::TestRequest::post()
            .uri("/api/auth/register")
            // ...existing code...
            .to_request();

        let resp = test::call_service(&app, register_req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let login_req = test::TestRequest::post()
            .uri("/api/auth/login")
            // ...existing code...
            .to_request();

        let resp = test::call_service(&app, login_req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_quiz_creation_and_retrieval() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        
        // Create a test user in the database
        let user = sqlx::query_as!(
            User,
            // ...existing code...
            "user"
        )
        .fetch_one(&pool)
        .await
        .unwrap();

        let token = generate_token(user.id, "user").unwrap();

        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;

        let quiz_data = CreateQuiz {
            title: "Test Quiz".to_string(),
            // ...existing code...
            created_by: user.id,
        };

        let create_resp = test::TestRequest::post()
            .uri("/api/quizzes")
            // ...existing code...
            .to_request();

        let quiz: CreateQuiz = test::call_and_read_body_json(&app, create_resp).await;
        assert_eq!(quiz.title, "Test Quiz");
        assert_eq!(quiz.description, Some("A test quiz".to_string()));
        assert_eq!(quiz.created_by, user.id);
    }

    #[actix_web::test]
    async fn test_user_registration_and_login() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;
        
        // Test registration
        let mut rng = StdRng::seed_from_u64(42);
        let username = format!("testuser_{}", rng.gen::<u32>());
        let password = "testpass123".to_string();
        
        let register_req = test::TestRequest::post()
            .uri("/api/auth/register")
            // ...existing code...
            .to_request();
            
        let register_resp = test::call_service(&app, register_req).await;
        assert_eq!(register_resp.status(), StatusCode::CREATED, "Registration failed with status: {}", register_resp.status());
        
        // Test login
        let login_req = test::TestRequest::post()
            .uri("/api/auth/login")
            // ...existing code...
            .to_request();
            
        let login_resp = test::call_service(&app, login_req).await;
        assert_eq!(login_resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_create_quiz() {
        let config = get_config().unwrap();
        let pool = sqlx::PgPool::connect(&config.database_url).await.unwrap();
        let app = test::init_service(
            App::new()
            // ...existing code...
        ).await;

        let mut rng = StdRng::seed_from_u64(42);
        let username = format!("testuser_{}", rng.gen::<u32>());
        let user_id = sqlx::query!(
            "INSERT INTO users (username, password_hash, role) 
            // ...existing code...
            "user"
        )
        .fetch_one(&pool)
        .await
        .unwrap()
        .id;

        let quiz_data = CreateQuiz {
            title: "Test Quiz".to_string(),
            // ...existing code...
            created_by: user_id,
        };

        let create_resp = test::TestRequest::post()
            .uri("/api/quizzes")
            // ...existing code...
            .to_request();

        let resp = test::call_service(&app, create_resp).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
```

### quiz-app-backend/tests/tests/test_utils.rs
```rs
use sqlx::PgPool;
use quiz_app_backend::{
    models::*,
    error::AppError,
};
use std::sync::Once;
use rand::{thread_rng, Rng};

static INIT: Once = Once::new();

pub async fn setup_test_db() -> PgPool {
    INIT.call_once(|| {
        dotenv::dotenv().ok();
    });
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
        
    // Clean up test database
    sqlx::query!("TRUNCATE TABLE users, quizzes, questions, answers CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to clean test database");
        
    pool
}

pub async fn create_test_user(pool: &PgPool) -> Result<User, AppError> {
    let username = format!("test_user_{}", thread_rng().gen::<u32>());
    let create_user = CreateUser {
        username: username.clone(),
        password: "test_password123".to_string(),
    };
    
    User::create(pool, create_user).await
}

pub async fn create_test_quiz(pool: &PgPool, user_id: i32) -> Result<Quiz, AppError> {
    let create_quiz = CreateQuiz {
        title: format!("Test Quiz {}", thread_rng().gen::<u32>()),
        description: Some("A test quiz".to_string()),
        created_by: user_id,
    };
    
    Quiz::create(pool, create_quiz).await
}

pub async fn create_test_question(pool: &PgPool, quiz_id: i32) -> Result<Question, AppError> {
    let create_question = CreateQuestion {
        quiz_id,
        question_text: format!("Test question {}", thread_rng().gen::<u32>()),
        order_num: Some(1),
    };
    
    Question::create(pool, create_question).await
}

pub async fn create_test_answer(pool: &PgPool, question_id: i32) -> Result<Answer, AppError> {
    let create_answer = CreateAnswer {
        text: format!("Test answer {}", thread_rng().gen::<u32>()),
        is_correct: true,
    };
    
    sqlx::query_as!(
        Answer,
        r#"
        INSERT INTO answers (question_id, text, is_correct, order_num)
        VALUES ($1, $2, $3, $4)
        RETURNING id, question_id, text, is_correct, order_num, created_at, updated_at
        "#,
        question_id,
        create_answer.text,
        create_answer.is_correct,
        1  // Default order_num
    )
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_create_test_user() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await.unwrap();
        assert!(!user.username.is_empty());
        assert!(!user.password_hash.is_empty());
    }

    #[actix_web::test]
    async fn test_create_test_quiz() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await.unwrap();
        let quiz = create_test_quiz(&pool, user.id).await.unwrap();
        assert!(!quiz.title.is_empty());
        assert!(quiz.description.is_some());
    }

    #[actix_web::test]
    async fn test_create_test_question() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await.unwrap();
        let quiz = create_test_quiz(&pool, user.id).await.unwrap();
        let question = create_test_question(&pool, quiz.id).await.unwrap();
        assert!(!question.question_text.is_empty());
        assert_eq!(question.quiz_id, quiz.id);
    }

    #[actix_web::test]
    async fn test_create_test_answer() {
        let pool = setup_test_db().await;
        let user = create_test_user(&pool).await.unwrap();
        let quiz = create_test_quiz(&pool, user.id).await.unwrap();
        let question = create_test_question(&pool, quiz.id).await.unwrap();
        let answer = create_test_answer(&pool, question.id).await.unwrap();
        assert!(!answer.text.is_empty());
        assert_eq!(answer.question_id, question.id);
    }
}
```

### quiz-app-backend/tests/test_utils.rs
```rs
use quiz_app_backend::{
    models::{CreateUser, CreateQuiz, CreateQuestion, CreateAnswer, User, Quiz, Question, Answer, DbModel},
    error::AppError,
};
use sqlx::PgPool;
use rand::prelude::*;
use rand::SeedableRng;
use std::env;

pub async fn setup_test_db() -> PgPool {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.expect("Failed to connect to database")
}

pub async fn reset_db(pool: &PgPool) -> Result<(), AppError> {
    sqlx::query!("TRUNCATE TABLE users, quizzes, questions, answers, quiz_attempts CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn create_test_user(pool: &PgPool) -> Result<i32, AppError> {
    let mut rng = rand::rngs::StdRng::from_seed([42; 32]);
    let create_user = CreateUser {
        username: format!("testuser_{}", rng.gen::<u32>()),
        password: "password123".to_string(),
    };
    
    let user = User::create(pool, create_user).await?;
    Ok(user.id)
}

pub async fn create_test_quiz(pool: &PgPool, user_id: i32) -> Result<Quiz, AppError> {
    let mut rng = rand::rngs::StdRng::from_seed([42; 32]);
    let create_quiz = CreateQuiz {
        title: format!("Test Quiz {}", rng.gen::<u32>()),
        description: Some("Test Description".to_string()),
        created_by: user_id,
    };
    
    Quiz::create(pool, create_quiz).await
}

pub async fn create_test_question(pool: &PgPool, quiz_id: i32) -> Result<Question, AppError> {
    let mut rng = rand::rngs::StdRng::from_seed([42; 32]);
    let create_question = CreateQuestion {
        quiz_id,
        question_text: format!("Test Question {}", rng.gen::<u32>()),
        order_num: Some(1),
    };
    
    Question::create(pool, create_question).await
}

pub async fn create_test_answer(pool: &PgPool, _question_id: i32) -> Result<Answer, AppError> {
    let create_answer = CreateAnswer {
        text: "Test Answer".to_string(),
        is_correct: true,
    };
    
    Answer::create(pool, create_answer).await
}

pub async fn cleanup_test_data(pool: &PgPool) {
    reset_db(pool).await.expect("Failed to reset database");
}
```

### quiz-app-backend/tests/question_test.rs
```rs
#[cfg(test)]
mod question_tests {
    use actix_web::{
        test, web, App,
        http::StatusCode,
    };
    use quiz_app_backend::{
        models::{CreateUser, CreateQuiz, CreateQuestion},
        handlers::question,
        auth::{generate_token, Auth},
    };
    use sqlx::PgPool;
    use std::env;
    use chrono::Utc;

    async fn setup() -> PgPool {
        dotenv::dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // Set JWT secret for testing
        env::set_var("JWT_SECRET", "test_secret");
        PgPool::connect(&database_url).await.unwrap()
    }

    #[actix_web::test]
    async fn test_create_question() {
        let pool = setup().await;
        let user_id = create_test_user(&pool).await;
        let token = generate_token(user_id, "user").unwrap();
        let quiz_id = create_test_quiz(&pool, user_id).await;

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(
                    web::scope("/api")
                        .wrap(Auth)
                        .service(
                            web::scope("/quizzes/{quiz_id}/questions")
                                .service(question::create_question)
                                .service(question::list_questions)
                        )
                )
        ).await;

        let question_data = CreateQuestion {
            quiz_id: quiz_id,
            question_text: "Test Question".to_string(),
            order_num: Some(1),
        };

        let req = test::TestRequest::post()
            .uri(&format!("/api/quizzes/{}/questions", quiz_id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(&question_data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        // Cleanup
        let _ = sqlx::query!("DELETE FROM questions WHERE quiz_id = $1", quiz_id)
            .execute(&pool)
            .await;
        let _ = sqlx::query!("DELETE FROM quizzes WHERE id = $1", quiz_id)
            .execute(&pool)
            .await;
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&pool)
            .await;
    }

    async fn create_test_user(pool: &PgPool) -> i32 {
        let new_user = CreateUser {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        sqlx::query!(
            r#"
            INSERT INTO users (username, password_hash, created_at)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            new_user.username,
            new_user.password,
            Utc::now()
        )
        .fetch_one(pool)
        .await
        .expect("Failed to create test user")
        .id
    }

    async fn create_test_quiz(pool: &PgPool, user_id: i32) -> i32 {
        let quiz_data = CreateQuiz {
            title: "Test Quiz".to_string(),
            description: Some("Test Description".to_string()),
            created_by: user_id,
        };

        sqlx::query!(
            r#"
            INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $4)
            RETURNING id
            "#,
            quiz_data.title,
            quiz_data.description,
            quiz_data.created_by,
            Utc::now()
        )
        .fetch_one(pool)
        .await
        .expect("Failed to create test quiz")
        .id
    }
}
```

### quiz-app-backend/tests/test_helpers.rs
```rs
use actix_web::test;
use sqlx::PgPool;
use quiz_app_backend::{
    models::CreateQuiz,
    test_helpers::{
        setup_test_app,
        create_test_quiz_with_title,
        setup_test_context,
        cleanup_test_data,
    },
};

#[actix_web::test]
async fn test_helpers_work_correctly() {
    let ctx = setup_test_context().await;
    let app = setup_test_app(ctx.pool.clone()).await;

    // Test creating a quiz
    let quiz_data = CreateQuiz {
        title: "Test Quiz".to_string(),
        description: Some("A test quiz description".to_string()),
        created_by: ctx.user_id,
    };

    let req = test::TestRequest::post()
        .uri("/api/quizzes")
        .insert_header(("Authorization", format!("Bearer {}", ctx.token)))
        .set_json(&quiz_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test creating quiz with title
    let quiz = create_test_quiz_with_title(&ctx.pool, ctx.user_id, "Another Test Quiz").await;
    assert_eq!(quiz.title, "Another Test Quiz");

    cleanup_test_data(&ctx.pool).await;
}
```

### quiz-app-backend/tests/api_tests.rs
```rs
use actix_web::{test, web, App};
use quiz_app_backend::routes;
use serde_json::{json, Value};
mod test_utils;
use test_utils::{setup_test_db, reset_db};
use tokio::time::{sleep, Duration};

#[actix_web::test]
async fn test_register_user() {
    let pool = setup_test_db().await;
    reset_db(&pool).await.unwrap();
    sleep(Duration::from_millis(100)).await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    ).await;

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "username": "testuser1",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("Register response: {:?}", String::from_utf8(body.to_vec()).unwrap());
    assert!(status.is_success());
}

#[actix_web::test]
async fn test_login_user() {
    let pool = setup_test_db().await;
    reset_db(&pool).await.unwrap();
    sleep(Duration::from_millis(100)).await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    ).await;

    // First register the user
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "username": "testuser2",
            "password": "password123"
        }))
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    let register_status = register_resp.status();
    let body = test::read_body(register_resp).await;
    println!("Register response in login test: {:?}", String::from_utf8(body.to_vec()).unwrap());
    assert!(register_status.is_success());

    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "username": "testuser2",
            "password": "password123"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("Login response: {:?}", String::from_utf8(body.to_vec()).unwrap());
    assert!(status.is_success());
}

#[actix_web::test]
async fn test_create_quiz() {
    let pool = setup_test_db().await;
    reset_db(&pool).await.unwrap();
    sleep(Duration::from_millis(100)).await;
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    ).await;

    // First register and login
    let register_req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(json!({
            "username": "testuser3",
            "password": "password123"
        }))
        .to_request();

    let register_resp = test::call_service(&app, register_req).await;
    let register_status = register_resp.status();
    println!("Register status: {:?}", register_status);
    assert!(register_status.is_success());

    let login_req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(json!({
            "username": "testuser3",
            "password": "password123"
        }))
        .to_request();

    let login_resp = test::call_service(&app, login_req).await;
    let login_status = login_resp.status();
    let login_body = test::read_body(login_resp).await;
    let login_json: Value = serde_json::from_slice(&login_body).unwrap();
    let token = login_json["token"].as_str().unwrap();
    println!("Login response: {:?}", String::from_utf8(login_body.to_vec()).unwrap());
    println!("Login status: {:?}", login_status);
    assert!(login_status.is_success());
    
    // Now create a quiz
    let req = test::TestRequest::post()
        .uri("/api/quizzes")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(json!({
            "title": "Test Quiz",
            "description": "A test quiz"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    let body = test::read_body(resp).await;
    println!("Create quiz status: {:?}", status);
    println!("Create quiz response: {:?}", String::from_utf8(body.to_vec()).unwrap());
    assert!(status.is_success());
}
```

### quiz-app-backend/tests/mod.rs
```rs
pub mod test_helpers;
pub mod handlers_test;
pub mod auth_test;
pub mod db_test;
pub mod question_test;
pub mod integration_test;
```

### quiz-app-backend/tests/handlers_test.rs
```rs
use actix_web::{test, http::StatusCode};
use quiz_app_backend::{
    models::{Quiz, CreateQuiz},
    test_helpers::{
        setup_test_app,
        create_test_quiz_with_title,
        verify_quiz_in_db,
        setup_test_context,
        cleanup_test_data,
        create_test_user,
    },
    auth::generate_token,
};
use serde_json::json;
use chrono::Utc;
use uuid::Uuid;
use bcrypt;

#[actix_web::test]
async fn test_create_quiz() {
    let ctx = setup_test_context().await;
    let app = setup_test_app(ctx.pool.clone()).await;

    // Create test user
    let user_id = create_test_user(&ctx.pool).await;

    let token = generate_token(user_id, "user").expect("Failed to generate token");

    let quiz_data = CreateQuiz {
        title: "Test Quiz".to_string(),
        description: Some("A test quiz".to_string()),
        created_by: user_id,
    };

    let req = test::TestRequest::post()
        .uri("/api/quizzes")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&quiz_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED, "Expected 201 Created for quiz creation");

    let quiz: Quiz = test::read_body_json(resp).await;
    assert_eq!(quiz.title, quiz_data.title);
    assert_eq!(quiz.description, quiz_data.description);
    assert_eq!(quiz.created_by, user_id);

    let db_quiz = verify_quiz_in_db(&ctx.pool, quiz.id).await;
    assert!(db_quiz.is_some(), "Quiz should exist in database");

    cleanup_test_data(&ctx.pool).await;
}

#[actix_web::test]
async fn test_update_quiz() {
    let ctx = setup_test_context().await;
    let app = setup_test_app(ctx.pool.clone()).await;

    let user_id = create_test_user(&ctx.pool).await;

    let quiz = create_test_quiz_with_title(&ctx.pool, user_id, "Original Title").await;

    let update_data = CreateQuiz {
        title: "Updated Title".to_string(),
        description: Some("Updated description".to_string()),
        created_by: user_id,
    };

    let token = generate_token(user_id, "user").expect("Failed to generate token");

    let req = test::TestRequest::put()
        .uri(&format!("/api/quizzes/{}", quiz.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK, "Expected 200 OK for quiz update");

    let updated_quiz: Quiz = test::read_body_json(resp).await;
    assert_eq!(updated_quiz.title, "Updated Title");
    assert_eq!(updated_quiz.description, Some("Updated description".to_string()));

    let db_quiz = verify_quiz_in_db(&ctx.pool, updated_quiz.id).await;
    assert!(db_quiz.is_some(), "Quiz should exist in database");
    let db_quiz = db_quiz.unwrap();
    assert_eq!(db_quiz.title, "Updated Title");

    cleanup_test_data(&ctx.pool).await;
}

#[actix_web::test]
async fn test_delete_quiz() {
    let ctx = setup_test_context().await;
    let app = setup_test_app(ctx.pool.clone()).await;

    let user_id = create_test_user(&ctx.pool).await;

    let token = generate_token(user_id, "user").expect("Failed to generate token");

    let quiz = create_test_quiz_with_title(&ctx.pool, user_id, "Quiz to Delete").await;

    let req = test::TestRequest::delete()
        .uri(&format!("/api/quizzes/{}", quiz.id))
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT, "Expected 204 No Content for quiz deletion");

    // Verify quiz no longer exists
    let deleted_quiz = verify_quiz_in_db(&ctx.pool, quiz.id).await;
    assert!(deleted_quiz.is_none(), "Quiz should be deleted from database");

    cleanup_test_data(&ctx.pool).await;
}

#[actix_web::test]
async fn test_delete_quiz_unauthorized() {
    // Set up initial test context
    let ctx = setup_test_context().await;
    let app = setup_test_app(ctx.pool.clone()).await;

    // Create first test user
    let user_id = create_test_user(&ctx.pool).await;

    // Create a quiz with the first user
    let quiz = create_test_quiz_with_title(&ctx.pool, user_id, "Quiz to Delete").await;

    // Verify quiz exists in the database
    let quiz_exists = verify_quiz_in_db(&ctx.pool, quiz.id).await;
    assert!(quiz_exists.is_some(), "Quiz should exist in database");

    // Create second test user
    let other_user_id = create_test_user(&ctx.pool).await;

    // Generate token for the other user
    let other_token = generate_token(other_user_id, "user").expect("Failed to generate token");

    // Try to delete the quiz with the other user
    let req = test::TestRequest::delete()
        .uri(&format!("/api/quizzes/{}", quiz.id))
        .insert_header(("Authorization", format!("Bearer {}", other_token)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN, "Expected 403 Forbidden when trying to delete another user's quiz");

    // Verify quiz still exists
    let quiz_exists = verify_quiz_in_db(&ctx.pool, quiz.id).await;
    assert!(quiz_exists.is_some(), "Quiz should still exist in database");

    cleanup_test_data(&ctx.pool).await;
}
```

### quiz-app-backend/tests/db_test.rs
```rs
#[cfg(test)]
mod db_tests {
    use sqlx::PgPool;
    use quiz_app_backend::models::CreateUser;
    use chrono::Utc;
    use dotenv::dotenv;

    async fn setup() -> PgPool {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgPool::connect(&database_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_user_creation() {
        let pool = setup().await;

        let new_user = CreateUser {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let result = sqlx::query!(
            r#"
            INSERT INTO users (username, password_hash, created_at)
            VALUES ($1, $2, $3)
            RETURNING id, username, password_hash, created_at
            "#,
            new_user.username,
            new_user.password,
            Utc::now()
        )
        .fetch_one(&pool)
        .await;

        assert!(result.is_ok());
        
        // Cleanup
        let user = result.unwrap();
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user.id)
            .execute(&pool)
            .await;
    }

    #[tokio::test]
    async fn test_quiz_creation() {
        let pool = setup().await;

        // First create a user to be the creator
        let user_result = sqlx::query!(
            r#"
            INSERT INTO users (username, password_hash, created_at)
            VALUES ($1, $2, $3)
            RETURNING id, username, password_hash, role, created_at, updated_at
            "#,
            "quizcreator",
            "password123",
            Utc::now()
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to create test user");

        let title = "Test Quiz".to_string();
        let description = "Test Description".to_string();
        let created_at = Utc::now();

        let result = sqlx::query!(
            r#"
            INSERT INTO quizzes (title, description, created_by, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            title,
            description,
            user_result.id,
            created_at
        )
        .fetch_one(&pool)
        .await;

        if let Err(ref e) = result {
            eprintln!("Quiz creation failed: {:?}", e);
        }
        assert!(result.is_ok());
        
        // Cleanup
        if let Ok(quiz) = result {
            let _ = sqlx::query!("DELETE FROM quizzes WHERE id = $1", quiz.id)
                .execute(&pool)
                .await;
        }
        let _ = sqlx::query!("DELETE FROM users WHERE id = $1", user_result.id)
            .execute(&pool)
            .await;
    }
}
```

### quiz-app-backend/src/seeder.rs
```rs
use sqlx::PgPool;

use crate::{
    auth::hash_password,
    models::{User, Answer},
};

#[allow(dead_code)]
struct QuizData {
    title: String,
    description: String,
    questions: Vec<QuestionData>,
}

#[allow(dead_code)]
struct QuestionData {
    text: String,
    points: i32,
    answers: Vec<Answer>,
}

#[allow(dead_code)]
pub async fn seed_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Confirm that the connection pool uses the correct DATABASE_URL

    // Create a test user
    let password_hash = hash_password("password123").unwrap();
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash, role)
        VALUES ($1, $2, $3)
        RETURNING id, username, password_hash, role, created_at, updated_at
        "#,
        "testuser",
        password_hash,
        "user"
    )
    .fetch_one(pool)
    .await?;

    // Create a test quiz
    let quiz = sqlx::query!(
        r#"
        INSERT INTO quizzes (title, description, created_by)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        "Test Quiz",
        "A quiz for testing",
        user.id
    )
    .fetch_one(pool)
    .await?;

    // Create test questions
    let questions = vec![
        ("What is 2 + 2?", 0),
        ("What is the capital of France?", 1),
        ("Who wrote Romeo and Juliet?", 2),
    ];

    for (question_text, order_num) in questions {
        let question = sqlx::query!(
            r#"
            INSERT INTO questions (quiz_id, question_text, order_num)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            quiz.id,
            question_text,
            order_num
        )
        .fetch_one(pool)
        .await?;

        // Create answers for each question
        let answers = match order_num {
            0 => vec![
                ("4", true, 0),
                ("3", false, 1),
                ("5", false, 2),
                ("6", false, 3),
            ],
            1 => vec![
                ("Paris", true, 0),
                ("London", false, 1),
                ("Berlin", false, 2),
                ("Madrid", false, 3),
            ],
            2 => vec![
                ("William Shakespeare", true, 0),
                ("Charles Dickens", false, 1),
                ("Jane Austen", false, 2),
                ("Mark Twain", false, 3),
            ],
            _ => vec![],
        };

        for (text, is_correct, order_num) in answers {
            sqlx::query!(
                r#"
                INSERT INTO answers (question_id, text, is_correct, order_num)
                VALUES ($1, $2, $3, $4)
                "#,
                question.id,
                text,
                is_correct,
                order_num
            )
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
```

### quiz-app-backend/src/models.rs
```rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{auth, error::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub creator_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuiz {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuiz {
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub text: String,
    pub order_num: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestion {
    pub quiz_id: Uuid,
    pub text: String,
    pub order_num: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuestion {
    pub text: Option<String>,
    pub order_num: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub id: Uuid,
    pub question_id: Uuid,
    pub text: String,
    pub is_correct: bool,
    pub order_num: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAnswer {
    pub question_id: Uuid,
    pub text: String,
    pub is_correct: bool,
    pub order_num: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAnswer {
    pub text: Option<String>,
    pub is_correct: Option<bool>,
    pub order_num: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuizAttempt {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub user_id: Uuid,
    pub score: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuizAttempt {
    pub quiz_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttemptAnswer {
    pub id: Uuid,
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_id: Uuid,
    pub is_correct: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAttemptAnswer {
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub answer_id: Uuid,
}

impl User {
    pub async fn create(pool: &PgPool, user: CreateUser) -> Result<User, AppError> {
        let password_hash = auth::hash_password(&user.password).await?;
        
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, role, created_at, updated_at)
            VALUES ($1, $2, $3, 'user', $4, $5)
            RETURNING id, username, email, password_hash, role, created_at, updated_at
            "#,
            user.username,
            user.email,
            password_hash,
            Utc::now(),
            Utc::now(),
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<User, AppError> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn update(pool: &PgPool, id: Uuid, user: UpdateUser) -> Result<User, AppError> {
        let current_user = Self::find_by_id(pool, id).await?;
        
        let password_hash = match user.password {
            Some(password) => Some(auth::hash_password(&password).await?),
            None => None,
        };

        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET 
                username = COALESCE($1, username),
                email = COALESCE($2, email),
                password_hash = COALESCE($3, password_hash),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $4
            RETURNING id, username, email, password_hash, role, created_at, updated_at
            "#,
            user.username,
            user.email,
            password_hash,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}

impl Quiz {
    pub async fn create(pool: &PgPool, creator_id: Uuid, quiz: CreateQuiz) -> Result<Quiz, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            INSERT INTO quizzes (title, description, creator_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, title, description, creator_id, created_at, updated_at
            "#,
            quiz.title,
            quiz.description,
            creator_id,
            Utc::now(),
            Utc::now(),
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Quiz, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            SELECT id, title, description, creator_id, created_at, updated_at
            FROM quizzes
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_creator(pool: &PgPool, creator_id: Uuid) -> Result<Vec<Quiz>, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            SELECT id, title, description, creator_id, created_at, updated_at
            FROM quizzes
            WHERE creator_id = $1
            "#,
            creator_id
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Quiz>, AppError> {
        let quizzes = sqlx::query_as!(
            Quiz,
            r#"
            SELECT id, title, description, creator_id, created_at, updated_at
            FROM quizzes
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(quizzes)
    }

    pub async fn update(pool: &PgPool, id: Uuid, quiz: UpdateQuiz) -> Result<Quiz, AppError> {
        sqlx::query_as!(
            Quiz,
            r#"
            UPDATE quizzes
            SET 
                title = COALESCE($1, title),
                description = COALESCE($2, description),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            RETURNING id, title, description, creator_id, created_at, updated_at
            "#,
            quiz.title,
            quiz.description,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM quizzes
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}

impl Question {
    pub async fn create(pool: &PgPool, question: CreateQuestion) -> Result<Question, AppError> {
        sqlx::query_as!(
            Question,
            r#"
            INSERT INTO questions (quiz_id, text, order_num, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, quiz_id, text, order_num, created_at, updated_at
            "#,
            question.quiz_id,
            question.text,
            question.order_num,
            Utc::now(),
            Utc::now(),
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_quiz(pool: &PgPool, quiz_id: Uuid) -> Result<Vec<Question>, AppError> {
        sqlx::query_as!(
            Question,
            r#"
            SELECT id, quiz_id, text, order_num, created_at, updated_at
            FROM questions
            WHERE quiz_id = $1
            ORDER BY order_num
            "#,
            quiz_id
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Question, AppError> {
        let question = sqlx::query_as!(Question,
            r#"
            SELECT id, quiz_id, text, order_num, created_at, updated_at
            FROM questions
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(question)
    }

    pub async fn update(pool: &PgPool, id: Uuid, question: UpdateQuestion) -> Result<Question, AppError> {
        sqlx::query_as!(
            Question,
            r#"
            UPDATE questions
            SET 
                text = COALESCE($1, text),
                order_num = COALESCE($2, order_num),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $3
            RETURNING id, quiz_id, text, order_num, created_at, updated_at
            "#,
            question.text,
            question.order_num,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM questions
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}

impl Answer {
    pub async fn create(pool: &PgPool, answer: CreateAnswer) -> Result<Answer, AppError> {
        sqlx::query_as!(
            Answer,
            r#"
            INSERT INTO answers (question_id, text, is_correct, order_num, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, question_id, text, is_correct, order_num, created_at, updated_at
            "#,
            answer.question_id,
            answer.text,
            answer.is_correct,
            answer.order_num,
            Utc::now(),
            Utc::now(),
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_question(pool: &PgPool, question_id: Uuid) -> Result<Vec<Answer>, AppError> {
        sqlx::query_as!(
            Answer,
            r#"
            SELECT id, question_id, text, is_correct, order_num, created_at, updated_at
            FROM answers
            WHERE question_id = $1
            ORDER BY order_num
            "#,
            question_id
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Answer, AppError> {
        let answer = sqlx::query_as!(Answer,
            r#"
            SELECT id, question_id, text, is_correct, order_num, created_at, updated_at
            FROM answers
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(answer)
    }

    pub async fn update(pool: &PgPool, id: Uuid, answer: UpdateAnswer) -> Result<Answer, AppError> {
        sqlx::query_as!(
            Answer,
            r#"
            UPDATE answers
            SET 
                text = COALESCE($1, text),
                is_correct = COALESCE($2, is_correct),
                order_num = COALESCE($3, order_num),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $4
            RETURNING id, question_id, text, is_correct, order_num, created_at, updated_at
            "#,
            answer.text,
            answer.is_correct,
            answer.order_num,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            DELETE FROM answers
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}

impl QuizAttempt {
    pub async fn create(pool: &PgPool, user_id: Uuid, attempt: CreateQuizAttempt) -> Result<QuizAttempt, AppError> {
        sqlx::query_as!(
            QuizAttempt,
            r#"
            INSERT INTO quiz_attempts (quiz_id, user_id)
            VALUES ($1, $2)
            RETURNING id, quiz_id, user_id, score, completed_at, created_at, updated_at
            "#,
            attempt.quiz_id,
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<QuizAttempt>, AppError> {
        sqlx::query_as!(
            QuizAttempt,
            r#"
            SELECT id, quiz_id, user_id, score, completed_at, created_at, updated_at
            FROM quiz_attempts
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn complete(pool: &PgPool, id: Uuid, score: i32) -> Result<QuizAttempt, AppError> {
        sqlx::query_as!(
            QuizAttempt,
            r#"
            UPDATE quiz_attempts
            SET 
                score = $1,
                completed_at = CURRENT_TIMESTAMP,
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            RETURNING id, quiz_id, user_id, score, completed_at, created_at, updated_at
            "#,
            score,
            id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }
}

impl AttemptAnswer {
    pub async fn create(pool: &PgPool, answer: CreateAttemptAnswer) -> Result<AttemptAnswer, AppError> {
        // First, verify if the answer is correct
        let is_correct = sqlx::query_scalar!(
            r#"
            SELECT is_correct
            FROM answers
            WHERE id = $1 AND question_id = $2
            "#,
            answer.answer_id,
            answer.question_id
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)?;

        sqlx::query_as!(
            AttemptAnswer,
            r#"
            INSERT INTO attempt_answers (attempt_id, question_id, answer_id, is_correct)
            VALUES ($1, $2, $3, $4)
            RETURNING id, attempt_id, question_id, answer_id, is_correct, created_at
            "#,
            answer.attempt_id,
            answer.question_id,
            answer.answer_id,
            is_correct
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::from)
    }

    pub async fn find_by_attempt(pool: &PgPool, attempt_id: Uuid) -> Result<Vec<AttemptAnswer>, AppError> {
        sqlx::query_as!(
            AttemptAnswer,
            r#"
            SELECT id, attempt_id, question_id, answer_id, is_correct, created_at
            FROM attempt_answers
            WHERE attempt_id = $1
            "#,
            attempt_id
        )
        .fetch_all(pool)
        .await
        .map_err(AppError::from)
    }
}
```

### quiz-app-backend/src/tests/test_helpers.rs
```rs
use actix_web::{
    test,
    web,
    App,
    Error,
    dev::{Service, ServiceResponse, ServiceRequest},
};
use sqlx::PgPool;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::{
    routes::configure_routes,
    models::{User, Quiz, CreateQuiz},
    auth::generate_token,
};

#[derive(Debug, Deserialize)]
pub struct TestUser {
    pub id: i32,
    pub username: String,
    pub token: String,
}

pub struct TestContext {
    pub pool: PgPool,
    pub user_id: i32,
    pub token: String,
    pub app: Box<dyn Service<actix_http::Request, Response = ServiceResponse, Error = Error, Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<ServiceResponse, Error>> + 'static>>> + 'static,
}

pub fn init_test_env() {
    std::env::set_var("JWT_SECRET", "test_secret_key_for_quiz_app_tests");
    std::env::set_var("DATABASE_URL", "postgres://postgres:password@localhost:5432/quiz_app_test");
}

pub async fn create_test_user(pool: &PgPool) -> (i32, String) {
    let mut attempt = 0;
    let max_attempts = 5;

    while attempt < max_attempts {
        let username = format!("testuser_{}", attempt);
        let password_hash = bcrypt::hash("testpass", bcrypt::DEFAULT_COST).unwrap();

        match sqlx::query!(
            "INSERT INTO users (username, password_hash, role) VALUES ($1, $2, 'user') RETURNING id",
            username,
            password_hash
        )
        .fetch_one(pool)
        .await {
            Ok(record) => {
                let token = generate_token(record.id, "user").unwrap();
                return (record.id, token);
            }
            Err(_) => attempt += 1,
        }
    }

    panic!("Failed to create unique test user after multiple attempts");
}

pub async fn setup_test_app(pool: PgPool) -> impl Service<actix_http::Request, Response = ServiceResponse, Error = Error> + 'static {
    test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    ).await
}

pub async fn create_test_quiz(ctx: &TestContext) -> Quiz {
    let quiz = CreateQuiz {
        title: "Test Quiz".to_string(),
        description: Some("A test quiz".to_string()),
        created_by: ctx.user_id,
    };

    let req = test::TestRequest::post()
        .uri("/api/quizzes")
        .insert_header(("Authorization", format!("Bearer {}", ctx.token)))
        .set_json(&quiz)
        .to_request();

    test::call_and_read_body_json(&ctx.app, req).await
}

pub async fn setup_test_context(pool: PgPool) -> TestContext {
    let (user_id, token) = create_test_user(&pool).await;
    let app = setup_test_app(pool.clone()).await;

    TestContext {
        pool,
        user_id,
        token,
        app: Box::new(app),
    }
}
```

### quiz-app-backend/src/middleware/test_context.rs
```rs

use sqlx::PgPool;
use chrono;
use crate::{
    routes::configure_routes,
    models::Quiz,
    config::get_config,
    auth::generate_token,
};
use actix_web::{
    test, 
    web, 
    App,
    dev::{Service, ServiceResponse},
    Error,
};
use actix_http::Request;
use uuid::Uuid;
use bcrypt;

pub struct TestContext {
    // ...existing code...
}

pub fn init_test_env() {
    // ...existing code...
}

pub async fn setup_test_context() -> TestContext {
    init_test_env();
    let config = get_config().expect("Failed to load config");
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    
    cleanup_test_data(&pool).await;

    let now = chrono::Utc::now();
    let unique_id = Uuid::new_v4();
    let username = format!("test_user_{}", unique_id);
    let password = bcrypt::hash("test_password123", bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");
    
    let user_id = sqlx::query!(
        r#"
        INSERT INTO users (username, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $3)
        RETURNING id
        "#,
        username,
        password,
        now
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to create test user")
    .id;

    TestContext {
        // ...existing code...
    }
}

pub async fn cleanup_test_data(pool: &PgPool) {
    sqlx::query!("DELETE FROM quizzes").execute(pool).await.expect("Failed to clean up quizzes");
    sqlx::query!("DELETE FROM users").execute(pool).await expect("Failed to clean up users");
}```

### quiz-app-backend/src/middleware/cache.rs
```rs
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};

pub struct CacheMiddleware;

impl<S> Transform<S, ServiceRequest> for CacheMiddleware 
where 
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = CacheMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CacheMiddlewareService { service })
    }
}

pub struct CacheMiddlewareService<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for CacheMiddlewareService<S>
where 
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // TODO: Implement actual caching logic
        // This is a placeholder that just passes the request through
        self.service.call(req)
    }
}
```

### quiz-app-backend/src/middleware/mod.rs
```rs
pub mod cache;
#[allow(unused_imports)]
pub use cache::CacheMiddleware;
```

### quiz-app-backend/src/config.rs
```rs
use std::env;
use crate::error::AppError;

#[derive(Clone, Debug)]
pub struct Config {
    pub jwt_secret: String,
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub workers: usize,
    pub log_level: String,
}

impl Config {
    pub fn new() -> Result<Self, AppError> {
        Ok(Config {
            jwt_secret: env::var("JWT_SECRET").map_err(|_| AppError::ConfigError("JWT_SECRET not set".to_string()))?,
            database_url: env::var("DATABASE_URL").map_err(|_| AppError::ConfigError("DATABASE_URL not set".to_string()))?,
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            workers: env::var("WORKERS")
                .unwrap_or_else(|_| num_cpus::get().to_string())
                .parse()
                .unwrap_or(num_cpus::get()),
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        })
    }
}

pub fn get_config() -> Result<Config, env::VarError> {
    Config::new()
}

pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://remixonwin:600181@localhost/quiz_app".to_string())
}

pub fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        "super_secure_jwt_secret_key".to_string() // Ensure a strong default
    })
}
```

### quiz-app-backend/src/lib.rs
```rs
pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod seeder;
pub mod utils;

pub mod test_helpers;

#[cfg(test)]
mod tests {
    // Internal test modules can go here
}
```

### quiz-app-backend/src/test_helpers.rs
```rs
use actix_web::{web, App};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    handlers::{
        user,
        quiz,
    },
    models::{Quiz, User},
    config::get_config,
    auth::generate_token,
};

pub struct TestContext {
    pub pool: PgPool,
    pub user_id: Uuid, 
    pub token: String,
}

pub fn init_test_env() {
    std::env::set_var("JWT_SECRET", "test_secret_key_for_quiz_app_tests");
}

pub async fn create_test_user(pool: &PgPool, username: &str, password: &str) -> Result<Uuid, sqlx::Error> { 
    let now = Utc::now().naive_utc();
    let password_hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");
    
    let user_id: Uuid = sqlx::query_scalar!(
        r#"
        INSERT INTO users (username, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, 'test', $3, $3)
        RETURNING id
        "#,
        username,
        password_hash,
        now
    )
    .fetch_one(pool)
    .await?;

    Ok(user_id)
}

pub async fn create_test_quiz(pool: &PgPool, user_id: Uuid) -> Uuid { 
    let now = Utc::now().naive_utc(); 
    let quiz_id: Uuid = sqlx::query_scalar!(
        r#"
        INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id
        "#,
        "Test Quiz",
        "A quiz for testing",
        user_id,
        now
    )
    .fetch_one(pool)
    .await
    .unwrap();

    quiz_id
}

pub async fn create_test_quiz_with_title(pool: &PgPool, user_id: Uuid, title: &str) -> Quiz {
    let now = Utc::now().naive_utc(); 
    let quiz = sqlx::query_as!(
        Quiz,
        r#"
        INSERT INTO quizzes (title, description, created_by, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $4)
        RETURNING id, title, description, created_by, created_at, updated_at
        "#,
        title,
        "A test quiz description",
        user_id,
        now
    )
    .fetch_one(pool)
    .await
    .expect("Failed to create test quiz");

    quiz
}

pub async fn cleanup_test_data(pool: &PgPool) {
    // Delete all quizzes first due to foreign key constraints
    match sqlx::query!("DELETE FROM quizzes")
        .execute(pool)
        .await
    {
        Ok(_) => (),
        Err(e) => eprintln!("Error cleaning up quizzes: {}", e),
    }

    // Then delete all test users
    match sqlx::query!("DELETE FROM users")
        .execute(pool)
        .await
    {
        Ok(_) => (),
        Err(e) => eprintln!("Error cleaning up users: {}", e),
    }

    // Verify cleanup
    let quiz_count = sqlx::query!("SELECT COUNT(*) as count FROM quizzes")
        .fetch_one(pool)
        .await
        .expect("Failed to count quizzes")
        .count
        .unwrap_or(0);

    let user_count = sqlx::query!("SELECT COUNT(*) as count FROM users")
        .fetch_one(pool)
        .await
        .expect("Failed to count users")
        .count
        .unwrap_or(0);

    assert_eq!(quiz_count, 0, "Failed to clean up all quizzes");
    assert_eq!(user_count, 0, "Failed to clean up all users");
}

pub async fn verify_quiz_in_db(pool: &PgPool, quiz_id: Uuid) -> Option<Quiz> {
    sqlx::query_as!(
        Quiz,
        r#"
        SELECT id, title, description, created_by, created_at, updated_at
        FROM quizzes 
        WHERE id = $1
        "#,
        quiz_id
    )
    .fetch_optional(pool)
    .await
    .expect("Failed to query database")
}

pub async fn setup_test_app(pool: PgPool) {
    let _app = actix_web::test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api/users")
                    .service(user::register)
                    .service(user::login)
                    .service(user::update_profile)
            )
            .service(
                web::scope("/api/quizzes")
                    .service(quiz::get_quizzes)
                    .service(quiz::create_quiz)
                    .service(quiz::get_quiz)
                    .service(quiz::update_quiz)
                    .service(quiz::delete_quiz)
                    .service(quiz::submit_quiz)
            )
    )
    .await;
}

pub async fn setup_test_context() -> TestContext {
    init_test_env();
    let config = get_config().expect("Failed to load config");
    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    
    // Clean up any existing test data
    sqlx::query!("DELETE FROM quizzes")
        .execute(&pool)
        .await
        .expect("Failed to clean up quizzes");

    sqlx::query!("DELETE FROM users")
        .execute(&pool)
        .await
        .expect("Failed to clean up users");

    // Create test user
    let now = Utc::now().naive_utc(); 
    let username = format!("test_user_{}", uuid::Uuid::new_v4());
    let password = bcrypt::hash("test_password123", bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");
    
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, 'test', $3, $3)
        RETURNING id, username, password_hash, role, created_at, updated_at
        "#,
        username,
        password,
        now
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to create test user");

    // Verify user was created
    let user_exists = sqlx::query!(
        "SELECT id FROM users WHERE id = $1",
        user.id
    )
    .fetch_optional(&pool)
    .await
    .expect("Failed to verify user creation");

    assert!(user_exists.is_some(), "Test user was not created successfully");
    
    let token = generate_token(user.id, "user").expect("Failed to generate token");
    
    TestContext {
        pool,
        user_id: user.id,
        token,
    }
}
```

### quiz-app-backend/src/db.rs
```rs
// Removed unused imports

pub async fn init_db(pool: &sqlx::postgres::PgPool) -> Result<(), sqlx::Error> {
    // Skip database initialization if SKIP_MIGRATIONS is set
    if std::env::var("SKIP_MIGRATIONS").is_ok() {
        println!("Skipping database migrations due to SKIP_MIGRATIONS environment variable");
        return Ok(());
    }

    // Create UUID extension if it doesn't exist
    sqlx::query!("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"").execute(pool).await?;

    // Drop existing tables and triggers in reverse order of dependencies
    sqlx::query!("DROP TRIGGER IF EXISTS update_quizzes_updated_at ON quizzes CASCADE").execute(pool).await?;
    sqlx::query!("DROP FUNCTION IF EXISTS update_updated_at_column CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS submitted_answers CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS quiz_attempts CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS answers CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS questions CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS quizzes CASCADE").execute(pool).await?;
    sqlx::query!("DROP TABLE IF EXISTS users CASCADE").execute(pool).await?;

    // Create users table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            username VARCHAR(255) NOT NULL UNIQUE,
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create quizzes table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS quizzes (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            title VARCHAR(255) NOT NULL,
            description TEXT,
            creator_id UUID NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create trigger function for automatically updating updated_at
    sqlx::query!(
        r#"
        CREATE OR REPLACE FUNCTION update_updated_at_column()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = CURRENT_TIMESTAMP;
            RETURN NEW;
        END;
        $$ language 'plpgsql'
        "#
    )
    .execute(pool)
    .await?;

    // Drop trigger if exists
    sqlx::query!(
        r#"
        DROP TRIGGER IF EXISTS update_quizzes_updated_at ON quizzes
        "#
    )
    .execute(pool)
    .await?;

    // Create trigger
    sqlx::query!(
        r#"
        CREATE TRIGGER update_quizzes_updated_at
            BEFORE UPDATE ON quizzes
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at_column()
        "#
    )
    .execute(pool)
    .await?;

    // Create questions table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS questions (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            quiz_id UUID NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
            text TEXT NOT NULL,
            question_type VARCHAR(50) NOT NULL,
            order_num INTEGER NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(quiz_id, order_num)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create answers table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS answers (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
            text TEXT NOT NULL,
            is_correct BOOLEAN NOT NULL,
            order_num INTEGER NOT NULL,
            UNIQUE(question_id, order_num)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create quiz_attempts table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS quiz_attempts (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            quiz_id UUID NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
            user_id UUID NOT NULL REFERENCES users(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            completed_at TIMESTAMPTZ,
            score INTEGER,
            UNIQUE(quiz_id, user_id, created_at)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Create submitted_answers table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS submitted_answers (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            attempt_id UUID NOT NULL REFERENCES quiz_attempts(id) ON DELETE CASCADE,
            question_id UUID NOT NULL REFERENCES questions(id),
            answer_id UUID NOT NULL REFERENCES answers(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(attempt_id, question_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_test_pool() -> Result<PgPool, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        create_pool(&database_url).await
    }

    #[tokio::test]
    async fn test_database_connection() {
        let pool = create_test_pool().await.expect("Failed to create pool");
        let result = sqlx::query!("SELECT 1").fetch_one(&pool).await;
        assert!(result.is_ok());
    }
}
```

### quiz-app-backend/src/auth.rs
```rs
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error,
    FromRequest,
    HttpMessage,
    HttpRequest,
    HttpResponse,
    body::EitherBody,
    http::header::AUTHORIZATION,
    web,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use chrono::{Duration, Utc};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use std::rc::Rc;

use crate::{config::Config, error::AppError, models::User};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub user_id: Uuid,
    pub role: String,
    pub exp: i64,
}

pub trait AuthService {
    fn generate_token(config: &Config, user: &User) -> Result<String, AppError>;
    fn decode_token(token: &str, config: &Config) -> Result<Claims, AppError>;
}

pub struct JwtAuthService;

impl AuthService for JwtAuthService {
    fn generate_token(config: &Config, user: &User) -> Result<String, AppError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            user_id: user.id,
            role: user.role.clone(),
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::TokenCreationError)
    }

    fn decode_token(token: &str, config: &Config) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::InvalidToken)
    }
}

pub async fn generate_token(user: &User) -> Result<String, AppError> {
    let config = Config::new()?;
    Ok(JwtAuthService::generate_token(&config, user)?)
}

pub async fn hash_password(password: &str) -> Result<String, AppError> {
    web::block(move || hash(password.as_bytes(), DEFAULT_COST))
        .await
        .map_err(AppError::from)?
        .map_err(|_| AppError::PasswordHashingError)
}

pub async fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    web::block(move || verify(password.as_bytes(), hash))
        .await
        .map_err(AppError::from)?
        .map_err(|_| AppError::PasswordVerificationError)
}

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers()
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let token = match auth_header {
            Some(auth_str) if auth_str.starts_with("Bearer ") => {
                auth_str[7..].to_string()
            }
            _ => return ready(Err(ErrorUnauthorized(json!({ 
                "error": "No valid authorization token" 
            })))),
        };

        let config = match Config::new() {
            Ok(config) => config,
            Err(_) => return ready(Err(ErrorUnauthorized(json!({
                "error": "Server configuration error"
            })))),
        };

        match JwtAuthService::decode_token(&token, &config) {
            Ok(claims) => {
                req.extensions_mut().insert(claims.clone());
                ready(Ok(claims))
            }
            Err(_) => ready(Err(ErrorUnauthorized(json!({
                "error": "Invalid token"
            })))),
        }
    }
}

pub struct JwtMiddleware<S> {
    service: Rc<S>,
}

impl<S> JwtMiddleware<S> {
    pub fn new(service: S) -> Self {
        JwtMiddleware {
            service: Rc::new(service),
        }
    }
}

impl<S, B> Service<ServiceRequest> for JwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            let config = match Config::new() {
                Ok(config) => config,
                Err(_) => {
                    return Ok(req.into_response(
                        HttpResponse::InternalServerError()
                            .json(json!({
                                "error": "Server configuration error"
                            }))
                            .map_into_right_body(),
                    ))
                }
            };

            let auth_header = req
                .headers()
                .get(AUTHORIZATION)
                .and_then(|h| h.to_str().ok());

            let token = match auth_header {
                Some(auth_str) if auth_str.starts_with("Bearer ") => auth_str[7..].to_string(),
                _ => {
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(json!({
                                "error": "Missing authorization token"
                            }))
                            .map_into_right_body(),
                    ))
                }
            };

            match JwtAuthService::decode_token(&token, &config) {
                Ok(claims) => {
                    req.extensions_mut().insert(claims);
                    let res = svc.call(req).await?;
                    Ok(res.map_into_left_body())
                }
                Err(_) => Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(json!({
                            "error": "Invalid token"
                        }))
                        .map_into_right_body(),
                )),
            }
        })
    }
}
```

### quiz-app-backend/src/bin/seed.rs
```rs
use quiz_app_backend::{config, seeder};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get database URL from config
    let database_url = config::get_database_url();

    // Create connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Run seeder
    seeder::seed_database(&pool).await?;

    Ok(())
}
```

### quiz-app-backend/src/handlers/user.rs
```rs
use actix_web::{web, HttpResponse, Responder, post, get, put, delete};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth,
    error::AppError,
    models::{CreateUser, UpdateUser, User},
};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    user_data: web::Json<CreateUser>,
) -> Result<impl Responder, AppError> {
    let user = User::create(&pool, user_data.0).await?;
    let token = auth::generate_token(&user).await?;

    Ok(HttpResponse::Ok().json(json!({
        "token": token,
        "user": user
    })))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    login_data: web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    let user = User::find_by_email(&pool, &login_data.email).await?;
    
    if !auth::verify_password(&login_data.password, &user.password_hash).await? {
        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let token = auth::generate_token(&user).await?;

    Ok(HttpResponse::Ok().json(json!({
        "token": token,
        "user": user
    })))
}

#[get("/profile")]
pub async fn get_profile(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    let user = User::find_by_id(&pool, user_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/profile")]
pub async fn update_profile(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<Uuid>,
    user_data: web::Json<UpdateUser>,
) -> Result<impl Responder, AppError> {
    let user = User::update(&pool, user_id.into_inner(), user_data.0).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/profile")]
pub async fn delete_profile(
    pool: web::Data<PgPool>,
    user_id: web::ReqData<Uuid>,
) -> Result<impl Responder, AppError> {
    User::delete(&pool, user_id.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
```

### quiz-app-backend/src/handlers/answer.rs
```rs
use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::Claims,
    error::AppError,
    models::{Answer, Question, Quiz, CreateAnswer, UpdateAnswer},
};

#[post("")]
pub async fn create_answer(
    pool: web::Data<PgPool>,
    claims: Claims,
    form: web::Json<CreateAnswer>,
) -> Result<HttpResponse, AppError> {
    let question = Question::find_by_id(&pool, form.question_id).await?;
    let quiz = Quiz::find_by_id(&pool, question.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let answer = Answer::create(&pool, question.id, form.into_inner()).await?;
    Ok(HttpResponse::Created().json(answer))
}

#[get("/{question_id}/answers")]
pub async fn get_answers(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let answers = Answer::find_by_question(&pool, question_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(answers))
}

#[get("/{answer_id}")]
pub async fn get_answer(
    pool: web::Data<PgPool>,
    answer_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let answer = Answer::find_by_id(&pool, answer_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(answer))
}

#[put("/{answer_id}")]
pub async fn update_answer(
    pool: web::Data<PgPool>,
    answer_id: web::Path<Uuid>,
    claims: Claims,
    form: web::Json<UpdateAnswer>,
) -> Result<HttpResponse, AppError> {
    let answer_id = answer_id.into_inner();
    let answer = Answer::find_by_id(&pool, answer_id).await?;
    let question = Question::find_by_id(&pool, answer.question_id).await?;
    let quiz = Quiz::find_by_id(&pool, question.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let answer = Answer::update(&pool, answer_id, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(answer))
}

#[delete("/{answer_id}")]
pub async fn delete_answer(
    pool: web::Data<PgPool>,
    answer_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let answer_id = answer_id.into_inner();
    let answer = Answer::find_by_id(&pool, answer_id).await?;
    let question = Question::find_by_id(&pool, answer.question_id).await?;
    let quiz = Quiz::find_by_id(&pool, question.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    Answer::delete(&pool, answer_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
```

### quiz-app-backend/src/handlers/question.rs
```rs
use actix_web::{get, post, put, delete, web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::Claims,
    error::AppError,
    models::{Question, Quiz, Answer, CreateQuestion, UpdateQuestion},
};

#[post("")]
pub async fn create_question(
    pool: web::Data<PgPool>,
    claims: Claims,
    form: web::Json<CreateQuestion>,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::find_by_id(&pool, form.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let question = Question::create(&pool, form.into_inner()).await?;
    Ok(HttpResponse::Created().json(question))
}

#[get("/{quiz_id}/questions")]
pub async fn get_questions(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let questions = Question::find_by_quiz(&pool, quiz_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(questions))
}

#[get("/{question_id}")]
pub async fn get_question(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let question = Question::find_by_id(&pool, question_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(question))
}

#[put("/{question_id}")]
pub async fn update_question(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
    claims: Claims,
    form: web::Json<UpdateQuestion>,
) -> Result<HttpResponse, AppError> {
    let question_id = question_id.into_inner();
    let question = Question::find_by_id(&pool, question_id).await?;
    let quiz = Quiz::find_by_id(&pool, question.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let question = Question::update(&pool, question_id, form.into_inner()).await?;
    Ok(HttpResponse::Ok().json(question))
}

#[delete("/{question_id}")]
pub async fn delete_question(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let question_id = question_id.into_inner();
    let question = Question::find_by_id(&pool, question_id).await?;
    let quiz = Quiz::find_by_id(&pool, question.quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    Question::delete(&pool, question_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/{question_id}/answers")]
pub async fn get_answers(
    pool: web::Data<PgPool>,
    question_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let answers = Answer::get_by_question(&pool, question_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(answers))
}
```

### quiz-app-backend/src/handlers/auth.rs
```rs
use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    auth::{self, Claims, generate_token, hash_password, verify_password},
    error::AppError,
    models::{CreateUser, LoginCredentials, User},
};

#[derive(serde::Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[post("/register")]
pub async fn register(
    pool: web::Data<PgPool>,
    form: web::Json<CreateUser>,
) -> Result<HttpResponse, AppError> {
    // Hash the password
    let password_hash = hash_password(&form.password).await?;

    // Create the user
    let user = User::create(&pool, CreateUser {
        username: form.username.clone(),
        email: form.email.clone(),
        password: password_hash,
    }).await?;

    // Generate a token
    let token = generate_token(&user)?;

    Ok(HttpResponse::Ok().json(json!({
        "user": user,
        "token": token
    })))
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginCredentials>,
) -> Result<HttpResponse, AppError> {
    // Find the user by username
    let user = User::get_by_username(&pool, &credentials.username).await
        .map_err(|_| AppError::InvalidCredentials)?;

    // Verify the password
    let is_valid = verify_password(&credentials.password, &user.password_hash).await?;
    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    // Generate a token
    let token = generate_token(&user)?;

    Ok(HttpResponse::Ok().json(json!({
        "user": user,
        "token": token
    })))
}

pub async fn me(claims: Claims, pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let user = User::get_by_id(&pool, claims.user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}
```

### quiz-app-backend/src/handlers/mod.rs
```rs
pub mod user;
pub mod quiz;
pub mod question;
pub mod answer;
```

### quiz-app-backend/src/handlers/quiz_attempt.rs
```rs
use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    auth::Claims,
    error::AppError,
    models::{QuizAttempt, CreateQuizAttempt, SubmitAnswer, Quiz},
};

pub async fn start_quiz_attempt(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    // Check if quiz exists
    let quiz = Quiz::get_by_id(&pool, quiz_id.into_inner()).await?;
    
    // Create a new quiz attempt
    let quiz_attempt = QuizAttempt::create(&pool, CreateQuizAttempt {
        quiz_id: quiz.id,
        user_id: claims.user_id,
        started_at: Utc::now(),
    }).await?;

    Ok(HttpResponse::Created().json(quiz_attempt))
}

pub async fn get_quiz_attempt(
    pool: web::Data<PgPool>,
    attempt_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let attempt = QuizAttempt::get_by_id(&pool, attempt_id.into_inner()).await?;
    
    // Check if user owns the attempt
    if attempt.user_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz attempt".to_string()));
    }

    Ok(HttpResponse::Ok().json(attempt))
}

pub async fn submit_answer(
    pool: web::Data<PgPool>,
    attempt_id: web::Path<Uuid>,
    claims: Claims,
    answer_data: web::Json<SubmitAnswer>,
) -> Result<HttpResponse, AppError> {
    let attempt = QuizAttempt::get_by_id(&pool, attempt_id.into_inner()).await?;
    
    // Check if user owns the attempt
    if attempt.user_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz attempt".to_string()));
    }

    // Submit the answer
    let submitted_answer = QuizAttempt::submit_answer(
        &pool,
        attempt.id,
        answer_data.into_inner()
    ).await?;

    Ok(HttpResponse::Ok().json(submitted_answer))
}

pub async fn finish_quiz_attempt(
    pool: web::Data<PgPool>,
    attempt_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let attempt = QuizAttempt::get_by_id(&pool, attempt_id.into_inner()).await?;
    
    // Check if user owns the attempt
    if attempt.user_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz attempt".to_string()));
    }

    // Finish the attempt and calculate score
    let completed_attempt = QuizAttempt::finish(
        &pool,
        attempt.id,
        Utc::now(),
    ).await?;

    Ok(HttpResponse::Ok().json(completed_attempt))
}

pub async fn get_user_quiz_attempts(
    pool: web::Data<PgPool>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let attempts = QuizAttempt::get_by_user_id(&pool, claims.user_id).await?;
    Ok(HttpResponse::Ok().json(attempts))
}

pub async fn get_quiz_attempts_by_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    // First check if user owns the quiz
    let quiz = Quiz::get_by_id(&pool, quiz_id.into_inner()).await?;
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let attempts = QuizAttempt::get_by_quiz_id(&pool, quiz.id).await?;
    Ok(HttpResponse::Ok().json(attempts))
}
```

### quiz-app-backend/src/handlers/quiz.rs
```rs
use actix_web::{get, post, put, delete, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    auth::Claims,
    error::AppError,
    models::{Quiz, CreateQuiz, UpdateQuiz},
};

#[get("")]
pub async fn get_quizzes(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let quizzes = Quiz::get_all(&pool).await?;
    Ok(HttpResponse::Ok().json(quizzes))
}

#[post("")]
pub async fn create_quiz(
    pool: web::Data<PgPool>,
    claims: Claims,
    quiz_data: web::Json<CreateQuiz>,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::create(&pool, claims.user_id, quiz_data.into_inner()).await?;
    Ok(HttpResponse::Created().json(quiz))
}

#[get("/{quiz_id}")]
pub async fn get_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::find_by_id(&pool, quiz_id.into_inner()).await?;
    Ok(HttpResponse::Ok().json(quiz))
}

#[put("/{quiz_id}")]
pub async fn update_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
    quiz_data: web::Json<UpdateQuiz>,
) -> Result<HttpResponse, AppError> {
    let quiz_id = quiz_id.into_inner();
    let quiz = Quiz::find_by_id(&pool, quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    let quiz = Quiz::update(&pool, quiz_id, quiz_data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(quiz))
}

#[delete("/{quiz_id}")]
pub async fn delete_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz_id = quiz_id.into_inner();
    let quiz = Quiz::find_by_id(&pool, quiz_id).await?;
    
    // Check if user owns the quiz
    if quiz.creator_id != claims.user_id {
        return Err(AppError::Forbidden("You do not own this quiz".to_string()));
    }

    Quiz::delete(&pool, quiz_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[post("/{quiz_id}/submit")]
pub async fn submit_quiz(
    pool: web::Data<PgPool>,
    quiz_id: web::Path<Uuid>,
    claims: Claims,
) -> Result<HttpResponse, AppError> {
    let quiz = Quiz::find_by_id(&pool, quiz_id.into_inner()).await?;
    Ok(HttpResponse::Created().json(quiz))
}
```

### quiz-app-backend/src/utils.rs
```rs
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|_| AppError::HashError)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash)
        .map_err(|_| AppError::HashError)
}
```

### quiz-app-backend/src/error.rs
```rs
use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use serde_json::json;
use sqlx::error::Error as SqlxError;
use std::convert::From;
use tokio::task::JoinError;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized: {}", _0)]
    Unauthorized(String),

    #[display(fmt = "Not Found: {}", _0)]
    NotFound(String),

    #[display(fmt = "Database Error: {}", _0)]
    DatabaseError(String),

    #[display(fmt = "Configuration Error: {}", _0)]
    ConfigError(String),

    #[display(fmt = "Token Creation Error")]
    TokenCreationError,

    #[display(fmt = "Invalid Token")]
    InvalidToken,

    #[display(fmt = "Password Hashing Error")]
    PasswordHashingError,

    #[display(fmt = "Password Verification Error")]
    PasswordVerificationError,

    #[display(fmt = "Forbidden: {}", _0)]
    Forbidden(String),

    #[display(fmt = "Hash Error")]
    HashError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": self.to_string()
                }))
            }
            AppError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(json!({
                    "error": message
                }))
            }
            AppError::Unauthorized(ref message) => {
                HttpResponse::Unauthorized().json(json!({
                    "error": message
                }))
            }
            AppError::NotFound(ref message) => {
                HttpResponse::NotFound().json(json!({
                    "error": message
                }))
            }
            AppError::DatabaseError(ref message) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": format!("Database error: {}", message)
                }))
            }
            AppError::ConfigError(ref message) => {
                HttpResponse::InternalServerError().json(json!({
                    "error": format!("Configuration error: {}", message)
                }))
            }
            AppError::TokenCreationError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Could not create authentication token"
                }))
            }
            AppError::InvalidToken => {
                HttpResponse::Unauthorized().json(json!({
                    "error": "Invalid or expired token"
                }))
            }
            AppError::PasswordHashingError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to hash password"
                }))
            }
            AppError::PasswordVerificationError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Failed to verify password"
                }))
            }
            AppError::Forbidden(ref message) => {
                HttpResponse::Forbidden().json(json!({
                    "error": message
                }))
            }
            AppError::HashError => {
                HttpResponse::InternalServerError().json(json!({
                    "error": "Hash error"
                }))
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TokenCreationError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::PasswordHashingError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordVerificationError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::HashError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> AppError {
        match error {
            SqlxError::RowNotFound => {
                AppError::NotFound("Requested resource not found".to_string())
            }
            SqlxError::Database(ref err) => {
                if err.code().as_deref() == Some("23505") {
                    AppError::BadRequest("Resource already exists".to_string())
                } else {
                    AppError::DatabaseError(err.to_string())
                }
            }
            _ => AppError::DatabaseError(error.to_string()),
        }
    }
}

impl From<JoinError> for AppError {
    fn from(_: JoinError) -> AppError {
        AppError::InternalServerError
    }
}

impl From<std::env::VarError> for AppError {
    fn from(error: std::env::VarError) -> AppError {
        AppError::ConfigError(error.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_: jsonwebtoken::errors::Error) -> AppError {
        AppError::InvalidToken
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(_: bcrypt::BcryptError) -> AppError {
        AppError::PasswordHashingError
    }
}
```

### quiz-app-backend/src/main.rs
```rs
use actix_web::{web, App, HttpServer, HttpResponse, middleware::Logger};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use log::{info, warn, error};

mod auth;
mod config;
mod db;
mod error;
mod handlers;
mod models;

pub mod handlers {
    pub mod auth;
    pub mod user;
    pub mod quiz;
    pub mod quiz_attempt;
}

use crate::{
    config::get_config,
    auth::JwtAuthService,
    handlers::quiz::{get_quizzes, get_quiz, create_quiz, update_quiz, delete_quiz},
    handlers::quiz_attempt::{start_quiz_attempt, get_quiz_attempts_by_quiz, get_user_quiz_attempts, get_quiz_attempt, submit_answer, finish_quiz_attempt},
};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let config = get_config().expect("Failed to load config");
    
    // Initialize logging with the configured level
    std::env::set_var("RUST_LOG", &config.log_level);
    env_logger::builder()
        .format_timestamp_millis()
        .init();
    
    info!("Starting Quiz App with configuration:");
    info!("Server: {}:{}", config.server_host, config.server_port);
    info!("Workers: {}", config.workers);
    info!("Log Level: {}", config.log_level);
    
    // Setup database connection pool
    info!("Initializing database connection pool...");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect(&config.database_url)
        .await {
            Ok(pool) => {
                info!("Database connection pool created successfully");
                pool
            },
            Err(e) => {
                error!("Failed to create database pool: {}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
            }
        };

    // Initialize database schema
    info!("Initializing database schema...");
    if let Err(e) = db::init_db(&pool).await {
        error!("Failed to initialize database: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, e));
    }
    info!("Database schema initialized successfully");

    info!("Configuring HTTP server...");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        let app = App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/quizzes")
                            .wrap(JwtAuthService)
                            .service(get_quizzes)
                            .service(get_quiz)
                            .service(create_quiz)
                            .service(update_quiz)
                            .service(delete_quiz)
                            .service(
                                web::scope("/{quiz_id}/attempts")
                                    .service(web::resource("")
                                        .route(web::post().to(start_quiz_attempt))
                                        .route(web::get().to(get_quiz_attempts_by_quiz))
                                    )
                            )
                    )
                    .service(
                        web::scope("/attempts")
                            .service(web::resource("")
                                .route(web::get().to(get_user_quiz_attempts))
                            )
                            .service(web::resource("/{attempt_id}")
                                .route(web::get().to(get_quiz_attempt))
                                .route(web::post().to(submit_answer))
                                .route(web::put().to(finish_quiz_attempt))
                            )
                    )
            )
            .service(web::resource("/health").to(health_check))
    })
    .bind((config.server_host.clone(), config.server_port))
    .map_err(|e| {
        error!("Failed to bind server to {}:{} - {}", config.server_host, config.server_port, e);
        e
    })?
    .workers(config.workers)
    .run()
    .await
}
```

### src/lib.rs
```rs
// ...existing code...
#[cfg(test)]
pub mod test_helpers;
// ...existing code...
```

### src/test_helpers.rs
```rs
use crate::{
    db::Db,
    models::{Quiz},
    schema::{quizzes, users},
};
```

### quiz-app-frontend/cypress/support/component.js
```js
// ***********************************************************
// This example support/component.js is processed and
// loaded automatically before your test files.
//
// This is a great place to put global configuration and
// behavior that modifies Cypress.
//
// You can change the location of this file or turn off
// automatically serving support files with the
// 'supportFile' configuration option.
//
// You can read more here:
// https://on.cypress.io/configuration
// ***********************************************************

// Import commands.js using ES2015 syntax:
import './commands'

// Alternatively you can use CommonJS syntax:
// require('./commands')

import { mount } from 'cypress/react18'

Cypress.Commands.add('mount', mount)

// Example use:
// cy.mount(<MyComponent />)```

### quiz-app-frontend/cypress/support/e2e.ts
```ts
import './commands'

// Alternatively you can use CommonJS syntax:
// require('./commands')
```

### quiz-app-frontend/cypress/support/commands.ts
```ts
import '@testing-library/cypress/add-commands'

declare global {
  namespace Cypress {
    interface Chainable {
      login(email: string, password: string): Chainable<void>
      setupTestEnv(): Chainable<void>
    }
  }
}

// You can add custom commands here if needed
// Login command
Cypress.Commands.add('login', (email: string, password: string) => {
  cy.visit('/login')
  cy.get('[data-testid="email-input"]').type(email)
  cy.get('[data-testid="password-input"]').type(password)
  cy.get('[data-testid="login-button"]').click()
})

// Setup test environment command
Cypress.Commands.add('setupTestEnv', () => {
  cy.intercept('GET', 'http://localhost:8080/api/quizzes', {
    statusCode: 200,
    body: [
      {
        id: 1,
        title: 'Test Quiz',
        description: 'A test quiz',
        questions: [
          {
            id: 1,
            text: 'Test Question',
            options: ['Option 1', 'Option 2', 'Option 3', 'Option 4'],
            correctAnswer: 0
          }
        ]
      }
    ]
  }).as('getQuizzes')

  cy.window().then((win) => {
    win.localStorage.setItem('token', 'test-token')
  })
})
```

### quiz-app-frontend/cypress/support/commands.js
```js
// ***********************************************
// This example commands.js shows you how to
// create various custom commands and overwrite
// existing commands.
//
// For more comprehensive examples of custom
// commands please read more here:
// https://on.cypress.io/custom-commands
// ***********************************************

// -- This is a parent command --
// Cypress.Commands.add('login', (email, password) => { ... })

// -- This is a child command --
// Cypress.Commands.add('drag', { prevSubject: 'element'}, (subject, options) => { ... })

// -- This is a dual command --
// Cypress.Commands.add('dismiss', { prevSubject: 'optional'}, (subject, options) => { ... })

// -- This will overwrite an existing command --
// Cypress.Commands.overwrite('visit', (originalFn, url, options) => { ... })
```

### quiz-app-frontend/cypress/support/e2e.js
```js
// Import commands.js using ES2015 syntax:
import './commands'

// Alternatively you can use CommonJS syntax:
// require('./commands')
```

### quiz-app-frontend/cypress/e2e/quiz.cy.ts
```ts
/// <reference types="cypress" />

describe('Quiz Application', () => {
  beforeEach(() => {
    // Intercept API calls
    cy.intercept('POST', 'http://localhost:8080/api/auth/login').as('login')
    cy.intercept('GET', 'http://localhost:8080/api/quizzes').as('getQuizzes')
    
    // Visit the home page and wait for it to load
    cy.visit('/')
    cy.window().its('document').should('exist')
    
    // Set a mock token
    cy.window().then((win) => {
      win.localStorage.setItem('token', 'mock-jwt-token')
    })
  })

  it('should load the home page', () => {
    cy.get('h1').should('exist')
  })

  it('should navigate to quiz page', () => {
    // Mock the quizzes response
    cy.intercept('GET', 'http://localhost:8080/api/quizzes', {
      statusCode: 200,
      body: [{
        id: 1,
        title: 'Test Quiz',
        description: 'A test quiz'
      }]
    }).as('getQuizzes')

    cy.get('[data-testid="start-quiz"]').should('be.visible').click()
    cy.url().should('include', '/quiz')
  })

  it('should be able to answer questions', () => {
    // Mock the quizzes and questions
    cy.intercept('GET', 'http://localhost:8080/api/quizzes', {
      statusCode: 200,
      body: [{
        id: 1,
        title: 'Test Quiz',
        description: 'A test quiz'
      }]
    }).as('getQuizzes')

    cy.intercept('GET', 'http://localhost:8080/api/quizzes/1', {
      statusCode: 200,
      body: {
        quiz: {
          id: 1,
          title: 'Test Quiz',
          description: 'A test quiz'
        },
        questions: [{
          id: 1,
          text: 'Test Question',
          options: ['Option 1', 'Option 2', 'Option 3', 'Option 4']
        }]
      }
    }).as('getQuizDetails')

    cy.get('[data-testid="start-quiz"]').should('be.visible').click()
    cy.get('[data-testid="quiz-question"]').should('be.visible')
    cy.get('[data-testid="answer-option"]').first().should('be.visible').click()
    cy.get('[data-testid="next-question"]').should('be.visible').click()
  })

  it('should show results at the end', () => {
    // Mock the necessary API responses
    cy.intercept('GET', 'http://localhost:8080/api/quizzes', {
      statusCode: 200,
      body: [{
        id: 1,
        title: 'Test Quiz',
        description: 'A test quiz'
      }]
    }).as('getQuizzes')

    cy.intercept('GET', 'http://localhost:8080/api/quizzes/1', {
      statusCode: 200,
      body: {
        quiz: {
          id: 1,
          title: 'Test Quiz',
          description: 'A test quiz'
        },
        questions: [{
          id: 1,
          text: 'Test Question',
          options: ['Option 1', 'Option 2', 'Option 3', 'Option 4']
        }]
      }
    }).as('getQuizDetails')

    cy.intercept('POST', 'http://localhost:8080/api/quizzes/1/submit', {
      statusCode: 200,
      body: {
        score: 80
      }
    }).as('submitQuiz')

    cy.get('[data-testid="start-quiz"]').should('be.visible').click()
    cy.get('[data-testid="answer-option"]').each(($option: JQuery<HTMLElement>) => {
      cy.wrap($option).should('be.visible').click()
      cy.get('[data-testid="next-question"]').should('be.visible').click()
    })
    cy.get('[data-testid="quiz-results"]').should('be.visible')
  })
})
```

### quiz-app-frontend/cypress/e2e/quiz.cy.js
```js
describe('Quiz App', () => {
  beforeEach(() => {
    cy.visit('/')
  })

  it('should display the home page', () => {
    cy.get('[data-testid="home"]').should('exist')
  })

  it('should start a quiz', () => {
    cy.get('[data-testid="start-quiz"]').click()
    cy.get('[data-testid="quiz-take"]').should('exist')
  })

  it('should complete a quiz and show results', () => {
    cy.get('[data-testid="start-quiz"]').click()
    cy.get('[data-testid="quiz-take"]').should('exist')
    
    // Answer all questions (assuming there are questions)
    cy.get('[data-testid="answer-option"]').first().click()
    cy.get('[data-testid="next-question"]').click()
    
    // Should show results at the end
    cy.get('[data-testid="quiz-results"]').should('exist')
  })
})
```

### quiz-app-frontend/babel.config.js
```js
module.exports = {
  presets: [
    '@babel/preset-env',
    '@babel/preset-typescript',
    '@babel/preset-react'
  ],
  plugins: [
    '@babel/plugin-transform-modules-commonjs'
  ]
};
```

### quiz-app-frontend/src/setupTests.ts
```ts
// jest-dom adds custom jest matchers for asserting on DOM nodes.
import '@testing-library/jest-dom';
import { configure } from '@testing-library/react';

// Configure testing library
configure({
  testIdAttribute: 'data-testid',
});
```

### quiz-app-frontend/src/hooks/useDataFetching.ts
```ts
import { useState, useEffect, useCallback } from 'react';

interface FetchState<T> {
  data: T | null;
  loading: boolean;
  error: string | null;
}

export function useDataFetching<T>(
  fetchFn: () => Promise<T>,
  dependencies: React.DependencyList = []
): FetchState<T> & { refetch: () => Promise<void> } {
  const [state, setState] = useState<FetchState<T>>({
    data: null,
    loading: true,
    error: null,
  });

  const fetchData = useCallback(async () => {
    setState(prev => ({ ...prev, loading: true, error: null }));
    try {
      const data = await fetchFn();
      setState({ data, loading: false, error: null });
    } catch (error) {
      setState({
        data: null,
        loading: false,
        error: error instanceof Error ? error.message : 'An error occurred',
      });
    }
  }, [fetchFn]);

  useEffect(() => {
    fetchData();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [fetchFn, ...dependencies]);

  return {
    ...state,
    refetch: fetchData,
  };
}

export default useDataFetching;
```

### quiz-app-frontend/src/hooks/useFormValidation.ts
```ts
import { useState, useCallback } from 'react';

interface ValidationRule {
  validate: (value: any) => boolean;
  message: string;
}

interface ValidationRules {
  [key: string]: ValidationRule[];
}

interface Errors {
  [key: string]: string[];
}

export function useFormValidation<T extends object>(
  initialValues: T,
  validationRules: ValidationRules
) {
  const [values, setValues] = useState<T>(initialValues);
  const [errors, setErrors] = useState<Errors>({});
  const [touched, setTouched] = useState<{ [key: string]: boolean }>({});

  const validateField = useCallback(
    (name: string, value: any) => {
      const fieldRules = validationRules[name] || [];
      const fieldErrors = fieldRules
        .filter(rule => !rule.validate(value))
        .map(rule => rule.message);
      
      return fieldErrors;
    },
    [validationRules]
  );

  const handleChange = useCallback(
    (name: keyof T, value: any) => {
      setValues(prev => ({ ...prev, [name]: value }));
      if (touched[name as string]) {
        const fieldErrors = validateField(name as string, value);
        setErrors(prev => ({ ...prev, [name]: fieldErrors }));
      }
    },
    [touched, validateField]
  );

  const handleBlur = useCallback(
    (name: string) => {
      setTouched(prev => ({ ...prev, [name]: true }));
      const fieldErrors = validateField(name, values[name as keyof T]);
      setErrors(prev => ({ ...prev, [name]: fieldErrors }));
    },
    [validateField, values]
  );

  const validateForm = useCallback(() => {
    const newErrors: Errors = {};
    let isValid = true;

    Object.keys(validationRules).forEach(fieldName => {
      const fieldErrors = validateField(fieldName, values[fieldName as keyof T]);
      if (fieldErrors.length > 0) {
        newErrors[fieldName] = fieldErrors;
        isValid = false;
      }
    });

    setErrors(newErrors);
    return isValid;
  }, [validateField, values, validationRules]);

  return {
    values,
    errors,
    touched,
    handleChange,
    handleBlur,
    validateForm,
    setValues,
  };
}

export default useFormValidation;
```

### quiz-app-frontend/src/types/quiz.ts
```ts
export interface Answer {
    id?: number;
    question_id?: number;
    answer_text: string;
    is_correct: boolean;
    created_at?: string;
    updated_at?: string;
}

export interface Question {
    id?: number;
    quiz_id?: number;
    text?: string;
    question_text: string;
    question_type: 'multiple_choice' | 'true_false';
    is_multiple_choice?: boolean;
    answers: Answer[];
    options?: string[];
    created_at?: string;
    updated_at?: string;
}

export interface Quiz {
    id?: number;
    title: string;
    description: string | null;
    category?: string;
    creator_id?: number;
    created_by?: number;
    created_at?: string;
    updated_at?: string;
    questions: Question[];
}

export interface CreateQuiz {
    title: string;
    description: string | null;
    category?: string;
    questions: Question[];
}

export interface QuizFormProps {
    initialData?: CreateQuiz;
    onSubmit: (quiz: CreateQuiz) => void;
    isLoading?: boolean;
}

export interface SubmittedAnswer {
    question_id: number;
    answer_id: number;
    is_correct: boolean;
}
```

### quiz-app-frontend/src/types/index.ts
```ts
export * from './quiz';

export interface Question {
  id: number;
  quiz_id: number;
  question_text: string;
  question_type: 'multiple_choice' | 'true_false';
  created_at: string;
}

export interface Answer {
  id: number;
  answer_text: string;
  is_correct: boolean;
}

export interface SubmittedAnswer {
  question_id: number;
  answer_id: number;
}

export interface QuizSubmission {
  quiz_id: number;
  answers: SubmittedAnswer[];
}
```

### quiz-app-frontend/src/services/api.ts
```ts
import axios from 'axios';
import { Quiz, Question, QuizSubmission } from '../types';
import { v4 as uuidv4 } from 'uuid';

const API_BASE_URL = process.env.REACT_APP_API_BASE_URL || 'http://localhost:8080/api';

// Add request interceptor for token
axios.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export const quizService = {
  getAllQuizzes: async (): Promise<Quiz[]> => {
    const response = await axios.get(`${API_BASE_URL}/quizzes`);
    return response.data;
  },

  getQuizDetails: async (quizId: string): Promise<Quiz> => { // Changed `quizId` type to `string` for UUID
    const response = await axios.get(`${API_BASE_URL}/quizzes/${quizId}`);
    // Transform the response to match Quiz type
    const { quiz, questions, answers } = response.data;
    return {
      ...quiz,
      questions: questions.map((q: Question) => ({
        ...q,
        answers: answers.filter((a: any) => a.question_id === q.id).map((a: any) => ({
          id: a.id,
          answer_text: a.answer_text,
          is_correct: a.is_correct,
          created_at: a.created_at,
          updated_at: a.updated_at
        }))
      }))
    };
  },

  submitQuiz: async (submission: QuizSubmission): Promise<{score: number}> => {
    const response = await axios.post(`${API_BASE_URL}/quizzes/submit`, submission);
    return response.data;
  }
};

export const authService = {
  login: async (username: string, password: string): Promise<{token: string}> => {
    try {
      const response = await axios.post(`${API_BASE_URL}/login`, { username, password });
      localStorage.setItem('token', response.data.token); // Store token
      return response.data;
    } catch (error) {
      console.error('Login failed', error);
      throw error;
    }
  },

  register: async (username: string, email: string, password: string): Promise<{token: string}> => {
    try {
      const response = await axios.post(`${API_BASE_URL}/register`, { username, email, password });
      localStorage.setItem('token', response.data.token); // Store token
      return response.data;
    } catch (error) {
      console.error('Registration failed', error);
      throw error;
    }
  }
};
```

### quiz-app-frontend/cypress.config.ts
```ts
const { defineConfig } = require('cypress')

module.exports = defineConfig({
  projectId: '3m8mi8',
  e2e: {
    baseUrl: 'http://localhost:3000',
    supportFile: 'cypress/support/e2e.ts',
    specPattern: 'cypress/e2e/**/*.cy.ts',
    video: false,
    screenshotOnRunFailure: true,
    setupNodeEvents(on, config) {
      return config
    }
  }
})
```

## Documentation

### quiz-app-backend/README.md

# Quiz App Backend

A Rust-based backend for a quiz application using Actix-web and PostgreSQL.

## Features

- User authentication and authorization
- CRUD operations for quizzes
- Question and answer management
- Quiz attempt tracking and scoring
- Error handling with custom error types
- Database migrations with SQLx

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- SQLx CLI

## Setup

1. Install dependencies:
   ```bash
   cargo install sqlx-cli
   ```

2. Set up the database:
   ```bash
   # Create the database
   sqlx database create
   
   # Run migrations
   sqlx migrate run
   ```

3. Run the server:
   ```bash
   cargo run
   ```

## API Endpoints

### Quizzes
- `GET /quizzes` - List all quizzes
- `POST /quizzes` - Create a new quiz
- `GET /quizzes/{id}` - Get a specific quiz
- `PUT /quizzes/{id}` - Update a quiz
- `DELETE /quizzes/{id}` - Delete a quiz

### Quiz Attempts
- `POST /quizzes/{id}/submit` - Submit a quiz attempt

## Error Handling

The application uses a custom `AppError` type that handles:
- Database errors
- Authentication errors
- Not found errors
- Bad request errors

Errors are returned as JSON with appropriate HTTP status codes.

## Running Endpoint Tests

To execute endpoint tests within a Docker container, follow these steps:

1. **Build the test image:**
   ```bash
   docker build --target endpoint-tester -t quiz-app-endpoint-tests .
   ```

2. **Run the tests:**
   ```bash
   docker run --rm --network host quiz-app-endpoint-tests
   ```

   *Ensure that the PostgreSQL database is running and accessible at the `DATABASE_URL` specified in the Dockerfile.*

### README.md

# Quiz App

A full-stack quiz application built with React and Rust.

## Quick Start

To use the Quiz App commands, first activate the development environment:

```bash
source scripts/activate
```

Once activated, you can use the following commands directly (without ./):

```bash
# Comprehensive check and start (recommended)
serve           # Checks everything, sets up if needed, and starts the app

# Individual commands
setup           # Set up the environment
start           # Start the application
test            # Run all tests
test backend    # Run backend tests only
test frontend   # Run frontend tests only
clean           # Clean up processes
```

The `serve` command performs the following checks before starting the application:
1. ✓ System requirements (node, npm, cargo, postgresql)
2. ✓ Network ports availability
3. ✓ Database connection
4. ✓ Project dependencies
5. ✓ Environment files
6. ✓ Quick system test

If any components are missing, it will automatically run setup before starting.

## Project Structure

```
quiz-app/
├── quiz-app-frontend/     # React frontend
├── quiz-app-backend/      # Rust/Actix-web backend
└── quiz-app-database/     # PostgreSQL database schemas
```

## Prerequisites

- Node.js (v14 or later)
- Rust (latest stable)
- PostgreSQL (v13 or later)

## Setup Instructions

### Frontend Setup

1. Navigate to the frontend directory:
   ```bash
   cd quiz-app-frontend
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Start the development server:
   ```bash
   npm start
   ```

### Backend Setup

1. Navigate to the backend directory:
   ```bash
   cd quiz-app-backend
   ```

2. Create a `.env` file with the following content:
   ```
   DATABASE_URL=postgres://username:password@localhost/quiz_app
   RUST_LOG=debug
   ```

3. Build and run the server:
   ```bash
   cargo run
   ```

### Database Setup

1. Create a PostgreSQL database:
   ```bash
   createdb quiz_app
   ```

2. Apply the schema:
   ```bash
   psql quiz_app < quiz-app-database/schema.sql
   ```

## Development Phases

1. Frontend Development (2 weeks)
2. Backend Development (4 weeks)
3. API Integration (1 week)
4. Database Integration (1 week)
5. Testing and Debugging (2 weeks)
6. Deployment (1 week)

## Security Measures

- HTTPS encryption
- Input validation
- Secure password practices
- Rate limiting

## Scalability Strategy

- Load balancing
- Caching
- CDN utilization

## API Documentation

API documentation will be available at `http://localhost:8080/docs` when running the backend server.

### codebase.md


