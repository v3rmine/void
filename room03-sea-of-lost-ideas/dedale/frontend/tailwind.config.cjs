/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/routes/**/*.{svelte,ts,js}"
  ],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
}

