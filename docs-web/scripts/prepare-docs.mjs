import { mkdir, readFile, rm, stat, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { unified } from 'unified';
import remarkParse from 'remark-parse';
import remarkGfm from 'remark-gfm';
import remarkStringify from 'remark-stringify';
import { stringify } from 'yaml';

const site = fileURLToPath(new URL('../', import.meta.url));
const root = path.resolve(site, '..');
const repository = 'https://github.com/eugenioenko/opentdf-sdk';
const base = '/opentdf-sdk/';
const markdown = unified().use(remarkParse).use(remarkGfm).use(remarkStringify, { fences: true, bullet: '-' });
const sources = [
  ...['go', 'typescript', 'python', 'java', 'csharp', 'rust', 'c', 'swift'].map((target, order) => [
    `docs/generated-${target}-library.md`, `languages/${target}`,
    { go: 'Go', typescript: 'TypeScript', python: 'Python', java: 'Java', csharp: 'C#', rust: 'Rust', c: 'C', swift: 'Swift (experimental)' }[target], order + 1,
  ]),
  ['docs/compatibility.md', 'reference/compatibility', 'Compatibility and limits', 1],
  ['docs/format.md', 'reference/format', 'TDF format', 2],
  ['docs/secure-profiles.md', 'contributing/secure-profiles', 'Local EC and DPoP profiles', 1],
  ['docs/platform.md', 'contributing/platform', 'Local test platform', 2],
  ['docs/final-delivery.md', 'contributing/builds', 'Build verification and evidence', 3],
];
const routes = new Map(sources.map(([source, route]) => [source, `${base}${route}/`]));

async function rewrite(node, source, parent) {
  if (node.type === 'html' && !node.value.startsWith('<!--')) {
    if (['root', 'blockquote', 'listItem'].includes(parent?.type)) {
      node.type = 'paragraph';
      node.children = [{ type: 'text', value: node.value }];
      delete node.value;
    } else node.type = 'text';
  }
  if (node.type === 'code' && node.lang?.includes(',')) node.lang = node.lang.split(',')[0];
  if (node.type === 'text') node.value = node.value.replace(/\s*—\s*/g, '; ');
  if (['link', 'definition', 'image'].includes(node.type) && !/^(?:[a-z][a-z\d+.-]*:|\/\/)/i.test(node.url)) {
    const [pathname, fragment = ''] = node.url.split('#');
    const target = path.posix.normalize(path.posix.join(path.posix.dirname(source), decodeURIComponent(pathname || path.posix.basename(source))));
    if (target.startsWith('../')) {
      if (node.type !== 'link') throw new Error(`Unsupported sibling reference: ${node.url}`);
      node.type = 'emphasis';
      delete node.url;
    } else if (routes.has(target)) {
      node.url = `${routes.get(target)}${fragment ? `#${fragment}` : ''}`;
    } else {
      let info;
      try { info = await stat(path.join(root, target)); }
      catch { throw new Error(`Missing reference in ${source}: ${node.url}`); }
      node.url = `${repository}/${info.isDirectory() ? 'tree' : 'blob'}/main/${target}${fragment ? `#${fragment}` : ''}`;
    }
  }
  for (const child of node.children ?? []) await rewrite(child, source, node);
}

for (const directory of ['reference', 'contributing']) {
  await rm(path.join(site, 'src/content/docs', directory), { recursive: true, force: true });
}
for (const [source, route, title, order] of sources) {
  const tree = markdown.parse(await readFile(path.join(root, source), 'utf8'));
  tree.children = tree.children.filter((node, index) => !(index === 0 && node.type === 'heading' && node.depth === 1));
  if (route.startsWith('languages/')) {
    let include = true;
    tree.children = tree.children.filter((node) => {
      if (node.type === 'heading' && node.depth === 2) {
        const title = node.children.map((child) => child.value ?? '').join('');
        include = !['CLI', 'Verification', 'Verification and interoperability', 'Validation evidence', 'Bounded acceptance repairs'].includes(title);
      }
      return include;
    });
  }
  await rewrite(tree, source);
  if (route.startsWith('languages/')) {
    const position = tree.children.findIndex((node) => node.type === 'heading' && node.depth === 2);
    const introduction = markdown.parse(`Choose [authentication examples](${base}languages/authentication/) for client credentials, application token providers, and DPoP in ${title}.`);
    tree.children.splice(position < 0 ? 0 : position, 0, ...introduction.children);
  }
  const file = path.join(site, 'src/content/docs', `${route}.md`);
  await mkdir(path.dirname(file), { recursive: true });
  const metadata = stringify({ title, sidebar: { order }, editUrl: `${repository}/edit/main/${source}` });
  await writeFile(file, `---\n${metadata}---\n\n${markdown.stringify(tree)}`);
}
console.log(`Prepared ${sources.length} language and reference guides from canonical Markdown.`);
