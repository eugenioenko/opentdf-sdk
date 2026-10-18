# OpenTDF SDK documentation

An Astro Starlight site generated from the SDK's Go comments, platform protobuf
schemas, native service declarations, and canonical Markdown guides.
Production URL: https://eugenioenko.github.io/opentdf-sdk/.

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

Preparation checks API comment coverage, generates the SDK and RPC references
with Goalchemy, and imports the native language guides. It installs no native
target toolchains and makes no live platform calls. The first run downloads
the documentation compiler, the pinned platform source, and Buf under
`.local/docs/`. Their pins come from `goalchemy.lock.json` and
`references.lock.json`. Buf builds descriptors with imports and source comments.
The documentation compiler pin is separate from the SDK build compiler.

For a local compiler checkout, set `GOALCHEMY_DOCS_ROOT=/path/to/goalchemy`.
For a clean platform checkout at the locked revision, set
`PLATFORM_DOCS_ROOT=/path/to/platform`.
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
- Edit reviewed examples in `examples.json`. Each includes eight native snippets
  and can attach to an SDK symbol or RPC method. Request and response JSON is
  checked against the protobuf schema; application validation is the server's
  responsibility. Snippets use the public host adapters and are not compiled or
  executed by documentation generation.
- RPC descriptions, fields, enums, presence, and validation options come from
  the pinned platform protobuf files. Native method declarations are read from
  `src/hosts/` using the service inventory from `generate-service-clients.py`.
  The docs generator does not regenerate the checked-in host adapters.
- Keep generated API pages, imported guides, JSON, Markdown exports, build output,
  and dependency installations ignored. Regenerate them instead of editing them.

The generator's Starlight template supplies the scaffold, language tabs, and
expandable RPC field viewer, preserving customized site files. The site explains
the shared source API and native host adapters. Type and method pages link to
their related declarations. The sidebar
uses section headings, and individual symbols remain accessible through package
indexes and search.

The published JSON catalog is `api.json`; portable Markdown is under
`reference/`. `llms.txt` links agents to both. The build checks local URLs,
fragments, assets, all eight target references, RPC coverage, reviewed example
coverage, and the repository base path.

## Publishing

`.github/workflows/docs.yml` generates and checks the site on relevant pull
requests. After merge, relevant pushes to main upload the static artifact and
deploy it with GitHub Pages. Manual runs on main can also publish. Pull requests
never deploy. The repository's Pages source is configured for GitHub Actions.
No custom domain is required. The Astro base is `/opentdf-sdk`; keep it aligned
with the repository name when changing hosting.
