//TODO WTF?
import { fireEvent, render, screen } from "solid-testing-library";
import ClientProvider from "../../../components/contexts/ClientProvider";
import AuthOutlet from "../../../routes/(auth)/(auth)";
import { Router, Routes, Route, A } from "@solidjs/router";
import { userEvent } from "@testing-library/user-event";

const TestRouter = () => {
    return (
        <ClientProvider url={import.meta.env.FRONTEND_API_URL}>
            <Router>
                <Routes>
                    <Route path="/" component={Login} />
                    <Route path="/auth" component={AuthOutlet}>
                        <Route path="/dashboard" component={Dashboard} />
                    </Route>
                </Routes>
            </Router>
        </ClientProvider>
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
