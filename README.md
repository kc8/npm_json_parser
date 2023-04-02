## NPM Scripts run checker

An experiment to play with rust. 

This is designed to run against an npm `pacakge.json` file. It 
tests that each 'script' that you run with `npm run [script-name] is actually present in the scripts object 
of the package.json file

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

## Areas of Improvment and TODOs 
- Ability to add line numbers for invalid npm run -> will require a different idea for sending data to our parser
- Testing of more corner cases and other uses casees
- Not sure how npm handles some kind of recursive script, maybe this is something we can check for?
- Better Rust practices
