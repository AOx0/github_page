# Parser combinator notes

These are just my notes from Scott Wlaschin’s lecture, available at [**YouTube**](https://www.youtube.com/watch?v=RDalzi7mhdY). Scott uses F# at his slices. I’m going to rewrite as good as my ability permits to Rust code. 

Parsing is a crucial task in computer science, as it allows us to process and interpret the data that is input to our programs. There are many approaches to parsing, and parser combinators are one powerful and flexible tool that can be used to build parsers.

Scott explains what parser combinators are and how they work. He starts by looking at the basics of how to build simple parsers that can match individual characters or patterns in input data. From there, he expands on how to combine these basic parsers into more complex parsers using combinator functions. By the end of the talk, you have a grasp of what parser combinators are and how to build your own parsers from simple blocks.

This essay, because of its nature of being my personal notes on the talk, will take a very close approach, building over Scotts slides my implementation and interpretation as objectively as possible with explanations about the code.

## Parsing a single character

Let’s start by creating a simple parser that matches for a character `'A'`. First, we will need an enumerator for returning errors. The code is pretty self-explanatory, we have two edge cases that result in failure; either we have an empty input string or the character we are matching against (`'A'`) is not present in the place we expect it to be within the input chars.

```rust
enum ParserError<'a> {
    EmptyInput,
    NotFound(&'a str),
}
```

Then we have a parser function `pcharA`, which verifies if the first character of the string being inputed matches the character `'A'` and returns the remaining input’s slice if successful.

```rust
fn pcharA(input: &str) -> Result<&str, ParserError> {
	
	// We return an error if the string is empty.
	// It may be empty because the parser has finished
    if input.is_empty() {
        Err(ParserError::EmptyInput)
    } 

	// If its not empty, we match the first character of the string with 'A'
	else if input.chars().nth(0).expect("Found empty string") == 'A' {
		// If it'a a match, we return an Ok status with the remainig string.
		// We do this because we can continue to parse from where this function
		// lefts it.
        Ok(&input[1..])
    } 

	else {
		// If its not a match, we return an error status.
        Err(ParserError::NotFound(input))
    }
}
```

As for now, we can visually see this function as a microchip with one input pin and two possible output pins, one for failure and another one for an `Ok` status.

![Parsing a constant char 'A'](/static/blog/wlaschin-parser-combinators/pcharA.png)

The next update is to make this code accept any character `match_char` to be parsed. This update demands us to update the error enumerator to include more information like what character was expected and what was found.

```rust
#[derive(Debug)]
enum ParserError {
    EmptyInput,
    NotFound { expected: char, found: char },
}
```

The function is no longer named `pcharA`, instead, `pchar` reflects its ability to parse any given char from any given string input.

```rust
fn pchar(input: &str, match_char: char) -> Result<(char, &str), ParserError> {
    if input.is_empty() {
        Err(ParserError::EmptyInput)
    } 
	// The only difference is now we match to the input character
	else if input.chars().nth(0).expect("Unexpected empty str") == match_char {
        Ok((match_char, &input[1..]))
    } else {
		// In case of failure, we now return more information as context.
        Err(ParserError::NotFound {
            expected: match_char,
            found: input.chars().nth(0).expect("Unexpected empty str"),
        })
    }
}
```

The updated microchip of our parser now looks like the following image.

![Parsing a char \`match\_char\`](/static/blog/wlaschin-parser-combinators/pchar.png)

We can then call the parser to test it out as follows:

```rust
fn main() {
    // Ok(('A', "BACADDA"))
    println!("{:?}", pchar("ABACADDA", 'A'));

    // Err(NotFound { expected: 'B', found: 'A' })
    println!("{:?}", pchar("ABACADDA", 'B'));

    // Err(EmptyInput)
    println!("{:?}", pchar("", 'A'));
}
```

We can take advantage of the parser’s Ok status return type since it contains a reference to the remaining input to parse to create parsing loops that end when the remaining is empty. 

```rust
fn main() {
    let input = "ABABBBAAB";
    let mut remain = input;
    let mut match_char = 'A';

    loop {
        match pchar(remain, match_char) {
            Ok((matched, rem)) => {
                println!("Found {matched}, remaining: {rem}");
                remain = rem;
            }
            Err(ParserError::EmptyInput) => {
                println!("Remaining is empty");
                break;
            }
            Err(ParserError::NotFound { expected, found }) => {
                println!("Found {found} but expected {expected}");
                match_char = match match_char {
                    'A' => 'B',
                    'B' => 'A',
                    c => panic!("Found unexpected char {c}"),
                };
            }
        }
    }
}
```

The code above is an elementary example of a toy parser that looks for characters `{'A', 'B'}`.  When an unexpected char is encountered, the matching char changes to the other alternative, if the character is not `'A'` nor `'B'` then a panic occurs. The point here is we can loop over the remaining string to parse until we find an `Err(ParserError::EmptyImput)`, in which case we know the remain string to parse is empty, and thus we have finished parsing.

## Taking it purely functional

Up to now, there’s nothing strange about these two past functions, they are what we all expect to be. The next step though includes one extra step which turns up the thing to be more interesting. 

Instead of having a function that takes a character to match while parsing, we are making a function that can craft functions. This is, instead of building a function to parse x char, we are programming a function that can build another function that parses a constant, bounded character. 

To fully understand this, let’s take a closer look at the concept of function-builder functions. A function-builder function is a function that creates and returns another function. It takes one or more arguments, and the function it returns is "bound" to those arguments – in other words, the returned function has access to the arguments that were passed to the function builder. 

Within Rust, as with many other languages, we have a concept named “closure”. A closure is an anonymous function that we can bound to variables and pass as an argument to other functions. In the parser world, and thanks to functional programming languages, this allows us to make complex combinations of smaller closures to create big complex parsing systems with ease.

With this in mind, `pchar` is now becoming a function-builder function that looks something like:

![Parser building](/static/blog/wlaschin-parser-combinators/bpchar.png)

Hence, `pchar` is a closure builder. The resulting closure looks:

![](/static/blog/wlaschin-parser-combinators/finput.png)

Another way to see this new paradigm is to interpret `pchar` as a function factory that literally records in bare metal the character the function is building will match for.

![](/static/blog/wlaschin-parser-combinators/funcfactory.png)

Note how `match_char` is embedded in the closure and thus no longer required as one of the closure inputs. The `pchar` function-builder code is the following:

```rust
fn pchar(match_char: char) -> impl Fn(&str) -> Result<(char, &str), ParserError> {
    move |input| {
        if input.is_empty() {
            Err(ParserError::EmptyInput)
        } else if input.chars().nth(0).expect("Unexpected empty str") == match_char {
            Ok((match_char, &input[1..]))
        } else {
            Err(ParserError::NotFound {
                expected: match_char,
                found: input.chars().nth(0).expect("Unexpected empty str"),
            })
        }
    }
}
```


