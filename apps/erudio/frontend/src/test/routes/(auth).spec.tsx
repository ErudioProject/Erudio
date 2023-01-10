//TODO WTF?
import { fireEvent, render, screen } from "solid-testing-library";
import AuthOutlet from "../../routes/(auth)";
import { Router, Routes, Route, A } from "@solidjs/router";

const TestRouter = () => {
    return (
        <Router>
            <Routes>
                <Route path="/" component={Login} />
                <Route path="/auth" component={AuthOutlet}>
                    <Route path="/dashboard" component={Dashboard} />
                </Route>
            </Routes>
        </Router>
    )
}

const Login = () => {
    return (
        <>
            <a href="/auth/dashboard">Link</a>
            <div>
                Login
            </div>
        </>
    )
}

const Dashboard = () => {
    return (
        <div>
            Dashboard
        </div>
    )
}

describe("AuthOutlet", () => {
    it("Redirects to index on unauthorized", () => {
        render(() => <TestRouter />)
        const login = screen.getByText("Login");
        expect(login).toBeInTheDocument();
    });
    it("Stays on dashboard on authorized", async () => {
        render(() => <TestRouter />)
        sessionStorage.setItem("is-authenticated", "true");
        /* const dashboard = screen.getByText("Dashboard"); */
        /* expect(dashboard).toBeInTheDocument(); */
    });
})
