# url_shortener-rs
Learning Rust, creating a URL Shortener

## User Stories:
- [ ] As a user, I want to input a long URL, so that I can get a shortened version of it.
- [ ] As a user, I want the shortened URL to be unique, so that it reliably redirects to the correct long URL.
- [ ] As a user, I want the shortened URL to be as short as possible, so that it is easy to share.
- [ ] As a user, I want to be able to use the shortened URL to be redirected to the original long URL, so that I can access the content easily.
- [ ] As a developer, I want to ensure that the short code generation is efficient and avoids collisions, so that the service remains reliable.
- [ ] As a developer, I want to store the mapping between long URLs and their short codes in a database, so that the data persists across sessions.
- [ ] As a developer, I want to validate the input URL to ensure it is a valid URL, so that the service does not generate short codes for invalid URLs.

## Technical Steps:
- [ ] Generate the simple mechanism to enter a long url and return a short url
- [ ] Build out a sqlite table to keep track of long url to short url combos
- [ ] Build the ability to store and retrieve from the database
- [ ] Build the ability to call the routine with the short url and have it launch the long url