/** @type {import('tailwindcss').Config} */

export const content = {
  darkMode: "class",
  files: ["*.html", "./src/**/*.rs"],
  transform: {
    rs: (content) => content.replace(/(?:^|\s)class:/g, " "),
  },
};
export const theme = {
  extend: {},
};
export const plugins = [];
