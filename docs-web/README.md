# OpenTDF SDK documentation

An Astro Starlight site generated from the SDK's Go comments and canonical
Markdown guides. Production URL: https://eugenioenko.github.io/opentdf-sdk/.

## Local development

Use Node 24, Python 3, Git, and Go. The package manager is pinned in package.json.

```sh
cd docs-web
pnpm install --frozen-lockfile
pnpm dev
pnpm check
pnpm build
pnpm preview
```

Preparation checks API comment coverage, generates the API reference with
Goalchemy, and imports the native language guides. It installs no native target
toolchains and does not contact an OpenTDF platform. The first run checks out
the documentation compiler under `.local/docs/compiler` at the revision in
`goalchemy.lock.json`. This pin is separate from the SDK build compiler because
the documentation command is not released yet.

For a local compiler checkout, set `GOALCHEMY_DOCS_ROOT=/path/to/goalchemy`.
Generation uses an isolated source module so it does not change the SDK's
go.mod, production compiler pin, or adjacent checkouts. Unchanged source reuses
the generated reference. Delete `.local/docs/source.sha256` to force generation.

## Content ownership

- Edit API descriptions in the shared Go source. `scripts/check-api-docs.go`
  checks handwritten exports, interface methods, and fields. Generated protocol
  code and handwritten native adapters have separate documentation.
- Edit native package guides in `docs/generated-<language>-library.md` and other
  canonical references in `docs/`. `scripts/prepare-docs.mjs` publishes them.
  Contributor-only CLI and validation sections are omitted from language pages;
  local test infrastructure and evidence have their own Contributing section.
- Edit the homepage and task guides under `src/content/docs/`.
- Keep generated API pages, imported guides, JSON, Markdown exports, build output,
  and dependency installations ignored. Regenerate them instead of editing them.

The generator's Starlight template supplies the initial scaffold and preserves
customized site files. The site separates the shared source API from native host
adapters. Type and method pages link to their related declarations. The sidebar
uses section headings, and individual symbols remain accessible through package
indexes and search.

The published JSON catalog is `api.json`; portable Markdown is under
`reference/`. `llms.txt` links agents to both. The build checks local URLs,
fragments, assets, all eight target references, and the repository base path.

## Publishing

`.github/workflows/docs.yml` generates and checks the site on relevant pull
requests. After merge, relevant pushes to main upload the static artifact and
deploy it with GitHub Pages. Manual runs on main can also publish. Pull requests
never deploy. The repository's Pages source is configured for GitHub Actions.
No custom domain is required. The Astro base is `/opentdf-sdk`; keep it aligned
with the repository name when changing hosting.
