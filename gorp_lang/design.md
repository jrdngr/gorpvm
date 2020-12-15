# Syntax Alternatives

## LISPy
(fn add (a b)
    (+ a b))

(fn add2 (a)
    (add a 2))

(set x 4)

(add2 x)



## JSONish
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


## LISPy without parentheses
fn add (a b) (+ a b)
fn add2 (a) (add a 2)
set x 4
add2 x


# Thoughts

set y "5"
// "5"
set z *5
// 5
set z **5
// "5"

// * Changes the form of something
// Kinda like Deref, but not for dereferencing
// It can only change to one thing
// Form chains


### Quoting

(x + 5)
// (x + 5)
*(x + 5)
// `(+ x 5)`
*`(+ x 5)`
// "x + 5" 
// That last one wouldn't actually behave that way
// If you can't think of a better example, it's a bad idea

### Types
5:int
'int 5 // It's a normal function
