# bible-api

A REST API written in Rust for accessing the CPDV Bible


## Documentation

`GET bible.youko.dev/`

Returns the entire Bible

`GET bible.youko.dev/book/<book_name>`

Returns the specified book

`GET bible.youko.dev/book/<book_name>/chapter/<chapter_number>`

Returns the specified chapter of the specified book

`GET bible.youko.dev/book/<book_name>/chapter/<chapter_number>/verse/<verse_number>`

Returns the specified verse of the specified chapter of the specified book


## License

[MIT](https://choosealicense.com/licenses/mit/)

