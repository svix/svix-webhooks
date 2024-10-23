/** @type {import('ts-jest').JestConfigWithTsJest} */
module.exports = {
  rootDir: "./src",
  preset: "ts-jest",
  testEnvironment: "node",
  transform: {
    "^.+\\.[tj]s$": [
      "ts-jest",
      {
        tsconfig: {
          allowJs: true,
        },
      },
    ],
  },
  transformIgnorePatterns: ["<rootDir>/node_modules/"],
};
