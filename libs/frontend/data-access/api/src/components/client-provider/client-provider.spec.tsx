import ClientProvider from './ClientProvider'
import { render } from 'solid-testing-library'

it('it works', async () => {
  const { getByText } = render(ClientProvider)

  expect(getByText('Hello component!'));
})
