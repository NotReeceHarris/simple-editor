const { addIconSelectors } = require('@iconify/tailwind');

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        window: '#353638',
        editor: '#27282a'
      },
    },
  },
  plugins: [
    addIconSelectors(['mdi', 'mdi-light']),
  ],
};