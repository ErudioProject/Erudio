/* eslint-disable */
export default {
  displayName: 'frontend-data-access-api',
  preset: '../../../../jest.preset.js',
  globals: {
    'ts-jest': {
      tsconfig: '<rootDir>/tsconfig.spec.json',
      babelConfig: {
        presets: ['babel-preset-solid', '@babel/preset-env'],
      },
    },
  },
  transform: {
    '^.+\\.t(sx|s)?$': 'ts-jest',
  },
  transformIgnorePatterns: ['node_modules/(?!solid-js.*|.*(?<=.[tj]sx))$'],
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['./src/setupTests.ts'],
  moduleFileExtensions: ['ts', 'js', 'html', 'tsx'],
  coverageDirectory: '../../../../coverage/libs/frontend/data-access/api',
};
