/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,rs}",
    "./index.html",
  ],
  theme: {
    extend: {
        fontFamily: {
            'open-sans': ['Open Sans', 'sans-serif'],
        },
    },
  },
  plugins: [],
}

