/* eslint-disable */
export default {
  displayName: 'frontend-data-access-api',
  preset: '../../../../jest.preset.js',
  globals: {
    'ts-jest': {
      tsconfig: '<rootDir>/tsconfig.spec.json'
    },
  },
  transform: {
    '^.+\\.t(sx|s)?$': [
      'ts-jest',
      {
        babelConfig: true
      }
    ]
  },
  moduleFileExtensions: ['ts', 'js', 'html', 'tsx'],
  coverageDirectory: '../../../../coverage/libs/frontend/data-access/api',
};
