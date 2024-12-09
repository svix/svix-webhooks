module.exports = {
  "env": {
    "shared-node-browser": true,
    "es6": true,
  },
  "parser": "@typescript-eslint/parser",
  "parserOptions": {
    "sourceType": "module",
  },
  "settings": {
  },
  "plugins": [
    "@typescript-eslint",
  ],
  "extends": [
    "eslint:recommended",
    "plugin:@typescript-eslint/eslint-recommended",
    "plugin:@typescript-eslint/recommended"
  ],
  "rules": {
    "@typescript-eslint/explicit-function-return-type": "off",
    "@typescript-eslint/no-use-before-define": "off",
    "@typescript-eslint/no-non-null-assertion": "off",
    "@typescript-eslint/no-explicit-any": "off",
    "@typescript-eslint/explicit-module-boundary-types": "off",
    "@typescript-eslint/member-delimiter-style": ["error", {
      "multiline": {
        "delimiter": "semi",
        "requireLast": true
      },
      "singleline": {
        "delimiter": "semi",
        "requireLast": false
      }
    }],
    "@typescript-eslint/no-unused-vars": ["warn", {
      "vars": "all",
      "args": "all",
      "ignoreRestSiblings": true,
      "argsIgnorePattern": "^_",
    }],

    "quotes": "off",
    "@typescript-eslint/quotes": ["error", "double", { "allowTemplateLiterals": true, "avoidEscape": true }],
    "semi": "off",
    "@typescript-eslint/semi": ["error", "always", { "omitLastInOneLineBlock": true }],
    "comma-dangle": ["error", {
      "arrays": "always-multiline",
      "objects": "always-multiline",
      "imports": "always-multiline",
      "exports": "always-multiline",
      "functions": "never"
    }],
    "comma-spacing": ["error"],
    "eqeqeq": ["error", "smart"],
    "indent": "off",
    "@typescript-eslint/indent": 0,
    "no-multi-spaces": "error",
    "object-curly-spacing": ["error", "always"],
    "arrow-parens": "error",
    "arrow-spacing": "error",
    "key-spacing": "error",
    "keyword-spacing": "error",
    "func-call-spacing": "off",
    "@typescript-eslint/func-call-spacing": ["error"],
    "space-before-function-paren": ["error", {
        "anonymous": "always",
        "named": "never",
        "asyncArrow": "always"
    }],
    "space-in-parens": ["error", "never"],
    "space-before-blocks": "error",
    "curly": ["error", "all"],
    "space-infix-ops": "error",
    "consistent-return": "error",
    "jsx-quotes": ["error"],
    "array-bracket-spacing": "error",
    "brace-style": "off",
    "@typescript-eslint/brace-style": [
      "error",
      "1tbs",
      { allowSingleLine: true },
    ],
    "no-useless-constructor": "off",
    "@typescript-eslint/no-useless-constructor": "warn",
  }
};
