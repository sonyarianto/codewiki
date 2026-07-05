import { defineConfig } from 'vitepress'

export default defineConfig({
  lang: 'en-US',
  title: 'WakaWiki',
  description: 'CLI that writes and maintains agent documentation for your codebase',

  themeConfig: {
    nav: [
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'GitHub', link: 'https://github.com/sonyarianto/wakawiki' },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Introduction',
          items: [
            { text: 'What is WakaWiki?', link: '/guide/getting-started' },
            { text: 'Installation', link: '/guide/installation' },
          ],
        },
        {
          text: 'Usage',
          items: [
            { text: 'Commands', link: '/guide/usage' },
            { text: 'Providers', link: '/guide/providers' },
            { text: 'Configuration', link: '/guide/configuration' },
          ],
        },
        {
          text: 'Integrations',
          items: [
            { text: 'GitHub Actions', link: '/guide/ci' },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/sonyarianto/wakawiki' },
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present',
    },
  },
})
