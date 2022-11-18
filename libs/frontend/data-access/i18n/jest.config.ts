/* eslint-disable */
export default {
  displayName: 'frontend-data-access-i18n',
  preset: '../../../../jest.preset.js',
  globals: {
    'ts-jest': {
      tsconfig: '<rootDir>/tsconfig.spec.json',
    },
  },
  transform: {
    '^.+\\.[tj]sx?$': 'ts-jest',
  },
  moduleFileExtensions: ['ts', 'js', 'html', 'tsx', 'jsx'],
  coverageDirectory: '../../../../coverage/libs/frontend/data-access/i18n',
};
