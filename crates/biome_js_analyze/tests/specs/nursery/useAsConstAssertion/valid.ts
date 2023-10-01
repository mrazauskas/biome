/* should not generate diagnostics */
// ref: https://github.com/typescript-eslint/typescript-eslint/blob/main/packages/eslint-plugin/tests/rules/prefer-as-const.test.ts

let foo = 'baz' as const;
let foo = 1 as const;
let foo = { bar: 'baz' as const };
let foo = { bar: 1 as const };
let foo = { bar: 'baz' };
let foo = { bar: 2 };
let foo = <bar>'bar';
let foo = <string>'bar';
let foo = 'bar' as string;
let foo = `bar` as `bar`;
let foo = `bar` as `foo`;
let foo = `bar` as 'bar';
let foo: string = 'bar';
let foo: number = 1;
let foo: 'bar' = baz;
let foo = 'bar';
let foo: 'bar';
let foo = { bar };
let foo: 'baz' = 'baz' as const;

class foo {
  bar = 'baz';
}

class foo {
  bar: 'baz';
}

class foo {
  bar;
}

class foo {
  bar = <baz>'baz';
}

class foo {
  bar: string = 'baz';
}

class foo {
  bar: number = 1;
}

class foo {
  bar = 'baz' as const;
}

class foo {
  bar = 2 as const;
}

class foo {
  get bar(): 'bar' {}
  set bar(bar: 'bar') {}
}

class foo {
  bar = () => 'bar' as const;
}

type BazFunction = () => 'baz';

class foo {
  bar: BazFunction = () => 'bar';
}

class foo {
  bar(): void {}
}
