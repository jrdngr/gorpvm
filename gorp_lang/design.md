# Syntax Alternatives

## LISPy
```
(fn add (a b)
    (+ a b))

(fn add2 (a)
    (add a 2))

(set x 4)

(add2 x)
```


## JSONish
```
// Verbose
{
    name: add
    args: [a, b],
    body: { 
        a + b 
    },
}

// Shorthand
{ add, [a, b], a+b }
```

## LISPy without parentheses
```
fn add (a b) (+ a b)
fn add2 (a) (add a 2)
set x 4
add2 x
```


# Thoughts

```
set y "5"
// "5"
set z *5
// 5
set z **5
// "5"
```

`*` Changes the form of something
Kinda like Deref, but not for dereferencing
It can only change to one thing
Form chains


### Quoting

```
(x + 5)
// (x + 5)
*(x + 5)
// `(+ x 5)`
*`(+ x 5)`
// "x + 5" 
```

That last one wouldn't actually behave that way
If you can't think of a better example, it's a bad idea

### Types
```
5:int
'int 5 // It's a normal function
```

int     64-bit integer
float   64-bit floating poing
str     UTF-8 string
bool    true/false
()      Fixed-size list, (element) == element
[]      Variable-size list
{}      Code block
int?    nullable int


### Executing functions
```
add2 (n) (+ n 2)
// List of 3 elements
*add2 3
// 5
```

### Binding
Maybe everything is just a binding. Defining a function is binding with function definition syntax

```
set add2 (n) {
    n + 2
}

set x 1
// x = 1
add2 4
// 6

set add2 (n) { n + 2 }
```


### Piping
```
fn add2 n { n + 2 }
fn mul3 n { n * 3 }
fn even n { n % 2 == 0 ? n ? nil }

[1, 2, 3, 4, 5]
|> add2
|> + 2
|> mul3
// [15, 18, 21, 24, 27]
|> even
// [18, 24]
```

