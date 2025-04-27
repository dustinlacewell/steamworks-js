# CLAUDE.md

steamworks.ts is an attempt to combine steamworks-rs and napi-rs to build a slick, modern, strict, dev friendly typescript library for interacting with steamworks.

steamworks-rs is located as a submodule in swrs for reference.

src/ is where our own rust code goes.

run `npm run build` to build the project.

there is a test file in main.ts we can use to test things.

however, you're in WSL, so it wont work for you. I can run it in Windows and report back.

But you can still build to and read the errors or see the success.

While we will want to provide a nice typescript library with types, etc, our first goal is simply to get this working at all. Our first goal is simply to list my workshop subscriptions. If we can get that working, we can go ahead and finish the rest of the library and cover all of steamworks-rs.

IF I DID NOT PROVIDE DOCUMENTATION ON HOW TO DO A THING - YOU *MUST ASK ME*

# Exports
Unlike defining modules in Node.js, we don't need to explicitly register exports like module.exports.xxx = xxx.

The #[napi] macro will automatically generate module registering code for you. This auto registering idea was inspired by node-bindgen.

## Function
Exporting a function is incredibly simple. Just decorate a normal rust function with #[napi]:

lib.rs
```rust
#[napi]
fn sum(a: u32, b: u32) -> u32 {
	a + b
}
```

## Const
lib.rs
```rust
#[napi]
pub const DEFAULT_COST: u32 = 12;
```

index.d.ts
```ts
export const DEFAULT_COST: number
```

## Class
See class section for more details.

lib.rs
```rust
#[napi(constructor)]
struct Animal {
  pub name: String,
  pub kind: u32,
}
 
#[napi]
impl Animal {
  #[napi]
  pub fn change_name(&mut self, new_name: String) {
    self.name = new_name;
  }
}
```

## Enum
See enum section for more details.

lib.rs
```
#[napi]
pub enum Kind {
  Dog,
  Cat,
  Duck,
}
```

# Class

import { Callout } from 'nextra-theme-docs'

<Callout>
  There is no concept of a class in Rust. We use `struct` to represent a
  JavaScript `Class`.
</Callout>

## `Constructor`

### Default `constructor`

If all fields in a `Rust` struct are `pub`, then you can use `#[napi(constructor)]` to make the `struct` have a default `constructor`.

```rust filename="lib.rs"
#[napi(constructor)]
pub struct AnimalWithDefaultConstructor {
  pub name: String,
  pub kind: u32,
}
```

```ts filename="index.d.ts"
export class AnimalWithDefaultConstructor {
  name: string
  kind: number
  constructor(name: string, kind: number)
}
```

### Custom `constructor`

If you want to define a custom `constructor`, you can use `#[napi(constructor)]` on your constructor `fn` in the struct `impl` block.

```rust filename="lib.rs"
// A complex struct which cannot be exposed to JavaScript directly.
pub struct QueryEngine {}

#[napi(js_name = "QueryEngine")]
pub struct JsQueryEngine {
  engine: QueryEngine,
}

#[napi]
impl JsQueryEngine {
  #[napi(constructor)]
  pub fn new() -> Self {
    JsQueryEngine { engine: QueryEngine::new() }
  }
}
```

```ts filename="index.d.ts"
export class QueryEngine {
  constructor()
}
```

<Callout type="warning" emoji="⚠️">
  **NAPI-RS** does not currently support `private constructor`. Your custom
  constructor must be `pub` in Rust.
</Callout>

## Factory

Besides `constructor`, you can also define factory methods on `Class` by using `#[napi(factory)]`.

```rust filename="lib.rs"
// A complex struct which cannot be exposed to JavaScript directly.
pub struct QueryEngine {}

#[napi(js_name = "QueryEngine")]
pub struct JsQueryEngine {
  engine: QueryEngine,
}

#[napi]
impl JsQueryEngine {
  #[napi(factory)]
  pub fn with_initial_count(count: u32) -> Self {
    JsQueryEngine { engine: QueryEngine::with_initial_count(count) }
  }
}
```

```ts filename="index.d.ts"
export class QueryEngine {
  static withInitialCount(count: number): QueryEngine
  constructor()
}
```

<Callout type="warning" emoji="⚠️">
  If no `#[napi(constructor)]` is defined in the `struct`, and you attempt to
  create an instance (`new`) of the `Class` in JavaScript, an error will be
  thrown.
</Callout>

```js {3} filename="test.mjs"
import { QueryEngine } from './index.js'

new QueryEngine() // Error: Class contains no `constructor`, cannot create it!
```

## `class method`

You can define a JavaScript class method with `#[napi]` on a struct method in **Rust**.

```rust filename="lib.rs"
// A complex struct which cannot be exposed to JavaScript directly.
pub struct QueryEngine {}

#[napi(js_name = "QueryEngine")]
pub struct JsQueryEngine {
  engine: QueryEngine,
}

#[napi]
impl JsQueryEngine {
  #[napi(factory)]
  pub fn with_initial_count(count: u32) -> Self {
    JsQueryEngine { engine: QueryEngine::with_initial_count(count) }
  }

  /// Class method
  #[napi]
  pub async fn query(&self, query: String) -> napi::Result<String> {
    self.engine.query(query).await
  }

  #[napi]
  pub fn status(&self) -> napi::Result<u32> {
    self.engine.status()
  }
}
```

