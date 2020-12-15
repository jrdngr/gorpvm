# Syntax Alternatives

## LISPy
(fn add (a b)
    (+ a b))

(fn add2 (a)
    (add a 2))

(bind x 4)

(add2 x)



## JSONish
{
    name: add
    args: [a, b],
    body: { 
        a + b 
    },
}

