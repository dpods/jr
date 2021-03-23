# jr
An easier way to view JSON files from the command line

Examples:

Passing in a JSON file as an argument
```
$ jr example.json 
{
    "is_true": true,
    "nested": {
        "one": 1,
        "two": {
            "level": 2
        }
  
```

Reading from stdin
```
$ cat example.json | jr
{
    "is_true": true,
    "nested": {
        "one": 1,
        "two": {
            "level": 2
        }
    }
}
```

You can use to `-i` flag with values between 0-8 to control the indent levels. The default nesting level is 4.
```
$ jr -i 2 example.json 
{
  "is_true": true,
  "nested": {
    "one": 1,
    "two": {
      "level": 2
    }
  }
}
```

```
$ jr -i 8 example.json 
{
        "is_true": true,
        "nested": {
                "one": 1,
                "two": {
                        "level": 2
                }
        }
}
```