```ts filename="index.d.ts"
export class QueryEngine {
  static withInitialCount(count: number): QueryEngine
  constructor()
  query(query: string) => Promise<string>
  status() => number
}
```

<Callout type="warning" emoji="⚠️">
  `async fn` needs the `napi4` and `tokio_rt` features to be enabled.
</Callout>

<Callout>
Any `fn` in `Rust` that returns `Result<T>` will be treated as `T` in JavaScript/TypeScript. If the `Result<T>` is `Err`, a JavaScript Error will be thrown.
</Callout>

## `Getter`

Define [JavaScript class `getter`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/get) using `#[napi(getter)]`. The Rust `fn` must be a struct method, not an associated function.

```rust {22-25} filename="lib.rs"
// A complex struct which cannot be exposed to JavaScript directly.
pub struct QueryEngine {}

#[napi(js_name = "QueryEngine")]
pub struct JsQueryEngine {
  engine: QueryEngine,
}

#[napi]
impl JsQueryEngine {
  #[napi(factory)]
  pub fn with_initial_count(count: u32) -> Self {
    JsQueryEngine { engine: QueryEngine::with_initial_count(count) }
  }

  /// Class method
  #[napi]
  pub async fn query(&self, query: String) -> napi::Result<String> {
    self.engine.query(query).await
  }

  #[napi(getter)]
  pub fn status(&self) -> napi::Result<u32> {
    self.engine.status()
  }
}
```

```ts {4} filename="index.d.ts"
export class QueryEngine {
  static withInitialCount(count: number): QueryEngine
  constructor()
  get status(): number
}
```

# Values

Conversions between Rust and JavaScript types.

### Undefined

Represent `undefined` in JavaScript.

```rust {3} filename="lib.rs"
#[napi]
fn get_undefined() -> Undefined {
	()
}

// default return or empty tuple `()` are `undefined` after converted into JS value.
#[napi]
fn log(n: u32) {
	println!("{}", n);
}
```

```ts filename="index.d.ts"
export function getUndefined(): undefined
export function log(n: number): void
```

### Null

Represents `null` value in JavaScript.

```rust {3} filename="lib.rs"
#[napi]
fn get_null() -> Null {
	Null
}

#[napi]
fn get_env(env: String) -> Option<String> {
	match std::env::var(env) {
		Ok(val) => Some(val),
		Err(e) => None,
	}
}
```

```ts filename="index.d.ts"
export function getNull(): null
export function getEnv(env: string): string | null
```

### Numbers

JavaScript `Number` type with Rust Int/Float types: `u32`, `i32`, `i64`, `f64`.

