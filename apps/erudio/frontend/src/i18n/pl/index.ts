import type { BaseTranslation } from '../i18n-types';

const pl: BaseTranslation = {
  index: {
    email: 'Email',
    password: 'Hasło',
    loginButton: 'Zaloguj się',
    invalid: 'Nieprawidłowe dane logowania',
    login: 'Login',
    errors: {
      required: 'Pole wymagane',
      email: 'Podaj poprawny adres e-mail'
    }
  },
  dashboard: {
    teacher: "Nauczyciel",
    student: "Uczeń",
    admin: "Administrator",
    director: "Dyrektor"
  }
};

export default pl;
