import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { unified } from 'unified';
import remarkParse from 'remark-parse';

const site = fileURLToPath(new URL('../', import.meta.url));
const docs = path.join(site, 'src/content/docs');
const markdown = unified().use(remarkParse);
const origin = 'https://docs.invalid';
const labels = {
  authorization: 'Authorization', entityresolution: 'Entity resolution',
  policy: 'Policy', wellknownconfiguration: 'Well-known configuration',
  actions: 'Actions', attributes: 'Attributes', namespaces: 'Namespaces',
  obligations: 'Obligations', registeredresources: 'Registered resources',
  resourcemapping: 'Resource mapping', subjectmapping: 'Subject mapping',
  dynamicvaluemapping: 'Dynamic value mapping', kasregistry: 'KAS registry',
  keymanagement: 'Key management', unsafe: 'Unsafe operations',
};

async function links(file, route) {
  const result = new Map();
  function visit(node) {
    if (node.type === 'link') {
      const label = node.children.map((child) => child.value ?? '').join('');
      const url = new URL(node.url, origin + route);
      if (url.origin === origin) result.set(label, url.pathname);
    }
    for (const child of node.children ?? []) visit(child);
  }
  visit(markdown.parse(await readFile(file, 'utf8')));
  return result;
}

function requiredLink(index, label) {
  const route = index.get(label);
  if (!route) throw new Error(`Generated RPC reference has no link for ${label}`);
  return route;
}

export async function rpcSidebar() {
  const catalog = JSON.parse(await readFile(path.join(site, 'public/api.json'), 'utf8'));
  const index = await links(path.join(docs, 'api/rpc/index.md'), '/api/rpc/');
  const groups = new Map();
  for (const service of catalog.rpc.services) {
    const overview = requiredLink(index, service.id);
    const methods = await links(path.join(docs, overview.slice(1).replace(/\/$/, '.md')), overview);
    const parts = service.id.split('.').slice(0, -1);
    const category = parts.shift();
    const group = groups.get(category) ?? { label: labels[category] ?? category, collapsed: true, items: [] };
    groups.set(category, group);
    const items = [
      { label: 'Overview', link: overview },
      ...service.methods.map((method) => ({ label: method.name, link: requiredLink(methods, method.name) })),
    ];
    const siblings = catalog.rpc.services.filter((candidate) => candidate.id.startsWith(category + '.'));
    if (parts.length || siblings.length > 1) {
      const name = parts.join('.') || 'v1';
      group.items.push({ label: labels[name] ?? name, collapsed: true, items });
    } else group.items.push(...items);
  }
  for (const group of groups.values()) {
    if (group.label === 'Policy') group.items.sort((a, b) => a.label.localeCompare(b.label));
  }
  return [{ label: 'Overview', link: '/api/rpc/' }, ...groups.values()];
}
