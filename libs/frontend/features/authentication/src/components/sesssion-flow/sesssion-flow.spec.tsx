import SesssionFlow from './SesssionFlow';
import { render } from 'solid-testing-library';

it('it works', async () => {
  const { getByText } = render(SesssionFlow);

  expect(getByText('Hello component!'));
});
