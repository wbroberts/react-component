# React Component

I wanted to learn Rust and wanted to compare it to Go. So I made a simple CLI tool to generate some boilerplate react component files.

## Installation

```bash
cargo install react-component
```

## How To Use

```bash
react-component --help
```

```bash
react-component <component name> <...options>
```

### Example

```bash
react-component Example --path src/components/examples
```

Generates three files:

- `src/components/examples/Example.component.tsx`

```tsx
import React from 'react';

export type ExampleProps = {};

export const Example: React.ComponentType<ExampleProps> = ({}) => {
  return (<div>Example renders</div>);
};
```

- `src/components/examples/Example.test.tsx`

```tsx
import { screen, render } from '@testing-library/react';

import { Example } from './Example.component';

describe('Example', () => {
  it('renders', () => {
    render(<Example />);
    expect(screen.getByText(/Example renders/)).toBeDefined();
  });
});
```

- `src/components/examples/Example.stories.tsx`

```tsx
import { ComponentMeta, Story } from '@storybook/react';

import { Example, ExampleProps } from './Example.component';

const story: ComponentMeta<typeof Example> = {
  title: 'Example'
};

export const ExampleStory: Story<ExampleProps> = (args) => {
  return (<Example {...args} />)
};

export default story;
```
