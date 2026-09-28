# Repository Guidelines

FPA manages projects using Function Point Analysis.

## Agent Scope & Runtime Assumptions

Act as a frontend development agent focused exclusively on `fpa-client/`. Treat `database`, `oauth-2`, and `fpa-server` as already running through Docker Compose. Implement and validate client changes against these existing services.

Backend code and configuration may be inspected to understand API contracts and authentication. Do not modify those modules or shared infrastructure unless explicitly requested. Do not start, rebuild, restart, or stop supporting services, reset databases, or run backend test suites as part of frontend work. If a dependency is unavailable, report the blocker and continue independent client work.

## Project Structure & Module Organization

Paths below are relative to this directory:

- `src/app/`: Angular components, services, and guards.
- `src/styles/`, `src/locale/`, and `public/`: shared SCSS, translations, and static assets.
- `../fpa-server/`, `../database/`, and `../oauth-2/`: supporting API, PostgreSQL, and Keycloak services configured by `../docker-compose.yaml`.

## Build, Test, and Development Commands

Run frontend commands from `fpa-client/` with a Node.js version supported by `package.json`:

- `npm ci`: install locked dependencies.
- `npm start`: serve the development client; `npm run pt` serves Portuguese.
- `npm run build`: build English and Portuguese production bundles.
- `npm test -- --watch=false --browsers=ChromeHeadless`: run Jasmine/Karma tests with Chrome installed.
- `npm run i18n`: extract translation messages into `src/locale/`.

Use `docker compose ps` from the repository root to check service status. Preserve existing client environment and authentication configuration.

## Coding Style & Naming Conventions

Match surrounding formatting: Angular files commonly use tabs, single-quoted TypeScript strings, and semicolons. Keep TypeScript strict checks enabled. Use PascalCase types, camelCase members, and filenames such as `projects.component.ts`. Keep templates and SCSS alongside their components. No frontend lint or formatter script is configured.

## Internationalization

English (`en`) is the default and source language. Provide Brazilian Portuguese (`pt-BR`) translations for every new or changed user-facing message, including labels, validation errors, tooltips, and accessibility text. Use Angular `i18n` and `i18n-*` attributes in templates and `$localize` for TypeScript messages; follow existing message ID conventions.

Run `npm run i18n` to update `src/locale/messages.xlf`, then update the corresponding translations in `src/locale/messages.pt.xlf`, preserving placeholders and interpolation. The current build uses the locale key `pt`; its translations must use Brazilian Portuguese. Validate both languages with `npm run build` and review the Portuguese UI with `npm run pt`.

## Testing Guidelines

Validate client changes with the production build and relevant Jasmine/Karma tests. Add tests as adjacent `*.spec.ts` files; generators currently skip tests. No coverage threshold is configured. For affected UI flows, check English and Portuguese rendering, routing, and authentication against the active services. Use disposable development data for checks that write records.

## Commit & Pull Request Guidelines

Use descriptive commit subjects in English or Portuguese; history has no fixed prefix convention. PRs should explain changes, link relevant issues, report validation, and include UI screenshots. Include lockfile updates with dependency changes.

## Security & Configuration

Keep credentials and tokens out of commits. Preserve existing authentication and tenant handling. Treat bundled development credentials as local-only.