For Rust types like `u64`, `u128`, `i128`, checkout [`BigInt`](#bigint) section.

```rust filename="lib.rs"
#[napi]
fn sum(a: u32, b: i32) -> i64 {
	(b + a as i32).into()
}
```

```ts filename="index.d.ts"
export function sum(a: number, b: number): number
```

### String

Represents JavaScript `String` type.

```rust {3} filename="lib.rs"
#[napi]
fn greet(name: String) -> String {
	format!("greeting, {}", name)
}
```

```ts filename="index.d.ts"
export function greet(name: string): string
```

### Boolean

Represents JavaScript `Boolean` type.

```rust filename="lib.rs"
#[napi]
fn is_good() -> bool {
	true
}
```

```ts filename="index.d.ts"
export function isGood(): boolean
```

### Buffer

```rust filename="lib.rs"
#[napi]
fn with_buffer(buf: Buffer) {
  let buf: Vec<u8> = buf.into();
  // do something
}

#[napi]
fn read_buffer(file: String) -> Buffer {
	Buffer::from(std::fs::read(file).unwrap())
}
```

```ts filename="index.d.ts"
export function withBuffer(buf: Buffer): void
export function readBuffer(file: string): Buffer
```

### Object

Represents JavaScript anonymous object values.

import { Callout } from 'nextra-theme-docs'

<Callout type="warning" emoji="⚠️">
**Performance**

The costs of `Object` conversions between JavaScript and Rust are higher than other primitive types.

Every call of `Object.get("key")` is actually dispatched to node side including two steps: fetch value, convert JS to rust value, and so is `Object.set("key", v)`.

</Callout>

```rust filename="lib.rs"
#[napi]
fn keys(obj: Object) -> Vec<String> {
	Object::keys(&obj).unwrap()
}

#[napi]
fn log_string_field(obj: Object, field: String) {
	println!("{}: {:?}", &field, obj.get::<String>::(field.as_ref()));
}

#[napi]
fn create_obj(env: Env) -> Object {
	let mut obj = env.create_object().unwrap();
	obj.set("test", 1).unwrap();
	obj
}
```

```ts filename="index.d.ts"
export function keys(obj: object): Array<string>
export function logStringField(obj: object): void
export function createObj(): object
```

If you want **NAPI-RS** to convert objects from JavaScript with the same shape defined in Rust, you can use the `#[napi]` macro with the `object` attribute.

```rust filename="lib.rs"
/// #[napi(object)] requires all struct fields to be public
#[napi(object)]
struct PackageJson {
	pub name: String,
	pub version: String,
	pub dependencies: Option<HashMap<String, String>>,
	pub dev_dependencies: Option<HashMap<String, String>>,
}

#[napi]
fn log_package_name(package_json: PackageJson) {
	println!("name: {}", package_json.name);
}

#[napi]
fn read_package_json() -> PackageJson {
	// ...
}
```

```ts filename="index.d.ts"
export interface PackageJson {
  name: string
  version: string
  dependencies: Record<string, string> | null
  devDependencies: Record<string, string> | null
}
export function logPackageName(packageJson: PackageJson): void
export function readPackageJson(): PackageJson
```

<Callout type="warning" emoji="⚠️">
**Clone over Reference**

The `#[napi(object)]` struct passed in Rust `fn` is cloned from **_JavaScript Object_**. Any mutation on it will not be reflected to the original **_JavaScript_** object.

</Callout>

```rust filename="lib.rs"
/// #[napi(object)] requires all struct fields to be public
#[napi(object)]
struct Animal {
	pub name: String,
}

#[napi]
fn change_animal_name(mut animal: Animal) {
  animal.name = "cat".to_string();
}
```

```js
const animal = { name: 'dog' }
changeAnimalName(animal)
console.log(animal.name) // "dog"
```

### Array

Because `Array` values in JavaScript can hold elements with different types, but rust `Vec<T>`
can only contains same type elements. So there two different way for array types.

<Callout type="warning" emoji="⚠️">
**Performance**

Because JavaScript `Array` type is backed with `Object` actually, so the performance of manipulating `Array`s would be the same as `Object`s.

The conversion between `Array` and `Vec<T>` is even heavier, which is in `O(2n)` complexity.

</Callout>

```rust filename="lib.rs"
#[napi]
fn arr_len(arr: Array) -> u32 {
  arr.len()
}

#[napi]
fn get_tuple_array(env: Env) -> Array {
  let mut arr = env.create_array(2).unwrap();

  arr.insert(1).unwrap();
  arr.insert("test").unwrap();

  arr
}

#[napi]
fn vec_len(nums: Vec<u32>) -> u32 {
  u32::try_from(nums.len()).unwrap()
}

#[napi]
fn get_nums() -> Vec<u32> {
  vec![1, 1, 2, 3, 5, 8]
}
```

```ts filename="index.d.ts"
export function arrLen(arr: unknown[]): number
export function getTupleArray(): unknown[]
export function vecLen(nums: Array<number>): number
export function getNums(): Array<number>
```

### BigInt

This requires the `napi6` feature.

<Callout type="warning" emoji="⚠️">
  The only way to pass `BigInt` in `Rust` is using `BigInt` type. But you can
  return `BigInt`, `i64n`, `u64`, `i128`, `u128`. Return `i64` will be treated
  as `JavaScript` number, not `BigInt`.
</Callout>

<Callout>
  The reason why Rust fn can't receive `i128` `u128` `u64` `i64n` as arguments
  is that they may lose precision while converting JavaScript `BigInt` into
  them. You can use `BigInt::get_u128`, `BigInt::get_i128` ... to get the value
  in `BigInt`. The return value of these methods also indicates if precision is
  lost.
</Callout>

```rust filename="lib.rs"
/// the return value of `get_u128` is (signed: bool, value: u128, lossless: bool)
#[napi]
fn bigint_add(a: BigInt, b: BigInt) -> u128 {
  a.get_u128().1 + b.get_u128().1
}

#[napi]
fn create_big_int_i128() -> i128 {
  100
}
```

```ts filename="index.d.ts"
export function bigintAdd(a: BigInt, b: BigInt): BigInt
export function createBigIntI128(): BigInt
```

### TypedArray

<Callout>
  Unlike JavaScript Object, the `TypedArray` passed into Rust fn is a
  **Reference**. No data `Copy` or `Clone` will be performed. Every mutation on
  the `TypedArray` will be reflected to the original JavaScript `TypedArray`.
</Callout>

```rust filename="lib.rs"
#[napi]
fn convert_u32_array(input: Uint32Array) -> Vec<u32> {
  input.to_vec()
}

#[napi]
fn create_external_typed_array() -> Uint32Array {
  Uint32Array::new(vec![1, 2, 3, 4, 5])
}

#[napi]
fn mutate_typed_array(mut input: Float32Array) {
  for item in input.as_mut() {
    *item *= 2.0;
  }
}
```

# Object

`Object` is very easy to confuse with the use of `Class`. Unlike `Class` you can't assign `function` or `method` to `Object`.

```rust filename="lib.rs"
#[napi(object)]
pub struct Pet {
  pub name: String,
  pub kind: u32,
}
```

Any `impl` block of this `struct` will not affect the JavaScript `Object`.

<Callout type="warning" emoji="⚠️">
  If you want to convert a Rust `struct` into JavaScript `Object` using
  `#[napi(object)]` attribute, you need to mark all of its fields as `pub`.
</Callout>

Once `struct` is marked as `#[napi(object)]`, you can use it as a function argument type or return type.

```rust filename="lib.rs"
#[napi(object)]
pub struct Pet {
  pub name: String,
  pub kind: u32,
}

#[napi]
fn print_pet(pet: Pet) {
  println!("{}", pet.name);
}

#[napi]
fn create_cat() -> Pet {
  Pet {
    name: "cat".to_string(),
    kind: 1,
  }
}
```

<Callout type="warning" emoji="⚠️">
  The JavaScript Object passed in or returned from Rust is cloned. This means
  any mutation on JavaScript `Object` will not affect the original Rust
  `struct`. And any mutation on Rust `struct` will not affect the JavaScript
  `Object` either.
</Callout>

# Function

Defining a JavaScript `function` is very simple in **NAPI-RS**. Just a plain Rust `fn`:

```rust filename="lib.rs"
#[napi]
fn sum(a: u32, b: u32) -> u32 {
  a + b
}
```

The most important thing you should keep in mind is **_NAPI-RS fn does not support every Rust type_**. Here is a table to illustrate how JavaScript types map to Rust types when they are `fn` arguments and return types:

## Arguments

| Rust Type                                               | JavaScript Type                                                               |
| ------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `u32`                                                   | `number`                                                                      |
| `i32`                                                   | `number`                                                                      |
| `i64`                                                   | `number`                                                                      |
| `f64`                                                   | `number`                                                                      |
| `bool`                                                  | `boolean`                                                                     |
| `String`                                                | `string`                                                                      |
| `Latin1String`                                          | `string`                                                                      |
| `UTF16String`                                           | `string`                                                                      |
| `#[napi(object)] struct`                                | `Object`                                                                      |
| `& struct` or `&mut struct`                             | [Class](./class) instance                                                     |
| `serde_json::Map`                                       | `Object`                                                                      |
| `serde_json::Value`                                     | `unknown`                                                                     |
| `std::collections::HashMap`                             | `Object`                                                                      |
| `Array`                                                 | `unknown[]`                                                                   |
| `Vec<T>` T must be types in this table                  | T[]                                                                           |
| `Buffer`                                                | `Buffer`                                                                      |
| `External`                                              | [`External`](https://nodejs.org/api/n-api.html#napi_create_external)          |
| `Null`                                                  | `null`                                                                        |
| `Undefined` / `()`                                      | `undefined`                                                                   |
| `Option<T>`                                             | `T or null`                                                                   |
| `Fn(Arg) ->T` `Arg `and `T` must be types in this table | `(arg: Arg) => T`                                                             |
| `Promise<T>`                                            | `Promise<T>`                                                                  |
| `AbortSignal`                                           | [`AbortSignal`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal) |
| `JsSymbol`                                              | `Symbol`                                                                      |
| `Int8Array` / `Uint8Array` / `Int16Array`...            | `TypedArray`                                                                  |
| `BigInt`                                                | `BigInt`                                                                      |

## Return Type

| Rust Type                                    | JavaScript Type  |
| -------------------------------------------- | ---------------- |
| `u32`                                        | `number`         |
| `i32`                                        | `number`         |
| `i64`                                        | `number`         |
| `f64`                                        | `number`         |
| `bool`                                       | `boolean`        |
| `String`                                     | `string`         |
| `Latin1String`                               | `string`         |
| `UTF16String`                                | `string`         |
| `#[napi(object)] struct`                     | `Object`         |
| `#[napi] struct`                             | [Class](./class) |
| `serde_json::Map`                            | `Object`         |
| `serde_json::Value`                          | `unknown`        |
| `std::collections::HashMap`                  | `Object`         |
| `Array`                                      | `unknown[]`      |
| `Vec<T>` T must be types in this table       | `T[]`            |
| `Buffer`                                     | `Buffer`         |
| `External`                                   | `External`       |
| `Null`                                       | `null`           |
| `Undefined` / `()`                           | `undefined`      |
| `Option<T>`                                  | `T` or `null`    |
| `AsyncTask<Task<JsValue = T>>`               | `Promise<T>`     |
| `JsSymbol`                                   | `Symbol`         |
| `Int8Array` / `Uint8Array` / `Int16Array`... | `TypedArray`     |
| `BigInt`                                     | `BigInt`         |
| `i64n`                                       | `BigInt`         |
| `i128`                                       | `BigInt`         |
| `u128`                                       | `BigInt`         |

# Wrap native object

Wraps a native instance in a JavaScript object. The native instance can be retrieved later using Env::unwrap.

```rust
struct Native {
  value: i32,
}
 
#[js_function(1)]
fn attach_native_object(ctx: CallContext) -> Result<JsUndefined> {
  let count: i32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let mut this: JsObject = ctx.this_unchecked();
  ctx
    .env
    .wrap(&mut this, Native { value: count + 100 })?;
  ctx.env.get_undefined()
}
 
#[js_function(1)]
fn get_native_object(ctx: CallContext) -> Result<JsNumber> {
  let count: i32 = ctx.get::<JsNumber>(0)?.try_into()?;
  let mut this: JsObject = ctx.this_unchecked();
  let native: Native = ctx
    .env
    .unwrap(&mut this)?;
  ctx.env.create_int32(native.value + 1)
}
```

```javascript
const obj = {
  attach: attachNativeObject,
  get: getNativeObject,
}
 
obj.attach(100)
obj.get() // 101
```

# JsValue
JsValue represent the JavaScript value in Rust.

## JsUndefined
Represent undefined in JavaScript. It can not be casted into Rust value, and no Rust value can be casted into JsUndefined.

The only way to create JsUndefined in Rust is calling Env::get_undefined().

## JsNull
Represent null in JavaScript. Like JsUndefined, it can not be casted to Rust value and no Rust value can be casted to it.

The only way to create JsNull in Rust is calling Env::get_null().

## JsNumber
f64	u32	i32	i64
From	Env::create_double	Env::create_uint32	Env::create_int32	Env::create_int64
Into	TryInto	TryInto	TryInto	TryInto

```rust
use std::convert::TryInto;
 
use napi::*;
 
#[js_function(1)]
fn fib(ctx: CallContext) -> Result<JsNumber> {
  let input_number: i64 = ctx.get::<JsNumber>(0)?.try_into()?;
  ctx.env.create_int64(fibonacci_native(input_number))
}
 
#[inline(always)]
fn fibonacci_native(n: i64) -> i64 {
  match n {
    1 | 2 => 1,
    _ => fibonacci_native(n - 1) + fibonacci_native(n - 2),
  }
}
```

## JsBoolean
JsBoolean represent boolean value in JavaScript.

Use JsBoolean::get_value() to convert JsBoolean into Rust bool. And Env::get_boolean() to convert Rust bool into JsBoolean.

```rust
#[js_function(1)]
fn not(ctx: CallContext) -> Result<JsBoolean> {
  let condition: JsBoolean = ctx.get(0)?;
  ctx.env.get_boolean(!condition.get_value()?)
}
not(true) // false
not(false) // true
```

## JsString
Represent string value in JavaScript. There 3 types of string encoding you can interactive with N-API: utf8, utf16 and latin1.

Using Env::create_string() you could create JsString from Rust &str. If you want get utf8 value from JsString, you must using the JsString::into_utf8() to get utf8 value explicit.

```rust
#[js_function(1)]
fn world(ctx: CallContext) -> Result<JsString> {
  // or into_utf16/into_latin1 here
  // if you want using into_latin1, you must enable `latin1` feature for `napi-rs`.
  let input_string = ctx.get::<JsString>(0)?.into_utf8()?;
  let output = format!("{} world!", input_string.as_str()?);
  ctx.env.create_string(output.as_str())
}
world('hello') // hello world!
```

## JsBuffer
Represents a Buffer value in Node.js. Passing data between JavaScript and Rust using JsBuffer has a small overhead so you might prefer it over other types.

For example, in some cases, converting a JavaScript string into a Buffer, pass it to Rust as a JsBuffer and cast it as a &[u8] is faster than passing the string directly to Rust. The string implementation in V8 is far more complicated than the ArrayBuffer one, which is what Buffer is implemented by.

```rust
#[js_function(1)]
fn set_buffer(ctx: CallContext) -> Result<JsUndefined> {
  let buf = &mut ctx.get::<JsBuffer>(0)?.into_value()?; // &mut [u8]
  buf[0] = 1;
  buf[1] = 2;
  ctx.env.get_undefined()
}
setBuffer(Buffer.from([0, 1])) // <Buffer 01 02>
```

## JsSymbol
Represent Symbol value in JavaScript. You can create JsSymbol from &str or JsString

```rust
// create from &str
#[js_function(1)]
fn create_symbol(ctx: CallContext) -> Result<JsSymbol> {
  let desc = ctx.get::<JsString>(0)?.into_utf8()?;
  ctx.env.create_symbol(Some(desc.as_str()?))
}
// create from JsString
#[js_function(1)]
fn create_symbol(ctx: CallContext) -> Result<JsSymbol> {
  let desc = ctx.get::<JsString>(0)?;
  ctx.env.create_symbol_from_js_string(desc)
}
```
## JsObject
Represent Object value in JavaScript. There are many object related API's in JsObject. See document.

```rust
#[js_function(1)]
fn set_bar(ctx: CallContext) -> Result<JsUndefined> {
  let mut obj = ctx.get::<JsObject>(0)?;
  let bar: JsString = obj.get_named_property("bar")?;
  let bar_str = bar.into_utf8()?;
  obj.set_named_property("bar", ctx.env.create_string_from_std(format!("{} bar", bar_str.as_str()?)))?;
  ctx.env.get_undefined()
}
setBar({ bar: 'bar' }) // { bar: "bar bar" }
```
## JsDate
Represent Date object in JavaScript. JavaScript Date objects are described in Section 20.3 of the ECMAScript Language Specification.

## JsBigint
Represent Bigint value in JavaScript.

## JsExternal
This API allocates a JavaScript value with external data attached to it. This is used to pass external data through JavaScript code, so it can be retrieved later by native code using Env::get_value_external.

# Reference
Ref is very similar to Rc in Rust. However it will not decrease the ref count when dropped, you need to call the unref method on it manually.

In some scenarios, you may need to manually extend the lifetime of certain JavaScript objects to prevent them from being reclaimed by the GC prematurely. Unlike using objects in JavaScript, keeping a reference or value of a JsObject in Rust is opaque to the Node.js GC system, meaning that the GC doesn't know that you still have an Object value that you want to use at some point in the future. GC will ruthlessly recycle it when it sees fit. At this point we need to create a Reference for this object, which is equivalent to telling the Node.js GC system: hey I still need this object, don't recycle it yet.

We usually need to manually extend the lifetime of an Object when doing some asynchronous operations, such as storing the value of some object in Task or ThreadSafeFunction. These objects can't be destroyed after the function finishes executing, so we need to wait until the asynchronous task finishes executing and then manually call the unref method to tell the GC that we don't need the object anymore.

```rust 
struct Hash(Ref<JsBufferValue>);
 
impl Task for Hash {
  type Output = String;
  type JsValue = JsString;
 
  fn compute(&mut self) -> Result<Self::Output> {
    Ok(base64(&self.0))
  }
 
  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    let result = env.create_string_from_std(output);
    self.0.unref(env)?;
    result
  }
}
 
#[js_function(1)]
// return Promise Object
fn async_hash(ctx: CallContext) -> Result<JsObject> {
  let input_data = ctx.get::<JsBuffer>(0)?.into_ref()?;
  let hash = Hash(input_data);
  ctx.env.spawn(hash).map(|async_task| async_task.promise_object())
}
 
fn base64(data: &[u8]) -> String {
  todo!();
}
asyncHash(Buffer::from([1, 2])).then((result) => {
  console.log(result) // 0102
})
```
For JavaScript objects other than JsBuffer, you can use Env::create_reference to create references for them and fetch these JavaScript objects back with Env::get_reference_value later. For example:

```rust
struct CallbackContext {
  callback: Ref<()>
}
 
#[napi]
pub fn wrap_in_obj(env: Env, js_fn: JsFunction) -> Result<JsObject> {
  let mut js_obj = env.create_object()?;
  // create a reference for the javascript function
  let js_fn_ref = env.create_reference(js_fn)?;
  let ctx = CallbackContext {
    callback: js_fn_ref,
  };
  // wrap it in an object
  env.wrap(&mut js_obj, ctx)?;
  Ok(js_obj)
}
 
#[napi]
pub fn call_wrapped_fn(env: Env, js_obj: JsObject) -> Result<()> {
  let ctx: &mut CallbackContext = env.unwrap(&js_obj)?;
  let js_fn: JsFunction = env.get_reference_value(&ctx.callback)?;
  // the javascript function should not be reclaimed before we call Ref::unref()
  js_fn.call_without_args(None)?;
  Ok(())
}
const logSomething = () => {
  console.log('hello')
}
const obj = wrapInObj(logSomething)
callWrappedFn(obj) // log 'hello'
```

You should always get JavaScript objects from references instead of caching these JavaScript objects directly. Referenced JsValue could still become invalid after some time.

# AsyncTask

We need to talk about `Task` before talking about `AsyncTask`.

## `Task`

Addon modules often need to leverage async helpers from libuv as part of their implementation. This allows them to schedule work to be executed asynchronously so that their methods can return in advance of the work being completed. This allows them to avoid blocking the overall execution of the Node.js application.

The `Task` trait provides a way to define such an asynchronous task that needs to run in the libuv thread. You can implement the `compute` method, which will be called in the libuv thread.

```rust {11-13} filename="lib.rs"
use napi::{Task, Env, Result, JsNumber};

struct AsyncFib {
  input: u32,
}

impl Task for AsyncFib {
  type Output = u32;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(fib(self.input))
  }

  fn resolve(&mut self, env: Env, output: u32) -> Result<Self::JsValue> {
    env.create_uint32(output)
  }
}
```

`fn compute` ran on the libuv thread, you can run some heavy computation here, which will not block the main JavaScript thread.

You may notice there are two associated types on the `Task` trait. The `type Output` and the `type JsValue`. `Output` is the return type of the `compute` method. `JsValue` is the return type of the `resolve` method.

import { Callout } from 'nextra-theme-docs'

<Callout>
  We need separate `type Output` and `type JsValue` because we can not call the
  JavaScript function back in `fn compute`, it is not executed on the main
  thread. So we need `fn resolve`, which runs on the main thread, to create the
  `JsValue` from `Output` and `Env` and call it back in JavaScript.
</Callout>

You can use the low-level API `Env::spawn` to spawn a defined `Task` in the libuv thread pool. See example in [Reference](../compat-mode/concepts/ref).

In addition to `compute` and `resolve`, you can also provide `reject` method to do some clean up when `Task` runs into error, like `unref` some object:

```rust {28} filename="lib.rs"
struct CountBufferLength {
  data: Ref<JsBufferValue>,
}

impl CountBufferLength {
  pub fn new(data: Ref<JsBufferValue>) -> Self {
    Self { data }
  }
}

impl Task for CountBufferLength {
  type Output = usize;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    if self.data.len() == 10 {
      return Err(Error::from_reason("len can't be 10".to_string()));
    }
    Ok((&self.data).len())
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    self.data.unref(env)?;
    env.create_uint32(output as _)
  }

  fn reject(&mut self, env: Env, err: Error) -> Result<Self::JsValue> {
    self.data.unref(env)?;
    Err(err)
  }
}
```

You can also provide a `finally` method to do something after `Task` is `resolved` or `rejected`:

```rust {27} filename="lib.rs"
struct CountBufferLength {
  data: Ref<JsBufferValue>,
}

impl CountBufferLength {
  pub fn new(data: Ref<JsBufferValue>) -> Self {
    Self { data }
  }
}

#[napi]
impl Task for CountBufferLength {
  type Output = usize;
  type JsValue = JsNumber;

  fn compute(&mut self) -> Result<Self::Output> {
    if self.data.len() == 10 {
      return Err(Error::from_reason("len can't be 5".to_string()));
    }
    Ok((&self.data).len())
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    env.create_uint32(output as _)
  }

  fn finally(&mut self, env: Env) -> Result<()> {
    self.data.unref(env)?;
    Ok(())
  }
}
```

<Callout>
The `#[napi]` macro above the `impl Task for AsyncFib` is just for `.d.ts` generation. If no `#[napi]` is defined here, the generated TypeScript type of returned `AsyncTask` will be `Promise<unknown>`.
</Callout>

## `AsyncTask`

The `Task` you define cannot be returned to JavaScript directly, the JavaScript engine has no idea how to run and resolve the value from your `struct`. `AsyncTask` is a wrapper of `Task` which can return to the JavaScript engine. It can be created with `Task` and an optional [`AbortSignal`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal).

```rust filename="lib.rs"
#[napi]
fn async_fib(input: u32) -> AsyncTask<AsyncFib> {
  AsyncTask::new(AsyncFib { input })
}
```

⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️

```ts filename="index.d.ts"
export function asyncFib(input: number) => Promise<number>
```

### Create `AsyncTask` With `AbortSignal`

In some scenarios, you may want to abort the queued `AsyncTask`, for example, using `debounce` on some compute tasks. You can provide `AbortSignal` to `AsyncTask`, so that you can abort the `AsyncTask` if it has not been started.

```rust {4} filename="lib.rs"
use napi::bindgen_prelude::AbortSignal;

#[napi]
fn async_fib(input: u32, signal: AbortSignal) -> AsyncTask<AsyncFib> {
  AsyncTask::with_signal(AsyncFib { input }, signal)
}
```

⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️

```ts filename="index.d.ts"
export function asyncFib(input: number, signal: AbortSignal) => Promise<number>
```

If you invoke `AbortController.abort` in the JavaScript code and the `AsyncTask` has not been started yet, the `AsyncTask` will be aborted immediately, and reject with `AbortError`.

```js {6} filename="test.mjs"
import { asyncFib } from './index.js'

const controller = new AbortController()

asyncFib(20, controller.signal).catch((e) => {
  console.error(e) // Error: AbortError
})

controller.abort()
```

You can also provide `Option<AbortSignal>` to `AsyncTask` if you don't know if the `AsyncTask` needs to be aborted:

```rust filename="lib.rs"
use napi::bindgen_prelude::AbortSignal;

#[napi]
fn async_fib(input: u32, signal: Option<AbortSignal>) -> AsyncTask<AsyncFib> {
  AsyncTask::with_optional_signal(AsyncFib { input }, signal)
}
```

⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️

```ts filename="index.d.ts"
export function asyncFib(
  input: number,
  signal?: AbortSignal | undefined | null,
): Promise<number>
```

<Callout>
  If `AsyncTask` has already been started or completed, the
  `AbortController.abort` will have no effect.
</Callout>

# ThreadsafeFunction

[`ThreadSafe Function`](https://nodejs.org/api/n-api.html#asynchronous-thread-safe-function-calls) is a complex concept in Node.js. As we all know, Node.js is single threaded, so you can't access [`napi_env`](https://nodejs.org/api/n-api.html#napi_env), [`napi_value`](https://nodejs.org/api/n-api.html#napi_value), and [`napi_ref`](https://nodejs.org/api/n-api.html#napi_ref) on another thread.

import { Callout } from 'nextra-theme-docs'

<Callout>
  [`napi_env`](https://nodejs.org/api/n-api.html#napi_env),
  [`napi_value`](https://nodejs.org/api/n-api.html#napi_value), and
  [`napi_ref`](https://nodejs.org/api/n-api.html#napi_ref) are low level
  concepts in `Node-API`, which the `#[napi]` macro of **NAPI-RS** is built on
  top of. **NAPI-RS** also provides a [low level
  API](../compat-mode/concepts/env) to access the original `Node-API`.
</Callout>

`Node-API` provides the complex `Threadsafe Function` APIs to call JavaScript functions on other threads. It's very complex so many developers don't understand how to use it correctly. **NAPI-RS** provides a limited version of `Threadsafe Function` APIs to make it easier to use:

```rust {10} filename="lib.rs"
use std::thread;

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};

#[napi]
pub fn call_threadsafe_function(callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx| {
      ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    })?;
  for n in 0..100 {
    let tsfn = tsfn.clone();
    thread::spawn(move || {
      tsfn.call(Ok(n), ThreadsafeFunctionCallMode::Blocking);
    });
  }
  Ok(())
}
```

⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️

```ts filename="index.d.ts"
export function callThreadsafeFunction(callback: (...args: any[]) => any): void
```

`ThreadsafeFunction` is very complex so **NAPI-RS** does not provide the precise TypeScript definition generation of it. If you want to have a better TypeScript type, you can use `#[napi(ts_args_type)]` to override the type of `JsFunction` argument:

```rust {8} filename="lib.rs"
use std::thread;

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};

#[napi(ts_args_type = "callback: (err: null | Error, result: number) => void")]
pub fn call_threadsafe_function(callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx| {
      ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    })?;
  for n in 0..100 {
    let tsfn = tsfn.clone();
    thread::spawn(move || {
      tsfn.call(Ok(n), ThreadsafeFunctionCallMode::Blocking);
    });
  }
  Ok(())
}
```

⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️

```ts filename="index.d.ts"
export function callThreadsafeFunction(
  callback: (err: null | Error, result: number) => void,
): void
```

## ErrorStrategy

There are two different error handling strategies for `Threadsafe Function`. The strategy can be defined in the second generic parameter of `ThreadsafeFunction`:

```rust filename="lib.rs"
let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = ...
```

The first argument in the generic parameter of course is the return type of the `Threadsafe Function`.

### `ErrorStrategy::CalleeHandled`

`Err` from Rust code will be passed into the first argument of the JavaScript callback. This behaviour follows the async callback conventions from Node.js: https://nodejs.org/en/learn/asynchronous-work/javascript-asynchronous-programming-and-callbacks#handling-errors-in-callbacks. Many async APIs in Node.js are designed in this shape, like `fs.read`.

With `ErrorStrategy::CalleeHandled`, you must call the `ThreadsafeFunction` with the `Result` type, so that the `Error` will be handled and passed back to the JavaScript callback:

```rust {10,17} filename="lib.rs"
use std::thread;

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};

#[napi(ts_args_type = "callback: (err: null | Error, result: number) => void")]
pub fn call_threadsafe_function(callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = callback
    .create_threadsafe_function(0, |ctx| {
      ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    })?;
  for n in 0..100 {
    let tsfn = tsfn.clone();
    thread::spawn(move || {
      tsfn.call(Ok(n), ThreadsafeFunctionCallMode::Blocking);
    });
  }
  Ok(())
}
```

### `ErrorStrategy::Fatal`

No `Error` will be passed back to the JavaScript side. You can use this strategy to avoid the `Ok` wrapping in the Rust side if your code will never return `Err`.

With this strategy, `ThreadsafeFunction` doesn't need to be called with `Result<T>`, and the first argument of JavaScript callback is the value from the Rust, not `Error | null`.

```rust {10,17} filename="lib.rs"
use std::thread;

use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};

#[napi(ts_args_type = "callback: (result: number) => void")]
pub fn call_threadsafe_function(callback: JsFunction) -> Result<()> {
  let tsfn: ThreadsafeFunction<u32, ErrorStrategy::Fatal> = callback
    .create_threadsafe_function(0, |ctx| {
      ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    })?;
  for n in 0..100 {
    let tsfn = tsfn.clone();
    thread::spawn(move || {
      tsfn.call(n, ThreadsafeFunctionCallMode::Blocking);
    });
  }
  Ok(())
}
```

⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️

```ts {2} filename="index.d.ts"
export function callThreadsafeFunction(callback: (result: number) => void): void
```

# async fn

<Callout>
You must enable the ***async*** or ***tokio_rt*** feature in `napi` to use `async fn`:

```toml {3} filename="Cargo.toml"
[dependencies]
napi = { version = "2", features = ["async"] }
```

</Callout>

You can do a lot of async/multi-threaded work with `AsyncTask` and `ThreadsafeFunction`, but sometimes you may want to use the crates from the Rust async ecosystem directly.

**NAPI-RS** supports the `tokio` runtime by default. If you `await` a tokio `future` in `async fn`, **NAPI-RS** will execute it in the tokio runtime and convert it into a JavaScript `Promise`.

```rust {6} filename="lib.rs"
use futures::prelude::*;
use napi::bindgen_prelude::*;
use tokio::fs;

#[napi]
async fn read_file_async(path: String) -> Result<Buffer> {
  fs::read(path)
    .map(|r| match r {
      Ok(content) => Ok(content.into()),
      Err(e) => Err(Error::new(
        Status::GenericFailure,
        format!("failed to read file, {}", e),
      )),
    })
    .await
}
```

⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️⬇️

```ts filename="index.d.ts"
export function readFileAsync(path: string): Promise<Buffer>
```

# Await Promise

Awaiting a JavaScript `Promise` in Rust sounds crazy, but it's feasible in **NAPI-RS**.

import { Callout } from 'nextra-theme-docs'

<Callout>
  Awaiting a JavaScript `Promise` needs the `tokio_rt` and `napi4` features to
  be enabled.
</Callout>

```rust filename="lib.rs"
use napi::bindgen_prelude::*;

#[napi]
pub async fn async_plus_100(p: Promise<u32>) -> Result<u32> {
  let v = p.await?;
  Ok(v + 100)
}
```

```js {4} filename="test.mjs"
import { asyncPlus100 } from './index.js'

const fx = 20
const result = await asyncPlus100(
  new Promise((resolve) => {
    setTimeout(() => resolve(fx), 50)
  }),
)

console.log(result) // 120
```