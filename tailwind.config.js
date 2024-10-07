const { addIconSelectors } = require('@iconify/tailwind');

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        window: '#353638',
        editor: '#27282a',
        primary: '#5f9c5d',
      },
    },
  },
  plugins: [
    addIconSelectors(['mdi', 'mdi-light']),
  ],
};