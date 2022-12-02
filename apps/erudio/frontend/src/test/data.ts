import { SchoolRelationType } from "../../../bindings";

export const apiTestData = {
    authorizedMail: "test@example.com",
    authorizedPassword: "Test123!",

    displayName: "Test User",
    schoolRelations: [
        {
            school_relation_type: "student" as SchoolRelationType,
            school: { name: "My School" }
        }, {
            school_relation_type: "teacher" as SchoolRelationType,
            school: { name: "My School 2" }
        }
    ]
}
