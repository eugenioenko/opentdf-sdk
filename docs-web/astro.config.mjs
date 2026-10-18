import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://eugenioenko.github.io',
  base: '/opentdf-sdk',
  trailingSlash: 'always',
  integrations: [starlight({
    title: 'OpenTDF SDK',
    description: 'TDF3 encryption and decryption from one shared implementation, with native APIs for eight languages.',
    favicon: '/favicon.svg',
    social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/eugenioenko/opentdf-sdk' }],
    editLink: { baseUrl: 'https://github.com/eugenioenko/opentdf-sdk/edit/main/docs-web/' },
    customCss: ['./src/styles/custom.css'],
    components: { Sidebar: './src/components/Sidebar.astro' },
    head: [{ tag: 'link', attrs: { rel: 'describedby', href: '/opentdf-sdk/llms.txt', type: 'text/plain' } }],
    sidebar: [
      { label: 'Getting started', items: [
        { label: 'Introduction', link: '/' },
        { label: 'Build and install', link: '/getting-started/' },
      ] },
      { label: 'Guides', items: [{ autogenerate: { directory: 'guides' } }] },
      { label: 'Languages', items: [{ autogenerate: { directory: 'languages' } }] },
      { label: 'API reference', items: [{ autogenerate: { directory: 'api' } }] },
      { label: 'Reference', items: [{ autogenerate: { directory: 'reference' } },
        { label: 'llms.txt', link: 'https://eugenioenko.github.io/opentdf-sdk/llms.txt' },
      ] },
      { label: 'Contributing', items: [{ autogenerate: { directory: 'contributing' } }] },
    ],
  })],
});
