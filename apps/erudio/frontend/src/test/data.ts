import { PiiData, School, SchoolRelationType, UserFull } from "../../../bindings";
import { faker } from '@faker-js/faker/locale/pl';


export const createSchool = (name?: string): School => {
    return {
        id: faker.datatype.uuid(),
        name: name ?? faker.company.name(),
        previous_data: []
    }
}

let schools: Array<School> = [];
Array.from({ length: 100 }).forEach(() => schools.push(createSchool()))

const createSchoolRelation = (user_id: string) => {
    const school = faker.helpers.arrayElement(schools)
    return {
        user_id,
        school_id: school.id,
        school_relation_type: faker.helpers.arrayElement(["admin", "student", "teacher", "director"]) satisfies SchoolRelationType,
        school: school
    }
}

const maybeNull = <T>(result: () => T) => faker.datatype.boolean() ? result() : null

const createPiiData = (user_id: string): PiiData => {
    const name = faker.name.fullName()
    return {
        id: faker.datatype.uuid(),
        user_id,
        grammatical_form: faker.helpers.arrayElement(["masculinine", "feminine", "indeterminate"]),
        email: maybeNull(() => faker.internet.email()),
        pesel: maybeNull(() => faker.datatype.string(11)),
        birth_date: maybeNull(() => faker.date.birthdate().toDateString()),
        legal_name: name,
        display_name: name,
        phone_prefix: maybeNull(() => faker.datatype.number({ min: 0, max: 100 }).toString()),
        phone_number: maybeNull(() => faker.phone.number()),
        previous_data: []
    }
}

export const createUser = (): UserFull => {
    const user_id = faker.datatype.uuid()
    let schoolRelations: Array<{ user_id: string, school_id: string, school_relation_type: SchoolRelationType, school: School }> = [];
    Array.from({ length: 3 }).forEach(() => schoolRelations.push(createSchoolRelation(user_id)))
    return {
        id: user_id,
        two_factor_auth_settings: null,
        user_school_relation: schoolRelations,
        pii_data: createPiiData(user_id)
    }
}

let users: Array<UserFull> = [];
Array.from({ length: 100 }).forEach(() => users.push(createUser()))

const userId = faker.datatype.uuid()

let schoolRelations: Array<[SchoolRelationType, string]> = [];
Array.from({ length: 5 }).forEach(() => {
    const relation = createSchoolRelation(userId)
    schoolRelations.push([relation.school_relation_type, relation.school.name])
})


export const apiTestData = {
    authorizedMail: "test@example.com",
    authorizedPassword: "Test123!",
    userId,

    displayName: faker.name.fullName(),

    schoolRelations,

    schools,
    users,
}
