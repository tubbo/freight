// @ts-check

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Freight',
  tagline: 'The package-oriented programming language.',
  favicon: 'img/favicon.ico',
  url: 'https://tubbo.github.io',
  baseUrl: '/freight',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },
  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      {
        docs: {
          sidebarPath: require.resolve('./docs/sidebars.js'),
          editUrl: 'https://github.com/tubbo/freight/tree/main/docs/',
        },
        blog: {
          showReadingTime: true,
          editUrl: 'https://github.com/tubbo/freight/tree/main/blog/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    {
      // Replace with your project's social card
      image: 'img/docusaurus-social-card.jpg',
      navbar: {
        title: 'Freight',
        logo: {
          alt: 'Freight Logo',
          src: 'img/logo.svg',
        },
        items: [
          { to: '/about', label: 'About', position: 'left' },
          { to: '/docs', label: 'Docs', position: 'left' },
          { to: '/blog', label: 'Blog', position: 'left' },
          {
            href: 'https://github.com/tubbo/freight',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              // {
              //   label: 'Getting Started',
              //   to: '/docs/getting-started',
              // },
              // {
              //   label: 'Language Reference',
              //   to: '/docs/reference',
              // },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'Discussions',
                href: 'https://github.com/tubbo/freight/discussions',
              },
              {
                label: 'Issue Tracker',
                href: 'https://github.com/tubbo/freight/issues',
              },
              {
                label: 'Source Code',
                href: 'https://github.com/tubbo/freight',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'About',
                to: '/about',
              },
              {
                label: 'Blog',
                to: '/blog',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Team Freight.`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
      colorMode: {
        respectPrefersColorScheme: true,
      },
    },
};

module.exports = config;
