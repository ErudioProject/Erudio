import { SchoolRelationType } from "../../../bindings";

export const apiTestData = {
    authorizedMail: "test@example.com",
    authorizedPassword: "Test123!",
    userId: "testid",

    displayName: "Test User",
    schoolRelations: [
        [
            "student" as SchoolRelationType,
            "My School"
        ],
        [
            "teacher" as SchoolRelationType,
            "Zespół Szkół Łączności"
        ]
    ]
}
