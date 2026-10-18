import { readFile, readdir, stat } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { parse } from 'parse5';

const dist = fileURLToPath(new URL('../dist/', import.meta.url));
const origin = 'https://eugenioenko.github.io';
const base = '/opentdf-sdk/';
const documents = new Map();
const failures = [];

async function collect(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) { await collect(file); continue; }
    if (!entry.name.endsWith('.html')) continue;
    const ids = new Set();
    const links = [];
    function visit(node) {
      const attrs = Object.fromEntries((node.attrs ?? []).map(({ name, value }) => [name, value]));
      if (attrs.id) ids.add(attrs.id);
      if (node.tagName === 'a' && attrs.href) links.push(attrs.href);
      if (['img', 'script', 'source'].includes(node.tagName) && attrs.src) links.push(attrs.src);
      if (node.tagName === 'link' && attrs.href && attrs.rel !== 'canonical') links.push(attrs.href);
      for (const child of node.childNodes ?? []) visit(child);
    }
    visit(parse(await readFile(file, 'utf8')));
    documents.set(file, { ids, links });
  }
}

await collect(dist);
const pageCount = documents.size;
for (const [file, { links }] of documents) {
  const pathname = path.relative(dist, file).split(path.sep).join('/').replace(/index\.html$/, '');
  for (const link of links) {
    const url = new URL(link, `${origin}${base}${pathname}`);
    if (url.origin !== origin) continue;
    if (!url.pathname.startsWith(base)) {
      failures.push(`${pathname}: link escapes the repository base ${link}`);
      continue;
    }
    let target = path.join(dist, decodeURIComponent(url.pathname.slice(base.length)));
    try {
      if ((await stat(target)).isDirectory()) target = path.join(target, 'index.html');
      await stat(target);
      if (url.hash && documents.has(target) && !documents.get(target).ids.has(decodeURIComponent(url.hash.slice(1)))) {
        failures.push(`${pathname}: missing anchor ${link}`);
      }
    } catch {
      failures.push(`${pathname}: missing file ${link}`);
    }
  }
}
const catalog = JSON.parse(await readFile(path.join(dist, 'api.json'), 'utf8'));
if (catalog.targets.length !== 8 || catalog.targets.some((target) => target.status !== 'available')) failures.push('API reference must include all eight native targets.');
const targets = ['go', 'typescript', 'python', 'java', 'csharp', 'rust', 'c', 'swift'];
const methods = catalog.rpc?.services.flatMap((service) => service.methods) ?? [];
const messages = new Set(catalog.rpc?.messages.map((message) => message.id) ?? []);
if (!methods.length || methods.some((method) =>
  !messages.has(method.request) || !messages.has(method.response) ||
  targets.some((target) => !method.bindings?.some((binding) => binding.target === target))
)) failures.push('Every RPC method needs linked request/response messages and all eight native declarations.');
const examples = JSON.parse(await readFile(new URL('../examples.json', import.meta.url), 'utf8')).examples;
if (!examples.length || examples.some((example) =>
  !catalog.examples?.some((generated) => generated.id === example.id) ||
  targets.some((target) => !example.snippets.some((snippet) => snippet.target === target && snippet.code.trim()))
)) failures.push('Every reviewed example must be published and include all eight native languages.');
if (!(await readFile(path.join(dist, 'llms.txt'), 'utf8')).includes(`${origin}${base}`)) failures.push('llms.txt must use the published repository URL.');
if (!(await readFile(path.join(dist, 'reference/index.md'), 'utf8')).includes('## Packages')) failures.push('Portable Markdown reference is missing.');
if (failures.length) {
  console.error([...new Set(failures)].join('\n'));
  process.exitCode = 1;
} else console.log(`Checked links, fragments, assets, and repository-base paths across ${pageCount} pages; ${methods.length} RPC methods and ${examples.length} examples cover all eight targets.`);
