import type { Translation } from '../i18n-types';
import pl from '../pl';

const de: Translation = {
  ...(pl as Translation),
  // this is an example Translation, just rename or delete this folder if you want
  HI: 'Hallo {name:string}! Bitte hinterlasse einen Stern, wenn dir das Projekt gefällt: https://github.com/ivanhofer/typesafe-i18n',
};

export default de;
