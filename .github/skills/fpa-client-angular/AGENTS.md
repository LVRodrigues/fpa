# Repository Guidelines

## Project Structure & Module Organization

- `fpa-client/`: Angular/TypeScript UI. Components, services, and guards live in `src/app/`; shared SCSS lives in `src/styles/`, translations in `src/locale/`, and static assets in `public/`.
- `fpa-server/`: Rust API using Axum and SeaORM. Request handlers live in `src/handlers/`, database models in `src/model/`, and integration tests in `tests/`, with helpers in `tests/shared/`.
- `database/setup/`: numbered SQL initialization scripts; preserve their execution order.
- `oauth-2/`: Keycloak image and realm configuration. Root `docker-compose.yaml` defines the four application services.

## Build, Test, and Development Commands

Run frontend commands from `fpa-client/`:

- `npm ci`: install dependencies from the committed lockfile. Use a Node.js version allowed by `package.json`.
- `npm start`: serve the development UI; `npm run pt` serves Portuguese.
- `npm run build`: build production English and Portuguese bundles.
- `npm test -- --watch=false --browsers=ChromeHeadless`: run Jasmine/Karma tests with Chrome installed.
- `npm run i18n`: extract translation messages after changing localized text.

## Coding Style & Naming Conventions

Follow nearby formatting: Angular TypeScript commonly uses tabs and single quotes; Rust uses four spaces and standard rustfmt formatting. Keep TypeScript strict checks enabled. Use PascalCase types, camelCase TypeScript members, and snake_case Rust functions/modules. Name Angular files by responsibility, such as `projects.component.ts` or `auth.service.ts`, with `app-` component selectors. No frontend lint or formatter script is configured.

## Testing Guidelines

Place Angular tests beside source files as `*.spec.ts`; generators currently skip tests. Rust integration tests use Tokio and reqwest and require the API at `fpa-server:5000`, initialized PostgreSQL, and Keycloak reachable as `oauth-2:8080`. Ensure hostname resolution before running them. Add regression coverage for changed behavior, especially authentication and tenant isolation. No numeric coverage threshold is configured.

## Commit & Pull Request Guidelines

History uses descriptive English and Portuguese subjects without a mandatory prefix scheme. Write focused commits describing the change. The client README documents GitFlow releases. For PRs, describe behavior changes, link relevant issues, report validation and prerequisites, and include screenshots for UI changes. Commit updated lockfiles with dependency changes.
