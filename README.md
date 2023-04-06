## NPM Scripts run checker

An experiment to play with rust. 

This is designed to run against an npm `pacakge.json` file. It 
tests that each 'script' that you run with `npm run [script-name] is actually present in the scripts object 
of the package.json file

## Why would you use this? 
Well... you probably would not want to. This was an attempt at playing with and learning rust 

I may or may not create a better lexer/parser at some point

## Example of invlaid scripts 

```json
  [....]
  "scripts": {
      "run": "npm run something",
      "something": "npm run lint && sh run-nothing" ,
      "lint": "linting",
      "invalid": "npm run this-is-not-valid"
  }
  [....]
```

## Example of valid scripts 

```json
  [....]
  "scripts": {
      "run": "npm run something && npm run lint",
      "something": "npm run lint && sh run-nothing" ,
      "lint": "linting",
  }
  [....]
```

## TODOs 
- Ability to add line numbers for invalid npm run -> will require a different idea for sending data to our parser
- Testing of more corner cases and other uses casees
- Not sure how npm handles some kind of recursive script, maybe this is something we can check for?
- Better Rust practices
- Deployable and easily usable in some kind of CI/CD?
