import type { BaseTranslation } from '../i18n-types';

const pl: BaseTranslation = {
  index: {
    email: 'Email',
    password: 'Hasło',
    loginButton: 'Zaloguj się',
    invalid: 'Nieprawidłowe dane logowania',
    login: 'Login',
  },
  admin: {
    schoolName: "Nazwa szkoły",
    page: "Strona: {0}",
    addSchool: "Dodaj szkołę",
    create: "Dodaj",
    cancel: "Anuluj",
    schoolNotFound: "Nie znaleziono szkoły"
  },
  dashboard: {
    teacher: "Nauczyciel",
    student: "Uczeń",
    admin: "Administrator",
    director: "Dyrektor"
  },
  user: {
    logout: "Wyloguj się"
  },
  common: {
    errors: {
      required: 'Pole wymagane',
      email: 'Podaj poprawny adres e-mail'
    }
  }
};

export default pl;
