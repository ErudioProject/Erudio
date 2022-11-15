import LoginPage from './LoginPage'
import { render } from 'solid-testing-library'

it('it works', async () => {
  const { getByText } = render(LoginPage)

  expect(getByText('Hello component!'));
})
