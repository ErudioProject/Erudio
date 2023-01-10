module.exports = {
    "env": {
        "es2021": true
    },
    "parser": "@typescript-eslint/parser",
    "plugins": ["solid", "jsx-a11y"],
    "extends": ["eslint:recommended", "plugin:solid/typescript", "plugin:jsx-a11y/recommended"],
    "overrides": [
    ],
    "parserOptions": {
        "ecmaVersion": "latest",
        "sourceType": "module"
    },
    "rules": {
    }
}
