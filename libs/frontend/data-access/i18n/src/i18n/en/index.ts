import type { Translation } from '../i18n-types';
import pl from '../pl';

const en: Translation = {
  ...pl as Translation,
  HI: 'Hi {name:string}! Please leave a star if you like this project: https://github.com/ivanhofer/typesafe-i18n',
};

export default en;
