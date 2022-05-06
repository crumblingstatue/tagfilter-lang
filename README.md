# tagfilter-lang

A small simple language for filtering stuff based on tags.
It is primarily developed for the cowbump image organizer application.

## Specification

Tagfilter-lang consists of a single line that specifies the tag requirements.
It consists of zero or more *requirements*, all of which must match.


### requirement
A requirement can either be a *tag*, or a *function call*

### tag
a tag can be anything that doesn't begin with the character `@`, and doesn't contain any spaces or `[`/`]`.
Valid examples:
```
hello-world
hello@i@am@a@tag
brick(character)
2
```

### function call
A function call begins with a keyword that starts with `@`, and the parameters are contained
in square brackets. There can be nested function calls. Parameters are separated by whitespaces.
If a function call doesn't have any parameters it doesn't need a square bracketed list.
For example `@untagged` can be a special function that means match things that don't have any tags.
Functions are defined by the application that embeds tagfilter-lang.

## Examples

```
# Matches the tag bicycle
bicycle
# Must match either foo or bar, and also it has to match baz
@any[foo bar] baz
# Matches either a cat, or a dog with a stick
@any[cat @all[dog stick]]
# Matches things that are not tagged
@untagged
```
