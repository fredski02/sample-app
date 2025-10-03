# Sample App (Rust + Axum + Askama + SQLite + Kafka)


## Run Kafka
```bash
docker compose up -d
```

## Set env var
for sqlx compile errors to show set the following var
typically this needs to be a URI to this project location followed by sample.db
```bash
EXPORT DATABASE_UR="path-to-local.db" 
```

## Configure
```bash
cp .env.example .env
```

## Run
```bash
cargo run
```


Open http://localhost:3000

## example event to the sample-command topic

```json
{"CreateSample":{"input":{"name":"From Kafka","description":"Sausages!","status":"active"},"user_id":1}}

```



## improvements and notes

#### Auth

- unique user CRUD access - currently any user can edit a post but it should be tied down to the user that created it.
- no register - there is no register functionality
- magic links - instead of using a password, consider using just an email address and sending magic links to emails for sign in
- oAuth & JWT - for API access we might consider having oAuth especially for other public usage 
- sessions storage - currently the session lives in memory, ideally the session would live in something like redis so as to not cause a perf hit to the main DB
- solid auth - auth can be quite tricky, it might be better to use an external provider like clerk instead of building our own auth and adding all the parts that make it really secure. i say this because the point of the application isn't to build authentication but to provide a way to edit samples in a secure manner. building an auth system that is extremely secure might take just as long as the actual core functionality of application. theres a lot of nuance to this conversation and "it depends".

#### Database

- use postgres - sqlite is nice and perhaps its actually fine for something small but its worth considering postgres should the application be intended to be used by a lot of people.

#### General features

- fleshing out user / auth more - the user model hasn't really been fully developed. the standard features of registering, resetting password, updating details like name and address etc aren't in. 

#### Templates

- general error page - at the moment im creating various error pages but a general error page could be made and the status code and error passed to that instead of having many separate error templates.
- edit form template - currently we duplicate the edit and new form fields but with some tweaks it can be just one form field in this template. 
- edit form template status options - currently we hard code the options to be set in the html form select element but these can be passed into the template
- htmx 2 doesn't support IE - browser support depending on the region might be worth taking into consideration.
- responsive styling - not much responsive styling in at the moment
- reactivity sprinkle - consider adding in a little bit of reactivity via alpine.js or using datastar.js ( htmx + alpine + signal + sse )

#### Security

- CRSF - there is no cross site request forgery protection
- XSS protection - use safe-nonce htmx plugin and sanitize data + CSP
- rate limiting - there is no rate limiting
- ddos protection - using something like cloudflare as a proxy might be worth doing
- recaptcha - to prevent bots using the forms we might consider adding google recaptcha
- form input sanitisation - form input isn't sanitized but it should be to avoid databases evaluating sql statements.
- js sec - right now there is minimal JS used however it should be scoped within an annonymously invoked function to not leak variables etc.

#### Misc

- encapsulate in docker - having the whole app running in docker will reduce system dependencies for other developers to need should the project grow
- api versioning - consider using /api/v{x}/{route} to version the API
- error handling - right now we're using anyhow's Result to handle errors but it might be more prudent to setup better error handling.
- unwrap() - related to error handling, some of the unwraps should be properly handled and the relevent error propagated up to main to be logged.
- swagger API docs

#### Performance & Analytics
- consider a main write database and a read database
- utilise redis for a cache of commonly rendered responses, especially marketing pages and a ttl for cache
- CDN for assets - during CI/CD deployments we might consider automatically sending static assets to a CDN for improved page load times.
- app analytics - sending standard analytics like memory usage and cpu usage to something like prometheus and grafana would be nice and for alerting if prudent. 
- bug tracking - having a frontend bug tracker to automatically send screenshots, meta information and stack traces with alerts is a nice way to know your application is breaking without needing to be alerted by users first which helps us respond quicker.
- google analytics - getting stats on how the site is used by users, heat maps etc can help to form opinions on UX etc
- minify assets ( css, js, images )
- add retry logic with exponential backoff 
- use preload htmx extension for fully static template parts


#### SEO

- no meta tags - the basic tags aren't in and its generally good to have some core meta tags like title and canonical tags in there for search engines

#### CI/CD/Environments

- CI/CD - just non-existant at this stage but its obvious we'd want a build pipeline with testing to be run
- dev/staging envrionments - its always nice to have a place where you can test things out in a working server environment

#### Testing
- a series of tests, unit tests for complex math, database tests, and maybe e2e for a core user path might be considered worth while depending on the core user path.

#### Dev experience
- maybe use a hot reloader to auto restart the server when html changes are detected.
- improve seedingof database
